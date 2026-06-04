---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
status: language-cleared
title: "Deploy app-orchestration-slm — the multi-archive GPU fleet broker"
target_repo: woodfine-fleet-deployment
target_path: woodfine-fleet-deployment/cluster-intelligence/guide-orchestration-slm-deploy.md
target_filename: guide-orchestration-slm-deploy.md
bcsc_class: no-disclosure-implication
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — app-orchestration-slm CLAUDE.md, HTTP surface"
research_inline: true
---

# Deploy app-orchestration-slm — the multi-archive GPU fleet broker

app-orchestration-slm is the stateless chassis that brokers Tier B (GPU) inference
for multiple Totebox Archives. Each Totebox Archive's Doorman registers with the
chassis at startup. The chassis proxies requests to the configured GPU nodes and
meters usage per tenant. This guide covers initial deployment on the workspace VM.

**Prerequisite:** Both `yoyo-batch` and `yoyo-express` GPU nodes must be deployed
and reachable before the chassis is started. See the batch and express deploy guides.

**License required:** `app-orchestration-slm` requires the `ORCHESTRATION_LICENSE_TOKEN`
environment variable. Obtain a license token from `software.pointsav.com` before
proceeding.

## Pre-flight

```bash
# Confirm binary is built
ls /srv/foundry/clones/project-intelligence/app-orchestration-slm/target/release/orchestration-slm-server

# Confirm both GPU nodes are reachable
curl -s http://${BATCH_IP}:11434/          # expect "Ollama is running"
curl -s http://${EXPRESS_IP}:11434/        # expect "Ollama is running"
```

## Step 1 — Configure the chassis environment

```bash
sudo mkdir -p /etc/local-orchestration
sudo tee /etc/local-orchestration/local-orchestration.env << EOF
# License
ORCHESTRATION_LICENSE_TOKEN=<your-license-token>

# YoYo nodes
ORCHESTRATION_YOYO_TRAINER_ENDPOINT=http://${BATCH_IP}:11434
ORCHESTRATION_YOYO_TRAINER_BEARER=${BATCH_BEARER}
ORCHESTRATION_YOYO_DEFAULT_ENDPOINT=http://${BATCH_IP}:11434
ORCHESTRATION_YOYO_DEFAULT_BEARER=${BATCH_BEARER}
ORCHESTRATION_YOYO_GRAPH_ENDPOINT=http://${EXPRESS_IP}:11434
ORCHESTRATION_YOYO_GRAPH_BEARER=${EXPRESS_BEARER}

# Bind
ORCHESTRATION_BIND=127.0.0.1:9180

# Module ID for the Woodfine Foundry tenant
ORCHESTRATION_MODULE_ID=woodfine
EOF
```

## Step 2 — Install as a systemd service

```bash
sudo cp app-orchestration-slm/infrastructure/systemd/local-orchestration.service \
  /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now local-orchestration.service
journalctl -u local-orchestration.service -f
```

Look for:
```
INFO orchestration_slm: serving on :9180
INFO orchestration_slm: yoyo fleet: trainer=configured graph=configured
```

## Step 3 — Register the Doorman with the chassis

The Doorman self-registers at startup if `SLM_ORCHESTRATION_ENDPOINT` is set.
Add to `/etc/local-doorman/local-doorman.env`:

```bash
SLM_ORCHESTRATION_ENDPOINT=http://127.0.0.1:9180
SLM_MODULE_ID=woodfine
SLM_ARCHIVE_ID=project-intelligence
SLM_TIER_B_SUBSCRIBED=true
```

Restart the Doorman:
```bash
sudo systemctl restart local-doorman.service
```

## Step 4 — Verify fleet registration

```bash
# Check fleet status
curl -s http://127.0.0.1:9180/v1/fleet | python3 -m json.tool
# Expect: "fleet_members": 1 (the Doorman just registered)

# Check readiness
curl -s http://127.0.0.1:9180/readyz | python3 -m json.tool
# Expect: yoyo nodes reachable

# Test a proxied inference request through the chassis
curl -s http://127.0.0.1:9180/v1/yoyo/proxy \
  -H "Authorization: Bearer woodfine" \
  -H "X-Foundry-Module-ID: woodfine" \
  -H "Content-Type: application/json" \
  -d '{"model":"olmo3","messages":[{"role":"user","content":"ping"}],"max_tokens":5}'
```

## Kill switch at chassis level

The chassis exposes per-label kill switches:

```bash
# Pause the trainer (batch) node
curl -X POST http://127.0.0.1:9180/v1/flow/kill/trainer -d '{"closed":true}'

# Resume
curl -X POST http://127.0.0.1:9180/v1/flow/kill/trainer -d '{"closed":false}'

# Global pause (all nodes)
curl -X POST http://127.0.0.1:9180/v1/flow/kill -d '{"closed":true}'
```

## Stateless design

The chassis holds no persistent state. The fleet registry is rebuilt from
`POST /v1/discovery/register` calls that each Doorman makes at startup. Metering
data is in-process only; the source of truth for cost accounting remains in each
Totebox Archive's own audit ledger.

If the chassis is restarted, all Doorman instances will re-register within their
next startup or health cycle. No data is lost.
