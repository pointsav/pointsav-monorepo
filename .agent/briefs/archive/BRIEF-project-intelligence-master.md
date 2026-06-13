---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-master
title: "project-intelligence Master BRIEF — Sovereign AI Platform for SMBs"
status: archived
owner: project-intelligence
created: 2026-06-04
updated: 2026-06-12
author: totebox@project-intelligence (claude-sonnet-4-6)
moved_to: project-intelligence
archived: 2026-06-12
supersedes:
  - BRIEF-slm-substrate-master.md
  - BRIEF-yoyo-cloud-run-migration.md
grounds_in:
  - service-slm/ARCHITECTURE.md
  - service-content/CLAUDE.md
  - service-content/ARCHITECTURE.md
  - app-orchestration-slm/CLAUDE.md
  - app-console-slm/CLAUDE.md
  - /home/mathew/.claude/plans/we-need-way-to-inherited-squirrel.md
  - DOCTRINE.md claims #44, #48, #49, #54
---

# BRIEF — project-intelligence Master

> **PRIMARY PLAN OF RECORD.** Every engineering session for service-slm,
> service-content, app-orchestration-slm, and app-console-slm reads this first.
>
> Reference docs (do not duplicate): `service-slm/ARCHITECTURE.md`,
> `service-content/CLAUDE.md`, `app-orchestration-slm/CLAUDE.md`.

---

## §1 — Core Vision

This platform delivers **agentic memory and learning** to small and medium businesses.
It is sovereign: the organization owns the weights, training data, and ontological graph.

The fundamental insight: commercial AI services provide general reasoning but know nothing
about any specific organization. This platform inverts that: a sophisticated ontological
DataGraph encoding everything about THIS business, injected into every inference. A compact
local model traversing a rich business ontology outperforms a large general model with no
organizational context on domain-specific work. The graph is the lever.

Research validation: temporal knowledge graph architecture → 18.5% accuracy improvement,
90% latency reduction vs vector similarity search, on the same underlying model.

**Generic terminology rule:** In all SOFT/CODE/TOPIC/GUIDE artifacts, use generic terms.
Vendor-specific names appear only in deployment GUIDEs.

| Deployment-specific | Generic equivalent |
|---|---|
| OLMo 7B / OLMo 3 32B Think | "local inference model" / "extended-reasoning model" |
| LadybugDB | "embedded organizational knowledge graph" |
| Doorman | "inference router" / "tier routing gateway" |
| Yo-Yo / YoYo | "burst GPU node" / "remote inference node" |
| Totebox | "local inference node" / "archive node" |
| GCE a2-highgpu-1g | "dedicated GPU instance" |
| Cloud Run / GCP | "serverless GPU service" / "cloud GPU provider" |
| `woodfine-node-gcp-free` | "[your-project]" |

---

## §2 — Current Live State (2026-06-05)

| Component | Status | Notes |
|---|---|---|
| `local-slm.service` | **ACTIVE PRIMARY** | OLMo 2 1124 7B Instruct Q4_K_M; Tier A; 3.38 tok/s (`:8080/metrics` → `/v1/status/tier-a`) |
| `local-doorman.service` | **ACTIVE** | Sprint 4+5 commit `1202e6ee`; `/v1/status/cost`, `/v1/status/tier-a`, enhanced `/v1/status/queue`; sha256 in binary-ledger |
| `local-content.service` | **ACTIVE** | 7,445 entities in LadybugDB (`module_id=woodfine`); 43,107 processed ledger entries; circuit breaker OPEN (Tier B offline) |
| `slm-mcp-server` | **INSTALLED + WIRED** | `/usr/local/bin/slm-mcp-server`; `.mcp.json` at archive root; 6 tools; `SLM_MODULE_ID=woodfine` |
| `yoyo-tier-b` (Cloud Run) | **DELETED (2026-06-04)** | Deleted — unpredictable per-second billing; see §9 |
| `yoyo-tier-b-1` (GCE Spot) | **TERMINATED** | europe-west4-a L4 stockout; static IP `34.6.204.25` released |
| `orchestration-slm-server` | **NOT YET DEPLOYED** | Code complete; Command Session install needed |

**Tier routing (current):**
- Tier A: **ENABLED PRIMARY** — OLMo 7B; all interactive + background routes here; 3.38 tok/s
- Tier B: **OFFLINE** — both Cloud Run and GCE VM deleted/terminated; circuit breaker OPEN
- Tier C: **NOT CONFIGURED** — never enable for training; opt-in only

**Apprenticeship (2026-06-05):**
- Queue pending: 246  in-flight: 3  paused: 0
- Queue done: 1,004  quarantine: 737  poison: 29
- Training corpus SFT: 304 samples in `2026-06-01-train.jsonl` at `/srv/foundry/data/training-corpus/apprenticeship/`
- DPO corpus: 591 degenerate tuples (Phase D quarantine pending)

