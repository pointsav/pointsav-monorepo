---
artifact: guide
schema: foundry-draft-v1
draft-id: project-data-20260614-jennifer-2-migration-stack
title: jennifer-2 Migration Stack — Operator Guide
language_protocol: project-editorial
route_to: project-editorial
destination: woodfine-fleet-deployment
created: 2026-06-14
status: staged
research_trail_source: BRIEF-jennifer-2-ingest-pipeline.md session 6; live stack operation 2026-06-14
research_trail_method: operational — commands verified against running services
research_trail_data: /tmp/service-fs-j2.log; /tmp/service-extraction-j2.log; /tmp/service-input-j2.log
research_trail_claims: Stack started and 59 documents migrated manually 2026-06-14; Phase 2 reference dir exhausted at offset 80
research_trail_gaps: Stack is not systemd-managed; dies on VM reboot
bcsc_class: internal
forbidden_terms_cleared: true
---

The jennifer-2 migration stack consists of three processes that move reference documents from the jennifer-1 research archive into the entity extraction pipeline. This guide covers starting the stack, running manual migrations, operating the nightly cron driver, and restarting after a VM reboot.

## Prerequisites

Confirm the following before starting the stack:

```bash
# Doorman must be running (Tier B circuit state affects migration gate)
curl -sf http://127.0.0.1:9080/readyz | python3 -m json.tool | head -20

# service-content (DataGraph / local-content.service) must be running
curl -sf http://127.0.0.1:9081/health

# Binaries must be present
ls /srv/foundry/cargo-target/mathew/debug/service-fs
ls /srv/foundry/cargo-target/mathew/debug/service-extraction
ls /srv/foundry/cargo-target/mathew/debug/service-input
```

If binaries are missing, rebuild from the project-data clone:

```bash
cd /srv/foundry/clones/project-data
cargo build -p service-fs -p service-extraction -p service-input 2>&1 | tail -10
```

## Starting the Stack

All three processes write to log files in `/tmp/`. Start them in order.

### 1. service-fs (jennifer-2 WORM + drop directory)

```bash
J2=/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer-2

env FS_BIND_ADDR=127.0.0.1:9103 \
    FS_MODULE_ID=jennifer \
    FS_LEDGER_ROOT=$J2/service-fs/worm \
    FS_WATCH_DROP_DIR=$J2/service-extraction/watch \
    /srv/foundry/cargo-target/mathew/debug/service-fs >> /tmp/service-fs-j2.log 2>&1 &

echo "service-fs j2 pid=$!"
# Confirm it is up:
curl -sf --retry 5 --retry-delay 1 http://127.0.0.1:9103/healthz && echo "service-fs j2 UP"
```

### 2. service-extraction (jennifer-2 watcher → jennifer-1 live corpus)

```bash
J1=/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer
J2=/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer-2

env EXTRACTION_WATCH_DIR=$J2/service-extraction/watch \
    EXTRACTION_EMIT_CORPUS_DIR=$J1/service-fs/data/service-content/ledgers \
    EXTRACTION_CORPUS_MODULE_ID=jennifer \
    /srv/foundry/cargo-target/mathew/debug/service-extraction >> /tmp/service-extraction-j2.log 2>&1 &

echo "service-extraction j2 pid=$!"
```

Drop files arrive in `EXTRACTION_WATCH_DIR`. After successful processing, the file moves to `EXTRACTION_WATCH_DIR/processed/`. Emitted CORPUS files land in the jennifer-1 ledgers directory where `local-content.service` picks them up.

### 3. service-input (migration API)

```bash
J2=/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer-2
ASSET_ROOT=/srv/foundry/deployments/cluster-totebox-jennifer

env SERVICE_INPUT_BIND=127.0.0.1:9106 \
    SERVICE_INPUT_MODULE_ID=jennifer \
    SERVICE_INPUT_FS_ENDPOINT=http://127.0.0.1:9103 \
    SERVICE_INPUT_DEST_ARCHIVE=cluster-totebox-jennifer-2 \
    SERVICE_INPUT_REFERENCE_ROOT=$ASSET_ROOT \
    SERVICE_INPUT_REFERENCE_DIR=$J2/service-research/reference \
    SERVICE_INPUT_JENNIFER2_ROOT=$J2 \
    SERVICE_INPUT_LEDGER=$J2/service-input/ledger.jsonl \
    SERVICE_INPUT_CONTENT_ENDPOINT=http://127.0.0.1:9081 \
    SERVICE_INPUT_RATE_PER_MIN=6 \
    /srv/foundry/cargo-target/mathew/debug/service-input >> /tmp/service-input-j2.log 2>&1 &

echo "service-input pid=$!"
curl -sf --retry 5 --retry-delay 1 http://127.0.0.1:9106/healthz && echo "service-input UP"
```

