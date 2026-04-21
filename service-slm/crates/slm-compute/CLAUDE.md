# CLAUDE.md — slm-compute

You are working on the Ring 1 bootstrap crate. It drives provisioning,
not inference.

## What this crate owns

- Cloud Run spin-up and tear-down.
- Container manifest parsing (`service-slm/compute/manifest.yaml`).
- Weights registry (`service-slm/compute/weights/registry.yaml`).
- Warm-pool toggle for sustained-load windows.
- Secret Manager reference resolution.

## Invariants specific to this crate

1. **Every bootstrap operation emits at least two ledger rows:**
   `BOOT_REQUEST` and `BOOT_COMPLETE` (or `BOOT_REQUEST` and an error
   row). Via `slm-ledger`.
2. **Secret values never appear in logs.** The type system must prevent
   a `SecretRef` from being `Display`ed. Wrap in a newtype; implement
   `Debug` to print `<redacted>`.
3. **Bootstrap is idempotent.** A second spin-up while one is in
   progress returns the existing handle, not a new node.
4. **Teardown writes the final cost.** `TEARDOWN_COMPLETE` row includes
   GPU seconds and USD cost pulled from the Cloud Run billing API.

## Next work units

See `TASKS.md`. First real task: parse `manifest.yaml` into a typed
`ComputeManifest` struct with `validator` rules.
