# CHANGELOG — system-core

One line per PATCH. MINOR bumps get a section header. See `~/Foundry/CLAUDE.md` §7 for versioning rules.

---

## 1.0.0 — 2026-05-27

- v1.0.0: API stable — Capability, WitnessRecord, LedgerAnchor, C2SP signed-note, RFC 9162 inclusion/consistency proofs; 62 tests; AGPL-3.0-or-later

## 0.2.x

- v0.2.0 (2026-05-20): Group 2 hygiene pass — 62 tests (11 negative-path additions in 2B), rustdoc on all public items, Cargo.toml metadata (license, description, repository, rust-version 1.73), ARCHITECTURE.md §5 extended, deleted master-relay.rs stub
- v0.1.21 (2026-04-28): Group 1 — inclusion_proof.rs + consistency_proof.rs (RFC 9162 §2.1.3 + §2.1.4); `checkpoint.rs` gains `verify_inclusion_proof` + `verify_consistency_proof` composed helpers; 51 tests; workspace member
- v0.1.4 (2026-04-26): Phase 1A increment 2 — C2SP signed-note checkpoint primitive; body parse/render; key-hash derivation; Ed25519 signature verification; multi-sig support including apex-handover predicate; 16 tests
- v0.1.1 (2026-04-26): Phase 1A increment 1 — Capability, WitnessRecord, LedgerAnchor, CapabilityType, Right defined per system-substrate-doctrine.md §3.1 + §5.1; framework §9 activation; workspace member registration; 6 tests
