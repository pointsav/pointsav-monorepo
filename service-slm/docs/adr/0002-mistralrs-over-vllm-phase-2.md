# ADR-0002: mistral.rs replaces vLLM in Phase 2

- **Status:** accepted
- **Date:** 2026-04-20
- **Deciders:** Peter M. Woodfine, Jennifer Woodfine
- **Supersedes:** partial override of [`specs/STACK.md`](../../specs/) Phase 1 guidance on vLLM
- **See also:** [ADR-0001](./0001-rust-end-to-end.md)

## Context

Phase 1 uses vLLM (Apache-2.0, Python, PyTorch-backed). Phase 1 is correct
for its validation goal: the trial's objective is to prove the
architecture works, not to settle the inference runtime question.

Phase 2 is the Rust rewrite, and vLLM is wrong for that phase for three
reasons. First, it is Python-native: embedding Python into the Rust
binary is operationally painful and defeats the single-artefact shape
that [ADR-0001](./0001-rust-end-to-end.md) requires. Second, its runtime
footprint (12 GB container, GIL-capped workers) is incompatible with
low-RAM Totebox hosts. Third, while vLLM is permissively licensed, its
dependency graph is not entirely under our preferred MIT/Apache-2.0
discipline once Python's own long tail is included.

## Decision

Phase 2 replaces vLLM with [mistral.rs](https://github.com/EricLBuehler/mistral.rs)
(MIT) as the inference runtime on the yo-yo node, on Totebox hosts, and
anywhere else service-slm serves tokens. vLLM remains the Phase 1 trial
runtime; no migration of the Phase 1 environment is required.

`candle-vllm` (same author, also MIT) and `rvllm` (MIT, February 2026
release) are tracked as fallbacks. `candle-core` (Apache-2.0/MIT dual)
underlies mistral.rs; if mistral.rs ever stagnates we retain a migration
path through candle.

## Rationale

- **Binary shape.** mistral.rs ships as a statically-linked Rust binary
  plus CUDA kernels (~200 MB). No Python, no GIL, no venv. This
  matches [ADR-0001](./0001-rust-end-to-end.md)'s single-artefact goal.
- **Gemma 4 native.** mistral.rs supports Gemma 4 natively with
  FlashAttention V2/V3, PagedAttention, prefix caching, and hot LoRA
  swap per token. These are the primitives the three-ring memory model
  in [YOYO-COMPUTE §3–§4](../../specs/YOYO-COMPUTE.md) relies on.
- **Licence.** MIT, end-to-end on the runtime. Conforms to our
  dependency allow-list (see [deny.toml](../../deny.toml)).
- **Operational scale.** mistral.rs and its siblings are deployed in
  production at third-party sites; it is not an experiment.

### Alternatives considered

- **Keep vLLM in Phase 2.** Straightforward incremental path, but
  conflicts with the single-artefact goal and the Totebox-fit goal.
- **candle direct (no mistral.rs).** More low-level; we would re-do work
  mistral.rs has already done (paged attention, prefix caching, OpenAI
  wire compatibility). Reserved as fallback.
- **rvllm.** Promising (16-kernel-launch fused pipeline for Gemma 4's
  dual-attention architecture), but Feb 2026 is too new for a
  load-bearing dependency. Track; don't adopt yet.

## Consequences

- **Positive.** Single-binary shape preserved. Gemma 4 features mapped
  directly. CPU and quantised paths support Totebox. LoRA hot-swap
  unlocks [YOYO-COMPUTE §4 (3b)](../../specs/YOYO-COMPUTE.md) adapter
  routing.
- **Negative.** mistral.rs is maintained by a small team. Mitigation:
  candle underneath is by Hugging Face; we have a migration path.
- **Follow-up.**
  - Phase-1 GCP-NODE container rebuild swaps `vllm serve` for
    `mistralrs-server` at Phase 2 cutover.
  - Write a regression test suite that pins the answer quality of
    the Phase 1 vLLM path and verifies mistral.rs matches it within
    tolerance.

## References

- [`specs/SLM-STACK.md`](../../specs/SLM-STACK.md) §3.1 — the mistral.rs
  finding.
- [`specs/YOYO-COMPUTE.md`](../../specs/YOYO-COMPUTE.md) §8 (Headroom),
  the entries on CUDA checkpoint/restore and LoRA hot swap — both
  supported by mistral.rs.
