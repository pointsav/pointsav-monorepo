# ARCHITECTURE — system-core

Part of `cluster/project-system` per workspace `PROJECT-CLONES.md`.

Implements primitives for Doctrine claim #33 (The Capability Ledger
Substrate) per `~/Foundry/conventions/system-substrate-doctrine.md`
§3.1 + §5. The Time-Bound Capabilities mechanism (claim #33's
Mechanism A) is realised in this crate's [`WitnessRecord`] type and
the kernel verifier downstream.

---

## 1. Scope

`system-core` is the data-primitive + format-primitive crate. It owns:

- The [`Capability`] type — kernel-mediated authorisation token,
  ledger-bound, with `(cap_type, rights, expiry_t, witness_pubkey,
  ledger_anchor)` per convention §5.1 + §3.1.
- The [`WitnessRecord`] type — extends a capability past `expiry_t`
  per Mechanism A.
- The [`LedgerAnchor`] type — references a C2SP signed-note
  checkpoint by `(origin, tree_size, root_hash)` per
  `worm-ledger-design.md` §3 D2.
- The [`Checkpoint`], [`NoteSignature`], [`SignedCheckpoint`] types
  in the [`checkpoint`] submodule — full C2SP signed-note wire
  format (parse + render) plus ed25519 verification, including
  multi-sig support that realises the apex-cosigning ceremony per
  convention §4.
- Supporting enums (`CapabilityType`, `Right`) and the `Hash256`
  alias.

It does NOT own (and must not absorb):

- Kernel-side ledger consultation **state machine** (which apex is
  current; which capabilities are revoked; cache of recent
  checkpoints) — that's `system-substrate` or a new
  `system-capability-ledger` / `system-ledger` crate (open
  architecture question; see §3 below). The crypto primitive lives
  here; the policy lives downstream.
- WORM tile storage — `service-fs` per `worm-ledger-design.md` §5.
- `ssh-keygen -Y verify` wrapper for witness-record signatures —
  that's a deploy-side concern; the data shape only is here. Note
  that witness records use the SSH-signing primitive (per
  `apprenticeship-substrate.md` §5) while checkpoints use raw C2SP
  ed25519 — two different signing surfaces, both supported.

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

16 unit tests on `cargo test -p system-core` (Rust stable):

**Capability data-shape (`tests`):**
- `capability_serialises_round_trip`
- `capability_hash_is_deterministic`
- `capability_hash_changes_with_expiry`
- `capability_hash_changes_with_anchor`
- `witness_record_serialises_round_trip`
- `ledger_anchor_serialises_round_trip`

**C2SP signed-note + apex-cosigning (`checkpoint::tests`):**
- `checkpoint_body_round_trip`
- `checkpoint_with_extensions_round_trip`
- `key_hash_derivation_is_deterministic`
- `key_hash_changes_with_name`
- `signed_checkpoint_wire_round_trip_single_sig`
- `single_signature_verifies`
- `signature_fails_under_wrong_pubkey`
- `multi_sig_apex_handover_round_trip` — both P-old + P-new signatures
  parse, render, and verify on the same checkpoint body
- `handover_fails_if_only_one_signs` — handover predicate refuses
  when only one apex signs
- `body_tampering_breaks_signature` — modifying the checkpoint after
  signing makes verification fail

Merkle inclusion / consistency proofs and the higher-level "subsequent
checkpoints require only P-new" state machine live downstream where
the consultation logic does — covered when that crate / extension is
chosen and built.
