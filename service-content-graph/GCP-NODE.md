# GCP-NODE.md
# GCP Cloud Run Provisioning Outline
**Version:** 1 · April 19, 2026
**This is an outline only. The actual scripts will be written after the trial validates
the architecture. Do not start coding until P1 blocking items are resolved.**

---

## Node Specification

| Parameter | Value |
|---|---|
| Service | Google Cloud Run (GPU) |
| GPU — Trial | NVIDIA L4 (24 GB VRAM) — confirm availability in your GCP region |
| GPU — Full run | NVIDIA A100 80 GB — ~$200–300 for full 2.5 GB corpus |
| Model | Gemma 4 26B A4B (Hugging Face: `google/gemma-4-26B-A4B-it`) |
| Inference runtime | vLLM |
| Serving framework | FastAPI on port 8080 |
| Spot/Preemptible | Yes — SkyPilot 0.11+ manages recovery automatically |
| Min instances | 0 (scales to zero when idle) |
| Max instances | 1 for trial |
| Timeout | 3600s (1 hour per request) |
| Concurrency | 1 (one batch job at a time) |

---

## Pre-Requisites Checklist

All must be complete before running any provisioning steps.

**GCP:**
- [ ] GCP project ID confirmed
- [ ] Billing enabled and verified
- [ ] GPU quota requested and approved in chosen region (submit several days in advance)
- [ ] `gcloud` CLI installed and authenticated on MacPro
- [ ] Artifact Registry enabled in GCP project
- [ ] Cloud Storage bucket created: `dka-[project-id]-checkpoints`
- [ ] Budget alert set in GCP Billing: alert at $50, hard cap at $400 per batch job

**Service account:**
- [ ] Service account created with roles:
  - `roles/run.invoker`
  - `roles/storage.objectAdmin` (for checkpoint bucket)
  - `roles/secretmanager.secretAccessor`

**SSH:**
- [ ] SSH key pair generated: `ssh-keygen -t ed25519 -C "laptop-a-dka"`
- [ ] Public key registered in GCP project metadata
- [ ] Private key on Laptop-A at `~/.ssh/dka_gcp`
- [ ] Connection tested: `ssh dka-gcp "echo connected"`

**API keys (environment variables only — not hardcoded):**
- [ ] `GOOGLE_API_KEY` (text-embedding-005)
- [ ] `GCP_PROJECT_ID`
- [ ] `GCS_CHECKPOINT_BUCKET`

**Laptop-A and MacPro:**
- [ ] `python3 --version` on Laptop-A (P1 blocking item)
- [ ] `pip list` on MacPro venv (P1 blocking item)
- [ ] Top two levels of 2.5 GB file tree (P1 blocking item)
- [ ] SSH key pair confirmed (P1 blocking item)

---

## Provisioning Steps (Outline)

### Phase A: Container Build

1. Write `Dockerfile`:
   - Base: vLLM GPU image from `us-docker.pkg.dev/deeplearning-platform-release`
   - Install: vLLM, FastAPI, ladybugdb/duckdb, google-generativeai, pydantic, instructor, plus full stack from STACK.md
   - Copy: `pipeline/ingest.py`, `pipeline/derivative_engine.py`
   - Entrypoint: start vLLM server + FastAPI

2. Write `cloudbuild.yaml` for automated builds

3. Build and push container:
   ```bash
   gcloud builds submit --tag gcr.io/[PROJECT_ID]/dka-gemma4:latest
   docker push [REGION]-docker.pkg.dev/[PROJECT_ID]/dka/dka-gemma4:latest
   ```

### Phase B: Model Preparation

4. Accept Gemma 4 license on Hugging Face (requires HF account)

5. Download model weights:
   ```bash
   huggingface-cli download google/gemma-4-26B-A4B-it --local-dir ./models/gemma4
   ```

6. Upload weights to GCS (avoids re-downloading on every node boot):
   ```bash
   gsutil -m cp -r ./models/gemma4 gs://[CHECKPOINT_BUCKET]/models/
   ```

### Phase C: Cloud Run Service Configuration

7. Write `cloudrun.yaml`:
   - GPU: `nvidia-l4` (trial) or `nvidia-a100-80gb` (full run)
   - Memory: 48Gi · CPU: 8 · Timeout: 3600s · Concurrency: 1
   - Min instances: 0 · Max instances: 1
   - Environment variables from Secret Manager or direct

8. Deploy:
   ```bash
   gcloud run services replace cloudrun.yaml --region [REGION]
   ```

