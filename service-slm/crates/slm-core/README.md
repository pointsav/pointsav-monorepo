# slm-core

Shared types, errors, and the **moduleId** discipline threaded through every
call in the workspace.

Every other crate in the workspace depends on `slm-core`. Keep this crate
small and stable: changes here propagate everywhere.

## What lives here

- **ModuleId** — the newtype that tags every inference event with its
  project namespace. See [YOYO-COMPUTE §6](../../specs/YOYO-COMPUTE.md) for
  the discipline it enforces across all five layers (bootstrap, KV cache,
  graph, adapters, ledger).
- **Error kinds** — the `thiserror`-derived enums other crates re-export.
- **Envelope types** — the RF2 envelope shared with `service-content`.
- **Time, id, and hashing primitives** — `Timestamp`, `EventId`,
  `InputHash`, etc., so we pick one implementation once.

## What does not live here

- Any I/O. `slm-core` is pure.
- Any crate-specific business logic.
- Any `async` runtime or HTTP handler code.
