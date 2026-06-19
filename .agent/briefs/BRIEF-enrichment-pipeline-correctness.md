---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-enrichment-pipeline-correctness
title: "Enrichment Pipeline Correctness — OLMo 3 Upgrade + Extraction Quality + Adapter Backup"
status: active
owner: project-intelligence
created: 2026-06-15
updated: 2026-06-19 (Session 23)
author: totebox@project-intelligence (claude-sonnet-4-6)
companion: BRIEF-slm-learning-loop.md
plan: /home/mathew/.claude/plans/goofy-rolling-nebula.md
---

# BRIEF — Enrichment Pipeline Correctness (V2 Overhaul)

## Context

### Original defects (Sessions 15-19 — resolved)

Two correctness defects in the enrichment pipeline were identified during the Phase 4b audit:

1. **SHA Ledger Bug** — `yoyo-daily-cycle.sh` wrote the file SHA to the processed ledger on
   `202-ACK` from `/v1/extract`, NOT after successful Tier B enrichment completion. During the
   Tier B outage, ~400 commits were stamped as "done" but never produced DPO pairs. Fix committed
   (`917871f` — SHA write moved after enrichment success), but the stale ledger entries remain.
   When Tier B returns those 400 commits will never be resubmitted.

2. **Phase 4b DataGraph wait too short** — The Phase 4b sweep polls for 60s (6×10s) after
   submitting to `/v1/extract`. At ~1.7–2 tok/s OLMo 7B throughput a document takes 90–120s to
   enrich. Result: `new_pairs=0` is meaningless; the pipeline exits before enrichment completes.
   Fix: make Phase 4b fire-and-forget (submit, record SHA, move on; DPO pair lands async).

### Root causes identified in Session 20 — extraction quality audit

Multi-agent research (3 Explore agents + web research) confirmed why entity extraction quality is poor:

1. **OLMo 2 7B instruction-following weakness** — returns `[]` when grammar constraints are applied (intentionally removed at `main.rs:506`); pre-fill trick (`[{"`) partially mitigates but model still extracts noise phrases (`outbox status`, `corpus payload key`) and hallucinates negated references (`service-research` from "not `service-research`").

2. **4,096-token context window** — long AI session transcripts silently truncate before extraction prompt reaches the text; OLMo 3 7B has 65,536-token context (16× increase).

3. **No few-shot examples** — documented +11-12 F1 point improvement available and entirely unused.

4. **Silent adapter pipeline gap** — 953 apprenticeship DPO pairs exist and training ran on 2026-06-15 (29-min budget). BUT: adapters write to `/home/mathew/adapters/` on yoyo-batch boot disk; daily cycle never pulls them back to workspace; `data/adapters/registry.yaml` is empty; no adapter has ever been promoted to any inference path. Training work is lost on VM reprovision.

5. **Training base model wastes time downloading** — `run-dpo-training.py` re-downloads `allenai/OLMo-2-1124-7B-Instruct` from HuggingFace each training run. OLMo 3 7B Think HF weights already on persistent weights disk at `/data/weights/olmo-3-7b-think-hf/` (put there by `vllm-weights-prep.sh`). Should point to local path.

6. **yoyo-batch lacks ADC** — cannot upload to GCS directly; workspace VM (which HAS cloud-platform ADC) must pull adapter via SSH and then upload.

## Decisions locked

- SHA write must occur AFTER confirmed Tier B enrichment, not on 202-ACK. (committed `917871f`)
- Phase 4b wait should be removed entirely — fire-and-forget matches the async pipeline design.
- OLMo 3 7B Instruct GGUF replaces OLMo 2 7B for Tier A (local-slm.service) — same memory footprint, 16× context, better instruction following.
- OLMo 3 7B Think HF weights already on persistent disk — training base model path updated to `/data/weights/olmo-3-7b-think-hf` (no re-download).
- Adapter output path moved from boot disk (`/home/mathew/adapters/`) to persistent weights disk (`/data/weights/adapters/`) — survives ALL VM cycles including reprovision.
- Adapter pull-back added to yoyo-daily-cycle.sh: workspace VM rsyncs adapter from yoyo-batch after training, then uploads to GCS as backup.
- Few-shot examples (8 examples) added to `EXTRACTION_SYSTEM_PROMPT` before any model upgrade.
- Grammar constraint env flag (`SERVICE_CONTENT_TIER_A_GRAMMAR`) added; test on OLMo 3; if grammar works on OLMo 3 enable via `local-content.service` env var.

