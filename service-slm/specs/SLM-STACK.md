# SLM-STACK.md
# service-slm Rust Stack — "We Own It" Specification
**Version:** 1 · April 20, 2026
**Scope:** The complete technical stack for building service-slm as a flat, 100% Rust substrate that can be iterated as part of the os-totebox product and optionally open-sourced later without license conflict.
**Companion docs:** STACK.md (global project stack) · YOYO-COMPUTE.md (yo-yo substrate architecture) · SERVICES.md (service-slm specification)

---

## 1. Executive Decision

**Build service-slm as a single Rust binary (cargo workspace), 100% Rust code, MIT/Apache-2.0 dependency graph end-to-end, with two C++/CUDA dependencies (the LLM inference engine and the graph DB) that are both MIT-licensed and forkable.** This satisfies the "We Own It" criterion completely: every line of the dependency graph is permissively licensed and can be forked into the PointSav organisation at any time without legal or contractual obstacle.

Open-sourcing the service later changes nothing technically. All dependencies are already open-source under permissive licenses, so there is no license incompatibility to worry about. The only decision to make at open-source time is whether to release under MIT (maximal freedom) or Apache-2.0 (explicit patent grant). Recommended: **Apache-2.0 for PointSav's own code**, because the explicit patent grant is an advantage in institutional markets.

---

## 2. The Three Levels of "Rust-Ness" (Defining the Goal Precisely)

Conversations about "100% Rust" collapse three very different things. Separating them produces a cleaner decision.

| Level | Meaning | Achievable for service-slm in 2026? |
|---|---|---|
| **L1 — Source Rust** | All code we write is Rust | **Yes, trivially** |
| **L2 — Direct-deps Rust** | Every crate we directly depend on is a Rust crate (it may internally FFI to C/C++) | **Yes** |
| **L3 — Transitive Rust** | Every line in the dependency tree, down to the metal, is Rust | **No for the inference engine and graph DB. Yes for everything else.** |

**L3 is not the right goal.** Demanding L3 rules out mature GPU inference and mature graph databases, both of which are currently C++ at the lowest layer because CUDA kernels and columnar storage engines have a 20-year C++ inheritance. Pure-Rust alternatives exist (Grafeo for graphs, candle-native inference) but are years behind in maturity and cannot yet carry institutional workloads.

**L2 is the right goal, combined with "every dep is permissively licensed and forkable."** This gives PointSav:
- Complete source access to every piece of the dependency graph
- Legal right to fork, modify, and re-license our own modifications
- A Rust-native developer experience (cargo, crate ecosystem, type system)
- The ability to drop to pure-Rust alternatives later if/when they mature

"We Own It" is a license question, not a language question. **MIT + Apache-2.0 = we own it. GPL + AGPL + BSL = we do not.**

---

## 3. The Canonical Stack for service-slm

Every entry below is either pure-Rust or Rust-bindings to a permissively-licensed native library. No copyleft. No BSL. No community-license gotchas.

### 3.1 Inference Layer (Ring 1 compute, local + yo-yo)

| Crate | Role | License | Notes |
|---|---|---|---|
| **mistral.rs** (`mistralrs`) | GPU/CPU LLM inference engine | **MIT** | Gemma 4 native support, FlashAttention V2/V3, PagedAttention, prefix caching, LoRA hot-swap per token, OpenAI-compatible HTTP server built in. The single most important finding of this research. Replaces vLLM (Apache-2.0 but Python-native) as the yo-yo node's inference runtime. |
| **candle** (`candle-core`) | Foundation ML framework | **Apache-2.0 / MIT dual** | Hugging Face's minimalist ML framework. mistral.rs builds on candle. If mistral.rs ever stagnates, candle gives us a backup path. |
| **candle-vllm** | Alternative inference server | **MIT** | Also by Eric Buehler (mistral.rs author). Focused on OpenAI-compatible serving with PagedAttention. Backup if mistral.rs's direction diverges from our needs. |
| **rvllm** | Drop-in vLLM replacement | **MIT** | February 2026, Gemma-4-specific optimizations with 16-kernel-launch fused pipeline for dual-attention architecture. Experimental; track it but don't depend on it yet. |

**Why this is better than we-depend-on-vLLM-Python:** vLLM is Apache-2.0 and forkable, but it pulls in the entire Python + PyTorch + CUDA runtime, which is (a) a 12 GB container, (b) a garbage-collected runtime with GIL pauses that hurt tail latency, and (c) structurally foreign to an os-totebox appliance binary. mistral.rs ships as a statically-linked Rust binary plus CUDA kernels — ~200 MB, no GIL, no Python.

