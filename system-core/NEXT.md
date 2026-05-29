# NEXT.md — system-core

> Last updated: 2026-05-29
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

- Nothing in progress.

## Queue

- **`Capability::canonical_bytes()`** — design for hash stability across serde-format
  swaps. v0.1.x uses serde JSON; canonical CBOR is the expected target per
  `worm-ledger-design.md` §3 D3. Future MINOR.

- **`IRQHandler` cap_type variant** — seL4 distinguishes IRQ handler capabilities
  from IRQ control. Add `CapabilityType::IrqHandler` distinct from `Irq`. Cross-check
  seL4 CDT taxonomy before landing.

## Blocked

- Nothing currently blocked.

## Deferred

- **`no_std` carve-out** — v0.1.x carries `std` for `Vec`, JSON serialization,
  base64, and ed25519-dalek (which can be no_std but feature-gated here).
  The kernel-consumption path needs `no_std` but only after the API surface stabilises.
  Candidate for v0.3.0 MINOR.

## Recently done

- 2026-05-27: v1.0.0 bump — API frozen. CHANGELOG.md created.
- 2026-05-20: Group 2A/2B test-gap closure — 4 new lib tests
  (`capability_hash_expiry_none_vs_some`, `capability_hash_changes_with_witness_pubkey`,
  `right_variants_round_trip`, `capability_type_variants_round_trip`) + 7 checkpoint.rs
  negative-path tests (ParseError variants, `VerifyError::BadPublicKey`,
  `consistency_proof_new_signature_invalid_rejects`). Total: 62 tests.
  Rustdoc added to all variants in CapabilityType + Right + Capability + WitnessRecord.
- 2026-05-20: Deleted `master-relay.rs` (residual sketch, defect per repo-layout rule).
- 2026-04-28: Phase 1A.5 — RFC 9162 §2.1.4 consistency proofs in `src/consistency_proof.rs`.
  Composed primitive `verify_consistency_proof` on `SignedCheckpoint`. system-core 0.1.4 → 0.2.0.
  51 tests total.
- 2026-04-27: Phase 1A.4 — RFC 9162 §2.1.3 inclusion proofs in `src/inclusion_proof.rs`.
  Composed primitive `verify_inclusion_proof` on `SignedCheckpoint`. system-core 0.1.3.
- 2026-04-27: Phase 1A.2 — C2SP signed-note checkpoint primitive in `src/checkpoint.rs`.
  Body parse/render, key-hash derivation, ed25519 verification, multi-sig apex handover predicate.
  system-core 0.1.2.
- 2026-04-26: Phase 1A.1 — Capability + WitnessRecord + LedgerAnchor types.
  Framework §9 activation. system-core 0.1.1.
