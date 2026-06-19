# NEXT.md — project-intelligence (Totebox)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-19 (Session 25 shutdown)

---

## Active (Totebox scope)

- [ ] **down_for_secs in TierBInfo** — `health_down_secs: Option<u64>` added to TierBInfo
      + `health_down_since_secs: Arc<AtomicU64>` wired in YoYoTierClient/run_health_probe;
      committed but deploy pending (Stage 6 + slm-doorman-server rebuild required)
      [2026-06-19 totebox@project-intelligence]
- [ ] **Phase 4b reconciliation pass** — 1,281 sweep-ledger entries written before Tier B online;
      DOC_sweep quarantine gate in place; Totebox sprint when Tier B restores; gated on
      yoyo-batch being provisioned in us-central1-a (operator approval required)
      [2026-06-15 command@claude-code]
- [x] **CLAUDE.md contamination** — confirmed clean (81 lines, correct project-intelligence
      SLM/Doorman/OLMo/LoRA/DataGraph content; no project-console text)
      [2026-06-19 totebox@project-intelligence]
- [ ] **Phase 5b — adapter pull verification** — pull wired in nightly-run.sh (Phase 5b block);
      pulls from yoyo-batch:/data/weights/adapters/apprenticeship-pointsav-wip/ at start of
      Phase 1 each cycle; verify after first successful yoyo-batch cycle:
      `ls /srv/foundry/data/adapters/apprenticeship-pointsav-incremental/`
      [2026-06-19 totebox@project-intelligence]
- [x] **Phase 6-D — enrichment spot-check** — 3 extractions confirmed; `tier_used: "tier_a_fallback"`;
      OLMo-2 Tier A returning clean entities (Person/Company/Location); f1879462 verified working
      [2026-06-19 totebox@project-intelligence]
- [ ] **Remove dead config** — `SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED=false` confirmed
      absent from all codebase files; must be in live systemd unit only; Command scope
      (systemd override cleanup + daemon-reload); routed via outbox
      [2026-06-19 totebox@project-intelligence]
- [x] **Bug: semaphore leak on client disconnect** — fixed 2026-06-19; 120 s timeout wrapper
      (`EXTRACT_DEADLINE_SECS`) around entire routing block in `/v1/extract` handler;
      `DoormanError::RequestTimeout` returned on deadline → permit drops via RAII; bounds
      permit hold to 120 s even when hyper 0.14 keeps handler alive after client disconnect
      [2026-06-19 totebox@project-intelligence]
- [x] **Bug: DeferReason wildcard in http.rs** — fixed 2026-06-19; added `TierAFailed`,
      `ParseError`, `Timeout`, `AllTiersUnavailable` variants to `DeferReason` enum in
      slm-core; both extract + batch handler wildcards now have explicit arms;
      `DoormanError::RequestTimeout` added to error.rs + ApiError status mapping
      [2026-06-19 totebox@project-intelligence]
- [ ] **Known: queue saturates OLMo in Tier B degraded mode** — corpus queue runs 2 in-flight
      (matching OLMo --parallel 2); when Tier B down, queue uses Tier A leaving 0 slots for
      interactive /v1/extract; resolves automatically when yoyo-batch restores (queue → Tier B);
      workaround: limit queue to 1 in-flight via SLM_BATCH_CONCURRENCY=1 when Tier B down
      [2026-06-19 totebox@project-intelligence]

## Blocked — Command Session (route via outbox)

- [x] **local-slm.service `--parallel 2`** — operator approved 2026-06-19; applied to
      threads.conf drop-in; daemon-reload + restart; service active; two slots now available
      [2026-06-19 command@claude-code]
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
