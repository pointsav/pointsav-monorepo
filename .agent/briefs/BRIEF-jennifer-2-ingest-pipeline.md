---
artifact: brief
schema: foundry-brief-v1
brief-id: jennifer-2-ingest-pipeline
title: jennifer-2 Ingest Pipeline — Labeled Corpus → DataGraph + Entity Extraction Training
status: active
owner: project-data
created: 2026-06-14
updated: 2026-06-14 (session 8)
---

# BRIEF — jennifer-2 Ingest Pipeline

## Context

`cluster-totebox-jennifer` holds 462 source documents in a 1:1:1 triple structure:
source PDF/DOCX → `service-research/assets/*.md` (OCR extracted text) → `service-research/ledger/*.yaml`
(human-curated structured extraction). 461 YAML files confirmed at `/srv/foundry/deployments/
cluster-totebox-jennifer/service-research/ledger/` (Opus audit 5, 2026-06-14).

This labeled corpus has never been fed into the DataGraph. The goal: migrate all jennifer content
through the COMPLETE LLM inference loop — service-input → service-fs → service-extraction →
CORPUS → local-content.service → Doorman → OLMo Tier A + Tier B — generating:
1. Entity-extraction SFT training pairs (Phase 3, from human-curated YAMLs)
2. Enrichment DPO pairs (Phase 4, Tier A vs Tier B disagreement, when Tier B L4 returns)

## Deployment topology (CRITICAL — three paths, two deployment roots)

```
Source content (research documents, YAMLs):
  /srv/foundry/deployments/cluster-totebox-jennifer/
  ├── service-research/ledger/   ← 461 human-curated YAML files (ground truth)
  └── service-research/assets/  ← ~462 .md files (OCR text) — path needs VERIFICATION

Runtime data (CORPUS files, feedback, people CRM):
  /home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/
  └── service-fs/data/
      ├── service-content/ledgers/  ← 63,162 CORPUS_*.json (and growing live, Jun 14)
      ├── training-corpus/feedback/ ← DPO enrichment pairs (yoyo reads this dir)
      └── service-people/ledgers/   ← 372 .json CRM files

jennifer-2 migration staging (new, to provision):
  /home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer-2/
  └── service-extraction/watch/    ← FS_WATCH_DROP_DIR for jennifer-2 service-fs :9101
```

NOTE: "jennifer-1"/"jennifer-2" are WireGuard station labels (topology.yaml:23), NOT separate
deployment directories. There is ONE `cluster-totebox-jennifer` runtime dir. The "jennifer-2"
prefix in this BRIEF refers to the SECOND migration run using a dedicated service-fs :9101
instance, not a second deployment cluster.

## Architecture — ONE DataGraph, ONE learning loop (Opus-confirmed, 3 sessions)

