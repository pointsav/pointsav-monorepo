---
schema: foundry-guide-v1
title: "Operating the Yo-Yo Tier B Deployment"
category: vault-privategit-source
status: stable
last_edited: 2026-04-30
editor: pointsav-engineering
audience: operators
---



This guide describes the operator-facing tasks for the Yo-Yo (Tier B) deployment on the workspace VM. It complements `infrastructure/yoyo-manual/README.md`, which is the bootstrap runbook. This guide covers operations after first boot.

## Daily operational state

The Yo-Yo deployment consists of three runtime components on two GCE VMs:

On the **workspace VM** (`foundry-workspace`, us-west1-a, e2-standard-4):
- `local-doorman.service` — Rust binary at `/usr/local/bin/slm-doorman-server`. Routes inference requests to Tier A, Tier B, or Tier C. Holds bearer tokens. Writes audit ledger.
- `yoyo-idle-monitor.timer` and `yoyo-idle-monitor.service` — bash cost-cap monitor at `/srv/foundry/bin/yoyo-idle-monitor.sh`. Polls Yo-Yo every 5 minutes; stops the VM after 30 minutes idle.

On the **Yo-Yo VM** (`yoyo-tier-b-1`, us-west1-a, g2-standard-4 with one L4 GPU):
- `yoyo-llama-server.service` — `llama.cpp` C++ binary at `/opt/llama.cpp/build/bin/llama-server`. Serves OLMo 2 32B Instruct Q4_K_S over HTTP at `127.0.0.1:8080` and the VM's internal IP. Bearer-authenticated.

The Doorman knows about Yo-Yo via four environment variables in `/etc/local-doorman/local-doorman.env`:

```
SLM_YOYO_ENDPOINT=http://10.138.0.21:8080
SLM_YOYO_BEARER=<64-character-hex-token>
SLM_YOYO_MODEL=olmo-2-32b-instruct-q4
SLM_YOYO_HOURLY_USD=0.71
```

If the Yo-Yo VM is reprovisioned and the internal IP changes, update `SLM_YOYO_ENDPOINT` and restart the Doorman:

```bash
sudo sed -i "s|^SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=http://NEW_IP:8080|" /etc/local-doorman/local-doorman.env
sudo systemctl restart local-doorman.service
```

Verify the Doorman recognises Yo-Yo:

```bash
curl -sS http://127.0.0.1:9080/readyz
```

The response should report `"has_yoyo": true`. If it reports `"has_yoyo": false`, the Doorman did not load the new endpoint; check `/etc/local-doorman/local-doorman.env` for an empty or malformed `SLM_YOYO_ENDPOINT` value.

## Verifying the deployment is healthy

Three checks confirm the deployment is operational:

```bash
# Doorman is up
curl -sS http://127.0.0.1:9080/readyz
# Expected: {"ready": true, "has_local": true, "has_yoyo": true, ...}

# Yo-Yo VM is in RUNNING state
gcloud compute instances describe yoyo-tier-b-1 \
  --project=woodfine-node-gcp-free --zone=us-west1-a \
  --format='value(status)'
# Expected: RUNNING

# Yo-Yo llama-server responds
YOYO_BEARER=$(grep '^SLM_YOYO_BEARER=' /etc/local-doorman/local-doorman.env | cut -d= -f2-)
curl -sS -H "Authorization: Bearer $YOYO_BEARER" http://10.138.0.21:8080/health
# Expected: {"status":"ok"}
```

If `/health` returns `{"error":"Loading model"}`, the Yo-Yo just started and the model is loading into GPU memory. Wait 60-180 seconds and retry. If `/health` does not respond at all, the Yo-Yo VM may be in idle-shutdown.

## When Yo-Yo has been idle-shut-down

The cost-cap monitor stops the Yo-Yo VM after 30 minutes with no active inferences. While Yo-Yo is stopped:

- `gcloud compute instances describe yoyo-tier-b-1 ... --format='value(status)'` returns `TERMINATED`.
- `/health` curls fail with connection-refused.
- The Doorman continues to accept inference requests; routes them to Tier A only.
- The §7C apprenticeship brief queue continues to accumulate. No briefs are lost.

This is the expected steady state during off-hours. To wake Yo-Yo for active development:

```bash
gcloud compute instances start yoyo-tier-b-1 \
  --project=woodfine-node-gcp-free --zone=us-west1-a
```

The VM takes approximately 90 seconds to start. The model takes another 60-180 seconds to cold-load into the L4 GPU. Total wake-up: approximately 3-4 minutes.

After Yo-Yo is up, the queued briefs from the idle window drain through the Doorman drain worker (30-second poll interval). A 50-brief backlog clears in approximately 5-10 minutes once Yo-Yo is responsive.

## When the cost cap fires unexpectedly

The 30-minute idle threshold is configurable. If you find Yo-Yo stopping during your active development session, increase the threshold via the systemd service:

```bash
sudo systemctl edit yoyo-idle-monitor.service
```

Add:

```
[Service]
Environment=IDLE_SHUTDOWN_MINUTES=60
```

Save. Reload:

```bash
sudo systemctl daemon-reload
```

The next monitor poll uses the new threshold. To revert, remove the override file at `/etc/systemd/system/yoyo-idle-monitor.service.d/override.conf` and `daemon-reload`.

For long-running sessions where you want the cap entirely off, stop the timer for the duration of the session:

```bash
sudo systemctl stop yoyo-idle-monitor.timer
# ... your work ...
sudo systemctl start yoyo-idle-monitor.timer
```

The monitor will not stop a running Yo-Yo while the timer is stopped. Re-enable it before stepping away.

## When Yo-Yo refuses to start

If `gcloud compute instances start` fails or the VM enters `RUNNING` but `/health` never becomes responsive, the boot has hit a recoverable issue. The most common are:

- **Spot preemption** (does not apply to current deployment; we use on-demand). The original spot model would have entered `TERMINATED (PREEMPTED)`. Current deployment is on-demand; this case does not occur.
- **CUDA driver mismatch after kernel update**. The DL VM image carries matched driver-and-kernel; auto-updates can drift. Check `/var/log/syslog` and `nvidia-smi` on the Yo-Yo VM via `gcloud compute ssh`. If `nvidia-smi` reports a kernel mismatch, the VM needs a fresh image (`gcloud compute instances delete` then re-fire `infrastructure/yoyo-manual/README.md` Step 2).
- **Disk full** (rare in steady state; does occur during fresh provisions if startup.sh is mid-build). `df -h /` on the Yo-Yo VM. Boot disk is 150 GB; steady-state usage is approximately 76 GB.

If `llama-server` is not running but the VM is up, start it manually:

```bash
gcloud compute ssh yoyo-tier-b-1 \
  --project=woodfine-node-gcp-free --zone=us-west1-a \
  --command='sudo systemctl start yoyo-llama-server.service'
```

If it fails to start, check the journal:

```bash
gcloud compute ssh yoyo-tier-b-1 \
  --project=woodfine-node-gcp-free --zone=us-west1-a \
  --command='sudo journalctl -u yoyo-llama-server.service -n 50 --no-pager'
```

The most informative lines are typically near the end (model-loading errors).

## When the audit ledger needs review

The Doorman writes an audit-ledger line per request. The ledger lives at `/var/lib/local-doorman/audit/<YYYY-MM>.jsonl` on the workspace VM. A monthly review verifies:

- Every Tier B request was routed correctly (`tier: yoyo`).
- Costs accumulated in the month against expectations.
- No requests carried bearer tokens in their bodies (`sanitised_outbound: true`).

```bash
sudo cat /var/lib/local-doorman/audit/$(date +%Y-%m).jsonl | jq -c '{ts: .timestamp_utc, tier: .tier, ms: .inference_ms, cost: .cost_usd, status: .completion_status}' | tail -50
```

