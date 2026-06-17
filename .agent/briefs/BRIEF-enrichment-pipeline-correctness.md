---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-enrichment-pipeline-correctness
title: "Enrichment Pipeline Correctness — OLMo 3 Upgrade + Extraction Quality + Adapter Backup"
status: active
owner: project-intelligence
created: 2026-06-15
updated: 2026-06-17 (Session 20)
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
- Few-shot examples (5 examples) added to `EXTRACTION_SYSTEM_PROMPT` before any model upgrade.
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
- 2026-06-17 Session 19 (Totebox rebuild + test): Archive CLAUDE.md + manifest.md contamination fixed
  (was project-knowledge content). service-content binary rebuilt from HEAD (5c3d7f5b, 40/40 tests,
  healthz 11935 entities). yoyo-batch confirmed RUNNING in us-central1-a but llama-server not running
  (port 8080 Connection refused) — outbox sent to Command. DOC_sweep gate verified via unit tests.
  Startup drain burst ongoing (8,140 new files); live DOC_sweep test file pending watcher pickup.

## Carry-forward

### V2 overhaul — status as of 2026-06-17 Session 20

- [x] **Phase 1 (few-shot)** — 5 examples in `EXTRACTION_SYSTEM_PROMPT`; 40/40 tests; commit e9deedbe
- [x] **Phase 2 (grammar flag)** — `SERVICE_CONTENT_TIER_A_GRAMMAR=json_schema` env var in `call_tier_a_extract()`; commit e9deedbe
- [x] **Phase 5a (training base model)** — `run-dpo-training.py` default → `/data/weights/olmo-3-7b-think-hf`; commit e9deedbe
- [x] **Phase 3 (OLMo 3 Tier A)** — `Olmo-3-7B-Instruct-Q4_K_M.gguf` deployed; ctx-size 8192; grammar smoke test PASS (4 entities); `SERVICE_CONTENT_TIER_A_GRAMMAR=json_schema` active; service-content rebuilt (Command, 2026-06-17)
- [x] **Phase 5b (adapter pull+GCS)** — rsync pull + GCS backup added to `yoyo-daily-cycle.sh`; output-dir updated to persistent weights disk; commit 7cb857e (Command, 2026-06-17)
- [ ] **Phase 4 (yoyo-batch OLMo 3 verify)** — BLOCKED: us-central1-a STOCKOUT for g2-standard-4 (L4 GPU). OPERATOR DECISION: wait for capacity to return — no zone change. `local-yoyo-daily.service` timer retries automatically.
- [ ] **Phase 6 (graph cleanup + repair-ledger)** — gate: Phase 4 done (Tier B circuit closed). Run `/v1/graph/cleanup?module_id=jennifer` then `repair-ledger.py --dry-run` first.
- [ ] **Grammar open decision** — closed: grammar PASS on OLMo 3; `SERVICE_CONTENT_TIER_A_GRAMMAR=json_schema` is the live setting; grammar mode is now the default for Tier A
- [ ] **Verify OLMo 3 target_modules** — confirm on first training run after Phase 4 unblocks; runtime assertion at `run-dpo-training.py:321-330` will verify
- [ ] **Adapter eval gate** — after first adapter pull to workspace post-Phase-4; run `eval-adapter.sh`; operator reviews before registry.yaml update