**New status endpoints (commit `1202e6ee`):**
- `GET /v1/status/tier-a` → `{reachable, tok_per_s, requests_processing, prompt_tokens_total}` — llama-server Prometheus metrics
- `GET /v1/status/cost` → `{daily_usd, local_usd, yoyo_usd, ext_usd, vm_hours_usd, request_count}` — cost ledger rollup
- `GET /v1/status/queue` → `{pending, in_flight, paused, done, poison, quarantine}` — full queue breakdown

---

## §3 — System Topology

```
WORKSPACE VM (Totebox, os-totebox)
┌────────────────────────────────────────────────────────────┐
│  service-slm (Doorman) :9080          Apache 2.0           │
│                                                            │
│  Tier A ── OLMo 7B llama-server :8080 (always-on)         │
│  Tier B ── app-orchestration-slm :9180 (GPU burst)         │
│  Tier C ── external API (opt-in, NEVER for training)       │
│                                                            │
│  [KILL SWITCH] per-label + global                          │
│  [EXPRESS LANE] /v1/express/* (bypasses queue, not switch) │
│  [FLOW POLICY] Balanced/DrainBatch/DrainExpress/LocalOnly  │
│  [QUEUE] P0/P1/P2 interleaved drain                        │
│  [MCP SERVER] :9090 — no-API-key entry point               │
└────────────────────────────────────────────────────────────┘
               │ SERVICE_CONTENT_ENDPOINT=:9081
               ▼
┌────────────────────────────────────────────────────────────┐
│  service-content (DataGraph) :9081    Apache 2.0           │
│  LadybugDB — one graph per archive, module_id scoped       │
│  → extraction via Doorman /v1/extract (Tier B)             │
│  → context via /v1/graph/context (read by Doorman)         │
└────────────────────────────────────────────────────────────┘
               │ SLM_ORCHESTRATION_ENDPOINT=:9180
               ▼
┌────────────────────────────────────────────────────────────┐
│  app-orchestration-slm :9180    Proprietary, $99/yr USDC   │
│  YoYo pool (planned):                                      │
│   "batch"   — L4 24GB, g2-standard-4, us-central1-a        │
│              daily drain 1–4 hrs, ~$0.71/hr                │
│   "express" — A100 40GB, a2-highgpu-1g, us-central1-a      │
│              on-demand, starts on request, $3.67/hr        │
│  Centralized CB + kill switch + VM lifecycle               │
└────────────────────────────────────────────────────────────┘
```

---

## §4 — The DataGraph — One Per Totebox Archive

**DOCTRINE SETTLED (do not re-open):**
- Doctrine claim #44: "per-tenant knowledge graph"
- Doctrine claim #48: "customer's intellectual property"
- One LadybugDB per Totebox Archive; module_id scopes logical tenants within it

All service-* write entities via Doorman `/v1/graph/mutate` partitioned by `module_id`.
service-bookkeeping (when built) is a Ring 2 sibling — it writes ontological facts
(vendors, contracts, account structures) to the SHARED graph, and transactional records
(individual invoices, journal entries) to service-fs WORM ledger. Never its own graph.

**Current entity types (5 live):**
- Person, Company, Project, Account, Location — from CORPUS extraction
- + taxonomy types: archetype (13), coa-profile, domain, theme, topic, guide, glossary-{domain}

**Schema (LadybugDB — must not change for backward compatibility):**
```
Entity(id STRING PK,          -- "module_id__entity_name_lowercase"
       entity_name STRING,
       classification STRING,  -- Person|Company|Project|Account|Location|...
       role_vector STRING,
       location_vector STRING,
       contact_vector STRING,
       module_id STRING,
       confidence DOUBLE,
       created_at STRING)
RelatedTo(FROM Entity TO Entity, relation_type STRING)
```

**Generic extensibility (already built):** entity types come from CSV files in
`SERVICE_CONTENT_ONTOLOGY_DIR`. Users configure their own ontology without code changes.
The extraction schema is derived from whatever classifications appear in those CSVs.

**Target (multi-hop, NOT hardcoded — user configures via CSV):**
- Ontological facts (who entities are, how they relate, what governs them) → graph
- Transactional records (individual invoices, journal entries) → service-fs WORM

---

## §5 — Memory & Learning = Graph + LoRA + Apprenticeship

Three components compound into organizational memory:

```
GRAPH: what this organization knows
  → entities + relationships + temporal validity
  → injected into every inference as [ENTITY CONTEXT] system message
  → grows richer with each extraction pass

LORA: how this organization reasons
  → domain vocabulary + decision patterns
  → trained on actual operator decisions
  → monthly refresh as corpus grows

APPRENTICESHIP: continuous learning signal
  → every commit, approval, correction → training data
  → DPO pairs from operator accept/reject
  → flywheel: better model → better extraction → richer graph
```

**Honest quality trajectory:**

| Stage | Condition | Quality vs. org-specific tasks |
|---|---|---|
| Now | 77 good SFT samples, graph live | ~65% |
| Month 6 | 5k samples, first LoRA trained | ~78% |
| Month 12 | 15k samples, multi-domain LoRA | ~90% |
| Month 18 | 25k+ samples, temporal traversal | ~95% |

