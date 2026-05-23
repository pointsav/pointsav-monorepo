---
artifact: brief
status: archived
archived: 2026-05-23
archived_reason: absorbed by BRIEF-flow-restructure.md §8; live gaps pulled into BRIEF-vm-hardening-and-consolidation.md todo
---

# Audit: service-slm & service-content — Development Plan

> Authored: 2026-05-16 Opus deep-think (226K tokens, 25 tool uses)
> Status: Active — primary engineering reference for both services
> Entry point: MASTER-PLAN-2026.md → this document
> Companion: sovereign-routing-comprehensive.md (Claude Code routing sprints)

---

## 1. Current State Assessment

### service-slm — three-crate Rust workspace, ~177 tests passing

**LIVE and verified:**
- Doorman HTTP server (`local-doorman.service`, port 9080) — production on workspace VM
- Tier A (llama-server, OLMo 2 1B Instruct Q4, port 8080) — always-on, RSS ~2GB
- Audit ledger — four `entry_type` discriminators; JSONL per-day at `$SLM_AUDIT_DIR`; contract v0.2.0
- Apprenticeship substrate (AS-1..7) — `/v1/brief`, `/v1/verdict`, `/v1/shadow` live; brief queue with `flock(2)` + drain worker; 453 engineering + 137 apprenticeship tuples accumulated
- Anthropic Messages shim (`POST /v1/messages`, http.rs:1214) — Sprint 0a complete; model→tier routing; **fake-SSE only** (full content buffered then emitted as 6 events at once)
- Multi-Yo-Yo HashMap — `"default"`, `"trainer"`, `"graph"` labels wired in `main.rs`
- Tier B resilience — 60s socket + 90s outer deadline, 3-state circuit breaker (5 failures → 300s cooldown), background `/health` probe every 30s
- Grammar substrate — Lark/GBNF/JsonSchema typed enum; Tier C rejects all; `llguidance` pre-validates Lark
- Graph context client — `GraphContextClient` injected at router.rs:128-160; prepends `[ENTITY CONTEXT]` system message; non-fatal on failure
- Yo-Yo idle monitor — Rust-native; `last_yoyo_dispatch` AtomicU64; preemption auto-restart live as of 2026-05-16

**Code-complete but inactive / deployment-gated:**
- Tier B (Yo-Yo) live infrastructure — VM TERMINATED; manual start-yoyo.sh required; **all three endpoint env vars must be manually sed-updated on every VM start** (start-yoyo.sh sed fails silently)
- Tier C — wired, no API key configured; `audit_proxy_client: None` → 503
- Mesh discovery — `DiscoveryProvider`/`MeshRegistry`/`DynamicRegistry` scaffolded; `route_async()` selects a node by label then **falls through to `route()` without using `node.endpoint`** — Phase 1 stub, dead code
- Phase 3 training threshold detection — `corpus-threshold.py` + Sunday cron units installed; marker-only mode pre-D4

**Scaffolded / stub:**
- `slm-ledger`, `slm-compute`, `slm-memory-kv/adapters`, `slm-inference-*`, `slm-api`, `slm-cli` crates from ARCHITECTURE.md §6 — not created
- `cognitive-forge/` legacy subcrate — workspace-excluded, rename pending
- `transient-queues/` — defect, triage pending

### service-content — single-binary watcher, port 9081

**LIVE:**
- LadybugDB graph store at `service-content/graph/entities.lbug` (MemoryMax=6G, ~16-min load for 10K entities)
- HTTP server (port 9081): `/healthz`, `/v1/graph/context`, `/v1/graph/mutate`, `/v1/config/guides*`
- File watcher on `service-fs/data/.../service-content/ledgers` — processes CORPUS_*.json files
- `POST /v1/extract` route via Doorman `route_yoyo_only("trainer")` — fixed 2026-05-14 after VM crash

