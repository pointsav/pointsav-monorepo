---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-merkle-proofs-as-substrate-primitive.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-04-28T01:00:00Z
updated: 2026-05-20T00:00:00Z
authored_by: task-project-system (session 181c94d9ca0491c5 skeleton; 37ac0f6b substance)
authored_with: opus-4-7 (skeleton), claude-sonnet-4-6 (substance)
references:
  - https://datatracker.ietf.org/doc/html/rfc9162
  - https://datatracker.ietf.org/doc/html/rfc9162#section-2.1.3
  - https://datatracker.ietf.org/doc/html/rfc9162#section-2.1.4
  - clones/project-system/system-core/src/inclusion_proof.rs
  - clones/project-system/system-core/src/consistency_proof.rs
  - clones/project-system/system-core/src/checkpoint.rs
  - clones/project-system/system-ledger/src/lib.rs
  - DOCTRINE.md claim #33 (The Capability Ledger Substrate)
  - DOCTRINE.md claim #34 (The Two-Bottoms Sovereign Substrate)
  - conventions/worm-ledger-design.md
notes_for_editor: |
  Substance pass complete. All 8 sections written from source code + bench
  numbers in BENCH-v0.2.0.md. Technical accuracy verified against:
    - system-core/src/inclusion_proof.rs (RFC 9162 §2.1.3 implementation)
    - system-core/src/consistency_proof.rs (RFC 9162 §2.1.4 implementation)
    - system-core/src/checkpoint.rs (C2SP signed-note composed primitives)
    - BENCH-v0.2.0.md (benchmark numbers; Phase 1A.4+1A.5 clean run)

  Algorithm walkthroughs use the RFC's variable names (fn_, sn, node, last_node)
  directly. Performance numbers are from the clean Phase 1A.4/1A.5 run on
  Intel Xeon 2.20 GHz, moderate-to-heavy load.

  Banned-vocab + BCSC discipline + bilingual generation: project-language enforces.
  No "blockchain" framings used — Certificate Transparency lineage only.
  Audience: financially literate reader without deep cryptography background.
---

# Merkle Proofs as a Substrate Primitive