This system does NOT match a large commercial model on general reasoning. It does
exceed it for THIS organization's specific domain because it knows everything about
this organization and the general model knows nothing about it.

**Training architecture (2026-05-31 revised):**
- SFT first (GPU L4, 77+ real diffs from post-Fix-A queue)
- CodeDPO ONLY on GPU (never CPU); never train on empty rejected samples
- LoRA: DISABLED pending GPU availability + operator approval (`SLM_LORA_AUTO_ENABLE=true`)

---

## §6 — Kill Switch & Billing Control

Nothing bypasses the kill switch. The express lane bypasses the queue, not the kill switch.

```
POST /v1/flow/kill/batch    { "closed": true }  — stops L4 VM + drain
POST /v1/flow/kill/express  { "closed": true }  — stops A100 VM + express lane
POST /v1/flow/kill          { "closed": true }  — everything stops
POST /v1/flow/kill/tier-c  { "closed": true }  — external API off
```

**Implementation:** `Arc<AtomicBool>` per label (lock-free, hot toggle).
File-persisted on change (`$SLM_DATA_DIR/flow-gate-{label}.lock`).
inotify watcher keeps AtomicBool in sync if toggled from shell.
State survives service-slm restart.

**When gate is CLOSED:**
- Queue grows naturally — nothing dropped, nothing dispatched
- VM stays stopped — $0 billing
- Express lane returns 503 Retry-After
- service-content files accumulate without extraction storms
- Opening the gate triggers VM start + drain

---

## §7 — Express Lane (202 Accepted Pattern)

```
POST /v1/express/chat      immediate chat (yoyo_label required)
POST /v1/express/extract   immediate DataGraph extraction
POST /v1/express/graph     immediate graph mutation
```

**Routing:**
- Kill switch CLOSED → 503 Retry-After (no bypass)
- YoYo Available → dispatch immediately (in-memory oneshot, no queue file)
- YoYo Stopped → 202 Accepted + `Location: /v1/status/{req_id}`; VM boots; client polls
- YoYo FailedStart/Zombie → Tier A fallback (chat only); extract → 503

**Why 202 not hold:** client timeouts (typically 30s) would drop the connection while
the A100 boots (2-3 min), leaving the VM running for nothing. 202 + polling is the
industry-standard pattern for cold-resource warmup.

**Concurrency (semaphore per label):**
- `express` (A100): 4 slots default — tunable: `PUT /v1/flow/slots/express`
- `batch` (L4): 2 slots default

---

## §8 — Priority Queue (Interleaved Drain — No FIFO Starvation)

Three file-backed queue classes, drained in interleaved cycles:

```
queue/p0-local/    Tier A only (background classify, no GPU)
queue/p1-batch/    L4 batch YoYo, background DataGraph extraction
queue/p2-brief/    apprenticeship briefs, DPO, training corpus
```

**Drain cycle:** 1×P0 → 1×P1 → 1×P2 → repeat. Empty level = skip.

**Tier affinity (hard — no silent fallback):**
- DataGraph extraction: Tier B only — 7B cannot produce grammar-constrained output
- Chat/classify: Tier A acceptable
- Apprenticeship briefs: Tier B for DPO; Tier A for classification

**Queue file format (backward compatible — must not change):**
```json
{ "brief": { "brief_id", "task_type", "scope", "body", "senior_identity", ... },
  "actual_diff": "string" }
```

**Prior fixes deployed (all confirmed in code):**
- Sprint 3A: `SLM_TIER_A_FIRST=true` — Tier A primary ✓
- Sprint 3B: WATCHER Tier A fallback (rate-limited, 300s interval) ✓
- Sprint 3C: drain worker pauses when Tier B open >1h ✓
- Sprint 2C: degenerate tuple guard at write time ✓
- `SLM_QUEUE_LEASE_EXPIRY_SEC=2100` (> drain wrapper 1860s timeout) ✓

---

## §9 — Yo-Yo Infrastructure (Burst GPU Nodes)

**History:**
1. `yoyo-tier-b-1` (GCE Spot, g2-standard-4, L4) — TERMINATED; europe-west4-a stockout
2. `yoyo-tier-b` (Cloud Run, Ollama 0.24.0, L4) — DELETED 2026-06-04; unpredictable billing

**New plan (two GCE VMs, stopped when not in use):**

| Label | VM | GPU | Use | Cost |
|---|---|---|---|---|
| `batch` | yoyo-batch, g2-standard-4, us-central1-a | L4 24GB | daily drain, corpus, extraction | $0.71/hr on-demand |
| `express` | yoyo-express, a2-highgpu-1g, us-central1-a | A100 40GB | urgent, interactive, DataGraph | $3.67/hr on-demand |

**Stopped VM costs only the boot disk (~$2/month). $0 when not running.**