## Running a Manual Migration Batch

The migration API accepts `POST /v1/migrate` with a batch size and offset. The pipeline processes files from the reference directory in offset order.

```bash
# Single batch of 10 documents starting at offset 0
curl -s -X POST http://127.0.0.1:9106/v1/migrate \
  -H 'Content-Type: application/json' \
  -d '{"batch_size":10,"offset":0}' | python3 -m json.tool
```

Response fields:
- `processed`: documents accepted and emitted to service-fs
- `skipped`: documents already in the ledger (already migrated)
- `offset_next`: starting offset for the next batch

Loop through all documents:

```bash
OFFSET=0
while true; do
  RESP=$(curl -s -X POST http://127.0.0.1:9106/v1/migrate \
    -H 'Content-Type: application/json' \
    -d "{\"batch_size\":10,\"offset\":$OFFSET}")
  PROCESSED=$(python3 -c "import json; print(json.loads('''$RESP''').get('processed',0))")
  SKIPPED=$(python3 -c "import json; print(json.loads('''$RESP''').get('skipped',0))")
  OFFSET=$(python3 -c "import json; print(json.loads('''$RESP''').get('offset_next',0))")
  echo "processed=$PROCESSED skipped=$SKIPPED offset_next=$OFFSET"
  [ "$((PROCESSED + SKIPPED))" -eq 0 ] && echo "Exhausted." && break
done
```

The Phase 2 reference directory was exhausted at offset 80 on 2026-06-14 (59 documents ingested).

## Monitoring

```bash
# Watch extraction activity
tail -f /tmp/service-extraction-j2.log

# Watch service-content picking up CORPUS files
sudo journalctl -u local-content -f --no-pager | grep -E "TIER|GRAPH|WATCHER"

# Count CORPUS files (total — includes all modules)
J1=/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer
ls $J1/service-fs/data/service-content/ledgers/ | wc -l

# Entity count in DataGraph
curl -sf http://127.0.0.1:9081/healthz | python3 -m json.tool | grep entity_count
```

## Nightly Cron Driver

The nightly migration script handles the health gate and DPO loss guard automatically. It is safe to run manually:

```bash
/srv/foundry/clones/project-data/service-input/scripts/nightly-jennifer-migrate.sh
```

The script will skip the run if:
- The Doorman reports Tier A alive and Tier B circuit open — this condition would cause `flush_tier_a()` in service-content to process documents without generating DPO feedback pairs.
- service-input `/v1/calibration-report` returns `go_no_go: stop`.

Register the nightly cron on the VM (Command Session task):

```
0 23 * * * /srv/foundry/clones/project-data/service-input/scripts/nightly-jennifer-migrate.sh
```

The 23:00 UTC start gives the script the full night window before the 05:00 UTC daily build window begins.

## Restarting After VM Reboot

The stack processes are not managed by systemd and do not survive a VM reboot. After reboot, run the three start commands above in order, then verify all three health endpoints respond before starting a migration batch.

The jennifer-1 WORM and CORPUS directories persist across reboots — the ledger at `$J2/service-input/ledger.jsonl` prevents re-migrating already-processed documents.

## SLM_DRAIN_PAUSED

When the apprenticeship queue is growing faster than OLMo 7B can drain it, the Doorman `SLM_DRAIN_PAUSED=true` environment variable pauses drain operations. This frees OLMo 7B for entity extraction without interference from training queue jobs.

Setting this variable requires restarting `local-doorman.service` (Command Session scope — requires `sudo systemctl` access).

To check current apprenticeship queue depth:

```bash
curl -sf http://127.0.0.1:9080/readyz | python3 -m json.tool | grep -i queue
```
