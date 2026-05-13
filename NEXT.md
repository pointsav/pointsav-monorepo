# NEXT.md — pointsav-monorepo

> **Scope: this repo only.** Cross-repo and workspace-level open
> items live at `~/Foundry/NEXT.md`.
>
> Read at session start when a Root Claude opens in this repo. Update
> at session end when repo-scope open items change.

Last updated: 2026-05-13.

---

## app-mediakit-knowledge — Phase 6A shipped (2026-05-13)

- [x] `inject_wiki_prefixes` trailing-quote bug fixed (`src/render.rs`)
- [x] Slug normalisation fallback: mixed-case URL → 301 → canonical lowercase (`src/server.rs`)
- [x] Redirect hatnote via `?redirectedfrom=` (`src/server.rs`, `static/style.css`)
- [x] 4 new integration tests in `tests/slug_test.rs` — all pass

**Stage 6 pending** (Master scope):
- [ ] `promote.sh` on `pointsav-monorepo` (12 commits ahead of origin) [2026-05-13 task@claude-code]
- [ ] `cargo build --release` from `app-mediakit-knowledge/` subdirectory [2026-05-13 task@claude-code]
- [ ] `sudo systemctl restart` all 3 wiki services + smoke verify [2026-05-13 task@claude-code]

**Phase 6B — DID portable identity**: gated on BP6 operator decision. Plan at `.agent/plans/PHASE-6B-DID-IDENTITY.md`.

---

## Currently open

### Layout hygiene — defect closures queued

Rule source: `.claude/rules/repo-layout.md` (introduced 2026-04-23).
Each item below is a separate commit via `tool-commit-as-next.sh`.

*(queue empty — Tier-2 project-root scripts closed 2026-04-23;
see Recently closed below and `cleanup-log.md`)*

### Awaiting cross-repo handoff

Entries lodged in `.claude/rules/handoffs-outbound.md`. Pattern is
passive — nothing moves until Master Claude or a Root Claude in
the destination repo picks up the entry and commits the add-side.
Source files remain in place here until the destination has
committed; only then does a follow-up Root Claude session commit
the source-remove.

- **`GUIDE-OPERATIONS.md` → `content-wiki-documentation`** — see
  outbox for destination path and rationale.
- **`USER_GUIDE_2026-03-30_V2.md` → `content-wiki-documentation`**
  (with `_V2` dropped in transit) — see outbox.

### Framework follow-ups

- **BIM project activations** — three of four BIM projects are still
  Reserved-folder. Follow the `app-console-bookkeeper` pilot pattern
  (framework §8): `app-console-bim`, `app-orchestration-bim`,
  `app-workplace-bim`, `service-bim` (the fourth, which triggered
  the taxonomy expansion).
- **`service-bookkeeper` forward reference** — the
  `app-console-bookkeeper` view reads "Awaiting service-bookkeeper
  sync" but that service is not in the registry. Decide: register
  as Reserved-folder, redirect to `service-fs/data/`, or correct
  the reference.
- **HTML-plugin vs Rust-crate `Type`-column refinement.**
  `app-console-*` and `app-network-*` projects contain both
  patterns; the registry's `Type` column does not distinguish.
  Surfaced during bookkeeper activation.
- **`BIM.zip` triage** — user-added working-tree artefact; determine
  whether source data, extraction seed, or stray; gitignore or
  delete.

### Rename series

*(queue empty — all five rename-series items closed 2026-04-23;
see Recently closed below and `cleanup-log.md` Completed
migrations)*

### Structural defects

- **Workspace `Cargo.toml` unification** — per 2026-04-18 audit,
  workspace declares only 8 of ~70+ crates as members. Other crates
  are treated as standalone workspaces (hence 23 stray
  `Cargo.lock` files). Unifying would consolidate targets and
  resolve profile inheritance.
- **Monorepo `.gitignore` deduplication** — the "Asymmetric Storage
  Protocol: Enforce Tier-1 Quarantine" block is duplicated four
  times. Normalise to a single copy.
- **Large binaries** — tracked artefacts that should move to
  build-time fetch:
  - `app-mediakit-telemetry/assets/GeoLite2-City.mmdb` (63.5 MB)
    — **still tracked**. Next candidate for fetch-at-build
    treatment. Paths reclassified 2026-04-23.
  - `service-slm/router-trainer/engine/llamafile` (35 MB) —
    **untracked since 2026-04-23** via `git rm --cached` + new
    `.gitignore` pattern. Physical file remains at path for the
    Python workflow. History still contains the blob; shrinking
    the repo requires `git-filter-repo`, separate task.
  - `service-slm/router-trainer/engine/weights/qwen2.5-coder-1.5b.gguf`
    (15 MB) — already covered by existing `**/weights/*` +
    `*.gguf` ignore patterns. Same history-blob caveat applies.
  - ISO / IMG artefacts in `os-infrastructure/`,
    `os-network-admin/`, `os-totebox/` (tracking status TBD).

