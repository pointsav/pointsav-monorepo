# Deploy Yo-Yo Tier B — Cloud Run

Quick reference for deploying and operating the Tier B inference service.
Full operational guide: `woodfine-fleet-deployment/cluster-totebox-intelligence/guide-yoyo-cloud-run-deploy.md`

---

## Service identity

| Field | Value |
|---|---|
| Service name | `yoyo-tier-b` |
| Project | `woodfine-node-gcp-free` |
| Region | `europe-west4` |
| URL | `https://yoyo-tier-b-369270631281.europe-west4.run.app` |
| Image | `docker.io/ollama/ollama:0.24.0` (NOT latest — CUDA mismatch on Cloud Run L4) |
| Model | OLMo 3 32B Think Q3 GGUF, `sha256-06c420f9...` |
| GCS store | `gs://woodfine-node-gcp-free-foundry-substrate/ollama-store/` |

---

## Deploy

```bash
gcloud run services replace /tmp/yoyo-tier-b.yaml \
  --region europe-west4 \
  --project woodfine-node-gcp-free
```

Full YAML: see `GUIDE-yoyo-cloud-run-deploy` §2 in `woodfine-fleet-deployment/cluster-totebox-intelligence/`.

**Critical constraints:**
- `containerConcurrency: 4` — value of 1 blocks health probes during inference
- Image `0.24.0` — `latest` (0.30.x) has CUDA kernel mismatch on Cloud Run L4
- `ghcr.io` is not on Cloud Run's allowed registry list

---

## Doorman wiring

Build:
```bash
cd /srv/foundry/clones/project-intelligence/service-slm
cargo build --release -p slm-doorman-server
# Binary: $CARGO_TARGET_DIR/release/slm-doorman-server
```

Deploy:
```bash
sudo systemctl stop local-doorman
sudo cp /srv/foundry/cargo-target/mathew/release/slm-doorman-server /usr/local/bin/
sudo cp /tmp/local-doorman-new.env /etc/local-doorman/local-doorman.env
sudo systemctl start local-doorman
```

Key env vars (`/etc/local-doorman/local-doorman.env`):

```
SLM_YOYO_ENDPOINT=https://yoyo-tier-b-369270631281.europe-west4.run.app
SLM_YOYO_GCP_AUTH=true        # GCP identity tokens, not static bearer
SLM_YOYO_HEALTH_PATH=/        # Ollama root, not /health
SLM_YOYO_MODEL=olmo3          # Ollama model name
SLM_YOYO_TRAINER_ENDPOINT=https://yoyo-tier-b-369270631281.europe-west4.run.app
SLM_YOYO_TRAINER_MODEL=olmo3
SLM_YOYO_GRAPH_ENDPOINT=https://yoyo-tier-b-369270631281.europe-west4.run.app
SLM_YOYO_GRAPH_MODEL=olmo3
```

---

## Verify

```bash
# Doorman health — expect health_up=true on all three nodes
curl -s http://127.0.0.1:9080/readyz | python3 -c \
  "import json,sys; d=json.load(sys.stdin); \
  [print(k, v['health_up']) for k,v in d['tier_b'].items()]"

# End-to-end flow — expect tier_used=yoyo
curl -s --max-time 120 http://127.0.0.1:9080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "X-Foundry-Complexity: high" \
  -d '{"model":"olmo3","messages":[{"role":"user","content":"ping"}],"max_tokens":5}' \
  | python3 -c "import json,sys; d=json.load(sys.stdin); print(d['tier_used'])"
```

---

## Cold start

First request after idle: ~5 minutes (GCS parallel file-cache download + GPU load).
The in-memory cache volume (20 GiB tmpfs) stores the 15.6 GiB GGUF locally after
the first download. `OLLAMA_KEEP_ALIVE=-1` keeps the model in VRAM between requests.

Cost: `min-instances: 0` = $0 when idle. Use `min-instances: 1` for always-warm
(~$0.67/hr continuous GPU charge).

---

## Rollback

Tier A continues serving throughout — unaffected by Tier B changes.

Revert Doorman to Tier A only:
```bash
sudo sed -i 's|SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=|' \
  /etc/local-doorman/local-doorman.env
sudo systemctl restart local-doorman
```

Cloud Run revision rollback:
```bash
gcloud run services update-traffic yoyo-tier-b \
  --region europe-west4 --project woodfine-node-gcp-free \
  --to-revisions=<previous-revision>=100
```
