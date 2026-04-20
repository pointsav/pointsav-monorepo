# YOYO-COMPUTE.md
# Yo-Yo Compute Substrate — service-slm Extension
**Version:** 1 · April 20, 2026
**Owner service:** service-slm (extends existing Doorman specification in SERVICES.md)
**Scope:** The reusable compute substrate that lets Gemma 4 (or any future LLM/SLM) be torn down and restarted across projects while retaining state, accumulating skill, and producing an auditable ledger of every spin-up and spin-down.

---

## 1. Framing

### Why this document exists

`service-slm` already owns the Doorman discipline — sanitise outbound, compute externally, re-hydrate inbound. What it does not yet specify is **what state survives teardown**, **what state is intentionally rebuilt**, and **how that distinction is logged**. Without that specification, the yo-yo is correct-in-theory but will produce either runaway cost (over-persistence) or silent knowledge loss (under-persistence) the moment it is used for a second project.

This document answers three questions:

1. How is the GCP node kept "ready" to spin up without paying the hyperscaler tax for idle GPU?
2. How does Gemma 4 retain memory across teardowns, and how does that memory grow as we add more projects?
3. How is every spin-up, spin-down, and state mutation captured in a SOC3-grade audit log?

### Why it belongs in service-slm

`service-slm` is already the single boundary where local insufficiency meets external compute. Every future project will either (a) have a Totebox Archive with enough resources to run an LLM locally, in which case `service-slm` is a thin passthrough, or (b) not have enough resources, in which case `service-slm` routes to the yo-yo substrate. The API key vault, the ledger, and the compute manifest all live at the same boundary. Splitting them across services would create three coordination problems for one concern.

### The Three-Ring Memory model

Ring | Name | Storage | Rebuild cost | Survives teardown? | Governed by
---|---|---|---|---|---
1 | Bootstrap | Container image + GCS-cached weights + Secret Manager | ~5–15 seconds | Yes (as artifacts) | `service-slm/compute/`
2 | Working (KV cache) | LMCache + Mooncake Store → object storage | Near-zero for cache hits | Yes (pooled) | `service-slm/memory/kv/`
3a | Long-term — graph | LadybugDB on service-content | None (query-time) | Yes (authoritative) | service-content (existing)
3b | Long-term — skill | LoRA adapter stack, OCI Artifacts | One-time per project | Yes (portable) | `service-slm/memory/adapters/`

Everything else is ephemeral and intentionally discarded.

---

## 2. Ring 1 — Bootstrap State (the "always ready" layer)

### The cost-vs-latency trap

The hyperscaler pattern presents a false binary. Either you pay for a warm GPU instance (Vertex AI endpoint: ~$600–3000/month minimum for a single A100 warm), or you pay the full cold-start penalty on every invocation (serverless GPU with model download: 60–120 seconds for a 26B model).

Neither is correct for our usage pattern. The DKA workload is **bursty but predictable** — weekly batch jobs, plus opportunistic query-time generation. The right target is **sub-30-second warm starts at zero idle cost**, not "zero cold start at high idle cost."

### The Four-Layer Bootstrap

The node is "always ready to spin up" because four layers are pre-staged in cheap, cold storage:

1. **Pre-built vLLM container** in Artifact Registry. Slim base image (`vllm/vllm-openai` or custom), ~15 GB. Pulled from regional mirror on node boot. ~5–10s.
2. **Pre-downloaded model weights** in GCS (`gs://dka-checkpoints/models/gemma-4-26b-a4b/`). Mounted via Cloud Storage FUSE or rsync'd on boot. Avoids re-downloading 50 GB from Hugging Face every cycle. Google's own 2026 Cloud Run GPU codelab uses this exact pattern for Gemma 4 on RTX 6000 Pro.
3. **Cloud Run GPU scale-to-zero** with drivers pre-installed. Per Google's April 2026 documentation, L4 and RTX PRO 6000 instances start in ~5 seconds with drivers already available. No driver install on the critical path.
4. **Warm pool opt-in via `min-instances=1`** only for sustained-load windows (e.g., overnight full-corpus runs). Switched on via SkyPilot before the window, switched off when the window closes. Idle billing applies only during that window.

### Why this beats the hyperscaler offering

Vertex AI's managed endpoints bill continuously for the GPU. Cloud Run GPU bills per-second of request-processing (or per-second of min-instance uptime if you opt in). For our pattern — a few hours of heavy compute per week, plus sporadic query-time calls — Cloud Run GPU is **~10–50× cheaper** than a warm Vertex endpoint while giving us ~15s worst-case cold start.