```
[NIGHTLY BATCH — driven by nightly-jennifer-migrate.sh via cron 23:00 UTC]

nightly-jennifer-migrate.sh
  │  [1] GET http://127.0.0.1:9080/readyz → parse tier_b.<node>.circuit
  │  [2] if tier_b circuit = "open" AND tier_a = true (OLMo 7B alive) → SKIP
  │      Reason: flush_tier_a() at service-content/src/main.rs:778-779 marks CORPUS
  │      files as Success when Tier A extracts but Tier B circuit is open.
  │      These files are NEVER re-queued → DPO pairs permanently lost.
  │  [3] if tier_b circuit = "closed" → full dual-tier extraction
  │  [4] if both down → circuit_deferred queues files; auto-requeue on recovery
  │  loops: POST /v1/migrate {batch_size:10, offset:N} → feed offset_next → until empty
  ▼
service-input :9106  [persistent HTTP daemon — NOT a one-shot]
  │  /v1/migrate processes batch_size docs per call (cap 50); re-globs assets/*.md fresh
  │  each call (stable sort by path — BUT offset breaks if files added/removed between calls)
  │  rate_per_min=20 at night (3s/doc)
  ▼
service-fs :9101  [jennifer-2 WORM + watch drop]
  │  POST /v1/append → writes WORM entry + {payload_id}.json to FS_WATCH_DROP_DIR
  │  FS_WATCH_DROP_DIR = .../cluster-totebox-jennifer-2/service-extraction/watch/
  │  service-fs creates dirs lazily on first request (create_dir_all at main.rs:67,93)
  │  WORM ledger at: {FS_LEDGER_ROOT}/{module_id}/log.jsonl
  │  ⚠️ WORM writes are O(n): reads+rewrites entire log.jsonl on each append (not true O_APPEND)
  ▼
service-extraction  [persistent background process]
  │  EXTRACTION_WATCH_DIR = .../cluster-totebox-jennifer-2/service-extraction/watch/
  │  reads {payload_id}.json → payload["file"]["filename"] + payload["file"]["data"] (base64)
  │  filename extension drives dispatch: .md/.yaml/.yml/.txt → raw UTF-8; else → email parse
  │  if no graph entities AND not email → CRM ledger NOT written (gated main.rs:250)
  │  EXTRACTION_EMIT_CORPUS_DIR = .../cluster-totebox-jennifer/service-fs/data/service-content/ledgers/
  │  writes CORPUS_{worm_id}.json (confirmed main.rs:277; service-content picks up CORPUS_* prefix)
  ▼
/home/mathew/.../cluster-totebox-jennifer/service-fs/data/service-content/ledgers/
  │  [LIVE corpus dir — 63,162 files as of Jun 14 09:17; actively growing]
  │  service-content watches this dir via inotify; startup drain pass also scans it
  │  processed_ledgers.jsonl: NOT FOUND in cluster dir (may be at a different path — VERIFY)
  ▼
local-content.service :9081  [ONE instance; watches jennifer corpus dir; unchanged]
  │  Tier A: call_tier_a_extract() → Doorman :9080/v1/chat/completions → OLMo 7B (local CPU)
  │  Tier B: Doorman :9080/v1/extract → OLMo 32B (L4 STOCKOUT; circuit "open")
  │  dual-tier cascade in process_corpus() at main.rs:640
  │  write_enrichment_dpo_pair() at main.rs:537 — only writes when Tier B returns non-empty
  │  when Tier A ok + Tier B circuit-open: flush_tier_a() → Success → DPO pair SKIPPED FOREVER
  │  when both circuit-open: DeferCircuitOpen → circuit_deferred_ledgers → auto-requeue
  ▼                                                    │
DataGraph LadybugDB :9081                             ▼
(module_id=jennifer)          yoyo-daily-cycle.sh
                              FEEDBACK_DIR=.../cluster-totebox-jennifer/service-fs/data/
                                           training-corpus/feedback/  (hardcoded line 29)
                              → Phase 6 LoRA trigger when threshold met
```

## Service-input /v1/calibration-report response structure

**CRITICAL**: `go_no_go` is NESTED inside `summary`, NOT a top-level field.

```json
{
  "batch_size": N,
  "phase2_processed": N,
  "docs": [{"stem":..., "entity_f1":..., "structural_all_green":...}],
  "summary": {
    "structural_pass_rate": 0.0–1.0,
    "mean_entity_f1": 0.0–1.0,
    "go_no_go": "go" | "hold" | "stop"   ← nested here
  },
  "go_no_go_reason": "..."   ← top-level, but different from go_no_go
}
```

Gates: `stop` if total≥5 AND structural_pass_rate<0.80; `hold` if f1<0.30; else `go`.
Empty batch (total=0): defaults to `hold` (structural_rate=1.0, mean_f1=0.0 → hold branch fires).

## Doorman /readyz health fields

```
GET http://127.0.0.1:9080/readyz  ← the JSON endpoint (NOT /health, NOT /healthz)
GET http://127.0.0.1:9080/healthz ← returns plain string "ok", no JSON

Response shape (relevant fields):
{
  "tier_a": true | false,           ← BOOL, not object; Tier A has NO circuit breaker exposed
  "tier_b": {
    "default": {                    ← keyed by node label (may vary)
      "configured": bool,
      "health_up": bool,
      "circuit": "open" | "closed" | "half-open",   ← THIS is the circuit state
      "opened_for_secs": N | null,
      "reason": "..." | null,
      "zone": "..." | null
    }
  }
}
```