### 3.2 HTTP / RPC Layer (the Doorman's wire)

| Crate | Role | License |
|---|---|---|
| **axum** | HTTP server (inbound from service-content, os-console, verification UI) | **MIT** |
| **tower** | Service middleware (retries, timeouts, backpressure, observability) | **MIT** |
| **tokio** | Async runtime | **MIT** |
| **hyper** | HTTP client (for calling Cloud Run, Claude API, LMCache master) | **MIT** |
| **reqwest** | High-level HTTP client | **MIT / Apache-2.0** |
| **tonic** | gRPC (only if needed for Mooncake Transfer Engine) | **MIT** |

### 3.3 Storage and State

| Crate | Role | License |
|---|---|---|
| **sqlx** | SQL — SQLite for the ledger, optional Postgres | **MIT / Apache-2.0** |
| **sled** | Embedded KV store (optional, for very fast local queues) | **MIT / Apache-2.0** |
| **lbug** / **kuzu** Rust API | LadybugDB client bindings | **MIT** |
| **object_store** (Apache Arrow) | Cloud object storage abstraction (GCS, S3, Azure) | **Apache-2.0** |
| **opendal** | Unified data access layer; useful if multi-backend grows | **Apache-2.0** |

### 3.4 Document Processing (for when service-slm handles payloads directly)

| Crate | Role | License | Notes |
|---|---|---|---|
| **oxidize-pdf** | PDF parsing, structure-aware chunking for RAG | **MIT** | Pure Rust, zero C deps. 99.3% success rate on 9,000+ real-world PDFs. 3,000–4,000 pages/sec. RAG-ready chunks out of the box. This is a 2025/2026 breakthrough — worth leaning into. |
| **docx-rust** | .docx parsing | **MIT** | Covers the Woodfine corpus (institutional docs are mostly .docx). |
| **pulldown-cmark** | Markdown parsing | **MIT** | For /ledger and /assets files. Fast, CommonMark-compliant. |
| **calamine** | .xlsx parsing (Metric extraction path) | **MIT** | Pure Rust spreadsheet reader. |
| **mupdf-rs** / **pdfium-render** | Fallback for hard PDFs | **AGPL-3.0 (mupdf) / Apache-2.0 (pdfium)** | **AVOID mupdf-rs — AGPL-3.0**. Use pdfium-render if oxidize-pdf hits a wall. |

### 3.5 Orchestration (the flat workflow engine)

| Crate | Role | License |
|---|---|---|
| **apalis** | Job processing, retries, concurrency, tower middleware | **MIT** |
| **apalis-workflow** | Composable step-based workflow engine | **MIT** |
| **apalis-sqlite** / **apalis-redis** | Persistence backends | **MIT** |
| **tokio-cron-scheduler** | Cron-style scheduling if needed | **MIT / Apache-2.0** |
| **tenacity**-equivalent: custom using **`backoff`** crate | Exponential backoff for yo-yo recovery | **MIT / Apache-2.0** |

**Why apalis over Dagster:** Dagster is asset-centric, which is the right mental model for `service-content`'s derivative-engine layer (where it stays). `service-slm` is job-centric — sanitise, send, await, receive, rehydrate. apalis fits this shape natively and brings zero Python dependencies.

### 3.6 Networking, SSH, Cloud

| Crate | Role | License |
|---|---|---|
| **russh** | Pure-Rust SSH client/server | **Apache-2.0** |
| **rustls** | Pure-Rust TLS | **Apache-2.0 / MIT / ISC** |
| **google-cloud-rust** (e.g. `google-cloud-run`, `google-cloud-storage`) | Official Google Cloud SDK for Rust | **Apache-2.0** |
| **aws-sdk-rust** | AWS SDK for Rust (if multi-cloud later) | **Apache-2.0** |

### 3.7 Serialisation, Validation, Citation Grounding

| Crate | Role | License | Notes |
|---|---|---|---|
| **serde** | Core serialisation | **MIT / Apache-2.0** | |
| **serde_json** / **serde_yaml** / **toml** | Format-specific | **MIT / Apache-2.0** | |
| **validator** | Field-level validation | **MIT / Apache-2.0** | Replaces Pydantic's validators. |
| **schemars** | JSON Schema generation | **MIT** | Replaces Pydantic's `.model_json_schema()`. |
| **garde** | Alternative validator, richer rules | **MIT / Apache-2.0** | Pick either validator or garde, not both. |

