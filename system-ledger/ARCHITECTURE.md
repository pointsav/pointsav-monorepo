# ARCHITECTURE — system-ledger

Part of `cluster/project-system` per workspace `PROJECT-CLONES.md`.

Implements the kernel-side state machine for Doctrine claim #33
(The Capability Ledger Substrate) per
`~/Foundry/conventions/system-substrate-doctrine.md` §3.1 (kernel
binding) + §4 (apex co-signing ownership transfer) + §5 (Mechanism
A — Time-Bound Capabilities).

Created 2026-04-27 per Master Claude directive (Option B
resolution: state machine in dedicated crate, not extension of
`system-substrate`).

---

## 1. Scope

`system-ledger` is the **state-machine consumer** of the
data-primitive types in `system-core`. The split is:

| Crate | Owns | Tier |
|---|---|---|
| `system-core` | `Capability`, `WitnessRecord`, `LedgerAnchor`, C2SP signed-note format (`Checkpoint`, `NoteSignature`, `SignedCheckpoint`), enums, hash function | data-primitive |
| `system-ledger` | Checkpoint cache, revoked-capability set, apex history + post-handover invariant, witness-record signature verification, `LedgerConsumer` trait | state-machine consumer |

This mirrors `worm-ledger-design.md` §2's four-layer stack at the
substrate tier: `system-core` is the L0 primitives, `system-ledger`
is the substrate-tier L1+L2 consumer (parallel to `service-fs` at
the application tier — same format, different consumers, different
layers).

## 2. Module layout

### `cache.rs` — recent-N checkpoint cache

LRU bounded cache keyed by `(origin, tree_size)` and `(origin,
root_hash)`. Cache hits avoid the full ed25519 verification cost;
cache misses fall through to userland verifier per convention §3.1.

Capacity default 64; revisit when criterion benchmarks land per
task #21.

### `revocation.rs` — revoked-capability set

`HashSet<Hash256>` for O(1) membership; sidecar `HashMap<Hash256,
RevocationEvent>` for audit detail (timestamp, signed-by,
ledger-height). Per convention §3.1: kernel consults this BEFORE
honoring an invocation.

`apply_revocation(event)` is idempotent — replay of an
already-revoked capability is a no-op.

### `apex.rs` — apex history + post-handover invariant

Append-only `Vec<ApexEntry>`. The current apex is the most recent
entry with `effective_until = None`. Apex handover (convention §4)
closes the prior entry's `effective_until` to the handover height
and appends the new entry as `effective_from = handover_height + 1`.

Post-handover invariant: signatures from a retired apex MUST be
refused on checkpoints at or above its `effective_until + 1`. The
verifier function `verify_checkpoint_against_apex(checkpoint,
height)` enforces this — the heart of the §4 N+3+ rule.

### `witness.rs` — ssh-keygen -Y verify wrapper

Witness records use the same SSH-signing primitive as commit
signing (per `~/Foundry/CLAUDE.md` §3) and apprenticeship verdicts
(per `apprenticeship-substrate.md` §5). The `-n` namespace tag is
**`capability-witness-v1`** to prevent cross-namespace replay.

Implementation strategy: `std::process::Command` driving
`ssh-keygen -Y verify` (or `tokio::process::Command` if the
consumer is async). Pubkey load uses `allowed_signers` format —
same primitive as `~/Foundry/identity/allowed_signers`.

### `lib.rs` — LedgerConsumer trait + InMemoryLedger

Public trait:

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
        record: &WitnessRecord,
        proof: InclusionProof,
        apex_name: &str,
        apex_pubkey: &[u8; 32],
    ) -> Result<(), LedgerError>;
}
```

`Verdict`: `Allow` / `Refuse(RefuseReason)` / `ExtendThenAllow {
new_expiry_t }`. The third variant signals "honor invocation AND
log the witness extension into the ledger" — the caller is
responsible for appending the witness record before honoring.

Concrete impl: `InMemoryLedger { cache, revocations, apex }` for
v0.1.x. The trait keeps the door open for `MoonshotDatabaseLedger`
once `moonshot-database` ships (per `worm-ledger-design.md` §3 D7
dual-target pattern).

## 3. Decision flow inside `consult_capability`

```
1. Verify current_root against the apex history:
   - apex.verify_checkpoint(current_root) → ApexValid | StaleApex
   - If StaleApex: return Refuse(StaleApex)
2. Check revocation set:
   - revocations.contains(&cap.hash()) → return Refuse(Revoked)
3. Check expiry:
   - If cap.expiry_t.is_none() OR now < expiry_t: return Allow
4. Capability past expiry:
   - If witness.is_none(): return Refuse(Expired)
   - If cap.witness_pubkey.is_none(): return Refuse(NotExtensible)
   - Verify witness.signature against cap.witness_pubkey
     (witness::verify_witness_signature):
       - Sig invalid: return Refuse(WitnessSignatureInvalid)
   - Verify witness's hash is in current_root's Merkle proof:
       - Not in ledger: return Refuse(WitnessNotInLedger)
   - All OK: return ExtendThenAllow { new_expiry_t: witness.new_expiry_t }
```

The Merkle inclusion proof check is enforced via `apply_witness_record`
(Phase 1A.4 / v0.2.0): the caller MUST supply an `InclusionProof` when
committing a witness record. `SignedCheckpoint::verify_inclusion_proof`
(from `system-core`) validates signature + Merkle path atomically before
the record is inserted into `witnessed`. No trust shortcut remains.

## 4. Cross-references

- `~/Foundry/DOCTRINE.md` §II claim #33 (constitutional anchor)
- `~/Foundry/conventions/system-substrate-doctrine.md` §3.1, §4, §5
- `~/Foundry/conventions/worm-ledger-design.md` §2 (four-layer
  stack — system-ledger is the substrate-tier L1+L2 consumer
  parallel to `service-fs`'s application-tier)
- `~/Foundry/CLAUDE.md` §3 (`allowed_signers`; ssh-keygen -Y verify)
- `~/Foundry/conventions/apprenticeship-substrate.md` §5 (same
  ssh-keygen primitive used here, different namespace)
- **Sibling crate `system-core`** — owns the data primitives this
  crate consumes. Decision rationale in
  `../system-core/ARCHITECTURE.md` §3.

## 5. Verification

**44 tests passing** on `cargo test -p system-ledger` (Rust stable),
zero warnings. **10 criterion benchmarks** in `benches/consult.rs`.

| Module | Tests |
|---|---|
| `cache.rs` | 7 (insert, lookup by tree_size + root_hash, eviction, miss) |
| `revocation.rs` | 5 (apply, contains, idempotent replay) |
| `apex.rs` | 10 (handover ceremony, post-handover invariant, StaleApex) |
| `witness.rs` | 5 (ssh-keygen happy path, sig failure, namespace cross-replay rejection) |
| `lib.rs` | 17 (consult, revoke, expiry, witness path, full N+3+ ceremony end-to-end) |

Key benchmark results (Intel Xeon 2.20 GHz; full table in `BENCHMARKS.md`):

| Benchmark | Mean |
|---|---|
| cache hit (most-recent) | 11.2 ns |
| cache miss (full 64-entry scan) | 362 ns |
| `verify_signer` (1-sig ed25519) | 4.01 ms |
| `consult_capability` (Allow path) | 3.74 ms |
| `apply_witness_record` (full path) | 3.71 ms |
| `InclusionProof::verify` (raw, 1024-leaf) | 17.74 µs |

Cache hit is ~358,000× faster than ed25519 verify — architecturally
critical for the kernel-side invocation hot path.
