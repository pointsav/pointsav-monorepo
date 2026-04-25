# ARCHITECTURE.md — service-slm

**Scope.** This document specifies the internal architecture of
`service-slm` — the Rust cargo workspace, the three-ring memory
model, the flat-binary deployment pattern, and the `moduleId`
discipline. It does not argue *why* this architecture was chosen;
that doctrine lives in
`content-wiki-documentation/topic-service-slm.md` (Rust-native stack
rationale, licence discipline, open-source posture) and
`content-wiki-documentation/topic-yoyo-compute.md` (three-ring model
rationale, audit-ledger commercial argument, 2030 headroom).

Target-state document. The code does not yet match this shape — see
`NEXT.md` for the scaffolding sequence.

---

## 1. Purpose and role

`service-slm` is the single secure boundary between the isolated
Totebox Archive and any external Large Language Model. It implements
the **Doorman Protocol**: gate compute, not generate content.

Four operations:

1. **Sanitise outbound.** Strip direct identifiers and structured
   data from the payload; emit prose the external model can process.
2. **Route compute.** Local when the host has resources; remote
   (the yo-yo substrate on GCP) when it does not. The caller does
   not see the difference.
3. **Receive deltas.** Structured responses — graph deltas, wiki
   drafts, extracted entities — return from compute.
4. **Rehydrate inbound.** Verify against the ledger, reattach the
   identifiers stripped at step 1, commit to the authoritative
   store.

Nothing inside this service generates text. Generation happens
externally (Claude API in Phase 1; `mistral.rs` on the yo-yo node in
Phase 2). If code in this project starts producing text directly, it
has exceeded its remit — see `CLAUDE.md` Hard constraints.

SYS-ADR-07 applies without exception: structured data never routes
through the external LLM. Prose only.

---

## 2. Three-Ring memory model

The substrate distinguishes four memory tiers by rebuild cost and
persistence. Getting these boundaries right is the difference
between runaway cost (over-persistence) and silent knowledge loss
(under-persistence).

| Ring | Name | Storage | Rebuild cost | Survives teardown? | Governed by |
|---|---|---|---|---|---|
| 1 | Bootstrap | Container image + GCS-cached weights + Secret Manager | ~5–15 s | Yes (as artefacts) | `service-slm/compute/` |
| 2 | Working (KV cache) | LMCache + Mooncake Store → object storage | Near-zero on cache hit | Yes (pooled) | `service-slm/memory/kv/` |
| 3a | Long-term — graph | LadybugDB in `service-content` | None (query-time) | Yes (authoritative) | `service-content` (read-only from here) |
| 3b | Long-term — skill | LoRA adapter stack, OCI Artifacts | One-time per project | Yes (portable) | `service-slm/memory/adapters/` |

Everything else is ephemeral and intentionally discarded.

### Ring 1 — Bootstrap

Four pre-staged artefacts in cheap cold storage, pulled on boot:

1. Pre-built container in Artifact Registry (Phase 1 vLLM-based,
   ~15 GB; Phase 2 `mistral.rs` binary, ~200 MB).
2. Pre-downloaded model weights in GCS (e.g.
   `gs://dka-checkpoints/models/gemma-4-26b-a4b/`), mounted via
   Cloud Storage FUSE or `rsync`'d on boot.
3. Cloud Run GPU scale-to-zero with drivers pre-installed.
4. Warm pool opt-in via `min-instances=1` only for sustained-load
   windows.

Bill-per-second for request processing; zero idle cost outside
explicitly-opened warm windows.

### Ring 2 — Working (KV cache)

**LMCache** hashes blocks of tokens and fetches matching KV cache
blocks from a tiered store (GPU → CPU DRAM → remote). **Mooncake
Store** is the remote tier — a distributed KV pool that survives
inference-instance teardown. The Woodfine workload is entirely
repeated-prefix (every document processed shares ~2,000 tokens of
Chart-of-Accounts spine and prompt scaffolding), so cache hit rates
compound rapidly.

