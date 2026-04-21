# CLAUDE.md — slm-doorman

You are working on the protocol crate. Every external call to service-slm
flows through here. Correctness matters more than performance.

## What this crate owns

- The five-step doorman protocol.
- The sanitisation rules: what fields are stripped before data crosses the
  trust boundary.
- The rehydration logic: what fields are re-attached on return.
- Retry, timeout, and backoff policies.

## Invariants specific to this crate

1. **Sanitisation is the only gate** between PointSav-trusted data and
   external compute. A bug here is a data-leak bug. Write the property
   test first.
2. **Every doorman cycle writes a ledger entry.** Via `slm-ledger`, not
   directly. The ledger call is part of the cycle, not optional.
3. **No silent fallback from sanitisation failure.** If sanitisation
   cannot be proven correct for a payload, refuse it. A refusal is a
   `DoormanError::SanitisationFailed`, not a degraded response.
4. **Rehydration is the inverse of sanitisation.** If the two ever drift
   out of sync, we have violated the protocol. Property test this on
   every schema addition.

## Next work units

See `TASKS.md`. The first real task here is usually the
`SanitisationPolicy` type and a minimal pass-through implementation that
proves the round-trip.