Merkle proofs are the cryptographic mechanism that lets Foundry's substrate
guarantee — to any third party, without trust — that a specific record is
part of an append-only log, and that the log has not been rewritten between
two observed points in time. These two guarantees together form the read-side
and the replication-safety side of the Capability Ledger Substrate (Doctrine
claim #33).

This topic explains what Merkle proofs are, why two distinct flavours exist,
how the `system-core` crate implements both per RFC 9162 Certificate
Transparency 2.0, and how the `system-ledger` crate uses them to gate
write-side validity without degrading the read-side hot path.


## 1. What Merkle proofs are

A SHA-256 hash function takes an arbitrary byte sequence and produces a
fixed-length 32-byte fingerprint. Any single-bit change to the input produces
a completely different fingerprint. Given only the fingerprint, recovering the
input is computationally infeasible — this is the collision-resistance property
that all subsequent reasoning rests on.

A Merkle tree builds on this property by organising a sequence of records into
a binary tree of hashes. Each leaf node holds the hash of one record. Each
internal node holds the hash of the concatenation of its two children. The root
— a single 32-byte value — is a commitment to the entire sequence: any change
to any record changes every hash on the path from that leaf to the root, and
therefore changes the root.

RFC 9162 §2.1 specifies one additional requirement: domain separation. Leaf
hashes are computed as SHA-256 of the byte `0x00` followed by the record data.
Internal node hashes are computed as SHA-256 of the byte `0x01` followed by
the left child hash followed by the right child hash. Without this distinction,
a hash of an internal node could be misrepresented as the hash of a leaf
occupying a different position in the tree, enabling second-preimage attacks.
The `system-core` crate implements these exactly:

```rust
// Leaf hash: SHA-256(0x00 || leaf_data)
pub fn rfc9162_leaf_hash(leaf_data: &[u8]) -> Hash256 { ... }

// Internal hash: SHA-256(0x01 || left || right)
pub fn rfc9162_internal_hash(left: &Hash256, right: &Hash256) -> Hash256 { ... }
```

A Merkle inclusion proof for a specific leaf is a list of sibling hashes along
the path from that leaf to the root. A verifier who holds the leaf hash and the
sibling list can reconstruct the root by hashing upward. If the reconstructed
root matches the claimed root, the leaf is in the tree. If not, either the leaf
or the root has been tampered with.

For a tree of `n` leaves, the proof contains at most ⌈log₂ n⌉ sibling hashes —
about 10 hashes for a tree of 1,000 records, 20 hashes for one million records.
The cost of proof storage and verification grows logarithmically, not linearly,
with log size. This is the property that makes Merkle proofs practical as a
substrate primitive at ledger scale.

The difference between *committing* to a log (producing the root) and *proving
membership* in a log (producing a sibling path) is structural: the log operator
maintains the full tree and can produce proofs on demand; a verifier needs only
the root hash, the leaf hash, and the proof path. The log operator cannot forge
a valid proof for a leaf that is not in the tree without breaking SHA-256
collision resistance.


## 2. Two flavours: inclusion and consistency

The same Merkle tree supports two distinct proof types that answer different
questions.

**Inclusion proofs** (RFC 9162 §2.1.3) answer: "Is this specific record's hash
present in the tree at this root?" The consumer holds a record hash, the
claimed position in the log, and the current root. The proof provides the
sibling hashes needed to reconstruct the root from that leaf. This is the
write-side validity check: before recording a witness into the capability
ledger, verify that the witness's hash appears in the Merkle tree covered by
the current signed checkpoint.

**Consistency proofs** (RFC 9162 §2.1.4) answer: "Is this newer tree state a
valid extension of the older one, or has history been rewritten?" The consumer
holds two `(root, tree_size)` pairs — old and new. The proof demonstrates that
the old root is embedded in the structure of the new tree, meaning every leaf
from 0 to `old_size - 1` is identical in both trees. A verifier comparing two
checkpoints from different points in time can use a consistency proof to confirm
that the log only appended records between those two snapshots — it did not
delete, reorder, or modify any earlier record.

The two flavours are complementary. Inclusion proofs gate individual writes
(every witness arrival must prove its place in the current log state before
being accepted). Consistency proofs gate log-mirror catch-up (a replica
advancing from checkpoint A to checkpoint B can verify the extension is clean
before trusting checkpoint B's state). Together they make the capability log
auditable without requiring any party to hold the full log history locally.


## 3. Inclusion proofs in `system-core`

The `InclusionProof` struct in `system-core/src/inclusion_proof.rs` carries
three fields:

```rust
pub struct InclusionProof {
    pub leaf_index: u64,    // 0-indexed position of the proven leaf
    pub tree_size: u64,     // log size at proof generation time
    pub sibling_hashes: Vec<Hash256>,  // path from leaf to root
}
```

Verification follows RFC 9162 §2.1.3 verbatim. The algorithm tracks two
counters named `fn_` (the current node's position, shifting right as it
ascends the tree) and `sn` (the size of the current layer, also shifting
right). At each step:

- If the current node is a right child (`fn_ & 1 == 1`) or has reached the
  frontier of the tree for this layer (`fn_ == sn`), the sibling is to the
  left: hash as `internal_hash(sibling, accumulator)`, then strip any trailing
  even bits from both counters (the inner strip — aligns the counters when the
  tree is not a power of two in size).
- Otherwise, the sibling is to the right: hash as
  `internal_hash(accumulator, sibling)`.

The algorithm terminates correctly when `sn` reaches zero (the root) and the
accumulated hash matches `expected_root`. The error taxonomy distinguishes four
structural conditions: `LeafIndexOutOfBounds`, `PathTooLong`, `PathTooShort`,
and `RootMismatch`.

The test suite covers 11 cases: domain-separation prefix sanity for leaf and
internal hashes; single-leaf tree (proof is empty, root equals the leaf hash);
two-leaf, four-leaf, and eight-leaf trees with every leaf index verified;
odd-sized trees (5 leaves, exercising the right-edge promotion path); tampered
sibling hash; wrong leaf hash; wrong root; leaf index out of bounds; path too
long; path too short; and proof-for-wrong-leaf rejection.

Performance from the clean Phase 1A.4 benchmark run on an Intel Xeon 2.20 GHz:

| Tree size | Sibling path length | Verification time |
|---|---|---|
| 8 leaves | 3 hashes | 5.37 µs |
| 1024 leaves | 10 hashes | 17.74 µs |

The logarithmic cost is confirmed: tripling the path length (3 → 10 hashes)
increases verification time by 3.3×, tracking the additional SHA-256
operations directly. For any tree size a capability ledger would reach in
practice, raw inclusion proof verification stays well under 100 µs.


## 4. Consistency proofs in `system-core`

The `ConsistencyProof` struct in `system-core/src/consistency_proof.rs`
carries a single field:

```rust
pub struct ConsistencyProof {
    pub hashes: Vec<Hash256>,
}
```

The apparent simplicity is deceptive. The algorithm operates two running
accumulators — `old_hash` and `new_hash` — that track the hash paths to the
old and new roots respectively. Both are seeded from `hashes[0]`, which is
the right-frontier leaf of the old tree (the last leaf at index
`old_size - 1`). The remaining `hashes[1..]` are consumed one at a time.

At each step, the verifier tracks `node` (the old tree's current frontier
position) and `last_node` (the new tree's frontier position). The decision at
each hash:

- If `node` is a right child (`node & 1 == 1`) or both frontiers have
  converged (`node == last_node`): combine both accumulators leftward — this
  hash is in the shared prefix of old and new trees. Apply the inner strip
  to both counters.
- Otherwise: combine only `new_hash` rightward — this hash is in the extension
  that exists only in the new tree; the old tree has not yet grown to cover it.

After all hashes are consumed, both `last_node` must be zero (the algorithm
reached the root of both trees), and the two accumulators must equal
`old_root` and `new_root` respectively.

The nine error variants distinguish:

| Error | Condition |
|---|---|
| `OldSizeIsZero` | Empty tree is a degenerate input; caller should handle separately |
| `OldSizeExceedsNewSize` | Trees only grow; this ordering is structurally invalid |
| `EqualSizesNonEmptyProof` | If sizes are equal, the identity proof must be empty |
| `EqualSizesRootMismatch` | Equal sizes, empty proof, but roots differ — roots are inconsistent |
| `EmptyProofForNonZeroOldSize` | Non-trivial extension requires at least one hash |
| `PathTooLong` | Proof consumed `last_node` before all hashes were used |
| `PathTooShort` | Hashes exhausted before `last_node` reached zero |
| `OldRootMismatch` | Accumulated old hash does not match the claimed old root |
| `NewRootMismatch` | Accumulated new hash does not match the claimed new root |

This taxonomy matters for a substrate consumer: `OldRootMismatch` means the
old checkpoint is being misrepresented; `NewRootMismatch` means the new
checkpoint is being misrepresented; `PathTooLong` or `PathTooShort` means the
proof itself was constructed incorrectly or has been tampered with. Each class
of failure has a different operational response.

The test suite includes 11 cases covering the identity case, `OldSizeIsZero`,
`OldSizeExceedsNewSize`, equal sizes with non-empty proof, single-leaf
extension (1 → 2), power-of-two extensions (2 → 4, 4 → 8), non-power-of-two
sizes (3→5, 4→7, 5→7, 6→8, 3→8), mismatched old root, mismatched new root,
corrupt proof hash, and the full 1..=8 grid — every `(old, new)` pair with
`0 < old ≤ new ≤ 8` verifying correctly against independently computed roots.

The full grid test is the critical conformance check: an oracle generates
proofs independently of the verifier (from tree structure, not from the RFC's
recursive PROOF function), and the verifier must accept all 36 pairs. This
cross-verification approach catches algorithm divergence that would pass
round-trip tests within a single implementation.


## 5. Composed primitives on `SignedCheckpoint`

A Merkle root hash is only as trustworthy as the authority that signs it. The
C2SP signed-note format, implemented in `system-core/src/checkpoint.rs`,
binds a Merkle root to a named, ed25519-signing apex. The wire format is:

```
<origin>
<tree-size>
<base64(root-hash)>
[<extension-line>...]

— <signer-name> <base64(4-byte-key-hash || 64-byte-ed25519-sig)>
[— <signer-name-2> ...]
```

The body — origin, tree size, root hash, and optional extension lines, each
terminated by a newline — is what the apex signs. Multiple signature lines on
the same body realise the multi-apex ownership-transfer ceremony: at the
handover checkpoint, both the outgoing apex (P-old) and the incoming apex
(P-new) sign the same body, producing two signature lines. The kernel verifier
confirms the transfer is complete by requiring both signatures at the handover
height.

The composed kernel-facing API sits on `SignedCheckpoint`:

```rust
impl SignedCheckpoint {
    // Chain: tree-size invariants → signature verify → inclusion proof verify
    pub fn verify_inclusion_proof(
        &self,
        proof: &InclusionProof,
        leaf_hash: &Hash256,
        signer_name: &str,
        signer_pubkey: &VerifyingKey,
    ) -> Result<(), CheckpointInclusionError> { ... }

    // Chain: tree-size invariants → both-signature verify → consistency proof verify
    pub fn verify_consistency_proof(
        &self,
        proof: &ConsistencyProof,
        old_checkpoint: &SignedCheckpoint,
        signer_name: &str,
        signer_pubkey: &VerifyingKey,
    ) -> Result<(), CheckpointConsistencyError> { ... }
}
```

`verify_inclusion_proof` performs three checks in sequence. First, the
`InclusionProof`'s `tree_size` field must equal the checkpoint's `tree_size`;
a proof generated against a different tree state is not valid for this
checkpoint. Second, the checkpoint's body must bear a valid ed25519 signature
from the named signer using the provided public key. Third, the raw
`InclusionProof::verify` must confirm the leaf hash reconstructs the root.
All three must pass.

The composition serves a specific purpose: consumers should not call raw
`InclusionProof::verify` directly. A raw proof verify against a root hash the
caller obtained by other means (a local variable, an unverified deserialization)
provides no authentication — only the composition with signature verification
makes the root hash trust-worthy. The `CheckpointInclusionError` taxonomy keeps
the two failure classes distinct: `SignatureError` means the apex cannot be
authenticated; `ProofError(InclusionVerifyError)` means the Merkle path does
not reconstruct the authenticated root.

`verify_consistency_proof` follows the same pattern with an additional step:
it verifies the signature on the *old* checkpoint as well as the new one,
ensuring both ends of the consistency chain are apex-authenticated before the
raw consistency proof is evaluated.


## 6. Consumer integration in `system-ledger`

The `system-ledger` crate owns the kernel-side state machine that decides
whether to honor a capability invocation. The `LedgerConsumer` trait defines
the public contract:

```rust
pub trait LedgerConsumer {
    fn consult_capability(
        &mut self,
        cap: &Capability,
        current_root: &SignedCheckpoint,
    ) -> Result<Verdict, ConsultError>;

    fn apply_witness_record(
        &mut self,
        record: &WitnessRecord,
        proof: &InclusionProof,
        current_checkpoint: &SignedCheckpoint,
        signer_name: &str,
        signer_pubkey: &VerifyingKey,
    ) -> Result<(), LedgerError>;

    // ... revocation and apex methods
}
```

Before Phase 1A.4, `apply_witness_record` accepted a `WitnessRecord` without
any cryptographic verification of its placement in the log. The caller was
trusted to supply only records that belonged to the current checkpoint. This
created a gap: a misconfigured or compromised caller could extend the ledger
with records that never appeared in the signed transparency log.

Phase 1A.4 closed this gap by promoting `apply_witness_record` to require an
`InclusionProof` and a `SignedCheckpoint`. The method now delegates to
`verify_inclusion_proof` before recording the witness. A record is accepted
only if the Merkle proof confirms the record's hash is in the tree covered by
the current apex-signed checkpoint. This is the v0.1.x → v0.2.0 breaking
change: trait signature changed, not just implementation.

The read side operates on a different cost curve. `consult_capability` must
respond quickly — it sits on the kernel-mediated invocation path. Signature
verification at ~4 ms per call would be prohibitive for any capability-intensive
workload. The `CheckpointCache` resolves this:

| Operation | Measured time | Notes |
|---|---|---|
| Cache hit (most-recent entry) | 11.2 ns | O(1) lookup by tree_size |
| Cache miss (full 64-entry scan) | 362 ns | Sequential scan; bounded |
| `verify_signer` (1-sig apex verify) | 4.01 ms | Ed25519, hardware-bound |
| `consult_capability` (Allow path) | 3.74 ms | Cache miss path; dominated by verify |

The 11.2 ns cache hit vs 4.01 ms signature verify is a ~358,000× difference.
Any checkpoint the cache already holds can be consulted without touching the
ed25519 verifier. In a steady-state kernel that sees the same checkpoint across
thousands of capability invocations per second, the cache hit rate is
effectively 100%; the signature verifier runs only when a new checkpoint is
published.

The cache holds the most recent 64 checkpoints by tree_size, keyed for O(1)
lookup. LRU eviction keeps memory bounded. The 64-entry bound was chosen to
cover the apex-handover window (during which P-old and P-new checkpoints
coexist) plus reasonable checkpoint publishing rates without exceeding kernel
working-set constraints.

One design interaction deserves explicit attention: at the handover height N+2,
the apex-handover checkpoint bears both P-old and P-new signatures. The
inclusion proof for a witness record at height N+2 can be verified against
either signer's public key — the question being answered is "is this record in
the tree?" not "who signed the tree?". Consumers requiring strict "both apexes
signed" semantics for handover checkpoints call `verify_apex_handover`
separately, a composed check on top of the individual signature verifies. The
layering is deliberate: inclusion-proof verification answers its own narrow
question; policy about who must sign composes above it.


## 7. Why this matters as a substrate primitive

The Capability Ledger Substrate (Doctrine claim #33) requires that every
capability authorization be anchored to a customer-rooted transparency log.
The customer — not any intermediary — holds the apex signing keys. The
customer can audit the full log. Third parties can verify individual records
against published checkpoints without holding the full log. This structure
has three properties that alternatives do not:

**Auditability without custody.** Any party holding a signed checkpoint and a
witness record can verify the record's presence independently. The customer
does not need to grant the auditor log access; the checkpoint and proof are
sufficient. Revocation of a capability is itself a log entry; an auditor
inspecting a checkpoint from after the revocation will not find a valid proof
for the revoked capability's witness record against that root.

**History immutability.** The combination of inclusion proofs and consistency
proofs makes log rewriting detectable. If the log operator published checkpoint
A at time T₁ and checkpoint B at time T₂, any party who recorded both
checkpoints can demand a consistency proof from A to B. If the operator cannot
produce a valid proof — or if the proof fails to verify — the operator has
rewritten history between T₁ and T₂. The consistency proof is the structural
mechanism that makes "append-only" a verifiable claim rather than a policy
assertion.

**No-trust replication.** A replica advancing from an old checkpoint to a new
one verifies the extension via consistency proof before accepting the new
state. A mirror that cannot produce a valid consistency proof from the last
confirmed state to the claimed new state is either behind (it missed
intermediate checkpoints) or presenting a forked log. The distinction matters
for the Two-Bottoms Sovereign Substrate (Doctrine claim #34): an os-* binary
running on the NetBSD compat-bottom must be able to verify its own capability
state against the same transparency log as the seL4 native-bottom instance,
using no runtime trust relationship between the two.

The `system-core` implementation is eligible for `no_std` compilation. The
crate carries `std` for `Vec` and JSON serialization in v0.2.x; neither
`inclusion_proof.rs` nor `consistency_proof.rs` uses any `std`-only
primitive. A future MINOR version will carve the `no_std` path, enabling
direct kernel consumption from `moonshot-kernel` (the planned Rust no_std
replacement for seL4's capability machinery) without a foreign-function
boundary. The substrate primitive that gates every capability invocation in
userspace can gate it at the kernel level using the same code.

The cryptographic primitives in `system-core` are not novel inventions. RFC 9162
Certificate Transparency 2.0 is a mature IETF standard with multiple
independent implementations. SHA-256 is FIPS 180-4. Ed25519 is RFC 8032.
The `ed25519-dalek` library is widely audited in the Rust ecosystem. What is
new is the composition: wiring a kernel capability type
(`seL4_CNode_derivation → Capability`) to a customer-rooted RFC 9162 log via
C2SP signed-note checkpoints, with inclusion proof gating on write-side
validity and consistency proof gating on replication safety. The composition
creates structural guarantees that neither the kernel nor the log provides in
isolation.


## 8. Cross-references

- **RFC 9162 — Certificate Transparency 2.0**
  https://datatracker.ietf.org/doc/html/rfc9162
  §2.1 (hash tree construction), §2.1.3 (inclusion proofs), §2.1.4
  (consistency proofs). The `system-core` implementation follows RFC 9162
  verbatim; the algorithm variable names in the source (`fn_`, `sn`, `node`,
  `last_node`) match the RFC's pseudocode.

- **C2SP signed-note specification**
  https://github.com/C2SP/C2SP/blob/main/signed-note.md
  The wire format for `Checkpoint` and `NoteSignature`. The 4-byte key-hash
  prefix in each signature line is `SHA-256("<name>\nED25519\n<32-byte-pubkey>")[..4]`.

- **`conventions/worm-ledger-design.md`**
  The WORM ledger design convention that the Capability Ledger Substrate extends.
  §3 D1 (C2SP tlog-tiles compatibility), §3 D3 (SHA-256 as the baseline hash
  function). `system-core` is the L0 schema layer; `system-ledger` is the
  substrate-tier L1+L2 consumer.

- **Doctrine claim #33 — The Capability Ledger Substrate**
  `~/Foundry/DOCTRINE.md` §II claim #33. The constitutional anchor for the
  transparency-log binding. Every capability authorization is a
  cryptographically auditable token anchored to a C2SP signed-note Merkle log
  with customer-held apex keys.

- **Doctrine claim #34 — The Two-Bottoms Sovereign Substrate**
  `~/Foundry/DOCTRINE.md` §II claim #34. The constitutional anchor for
  boot-anywhere capability verification. The seL4 native-bottom and NetBSD
  compat-bottom share the same capability ledger substrate; Merkle proofs are
  the verification mechanism that works identically across both.

- **`conventions/system-substrate-doctrine.md`**
  The operational specification for the capability ledger. §3.1 (Capability
  type schema), §4 (apex ownership-transfer ceremony — N+3+ post-handover
  invariant), §5.1 (WitnessRecord schema), §6.1 (reproducible-verification
  artefact format).

- **`cluster/project-system` — Phase 1A implementation commits**
  - `9b5e4fd` — system-core: inclusion_proof.rs + SignedCheckpoint::verify_inclusion_proof (system-core 0.1.3)
  - `82b659f` — system-core: consistency_proof.rs + SignedCheckpoint::verify_consistency_proof (system-core 0.2.0)
  - `2b9ca9c` — system-ledger: apply_witness_record inclusion-proof gated (v0.1.x → v0.2.0 breaking change)
  - `0d6da97` — criterion benchmarks for the composed verification paths
