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

## 2026-05-27 — v1.0.0 version bumps + PhD thesis BRIEF

Two commits to project-system cluster branch.

**PhD thesis BRIEF (`edd4928`, Jennifer Woodfine)**
- `.agent/briefs/BRIEF-substrate-phd-thesis-2026-05-27.md` (719 lines) created.
- Yale PhD thesis-quality research document: system-* layer, service-fs,
  seL4/NetBSD two-bottom architecture, Totebox Archive transferability.
- Schema: `foundry-draft-v1`, `language_protocol: PROSE-RESEARCH`,
  `audience: academic`, `bcsc_class: no-disclosure-implication`.
- 12 Opus research agents used for source material gathering.
- Pre-publication checklist in notes_for_editor: bench #9 quiet-VM re-run;
  Group 3A AArch64 decisions; citation promotion; language pass; ES panorama.

**v1.0.0 version bumps (`c2ae1e9`, Jennifer Woodfine)**
- `system-core/Cargo.toml`: 0.2.0 → 1.0.0 (API stable; leaf crate).
- `system-ledger/Cargo.toml`: 0.2.1 → 1.0.0 (API stable; depends on system-core path).
- `system-core/CLAUDE.md`: version header updated to 1.0.0; last-updated 2026-05-27.
- `system-ledger/CLAUDE.md`: version header updated to 1.0.0; test count 44→47,
  bench count 10→12; last-updated 2026-05-27.
- `system-core/CHANGELOG.md` created: 4 entries covering v1.0.0/0.2.0/0.1.21/0.1.4/0.1.1.
- `system-ledger/CHANGELOG.md` created: 4 entries covering v1.0.0/0.2.1/0.2.0/0.1.21.
- Outbox msg `project-system-20260527-stage6-v100` written to Command Session.
- 62 + 47 tests passing; zero warnings.

**Open questions surfaced this session:**
- `pointsav-fleet-deployment` working tree has file-mode drift (32 files 644→755,
  likely extraction artefact) + `.claude/rules/project-registry.md` appears deleted
  (structural: `.claude/` is now a symlink to `.agent/`). Neither committed.
  Needs Command Session review.
- `pointsav-monorepo` sub-clone is on `main` branch (switched to `cluster/project-system`
  this session, but the system-* code lives in the project-system archive directly).

## 2026-05-21 — Groups 6 + 7 — Stage-6 prep + WFD housekeeping

Five commits to project-system cluster branch, one commit to WFD sub-repo.

**Consistency-proof bench fix + BENCHMARKS.md (`d2f6a5a`, Jennifer Woodfine)**
- `system-ledger/benches/consult.rs` benches 11–12 were broken: proof was
  constructed with 1 hash (MTH of right subtree per RFC generation algorithm)
  but the accumulator verifier requires 4 hashes for 4→8: anchor=leaves[3],
  two BOTH-branch siblings (leaves[2], internal(l0,l1)), one ELSE-branch
  sibling (right-half root). Fixed PathTooShort panic.
- BENCHMARKS.md extended from 10 to 12 entries. Bench 11: 10.86 µs [10.73,
  11.00]. Bench 12: 8.37 ms [8.22, 8.53]. New consistency-proof architectural
  observation: composed cost ≈ 2× verify_signer (two Ed25519 ops); raw proof
  is a trivial fraction.

**Cargo.toml metadata + ARCHITECTURE.md corrections (`6e25c46`, Jennifer Woodfine)**
- `license = "AGPL-3.0-or-later"` added to system-core, system-ledger,
  moonshot-toolkit Cargo.toml — resolved from LICENSE-MATRIX.md §4.2
  (system-* and moonshot-* prefix categories both AGPL-3.0-or-later).
- `description`, `repository`, `keywords`, `categories`, `rust-version` filled
  on all three crates. MSRV: system-core/system-ledger 1.73 (div_ceil);
  moonshot-toolkit 1.74 (clap 4.5+).
- system-core/ARCHITECTURE.md: §3 system-ledger ref corrected (44→47 tests,
  10→12 benches); §5 test count 51→62; test lists extended for Group 2A/B
  additions; new §5 "Platform requirements and no_std roadmap" declaring MSRV
  and documenting the planned no_std carve-out MINOR.
- Outbox to Command Session: 4 Group 3B gate decisions needed for v1.0.0
  (LedgerConsumer API finality, promote strategy, attribution, quiet-VM bench).

**STAGING-cargo-dep-options.md resolution note (`bb5d415`, Jennifer Woodfine)**
- Added header resolving to Option E: block on Stage-6. system-core v0.2.0 is
  API-stable; no Cargo [patch] bridging required.

**WFD sub-repo housekeeping (`9ba968d`, Peter Woodfine, WFD branch)**
- Created `woodfine-fleet-deployment/CLAUDE.md` — was missing; the only
  required root file absent per repo-layout.md.
- Added `fleet-infrastructure-leased/README.es.md` and
  `node-console-operator/README.es.md` — bilingual audit found these two gaps;
  all 16 present deployment directories now have paired READMEs.
