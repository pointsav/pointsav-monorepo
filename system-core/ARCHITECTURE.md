# ARCHITECTURE — system-core

Part of `cluster/project-system` per workspace `PROJECT-CLONES.md`.

Implements primitives for Doctrine claim #33 (The Capability Ledger
Substrate) per `~/Foundry/conventions/system-substrate-doctrine.md`
§3.1 + §5. The Time-Bound Capabilities mechanism (claim #33's
Mechanism A) is realised in this crate's [`WitnessRecord`] type and
the kernel verifier downstream.

---

## 1. Scope

`system-core` is the data-primitive crate. It owns:

- The [`Capability`] type — kernel-mediated authorisation token,
  ledger-bound, with `(cap_type, rights, expiry_t, witness_pubkey,
  ledger_anchor)` per convention §5.1 + §3.1.
- The [`WitnessRecord`] type — extends a capability past `expiry_t`
  per Mechanism A.
- The [`LedgerAnchor`] type — references a C2SP signed-note
  checkpoint by `(origin, tree_size, root_hash)` per
  `worm-ledger-design.md` §3 D2.
- Supporting enums (`CapabilityType`, `Right`) and the `Hash256`
  alias.

It does NOT own (and must not absorb):

- Kernel-side ledger consultation logic — that's `system-substrate`
  or a new `system-capability-ledger` / `system-ledger` crate (open
  architecture question; see §3 below).
- WORM tile storage — `service-fs` per
  `worm-ledger-design.md` §5.
- Witness-signature verification — that wraps `ssh-keygen -Y verify`
  and lives wherever the consultation logic lives.

## 2. Why these types live here

Every other `system-*` and `moonshot-*` crate that deals with
capabilities binds against the same `Capability` shape. Putting the
type in a leaf crate keeps the dependency graph clean: `system-core`
depends on nothing in the workspace; everything else depends on it.

This mirrors the WORM-ledger pattern in
`worm-ledger-design.md` §2 — the L2 `LedgerBackend` trait surface is
defined once in service-fs, and downstream consumers bind against
the trait. `system-core` plays the same role for the capability-
ledger primitive: the schema lives here, the implementations live in
the consumers.

## 3. Open architecture question — kernel binding location

Per the cluster brief (Phase 1A item 3), the kernel-side ledger
consultation logic — given a capability invocation, verify against
the current Merkle root before honoring — needs to live somewhere.
Two candidates:

**Option A** — extend `system-substrate`. The substrate crate already
owns the kernel binding for hardware bridges (per registry
descriptions). Adding a `ledger_consultation` module keeps the
kernel-binding code colocated.

**Option B** — new crate `system-capability-ledger` (or shorter
`system-ledger`). Carves the substrate-level WORM-ledger consumer
out as a focused unit. Mirrors the `service-fs` model where the
WORM primitive is its own concern.

Decision criteria:
- Does the kernel-side cache (per convention §3.1) have non-trivial
  state? If yes → carve to dedicated crate (Option B).
- Does seL4 CDT integration (Phase 4+) cleanly compose with
  hardware-bridge code in `system-substrate`? If yes → keep in
  `system-substrate` (Option A).

**Status:** undecided. Will be resolved when the consultation
simulator is sketched (next Phase 1A increment). Update this section
when the decision is made; surface to Master via outbox.

## 4. Cross-references

- `~/Foundry/DOCTRINE.md` §II claim #33 (constitutional anchor)
- `~/Foundry/conventions/system-substrate-doctrine.md` §3.1 (kernel
  binding), §5.1 (Mechanism A schemas)
- `~/Foundry/conventions/worm-ledger-design.md` §2 (four-layer stack
  — `system-core` is L0 schema; `service-fs` carries L1+L2)
- `~/Foundry/RESEARCH-system-substrate.md` §1.1 + §2 (the leapfrog
  framing) and Appendix E (capability-as-ledger-entry as the
  structural-slot novelty)
- `~/Foundry/CLAUDE.md` §3 (`allowed_signers` SSH-signing primitive
  generalised here for `capability-witness-v1` namespace)

## 5. Verification

Six unit tests in `src/lib.rs` cover:
- Round-trip serialisation for `Capability`, `WitnessRecord`,
  `LedgerAnchor`
- Hash determinism for `Capability::hash()`
- Hash sensitivity to `expiry_t` and `ledger_anchor` fields

Tests pass on `cargo test -p system-core` (Rust stable).

These cover the data-shape invariants. Cryptographic-correctness
fixtures (signature round-trips, Merkle inclusion proofs, apex-cosign
ceremony replay) live downstream where the consultation logic does.
