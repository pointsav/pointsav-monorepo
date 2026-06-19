---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-tier-b-gpu-restoration
title: "Tier B GPU Restoration — yoyo-batch Zone Strategy"
status: active
owner: project-intelligence
created: 2026-06-15
updated: 2026-06-19
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

## Carry-forward

- [ ] Command: provision/start yoyo-batch (operator action); confirm VM is RUNNING
- [ ] Command: verify ML libs (`trl 1.5.1` in `~/training-venv`) still intact after VM restart
- [ ] Command: Stage 6 + `cargo build --release -p slm-doorman-server` + `bin/deploy-binary.sh`
      + `sudo systemctl restart local-doorman.service` (for Phase B health_down_secs)
- [ ] Command: dead config removal — `SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED=false` from live
      systemd unit + `daemon-reload + restart local-content.service`
- [ ] Totebox: verify `health_down_secs` appears in `/readyz` after Doorman rebuild
- [ ] Totebox: verify adapter pull works after first successful nightly cycle
      (`ls /srv/foundry/data/adapters/apprenticeship-pointsav-incremental/`)
- [ ] Totebox: Phase 4b reconciliation (1,281 sweep-ledger entries) — gate on Tier B stable ≥1 cycle
