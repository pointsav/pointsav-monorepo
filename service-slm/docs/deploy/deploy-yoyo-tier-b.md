# Deploy Yo-Yo Tier B — Operator Runbook

Covers the full setup sequence for Yo-Yo #1 (`yoyo-tier-b-1`): image build,
infrastructure provisioning, Doorman wiring, and daily operations. After this
runbook is complete the VM starts every night at 02:00 UTC, drains the
apprenticeship brief queue autonomously, and stops itself when idle.

---

## Prerequisites

Before running any steps, confirm:

- [ ] `woodfine-node-gcp-free` GCP project created with billing enabled
- [ ] Compute Engine API enabled in `woodfine-node-gcp-free`
- [ ] L4 GPU quota approved in `us-west1` (request via GCP console — Quota: `NVIDIA_L4_GPUS`, region `us-west1`)
- [ ] `packer` ≥ 1.10 installed on the machine where you run the build
- [ ] `tofu` (OpenTofu) ≥ 1.6 installed
- [ ] `gcloud` CLI authenticated as an account with `roles/owner` on `woodfine-node-gcp-free`
- [ ] OLMo 3 32B-Think Q4 weights downloaded (~20 GB GGUF file)

---

## Step 1 — Build the GCE image

Run from `service-slm/compute/packer/`:

```bash
cd service-slm/compute/packer/
packer init .
packer build yoyo-image.pkr.hcl
```

Packer will:
1. Launch a temporary `g2-standard-4` VM in `us-central1-a`
2. Install CUDA 12, vLLM ≥ 0.12, Nginx
3. Publish the resulting image to the `slm-yoyo` family in `woodfine-node-gcp-free`

Build takes ~20–30 min (most time is CUDA install).

Optional: override zone or project:
```bash
packer build -var project_id=woodfine-node-gcp-free -var zone=us-west1-c yoyo-image.pkr.hcl
```

---

## Step 2 — Provision infrastructure

Run from `service-slm/compute/opentofu/`:

```bash
cd service-slm/compute/opentofu/
tofu init
tofu apply \
    -var bearer_token="$(openssl rand -hex 32)" \
    -var workspace_ip="$(curl -sf https://ifconfig.me)"
```

Save the generated bearer token — you will need it in Step 5. OpenTofu creates:
- Instance Schedule (`yoyo-tier-b-1-nightly-start`) — starts VM at 02:00 UTC nightly
- Persistent SSD data disk (`yoyo-tier-b-1-weights`, 100 GB)
- `g2-standard-4` Spot VM with L4 GPU, attached to the weights disk
- Firewall rule allowing only the workspace IP on port 9443
- IAM binding granting the workspace VM SA permission to stop the Yo-Yo

After `tofu apply` completes, note the outputs:
```
yoyo_external_ip = "35.x.x.x"
weights_disk_name = "yoyo-tier-b-1-weights"
```

---

## Step 3 — Upload model weights

SSH into the VM and prepare the weights directory:

```bash
gcloud compute ssh yoyo-tier-b-1 --zone us-central1-a --project woodfine-node-gcp-free \
    -- "sudo mkdir -p /data/weights && sudo chmod 777 /data/weights"
```

Upload the GGUF file (adjust the local path):

```bash
gcloud compute scp \
    /path/to/olmo-3-32b-think-q4.gguf \
    yoyo-tier-b-1:/data/weights/olmo-3-32b-think-q4.gguf \
    --zone us-central1-a \
    --project woodfine-node-gcp-free
```

Upload takes ~10–15 min over a GCP internal network.

---

## Step 4 — Reboot the VM once to run rc.local

The startup script (`/etc/rc.local`) mounts the weights disk and writes the Nginx
auth map on first boot. Trigger it now so the image is fully configured:

```bash
gcloud compute instances stop  yoyo-tier-b-1 --zone us-central1-a --project woodfine-node-gcp-free
gcloud compute instances start yoyo-tier-b-1 --zone us-central1-a --project woodfine-node-gcp-free
```

Allow ~3 min for the VM to boot and vLLM to load the model.

Verify vLLM is reachable via Nginx:

```bash
BEARER=<token-from-step-2>
IP=<yoyo_external_ip-from-step-2>

curl -k -s -o /dev/null -w "%{http_code}" \
    -H "Authorization: Bearer ${BEARER}" \
    "https://${IP}:9443/health"
# Expected: 200
```

---

