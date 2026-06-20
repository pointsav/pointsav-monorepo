---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-tier-b-gpu-restoration
title: "Tier B GPU Restoration — yoyo-batch Zone Strategy"
status: active
owner: project-intelligence
created: 2026-06-15
updated: 2026-06-19T04:30Z
author: totebox@project-intelligence (claude-sonnet-4-6)
companion: BRIEF-slm-learning-loop.md
---

# BRIEF — Tier B GPU Restoration

## Context

`yoyo-batch` (L4 GPU VM, us-central1-a) was TERMINATED on 2026-06-13 due to zone stockout.
All DPO enrichment, LoRA training, and entity quality improvement (Q7, Q8) are blocked until
Tier B is restored. Zone fallback is FORBIDDEN per operator policy
(memory: `feedback_yoyo_no_zone_fallback.md`) — zone migration cost is too high for test workloads.

## Decisions locked

- Zone fallback (`--enable-zone-fallback`) is never used.
- `SLM_YOYO_GCP_INSTANCE=yoyo-batch` must be passed explicitly to `start-yoyo.sh` (default is stale).
- Primary zone: `us-central1-a`. Alternate zones for reprovisioning only: `us-central1-b` → `us-east4-a`.

## Decisions open

- [ ] Which zone to reprovision in? (us-central1-b first; us-east4-a if stockout persists)
- [ ] Operator approval required before GCE provisioning — Command scope.
- [ ] ML training libraries (`trl`, `peft`, `transformers`, `accelerate`, `bitsandbytes`) must be
      installed on the new VM before training can execute (Bug 3 — outbox msg
      `project-intelligence-20260615-command-scope-bug3-qemu`).

## Work log

- 2026-06-13: yoyo-batch TERMINATED; us-central1-a L4 stockout confirmed.
- 2026-06-15: Outbox message sent to Command requesting reprovisioning (`project-intelligence-20260615-command-scope-bug3-qemu`). Zone targets: us-central1-b → us-east4-a.
- 2026-06-19 (Session 26): yoyo-batch expected online today. Pre-cycle cleanup completed:
  - **Phase A**: Fixed `LORA_TARGET_MODULES` in `run-dpo-training.py` (LLaMA → OLMo 3 names:
    `att_proj/ff_proj/ff_out/attn_out`). Training was silently producing no-op adapters.
  - **Phase B**: Added `health_down_secs: Option<u64>` to `TierBInfo` + `health_down_since_secs:
    Arc<AtomicU64>` in `YoYoTierClient` — now /readyz surfaces probe-failure duration even when
    circuit stays "closed". Stage 6 + rebuild pending.
  - **Phase C**: Wired Phase 5b adapter pull in `nightly-run.sh` — pulls
    `/data/weights/adapters/apprenticeship-pointsav-wip/` to workspace at Phase 1 start.
  - **Phase 6-D**: Tier A fallback verified: `tier_used: "tier_a_fallback"`, OLMo-2, clean entities.
  - **Training approval tag**: `coding-lora-2026-06-19.tag` created at `/srv/foundry/data/training-approved/`.
  - **Queue state**: pending=886, quarantine=0, poison=77; 18 training markers queued.
  - **Flow test**: 1,021 DPO corpus pairs confirmed; DataGraph enrichment active
    (`entity_count=2–5` per extraction). Two bugs found + fixed:
    - Bug 1: `OwnedSemaphorePermit` not released on client disconnect → 120 s
      `tokio::time::timeout` wrapper bounds hold to EXTRACT_DEADLINE_SECS.
      `DoormanError::RequestTimeout` added. Commit `a7b1572c`.
    - Bug 2: `_ => DeferReason::YoyoTransient` wildcard masked Tier A failures;
      added `TierAFailed`, `ParseError`, `Timeout`, `AllTiersUnavailable` variants.
      Both single + batch handler wildcards now explicit. Commit `a7b1572c`.

## Session 26 continued (yoyo-batch active, drain running)

Drain dispatch had three layered bugs:
1. `SLM_APPRENTICESHIP_TIER_A_ONLY=true` → `tier_a_first=true` → always `Tier::Local`
2. `DEFAULT_BRIEF_TIER_B_THRESHOLD_CHARS=8000` vs actual brief bodies at 69–356 chars → never Tier B
3. `yoyo_label: None` → even if Tier B selected, routes to "default" node (offline) not "trainer"

Fix committed as `75849f60`: drain worker creates `drain_cfg` with `tier_a_first=false`,
`brief_tier_b_threshold_chars=0`, `yoyo_dispatch_label=Some("trainer")`.

Opus pipeline + throughput audit completed. Key findings:
- Only 229/1,021 DPO pairs survive training filters (77.5% filtered — ratio/length)
- `acceptance_test` empty on ALL shadow briefs → OLMo can't self-evaluate → 21% empty attempts
- Serial drain = ~71h to clear 1,128 items; N-worker drain = 18h with concurrency=4
- `SLM_QUEUE_DRAIN_INTERVAL_SEC=1` (zero-code) saves 9.4h idle time

Three follow-on commits coded (pending compile verification):
- `N-worker drain`: `SLM_DRAIN_CONCURRENCY` env var + for loop around spawn in `main.rs`
- `tier_used in DPO pairs`: `verdict.rs:write_dpo_pair()` now records `"tier_used"` field
- `acceptance_test auto-population`: `http.rs:/v1/shadow` populates from `diff --git` headers

## Carry-forward

- [ ] Command: Stage 6 covering commits `c0448b81`→`75849f60` (6 commits) + rebuild + redeploy
      (drain dispatch fix + N-worker drain + tier_used + acceptance_test + all prior fixes)
      NOTE: new outbox message needed after this session with updated commit range
- [ ] Command: add `SLM_DRAIN_CONCURRENCY=4` and `SLM_QUEUE_DRAIN_INTERVAL_SEC=1` to
      `local-doorman.service` systemd override — saves ~63h on 1,128-item backlog
- [ ] Command: dead config removal — `SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED=false` from live
      systemd unit + `daemon-reload + restart local-content.service`
- [ ] Totebox: verify `health_down_secs` appears in `/readyz` after Doorman rebuild
- [ ] Totebox: verify adapter pull works after first successful nightly cycle
      (`ls /srv/foundry/data/adapters/apprenticeship-pointsav-incremental/`)
- [ ] Totebox: Phase 4b reconciliation (1,281 sweep-ledger entries) — gate on Tier B stable ≥1 cycle
- [ ] Totebox: after 500+ new DPO pairs from Tier B enrichment, re-run training filter audit
      (target: >500 pairs surviving MAX_LENGTH_RATIO=8.0 gate)