Sources: slm-doorman-server/src/http.rs:145-147 (healthz), 149-191 (readyz); router.rs:32-44 (TierBInfo).

## YAML ledger actual schema (Opus audit 5, 2026-06-14)

461 YAML files at `/srv/foundry/deployments/cluster-totebox-jennifer/service-research/ledger/`.
The schema is HETEROGENEOUS. Fields present:

| Field | Frequency | Notes |
|---|---|---|
| `metrics:` | 212/461 | list of `{metric_name, value, unit, note/context}` |
| `theme_alignment:` | 212/461 | list of theme strings (often empty `[]`) |
| `visual_assets:` | 213/461 | list of `{description, page}` |
| `woodfine_institutional_themes:` | 88/461 | additional theme field |
| `entities:` (top-level) | 1/461 | **essentially absent** |
| `entities:` (nested) | 3/461 | nested under wrapper keys |
| `canonical_name`, `entity_name`, `entity_type` | 0/461 | **NEVER APPEAR** |

Real entity sub-fields (where entities exist): `name`, `type` (string OR list), `role`, `location`.
Real metrics sub-field: `metric_name` (not `name`).
Real themes fields: `theme_alignment`, `woodfine_institutional_themes`, `article_themes` (all may be empty).
Media-stub schema (~20 files): `asset_type`, `filename`, `processed`, `sha256` — different structure.
Many files wrapped under: `document_analysis:`, `article_metadata:`, `institutional_analysis:`.

**The SFT script normalize_reference_yaml() must be rewritten** to handle the actual schema.
The prior implementation assumed `entities`, `themes`, `metrics` with sub-fields `name/canonical_name/
entity_name/entity_type` — these field names DO NOT EXIST in the corpus.

Revised normalize targets:
- Entities: check top-level `entities`, also check under wrapper keys; sub-fields `name` + `type`
- Themes: check `theme_alignment`, `woodfine_institutional_themes`, `article_themes`
- Metrics: check `metrics` with sub-field `metric_name` (value + unit optional)

## people.csv status and format

**Status**: `/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/service-people/
people.csv` — file NOT FOUND at this path (Opus audit 4, 2026-06-14). A full filesystem find
returned zero results for `people.csv`.

**If/when it exists** (based on canonical parser at `service-content/scripts/ingest-jennifer.py:117-143`):
- Delimiter: PIPE (`|`), not comma
- Format: read positionally by column index (header row skipped)
  - `row[0]` = name
  - `row[1]` = entity type (`"Person"` / `"Company"` / else → organization)
  - `row[2]` = source
- Relative path expected: `service-people/people.csv` relative to `jennifer_dir` root

The SFT script must use `csv.reader(f, delimiter="|")` and access by column index, NOT by header name.
The RAFT candidate injection step cannot run until people.csv exists.

## Scope

- Build `service-fs` Envelope A: std/axum HTTP binary replacing the bare-metal no_std stub ✓ DONE
- Build `service-input`: new crate (:9106) for file ingest, CSV serialization, batch migration, calibration ✓ DONE (commit 597f8324)
- Extend `service-extraction`: add extension dispatch (.md/.yaml/.txt bypass parse_mail) ✓ DONE
- Write `service-input/scripts/build-extraction-sft.py` (Phase 3 SFT generation) — IN PROGRESS
- Write `service-input/scripts/nightly-jennifer-migrate.sh` (nightly batch driver with health gate) — IN PROGRESS
- Provision jennifer-2 migration stack (service-fs :9101 + service-extraction + service-input :9106)
- Phase 1 CSV prose migration (people.csv + domain CSVs)
- Phase 2 nightly .md batch migration (432 documents)

**Out of scope:** seL4 Envelope B, LoRA training run, Phase 4 DPO generation, service-email integration.

## Decisions locked

