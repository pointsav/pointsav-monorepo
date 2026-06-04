---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
status: language-cleared
title: "Deploy the batch GPU node (L4) on GCP for daily inference work"
target_repo: woodfine-fleet-deployment
target_path: woodfine-fleet-deployment/cluster-intelligence/guide-tier-b-batch-gcp-deploy.md
target_filename: guide-tier-b-batch-gcp-deploy.md
bcsc_class: no-disclosure-implication
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — two-YoYo GCE plan, GCP REST API research"
research_inline: true
---

# Deploy the batch GPU node (L4) on GCP for daily inference work

The batch GPU node (`yoyo-batch`) handles daily background inference work: corpus
extraction for the organizational knowledge graph, apprenticeship brief processing,
and training corpus generation. It runs 1–4 hours per day, stopped when not needed.
This guide covers provisioning the VM, installing Ollama and the OLMo model, and
wiring it to the Doorman.

## Pre-flight

```bash
# Confirm GCP project and quota
gcloud compute regions describe us-central1 \
  --project=woodfine-node-gcp-free --format="value(quotas[].limit)"

# Confirm L4 quota: "NVIDIA_L4_GPUS" should be >= 1 in us-central1

# Confirm the model is in GCS
gsutil ls gs://woodfine-node-gcp-free-foundry-substrate/ollama-store/blobs/sha256-06c420f9
```

## Step 1 — Create the VM

```bash
gcloud compute instances create yoyo-batch \
  --project=woodfine-node-gcp-free \
  --zone=us-central1-a \
  --machine-type=g2-standard-4 \
  --boot-disk-size=50GB \
  --boot-disk-type=pd-balanced \
  --maintenance-policy=TERMINATE \
  --no-restart-on-failure \
  --scopes=cloud-platform \
  --image-family=debian-12 \
  --image-project=debian-cloud
```

Note the external IP assigned. Create a static IP if you want a stable endpoint:
```bash
gcloud compute addresses create yoyo-batch-ip --region=us-central1
gcloud compute instances delete-access-config yoyo-batch --zone=us-central1-a \
  --access-config-name="External NAT"
gcloud compute instances add-access-config yoyo-batch --zone=us-central1-a \
  --address=$(gcloud compute addresses describe yoyo-batch-ip --region=us-central1 --format="value(address)")
BATCH_IP=$(gcloud compute addresses describe yoyo-batch-ip --region=us-central1 --format="value(address)")
```

## Step 2 — Install Ollama and load the model

SSH into the VM:
```bash
gcloud compute ssh yoyo-batch --zone=us-central1-a
```

On the VM:
```bash
# Install Ollama (pin to 0.24.0 — newer versions have CUDA issues on this L4 driver)
curl -fsSL https://ollama.com/install.sh | OLLAMA_VERSION=0.24.0 sh

# Copy model weights from GCS (direct copy, not FUSE mount — much faster)
gsutil cp gs://woodfine-node-gcp-free-foundry-substrate/ollama-store/blobs/sha256-06c420f9 \
  /tmp/olmo-3-32b-think-q3.gguf

# Create Ollama model from GGUF
cat > /tmp/Modelfile << 'EOF'
FROM /tmp/olmo-3-32b-think-q3.gguf
PARAMETER num_ctx 4096
PARAMETER reasoning_format deepseek
PARAMETER reasoning_budget 1024
EOF
OLLAMA_MODELS=/srv/ollama ollama create olmo3 -f /tmp/Modelfile

# Start Ollama as a service
cat > /etc/systemd/system/ollama.service << 'EOF'
[Unit]
Description=Ollama inference server
After=network.target

[Service]
Environment="OLLAMA_MODELS=/srv/ollama"
Environment="OLLAMA_HOST=0.0.0.0:11434"
Environment="OLLAMA_KEEP_ALIVE=-1"
ExecStart=/usr/local/bin/ollama serve
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
EOF
systemctl daemon-reload && systemctl enable --now ollama.service

# Verify
curl -s http://localhost:11434/
# Expect: "Ollama is running"
```

## Step 3 — Set a bearer token

Set a random bearer token in instance metadata (the Doorman reads this for auth):
```bash
BEARER=$(openssl rand -hex 32)
gcloud compute instances add-metadata yoyo-batch --zone=us-central1-a \
  --metadata=slm-bearer-token=${BEARER}
echo "Bearer token: ${BEARER}"
# Save this — you will need it for the Doorman env file
```

Open port 11434 to the workspace VM:
```bash
gcloud compute firewall-rules create yoyo-batch-doorman \
  --project=woodfine-node-gcp-free \
  --allow=tcp:11434 \
  --source-ranges=<WORKSPACE_VM_EXTERNAL_IP>/32 \
  --target-tags=yoyo-batch
gcloud compute instances add-tags yoyo-batch --zone=us-central1-a --tags=yoyo-batch
```

## Step 4 — Wire the Doorman

On the workspace VM, add to `/etc/local-doorman/local-doorman.env`:

```bash
SLM_YOYO_BATCH_ENDPOINT=http://${BATCH_IP}:11434
SLM_YOYO_BATCH_BEARER=${BEARER}
SLM_YOYO_BATCH_MODEL=olmo3
SLM_YOYO_BATCH_HOURLY_USD=0.71
SLM_YOYO_BATCH_GCP_INSTANCE=yoyo-batch
SLM_YOYO_BATCH_GCP_ZONE=us-central1-a
SLM_YOYO_BATCH_GCP_PROJECT=woodfine-node-gcp-free
SLM_YOYO_BATCH_CONCURRENCY=2
SLM_YOYO_BATCH_IDLE_SHUTDOWN_MIN=30
```

Restart the Doorman:
```bash
sudo systemctl restart local-doorman.service
```

## Step 5 — Verify Tier B batch

```bash
# Check readyz shows batch node
curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool
# Expect: "batch" node in tier_b map, health_up: true

# Test inference through batch node
curl -s http://127.0.0.1:9080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "X-Foundry-Yoyo-Label: batch" \
  -d '{"model":"olmo3","messages":[{"role":"user","content":"ping"}],"max_tokens":5}'
```

## Daily operation

```bash
# Start batch node before daily drain
gcloud compute instances start yoyo-batch --zone=us-central1-a

# Monitor drain progress
journalctl -u local-doorman.service -f | grep -E "(batch|extract|circuit)"

# Stop after drain is complete (or it auto-stops after SLM_YOYO_BATCH_IDLE_SHUTDOWN_MIN)
gcloud compute instances stop yoyo-batch --zone=us-central1-a
```

Cost: VM stopped = ~$2/month (boot disk only). VM running = $0.71/hr.
At 2 hrs/day × 30 days = ~$43/month.
