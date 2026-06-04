---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
status: staged-pending-editorial
title: "Yo-Yo Tier B — Cloud Run Substrate"
target_path: media-knowledge-documentation/substrate/yoyo-cloud-run-substrate.md
paired_with: yoyo-cloud-run-substrate.es.md
bcsc_class: no-disclosure-implication
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-03 project-intelligence session — Cloud Run migration, 1-hour flow test, cost audit"
research_inline: true
---

# Yo-Yo Tier B — Cloud Run Substrate

The Yo-Yo substrate is the second inference tier in the three-tier Doorman routing
architecture. Tier A handles requests locally on the workspace node using a
compact model. Tier B routes to a higher-capability model on GPU compute when the
complexity of a request warrants it. Tier C routes to external commercial API
providers. This document covers the current Tier B implementation: a Google Cloud Run
service running the OLMo 3 32B Think model.

## Architecture

The Tier B service runs as a Cloud Run instance in the `europe-west4` region
(the Netherlands), hosted in the `woodfine-node-gcp-free` project.

| Component | Value |
|---|---|
| Service | `yoyo-tier-b` |
| Image | `docker.io/ollama/ollama:0.24.0` |
| GPU | NVIDIA L4 (1×) |
| CPU | 8 vCPU |
| Memory | 32 GiB |
| Scaling | 0–1 instances (scale to zero when idle) |
| Concurrency | 4 requests per instance |
| Region | europe-west4 |

The model — OLMo 3 32B Think, quantized to Q3 GGUF format at 15.6 GiB — is stored
in a Google Cloud Storage bucket (`woodfine-node-gcp-free-foundry-substrate`) under
the path `ollama-store/blobs/`. The Ollama process reads the model from this bucket
on cold start via the GCS FUSE filesystem driver.

A duplicate copy of the weights is also stored at `base-models/olmo-3-32b-think-q3.gguf`
in the same bucket. Both copies carry an identical SHA-256 digest (`06c420f9...`), providing
independent verification.

## Cold-start profile

Cold start is the period from when a request arrives at a scaled-to-zero instance
to when the first inference token is returned. The key constraint is that 15.6 GiB
of model weights must be loaded from network-attached storage into GPU memory before
inference can begin.

The naive approach — mounting the GCS bucket directly and allowing the Ollama process
to memory-map the GGUF file — produces a load time of approximately 30 minutes. This
occurs because memory-mapped reads over GCS FUSE issue one 4 KB HTTP GET per page
fault, achieving an effective throughput of approximately 18 MB/s against a 15.6 GiB file.

The current configuration eliminates this by using the GCS FUSE file-cache feature:

1. An in-memory volume (20 GiB tmpfs) is mounted as the file-cache target.
2. The GCS FUSE driver is configured to download the GGUF file into this volume using
   16 parallel workers with 200 MB chunk size before the Ollama process reads it.
3. The startup probe (TCP on port 8080) waits up to 900 seconds for the service to
   become ready, covering the download and model-load window.

The result is a cold-start time of approximately 5 minutes, dominated by the parallel
download of 15.6 GiB from GCS. Once the model is in the in-memory volume, Ollama reads
it at RAM speed, loading into GPU VRAM in seconds.

The `OLLAMA_KEEP_ALIVE=-1` environment variable instructs Ollama to retain the loaded
model in VRAM indefinitely, so subsequent requests on a warm instance respond without
any reload delay.

## Scale-to-zero economics

With `min-instances: 0`, the Cloud Run service scales to zero when there are no active
requests. A scaled-to-zero instance incurs no compute charges. The service scales up
automatically when a request arrives, cold-starting the instance.

At approximately 2 hours of active use per day, the monthly compute cost is approximately
$57 using the current 8 vCPU / 32 GiB / L4 configuration. The prior approach (a GCE
Spot VM reserved at all times in a single zone) cost approximately $165 per month and
was unavailable during zone stockout events.

For workloads requiring zero cold-start latency, the service can be configured with
`min-instances: 1`, which keeps one instance running at all times at a continuous cost
of approximately $0.67 per hour for the L4 GPU.

## Authentication

The Cloud Run service does not use static bearer tokens. Access is controlled by GCP
identity tokens issued by the Compute Engine metadata service. When the Doorman on
the workspace node sends a request to Tier B, it fetches a short-lived identity token
from the local metadata service and includes it in the `Authorization: Bearer` header.
Tokens expire after one hour; the Doorman fetches a fresh token on each request.

The `SLM_YOYO_GCP_AUTH=true` environment variable enables this behaviour in the
Doorman binary. When this flag is set, a `MetadataBearer` provider is used instead
of the static `StaticBearer`.

## Health probe

The Doorman maintains a background health probe for each configured Tier B node. The
probe polls the Ollama root path (`/`) every 30 seconds using the same identity token
mechanism as inference requests. A response of "Ollama is running" indicates the
instance is healthy.

Three consecutive probe failures mark the node as unavailable. The Doorman's circuit
breaker then routes all traffic to Tier A until the probe recovers. Because the
Cloud Run service is configured with `concurrency: 4`, health probes can reach the
instance during active inference requests without being queued or throttled.

The `SLM_YOYO_HEALTH_PATH=/` environment variable configures the probe path. The
default (`/health`) is correct for llama.cpp server but incorrect for Ollama, which
serves its health indicator at the root path.

## Doorman integration

The Doorman routes requests to Tier B when the `X-Foundry-Complexity: high` header
is present, or when the `SLM_TIER_A_FIRST` flag is disabled. The model name seen by
the Ollama endpoint is `olmo3`, configured via `SLM_YOYO_MODEL=olmo3`.

The service URL is:
```
https://yoyo-tier-b-369270631281.europe-west4.run.app
```

## Observed performance

A 1-hour flow test conducted on 2026-06-03 produced 15 consecutive successful
requests routed through the full Doorman → Cloud Run → OLMo 3 32B Think chain:

| Metric | Value |
|---|---|
| Requests | 15 |
| Success rate | 100% (15/15 HTTP 200) |
| `tier_used` | `yoyo` (all 15) |
| Fastest response | 26.5 s |
| Slowest response | 169.5 s |
| Average response | ~72 s |
| Total cost | ~$0.41 |

The variance in response time reflects the OLMo 3 Think model's reasoning behaviour:
it generates internal reasoning tokens before producing the final answer, and the
length of the reasoning chain varies by request.