**Machine-type upgrade path (no reprovision needed):**
```bash
gcloud compute instances stop yoyo-express --zone=us-central1-a
gcloud compute instances set-machine-type yoyo-express \
  --zone=us-central1-a --machine-type=g2-standard-4   # downgrade to L4
gcloud compute instances start yoyo-express --zone=us-central1-a
```

**Cold-start target:** ~2-2.5 min (VM boot ~2 min + model load from local SSD ~15s).
Uses direct `gsutil cp` from GCS bucket, NOT GCS FUSE (FUSE is 10× slower for sequential reads).

**Model:** OLMo 3 32B Think Q3 GGUF (15.6 GiB).
**GCS location:** `gs://woodfine-node-gcp-free-foundry-substrate/ollama-store/blobs/sha256-06c420f9`

**Startup script:**
```bash
gsutil cp gs://woodfine-node-gcp-free-foundry-substrate/ollama-store/blobs/sha256-06c420f9 \
  /tmp/olmo-3-32b-think-q3.gguf
OLLAMA_MODELS=/tmp OLLAMA_KEEP_ALIVE=-1 ollama serve
```

**Critical llama-server flags (verified from §10 prior BRIEF):**
- `-ngl 99` (all layers on GPU)
- `-np 1` NOT `-np 4` (with -c 4096, -np 4 truncates to 1024 tokens/slot)
- `-fa on` NOT bare `-fa` (bare flag crashes)
- `--reasoning-format deepseek`, `--reasoning-budget 1024`

**Zone discipline (us-central1-a):** cheapest region for both L4 and A100.
No zone fallback — if stockout, wait. Cost of waiting = $0.

---

## §9b — DataGraph Access from Claude Code (2026-06-05)

**MCP server wired.** Claude Code sessions in `project-intelligence` now have direct access
to LadybugDB via the `foundry` MCP server defined in `.mcp.json` at the archive root.

**B3 root cause (resolved):** Entities are stored under `module_id="woodfine"` (written by
the extraction pipeline). Prior queries used `module_id="jennifer"` or `"mcp-foundry"` —
both returned `[]`. Fix: `.mcp.json` sets `SLM_MODULE_ID=woodfine`.

**Confirmed live:**
```bash
curl "http://127.0.0.1:9081/v1/graph/context?q=a&module_id=woodfine&limit=3"
# → Woodfine Capital Projects, Woodfine Management Corp., Peter M. Woodfine, ...
```

**6 MCP tools available in Claude Code sessions:**

| Tool | What it does |
|---|---|
| `query_datagraph(q, limit?)` | Search entities by name substring — primary lookup |
| `get_entity_context(entity)` | Convenience wrapper; top-5 hits for an entity name |
| `mutate_datagraph(mutation)` | Create/update entities (proxied through Doorman) |
| `submit_extraction(text, schema)` | Submit prose for entity extraction pipeline |
| `get_corpus_stats()` | Corpus stats + daily cost summary |
| `doorman_health()` | Tier availability, circuit state, entity count |

**Path 1 (ANTHROPIC_BASE_URL automatic injection) — blocked until Sprint 1:**
The shim at `http.rs:1273` (Sprint 0a) strips `tool_use`/`tool_result` content blocks —
enabling it now would break all Claude Code tool calls. Also: `graph_context_enabled: Some(false)`
at ~line 1449 must be removed. Sprint 1 (canonical IR) is the prerequisite.

**User-level wiring (2026-06-05):** `~/.claude.json` `mcpServers` updated with the `foundry`
server entry. All 25 Totebox archives + Command Session now have MCP access automatically —
no per-archive `.mcp.json` approval required.

---

## §9c — ask_local Tool: OLMo 7B from Claude Code (2026-06-05)

**Model confirmed:** OLMo 2 1124 7B Instruct Q4_K_M running at ~3.7 tok/s on Tier A.

**New MCP tool:** `ask_local(prompt, max_tokens?)` added to `slm-mcp-server/src/main.rs`.
Calls `POST /v1/chat/completions` on the Doorman — the same endpoint used by all inference
routing. Doorman auto-injects DataGraph entity context before routing to OLMo. No data
leaves the VM (SYS-ADR-07 compliant).

| Field | Value |
|---|---|
| Endpoint | `POST /v1/chat/completions` on Doorman `:9080` |
| Default max_tokens | 300 (hard cap 400 — ~108 s at 3.7 tok/s) |
| Per-request timeout | 180 s (overrides client-level 120 s) |
| Graph context | Injected automatically by Doorman before inference |
| Cost | $0.00 (local inference) |

**Status:** LIVE — commit `e7460446`, binary installed at `/usr/local/bin/slm-mcp-server`.

**Sprint 1 connection:** Once Path 1 shim is fixed (canonical IR), DataGraph injection and
OLMo routing become automatic for ALL Claude Code queries — not just explicit `ask_local`
calls. `ask_local` will remain useful for explicit local-only invocations.

---

### FAULT LOG — Graph context not reaching OLMo (2026-06-05, unresolved)