The vLLM project has an open RFC (as of February 2026) for **CUDA checkpoint/restore**, which Modal has already demonstrated delivers 10× cold-start improvement by snapshotting GPU virtual address space to disk. When that ships (likely late 2026 / 2027), Ring 1 warm start drops to single digits even for the 26B model. We should architect service-slm to accept a checkpoint bundle as an optional bootstrap input now, so adopting this is a config change rather than a rewrite.

### service-slm/compute/ file layout

```
service-slm/compute/
├── manifest.yaml           ← current GCP node config (image, GPU tier, region)
├── container/
│   ├── Dockerfile          ← vLLM + LMCache + FastAPI + pipeline scripts
│   ├── requirements.txt    ← pinned to STACK.md versions
│   └── build.sh            ← Cloud Build invocation
├── weights/
│   └── registry.yaml       ← GCS paths for each model (Gemma 4, OLMo 3, Phi-4, etc.)
├── sky/
│   ├── ingest.yaml         ← SkyPilot task: batch graph build
│   ├── warmpool.yaml       ← SkyPilot service: min-instances=1 for burst windows
│   └── teardown.yaml       ← SkyPilot task: explicit scale-to-zero + log
└── keys/
    └── secret-refs.yaml    ← Secret Manager references (not keys themselves)
```

---

## 3. Ring 2 — Working Memory (KV Cache That Survives Teardown)

### The problem this solves

When the GCP node tears down, the GPU KV cache dies with it. On the next spin-up, Gemma re-prefills every prompt from scratch — even if 90% of the input (the graph subgraph, the system prompt, the CoA classification context) is identical to what it just processed. This is the primary cause of the "yo-yo feels slow on the second run" experience.

### The fix: LMCache + Mooncake Store

**LMCache** (integrates with vLLM via the KV connector interface) hashes blocks of tokens and fetches matching KV cache blocks from a tiered store: GPU → CPU DRAM → remote storage. **Mooncake Store** is the remote storage tier — a distributed KV pool that **survives vLLM instance teardown** because it persists in CPU DRAM on persistent hosts or in SSD-backed object storage (S3/GCS).

Per the Ceph.io benchmarks (December 2025) and Moonshot AI's production Kimi deployment: the approach delivers order-of-magnitude TTFT improvements for repeated-prefix workloads. The DKA workload is *entirely* repeated-prefix: every document you process against the Woodfine graph shares ~2,000 tokens of CoA spine, archetype catalog, and prompt scaffolding.

### Concrete configuration

The vLLM serve command on the GCP node:

```bash
vllm serve google/gemma-4-26b-a4b-it \
  --max-model-len 32768 \
  --enable-prefix-caching \
  --kv-transfer-config '{"kv_connector":"LMCacheConnectorV1","kv_role":"kv_both"}'
```

LMCache config (`mooncake-config.yaml`):

```yaml
local_cpu: true
max_local_cpu_size: 4            # 4 GiB local buffer
remote_url: "mooncakestore://gcs.mooncake.dka.internal:50051/"
pre_caching_hash_algorithm: sha256_cbor_64bit

extra_config:
  protocol: "tcp"                 # RDMA where available, TCP otherwise
  global_segment_size: 21474836480  # 20 GiB per worker
  master_server_address: "gcs.mooncake.dka.internal:50051"
  moduleId: "${DKA_MODULE_ID}"    # namespaces the cache by project
```

The `moduleId` namespace is the critical detail. Woodfine's KV cache blocks never collide with a future PointSav client's. The same physical cache pool serves many projects, but queries only hit their own module's blocks.

### Cost and operational footprint

- **Mooncake Store master**: 1 small always-on VM (~$20/month) or run on a Totebox node
- **Storage tier**: GCS standard class, ~1–5 GB per project, pennies per month
- **Network cost**: intra-region GCS reads are free or cheap; RDMA where it exists

This is not an expensive addition. The total Ring-2 infrastructure cost for the first ten projects is under $100/month.

### service-slm/memory/kv/ file layout

```
service-slm/memory/kv/
├── config.yaml             ← LMCache + Mooncake master config
├── hash-seed               ← PYTHONHASHSEED for consistent block hashing across processes
├── master.yaml             ← Mooncake master service deployment spec
└── stats.csv               ← append-only log: timestamp, moduleId, cache_hits, cache_misses, bytes_transferred
```

---

## 4. Ring 3 — Long-Term Semantic Memory

