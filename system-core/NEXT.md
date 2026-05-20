# NEXT.md — system-core

> Last updated: 2026-05-20
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

- Nothing in progress. Group 2B test-gap items are queued below.

## Queue

- **Group 2B — test gap closure (4 items):**
  - Add negative-path tests in `lib.rs`: `expiry_t: None` vs `Some` hash sensitivity;
    `witness_pubkey` change changes hash; `Right` and `CapabilityType` round-trip exhaustion.
    Target: 4 additional tests.
  - Add `ParseError` variant tests in `checkpoint.rs` (one test per variant:
    `NotUtf8`, `Truncated`, `MissingNewline`, `BadRootHashLength`, `MissingSignatureSeparator`).
  - Add `VerifyError::BadPublicKey` explicit test (pass malformed pubkey to `verify_signer`).
  - Add `verify_consistency_proof` `NewSignatureInvalid` coverage (old sig valid, new sig invalid).

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