**Broken / missing:**
- ARCHITECTURE.md declares the current `src/main.rs` **deprecated legacy**; MCP-server rebuild not implemented
- `processed_ledgers` is a RAM-only `Vec<String>` (main.rs:102) — every restart re-attempts every CORPUS file (root cause of 2026-05-13 8.5-hour retry storm)
- `unwrap()` on startup dir creation (lines 47, 48, 53) and `fs::write` (line 293) → panic on disk-full
- `/healthz` always returns OK — doesn't probe graph or Doorman
- `RelatedTo` graph table declared but never populated → graph is node-only; no edges
- No `module_id` validation → `__taxonomy__` namespace can be overwritten by malicious CORPUS file
- `content-compiler/Cargo.toml` has only `serde_json` + `chrono` — essentially empty crate

---

## 2. Critical Gaps (Production-Reliability Blockers)

| # | Gap | Service | Impact |
|---|---|---|---|
| 1 | `processed_ledgers` non-persistent | content | 114-file retry storm on every restart; root cause of 2026-05-13 VM crash |
| 2 | `start-yoyo.sh` sed silently fails on IP update | slm/ops | All Tier B requests fail until 3 env vars manually re-written |
| 3 | `route_yoyo_only` writes NO audit entry | slm | `/v1/extract` traffic invisible in ledger — compliance untraceable |
| 4 | `graph-query` event_type not in `AUDIT_CAPTURE_VALID_EVENT_TYPES` | slm | Graph proxy handlers bypass their own validation (http.rs:1072) |
| 5 | Tier C unconfigured → opus-routed shim requests return 503 | slm | Sprint 0a viable for haiku/sonnet only |
| 6 | service-content `/healthz` always 200 | content | Monitor cannot detect broken state |
| 7 | No `module_id` validation in service-content (`__` prefix not blocked) | content | Taxonomy namespace injection via crafted CORPUS file |
| 8 | `service-content/src/main.rs:293` unwrap on `fs::write` | content | Disk-full → panic → service exits |
| 9 | `ExtractionAuditEntry` missing `model`, `cost_usd`, `sanitised_outbound` | slm | Per-extraction cost accounting impossible |
| 10 | `route_async()` dead code (Orchestrator never set in main.rs) | slm | Mesh code paths compile but cannot execute; misleading |
| 11 | Sprint 0a flattens `tool_use`/`tool_result` blocks | slm | Claude Code agentic loop breaks until Canonical IR (Sprint 1) |
| 12 | Fake-SSE emits whole response as single burst | slm | Claude Code timeouts on long responses |

---

## 3. Test Coverage Audit

**What exists (~177 tests):**
- `slm-core`: 14 (serde round-trips, ModuleId, grammar variants)
- `slm-doorman`: 102 — tier clients, ledger, audit_proxy, grammar_validation, apprenticeship (incl shadow dedup), verdict, brief queue, citations, redact
- `slm-doorman-server`: 48 HTTP + 4 audit-integration + 5 queue + 3 idle-monitor

**Highest-risk untested paths:**
1. **Anthropic shim end-to-end** — no integration tests for `/v1/messages`; `flatten_anthropic_content` `Blocks` variant has no test coverage for tool_use/tool_result
2. **Yo-Yo idle monitor preemption auto-restart** — no E2E with actual GCP preemption simulation
3. **service-content has no test suite** — Cargo.toml shows no `[[test]]` or `[dev-dependencies]`
4. **Graph context injection** — failure path, `graph_context_enabled=Some(false)` short-circuit untested
5. **Cross-service audit chain** — no test verifies a `/v1/messages` request produces correct ledger entries when Tier B selected
6. **Circuit breaker recovery under real network** — only mock tests; no fault-injection tests
7. **`route_yoyo_only` defer paths** — `DeferReason::{YoyoLabelUnconfigured, YoyoCircuitOpen, YoyoTransient}` mapping
8. **Multi-Yo-Yo label fallback** — when no label provided, picks "first entry in HashMap insertion order" (router.rs:267) — non-deterministic; no test pins behaviour
9. **`/v1/audit/proxy` two-entry ledger consistency** — stub-written-but-call-failed path
10. **Graph proxy `/v1/graph/{query,mutate}`** — built `ReqwestClient::new()` per request; no integration tests

