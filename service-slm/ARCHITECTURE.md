---
schema: foundry-doc-v1
document_version: 1.1.0
research_provenance: tacit
research_inline: false
cites: []
---

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

**Current state.** The Doorman is in production service on the
workspace VM (`local-doorman.service` systemd unit). Tier A
(local llama-server, `local-slm.service`) is live and verified (B5,
2026-04-26). Tier B (Yo-Yo GCE burst) is code-complete with
multi-Yo-Yo HashMap routing (§9); the
`infrastructure/slm-yoyo/tofu/` OpenTofu module is authored but
`tofu apply` is gated on the D4 image-build pipeline (a Master-tier
action). Tier C (external API) is wired with mock-only tests per the
operator cost guardrail. The mesh discovery framework (§9.4) is
scaffolded; `route_async()` is a Phase 1 stub. Apprenticeship
substrate (§11) is code-complete but disabled on the workspace VM
(`SLM_APPRENTICESHIP_ENABLED` unset; re-enable is a pending
Master-tier action).

---

## 1. Purpose and role

`service-slm` is the single secure boundary between the isolated
Totebox Archive and any external Large Language Model. It implements
the **Doorman Protocol**: gate compute, not generate content.

Four operations:

1. **Sanitise outbound.** Strip direct identifiers and structured
   data from the payload; emit prose the external model can process.
2. **Route compute.** Local when the host has resources; remote
   (the Yo-Yo substrate on GCP) when it does not. The caller does
   not see the difference.
3. **Receive deltas.** Structured responses — graph deltas, wiki
   drafts, extracted entities — return from compute.
4. **Rehydrate inbound.** Verify against the ledger, reattach the
   identifiers stripped at step 1, commit to the authoritative
   store.

Nothing inside this service generates text. Generation happens
externally (local llama-server on Tier A; vLLM on the Yo-Yo node on
Tier B; Anthropic / Gemini / OpenAI on Tier C). If code in this
project starts producing text directly, it has exceeded its remit —
see `CLAUDE.md` Hard constraints.

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
| 1 | Bootstrap | systemd-unit `ReadWritePaths` + GCS-cached weights + Secret Manager | ~5–15 s | Yes (as artefacts) | `service-slm/compute/` |
| 2 | Working (KV cache) | LMCache + Mooncake Store → object storage | Near-zero on cache hit | Yes (pooled) | `service-slm/memory/kv/` |
| 3a | Long-term — graph | LadybugDB in `service-content` | None (query-time) | Yes (authoritative) | `service-content` (read-only from here) |
| 3b | Long-term — skill | LoRA adapter stack, GCS-archived (signed, SLSA-attested) | One-time per project | Yes (portable) | `service-slm/memory/adapters/` |

Everything else is ephemeral and intentionally discarded.

### Ring 1 — Bootstrap

Four pre-staged artefacts in cheap cold storage, pulled on boot:

1. Pre-built native binary in the `pointsav-public` GCE image
   family per `infrastructure/slm-yoyo/tofu/` precedent (Tier A
   `llama-server` ELF + GGUF weights pulled at boot; Tier B
   `vLLM` server). No container runtime —
   `~/Foundry/conventions/zero-container-runtime.md` is structural
   doctrine.
2. Pre-downloaded model weights in GCS (e.g.
   `gs://dka-checkpoints/models/gemma-4-26b-a4b/`), mounted via
   Cloud Storage FUSE or `rsync`'d on boot.
3. GCE GPU instance with `idle_shutdown_minutes=N` per
   `infrastructure/slm-yoyo/tofu/`. The OpenTofu module manages
   provision-on-demand and tears the instance down after N
   minutes of inactivity. CUDA drivers ship in the
   `pointsav-public` GCE image family per the convention's
   trade-off section (`~/Foundry/conventions/zero-container-runtime.md`,
   "What is used instead" table).
4. Warm-VM mode opt-in: hold the GCE instance running between
   requests within a configurable window (default 15 min) when
   latency-critical. Operator extends `idle_shutdown_minutes`
   per workload; Doorman may also start the VM ahead of an
   anticipated burst (predictive warm-up). Per the convention's
   "Cold-start: the only honest concern" section.

Bill-per-second for request processing; zero idle cost once the
`idle_shutdown_minutes=N` timer fires and the instance stops.

### Ring 2 — Working (KV cache)

**LMCache** hashes blocks of tokens and fetches matching KV cache
blocks from a tiered store (GPU → CPU DRAM → remote). **Mooncake
Store** is the remote tier — a distributed KV pool that survives
inference-instance teardown. The Woodfine workload is entirely
repeated-prefix (every document processed shares ~2,000 tokens of
Chart-of-Accounts spine and prompt scaffolding), so cache hit rates
compound rapidly.

Ring 2 is not active in the current implementation. It is planned
for the phase following the first commercial deployment.

### Ring 3a — Long-term graph (read-only from here)