Phase 1 skips this ring. Phase 2 adds it.

### Ring 3a — Long-term graph (read-only from here)

The LadybugDB graph in `service-content` is the long-term semantic
memory. `service-slm` reads from it at context-assembly time.
`service-slm` never writes to it directly — writes flow through
`service-content`'s own ingest path after the sanitise / compute /
rehydrate cycle completes.

### Ring 3b — Long-term skill (LoRA adapter stack)

Small, versioned, frozen-weight modules that sit on top of the base
model and encode task-specific behaviour (CoA classification,
archetype detection, entity resolution, wiki synthesis). Each
adapter is trained once, versioned, stored as an OCI Artifact
(Sigstore-signed, SLSA-attested), and loaded at inference boot.
`moduleId` (§4) selects which adapter stack activates for a given
call.

Phase 3 scope — first adapters train after the first commercial
deployment beyond Woodfine.

---

## 3. Flat architecture — one binary

`service-slm` is one Rust binary running as a systemd unit (or
os-totebox init service). Inside the binary, logical modules talk
via Rust function calls, not RPC. External calls (Cloud Run,
Mooncake sidecar, Claude API, LadybugDB in `service-content`) are
the only network boundaries.

Consequences:

- One process to install, start, stop, update.
- One log stream (`tracing` → `journald` or Loki).
- One set of metrics.
- One binary to sign with Sigstore.
- One configuration file (TOML).

This is the shape an os-totebox appliance component wants. It is
not a set of Python-service processes coordinated over a message
bus.

---

## 4. The `moduleId` discipline

`moduleId` exists in the RF2 envelope (SCHEMA.md) and appears on
every node. This service extends its reach into compute. Every
call into `service-slm` carries a `moduleId`, which propagates
through:

| Ring / layer | Job |
|---|---|
| 1 — Bootstrap | Selects which container variant to boot (usually same across projects) |
| 2 — KV cache | Namespaces Mooncake block hashes so Project A never sees Project B's blocks |
| 3a — Graph | Scopes the traversal to the right `moduleId` partition of LadybugDB |
| 3b — Adapters | Selects which LoRA adapter stack to activate |
| Ledger | Tags every entry so per-project cost accounting is trivial |

One field, five jobs. This is a load-bearing primitive. Every new
code path that handles a request must propagate `moduleId`; the
`slm-core` crate defines the type and its serialisation.

---

## 5. Stack

Every dependency is MIT / Apache-2.0 / BSD / ISC / MPL-2.0 or Zlib.
No AGPL, GPL, LGPL, BSL, or custom "community" licences. Enforcement
is `cargo deny check licenses` in CI; `deny.toml` blocks new entries
with disallowed licences at build time. See `DEVELOPMENT.md` for the
policy file itself.

### 5.1 Inference layer

| Crate | Role | Licence |
|---|---|---|
| `mistralrs` | GPU/CPU LLM inference engine; OpenAI-compatible HTTP server built in | MIT |
| `candle-core` | Foundation ML framework underlying `mistralrs` | Apache-2.0 / MIT |

`mistral.rs` replaces Python + vLLM in Phase 2. Ships as a
statically-linked Rust binary plus CUDA kernels — ~200 MB, no GIL,
no Python runtime.

### 5.2 HTTP / RPC layer (the Doorman's wire)

| Crate | Role | Licence |
|---|---|---|
| `axum` | HTTP server (inbound from `service-content`, `os-console`, verification UI) | MIT |
| `tower` | Service middleware (retries, timeouts, backpressure) | MIT |
| `tokio` | Async runtime | MIT |
| `hyper` | HTTP client (Cloud Run, Claude API, LMCache master) | MIT |
| `reqwest` | High-level HTTP client | MIT / Apache-2.0 |
| `tonic` | gRPC (only if Mooncake Transfer Engine requires it) | MIT |

### 5.3 Storage and state

