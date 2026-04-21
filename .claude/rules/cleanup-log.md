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
| `service-parser` | `service-extraction` | In flight — both names present | Working name still used in portions of the active codebase. Canonical name is the long-term target. Consolidation planned for a future release. |
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
| Verification Surveyor daily throttle number | Under operational review. Do not cite a specific number. Refer to it as "a system-enforced daily limit" until confirmed in a future MEMO version. |
| User Guide language on Sovereign Data Foundation | The User Guide contains language treating the Foundation as a current equity holder and active auditor. Requires a language review pass before any User Guide content is reused in public-facing materials. Flag any passage that describes the Foundation as current or active. |
| `service-search` inclusion in the next MEMO | Confirmed for inclusion in the next MEMO version. Treat as canonical in code; note the doc catch-up is pending. |
| Is the per-crate independent workspace pattern intentional (some crates meant to be extractable and published separately) or accidental drift? | Pending decision — do not act on related findings until answered. |
| Are `app-console-*` and `app-network-*` directories without `Cargo.toml` intentional scaffolding for planned work, or abandoned attempts? | Pending decision — do not act on related findings until answered. |
| Should the doubly-nested `service-email-egress-{ews,imap}` structure be flattened, or does the nesting reflect a real protocol-implementation hierarchy? | Pending decision — do not act on related findings until answered. |
| What is `discovery-queue` — runtime data that should be gitignored, reference data that belongs elsewhere, or a misplaced crate? | Pending decision — do not act on related findings until answered. |
| Does `vendors-maxmind` (containing a GeoLite2 database, not code) belong as a `vendor-*` crate at all, or should it move to a non-workspace data directory? | Pending decision — do not act on related findings until answered. |

---

## Completed migrations

Migrations fully resolved in the repo. Moved here from **Active legacy-to-canonical renames** when the last occurrence of the legacy name is removed. Empty for now.

*(none yet — move rows here as migrations close out)*

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

## 2026-04-20 — service-slm: slm-inference-remote boot+ledger lands (task [20])

- First real implementation in `slm-inference-remote`. Crate moves scaffold → alpha. `RemoteInferenceClient::boot(&mut LedgerWriter)` probes `/healthz` and emits `BOOT_REQUEST` + `BOOT_COMPLETE` rows with `completion_status` / `error_code` populated correctly on success and on failure. Two wiremock-driven integration tests cover happy path (200 + `node_id`) and transport failure (503 → `error_code=HTTP_503`). Scaffold placeholder removed from `lib.rs`; `tests/smoke.rs` deleted. Error surface is `RemoteInferenceError` with a `ledger_code()` helper producing stable strings.
- **MSRV trap resolved:** `reqwest` pulls `url → idna → idna_adapter → icu_normalizer`; `idna_adapter 1.2+` routes to `icu_normalizer 2.2+` which requires Rust 1.86 (workspace MSRV is 1.85). Separately, `wiremock 0.6.3+` uses let-chains (Rust 1.88). Fixed with four precise lockfile pins (`url=2.5.4`, `idna=1.0.3`, `idna_adapter=1.1.0`, `wiremock=0.6.2`) **plus** three `cargo-deny` ban rules with version ranges (`idna_adapter:>=1.2.0`, `icu_collections:>=2.2.0`, `icu_normalizer:>=2.2.0`) so that `cargo update` cannot walk the tree forward and silently re-trigger the trap. The `wiremock` pin is exact-pinned with `=0.6.2` in `[workspace.dependencies]`. Rules reference each other in the comments so future maintainers find the full context from any one entry point. Precedent: task [15] `validator` removal (same MSRV chain, different surface).
- **`cargo-deny` wired into CI for the first time.** New `deny` job in `.github/workflows/ci.yml` uses `taiki-e/install-action@v2` to pull `cargo-deny@0.18.3` (0.19.x requires Rust 1.88). Runs `cargo deny check bans sources` only. `deny.toml` gained `allow-wildcard-paths = true` under `[bans]` — without it, workspace path deps (`slm-cli → slm-api`, etc.) tripped the external-wildcard ban. `scripts/check-all.sh` aligned to match CI (was running all subcommands and would have failed on the two deferred checks).
- **Two `cargo-deny` checks intentionally deferred**, tracked as explicit follow-up tasks:
  - **[30]** — `licenses` subcommand fails because 10 workspace crates declare `license = "AGPL-3.0-only"` (ADR-0003) but `deny.toml [licenses].exceptions` is empty. Fix is to add a bounded exception row per workspace crate. Preserves the SLM-STACK §7 "We Own It" discipline for all non-workspace deps.
  - **[33]** — `advisories` subcommand fails because `cargo-deny 0.18.3` cannot parse CVSS-4.0 vector strings present in recent RUSTSEC advisories. Fixed in `0.19.x` but that requires Rust 1.88. Lifts naturally on the next MSRV bump.