**Symptom:** T2 test (`ask_local` prompt: "List the Woodfine companies you know about") — OLMo
invented a UK wood products company instead of citing the actual Woodfine entities in LadybugDB.
Expected: response grounded in `[ENTITY CONTEXT]` injected by the Doorman's `GraphContextClient`.
Actual: hallucinated response with no reference to Woodfine Capital Projects or Woodfine Management Corp.

**What is confirmed working:**
- `service-content` at `:9081` is up and returns correct entities directly:
  `curl "http://127.0.0.1:9081/v1/graph/context?q=woodfine&module_id=woodfine&limit=3"` → 3 entities
- `readyz` and `healthz` both healthy on Doorman at `:9080`
- `ask_local` tool invocation succeeds end-to-end (tier=local, cost=$0.00)

**Hypothesis (most likely):** The Doorman's `GraphContextClient` is calling service-content
but either (a) using `module_id="foundry"` (its internal default) rather than `"woodfine"` — so
the query returns 0 entities — or (b) failing silently and skipping injection (non-fatal by design).
The circuit breaker being OPEN may also be suppressing the graph fetch.

**Investigation steps for next session:**
1. Grep `GraphContextClient` in `slm-doorman/src/` — find where `module_id` is set for the
   graph context fetch call; confirm it uses the request's `X-Foundry-Module-ID`, not a hardcoded value.
2. Add a `journalctl -u local-doorman -n 50` check during a `/v1/chat/completions` call —
   look for graph context fetch log lines and any errors.
3. Check `slm-doorman-server/src/http.rs` chat completions handler — confirm
   `graph_context_enabled` is not being forced to `false` for this code path.
4. If module_id is wrong: fix the `GraphContextClient` call to pass the request's module_id.
5. If circuit breaker is suppressing it: check whether the circuit breaker state (OPEN for Tier B)
   is incorrectly gating the graph context fetch, which routes to Tier A / service-content only.

**Priority:** Medium — `ask_local` is functional; this fault reduces response quality but does
not block the tool. Fix before relying on OLMo for Foundry-specific knowledge tasks.

---

## §10 — MCP Server (No-API-Key Entry Point)

service-slm exposes an MCP server at `:9090` alongside the existing HTTP API at `:9080`.

**How it works (inbound direction):**
1. User has a subscription to an MCP-capable AI client ($20/month plan)
2. User adds service-slm's MCP endpoint as a custom connector in client settings
3. The AI client CALLS service-slm tools — no API key needed in service-slm
4. service-slm provides sovereign organizational memory; the client provides reasoning

This is fully ToS-compliant, production-proven (Zep Graphiti does this for Cursor IDE),
available on Pro plans of major AI clients today.

**MCP tools exposed:**
```
search_graph(query: string)                     → [Entity]
get_entity(name: string)                        → Entity + temporal observations
add_observation(entity_id, fact: string)        → Observation
search_decisions(project?: string)              → [Decision]  (via ontology CSV)
add_entity(name, type, observations[])          → Entity
get_recent_events(hours?: int)                  → [Event]
```

**Write tools (add_observation, add_entity) proxy through Doorman `/v1/graph/mutate`.**
They never write directly to service-content. Hard constraint per SYS-ADR-07.

**Authentication:** OAuth 2.1. Simplest viable implementation: self-issued JWTs signed
with a key stored in the systemd environment. One token per MCP client. Revoke by
rotating the signing key.

**Crate:** `rmcp` (official Model Context Protocol Rust SDK).
**Transport:** SSE/HTTP (remote connections from AI client web apps).

**Generic naming in artifacts:** "organizational memory provider", "knowledge graph
connector" — never reference specific AI client names.

---

## §11 — Four Binary Products

| Binary | Product Name | License | Price | Runs on |
|---|---|---|---|---|
| `slm-doorman-server` | PointSav SLM Router | Apache 2.0 | Free | os-totebox |
| `orchestration-slm-server` | PointSav SLM Orchestration | Proprietary | $99/yr USDC | os-orchestration |
| `service-content` | PointSav DataGraph | Apache 2.0 | Free | os-totebox |
| `app-console-slm` TUI | PointSav SLM Console | Apache 2.0 | Free | os-console |

**Zero code difference** between generic binary and Foundry deployment — configuration only.
Customers configure their own endpoints, ontology CSVs, and API keys via env vars.

**Marketplace:** add 4 entries to `app-privategit-marketplace/catalog/products.yaml`.
**License enforcement (orchestration):** Ed25519 token in `ORCHESTRATION_LICENSE_TOKEN`
env var; verified at startup; 30-day offline grace period.

---

## §12 — app-console-slm

**Phase D (2026-05-31) — confirmed live:**
- ratatui 0.29 + crossterm 0.28
- F9 cartridge: Doorman health, Tier A/B circuit state, entity count, 10s poll + R refresh
- Queries: `/healthz`, `/readyz`, `/v1/contract`