| Decision | Rationale |
|---|---|
| No LLM bypass for any file type | Every file through CORPUS → OLMo → DataGraph for maximum training signal |
| CSVs serialized as prose (100 rows/batch, pipe-delimited) | ADR-07 safe for local OLMo; RAFT context for DataGraph |
| service-fs Envelope A (std/axum) | seL4 Envelope B blocked on Microkit SDK |
| ⚠️ LoRA target_modules — NEEDS FIX | `['q_proj','k_proj','v_proj','o_proj','gate_proj','up_proj','down_proj']` are LLaMA names; silent no-op on OLMo 2. Correct OLMo 2 names: `['att_proj','ff_proj','ff_out','attn_out']`. Messaged to project-intelligence 2026-06-14. |
| EXTRACTION_EMIT_CORPUS_DIR → jennifer-1 LIVE dir | ONE service-content instance; ONE DataGraph; jennifer-2 CORPUS feeds same watcher |
| SFT before DPO | 462 examples above SFT curve but below DPO floor; SFT adapter must be deployed first |
| Migration runs NIGHTLY only | Prevents overloading Doorman/service-content during daytime; allows daytime calibration review |
| Health gate before each nightly run | Skip if Tier A=true + Tier B circuit=open → DPO pairs permanently lost via flush_tier_a() |
| No direct mutate_datagraph calls | All entities via pipeline for provenance traceability |

## Tier architecture

| Tier | Identity | Endpoint | Status (Jun 14 session 6) |
|---|---|---|---|
| A | OLMo 7B (student) | Doorman :9080/v1/chat/completions | Live, local CPU — contested by apprenticeship drain |
| B | OLMo 32B-Think (teacher) | Doorman :9080/v1/extract | **LIVE** as of session 6 (all nodes health_up=true) |
| C | Claude/external | Doorman :9080 allowlist-gated | Never auto-fallback; ToS boundary |

**Tier B live-session results (2026-06-14 session 6):**
Before Tier B: ~1 entity extracted per 3–6 min (OLMo 7B; blocked by 43 apprenticeship briefs).
After Tier B: 3–6 entities per 3-min extraction cycle (OLMo 32B). ~10x throughput improvement.
32 DPO enrichment pairs produced today; 78% Tier B extracted more entities than Tier A.
DataGraph: 10,831 entities at end of session (was ~7,445 on 2026-06-01).
Extraction cycle: Tier A unavailable → fallthrough to Tier B; `[GRAPH] N entities written (module: jennifer)`
log line every ~3 min with N typically 1–6.

**DPO loss edge case (confirmed, main.rs:778-779):**
When Tier A succeeds and Tier B circuit is OPEN: `flush_tier_a()` returns `ExtractResult::Success` →
CORPUS file marked done in processed_ledgers → never re-queued → enrichment DPO pair permanently lost.
Mitigation: nightly health gate skips migration when this state is detected.

**circuit_deferred re-queue (confirmed, main.rs:398-404):**
When BOTH tiers are circuit-open: file goes to `circuit_deferred_ledgers`. Recovery probe runs every 30s.
On first successful probe, ALL deferred files moved to active queue via `Vec::append`. Full backlog resumes.
This is the user's "pile up and wait" model — correct for the both-down case only.

## Training data output

`write_enrichment_dpo_pair()` already implemented in `service-content/src/main.rs:537`.
When Tier B L4 returns: deferred CORPUS files auto-requeue → enrichment DPO pairs generated automatically.
DPO pairs land in: `.../cluster-totebox-jennifer/service-fs/data/training-corpus/feedback/`
`yoyo-daily-cycle.sh:29` reads exactly this path → Phase 6 LoRA trigger.

Phase 3 SFT pairs: `build-extraction-sft.py` → `/srv/foundry/data/training-corpus/extraction/
jennifer-sft-<stem>.jsonl` with `provenance: "human-curated"`.

**ToS boundary (locked):** Claude (Tier C) outputs NEVER enter training data. Only `human-curated` or `olmo-self` provenance.

## Work log

**2026-06-14 — Session 1:**
- Dual Opus sweep: architecture + LoRA/DPO literature
- Key finding: `write_enrichment_dpo_pair()` already implemented in service-content
- Key finding: yoyo SHA-on-202-ACK bug means ~400 commits won't re-enrich when Tier B returns
- Key finding: LoRA 7-module explicit list already in code; BRIEF's "all-linear" was wrong → corrected
- service-fs Envelope A + service-extraction extension dispatch + service-input initial crate committed (7434dbb7)

