# Learning Loop Master Plan — service-slm + service-content
> **Authored:** 2026-05-18 totebox@claude-code (project-intelligence)
> **Synthesis of:** 10-agent parallel audit consolidating MASTER-PLAN-2026.md, leapfrog-2026.md, service-slm-architecture-2026.md, service-content-architecture-2026.md, tier-architecture-2026.md, sovereign-routing-comprehensive.md, universal-ai-gateway.md, service-audit-2026-05-16.md, service-slm-hardening-2026-05-18.md, d5-canonical-message-sprint1.md.
> **Supersedes:** the eight plans above as the operative TODO list. Those plans remain authoritative for design rationale; this plan is the execution sequence.

---

## Executive summary

**The learning flow is built but DARK.** Capture is wired (866 engineering + 495 apprenticeship tuples on disk), but the loop has never closed:

- Zero signed verdicts. F12 substrate dormant. All 495 apprenticeship tuples stuck at `verdict:null`.
- `/v1/messages` Anthropic shim routes Tier C but does NOT enqueue any corpus tuple — Claude Code sessions through the Doorman produce no learning signal today.
- No LoRA training pipeline binaries on disk (`bin/export-dpo.sh`, `bin/lora-update.sh` don't exist). Markers accumulate in `data/training-pending/` with no consumer.
- No eval harness, no held-out set, no adapter-version tagging. Adapter promotion is undefined behaviour.
- No code-level Tier-C exclusion guard. The structural invariant in `pick_tier_for_brief` is one-line-deletable. Anthropic ToS gap.
- GCP billing budget alerts DISABLED on `woodfine-node-gcp-free`. No cloud-side spend cap.

**Cost ceiling target: $500/mo all-in** (operator-confirmable).

---

## Q1 — Resources needed on local VM

**Decision: stay CPU-only on workspace VM; offload all GPU work to cloud burst.**

| Component | Decision | Rationale |
|---|---|---|
| Tier A inference | Keep 7B on CPU (current 6.3/7G RAM, --parallel 1) | 13B Q4_K_M needs ~10G; doesn't fit. Local GPU reshape costs $500-900/mo, kills the economics. |
| Tier B inference | Keep Yo-Yo L4 cloud burst (~$50-150/mo current cadence) | Already working, idle-monitor + RestartBudget + watchdog all hardened. |
| LoRA training | Provision **separate** nightly training Yo-Yo, preemptible L4, ~$15-40/mo | Don't run training on the inference Yo-Yo — would block production. |
| Embeddings | Run as second model slot on Yo-Yo trainer (NOT workspace VM) | `nomic-embed-text-v1.5` (US, Apache-2.0). RAM-impossible on workspace VM. **Reject `bge-small`** (BAAI/CN) per BCSC posture. |
| Vector index | **sqlite-vec** or **redb + `instant-distance` HNSW crate** | Avoid new daemon (Qdrant = +300 MB RAM). Stack is already SQLite-heavy. ~400 LOC service-content extension. |
| Disk | Free ~1.5 GB via `journalctl --vacuum-size=500M` + `service-extraction/target` prune | Currently 72% used; embeddings index pushes us toward 90%. |

**Workspace VM specs (current):** GCE n2-standard-4, 4 vCPU, 15 GiB RAM, 77 GiB disk, no GPU. **Recommendation: do not resize.** If we ever need more headroom, n2-standard-8 (16 vCPU / 32 GiB, ~$280/mo, +$130/mo) is the next step — but only after the loop closes and we've measured what's actually bound by RAM.

---

## Q2 — Spending hardening (target ceiling: $300/mo with Tier C off)

> **Operator directive 2026-05-18:** stay away from the Claude API key
> for now; stick with Claude Pro Max 20x for Claude usage due to cost.
> Implication: **Tier C remains UNCONFIGURED in production**
> (`SLM_TIER_C_ANTHROPIC_API_KEY` UNSET; 503 failsafe is correct). The
> Anthropic API spend vector is eliminated for now. Pro Max sessions
> do NOT transit Doorman — learning-loop capture for those sessions
> comes from the git post-commit hook (`bin/capture-edit.py`), not the
> `/v1/messages` shim.

| Layer | Current control | Gap | New control |
|---|---|---|---|
| Yo-Yo runtime | Optional `--runtime=Nh` watchdog (SCRIPT_DIR bug fixed) | nightly-run.sh invokes without `--runtime` | Default `--runtime=14h` in `nightly-run.sh` and SLM_YOYO_AUTO_START spawns |
| Yo-Yo idle | 30-min idle threshold + 5-min poll + crash-guard | RestartBudget cap 3/hr × $0.40 = $1.20/hr sustained ≈ **$300/mo** thrash | Lower `SLM_YOYO_MAX_RESTARTS_PER_HOUR=2`; widen window via new `SLM_YOYO_RESTART_WINDOW_SEC=14400` |
| Tier C key | **Intentionally UNSET per operator directive** — 503 failsafe correct | Tier-C provenance guard needed in case it's ever re-enabled | Code-level guard shipped 2026-05-18 (this session): `source_tier=external` rejected at `/v1/shadow`; `Tier::External` attempt rejected at `write_shadow_tuple`; `tier_used` promoted to top-level JSONL field |
| Tool-loop | None | If Tier A/B routing ever exposes a runaway loop | Lower priority now that Tier C is off; deferred to P3 |
| GCP billing | **Billing Budget API DISABLED** on project 369270631281 | No cloud-side safety net at all | **P0: enable Billing Budget API; create $300/mo budget; 50/80/100% alerts; 100% triggers Cloud Function that stops `yoyo-tier-b-1`.** Tier-C key rotation step is moot — no key to rotate. Requires operator's laptop with `roles/billing.admin` (compute SA does not have it). |
| Workspace token spend | BUDGET.md exists; `bin/watcher.sh` budget.jsonl referenced but not wired | Audit-log `cost_usd` not ingested | Wire `bin/watcher.sh` to ingest Doorman audit-log cost into budget.jsonl (daily $50 / weekly $250 / monthly $1000 — Tier A/B only) |

**Worst-case ceiling after hardening (Tier C off):** $200 Yo-Yo Spot + $100 workspace baseline = **$300/mo**. Claude Pro Max 20x sub is a separate operator-side line item, not on the Doorman path.

**If Tier C is ever re-enabled later:** code-level `tier_used` top-level JSONL field + `source_tier=external` 403 at `/v1/shadow` + `Tier::External` rejection in `write_shadow_tuple` are already in place (shipped 2026-05-18). Anthropic Console spend cap + per-day spend ledger + tool-loop counter would still be required at that point — kept in P3 as deferred items.

---

## The learning flow — what's missing in one diagram

```
┌─ AUTHORED ──────────────────────────────────────────────────────────────┐
│  Developer in Claude Code session (ANTHROPIC_BASE_URL=doorman)          │
│   ↓ /v1/messages                                                        │
│   ↓                                                                     │
│   ▶ GAP: shim does NOT call enqueue_shadow() — turns are not captured   │
│   ↓                                                                     │
│  Tier A/B/C routing  →  response  +  x-foundry-tier-used                │
│                                                                         │
│  git commit → post-commit hook (capture-edit.py — already installed)    │
│   ↓                                                                     │
│  ShadowQueueEntry{brief, actual_diff} → data/apprenticeship/queue/      │
└─────────────────────────────────────────────────────────────────────────┘
                                  ↓
┌─ DRAIN (CURRENTLY PAUSED) ──────────────────────────────────────────────┐
│  SLM_APPRENTICESHIP_ENABLED=false today (Doorman not redeployed)        │
│   ↓ once re-enabled                                                     │
│  Drain worker → dispatch_shadow → write_shadow_tuple                    │
│   ▶ GAP: no Tier-C provenance check on actual_diff                      │
│   ▶ GAP: no quality gate (min length, dedup, BCSC scan)                 │
│   ↓                                                                     │
│  data/training-corpus/apprenticeship/<task>/shadow-*.jsonl              │
│  (verdict: null, stage_at_capture: review — 495 stuck here)             │
└─────────────────────────────────────────────────────────────────────────┘
                                  ↓
┌─ HUMAN GATE (NOT IMPLEMENTED) ──────────────────────────────────────────┐
│  ▶ GAP: F12 manual gate per SYS-ADR-10                                  │
│  ssh-signed verdict batch → promote_corpus_tuple                        │
│  → DPO pair lands in data/training-corpus/feedback/ (currently EMPTY)   │
└─────────────────────────────────────────────────────────────────────────┘
                                  ↓
┌─ TRAIN (NOT WIRED) ─────────────────────────────────────────────────────┐
│  ▶ GAP: no bin/export-dpo.sh                                            │
│  ▶ GAP: no bin/corpus-threshold.py (referenced; not on disk)            │
│  ▶ GAP: no nightly-run.timer enabled                                    │
│  ▶ GAP: no Yo-Yo training image provisioned                             │
│  Unsloth DPOTrainer / QLoRA NF4 → data/lora/coding-lora-<date>/         │
└─────────────────────────────────────────────────────────────────────────┘
                                  ↓
┌─ EVAL + PROMOTE (NOT WIRED) ────────────────────────────────────────────┐
│  ▶ GAP: no held-out eval set                                            │
│  ▶ GAP: no bin/eval-adapter.sh                                          │
│  ▶ GAP: no Sigstore signing                                             │
│  ▶ GAP: no adapter version registry                                     │
└─────────────────────────────────────────────────────────────────────────┘
                                  ↓
┌─ SERVE (RUNTIME NOT WIRED) ─────────────────────────────────────────────┐
│  ▶ GAP: llama-server `--lora` hot-swap path                             │
│  ▶ GAP: ComputeRequest carries no `adapter_version`                     │
│  Adapter served → response →  loop closes                               │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 0 — Containment (this week, ~2 days)

**Goal:** stop bleeding, close legal/cost gaps, redeploy with the drain fix.

### P0-0.1 — Doorman redeploy + apprenticeship on
- Stage 6 promote pending commits `561b74ce`, `ae653cdb`, `c67bb284`, `9915eddf`, `e365e10e`, `c8c8e1bb`.
- Rebuild `slm-doorman-server`, install to `/usr/local/bin/local-doorman`, flip `SLM_APPRENTICESHIP_ENABLED=true`, restart, drain 27 paused/pending briefs.
- **Owner:** Command Session. **Effort:** 15 min. **Reference:** `service-slm-hardening-2026-05-18.md` Task 2.

### P0-0.2 — ~~Anthropic Console spend cap~~ — **DEFERRED** per operator directive
- Operator directive 2026-05-18: no Commercial API key in production for now; rely on Claude Pro Max 20x.
- Re-open this item if/when Tier C is ever wired into Doorman.

### P0-0.3 — GCP Billing Budget alerts
- Operator from a laptop with `roles/billing.admin` on `0169E0-25F3AE-A5F545`: enable Billing Budget API; create **$300/mo budget** (down from $500 — Tier C off); 50/80/100% email + Pub/Sub alerts.
- 100% trigger: Cloud Function stops `yoyo-tier-b-1`. (Tier-C key rotation step is moot — no key to rotate.)
- **Owner:** operator. **Effort:** 2 hr. **Cannot be done from VM** (Compute SA lacks billing.admin).

### P0-0.4 — Tier-C provenance guard (code, defense-in-depth)
- Add `source_tier: Option<String>` to `ShadowWireBody` + `ShadowQueueEntry`.
- `shadow_handler` returns 422 if `source_tier == "external"`.
- Add early-return guard in `write_shadow_tuple` (`apprenticeship.rs:299`) if `attempt.tier == Tier::External`.
- Add `tier_used` as top-level field on tuple JSONL (currently nested) so `jq 'select(.tier_used=="external")'` is O(n).
- **Effort:** ~50 LOC + tests. **Where:** `slm-doorman/src/apprenticeship.rs`, `slm-doorman-server/src/http.rs`.

### P0-0.5 — `--runtime=14h` default in nightly-run.sh
- One-line addition; backstops idle-monitor failure.
- **Effort:** 5 min. **Where:** `service-slm/scripts/nightly-run.sh:60`.

### P0-0.6 — Disk pressure relief
- `sudo journalctl --vacuum-size=500M` (frees ~800 MB).
- Prune `service-extraction/target/` (frees ~600 MB).
- **Effort:** 5 min.

**Phase 0 exit criteria:** Doorman live with apprenticeship enabled; queue draining; Tier-C contamination structurally impossible AND defended in code; cost ceiling enforced on both sides of the gateway.

---

## Phase 1 — Close the loop (next 2-3 weeks, ~10-15 dev-days)

**Goal:** make the apprenticeship corpus produce its first signed adapter through a working pipeline.

### P1-1.1 — Corpus quality gate (~150 LOC, 2 days)
**Where:** new `slm-doorman/src/corpus_gate.rs` called from `apprenticeship.rs::write_shadow_tuple` and `verdict.rs::write_dpo_pair`.

- Min brief 50 chars; min diff 1 LOC; max diff 1000 LOC.
- Dedup `(sha256(brief.body), sha256(actual_diff))` via JSONL sidecar at `data/training-corpus/.corpus-index.jsonl`.
- Reject empty `actual_diff`.
- BCSC scan: flag (not block) "Sovereign Data Foundation" without `planned|intended|may|target` in same sentence.
- Do-Not-Use scan: regex set from POINTSAV-Project-Instructions.md §5 → reject on match.
- Add `bcsc_scan_passed: bool`, `donotuse_hits: []` fields to tuple.

**Dependency:** project-editorial must ratify the machine-readable Do-Not-Use list. **Send outbox today.**

### P1-1.2 — Hold-out eval set + signing (~50 LOC + curation, 2 days)
- Curate 100 (brief, expected_diff) tuples from existing 866-engineering / 495-apprenticeship corpus.
- Operator SSH-signs `data/training-corpus/eval/holdout-v1.jsonl` (never used in training).
- This is the F12 gate-keeper for adapter promotion.

### P1-1.3 — `bin/eval-adapter.sh` (~100 LOC, 1 day)
- Loads candidate adapter, runs 100-pair held-out set through Tier A.
- Scores via edit-distance + JSON-schema validation against `expected_diff`.
- Emits `data/training-corpus/eval/results/<adapter_version>.json` with `pass_rate`, `regression_vs_baseline`, `promoted: bool`.
- **F12 contract:** `promoted: true` is the ONLY permission to swap production adapter weights.

### P1-1.4 — F12 corpus promotion gate (~80 LOC + bin script, 1 day)
- Drain worker writes to `data/training-corpus/apprenticeship/<task>/_review/shadow-*.jsonl` (review subdir).
- New `bin/promote-corpus.sh`: operator-signed (reuses `VerdictVerifier`); the ONLY path that moves files into `data/training-corpus/apprenticeship/<task>/` proper.
- SYS-ADR-10 compliance closed.

### P1-1.5 — Sign first verdict batch (~1 day operator time)
- Surface 10 high-confidence shadow tuples to operator for review.
- Operator signs verdict batch via `ssh-keygen -Y sign`; appended to `data/apprenticeship/ledger.md`.
- Triggers `verdict.rs` → first DPO pairs land in `data/training-corpus/feedback/`.

### P1-1.6 — Adapter-version tagging (~80 LOC, 1 day)
- Add `adapter_version: Option<String>` to `ComputeRequest`, propagate to `AuditEntry` / `ExtractionAuditEntry`.
- Tier B clients echo `adapter_version` back in `ComputeResponse`.
- Without this, retrospective "did v3 beat v2 on category Y" queries are impossible.

### P1-1.7 — Tool-use round-trip completion (~300 LOC, 4-5 days)
**The real D5 / Sprint 1 completion.** D5 fixed inbound messages only; tools were silently dropped.

**Where:** `slm-core/src/lib.rs`, `slm-doorman/src/tier/{local,yoyo,external}.rs`, `slm-doorman-server/src/http.rs:1395`.

- Add `tools: Option<Vec<ToolDef>>` + `tool_choice` to `ComputeRequest`; thread through all three tier wire bodies.
- Replace `ComputeResponse.content: String` with `content: Vec<ContentBlock>` (deprecate flat string).
- Re-add the `#[ignore]`-removed tool_use integration test; assert assistant `tool_use` block survives.
- **Unblocks Phase 1.9** — without this, Claude Code agent-mode turns lose tool-use signal (most of the agentic value).

### P1-1.8 — Wire `/v1/messages` → enqueue_shadow (~60 LOC, 1 day)
**Lower priority now: Pro Max sessions do NOT transit Doorman, so this path captures only SDK clients that explicitly point at Doorman.** Still valuable for Tier A/B routing telemetry.

- After `anthropic_messages()` returns the response, fire-and-forget `enqueue_shadow(ShadowQueueEntry{brief, actual_diff:"", source_tier: resp.tier_used.as_str().into()})`.
- Gate on `SLM_SHIM_TRAINING_CAPTURE=true`.
- Tier-C contamination guard already in code (shipped 2026-05-18): the new `source_tier` field is propagated end-to-end and the gate at `/v1/shadow` rejects `external` with 403.
- **No legal pre-condition needed for Tier A/B capture** — only Tier C is constrained by Anthropic ToS, and Tier C is off.
- **The actual capture path for Claude Pro Max sessions is the post-commit hook** (`bin/capture-edit.py`, already installed). That's what feeds 866 engineering tuples on disk today.

### P1-1.9 — LoRA training toolchain (~3-4 days)
**Where:** new `bin/export-dpo.sh`, `bin/corpus-threshold.py`, `bin/lora-update.sh`; new `data/training-corpus/snapshots/`; new systemd unit set under `service-slm/compute/systemd/`.

- `bin/export-dpo.sh`: walks `data/training-corpus/apprenticeship/*/shadow-*.jsonl` for `verdict != null`, emits DPO JSONL to `data/corpus/dpo/<date>.jsonl`.
- `bin/corpus-threshold.py`: counts vs 1000-pair LIMA threshold; F12-gated dispatch only.
- `bin/corpus-snapshot.sh`: tar + sha256 of `data/training-corpus/` before any training run; manifest at `data/training-corpus/snapshots/<date>/manifest.json`. Required for replayability.
- `bin/lora-update.sh`: shells to Yo-Yo trainer with snapshot ID; runs Unsloth DPOTrainer on OLMo-2-7B; writes adapter to `data/lora/coding-lora-<date>/`; calls `bin/eval-adapter.sh`; promotes only if `promoted: true`.
- systemd: `lora-update.timer` Sunday 02:00 UTC, **disabled by default**; operator-armed via signed flag file `data/training-approved/<adapter>.tag`.

### P1-1.10 — First dry-run training pass (1 day on Yo-Yo)
- Snapshot corpus, spin up Yo-Yo trainer, run Unsloth pass on 806-tuple `engineering-pointsav` marker (already accumulated).
- Throwaway adapter — do NOT deploy. Goal: prove the pipeline end-to-end + measure $ + measure regression.
- Cost: ~$5-10 GCS + L4 instance-hours.

**Phase 1 exit criteria:** one signed adapter trained, evaluated, and either accepted (promoted to llama-server `--lora`) or rejected (>5% regression) through the F12 gate. End-to-end loop demonstrated, $-bounded, ToS-clean.

---

## Phase 2 — Content creation flow (next 4-6 weeks, parallel)

**Goal:** make the datagraph a real grounding surface for editorial. project-intelligence exposes ENDPOINTS/DATA; project-editorial authors TOPICs (per memory feedback).

### P2-2.1 — `worm_id` + `cites: []` on Entity (~80 LOC, 2 days)
**Where:** `service-content/src/graph.rs:7-15`, extraction Doorman path, `upsert_entities`.
- Every graph entity carries the CORPUS file ID that supports it + its citation list.
- **Prerequisite for any verifiable citation in editorial output.**

### P2-2.2 — Wire `RelatedTo` edges (~150 LOC, 3 days)
**Where:** `service-content/src/graph.rs:79-84` (table declared but unpopulated), extraction prompt.
- Without edges, `graph_context` field in tuples will be sparse; co-evolution loop has nothing to mutate.
- Ask extractor LLM for relations, not just nodes.
- Expose `GET /v1/graph/neighbors?entity_id=&depth=1`.

### P2-2.3 — `POST /v1/editorial/seed` aggregated endpoint (~150 LOC, 2 days)
- One call returns `{entities[], citations[], gravity_keywords[], banned_vocab[], glossary_terms[]}`.
- Editorial Task consumes once per TOPIC seed; eliminates manual 4-surface lookup.
- Module_id-namespaced, rate-limited.

### P2-2.4 — `citations.yaml` ⇄ graph resolver (~120 LOC, 2 days)
**Where:** new `service-content/src/citations.rs`.
- Load `/srv/foundry/citations.yaml` at startup; hot-reload on file change.
- `GET /v1/citations/resolve?q=<alias_or_url>` → CitationId or 404.
- **Hardens against hallucinated citation IDs in LoRA-generated prose.**

### P2-2.5 — `graph_context` field on apprenticeship JSONL (~50 LOC, 1 day)
**Where:** `slm-doorman/src/apprenticeship.rs`.
- When a turn happens, query `GraphContextClient.query_context()` and embed result in tuple.
- LoRA training then learns `(brief + graph_context → citation-grounded prose)`.
- **This is the flywheel for citation-faithful content creation.**

### P2-2.6 — `POST /v1/editorial/grammar` (~80 LOC, 1 day)
- Versioned snapshot of banned-vocab tokens, BCSC forward-looking phrase set, Do-Not-Use list, glossary canonical forms.
- Editorial AS-2 grammar-guided decode consumes this.

### P2-2.7 — Deprecate `/v1/draft/generate` (~-120 LOC, 1 day)
- Ring 2 generating Ring 3 text is an architectural violation.
- Move text generation to slm-doorman; project-intelligence stops emitting prose entirely — only data.

### P2-2.8 — Embeddings + vector index (~400 LOC, 4-5 days)
- Run `nomic-embed-text-v1.5` as second model slot on Yo-Yo trainer.
- sqlite-vec or redb + `instant-distance` HNSW.
- Index targets: 866 engineering tuples + 495 apprenticeship tuples + ontology + corpus ledger text.
- Editorial RAG retrieval: `POST /v1/editorial/retrieve?query=&k=10` returns top-K + citations.

---

## Phase 3 — Observability & scale (next 6-8 weeks, can parallelize with Phase 2)

### P3-3.1 — `metrics-exporter-prometheus` wired (~120 LOC, 2 days)
- Counters: `slm_requests_total{tier,model,adapter_version}`, `slm_cost_usd_total{tier}`, `slm_latency_seconds{tier}` histogram, `slm_yoyo_dispatch_age_seconds` gauge, `slm_audit_writes_total{entry_type}`.
- Without these, dispatch age / cost / latency are blind.

### P3-3.2 — Canary task set (~10 task specs + ~80 LOC, 2 days)
- `data/canary/v1.yaml`: 10 fixed tasks (classify-corpus-entry, summarize-subgraph, extract-entities-from-prose, …).
- `bin/canary-run.sh` runs every adapter version against all 10; results in `data/canary/results/<adapter_version>.json`.
- Drift detector: post-retrain, flag categories with >10% drop.

### P3-3.3 — `/v1/shadow-tier` A/B comparison (~100 LOC, 2 days)
- Fans 1% of `/v1/messages` traffic to both Tier A and Tier C.
- Records both outputs + cheap third-tier judge OR human review.
- Produces "how close is OLMo to Claude on permitted prompts" dataset.

### P3-3.4 — Adapter signing (Sigstore) + version registry (~150 LOC, 3 days)
- Sigstore-sign each adapter on training completion.
- `data/adapters/registry.yaml` with `(adapter, version, base_model, doctrine_version, corpus_sha, trained_at, signer, eval_result, promoted)`.
- SYS-ADR-19 compliance at the artifact layer.

### P3-3.5 — Per-day spend ledger + kill-switches (~150 LOC, 2 days)
- `SLM_TIER_C_DAILY_USD_CAP=$25` enforced in `external.rs`; persist `~/.local/share/slm/tier-c-spend.jsonl`.
- Tool-loop counter in `/v1/messages`: >50× same lineage in 5 min returns 429.
- Per-request `max_tokens` clamp on Tier C (`min(body.max_tokens, 4096)`).
- Wire BUDGET.md `budget.jsonl` from audit-log `cost_usd`.

### P3-3.6 — Burn-and-restart runbook (~1 page, 0.5 day)
**Where:** `service-slm/docs/runbook-corpus-contamination.md`.
- (1) stop drain worker, (2) `mv data/training-corpus data/training-corpus.quarantine.<ts>`, (3) revert all adapters to base OLMo, (4) re-seed from Tier A/B native extractions, (5) post NOTAM + legal review before re-enabling.

---

## Open questions (operator decisions needed before Phase 1 can finish)

1. **Anthropic legal review** of "competing models" clause for Tier-A/B-only capture: clear or not? **Blocks P1-1.8 going live.**
2. **Eval scorer choice for diffs:** edit-distance + JSON-schema, or a Tier C judge call, or human spot-check? Has cost + ToS implications.
3. **Adapter promotion authority:** is `mathew` ssh-key sufficient for F12 signing, or does it need a multi-sig (Master + Task)?
4. **Do-Not-Use list ratification:** project-editorial must publish machine-readable regex set. **Outbox today.**
5. **GCS bucket for adapters:** `gs://woodfine-node-gcp-free-foundry-substrate/adapters/` already named — confirmed correct?

---

## Cost summary (revised 2026-05-18 — Tier C off per operator directive)

| Item | $/mo |
|---|---|
| Workspace VM (n2-standard-4) baseline | ~$100 |
| Yo-Yo inference (Tier B, current cadence) | $50-150 |
| Yo-Yo trainer (nightly, preemptible L4) | $15-40 |
| ~~Anthropic Tier C~~ | **$0 — disabled** |
| **Doorman-path ceiling** | **≤$300** |
| Claude Pro Max 20x (operator-side, separate billing) | (operator-managed) |

**Effort total to ship complete loop:** ~25-35 engineer-days across Phase 0+1, plus ~20-30 engineer-days for Phase 2+3. Phase 0 is the bleeding-edge: ~2 days for irreversible/legal containment.

---

## Execution order — TODO checklist

### Phase 0 — this week
- [ ] P0-0.1 — Stage 6 promote 6 pending commits; redeploy Doorman; flip apprenticeship on (Command Session, 15 min)
- [x] ~~P0-0.2 — Anthropic Console: $200/mo limit on Commercial key~~ **DEFERRED per operator directive 2026-05-18 (no Tier C in production)**
- [ ] P0-0.3 — GCP Billing Budget: **$300/mo** with 50/80/100% alerts + auto-stop function (operator, 2 hr)
- [x] P0-0.4 — Tier-C provenance guard + `tier_used` top-level field (~50 LOC + tests, shipped 2026-05-18 by task@claude-code)
- [x] P0-0.5 — `--runtime=14h` default in nightly-run.sh (shipped 2026-05-18 by task@claude-code)
- [ ] P0-0.6 — journalctl vacuum (Command, sudo) + service-extraction/target prune (task, this session)
- [ ] Outbox: send Do-Not-Use list ratification request to project-editorial (staged in outbox.md)

### Phase 1 — next 2-3 weeks
- [ ] P1-1.1 — Corpus quality gate (~150 LOC, 2 days)
- [ ] P1-1.2 — Hold-out eval set curation + signing (2 days)
- [ ] P1-1.3 — `bin/eval-adapter.sh` (1 day)
- [ ] P1-1.4 — F12 corpus promotion gate + `bin/promote-corpus.sh` (1 day)
- [ ] P1-1.5 — Operator signs first verdict batch (1 day)
- [ ] P1-1.6 — Adapter-version tagging (1 day)
- [ ] P1-1.7 — Tool-use round-trip completion (~300 LOC, 4-5 days)
- [ ] P1-1.8 — Wire `/v1/messages` → enqueue_shadow (1 day, AFTER legal clear)
- [ ] P1-1.9 — LoRA training toolchain (~3-4 days)
- [ ] P1-1.10 — First dry-run training pass on Yo-Yo (1 day, $5-10)

### Phase 2 — content creation flow (parallel, 4-6 weeks)
- [ ] P2-2.1 — `worm_id` + `cites:` on Entity (2 days)
- [ ] P2-2.2 — `RelatedTo` edges + neighbors endpoint (3 days)
- [ ] P2-2.3 — `POST /v1/editorial/seed` (2 days)
- [ ] P2-2.4 — `citations.yaml` resolver (2 days)
- [ ] P2-2.5 — `graph_context` field on apprenticeship JSONL (1 day)
- [ ] P2-2.6 — `POST /v1/editorial/grammar` (1 day)
- [ ] P2-2.7 — Deprecate `/v1/draft/generate` (1 day)
- [ ] P2-2.8 — Embeddings + sqlite-vec HNSW (~400 LOC, 4-5 days)

### Phase 3 — observability & scale (parallel, 6-8 weeks)
- [ ] P3-3.1 — Prometheus metrics exporter (2 days)
- [ ] P3-3.2 — Canary task set (2 days)
- [ ] P3-3.3 — `/v1/shadow-tier` A/B (2 days)
- [ ] P3-3.4 — Sigstore adapter signing + registry (3 days)
- [ ] P3-3.5 — Per-day spend ledger + kill-switches + BUDGET.md wiring (2 days)
- [ ] P3-3.6 — Burn-and-restart runbook (0.5 day)

### Carry-forward from existing plans (still valid; will be folded into the above)
- [ ] Task 3 from `service-slm-hardening-2026-05-18.md`: Tier A 503 + Retry-After when --parallel 1 saturated (~1 hr) — folds into P1-1.1
- [ ] Task 4 from `service-slm-hardening-2026-05-18.md`: Anthropic-shim integration test with tool_use — folds into P1-1.7
- [ ] `service-audit-2026-05-16.md` D5 audit items #18-20 (Prometheus) — folds into P3-3.1
- [ ] `vm.swappiness=10` verification (1 min) — VM stability
- [ ] Yo-Yo boot-disk snapshot before next start (operator)

---

## Files this plan supersedes for execution ordering

These plans remain authoritative for design rationale but are no longer the operative TODO:

- `.agent/plans/MASTER-PLAN-2026.md` (Tier 1-5 phasing) → folded
- `.agent/plans/leapfrog-2026.md` (sovereign moat thesis) → still the WHY; this is the WHEN
- `.agent/plans/service-slm-architecture-2026.md` → still architectural reference
- `.agent/plans/service-content-architecture-2026.md` → still architectural reference
- `.agent/plans/tier-architecture-2026.md` → still tier model reference
- `.agent/plans/sovereign-routing-comprehensive.md` → folded into Phase 0+1
- `.agent/plans/universal-ai-gateway.md` → folded into Phase 1 tool-use work
- `.agent/plans/service-audit-2026-05-16.md` → folded into Phase 3 observability
- `.agent/plans/service-slm-hardening-2026-05-18.md` → folded into Phase 0+1
- `.agent/plans/d5-canonical-message-sprint1.md` → SHIPPED (ae653cdb); follow-up is P1-1.7

Update NEXT.md when Phase 0 items are checked off.