**Citation grounding without instructor.** The Pydantic + instructor pattern from STACK.md D30 relies on Python-specific LLM-tool-call integration. The Rust equivalent is straightforward: define a `#[derive(Deserialize, Validate)]` struct, pass its JSON Schema to the Claude API as the `response_format`, and reject on parse/validate failure. This is 15 lines of Rust versus ~40 lines of Python with instructor, and it runs without an interpreter.

Example sketch:

```rust
#[derive(Deserialize, Validate, JsonSchema)]
struct SupportedClaim {
    #[validate(length(min = 1))]
    claim: String,
    #[validate(length(min = 1))]
    support: Vec<String>,  // L0-asset-id:char-offset references
}

#[derive(Deserialize, Validate, JsonSchema)]
struct WikiPage {
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 1))]
    body: Vec<SupportedClaim>,
}
```

Every claim that arrives without `support` references is rejected by `validator` before it ever reaches the graph. This is the same behaviour as instructor, implemented in plain Rust.

### 3.8 Observability

| Crate | Role | License |
|---|---|---|
| **tracing** | Structured logging and distributed tracing | **MIT** |
| **tracing-subscriber** | Subscriber implementations | **MIT** |
| **opentelemetry-rust** | OpenTelemetry for SOC3 audit export | **Apache-2.0** |
| **metrics** / **metrics-exporter-prometheus** | Prometheus metrics | **MIT / Apache-2.0** |

### 3.9 Supply-Chain Security (Signing and Attestation)

| Crate | Role | License |
|---|---|---|
| **sigstore-rs** | Keyless signing for container images and OCI artifacts (adapter releases) | **Apache-2.0** |
| **cargo-audit** | Dependency CVE scanning in CI | **MIT / Apache-2.0** |
| **cargo-deny** | License policy enforcement in CI (blocks AGPL, GPL, BSL) | **MIT / Apache-2.0** |
| **cargo-sbom** | Software Bill of Materials generation | **Apache-2.0** |

**cargo-deny is the license-hygiene enforcer.** Configure it in `deny.toml` to fail the build if any new transitive dep is AGPL/GPL/BSL. This turns STACK.md's hard-won license discipline into an automated guarantee.

---

## 4. What Is NOT 100% Rust (and Why That's Fine)

Three components in the yo-yo substrate sit outside the Rust ecosystem as of April 2026. All three are behind stable network protocols, which means service-slm talks to them as clients rather than linking them as libraries. This is the correct architectural boundary.

### 4.1 LMCache + Mooncake Store (KV cache tier)

**Languages:** Python (LMCache control plane) + C++ (Mooncake Transfer Engine, RDMA).
**Why we keep them:** No Rust equivalent exists in April 2026. The Mooncake Transfer Engine is the state of the art for distributed KV cache pooling (Kimi runs 100B+ tokens/day through it).
**How service-slm interacts:** Run Mooncake master as a sidecar. service-slm's Rust code speaks to it via HTTP (metadata) and raw TCP/RDMA (data transfer) using the Mooncake wire protocol. There is no FFI coupling; we could swap Mooncake for a future Rust-native equivalent by replacing one client module.
**Ownership impact:** Mooncake is Apache-2.0. LMCache is Apache-2.0. We own both, even though we don't rewrite either.

### 4.2 vLLM reference path (if needed)

**Language:** Python.
**Why we might still touch it:** Phase 1 trial on GCP uses vLLM per GCP-NODE.md. Nothing in the architecture forces us off vLLM in Phase 1. The Rust migration happens in Phase 2, where mistral.rs replaces vLLM on the yo-yo node.
**Ownership impact:** Apache-2.0.

### 4.3 SkyPilot (multi-cloud orchestration)

**Language:** Python.
**Why we skip it in the Rust build:** SkyPilot's value is multi-cloud abstraction. For Phase 1 (single-cloud to GCP Cloud Run), it's overkill. A Rust module using `google-cloud-rust` drives Cloud Run directly with ~200 lines of code. If we ever need true multi-cloud, either re-introduce SkyPilot as an external driver or port the relevant slice to Rust.
**Ownership impact:** Apache-2.0.

