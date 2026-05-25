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
authored_by: task-project-system (session 181c94d9ca0491c5, ps-administrator identity)
authored_with: opus-4-7
references:
  - https://datatracker.ietf.org/doc/html/rfc9162
  - https://datatracker.ietf.org/doc/html/rfc9162#section-2.1.3
  - https://datatracker.ietf.org/doc/html/rfc9162#section-2.1.4
  - clones/project-system/pointsav-monorepo/system-core/src/inclusion_proof.rs
  - clones/project-system/pointsav-monorepo/system-core/src/consistency_proof.rs
  - clones/project-system/pointsav-monorepo/system-core/src/checkpoint.rs
  - clones/project-system/pointsav-monorepo/system-ledger/src/lib.rs
  - DOCTRINE.md claim #33 (The Capability Ledger Substrate)
  - DOCTRINE.md claim #34 (The Two-Bottoms Sovereign Substrate)
  - conventions/worm-ledger-design.md
notes_for_editor: |
  Tetrad backfill skeleton per project-tetrad-discipline.md §"Backfill from
  Triad" — section headings established here demonstrate intent; substantive
  prose follows in milestone N+1.

  Subject grounding: cluster/project-system shipped Phase 1A.4 (RFC 9162
  §2.1.3 inclusion proofs in commit 9b5e4fd) and Phase 1A.5 (RFC 9162 §2.1.4
  consistency proofs in commit 82b659f). Together with the C2SP signed-note
  checkpoint primitive (commit cdbed97 Phase 1A.2) these form the
  cryptographic floor of the Capability Ledger Substrate (Doctrine claim
  #33).

  Audience: vendor-public TOPIC. A financially literate reader without deep
  cryptography background should be able to follow the structure: what
  Merkle proofs are, why two flavours exist (inclusion vs consistency), how
  Foundry composes them on top of signed checkpoints to gate ledger
  apply-side validity, and how this composition makes substrate-level
  guarantees inheritable per Doctrine claim #34.

  Tone: Bloomberg-standard explainer. Algorithm walkthroughs use the RFC's
  variable names directly; cite §-numbers verbatim. Avoid "blockchain"
  framings — Merkle proofs predate blockchain by decades and the Foundry
  use is per RFC 9162 (Certificate Transparency lineage), not consensus-
  log lineage.

  Banned-vocab + BCSC discipline + bilingual generation: project-language
  enforces. Don't pre-pare; leave technical depth in.
---

# Merkle Proofs as a Substrate Primitive

*(draft-pending — substance follows in milestone N+1)*

## 1. What Merkle proofs are

*(draft-pending — substance follows in milestone N+1)*

Section will cover: hash trees as a data structure for committing to a
sequence; the leaf-and-internal-node construction (0x00 vs 0x01 prefix
domain separation per RFC 9162 §2.1.1); why proofs are logarithmic in
the tree size; the difference between *committing to* a sequence (the
root hash) and *proving membership / extension* of that sequence (the
proof path); the relationship to the underlying hash function's
collision resistance.

## 2. Two flavours: inclusion and consistency

*(draft-pending — substance follows in milestone N+1)*

Section will cover: inclusion proofs (RFC 9162 §2.1.3) prove a single
leaf is present in a tree of a given size; consistency proofs (RFC 9162
§2.1.4) prove that a tree of size N+k is a valid extension of a tree of
size N. The two flavours answer different questions: "is this leaf in
the log?" vs "is this newer log a continuation of the older one?"
Different consumers need different proofs; the same hash tree supports
both.

## 3. Inclusion proofs in `system-core`

*(draft-pending — substance follows in milestone N+1)*

Section will cover: the `InclusionProof` struct shape; the `verify`
method's tree-walking algorithm; the `rfc9162_leaf_hash` (0x00 prefix)
and `rfc9162_internal_hash` (0x01 prefix) helpers; the test coverage
(11 tests in `inclusion_proof.rs`) including the full grid 1..=8
verification; the algorithm's logarithmic cost (~10-20 microseconds for
typical tree sizes per Phase 1A.4 benchmarks).

## 4. Consistency proofs in `system-core`

*(draft-pending — substance follows in milestone N+1)*

Section will cover: the `ConsistencyProof` struct shape; the `verify`
method's algorithm seeding both accumulators from `hashes[0]` and
tracking `fn` / `sn` counters until both reach zero; the 9 error
variants distinguishing structural from cryptographic failures; the
edge cases (`old_size == 0` per RFC 9162 vs RFC 6962, `old_size ==
new_size` requiring empty proof, power-of-two boundary handling); the
test coverage (11 tests in `consistency_proof.rs`).

## 5. Composed primitives on `SignedCheckpoint`

*(draft-pending — substance follows in milestone N+1)*

Section will cover: the C2SP signed-note checkpoint format
(`Checkpoint` body + `NoteSignature` lines per RFC 9162); the
composition pattern where `SignedCheckpoint::verify_inclusion_proof`
and `verify_consistency_proof` chain tree-size invariants → signature
verification → raw-proof verification in a single call; why this
composition is the kernel-facing API surface (raw `InclusionProof` and
`ConsistencyProof` are deliberately not advertised — they are
building blocks); the `CheckpointInclusionError` and
`CheckpointConsistencyError` taxonomies that distinguish "this
checkpoint is malformed" from "this proof is invalid" from "the
signer's public key is wrong".

## 6. Consumer integration in `system-ledger`

*(draft-pending — substance follows in milestone N+1)*

Section will cover: the `LedgerConsumer` trait; how
`apply_witness_record` was promoted in Phase 1A.4 from a "trust the
caller" shortcut to an inclusion-proof-gated apply (commit 2b9ca9c —
the v0.1.x → v0.2.0 trait-signature change); how the kernel-side
consult-capability hot path stays microsecond-fast via the
`CheckpointCache` while the apply-side gains write-validity
guarantees from Merkle inclusion proofs; the cache-hit vs cache-miss
characteristics under load (per Phase 1A.4 criterion benchmarks);
the apex-handover ceremony's interaction with inclusion-proof verify
at handover height (any valid signer suffices for inclusion;
strict-handover policies layer on top via `verify_apex_handover`).

## 7. Why this matters as a substrate primitive

*(draft-pending — substance follows in milestone N+1)*

Section will cover: the Capability Ledger Substrate (Doctrine claim
#33) requires a customer-rooted Merkle log to make capability state
auditable; the Two-Bottoms Sovereign Substrate (Doctrine claim #34)
requires that deployment-as-cryptographic-artefact be inheritable
across native (seL4) and compat (NetBSD) substrates without
re-trusting the runtime; Merkle inclusion + consistency proofs are
the cryptographic primitives that make both possible without
requiring consumer good-behaviour. The primitive runs in `no_std`-
compatible Rust today (kernel-consumption-eligible) and is reused
across `system-core` consumers without per-consumer reimplementation.

## 8. Cross-references

*(draft-pending — substance follows in milestone N+1)*

Section will cite: RFC 9162 (Certificate Transparency 2.0); the
C2SP signed-note checkpoint specification; the `worm-ledger-design.md`
convention; Doctrine claim #33 (The Capability Ledger Substrate) and
claim #34 (The Two-Bottoms Sovereign Substrate); the cluster-wiki-
draft-pipeline that brings this TOPIC into being.