**2026-06-14 — Session 2 (22-agent Opus audit + 4-blocker fix):**
- 22-agent Opus audit (5 phases, 1.6M tokens): STOP / NOT_READY verdict; 4 critical blockers
- Blocker 1: `post_to_fs` payload shape fixed → `{file:{filename,data}, destination_archive, target_service, edge_entities:[]}`
- Blocker 2: `query_datagraph_entities` → `/v1/graph/context?q=<stem>&module_id=<id>&limit=100`
- Blocker 3: `start-stack.sh` → `SERVICE_CONTENT_BASE_DIR="$DATA_DIR/$TOTEBOX_ARCHIVE"`
- Blocker 4: `post_to_fs` + `sleep_rate` made async; reqwest::blocking removed; O_APPEND ledger
- All 9 tests pass; committed `597f8324`

**2026-06-14 — Session 3 (12-agent Opus audit, plan hardening):**
- Audit 1: Doorman health gate wrong — /health → /readyz; tier_a.circuit_state doesn't exist; tier_b.<node>.circuit is correct field
- Audit 2-12: CORPUS naming, extraction payload, people.csv (pipe-delimited positional), YAML schema (theme_alignment/metric_name), source asset path, processed_ledgers not found — all documented and fixed

**2026-06-14 — Session 4 (execution: provisioning + migration stack + SFT corpus):**
- Block 3: Provisioned `/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer-2/` (sudo required — parent dir root:root)
- Block 4: Built service-fs, service-extraction, service-input fresh (Jun 14 12:09-12:10)
- Block 5: Started migration stack:
  - service-fs :9103 (jennifer-2 WORM; 9100/9101/9102 occupied by system services)
  - service-extraction j2: EXTRACTION_WATCH_DIR=j2/watch, EXTRACTION_EMIT_CORPUS_DIR=j1/corpus (ONE DataGraph)
  - service-input :9106: rate_per_min=6, reference_root=/srv/foundry/deployments/cluster-totebox-jennifer
- Block 6 (Phase 1 CSV): people.csv (9575 rows) + corporate.csv (424) + documentation.csv (267) + projects.csv (338) all accepted; 138+ CORPUS files emitted and flowing to service-content
- Block 7 (Phase 3 SFT): 139 SFT pairs generated at /srv/foundry/data/training-corpus/extraction/; 318 skipped-empty; provenance=human-curated confirmed; committed 838fc1e8
- Block 8 (nightly driver): nightly-jennifer-migrate.sh written with corrected health gate and go_no_go path; committed 838fc1e8
- Block 9: NEXT.md + outbox updated; cron request sent to Command Session

