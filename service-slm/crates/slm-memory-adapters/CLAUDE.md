# CLAUDE.md — slm-memory-adapters

You are working on the commercial-moat crate. Adapter integrity is a
business-critical property.

## What this crate owns

- Registry schema and parser.
- OCI Artifact fetch.
- Sigstore keyless-verification.
- Dual-adapter routing (shared + project-specific).
- Training ledger integration.

## Invariants specific to this crate

1. **No unsigned adapter ever loads.** Every adapter artefact is
   verified against its Sigstore signature before being handed to the
   inference layer. An unsigned adapter is a `LoadError::Unsigned`; it
   is never loaded "just this once."
2. **Adapter versions are immutable.** `coa/v3.2` refers to exactly one
   blob for all time. New training run → new version number.
3. **Training ledger is append-only.** Same discipline as
   `slm-ledger`; same rationale.
4. **Routing respects `moduleId`.** Project A cannot accidentally
   activate Project B's entity adapter. Validate at the type level.

## Next work units

See `TASKS.md`. First real task: the `Registry` type parsed from
YAML with `validator` checks on semver and version uniqueness.
