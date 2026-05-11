---
schema: foundry-doc-v1
document_version: 0.1.0
cluster: project-system
branch: cluster/project-system
produced: 2026-04-27
crate_versions: { system-core: "0.2.0", system-ledger: "0.2.1" }
---

# Stage-6 Promotion Readiness Checklist
## system-core 0.2.0 → 1.0.0  |  system-ledger 0.2.1 → 1.0.0

Produced from Task-tier code review (claude-sonnet-4-6).
All items fact-checked against the actual source, benches, and docs.
Read-only on the monorepo; this file does not modify any crate.

---

## 1. What v1.0.0 means for these crates

Per `CLAUDE.md` §7 ("first declared-stable"), v1.0.0 commits the crate to an API contract that applies for the life of the MAJOR version: every `pub` item is deliberately public and will not be removed or signature-changed without a MAJOR bump. For substrate crates consumed by `system-*` and `moonshot-*` crates, this is consequential — downstream crates that pin `system-core = "1"` or `system-ledger = "1"` will not receive breaking changes.

Concretely, v1.0.0 requires: (a) rustdoc on every `pub` item sufficient for an external consumer to use it without reading source; (b) bench numbers published as performance characteristics so users can size the kernel-side consultation budget; (c) Cargo.toml metadata complete enough for crates.io publication; (d) no `*` or `>=` version constraints that would defeat the semver contract; and (e) the CLAUDE.md and NEXT.md housekeeping files reflecting the v1.0.0 surface so future sessions start from a clean state.

---

## 2. system-core 0.2.0 → 1.0.0 checklist

### 2a. Test coverage

Test count at v0.2.0: **51 tests** across 4 modules.

| Module | LOC (approx) | Tests | Tests/100 LOC | Assessment |
|---|---|---|---|---|
| `src/lib.rs` | ~185 | 6 | ~3.2 | Low. 6 tests cover round-trips and hash determinism. No negative-path tests on `Capability::hash` stability across anchor variants, no `Right`/`CapabilityType` exhaustion tests. |
| `src/checkpoint.rs` | ~900 | 20 | ~2.2 | Adequate for the happy paths; thin on `ParseError` variants. |
| `src/inclusion_proof.rs` | ~352 | 14 | ~4.0 | Good coverage; all `InclusionVerifyError` variants exercised. |
| `src/consistency_proof.rs` | ~506 | 11 | ~2.2 | Good for the full 1..=8 grid; 9-variant `ConsistencyVerifyError` fully covered. |

