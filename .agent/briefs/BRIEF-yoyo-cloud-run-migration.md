---
artifact: brief
status: archived
archived_date: 2026-06-02
superseded_by: BRIEF-project-intelligence-master.md §9
title: Yo-Yo Tier B — Cloud Run Migration (No Docker)
created: 2026-06-02
updated: 2026-06-02
author: totebox@project-intelligence (claude-sonnet-4-6)
grounds_in:
  - service-slm/docs/deploy/deploy-yoyo-tier-b.md
  - service-slm/compute/packer/scripts/llama-server.service
  - /etc/local-doorman/local-doorman.env
  - archive/BRIEF-slm-substrate-master.md (§1 live state)
---

# BRIEF — Yo-Yo Tier B Cloud Run Migration

> **Why this exists:** `yoyo-tier-b-1` is TERMINATED due to L4 GPU stockout in
> europe-west4-a. Restarting the GCE Spot VM approach is not viable — zone exhaustion
> means restarts fail unpredictably. Cloud Run eliminates the zone lottery: Google manages
> the GPU pool, no quota request needed.

---

## §1 — Current state (as of 2026-06-02)

| Component | Status | Notes |
|---|---|---|
| `yoyo-tier-b-1` (GCE g2-standard-4) | **TERMINATED** | europe-west4-a L4 stockout; static IP 34.6.204.25 retained |
| Tier A (`local-slm.service`) | **active** | Primary tier; OLMo 2 7B Q4_K_M on CPU; carrying all traffic |
| Doorman Yo-Yo endpoint | **stale** | Points to `https://34.6.204.25:9443`; circuit open; all traffic to Tier A |

---

## §2 — Migration approach

**No Docker.** Use the pre-built `ghcr.io/ggerganov/llama.cpp:server-cuda` image from
the official llama.cpp project. No Dockerfile to write or maintain.

**Model storage.** The GGUF (`olmo-3-32b-think-q3.gguf`, ~20 GB) is loaded from a GCS
bucket via Cloud Run's native Cloud Storage volume mount (FUSE). Model is read at cold
start; not baked into the image.

**Auth.** Deploy `--allow-unauthenticated`. The Cloud Run URL is a random UUID subdomain
— not guessable. The old Nginx bearer token validation is dropped (Nginx is gone).
Doorman still sends `SLM_YOYO_BEARER` in the Authorization header; Cloud Run and
llama-server both ignore it. Follow-up: add Cloud Run IAM auth in a future session.

**Cost.** ~$70/month at 2 hrs/day usage (4 vCPU + 16 GiB + L4, billed per second,
$0 when idle). No billing for cold start except ~$0.01 for the 15–30s startup time.

---

## §3 — Critical llama-server flags (from old systemd unit)

These are verified correct per `BRIEF-slm-substrate-master.md §2.8` (2026-06-01):

| Flag | Value | Note |
|---|---|---|
| `-ngl` | `99` | All layers on GPU — required |
| `-np` | `1` | NOT 4; `-np 4` with `-c 4096` truncates to 1024 tokens/slot |
| `-fa` | `on` | NOT bare `-fa`; bare flag consumes next arg as value → crash |
| `--host` | `0.0.0.0` | Changed from `127.0.0.1`; Cloud Run routes externally |
| `--port` | `8080` | Cloud Run default port |
| `-c` | `4096` | Context size |
| `-a` | `Olmo-3-1125-32B-Think` | Model alias (three aliases set) |
| `--reasoning-format` | `deepseek` | Required for OLMo-3 Think |
| `--reasoning-budget` | `1024` | |

---

## §4 — Migration checklist

### Pre-flight
- [ ] Confirm `woodfine-node-gcp-free` project has Cloud Run GPU quota in europe-west4:
  ```bash
  gcloud run regions describe europe-west4 --project woodfine-node-gcp-free
  ```