Ring 3 has two distinct sub-layers. They are often conflated in sloppy "agent memory" literature. They have different storage, different refresh cadences, different licensing implications, and different commercial value.

### 3a — Graph memory (already yours)

The LadybugDB graph in `service-content` **is** the long-term semantic memory for any project. Gemma reads from it at context-assembly time (Role 2 of service-slm — the content generation bridge). It never writes back directly; writes flow through `apply_delta.py` after the sanitise / compute / rehydrate cycle.

This layer is project-scoped by design. Woodfine's graph is Woodfine's. Nothing leaks across projects without an explicit export through service-marketplace. This is the right behaviour for data. It is the wrong behaviour for skill — see 3b.

### 3b — Adapter memory (the cross-project skill library)

This is the layer Jennifer's question was really about: "how do we make Gemma grow across all our projects?"

The answer is a **LoRA adapter stack** — small, versioned, frozen-weight modules that sit on top of the base Gemma weights and encode task-specific behaviour (CoA classification, archetype detection, entity resolution, wiki synthesis). A 26B base model plus a 50 MB LoRA adapter runs at near-base speed on the same GPU.

Each adapter is trained once, versioned, stored as an OCI Artifact (Sigstore-signed, SLSA-attested per STANDARDS.md §Sovereign Identity), and loaded at vLLM boot:

```bash
vllm serve google/gemma-4-26b-a4b-it \
  --lora-modules \
    dka-coa=gs://dka-adapters/coa/v3.2 \
    dka-archetype=gs://dka-adapters/archetype/v2.1 \
    woodfine-entity=gs://dka-adapters/woodfine/entity/v1.4
```

At query time, the caller selects which adapter(s) to activate via the `moduleId` + task routing logic. A Woodfine query loads `dka-coa` + `dka-archetype` + `woodfine-entity`. A future PointSav client loads `dka-coa` + `dka-archetype` + `{client-id}-entity`. The shared adapters (`dka-coa`, `dka-archetype`) improve over time and every project benefits. The project-specific adapter stays with that project.

**This is the compounding asset.** Every project leaves behind an adapter. The base model is commodity. The adapter library is the DKA moat.

### Handling catastrophic forgetting

The 2025–2026 continual-learning research is directly relevant here. Naïve sequential fine-tuning of one adapter across multiple projects produces catastrophic forgetting — project 3 overwrites what project 1 taught. Two mitigations from the current literature:

1. **C-LoRA (Zhang et al. 2025, TPAMI):** single adapter with a learnable routing matrix + orthogonality constraints. Reaches 90% last-accuracy across 20 incremental sessions where basic LoRA degrades to 81%.
2. **CL-LoRA dual-adapter (He et al. 2025):** task-shared adapter for cross-task knowledge + task-specific adapters for unique features. The dual structure is the more conservative choice because it isolates project-specific knowledge behind a clean boundary.

**Recommendation for DKA:** start with CL-LoRA's dual pattern (shared `dka-*` adapters + per-project `{client}-*` adapters). Migrate to C-LoRA-style single routed adapters only when you have >10 projects and the proliferation becomes a management burden.

### service-slm/memory/adapters/ file layout

```
service-slm/memory/adapters/
├── registry.yaml           ← catalogue: adapter_id, version, base_model, training_data_hash, signature
├── train/
│   ├── coa.py              ← training script for CoA classification adapter
│   ├── archetype.py        ← training script for archetype detection adapter
│   └── entity.py.template  ← per-project entity adapter template
└── ledger/
    └── training.csv        ← append-only: adapter_id, version, trained_at, training_data_hash, evaluator_id
```

### The Letta / MemGPT analogy (and why we don't adopt it wholesale)

Letta (formerly MemGPT) offers an "LLM-as-OS" model with core memory (always-in-context), recall memory (searchable history), and archival memory (vector DB). It is the right shape for *interactive agents*, which is not the DKA workload. DKA is batch + query. We borrow the *tiering insight* (working memory ≠ long-term memory) but reject the runtime coupling — we are not running Letta as a dependency. Ring 2 + 3a + 3b is our equivalent, built from primitives we own.

---

## 5. The Audit Ledger — SOC3 Processing Integrity

Every yo-yo event is logged. Append-only CSV, never modified after write, included in the nightly YAML snapshot. Schema:

```csv
event_id, timestamp_utc, event_type, moduleId, node_id, job_id,
input_hash, adapter_versions, cache_hit_ratio, tokens_processed,
gpu_seconds, cost_usd, completion_status, error_code, operator_id
```