| Crate | Role | Licence |
|---|---|---|
| `sqlx` | SQL — SQLite for the ledger, optional Postgres | MIT / Apache-2.0 |
| `sled` | Embedded KV store (optional, fast local queues) | MIT / Apache-2.0 |
| `kuzu` | LadybugDB client bindings (Rust API) | MIT |
| `object_store` | Cloud object storage abstraction (GCS, S3, Azure) | Apache-2.0 |

### 5.4 Document processing

| Crate | Role | Licence |
|---|---|---|
| `oxidize-pdf` | PDF parsing, structure-aware chunking | MIT |
| `docx-rust` | `.docx` parsing | MIT |
| `pulldown-cmark` | Markdown parsing | MIT |
| `calamine` | `.xlsx` parsing | MIT |

`mupdf-rs` is not permitted — AGPL-3.0. `pdfium-render`
(Apache-2.0) is the fallback if `oxidize-pdf` hits a wall.

### 5.5 Orchestration

| Crate | Role | Licence |
|---|---|---|
| `apalis` | Job processing, retries, concurrency | MIT |
| `apalis-workflow` | Composable step-based workflow engine | MIT |
| `apalis-sqlite` | Persistence backend | MIT |
| `backoff` | Exponential backoff for yo-yo recovery | MIT / Apache-2.0 |

### 5.6 Networking, SSH, cloud

| Crate | Role | Licence |
|---|---|---|
| `russh` | Pure-Rust SSH | Apache-2.0 |
| `rustls` | Pure-Rust TLS | Apache-2.0 / MIT / ISC |
| `google-cloud-*` | Official Google Cloud SDK for Rust | Apache-2.0 |

### 5.7 Serialisation, validation, citation grounding

| Crate | Role | Licence |
|---|---|---|
| `serde` | Core serialisation | MIT / Apache-2.0 |
| `serde_json` / `serde_yaml` / `toml` | Format-specific | MIT / Apache-2.0 |
| `validator` | Field-level validation | MIT / Apache-2.0 |
| `schemars` | JSON Schema generation (LLM `response_format`) | MIT |

Citation grounding: `#[derive(Deserialize, Validate, JsonSchema)]`
on a struct, pass the JSON Schema to the external LLM as
`response_format`, reject on parse-or-validate failure. No Python,
no `instructor` runtime.

### 5.8 Observability

| Crate | Role | Licence |
|---|---|---|
| `tracing` | Structured logging and distributed tracing | MIT |
| `tracing-subscriber` | Subscriber implementations | MIT |
| `opentelemetry` | OpenTelemetry for SOC3 audit export | Apache-2.0 |
| `metrics-exporter-prometheus` | Prometheus metrics | MIT / Apache-2.0 |

### 5.9 Supply-chain security

| Crate | Role | Licence |
|---|---|---|
| `sigstore` | Keyless signing for container images and OCI artefacts (adapter releases) | Apache-2.0 |

See `DEVELOPMENT.md` for `cargo-audit`, `cargo-deny`, and
`cargo-sbom` invocation in CI.

### 5.10 Not-Rust components, behind network protocols

Three components sit outside the Rust ecosystem. All three are
behind stable network protocols; `service-slm` talks to them as a
client.

| Component | Language | Why kept | Integration |
|---|---|---|---|
| LMCache + Mooncake Store | Python + C++ (RDMA) | No Rust equivalent in 2026 | HTTP metadata + raw TCP/RDMA data-transfer |
| vLLM (Phase 1 only) | Python | Phase-1 reference path | HTTP — replaced by `mistral.rs` in Phase 2 |
| SkyPilot (if used) | Python | Multi-cloud abstraction, overkill for Phase 1 single-cloud | External driver, not linked |

All three are Apache-2.0 and forkable. The Rust binary calls them.
None of them infects this binary's licence or build chain.

---

## 6. Cargo workspace