### New projects to register — Reverse-Flow Substrate (Doctrine claim #52)

Six new Reserved-folder projects are named in DOCTRINE.md claim #52
(ratified 2026-04-30) and `conventions/reverse-flow-substrate.md`.
Create directory + bilingual READMEs + registry row in one commit per
project (workspace §9: directory creation and registry row must land
together). Activation to Active follows the standard framework §8
procedure (CLAUDE.md + NEXT.md + registry row update).

| Project | Prefix type | App OS | Notes |
|---|---|---|---|
| `service-market` | `service-*` | `os-totebox` | Ring 2 data marketplace — outbound connectors (Snowflake, AWS Data Exchange, LiveRamp) + inbound Delta Sharing API |
| `service-exchange` | `service-*` | `os-totebox` | Ring 2 ad exchange — IAB OpenRTB 2.6; SSP + DSP bidirectional; Prebid Server sidecar; `iab-specs-openrtb` crate |
| `app-orchestration-market` | `app-orchestration-*` | `os-orchestration` | Browser marketplace storefront; deployed as `gateway-orchestration-market-N` |
| `app-orchestration-exchange` | `app-orchestration-*` | `os-orchestration` | Browser ad campaign UI; deployed as `gateway-orchestration-exchange-N` |
| `app-console-market` | `app-console-*` | `os-console` | Secure TUI for industries where web delivery is too risky (financial, health, legal data) |
| `app-console-exchange` | `app-console-*` | `os-console` | Secure TUI ad exchange surface; same risk-profile rationale |

Also note: `app-orchestration-gis` (from `project-gis` cluster,
deployed as `gateway-orchestration-gis-1`) is absent from the project
registry — close this registry drift in the same pass.

### Conformance and activations

- **`app-workplace-memo` activation.** Scaffold-coded with 47 files,
  described by its sibling as "running on Linux Mint." Needs
  `CLAUDE.md` + `NEXT.md` to become Active per framework §8.
- **`app-workplace-proforma` CLAUDE.md commit-convention decision.**
  Its `CLAUDE.md` exists but is marked "not committed to git." Per
  the 2026-04-22 framework decision (committed convention is
  canonical), this file either needs committing or explicit
  conformance to a local-only exception.
- **`service-extraction/CLAUDE.md` staleness.** The in-module
  `CLAUDE.md` describes v0.2/v0.4 development but the code is a
  149-line filesystem-watching router — different implementation.
  Align before any new refactor of this service.

### Stashes parked in this repo

- `stash@{0}` — 2026-04-22 — "task21 WIP before worktree removal"
  (on `audit-layer-1-findings`; engineering work on `slm-memory-kv`
  crate, renames, untracked research doc). Restore with
  `git stash pop` when ready to resume.
- `stash@{1}` — pre-existing — "On service-extraction-v04: main:
  registry + BIM untracked — parked before task [21] resume".

## Recently closed (2026-04-23)

- Repo-layout rule introduced — `.claude/rules/repo-layout.md`
  codifies allowed files at the monorepo root and at each project
  directory root; names the sibling repos
  (`content-wiki-documentation`, `pointsav-design-system`, etc.)
  where cross-cutting content belongs. Anchor for the "Layout
  hygiene" queue above.
- `force_build.sh` relocated — root → `vendor-sel4-kernel/scripts/`.
  Zero runtime callers; script uses absolute paths so no content
  edits were needed. Repo root is now one file lighter against the
  new rule.
- `os-infrastructure/build_iso/forge_iso.sh` renamed to
  `compile_binary.sh` — resolves filename collision with the
  sibling ISO-assembly script at the project root. In-file header
  updated. Zero external callers. New open question logged in
  `cleanup-log.md`: the compile and assembly scripts are not wired
  together.
- `app-console-content/src/{pointsav-surveyor.sh,surveyor.py}`
  relocated to `app-console-content/scripts/`. Both files moved as
  100% renames. Shell wrapper is relative (`$(dirname "$0")`),
  Python script uses absolute paths — neither needed content
  edits. Throttle open-question row in `cleanup-log.md` updated
  with a code-reference pointer to the new path; the operator
  decision on `MAX_DAILY_VERIFICATIONS = 10` remains open.
- Handoff-outbound pattern introduced —
  `.claude/rules/handoffs-outbound.md` logs cross-repo file moves
  kept in place here until a Root Claude in the destination repo
  commits them. Two entries lodged (`GUIDE-OPERATIONS.md`,
  `USER_GUIDE_2026-03-30_V2.md`, both to
  `content-wiki-documentation`). Formalisation of the pattern in
  `~/Foundry/CLAUDE.md` §9 and §10 surfaced for Master Claude in
  `cleanup-log.md`.
