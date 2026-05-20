# NEXT.md — system-ledger

> Last updated: 2026-05-20
> Read at session start. Update before session end.

---

## Right now

- Nothing in progress. Group 2D test-gap items are queued below.

## Queue

- **Group 2D — test gap closure (3 items):**
  - Add `ConsultError::InconsistentState` explicit test.
  - Add `LedgerError::NoApexForCheckpoint` explicit test.
  - Add `apply_witness_record` at handover height test (confirm inclusion
    proof is checked against the handover checkpoint root).

## Blocked

- Nothing currently blocked.

## Deferred

- **`MoonshotDatabaseLedger` impl** — requires `moonshot-database` to
  ship (currently 4-file placeholder per registry). Trait keeps the
  door open per `worm-ledger-design.md` §3 D7 dual-target pattern.
- **Multi-threaded `LedgerConsumer`** — v0.1.x is single-writer per
  the kernel substrate model. Future MINOR may add Arc<Mutex<_>>.

## Recently done

- 2026-05-20: Group 2C hygiene — CLAUDE.md + NEXT.md + ARCHITECTURE.md
  updated to reflect v0.2.1 fully-delivered state. BENCHMARKS.md added.
- 2026-04-28 (Phase 1A.5): `apply_witness_record` upgraded to take
  `InclusionProof` parameter; witness arrivals now Merkle-proof-gated.
  system-ledger 0.2.0 → 0.2.1. BENCH-v0.2.0.md clean run produced.
- 2026-04-27 (Phase 1A.4): LedgerConsumer trait signature change for
  `apply_witness_record`; `current_checkpoint` field + setter on
  InMemoryLedger; `witness_record_leaf_hash` uses rfc9162_leaf_hash.
  4 new lib tests + 1 migrated. system-ledger 0.1.5 → 0.2.0 (MINOR:
  breaking trait-signature change). 4 criterion benches for inclusion
  proof path.
- 2026-04-27 (Phase 1A.3 — benchmarks): 6 criterion benches
  (cache hit 8.08 ns, cache miss 338 ns, verify_signer 3.40 ms,
  consult 3.39 ms). system-ledger 0.2.0 → 0.2.1 PATCH.
- 2026-04-27 (Phase 1A.3 — LedgerConsumer impl): Full InMemoryLedger
  on the LedgerConsumer trait. End-to-end N+3+ ceremony test. 13 lib
  tests. 40 total.
- 2026-04-27 (Phase 1A.3 — witness.rs): ssh-keygen -Y verify wrapper.
  Cross-namespace rejection security property test. 5 tests.
- 2026-04-27 (Phase 1A.3 — apex.rs): ApexHistory + post-handover
  invariant. 10 tests.
- 2026-04-27 (Phase 1A.3 — cache.rs + revocation.rs): CheckpointCache
  LRU + RevocationSet. 12 tests.
- 2026-04-27 (Phase 1A increment 3): crate created; skeleton commit;
  module stubs; workspace member.
