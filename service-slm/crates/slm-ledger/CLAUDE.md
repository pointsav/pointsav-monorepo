# CLAUDE.md — slm-ledger

You are working on the SOC3 audit-ledger crate. Treat writes here with
the same care as writes to a financial journal.

## What this crate owns

- The `Event` struct and its 10 variants (per YOYO-COMPUTE §5).
- Append-only CSV writer with fsync semantics.
- SQLite mirror and its indices.
- Export format for external auditors.

## Invariants specific to this crate

1. **Append-only.** Never rewrite a row. Never delete. Corrections are
   new rows referencing the corrected event id.
2. **fsync on every commit.** A crash between write and sync must not
   lose a ledger row. Benchmark only after correctness is proven.
3. **Every row has a `moduleId`.** No exceptions. Validated at the type
   level — there should be no code path that can write a row without
   one.
4. **SQLite is a mirror, not the source of truth.** If the CSV and
   SQLite disagree, the CSV wins. Provide a reconcile command.
5. **Schema changes are ADRs.** Any addition or change to the `Event`
   schema requires an ADR and a migration plan.

## Next work units

See `TASKS.md`. First real task is the `Event` type and its 10 variants
with serde round-trip tests against the YOYO-COMPUTE §5 column spec.