- **Two follow-up work items surfaced and added** to `TASKS.md` for the crate's next-wave work: **[25]** retry + exponential-backoff policy (crate CLAUDE.md invariant 3) and **[26]** `JOB_*` / `TEARDOWN_*` / `PREEMPTION` events (remaining YOYO-COMPUTE §5 event types; `TEARDOWN_COMPLETE` also needs Cloud Run billing-API integration for `gpu_seconds` + `cost_usd`).
- No monorepo-wide renames or deprecations touched.
- **Hand-off for next session** (session ended mid-branch, not yet merged):
  - Branch: `audit-layer-1-findings`, one commit ahead of previous tip (`08dab64 feat(slm-inference-remote): boot probe + BOOT_* ledger events`). Not pushed.
  - Natural next task on resumption is **[25]** (retry/backoff) or **[26]** (`JOB_*` / `TEARDOWN_*` / `PREEMPTION` events). Both are p2 and either is a reasonable continuation. `/next-task` should pick one automatically.
  - **Uncommitted working-tree noise to disregard** (not part of task [20]; appeared during the session from elsewhere): `LICENCE_Research_Report.md` deletion + `LICENCE_Research_Report_2026-04-20.md` at monorepo root; `app-workplace-presentation/` modifications (`NEXT.md`, `src/index.html`, `src/js/editor.js`, `src/js/schema.js`, `src/js/slides.js`, `src/styles/app.css`); `service-slm/.claude/scheduled_tasks.lock` (transient). Next session must not conflate these with task [20] or roll them into a task-scoped commit.
  - **Unresolved from 2026-04-20 slm-core session**: the proposed `#![cfg_attr(test, allow(clippy::disallowed_methods))]` repo-wide fix (see that entry). Hit again in this session — `tests/boot.rs` carries an explicit `#![allow(clippy::disallowed_methods)]` as a workaround. A standalone PR touching the ten crates' `lib.rs` files is still the right shape; it has not been filed as a formal task in `TASKS.md` because the fix crosses ten crates and is infrastructure, not feature work. Worth scoping when bandwidth allows.

---

## 2026-04-20 — service-slm: realigned to AGPL-3.0-only + CLA (task [2])

- Licence flip EUPL-1.2 → AGPL-3.0-only per `factory-release-engineering/README.md §3` canonical mapping for `service-*` repos. 56 files' SPDX headers swept; `LICENSE` replaced with verbatim FSF AGPL-3.0 text from `factory-release-engineering/licenses/AGPL-3.0.txt`; REUSE plumbing (`LICENSES/`, `.reuse/dep5`) updated; `docs/adr/0003` renamed `0003-eupl-for-own-code` → `0003-agpl3-for-own-code` and rewritten with editor's-note block removed; `Cargo.toml` workspace `license` field flipped; `CLAUDE.md` Invariant 1 updated.
- Contribution certification flipped DCO → CLA per `factory-release-engineering/README.md §2` policy for AGPLv3 repos. `CLAUDE.md` Invariant 4 updated; CI `dco:` job removed from `.github/workflows/ci.yml`; PR template DCO checkbox → CLA checkbox; `CONTRIBUTING.md` workflow step 5 rewritten; `scripts/bootstrap.sh` DCO git-hook installation removed.
- Root-level `SLM-STACK.md` and `YOYO-COMPUTE.md` duplicates deleted (orphans with no references; `specs/` copies remain canonical and read-only).
- `NOTICE` edited manually by Peter (the `.claude/settings.json` `deny` list has `Edit(NOTICE)` as a legal-review guardrail; guardrail preserved).
- **Future cleanup surfaced (track in next sessions):**
  - `specs/SLM-STACK.md` lines 279 (DCO guidance) and 413 (D45 Apache-2.0 + DCO decision record) still reference pre-alignment state. Per CLAUDE.md invariant, `specs/` is read-only; fix at monorepo master per Q1(A+B) decision and re-copy to `service-slm/specs/`. Target: supersession markers pointing to ADR-0003 and `factory-release-engineering/README.md §3`.
  - CLA Assistant tooling activation on GitHub depends on `factory-release-engineering/cla/cla-assistant-config.yml` shipping (factory-release-engineering §4 Phase 7). Until then, documented CLA policy runs ahead of CI enforcement; contributors aren't yet auto-prompted.
  - `service-slm-PRE-COMMIT-CHECKLIST.md` and `PRE-COMMIT-CHECKLIST.md` have identical content (6068 bytes each). Dedupe or symlink in a follow-up PR.
  - `.claude/settings.json:37` permission `Bash(git commit --no-sign-off *)` is harmless residue after the DCO flip; remove in a housekeeping PR.
  - `factory-release-engineering/` `headers/`, `cla/`, `readmes/`, `mapping/`, `scripts/propagate-licenses.sh` artefacts don't exist yet; when they ship, re-verify service-slm against the canonical artefacts and align where needed.

---

## 2026-04-20 — service-slm: slm-core ModuleId lands (task [10])

- First real implementation in service-slm. `slm-core` moves scaffold → alpha: `ModuleId` newtype with validation, `Display`/`FromStr`, serde round-trip, 17 tests. Placeholder `__scaffold_placeholder` removed from `crates/slm-core/src/lib.rs`.
- **New open question surfaced (project config):** `service-slm/clippy.toml` has a comment saying `.unwrap()`/`.expect()` are disallowed "outside tests," but clippy's `disallowed-methods` config has no test-scoping mechanism. The lint fires against `--all-targets`. Worked around locally by adding `#[allow(clippy::disallowed_methods)]` at two test sites. Proposed follow-up: add `#![cfg_attr(test, allow(clippy::disallowed_methods))]` at each crate's `lib.rs` in a separate PR rather than annotating every test file — touches ~10 crates and belongs in its own change.
- No monorepo-wide renames or deprecations touched.

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
