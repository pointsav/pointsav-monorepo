# CLAUDE.md — slm-inference-local

You are working on the local inference crate. The constraint is RAM,
not throughput.

## What this crate owns

- `LocalInferenceEngine` implementation over `mistralrs`.
- Quantisation selection (fp16, int8, int4, per target host).
- Local adapter activation.
- CPU-mode fallback.

## Invariants specific to this crate

1. **Memory footprint is configurable and bounded.** A misconfiguration
   must fail closed, not OOM the host. Check available RAM before load.
2. **Every inference call emits a ledger row.** Via `slm-ledger`, with
   the adapter versions that were active recorded on the row.
3. **`moduleId` selects the adapter stack.** No ambient state.
4. **CPU fallback is exercised in CI.** A test that asserts the crate
   works without CUDA is non-negotiable — Totebox hosts do not have
   GPUs.

## Next work units

See `TASKS.md`. First real task: probe available memory and select a
quantisation profile.
