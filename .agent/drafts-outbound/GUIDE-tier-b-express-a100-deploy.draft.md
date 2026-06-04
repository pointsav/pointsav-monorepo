---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
status: language-cleared
title: "Deploy the express GPU node (A100) on GCP for priority inference"
target_repo: woodfine-fleet-deployment
target_path: woodfine-fleet-deployment/cluster-intelligence/guide-tier-b-express-a100-deploy.md
target_filename: guide-tier-b-express-a100-deploy.md
bcsc_class: no-disclosure-implication
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — two-YoYo GCE plan, machine-type upgrade path"
research_inline: true
---

# Deploy the express GPU node (A100) on GCP for priority inference

The express GPU node (`yoyo-express`) handles time-sensitive inference work: urgent
organizational graph updates, interactive inference sessions, and batch backlog clearing.
It stops when idle (15 minutes default) and starts on demand in response to express-lane
requests. This guide covers provisioning the A100 VM and wiring it as the express tier.

## Pre-flight

```bash
# Confirm A100 quota in us-central1
gcloud compute regions describe us-central1 \
  --project=woodfine-node-gcp-free \
  --format="value(quotas[].limit)" | grep -i a100
# Need: "NVIDIA_A100_GPUS" >= 1

# Confirm model weights are in GCS (shared with batch node)
gsutil ls gs://woodfine-node-gcp-free-foundry-substrate/ollama-store/blobs/sha256-06c420f9
```

## Step 1 — Create the A100 VM

```bash
gcloud compute instances create yoyo-express \
  --project=woodfine-node-gcp-free \
  --zone=us-central1-a \
  --machine-type=a2-highgpu-1g \
  --boot-disk-size=50GB \
  --boot-disk-type=pd-balanced \
  --maintenance-policy=TERMINATE \
  --no-restart-on-failure \
  --scopes=cloud-platform \
  --image-family=debian-12 \
  --image-project=debian-cloud

# Assign static IP
gcloud compute addresses create yoyo-express-ip --region=us-central1
gcloud compute instances delete-access-config yoyo-express --zone=us-central1-a \
  --access-config-name="External NAT"
gcloud compute instances add-access-config yoyo-express --zone=us-central1-a \
  --address=$(gcloud compute addresses describe yoyo-express-ip --region=us-central1 --format="value(address)")
EXPRESS_IP=$(gcloud compute addresses describe yoyo-express-ip --region=us-central1 --format="value(address)")
```

## Step 2 — Install Ollama and load the model

Same procedure as the batch node (`guide-tier-b-batch-gcp-deploy.md` §2).
The A100 uses the same OLMo model from the same GCS bucket.

For the A100, increase concurrency (more VRAM available):
```bash
# In /etc/systemd/system/ollama.service on yoyo-express:
Environment="OLLAMA_NUM_PARALLEL=4"
```

## Step 3 — Set bearer token and open firewall

```bash
EXPRESS_BEARER=$(openssl rand -hex 32)
gcloud compute instances add-metadata yoyo-express --zone=us-central1-a \
  --metadata=slm-bearer-token=${EXPRESS_BEARER}

gcloud compute firewall-rules create yoyo-express-doorman \
  --project=woodfine-node-gcp-free \
  --allow=tcp:11434 \
  --source-ranges=<WORKSPACE_VM_EXTERNAL_IP>/32 \
  --target-tags=yoyo-express
gcloud compute instances add-tags yoyo-express --zone=us-central1-a --tags=yoyo-express
```

## Step 4 — Wire the Doorman

Add to `/etc/local-doorman/local-doorman.env` on the workspace VM:

```bash
SLM_YOYO_EXPRESS_ENDPOINT=http://${EXPRESS_IP}:11434
SLM_YOYO_EXPRESS_BEARER=${EXPRESS_BEARER}
SLM_YOYO_EXPRESS_MODEL=olmo3
SLM_YOYO_EXPRESS_HOURLY_USD=3.67
SLM_YOYO_EXPRESS_GCP_INSTANCE=yoyo-express
SLM_YOYO_EXPRESS_GCP_ZONE=us-central1-a
SLM_YOYO_EXPRESS_GCP_PROJECT=woodfine-node-gcp-free
SLM_YOYO_EXPRESS_CONCURRENCY=4
SLM_YOYO_EXPRESS_IDLE_SHUTDOWN_MIN=15
```

Restart the Doorman:
```bash
sudo systemctl restart local-doorman.service
```

## Step 5 — Test the express lane

With the VM running, test that express requests are served immediately:

```bash
# Start the VM first
gcloud compute instances start yoyo-express --zone=us-central1-a
# Wait ~2 min for boot + model load

# Express inference (immediate, no queue)
curl -s http://127.0.0.1:9080/v1/express/chat \
  -H "Content-Type: application/json" \
  -H "X-Foundry-YoYo-Label: express" \
  -d '{"messages":[{"role":"user","content":"ping"}],"max_tokens":5}'
# Expect: immediate response (not 202)

# With VM stopped, test 202 pattern
gcloud compute instances stop yoyo-express --zone=us-central1-a
curl -v http://127.0.0.1:9080/v1/express/chat \
  -H "Content-Type: application/json" \
  -H "X-Foundry-YoYo-Label: express" \
  -d '{"messages":[{"role":"user","content":"ping"}],"max_tokens":5}'
# Expect: HTTP 202 + Location header
# Poll Location until 200
```

## Machine-type upgrade/downgrade path

To switch the express node between A100 and L4 without reprovisioning:

```bash
# Downgrade to L4 (save cost when A100 not needed)
gcloud compute instances stop yoyo-express --zone=us-central1-a
gcloud compute instances set-machine-type yoyo-express \
  --zone=us-central1-a --machine-type=g2-standard-4
gcloud compute instances start yoyo-express --zone=us-central1-a
# Update SLM_YOYO_EXPRESS_HOURLY_USD=0.71 in doorman.env

# Upgrade back to A100
gcloud compute instances stop yoyo-express --zone=us-central1-a
gcloud compute instances set-machine-type yoyo-express \
  --zone=us-central1-a --machine-type=a2-highgpu-1g
gcloud compute instances start yoyo-express --zone=us-central1-a
# Update SLM_YOYO_EXPRESS_HOURLY_USD=3.67 in doorman.env
```

The boot disk, model weights, bearer token, and firewall rules are unchanged by
the machine-type swap.

## Cost

Stopped: ~$2/month (boot disk).
Running A100: $3.67/hr.
Running L4: $0.71/hr.

The express node is designed to run only when needed. At 15 min idle shutdown,
a 30-minute interactive session costs ~$1.84 on A100 or ~$0.36 on L4.
