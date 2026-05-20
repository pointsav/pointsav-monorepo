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
  checkpoints) — that's the new sibling crate **`system-ledger`**
  (architecture decision resolved in §3 below; Master directive
  2026-04-26). The crypto primitive lives here; the policy lives
  downstream.
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

## 3. Architecture decision — kernel binding lives in `system-ledger`

**Resolved 2026-04-26 (Master Claude reply, archived inbox):**
**Option B** — the kernel-side ledger consultation state machine
lives in a new sibling crate **`system-ledger`** that depends on
`system-core` for the data-primitive types. `system-substrate`
keeps its hardware-bridge focus per its registry description.

Rationale (Master + Task agreed):

- **Clean crate boundary for non-trivial state.** Checkpoint cache
  + revocation set + apex-history + post-handover invariant
  enforcement is a state machine that deserves a focused unit.
- **Mirrors the service-fs pattern.** `worm-ledger-design.md` puts
  the WORM primitive in `service-fs`; the substrate-tier consumer
  parallels in `system-ledger`. Two crates, same C2SP signed-note
  format, decoupled by layer.
- **`system-substrate` keeps hardware-bridge focus.** Conflating
  ledger consultation with hardware bridges would muddy the
  substrate crate's identity.
- **Convention §3.1 alignment.** The convention specifies "extend
  the seL4 capability-derivation tree (CDT) to carry a
  `ledger_anchor` field per capability" — that extension code is
  ledger-side consumer logic, naturally lives in `system-ledger`.
  Convention text remains as-written; the crate boundary is
  refinement below the convention's altitude.

### Module layout in `system-ledger`

| Module | Owns |
|---|---|
| `cache.rs` | Recent-N checkpoint cache; lookup by `(origin, tree_size)` and `(origin, root_hash)`; LRU eviction at N entries |
| `revocation.rs` | Revoked-capability set keyed by `capability_hash`; `apply_revocation`, `is_revoked` |
| `apex.rs` | Apex history; post-handover invariant ("only P-new accepted from N+3+"); `apply_apex_handover` |
| `witness.rs` | `ssh-keygen -Y verify` wrapper for witness-record signatures (namespace `capability-witness-v1`); shells out via `tokio::spawn_blocking` per the apprenticeship VerdictVerifier pattern in project-slm |
| `lib.rs` | `LedgerConsumer` trait; `consult_capability(cap, current_root) -> Result<Verdict>`; in-memory impl `InMemoryLedger` for v0.1.x |

`system-ledger` depends on `system-core` for `Capability`,
`WitnessRecord`, `SignedCheckpoint`, `LedgerAnchor`. Workspace
member.

Trait keeps the door open for `MoonshotDatabaseLedger` future
MINOR (mirrors the `LedgerBackend` trait pattern in
`worm-ledger-design.md` §3 D7).

### Status

**RESOLVED + IMPLEMENTED.** `system-ledger` v0.2.1 is an active workspace
member. All five modules (cache, revocation, apex, witness, lib) are fully
implemented with 44 tests and 10 criterion benchmarks. See `system-ledger/`
for the full implementation.

## 4. Cross-references

- `~/Foundry/DOCTRINE.md` §II claim #33 (constitutional anchor)
- `~/Foundry/conventions/system-substrate-doctrine.md` §3.1 (kernel
  binding), §5.1 (Mechanism A schemas)
- `~/Foundry/conventions/worm-ledger-design.md` §2 (four-layer stack
  — `system-core` is L0 schema; `service-fs` carries L1+L2 for the
  application-tier WORM ledger; `system-ledger` carries the
  substrate-tier consumer of the same format)
- `~/Foundry/RESEARCH-system-substrate.md` §1.1 + §2 (the leapfrog
  framing) and Appendix E (capability-as-ledger-entry as the
  structural-slot novelty)
- `~/Foundry/CLAUDE.md` §3 (`allowed_signers` SSH-signing primitive
  generalised here for `capability-witness-v1` namespace; `ssh-keygen
  -Y verify` wrapper lives in `system-ledger/src/witness.rs`)
- **Sibling crate `system-ledger`** (Phase 1A increment 3) — the
  state-machine consumer of these primitives. Owns checkpoint cache,
  revocation set, apex history, witness verification.

## 5. Verification

**51 tests passing** on `cargo test -p system-core` (Rust stable), zero warnings.

**Capability data-shape (`lib::tests`)** (6 tests):
- `capability_serialises_round_trip`
- `capability_hash_is_deterministic`
- `capability_hash_changes_with_expiry`
- `capability_hash_changes_with_anchor`
- `witness_record_serialises_round_trip`
- `ledger_anchor_serialises_round_trip`

**C2SP signed-note + apex-cosigning (`checkpoint::tests`)** (10 tests):
- `checkpoint_body_round_trip`
- `checkpoint_with_extensions_round_trip`
- `key_hash_derivation_is_deterministic`
- `key_hash_changes_with_name`
- `signed_checkpoint_wire_round_trip_single_sig`
- `single_signature_verifies`
- `signature_fails_under_wrong_pubkey`
- `multi_sig_apex_handover_round_trip`
- `handover_fails_if_only_one_signs`
- `body_tampering_breaks_signature`

**RFC 9162 §2.1.3 inclusion proofs (`inclusion_proof::tests`)** (14 tests):
- `rfc9162_leaf_hash_includes_zero_prefix`
- `rfc9162_internal_hash_includes_one_prefix`
- `single_leaf_tree_proof_is_empty`
- `two_leaf_tree_proofs_verify`
- `four_leaf_tree_proofs_verify`
- `eight_leaf_tree_proofs_verify`
- `odd_leaf_tree_proofs_verify`
- `tampered_sibling_fails`
- `wrong_leaf_hash_fails`
- `wrong_root_fails`
- `leaf_index_out_of_bounds_fails`
- `path_too_long_fails`
- `path_too_short_fails`
- `proof_does_not_verify_for_other_leaf`

**RFC 9162 §2.1.4 consistency proofs (`consistency_proof::tests`)** (11 tests):
- `identity_case_empty_proof_same_root_verifies`
- `old_size_zero_rejected`
- `old_size_exceeds_new_size_rejected`
- `equal_sizes_non_empty_proof_rejected`
- `single_leaf_extension_verifies`
- `power_of_two_extensions_verify`
- `non_power_of_two_sizes_verify`
- `mismatched_old_root_rejected`
- `mismatched_new_root_rejected`
- `corrupt_proof_hash_rejected`
- `full_grid_1_to_8_verifies` — all 36 `(old, new)` pairs with `0 < old ≤ new ≤ 8`

**Composed checkpoint primitives — `verify_inclusion_proof` (Phase 1A.4) and `verify_consistency_proof` (Phase 1A.5)** (10 tests, in `checkpoint::tests`):
- `verify_inclusion_proof_valid`
- `verify_inclusion_proof_tree_size_mismatch`
- `verify_inclusion_proof_bad_signature`
- `verify_inclusion_proof_proof_corrupted`
- `verify_inclusion_proof_wrong_leaf_hash`
- `verify_consistency_proof_valid`
- `verify_consistency_proof_old_size_mismatch`
- `verify_consistency_proof_new_size_mismatch`
- `verify_consistency_proof_old_signature_invalid`
- `verify_consistency_proof_proof_corrupted`

The higher-level state machine ("subsequent checkpoints require only P-new") lives in
`system-ledger` — covered there with 44 tests including the end-to-end handover ceremony.
