# Sovereign Stack Master Plan — 2026

> Authored: 2026-05-14 task@project-intelligence (three parallel Opus deep-thinks)
> Status: Active — primary planning reference for this archive
> Last crash-recovery session: 2026-05-14 (VM reset after 8.5h retry storm; fixes committed)

This is the single entry point. Detailed analysis lives in companion docs.

---

## Companion documents

| Plan | Contents |
|---|---|
| `tier-architecture-2026.md` | Tier A/B/C model selection, BCSC permissible families, own-model gap analysis, mistralrs timing |
| `service-content-architecture-2026.md` | Ring 2 architecture defects, PUSH inversion roadmap, graph store gaps, HTTP surface |
| `service-slm-architecture-2026.md` | Routing paths, audit ledger, Sprint 0a prerequisites, "never generate text" audit |
| `leapfrog-2026.md` | Full strategic deep-think: thesis validation, capability gaps, compound loop, 2026–2030 model trajectory, product vision |
| `universal-ai-gateway.md` | Sprint-level implementation plan for Sprint 0a (Anthropic shim) and beyond |

---

## Context: what happened on 2026-05-14

**The VM crash (prior boot, 2026-05-13T17:39 → 2026-05-14T02:06):**
- Root cause: service-content calling `/v1/chat/completions` (Tier A fallback allowed) for entity extraction
- OLMo 1B cannot produce structured JSON; Doorman returned HTTP 200 with malformed body
- Watcher loop retried immediately (no backoff for 200-with-bad-JSON path)
- 8.5 hours of tight loop → KV-cache grew to 6.2GB (ctx=8192 × extraction prompts) → virtio_balloon memory exhaustion → manual reset

