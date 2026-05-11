---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: pointsav-monorepo
target_path: system-core/
target_filename: README.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-README
authored: 2026-04-28T02:00:00Z
authored_by: task-project-system (session 181c94d9ca0491c5, ps-administrator identity)
authored_with: sonnet-4-6
references:
  - commit 9b5e4fd (Phase 1A.4 InclusionProof)
  - commit 82b659f (Phase 1A.5 ConsistencyProof)
  - DOCTRINE.md claim #33
  - RFC 9162 §2 (Certificate Transparency 2.0)
notes_for_editor: |
  README refresh per v0.1.31 Reverse-Funnel pattern. system-core 0.1.0 -> 0.2.0
  shipped substantial public-API surface: Capability + WitnessRecord +
  LedgerAnchor + C2SP SignedCheckpoint + RFC 9162 inclusion + consistency
  proofs. v0.2.x is structurally complete per Master's framing. Audience is
  vendor-public for the GitHub README; assumes financially-literate-but-not-
  cryptography-expert reader.
---

# system-core

[ Leer este documento en Español ](./README.es.md)

**Entity:** PointSav Digital Systems (The Vendor)
**Taxonomy:** Tier-1 Core Component — Capability Ledger Substrate primitives
**Version:** 0.2.0 — structurally complete
**Cluster:** `cluster/project-system` per workspace `PROJECT-CLONES.md`

---

