# 🏗️ SYSTEM-LEDGER

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems™ (The Vendor)
**Taxonomy:** Tier-1 Core Component
**Status:** Active (Phase 1A increment 3 — kernel-side state machine)
**Cluster:** [`cluster/project-system`](../../) per workspace `PROJECT-CLONES.md`

---

## I. PURPOSE

`system-ledger` is the substrate-tier **state-machine consumer** of
the Capability Ledger Substrate primitives in `system-core`. It owns
the kernel-side decision logic that decides whether to honor a
capability invocation: checkpoint cache, revoked-capability set,
apex history with post-handover invariant enforcement, witness-record
signature verification.

Created 2026-04-27 per Master Claude Option B directive. Mirrors
the `service-fs` pattern from `worm-ledger-design.md`: same C2SP
signed-note format, decoupled by tier.

Constitutional anchor: Doctrine claim #33. Operational specification:
[`conventions/system-substrate-doctrine.md`](../../../conventions/system-substrate-doctrine.md)
§3.1 + §4 + §5.

## II. WHAT IT CONTAINS

- `LedgerConsumer` trait — the kernel-facing API
- `InMemoryLedger` — concrete impl for v0.1.x
- `cache::CheckpointCache` — recent-N checkpoint LRU cache
- `revocation::RevocationSet` — O(1) revoked-capability membership
- `apex::ApexHistory` — append-only apex history with post-handover
  invariant
- `witness::verify_witness_signature` — `ssh-keygen -Y verify`
  wrapper for witness records (namespace `capability-witness-v1`)
- `Verdict` / `RefuseReason` / `ConsultError` / `LedgerError` enums

## III. WHAT IT DOES NOT CONTAIN

- Data primitives (Capability, WitnessRecord, SignedCheckpoint,
  LedgerAnchor) — those live in `system-core`.
- WORM tile storage — `service-fs` per `worm-ledger-design.md`.
- seL4 CDT integration — Phase 4+ when the substrate touches
  bare-metal seL4.

## IV. BUILD AND TEST

```
cargo check -p system-ledger
cargo test  -p system-ledger
```

## V. STATE

Skeleton: trait + types + module stubs. Module impls land in
subsequent commits per cluster task list (#18 cache, #19 revocation,
#11 apex, #12 witness, #20 LedgerConsumer impl, #21 benchmarks).

## VI. LICENSING

Inherits the monorepo `LICENSE` at the repo root.

## VII. REFERENCES

- `~/Foundry/DOCTRINE.md` §II claim #33
- `~/Foundry/conventions/system-substrate-doctrine.md` §3.1 + §4 + §5
- Sibling crate `../system-core/`
- This project's `CLAUDE.md`, `ARCHITECTURE.md`, `NEXT.md`
