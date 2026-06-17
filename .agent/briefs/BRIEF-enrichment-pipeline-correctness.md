---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-enrichment-pipeline-correctness
title: "Enrichment Pipeline Correctness — SHA Ledger + Phase 4b Wait Semantics"
status: active
owner: project-intelligence
created: 2026-06-15
updated: 2026-06-17 (Session 19)
author: totebox@project-intelligence (claude-sonnet-4-6)
companion: BRIEF-slm-learning-loop.md
---

# BRIEF — Enrichment Pipeline Correctness

## Context

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

## Decisions locked

- SHA write must occur AFTER confirmed Tier B enrichment, not on 202-ACK. (committed `917871f`)
- Phase 4b wait should be removed entirely — fire-and-forget matches the async pipeline design.

## Decisions open

- [x] Repair script for stale ledger entries — committed `52746a3c`
      (`service-slm/scripts/repair-ledger.py`). Produces clean output (ledger at 0 entries).
      Run again when Tier B restores and enrichment cycle completes.
- [x] Phase 4b wait removal — already fixed in prior session (`6a377cc`).
      `phase_4b_datagraph_sweep()` is already fire-and-forget; no code change needed.
- [x] Quarantine re-drive: DONE (2026-06-16 by Command). 737 entries re-driven;
      `queue_quarantine=0`, `queue_pending=785` confirmed post-rerun.

## Work log

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

- [x] Run `redrive-quarantine.py` — DONE 2026-06-16 by Command (737 re-driven; queue_quarantine=0)
- [ ] Run `repair-ledger.py` once Tier B is restored (gate: [[project-intelligence-tier-b-gpu-restoration]])
- [x] Stage 6 for 23b012a1 + 4a9c81b9 (Totebox audit fixes) — in origin/main as of 7df62961
- [x] Service-content binary rebuild — DONE 2026-06-17 Session 19 (Totebox self-service; sha256 5c3d7f5b;
      40/40 tests; healthz 11935 entities; ledger entry written; outbox sent to Command)
- [ ] Graph cleanup pass (`/v1/graph/cleanup?module_id=jennifer`) — expect >0 entities flagged now that noise filter is live
- [x] DOC_sweep gate verified via unit test `dpo_sweep_docs_never_generate_pairs` (40/40 pass);
      live test file pending watcher loop pickup after startup drain completes
- [ ] Tier B (llama-server) down on yoyo-batch 10.128.0.24:8080 — Connection refused; VM running but port closed;
      outbox sent to Command (2026-06-17); operator action needed to start llama-server on yoyo VM
- [ ] Run `repair-ledger.py` once Tier B is restored (gate: [[project-intelligence-tier-b-gpu-restoration]])
- [ ] yoyo-batch restart — operator approval required; confirm ML libs installed; us-central1-b ONLY for re-provisioning
