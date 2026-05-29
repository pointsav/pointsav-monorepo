---
schema: foundry-draft-v1
artifact: JOURNAL-NOTES
journal: j2
journal_title: "Composing Trustworthy Systems from Verified Primitives"
state: draft-pending-editorial-review
originating_cluster: project-data
created: 2026-05-29
to: project-editorial
language_protocol: PROSE-TOPIC
bcsc_class: current-fact
research_trail:
  source_files:
    - service-fs/src/ledger.rs
    - service-fs/src/posix_tile.rs
    - service-fs/anchor-emitter/src/main.rs
    - service-fs/src/mcp.rs
  notes: >
    No formal benchmarks exist. Only integration tests (functional correctness).
    All implementation facts below have been verified against source code in this
    session. Criterion-based performance measurements are flagged as future work
    (see service-fs/NEXT.md).
---

# JOURNAL-NOTES — J2: Composing Trustworthy Systems from Verified Primitives

**Routing note:** These notes map the service-fs implementation to J2 §4
(Verified Primitive instance pattern). They are qualitative — no throughput
numbers or latency measurements exist yet. Route to project-editorial for
inclusion in the J2 submission context file once editorial has reviewed.

---

## J2 §4 mapping: service-fs as a Verified Primitive instance

J2's §4 "Verified Primitive" pattern describes a component that enforces
correctness invariants at its boundary such that composed systems inherit
the guarantees without re-implementing verification internally.
service-fs (`service-fs/`) in the PointSav Ring 1 architecture is a
direct implementation instance of this pattern.

### D4 atomic-write discipline

`service-fs/src/posix_tile.rs` enforces the D4 atomic-write sequence on
every ledger append:

1. Write candidate bytes to `<log_path>.tmp` (temporary file)
2. `fsync` the temp file (kernel durability guarantee before rename)
3. Atomic POSIX `rename` from `.tmp` to `<log_path>` (visibility is
   atomic; no reader ever sees a partial write)
4. `chmod 0o444` on the final log path (immutable post-write; no
   subsequent process can overwrite in place)

This sequence is enforced in code — callers cannot bypass it via the
`PosixTileLedger` API. The D4 guarantee is a primitive: composing
systems that write through service-fs inherit crash-safe append
semantics without implementing the sequence themselves.

### Linear SHA-256 hash chain

`service-fs/src/ledger.rs` maintains a linear SHA-256 hash chain over
all appended entries. Each entry's `this_hash` is computed as:

```
SHA-256(prev_hash || cursor || payload_id || payload_canonical_bytes)
```

The first entry's `prev_hash` is `SHA-256(CHAIN_ORIGIN)` where
`CHAIN_ORIGIN = b"service-fs:linear-chain:v1"` — a domain separator
that pins the chain origin and prevents cross-ledger collision attacks.
Tamper-evidence is structural: any modification to an entry
invalidates all subsequent hashes, detectable by recomputing the chain
from the origin.

The trait surface (`verify_inclusion`, `verify_consistency`) is
algorithm-agile: the linear chain is the v0.1.x baseline; an upgrade
to a Merkle tree would present the same interface, retaining
composability guarantees.

### Ed25519 checkpoint signing (C2SP signed-note format)

`service-fs/src/ledger.rs` defines the `Checkpoint` struct with a
`signature` field and an `algorithm` field (per worm-ledger-design.md
§3 D2–D3). Checkpoint signing uses the C2SP signed-note wire format
with Ed25519. The signing key is operator-supplied at deploy time
(`FS_SIGNING_KEY` environment variable).

A signed checkpoint is an operator-independent, verifiable declaration
of the chain's state at a given `tree_size`. Any party holding the
Ed25519 public key can verify the checkpoint without access to the
service-fs instance — the verification is self-contained.

### Monthly Rekor v2 hashedrekord anchoring

`service-fs/anchor-emitter/src/main.rs` implements a monthly oneshot
binary (`fs-anchor-emitter`) that:

1. Reads the current checkpoint from service-fs `/v1/checkpoint`
2. Wraps it in a `hashedRekordRequestV002` body (Sigstore Rekor v2
   entry format) with an ephemeral Ed25519 keypair generated per run
3. POSTs to `https://log2025-1.rekor.sigstore.dev/api/v2/log/entries`
   (Sigstore's 2025 shard; configurable via `REKOR_URL` env var)
4. POSTs the tlog entry returned by Rekor back to service-fs `/v1/append`
   (tlog writeback closes the loop — the ledger contains a record of
   its own external anchoring)

The Rekor log is a public, append-only, transparency log. Once a
checkpoint is published there, any third party can verify that the
service-fs chain existed at the recorded `tree_size` at the recorded
timestamp, independently of the operator.

### Per-tenant module-ID boundary enforcement

`service-fs/anchor-emitter/src/main.rs` uses `X-Foundry-Module-ID`
header (`FS_MODULE_ID` environment variable) on all HTTP requests to
service-fs. The service-fs HTTP layer enforces this header: every
append and read operation is scoped to the module's ledger namespace.

This is an isolation primitive: a composing service cannot read or
write another tenant's ledger entries by accident — the boundary is
enforced at the HTTP layer, not by convention.

---

## What is not yet available

The following measurements do not yet exist and should be flagged in
the J2 submission as future work:

- **Append throughput** (entries/second, bytes/second) under sustained
  load — no criterion benchmark exists.
- **Checkpoint latency** (time to produce and sign a checkpoint over
  N entries) — no criterion benchmark exists.
- **Rekor round-trip time** (time from `fs-anchor-emitter` invocation
  to tlog writeback confirmed) — no criterion or timing harness exists.
  Manual observation only (not recorded).

These measurements are planned as `criterion`-based benchmarks in a
future session. See `service-fs/NEXT.md` (Queue section) for the
open item.

---

## J5 dependency note

J5 (session isolation measurements) is on HOLD pending J2 submission
per the relay message. service-data session isolation measurements
are not available from this cluster.

---
