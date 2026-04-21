# slm-compute

Ring 1 of the three-ring memory model: bootstrap state.

This crate drives the Cloud Run GPU node lifecycle. It pulls the pre-built
vLLM/mistral.rs container from Artifact Registry, mounts the model weights
from GCS via Cloud Storage FUSE, and starts the inference server with the
correct arguments for the current `moduleId`.

Specification: [YOYO-COMPUTE §2](../../specs/YOYO-COMPUTE.md).

## What lives here

- The Cloud Run driver (spin up, tear down, status).
- The warm-pool control (`min-instances=1` opt-in/opt-out).
- The container build metadata parser (`compute/manifest.yaml`).
- The weights registry reader (`compute/weights/registry.yaml`).
- Secret Manager integration for API keys (phase 2 key management).

## What does not live here

- The actual inference — that's `slm-inference-remote`.
- The KV cache — that's `slm-memory-kv` (Ring 2).
- The adapter registry — that's `slm-memory-adapters` (Ring 3b).
- The GCP credentials themselves — those live in Secret Manager;
  this crate only names references.
