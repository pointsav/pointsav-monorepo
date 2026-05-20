---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-capability-ledger-substrate.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-20T00:00:00Z
authored_by: task-project-system (session 37ac0f6b)
authored_with: claude-sonnet-4-6
references:
  - DOCTRINE.md claim #33 (The Capability Ledger Substrate)
  - DOCTRINE.md claim #34 (The Two-Bottoms Sovereign Substrate)
  - conventions/system-substrate-doctrine.md
  - conventions/worm-ledger-design.md
  - clones/project-system/system-core/src/lib.rs
  - clones/project-system/system-ledger/src/lib.rs
  - clones/project-system/system-ledger/src/cache.rs
  - clones/project-system/system-ledger/src/apex.rs
  - clones/project-system/system-ledger/src/revocation.rs
  - clones/project-system/system-ledger/src/witness.rs
  - BENCH-v0.2.0.md
notes_for_editor: |
  This TOPIC is the primary "what is it" explainer for Doctrine claim #33.
  The Merkle proofs TOPIC (topic-merkle-proofs-as-substrate-primitive.md)
  is the companion technical deep-dive; this TOPIC should link to it rather
  than repeat the proof mechanics.

  Audience: financially literate reader. Explain what a kernel capability is
  (seL4 mental model) before explaining the ledger extension. The claim #33
  innovation is the COMPOSITION — existing primitives (seL4 caps, Merkle logs,
  SSH signing) wired together so kernel-mediated access control is
  cryptographically auditable by the customer.

  Banned-vocab + BCSC discipline + bilingual generation: project-language enforces.
  Anti-recycling discipline: be specific about what seL4 capabilities are vs.
  what the Capability Ledger adds. Do not conflate.

  Tone: Bloomberg-standard explainer. Not marketing. "Honest We Own It" posture
  from conventions/system-substrate-doctrine.md §8.
---

# The Capability Ledger Substrate

The Capability Ledger Substrate is the mechanism by which every access-control
decision in a Foundry deployment becomes a cryptographically auditable event
anchored to a log the customer controls. It extends the seL4 microkernel's
native capability model — which is correct by formal proof — with a
transparency layer that makes the audit record portable, customer-rooted, and
verifiable by third parties without any trust relationship with the operator.

This is Doctrine claim #33.


## 1. What the Capability Ledger Substrate is

An operating system kernel mediates access to hardware resources. When a
process wants to write to a memory region, open a file, or send a network
packet, the kernel decides whether the process is permitted. This decision is
what "access control" means at the system layer.

The seL4 microkernel makes access-control decisions using *capabilities* —
unforgeable tokens that encode exactly what resource a process holds and what
operations it may perform on that resource. A process that does not hold a
valid capability for a resource cannot access it; there is no override, no
ambient authority, no root user that bypasses the check. The seL4 capability
model has been formally verified: the kernel's C implementation is proven
correct against a mathematical specification. This is the foundation.

The Capability Ledger Substrate does not replace this foundation. It adds one
new property: every capability invocation decision can be linked, via a Merkle
inclusion proof, to a checkpoint in a signed transparency log. The customer
holds the signing keys for that log. The customer can audit the full history.
Third parties can verify individual entries against published checkpoints
without access to the full log.

