---
mailbox: outbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: task-project-intelligence
to: master
re: Yo-Yo #1 Rust hardening complete — operator actions required for deploy
created: 2026-05-06T04:00Z
---

## What landed (commit 47a230e, Peter Woodfine)

Tier B resilience stack is code-complete and 175/175 tests passing:

- **B1** — 60 s reqwest socket + 90 s tokio outer deadline; `TierBTimeout` error type
- **B2** — `circuit_breaker.rs`: 3-state breaker, 5-failure threshold, 5 min cooldown
- **B3** — Background `/health` probe every 30 s; `health_up: Arc<AtomicBool>` on client
- **B4** — `router.rs` Tier A fallback: health/circuit fast-path, transient-error catch,
  `X-Foundry-Tier-Used` response header, `write_audit` uses `resp.tier_used` (actual tier)
- **B5** — `idle_monitor.rs`: polls `/metrics` every 5 min, calls GCP `instances.stop`
  via metadata ADC token after `SLM_YOYO_IDLE_MINUTES` idle (default 30)
- **B6** — `send_once()` confirmed as auth boundary (inline 401/403 refresh, no refactor)
- **B7** — `#[tracing::instrument]` on `complete()` with latency_ms, circuit_open, cold_start

## Operator actions needed before Yo-Yo goes live

### Track A — Create the VM (operator only)

```bash
# A1 — Create Spot VM
gcloud compute instances create yoyo-tier-b-1 \
  --project=woodfine-node-gcp-free \
  --zone=us-west1-a \
  --machine-type=g2-standard-4 \
  --provisioning-model=SPOT \
  --instance-termination-action=STOP \
  --accelerator=type=nvidia-l4,count=1 \
  --maintenance-policy=TERMINATE \
  --boot-disk-size=100GB \
  --boot-disk-type=pd-ssd \
  --image-family=ubuntu-2204-lts \
  --image-project=ubuntu-os-cloud \
  --metadata-from-file=startup-script=/home/mathew/Foundry/infrastructure/yoyo-manual/startup.sh \
  --tags=yoyo-slm

# A2 — Attach daily boot schedule (17:00 Pacific = 01:00 UTC)
gcloud compute resource-policies create instance-schedule yoyo-daily-boot \
  --project=woodfine-node-gcp-free \
  --region=us-west1 \
  --vm-start-schedule="0 1 * * *" \
  --timezone="America/Los_Angeles"

gcloud compute instances add-resource-policies yoyo-tier-b-1 \
  --project=woodfine-node-gcp-free \
  --zone=us-west1-a \
  --resource-policies=yoyo-daily-boot

# A3 — Retrieve IP and bearer token (after ~20 min boot)
gcloud compute instances describe yoyo-tier-b-1 \
  --zone=us-west1-a --project=woodfine-node-gcp-free \
  --format='get(networkInterfaces[0].accessConfigs[0].natIP)'

gcloud compute ssh yoyo-tier-b-1 --zone=us-west1-a --project=woodfine-node-gcp-free \
  --command="sudo cat /etc/yoyo-bearer"
```

### Track C — Wire env vars once IP + bearer are known (C1)

Add to `/etc/local-doorman/local-doorman.env`:
```
SLM_YOYO_ENDPOINT=http://<VM_EXTERNAL_IP>:8080
SLM_YOYO_BEARER=<from /etc/yoyo-bearer on VM>
SLM_YOYO_MODEL=OLMo-2-0325-32B-Instruct-Q4_K_S
SLM_YOYO_HOURLY_USD=0.18
SLM_APPRENTICESHIP_ENABLED=true
SLM_YOYO_IDLE_MINUTES=30
SLM_YOYO_GCP_PROJECT=woodfine-node-gcp-free
SLM_YOYO_GCP_ZONE=us-west1-a
SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-1
```

### C2 — Rebuild and install binary

```bash
cd /srv/foundry/clones/project-intelligence/service-slm
cargo build --release -p slm-doorman-server
sudo cp target/release/slm-doorman-server /usr/local/bin/local-doorman
sudo systemctl daemon-reload && sudo systemctl restart local-doorman
```

### C3/C4 — Smoke tests

```bash
# Tier B inference
curl -s http://127.0.0.1:9080/v1/chat/completions \
  -H "X-Foundry-Module-ID: test" -H "X-Foundry-Complexity: high" \
  -d '{"messages":[{"role":"user","content":"ping"}],"max_tokens":20}' \
  | jq '{tier: .["x-foundry-tier-used"], model: .model}'
# → x-foundry-tier-used: yoyo

# Tier A fallback (stop VM first)
gcloud compute instances stop yoyo-tier-b-1 --zone=us-west1-a --project=woodfine-node-gcp-free
curl -s http://127.0.0.1:9080/v1/chat/completions \
  -H "X-Foundry-Module-ID: test" -H "X-Foundry-Complexity: high" \
  -d '{"messages":[{"role":"user","content":"ping"}],"max_tokens":20}' \
  | jq '{tier: .["x-foundry-tier-used"]}'
# → x-foundry-tier-used: local (fallback active)
```

### C5 — Update guide-operating-yoyo.md

Needs VM IP, bearer, GCP project/zone/instance filled in from A3 output.
File: `woodfine-fleet-deployment/vault-privategit-source/guide-operating-yoyo.md`

## Ready state

The binary is gated only on the VM existing and env vars being set.
The idle monitor auto-starts when `SLM_YOYO_GCP_PROJECT` is in env.
Stage-6 promote (C7) can follow once smoke tests pass.

