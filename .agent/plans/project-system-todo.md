---
schema: foundry-plan-v1
title: project-system — Comprehensive Work Plan
created: 2026-05-20
updated: 2026-05-21 (Group 6 mechanical done; Group 7 complete)
author: task@claude-code (session startup, sonnet-4-6)
status: active
---

# project-system — Comprehensive Work Plan

> Living document. Check items off as they complete. Add new items as they surface.
> Update the `updated:` frontmatter date on each edit.
>
> Sources synthesized into this plan:
> - `.agent/manifest.md` — tetrad legs and cluster scope
> - `.agent/CHECKLIST-stage6-promotion-readiness.md` — v1.0.0 readiness detail
> - `.agent/BENCH-v0.2.0.md` — benchmark report
> - `.agent/RESEARCH-netbsd-veriexec-bootflow.md` — Phase 2 forward-prep
> - `.agent/SURVEY-os-candidates-for-phase2.md` — Phase 2 os-* candidates
> - `.agent/STAGING-outbox-draft.md` — last outbox to Master
> - `.agent/AUDIT-moonshot-toolkit-arch-vs-cli.md` — moonshot-toolkit drift
> - `.agent/drafts-outbound/*` — all 8 staged drafts
> - `cleanup-log.md` — session history and open questions

---

## How to use this plan

- Work top-to-bottom within each group.
- Items marked **[BLOCKED: ...]** cannot start until the named blocker resolves.
- Items marked **[OPERATOR DECISION]** require an answer from the operator before proceeding.
- Items marked **[MASTER DECISION]** require Master Claude sign-off.
- Items marked **[MECHANICAL]** are low-judgment, can be sub-agented or done quickly.
- Strike through `~~item~~` when superseded rather than deleting it.

---

## Group 0 — Immediate / housekeeping (do before any new work)

- [x] **Reset WFD sub-clone to canonical HEAD** _(done 2026-05-20 — HEAD already at 7fdf36b; no reset needed)_

- [x] **Confirm WFD spoke-configs awareness** _(done 2026-05-20 — noted; 3 inbox messages archived to inbox-archive.md)_

- [x] **Preserve BENCH numbers to tracked file before next VM restart** _(done 2026-05-20 Group 2C — `system-ledger/BENCHMARKS.md` created; commit `0881091`)_

---

## Group 1 — Content artifacts: complete existing drafts

These artifacts are staged in `.agent/drafts-outbound/` with full substance written. They need the language-pass gate, then committed to their target repos.

### 1A — README drafts (ready for language pass)

Three pairs of README drafts are effectively publication-ready. They need:
1. A language pass by project-editorial (BCSC posture, banned-vocab check, bilingual alignment)
2. Then commitment to `pointsav-monorepo` replacing the stale existing READMEs

- [x] **Route README-system-core.draft.md + .es.md → project-editorial** _(done 2026-05-20 — outbox msg project-system-20260520-readme-drafts-ready)_
- [x] **Route README-system-ledger.draft.md + .es.md → project-editorial** _(done 2026-05-20 — same outbox message)_
- [x] **Route README-moonshot-toolkit.draft.md + .es.md → project-editorial** _(done 2026-05-20 — same outbox message)_

- [ ] **Apply approved README drafts to monorepo** (after editorial returns them)
  `system-core/README.md`, `system-ledger/README.md`, `moonshot-toolkit/README.md`
  Commit via `bin/commit-as-next.sh`.

### 1B — TOPIC: Merkle Proofs as a Substrate Primitive ✓

The TOPIC substance pass is complete (2026-05-20). Full prose written for all 8 sections from source code + BENCH-v0.2.0.md numbers. File at `.agent/drafts-outbound/topic-merkle-proofs-as-substrate-primitive.md`.

