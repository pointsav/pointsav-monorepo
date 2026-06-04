---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
status: staged-pending-editorial
title: "Cloud Run Tier B — Deploy and Operate"
target_path: woodfine-fleet-deployment/cluster-totebox-intelligence/guide-yoyo-cloud-run-deploy.md
bcsc_class: no-disclosure-implication
supersedes: service-slm/docs/deploy/deploy-yoyo-tier-b.md
---

# Cloud Run Tier B — Deploy and Operate

Operational guide for the Yo-Yo Tier B inference service on Google Cloud Run.
Covers initial deployment, Doorman wiring, cost management, and rollback.

**Service:** `yoyo-tier-b` — `europe-west4`, project `woodfine-node-gcp-free`  
**Model:** OLMo 3 32B Think Q3 GGUF (15.6 GiB)  
**Endpoint:** `https://yoyo-tier-b-369270631281.europe-west4.run.app`

---

## §1 Pre-flight

Before deploying or redeploying, verify:

```bash
# Model blob present in GCS (must be 15.6 GiB)
gcloud storage ls -l \
  gs://woodfine-node-gcp-free-foundry-substrate/ollama-store/blobs/sha256-06c420f9*

# Ollama manifest present
gcloud storage ls \
  gs://woodfine-node-gcp-free-foundry-substrate/ollama-store/manifests/registry.ollama.ai/library/olmo3/

# Cloud Run API enabled and SA has storage access
gcloud run regions describe europe-west4 --project woodfine-node-gcp-free
```

The Compute Engine SA (`369270631281-compute@developer.gserviceaccount.com`) must have
`roles/storage.objectViewer` on the bucket.

---

## §2 Deploy / redeploy

Use `gcloud run services replace` with a YAML spec. The YAML below is the authoritative
configuration — copy it to `/tmp/yoyo-tier-b.yaml` and apply.

**Critical constraints:**
- Image must be `docker.io/ollama/ollama:0.24.0` — NOT `latest`. The 0.30.x series
  (current `latest`) has a CUDA kernel mismatch on Cloud Run's L4 GPU driver.
- `ghcr.io` is not on Cloud Run's allowed registry list — use `docker.io` only.
- `containerConcurrency` must be 4 or higher. A value of 1 causes the Doorman's health
  probes to fail during active inference, which opens the circuit breaker.
- `autoscaling.knative.dev/minScale` should be `'0'` for normal operation
  (scale-to-zero, $0 when idle). Use `'1'` only for sustained testing sessions.

**YAML spec** (`/tmp/yoyo-tier-b.yaml`):

```yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  annotations:
    run.googleapis.com/ingress: all
  labels:
    cloud.googleapis.com/location: europe-west4
  name: yoyo-tier-b
  namespace: '369270631281'
spec:
  template:
    metadata:
      annotations:
        autoscaling.knative.dev/maxScale: '1'
        autoscaling.knative.dev/minScale: '0'
        run.googleapis.com/client-name: gcloud
        run.googleapis.com/cpu-throttling: 'false'
        run.googleapis.com/gpu-zonal-redundancy-disabled: 'true'
        run.googleapis.com/startup-cpu-boost: 'true'
      labels:
        run.googleapis.com/startupProbeType: Custom
    spec:
      containerConcurrency: 4
      containers:
      - env:
        - name: OLLAMA_HOST
          value: 0.0.0.0:8080
        - name: OLLAMA_NUM_PARALLEL
          value: '1'
        - name: OLLAMA_MODELS
          value: /models/ollama-store
        - name: OLLAMA_KEEP_ALIVE
          value: '-1'
        - name: OLLAMA_LOAD_TIMEOUT
          value: 30m
        image: docker.io/ollama/ollama:0.24.0
        ports:
        - containerPort: 8080
          name: http1
        resources:
          limits:
            cpu: '8'
            memory: 32Gi
            nvidia.com/gpu: '1'
        startupProbe:
          failureThreshold: 90
          periodSeconds: 10
          tcpSocket:
            port: 8080
          timeoutSeconds: 5
        volumeMounts:
        - mountPath: /models
          name: gcs-models
        - mountPath: /model-cache
          name: model-cache
      nodeSelector:
        run.googleapis.com/accelerator: nvidia-l4
      serviceAccountName: 369270631281-compute@developer.gserviceaccount.com
      timeoutSeconds: 1800
      volumes:
      - emptyDir:
          medium: Memory
          sizeLimit: 20Gi
        name: model-cache
      - csi:
          driver: gcsfuse.run.googleapis.com
          readOnly: true
          volumeAttributes:
            bucketName: woodfine-node-gcp-free-foundry-substrate
            mountOptions: >-
              cache-dir=cr-volume:model-cache,
              file-cache-max-size-mb=-1,
              file-cache-enable-parallel-downloads=true,
              file-cache-max-parallel-downloads=16,
              file-cache-download-chunk-size-mb=200,
              metadata-cache-ttl-secs=-1
        name: gcs-models
  traffic:
  - latestRevision: true
    percent: 100
```

**Apply:**
```bash
gcloud run services replace /tmp/yoyo-tier-b.yaml \
  --region europe-west4 \
  --project woodfine-node-gcp-free
```

Wait for `Done.` — the startup probe must pass (up to 15 min on cold revision).

---

## §3 Doorman wiring