The result is a security substrate with two independently verifiable layers:
the kernel layer (seL4 formal proof — the kernel cannot be misled into
honoring a capability it should refuse) and the ledger layer (Merkle audit
trail — the history of capability state changes cannot be rewritten without
the customer's apex keys). The combination is what Doctrine claim #33 names
as the leapfrog: neither layer alone provides both properties.


## 2. The `Capability` type — fields, kernel binding, and the ledger anchor

In `system-core`, the `Capability` struct is the ledger-bound authorisation
token:

```rust
pub struct Capability {
    pub cap_type: CapabilityType,       // Endpoint, Memory, Irq, Notification, CNode
    pub rights: Vec<Right>,             // Read, Write, Invoke, Grant, Revoke
    pub expiry_t: Option<u64>,          // Unix seconds; None = no built-in expiry
    pub witness_pubkey: Option<String>, // SSH public key for expiry extension
    pub ledger_anchor: LedgerAnchor,    // Points into the customer Merkle log
}
```

The `cap_type` and `rights` fields map directly to seL4 CDT (Capability
Derivation Tree) semantics: an `Endpoint` capability with `[Invoke]` rights
is the seL4 object that lets one protection domain send a message to another.
A `Memory` capability with `[Read, Write]` is an IPC-shared memory region.
These are the kernel's native vocabulary; the Capability Ledger Substrate
adopts them without modification.

The `ledger_anchor` field is the new binding. It identifies the C2SP
signed-note checkpoint in the customer transparency log at which this
capability was committed:

```rust
pub struct LedgerAnchor {
    pub origin: String,      // e.g. "foundry.<module-id>.capability-ledger"
    pub tree_size: u64,      // Log size at commitment time
    pub root_hash: Hash256,  // Merkle root at that size
}
```

A capability that carries a `ledger_anchor` pointing to tree position 1,247
in a log with a known root can be verified: produce an inclusion proof that
the capability's SHA-256 hash is at leaf 1,247 in the tree, and verify that
proof against the signed checkpoint at tree_size = 1,247. If the proof
verifies, the capability's existence in the log is established. If the log
has a published checkpoint history, the anchor ties the capability to a
specific point in that auditable history.

The `Capability::hash()` method computes the SHA-256 of the JSON-serialized
struct. This is the value used as the leaf in the Merkle tree and as the key
in the revocation set. Determinism is tested: the same struct always produces
the same hash; changing any field — including `expiry_t` or the anchor's
`tree_size` — produces a different hash.


## 3. Time-Bound Capabilities (Mechanism A)

A capability with a non-None `expiry_t` field may not be invoked after the
Unix timestamp `t` without an extension. This is Mechanism A, Time-Bound
Capabilities per `system-substrate-doctrine.md` §5.

The extension mechanism requires two parties: the capability itself names a
`witness_pubkey` (an SSH ed25519 public key), and the holder of that key signs
a `WitnessRecord`:

```rust
pub struct WitnessRecord {
    pub capability_hash: Hash256, // identifies which capability is extended
    pub new_expiry_t: u64,        // must be greater than the previous expiry_t
    pub signature: Vec<u8>,       // ssh-keygen -Y sign, namespace "capability-witness-v1"
}
```

The namespace tag `capability-witness-v1` prevents cross-namespace replay.
An ed25519 signature produced by `ssh-keygen -Y sign` over a commit message
or an apprenticeship verdict cannot be replayed as a capability witness
extension, because the namespace tags differ. The `system-ledger`
`witness.rs` module verifies signatures by shelling out to `ssh-keygen -Y verify`
with the correct namespace.

The kernel decision flow for a time-bound capability is:
1. If `now < expiry_t`: honor the invocation (no witness needed)
2. If `now >= expiry_t` and no witness is supplied: `Refuse(Expired)`
3. If `now >= expiry_t` and a witness is supplied:
   - Verify the witness signature against the capability's `witness_pubkey`
   - Verify a Merkle inclusion proof that the witness record's hash is in the current log
   - If both pass: `ExtendThenAllow { new_expiry_t }` — honor the invocation and update the ledger
   - If either fails: `Refuse(WitnessSignatureInvalid)` or `Refuse(WitnessNotInLedger)`

The inclusion-proof requirement on the witness record is the key property: a
witness extension cannot be honored until it has been committed to the
customer's transparency log and an apex-signed checkpoint covers it. The
customer's apex sign-off is a prerequisite for capability lifetime extension.
This cannot be forged without the customer's ed25519 apex keys.


## 4. The apex-ownership ceremony (N+3+ handover)

The customer's apex keys are the root of trust for the entire capability
ledger. A transparency log is only as trustworthy as the process for
establishing and rotating those keys. The Capability Ledger Substrate
specifies a formal ownership-transfer ceremony that produces an auditable
handover record in the same log it secures.

The ceremony proceeds at checkpoints N, N+1, N+2, N+3+:

| Height | Action | Required signatures |
|---|---|---|
| N | Final checkpoint under P-old authority | P-old only |
| N+1 | Revocation entry: P-old is revoked | P-old (signing its own revocation) |
| N+2 | Handover checkpoint — co-signed by both apexes | P-old AND P-new (both required) |
| N+3+ | Post-handover checkpoints | P-new only; P-old refused with `StaleApex` |

The C2SP signed-note format directly supports multi-signature: the same
checkpoint body (origin + tree_size + root_hash) can carry multiple signature
lines, each from a different named key. The `SignedCheckpoint::verify_apex_handover`
composed primitive checks both signatures on a handover checkpoint.

The `system-substrate-doctrine.md` §4 N+3+ invariant is: any capability
invocation presenting a checkpoint signed only by P-old at height N+3 or
later is refused with `Refuse(StaleApex)`. The `ApexHistory` module in
`system-ledger` tracks the effective lifetime of each apex (`effective_from`
and `effective_until` heights) and enforces this invariant at consult time.

The ceremony has three properties that matter to a customer:
1. **Atomicity**: the handover is a single, self-contained event in the log.
   There is no out-of-band state migration. The log records the full ceremony.
2. **Auditability**: any third party examining the log can identify the exact
   checkpoint where P-old's authority ended and P-new's authority began.
3. **Finality**: once the N+2 handover checkpoint is published, P-old cannot
   produce a valid checkpoint for N+3+ that the kernel will accept. The key
   rotation is permanent absent a further ceremony.

An end-to-end test in `system-ledger` (`full_handover_ceremony_end_to_end`)
verifies all four stages: pre-handover P-old allows; revocation applies;
handover with both signatures accepts; post-handover P-old-only is refused
with `StaleApex`.


## 5. The `LedgerConsumer` state machine — consult flow and Verdict types

The `LedgerConsumer` trait in `system-ledger` is the kernel-facing interface:

```rust
pub trait LedgerConsumer {
    fn consult_capability(
        &self, cap: &Capability, current_root: &SignedCheckpoint,
        now: u64, witness: Option<&WitnessRecord>
    ) -> Result<Verdict, ConsultError>;

    fn apply_revocation(&mut self, event: RevocationEvent) -> Result<(), LedgerError>;
    fn apply_apex_handover(&mut self, ...) -> Result<(), LedgerError>;
    fn apply_witness_record(&mut self, record: WitnessRecord, proof: InclusionProof)
        -> Result<(), LedgerError>;
}
```

`consult_capability` is the read-side hot path. It returns one of three verdicts:

| Verdict | Meaning | Kernel action |
|---|---|---|
| `Allow` | Capability is current and unexpired | Honor the invocation |
| `Refuse(reason)` | Capability invalid; reason provided | Deny the invocation |
| `ExtendThenAllow { new_expiry_t }` | Witness extension accepted | Extend + honor |

The consultation decision flow:
1. **Apex validity check**: is the current_root signed by the recognized apex? If the checkpoint is not apex-signed, all bets are off — `Refuse(ApexInvalid)`.
2. **Post-handover invariant check**: if an apex handover has occurred, is this checkpoint from a refused (stale) apex? If so, `Refuse(StaleApex)`.
3. **Revocation check**: is the capability's hash in the revocation set? If so, `Refuse(Revoked)`.
4. **Expiry check**: is `now < expiry_t` (or is `expiry_t` None)? If so, `Allow`.
5. **Witness path**: if expired, attempt the witness extension flow (§3 above).

The three write-side methods (`apply_revocation`, `apply_apex_handover`,
`apply_witness_record`) update ledger state that subsequent consultations
read. They are separated from the read-side precisely because the kernel's
read/write access patterns differ: `consult_capability` is called on every
invocation; the write methods are called much less frequently (revocation
events and apex handovers are rare; witness extensions happen at capability
expiry boundaries, which may be hours or days apart).


## 6. Cache discipline — why the 358,000× speedup is architecturally critical

The `consult_capability` decision flow requires verifying that the current
checkpoint is apex-signed. An ed25519 signature verification takes
approximately 4 milliseconds on the Intel Xeon 2.20 GHz hardware where
the substrate is developed. Any workload calling `consult_capability` hundreds
of times per second would spend most of its time doing signature verification
— an unacceptable overhead for kernel-mediated access control.

The `CheckpointCache` in `system-ledger` resolves this. It holds the most
recent N (default 64) verified checkpoints, keyed by tree_size. A cache hit
— looking up a checkpoint the ledger has already verified — costs 11.2
nanoseconds. The cache stores the verified `SignedCheckpoint` objects; a
lookup confirms the checkpoint is present and returns it without re-running
ed25519.

| Operation | Time | Ratio |
|---|---|---|
| Cache hit (most-recent lookup) | 11.2 ns | 1× (baseline) |
| Cache miss (full 64-entry scan) | 362 ns | ~32× |
| `verify_signer` (ed25519, 1 apex) | 4.01 ms | ~358,000× |
| `consult_capability` (full path, cache miss) | 3.74 ms | ~334,000× |

In steady-state operation, the kernel publishes checkpoints infrequently
(each checkpoint commits a batch of capability state changes). Between
checkpoint publications, every capability invocation hits the cache. The
cache hit rate approaches 100% for any sustained workload. The ed25519
verifier runs only when a new checkpoint is published — which happens on the
write path, not the read path.

The 64-entry bound covers: the apex-handover window (during which P-old and
P-new checkpoints coexist at heights N+1 and N+2), the overlap period when
multiple system components hold references to slightly different recent
checkpoints, and reasonable checkpoint publishing rates without exceeding
kernel working-set constraints. The bound is a configuration choice, not a
protocol requirement.

The architectural lesson is that the cache and the Merkle inclusion proofs
are not alternatives — they answer different questions on different access
paths. The cache makes the read path fast. The inclusion proofs make the
write path trustworthy. Both are required for a production substrate.


## 7. Revocation and post-handover invariants

**Revocation** is the mechanism for permanently revoking a capability's
authority. A `RevocationEvent` carries the capability's SHA-256 hash and
a revocation record. After `apply_revocation` is called, `consult_capability`
returns `Refuse(Revoked)` for that capability hash regardless of expiry or
witness state. The revocation set is a `HashSet<Hash256>` in the in-memory
implementation — O(1) membership check.

Revocation events are themselves log entries, anchored to the same
customer-signed transparency log as capability commitments. An auditor
examining the log sees the full sequence: capability committed at height N₁,
revoked at height N₂, and no subsequent witness extension is valid after N₂
because any such extension would need to reference a checkpoint at N₂ or
later, where the revocation entry is visible.

The **post-handover invariant** is a different property: it governs which
apex signing key is authoritative on a given checkpoint, independent of
individual capability revocation. Per `system-substrate-doctrine.md` §4,
once the apex handover ceremony completes at height N+2, any invocation
presenting a checkpoint signed only by P-old at height N+3 or later is
refused. This prevents P-old from re-asserting authority after transferring
it — the log records the transfer, and the kernel enforces the cutoff height.

The two properties operate at different levels of the substrate:
- Revocation is per-capability (a specific token is invalid)
- Post-handover is per-epoch (the old apex key is invalid for an entire
  period of time)

Both are enforced by the `InMemoryLedger` in `system-ledger`. Both are tested
in the `full_handover_ceremony_end_to_end` integration test.


## 8. Relationship to the WORM ledger

The WORM (Write-Once Read-Many) ledger substrate, specified in
`conventions/worm-ledger-design.md`, is the foundational record-storage layer
of the Foundry architecture. It implements a C2SP tlog-tiles compatible
transparency log: append-only, content-addressed, cryptographically signed
by an apex. Service-level consumers (notably `service-fs`, the Ring 1 WORM
backend) interact with it at the application tier.

The Capability Ledger Substrate is the substrate-tier consumer of the same
log format. The data-primitive types (`Capability`, `WitnessRecord`,
`LedgerAnchor`, `SignedCheckpoint`, `InclusionProof`, `ConsistencyProof`)
defined in `system-core` are the L0 schema layer. The state machine in
`system-ledger` is the L1+L2 consumer.

The parallel with `service-fs` is structural and deliberate:
- `service-fs` is the application-tier WORM consumer: Ring 1, userspace,
  network-accessible, human-scale record throughput
- `system-ledger` is the substrate-tier WORM consumer: kernel-adjacent,
  single-threaded, microsecond-scale read latency required

Both use the same C2SP signed-note format for checkpoints. Both verify
apex ed25519 signatures. Both accept capability-state changes (revocations,
witness records) that need Merkle inclusion proofs to be trusted. The
difference is the access pattern and the consequence of failure: in
`service-fs`, a slow verification delays a file operation; in
`system-ledger`, a slow verification delays a kernel-mediated capability
invocation, which is a system-level performance constraint.

This layer separation — same cryptographic primitives, different deployment
tiers, different performance envelopes — is the architectural pattern that
Doctrine claim #33 formalizes. A future system that replaces `service-fs`
or adds a second WORM consumer (e.g., for audit-ledger) does not need to
reimplement the proof mechanics; it shares `system-core` and composes the
same composed primitives at its own tier.


## 9. Cross-references

- **Doctrine claim #33 — The Capability Ledger Substrate**
  `~/Foundry/DOCTRINE.md` §II claim #33. The constitutional anchor for
  every capability authorization being anchored to a customer-rooted Merkle log.

- **Doctrine claim #34 — The Two-Bottoms Sovereign Substrate**
  `~/Foundry/DOCTRINE.md` §II claim #34. The Two-Bottoms composition — seL4
  native-bottom + NetBSD compat-bottom — shares the same capability ledger
  substrate. The audit trail travels with the capability regardless of which
  bottom it runs on.

- **`conventions/system-substrate-doctrine.md`**
  The 12-section operational specification. §3.1 (Capability type schema),
  §4 (N+3+ apex handover ceremony), §5.1 (WitnessRecord schema and Mechanism A),
  §6.1 (reproducible-verification artefact format), §8 (Honest We Own It scoresheet).

- **`conventions/worm-ledger-design.md`**
  The WORM ledger design. §3 D1 (C2SP tlog-tiles wire format), §3 D3 (SHA-256
  baseline). The Capability Ledger is a substrate-tier consumer of the same design.

- **`topic-merkle-proofs-as-substrate-primitive.md`**
  Companion technical TOPIC covering RFC 9162 inclusion and consistency proofs,
  the `InclusionProof` and `ConsistencyProof` structs, verification algorithms,
  and performance benchmarks. Read alongside this topic for the cryptographic
  grounding.

- **cluster/project-system — Phase 1A implementation state**
  - `system-core` v0.2.0: `Capability`, `WitnessRecord`, `LedgerAnchor`, `Checkpoint`,
    `NoteSignature`, `SignedCheckpoint`, `InclusionProof`, `ConsistencyProof`. 51 tests.
  - `system-ledger` v0.2.1: `LedgerConsumer` trait, `InMemoryLedger`, `CheckpointCache`,
    `RevocationSet`, `ApexHistory`, `verify_witness_signature`. 44 tests + 10
    criterion benchmarks.
