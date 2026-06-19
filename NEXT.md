# NEXT.md — project-intelligence (Totebox)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-19

---

## Active (Totebox scope)

- [ ] **down_for_secs in TierBInfo** — expose seconds-since-last-healthy-check in /readyz;
      circuits currently report "closed" with health_up=false causing false routing to Tier B;
      Bug 4 drain-hold predicate fix is live but this TierBInfo field extension is outstanding
      [2026-06-15 command@claude-code]
- [ ] **Phase 4b reconciliation pass** — 1,281 sweep-ledger entries written before Tier B online;
      DOC_sweep quarantine gate in place; Totebox sprint when Tier B restores; gated on
      yoyo-batch being provisioned in us-central1-a (operator approval required)
      [2026-06-15 command@claude-code]
- [ ] **CLAUDE.md contamination** — still contains project-console mission text; replace with
      correct project-intelligence SLM/Doorman/OLMo/LoRA/DataGraph content; Totebox scope
      [2026-06-16 command@claude-code]

## Blocked — Command Session (route via outbox)

- [ ] **local-slm.service `--parallel 2`** — slot contention root cause: service-content
      calls `/v1/chat/completions` (Doorman→llama-server) every 3 min; single slot means corpus
      drain and foreground tests block each other; fix is `--parallel 2`; BLOCKED pending
      explicit operator approval [2026-06-18 command@claude-code]
- [ ] **yoyo-batch ML libs** — trl/peft/transformers/accelerate/bitsandbytes not installed in
      training venv on GPU VM; LoRA training has never produced a real adapter; install needed
      before next training cycle; yoyo-batch TERMINATED (us-central1-a STOCKOUT); restart
      requires operator approval [2026-06-16 operator]

## Completed (Sessions 18–25)

- [x] **Stage 6 complete — 13 commits total** — 8 commits (088b8e21→4886129d) + 5 commits
      (1fe42506→12076cf1) on canonical; includes Doorman Tier A fallback (f1879462),
      LoRA r=32/alpha=64 + sigmoid_norm DPO (60e88399), batch-extract endpoint, drain-hold fix,
      repair-ledger.py, DOC_sweep quarantine gate, entity_filter.rs hardening
      [2026-06-19 command@claude-code]
- [x] **Doorman Tier A fallback (f1879462)** — `/v1/extract` now falls back to Tier A when
      Tier B circuit open; canonical but binary rebuild pending (in-flight 2026-06-19)
      [2026-06-19 command@claude-code]
- [x] **service-content rebuilt** — binary from 631574ee (prompt v3 + entity_filter.rs);
      local-content.service active; entity_count=12,080 [2026-06-19 command@claude-code]
- [x] **OOV cleanup** — 531 pre-OLMo3 entities + 84 noise-name entities deleted;
      615 total removed; DataGraph healthier post-cleanup [2026-06-19 totebox@project-intelligence]
- [x] **Phase 7 Tier A test** — 12/14 tests passed (prompt v3); two remaining are semantic
      edge cases (GCP zone context + Doorman entity classification) [2026-06-19 totebox@project-intelligence]
- [x] **yoyo-batch /data/weights/adapters** — directory created; June 14 adapter rsync'd;
      1,043 pairs queued; training will succeed on next cycle when VM restarts
      [2026-06-19 totebox@project-intelligence]
- [x] **LoRA target_modules fix** — OLMo 2 names: att_proj/ff_proj/ff_out/attn_out; startup
      assertion added; real LoRA training now possible [2026-06-16 totebox@project-intelligence]
- [x] **Bug 1: SHA-on-202-ACK** — repair-ledger.py (52746a3c) ran; stale SHA entries cleared;
      ~400 files will re-enrich automatically when Tier B restores [2026-06-16 totebox@project-intelligence]
- [x] **Doorman batch-extract endpoint** — POST /v1/batch/extract; Semaphore(4) Tier A /
      Semaphore(1) Tier B; CONTENT_BATCH_SIZE env var; commit e5c0ee4f [2026-06-16 command@claude-code]
- [x] **redrive-quarantine.py** — 737 quarantined briefs → queue; queue_quarantine=0
      [2026-06-16 command@claude-code]
- [x] **NEXT.md contamination repaired** — project-gis content replaced with correct
      project-intelligence state [2026-06-19 command@claude-code]