A typical month for the workspace dogfood deployment carries approximately 100-500 entries. Bulk Tier B activity (e.g., editorial-pipeline batch refinement) increases this by 10-50x.

## When Yo-Yo needs a model upgrade

The current model is `OLMo-2-0325-32B-Instruct-Q4_K_S`. AllenAI publishes successive OLMo releases; when an OLMo 3 32B Instruct or Think Q4 GGUF appears, swap is one configuration change.

On the Yo-Yo VM:

```bash
# Download the new GGUF (replace URL)
sudo aria2c \
  --max-connection-per-server=4 --split=4 \
  --max-tries=0 --retry-wait=10 --continue=true \
  --dir=/var/lib/yoyo/models \
  --out=NEW-MODEL.gguf \
  https://huggingface.co/allenai/NEW-MODEL/resolve/main/NEW-MODEL.gguf

# Update the systemd unit's --model flag
sudo sed -i "s|--model /var/lib/yoyo/models/.*\.gguf|--model /var/lib/yoyo/models/NEW-MODEL.gguf|" /etc/systemd/system/yoyo-llama-server.service

# Update the --alias flag (Doorman expects this name to match SLM_YOYO_MODEL)
sudo sed -i "s|--alias .*|--alias NEW-MODEL-ALIAS|" /etc/systemd/system/yoyo-llama-server.service

# Restart
sudo systemctl daemon-reload
sudo systemctl restart yoyo-llama-server.service
```

On the workspace VM:

```bash
sudo sed -i "s|^SLM_YOYO_MODEL=.*|SLM_YOYO_MODEL=NEW-MODEL-ALIAS|" /etc/local-doorman/local-doorman.env
sudo systemctl restart local-doorman.service
```

Delete the old GGUF after verifying the new model loads cleanly.

The disk has approximately 70 GB free; one extra 20-30 GB GGUF fits. If you swap a larger model (e.g., 70B Q4), bump the boot disk first.

## When Yo-Yo is no longer needed

To decommission entirely:

```bash
# 1. Disable + stop the cost-cap monitor (no longer needed; nothing to monitor)
sudo systemctl disable --now yoyo-idle-monitor.timer

# 2. Remove the Doorman Yo-Yo wiring
sudo sed -i 's|^SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=|' /etc/local-doorman/local-doorman.env
sudo sed -i 's|^SLM_YOYO_BEARER=.*|SLM_YOYO_BEARER=|' /etc/local-doorman/local-doorman.env
sudo systemctl restart local-doorman.service
# /readyz should now report has_yoyo=false

# 3. Delete the Yo-Yo VM
gcloud compute instances delete yoyo-tier-b-1 \
  --project=woodfine-node-gcp-free --zone=us-west1-a --quiet

# 4. Delete the firewall rule
gcloud compute firewall-rules delete yoyo-tier-b-from-workspace --quiet
```

The workspace VM continues to operate Tier A only. Briefs that needed Tier B (greater than 500 characters) accumulate in the queue; if Yo-Yo never returns, operator-presence is required to manually drain or expire those briefs.

## When something does not match this guide

This guide reflects the workspace v0.1.91 state. If you find behaviour that diverges from what is documented here, the divergence is either a regression to investigate or an enhancement to backfill into this guide. Either way, surface via `~/Foundry/.claude/inbox.md` (Master inbox) so the next workspace session can address it.


---

## Provenance

Authored by Master Claude at workspace v0.1.91 (2026-04-30) by direct observation of live operational state. Validated against `infrastructure/yoyo-manual/README.md` (bootstrap runbook), `bin/yoyo-idle-monitor.sh` (cost-cap monitor), and the workspace v0.1.81 → v0.1.91 commit chain. Cold-start timing measured empirically; disk usage calculated from live provisioning runs; NVIDIA driver compatibility confirmed against DL VM image `common-cu129-ubuntu-2404-nvidia-580`.