**Test infrastructure to build:**
- End-to-end harness: real Doorman + mock Anthropic + mock vLLM + mock service-content under a single zero-container test runner
- Property-based tests for `ModuleId` validation, sanitize regex stability
- Snapshot/golden-file test for SSE event ordering in `anthropic_sse_body`
- service-content needs unit tests for `taxonomy.rs`, `graph.rs`, `http.rs` from zero
- Fault-injection middleware for circuit breaker / health probe

---

## 4. NOW Plan — Near-Term (1–8 weeks)

Ordered by priority.

| # | Task | Files | LOC | Benefit |
|---|---|---|---|---|
| 1 | **Persist `processed_ledgers` to disk** (SQLite or sidecar JSONL) | service-content/src/main.rs | ~80 | Closes root cause of 2026-05-13 retry-storm |
| 2 | **Fix `start-yoyo.sh` sed unconditional update** | scripts/start-yoyo.sh | ~20 | Removes manual 3-line sed on every VM start |
| 3 | **Replace all `unwrap()` in service-content** with `?` + structured error logging | service-content/src/main.rs:47,48,53,293 | ~50 | Disk-full no longer panics the service |
| 4 | **Real `/healthz` in service-content** (graph ping + Doorman probe, 2s timeout) | service-content/src/http.rs | ~40 | Monitor detects broken state |
| 5 | **`module_id` validation in service-content** (reject `__` prefix, enforce `[a-z0-9-]{1,64}`) | service-content/src/{main,http}.rs | ~30 | Closes taxonomy-namespace injection vector |
| 6 | **Audit-write on `route_yoyo_only`** + extend `ExtractionAuditEntry` schema | slm-doorman/src/router.rs, ledger.rs, http.rs | ~120 | Extraction traffic traceable + cost-accounted |
| 7 | **Add `graph-query` to `AUDIT_CAPTURE_VALID_EVENT_TYPES`** | slm-doorman/src/ledger.rs | ~5 | Doorman stops bypassing its own validation |
| 8 | **Sprint 0b: real SSE token streaming** | tier/yoyo.rs (bytes_stream), http.rs::anthropic_sse_body | ~120 | Claude Code no longer times out; true streaming UX |
| 9 | **Sprint 0b: on-demand Yo-Yo lazy-start gate** (`SLM_YOYO_AUTO_START=true`) | router.rs, start-yoyo.sh | ~80 | First request auto-spins Yo-Yo VM |
| 10 | **`anthropic_messages` integration tests** (haiku→A, sonnet→B, opus→C; stream; tool_use) | tests/anthropic_shim_test.rs | ~250 | Locks Sprint 0a contract; prevents regression |
| 11 | **Sprint 1: `CanonicalMessage` + `ContentBlock` in slm-core** | slm-core/src/lib.rs + all tier clients | ~230 | Unlocks Claude Code agentic loop (tool_use round-trip) |
| 12 | **Sprint 1.5: preserve `cache_control` blocks end-to-end** | slm-core + tier/external.rs | ~60 | 5–10× cost reduction on Tier C calls |
| 13 | **Sprint 2: native Anthropic outbound for Tier C** | tier/external.rs | ~150 | Removes one translation hop; prompt-caching native |
| 14 | **Sprint 2: `POST /v1/responses` (OpenAI Responses API inbound)** | slm-doorman-server/src/http.rs | ~180 | Chat-Completions sunset prep |
| 15 | **service-content Sprint 1: deterministic Source node write** (before Doorman call) | service-content/src/main.rs ~line 198 | ~30 | Graph grows even when Tier B is down |
| 16 | **service-content `RelatedTo` writes + schema migration** | service-content/src/graph.rs | ~150 | Graph stops being node-only; enables traversal queries |
| 17 | **service-content SIGTERM handler** (graceful shutdown) | service-content/src/main.rs | ~40 | Systemd restart doesn't lose in-flight CORPUS file |
| 18 | **Observability: `metrics-exporter-prometheus` on Doorman** (per-tier latency histograms, circuit-state gauge, queue depth, audit-entry counter) | slm-doorman-server/src/main.rs | ~150 | Real visibility into production system |
| 19 | **Structured logging (`tracing` JSON output) in service-content** | service-content/src/main.rs | ~40 | Currently `println!` — no request_id, no level, no correlation |
| 20 | **Idle monitor: emit per-cycle metrics** (`slm_yoyo_idle_seconds`, `slm_yoyo_dispatches_total`) | slm-doorman-server | ~30 | Watchdog status observable from outside |
| 21 | **Brief queue lease-expiry alerting** (NOTAM via outbox when >5 leased >1h) | slm-doorman-server | ~50 | Stuck briefs surface to operator without manual polling |
| 22 | **`/readyz` extended fields** (`lark_validation_active`, `apprenticeship_enabled`, `tier_b_circuit_state`, `last_yoyo_dispatch_age_s`) | slm-doorman-server/src/http.rs | ~40 | Health checks become diagnostic |
| 23 | **Refactor graph-proxy out of http.rs into slm-doorman crate** | new slm-doorman/src/graph_proxy.rs | ~150 net | Coupling concern; per-request `ReqwestClient::new()` removed |

