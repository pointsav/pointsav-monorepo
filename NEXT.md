# NEXT.md — project-data (Totebox)

## Active (Totebox scope)

- [ ] Phase 2 batch .md migration: nightly-jennifer-migrate.sh will drive; requires Command Session
      to add cron entry: `0 23 * * * /srv/foundry/clones/project-data/service-input/scripts/nightly-jennifer-migrate.sh`
      → outbox msg project-data-20260614-cron-request [2026-06-14 totebox@claude-code]
- [ ] Monitor jennifer-2 pipeline: watch /tmp/service-extraction-j2.log and CORPUS count in
      /home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/service-fs/data/service-content/ledgers/
      [2026-06-14 totebox@claude-code]

## Completed (2026-06-15, session 10)

- [x] Bug 5 closed: calibration gate returns infrastructure-hold when all Tier B nodes health_up=false;
      nightly script only aborts on 'stop'; SERVICE_INPUT_DOORMAN_ENDPOINT added [2026-06-15 totebox@claude-code]
- [x] service-content Changes A/B/C: HashSet O(1) ledger, 4-worker parallel drain, adaptive 3/30s timeout
      [2026-06-15 totebox@claude-code]
- [x] Committed 590f6aee (pwoodfine) — Stage 6 pending [2026-06-15 totebox@claude-code]
- [x] Messages to project-intelligence (Bugs 1-4 + Bug 9 batch endpoint) and Command Session
      (cron, Stage 6, DPO quarantine, SLM_DRAIN_PAUSED) [2026-06-15 totebox@claude-code]

## Completed (2026-06-14, session 9)

- [x] Flow analysis (pipeline + extraction + DPO) — identified 3 bugs: A (DPO filter mismatch),
      B (SHA always marked), C (DPO write before graph commit) [2026-06-14 totebox@claude-code]
- [x] All 3 bugs fixed in service-content; 35/35 tests; clippy clean; commit 1ba2f459

## Completed (2026-06-14)

- [x] Phase 1 CSV migration: people.csv (9575 rows) + corporate.csv (424) + documentation.csv (267) +
      projects.csv (338) all accepted by service-input :9106; pipeline flowing; 63,412+ CORPUS files
      [2026-06-14 totebox@claude-code]
- [x] Phase 3 SFT pairs: 182 pairs (up from 139; Bug A/B fixes in normalize_reference_yaml)
      provenance=human-curated; commit c295f4ed [2026-06-14 totebox@claude-code]
- [x] service-input/scripts/build-extraction-sft.py — audit-hardened (Bug A: metric key,
      Bug B: dict-themes, format spec in instruction, document-text candidate injection)
- [x] service-input/scripts/nightly-jennifer-migrate.sh — stdin Python parsing (no more
      apostrophe-in-filename offset-reset); hard abort on curl failure; go_no_go_reason logged
- [x] jennifer-2 migration stack provisioned + started: service-fs :9103, service-extraction j2,
      service-input :9106 [2026-06-14 totebox@claude-code]

## Blocked — requires Command Session (route via outbox)

- [ ] Provision cluster-totebox-jennifer-2 → outbox msg project-data-20260614-jennifer2-provision [2026-06-14 totebox@claude-code]
- [ ] Stage 6 promotion of commits 597f8324 + 38708234 + c295f4ed + 1a914564 + 1ba2f459 + 590f6aee → outbox msgs
      project-data-20260614-stage6-blocker-fix + project-data-20260614-stage6-extraction-fix
      + command-20260615-stage-6-pending-project-data-service-con [2026-06-15 totebox@claude-code]
- [ ] After Stage 6 + restart: run DataGraph cleanup dry-run, then destructive pass:
      curl 'http://127.0.0.1:9081/v1/graph/cleanup?module_id=jennifer&dry_run=true'
      curl 'http://127.0.0.1:9081/v1/graph/cleanup?module_id=jennifer&dry_run=false'
      [2026-06-14 totebox@claude-code]

## Blocked — requires Command Session (URGENT)

- [ ] SLM_DRAIN_PAUSED=true on local-doorman: 48-deep apprenticeship queue monopolizing OLMo 7B.
      Entity extraction stalled at ~2 entities/20min. Steps: sudo systemctl stop local-doorman,
      add Environment=SLM_DRAIN_PAUSED=true, restart. → outbox project-data-20260614-drain-paused
      [2026-06-14 totebox@claude-code]

## Blocked — requires project-intelligence

- [ ] yoyo-daily-cycle.sh SHA-on-202-ACK bug fix (blocks enrichment DPO when Tier B L4 returns)
      → msg sent 2026-06-14: project-data-20260614-jennifer-2-ingest-results-4-code-fixes-n
      [2026-06-14 totebox@claude-code]
- [ ] run-dpo-training.py LoRA target_modules → correct OLMo 2 names (att_proj/ff_proj/ff_out/attn_out)
      → msg sent 2026-06-14 [2026-06-14 totebox@claude-code]
- [ ] ML libs: trl/peft/transformers not installed on GPU VM — training has never run
      → msg sent 2026-06-14 [2026-06-14 totebox@claude-code]
- [ ] slm-doorman drain-hold predicate: remove !tier_a_first guard; add down_for_secs to TierBInfo
      (circuit stays "closed" with health_up=false; hold never fires; see main.rs:308-316)
      → msg sent 2026-06-14 [2026-06-14 totebox@claude-code]

## Completed this session (2026-06-14)

- [x] 22-agent Opus audit: 4 critical blockers identified (STOP/NOT_READY verdict)
- [x] Blocker 1: post_to_fs payload shape fixed — {file:{filename,data}, destination_archive, target_service, edge_entities:[]}
- [x] Blocker 2: query_datagraph_entities → /v1/graph/context (was /v1/entities — 404)
- [x] Blocker 3: start-stack.sh SERVICE_CONTENT_BASE_DIR fix — value=$DATA_DIR/$TOTEBOX_ARCHIVE
- [x] Blocker 4: post_to_fs + sleep_rate async; reqwest::blocking removed; tokio::time::sleep
- [x] write_ledger_entry O_APPEND rewrite (eliminates lost-update race)
- [x] 9/9 tests pass; committed 597f8324
- [x] BRIEF-jennifer-2-ingest-pipeline.md work log updated; LoRA decision corrected
- [x] service-content/src/main.rs + service-extraction/src/main.rs audit fixes committed 38708234 (cargo check exit 0)