**Fixes committed this session:**
- `service-content/src/main.rs` → switched to `POST /v1/extract` (Doorman's `route_yoyo_only()`, no Tier A fallback). Commit `832db9c`.
- `/srv/foundry/infrastructure/local-slm/local-slm.service` → `--ctx-size 4096`, removed `--reasoning-format deepseek`, `MemoryMax=3G`. Commit `9d6532c`.
- `local-doorman.service` → `SLM_LOCAL_MODEL=OLMo-2-0425-1B-Instruct-Q4_K_M.gguf` (was wrong 7B Think). Commit `1a720c2`.
- `service-slm/CLAUDE.md` → corrected Tier A model name, apprenticeship state.

**Memory state restored:** free 6.7Gi, available 11Gi, swap 196Mi. 114 CORPUS files defer gracefully in ~11s.

---

## Architecture overview

```
Ring 1: WORM Ingest (os-totebox substrate)
  service-fs / service-people / service-email → CORPUS_*.json files

Ring 2: Knowledge & Processing (deterministic, no AI required)
  service-content → watches CORPUS_*.json
                  → extracts entities via Ring 3 (optional enrichment)
                  → LadybugDB knowledge graph (:9081)

Ring 3: Optional Intelligence (service-slm Doorman, :9080)
  Tier A  local OLMo 2 1B Instruct Q4, always-on, :8080
          tasks: read/summarise/grep (current; upgrade to 7B pending)
  Tier B  Yo-Yo L4 GPU, on-demand, OLMo 3 32B Think
          tasks: entity extraction, moderate code edits
  Tier C  Anthropic API passthrough
          tasks: complex debugging, architecture decisions, multi-step chains

Community tier = Rings 1+2 (AGPL, free, no AI compute required)
SMB tier       = Rings 1+2+3 (FSL commercial)
```

**The three-ring principle:** Ring 3 is structurally optional. Rings 1+2 must work without it. Currently violated: service-content's `process_corpus()` halts when Doorman is unavailable.

---

## Decisions ratified this session

| Decision | Rationale | Ref |
|---|---|---|
| OLMo family (AI2) required for Tier A base and own-model | Only open-training-data family; required for BCSC posture and fine-tune provenance | tier-architecture-2026.md §5 |
| Qwen / DeepSeek / Yi / GLM excluded | PRC-headquartered; cannot sustain NI 51-102 Sovereign Data Foundation disclosure | tier-architecture-2026.md §5 |
| Anthropic Tier C: permissible | US PBC; SYS-ADR-07 sanitise step ensures structured data never crosses boundary | tier-architecture-2026.md §5 |
| Llama / Mistral / Gemma: process-only | Open weights, closed training data; permissible for Tier B "graph" backend (audit-flagged); not for Tier A or own-model base | tier-architecture-2026.md §5 |
| Tier A upgrade to OLMo 2 1124 7B Instruct (pending) | 1B cannot produce flat-schema tool-call args; 7B required for Sprint 0a haiku-tier routing | tier-architecture-2026.md §2 |
| mistralrs-server migration: defer to Sprint 1.5 | Trigger = first LoRA needs runtime hot-swap; premature migration adds risk without benefit | tier-architecture-2026.md §3 |
| OLMo 3 32B Think for Tier B "trainer": keep | L4 GPU rental cost is dominated by instance-hour not model size; 7B saves nothing, loses quality | tier-architecture-2026.md §4 |
| Ring 2→Ring 3 PUSH inversion: target architecture | Eliminates Ring 2 dependency on Ring 3; `/v1/graph/mutate` already exists; migration is incremental | service-content-architecture-2026.md §8 |
| `graph_context_enabled` gate on `ComputeRequest`: required for Sprint 0a | Without it, DataGraph entities inject into every Claude Code request | service-slm-architecture-2026.md §6 |
| Opus → Tier C path in Sprint 0a: use Path B (Tier B fallback) | `FOUNDRY_DEFAULT_ALLOWLIST` incompatible with general purpose; fix in Sprint 0b | service-slm-architecture-2026.md §6 |
| The moat is substrate + jurisdiction, not the fine-tuned model | Hyperscalers can match models; cannot match WORM + BCSC + customer-controlled hardware + own-corpus fine-tune | leapfrog-2026.md §0 |

---

## Ordered action plan

### Tier 1 — Sprint 0a prerequisites (do first, ~1 week)

**1a. `graph_context_enabled` gate** [BLOCKS Sprint 0a shipping]
- Add `pub graph_context_enabled: Option<bool>` to `ComputeRequest` in `slm-core/src/lib.rs`
- Gate graph-context injection at `slm-doorman/src/router.rs:125` on `req.graph_context_enabled.unwrap_or(true)`
- Shim handler sets `Some(false)` on every `/v1/messages` request

**1b. Apprenticeship flag drift reconciliation**
- On-VM `local-doorman.service` has `SLM_APPRENTICESHIP_ENABLED=true` (intentional since v0.1.39)
- Clone `compute/systemd/slm-doorman.service:37` still says `false`
- Propagate `true` to the clone unit file + update CLAUDE.md (done this session) 

**1c. Implement Sprint 0a: `POST /v1/messages`** (~305 LOC)
- See `universal-ai-gateway.md` for full implementation spec
- Model routing: `claude-haiku-*` → Tier A, `claude-sonnet-*` → Tier B "trainer", `claude-opus-*` → Tier B (Sprint 0a) → Tier C (Sprint 0b)
- Conversion: `AnthropicMessagesBody → ComputeRequest`; system prompt prepend; SSE streaming path
- Set `graph_context_enabled: Some(false)` on all shim requests

### Tier 2 — Close the compounding loop (next 30 days)

**2a. Git post-commit hook** (~50 LOC) [corpus capture]
- Wire `actual_diff` in `/v1/shadow` (`slm-doorman-server/src/http.rs:349-358`)
- Mechanism: git post-commit hook invokes `POST /v1/shadow` with `{brief_id, actual_diff}`
- Every accepted commit becomes a training pair from this point forward

**2b. Ratify `conventions/permissible-model-substrate.md`** (~1 week writing)
- BCSC posture policy; OLMo-only-for-Tier-A rule; upgrade trigger criteria; annual Q1 review
- Must exist before any further model changes
- See tier-architecture-2026.md §7 for required sections

**2c. Tier A upgrade to OLMo 2 1124 7B Instruct**
- Download weights to `/var/lib/local-slm/weights/OLMo-2-1124-7B-Instruct-Q4_K_M.gguf`
- Update `local-slm.service`: model path + `MemoryMax=6G`
- Deploy + verify RSS stays under 6G alongside other services
- Update `SLM_LOCAL_MODEL` in `local-doorman.service` to match

### Tier 3 — Production hardening (next 60 days)

**3a. Persistent extraction queue** (replaces per-boot retry)
- Disk-backed set for `processed_ledgers` (sidecar JSONL or SQLite)
- Yo-Yo-up notification from Doorman triggers retry of deferred files
- Eliminates the boot-time extraction storm by construction

**3b. Corpus quality gate + PII scrub** (~150 LOC)
- Min brief length 50 chars, min diff size 1 LOC
- Dedup by `(brief_hash, diff_hash)`
- PII scrub via existing `sanitize` path
- Required BEFORE first LoRA training run

**3c. Eval harness for Tier A and Tier B** (~200 LOC + held-out set)
- Held-out eval set (90/10 split from existing corpus)
- Eval runs before each adapter deployment
- Task classes: file summarisation, entity extraction, flat-schema tool-call args, KG query
- Required BEFORE first LoRA training run

### Tier 4 — Ring 2 structural repairs (parallel, 60–90 days)

**4a. Sprint 1 (service-content): deterministic Source node write** (~30 LOC)
- Before calling `POST /v1/extract`, write a Source node: `{worm_id, module_id, received_at}`
- Graph grows regardless of Ring 3 reachability — fixes Community Tier principle
- File: `service-content/src/main.rs` in `process_corpus()` before line 198

**4b. Validate module_id; reject `__` prefix**
- `main.rs:167-170` (per-file override) and `http.rs:99-108` (mutate endpoint)
- Pattern: `[a-z0-9-]{1,64}`, reject `__` prefix
- Blocks taxonomy namespace corruption via malformed CORPUS file

**4c. Wire `RelatedTo` edges in graph store**
- `graph.rs:66-72` declares the table; it is never populated
- Extraction schema must ask the model for edges, not just nodes
- `upsert_entities` becomes `apply_mutation(entities, relations)`

**4d. `ExtractionAuditEntry` missing fields**
- Add `model: String`, `cost_usd: f64`, `sanitised_outbound: bool` to `ledger.rs:286-309`
- Populate from `ComputeResponse` before write in `http.rs:573-585`

**4e. Fix critical `unwrap()` calls in service-content**
- `main.rs:293`: `fs::write(...).unwrap()` → match + log on disk-full/permission error
- `main.rs:47,48,53`: startup dir creation → graceful exit with log

### Tier 5 — Leapfrog compound loop (months 3–9)

**5a. First LoRA training run on Yo-Yo #1**
- Prerequisites: 3a + 3b + 3c + 2a all complete
- OLMo 3 32B Think as base; `engineering-pointsav` adapter
- Validate against eval harness; gate deployment on regression test ≥98% current score

**5b. mistralrs-server migration (Tier A engine swap)**
- Trigger: first LoRA needs runtime hot-swap
- LoRA hot-swap is first-class in mistralrs-server; fragile in llama.cpp
- Also enables native `/v1/messages` at the inference engine layer

**5c. Adapter versioning + signing (Ring 3b)**
- GCS-archived, Sigstore-signed (as per ARCHITECTURE.md §Ring 3b)
- Required to meet SYS-ADR-19 at the moment a customer instance loads a sovereign adapter

### Tier 6 — Product surface (months 6–14)

**6a. Sprint 4: app-console-slm code mode**
- The product surface customers interact with
- Builds on Sprint 0a (shim) + Sprint 1 (canonical IR) + Sprint 2 (native Anthropic Tier C)
- See `universal-ai-gateway.md` §Sprint 4

---

## Current service state (as of 2026-05-14 post-crash-recovery)

| Service | State | Notes |
|---|---|---|
| `local-slm.service` | Running | OLMo 2 0425 1B Q4, ctx=4096, RSS ~2GB |
| `local-doorman.service` | Running | `SLM_LOCAL_MODEL` corrected this session; apprenticeship enabled |
| `local-content.service` | Running | 114 CORPUS files deferring gracefully to Tier B |
| Yo-Yo #1 ("trainer", L4) | TERMINATED | vllm 0.12 issue; inbox item 7 — mask vllm, enable llama-server |
| Yo-Yo #2 ("graph", H100) | Not provisioned | Code-complete; deployment after Yo-Yo #1 is stable |

---

## Key defects summary (by service)

### service-content (full list: `service-content-architecture-2026.md`)
| Priority | Defect |
|---|---|
| Critical | Ring 2 ingest halts when Ring 3 unavailable (no deterministic pass) |
| Critical | `processed_ledgers` RAM-only Vec; 114-file retry storm on boot |
| Critical | No module_id validation — taxonomy namespace injection possible |
| Critical | `RelatedTo` edges never populated — graph is node-only |
| Critical | `main.rs:293` unwrap panics on disk-full |
| High | `/healthz` always returns OK; cannot detect broken state |
| High | `/v1/draft/generate` is Ring 2 calling Ring 3 for text generation — violation |
| Medium | 250ms fixed debounce silently drops slow-written files |

### service-slm (full list: `service-slm-architecture-2026.md`)
| Priority | Defect |
|---|---|
| High | `graph_context_enabled` missing — blocks Sprint 0a |
| High | Apprenticeship flag drift (clone vs on-VM) |
| Medium | `ExtractionAuditEntry` missing `model`, `cost_usd`, `sanitised_outbound` |
| Medium | `"graph-query"` not in `AUDIT_CAPTURE_VALID_EVENT_TYPES` but written by graph proxy |
| Low | `route_yoyo_only()` has no audit write — footgun for future callers |
| Low | `route_async()` dead code — orchestrator never set |

---

## BCSC permissible model families (authoritative)

| Family | Maker | BCSC Posture | Sovereign Roadmap |
|---|---|---|---|
| OLMo 2/3 | AI2 (US 501c3) | **Permissible** | **Required** — Tier A + own-model base |
| Anthropic Claude | Anthropic (US PBC) | **Permissible** | Tier C passthrough only |
| Llama 3/4 | Meta (US) | Permissible w/ disclosure | Process-only (Tier B "graph") |
| Mistral | Mistral AI (FR) | Permissible w/ disclosure | Process-only |
| Gemma | Google (US) | Permissible w/ disclosure (check use terms) | Avoid |
| Qwen | Alibaba (CN) | **Excluded** | Excluded |
| DeepSeek | DeepSeek (CN) | **Excluded** | Excluded |
| Yi | 01.AI (CN) | **Excluded** | Excluded |
| GLM | Zhipu (CN) | **Excluded** | Excluded |

*Principle: compute substrate sourced exclusively from jurisdictions whose data-export and AI-governance regimes are reciprocally aligned with Canadian NI 51-102 disclosure expectations.*

---

## Strategic one-liner

> The moat is the WORM substrate + BC jurisdiction + audit trail + customer-fine-tuned model trained on customer data inside customer infrastructure — not the model itself. Hyperscalers can match models; they cannot match structural sovereignty. Ship Sprint 0a, wire the post-commit hook, ratify the permissible-model convention. Everything else follows.
