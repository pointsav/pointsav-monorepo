# system-ledger

<div align="center">

[ Leer este documento en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems (The Vendor)
**Taxonomy:** Tier-1 Core Component
**Version:** 0.2.1
**Status:** Active — Phase 1A structurally complete
**Cluster:** `cluster/project-system` per workspace `PROJECT-CLONES.md`

---

Kernel-side ledger-consultation state machine. Consumes the cryptographic
primitives in `system-core` to decide whether to honor a capability
invocation.

---

## I. What it provides

`system-ledger` is the substrate-tier consumer of the Capability Ledger
Substrate primitives defined in `system-core` (Doctrine claim #33). The
data types — `Capability`, `WitnessRecord`, `SignedCheckpoint`,
`LedgerAnchor`, `InclusionProof` — live in `system-core`. This crate
owns the state machine that composes them into a kernel-side verifier.

### Public surface

| Item | Location | Purpose |
|---|---|---|
| `LedgerConsumer` trait | `src/lib.rs` | Kernel-facing API for capability consultation and ledger mutation |
| `InMemoryLedger` | `src/lib.rs` | Concrete implementation for v0.2.x |
| `CheckpointCache` | `src/cache.rs` | LRU checkpoint cache; default capacity 64 |
| `RevocationSet` | `src/revocation.rs` | O(1) revoked-capability membership + audit sidecar |
| `ApexHistory` | `src/apex.rs` | Append-only apex history with post-handover invariant |
| `verify_witness_signature` | `src/witness.rs` | `ssh-keygen -Y verify` wrapper; namespace `capability-witness-v1` |
| `Verdict` / `RefuseReason` / `ConsultError` / `LedgerError` | `src/lib.rs` | Decision and error enumerations |

### What it does not contain

- Capability data primitives (`Capability`, `WitnessRecord`,
  `SignedCheckpoint`, `LedgerAnchor`, `InclusionProof`,
  `ConsistencyProof`) — those live in `system-core`.
- WORM tile storage — that is `service-fs` per `worm-ledger-design.md`.
- seL4 CDT integration — Phase 4+ when the substrate reaches bare-metal
  seL4.

---

## II. Status

Version 0.2.1. The v0.2.x scope is structurally complete:

- 44 unit and integration tests passing
- 10 criterion benchmarks in `benches/consult.rs`
- `cargo check -p system-ledger` and `cargo test -p system-ledger` are
  clean with zero warnings

The version 0.2.0 release introduced a breaking change to the
`LedgerConsumer` trait (see §V). The 0.2.1 patch added the Phase 1A.4
inclusion-proof benchmarks.

---

## III. The `LedgerConsumer` trait

The trait presents three methods to the kernel:

```rust
pub trait LedgerConsumer {
    fn consult_capability(
        &self,
        cap: &Capability,
        current_root: &SignedCheckpoint,
        now: u64,
        witness: Option<&WitnessRecord>,
    ) -> Result<Verdict, ConsultError>;

    fn apply_revocation(&mut self, event: RevocationEvent) -> Result<(), LedgerError>;

    fn apply_apex_handover(
        &mut self,
        old_apex_name: &str,
        old_apex_pubkey: &[u8; 32],
        new_apex_name: &str,
        new_apex_pubkey: &[u8; 32],
        handover_checkpoint: &SignedCheckpoint,
    ) -> Result<(), LedgerError>;

    fn apply_witness_record(
        &mut self,
        record: WitnessRecord,
        proof: InclusionProof,
    ) -> Result<(), LedgerError>;
}
```

`consult_capability` returns a `Verdict`:

- `Allow` — capability is current and unexpired.
- `Refuse(RefuseReason)` — invocation refused; reason is structured.
- `ExtendThenAllow { new_expiry_t }` — honor the invocation AND log the
  witness extension; the caller must append the witness record to the
  ledger before honoring.

The decision flow inside `consult_capability`:

1. Verify `current_root` against the apex history. If the checkpoint is
   not signed by the apex(es) valid at its tree height: `Refuse(StaleApex)`.
2. Check the revocation set. If revoked: `Refuse(Revoked)`.
3. Check expiry. If the capability is unexpired (or has no expiry):
   `Allow`.
4. Witness extension path. If no witness supplied: `Refuse(Expired)`. If
   the capability has no `witness_pubkey`: `Refuse(NotExtensible)`. If
   the witness record is not present in the current Merkle root:
   `Refuse(WitnessNotInLedger)`. If the SSH signature is invalid:
   `Refuse(WitnessSignatureInvalid)`. On all checks passing:
   `ExtendThenAllow`.

---

## IV. The §4 N+3+ apex-handover ceremony

`system-ledger` implements the ownership-transfer ceremony specified in
`conventions/system-substrate-doctrine.md` §4. The ceremony is a
four-height sequence in the ledger:

```
height N      previous apex P-old; single-sig checkpoints accepted
height N+1    P-old signs revocation entry (release to new owner)
height N+2    P-old AND P-new co-sign the handover checkpoint
              (C2SP signed-note multi-signature primitive)
height N+3+   only P-new accepted; P-old's signature REFUSED
```

The post-handover invariant — "P-old's signature on checkpoints at
heights N+3 and above is refused" — is enforced by `ApexHistory` and
`consult_capability` together. `ApexHistory.check_height(h)` returns:

- `ApexVerdict::Single { apex }` for standard heights (one valid apex)
- `ApexVerdict::Handover { old_apex, new_apex }` exactly at height N+2
  (both apexes valid simultaneously)
- `ApexVerdict::NoApex` before genesis

A checkpoint at height N+2 must carry both signatures
(`verify_apex_handover` predicate from `system-core`). After
`apply_apex_handover` records the transition, `check_height` at N+3
returns `Single { apex: P-new }` — a checkpoint signed only by P-old
at that height returns `Refuse(StaleApex)`.

The end-to-end test `full_handover_ceremony_end_to_end` in `src/lib.rs`
asserts all four phases of the ceremony: pre-handover P-old allows;
revocation entry by P-old applies; handover checkpoint with both
signatures accepts; post-handover P-new-only accepts; post-handover
P-old-only returns `Refuse(StaleApex)`.

The ceremony is atomic. It does not require state migration, downtime,
or vendor involvement. The new apex inherits all capability state, all
audit history, and all operational identity. The ledger records the
prior apex's tenure as immutable history from genesis to height N+1.

---

## V. `apply_witness_record` is inclusion-proof gated

Version 0.2.0 introduced a breaking change to the `LedgerConsumer`
trait: `apply_witness_record` now takes an `InclusionProof` parameter.
This is the Phase 1A.4 trait change (commit `2b9ca9c`).

The production path:

1. The caller supplies a Merkle inclusion proof demonstrating that the
   witness record's leaf hash (RFC 9162 §2.1, `SHA-256(0x00 || bytes)`)
   is present in the current ledger root.
2. `InMemoryLedger` calls `SignedCheckpoint::verify_inclusion_proof` from
   `system-core` to validate the proof against the current checkpoint's
   root hash and apex signature.
3. On failure: `Err(LedgerError::WitnessNotInRoot(_))`.
4. On success: the witness leaf hash is inserted into the `witnessed` set
   and subsequent `consult_capability` calls on the same record see it as
   logged.

The pre-0.2.0 path — trusting that the caller had already checked ledger
membership — is a test-only shortcut:

```rust
#[cfg(test)]
pub fn apply_witness_record_unchecked(&mut self, record: WitnessRecord) { ... }
```

Production code must use the proof-gated path.

`LedgerError` additions in 0.2.0:

| Variant | When |
|---|---|
| `NoCurrentCheckpoint` | `apply_witness_record` called with no current checkpoint set |
| `WitnessNotInRoot(_)` | Inclusion proof failed against current root |
| `NoApexForCheckpoint` | No apex recorded for the current checkpoint height |

---

## VI. Cache discipline

`CheckpointCache` is an LRU cache keyed by `(origin, tree_size)` and
`(origin, root_hash)`. Its role is to bypass the full ed25519 signature
verification on the hot path of `consult_capability`. Cache hits skip
the signature check entirely; cache misses fall through to the apex
verification step.

Phase 1A.3 benchmark (GCP n2-class hardware, release profile with
`opt-level=z`):

| Path | Median |
|---|---|
| Cache lookup hit (most-recent entry) | 8 ns |
| Cache lookup miss (64-entry full scan) | 338 ns |
| `SignedCheckpoint::verify_signer` (1-sig ed25519) | 3.40 ms |

The cache hit is approximately 420,000 times faster than one ed25519
verify. The cache is architecturally critical on the hot path.

Cache and inclusion proofs are complementary, not redundant. The cache
answers "has this checkpoint been seen and verified before?" The
inclusion proof answers "is this witness record committed to this
particular ledger root?" They operate on different data and serve
different purposes in the decision flow.

Phase 1A.4 benchmark (same hardware; VM under higher concurrent load
than the 1A.3 session — absolute numbers are 50–150% above the
equivalent 1A.3 values):

| Benchmark | Median |
|---|---|
| `InclusionProof::verify` (raw, 8-leaf tree, 3-hash path) | ~6 μs |
| `InclusionProof::verify` (raw, 1024-leaf tree, 10-hash path) | ~20 μs |
| `SignedCheckpoint::verify_inclusion_proof` (composed, 1024-leaf) | tracks ed25519 verify + 0.4% inclusion overhead |
| `apply_witness_record` (full path: proof verify + insert) | tracks composed verify |

A P1 BENCH-v0.2.0 report with controlled-load numbers will be issued
separately once the re-run is complete. The architectural shape —
verification dominated by ed25519 cost; inclusion overhead a rounding
error — is stable across load conditions.

---

## VII. Witness signature namespace

The namespace tag for capability witness signatures is
`capability-witness-v1`, bound to the `-n` flag of `ssh-keygen -Y sign
/ verify`.

The same `ssh-keygen` primitive is used in three places in this
workspace:

| Use | Namespace |
|---|---|
| Git commit signing | `git` |
| Apprenticeship verdict signing (per `apprenticeship-substrate.md` §5) | `apprenticeship-verdict-v1` |
| Capability witness records (this crate) | `capability-witness-v1` |

Cross-namespace replay — presenting a commit-signing or
apprenticeship-verdict signature as a capability witness — is rejected
by `ssh-keygen -Y verify` because the namespace field is part of the
signed data. This property is tested explicitly in `src/witness.rs`
(`verify_rejects_cross_namespace_signature`).

---

## VIII. Build and test

```
cargo build -p system-ledger
cargo test  -p system-ledger
cargo bench -p system-ledger   # criterion; release profile; output in target/criterion/
```

The crate requires `ssh-keygen` to be present on `PATH` for the witness
signature tests. On the Foundry workspace VM this is satisfied by the
OpenSSH installation.

---

## IX. Cross-references

- Sibling crate `../system-core/` — owns the data primitives this crate
  consumes.
- `DOCTRINE.md` §II claim #33 — The Capability Ledger
  Substrate (constitutional anchor).
- `conventions/system-substrate-doctrine.md` §3.1, §4, §5 —
  kernel binding, apex co-signing ownership transfer, Time-Bound
  Capabilities.
- `conventions/worm-ledger-design.md` §2 — four-layer stack;
  `system-ledger` is the substrate-tier L1+L2 consumer, parallel to
  `service-fs` at the application tier.
- `topic-merkle-proofs-as-substrate-primitive.md` (planned,
  `content-wiki-documentation`) — narrative background on RFC 9162
  inclusion and consistency proofs as used in this crate.

---

## X. Licensing

Inherits the monorepo `LICENSE` at the repository root.