The LadybugDB graph in `service-content` is the long-term semantic
memory. `service-slm` reads from it at context-assembly time via the
`GraphContextClient` in `slm-doorman/src/graph.rs` — it queries
`GET {SERVICE_CONTENT_ENDPOINT}/v1/graph/context` before every inference
call and prepends a `[ENTITY CONTEXT]` system message. Non-fatal if
`service-content` is unavailable (WARN + proceed).

`service-slm` never writes to the graph directly — writes flow through
`service-content`'s own ingest path after the sanitise / compute /
rehydrate cycle completes.

**Phase 2 GraphStore trait discipline (Master-ratified 2026-04-30):**
The graph dependency MUST be wrapped behind a `GraphStore` trait
injected at startup. Business logic calls the trait; the concrete
implementation is LadybugDB today, `moonshot-database` when it
matures. No direct LadybugDB API calls in business logic. This is
the same boundary discipline as the Doorman pattern — one interface
swap should be sufficient to migrate the graph substrate without
touching the calling code.

**Planned Phase 4 — service-content Tier C drafting pipeline:**
`service-content` will query LadybugDB for a ~2K-token entity context
payload, assemble a structured prose prompt, and route it to Claude 3.5
Sonnet via the Doorman's `POST /v1/audit/proxy` (Tier C,
`"initial-graph-build"` purpose). The grammar constraint injection path
(service-content JSON Schema → Doorman `grammar` field → Yo-Yo #2 for
ontological strictness) is designed but not yet implemented. This is
Leapfrog 2030 Phase 4; see §9.2 Tier B and `service-slm/docs/topic-leapfrog-architecture.md`.

### Ring 3b — Long-term skill (LoRA adapter stack)

Small, versioned, frozen-weight modules that sit on top of the base
model and encode task-specific behaviour (CoA classification,
archetype detection, entity resolution, wiki synthesis). Each
adapter is trained once, versioned, stored as a GCS object (Sigstore-signed
via the sigstore crate, SLSA-attested), and loaded at inference boot.
`moduleId` (§4) selects which adapter stack activates for a given
call.

Ring 3b is planned for the phase following the first commercial
deployment beyond Woodfine.

---

## 3. Flat architecture — one binary

`service-slm` is one Rust binary running as a systemd unit (or
os-totebox init service). Inside the binary, logical modules talk
via Rust function calls, not RPC. External calls (GCE Yo-Yo instances,
Claude API, LadybugDB in `service-content`) are the only network
boundaries.

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
| 1 — Bootstrap | Selects which `systemd` unit `ExecStart` per `moduleId` (usually same across projects) |
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
| `vLLM` (non-Rust, Tier B) | GPU/CPU LLM inference engine; OpenAI-compatible HTTP server | Apache-2.0 |
| `candle-core` | Foundation ML framework (potential future Tier A path) | Apache-2.0 / MIT |

Tier A uses llama-server (llama.cpp) deployed as a native binary via
`infrastructure/local-slm/bootstrap.sh`. Tier B uses vLLM (≥0.12)
deployed on a GCE GPU instance via `infrastructure/slm-yoyo/tofu/`.
The Doorman communicates with both over HTTP; neither is a Rust
dependency of `service-slm`.

### 5.2 HTTP / RPC layer (the Doorman's wire)

| Crate | Role | Licence |
|---|---|---|
| `axum` | HTTP server (inbound from `service-content`, `os-console`, verification UI) | MIT |
| `tower` | Service middleware (retries, timeouts, backpressure) | MIT |
| `tokio` | Async runtime; `tokio::sync::Semaphore` for per-tenant concurrency caps on audit endpoints | MIT |
| `hyper` | HTTP client (Yo-Yo GCE endpoints, Mooncake master) | MIT |
| `reqwest` | High-level HTTP client (tier/local, tier/yoyo, tier/external, audit_proxy) | MIT / Apache-2.0 |

### 5.3 Grammar substrate (PS.3)

| Crate | Role | Licence |
|---|---|---|
| `llguidance` 1.7 | Lark grammar pre-validation at the Doorman boundary; `ParserFactory::create_parser` for compile-time error detection | MIT |

The `llguidance` crate is a decode-time library for LLM samplers.
The Doorman uses only its Lark compilation surface (`TopLevelGrammar::from_lark`)
to pre-validate caller-submitted Lark grammars before relaying to
Tier B. No sampler loop integration exists in the Doorman; vLLM applies
`llguidance` natively on its own sampler side.

### 5.4 Storage and state

| Crate | Role | Licence |
|---|---|---|
| `sqlx` | SQL — SQLite for the ledger, optional Postgres | MIT / Apache-2.0 |
| `sled` | Embedded KV store (optional, fast local queues) | MIT / Apache-2.0 |
| `ladybugdb` (or successor) | Graph DB client — LadybugDB Phase 2; `moonshot-database` long-term | MIT |
| `object_store` | Cloud object storage abstraction (GCS, S3, Azure) | Apache-2.0 |

