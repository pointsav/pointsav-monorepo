# Design Sketch — `system-core/src/consistency_proof.rs`

Ratified per Master Claude direction v0.1.33-pending (Option C, Brief 1).
Pattern source: `system-core/src/inclusion_proof.rs`. Design only — no implementation.

---

## 1. Algorithm summary

A consistency proof answers: "Is `(old_root, old_size)` a prefix of `(new_root,
new_size)`?" Per RFC 9162 §2.1.4, `VERIFY_CONSISTENCY` reconstructs two running
hashes — one recomputing the old root, one extending to the new root — by walking
a path derived from the binary representations of `old_size` and `new_size`. Both
reconstructed values must match the supplied roots. This is the complement of an
inclusion proof: inclusion proves a leaf is present in a specific tree; consistency
proves a tree has grown monotonically without rewriting history. The same 0x01
internal-node domain-separation prefix (RFC 9162 §2.1) applies; no new hash
helpers are needed.

---

## 2. Proposed API — `system-core/src/consistency_proof.rs`

### 2.1. Module rustdoc

```rust
//! RFC 9162 v2 Merkle consistency proofs (compatible with C2SP
//! tlog-tiles per `~/Foundry/conventions/worm-ledger-design.md` §3 D1).
//!
//! Provides "does this newer root extend the older one?" — the
//! replication-safety primitive for ledger-mirror catch-up and
//! multi-witness checkpoint advancement.
//!
//! Reuses [`inclusion_proof::rfc9162_internal_hash`]; no new hash helper.
//! Per RFC 9162 §2.1.4. Composed kernel-facing API:
//! [`SignedCheckpoint::verify_consistency_proof`].
```

### 2.2. Public types

```rust
/// RFC 9162 v2 consistency proof between two tree states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsistencyProof {
    /// Leaf count of the older tree. Corresponds to `old_size` in RFC 9162 §2.1.4.
    pub old_size: u64,
    /// Leaf count of the newer tree. Corresponds to `new_size` in RFC 9162 §2.1.4.
    pub new_size: u64,
    /// Intermediate hashes per RFC 9162 §2.1.4. Empty iff `old_size == 0`
    /// or `old_size == new_size`.
    pub hashes: Vec<Hash256>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsistencyVerifyError {
    /// `old_size > new_size`. Trees can only grow.
    OldSizeExceedsNewSize,
    /// `old_size` is non-zero, `old_size != new_size`, but `hashes` is empty.
    EmptyProofForNonZeroOldSize,
    /// Proof contained more hashes than the tree shape requires.
    PathTooLong,
    /// Reconstructed old root does not match `old_root`.
    OldRootMismatch,
    /// Reconstructed new root does not match `new_root`.
    NewRootMismatch,
}
```

**Editorial note:** RFC 9162 §2.1.4 has a single inconsistency outcome; splitting
into `OldRootMismatch` / `NewRootMismatch` follows `InclusionVerifyError::RootMismatch`
diagnostic discipline. If the algorithm structure does not allow distinguishing the
two, collapse to a single `RootMismatch` variant.

### 2.3. `ConsistencyProof::verify` — prose pseudocode

```rust
impl ConsistencyProof {
    /// Per RFC 9162 §2.1.4 verbatim. `old_root` and `new_root` are
    /// the roots of the two tree states to verify consistency between.
    pub fn verify(
        &self,
        old_root: Hash256,
        new_root: Hash256,
    ) -> Result<(), ConsistencyVerifyError> {
        // STEP 0 — size invariant
        //   old_size > new_size  =>  OldSizeExceedsNewSize
        //
        // STEP 1 — trivial cases
        //   old_size == 0:
        //     hashes non-empty  =>  PathTooLong
        //     return Ok(())     (empty tree is prefix of anything;
        //                        old_root is not inspected — RFC 9162 §2.1.4)
        //   old_size == new_size:
        //     hashes non-empty  =>  PathTooLong
        //     old_root != new_root  =>  OldRootMismatch
        //     return Ok(())
        //
        // STEP 2 — initialise (RFC §2.1.4 notation)
        //   fn = old_size - 1
        //   sn = new_size - 1
        //
        // STEP 3 — strip common right-edge bits (RFC §2.1.4 step 1)
        //   while fn is even:
        //     fn >>= 1;  sn >>= 1
        //
        // STEP 4 — first hash seeds both running accumulators
        //   hashes empty at this point  =>  EmptyProofForNonZeroOldSize
        //   let mut iter = hashes.iter()
        //   old_hash = *iter.next()
        //   new_hash = old_hash      (same starting node)
        //
        // STEP 5 — consume remaining hashes (RFC §2.1.4 step 2)
        //   for p in iter:
        //     sn == 0  =>  PathTooLong
        //     if fn is odd OR fn == sn:
        //       old_hash = rfc9162_internal_hash(p, &old_hash)
        //       new_hash = rfc9162_internal_hash(p, &new_hash)
        //       while fn is even AND fn != 0:
        //         fn >>= 1;  sn >>= 1
        //     else:
        //       new_hash = rfc9162_internal_hash(&new_hash, p)
        //     fn >>= 1;  sn >>= 1
        //
        //   TODO (Task): re-verify loop body and parity conditions
        //   against RFC 9162 §2.1.4 listing before implementing.
        //   The above is a structural approximation; RFC text is authoritative.
        //
        // STEP 6 — root checks
        //   sn != 0  =>  PathTooLong  (path ran short for the new tree)
        //   old_hash != old_root  =>  OldRootMismatch
        //   new_hash != new_root  =>  NewRootMismatch
        //   Ok(())
        todo!()
    }
}
```

### 2.4. Helpers

None introduced. Import from `inclusion_proof`:

```rust
use crate::inclusion_proof::rfc9162_internal_hash;
```

---

## 3. Composed primitive on `SignedCheckpoint`

Add to `checkpoint.rs` alongside `CheckpointInclusionError` / `verify_inclusion_proof`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckpointConsistencyError {
    /// `old_signed_checkpoint.tree_size != proof.old_size`.
    OldTreeSizeMismatch,
    /// `self.checkpoint.tree_size != proof.new_size`.
    NewTreeSizeMismatch,
    /// `signer_pubkey` is malformed.
    BadSignerPublicKey,
    /// Old checkpoint signature did not verify under `(signer_name, signer_pubkey)`.
    OldSignatureInvalid,
    /// New checkpoint (self) signature did not verify.
    NewSignatureInvalid,
    /// Raw consistency proof failed.
    Consistency(ConsistencyVerifyError),
}

impl SignedCheckpoint {
    /// Composed primitive: verify `old_signed_checkpoint` is a consistent
    /// prefix of `self`, with both signed by `(signer_name, signer_pubkey)`.
    ///
    /// Check order (first failure returns):
    /// 1. old tree-size match  →  OldTreeSizeMismatch
    /// 2. new tree-size match  →  NewTreeSizeMismatch
    /// 3. old signature valid  →  BadSignerPublicKey / OldSignatureInvalid
    /// 4. new signature valid  →  BadSignerPublicKey / NewSignatureInvalid
    /// 5. proof.verify(old_root, new_root)  →  Consistency(_)
    pub fn verify_consistency_proof(
        &self,
        proof: &ConsistencyProof,
        old_signed_checkpoint: &SignedCheckpoint,
        signer_name: &str,
        signer_pubkey: &[u8; 32],
    ) -> Result<(), CheckpointConsistencyError> {
        todo!()
    }
}
```