```
service-slm/
├── Cargo.toml                  workspace manifest
├── deny.toml                   licence policy (block AGPL/GPL/BSL)
├── rust-toolchain.toml         pin compiler version
├── crates/
│   ├── slm-core/               shared types, errors, moduleId discipline
│   ├── slm-doorman/            sanitise / send / receive / rehydrate protocol
│   ├── slm-ledger/             append-only CSV + SQLite audit trail
│   ├── slm-compute/            Ring 1 bootstrap (Cloud Run driver, container mgmt)
│   ├── slm-memory-kv/          Ring 2 client (Mooncake + LMCache wire protocol)
│   ├── slm-memory-adapters/    Ring 3b adapter registry and loader
│   ├── slm-inference-local/    mistral.rs-backed local inference
│   ├── slm-inference-remote/   GCP yo-yo driver
│   ├── slm-api/                axum server: inbound endpoints
│   └── slm-cli/                operator CLI (main binary entry point)
└── xtask/                      build helpers, release automation
```

One binary produced (`slm-cli`). Shared crates above. Zero
microservice sprawl.

**Resolved 2026-04-25 — standalone workspace.** B1 scaffolding
(`crates/slm-core`, `crates/slm-doorman`, `crates/slm-doorman-server`)
landed under a self-contained `service-slm/Cargo.toml`. The decision
was the lowest-blast-radius path: it touches no code outside
`service-slm/` and leaves the monorepo unification cleanup item
(2026-04-18 audit, 8 of ~70+ crates declared) to be settled
separately. Conversion to a nested layout later — making
`service-slm/crates/*` members of a unified monorepo workspace — is
mechanical (move members up, drop nested `Cargo.toml`). The existing
`cognitive-forge/` subcrate remains an `exclude` member until its
rename is paired with `tool-cognitive-forge` per `NEXT.md`.

---

## 7. File tree

Full target layout, showing Phase-1 scope vs later phases.

```
service-slm/
├── README.md                     bilingual (English)
├── README.es.md                  bilingual (Spanish)
├── CLAUDE.md
├── NEXT.md
├── ARCHITECTURE.md               this file
├── DEVELOPMENT.md                build/CI policy, migration phases
├── Cargo.toml                    workspace manifest (Phase 1)
├── deny.toml                     licence policy (Phase 1)
├── rust-toolchain.toml
│
├── scripts/                      operator helpers, not production code
│   └── cognitive-bridge.sh       to be moved from project root
│
├── crates/                       see §6
│
├── outbound/                     Phase 1: sanitised payloads pending send
├── inbound/                      Phase 1: received graph deltas
├── log/                          Phase 1: doorman transaction log
│
├── compute/                      Phase 1: Ring 1 artefacts
│   ├── manifest.yaml             current GCP node config
│   ├── container/
│   │   ├── Dockerfile
│   │   ├── requirements.txt      Phase-1 reference path only
│   │   └── build.sh
│   ├── weights/
│   │   └── registry.yaml         GCS paths for each model
│   ├── sky/
│   │   ├── ingest.yaml
│   │   ├── warmpool.yaml
│   │   └── teardown.yaml
│   └── keys/
│       └── secret-refs.yaml      Secret Manager references (not keys)
│
├── memory/                       Phase 2+: Rings 2 and 3b
│   ├── kv/
│   │   ├── config.yaml           LMCache + Mooncake master config
│   │   ├── hash-seed
│   │   ├── master.yaml           Mooncake master deployment spec
│   │   └── stats.csv             append-only cache hit/miss log
│   └── adapters/                 LoRA skill library
│       ├── registry.yaml         catalogue: adapter_id, version, signature
│       ├── train/                training scripts per adapter
│       └── ledger/
│           └── training.csv      append-only training provenance
│
└── ledger/                       Phase 1: yo-yo audit log
    ├── events.csv                master append-only ledger
    └── schema.md                 ledger schema documentation
```

Only `compute/`, `outbound/`, `inbound/`, `log/`, `ledger/`, plus
the doorman / ledger / api crates, fall in Phase 1 scope.
`memory/kv/` is Phase 2. `memory/adapters/` is Phase 3.