### 5.5 Document processing

| Crate | Role | Licence |
|---|---|---|
| `oxidize-pdf` | PDF parsing, structure-aware chunking | MIT |
| `docx-rust` | `.docx` parsing | MIT |
| `pulldown-cmark` | Markdown parsing | MIT |
| `calamine` | `.xlsx` parsing | MIT |

`mupdf-rs` is not permitted — AGPL-3.0. `pdfium-render`
(Apache-2.0) is the fallback if `oxidize-pdf` hits a wall.

### 5.6 Orchestration

| Crate | Role | Licence |
|---|---|---|
| `apalis` | Job processing, retries, concurrency | MIT |
| `apalis-workflow` | Composable step-based workflow engine | MIT |
| `apalis-sqlite` | Persistence backend | MIT |
| `backoff` | Exponential backoff for yo-yo recovery | MIT / Apache-2.0 |

### 5.7 Networking, SSH, cloud

| Crate | Role | Licence |
|---|---|---|
| `russh` | Pure-Rust SSH | Apache-2.0 |
| `rustls` | Pure-Rust TLS | Apache-2.0 / MIT / ISC |
| `google-cloud-compute` | GCE instance lifecycle for Tier B management | Apache-2.0 |

### 5.8 Serialisation, validation, citation grounding

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

### 5.9 Observability

| Crate | Role | Licence |
|---|---|---|
| `tracing` | Structured logging and distributed tracing | MIT |
| `tracing-subscriber` | Subscriber implementations | MIT |
| `opentelemetry` | OpenTelemetry for SOC3 audit export | Apache-2.0 |
| `metrics-exporter-prometheus` | Prometheus metrics | MIT / Apache-2.0 |

### 5.10 Supply-chain security

| Crate | Role | Licence |
|---|---|---|
| `sigstore` | Keyless signing for native binaries and unit files; SSH commit signing per workspace `CLAUDE.md` §3 is the primary commit-time authority, with `sigstore` reserved for release-artefact signing (adapter releases) | Apache-2.0 |

See `DEVELOPMENT.md` for `cargo-audit`, `cargo-deny`, and
`cargo-sbom` invocation in CI.

### 5.11 Not-Rust components, behind network protocols

Two components sit outside the Rust ecosystem. Both are behind
stable network protocols; `service-slm` talks to them as a client.

| Component | Language | Why kept | Integration |
|---|---|---|---|
| LMCache + Mooncake Store | Python + C++ (RDMA) | No Rust equivalent in 2026 | HTTP metadata + raw TCP/RDMA data-transfer |
| vLLM ≥0.12 | Python | Multi-LoRA serving + structured-output grammar support | HTTP (OpenAI-compatible) — Tier B compute |

Both are Apache-2.0 and forkable. The Rust binary calls them.
Neither infects this binary's licence or build chain.

---

## 6. Cargo workspace

Standalone workspace resolved 2026-04-25. The decision was the
lowest-blast-radius path: it touches no code outside `service-slm/`
and leaves the monorepo unification cleanup item (2026-04-18 audit,
8 of ~70+ crates declared) to be settled separately. The existing
`cognitive-forge/` subcrate remains an `exclude` member until its
rename is paired with `tool-cognitive-forge`.

```
service-slm/
├── Cargo.toml                  workspace manifest (resolver = "2")
├── deny.toml                   licence policy (block AGPL/GPL/BSL)
├── rust-toolchain.toml         pin compiler version (stable channel)
├── crates/
│   ├── slm-core/               shared types, errors, moduleId discipline
│   ├── slm-doorman/            sanitise / route / receive / rehydrate (lib)
│   └── slm-doorman-server/     axum HTTP server entry point (bin + lib)
└── cognitive-forge/            legacy subcrate — workspace `exclude`
```

Three workspace members. The broader `slm-ledger`, `slm-compute`,
`slm-memory-kv`, `slm-memory-adapters`, `slm-inference-local`,
`slm-inference-remote`, `slm-api`, `slm-cli` crates described in
prior versions of this document are planned but not yet scaffolded.
The current implementation delivers the Doorman protocol end-to-end
through the three members above.

---

## 7. Crate responsibilities

### `slm-core`

Shared types and small value-objects. No async runtime, no HTTP
client, no I/O. All crates that route, log, or serve HTTP depend on
this crate; nothing in this crate depends on them.

Public types:

| Type | Purpose |
|---|---|
| `ModuleId` | Multi-tenant namespace tag; validated `[a-z0-9-]` 1–64 chars |
| `RequestId` | UUIDv7 request correlator |
| `Tier` | `Local \| Yoyo \| External` — three compute tiers |
| `Complexity` | `Low \| Medium \| High` — advisory routing hint |
| `GrammarConstraint` | Adjacent-tagged enum: `Lark(String) \| Gbnf(String) \| JsonSchema(Value)` (PS.3 step 1) |
| `ComputeRequest` | Request crossing the Doorman boundary; carries `grammar: Option<GrammarConstraint>`, `yoyo_label: Option<String>` (selects named Yo-Yo node; §9.2) |
| `ComputeResponse` | Response returned through the Doorman; carries `tier_used`, `cost_usd` |
| `ChatMessage` | OpenAI-compatible `{role, content}` message |
| `AuditProxyRequest` / `AuditProxyResponse` / `AuditUsage` | Wire shapes for `POST /v1/audit/proxy` (PS.4 step 1) |
| `AuditCaptureRequest` / `AuditCaptureResponse` | Wire shapes for `POST /v1/audit/capture` (PS.4 step 4) |
| Apprenticeship types | `ApprenticeshipBrief`, `ApprenticeshipAttempt`, `ApprenticeshipVerdict` (§11) |
| Mesh types | `NodeId`, `NodeDescriptor`, `EnvironmentMetadata`, `EnergySource` — compute node identity and environmental metadata (§9.4) |

### `slm-doorman` (lib)

Three-tier router and all business logic. Source modules:

| Module | Responsibility |
|---|---|
| `router.rs` | `Doorman::route()` — selects tier, calls tier client, writes ledger entry; `Doorman::route_async()` — mesh-aware dispatch stub (§9.4); `Orchestrator` wraps `Box<dyn MeshRegistry>` |
| `mesh.rs` | `MeshRegistry` trait + `DiscoveryProvider` trait + `DynamicRegistry` struct (background tokio polling; `Arc<RwLock<Vec<NodeDescriptor>>>`) |
| `tier/local.rs` | Tier A HTTP client — llama-server; serialises GBNF/JsonSchema grammar; rejects Lark |
| `tier/yoyo.rs` | Tier B HTTP client — vLLM ≥0.12; serialises all three grammar variants via `extra_body.structured_outputs`; bearer-token auth with retry |
| `tier/external.rs` | Tier C HTTP client — Anthropic / Gemini / OpenAI; rejects all grammar variants; compile-time `ExternalAllowlist` label gate |
| `ledger.rs` | Append-only JSONL audit ledger; four entry types with `entry_type` discriminator |
| `audit_proxy.rs` | `AuditProxyClient` — relay surface for `POST /v1/audit/proxy`; `AuditProxyPurposeAllowlist` and `FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST` |
| `grammar_validation.rs` | `LarkValidator` — `llguidance` pre-validation of Lark grammars; shared `Arc<ParserFactory>` |
| `apprenticeship.rs` | AS-2 dispatcher — `ApprenticeshipDispatcher::dispatch_brief()` + shadow path |
| `verdict.rs` | AS-3 verdict pipeline — signature verify + DPO pair write + promotion ledger |
| `promotion_ledger.rs` | Append-only `ledger.md` + stats sidecar + `stages.json`; `flock(2)` serialisation |
| `brief_cache.rs` | In-process FIFO `(brief_id, attempt_id)` → `(brief, attempt)` cache |
| `redact.rs` | Sanitise-outbound filter; port of `bin/capture-edit.py` REDACTIONS |
| `citations.rs` | Best-effort `citations.yaml` resolver |
| `error.rs` | `DoormanError` enum (all error variants; maps to HTTP status codes in server) |

### `slm-doorman-server` (bin + lib)

Axum HTTP server. The `[lib]` target (`src/lib.rs`) exposes
`pub mod http` (containing `AppState` and `router`) and
`pub mod test_helpers` (factory helpers for tests). `main.rs`
imports from `slm_doorman_server::http`. The split is required so
integration tests under `tests/` can import from the server crate,
which is otherwise a binary target.

---

## 8. HTTP endpoints

All endpoints are served by the `slm-doorman-server` binary.

| Endpoint | Method | Purpose | Notes |
|---|---|---|---|
| `/healthz` | GET | Liveness — always 200 | No auth; safe for load-balancer probes |
| `/readyz` | GET | Readiness — returns tier flags | Body: `{ready, has_local, has_yoyo, has_external}` |
| `/v1/contract` | GET | Doorman version + contract versions + tier summary | Body: `ContractInfo` |
| `/v1/chat/completions` | POST | Route an inference request through the Doorman | OpenAI-compatible body + optional `X-Foundry-*` headers; respects `grammar` field on `ComputeRequest` |
| `/v1/brief` | POST | Submit an `ApprenticeshipBrief` | Gated: `SLM_APPRENTICESHIP_ENABLED=true`; 404 otherwise |
| `/v1/verdict` | POST | Submit a signed verdict | Gated: same |
| `/v1/shadow` | POST | Submit a brief + actual diff for shadow capture | Gated: same; 200 empty body on success |
| `/v1/audit/proxy` | POST | Audited external provider call (PS.4) | Body: `AuditProxyRequest`; response: `AuditProxyResponse`; two-entry ledger design |
| `/v1/audit/capture` | POST | Cross-cluster local-work audit event push (PS.4) | Body: `AuditCaptureRequest`; response: `AuditCaptureResponse`; single-entry ledger |