- [x] **Write substance for §1 — What Merkle proofs are** _(done 2026-05-20)_
- [x] **Write substance for §2 — Two flavours: inclusion and consistency** _(done 2026-05-20)_
- [x] **Write substance for §3 — Inclusion proofs in `system-core`** _(done 2026-05-20)_
- [x] **Write substance for §4 — Consistency proofs in `system-core`** _(done 2026-05-20)_
- [x] **Write substance for §5 — Composed primitives on `SignedCheckpoint`** _(done 2026-05-20)_
- [x] **Write substance for §6 — Consumer integration in `system-ledger`** _(done 2026-05-20)_
- [x] **Write substance for §7 — Why this matters as a substrate primitive** _(done 2026-05-20)_
- [x] **Write substance for §8 — Cross-references** _(done 2026-05-20)_

- [x] **Update Spanish overview** _(done 2026-05-20 — full strategic-adaptation panorama written for all 8 sections)_

- [x] **Route completed TOPIC → project-editorial outbox** _(done 2026-05-20 — outbox message project-system-20260520-topic-merkle-ready written)_

### 1C — TOPIC: The Capability Ledger Substrate ✓

Target: `vendor/content-wiki-documentation/topic-capability-ledger-substrate.md`

- [x] **Create TOPIC with full substance** _(done 2026-05-20 — all 9 sections written from source code; file at `.agent/drafts-outbound/topic-capability-ledger-substrate.md`)_
- [x] **Create Spanish panorama** _(done 2026-05-20 — `.agent/drafts-outbound/topic-capability-ledger-substrate.es.md`)_
- [x] **Route → project-editorial outbox** _(done 2026-05-20 — outbox message project-system-20260520-topic-capability-ready written)_

### 1D — TOPIC: The Two-Bottoms Sovereign Substrate (not started)

Target: `vendor/content-wiki-documentation/topic-two-bottoms-sovereign-substrate.md`
Listed in manifest `planned_topics`. Write after Phase 2 work begins (future-leaning content needs Phase 2 proof-points to be credible).

- [ ] **Create TOPIC skeleton with `foundry-draft-v1` frontmatter**
  _[BLOCKED: Phase 2 prototype should be underway first so §5 can be concrete, not speculative]_
  Planned sections:
  - §1 The two-bottoms design (native seL4 + compat NetBSD)
  - §2 Why NetBSD, not Linux, as the compat bottom
  - §3 Verified-image boot via Veriexec (strict mode 3)
  - §4 The thin shim — Cargo feature flag, `CapabilityInvoker` trait
  - §5 Rump kernels as the IT/OT bridge
  - §6 Customer verification chain (build.sh + veriexecgen + apex signing)
  - §7 Cross-references

- [ ] **Write all sections** (after Phase 2 first-deliverable exists)
- [ ] **Create Spanish pair**
- [ ] **Route → project-editorial outbox**

### 1E — GUIDEs: substrate rollout (not started)

These are the vendor-side operational GUIDEs showing how to roll out the substrate to each fleet infrastructure tier. Target: `pointsav-fleet-deployment/fleet-infrastructure-{onprem,cloud,leased}/`.

- [ ] **[BLOCKED: Phase 1C complete]** Draft `GUIDE-substrate-rollout-onprem.md`
  Content: how to install and verify `system-core` + `system-ledger` + `moonshot-toolkit` on an on-premises Totebox instance.

- [ ] **[BLOCKED: Phase 1C complete]** Draft `GUIDE-substrate-rollout-cloud.md`
  Content: same for cloud-hosted fleet infrastructure.

- [ ] **[BLOCKED: Phase 1C complete]** Draft `GUIDE-substrate-rollout-leased.md`
  Content: same for leased/VPN infrastructure.

All three will route through project-editorial after drafting.

---

## Group 2 — Crate documentation hygiene (pre-v1.0.0)

These are the doc-quality items identified in `CHECKLIST-stage6-promotion-readiness.md`.
Prerequisite for Stage-6 promotion of `system-core` and `system-ledger` to v1.0.0.

### 2A — `system-core` hygiene ✓ _(done 2026-05-20, commit `dcb2700`)_

- [x] Delete `system-core/master-relay.rs`
- [x] Update `system-core/CLAUDE.md` version header
- [x] Clean `system-core/NEXT.md` queue
- [x] Update `system-core/ARCHITECTURE.md` §5 test count + §3 system-ledger status
- [x] Add `///` rustdoc to all `pub` items in `system-core/src/lib.rs` + `checkpoint.rs`
- [ ] **Add missing Cargo.toml metadata to `system-core/Cargo.toml`**
  _[OPERATOR DECISION: what SPDX license value? Check `factory-release-engineering/LICENSE-MATRIX.md`]_