- [ ] Confirm Compute Engine SA has `roles/storage.objectViewer` on the weights bucket

### Step 1 — Get model into GCS as an object
```bash
# Check if already there:
gsutil ls gs://woodfine-node-gcp-free-foundry-substrate/weights/

# If not: restore disk snapshot to a temp disk, mount, copy, delete disk
gcloud compute disks create yoyo-weights-restore \
  --source-snapshot=yoyo-tier-b-1-weights-20260513-1923 \
  --zone=europe-west4-a --project=woodfine-node-gcp-free
# Attach to a small temp VM → mount → gsutil cp /data/weights/olmo-3-32b-think-q3.gguf gs://...
# Then: gcloud compute disks delete yoyo-weights-restore --zone=europe-west4-a
```

### Step 2 — Deploy Cloud Run service
```bash
gcloud run deploy yoyo-tier-b \
  --image ghcr.io/ggerganov/llama.cpp:server-cuda \
  --region europe-west4 \
  --project woodfine-node-gcp-free \
  --gpu 1 --gpu-type nvidia-l4 \
  --cpu 4 --memory 16Gi \
  --min-instances 0 --max-instances 1 \
  --concurrency 1 \
  --timeout 1800 \
  --allow-unauthenticated \
  --add-volume name=weights,type=cloud-storage,bucket=woodfine-node-gcp-free-foundry-substrate \
  --add-volume-mount volume=weights,mount-path=/models \
  --args="--model,/models/weights/olmo-3-32b-think-q3.gguf,-ngl,99,--host,0.0.0.0,--port,8080,-c,4096,-a,Olmo-3-1125-32B-Think,-a,Olmo-3-32B-Think,-a,olmo-3-32b,-np,1,-fa,on,--reasoning-format,deepseek,--reasoning-budget,1024,--metrics" \
  --port 8080
```

### Step 3 — Test the endpoint directly (before touching Doorman)
```bash
CLOUD_RUN_URL=https://yoyo-tier-b-<hash>-ew.a.run.app

curl $CLOUD_RUN_URL/health
# Expected: {"status":"ok"}

curl -s $CLOUD_RUN_URL/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"Olmo-3-1125-32B-Think","messages":[{"role":"user","content":"ping"}],"max_tokens":10}'
```
**Do not proceed until health + inference both pass.**

### Step 4 — Update Doorman config
Edit `/etc/local-doorman/local-doorman.env` — three vars:
```
SLM_YOYO_ENDPOINT=https://yoyo-tier-b-<hash>-ew.a.run.app
SLM_YOYO_TRAINER_ENDPOINT=https://yoyo-tier-b-<hash>-ew.a.run.app
SLM_YOYO_GRAPH_ENDPOINT=https://yoyo-tier-b-<hash>-ew.a.run.app
```
Then: `sudo systemctl restart local-doorman`

### Step 5 — Verify end-to-end through Doorman
```bash
curl -s http://127.0.0.1:9080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"Olmo-3-1125-32B-Think","messages":[{"role":"user","content":"test"}],"max_tokens":20}'

journalctl -u local-doorman -f   # confirm Tier B hit in logs
```

---

## §5 — Rollback

Revert the three `SLM_YOYO_*` vars to `https://34.6.204.25:9443` and restart Doorman.
Tier A continues serving throughout — unaffected by any step above.

---

## §6 — Follow-up items (post-migration)

- [ ] Add Cloud Run IAM auth (replace `--allow-unauthenticated`; update Doorman to fetch
      GCP identity token from metadata service instead of static bearer token)
- [ ] Set `--min-instances 1` if cold-start latency (~60–90s for 20 GB model load) is
      unacceptable in practice; adds ~$0.67/hr continuous GPU cost
- [ ] Update `service-slm/docs/deploy/deploy-yoyo-tier-b.md` with Cloud Run procedure
- [ ] Release the static IP `34.6.204.25` if the old VM is permanently retired