**4 new endpoints needed (build sprint 5):**
```
GET /v1/status/queue    → {p0, p1, p2, done, poison}
GET /v1/status/cost     → {daily_usd, batch_usd, express_usd, tier_c_calls}
GET /v1/status/yoyo     → {batch: {state, latency_ms}, express: {state, latency_ms}}
GET /v1/status/flow     → {policy, batch_closed, express_closed, global_closed}
```

**Target F9 layout:**
```
╭─ F9 — SLM + DataGraph ────────────────────────────────────╮
│  Gateway ● running  Policy: balanced                      │
│  Tier A: ✓  Tier C: ○                                    │
├─ YoYo Fleet ──────────────────────────────────────────────┤
│  batch   (L4)    ● available  145ms  kill: OPEN           │
│  express (A100)  ○ stopped    —      kill: OPEN           │
├─ DataGraph ───────────────────────────────────────────────┤
│  Entities: 7,445  Circuit: closed  Last: 4 min ago        │
├─ Queue ────────────────────────────────────────────────────┤
│  P0: 0   P1: 12   P2: 391   done: 799   poison: 11        │
├─ Cost Today ───────────────────────────────────────────────┤
│  $0.00  batch: $0.00  express: $0.00  Tier C: $0.00       │
╰─ [K]ill  [P]olicy  [G]raph  R=refresh  ?=help ────────────╯
```

---

## §13 — Data Preservation (DO NOT LOSE)

Fresh-sheet rewrite changes `src/` only. All data survives unchanged at the same paths.

| Data | Path | Format |
|---|---|---|
| LadybugDB graph | `service-content/data/jennifer-graph/entities.lbug` + `.wal` | LadybugDB v0.16 |
| Processed ledger | `service-content/data/jennifer-graph/processed_ledgers.jsonl` | plain text |
| Brief queue | `/srv/foundry/data/apprenticeship/queue*/` | JSONL ShadowQueueEntry |
| Training corpus | `/srv/foundry/data/corpus/sft/2026-06-01-train.jsonl` | JSONL |
| Audit ledger | `/srv/foundry/data/audit-ledger/` | JSONL append-only |
| Cost ledger | `/srv/foundry/data/cost-ledger/` | per-day JSONL |
| Ontology CSVs | `service-content/ontology/` | CSV |
| GCS model weights | `gs://woodfine-node-gcp-free-foundry-substrate/ollama-store/blobs/sha256-06c420f9` | GGUF 15.6 GiB |

**Cutover:** set `SLM_DRAIN_PAUSED=true` → wait for queue-in-flight/ to empty →
stop old services → verify data stable → deploy new binaries → start → validate 30 min.

---

## §14 — Architectural Decisions (ADRs)

| ADR | Decision | Rationale |
|---|---|---|
| SYS-ADR-07 | Structured data never crosses external LLM boundary | Auditability + sovereignty |
| ADR-01 | module_id in every graph query + audit entry | Multi-tenant isolation |
| ADR-02 | GraphStore trait (LadybugDB or SQLite) | Swap substrate without calling code change |
| ADR-03 | One DataGraph per Totebox Archive (module_id partitions logical tenants) | Cross-domain reasoning requires one graph |
| ADR-04 | service-bookkeeping writes ontological facts to shared graph; transactional records to service-fs | Graph = ontology; ledger = transactions |
| ADR-05 | OLMo-series models only (no external API for training) | Provenance moat; ToS compliance |
| ADR-06 | Express lane bypasses queue, NOT kill switch | Billing control is inviolable |
| ADR-07 | No zone fallback for GPU stockouts — wait | Cost of waiting = $0; misplaced VM costs real money |
| ADR-08 | Entity types from user-supplied CSVs (SERVICE_CONTENT_ONTOLOGY_DIR), not hardcoded | Generic product; customer-configurable ontology |
| ADR-09 | MCP writes proxy through Doorman (not direct to service-content) | SYS-ADR-07 applies to MCP tools too |
| ADR-10 | Never train on empty rejected DPO samples | Model degradation (arxiv 2506.12725) |

---

## §15 — Open Items

### P0 (Blockers)

- [ ] **Tier B offline** — both Cloud Run deleted and GCE Spot terminated. No GPU available.
  Action: provision `yoyo-batch` (L4) and `yoyo-express` (A100) in us-central1-a (Phase 4).
  Until then: Tier A only, extraction deferred, learning loop stalled.

- [ ] **LoRA training** — disabled pending: (1) GPU available, (2) corpus at 5k+ samples,
  (3) operator approval (`SLM_LORA_AUTO_ENABLE=true`).
  Current corpus: 77 good SFT samples. DPO: 591 degenerate (quarantined).

### P1

- [ ] **Stage 6 promote** — 2+ commits on main ahead of origin/main. Rebase required per
  inbox `command-20260520-stage6-rebase-required`. Then `bin/promote.sh` + `bin/sync-local.sh --all`.

- [ ] **service-content path decoupling** — hardcoded `/srv/foundry/...` paths.
  Replace with `INFRASTRUCTURE_ROOT` + `CORPUS_ROOT` env vars (defaults preserve current).
  ~20 LOC in `service-content/src/main.rs`.