- [ ] **lib.rs: add negative-path tests.** No test for `Capability` with `expiry_t: None` vs `Some` hash sensitivity. No test that changing `witness_pubkey` changes the hash. No test for `Right` / `CapabilityType` round-trip exhaustion. Minimum: 4 additional tests to raise lib.rs floor to ~5/100 LOC.
- [ ] **checkpoint.rs: cover all `ParseError` variants.** Of 10 `ParseError` variants (`NotUtf8`, `Truncated`, `MissingNewline`, `BadTreeSize`, `BadRootHash`, `BadRootHashLength`, `MissingSignatureSeparator`, `MissingEmDash`, `MalformedSignature`, `NoSignatures`), the test module does not have dedicated tests for `NotUtf8`, `Truncated`, `MissingNewline`, `BadRootHashLength`, or `MissingSignatureSeparator`. At least one test per `ParseError` variant is needed for v1.0.0 because the parse path is the consumer-facing API.
- [ ] **checkpoint.rs: cover `VerifyError::BadPublicKey` explicitly.** Currently no test passes a malformed 32-byte pubkey to `verify_signer` to trigger the `BadPublicKey` path (as distinct from key-hash mismatch returning `Ok(false)`). Add one test passing a known-invalid pubkey byte-sequence.
- [ ] **checkpoint.cs: `verify_consistency_proof` sig-failure coverage gap.** The `NewSignatureInvalid` branch of `CheckpointConsistencyError` (step 4 — new checkpoint's own signature invalid) is not exercised by any existing test. The `consistency_proof_wrong_signer_pubkey_rejects` test only reaches `OldSignatureInvalid` (step 3 fails first). Add one test where the old checkpoint signature is valid but the new checkpoint's signature is invalid.
- [ ] **Property-based tests: assess need.** The `ConsistencyProof::verify` has a complex state machine (9 error variants, multiple tree-shape branches). A proptest/quickcheck fuzz over `(old_size, new_size)` pairs would strengthen confidence that no size combination panics or silently accepts a bad proof. **Recommended: add 1–2 proptest/quickcheck cases or document why the full-grid test is sufficient.** This is the one item here that requires a judgment call.

### 2b. Bench numbers

No `[[bench]]` section exists in `system-core/Cargo.toml`; no `benches/` directory. All system-core performance coverage is via `system-ledger/benches/consult.rs`. This is correct per the BENCH report §2 ("all system-core performance coverage is via that suite") and the architectural design (benches exercise the composed primitives rather than the data-shape layer alone).

- [x] **Capability::hash bench**: covered via `system-ledger/benches/consult.rs` bench 1.
- [ ] **Publish bench numbers with hardware qualifier.** The BENCH report §6 recommends using Phase 1A.3 numbers for pre-1A.4 benches and this run's numbers for the 1A.4 benches. This recommendation has not yet been captured in a tracked file — the raw log is on tmpfs (`/tmp/bench-system-ledger.log`) and may not survive reboot. **Action: copy the §3 table from `BENCH-v0.2.0.md` into a tracked location (e.g., `system-core/ARCHITECTURE.md` §5 or a new `BENCHMARKS.md` in the crate root) before the next VM restart.** The BENCH file itself is tracked in `.claude/` but that is local-only and not part of a crates.io publication.
- [ ] **Consistency-proof bench missing.** No bench for `ConsistencyProof::verify` exists in `system-ledger/benches/consult.rs`. For v1.0.0 the published bench table should include at least one consistency-proof measurement (e.g., raw `ConsistencyProof::verify` for a 4→8 transition and the composed `verify_consistency_proof` at the same sizes). Add to `benches/consult.rs`.
- [ ] **Hardware qualifier statement in README.** The bench numbers are for an Intel Xeon 2.20 GHz x86_64 GCP n2-class VM. ARM embedded targets (the seL4 kernel-consumption path) will be 10–50× slower per curve25519-dalek perf data noted in the Phase 1A.3 cleanup-log. This qualification must appear in the published benchmark description — either in a `BENCHMARKS.md` or in `ARCHITECTURE.md` §5. Currently absent.

### 2c. Public-API stability statement

- [ ] **rustdoc on all `pub` items in `lib.rs`.** `Capability`, `WitnessRecord`, `LedgerAnchor`, `CapabilityType`, `Right`, `Hash256` have inline comments but no `///` rustdoc on struct fields. The fields are `pub` with short comments; for v1.0.0 every field needs a `///` doc-comment describing its semantics (not just repeating the name). `Capability::expiry_t` and `Capability::witness_pubkey` have good inline comments; those should become rustdoc. Missing: `Capability::cap_type`, `Capability::rights`, `Capability::ledger_anchor`; all `WitnessRecord` fields; all `LedgerAnchor` fields; `CapabilityType` and `Right` variants.
- [ ] **rustdoc examples in `lib.rs`.** `Capability::hash()` is the primary consumer-facing method in this module. Add a `/// # Examples` block showing construction and hash invocation. Per CLAUDE.md §2 "examples in rustdoc where the API is non-obvious."
- [ ] **rustdoc on `checkpoint.rs` error types.** `ParseError` and `VerifyError` variants have no rustdoc. For v1.0.0 each variant doc must describe the condition that produces it. Currently: no variant-level docs.
- [ ] **`CheckpointInclusionError` and `CheckpointConsistencyError` rustdoc**: already present as `///` doc-comments on each variant — these are v1.0.0 ready. Confirm they pass `cargo doc --no-deps` without warnings.
- [ ] **Versioning policy in README.** Neither `system-core/README.md` nor `ARCHITECTURE.md` states the semver contract explicitly ("MAJOR is frozen for the v1.x.x series; breaking changes require v2.0.0; deprecation path: mark with `#[deprecated]` for one MINOR before removal"). This statement is required for v1.0.0. The P3 README draft in `.claude/drafts-outbound/README-system-core.draft.md` should include this section. **Check the draft and confirm it covers versioning policy before promotion.**
- [ ] **`master-relay.rs` defect closure.** The file `system-core/master-relay.rs` is a residual sketch predating this cluster. It is not a Cargo binary target and does not compile as part of the crate. Per `repo-layout.md`, loose `*.rs` files at the project root are a defect. Must be removed or moved before v1.0.0 (`git rm system-core/master-relay.rs`). This is mechanical.

### 2d. Doc completeness

- [ ] **README.md refresh.** The current `system-core/README.md` (P3 draft exists at `.claude/drafts-outbound/README-system-core.draft.md`) describes only 6 unit tests; §II "WHAT IT DOES NOT CONTAIN" still references the pre-Option-B framing ("new `system-capability-ledger` crate (architecture decision pending)"). These sections must reflect the v0.2.0 surface (51 tests; 4 public modules; 3 composed primitives). **Pickup: apply the P3 draft to the crate.**
- [ ] **ARCHITECTURE.md v0.2.0 refresh.** `system-core/ARCHITECTURE.md` §5 "Verification" still lists 16 unit tests (Phase 1A.2 count; the actual count is 51). §3 still includes the sentence "Status: `system-ledger` not yet created — Phase 1A increment 3 builds it." Both must be updated. See task #26 (P5 — Apply S6 ARCHITECTURE.md drift fixes).
- [ ] **CLAUDE.md state header.** `system-core/CLAUDE.md` header reads `Version: 0.1.4` (updated 2026-04-27 before the consistency-proof bump to 0.2.0). Must be updated to `Version: 0.2.0`.
- [ ] **NEXT.md v0.x.x item cleanup.** `system-core/NEXT.md` Queue still lists items that are now resolved (inclusion_proof, consistency_proof, `LedgerEntry` enum). Queue should be cleaned down to remaining open items only: `Capability::canonical_bytes()` (CBOR stability), `IRQHandler` cap_type variant, `no_std` carve-out path. "Blocked" section is now unblocked. "Deferred" section is accurate.

### 2e. Dependency hygiene

- [ ] **`description` field absent.** `system-core/Cargo.toml` has no `description`, `license`, `repository`, `keywords`, or `categories` fields. For a crate that may be published, all five are required. For a workspace-internal crate, at minimum `description` and `license` are needed so `cargo doc` and tooling report correctly.
- [ ] **`rust-version` (MSRV) absent.** No `rust-version` field. Minimum supported Rust version is currently undocumented. For v1.0.0, the MSRV must be declared. Given `div_ceil` usage in the test helpers (stabilized in Rust 1.73) and the overall code shape, MSRV is approximately Rust 1.73.0 or later.
- [ ] **`no_std` eligibility verification.** Per `system-core/CLAUDE.md` hard constraints: "The crate stays `no_std`-eligible long-term (the kernel may consume it). v0.1.x carries `std` for `Vec` + JSON serialisation; future MINOR carves the `no_std` path." For v1.0.0 the documentation must explicitly state the current `std` dependency and the planned MINOR for `no_std` carve-out. The `no_std` refactor itself is NOT required for v1.0.0 (it is a future MINOR item) but the documentation commitment is. Verify: confirm that all uses of `std` are behind `use std::...` (not `use core::...`) and that a `no_std` carve-out is feasible without API changes.
- [ ] **License field.** `Cargo.toml` currently has no `license` field; crate inherits the monorepo `LICENSE` per `README.md` §V. For v1.0.0 the `license` field must be explicit (e.g., `license = "MIT"` or whichever value `factory-release-engineering/LICENSE-MATRIX.md` assigns to `pointsav-monorepo`). Confirm with Master/operator before filing.
- [ ] **Dependency version constraints are safe.** `serde = "1"`, `serde_json = "1"`, `sha2 = "0.10"`, `ed25519-dalek = "2"`, `base64 = "0.22"` all use `"MAJOR"` or `"MAJOR.MINOR"` style — no `*` or `>=` constraints. No `[patch]` stanzas. This group passes.

### 2f. CI / verification

- [ ] **`cargo check -p system-core`**: passes (confirmed in cleanup-log). Verify clean on v0.2.0 HEAD before promotion.
- [ ] **`cargo test -p system-core`**: 51 tests pass. Verify on v0.2.0 HEAD.
- [ ] **`cargo clippy -p system-core -- -D warnings`**: not confirmed in any cleanup-log entry. Run and confirm zero warnings.
- [ ] **`cargo fmt --check -p system-core`**: not confirmed. Run and confirm.
- [ ] **`cargo doc -p system-core --no-deps`**: not confirmed. Run and confirm zero warnings (any undocumented public item will produce a warning if `#![warn(missing_docs)]` is set, or needs manual check).
- [ ] **Verification outcome recorded in `cleanup-log.md`.** Per session-end discipline, the v1.0.0 CI pass must be logged.

---

## 3. system-ledger 0.2.1 → 1.0.0 checklist

### 3a. Test coverage

Test count at v0.2.1: **44 tests** across 5 modules (lib + 4 module files).

| Module | LOC (approx) | Tests | Tests/100 LOC | Assessment |
|---|---|---|---|---|
| `src/lib.rs` | ~984 | 17 | ~1.7 | Low raw ratio, but the tests are integration-weight (full handover ceremony = ~70 LOC of test body). The `apply_witness_record_*` tests are thorough. Missing: `ConsultError` variants not triggered by any test. |
| `src/cache.rs` | ~184 | 7 | ~3.8 | Good; all code paths covered including zero-capacity and duplicate insert. |
| `src/revocation.rs` | ~157 | 5 | ~3.2 | Good; idempotency + audit-detail non-overwrite tested. |
| `src/apex.rs` | ~387 | 10 | ~2.6 | Good; full handover ceremony + chained handovers covered. `NoOpHandover` and `HandoverHeightBeforeCurrent` error variants tested. |
| `src/witness.rs` | ~283 | 5 | ~1.8 | Adequate; cross-namespace rejection and truncated-sig are the security-critical paths and both are tested. |

- [ ] **`ConsultError::InconsistentState` never triggered in tests.** The `consult_capability` path returns `ConsultError::InconsistentState` when `verify_signer` returns `Err(VerifyError::BadPublicKey)` — which requires a malformed 32-byte key stored in apex history. No test exercises this path. For v1.0.0, at least one test should trigger `ConsultError::InconsistentState` (or document why the path is unreachable in practice).
- [ ] **`LedgerError::NoApexForCheckpoint` triggered by only one test.** `apply_witness_record_with_no_current_checkpoint_errors` covers `NoCurrentCheckpoint`; but `NoApexForCheckpoint` (no apex recorded when `apply_witness_record` is called) has no dedicated test — only exercised implicitly. Add one explicit test.
- [ ] **`apply_witness_record` handover-height path lacks dedicated test.** The `ApexVerdict::Handover` branch in `apply_witness_record` (which uses `old_apex` for inclusion-proof verification) is not exercised by any test in `lib.rs`. This is a consequential path per the doctrine: at the handover height, either apex's signature suffices. Add one test that sets the current checkpoint at the handover height and verifies that `apply_witness_record` succeeds.
- [ ] **`witness.rs`: `WitnessVerifyError::TempFileFailed` and `NonUtf8Path` not tested.** These are infrastructure-failure paths; unit-testing them requires mocking or OS-level injection. Document in the module's `///` doc whether these are considered untestable in CI (acceptable) or add integration-level coverage.
- [ ] **Property-based tests: not warranted.** The `consult_capability` state machine has six explicit `RefuseReason` variants and two `Verdict` variants; each is reachable from a specific test. The combination space is small enough that property testing adds limited value over the current integration fixture. No action required.

### 3b. Bench numbers

10 benches in `system-ledger/benches/consult.rs`; all confirmed passing. Per `BENCH-v0.2.0.md`:

| Bench | Publication status | Action needed |
|---|---|---|
| `Capability::hash` (6.44 µs) | Borderline — 29% above quiet baseline | Re-run on quiet VM OR publish with "±30% depending on VM load" caveat |
| `verify_signer (1-sig)` (4.01 ms) | 5 high-severe outliers | Publish with "Intel Xeon 2.20 GHz" qualifier; note hardware-bound |
| `verify_apex_handover (2-sig)` (7.65 ms) | 6 high-severe outliers | Same qualifier |
| `cache hit` (11.2 ns) | 9 outliers; acceptable for order-of-magnitude | Publish with caveat |
| `cache miss` (362 ns) | Zero outliers — publication-quality | Publish as-is |
| `consult_capability Allow` (3.74 ms) | 7 high-severe outliers; hardware-bound | Publish with hardware qualifier |
| `InclusionProof::verify raw 8-leaf` (5.37 µs) | 5 outliers; acceptable | Publish |
| `InclusionProof::verify raw 1024-leaf` (17.74 µs) | Zero outliers — publication-quality | Publish as-is |
| `verify_inclusion_proof composed 1024-leaf` (4.72 ms) | 22 outliers, 20 high-severe, ±11% CI | **Needs quieter-VM re-run before publication** |
| `apply_witness_record full path` (3.71 ms) | Zero outliers — publication-quality | Publish as-is |

- [ ] **`verify_inclusion_proof composed` bench: re-run on quiet VM.** The 22-outlier, ±11% CI bench is not publication-quality per the BENCH report §6. All other benches may be published with the qualifiers above. This is the one bench that needs a dedicated quiet-VM pass.
- [ ] **Consistency-proof benches missing.** `ConsistencyProof::verify` and `SignedCheckpoint::verify_consistency_proof` are not benchmarked. For v1.0.0 the public bench table should include at least a raw `ConsistencyProof::verify` bench. Add to `benches/consult.rs`.
- [ ] **Bench numbers captured to a tracked file.** The BENCH-v0.2.0.md file is tracked only in `.claude/` (local-only). Before Stage-6 promotion, the release-note bench table (per BENCH report §6 recommendation) must be committed to a tracked path — suggested: `system-ledger/BENCHMARKS.md` or a section in `system-ledger/ARCHITECTURE.md`.
- [ ] **ARM embedded caveat documented.** Same as system-core §2b: the 10–50× ARM slowdown for Ed25519 operations must appear in the published bench description.

### 3c. `LedgerConsumer` trait stability (v1.0.0 consequential)

The trait signature changed in v0.2.0 (breaking change that justified the MINOR bump: `apply_witness_record` gained an `InclusionProof` parameter). Freezing at v1.0.0 means this signature is locked.

- [ ] **Confirm `apply_witness_record` signature is final.** Master v0.1.26 ratified the current signature (`record: WitnessRecord, proof: InclusionProof`). The question for v1.0.0 is whether `witness: Option<&WitnessRecord>` on `consult_capability` should become a separate `consult_capability_with_witness` method or similar. If this API shape could change before v1.0.0, it must change now — not after. **Surface to Master/operator as open question §8.1.**
- [ ] **`set_current_checkpoint` inherent-vs-trait decision documented.** Master v0.1.26 §5a confirmed `set_current_checkpoint` stays inherent on `InMemoryLedger` for v0.2.x. For v1.0.0, is this still correct? If `MoonshotDatabaseLedger` also needs a `set_current_checkpoint`, it should be on the trait. **Document the rationale in `ARCHITECTURE.md` or `CLAUDE.md`.** The constraint is already in the cleanup-log; promote to a doc comment on `set_current_checkpoint` before v1.0.0.
- [ ] **`apply_witness_record_unchecked` disposition for v1.0.0.** The `#[cfg(test)]` carve-out is currently the backward-compat shortcut for tests that do not construct full Merkle fixtures. For v1.0.0: this method is `pub` and `#[cfg(test)]` — it does not appear in the public API surface seen by downstream consumers. The carve-out is architecturally appropriate and should be retained. **Document explicitly in the rustdoc for `apply_witness_record` that `apply_witness_record_unchecked` is the test-only shortcut.** Already partially documented in the `InMemoryLedger` struct doc; confirm the message is clear.

### 3d. Witness namespace durable promise

- [x] **`WITNESS_NAMESPACE = "capability-witness-v1"` is a v1.0.0 durable promise.** Changing this after v1.0.0 would break cross-namespace replay-rejection properties for existing deployed signatures. The constant is correctly named and documented in `witness.rs`. No action needed — confirm only that the doc comment names this as a stability-critical constant before promotion.

### 3e. Doc completeness

- [ ] **README.md refresh.** Current `system-ledger/README.md` §V "STATE" still reads "Skeleton: trait + types + module stubs. Module impls land in subsequent commits." This must reflect the v0.2.1 delivered state (44 tests; all modules fully implemented; bench suite passing). P4 draft at `.claude/drafts-outbound/README-system-ledger.draft.md` — apply before promotion.
- [ ] **ARCHITECTURE.md v0.2.1 refresh.** `system-ledger/ARCHITECTURE.md` §3 Decision Flow shows a pseudocode comment "The Merkle inclusion proof check (step 4 last) currently relies on the consumer pre-validating..." — this is no longer accurate; the production path now validates via `InclusionProof` in `apply_witness_record`. §5 "Verification" still reflects the skeleton ("zero tests"). Both must be updated (see task #26 P5).
- [ ] **CLAUDE.md state header.** `system-ledger/CLAUDE.md` header reads `Version: 0.2.1` — correct. Last-updated date is 2026-04-27 — correct. No change needed to the header. Confirm the "Current state" body paragraph does not still say "Skeleton commit landed."
- [ ] **NEXT.md v0.x.x item cleanup.** `system-ledger/NEXT.md` Queue still lists all six original implementation tasks (#18, #19, #11, #12, #20, #21) as future work; they are now complete. Clean the Queue down to remaining open items: `MoonshotDatabaseLedger` (correctly in Deferred), multi-threaded LedgerConsumer (correctly in Deferred). "Right now" section should be cleared.

### 3f. Dependency hygiene

- [ ] **`description`, `license`, `repository`, `keywords`, `categories` absent.** Same gap as system-core. `system-ledger/Cargo.toml` has none of these fields. Required for v1.0.0.
- [ ] **`rust-version` (MSRV) absent.** Same gap. Must declare. The `iter().rposition()` usage and `div_ceil` in test helpers bound MSRV.
- [ ] **`tempfile = "3"` in `[dependencies]` (not `[dev-dependencies]`).** `tempfile` is listed under `[dependencies]` rather than `[dev-dependencies]`. It is only used in `witness.rs` for the `ssh-keygen` tempfile approach — which is production code (not test-only). This is intentional and correct; do NOT move to dev-dependencies. But note: if the `no_std` path ever becomes required, `tempfile` is an `std`-dependent crate. Document this constraint in `CLAUDE.md` hard constraints.
- [ ] **`ed25519-dalek = "2"` in `[dev-dependencies]`.** Correct placement; only used in `benches/consult.rs` and `lib.rs` `#[cfg(test)]` blocks. Passes.
- [ ] **No `*` or `>=` constraints.** All constraints use specific MAJOR versions. Passes.

### 3g. CI / verification

- [ ] **`cargo check -p system-ledger`**: confirmed passing in cleanup-log. Verify on HEAD.
- [ ] **`cargo test -p system-ledger`**: 44 tests pass. Verify on HEAD.
- [ ] **`cargo clippy -p system-ledger -- -D warnings`**: not confirmed. Run and confirm.
- [ ] **`cargo fmt --check -p system-ledger`**: not confirmed. Run and confirm.
- [ ] **`cargo bench -p system-ledger`**: confirmed passing (BENCH report). Verify on HEAD.
- [ ] **`cargo doc -p system-ledger --no-deps`**: not confirmed. Run and confirm zero warnings.
- [ ] **Verification outcome recorded in `cleanup-log.md`.**

---

## 4. Cross-crate concerns

### 4a. Promote together or independently?

`system-ledger` has a hard path dependency on `system-core` (`path = "../system-core"`). If both are promoted in the same Stage-6 run, the monorepo version on `pointsav/pointsav-monorepo main` will show them landing at the same commit — clean, auditable.

If promoted independently, `system-core` 1.0.0 must land first; `system-ledger` 1.0.0 would then update its `Cargo.toml` dependency from `path = "../system-core"` to a version constraint — which only applies after `system-core` is on crates.io or the promoted branch. For an internal monorepo with path dependencies, this distinction may not matter: both crates remain path-linked regardless of version number.

**Recommendation: promote together in the same Stage-6 run.** The API surface of both crates was designed as a unit; their tests cross-reference each other (bench file imports from both); the ARCH documents describe the split. Landing them together in one promotion commit is the cleanest audit trail. If sequencing is forced, `system-core` goes first.

### 4b. `set_current_checkpoint` — inherent vs trait (Master v0.1.28 confirmed)

Master v0.1.26 §5a confirmed `set_current_checkpoint` stays inherent on `InMemoryLedger` for v0.2.x. For v1.0.0, the question is whether a `MoonshotDatabaseLedger` implementor would also need `set_current_checkpoint`. Since that implementor is not in scope for v1.0.0 (it is a future MINOR item per `NEXT.md`), the inherent approach is acceptable at v1.0.0. However, this must be documented as a known limitation so future implementors know to add the method themselves. **Capture in `LedgerConsumer` trait doc comment.**

---

## 5. Items NOT in scope of Stage-6 promotion

The following are explicitly excluded. Do not treat these as blockers.

- **Consistency-proof bench re-run on quieter VM** — this refines the already-published `verify_inclusion_proof composed` number (currently ±11% CI). The number is close enough for architecture claims; publication-grade precision is a 0.2.2 release-note item.
- **`MoonshotDatabaseLedger` implementor** — a separate crate at 0.x.x; `LedgerConsumer` trait is designed to be implemented by it but it does not exist yet. Stage-6 promotes the trait, not the implementor.
- **`no_std` carve-out** — documented in `system-core/CLAUDE.md` as a future MINOR. Does not block v1.0.0; must be documented.
- **TOPIC wiki drafts** — `drafts-outbound/topic-merkle-proofs-as-substrate-primitive.md` and `.es.md` are staged but their delivery through the project-language gateway is an independent pipeline. Do not block promotion on TOPIC delivery.
- **seL4 CDT integration** — Phase 4+ (per `system-ledger/ARCHITECTURE.md` §3 "What it does not contain"). Not required for v1.0.0.
- **`Capability::canonical_bytes()` (CBOR stability)** — queued in `system-core/NEXT.md`; a future MINOR when the format-migration path is defined. Not v1.0.0.
- **`IRQHandler` cap_type variant** — queued in `system-core/NEXT.md`; a future MINOR; requires seL4 CDT cross-check. Not v1.0.0.

---

## 6. Estimated effort

### system-core

| Group | Items | Effort | Mechanical? |
|---|---|---|---|
| Test coverage additions (§2a) | 7–9 new tests | 2–3 hours | Mostly mechanical — each test is a fixture + assert pattern matching existing tests |
| Bench: add consistency-proof bench + capture table to tracked file (§2b) | 2 benches + 1 file write | 1–2 hours | Mechanical |
| rustdoc completeness (§2c) | ~20 missing `///` doc-comments | 2–3 hours | Mechanical (terminology is defined) |
| Versioning policy statement (§2c) | 1 paragraph in README | 30 min | Master judgment required — policy statement |
| `master-relay.rs` deletion (§2c) | `git rm` + commit | 15 min | Mechanical |
| README and ARCHITECTURE.md updates (§2d) | Apply P3 draft; update §5 counts | 1 hour | Mostly mechanical |
| CLAUDE.md + NEXT.md cleanup (§2d) | Header version + queue cleanup | 30 min | Mechanical |
| Cargo.toml metadata (§2e) | 5 fields + rust-version | 30 min | Requires operator decision on license value |
| CI verification pass (§2f) | Run 5 commands; log results | 1 hour | Mechanical |
| **Total system-core** | | **~9–13 hours** | ~80% mechanical |

### system-ledger

| Group | Items | Effort | Mechanical? |
|---|---|---|---|
| Test coverage additions (§3a) | 3–4 new tests | 2–3 hours | Mostly mechanical |
| Bench: add consistency-proof bench + quiet-VM re-run + capture table (§3b) | 2 benches + 1 re-run + 1 file write | 3–4 hours (re-run may require scheduling a quiet-VM window) | Requires quiet-VM window — operator-coordinated |
| `LedgerConsumer` trait docs + stability confirmation (§3c) | 3 doc-comment additions | 1–2 hours | 1 item requires Master judgment (API finality) |
| README and ARCHITECTURE.md updates (§3e) | Apply P4 draft; update §5; fix §3 stale text | 1 hour | Mostly mechanical |
| CLAUDE.md + NEXT.md cleanup (§3e) | Body text + queue cleanup | 30 min | Mechanical |
| Cargo.toml metadata (§3f) | 5 fields + rust-version | 30 min | Requires operator decision on license value |
| CI verification pass (§3g) | Run 6 commands; log results | 1 hour | Mechanical |
| **Total system-ledger** | | **~9–14 hours** | ~75% mechanical |

**Combined Stage-6 effort: ~18–27 hours across 2–4 sessions.** The dominant open item requiring a calendar slot is the quiet-VM bench re-run for `verify_inclusion_proof composed`.

---

## 7. Recommended sequencing

If Stage-6 work starts now (parallel to any Phase 2 work):

**Entry point 1 — Mechanical cleanup (sub-agentable, Sonnet tier):** Start with Cargo.toml metadata + CLAUDE.md/NEXT.md header cleanup + `master-relay.rs` deletion + README draft application (P3 and P4 drafts already exist in `drafts-outbound/`). Zero judgment calls; all four items are mechanical. Estimated 2–3 hours. Prerequisite: none.

**Entry point 2 — rustdoc pass (sub-agentable, Sonnet tier):** After entry point 1 merges, run the rustdoc completeness pass on both crates. All terminology is defined in the doctrine; the task is applying `///` docs to existing public items. Estimated 2–3 hours. Prerequisite: none (can run in parallel with entry point 1 on a separate clone).

**Entry point 3 — Test gap closure (Task-tier judgment):** The missing tests for `ParseError` variants, `ConsultError::InconsistentState`, `apply_witness_record` handover-height path, and `verify_consistency_proof NewSignatureInvalid` require a Task session that understands the state machine. Estimated 3–4 hours. Prerequisite: entry points 1 + 2 done (clean baseline before adding tests). This is the item that benefits most from a Task Claude with context continuity.

**Dependencies:**
- Entry points 1 and 2 can run in parallel.
- Entry point 3 should follow 1 and 2.
- Bench re-run (quiet-VM slot) can happen any time; it is independent of all other items.
- Master/operator sign-off on license value (§2e/§3f) should be requested early — it gates the Cargo.toml metadata item.

---

## 8. Open questions for Master / operator

These cannot be resolved at Task tier. Surface before Stage-6 work begins.

1. **Promote together or independently?** This checklist recommends promoting both crates in the same Stage-6 run. If the operator wants independent promotion (e.g., to get `system-core` 1.0.0 into other consumers sooner), `system-core` goes first. Decision affects whether the Stage-6 branch includes one or two simultaneous version bumps.

2. **`LedgerConsumer` trait API finality.** Before freezing at v1.0.0, confirm: is the `consult_capability(cap, current_root, now, witness: Option<…>)` signature final? Should `witness` be a separate method call to keep the common path thin? This is an architectural judgment that cannot be made at Task tier — the answer determines whether a final MINOR (e.g., 0.3.0) is needed before v1.0.0.

3. **License declaration.** `Cargo.toml` has no `license` field in either crate. The monorepo `README.md` references `factory-release-engineering/LICENSE-MATRIX.md` as the authority. What SPDX expression should be declared in `Cargo.toml` for these two crates? The answer belongs to Master/operator. Until this is answered, the Cargo.toml metadata items are blocked.

4. **Stage-6 commit attribution.** The Stage-6 promotion commit lands on `pointsav/pointsav-monorepo main`. Per CLAUDE.md §2, commits to `pointsav/*` canonical repos come via the staging → promotion flow (`bin/promote.sh`, Stage 6), attributed to Jennifer or Peter via the alternating toggle. Confirm: is the v1.0.0 version-bump commit attributed via the normal `bin/commit-as-next.sh` alternation, or is this a `ps-administrator` admin-tier commit?

5. **Quiet-VM bench re-run window.** The `verify_inclusion_proof composed` bench needs a quiet-VM pass. When is the next available window where the foundry-workspace VM will have load averages near zero? This is an operational scheduling question; the Task cannot control VM load. Operator should flag this as a calendar item.

---

*Generated by Task Claude (claude-sonnet-4-6) — 2026-04-27*
*Inputs: system-core 0.2.0 + system-ledger 0.2.1 source; BENCH-v0.2.0.md; outbox-archive Phase 1A.5; inbox-archive Master v0.1.33 + v0.1.42 + v0.1.28 ratification messages; conventions/system-substrate-doctrine.md; cleanup-log.md.*
