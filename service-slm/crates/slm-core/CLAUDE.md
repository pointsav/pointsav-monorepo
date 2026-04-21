# CLAUDE.md — slm-core

You are working in the foundational crate. Changes here propagate to every
other crate in the workspace, so diffs should be small, well-tested, and
almost always accompanied by doc updates.

## What this crate owns

- `ModuleId` newtype and its validation rules.
- Shared `Error` and `Result` types (re-exported by other crates).
- The RF2 envelope struct shared across the workspace.
- Primitive wrappers (`Timestamp`, `EventId`, `InputHash`) that pin our
  implementation choice.

## Invariants specific to this crate

1. **No I/O.** This crate is pure. If you find yourself reaching for
   `tokio`, `sqlx`, `reqwest`, or `std::fs`, the code you are writing
   belongs in another crate.
2. **No generic-over-runtime abstractions.** Keep types concrete. Other
   crates build the abstractions on top.
3. **Every public type derives at least `Debug`, `Clone`, `Eq`, and
   `Hash` where it makes sense.** This is a foundational convenience for
   downstream crates.
4. **Every public type has `serde::{Serialize, Deserialize}`** unless
   there is a specific reason not to (in which case document it).

## Next work units

See `TASKS.md` at the workspace root. The first real implementation here
will be the `ModuleId` newtype with validation and serde round-trip tests.
