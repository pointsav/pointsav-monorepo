# NEXT.md — system-core

> Last updated: 2026-04-26
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

- Phase 1A increment 1 landed: `Capability`, `WitnessRecord`,
  `LedgerAnchor`, `CapabilityType`, `Right` defined; 6 unit tests
  pass; crate is a workspace member. Source: cluster brief at
  `~/Foundry/clones/project-system/.claude/inbox.md`.

## Queue

- Define `LedgerEntry` enum covering grant / revoke / extend (witness
  record) / apex-rotate variants. Each entry is the payload type the
  customer's WORM ledger actually stores (per
  `system-substrate-doctrine.md` §3 — "the deployment IS the ledger").
- Choose: does the kernel-side ledger-consultation logic live in
  `system-substrate` (extending the existing crate) or in a new
  `system-capability-ledger` / `system-ledger` crate? Document the
  decision in `ARCHITECTURE.md` once made; surface to Master via
  outbox.
- Design `Capability::canonical_bytes()` for hash stability across
  serde-format swaps. v0.1.x uses serde JSON; canonical CBOR is the
  expected target for hash stability under format migration.
- Add a `cap_type` variant for `IRQHandler` distinct from `Irq`
  (matches seL4 capability classes — Irq is the broad slot,
  IRQHandler is the actual handler capability). Cross-check seL4
  CDT taxonomy first.
- Optional: `criterion` benchmark of `Capability::hash()` so the
  cache-hit / cache-miss latency budget for kernel-side consultation
  has a concrete number to reason against.

## Blocked

- C2SP signed-note primitive integration — Blocked on: architecture
  decision (borrow Sigstore Rust code vs implement from C2SP spec).
  Tracked at the cluster level in the inbox brief; will be resolved
  alongside the system-ledger / system-substrate carve-out decision.

## Deferred

- `master-relay.rs` defect — Deferred: the file predates this cluster
  and shells out to non-existent binaries. Belongs in a closure pass
  that audits all top-level `*.rs` files in projects against
  `repo-layout.md`. Not blocking Phase 1A.
- `no_std` carve-out — Deferred: v0.1.x carries `std` for `Vec` and
  JSON. The kernel-consumption path needs `no_std` but only after
  Phase 1A scaffolding stabilises.

## Recently done

- 2026-04-26: Phase 1A increment 1 — Capability + WitnessRecord +
  LedgerAnchor types defined; 6 tests pass; workspace-member
  registration; framework §9 activation (CLAUDE.md + AGENTS.md +
  NEXT.md + ARCHITECTURE.md).