### Phase D: SkyPilot Configuration

9. Write `skypilot.yaml`:
   - Job idempotent by `(input_hash, job_version)` using GCS conditional writes
   - `mode: MOUNT_CACHED` for checkpoint bucket
   - Stable `$SKYPILOT_TASK_ID` across retries
   - `tenacity` exponential backoff, hard cap 5 retries

10. Test SkyPilot job submission with 1 dummy file before trial

### Phase E: Payload Transfer and Job Execution

11. Sanitise payload on Laptop-A (service-slm strips PII, coordinates, sensitive identifiers)

12. Transfer sanitised /ledger + /assets to GCP:
    ```bash
    rsync -avz --progress ./payload/ dka-gcp:/tmp/payload/
    ```

13. Trigger ingest job via HTTP POST to Cloud Run endpoint:
    ```bash
    curl -X POST https://[SERVICE_URL]/ingest \
      -H "Authorization: Bearer $(gcloud auth print-identity-token)" \
      -H "Content-Type: application/json" \
      -d '{"payload_path": "/tmp/payload", "job_id": "[JOB_ID]"}'
    ```

14. Monitor via Cloud Run logs: `gcloud run services logs tail dka-ingest`

### Phase F: Sync Results to Laptop-A

15. Pull graph delta from GCS:
    ```bash
    gsutil -m cp -r gs://[CHECKPOINT_BUCKET]/jobs/[JOB_ID]/delta/ ./delta/
    ```

16. Apply delta to local LadybugDB graph on Laptop-A:
    ```bash
    python3 pipeline/apply_delta.py --delta ./delta/ --db ./graph/knowledge.db
    ```

17. Verify graph integrity:
    ```bash
    python3 pipeline/verify_graph.py --db ./graph/knowledge.db
    ```

18. Record ingest completion in service-content/state/sync.db

19. Generate YAML snapshot:
    ```bash
    python3 pipeline/snapshot.py --db ./graph/knowledge.db --output ./state/snapshots/
    ```

### Phase G: Node Teardown

20. Cloud Run automatically scales to zero after request completes.
    Verify: `gcloud run services describe dka-ingest --region [REGION]`

---

## Cost Controls

- Set budget alert at $50, hard cap at $400 per batch job in GCP Billing
- Spot/Preemptible instances reduce GPU cost 60–70%
- SkyPilot handles spot instance recovery automatically
- Trial run on 3 files should cost < $5 — verify this before full corpus spend
- text-embedding-005: ~$3 total for full 2.5 GB corpus (one-time, 2.5 GB × ~$0.006/MTok)

---

## GPU Cost Comparison

| GPU | VRAM | Est. throughput | Est. time (2.5 GB corpus) | Approx. cost |
|---|---|---|---|---|
| NVIDIA L4 | 24 GB | ~80 tok/s | ~85 hrs | ~$340 |
| NVIDIA A100 40GB | 40 GB | ~200 tok/s | ~35 hrs | ~$280 |
| NVIDIA A100 80GB | 80 GB | ~350 tok/s | ~20 hrs | ~$200 |

**A100 80GB is the most cost-effective for the full run.** Trial run on L4 first to calibrate
actual tok/s — this is the single most important output of the trial.

---

## Recovery Procedure (If Node Fails Mid-Job)

1. Check last checkpoint: `gsutil ls gs://[CHECKPOINT_BUCKET]/jobs/[JOB_ID]/checkpoints/`
2. Resubmit with `--resume [CHECKPOINT_PATH]` flag
3. Job resumes from last checkpoint — no reprocessing of completed files
4. SkyPilot handles this automatically when using SkyPilot job submission

---

## Scripts to Write (After Trial Validation)

These will be written after the trial confirms the architecture works:

| Script | Purpose |
|---|---|
| `Dockerfile` | Container definition |
| `pipeline/ingest.py` | Main ingest script — reads /ledger + /assets, runs Gemma 4 |
| `pipeline/derivative_engine.py` | Computes derivative layers L1–L4 from extracted data |
| `pipeline/apply_delta.py` | Applies GCP graph delta to local LadybugDB |
| `pipeline/verify_graph.py` | Graph integrity checks (node counts, edge validity, vector index) |
| `pipeline/snapshot.py` | YAML snapshot generator for SOC3 audit trail |
| `cloudbuild.yaml` | GCP build configuration |
| `cloudrun.yaml` | Cloud Run service definition |
| `skypilot.yaml` | SkyPilot job configuration |