**Total NOW LOC ≈ 1,800 net.** Items 1–7 are pure correctness/safety — land before Sprint 0b. Sprints 0b+1+2 (~870 LOC) are the sovereign-routing high-leverage block.

---

## 5. LONG-TERM Plan — 3–18 months

### 5.1 Canonical IR completion (Sprint 1+)
Replace `ChatMessage { role, content: String }` with `CanonicalMessage { role, content: Vec<ContentBlock> }`. `ContentBlock` discriminated union: `Text`, `ToolUse`, `ToolResult`, `Thinking`, `Image`, `Cache`. Per-tier adapters in/out (Anthropic native, OpenAI Chat-Completions, OpenAI Responses, vLLM, llama-server). ~510 LOC net across 5 sprints — wire-format leapfrog 2030 plan.

### 5.2 MCP server (Sprint 3, ~400 LOC new crate `slm-mcp-server`)
Use `rmcp` Rust SDK; stdio transport per MCP spec 2025-11-25. Tools: `foundry:query-datagraph`, `foundry:mutate-datagraph`, `foundry:get-entity-context`, `foundry:get-corpus-stats`, `foundry:submit-extraction`, `foundry:doorman-health`. Highest leverage: one binary serves Claude Code, Cursor, Cline, Continue, Goose simultaneously.

### 5.3 A2A integration (Sprint 5, ~4 weeks)
Agent card at `GET /a2a/agent-card`; task submit at `POST /a2a/tasks/send`. Governance fields: `audit_trail=true`, `sovereignty=on-premise`, `compliance=[ISO-42001, NI-51-102]`. Lives in `moonshot-protocol`. Positions Foundry as sovereign node in Linux Foundation A2A mesh.

### 5.4 Training pipeline wiring (`moonshot-slm`)
Post-commit hook installation (`git config --global core.hooksPath`). `SLM_SHIM_TRAINING_CAPTURE` gate (Tier A/B only; Tier C excluded per ToS). Sunday 02:00 UTC LoRA training cron. Unsloth + TRL `DPOTrainer` on OLMo 2/3 base; r=16, lora_alpha=32, beta=0.1. Replay buffer 20–30%; held-out 100-pair eval gate (reject on >5% regression). Adapter signing via `sigstore` → GCS-archived with SLSA attestation (Ring 3b). 1,000-pair LIMA threshold gate before first real run.

### 5.5 service-content PUSH inversion (5 sprints, net code reduction)
- Sprint 1: deterministic Source node (in NOW plan)
- Sprint 2: schema migration (`node_type`, `source_worm_id`, `RelatedTo` writes)
- Sprint 3: Doorman-side extraction queue + PUSH to `/v1/graph/mutate`; delete PULL from service-content (−120 LOC)
- Sprint 4: move `/v1/draft/generate` to Doorman; delete from service-content (−120 LOC; closes Ring 2→Ring 3 violation)
- Sprint 5: `processed_ledgers` → graph query; 114-file migration script (−20 LOC)