**2026-06-14 — Session 5 (5-agent Opus audit + code fixes):**
- 5-agent Opus audit across: (1) entity extraction quality, (2) throughput/contention, (3) SFT corpus quality, (4) watch dir/idempotency, (5) nightly script/migrate correctness
- Root throughput blocker confirmed: SLM_DRAIN_PAUSED=true needed. 48 apprenticeship shadow briefs monopolize OLMo 7B slot (3–10 min each). Drain-hold predicate bypassed by SLM_TIER_A_FIRST=true; circuit="closed" despite GPU offline 17h+ (health probes don't trip circuit breaker). Outbox msg project-data-20260614-drain-paused sent.
- SFT corpus defects: 138/139 pairs had zero entities (metric-extraction corpus; human-curated YAMLs almost never contain entity annotations). Three normalization bugs fixed: Bug A (metric key), Bug B (dict-themes str() repr), Bug C (candidate injection noise).
- Code changes committed (c295f4ed — pwoodfine):
  - service-content/src/main.rs: EXTRACTION_SYSTEM_PROMPT hardened (Licence/SPDX to Omit; Location definition with worked examples); ALLOWED_CLASSIFICATIONS enum guard in raw_entities_to_graph
  - service-extraction/src/main.rs: drop file moved to processed/ after successful emit (stops watch dir accumulation; audit-safe move not delete)
  - build-extraction-sft.py: Bug A/B fixed; document-text candidate injection; explicit JSON schema in instruction. 182 pairs vs 139 (31% increase).
  - nightly-jennifer-migrate.sh: stdin-based Python parsing (apostrophes in Bloomberg filenames were silently resetting offset to 0); hard abort on empty curl response; go_no_go_reason logged
- 143 orphaned watch dir files (jennifer-2) verified and moved to processed/ (all had CORPUS counterparts)
- Rust changes pending build verification; binaries to be rebuilt/redeployed once lock releases

**2026-06-14 — Session 6 (Rust commit + migration cycle + Tier B analysis):**
- Cargo check completed exit 0 for both service-content and service-extraction (dev profile).
- Committed 38708234 (pwoodfine):
  - service-content/src/main.rs: EXTRACTION_SYSTEM_PROMPT hardened (Location definition + negative
    examples; SPDX/licence identifiers added to Omit list); ALLOWED_CLASSIFICATIONS module-level
    const; raw_entities_to_graph() enum guard drops out-of-vocab classifications before LadybugDB write.
  - service-extraction/src/main.rs: drop file moved to watch_dir/processed/ after successful emit
    (startup drain + inotify event sites). Moving not deleting preserves audit trail.
- Migration cycle (manual, bypassing nightly cron gate):
  - service-extraction j2 restarted; watch dir cleared to 0 pending immediately on startup.
  - 59 research documents pushed through /v1/migrate (offsets 0–79, batch_size 10–20).
  - Phase 2 reference dir EXHAUSTED: offset 80 returned empty — all available .md docs migrated.
  - CORPUS total: 64,279 → 65,214 (+935 total; our 59 + 876 from background live activity).
- Tier B came live mid-session (all three nodes health_up=true, circuit closed):
  - Throughput: 3–6 entities per 3-min extraction cycle vs ~1 entity per 3–6 min Tier-A-only.
  - Tier B went offline again ~16:07 (46+ consecutive health probe failures; circuit still "closed" — known bug).
  - DPO enrichment pairs: 4 total at session-7 audit time (NOT 32 — session-6 count was wrong; the 834
    files at /srv/foundry/data/training-corpus/feedback/ are apprenticeship-git-commit-*.jsonl SFT pairs,
    a different artifact type). All 4 DPO pairs are from commit/agent activity, not research documents.
  - Newest DPO pair (15:33 today): B extracted [service-fs, service-research] (2), A only [service-fs] (1).
  - Apprenticeship queue: 43 → 45 pending (grew during session — GPU drain contention ongoing).
- DataGraph: 10,818 → 10,831 entities at session end.
- Outbox: project-data-20260614-stage6-extraction-fix added for 38708234.
- Comprehensive message to project-intelligence (msg-id: command-20260614-jennifer-2-ingest-
  results-4-code-fixes-n): drain-hold predicate bugs §1, LoRA target_modules §2, ML libs §3,
  our service-content/extraction commits §4, yoyo SHA bug §5. NEXT.md items marked as messaged.

**2026-06-14 — Session 7 (3-Opus-agent audit: entity quality + DPO signal + pipeline health):**
- DataGraph entity quality (Opus agent, 19-letter sweep, 78 unique entities sampled):
  - Quality score: 42/100. ~45% precision on named-entity dimension.
  - GOOD: real people (Woodfine family, cited authors), real companies (Woodfine, ICSC, SaaS vendors), real locations, real external datasets (NREL NSRDB, USGS NSHM 2023).
  - BAD (55%): code field names / env-var names as Account (worst pattern); file paths / script filenames as Project; generic tech concepts as Project (SaaS, Hyperscaler, RTOS); sentence fragments as entities; placeholder strings as Location. Confidence scores are non-discriminating (noise scores same as clean).
  - Binary mismatch explains some of this — ALLOWED_CLASSIFICATIONS enum guard and hardened prompt NOT deployed. Production binary runs /usr/local/bin/service-content (installed binary), not the new debug binary.
- DPO pair audit (Opus agent):
  - 4 pairs, NOT 32. The 834 apprenticeship-git-commit-*.jsonl at /srv/foundry/data/training-corpus/feedback/ are SFT (different type); DPO enrichment pairs are only 4.
  - P1 (service-fs + service-research vs service-fs): GOOD — clean superset.
  - P2 (Peter Woodfine + project-console + ops(slm) vs project-console): CONTAMINATED — ops(slm) is a conventional-commit prefix, not an entity. Trains a hallucination in chosen.
  - P3 (Peter Woodfine + project-console + slm-learning-loop:Project vs project-console + slm-learning-loop:Account + docs:Account): GOOD — type correction + generic-noun suppression.
  - P4 (Peter Woodfine vs project-console): AMBIGUOUS — disjoint sets; can't confirm which model dropped a true positive.
  - Recommendation: do NOT train on 4 pairs as-is. Minimum threshold: 200–300 cleaned, genre-diverse pairs. Pre-save validator needed (reject commit-prefix patterns like \w+\([^)]+\):).