## Decisions open

- [x] Repair script for stale ledger entries — committed `52746a3c`
      (`service-slm/scripts/repair-ledger.py`). Produces clean output (ledger at 0 entries).
      Run again when Tier B restores and enrichment cycle completes.
- [x] Phase 4b wait removal — already fixed in prior session (`6a377cc`).
      `phase_4b_datagraph_sweep()` is already fire-and-forget; no code change needed.
- [x] Quarantine re-drive: DONE (2026-06-16 by Command). 737 entries re-driven;
      `queue_quarantine=0`, `queue_pending=785` confirmed post-rerun.
- [ ] Verify OLMo 3 target_modules match: runtime assertion in `run-dpo-training.py:321-330` will confirm on first training run; expected to match (OLMo 3 uses same LLaMA-style arch as OLMo 2).
- [ ] Grammar constraint test: after OLMo 3 Tier A live, run smoke test with `SERVICE_CONTENT_TIER_A_GRAMMAR=json_schema`; decision: keep grammar or revert to pre-fill.
- [ ] Adapter eval gate: after first adapter pull to workspace, run `eval-adapter.sh`; operator reviews result before registry.yaml update.

## Work log

- 2026-06-17 Session 20 (V2 overhaul plan): 3 Explore agents + web research audited extraction quality.
  Root causes confirmed: OLMo 2 7B grammar failure, no few-shot examples, 4k context truncation, adapter
  pipeline gap (953 training pairs, 0 promoted adapters), boot disk loss risk. OLMo 3 7B Think HF weights
  confirmed at `/data/weights/olmo-3-7b-think-hf/` on persistent disk. Full plan at
  `/home/mathew/.claude/plans/goofy-rolling-nebula.md`. Execution order: Step 0 (BRIEF) → Phase 1
  (few-shot) → Phase 2 (grammar flag) → Phase 5a (training base model) — all Totebox; Phase 3+4+5b —
  Command+operator gate.
- 2026-06-10: SHA ledger bug discovered; fix committed `917871f`.
- 2026-06-13: Phase 4b wait semantic issue identified (60s < actual enrichment time).
- 2026-06-15: Both defects surfaced in outstanding-questions sweep; BRIEF created.
- 2026-06-15: `repair-ledger.py` written and committed (`52746a3c`); Phase 4b confirmed already fixed.
- 2026-06-16: Q8 quarantine work: `queue_quarantine` added to Doorman `/readyz`; `redrive-quarantine.py`
  written; 737 entries confirmed in quarantine dir. Awaiting Stage 6 to re-drive.
- 2026-06-16/17: Stage 6 complete (8 commits `088b8e21`→`4886129d`). Command ran quarantine re-drive
  immediately: 737 entries back in queue; queue_quarantine=0 confirmed. Batch extract endpoint
  (e5c0ee4f) also live in production.
- 2026-06-17 Session 17 (Totebox audit): Opus agents audited yoyo-batch cycle. Additional bugs found:
  LoRA target modules wrong (401827c7 used OLMo-1 names; fixed 23b012a1 to LLaMA-arch names);
  DataGraph noise filter too narrow (fixed 23b012a1: +numeric prefix, +10 fragment starters, +18 abstract nouns);
  sweep ledger stuck at 0 permanently (fixed 4a9c81b9: DOC_sweep quarantine gate + unconditional mark_sweep_sha_complete).
  yoyo-batch stopped by operator — do not restart until fixes promoted and verified.