- Tier-2 project-root scripts relocated — 18 files across 9
  projects moved to their respective `scripts/` subfolders in 9
  separate commits (`8f5cc48` through `faae141`). Every file
  registered as a 100% rename; no callers needed updating.
  Projects touched: `os-totebox`, `service-content`,
  `service-email`, `service-slm`, `tool-cognitive-forge`,
  `os-network-admin`, `vendor-phi3-mini`, `service-vpn`,
  `app-mediakit-telemetry`. Stray `tool-cognitive-forge/llama.log`
  surfaced as a separate housekeeping item.
- `service-parser/` removed — first rename-series closure.
  Directory contained only a README describing a superseded
  AI-routing framing; zero runtime references, never a workspace
  member, one commit in history. Nothing recyclable into
  `service-extraction` (which describes a different, deterministic
  Parser-Combinators approach). Rename-table row moved to
  Completed migrations; registry row removed (Defect count
  5 → 4, Total rows 100 → 99).
- `pointsav-pty-bridge` → `service-pty-bridge` — second
  rename-series closure. Directory renamed via `git mv` (4 files,
  all 100% renames); `Cargo.toml` `name` field updated in the
  same commit. Registry row moved from "Other / special" into
  the Service table; reclassified Defect → Scaffold-coded
  (Defect 4 → 3, Scaffold-coded 51 → 52). Zero external import
  references; not a workspace member; stray `Cargo.lock` left
  in place (resolves with workspace unification).
- Fifth (final) rename-series closure — Cognitive Forge term
  retired in one commit. `service-slm/cognitive-forge/` renamed
  to `service-slm/router/`; former top-level `tool-cognitive-forge/`
  moved to `service-slm/router-trainer/`. Rust runtime
  (`router/`) and Python distillation workflow
  (`router-trainer/`) now live together as producer/consumer.
  Cargo.toml `name` + `main.rs` usage string updated.
  `distill_knowledge.py` moved from non-canonical `src/` to
  `scripts/`. Three binary/log files untracked via `git rm
  --cached` + new `.gitignore` patterns (llamafile 35 MB,
  engine.log, llama.log) — physical files remain at new paths.
  Registry Scaffold-coded 54 → 53, Total 98 → 97. Closes the
  rename-series queue entirely (5 of 5) and the separate
  `llama.log` housekeeping item.
- `service-email-egress-{ews,imap}` wrappers flattened — fourth
  rename-series closure. Consolidation-to-`service-email-egress`
  plan reversed after sub-crate review: EWS and IMAP are two
  protocol adapters, not duplicates, and merging them would erase
  the architectural distinction. Instead, the redundant
  doubly-nested wrapper directories were flattened — 73 files
  promoted up one level. Registry reclassified both from
  Defect → Scaffold-coded; Defect count 2 → 0 (registry is now
  Defect-free). The 13 dir-name / Cargo-name mismatches from the
  2026-04-18 audit remain separate.
- `vendors-maxmind` reclassified to
  `app-mediakit-telemetry/assets/` — third rename-series closure.
  Data-only directory moved to the authoritative path already
  documented in the vendor's README; `.mmdb` (63.5 MB) + both
  READMEs travelled together; empty `vendors-maxmind/` removed.
  Open question "does it belong as a `vendor-*` crate at all?"
  closed (answer: no; non-workspace data directory).
  `repo-layout.md` extended to name `assets/` and `data/` as
  conventional subfolders. Registry Defect 3 → 2, Total rows
  99 → 98. In-transit edit to `USER_GUIDE_2026-03-30_V2.md`
  line 902 updates the path reference — travels with the pending
  cross-repo handoff. Separate `.mmdb` → build-time-fetch task
  remains open under Structural defects.

## Recently closed (2026-04-22)

- Audit cleanup — removed 2 `__MACOSX/` directories and 16 tracked
  `.DS_Store` / AppleDouble files from egress extraction-artefact
  scaffolding. `.DS_Store` added to `.gitignore`. Commit `0eeaeba`.
- Project registry bootstrap — 96-row inventory covering every
  top-level directory. Commit `fd7811f`.
- BIM-research project rows + cleanup-log bootstrap on `main` (drift
  closed) + taxonomy-expansion session entry. Commit `3cc8f4a`.
- `app-console-bookkeeper` activation pilot — Reserved-folder
  (mis-classified) → Active. Commit `27ad6d2`.

## Pointers

- Workspace-level open items: `~/Foundry/NEXT.md`
- Workspace changelog: `~/Foundry/CHANGELOG.md`
- Project registry: `.claude/rules/project-registry.md`
- Cleanup log: `.claude/rules/cleanup-log.md`
- Repo layout rule: `.claude/rules/repo-layout.md`
- Handoffs outbound: `.claude/rules/handoffs-outbound.md`
