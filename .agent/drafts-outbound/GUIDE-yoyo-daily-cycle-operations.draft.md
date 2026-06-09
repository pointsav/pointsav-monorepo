---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: PROSE-GUIDE
title: "Operating the Yo-Yo Daily Enrichment Cycle"
slug: guide-yoyo-daily-cycle-operations
language: en
status: draft
target_repo: woodfine-fleet-deployment
target_path: cluster-totebox-jennifer/guide-yoyo-daily-cycle-operations.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "BRIEF-slm-learning-loop.md §11 §13 §14; bin/yoyo-daily-cycle.sh; service-slm/NEXT.md pre-run checklist"
research_inline: true
created: 2026-06-09
author: totebox@project-intelligence (claude-sonnet-4-6)
---

# Operating the Yo-Yo Daily Enrichment Cycle

This guide covers day-to-day operations for the Yo-Yo daily enrichment cycle: checking
that the cycle ran, reading logs, using the kill switch, and arming the Phase 6 training
trigger. For background on what the cycle does and why, see
`topic-yoyo-daily-enrichment-cycle.md`.

## Normal daily operations

The cycle fires automatically at **10:00 AM PDT (Vancouver)** / 17:00 UTC every day.
No operator action is required for a normal enrichment-only run.

**Check the timer and next scheduled fire:**
```bash
systemctl list-timers local-yoyo-daily.timer
```

**Read the log from the most recent cycle:**
```bash
cat $(ls -t /srv/foundry/data/yoyo-cycle-logs/ | head -1 | xargs -I{} echo /srv/foundry/data/yoyo-cycle-logs/{})
```

**Watch a cycle in progress (run before 10:00 AM to see it live):**
```bash
journalctl -u local-yoyo-daily -f
```

A healthy cycle completes in under 45 minutes and ends with lines similar to:
```
=== CYCLE COMPLETE ===
Total elapsed:   2180s (36m 20s)
Entity count:    9692 → 9714
Enrichment DPO:  0 → 9 (+9)
VM final status: TERMINATED
```

## Checking VM status

The VM should be `TERMINATED` at any time outside the 45-minute cycle window.

**Check current VM status:**
```bash
gcloud compute instances describe yoyo-batch --zone us-central1-a --format="get(status)"
```

Expected outside a cycle: `TERMINATED`
Expected during a cycle: `RUNNING`

**If the VM is RUNNING outside a cycle window**, stop it manually:
```bash
gcloud compute instances stop yoyo-batch --zone us-central1-a
```

Then check whether the kill switch should be activated to prevent the next scheduled
start while you investigate.

## Kill switch

The kill switch immediately prevents the next cycle from starting the VM. It does not
stop a VM that is already running.

**Activate (suppress all VM starts until explicitly reversed):**
```bash
touch /srv/foundry/data/yoyo-disabled
```

**Deactivate (resume normal daily cycles):**
```bash
rm /srv/foundry/data/yoyo-disabled
```

**Check current state:**
```bash
ls /srv/foundry/data/yoyo-disabled 2>/dev/null && echo "KILL SWITCH ACTIVE" || echo "normal operation"
```

The kill switch takes effect at the start of the next cycle. It does not affect a cycle
that is already running.

## Emergency VM stop

If the VM is running unexpectedly or a cycle must be interrupted:

```bash
gcloud compute instances stop yoyo-batch --zone us-central1-a
```

Then activate the kill switch if you want to prevent the next scheduled start:
```bash
touch /srv/foundry/data/yoyo-disabled
```

## Checking corpus and enrichment output

**Count enrichment DPO pairs accumulated since last corpus reset:**
```bash
ls /home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/service-fs/data/training-corpus/feedback/enrichment-*.jsonl 2>/dev/null | wc -l
```

**Spot-check the most recent pair (verify it contains document text, not prompt examples):**
```bash
cat $(ls -t /home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/service-fs/data/training-corpus/feedback/enrichment-*.jsonl 2>/dev/null | head -1) | python3 -c "
import sys, json
d = json.load(sys.stdin)
print('PROMPT (first 200 chars):', d['prompt'][:200])
print('CHOSEN:', d['chosen'][:120])
print('REJECTED:', d['rejected'][:120])
"
```