- 2026-06-18 Session 21 (extraction quality + Phase 5–7): Iterative prompt engineering raised extraction
  score from 7/14 → 13/14. Fixes: Account noise filters (hyphen+space), distraction defense examples,
  path fragment example, Doorman test expectation corrected, KV warm-up removed (was corrupting state).
  Key finding: all poor runs were caused by KV state corruption from competing local-content requests or
  warm-up timeouts; clean llama-server restart is prerequisite for reliable tests. Phase 4 (yoyo-batch)
  confirmed RUNNING (circuits CLOSED) — not blocked as BRIEF stated. Phase 5-A gap found: daily cycle
  training SSH uses wrong base model (OLMo 2 remote ID). Phase 6-B ran: sweep ledger empty, nothing to
  repair. 13/14 is CPU-mode ceiling for GCP zone test; grammar constraints on Tier B would fix the last one.
  7 commits promoted by Command (Stage 6); 3 more pending (eb7ad67f, b49a950c, b43af58d).
- 2026-06-18 Session 22 (Tier A vs Tier B deep comparison): First side-by-side comparison run using
  new `test_tier_ab_deep.py` (verbose per-step filter chain logging + JSONL output). Tier B (yoyo-batch
  OLMo 3 7B Think, 10-11 tok/s GPU) scored 7/14 raw vs Tier A (OLMo 3 7B Instruct, CPU) at 11/14.
  Corrected: earlier "13/14" was on a stale test script (example 6 still had old Mathew/us-central1-a
  text while production main.rs had already switched to Peter/us-central1-b). True production score: 11/14.
  Two filter gaps found and fixed (commit d406e1cd, 42/42 tests):
    1. `.service` suffix missing from PATH_SUFFIXES — caught local-content.service as false Project
    2. Multi-word lowercase Project phrase rule — rejected "outbox status", "corpus payload key",
       "enrichment queue", "automation bot". Real project names use hyphens not spaces.
  Predicted Tier B after filters: 9/14. Remaining 5 Tier B failures are model-level:
    - Think model ignores negation ("not authored by Peter Woodfine", "service-bim not active")
    - Think model strips path/CLI context and returns project name (service-slm from path,
      slm-doorman-server from -p flag)
    - Both tiers extract yoyo-batch from "yoyo-batch GPU VM in us-central1-a" (model-level; no filter fix)
  Tier B **outperforms** Tier A on: GCP zone recognition (us-central1-b correctly as Location).
  Tier A **outperforms** Tier B on: negation, noise rejection, instruction following.
  Strategic implication: the two models are complementary. Current Tier A → Tier B routing is correct.
  Tier B JSONL saved to: service-slm/scripts/test_results_deep_20260619T025702Z.jsonl.
  Commits pending Stage 6: d406e1cd + earlier eb7ad67f, b49a950c, b43af58d.
- 2026-06-18 Session 22 (Claude, codebase audit): Full Tier A vs Tier B side-by-side JSONL comparison
  completed. Tier A: 11/14 (prompt v2 cold-start, CPU). Tier B: 7/14 raw → predicted 9/14 with new filters.
  3 shared failures: shell command / path fragment (model context-strips, returns crate name) + multi-entity
  GCP (Tier A truncates at 2 entities; Tier B gets all 4 + extra yoyo-batch). 3 code fixes committed:
    1. entity_filter.rs: single-word all-lowercase Account → None (catches "outbox" noise, 43/43 tests)
    2. run-dpo-training.py: source-type-aware MIN_REJECTED_CHARS — enrichment pairs (entity JSON arrays)
       now use 10-char floor vs 80-char floor for diff pairs. CRITICAL: 80-char floor was silently dropping
       ALL enrichment DPO pairs, zeroing extraction-quality training signal reaching the LoRA trainer.
    3. main.rs Code identifiers Omit rule: explicitly names CLI arguments and build tool commands.
  OLMo research (WebSearch agent): OLMo 3 7B Instruct is latest 7B family (no OLMo 4 as of Jun 2026).
  OLMo 3.1 32B Instruct/Think exists but no 7B 3.1. unsloth/Olmo-3-7B-Instruct-GGUF offers UD-Q4_K_XL
  Dynamic 2.0 quant (reportedly better than standard K-quant). OLMo 3 7B Instruct IFEval: 85.6 vs
  Qwen 2.5 7B 73.4 — our OLMo-only policy is well-founded; current model is best available.
  Instruct (not Think) is correct for extraction tasks — Think is designed for math/code reasoning.
  DPO pair quality audit: 4/10 enrichment pairs had issues (2 empty-rejected; 2 noise-in-chosen).
  The "corpus payload" and "outbox status" noise now caught by filters. "drain loop code" empty-rejected
  already filtered by run-dpo-training.py empty check. "outbox" single-word Account now filtered.
