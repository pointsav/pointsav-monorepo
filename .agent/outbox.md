---
from: totebox@claude-code
to: command@claude-code
re: project-intelligence 2026-06-10 — Command-scope items + Stage 6 pending (updated)
created: 2026-06-10T17:40:00Z
priority: high
status: pending
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
# sha256: 0cfbb9d9010e9b09d5bd204bf9d18c55c9bb8ed856398d8b652515a0b9db6c84
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
