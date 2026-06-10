---
from: totebox@claude-code
to: command@claude-code
re: project-intelligence 2026-06-10 — Command-scope items + Stage 6 pending
created: 2026-06-10T16:55:00Z
priority: high
status: pending
msg-id: project-intelligence-20260610-command-scope
---

Four items requiring Command Session /etc/ writes or promotions:

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
# sha256: 0cfbb9d9010e9b09d5bd204bf9d18c55c9bb8ed856398d8b652515a0b9db6c84
sudo systemctl restart local-doorman.service
sudo systemctl start local-content.service
```

**Context:**
- yoyo-batch VM started (us-central1-a, 10.128.0.24) — circuit CLOSED
- Corpus at 0 (reset 2026-06-09); pairs accumulate as commits fire the post-commit hook
- Approval tag (coding-lora-2026-06-10.tag) NOT yet created — waiting for ≥50 genuine pairs
- eval-adapter.sh + data/adapters/registry.yaml scaffolded (Phase 1; full GPU eval in Phase 2)
