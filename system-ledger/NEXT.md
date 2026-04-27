# NEXT.md — system-ledger

> Last updated: 2026-04-27
> Read at session start. Update before session end.

---

## Right now

- Skeleton landed. Phase 1A increment 3 work shape: fill the four
  module stubs in this order — cache → revocation → apex → witness
  — then wire `LedgerConsumer` impl on `InMemoryLedger`. Each is a
  separate commit per cluster task #18 / #19 / #11 / #12 / #20.

## Queue

- Implement `cache::CheckpointCache` per task #18: insert / lookup
  by `(origin, tree_size)` and `(origin, root_hash)`; LRU eviction
  at `capacity`; tests for insert / lookup / eviction / miss.
- Implement `revocation::RevocationSet` per task #19: add the
  `apply_revocation` API; sidecar `detail` HashMap accessor;
  membership tests; replay-of-already-revoked is a no-op (idempotent).
- Implement `apex::ApexHistory` per task #11: `record_apex(name,
  pubkey, effective_from)`; `apply_apex_handover(...)` that closes
  the prior apex's `effective_until` and appends the new entry;
  `verify_checkpoint_against_apex(checkpoint, height)` that enforces
  the post-handover invariant. Tests: full handover ceremony fixture
  per inbox brief Phase 1A item 4.
- Implement `witness::verify_witness_signature` per task #12: shell
  out to `ssh-keygen -Y verify` with namespace
  `capability-witness-v1`; wrap stdin / stdout / exit code; tests
  use `tempfile` + a fixed test keypair generated via
  `ssh-keygen -t ed25519 -f /tmp/test_key -N ""`.
- Wire the `LedgerConsumer` impl on `InMemoryLedger` per task #20:
  `consult_capability` orchestrates revocation check → apex
  validity → expiry check (with witness extension path); end-to-end
  integration tests.
- criterion benchmarks per task #21: surface cache-hit / cache-miss
  / verify-signer / full-consult numbers for Master 4b deliverable.

## Blocked

- Nothing blocking; module order is strictly internal.

## Deferred

- `MoonshotDatabaseLedger` impl — Deferred: requires
  `moonshot-database` to ship (currently 4-file placeholder per
  registry). Trait keeps the door open per
  `worm-ledger-design.md` §3 D7 dual-target pattern.
- Multi-threaded / concurrent `LedgerConsumer` — Deferred: v0.1.x
  is single-writer matching the kernel substrate model. Future
  MINOR may add `Arc<Mutex<_>>` wrapping or fine-grained locking.

## Recently done

- 2026-04-27: skeleton commit — four module stubs + `LedgerConsumer`
  trait + `Verdict` / `RefuseReason` enums + `InMemoryLedger`
  struct (impl pending). Workspace member; `cargo check -p
  system-ledger` passes; zero warnings; framework §9 activation
  complete.