`CheckpointConsistencyError` and `ConsistencyProof` must both be re-exported
from `lib.rs` at crate root, matching the `CheckpointInclusionError` /
`InclusionProof` re-export pattern from system-core 0.1.4.

---

## 4. Unit-test sketches

Match `inclusion_proof.rs::tests` style: `build_root` helper, `fixture_leaves(n)`,
real SHA-256. A `make_consistency_proof(old_leaves, new_leaves)` oracle helper is
needed (implementing Task writes from RFC 9162 §2.1.4 `PROOF` sub-function).

- **`identity_case_empty_proof_verifies`** — `old_size == new_size`, same root,
  empty `hashes` → `Ok(())`.
- **`single_leaf_extension_verifies`** — `old_size=1`, `new_size=2`, one hash.
  Minimal non-trivial extension.
- **`multi_leaf_non_power_of_two_verifies`** — `old_size=4`, `new_size=7`.
  Exercises right-edge odd-promotion in RFC 9162 §2.1.
- **`old_size_zero_verifies`** — `old_size=0`, `new_size=4`, empty `hashes`.
  Confirms RFC 9162's `old_size == 0` edge case (see §5).
- **`mismatched_old_root_rejects`** — valid proof but wrong `old_root` supplied
  → `Err(OldRootMismatch)`.
- **`corrupt_hash_in_proof_rejects`** — one `hashes` entry zeroed → root mismatch
  error.
- **`old_size_exceeds_new_size_rejects`** — `old_size > new_size`
  → `Err(OldSizeExceedsNewSize)`.

---

## 5. RFC v2 vs v1 subtleties

This implementation follows **RFC 9162 v2 §2.1.4** exclusively.

**`old_size == 0` difference from RFC 6962:** RFC 6962 required the proof to
include the new root hash when `old_size == 0`. RFC 9162 §2.1.4 drops that
requirement — when `old_size == 0` the proof is empty and `old_root` is not
inspected. STEP 1 above follows RFC 9162. Do not backport the RFC 6962 behaviour.

**TODO (Task):** Confirm against the published text at
https://datatracker.ietf.org/doc/html/rfc9162#section-2.1.4 and note any errata
before implementing. Pre-training knowledge (cutoff August 2025) is the basis here.

---

## 6. Integration notes

- **Replication catch-up:** `service-fs` mirror consumer trusted at tree-size N
  calls `ConsistencyProof::verify` before advancing its stored root to N+k.
- **Multi-witness advancement:** `SignedCheckpoint::verify_consistency_proof`
  is the natural addition to `system-ledger`'s `LedgerConsumer` trait as an
  `advance_checkpoint` API (future MINOR after raw proof lands).
- **Stage-6 anchor stream:** `service-fs-anchor-emitter` (project-data cluster,
  `infrastructure/local-fs-anchoring/`) will need consistency proofs once
  multiple Rekor anchor checkpoints exist. Downstream dependency; not a blocker.

---

## 7. References

- RFC 9162 §2.1.4: https://datatracker.ietf.org/doc/html/rfc9162#section-2.1.4
- Pattern source: `system-core/src/inclusion_proof.rs`
- Composition source: `system-core/src/checkpoint.rs`
  (`verify_inclusion_proof` + `CheckpointInclusionError`)
- Master direction: ratified per Brief 1 (Option C), workspace v0.1.33-pending