- [ ] **`Requires=` → `Wants=`** in `local-content.service` systemd unit.
  Doorman restart currently silently stops service-content. `Wants=` gives ordering-only.

- [ ] **Degenerate DPO quarantine** — 591 empty-rejected-sample tuples in
  `/srv/foundry/data/corpus/`. Already identified as harmful; not yet formally quarantined.

### P2

- [ ] **Doorman audit ledger sha256** — add `sha256: String` (blake3 hash) to every
  `LedgerEntry`. File: `slm-doorman-server/src/ledger.rs`. ~10 LOC.

- [ ] **readyz structured circuit state** — add `reason: Option<String>` + `zone: Option<String>`
  to Tier B circuit JSON in `/readyz`. ~30 LOC in `slm-doorman-server/src/http.rs`.

- [ ] **orchestration-slm persistence** — replace ephemeral HashMap metering with
  Redb/SQLite. Required for production audit trail. Estimate: 1 session.

- [ ] **LbugGraphStore integration tests** — Sprint 5 tests only cover SqliteGraphStore.
  Deferred until CI has lbug-capable runner.

### P3 / Deferred

- [ ] **Tier C activation** — add `ANTHROPIC_API_KEY` to `local-doorman.env`. Operator decision.
- [ ] **G-items from BRIEF-flow-restructure.md** (G5/G6/G9/G11-G16/G18) — original BRIEF
  lost to Stage-6 rebase 2026-05-22. Items treated as superseded by Session 6 work.
  Command to confirm disposition.
- [ ] **Packer image rebuild** — apply `-np 1` + `-fa on` fixes. Low priority (Cloud Run gone).

---

## §16 — Fresh-Sheet Build Plan

### What is being rebuilt

| Component | Action | Preserve |
|---|---|---|
| service-slm | **Rewrite** fresh src/ | All data; tier router logic; audit substrate |
| service-content | **Rewrite** fresh src/ | All data; existing corpus pipeline + circuit breaker patterns |
| app-orchestration-slm | **Build** from scaffold | Nothing — new component |
| app-console-slm | **Extend** Phase D | Existing ratatui code |

### service-content rewrite clarification

The rewrite produces a clean, generic binary. The ontology CSV system already makes
it configurable — users drop CSV files in `SERVICE_CONTENT_ONTOLOGY_DIR` to define
their entity types. The rewrite does NOT hardcode Decision/Policy/Contract etc.;
those come from user-supplied CSVs.

**What changes in the rewrite:**
- Cleaner src/ directory layout (single crate)
- Single `CorpusQueueEntry` state machine (replaces 3-Vec deferred/circuit/in-flight)
- Generic env var defaults (no hardcoded `/srv/foundry/` paths)
- `Requires=` → `Wants=` in systemd unit
- All extraction calls: `X-Foundry-Priority: p1` header
- `gcp_auth` crate for token refresh (background task, not per-request)