Substrate-primitive crate for The Capability Ledger Substrate (Doctrine
claim #33). Defines the pure-data and cryptographic types every other
`system-*` and `moonshot-*` crate binds against.

---

## I. What it provides

`system-core` is the leaf crate in the Capability Ledger Substrate
dependency graph — it depends on nothing else in the workspace; every
crate that deals with capabilities depends on it. Two layers of
primitives live here.

**Capability data primitives.** The `Capability` type is the
kernel-mediated authorization token that the seL4 capability-derivation
tree (CDT) carries. Each capability names its type, permitted rights,
an optional expiry timestamp, and a `LedgerAnchor` that pins it to a
specific position in the customer-rooted Merkle log. The `WitnessRecord`
type extends a capability past its expiry per Mechanism A (Time-Bound
Capabilities) — the kernel consults the ledger to verify the witness
record's Merkle inclusion before honoring the extension.

**Cryptographic primitives.** The `checkpoint` module implements the
full C2SP signed-note wire format: parse, render, and ed25519
verification for single-signer and multi-signer checkpoints. Multi-
signature support is the apex-rotation primitive — at a ledger
ownership handover, the outgoing and incoming apex holders co-sign the
same checkpoint body; the kernel accepts the handover only when both
signatures verify. The `inclusion_proof` and `consistency_proof`
modules implement RFC 9162 v2 Merkle proofs compatible with C2SP
tlog-tiles. Inclusion proofs gate witness-record arrival; consistency
proofs allow ledger mirrors to verify they hold an honest prefix of the
canonical log.

Sibling crates in the same substrate layer: `system-substrate`
(hardware bridge), `system-security` (cryptographic pairing),
`system-verification` (proof-artefact container), `system-audit`
(audit sub-ledger), `system-ledger` (state-machine consumer of
these primitives).

## II. Status

Version 0.2.0. The v0.2.x public-API surface is structurally complete:
all six type families listed below are defined, exported, and covered
by 51 passing unit tests on Rust stable. The Stage-6 promotion path is
structurally unblocked; v1.0.0 awaits test-coverage and benchmark
ratification per the project brief. `system-ledger` (the state-machine
consumer) is the active development frontier as of Phase 1A increment 3.

## III. Type taxonomy

### Capability primitives

| Type | Role |
|---|---|
| `Capability` | Kernel-mediated authorization token — `(cap_type, rights, expiry_t, witness_pubkey, ledger_anchor)` |
| `CapabilityType` | Enum of authorized resource classes: `Endpoint`, `Memory`, `Irq`, `Notification`, `CNode` |
| `Right` | Permitted operations: `Read`, `Write`, `Invoke`, `Grant`, `Revoke` |
| `LedgerAnchor` | Reference to a C2SP signed-note checkpoint by `(origin, tree_size, root_hash)` |
| `WitnessRecord` | Extends a `Capability` past `expiry_t`; kernel verifies Merkle inclusion before honoring |
| `Hash256` | `[u8; 32]` alias — SHA-256 baseline per `worm-ledger-design.md` §3 D3 |

### C2SP signed-note checkpoint

| Type | Role |
|---|---|
| `Checkpoint` | Checkpoint body: `(origin, tree_size, root_hash, extensions)` with canonical `body_bytes()` / `parse_body()` |
| `NoteSignature` | One signature line: signer name + 4-byte key-hash prefix + 64-byte ed25519 signature; `to_line()` / `parse_line()` |
| `SignedCheckpoint` | Body + ≥ 1 signature lines; full wire-format `to_wire()` / `parse()`; composed verification methods |

### RFC 9162 Merkle proofs

| Type | Role |
|---|---|
| `InclusionProof` | Proves a leaf is in the tree at `(tree_size, root_hash)`; per RFC 9162 §2.1.3 |
| `ConsistencyProof` | Proves tree at `(old_root, old_size)` is a prefix of `(new_root, new_size)`; per RFC 9162 §2.1.4 |

## IV. Composed verification methods on SignedCheckpoint

These are the kernel-facing primitives. Call these rather than the
raw proof verifiers directly; treating signature verification and
Merkle proof verification as a single atomic operation prevents
"verified proof against an untrusted root" errors.

### `verify_signer(signer_name, pubkey) -> Result<bool, VerifyError>`

Verifies a specific named signer's ed25519 signature over the
checkpoint body. Returns `Ok(true)` if a signature line matching the
name and key-hash prefix is found and verifies; `Ok(false)` if no
matching line is found.

### `verify_apex_handover(old_name, old_pubkey, new_name, new_pubkey) -> Result<bool, VerifyError>`

The apex-rotation handover predicate. Returns `Ok(true)` only when
both the outgoing and incoming apex signatures are present and verify
on the same checkpoint body. A single signature — even a valid one —
returns `Ok(false)`.

### `verify_inclusion_proof(proof, leaf_hash, signer_name, signer_pubkey) -> Result<(), CheckpointInclusionError>`

Composed primitive. Verification order:

1. `proof.tree_size == self.checkpoint.tree_size` — `TreeSizeMismatch` on failure
2. Signer signature valid — `SignatureInvalid` on failure
3. `proof.verify(leaf_hash, &self.checkpoint.root_hash)` — `Inclusion(InclusionVerifyError)` on failure

### `verify_consistency_proof(proof, old_size, new_size, old_signed_checkpoint, signer_name, signer_pubkey) -> Result<(), CheckpointConsistencyError>`

Composed primitive. Verification order:

1. `old_signed_checkpoint.tree_size == old_size` — `OldTreeSizeMismatch`
2. `self.checkpoint.tree_size == new_size` — `NewTreeSizeMismatch`
3. Old checkpoint signature valid — `OldSignatureInvalid`
4. New checkpoint (self) signature valid — `NewSignatureInvalid`
5. `proof.verify(old_root, old_size, new_root, new_size)` — `Consistency(ConsistencyVerifyError)`

## V. Build and test

```
cargo build -p system-core
cargo test  -p system-core
```

51 tests on Rust stable; no external services required. Distribution
across modules:

| Module | Tests |
|---|---|
| `lib.rs` (capability data shape) | 6 |
| `checkpoint.rs` (C2SP signed-note + apex-cosigning) | 16 |
| `inclusion_proof.rs` (RFC 9162 §2.1.3) | 14 |
| `checkpoint.rs` composed inclusion (Phase 1A.4) | 5 |
| `consistency_proof.rs` (RFC 9162 §2.1.4) | 11 |
| `checkpoint.rs` composed consistency (Phase 1A.5) | 5 |
| **Total** | **51** |

Full test run on `cargo test -p system-core` with zero warnings.
Workspace-level `cargo check -p system-core -p system-ledger` also
passes clean.

## VI. Hard constraints

These constraints are doctrine-bound and must not be changed without
a doctrine MINOR per `system-substrate-doctrine.md` §10:

- **`Capability` field set is fixed.** Adding or removing fields
  requires a doctrine MINOR, not an in-crate decision.
- **SHA-256 is the baseline hash function.** Algorithm-agility is
  structural — a future MINOR may add BLAKE3 or SHA-3 alongside
  SHA-256, never instead of it.
- **Witness-record signature namespace is `capability-witness-v1`.**
  Do not reuse the commit-signing or apprenticeship-verdict
  namespaces. Cross-namespace replay protection is the purpose of
  the namespace tag.
- **The crate is no_std-eligible long-term.** The kernel may consume
  it. v0.2.x carries `std` for `Vec` and JSON serialization; a
  future MINOR carves the no_std path. Do not add std-only
  dependencies without surfacing the architectural question.

## VII. Dependencies

All are no_std-capable (std feature enabled for v0.2.x):

| Crate | Purpose |
|---|---|
| `serde` + `serde_json` | Serialization for capability data primitives (v0.1.x; future MINOR moves to canonical CBOR) |
| `sha2` | SHA-256 for `Capability::hash()` and RFC 9162 Merkle hash helpers |
| `ed25519-dalek` | ed25519 public-key verification for `SignedCheckpoint::verify_signer` |
| `base64` | Base64 encoding for the C2SP signed-note wire format |

No workspace-internal dependencies. Zero external transitive crates
beyond these five.

## VIII. What is downstream

**`system-ledger`** (Phase 1A increment 3, Active) is the state-machine
consumer. It imports `Capability`, `WitnessRecord`, `SignedCheckpoint`,
and `LedgerAnchor` from this crate and owns the checkpoint cache,
revocation set, apex history, and the `LedgerConsumer` trait with its
`InMemoryLedger` implementation. `system-ledger` is the crate that
actually runs `consult_capability(cap, current_root) -> Verdict`.

Future consumers of `system-ledger`'s `LedgerConsumer` trait include
a planned `MoonshotDatabaseLedger` implementation that would swap
in-memory state for durable storage — the trait is the seam.

The eventual kernel-side carve-out (no_std path for the capability
primitives consumed directly in seL4 CDT extension code) is a future
MINOR tracked in `system-core/NEXT.md`.

## IX. Cross-references

- `DOCTRINE.md` §II claim #33 — The Capability Ledger Substrate
  (constitutional anchor)
- `conventions/system-substrate-doctrine.md` §3.1 + §5.1 — kernel
  binding and Mechanism A schemas
- `conventions/worm-ledger-design.md` §2 — four-layer WORM stack;
  `system-core` is the L0 schema layer
- [RFC 9162](https://datatracker.ietf.org/doc/html/rfc9162) —
  Certificate Transparency 2.0 §2 (Merkle tree algorithms)
- [C2SP signed-note](https://github.com/C2SP/C2SP/blob/main/signed-note.md)
  and tlog-checkpoint — wire format for `Checkpoint` / `SignedCheckpoint`
- `topic-merkle-proofs-as-substrate-primitive.md` (planned TOPIC in
  `content-wiki-documentation`) — reader-facing narrative on how
  Merkle proofs function as substrate primitives in this architecture
- `system-core/ARCHITECTURE.md` — crate-scope and crate-boundary
  rationale; §3 records the `system-ledger` architecture decision
- `system-core/CLAUDE.md` — active-session operational guidance

## X. Licensing

Inherits the monorepo `LICENSE` at the repo root. Per
`vendor/factory-release-engineering/LICENSE-MATRIX.md` mapping for
`pointsav-monorepo`.