`event_type` values:
- `BOOT_REQUEST` — SkyPilot has been asked to spin up
- `BOOT_COMPLETE` — node is serving
- `JOB_START` — an ingest or query job has been submitted
- `JOB_COMPLETE` — job finished, delta returned
- `CHECKPOINT` — GCS checkpoint written
- `TEARDOWN_REQUEST` — explicit tear-down issued
- `TEARDOWN_COMPLETE` — node is gone, final cost recorded
- `PREEMPTION` — spot instance preempted
- `ADAPTER_LOAD` — a LoRA adapter was activated for a request
- `KV_POOL_SYNC` — Mooncake Store reconciliation event

### Why this matters commercially

This ledger is a SOC3 Processing Integrity artifact the hyperscalers structurally cannot produce. Vertex AI does not tell you which adapter weighed in on which answer, how much of the answer came from cached KV blocks versus fresh prefill, or what the per-call cost decomposition was. We log all of it. Every wiki page, every Data Marketplace export, every Ad Exchange segment traces back through this ledger to the exact compute event that produced it, the exact adapter versions, the exact source chunks.

This is the DARP I1 compliance chain extended from source-to-graph into source-to-output. It is one of the four or five structural arguments for why DKA data commands a premium price.

---

## 6. The moduleId Discipline

`moduleId` already exists in your RF2 envelope (SCHEMA.md — it appears on every node). We are not adding a new concept. We are extending its reach into compute.

Every call into `service-slm` carries a `moduleId`. The value propagates through:

- **Ring 1 (bootstrap):** selects which container variant to boot (rare — usually same for all projects)
- **Ring 2 (KV cache):** namespaces the Mooncake block hashes so Project A never sees Project B's blocks
- **Ring 3a (graph):** scopes the graph traversal to the right `moduleId` partition of LadybugDB
- **Ring 3b (adapters):** selects which LoRA adapter stack to activate
- **Ledger:** every entry is tagged, making per-project cost accounting trivial

**One field, five jobs.** This is why the RF2 envelope decision in DECISIONS.md D33 pays off so heavily — the field was added for versioning, but it also turned out to be the right primitive for multi-tenant isolation.

---

## 7. What Hyperscalers Structurally Cannot Offer

Not "do not offer today" — **cannot offer while their business model stands**.

Differentiator | Hyperscaler reality | DKA substrate
---|---|---
**Portable per-project skill library** | Vendor-locked fine-tuning endpoints | OCI Artifact-signed LoRA adapters, move freely across clouds
**Cross-project KV cache sharing** | Billed per-tenant, pool = security risk for them | Mooncake pool, `moduleId`-isolated, runs on our infra
**Per-call audit ledger linking source → output** | Black-box model providers | Every event logged, SOC3-attestable
**Air-gap during inference** | All your data flows through their model endpoint | Sanitise outbound, hydrate inbound, verifier uses own browser
**Pay zero when idle with <30s warm start** | Choose: expensive warm or painful cold | Bootstrap artifacts pre-staged; Cloud Run GPU 5s + vLLM load
**Open source end-to-end** | Proprietary at every layer | Every primitive (vLLM, LMCache, Mooncake, SkyPilot) forkable

The hyperscalers cannot match this without cannibalising their managed-fine-tuning, managed-endpoint, and managed-vector-DB revenue lines. They will not do that.

---

## 8. 2030 Headroom — What Plugs In Later

This substrate is designed so 2026–2028 primitives that are still research or RFC today slot in without architectural change:

| Primitive | Status today | Hook in service-slm |
|---|---|---|
| **CUDA checkpoint/restore** | vLLM RFC #34303, Feb 2026. Modal demonstrated 10× cold-start gain | Ring 1: accept checkpoint bundle as optional bootstrap input |
| **Cumulative LoRA (C-LoRA single-adapter)** | Published 2025 (arXiv 2502.17920) | Ring 3b: migrate registry schema from dual→single when adopted |
| **Distributed KV across clouds** | SkyPilot 0.11 Multi-Cloud Pools (Dec 2025) + Mooncake | Ring 2: Mooncake master runs on multi-cloud pool |
| **FP8 KV cache quantisation** | Already in vLLM (`KV_CACHE_DTYPE=fp8`) | Ring 2: config flag, ~2× memory reduction |
| **Sleep-time compute (async memory management)** | Letta sleep-time agents, 2025 | Ring 3b: nightly LoRA retraining on Batch API (50% discount) |
| **Encode-Prefill-Decode disaggregation** | SGLang + Mooncake, Dec 2025 | Ring 2 evolution: separate prefill and decode pools |
| **Homomorphic inference** | Academic, 2026+ | Ring 2 replacement: KV cache encrypted end-to-end. Speculative. |