**What does NOT change:**
- LadybugDB schema (backward compatible with 7,445 existing entities)
- Processed ledger format (one filename per line, plain text)
- HTTP API surface (/v1/graph/context, /v1/graph/mutate, /v1/config/*)
- Corpus drain logic and circuit-breaker pattern (proven; preserve the logic)

### service-slm codebase layout (fresh)

```
service-slm/src/
├── main.rs            boots :9080 (HTTP) + :9090 (MCP server)
├── router.rs
├── express_lane.rs    202 pattern, oneshot, semaphores
├── flow_gate.rs       AtomicBool per label, HTTP, file persist, inotify
├── flow_policy.rs     FlowPolicy enum, RwLock, runtime switch
├── priority_queue.rs  p0/p1/p2 interleaved, tier affinity guard
├── vm_lifecycle.rs    7-state FSM: Unknown→Stopped→Staging→Running→Available
├── gcp.rs             REST API, gcp_auth crate
├── tier_a.rs
├── tier_b.rs
├── tier_c.rs          Groq/Together/Batch clients; cost guard
├── cost_ledger.rs     background channel (non-blocking)
├── graph_context.rs   CB-wrapped, multi-hop query
├── mcp/
│   ├── server.rs      MCP SSE :9090, rmcp crate
│   ├── tools.rs       6 tools (search_graph, get_entity, ...)
│   ├── auth.rs        OAuth 2.1 (self-issued JWT)
│   └── resources.rs
└── handler/
    ├── chat.rs        POST /v1/chat/completions
    ├── extract.rs     POST /v1/extract
    ├── express.rs     POST /v1/express/{chat,extract,graph}
    ├── flow.rs        POST /v1/flow/{kill,policy,slots}
    └── status.rs      /healthz /readyz /v1/status/* /v1/status/{req_id}
```

### Sprint plan

| Sprint | Deliverable |
|---|---|
| 1 | `flow_gate.rs` + `flow_policy.rs` + `priority_queue.rs` (interleaved drain) |
| 2 | `vm_lifecycle.rs` + `gcp.rs` + `express_lane.rs` (202 pattern) |
| 3 | `app-orchestration-slm` rebuilt: fleet + shared CB + priority router + license.rs |
| 4 | `mcp/` module: server, tools, auth — the no-API-key Claude connector |
| 5 | app-console-slm F9 expansion + service-content rewrite + corpus fixes |

---

## §17 — Documentation Map

**TOPICs → media-knowledge-documentation/ (bilingual, generic terms):**
- `substrate/soft-slm-tiered-gateway.md` + `.es.md` — product architecture
- `substrate/ontological-datagraph.md` + `.es.md` — graph, entity types, multi-hop
- `substrate/yoyo-batch-gcp-substrate.md` + `.es.md` — L4 batch node
- `substrate/yoyo-express-a100-substrate.md` + `.es.md` — A100 express node
- `applications/app-console-slm.md` + `.es.md` — TUI monitoring console
- Archive `slm-tiered-substrate.md` → `superseded_by: soft-slm-tiered-gateway`

**GUIDEs → woodfine-fleet-deployment/cluster-intelligence/ (EN-only, vendor-specific OK):**
- `guide-slm-tier-a-local-deploy.md`
- `guide-datagraph-ontology-setup.md`
- `guide-tier-b-batch-gcp-deploy.md`
- `guide-tier-b-express-a100-deploy.md`
- `guide-orchestration-slm-deploy.md`
- `guide-slm-service-content-wiring.md`
- `guide-console-slm-deploy.md`

**Existing drafts to commit:** A37 (TOPIC-yoyo-cloud-run-substrate) + A38 (GUIDE-yoyo-cloud-run).
Split A37 into generic TOPIC + deployment GUIDE before staging to project-editorial.

---

## §18 — 10 Verification Tests

1. Kill switch: `POST /v1/flow/kill {"closed":true}` → VMs stop; express → 503
2. Express (VM available): POST → immediate response, no 202
3. Express (VM stopped): POST → 202; poll /v1/status/{id} → eventually 200; VM auto-started
4. Priority: 10 P2 briefs queued → P0 request → P0 served before any P2 drained
5. Interleave: P0+P1+P2 in queue → drain log shows 1/1/1 cycle
6. drain-batch policy: A100 stays off; L4 drains all
7. local-only policy: both VMs stop; extract → 503; chat → Tier A
8. DataGraph E2E: service-content extraction → entity in graph → next /v1/chat gets context
9. Multi-hop: query returns chain traversal (requires Tier 1 ontology additions via CSV)
10. Console: F9 shows both YoYo states, queue P0/P1/P2 depth, cost-today, entity count

---

## §19 — Reference Documents

| Document | What it covers |
|---|---|
| `service-slm/ARCHITECTURE.md` | Three-ring model, Doorman protocol, tier routing, audit ledger |
| `service-content/CLAUDE.md` | Feature table, env vars, build commands |
| `service-content/ARCHITECTURE.md` | Phase 3 rebuild blueprint (MCP, SQLite, vector, temporal) |
| `app-orchestration-slm/CLAUDE.md` | Stateless chassis, endpoints, commercial model |
| `service-slm/NEXT.md` | Session-specific open items |
| `service-slm/docs/deploy/deploy-yoyo-tier-b.md` | Cloud Run runbook (archived; superseded) |
| `/home/mathew/.claude/plans/we-need-way-to-inherited-squirrel.md` | Full session plan |

**Key env vars:**

| Var | Service | Default | Purpose |
|---|---|---|---|
| `SLM_LOCAL_ENDPOINT` | service-slm | `http://127.0.0.1:8080` | Tier A local model |
| `SLM_YOYO_BATCH_ENDPOINT` | service-slm | — | L4 batch YoYo |
| `SLM_YOYO_EXPRESS_ENDPOINT` | service-slm | — | A100 express YoYo |
| `SLM_ORCHESTRATION_ENDPOINT` | service-slm | `http://127.0.0.1:9180` | Chassis |
| `SLM_TIER_A_FIRST` | service-slm | `true` | Tier A as primary |
| `SLM_DRAIN_PAUSED` | service-slm | `false` | Pause queue drain |
| `SLM_QUEUE_LEASE_EXPIRY_SEC` | service-slm | `2100` | > drain wrapper 1860s |
| `SERVICE_CONTENT_ENDPOINT` | service-slm | `http://127.0.0.1:9081` | Graph context |
| `SERVICE_CONTENT_MODULE_ID` | service-content | `woodfine` | Tenant scope |
| `SERVICE_CONTENT_ONTOLOGY_DIR` | service-content | `./ontology` | CSV entity types |
| `SERVICE_CONTENT_GRAPH_DIR` | service-content | `/var/lib/local-content/graph` | LadybugDB location |
| `SERVICE_CONTENT_LBUG_BUFFER_POOL_MB` | service-content | `2048` | Graph buffer pool |
| `SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED` | service-content | `false` | Rate-limited Tier A extraction |
| `ORCHESTRATION_LICENSE_TOKEN` | app-orchestration-slm | required | Ed25519 license check |
