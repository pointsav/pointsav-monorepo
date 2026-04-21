# slm-memory-adapters

Ring 3b of the three-ring memory model: the cross-project skill library.

LoRA adapters are small (~50 MB) frozen-weight modules that sit on top of
the base Gemma 4 weights and encode task-specific behaviour — CoA
classification, archetype detection, per-client entity resolution. Each
adapter is trained once, versioned, stored as an OCI Artifact,
Sigstore-signed, SLSA-attested, and loaded at inference start.

**The adapter library is the DKA moat.** Every project leaves behind an
adapter; the base model is commodity. This crate is therefore the
commercial differentiator wrapped in Rust types.

Specification: [YOYO-COMPUTE §4 (3b)](../../specs/YOYO-COMPUTE.md).

## What lives here

- The adapter `Registry` (parsed from `memory/adapters/registry.yaml`).
- The OCI Artifact downloader and verifier.
- The Sigstore signature check.
- The dual-adapter routing logic (CL-LoRA pattern per YOYO-COMPUTE §4).
- The training-ledger integration.

## What does not live here

- The training runs themselves — those are Python jobs in
  `memory/adapters/train/` that produce artefacts this crate consumes.
- The inference activation — that's `slm-inference-{local,remote}`,
  using the adapter handles this crate provides.
