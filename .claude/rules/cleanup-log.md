# Cleanup Log — pointsav-monorepo

Living record of in-flight cleanup work, open questions, and decisions made during active development. This file is read at session start and updated at session end when meaningful cleanup occurs. Maintained in-repo so the history travels with the code.

---

## How this file is maintained

- **Read at session start.** Claude Code reads this file at the start of every session (per the instruction in `CLAUDE.md`). The tables below reflect the current state of in-flight work. Apply the guidance before touching any related files.
- **Update at session end.** When a session includes meaningful cleanup — renames across multiple files, deprecated code removal, resolving an open question, surfacing a new one — append a dated entry to the top of the **Session entries** section at the bottom of this file.
- **Do not log trivial edits.** Single-file typo fixes, comment tweaks, or routine formatting changes do not belong here. This log is a record of decisions, not of every keystroke.
- **Commit each update with the code changes it describes.** The log and the work it documents travel together through git history.

---

## Interpreting build signals during cleanup

Until the workspace `Cargo.toml` is unified (see Layer 1 audit findings), `cargo build --workspace` and `cargo check` at the repo root only exercise the 8 declared members. The other ~70 crates are not covered by workspace-level commands. When making changes to any crate outside the declared members, run `cargo check` inside that crate's directory specifically. Do not rely on workspace-root build signals to confirm correctness across the full repo. This caveat lifts when the workspace is unified.

---

## Active legacy-to-canonical renames

These substitutions are known and in progress. Canonical names are from the Nomenclature Matrix. When the last occurrence of a legacy name is removed from the repo, move the row to the **Completed migrations** section with the date of completion.

| Legacy | Canonical | Status | Notes |
|---|---|---|---|
| `service-llm` | `service-slm` | Documentation-only inconsistency | Code references are correct. Legacy appearances in docs should be read as `service-slm`. |
| `cluster-totebox-real-property` | `cluster-totebox-property` | In flight | Appears in older deployment manifests and doc references. |
| `os-interface`, `os-integration` | `os-orchestration` | In flight | Legacy names predate the current three-layer stack nomenclature. |
| `RealPropertyArchive` | `PropertyArchive` | In flight | Appears in older archive-type documentation and possibly in legacy code comments. |

---

## Deprecations — flag and remove

Names no longer in use. Any occurrence in the repo should be flagged and removed. If a removal blocks something active, surface it — do not leave the legacy name in place silently.

| Name | Status | Notes |
|---|---|---|
| `fleet-command-authority` | Deprecated — remove | Node no longer in use. Should not appear in any current deployment manifest, build script, or documentation. |

---

## Intentional exceptions — do not migrate

Items that may look like candidates for cleanup but are intentionally preserved as-is. Do not "fix" these without confirmation.

| Item | Rationale |
|---|---|
| `cluster-totebox-personnel-1` and other numbered personnel instances | Exist locally but intentionally absent from GitHub and the MEMO. Not a naming error. Do not flag as legacy. |
| Two ConsoleOS operating patterns (multi-service `node-console-operator` and single-service nodes) | Both patterns are valid. The MEMO documents `node-console-operator` only, by design, to keep official documentation clean. Do not flag the single-service pattern as an inconsistency. |

---

## Open questions

Pending confirmations that affect how Claude should describe or reason about parts of the system. Do not invent values for these. If a task requires an answer, stop and surface the question.