### X-Foundry-* request headers for `/v1/chat/completions`

| Header | Type | Required | Notes |
|---|---|---|---|
| `X-Foundry-Module-ID` | string | no | Validated as `ModuleId`; defaults to `"foundry"` |
| `X-Foundry-Request-ID` | string | no | UUIDv7; generated server-side if absent |
| `X-Foundry-Complexity` | string | no | `"low" \| "medium" \| "high"`; advisory routing hint |
| `X-Foundry-Tier-C-Label` | string | no | Required for Tier C dispatch; must match the compile-time `ExternalAllowlist` |

Production callers SHOULD supply all four headers per CONTRACT.md.
Ad-hoc `curl` probes work without headers — server generates safe
defaults.

---

## 9. Three-tier routing and grammar handling

The Doorman selects a compute tier per `ComputeRequest` and
translates the optional `GrammarConstraint` into the tier's native
wire format.

### Tier A — local llama-server

- HTTP endpoint: `SLM_LOCAL_ENDPOINT` (default `http://127.0.0.1:8080`)
- Grammar handling:
  - `Gbnf(s)` → `{ "grammar": s }` in the request body
  - `JsonSchema(v)` → `{ "json_schema": v }` in the request body
  - `Lark(s)` → **rejected** immediately with `DoormanError::TierAGrammarUnsupported` (HTTP 400). llama-server does not accept Lark grammars on its wire protocol.

### Tier B — Yo-Yo / vLLM ≥0.12

**Multi-Yo-Yo routing (landed 2026-05-04):**  
`DoormanConfig.yoyo` is a `HashMap<String, YoYoTierClient>` — one entry per named
Yo-Yo node. `slm-doorman-server/src/main.rs` inserts three entries on startup:

| Key | Env vars | Intended role |
|---|---|---|
| `"default"` | `SLM_YOYO_ENDPOINT`, `SLM_YOYO_BEARER`, `SLM_YOYO_MODEL`, `SLM_YOYO_HOURLY_USD` | Fallback / general-purpose burst |
| `"trainer"` | `SLM_YOYO_TRAINER_ENDPOINT`, `SLM_YOYO_TRAINER_BEARER`, `SLM_YOYO_TRAINER_MODEL`, `SLM_YOYO_TRAINER_HOURLY_USD` | Yo-Yo #1 — training workloads (L4, OLMo 3 32B-Think, night-shift) |
| `"graph"` | `SLM_YOYO_GRAPH_ENDPOINT`, `SLM_YOYO_GRAPH_BEARER`, `SLM_YOYO_GRAPH_MODEL`, `SLM_YOYO_GRAPH_HOURLY_USD` | Yo-Yo #2 — graph extraction (H100, Llama 3.3 70B, manual batch) |

Dispatch logic: `ComputeRequest.yoyo_label: Option<String>` selects the target entry
by key. No label → first entry in map insertion order (deterministic at current
scale; callers should always set a label in production). Unrecognised label →
`DoormanError::TierUnavailable`.

- Grammar handling (all three variants accepted; vLLM 0.12+ API — `guided_*` fields removed in 0.12):
  - `Lark(s)` → **pre-validated** via `LarkValidator` (llguidance); `MalformedLarkGrammar` (HTTP 400) if compilation fails with parse-error location in the response body. On valid parse → `extra_body.structured_outputs.grammar = s`
  - `Gbnf(s)` → `extra_body.structured_outputs.grammar = s`
  - `JsonSchema(v)` → `extra_body.structured_outputs.json = v`
- Additional `X-Foundry-*` headers forwarded per CONTRACT.md: `Request-ID`, `Module-ID`, `Contract-Version`, `Complexity`
- Retry policy: 503 + `Retry-After` → retry once; 401/403 → refresh bearer token, retry once; 410 → `DoormanError::ContractMajorMismatch` (loud fail, no retry)
- Deployment gated on D4 image-build pipeline (Master-tier action)

### Tier C — external API

- Providers: Anthropic, Gemini, OpenAI; configured via `SLM_TIER_C_{PROVIDER}_ENDPOINT` + `SLM_TIER_C_{PROVIDER}_API_KEY`
- Grammar handling: **all three variants rejected** with `DoormanError::TierCGrammarUnsupported` (HTTP 400). External vendors offer no arbitrary grammar support.
- Dispatch requires `tier_c_label` matching a compile-time `ExternalAllowlist` entry; otherwise `DoormanError::ExternalNotAllowlisted` (HTTP 403)
- Implemented with wiremock-only tests per operator cost guardrail; no live provider calls

### 9.4 Mesh Discovery (Phase 1 scaffold — stub dispatch)

The mesh discovery framework provides a trait-based abstraction for dynamic
compute node registration and selection. It is scaffolded in the current codebase
but the dispatch path is a stub.

**Types (in `slm-core/src/mesh.rs`):**