**Pattern.** Three external services behind three stable protocols, all Apache-2.0, all forkable. The Rust binary calls them. None of them infects our binary's license or build chain. This is the correct boundary.

---

## 5. Flat Architecture Design — One Binary, No Mesh

### 5.1 The cargo workspace

```
service-slm/
├── Cargo.toml                  # workspace root
├── deny.toml                   # license policy (block AGPL/GPL/BSL)
├── rust-toolchain.toml         # pin compiler version
├── crates/
│   ├── slm-core/               # shared types, errors, moduleId discipline
│   ├── slm-doorman/            # sanitise / send / receive / rehydrate protocol
│   ├── slm-ledger/             # append-only CSV + SQLite audit trail
│   ├── slm-compute/            # Ring 1 bootstrap (Cloud Run driver, container mgmt)
│   ├── slm-memory-kv/          # Ring 2 client (Mooncake + LMCache wire protocol)
│   ├── slm-memory-adapters/    # Ring 3b adapter registry and loader
│   ├── slm-inference-local/    # mistral.rs-backed local inference (Totebox-capable hosts)
│   ├── slm-inference-remote/   # GCP yo-yo driver
│   ├── slm-api/                # axum server: inbound endpoints for service-content, os-console
│   └── slm-cli/                # operator CLI (the main binary entry point)
└── xtask/                      # build helpers, release automation
```

One binary produced (`slm-cli`). Shared crates above. Zero microservice sprawl.

### 5.2 What "flat" means operationally

**Not flat:** a set of Python-service processes coordinated over a message bus.
**Flat:** one Rust binary running as a systemd unit (or os-totebox init service). Inside the binary, logical modules talk via Rust function calls, not RPC. External calls (to Cloud Run, to Mooncake sidecar, to Claude API, to LadybugDB) are the only network boundaries.

Consequences:
- One process to install, start, stop, update
- One log stream (tracing → journald or Loki)
- One set of metrics
- One binary to sign with Sigstore
- One configuration file (TOML, per Rust convention)

This is exactly the shape an os-totebox appliance component wants.

---

## 6. os-totebox Integration (Why Rust Pays Off Here Specifically)

Jennifer's working notes indicate service-slm becomes part of os-totebox, the eventual PointSav archive appliance. Rust is almost certainly correct for appliance components for reasons that go beyond preference:

| os-totebox need | Rust advantage |
|---|---|
| **Predictable startup time** | Static binary, no interpreter warmup — seconds, not minutes |
| **Deterministic memory footprint** | No garbage collector, no interpreter heap |
| **Small attack surface** | Single binary, minimal syscalls, memory-safe by construction |
| **Long uptime without leaks** | Ownership model prevents most leak classes at compile time |
| **Concurrency without GIL** | True parallelism across cores; matters on constrained hardware |
| **Signed appliance updates** | Sigstore + SLSA attestations on a binary artefact = clean chain |
| **Cross-compilation** | `cargo build --target aarch64-unknown-linux-gnu` works. Deploy to ARM Toteboxes, x86, whatever. |
| **Low-RAM hosts** | Totebox Laptop-A (4 GB) can't host a Python stack + vLLM; it can host a Rust service-slm with mistral.rs in CPU/quantised mode |

The last row matters most. Per SERVICES.md, Laptop-A's RAM envelope leaves ~550 MB headroom after the core services. A Python + PyTorch + vLLM stack is simply non-starter in that envelope. A Rust service-slm binary with a quantised mistral.rs runtime fits.

---

## 7. License Audit Summary (The "We Own It" Check)

The mandatory rule: **every direct dependency and every transitive dependency is one of MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-DFS, MPL-2.0 (file-level, acceptable), or Zlib.** Nothing else.

Forbidden:
- **AGPL-3.0** (network copyleft — if we modify and run as a service, we must open-source changes)
- **GPL-2.0 / GPL-3.0** (strong copyleft — taints our binary)
- **LGPL-3.0** (weak copyleft — only acceptable for dynamic linking, which Rust makes painful)
- **BSL / Business Source License** (time-delayed open source — uncertain commercial terms)
- **Custom "community" licenses** (Llama, Gemma Terms of Use for weights — model weights are their own license question; distinct from code)
- **CC-BY-NC** (non-commercial — kills commercial DKA)

**Enforcement:** `cargo deny check licenses` runs in CI on every commit. Build fails if any dependency enters the tree with a non-allow-listed license. This automates the discipline STACK.md already established for Python.