- 2026-06-19 Session 22 continued (taxonomy audit + pipeline fixes + prompt v3):
  Taxonomy audit: Archetypes/COA/Domains/Topics/Themes/Guides all loading correctly via
  taxonomy::load_taxonomy_from_dir(). Two latent bugs found and fixed:
    1. taxonomy.rs active_state not filtered — themes_to_entities() and topics_to_entities()
       now skip rows where active_state != "active". No functional impact today (all rows are
       active) but prevents stale taxonomy from reaching the graph.
    2. graph_cleanup OOV check missing — /v1/graph/cleanup now flags entities whose
       classification is not in ALLOWED_CLASSIFICATIONS (e.g. 'Technology', 'Organisation'
       from pre-OLMo3 era). Reason code 'oov-classification'. http.rs. Fix C from audit plan.
  Note: taxonomy entities (module_id="__taxonomy__") are correctly isolated from the inference
  context injection path — the Doorman's graph_context query uses the session module_id
  (e.g. "jennifer"), so taxonomy is reference-only, not injected into extraction prompts.
  Prompt v3 (8 examples, commit 3e05f810): Split combined example 5 (path+cargo) into two
  dedicated examples (path fragment alone, CLI command alone). Added automation bot example (8).
  Extended code-identifier instruction to explicitly name build tool commands and CLI arguments.
  Synced all 3 files: main.rs, test_tier_a_production.py, test_tier_ab_deep.py. 43/43 tests.
  Complete Tier A vs Tier B comparison from deep test JSONL:
    - Both pass: core(2/2), negation trap, commit prefix, workspace(2/2), Doorman
    - Tier A only: negated person excluded, conditional exclusion, env var, abstract tech nouns
    - Both fail: shell command, path fragment, multi-entity (3 shared failures)
  Commits pending Stage 6: d406e1cd, 3e05f810 + earlier eb7ad67f, b49a950c, b43af58d.
- 2026-06-19 Session 23 (prompt comparison + final Tier A verdict): Parallel production test run
  against MY 7-example prompt (combined path+CLI example 5, provisioning Jennifer/us-east1-b/WMC
  as example 7). Result: **12/14** (1 better than prompt v2). Two remaining failures:
    1. [noise] Rust path fragment → model extracts `slm-doorman-server` as Project even from combined
       path+CLI example; agent's dedicated example (v3 example 5 = exact test sentence) gives
       memorization-pass. Combined approach insufficient.
    2. [edge] Multi-entity GCP: model returns [Person:Mathew, Project:yoyo-batch] instead of
       [Person, Company, Location]. Provisioning example DID NOT HELP — yoyo-batch is the distractor
       and the model classifies it as Project regardless. Pre-fill bias stops at 2 entities.
  **Final verdict**: Agent's committed prompt v3 (3e05f810) is the better prompt; expected 13/14
  on clean slot (path fragment passes via exact memorization). Multi-entity failure is a fundamental
  Tier A limitation — 7B Instruct CPU cannot reliably extract 3-entity combinations when a
  known infrastructure name appears as a distractor. Resolution path: DPO training (deb592ac fix
  now lets enrichment pairs flow into training signal). Tier B is the correct tier for multi-entity.

- 2026-06-17 Session 19 (Totebox rebuild + test): Archive CLAUDE.md + manifest.md contamination fixed
  (was project-knowledge content). service-content binary rebuilt from HEAD (5c3d7f5b, 40/40 tests,
  healthz 11935 entities). yoyo-batch confirmed RUNNING in us-central1-a but llama-server not running
  (port 8080 Connection refused) — outbox sent to Command. DOC_sweep gate verified via unit tests.
  Startup drain burst ongoing (8,140 new files); live DOC_sweep test file pending watcher pickup.