| Type | Purpose |
|---|---|
| `NodeId` | Opaque string identifier for a compute node |
| `NodeDescriptor` | `{ id, endpoint, capabilities: Vec<String>, environment }` — full node description |
| `EnvironmentMetadata` | `{ carbon_intensity: u32, energy_source: EnergySource }` — for future energy-aware routing |
| `EnergySource` | `Grid \| Solar \| Wind \| Geothermal` |

**Traits (in `slm-doorman/src/mesh.rs`):**

| Trait / Type | Responsibility |
|---|---|
| `DiscoveryProvider: Send + Sync` | `async fn poll_nodes() -> Result<Vec<NodeDescriptor>>` — pluggable back end for node enumeration |
| `MeshRegistry: Send + Sync` | `async fn discover_nodes()` + `async fn select_optimal(req) -> Option<NodeDescriptor>` |
| `DynamicRegistry` | Concrete `MeshRegistry` impl; spawns a background tokio task that polls a `DiscoveryProvider` at a configurable interval and stores results in `Arc<RwLock<Vec<NodeDescriptor>>>`. `select_optimal` returns the first node (Phase 1 naive implementation). |

**`route_async()` on `Doorman` (Phase 1 stub):**

```rust
pub async fn route_async(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
    if let Some(ref orch) = self.orchestrator {
        if let Some(node) = orch.registry.select_optimal(req).await {
            info!("selected node: {}", node.id.0);
            // Phase 1: logs selected node; does NOT use node.endpoint
        }
    }
    self.route(req).await   // falls through to existing tier dispatch
}
```

`Doorman.orchestrator: Option<Orchestrator>` is `None` in the current server
configuration. No `DiscoveryProvider` implementation exists yet (no HTTP
provider, no static-config provider). The trait boundary is in place so a
concrete provider can be wired in without touching the router.

**What is missing before mesh dispatch is real:**
1. A concrete `DiscoveryProvider` implementation (HTTP endpoint poller or static config)
2. `route_async()` must use `node.endpoint` to select the target Yo-Yo client by endpoint URL, not fall through to `route()`
3. Callers (service-content, other services) must set `yoyo_label` OR the orchestrator must override the label from the selected node

---

## 10. Audit substrate

Wire contract: `service-slm/docs/audit-endpoints-contract.md` v0.2.0.
That document is the canonical reference for request/response shapes,
validation order, HTTP status codes, purpose allowlist, event_type
vocabulary, and the two-entry ledger design. This section provides
the architectural overview; callers and implementers should read the
contract doc.

### Audit ledger

Per-day JSONL files at `<ledger_base_dir>/<YYYY-MM-DD>.jsonl`.
Default base: `$HOME/.service-slm/audit/`; `SLM_AUDIT_DIR` env var
overrides (wired in `slm-doorman-server/src/main.rs`; the
`infrastructure/local-doorman/` systemd unit sets a stable path).

Four entry types coexist in the same JSONL stream, distinguished
by the `entry_type` string discriminator field (added in contract
v0.2.0 / iter-15):

| Entry type | `entry_type` value | Written by |
|---|---|---|
| `AuditEntry` | `"chat-completion"` | `POST /v1/chat/completions` |
| `AuditProxyStubEntry` | `"audit-proxy-stub"` | `POST /v1/audit/proxy` (before relay) |
| `AuditProxyEntry` | `"audit-proxy"` | `POST /v1/audit/proxy` (after relay) |
| `AuditCaptureEntry` | `"audit-capture"` | `POST /v1/audit/capture` |

The `entry_type` field is set by `AuditLedger::append_*` at write
time regardless of what the caller placed in the struct. This is
the canonical discrimination path; a field-presence fallback covers
entries predating contract v0.2.0 (see contract doc §3.2).

### `POST /v1/audit/proxy`

Cross-cluster callers (e.g., project-language editorial gateway)
hold no provider API keys. They submit an `AuditProxyRequest` to
the Doorman; the Doorman authenticates with the named provider,
writes two ledger entries, and returns the provider's response.

Two-entry ledger design: (1) stub entry written immediately after
validation, before the upstream call — guarantees a paper trail
even if the process crashes mid-relay; (2) final `AuditProxyEntry`
written after the upstream call returns with token counts, cost,
latency, and status.

Purpose allowlist (`FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST`): four
documented purposes — `"editorial-refinement"`,
`"citation-grounding"`, `"entity-disambiguation"`,
`"initial-graph-build"`. Unenumerated purposes are rejected 403.
Compile-time allowlist; extensions require code review.

Hardening (iter-16): 64 KiB raw-body cap (checked before JSON
deserialisation); per-tenant `tokio::sync::Semaphore` concurrency
cap (default 4, shared with `/v1/audit/capture`) via
`SLM_AUDIT_TENANT_CONCURRENCY_CAP`.

### `POST /v1/audit/capture`

