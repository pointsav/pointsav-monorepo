# CLAUDE.md — slm-memory-kv

You are working on the Ring 2 working-memory client. This is the crate
that makes the yo-yo fast on the second invocation.

## What this crate owns

- Mooncake Store metadata client (HTTP).
- LMCache config generation for mistral.rs.
- Deterministic KV block hashing.
- `moduleId` namespacing.
- Stats collection (`stats.csv`).

## Invariants specific to this crate

1. **We talk to Mooncake over its published wire protocol, never as a
   library.** No FFI, no linked C++. If the temptation arises, check
   SLM-STACK §4.1.
2. **Block hashes are stable across processes and hosts.** A 4KiB block
   on host A must hash to the same value on host B. No wall-clock, no
   process id, no host-specific entropy.
3. **`moduleId` is part of the block hash.** Project A and Project B
   can have identical prompt content and must still miss each other's
   cache. This is a tenancy invariant.
4. **Every `KV_POOL_SYNC` event writes a ledger row.** Via `slm-ledger`.

## Next work units

See `TASKS.md`. First real task: the block hash function with property
tests proving stability across re-runs.