### 2B — `system-core` test gap closure ✓ _(done 2026-05-20, commit `334462b`)_

- [x] Negative-path tests in `lib.rs` (4 new tests — hash sensitivity, round-trips)
- [x] `ParseError` variant tests in `checkpoint.rs` (5 variants covered)
- [x] `VerifyError::BadPublicKey` test (y=2 non-curve point)
- [x] `verify_consistency_proof` `NewSignatureInvalid` coverage
- [ ] **[OPTIONAL, OPERATOR DECISION] Property-based tests for `ConsistencyProof::verify`**

### 2C — `system-ledger` hygiene ✓ _(done 2026-05-20, commit `0881091`)_

- [x] Update `system-ledger/CLAUDE.md`, `NEXT.md`, `ARCHITECTURE.md`
- [x] Commit `BENCH-v0.2.0.md` numbers to `system-ledger/BENCHMARKS.md`
- [ ] **Add `description`, `license`, `repository`, `keywords`, `categories`, `rust-version` to `system-ledger/Cargo.toml`**
  _[OPERATOR DECISION: same license question as system-core]_

### 2D — `system-ledger` test gap closure ✓ _(done 2026-05-20, commit `cb935f9`)_

- [x] `ConsultError::InconsistentState` test (bad apex pubkey y=2)
- [x] `LedgerError::NoApexForCheckpoint` explicit test
- [x] `apply_witness_record` at handover height (`ApexVerdict::Handover` branch)
- [x] **`WitnessVerifyError::TempFileFailed` and `NonUtf8Path` coverage decision**
  _Decided: untestable in unit CI (requires injecting OS tempfile failure or non-UTF-8 path; both are OS-level fault injection not suitable for cargo test). Documented as infrastructure-failure paths; no test added._

### 2E — `moonshot-toolkit` ARCHITECTURE.md drift ✓ _(confirmed already applied — no commit needed)_

### 2F — CI verification pass ✓ _(done 2026-05-20, commit `54fb7e7`)_

- [x] `cargo clippy -D warnings` — clean (1 lint fixed: `push_str("…")` → `push('…')`)
- [x] `cargo fmt --check` — clean (19 diffs fixed)
- [x] `cargo doc --no-deps` — clean (7 broken intra-doc links + 1 bare URL fixed)

---

## Group 3 — Architecture decisions (gate items — need answers before code)

### 3A — Phase 1C decisions (seL4 cross-compile) ✓ _(decided 2026-05-27)_

- [x] **Decision: cross-compile toolchain** — _(decided 2026-05-27)_
  **CHOSEN: operator-installed system dependency** — `apt-get install gcc-aarch64-linux-gnu qemu-system-aarch64`.
  Simplest; workspace VM is long-lived so one-time install is appropriate.

- [x] **Decision: seL4 source vendoring strategy** — _(decided 2026-05-27)_
  **CHOSEN: `vendor-sel4-kernel/` snapshot** — already at 1074 files; use it.
  Maintains hermetic-build property (no network at build time). Accept maintenance commitment for version bumps.

- [x] **Decision: toolchain installation ownership** — _(decided 2026-05-27)_
  **CHOSEN: operator-manual one-time install** — operator runs `sudo apt-get install` directly.
  No infrastructure scripting overhead for a single VM install.

### 3B — Stage-6 promotion decisions

_[OPERATOR DECISION + MASTER DECISION]_

- [ ] **Decision: license SPDX value for `system-core/Cargo.toml` and `system-ledger/Cargo.toml`**
  Source of truth: `vendor/factory-release-engineering/LICENSE-MATRIX.md`.
  Required before Cargo.toml metadata can be completed.

- [ ] **Decision: `LedgerConsumer` trait API finality**
  Is `consult_capability(cap, current_root, now, witness: Option<…>)` the final v1.0.0 signature?
  If not, a MINOR (0.3.0) bump is needed before v1.0.0 freezes the API.