Inverse direction: cross-cluster callers push audit events for
work they performed locally. Five accepted `event_type` values:
`"prose-edit"`, `"design-edit"`, `"graph-mutation"`,
`"anchor-event"`, `"verdict-issued"`. Single-entry ledger design.
16 KiB payload cap. Same per-tenant concurrency cap as
`/v1/audit/proxy`.

---

## 11. Apprenticeship Substrate (Doctrine claim #32)

The Doorman flips polarity for code-shaped work. Service-slm becomes
the **first responder**; Master / Root / Task Claude becomes the
**senior reviewer**. The disagreement between them — captured as
signed, append-only training tuples — is the highest-quality
continued-pretraining signal Foundry produces. Specification at
`~/Foundry/conventions/apprenticeship-substrate.md`. Convention
ratified 2026-04-26 in Doctrine v0.0.7.

Three endpoints gated behind `SLM_APPRENTICESHIP_ENABLED=true`;
404 when unset so existing deployments are unchanged.

**Current on-VM state:** disabled. The workspace `local-doorman.service` unit runs
with `SLM_APPRENTICESHIP_ENABLED` unset. The B7 re-deploy (2026-04-29, Master
v0.1.68) set the flag; a subsequent service restart left the flag unset again.
Re-enabling requires an operator-presence pass: update the
`infrastructure/local-doorman/service.d/env-file.conf` drop-in and restart the
unit. The corpus capture path (brief queue §11 configuration below) is fully
implemented and will resume accumulating tuples immediately once the flag is set.

| Endpoint | Purpose | Returns |
|---|---|---|
| `POST /v1/brief` | Senior posts an `ApprenticeshipBrief`; Doorman dispatches the apprentice; returns the parsed `ApprenticeshipAttempt` | `ApprenticeshipAttempt` (with `escalate=true` when self-confidence < 0.5 per design-pass Q2) |
| `POST /v1/verdict` | Senior posts the signed verdict body + base64 SSH signature; Doorman verifies via `ssh-keygen -Y verify -n apprenticeship-verdict-v1`, writes the apprenticeship corpus tuple, appends the ledger event under `flock(2)`, and on threshold cross appends a `promotion` event | `VerdictDispatchOutcome` |
| `POST /v1/shadow` | Senior posts a brief + the actual diff committed the existing way; Doorman dispatches the apprentice internally and captures the (brief, attempt, actual_diff) tuple as `stage_at_capture: shadow`. Idempotent on `brief_id` | `200 OK` empty body |

### Wire types (`slm-core::apprenticeship`)

- `ApprenticeshipBrief { brief_id, created, senior_role, senior_identity, task_type, scope { cluster, files }, acceptance_test, doctrine_citations, shadow, body }`
- `ApprenticeshipAttempt { brief_id, attempt_id, created, model, adapter_composition, self_confidence, escalate, inference_ms, tier, cost_usd, reasoning, diff }`
- `ApprenticeshipVerdict { brief_id, attempt_id, verdict, created, senior_identity, final_diff_sha, notes, body, signature }` — `signature` is base64-encoded for JSON transport (design-pass Q5)

### Verdict signing

`ssh-keygen -Y sign -n apprenticeship-verdict-v1` against the senior's
key under `~/Foundry/identity/<id>/id_<id>`. Verification on the
Doorman side shells out to `ssh-keygen -Y verify -n
apprenticeship-verdict-v1` against `${FOUNDRY_ROOT}/identity/allowed_signers`.
Per design-pass Q1 the shell-out is the v0.1 path; native Rust
verification is a v0.5+ follow-up. The namespace tag prevents
re-using a commit-signing signature as a verdict.

### Promotion ledger

`${FOUNDRY_ROOT}/data/apprenticeship/ledger.md` (markdown event log,
one row per verdict-batch + one row per promotion event), plus a
sidecar `.stats.jsonl` (one row per verdict for cheap rolling-rate
computation) and `stages.json` (current stage per task-type). All
writes happen under `flock(2)` on `.ledger.lock` (design-pass Q3).
Promotion thresholds per convention §2: review→spot-check at n≥50
+ accept-rate ≥0.85; spot-check→autonomous at n≥100 + accept-rate
≥0.95.

### Corpus tuple paths

- Production routing (verdict accepted/refined/rejected/deferred):
  `${FOUNDRY_ROOT}/data/training-corpus/apprenticeship/<task-type>/<ulid>.jsonl`
- Shadow routing: `${FOUNDRY_ROOT}/data/training-corpus/apprenticeship/<task-type>/shadow-<brief_id>.jsonl`
- DPO pair (refine / reject only, per convention §8 +
  `trajectory-substrate.md` §6):
  `${FOUNDRY_ROOT}/data/training-corpus/feedback/apprenticeship-<task-type>-<ulid>.jsonl`

Every body field passes through the `crate::redact::sanitize` filter
(Rust port of `bin/capture-edit.py` REDACTIONS) before write per
convention §9 sanitize-outbound rule.

### In-process state