A healthy pair shows document text in the prompt (not names like "Jane Smith"),
non-empty `chosen` and `rejected` values that differ from each other.

**Check entity count in the DataGraph:**
```bash
curl -sf http://127.0.0.1:9081/healthz | python3 -c "import sys,json; d=json.load(sys.stdin); print(f\"entities: {d['entity_count']}\")"
```

## Training markers

Training markers are written by `corpus-threshold.py` when accumulated data exceeds
the configured threshold. They are a prerequisite for Phase 6 (LoRA training).

**Check how many markers are present:**
```bash
ls /srv/foundry/data/training-pending/*.json 2>/dev/null | wc -l
```

Markers accumulate daily. They are not consumed by the cycle — they persist until a
training run is completed and the operator clears them.

## Arming Phase 6 (LoRA training trigger)

Phase 6 requires three conditions to be met simultaneously. Check all three before
creating the approval tag.

**Gate 1 — Training markers present:**
```bash
ls /srv/foundry/data/training-pending/*.json | wc -l   # must be > 0
```

**Gate 2 — ML libraries installed on the batch VM** (requires VM to be running):
```bash
ssh -i ~/.ssh/google_compute_engine mathew@10.128.0.24 \
  "~/training-venv/bin/python3 -c 'import trl; print(trl.__version__)'"
```
Expected: prints a version number such as `1.5.1`.

**Gate 3 — Approval tag for today:**
```bash
# Create the tag (run on the morning of the day you want training to fire):
echo "supervised run" > /srv/foundry/data/training-approved/coding-lora-$(date +%Y-%m-%d).tag

# Verify it exists:
ls /srv/foundry/data/training-approved/coding-lora-$(date +%Y-%m-%d).tag
```

The approval tag is date-specific. It must be created on the same calendar day that
the 10:00 AM cycle fires. Creating it the night before will not work — the tag uses
today's date and the cycle checks at fire time.

**After a training run**, the adapter is written to:
```bash
ls /home/mathew/adapters/apprenticeship-pointsav-incremental/
```

Monitor Phase 6 during the cycle:
```bash
journalctl -u local-yoyo-daily -f | grep -E "Phase 6|train|adapter"
```

## Changing the schedule

The cycle currently fires at 17:00 UTC (10:00 AM PDT). The plan is to move it to
02:30 UTC (7:30 PM PDT) after 1–2 weeks of confirmed stable operation.

**To change the schedule:**
1. Edit `/srv/foundry/infrastructure/local-yoyo-daily.timer` — update `OnCalendar=`
2. Copy to `/etc/systemd/system/local-yoyo-daily.timer`
3. Run `sudo systemctl daemon-reload`
4. Verify: `systemctl list-timers local-yoyo-daily.timer`

## Cost reference

| Scenario | Cost |
|---|---|
| VM running | ~$0.71/hr |
| VM TERMINATED | $0.00 |
| Normal 45-minute cycle | ~$0.53 |
| Full 45-minute cycle with Phase 6 training | ~$0.53 (training runs within the same 45-min window) |
| VM running unexpectedly for 24 hours | ~$17.00 |
| Kill switch active, no cycles | $0.00 |

## Health check summary

Run this at the start of any session that involves the Yo-Yo pipeline:

```bash
# Doorman tier status
curl -s http://127.0.0.1:9080/readyz | python3 -c "import sys,json; d=json.load(sys.stdin); print(f\"tier_a={d.get('has_local')} tier_b={d.get('has_yoyo')}\")"

# DataGraph entity count
curl -sf http://127.0.0.1:9081/healthz | python3 -c "import sys,json; d=json.load(sys.stdin); print(f\"entities={d.get('entity_count')}\")"

# Enrichment corpus size
ls /home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/service-fs/data/training-corpus/feedback/enrichment-*.jsonl 2>/dev/null | wc -l

# Training markers
ls /srv/foundry/data/training-pending/*.json 2>/dev/null | wc -l

# VM status
gcloud compute instances describe yoyo-batch --zone us-central1-a --format="get(status)" 2>/dev/null

# Kill switch state
ls /srv/foundry/data/yoyo-disabled 2>/dev/null && echo "KILL SWITCH ACTIVE" || echo "normal operation"
```