## Step 5 — Wire Doorman env vars

On the workspace VM, add to `/etc/local-doorman/local-doorman.env`:

```bash
# Tier B — Yo-Yo #1
SLM_YOYO_ENDPOINT=https://<yoyo_external_ip>:9443
SLM_YOYO_BEARER=<bearer-token-from-step-2>
SLM_YOYO_MODEL=Olmo-3-1125-32B-Think
SLM_YOYO_HOURLY_USD=0.84
SLM_YOYO_METRICS_KEY=vllm:num_requests_running

# Idle monitor — all four required; absent any one, auto-stop is disabled
SLM_YOYO_GCP_PROJECT=woodfine-node-gcp-free
SLM_YOYO_GCP_ZONE=us-central1-a
SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-1
SLM_YOYO_IDLE_MINUTES=30
```

---

## Step 6 — Restart Doorman

```bash
sudo systemctl restart local-doorman
```

Verify Tier B is live:

```bash
curl -s http://127.0.0.1:9080/readyz | jq '{has_yoyo, tier_b_circuit}'
# Expected: { "has_yoyo": true, "tier_b_circuit": "closed" }
```

If the circuit shows `open`, check Doorman logs:
```bash
journalctl -u local-doorman -f | grep -i yoyo
```

The health probe runs every 30 seconds; the circuit closes within 30 s of vLLM
reporting healthy.

---

## Step 7 — Smoke test the nightly drain

Start the VM on-demand (it was stopped automatically at the end of Step 4 if
vLLM went idle — or stop it manually and restart):

```bash
./scripts/start-yoyo.sh
```

Push one test shadow brief:

```bash
curl -s -X POST http://127.0.0.1:9080/v1/shadow \
    -H "Content-Type: application/json" \
    -H "X-Foundry-Module-ID: smoke-test" \
    -d '{
      "brief": {
        "task": "Explain the idle monitor drain loop in one sentence.",
        "scope": { "files": [] },
        "acceptance_test": "Must mention circuit breaker."
      },
      "actual_diff": ""
    }' | jq .status
# Expected: "queued" or "dispatched"
```

Watch the drain in Doorman logs:

```bash
journalctl -u local-doorman -f | grep -E "drain|yoyo|idle"
```

After the brief is dispatched, leave the VM running. In ~30–35 min the idle
monitor fires and calls `instances.stop`. Confirm in GCP Console or:

```bash
gcloud compute instances describe yoyo-tier-b-1 \
    --zone us-central1-a --project woodfine-node-gcp-free \
    --format="value(status)"
# Expected: TERMINATED
```

---

## Step 8 — Enable apprenticeship (separate operator step)

Once Tier B is verified, set in `/etc/local-doorman/local-doorman.env`:

```bash
SLM_APPRENTICESHIP_ENABLED=true
```

Then restart Doorman. Apprenticeship briefs will queue automatically and drain
each night when the VM starts.

---

## Daily operations

| Task | Command |
|---|---|
| Start VM on-demand | `./scripts/start-yoyo.sh` |
| Stop VM immediately | `./scripts/stop-yoyo.sh` |
| Check VM state | `gcloud compute instances describe yoyo-tier-b-1 --zone us-central1-a --project woodfine-node-gcp-free --format="value(status)"` |
| Check queue depth | `curl -s http://127.0.0.1:9080/readyz \| jq .queue_pending` |
| Watch drain live | `journalctl -u local-doorman -f \| grep -E "drain\|yoyo\|idle"` |
| Nightly start time | 02:00 UTC — GCP Instance Schedule fires automatically |
| Auto-stop | ~30 min after queue empties — idle monitor calls instances.stop |

---

## Cost model

| Component | Rate | Typical usage | Monthly estimate |
|---|---|---|---|
| `g2-standard-4` Spot + L4 | ~$0.84/hr | 2–4 hr/night × 30 nights | ~$50–$100 |
| Persistent SSD (100 GB) | ~$0.17/GB/mo | 100 GB always-on | ~$17 |
| Egress (workspace → Yo-Yo) | ~$0.01/GB | Minimal | < $5 |
| **Total** | | | **~$72–$122/mo** |

Spot preemption risk: L4 Spot preemption is rare in `us-west1` during off-peak
hours (02:00–06:00 UTC). If preempted, the nightly start reschedules the next day.
The brief queue is durable (file-backed FIFO) — no work is lost on preemption.