`BriefCache` — in-process FIFO from `(brief_id, attempt_id)` to the
brief / attempt produced by `/v1/brief`. The verdict path looks the
pair up by key; cache miss surfaces as `BriefCacheMiss` (HTTP 410
Gone) and the senior reissues. Process restart loses pending briefs;
SQLite-backed durability is a v0.5+ upgrade.

### Configuration

| Env var | Default | Purpose |
|---|---|---|
| `SLM_APPRENTICESHIP_ENABLED` | unset (off) | Gates the three endpoints; `true` or `1` to enable |
| `FOUNDRY_ROOT` | `/srv/foundry` | Resolves `scope.files`, `citations.yaml`, ledger / corpus paths |
| `FOUNDRY_ALLOWED_SIGNERS` | `${FOUNDRY_ROOT}/identity/allowed_signers` | Path passed to `ssh-keygen -Y verify -f` |
| `FOUNDRY_DOCTRINE_VERSION` | `0.0.7` | Embedded in corpus tuples |
| `FOUNDRY_TENANT` | `pointsav` | Tenant tag on corpus tuples |
| `SLM_BRIEF_TIER_B_THRESHOLD_CHARS` | `8000` | Char-budget proxy for Tier-B routing on `/v1/brief` |

---

## 12. os-totebox integration

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
is non-starter; a Rust binary with a quantised llama-server runtime
fits. This is the proof-of-need for the Rust path on
resource-constrained Toteboxes specifically.

---

## 13. 2030 headroom

Primitives that are still research or RFC today slot into this
architecture without rewriting it. Each is a config or subdirectory
addition, not a refactor.

| Primitive | Hook |
|---|---|
| CUDA checkpoint/restore (vLLM RFC #34303) | Ring 1: optional checkpoint bundle as bootstrap input |
| C-LoRA single-adapter (arXiv 2502.17920) | Ring 3b: registry schema migration from dual → single |
| FP8 KV-cache quantisation | Ring 2: config flag (`KV_CACHE_DTYPE=fp8`) |
| Sleep-time compute (async memory management) | Ring 3b: nightly LoRA retraining on Batch API |
| Encode-Prefill-Decode disaggregation (SGLang + Mooncake) | Ring 2 evolution: separate prefill and decode pools |

None of these require rewriting `service-slm`.

### Leapfrog 2030 — named Yo-Yo node profiles

The multi-Yo-Yo HashMap (§9.2) is designed to support two permanent named
nodes alongside any number of ephemeral burst nodes. Current target profiles:

**Yo-Yo #1 — Trainer** (`key: "trainer"`)

| Field | Value |
|---|---|
| Machine | `g2-standard-4` (1× L4 24 GB VRAM) |
| Model | OLMo 3 32B-Think Q4 (no commercial API; must self-host) |
| Schedule | Night-shift 23:00–06:00 PT (`idle_shutdown_minutes` clears at 06:00) |
| Cost | ~$0.18/hr spot (us-west1); ~$0.45/hr on-demand |
| Purpose | QLoRA fine-tuning on the apprenticeship corpus; produces `engineering-pointsav` adapter |

**Yo-Yo #2 — Graph Extractor** (`key: "graph"`)

| Field | Value |
|---|---|
| Machine | `a3-highgpu-1g` (1× H100 80 GB VRAM) |
| Model | Llama 3.3 70B Instruct (broad general capability) |
| Schedule | Manual batch — operator spins up; idle-shutdown after batch completes |
| Purpose | Ontologically-strict entity extraction for Ring 3a graph; will accept grammar constraints from service-content JSON Schema via `extra_body.structured_outputs` |

Full phase roadmap: `service-slm/docs/topic-leapfrog-architecture.md`.
Phase 3 (training threshold detection + cron trigger) has not started;
gated on operator go-ahead and Phase 3 scoping.

---

## 14. Cross-references

- `CLAUDE.md` — state header, hard constraints, project-layer rules
- `NEXT.md` — in-flight work and blocking items
- `DEVELOPMENT.md` — build, CI, licence policy, phase roadmap
- `service-slm/docs/audit-endpoints-contract.md` v0.2.0 — canonical wire contract for `POST /v1/audit/proxy` + `POST /v1/audit/capture` (request/response shapes, validation order, ledger schema, error table)
- `content-wiki-documentation/topic-service-slm.md` — Rust-native
  stack rationale, licence discipline, open-source posture
  *(destination not yet committed — see workspace `NEXT.md`)*
- `content-wiki-documentation/topic-yoyo-compute.md` — three-ring
  model rationale, audit-ledger commercial argument, hyperscaler
  differentiators *(destination not yet committed)*
- Workspace `CLAUDE.md` — identity store, commit flow, cluster
  session pattern, ADR hard rules
- `~/Foundry/conventions/apprenticeship-substrate.md` — full spec for §11
- `~/Foundry/conventions/zero-container-runtime.md` — structural doctrine for the deployment model
- `~/Foundry/conventions/three-ring-architecture.md` — ring model rationale
