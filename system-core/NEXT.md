# NEXT.md — system-core

> Last updated: 2026-04-26
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

- Nothing in progress — Phase 1A increments 1 + 2 landed in this
  session. Pick from Queue.

## Queue

- Define `LedgerEntry` enum covering grant / revoke / extend
  (witness record) / apex-rotate variants. Each entry is the payload
  type the customer's WORM ledger actually stores (per
  `system-substrate-doctrine.md` §3 — "the deployment IS the
  ledger"). Should compose with `SignedCheckpoint` for the periodic
  state-anchoring entries.
- Resolve the open architecture question (`ARCHITECTURE.md` §3):
  does the kernel-side state machine — current-apex tracking,
  revocation cache, "subsequent checkpoints require only P-new" —
  live in `system-substrate` (extension) or `system-capability-ledger`
  (new crate)? The C2SP primitive in `checkpoint::*` is signer-
  agnostic; the state machine consumes it.
- Implement `inclusion_proof` and `consistency_proof` per RFC 9162
  + C2SP tlog-tiles to complete the Merkle-log half of the
  capability ledger. Gives `system-core` the full primitive surface
  for downstream use.
- Design `Capability::canonical_bytes()` for hash stability across
  serde-format swaps. v0.1.x uses serde JSON; canonical CBOR is the
  expected target for hash stability under format migration.
- Add a `cap_type` variant for `IRQHandler` distinct from `Irq`
  (matches seL4 CDT taxonomy). Cross-check before landing.
- Optional: `criterion` benchmark of `Capability::hash()` and
  `SignedCheckpoint::verify_signer` so the cache-hit / cache-miss
  latency budget for kernel-side consultation (the Phase 1A
  measurement question per inbox brief) has concrete numbers.

## Blocked

- Higher-level apex-rotation state machine ("only P-new accepted
  on subsequent checkpoints") — Blocked on: kernel-binding-location
  architecture decision. The C2SP primitive is ready; the state
  machine that consumes it needs a home crate.

## Deferred

- `master-relay.rs` defect — Deferred: the file predates this cluster
  and shells out to non-existent binaries. Belongs in a closure pass
  that audits all top-level `*.rs` files in projects against
  `repo-layout.md`. Not blocking.
- `no_std` carve-out — Deferred: v0.1.x carries `std` for `Vec`,
  JSON, base64, ed25519-dalek (which can be no_std but feature-gated
  here). The kernel-consumption path needs `no_std` but only after
  Phase 1A scaffolding stabilises.
- ssh-keygen-format witness-record signatures — Deferred to the
  consumer crate. Witness-record `signature` is a `Vec<u8>` here;
  the `ssh-keygen -Y verify` wrapper lives where the consultation
  state machine lives.

## Recently done

- 2026-04-26: Phase 1A increment 2 — C2SP signed-note checkpoint
  primitive in `src/checkpoint.rs`. Body parse/render, key-hash
  derivation, ed25519 signature verification, multi-sig support
  including the apex-handover predicate. 10 new tests; 16 total in
  the crate.
- 2026-04-26: Phase 1A increment 1 — Capability + WitnessRecord +
  LedgerAnchor types defined; framework §9 activation (CLAUDE.md +
  AGENTS.md + NEXT.md + ARCHITECTURE.md); workspace member
  registration.