- `NEXT.md` updated: 3 prior open items closed; new open question surfaced —
  `gateway-orchestration-gis-1` and `gateway-knowledge-documentation-1` in the
  registry but absent from HEAD 7fdf36b (possible filter-repo removal).

**Open questions surfaced this session:**
- WFD registry drift: `gateway-orchestration-gis-1` and
  `gateway-knowledge-documentation-1` listed as Scaffold-coded but absent from
  `cluster/project-system` HEAD. Needs reconciliation against WFD `main`.
- Quiet-VM bench re-run for bench #9 still pending (load avg < 1.0 required).

## 2026-05-20 — Group 2 mechanical hygiene — system-core, system-ledger, moonshot-toolkit

Closed all 6 sub-groups of the Group 2 plan (project-system-todo.md). Six commits.

**Group 2A — system-core rustdoc + doc updates (`dcb2700`, Peter Woodfine)**
- Added per-variant rustdoc to `CapabilityType` (5 variants) and `Right` (5 variants)
  in `src/lib.rs`; field docs on `Capability`, `WitnessRecord`; `/// # Examples`
  block on `Capability::hash()`.
- 4 new tests: `capability_hash_expiry_none_vs_some`,
  `capability_hash_changes_with_witness_pubkey`, `right_variants_round_trip`,
  `capability_type_variants_round_trip`. Total: 62 tests (was 51+1 doctest = 52).
- `system-core/ARCHITECTURE.md` §3 resolved to IMPLEMENTED; §5 updated to 62 tests.
- `system-core/NEXT.md` fully rewritten to reflect v0.2.0 structurally complete state.
- `system-core/CLAUDE.md` updated: current state, test count, file layout.
- `system-core/master-relay.rs` deleted (`git rm`; legacy stub with hardcoded
  nonexistent `/bin/service-*` paths, never a `[[bin]]` target).

**Group 2B — 11 new negative-path tests in system-core (`334462b`, Peter Woodfine)**
- `checkpoint.rs`: 7 tests covering `ParseError` variants (NotUtf8, Truncated,
  MissingNewline, BadRootHashLength, MissingSignatureSeparator), `VerifyError::BadPublicKey`
  (y=2 is a quadratic non-residue on Ed25519; smallest non-curve point per Legendre
  symbol computation), `consistency_proof_new_signature_invalid_rejects`.
- `lib.rs`: 4 tests covering `capability_hash_expiry_none_vs_some`,
  `capability_hash_changes_with_witness_pubkey`, round-trip serialisation variants.
- Ed25519 non-curve point: `[0u8; 32]` (y=0) IS accepted by ed25519-dalek v2.2.0
  `from_bytes` (4-torsion, not rejected). Used `bad_pubkey[0] = 2` (y=2, QNR mod p).

**Group 2C — system-ledger doc updates + BENCHMARKS.md (`0881091`, Jennifer Woodfine)**
- `system-ledger/CLAUDE.md`, `NEXT.md`, `ARCHITECTURE.md` all updated from skeleton
  language to v0.2.1 fully-implemented state.
- `system-ledger/BENCHMARKS.md` created: 10 criterion benchmark results from
  `BENCH-v0.2.0.md`, run conditions, architectural observations.

**Group 2D — 3 new gap tests in system-ledger (`cb935f9`, Peter Woodfine)**
- `consult_with_bad_apex_pubkey_returns_inconsistent_state`: bad_pk[0]=2 (non-curve)
  → `ConsultError::InconsistentState`.
- `apply_witness_record_no_apex_returns_no_apex_for_checkpoint`: no genesis →
  `LedgerError::NoApexForCheckpoint`.
- `apply_witness_record_at_handover_height_succeeds`: 2-leaf Merkle tree, tree_size
  matches proof (must match for verify_inclusion_proof), handover path → Ok.
  Bug during development: original test used tree_size=50 but proof covered 2 leaves
  → `TreeSizeMismatch`. Fixed by setting checkpoint tree_size=2.
- Total: 47 tests (was 44).

**Group 2E — moonshot-toolkit ARCHITECTURE.md drift audit (`no-commit` — already applied)**
- Read `AUDIT-moonshot-toolkit-arch-vs-cli.md` (9 proposed edits) against current
  `ARCHITECTURE.md`. All 9 edits already applied in a prior session.
  No code changes needed; confirmed complete.

**Group 2F — clippy/fmt/rustdoc CI pass (`54fb7e7`, Jennifer Woodfine)**
- `cargo fmt`: fixed 5 diffs system-core, 9 diffs system-ledger, 4 diffs moonshot-toolkit.
- `cargo clippy -D warnings`: fixed `push_str("…")` → `push('…')` in moonshot-toolkit.
- `cargo doc --no-deps`: fixed 7 broken intra-doc links (3 system-core, 3 system-ledger,
  1 moonshot-toolkit) and 1 bare URL in system-core/checkpoint.rs.
- Final state: 0 warnings across all three crates for clippy, fmt, doc.
  139 tests passing (62 + 47 + 30).

> **Archived entries:** session logs before this point are in `cleanup-log-archive.md`.
