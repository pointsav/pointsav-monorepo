---
description: Create a new workspace crate with the house scaffold. Usage: /scaffold-crate <crate-name>
argument-hint: <crate-name>
---

Create a new crate named `$ARGUMENTS` under `crates/$ARGUMENTS/` with the
workspace's standard scaffold. This should only be used after an ADR
has approved the new crate — one-binary discipline means new crates
need architectural justification.

Steps:

1. Confirm that an ADR authorising this crate exists. If not, **stop**
   and ask the human to write one first.
2. Create the directory structure: `src/`, `tests/`, `examples/`.
3. Write `Cargo.toml` inheriting from `[workspace.package]` with:
   - `#![forbid(unsafe_code)]` implied via `[lints.rust]`.
   - `missing_docs = "warn"`.
   - Pedantic clippy on.
4. Write `src/lib.rs` with the SPDX header, the AGPL-3.0-only copyright
   line, a `//!` crate-level doc comment, and the
   `__scaffold_placeholder()` stub.
5. Write `tests/smoke.rs` with the standard compile-check.
6. Write `README.md` explaining what the crate owns and what it does
   not own.
7. Write `CLAUDE.md` with crate-specific invariants and a pointer to
   the next-work-unit in `TASKS.md`.
8. Add the crate to `[workspace].members` in the root `Cargo.toml`.
9. Add a row to the crate status matrix in `STATUS.md`.

Stop and present the diff for review before committing.