- [ ] **Decision: promote system-core + system-ledger together or independently?**
  Recommendation: together (they are a designed unit; the bench file cross-references both).

- [ ] **Decision: v1.0.0 commit attribution**
  Normal alternating-toggle (jwoodfine/pwoodfine) via `bin/commit-as-next.sh`, or admin-tier?

### 3C — Cross-cluster Cargo dependency (project-data)

_[MASTER DECISION — surfaced in STAGING-outbox-draft.md §6]_

- [ ] **Decision: system-core API stability for Stage-6**
  If stable → promote-then-consume (zero bridging machinery for project-data's `service-fs`).
  If breaking changes still expected → Cargo `[patch]` bridge in project-data.

- [ ] **Decision: should `fs-anchor-emitter` promote same Stage-6 run as `system-core`?**

### 3D — Phase 2 decisions ✓ _(decided 2026-05-27)_

- [x] **Decision: AArch64 hardware target** — _(decided 2026-05-27)_
  **CHOSEN: QEMU on workspace VM** — `qemu-system-aarch64`; available immediately once installed.
  Fast prototype iteration; no procurement. Real hardware is Phase 3+.

- [x] **Decision: shim crate location** — _(decided 2026-05-27)_
  **CHOSEN: new crate `system-substrate-netbsd/`** — parallel to `system-substrate-broadcom/`
  and `system-substrate-freebsd/`. Consistent naming pattern; isolates BSD compat code.

- [x] **Decision: Phase 2 os-* candidate** — _(decided 2026-05-27)_
  **CHOSEN: `os-totebox`** — Doctrine claim #34 names Totebox as the compat-bottom boot vehicle.
  Most direct falsification test for the Two-Bottoms thesis claim.

- [x] **Decision: image signing key for `signatures.veriexec`** — _(decided 2026-05-27)_
  **CHOSEN: new dedicated image-signing key** — separate from `ps-administrator` (commit signing).
  Trust domains must not be conflated. Requires Master Session to provision key in identity store.
  _[BLOCKED: Master Session must generate + add key before Phase 2 Veriexec table can be signed]_

---

## Group 4 — Phase 1C: seL4 cross-compile _(gate decisions resolved 2026-05-27 — UNBLOCKED)_

- [ ] **Install AArch64 cross-compile toolchain on workspace VM** — _[operator runs: `sudo apt-get install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu qemu-system-aarch64`]_

- [ ] **Audit `vendor-sel4-kernel/` contents and version**
  Confirm seL4 version, Microkit version, and that it matches `moonshot-toolkit/ARCHITECTURE.md` §5 reference.
  (seL4 v15.0.0, Microkit 2.2.0, rust-sel4 4.0.0 per manifest)

- [ ] **Implement `moonshot-toolkit build` subcommand — actual execution**
  Replace the "would run" stub with real `std::process::Command` invocations.
  Each `BuildStep` variant (`CompilePd`, `AssembleImage`) gets an executor function.
  Gate: hermetic build property must hold (no network at build time).

- [ ] **Write a minimal `system-spec.toml` for a seL4 hello-world**
  One PD, no channels, one memory region (UART for output).
  This is the validation fixture for `moonshot-toolkit build`.

- [ ] **Boot seL4 hello-world in QEMU AArch64**
  `qemu-system-aarch64 -machine virt -cpu cortex-a72 -m 2G -nographic \`
  `-kernel <image> -append "..."`
  Confirm PD runs and outputs via UART.

- [ ] **Cosign the resulting image with Sigstore Cosign + customer-apex key**
  Per `system-substrate-doctrine.md` §6.1 release-artefact format.
  The `plan_hash` becomes the signed artefact.

- [ ] **Write Phase 1C commit(s) and stage for promotion**
  moonshot-toolkit v0.1.x → v0.2.0 (MINOR: `build` now actually builds).
  Outbox to Command Session: "Phase 1C closed; Phase 2 greenlit."

---

## Group 5 — Phase 2: NetBSD compat-bottom prototype (gate: Phase 1C complete + image-signing key provisioned by Master)

_[Group 3D decisions resolved 2026-05-27. Remaining blocker: Phase 1C seL4 hello-world; image-signing key (Master Session needed)]_
_Target: `os-totebox` as first compat-bottom boot vehicle. Shim crate: `system-substrate-netbsd/`._

### 5A — NetBSD AArch64 VM provisioning

- [ ] **Cross-compile NetBSD AArch64 image on workspace VM**
  `./build.sh -m evbarm -a aarch64 -U MKREPRO=yes tools distribution release`
  KERNCONF: `GENERIC64` (includes `options VERIFIED_EXEC`).
  Estimated build time: 2–4 hours for first `tools` phase.
  _Source: RESEARCH-netbsd-veriexec-bootflow.md §3.5_

- [ ] **Boot NetBSD AArch64 in QEMU on workspace VM**
  `qemu-system-aarch64 -machine virt -cpu cortex-a72 -m 2G ...`
  Confirm boot to login prompt.

- [ ] **Set up Veriexec in strict mode 3**
  Run `veriexecgen -s SHA512 -D <rootfs> > /etc/signatures.veriexec`.
  Confirm: `veriexec(8) dump` shows loaded table.
  Confirm: `sysctl kern.veriexec.strict=3` takes effect.
  Confirm: an unregistered binary is refused exec with EPERM.

- [ ] **Sign `signatures.veriexec` with apex key**
  `ssh-keygen -Y sign -f <apex-key> -n foundry-image-v1 signatures.veriexec`
  Key: per Group 3D decision.

### 5B — Compat-bottom shim crate

- [ ] **Create / extend shim crate** (per Group 3D shim-location decision)
  Implement `CapabilityInvoker` trait (NetBSD compat backend via POSIX).
  seL4 native backend is a stub (Phase 4).
  Cargo feature flags: `features = ["compat"]` / `features = ["native"]`.

- [ ] **Add chosen `os-*` candidate to workspace `[members]`**
  Whichever of `os-console` or `os-totebox` is chosen in Group 3D.
  Confirm `cargo check -p <os-*>` passes on Linux before compat-bottom work begins.

- [ ] **Port chosen `os-*` binary to depend on shim crate**
  Replace Python relay (in `os-console`) or cargo-new stub (in `os-totebox`) with Rust binary
  that calls `system-core` types and routes through `CapabilityInvoker`.

- [ ] **Build and run chosen `os-*` binary on Linux (workspace VM)**
  Baseline run: confirm capability event flows to `service-fs` WORM ledger (`127.0.0.1:9100`).
  Verify `system-core::verify_inclusion_proof` on the event succeeds.
  Verify `system-ledger::consult_capability` returns `Verdict::Allow`.

- [ ] **Cross-compile chosen `os-*` binary for AArch64 NetBSD**
  Build with `--target aarch64-unknown-netbsd` (or the correct target triple).
  Confirm `cargo check` passes under the compat feature flag.

- [ ] **Run `os-*` binary on QEMU AArch64 NetBSD VM**
  Add binary to `signatures.veriexec` → re-sign → re-image → boot.
  Confirm same capability event flows through and `verify_inclusion_proof` succeeds.

- [ ] **Demo end-state confirmation**
  Same `os-*` binary: runs on Ubuntu/x86_64 (workspace VM) AND on QEMU AArch64 NetBSD.
  Capability event visible in WORM ledger on both platforms.
  `Verdict::Allow` returned on both platforms.

---

## Group 6 — Stage-6 promotion (gate: Group 2 hygiene + Group 3B decisions)

_[BLOCKED: Groups 2 + 3B must complete first]_

- [ ] **Quiet-VM bench re-run for `verify_inclusion_proof composed`**
  Current number (4.72 ms, ±11% CI, 22 outliers) is not publication-quality.
  Needs a run with 1-min load average < 1.0 on the workspace VM.
  Schedule with operator when workspace is idle.

- [x] **Add consistency-proof bench to `system-ledger/benches/consult.rs`** _(done 2026-05-21, commit `d2f6a5a` — benches 11-12; BENCHMARKS.md updated to 12 entries)_
  Results: ConsistencyProof::verify (raw, 4→8) 10.86 µs [10.73, 11.00]; verify_consistency_proof (composed) 8.37 ms [8.22, 8.53].

- [x] **Run full CI verification pass on clean HEAD** _(done 2026-05-21 — clippy, fmt, cargo doc all clean across system-core, system-ledger, moonshot-toolkit)_

- [x] **Fill Cargo.toml metadata** _(done 2026-05-21, commit `6e25c46` — license AGPL-3.0-or-later from LICENSE-MATRIX.md §4.2; description, repository, keywords, categories, rust-version on all three crates)_

- [x] **system-core/ARCHITECTURE.md corrections** _(done 2026-05-21, commit `6e25c46` — test count 51→62; no_std roadmap + MSRV documented; system-ledger bench ref updated)_

- [x] **Outbox to Command Session: Group 3B gate decisions** _(done 2026-05-21, commit `6e25c46` — msg project-system-20260521-v100-gate-decisions)_
  Surfaces: LedgerConsumer API finality, promote-together-or-separate, commit attribution, quiet-VM bench re-run.

- [x] **Stage system-core + system-ledger v1.0.0 version bumps** _(done 2026-05-27, commit `c2ae1e9`)_
  Decisions resolved in session: LedgerConsumer API final as-is; promote together; normal attribution; bench #9 opportunistic.
  Updated: `Cargo.toml` (0.2.0/0.2.1 → 1.0.0), `CLAUDE.md` headers, `CHANGELOG.md` created for each.

- [x] **Commit v1.0.0 bumps and outbox "Stage-6 ready" to Command Session** _(done 2026-05-27)_
  Outbox msg `project-system-20260527-stage6-v100` prepended; prior gate-decisions msg marked actioned.

---

## Group 7 — Ongoing housekeeping ✓ _(done 2026-05-21)_

- [x] **WFD sub-clone housekeeping** _(done 2026-05-21, WFD commit `9ba968d`)_
  - Created repo-level `CLAUDE.md` (was missing; required per repo-layout.md)
  - Bilingual README audit: added `README.es.md` to `fleet-infrastructure-leased`
    and `node-console-operator` — all 16 present directories now paired
  - `NEXT.md` updated: closed 3 items; surfaced registry-drift open question
    (`gateway-orchestration-gis-1` + `gateway-knowledge-documentation-1` absent
    from HEAD 7fdf36b — needs reconciliation against main)
  - No new guides needed for cluster-totebox-corporate or cluster-totebox-personnel
    (existing guides are current; no substrate-adjacent work surfaced)

- [x] **STAGING-cargo-dep-options.md decision** _(done 2026-05-21, commit `bb5d415`)_
  Resolved as Option E: block on Stage-6. `system-core` v0.2.0 is API-stable;
  Stage-6 imminent pending 3B decisions. No Cargo `[patch]` required.
  Master/Operator confirmation closes the document.

- [x] ~~**`system-core/master-relay.rs` deletion**~~ _(done in Group 2A, commit `dcb2700`)_

---

## Appendix — Key reference locations

| What | Where |
|---|---|
| Doctrine claims #33 + #34 | `~/Foundry/DOCTRINE.md` §II |
| Kernel binding + Mechanism A spec | `~/Foundry/conventions/system-substrate-doctrine.md` §3–§5 |
| WORM-ledger 4-layer stack | `~/Foundry/conventions/worm-ledger-design.md` §2 |
| Phase 2 NetBSD research | `.agent/RESEARCH-netbsd-veriexec-bootflow.md` |
| Phase 2 os-* candidate survey | `.agent/SURVEY-os-candidates-for-phase2.md` |
| v1.0.0 readiness detail (all sub-items) | `.agent/CHECKLIST-stage6-promotion-readiness.md` |
| Benchmark numbers | `.agent/BENCH-v0.2.0.md` |
| moonshot-toolkit architecture drift detail | `.agent/AUDIT-moonshot-toolkit-arch-vs-cli.md` |
| All 8 staged drafts | `.agent/drafts-outbound/` |
| Cross-cluster Cargo dep options | `.agent/STAGING-cargo-dep-options.md` |
| Project registry (all 97 rows) | `.claude/rules/project-registry.md` |
| Open questions log | `.claude/rules/cleanup-log.md` |