| Question | Current handling |
|---|---|
| Verification Surveyor daily throttle number | Under operational review. Do not cite a specific number. Refer to it as "a system-enforced daily limit" until confirmed in a future MEMO version. **Code reference (2026-04-23):** `app-console-content/scripts/surveyor.py` hard-codes `MAX_DAILY_VERIFICATIONS = 10`; whether this value is authoritative or drift is the pending decision. |
| User Guide language on Sovereign Data Foundation | The User Guide contains language treating the Foundation as a current equity holder and active auditor. Requires a language review pass before any User Guide content is reused in public-facing materials. Flag any passage that describes the Foundation as current or active. |
| `service-search` inclusion in the next MEMO | Confirmed for inclusion in the next MEMO version. Treat as canonical in code; note the doc catch-up is pending. |
| Is the per-crate independent workspace pattern intentional (some crates meant to be extractable and published separately) or accidental drift? | Pending decision — do not act on related findings until answered. |
| Are `app-console-*` and `app-network-*` directories without `Cargo.toml` intentional scaffolding for planned work, or abandoned attempts? | Pending decision — do not act on related findings until answered. |
| Should the doubly-nested `service-email-egress-{ews,imap}` structure be flattened, or does the nesting reflect a real protocol-implementation hierarchy? | Pending decision — do not act on related findings until answered. |
| What is `discovery-queue` — runtime data that should be gitignored, reference data that belongs elsewhere, or a misplaced crate? | Pending decision — do not act on related findings until answered. |
| ~~Does `vendors-maxmind` (containing a GeoLite2 database, not code) belong as a `vendor-*` crate at all, or should it move to a non-workspace data directory?~~ | **Answered 2026-04-23:** non-workspace data directory. Moved to `app-mediakit-telemetry/assets/` (matching the authoritative target path already documented in the vendor's README). `vendor-*` crate framing rejected: the directory contained only data, no code. |

---

## Completed migrations

Migrations fully resolved in the repo. Moved here from **Active legacy-to-canonical renames** when the last occurrence of the legacy name is removed. Empty for now.

| Legacy | Canonical | Closed | Notes |
|---|---|---|---|
| `service-parser` | `service-extraction` | 2026-04-23 | Legacy-era scaffold containing only a README that described an AI-routing architecture since superseded by `service-extraction`'s deterministic Parser-Combinators approach. Zero runtime references, never a workspace member, one commit in history. No code or data to recycle into `service-extraction`; README deleted without migration. |
| `pointsav-pty-bridge` | `service-pty-bridge` | 2026-04-23 | Prefix-violation defect flagged in 2026-04-18 audit (brand prefix `pointsav-` not one of the seven canonical prefixes). Canonical target `service-pty-bridge` fits the daemon runtime role. Working Rust crate with one source file; directory renamed via `git mv`, `Cargo.toml` `name` field updated in the same commit. Not a workspace member, zero external import references, no callers needed updating. |
| `tool-cognitive-forge` + `service-slm/cognitive-forge` | `service-slm/router-trainer/` + `service-slm/router/` | 2026-04-23 | Closes the last rename-series item and removes the "Cognitive Forge" Do-Not-Use term in one commit. The Rust runtime sub-crate at `service-slm/cognitive-forge/` renamed to `service-slm/router/` (Cargo.toml `name` field + `main.rs` usage string updated). The Python distillation workflow at `tool-cognitive-forge/` moved in to `service-slm/router-trainer/`, joining the runtime as producer/consumer pair. Rationale for split naming: the runtime is a router (of messages to service handlers); the trainer distils knowledge to produce the routing model. Inside `router-trainer/`, `distill_knowledge.py` moved from a non-canonical `src/` into `scripts/` alongside `ignite_teacher.sh`. Three binary/log files untracked from Git and covered by new `.gitignore` patterns (still physically present at new paths for the Python workflow): 35 MB `engine/llamafile`, 22 KB `engine/engine.log`, 89 B `llama.log`. The 15 MB `engine/weights/qwen2.5-coder-1.5b.gguf` was already covered by the existing `**/weights/*` + `*.gguf` patterns — no new ignore needed. Git history retains all blobs; shrinking history is separate `git-filter-repo` work. Registry: `tool-cognitive-forge` row removed; Scaffold-coded 54 → 53, Total 98 → 97. `llama.log` surfaced earlier in this session is closed by this commit. |
| `vendors-maxmind` | `app-mediakit-telemetry/assets/` | 2026-04-23 | Not a rename but a reclassification: the `vendors-maxmind` directory was a data container holding `GeoLite2-City.mmdb` + READMEs, no code. The vendor's own README already named `app-mediakit-telemetry/assets/` as the intended location — the monorepo had never realised that path. Moved the `.mmdb` + READMEs into their documented target; deleted the empty `vendors-maxmind/` directory. Monorepo `README.md` line 151 and `USER_GUIDE_2026-03-30_V2.md` line 902 updated to the new path. `repo-layout.md` extended to name `assets/` as a conventional project subfolder. Python script reference in `app-mediakit-telemetry/scripts/generic-omni-matrix-engine.py` left unchanged — it reads a deployment-side path relative to CWD, not the monorepo-side path. Separate `.mmdb` → build-time-fetch task remains open under Structural defects. |

---

## Session entries

Newest on top. Append a dated block when a session includes meaningful cleanup work. Format:

```
## YYYY-MM-DD
- What changed (files touched, counts, rationale)
- What was left pending and why
- New open questions surfaced
```

---

## 2026-04-27 — Task `cluster/project-system` (Phase 1B moonshot-toolkit activation + CLI rewrite; system-core 1A.4 hotfix)

Continuous Phase 1B work across a token-budget gap. Six commits; two
clusters of related work combined into one entry.

- **`b809cbc`** (Peter Woodfine) — system-ledger: rustdoc on
  `apply_witness_record` naming the handover-height inclusion-proof
  verify semantics. Master's only concrete ask from the v0.1.26 reply:
  document which apex signs at handover height so consumers do not
  re-litigate the policy. No functional change; zero new tests.

- **`ba34cd8`** (Peter Woodfine) — moonshot-toolkit activation per
  framework §9 (Master Option A; Phase 1B greenlit). Files added:
  `CLAUDE.md`, `AGENTS.md`, `NEXT.md`, `ARCHITECTURE.md`, bilingual
  `README.md` + `README.es.md`, workspace-member entry in root
  `Cargo.toml`, registry row. Activated against the prior 14-line
  legacy stub `src/main.rs` and shell sketch `build-totebox.sh` —
  both preserved in place pending the CLI rewrite in subsequent
  commits.

- **`045e5cc`** (Jennifer Woodfine) — moonshot-toolkit: `src/spec.rs`
  SystemSpec + TOML parser. Microkit-equivalent system-description
  schema. Invariants enforced: PD count ≤ 63, channels-per-PD ≤ 63,
  no PD name collisions, channel endpoints reference declared PDs.
  12 tests; crate total 12.

- **`59d1fc0`** (Peter Woodfine) — moonshot-toolkit: `src/plan.rs`
  BuildPlan deterministic generator. Content-addressed inputs via
  SHA-256 hash per input file; BuildPlan body bytes canonical →
  plan_hash is deterministic across runs given same SystemSpec.
  10 tests; crate total 22.

- **`abef0e3`** (Peter Woodfine) — system-core: re-export
  `CheckpointInclusionError` at crate root to fix system-ledger build.
  Phase 1A.4 regression: system-ledger imports the type from
  `system_core` top-level but the `lib.rs` re-export was missing;
  HEAD was broken for commits `9b5e4fd`–`59d1fc0`. system-core
  0.1.3 → 0.1.4 PATCH. Cargo.lock also updated to HEAD's
  moonshot-toolkit 0.1.2 (lockfile-refresh oversight from `045e5cc`
  that silently persisted until caught here).

- **`af6073f`** (Peter Woodfine) — moonshot-toolkit: `src/main.rs`
  CLI rewrite (clap; `validate` / `plan` / `build` subcommands).
  `build` is a stub that prints `would run X` per step — Phase 1B
  v0.1.x scope by design; actual cross-compile is FUTURE task #14.
  8 inline tests using `tempfile`. Adds `clap = "4"` +
  `tempfile = "3"` dev-dependency. moonshot-toolkit 0.1.2 → 0.1.3.
  Total 30 tests in crate (22 lib + 8 main).

**Alternation drift observation.** The final three commits
(`59d1fc0`, `abef0e3`, `af6073f`) are consecutive Peter Woodfine
commits. This is identity-toggle drift from the workspace-wide
`~/Foundry/identity/.toggle` being modified by concurrent commits in
other clusters between this Task's commits. The toggle is intentionally
global per CLAUDE.md §8; concurrent-cluster activity between Task
commits can shift the expected alternation. Not a defect in this
cluster's work — an operational-substrate observation.

**Phase 1B v0.1.x scope closed.** validate/plan/build CLI working;
build is stub. Phase 1A is structurally complete on the v0.2.x scope:
system-core 0.1.4 + system-ledger 0.2.1 + moonshot-toolkit 0.1.3.

**Tasks deferred / explicit FUTURE.**
- Task #14 (actual seL4 cross-compile + QEMU AArch64 boot): FUTURE —
  requires decisions on cross-compile toolchain (Nix vs Bazel
  reproducible-build harness?), seL4 source vendoring strategy (git
  submodule vs Cargo build.rs fetch vs vendor-sel4-kernel snapshot?),
  and toolchain installation responsibility (operator vs Master vs
  Task). Decision points surfaced for Master direction.
- Tasks #22 (system-substrate hygiene) + #23 (system-security hygiene)
  remain deferred per Master's "natural-touch session" framing — neither
  was crossed by Phase 1B work.

**Cross-cluster Cargo dep visibility.** A writeup at
`/srv/foundry/clones/project-system/.claude/STAGING-cargo-dep-options.md`
covers four resolution options (Cargo patch, submodule, branch merge,
promote-then-consume). To be surfaced in the session-end outbox for
Master direction.

**Verification.** `cargo test -p moonshot-toolkit -p system-core -p system-ledger`: 30 + 35 + 44 = 109 tests passing, zero warnings.

## 2026-04-27 — Task `cluster/project-system` (Phase 1A.4 — Merkle inclusion proofs; v0.1.x → v0.2.x)

Phase 1A.4 closes the v0.1.x structural-completion gap Master
flagged: the substrate's apply-side no longer relies on consumer
good behaviour — Merkle inclusion proofs gate witness arrivals.
Three commits + cleanup-log update.

- **`9b5e4fd`** — system-core Phase 1A.4 increment: new
  `inclusion_proof.rs` module per RFC 9162 v2 (rfc9162_leaf_hash
  with 0x00 prefix; rfc9162_internal_hash with 0x01 prefix;
  InclusionProof + verify; 14 tests). Composed kernel-facing
  primitive `SignedCheckpoint::verify_inclusion_proof(proof,
  leaf_hash, signer_name, signer_pubkey)` per Master directive
  ("don't expose raw InclusionProof::verify as the kernel-facing
  API"; 5 tests). Re-exports at crate root. system-core 0.1.2 →
  0.1.3; 30 → 35 tests.
- **`2b9ca9c`** — system-ledger Phase 1A.4: LedgerConsumer trait
  signature change for `apply_witness_record` (now takes
  InclusionProof). InMemoryLedger.current_checkpoint field +
  set_current_checkpoint setter. witness_record_leaf_hash uses
  rfc9162_leaf_hash (matches Merkle leaf format). LedgerError
  additions: NoCurrentCheckpoint, WitnessNotInRoot(...),
  NoApexForCheckpoint. apply_witness_record_unchecked
  `#[cfg(test)]` shortcut for backward compat. 4 new lib tests
  + 1 migrated test. system-ledger 0.1.5 → **0.2.0** (MINOR bump:
  breaking trait-signature change).
- **`0d6da97`** — Phase 1A.4 benchmarks. 4 new criterion benches
  (raw inclusion-proof verify @ 8-leaf and 1024-leaf trees;
  composed verify_inclusion_proof @ 1024-leaf; full
  apply_witness_record path). Numbers run with VM under heavier
  load than the 1A.3 session; absolute values ~50-150% higher
  across the board, but the architectural shape holds:
  - Raw inclusion verify ~6-20 μs (Master expectation 10-100 μs
    confirmed)
  - Composed verify ≈ verify_signer + 0.4% inclusion overhead
    (verify-dominated as Master predicted)
  - apply_witness_record full path tracks composed verify cost
  - Cache + inclusion proofs are complementary, not redundant
  system-ledger 0.2.0 → 0.2.1 PATCH bump.

Phase 1A is now structurally complete on v0.1.x → v0.2.x scope:
- system-core (0.1.3): Capability + WitnessRecord + LedgerAnchor
  + C2SP signed-note + apex-cosigning predicate + Merkle
  inclusion proofs (35 tests)
- system-ledger (0.2.1): cache + revocation + apex history +
  ssh-keygen witness verify + LedgerConsumer trait + InMemoryLedger
  with proof-validated apply_witness_record (44 tests + 10
  benchmarks)
- End-to-end §4 N+3+ ceremony works: pre-handover P-old allows;
  handover with both sigs accepts; post-handover P-new-only
  accepts; post-handover P-old-only REFUSED with StaleApex
- Witness-record arrival is gated by Merkle inclusion proof
  against the current root — no more "trust shortcut"

Total Phase 1A.3 + 1A.4 commits: 11 (cdbed97 → 0d6da97 in this
session-block plus the prior-session cdbed97 + dcca616 + ae3dbb9
+ c378eaa + f614ca2 + b0dba5e + f35ebf7 + 9a361de). Total cluster
commits since session 1: 13.

Tasks #22 (system-substrate hygiene) + #23 (system-security
hygiene) remain deferred per Master ("natural-touch session").
Tasks #13 + #14 (moonshot-toolkit Phase 1B) remain deferred per
Master ("no urgency from operator").

## 2026-04-27 — Task `cluster/project-system` (Phase 1A.3 module impls + benchmarks)

Continuation of the 2026-04-27 entry below; this session landed
five more commits filling out the system-ledger crate after the
skeleton commit.

- **`ae3dbb9`** — cache.rs + revocation.rs implementations.
  CheckpointCache: LRU bounded, lookup by tree_size + by
  root_hash, eviction at capacity. RevocationSet: HashSet for O(1)
  membership + sidecar HashMap for audit detail; idempotent replay.
  12 tests (7 cache + 5 revocation).
- **`c378eaa`** — apex.rs (apex history + post-handover invariant).
  ApexEntry { name, pubkey, effective_from, effective_until };
  ApexHistory with record_genesis / apply_handover / current /
  check_height. ApexVerdict: NoApex / Single / Handover. Closes
  the apex-history half of inbox brief Phase 1A item 4. 10 apex
  tests (22 total).
- **`f614ca2`** — witness.rs (ssh-keygen -Y verify wrapper).
  Real shellout to /usr/bin/ssh-keygen via std::process::Command;
  tempfiles for signature + allowed_signers; payload to stdin.
  WITNESS_NAMESPACE = "capability-witness-v1" prevents
  cross-namespace replay against commit-signing or apprenticeship-
  verdict signatures. 5 tests including the cross-namespace
  rejection security property (27 total).
- **`b0dba5e`** — LedgerConsumer impl on InMemoryLedger +
  END-TO-END handover ceremony fixture. The `full_handover_
  ceremony_end_to_end` test asserts: pre-handover P-old
  checkpoint allows; revocation entry by P-old applies; handover
  with both P-old + P-new sigs accepts; post-handover P-new-only
  accepts; post-handover P-old-only REFUSED with StaleApex. The
  §4 N+3+ invariant works end-to-end. Also added
  `apply_witness_record` API + InMemoryLedger.witnessed HashSet
  for "is this witness logged in the ledger?" check (will be
  replaced with Merkle inclusion-proof check once system-core
  ships RFC 9162 proof machinery). 13 lib tests (40 total).
- **`f35ebf7`** — criterion benchmarks for Master 4b deliverable.
  `system-ledger/benches/consult.rs`. Six measurements (release
  profile; opt-level=z; x86 GCP n2-class hardware):

  | Benchmark | Median |
  |---|---|
  | Capability::hash | 5.0 μs |
  | SignedCheckpoint::verify_signer (1-sig) | 3.40 ms |
  | SignedCheckpoint::verify_apex_handover (2-sig) | 6.80 ms |
  | cache lookup hit (most-recent) | 8.08 ns |
  | cache lookup miss (full 64-entry scan) | 338 ns |
  | consult_capability (Allow path) | 3.39 ms |

  Cache hit is ~420,000× faster than ed25519 verify. Cache is
  architecturally critical. Full consult cost is dominated by
  apex-verify (orchestration overhead in measurement noise).
  ARM embedded targets will be 10-50× slower per the
  curve25519-dalek perf data — surface to Master in outbox.

- **Crate state:** system-ledger version 0.1.5, 40 unit/integration
  tests + 6 criterion benchmarks, cargo check + cargo test +
  cargo bench all clean, zero warnings.
- **Tasks deferred to natural-touch session:**
  - #22 system-substrate hygiene — Master suggested as natural
    neighbour; not touched this session because Phase 1A.3
    deliverables fit cleanly inside system-core + system-ledger
    only. Activate when seL4 CDT integration (Phase 4+) crosses
    into system-substrate territory.
  - #23 system-security hygiene — same; will activate naturally
    when capability-handshake / cryptographic-pairing work crosses
    Phase 1A's primitive surface.

## 2026-04-27 — Task `cluster/project-system` (Phase 1A increment 3 — system-ledger crate created)

- **Master directive received + actioned.** Master Claude reply
  archived 2026-04-27 (Option B resolution: kernel-side state
  machine in new `system-ledger` crate, not extension of
  `system-substrate`).
- **`system-core/ARCHITECTURE.md` updated** per Master directive
  (§3 rewritten from "Open architecture question" to "RESOLVED
  Option B"; §4 cross-references add system-ledger pointer; §1
  "what does NOT live here" reference clarified). Convention text
  stays as-written per Master's instruction. Cargo.lock catches
  up to system-core 0.1.2 (regression from c3766de).
- **`system-ledger` crate created and activated.** New workspace
  member at row 9 of `[workspace] members =`. Activated per
  framework §9 at creation: CLAUDE.md + AGENTS.md + NEXT.md +
  ARCHITECTURE.md + bilingual README pair. Registry row added
  Active from creation; Active count 5 → 6; Total rows 97 → 98.
- **Module skeletons in place.** Four module files created with
  type definitions but no functional impls:
  - `cache.rs` — `CheckpointCache` struct with `with_capacity` /
    `len` / `is_empty`; LRU implementation pending #18
  - `revocation.rs` — `RevocationSet` + `RevocationEvent` with
    `contains` / `len`; `apply_revocation` API pending #19
  - `apex.rs` — `ApexHistory` + `ApexEntry` with `current` / `len`;
    `apply_apex_handover` pending #11
  - `witness.rs` — `WITNESS_NAMESPACE` constant +
    `verify_witness_signature` stub returning `NotImplemented`;
    real shell-out lands per #12
  - `lib.rs` — `LedgerConsumer` trait + `Verdict` /
    `RefuseReason` / `ConsultError` / `LedgerError` enums +
    `InMemoryLedger` struct (impl pending #20)
- **Verification.** `cargo check -p system-ledger` passes; zero
  warnings on the new code. No tests yet (tests land alongside
  each module impl).
- **Dependency footprint.** Just `system-core` (workspace path
  dep). Zero external transitive crates beyond what system-core
  already pulls. Future deps as modules need them: tokio /
  std::process for witness.rs, criterion for benchmarks (#21),
  tempfile for witness tests (#12).



- **`system-core` Phase 1A increment 2 landed: C2SP signed-note
  checkpoint primitive.** New module `src/checkpoint.rs` (~290 LoC
  source + tests). Implements:
  - `Checkpoint { origin, tree_size, root_hash, extensions }` —
    body data per the C2SP signed-note spec, with `body_bytes()` /
    `parse_body()` for canonical-bytes round-trip.
  - `NoteSignature` — one signature line including the 4-byte
    key-hash prefix `SHA-256("<name>\nED25519\n<32-byte-pubkey>")[..4]`
    per spec, with `to_line()` / `parse_line()`.
  - `SignedCheckpoint` — body + ≥ 1 signature lines, with `to_wire()`
    / `parse()` for full wire format (em-dash signature lines, blank
    line separator).
  - `verify_signer()` — ed25519 verification of a specific signer's
    signature against the body via `ed25519-dalek`.
  - `verify_apex_handover()` — both-signatures-required predicate
    realising convention §4 ownership-transfer ceremony.
- **Dependencies added.** `ed25519-dalek = "2"` (no_std-capable;
  `std` feature kept default for v0.1.x), `base64 = "0.22"`. Total
  system-core deps: 5 (serde, serde_json, sha2, ed25519-dalek,
  base64). New transitive crates: ed25519, signature, subtle,
  zeroize, curve25519-dalek, semver, rustc_version.
- **Tests.** 10 new tests in `checkpoint::tests` covering body
  round-trip, key-hash determinism, key-hash sensitivity to name,
  single-sig wire round-trip, single-sig verification, wrong-pubkey
  rejection, multi-sig wire round-trip, apex-handover predicate
  positive case, apex-handover predicate negative case (only one
  signs), body-tampering rejection. Crate total: 16 tests, all
  passing on `cargo test -p system-core`.
- **What's still open downstream.** The state machine ("subsequent
  checkpoints require only P-new" — convention §4 height-N+3+
  invariant) is NOT in this commit. The cryptographic primitive that
  enables it lives here; the policy / cache that consumes it needs
  a home crate (architecture question still open in
  `system-core/ARCHITECTURE.md` §3).
- **Honest scope.** Merkle-log inclusion/consistency proofs (RFC
  9162 + C2SP tlog-tiles half of the capability ledger) are NOT in
  this commit either. Tracked in `system-core/NEXT.md` queue.
- **Verification.** `cargo check -p system-core` and
  `cargo test -p system-core` both pass on Rust stable. Zero
  warnings on the new module.
- **Version bump.** `system-core/Cargo.toml` 0.1.1 → 0.1.2.

## 2026-04-26 — Task `cluster/project-system`

- **`system-core` activated; Phase 1A increment 1 landed.** Per the
  cluster brief at
  `~/Foundry/clones/project-system/.claude/inbox.md` (Phase 0
  hygiene interleaved with Phase 1A). Files added to `system-core/`:
  `CLAUDE.md`, `AGENTS.md`, `NEXT.md`, `ARCHITECTURE.md` (per
  framework §9 Tier 2 + ARCHITECTURE.md skeleton citing claims
  #33/#34 + `system-substrate-doctrine.md` §3.1+§5). Files modified:
  `Cargo.toml` (added `serde`/`serde_json`/`sha2` deps),
  `src/lib.rs` (replaced 1-function scaffold with `Capability` +
  `WitnessRecord` + `LedgerAnchor` + `CapabilityType` + `Right`
  types per convention §5.1 schema; six unit tests covering
  serialisation round-trips and hash determinism), `README.md` +
  `README.es.md` (stripped "Pending Engineering Cycle" placeholder
  language; bilingual pair updated to Active state).
- **Workspace `[members]` updated.** `system-core` added to monorepo
  `Cargo.toml` `[workspace] members =`. Total members: 8 → 9. List
  of canonical/in-workspace `system-*` projects: 2 → 3 (joins
  `system-gateway-mba` + `system-security`). 11 system-* and 9
  moonshot-* remain to be added across future Phase 0 / Phase 1
  increments.
- **Registry updated.** `system-core` row Scaffold-coded → Active;
  Active count 4 → 5; Scaffold-coded 53 → 52; Total 97 unchanged.
- **Open architecture question surfaced.** Where does the kernel-side
  ledger-consultation logic live: extension of `system-substrate` or
  a new `system-capability-ledger` / `system-ledger` crate?
  Documented in `system-core/ARCHITECTURE.md` §3; will resolve in the
  next Phase 1A increment (consultation simulator). Not blocking.
- **Residual sketch left in place.** `system-core/master-relay.rs`
  shells out to nonexistent `/bin/service-*` binaries and predates
  this cluster. Not a binary target, not in `Cargo.toml [[bin]]`.
  Tracked in `system-core/NEXT.md` deferred queue and surfaced as a
  per-project defect-closure pass that should audit all top-level
  `*.rs` files in projects against `repo-layout.md`. Not blocking
  Phase 1A.
- **Verification.** `cargo check -p system-core` and
  `cargo test -p system-core` both pass on Rust stable in workspace
  context. Six tests; zero warnings on the new code.

---

## 2026-04-23

- **Repo-layout rule introduced.** Added
  `.claude/rules/repo-layout.md` codifying the allowed file set at
  the monorepo root and at each project directory root, and naming
  the sibling repos where cross-cutting content belongs (user guides,
  ADRs, design-system material). Anchor for the file-relocation work
  queued behind it (see `NEXT.md`).
- **Defects surfaced at root by this rule** — staged for separate
  commits, not moved in this session:
  - ~~`force_build.sh` (tracked, at repo root) → queued move to
    `vendor-sel4-kernel/scripts/`~~ **Closed 2026-04-23** — moved
    via `git mv` in a follow-up commit within this session. Zero
    runtime callers; script body uses absolute paths so no content
    edits required.
  - `GUIDE-OPERATIONS.md` (tracked, at repo root) → queued move to
    `content-wiki-documentation/`.
  - `USER_GUIDE_2026-03-30_V2.md` (tracked, at repo root) → queued
    move to `content-wiki-documentation/` with `_V2` dropped, per
    CLAUDE.md §6 edit-in-place rule.
  - ~~`app-console-content/src/{pointsav-surveyor.sh,surveyor.py}` →
    queued move to `app-console-content/scripts/`~~ **Closed
    2026-04-23** — both files moved via `git mv` (recognised as
    100% renames). Shell wrapper uses `$(dirname "$0")/surveyor.py`
    (relative) so the pair moves together without edits. Python
    script uses absolute paths into `woodfine-fleet-deployment` so
    location-independent. Zero intra-repo runtime callers; no cron
    entries found. The clone at `~/Foundry/clones/service-slm/`
    retains its copy on branch `cluster/service-slm` (separate
    `.git/`) and is unaffected by this move on `main`; it will
    receive the change only when that branch merges.
  - ~~`os-infrastructure/build_iso/forge_iso.sh` → queued rename to
    `os-infrastructure/build_iso/compile_binary.sh`~~ **Closed
    2026-04-23** — renamed via `git mv`; in-file header comment
    updated to reflect the new name and record the rename
    rationale. Zero external callers.
- ~~**Project-root scripts flagged (not yet moved):** ~15 scripts sit
  at project root instead of under `scripts/` across `service-vpn`
  (5 generator scripts), `service-email` (`spool-daemon.sh`),
  `service-slm` (`cognitive-bridge.sh`), `service-content`
  (`forge-seeds.sh`), `os-network-admin` (2 scripts),
  `os-totebox` (1), `tool-cognitive-forge` (1),
  `vendor-phi3-mini` (2), `app-mediakit-telemetry` (5 generic
  scaffold scripts). Each project is a separate closure task.~~
  **Closed 2026-04-23** — all 9 projects relocated in 9 separate
  `git mv` commits (18 files total, every one a 100% rename).
  Commit chain: `8f5cc48` os-totebox → `2456ea6` service-content
  → `30ff629` service-email → `cda2ce5` service-slm → `654d255`
  tool-cognitive-forge → `503f922` os-network-admin → `6df4be0`
  vendor-phi3-mini → `6f95279` service-vpn → `faae141`
  app-mediakit-telemetry. No callers needed updating; the only
  in-script references found were self-usage strings that remain
  valid after the move.
- **Stray runtime log surfaced.** `tool-cognitive-forge/llama.log`
  at project root — runtime log, almost certainly should be
  gitignored (and removed from tracking if tracked). Not addressed
  in this session. Added to `NEXT.md` as a separate item.
- **First rename-series closure: `service-parser` removed.**
  `service-parser/` directory deleted (`git rm -r`); contained
  only a README describing an abandoned AI-routing framing — no
  code, no data, no subdirectories. Zero runtime references
  anywhere in the repo. Rename-table row moved to Completed
  migrations; registry row removed; registry Defect count updated
  from 5 to 4 and Total rows from 100 to 99.
- **Second rename-series closure: `pointsav-pty-bridge` →
  `service-pty-bridge`.** Directory renamed via `git mv` (four
  100% renames: `.gitignore`, `Cargo.toml`, `Cargo.lock`,
  `src/main.rs`); `target/` left in place because it is gitignored
  build output. `Cargo.toml` `name` field updated in the same
  commit. Registry row moved from "Other / special" to the
  Service section, alphabetically between `service-people` and
  `service-search`, reclassified Defect → Scaffold-coded. Summary
  counters: Defect 4 → 3, Scaffold-coded 51 → 52, Total stays 99.
  Zero external Rust imports, no callers needed updating; not a
  workspace member. Stray `Cargo.lock` inside the renamed
  directory remains — resolves with workspace `Cargo.toml`
  unification (separate open structural defect).
- **Handoffs-outbound entries made self-executing.** Each outbox
  entry now carries a "Prescriptive actions" subsection with the
  exact commands a destination Root Claude can run mechanically —
  `cp` commands from source absolute path, `git add`, commit
  message, any in-transit edits, and the completion-signal commit
  pattern. Header also describes the convention so future outboxes
  follow the same shape. Two existing entries for
  `GUIDE-OPERATIONS.md` and `USER_GUIDE_2026-03-30_V2.md` updated
  with their prescriptive actions. This lets a cold-start Root
  Claude session in `content-wiki-documentation/` execute the
  handoffs without reading anything from this session's context.
- **Fifth (final) rename-series closure: Cognitive Forge term
  retired.** `service-slm/cognitive-forge/` renamed to
  `service-slm/router/`; former top-level `tool-cognitive-forge/`
  moved in as `service-slm/router-trainer/`. Producer/consumer
  now live together under `service-slm`. Rust Cargo.toml `name`
  field + `main.rs` usage string updated. Python
  `distill_knowledge.py` relocated from non-canonical `src/` to
  `scripts/` alongside `ignite_teacher.sh`. Three binary/log
  files stopped being tracked (`llamafile` 35 MB, `engine.log`,
  `llama.log`) via `git rm --cached` + new `.gitignore` section;
  physical files remain at new paths so the Python workflow still
  finds them. The 15 MB `qwen2.5-coder-1.5b.gguf` under `weights/`
  was already ignored. Registry Scaffold-coded 54 → 53, Total
  98 → 97 (one top-level project absorbed into `service-slm`).
  This closes the rename-series queue (5 of 5 done) and the
  separate `llama.log` stray item surfaced earlier in this
  session.
- **Fourth rename-series closure: `service-email-egress-{ews,imap}`
  wrappers flattened; consolidation plan reversed.** After
  reviewing sub-crate contents, EWS and IMAP are two
  protocol-specific adapters — not duplicates. Shared sub-crates:
  `egress-ingress`, `egress-ledger`, `egress-roster`,
  `data-ledgers/`. Protocol-specific: `egress-archive-ews` /
  `egress-archive-imap`; EWS-only: `egress-prune`,
  `egress-balancer`. Merging them would erase that architectural
  distinction. Instead, flattened the redundant
  `service-email-egress-ews/service-email-egress-ews/` wrapper
  (and the imap equivalent) — 73 files promoted up one level.
  Relative `../data-ledgers/` paths in Rust sources remain valid
  because crate dirs and `data-ledgers/` both moved together.
  Registry reclassified both from Defect → Scaffold-coded;
  Defect count 2 → 0 (registry is now Defect-free); Scaffold-coded
  52 → 54. The 13 dir-name / Cargo-name mismatches the 2026-04-18
  audit flagged (e.g., dir `egress-ingress` containing
  `Cargo.toml` with `name = "service-email-batch-ingress"`) are
  unaddressed and remain as a separate audit finding.
- **Third rename-series closure: `vendors-maxmind` reclassified
  to `app-mediakit-telemetry/assets/`.** Not a rename but a
  data-reclass: the directory held only the 63.5 MB
  `GeoLite2-City.mmdb` + READMEs with no code. The vendor's own
  README already named `app-mediakit-telemetry/assets/` as the
  intended target path — the monorepo had never realised that
  path. Moved the `.mmdb` + both READMEs into the documented
  target; removed `vendors-maxmind/.keep`; empty directory
  auto-removed by git. Closed the related "does it belong as a
  `vendor-*` crate at all?" open question (answer: no;
  non-workspace data directory). Updated monorepo `README.md`
  line 151 and `USER_GUIDE_2026-03-30_V2.md` line 902 (in-transit
  edit travels with the cross-repo handoff). Extended
  `repo-layout.md` to name `assets/` and `data/` as conventional
  project subfolders. Registry row removed; Defect 3 → 2, Total
  rows 99 → 98. Python script reference in
  `app-mediakit-telemetry/scripts/generic-omni-matrix-engine.py`
  left unchanged (it refers to deployment-side path relative to
  CWD — independent of monorepo-side layout). Separate `.mmdb` →
  build-time-fetch task remains open under Structural defects.
- **Open question surfaced.** `surveyor.py` hard-codes
  `MAX_DAILY_VERIFICATIONS = 10`. The existing cleanup-log open
  question — "Verification Surveyor daily throttle number — Under
  operational review. Do not cite a specific number" — must
  reconcile: either the code is authoritative (close the question,
  value is 10) or the doc is authoritative (the code is out of step
  and needs updating). Do not cite the number externally until
  resolved.
- **Second open question surfaced (os-infrastructure build
  pipeline).** The two scripts `os-infrastructure/forge_iso.sh`
  (ISO assembly) and `os-infrastructure/build_iso/compile_binary.sh`
  (binary compile, renamed this session) are sequential build
  stages but are not wired together — the assembly script does not
  invoke the compile script, and there is no Makefile or top-level
  driver. Operator must run them manually in order. Is this
  intentional (operator-gated two-step) or drift (should become a
  single driver script)? Pending decision before next pipeline
  refactor.
- **Handoff-outbound pattern piloted.** Added
  `.claude/rules/handoffs-outbound.md` as a cross-repo file-move
  outbox. Two entries lodged: `GUIDE-OPERATIONS.md` and
  `USER_GUIDE_2026-03-30_V2.md` both → `content-wiki-documentation`.
  Both files remain in place in this repo until a Root Claude in
  the destination repo commits the add-side; only then does a
  follow-up Root Claude session here commit the source-remove.
  The pattern is passive — an outbox entry waits for pickup.
- **Surfaced for Master Claude** (workspace-scope changes, outside
  Root Claude's write lane per §9):
  1. Formalise the cross-repo handoff pattern as an addendum in
     `~/Foundry/CLAUDE.md` §9. Current §9 stops at clone
     provisioning; the handoff mechanic is the natural extension
     for file movement between engineering repos.
  2. Extend `~/Foundry/CLAUDE.md` §10's `.claude/rules/` canonical
     list from three files to four — add `handoffs-outbound.md`
     alongside `repo-layout.md`, `project-registry.md`, and
     `cleanup-log.md`.
  3. Propagate both the `repo-layout.md` rule (§10 already names
     the monorepo as reference implementation) and the new
     `handoffs-outbound.md` pattern to the other engineering repos
     over time. Order of propagation is `~/Foundry/NEXT.md`'s
     concern.

---

## 2026-04-22

- **Project framework bootstrap.** Added `.claude/rules/project-registry.md`
  with 100-row inventory of every top-level directory, classified by
  state per `~/Foundry/CLAUDE.md` §8 (Reserved-folder /
  Scaffold-coded / Active / Defect / Not-a-project). Framework docs,
  templates, and activation procedure live workspace-level. This
  cleanup-log was also introduced onto `main` today (previously
  present only on feature branches — drift closed).
- **Taxonomy expanded to seven domains.** Added `app-orchestration-*`
  to the in-force `app-[os]-*` list in
  `~/Foundry/IT_SUPPORT_Nomenclature_Matrix_V8.md` §3. Triggered by
  `app-orchestration-bim` appearing during the session — would have
  been an unmatched-prefix defect under the original six-domain
  rule. Now conformant; `os-orchestration` already exists as a
  Systemic Wordmark (§2).
- **Four BIM-research directories registered.** `app-console-bim`,
  `app-orchestration-bim`, `app-workplace-bim`, `service-bim` — each
  with a single `RESEARCH.md`. Classified as Reserved-folder pending
  decision to activate.
- **Audit cleanup.** Removed 2 `__MACOSX/` directories and 16
  tracked `.DS_Store` / AppleDouble files from extraction-artefact
  scaffolding in the egress crates. Added `.DS_Store` to
  `.gitignore`.

---

## 2026-04-18 — Layer 1 structural audit — findings

- **Headline finding:** Workspace `Cargo.toml` declares only 8 of ~70+ crates as members. Everything else is treated as standalone workspaces, which explains the 23 stray `Cargo.lock` files scattered through the repo. `cargo build --workspace` will skip almost everything; profile/edition inheritance is not reaching most crates.
- **Severity counts:** 1 Critical, 1 High, 4 Medium, 1 Low.
  - Critical: workspace under-declaration (8 of ~70+ crates).
  - High: 23 stray `Cargo.lock` files inside member crates.
  - Medium: prefix violations (2); dir-name vs `Cargo.toml` name mismatches (13); doubly-nested `service-email-egress-{ews,imap}` scaffolding; many `app-console-*` / `app-network-*` directories without `Cargo.toml`.
  - Low: `discovery-queue` orphan data directory at root.
- **Good news on prefix adherence:** across ~85 directories, adherence to the seven canonical prefixes is approximately 97.6%. Only two violations found: `pointsav-pty-bridge` (no recognized prefix) and `vendors-maxmind` (plural form instead of canonical `vendor-`).
- **Nested redundancy:** `service-email-egress-ews` and `service-email-egress-imap` both contain a redundant intermediate directory of the same name — a doubly-nested copy-paste scaffolding pattern producing depth-3 crates. All 13 directory-name / `Cargo.toml`-name mismatches are concentrated in these nested egress areas (short dir names like `egress-ingress` aliasing qualified crate names like `service-email-batch-ingress`).
- **No modifications were made in this session — audit only.**
- **Next:** Open Questions section of this log to be updated separately with five new questions raised by the audit.

---

## 2026-04-18

- Initialized this cleanup log. Seeded active renames, deprecations, intentional exceptions, and open questions from Section 13 of the PointSav Project Instructions.
- Established the session-start / session-end read-and-update pattern in CLAUDE.md.
- No code changes in this session. Next session should confirm the active renames table against a fresh grep of the repo to establish a baseline count of remaining occurrences per legacy term.
- Open question surfaced: whether the `service-parser` / `service-extraction` consolidation is scoped for a specific MEMO version or tracked informally. Answer will determine how we prioritize closing that migration.
