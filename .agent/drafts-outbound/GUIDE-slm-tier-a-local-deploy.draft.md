---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
status: language-cleared
title: "Deploy service-slm Tier A — local inference on the workspace VM"
target_repo: woodfine-fleet-deployment
target_path: woodfine-fleet-deployment/cluster-intelligence/guide-slm-tier-a-local-deploy.md
target_filename: guide-slm-tier-a-local-deploy.md
bcsc_class: no-disclosure-implication
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — service-slm ARCHITECTURE.md, live env config"
research_inline: true
---

# Deploy service-slm Tier A — local inference on the workspace VM

This guide covers deploying the Tier A inference server (`local-slm.service`) and the
Doorman gateway (`local-doorman.service`) on the workspace VM (Totebox). On completion,
the Doorman accepts inference requests and routes them to the local OLMo 7B model.

## Pre-flight

```bash
# Confirm the model binary exists
ls -lh /srv/foundry/clones/project-intelligence/service-slm/model/
# Expect: *.gguf file, ~4 GiB

# Confirm llama-server is installed
which llama-server || which llama.cpp-server

# Confirm the service-slm binary is built
ls /srv/foundry/clones/project-intelligence/service-slm/target/release/slm-doorman-server
```

## Step 1 — Configure the local model service

The model runs as `local-slm.service`. The systemd unit file is in
`service-slm/infrastructure/systemd/local-slm.service`.

```bash
sudo cp service-slm/infrastructure/systemd/local-slm.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now local-slm.service
```

Verify it started:
```bash
systemctl status local-slm.service
curl -s http://127.0.0.1:8080/health
# Expect: {"status":"ok"} or similar
```

## Step 2 — Configure the Doorman

Copy the environment file template:
```bash
sudo cp service-slm/infrastructure/systemd/local-doorman.service /etc/systemd/system/
sudo mkdir -p /etc/local-doorman
sudo cp service-slm/infrastructure/config/local-doorman.env.example /etc/local-doorman/local-doorman.env
```

Edit `/etc/local-doorman/local-doorman.env` with the minimum Tier A configuration:

```bash
# Tier A — local model
SLM_LOCAL_ENDPOINT=http://127.0.0.1:8080
SLM_TIER_A_FIRST=true

# Corpus + queue
SLM_APPRENTICESHIP_ENABLED=true
SLM_APPRENTICESHIP_BASE_DIR=/srv/foundry/data/apprenticeship
SLM_QUEUE_LEASE_EXPIRY_SEC=2100

# Graph context (optional; service-content must be running)
SERVICE_CONTENT_ENDPOINT=http://127.0.0.1:9081

# Audit
SLM_AUDIT_DIR=/srv/foundry/data/audit-ledger

# Tier B and C — leave unset for Tier A only
# SLM_YOYO_BATCH_ENDPOINT=
# SLM_YOYO_EXPRESS_ENDPOINT=
# ANTHROPIC_API_KEY=
```

## Step 3 — Start the Doorman

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now local-doorman.service
journalctl -u local-doorman.service -f
```

Look for startup log lines confirming Tier A is live:
```
INFO slm_doorman: tier A endpoint: http://127.0.0.1:8080
INFO slm_doorman: serving on :9080
```

## Step 4 — Verify end-to-end

```bash
# Health check
curl -s http://127.0.0.1:9080/healthz
# Expect: {"status":"ok"}

# Readiness check
curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool
# Expect: "ai_available": true, "has_local": true

# Test inference
curl -s http://127.0.0.1:9080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"any","messages":[{"role":"user","content":"Reply with: hello"}],"max_tokens":10}'
# Expect: response with tier="local" in Doorman logs
```

## Step 5 — Install the post-commit hook (apprenticeship)

To start accumulating training data from commits in this archive:

```bash
cp service-slm/scripts/git-post-commit-hook.sh .git/hooks/post-commit
chmod +x .git/hooks/post-commit
```

Repeat in every archive where training data should be collected.

## Troubleshooting

| Symptom | Check |
|---|---|
| `local-slm.service` fails to start | Check GPU/CPU memory: `free -h`; model needs ~4 GiB |
| `readyz` shows `has_local: false` | Check `SLM_LOCAL_ENDPOINT`; verify llama-server is running |
| Inference returns 503 | Check Doorman logs: `journalctl -u local-doorman.service --since=-5m` |
| Queue drain stalled | Check `SLM_DRAIN_PAUSED` in env file; verify not set to `true` |
