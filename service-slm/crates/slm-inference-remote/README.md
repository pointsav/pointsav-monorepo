# slm-inference-remote

The remote counterpart to `slm-inference-local`. Drives the GCP yo-yo
node: spin up via `slm-compute`, ship the sanitised request via HTTP,
await the response, record ledger entries at every phase.

This is the crate that makes "yo-yo" more than a metaphor.

## What lives here

- The `RemoteInferenceEngine` trait implementation.
- The HTTP client for the Cloud Run GPU node.
- Timeout, retry, and backoff policies coordinated with `slm-doorman`.
- Ledger event emission on every phase transition.
- Preemption handling (spot instances can be pulled at any time).

## What does not live here

- The provisioning itself — that's `slm-compute`.
- The sanitisation — that's `slm-doorman`.
- The KV cache — that's `slm-memory-kv`.