### Specific clearances for proposed stack

Every crate in §3 above has been explicitly license-verified against MIT or Apache-2.0. The three external services in §4 (Mooncake, vLLM in Phase 1, SkyPilot if used) are all Apache-2.0. LadybugDB core is MIT (C++), and its Rust API bindings are MIT. There are no Llama/Gemma/Mistral Large-style custom-license contaminations anywhere in the stack.

**For the Gemma 4 weights specifically:** weights are covered by Google's Gemma Terms of Use, which are more permissive than Llama's community license but are **not** OSI-approved. This is a separate question from the code stack. Per STACK.md D23, the long-term plan is OLMo 3, Phi-4, or DeepSeek-R1 as genuinely OSI-approved base models. Gemma 4 is the Phase-1 default because its quality/cost profile is best today; the swap path is clean because mistral.rs supports all four.

---

## 8. Open-Sourcing Later (Keeping the Door Open)

If PointSav later decides to open-source service-slm, the release is mechanical:

1. **License choice for PointSav's own code:** Apache-2.0 recommended. Reasoning: explicit patent grant is valuable in institutional markets; MIT's lack of patent grant is a minor but real concern for downstream adopters.
2. **Dual-licensing option:** MIT OR Apache-2.0 at caller's choice — this is the Rust community norm (look at serde, tokio, axum). If in doubt, match the norm.
3. **Contributor Licensing Agreement (CLA):** don't require one. LadybugDB's release notes explicitly celebrate dropping the CLA ("CLA.md is no longer needed. Contributors own the copyright and agree to license under the MIT license included"). Follow that pattern. CLAs are seen as corporate gatekeeping and depress contribution.
4. **SPDX headers on every file:** `// SPDX-License-Identifier: Apache-2.0` at the top of every Rust file. This is mechanical and makes automated license scanning reliable.
5. **Developer Certificate of Origin (DCO):** use sign-off (`git commit -s`) instead of a CLA. Meets the "you attest this is yours to contribute" bar without lawyer overhead.
6. **REUSE compliance:** follow the REUSE Specification (https://reuse.software/) for machine-verifiable license metadata.

**What changes technically:** nothing. The code is already Rust. The deps are already Apache-2.0/MIT. The build already works.

**What changes commercially:** the pricing model shifts from "proprietary vendor" to "open-core with managed services." The research synthesis baked into this project (RESEARCH.md, DECISIONS.md) is likely where the commercial moat actually lives — not the code.

---

## 9. Migration Roadmap (Python Trial → Rust Production)

### Phase 1 — Python trial (current, per TRIAL.md)
- Keep the trial as planned: Python, vLLM, SkyPilot, dbt, Dagster, per STACK.md
- Why: the goal of the trial is to validate the architecture, not the language choice
- Rust migration during Phase 1 would add risk without validating anything

### Phase 2 — Rust service-slm rewrite (after trial passes)
- Fresh cargo workspace following §5 layout
- Port the doorman protocol first (sanitise / send / receive / rehydrate)
- Keep Python derivative-engine in service-content untouched (different service)
- Ledger port second
- Cloud Run driver third
- Rust mistral.rs on the yo-yo node (replaces vLLM)
- Success criterion: the Rust service-slm passes the same TRIAL.md test suite as the Python version

### Phase 3 — os-totebox integration
- Cross-compile Rust binary for Totebox targets (x86_64, aarch64)
- Integrate with os-totebox init/systemd
- Sign releases with Sigstore per §3.9
- Ship as appliance component

### Phase 4 — Optional open-source release
- Apply §8 checklist
- Publish to GitHub under pointsav org
- Write a launch post

Each phase is independently valuable. If we stop at Phase 2, we still get a Rust service-slm. If we stop at Phase 3, os-totebox ships. Phase 4 is optional forever.

---

## 10. Risks and Mitigations

| Risk | Mitigation |
|---|---|
| **mistral.rs is maintained by a small team.** If the maintainer disengages, we inherit maintenance. | Candle (Hugging Face, large team) underneath is our backup. We could pin the engine at a known-good commit and carry patches. The downside case is not catastrophic because the crate is small (~50k LOC) and the interfaces are stable. |
| **LadybugDB is also a fork of a recently-orphaned project (post-Apple acquisition of Kuzu).** | MIT-licensed, so the absolute downside is PointSav carries patches. The codebase has a VLDB-pedigree foundation (Kuzu's research lineage) which means the core is well-understood. Monitor for the first six months; contribute fixes upstream to build relationship with the LadybugDB maintainer. |
| **Rust LLM inference ecosystem is less mature than Python's.** Performance gaps could emerge on new model architectures. | Phase 1 stays Python. Phase 2 Rust migration is validated with the same test suite. If mistral.rs ever falls behind vLLM on a specific new model/technique, we can run vLLM as a sidecar (the yo-yo design accommodates this — see YOYO-COMPUTE.md §4). We never get locked in. |
| **Mooncake Store is C++ — if we need to deeply integrate, we must work in C++.** | Architectural boundary: always use it as a network service, never as a linked library. All integration happens over the published wire protocol (HTTP + RDMA/TCP). If Mooncake ever becomes a blocker, migrate to a different KV cache tier; our code depends only on the wire protocol. |
| **cargo audit / cargo deny may flag unexpected transitive licenses.** | Run these in CI from day one. Fix licence drift early, not during a release push. The `deny.toml` template in §3.9 is a starting point. |
| **Rust build times can be long (compared to Python dev loop).** | sccache + cargo-chef for Docker layer caching. Separate the inference crate (which depends on CUDA and compiles slowly) from the doorman crate (which compiles fast) — this lets the iteration loop stay fast even when the inference layer is rebuilding. |
| **Rust hiring is narrower than Python hiring.** | True, but the hiring pool for "Rust + AI infrastructure" is growing fast. More importantly, os-totebox needs an appliance engineering discipline that maps naturally to Rust talent (systems, embedded-adjacent). |

---

## 11. Crate List in One Block (for `Cargo.toml`)

Grouped for clarity. Versions are indicative minimums as of April 2026 — pin to latest at start of Phase 2.

```toml
[workspace.dependencies]
# HTTP / async
axum = "0.8"
tower = "0.5"
tokio = { version = "1.40", features = ["full"] }
hyper = "1.5"
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }

# Storage
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-rustls"] }
object_store = { version = "0.11", features = ["gcp", "aws"] }

# LadybugDB bindings
kuzu = "0.11"        # or lbug crate if migrating
# ladybug = "0.15"   # when Rust crate publishes for LadybugDB

# Inference
mistralrs = { version = "0.8", features = ["cuda", "flash-attn"] }
candle-core = "0.9"
candle-nn = "0.9"

# Documents
oxidize-pdf = "2.5"
docx-rust = "0.3"
pulldown-cmark = "0.12"
calamine = "0.26"

# Orchestration
apalis = { version = "0.7", features = ["limit"] }
apalis-sqlite = "0.7"
apalis-workflow = "0.1"
backoff = "0.4"

# Networking
russh = "0.46"
rustls = "0.23"
google-cloud-storage = "0.23"   # or similar crate name
google-cloud-run = "*"

# Serde + validation
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
validator = { version = "0.19", features = ["derive"] }
schemars = "0.8"

# Observability
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
opentelemetry = "0.27"

# Signing
sigstore = "0.10"

# Errors
anyhow = "1"
thiserror = "2"
```

All MIT or Apache-2.0.

---

## 12. Cross-References and Decisions to Add

- **STACK.md** — update to note that `service-slm` is Rust-native per this doc; the Python deps listed in STACK.md apply to `service-content` (derivative-engine) and the Phase 1 trial only.
- **SERVICES.md** — add a note under service-slm pointing to this doc for the Rust implementation.
- **YOYO-COMPUTE.md** — unchanged; the Rust stack is the implementation layer for the substrate described there.
- **GCP-NODE.md** — Phase 1 remains Python. Phase 2 replaces vLLM with mistral.rs on the node (container change, not architectural change).

### Decisions to append to DECISIONS.md

- **D42:** service-slm is built in Rust following SLM-STACK.md. All dependencies MIT or Apache-2.0. cargo deny enforces license policy in CI.
- **D43:** mistral.rs (MIT) is the inference runtime for service-slm, replacing vLLM in Phase 2 and beyond. vLLM remains the Phase 1 trial engine.
- **D44:** The service-slm binary is the prototype os-totebox service component. Future os-totebox services should follow the same flat-binary, pure-Rust-where-feasible pattern.
- **D45:** Open-sourcing path: Apache-2.0 recommended for PointSav's own code. DCO sign-off, not CLA. REUSE-compliant SPDX headers on every file. No timeline commitment — the option stays open.