Net: service-content shrinks; queue lives where it belongs (Ring 3).

### 5.6 LadybugDB maturity / `GraphStore` trait swap
Business logic talks to `GraphStore` trait, not lbug directly. Current `LbugGraphStore` is the only implementation; needs second (in-memory mock, or SQLite-graph fallback) to validate the abstraction. LadybugDB is a post-Apple-acquisition Kuzu fork (Oct 2025) — maintenance signal uncertain; carrying patches may be required. Ready-to-swap architecture enables `moonshot-database` when it matures.

### 5.7 Vector store (sqlite-vec)
ARCHITECTURE.md §5 calls for `sqlite-vec` co-located with the graph. Embedding via Doorman-mediated call (no Python embedding service). Required for `vector_search` MCP tool.

### 5.8 Mesh discovery — make `route_async` actually dispatch
`StaticConfigProvider` (reads `SLM_MESH_NODES` env var) needs wiring. `route_async` must use `node.endpoint` to construct a request, not delegate to `route()`. Carbon-aware selection eventually (EnergySource on NodeDescriptor → solar nodes preferred when available).

### 5.9 Substrate completion — remaining service-slm crates
`slm-ledger` (split when SQLite index sits alongside JSONL append log), `slm-memory-kv` (LMCache + Mooncake Store integration), `slm-memory-adapters` (LoRA adapter registry, GCS-backed, Sigstore-verified), `slm-inference-{local,remote}` (split runtime concerns out of `slm-doorman/tier/`). Each waits for a real consumer before scaffolding.

### 5.10 service-content full rebuild
ARCHITECTURE.md declares `main.rs` deprecated. Full MCP-server rebuild: Cypher-subset query language, temporal-query support, vector store, seed pack loader. Decision required: incremental hardening (items 1–17 of NOW plan) buys ~6 months, OR commit to rebuild as a project-data Task.

---

## 6. Integration Points Between the Two Services

**Today (working):**
1. Doorman → service-content via `GraphContextClient` — pre-inference enrichment; non-fatal; injects `[ENTITY CONTEXT]` system message; first 200 chars of last user message as query (router.rs:120-160)
2. service-content → Doorman via `POST /v1/extract` — `route_yoyo_only("trainer")`, JsonSchema-constrained, no Tier A fallback (fix landed 2026-05-14)
3. Doorman → service-content via `POST /v1/graph/query` and `/v1/graph/mutate` proxies — new `ReqwestClient` per request; audit-logged as `event_type: "graph-query"` / `"graph-mutation"`

**Fragile:**
- Graph-proxy handlers in `http.rs:1011-1146` build new HTTP client per call (no pooling); live in server crate instead of `slm-doorman` (coupling concern)
- No health check on service-content endpoint; if service-content is in 16-min boot, graph proxy returns 5xx with no degraded mode
- `graph-query` not in validation set → Doorman bypasses its own validation
- Per-request `ReqwestClient::new()` means no keepalive, no TLS session resumption

**Should be redesigned:**
- Replace ad-hoc HTTP with shared `GraphContextClient` pool (single `ReqwestClient`, configurable pool size)
- Move graph-proxy handlers from http.rs into `slm-doorman::graph_proxy`
- Long-term: replace HTTP with MCP — service-content exposes `graph_query`/`graph_mutate` MCP tools; Doorman calls via same `slm-mcp-server` integration that external clients use
- Health-check protocol: Doorman polls service-content `/readyz` and pauses extraction during 16-min boot window

---

## 7. Operational Hardening

