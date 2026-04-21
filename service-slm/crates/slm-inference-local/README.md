# slm-inference-local

Local inference using [`mistral.rs`](https://github.com/EricLBuehler/mistral.rs).
For Totebox hosts with enough compute to serve the model directly.

Per SLM-STACK §3.1, `mistral.rs` replaces vLLM as the Phase-2 inference
runtime. It ships as a ~200 MB statically-linked Rust binary plus CUDA
kernels; no Python, no GIL, no 12 GB container. On a low-RAM host
(Totebox Laptop-A) it runs in quantised CPU mode within the 550 MB
headroom.

## What lives here

- The `LocalInferenceEngine` trait implementation backed by `mistralrs`.
- Quantisation and model-variant selection logic.
- LoRA adapter activation via the `slm-memory-adapters` handles.
- CPU-mode fallback for memory-constrained hosts.

## What does not live here

- The remote (GCP yo-yo) driver — that's `slm-inference-remote`.
- The adapter registry — that's `slm-memory-adapters`.
- The HTTP API — that's `slm-api`.
