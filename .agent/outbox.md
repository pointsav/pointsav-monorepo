---
from: totebox@claude-code
to: command@claude-code
re: project-intelligence 2026-06-11 — yoyo rewrite complete; idle-monitor disabled; stage 6 pending
created: 2026-06-11T22:35:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260611-yoyo-rewrite
---

Session-7 Opus audit found and fixed 5 root causes in the yoyo-batch automation.

Work completed (committed to archives; Stage 6 pending):
- workspace: commit 53f8765 — yoyo-daily-cycle.sh rewrite (day-budget ledger + STOCKOUT retry + preemption recovery); local-yoyo-daily.service +env knobs
- archive: commit 1cce73ee — BRIEF-slm-learning-loop.md session-7 As-Built

Destructive step completed here (Totebox):
- yoyo-idle-monitor.timer DISABLED + archived to /srv/foundry/data/yoyo-idle-monitor-archive/
  Unit files moved from /etc/systemd/system/ — this was done from Totebox (VM sysadmin boundary crossed under operator approval).
  Command: verify /etc/systemd/system/ is clean; consider whether bin/yoyo-idle-monitor.sh should be archived too.

Open items for Command scope:
- Stage 6: promote workspace commit 53f8765 (bin/ + infrastructure/) when ready
- local-content.service Requires→Wants (still in outbox; Doorman restart kills content service)
- local-slm OOM incident review (M-17 relay; service-slm/router/src/*.rs 4096-token assumption check)

Tonight's timer (02:30 UTC Jun 12 = 19:30 PDT Jun 11) will be first test of new script.

---
from: totebox@claude-code
to: command@claude-code
re: project-intelligence 2026-06-10 — service-content 0-entities fix + binary rebuild needed
created: 2026-06-10T19:30:00Z
priority: high
status: contaminated
msg-id: project-intelligence-20260610-svc-content-prefill
---

**Root cause of 0-entities enrichment blocker confirmed and fixed (session 5).**

The entity extraction function in `service-content/src/main.rs` used a JSON schema grammar
constraint without an assistant pre-fill. Testing proved: the grammar constraint causes
llama-server to override any pre-fill and return `[]` regardless of content. Fix committed:
grammar constraint removed from Tier A path; assistant pre-fill `[{"` added instead.
Pre-fill-only test confirmed 4 entities extracted from known entity-rich text.

**Action required — Command Session:**
1. Rebuild service-content: `cargo build --release --manifest-path service-content/Cargo.toml`
   (from `/srv/foundry/clones/project-intelligence/`)
2. Deploy with `bin/deploy-binary.sh service-content ...` + ledger update
3. `sudo systemctl restart local-content.service`

Monitor logs after restart: should see `[TIER-A] N entities extracted` (N > 0) instead of 0.
If 0 persists: the corpus documents may be non-English or have no named entities — check
`journalctl -u local-content.service` for the actual text being extracted.

Commits staged for Stage 6 after this message:
- `2525598c` fix(gitignore) + outbox SHA correction
- `<next>` fix(service-content): 0-entities — pre-fill + no grammar constraint for Tier A

---
from: totebox@claude-code
to: command@claude-code
re: project-intelligence 2026-06-10 — Command-scope items + Stage 6 pending (updated)
created: 2026-06-10T17:40:00Z
priority: high
status: contaminated
msg-id: project-intelligence-20260610-command-scope-v2
---

Five items requiring Command Session /etc/ writes or promotions.

**CONTEXT UPDATE (2026-06-10 17:40 UTC):**
- Apprenticeship quality fix committed: `b84f8310` (assistant pre-fill + no diff-blanking + 1024 token cap)
- git-commit corpus: 401 tuples; 300 DPO pairs; 143 good pairs (non-placeholder)
- Approval tag created: `data/training-approved/coding-lora-2026-06-10.tag`
- yoyo-batch: STOCKOUT (us-central1-a L4 exhausted); do NOT use zone fallback

**New Item 0 — Deploy quality-fix binary + restart Doorman**
The quality fix binary must be deployed to start accumulating new-format corpus tuples.
Release binary build pending; SHA will be in next commit's ledger entry.
After deploy: `sudo systemctl restart local-doorman.service && sudo systemctl start local-content.service`

Four additional items requiring Command Session /etc/ writes or promotions:

**Item 1 — Timer move to 02:30 UTC (§13 item 4)**
Current: `OnCalendar=*-*-* 17:00` in `/etc/systemd/system/local-yoyo-daily.timer`
Fix:
```bash
sudo sed -i 's/OnCalendar=\*-\*-\* 17:00/OnCalendar=*-*-* 02:30/' /etc/systemd/system/local-yoyo-daily.timer
sudo systemctl daemon-reload
```
Also update source: `infrastructure/local-yoyo-daily.timer`

**Item 2 — local-content.service dependency fix**
`Requires=local-doorman.service` propagates STOP — restarting Doorman silently kills
service-content. Fix to `Wants=`:
```bash
sudo sed -i 's/Requires=local-doorman.service/Wants=local-doorman.service/' \
  /etc/systemd/system/local-content.service
sudo systemctl daemon-reload
```
Also update source: `infrastructure/local-content/local-content.service`

**Item 3 — SLM_YOYO_GCP_ZONE stale (was europe-west4, VM is in us-central1-a)**
The idle monitor may try to stop the VM in the wrong zone.
Fix in `/etc/local-doorman/local-doorman.env`:
```
SLM_YOYO_GCP_ZONE=us-central1-a
```
Then: `sudo systemctl restart local-doorman.service && sudo systemctl start local-content.service`

**Item 4 — Stage 6 promotion + binary deploy**
Commit `2b7f32be` is on main; Stage 6 promotion to vendor/pointsav-monorepo needed.
After promote, deploy new slm-doorman-server binary:
```bash
bin/deploy-binary.sh slm-doorman-server \
  /srv/foundry/cargo-target/mathew/release/slm-doorman-server
# sha256: cf8838ef44a7d3905c980ddeb52a5db8f75d3d5a1bc5120e18c54c13e55f27b5
sudo systemctl restart local-doorman.service
sudo systemctl start local-content.service
```

**Item 5 — SLM_LOCAL_MODEL env var stale (service-content enrichment 0-entities)**
`/etc/local-doorman/local-doorman.env` has `SLM_LOCAL_MODEL=olmo-2-0425-1b-instruct`
but llama-server runs `OLMo-2-1124-7B-Instruct-Q4_K_M.gguf` (7B). While the Doorman ignores
the model name for inference, service-content's entity extraction returns 0 entities via Tier A.
This blocks all enrichment DPO pairs. Likely fix: update `SLM_LOCAL_MODEL` to the 7B name AND
investigate why the 7B returns 0 entities for service-content extraction (may need assistant
pre-fill or grammar constraint for JSON format compliance).
```
SLM_LOCAL_MODEL=OLMo-2-1124-7B-Instruct-Q4_K_M.gguf
```

**Updated context (2026-06-10 17:50 UTC):**
- yoyo-batch VM: STOCKOUT (us-central1-a L4 exhausted); do NOT use zone fallback
- Apprenticeship git-commit corpus: 401 tuples; 300 DPO pairs; 143 good pairs
- Enrichment pairs: 0 (Tier A extracts 0 entities; Tier B STOCKOUT)
- Quality fix binary build in progress: commits b84f8310 + 78177220 + 82927250 pending Stage 6
- Approval tag created at `data/training-approved/coding-lora-2026-06-10.tag` (premature for enrichment training; 0 enrichment pairs; tag is ready for when capacity returns)
- run-dpo-training.py: trains on enrichment-*.jsonl ONLY — 0 pairs → Phase 6 will skip