| Area | Now state | Required |
|---|---|---|
| **Monitoring** | `journalctl` only; no Prometheus | Wire `metrics-exporter-prometheus` (Doorman: per-tier latency histogram, circuit gauge, queue depth, audit counter, last_yoyo_dispatch_age; service-content: file-watcher event rate, processed_ledgers size, graph entity count) |
| **Alerting** | Manual NOTAM in mailbox | Alertmanager rules: Tier B circuit open >5min; service-content processed_ledgers >50 deferred; audit ledger write failure; OOM-killer event; idle monitor stop_sent without successful start within 1h |
| **Systemd hardening** | Basic units; ProtectHome fix outstanding (NEXT.md:117) | Add `MemoryHigh` (soft cap below `MemoryMax`); `Restart=on-failure` with `RestartSec=10s`; `RuntimeMaxSec`; `OOMScoreAdjust`; `ProtectKernelTunables`; `SystemCallFilter` |
| **Graceful degradation** | service-content halts when Doorman unavailable | Sprint 1 fix in NOW plan (deterministic Source node). Long-term PUSH inversion |
| **Restart sequencing** | Manual 5-step NEXT.md runbook per VM start | Encode as `start-foundry.sh` orchestrator: yoyo start → wait Doorman 90s probe → restart content → verify extraction |
| **Credential rotation** | Static bearer tokens | `BearerTokenProvider` trait exists — implement `GcpWorkloadIdentityBearer`; rotate on 401/403 |
| **Snapshot discipline** | Snapshots manual | Nightly `boot-disk-snapshot.timer` systemd unit |
| **Image rebuild** | Manual VM patches | Bake fixes (vllm masked, llama-server enabled, tokenizer fix, nginx auth map split) into next `pointsav-public` Packer image |
| **Apprenticeship corpus lifecycle** | Tuples accumulate; no quality gate | Min brief/diff size, dedup by hash, PII scrub, held-out 100-pair eval set, ≥98% baseline gate before adapter deploy |
| **Log volume** | journald defaults; ~16 min/restart × 10K entity log spam | Loki aggregator or LogRotate; structured JSON output to enable `jq`-grep at scale |

---

## 8. Open Questions for the Operator

1. **Anthropic Commercial API key** — when provisioned + monthly cap? Sprint 0a opus-tier is 503 until set
2. **Legal review for `SLM_SHIM_TRAINING_CAPTURE`** — when is counsel review booked? (See topic-tos-training-constraints.md §4)
3. **`route_async()` dead code** — kill now, or keep as documented Phase 1 stub?
4. **Tier A model upgrade (1B → 7B)** — what is the upgrade window? Required for haiku-tier quality
5. **Packer image rebuild** — when? Every manual VM start requires sed-fixes that should be baked in
6. **Cmake on workspace VM** — confirmed installed, or schedule? Blocks `cargo build` of service-content on `lbug = "0.16"`
7. **Phase 3 training threshold go-ahead** — corpus is past threshold (453+137 tuples); gated on operator green-light + D4 image
8. **Multi-Yo-Yo default behaviour** — when `yoyo_label` is `None`, should router error or pick `"default"` explicitly?
9. **service-content rebuild vs incremental** — incremental hardening (items 1–17) buys ~6 months; full rebuild is a longer commitment. Operator decision required
10. **Sprint 4a timing** — `app-console-slm` admin TUI (1–2 weeks): start after Sprint 3 or earlier?
11. **`SLM_YOYO_WEIGHTS_GCS_BUCKET` for training markers** — configure now or wait for D4?

---

## Key File References

| File | Purpose |
|---|---|
| `service-slm/crates/slm-doorman/src/router.rs` | Tier routing, graph context injection, route_async stub |
| `service-slm/crates/slm-doorman-server/src/http.rs:1213-1448` | Anthropic shim (Sprint 0a) |
| `service-slm/crates/slm-doorman/src/tier/yoyo.rs` | Tier B client, circuit breaker, idle monitor |
| `service-slm/crates/slm-doorman/src/apprenticeship.rs` | /v1/shadow, brief queue, drain worker |
| `service-content/src/main.rs` | Watcher loop, processed_ledgers (RAM-only — gap #1) |
| `service-content/ARCHITECTURE.md` | Full rebuild blueprint (current main.rs = deprecated legacy) |
| `service-slm/NEXT.md` (lines 9-52) | Manual VM start runbook (⚠️ CRITICAL) |
| `.agent/plans/sovereign-routing-comprehensive.md` | Claude Code routing sprints (companion) |
| `.agent/plans/MASTER-PLAN-2026.md` | Entry point / companion index |