None of these require rewriting service-slm. They require adding a config file and a new subdirectory under `service-slm/`.

---

## 9. Extended service-slm File Tree

Full updated tree, showing Phase-1 scope vs later phases:

```
service-slm/
├── outbound/                      ← Phase 1: sanitised payloads pending send
├── inbound/                       ← Phase 1: received graph deltas
├── log/                           ← Phase 1: doorman transaction log (CSV, append-only)
│
├── compute/                       ← Phase 1: Ring 1
│   ├── manifest.yaml
│   ├── container/
│   ├── weights/
│   ├── sky/
│   └── keys/
│
├── memory/                        ← Phase 2: Rings 2 and 3b
│   ├── kv/                        ← LMCache + Mooncake
│   │   ├── config.yaml
│   │   ├── hash-seed
│   │   ├── master.yaml
│   │   └── stats.csv
│   └── adapters/                  ← LoRA skill library
│       ├── registry.yaml
│       ├── train/
│       └── ledger/
│
└── ledger/                        ← Phase 1: yo-yo audit log
    ├── events.csv                 ← the master append-only ledger
    └── schema.md                  ← ledger schema documentation
```

---

## 10. Phase Roadmap

### Phase 1 (current — trial)
- Ring 1 fully built (bootstrap, Cloud Run GPU, SkyPilot)
- Ledger fully built with all event types defined from day one
- `moduleId` passed through every call even though there's only one project
- **Skip Rings 2 and 3b** — not yet needed, would bloat trial

### Phase 2 (after trial passes, before full corpus run)
- Add Ring 2: LMCache + Mooncake Store
- Measure cache hit ratio on second full corpus run
- Target: 60%+ cache hit rate on repeated-prefix work

### Phase 3 (after first commercial deployment beyond Woodfine)
- Add Ring 3b: first LoRA adapters (CoA, archetype, entity)
- Dual-adapter C-LoRA pattern
- Adapter registry + training pipeline

### Phase 4 (optional, when the research matures)
- CUDA checkpoint/restore integration
- Single-adapter C-LoRA migration
- Multi-cloud KV pool

---

## 11. Blocking Items Before Phase 2 Build-Out

These do not block Phase 1 trial — they block the Ring 2 / Ring 3b expansion.

1. **Confirm Mooncake/LMCache licensing posture.** Both are Apache 2.0 or MIT as of April 2026, but verify at adoption time. They are now in the PyTorch Ecosystem (Feb 2026), which is a strong signal of upstream stability.
2. **Decide Mooncake master hosting.** Three options: (a) small always-on GCE VM (~$20/mo), (b) co-host on a Totebox node, (c) managed via SkyPilot Pool with min_replicas=1. Recommend (a) for Phase 2; revisit when Totebox stabilises.
3. **Adapter training hardware.** LoRA training for a 26B model needs an A100 40GB for ~4 hours per adapter. Batch API (50% discount) makes this ~$30 per adapter training run. Budget accordingly.
4. **Adapter evaluation protocol.** Every new adapter version must pass a regression suite before being published to the registry. Protocol not yet designed. Ties into R7 in DECISIONS.md (archetype promotion thresholds).
5. **Key management upgrade.** Phase 1 uses SSH env vars (per STACK.md). Phase 2 must move to Secret Manager with service-account-scoped access. Add to P2 decision list.

---

## 12. Cross-References

- **SERVICES.md** — original service-slm definition (Two Outbound Roles, No Local Model). This document extends but does not replace that specification.
- **SCHEMA.md** — RF2 envelope including `moduleId`. The field we are reusing.
- **STACK.md** — SkyPilot, vLLM, Pydantic+instructor, SeaweedFS. All already on the stack; LMCache and Mooncake are additions to propose.
- **STANDARDS.md** — SLSA, Sigstore, OCI Artifacts. The signing chain for adapter releases.
- **GCP-NODE.md** — Phase 1 provisioning outline. Ring 1 specification above supersedes the "scripts to write" section only insofar as the container and sky YAMLs will now include LMCache hooks from day one (null-op for Phase 1 trial).
- **VERIFICATION.md** — ledger schema pattern was the model for the yo-yo ledger schema here.
- **DECISIONS.md** — add D41: "Yo-yo compute substrate specified in YOYO-COMPUTE.md; owned by service-slm."