The Doorman binary must be rebuilt and deployed after any change to Doorman source
code. The current binary includes GCP identity token auth and the Ollama health path.

### Build

```bash
cd /srv/foundry/clones/project-intelligence/service-slm
cargo build --release -p slm-doorman-server
# Binary lands at $CARGO_TARGET_DIR/release/slm-doorman-server
# On foundry-workspace: /srv/foundry/cargo-target/mathew/release/slm-doorman-server
```

### Deploy

```bash
sudo systemctl stop local-doorman
sudo cp /srv/foundry/cargo-target/mathew/release/slm-doorman-server \
  /usr/local/bin/slm-doorman-server
```

### Apply env

The env file at `/etc/local-doorman/local-doorman.env` must contain:

```bash
SLM_YOYO_ENDPOINT=https://yoyo-tier-b-369270631281.europe-west4.run.app
SLM_YOYO_TRAINER_ENDPOINT=https://yoyo-tier-b-369270631281.europe-west4.run.app
SLM_YOYO_GRAPH_ENDPOINT=https://yoyo-tier-b-369270631281.europe-west4.run.app
SLM_YOYO_GCP_AUTH=true          # use GCP identity tokens (not static bearer)
SLM_YOYO_HEALTH_PATH=/          # Ollama health is at /, not /health
SLM_YOYO_MODEL=olmo3            # Ollama model name (not Olmo-3-1125-32B-Think)
SLM_YOYO_TRAINER_MODEL=olmo3
SLM_YOYO_GRAPH_MODEL=olmo3
SLM_YOYO_HOURLY_USD=1.40        # L4 billed rate for cost tracking
```

A prepared env file is at `/tmp/local-doorman-new.env` on the workspace VM.

```bash
sudo cp /tmp/local-doorman-new.env /etc/local-doorman/local-doorman.env
sudo systemctl start local-doorman
```

---

## §4 Verification

### Step 1 — Doorman health check

```bash
curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool
```

Expected: all three `tier_b` nodes show `"health_up": true, "circuit": "closed"`.

If `health_up` is `false`, check that Cloud Run has at least one warm instance
(the service may be scaled to zero — send a direct request to wake it).

### Step 2 — Warm the model (cold start)

On a newly deployed or scaled-to-zero instance, send a direct request to Cloud Run
to trigger model loading (~5 minutes):

```bash
TOKEN=$(curl -s -H "Metadata-Flavor: Google" \
  "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity?audience=https://yoyo-tier-b-369270631281.europe-west4.run.app")

curl -s --max-time 600 \
  -H "Authorization: Bearer $TOKEN" \
  https://yoyo-tier-b-369270631281.europe-west4.run.app/
# Expect: "Ollama is running"
```

### Step 3 — Flow test through Doorman

```bash
curl -s --max-time 120 http://127.0.0.1:9080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "X-Foundry-Complexity: high" \
  -d '{"model":"olmo3","messages":[{"role":"user","content":"ping"}],"max_tokens":5}' \
  | python3 -c "import json,sys; d=json.load(sys.stdin); print(d['tier_used'], d['content'])"
```

Expected output: `yoyo <response>` — confirms the full Doorman → Cloud Run chain.

---

## §5 Cost management

| Setting | Effect | Use case |
|---|---|---|
| `min-instances: 0` | Scale to zero — $0 when idle | Normal operation |
| `min-instances: 1` | Always-warm — ~$0.67/hr continuous | Sustained testing |

To change min-instances, update the YAML `autoscaling.knative.dev/minScale` annotation
and run `gcloud run services replace` again.

To run a timed warm-instance test:
1. Set `min-instances: 1`, deploy
2. Run test
3. Set `min-instances: 0`, deploy — billing stops within ~1 minute

---

## §6 Rollback

If the Cloud Run service has issues, the Doorman falls back to Tier A automatically
when the circuit breaker opens (after 3 health probe failures). Tier A continues
serving throughout and is unaffected by any Tier B change.

To explicitly revert Doorman to Tier A only:
```bash
sudo sed -i 's|SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=|' \
  /etc/local-doorman/local-doorman.env
sudo systemctl restart local-doorman
```

To revert to a previous Cloud Run revision:
```bash
gcloud run revisions list --service yoyo-tier-b \
  --region europe-west4 --project woodfine-node-gcp-free
# Then:
gcloud run services update-traffic yoyo-tier-b \
  --region europe-west4 --project woodfine-node-gcp-free \
  --to-revisions=<revision>=100
```

---

## §7 Known constraints

| Constraint | Detail |
|---|---|
| Ollama version | Use `0.24.0` only — `0.30.x` (current `latest`) has CUDA kernel mismatch on Cloud Run L4 |
| Container concurrency | Must be ≥ 4 — value of 1 blocks health probes during inference, opening the circuit breaker |
| Registry | `ghcr.io` is not allowed — use `docker.io` or Artifact Registry |
| Identity tokens | Expire after 1 hour; `MetadataBearer` fetches fresh tokens per request — no rotation needed |
| GCP zone | Cloud Run manages the GPU pool; no zone configuration required (unlike GCE Spot VMs) |
| Model name | Ollama uses `olmo3`, not `Olmo-3-1125-32B-Think` — `SLM_YOYO_MODEL` must match |
| `SLM_TIER_A_FIRST` | When `true`, Tier B only activates on explicit `X-Foundry-Complexity: high` hint |