---

## 8. Audit ledger

Every yo-yo event is logged. Append-only CSV, never modified after
write. Schema:

```
event_id, timestamp_utc, event_type, moduleId, node_id, job_id,
input_hash, adapter_versions, cache_hit_ratio, tokens_processed,
gpu_seconds, cost_usd, completion_status, error_code, operator_id
```

`event_type` vocabulary:

- `BOOT_REQUEST` — SkyPilot asked to spin up
- `BOOT_COMPLETE` — node is serving
- `JOB_START` — ingest or query job submitted
- `JOB_COMPLETE` — job finished, delta returned
- `CHECKPOINT` — GCS checkpoint written
- `TEARDOWN_REQUEST` — explicit tear-down issued
- `TEARDOWN_COMPLETE` — node is gone, final cost recorded
- `PREEMPTION` — spot instance preempted
- `ADAPTER_LOAD` — LoRA adapter activated for a request
- `KV_POOL_SYNC` — Mooncake Store reconciliation event

The ledger is a SOC3 Processing Integrity artefact. Every wiki
page, every Data Marketplace export, every Ad Exchange segment
traces back through this ledger to the exact compute event that
produced it, the exact adapter versions, the exact source chunks.
Doctrine rationale for why this matters commercially lives in
`content-wiki-documentation/topic-yoyo-compute.md`; this document
specifies only schema and vocabulary.

---

## 9. os-totebox integration

`service-slm` is the prototype os-totebox service component. The
flat-binary / pure-Rust-where-feasible pattern should template
future os-totebox services. Rationale for why Rust specifically
fits the appliance envelope (static binary, no GC, small attack
surface, deterministic memory footprint, true parallelism without
GIL, cross-compilation to ARM) lives in
`content-wiki-documentation/topic-service-slm.md`.

Implementation detail: cross-compilation targets include
`aarch64-unknown-linux-gnu` (ARM Toteboxes) and
`x86_64-unknown-linux-gnu`. The binding constraint for Laptop-A
hosts is the 4 GB RAM envelope — a Python + PyTorch + vLLM stack
is non-starter; a Rust binary with a quantised `mistral.rs`
runtime fits. This is the proof-of-need for the Rust path on
resource-constrained Toteboxes specifically.

---

## 10. 2030 headroom

Primitives that are still research or RFC today slot into this
architecture without rewriting it. Each is a config or subdirectory
addition, not a refactor.

| Primitive | Hook |
|---|---|
| CUDA checkpoint/restore (vLLM RFC #34303) | Ring 1: optional checkpoint bundle as bootstrap input |
| C-LoRA single-adapter (arXiv 2502.17920) | Ring 3b: registry schema migration from dual → single |
| Distributed KV across clouds (SkyPilot 0.11 + Mooncake) | Ring 2: Mooncake master on multi-cloud pool |
| FP8 KV-cache quantisation | Ring 2: config flag (`KV_CACHE_DTYPE=fp8`) |
| Sleep-time compute (async memory management) | Ring 3b: nightly LoRA retraining on Batch API |
| Encode-Prefill-Decode disaggregation (SGLang + Mooncake) | Ring 2 evolution: separate prefill and decode pools |

None of these require rewriting `service-slm`.

---

## 11. Cross-references

- `CLAUDE.md` — state header, hard constraints, project-layer rules
- `NEXT.md` — in-flight work and blocking items
- `DEVELOPMENT.md` — build, CI, licence policy, phase roadmap
- `content-wiki-documentation/topic-service-slm.md` — Rust-native
  stack rationale, licence discipline, open-source posture
  *(destination not yet committed — see workspace `NEXT.md`)*
- `content-wiki-documentation/topic-yoyo-compute.md` — three-ring
  model rationale, audit-ledger commercial argument, hyperscaler
  differentiators *(destination not yet committed)*
- Workspace `CLAUDE.md` — identity store, commit flow, cluster
  session pattern, ADR hard rules
