# CHANGELOG — system-ledger

One line per PATCH. MINOR bumps get a section header. See `~/Foundry/CLAUDE.md` §7 for versioning rules.

---

## 1.0.0 — 2026-05-27

- v1.0.0: API stable — LedgerConsumer trait, InMemoryLedger, CheckpointCache, RevocationSet, ApexHistory, witness verification; 47 tests, 12 benchmarks; AGPL-3.0-or-later

## 0.2.x

- v0.2.1 (2026-05-20): Group 2 hygiene pass — 47 tests (3 gap additions in 2D: InconsistentState, NoApexForCheckpoint, handover-height success), clippy/fmt/rustdoc clean, BENCHMARKS.md published (12 entries), Cargo.toml metadata (license, description, repository, rust-version 1.73), benches 11–12 fixed (consistency-proof path-length correction)
- v0.2.0 (2026-04-27): All five modules fully implemented — cache.rs (CheckpointCache LRU 64-entry), revocation.rs (RevocationSet O(1)), apex.rs (ApexHistory + N+3+ invariant), witness.rs (ssh-keygen -Y verify wrapper), lib.rs (LedgerConsumer + InMemoryLedger); 44 tests; 10 criterion benchmarks; apply_witness_record takes InclusionProof parameter
- v0.1.21 (2026-04-27): Skeleton commit — all four modules as compilable stubs; LedgerConsumer trait + public API surface defined; workspace member registration