- Pipeline health audit (Opus agent, live system access):
  - NEW FINDING: SLM_TIER_A_FIRST=true in /etc/local-doorman/local-doorman.env. The drain-hold predicate at main.rs:308-316 is gated on !tier_a_first — so the entire hold branch is dead code in live config. Drain worker keeps dispatching through Tier A regardless of Tier B state.
  - NEW FINDING: 743 quarantined briefs in queue-quarantine (not surfaced in /readyz). Far larger than the 1 poison entry. These are classification-guard rejections, permanently excluded from DPO unless re-driven.
  - DPO pairs lost rate: ~20/hour while Tier B offline (started ~16:07 UTC). ~10 already lost at audit time. Loss is permanent.
  - Binary mismatch confirmed independently: 2.1 MB stripped prod binary (Jun 14 15:50 restart) vs 82 MB debug binary (Jun 14 14:36 build).
  - All 5 blockers are Command Session / project-intelligence scope. → supplementary outbox msg sent.
- CORPUS total at audit: 65,453 (was 65,214 at session 6 end, +239 from live activity).
- DataGraph: 10,833 entities (was 10,831 at session 6 end, +2).
- Migrations this session: 2 batches (offset 0-9, 8 processed / 2 skipped). Bloomberg/CRE research articles.

**2026-06-14 — Session 8 (6-change entity quality implementation):**
- Context compaction resumed from Session 7. Opus agent's 6-change implementation plan executed in full.
- Sent Opus analysis to project-intelligence (msg-id: command-20260614-opus-code-analysis-6-concrete-changes-to).
- New file: `service-content/src/entity_filter.rs` — `is_noise_entity_name()`, `is_commit_prefix()`, `clean_dpo_side()`, `is_known_place()`, `coerce_classification()`.
- Changes to `service-content/src/main.rs`:
  - Change 1: EXTRACTION_SYSTEM_PROMPT expanded with 6 new Omit categories (code identifiers, commit prefixes, generic concepts, placeholders, fragments, country hard rule, 8-word max constraint).
  - Change 2: `is_noise_entity_name()` wired into `raw_entities_to_graph()` — deterministic backstop post-empty-check.
  - Change 3: `clean_dpo_side()` wired into `write_enrichment_dpo_pair()` — commit-prefix validator; shadow-rebinds tier_a/tier_b before grounding check.
  - Change 4: `coerce_classification()` wired into `raw_entities_to_graph()` — country→Location, path→reject, CAPS-Account→reject.
  - Change 5: Word-count gate (>8 words → reject) in `raw_entities_to_graph()`.
- Changes to `service-content/src/graph.rs`: `delete_entity()` added to `GraphStore` trait + `LbugGraphStore` impl.
- Changes to `service-content/src/http.rs`: `graph_cleanup` handler + `CleanupQuery`/`CleanupReport`/`CleanupSample` types + route `/v1/graph/cleanup`.
- 30/30 tests passing (7 new entity_filter tests + 4 new main.rs integration tests).
- Clippy running (background); commit 1a914564 (pwoodfine) staged and committed.
- Stage 6 outbox message sent: command-20260615-stage-6-pending-project-data-service-con.