## Carry-forward

### V2 overhaul — status as of 2026-06-19 Session 23

**Extraction quality: 12/14** (my 7-example prompt, production test b19oomhj7, ~17 min).
Agent's committed prompt v3 (3e05f810) expected: **13/14** on clean slot (path fragment passes
via memorization of example 5 = exact test sentence). Multi-entity remains a known Tier A gap.
Test suite: `service-slm/scripts/test_tier_a_production.py` (synced to production main.rs prompt v3).

Failures in both prompt approaches:
- Path fragment: model strips path and returns crate name (v3 example 5 = memorization fix; combined approach not enough)
- Multi-entity: yoyo-batch classified as Project regardless of provisioning examples; pre-fill stops at 2 entities
Root cause of multi-entity: `yoyo-batch` is a known name in model's context, classified as Project.
Resolution: DPO training (deb592ac fix now allows enrichment pairs to flow — long-term improvement).

- [x] **Phase 1 (few-shot)** — 8 examples in `EXTRACTION_SYSTEM_PROMPT` (prompt v3, 3e05f810); 43/43 tests
- [x] **Phase 2 (grammar flag)** — `SERVICE_CONTENT_TIER_A_GRAMMAR` env var wired; reverted on CPU (unusable); stays for Tier B GPU
- [x] **Phase 3 (OLMo 3 Tier A)** — `Olmo-3-7B-Instruct-Q4_K_M.gguf`, ctx-size 8192, `--no-jinja` (deliberately kept)
- [x] **Phase 4 (yoyo-batch)** — RUNNING in us-central1-a; all 3 Tier B circuits CLOSED (trainer/default/graph); queue_pending=876
- [x] **Phase 5a (training base model)** — `run-dpo-training.py` default → `/data/weights/olmo-3-7b-think-hf`
- [x] **Phase 5b (adapter pull+GCS)** — Phase 6b in `yoyo-daily-cycle.sh` (lines 600–618) already coded; adapter dir exists but empty
- [x] **Phase 5-A FIX (yoyo-daily-cycle.sh base model)** — Confirmed fixed by Command. Lines 539 + 576 in `/srv/foundry/bin/yoyo-daily-cycle.sh` already have `/data/weights/olmo-3-7b-think-hf` (verified Session 23 by BRIEF cross-check agent).
- [ ] **Phase 5b verification** — after 5-A fix + next daily cycle (02:30 UTC): check `data/adapters/apprenticeship-pointsav-incremental/adapter_config.json`
- [ ] **Phase 5c (adapter eval)** — after adapter pull: run `eval-adapter.sh`; operator approval gates `registry.yaml` update
- [ ] **Phase 6-A (graph cleanup)** — run `curl http://127.0.0.1:9081/v1/graph/cleanup?module_id=jennifer` after local-content restarts; removes OLMo 2 noise entities
- [x] **Phase 6-B/C (repair-ledger)** — ran `repair-ledger.py --dry-run`; sweep ledger absent/empty; nothing to repair
- [ ] **Phase 6-D (enrichment spot-check)** — run 3-5 test extractions via Doorman after local-content restarts
- [ ] **Phase 7 (final extraction test)** — 12/14 (my 7-example prompt, production test b19oomhj7); prompt v3 (3e05f810) expected 13/14 on clean slot; 6 commits pending Stage 6 (fa97936c d406e1cd 2124c8b6 3e05f810 99f09ed7 deb592ac); after rebuild run production test for v3 baseline confirmation
- [ ] **Verify OLMo 3 target_modules** — runtime assertion in `run-dpo-training.py:321-330` verifies on first training run
- [ ] **Adapter eval gate** — operator reviews eval output before `registry.yaml` promoted
- [ ] **OLMo upgrade path** — consider `unsloth/Olmo-3-7B-Instruct-GGUF` UD-Q4_K_XL for better accuracy at same size; operator decision gate before local-slm.service restart
