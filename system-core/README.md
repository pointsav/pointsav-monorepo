# 🏗️ SYSTEM-CORE

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems™ (The Vendor)
**Taxonomy:** Tier-1 Core Component
**Status:** Active (Phase 1A — Capability Ledger Substrate primitives)
**Cluster:** [`cluster/project-system`](../../) per workspace `PROJECT-CLONES.md`

---

## I. PURPOSE

`system-core` is the data-primitive crate for **The Capability
Ledger Substrate** — the substrate's capability state IS the WORM
ledger. This crate defines the `Capability` and `WitnessRecord`
types every other `system-*` and `moonshot-*` crate binds against.

Constitutional anchor: Doctrine claim #33. Operational specification:
[`conventions/system-substrate-doctrine.md`](../../../conventions/system-substrate-doctrine.md)
§3.1 + §5.

## II. WHAT IT CONTAINS

- `Capability { cap_type, rights, expiry_t, witness_pubkey,
  ledger_anchor }` — the kernel-mediated authorisation token,
  ledger-bound.
- `WitnessRecord` — extends a capability past `expiry_t` per
  Mechanism A (Time-Bound Capabilities).
- `LedgerAnchor` — references a C2SP signed-note checkpoint by
  `(origin, tree_size, root_hash)`.
- `CapabilityType`, `Right`, `Hash256` — supporting enums and aliases.

## III. WHAT IT DOES NOT CONTAIN

- Kernel-side ledger consultation logic — lives downstream in
  `system-substrate` or a new `system-capability-ledger` crate
  (architecture decision pending; see `ARCHITECTURE.md` §3).
- WORM tile storage — `service-fs` per `worm-ledger-design.md`.
- Witness signature verification — wraps `ssh-keygen -Y verify` in
  the consumer crate.

## IV. BUILD AND TEST

```
cargo check -p system-core
cargo test  -p system-core
```

Six unit tests cover serialisation round-trips and hash determinism.

## V. LICENSING

Inherits the monorepo `LICENSE` at the repo root. Per
`vendor/factory-release-engineering/LICENSE-MATRIX.md` mapping for
`pointsav-monorepo`.

## VI. REFERENCES

- `~/Foundry/DOCTRINE.md` §II claim #33
- `~/Foundry/conventions/system-substrate-doctrine.md`
- `~/Foundry/RESEARCH-system-substrate.md` (workspace-root staging)
- This project's `CLAUDE.md`, `ARCHITECTURE.md`, `NEXT.md`