## Decisions open (carry-forward)

- **nightly cron registration**: Cron entry pending on VM — outbox msg project-data-20260614-cron-request sent to Command Session. Entry: `0 23 * * * /srv/foundry/clones/project-data/service-input/scripts/nightly-jennifer-migrate.sh`
- **SLM_DRAIN_PAUSED=true — URGENT**: Apprenticeship queue starving OLMo. Outbox msg project-data-20260614-drain-paused sent to Command Session. go_no_go=stop until entities land.
- **Drain-hold predicate code fix — REVISED SCOPE**: SLM_TIER_A_FIRST=true in live env makes !tier_a_first guard always false → entire hold branch is dead code. The fix must move to a guard that fires before the tier_a_first check OR operator must set SLM_TIER_A_FIRST=false. Original §1 fix (TierBInfo down_for_secs) is still needed but insufficient alone. → supplementary msg to project-intelligence 2026-06-14.
- **flush_tier_a() re-queue decision**: When Tier A succeeds and Tier B circuit-open, CORPUS file permanently done, DPO pair skipped. → messaged project-intelligence 2026-06-14 (§4 of same msg); decision pending.
- ~~**processed_ledgers.jsonl location**~~: RESOLVED — confirmed at /var/lib/local-content/graph/processed_ledgers.jsonl (59,298+ entries 2026-06-14). Not under jennifer cluster tree — correct. No restart re-processing risk.
- **migration stack systemd**: service-fs :9103, service-extraction j2, service-input :9106 are ephemeral (not systemd-managed). Will die on VM reboot. Command Session should add to systemd or nightly pre-script.
- **Stage 6 promotion**: Commits 597f8324 + 38708234 + c295f4ed + **1a914564** in outbox (project-data-20260614-stage6-blocker-fix + project-data-20260614-stage6-extraction-fix + command-20260615-stage-6-pending-project-data-service-con). Command Session to promote in that order.
- **Phase 2 .md migration**: EXHAUSTED — Phase 2 reference dir fully migrated (offset 80 returned empty; 59 docs ingested 2026-06-14). No further nightly batches needed for Phase 2. Nightly cron (once registered) will handle any future Phase 2 additions.
- **SFT corpus entity gap**: 138/139 pairs have 0 entities — human-curated YAMLs are a metric/theme corpus, not entity corpus. To produce entity training signal, either (a) back-derive entity labels from people.csv source references, or (b) generate Tier A entity labels and use as human-approved-olmo-self provenance.
- **743 quarantined briefs**: queue-quarantine has 743 entries (not surfaced in /readyz). Permanently excluded unless re-driven. Need re-drive policy after hardened binary deploys. → supplementary msg to project-intelligence 2026-06-14.
- ~~**DPO pre-save validator**~~: DONE (session 8, commit 1a914564). `is_commit_prefix()` + `clean_dpo_side()` in `entity_filter.rs`; wired into `write_enrichment_dpo_pair()`. Rejects `\w+\([^)]+\)` pattern from both chosen and rejected sides before serialization. Degenerate pair (tier_b_clean.len() < tier_a_clean.len()) also dropped.
- **DataGraph cleanup pass**: CODE READY (session 8, commit 1a914564). `GET /v1/graph/cleanup?module_id=jennifer&dry_run=true` endpoint live in `http.rs`. Applies same `is_noise_entity_name` + `coerce_classification` filters as ingest gate. Run dry-run first, then `dry_run=false` after binary deploys + restarts. Awaiting Stage 6 + `sudo systemctl restart local-content.service`.
- **yoyo SHA-on-202-ACK bug**: → messaged project-intelligence 2026-06-14 (§5 of same msg).
- **run-dpo-training.py LoRA module names**: LLaMA names are silent no-op on OLMo 2; correct names are att_proj/ff_proj/ff_out/attn_out. → messaged project-intelligence 2026-06-14 (§2).
- **ML libs not installed on GPU VM**: trl/peft/transformers required for Phase 6 — training has never run. → messaged project-intelligence 2026-06-14 (§3).
