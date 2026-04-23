# NEXT.md — pointsav-monorepo

> **Scope: this repo only.** Cross-repo and workspace-level open
> items live at `~/Foundry/NEXT.md`.
>
> Read at session start when a Root Claude opens in this repo. Update
> at session end when repo-scope open items change.

Last updated: 2026-04-23.

---

## Currently open

### Layout hygiene — defect closures queued

Rule source: `.claude/rules/repo-layout.md` (introduced 2026-04-23).
Each item below is a separate commit via `tool-commit-as-next.sh`.

- **Move `GUIDE-OPERATIONS.md`** → `content-wiki-documentation/`
  (cross-repo move; commit in each repo).
- **Move `USER_GUIDE_2026-03-30_V2.md`** →
  `content-wiki-documentation/` with `_V2` suffix dropped per
  CLAUDE.md §6. BCSC-language review of the User Guide content
  remains a separate open question (see cleanup-log).
- **Move `app-console-content/src/{pointsav-surveyor.sh,surveyor.py}`**
  → `app-console-content/scripts/`. `surveyor.py` is the live
  Verification Surveyor operational tool; check callers before
  move. Reconcile the `MAX_DAILY_VERIFICATIONS = 10` value with
  the corresponding cleanup-log open question in the same commit.
- **Rename `os-infrastructure/build_iso/forge_iso.sh`** →
  `os-infrastructure/build_iso/compile_binary.sh` (resolve
  duplicate-filename collision with the sibling ISO-assembly
  script at the project root).
- **Project-root scripts → `scripts/` subfolders** (separate
  commit per project): `service-vpn` (5 scripts),
  `service-email` (`spool-daemon.sh`), `service-slm`
  (`cognitive-bridge.sh`), `service-content` (`forge-seeds.sh`),
  `os-network-admin` (2 scripts), `os-totebox` (1 script),
  `tool-cognitive-forge` (1 script), `vendor-phi3-mini` (2
  scripts), `app-mediakit-telemetry` (5 generic scaffold
  scripts).

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

### Rename series (active — see `.claude/rules/cleanup-log.md`)

- `vendors-maxmind` → `vendor-maxmind` (typo) + data-category
  reclass (move `.mmdb` out of Git to build-time fetch).
- `pointsav-pty-bridge` → `service-pty-bridge` (brand-prefix
  violation; daemon runtime fits `service-*`).
- `service-parser` → remove (legacy name; canonical is
  `service-extraction`).
- `service-email-egress-{ews,imap}` → `service-email-egress`
  (consolidate per Q3a decision).
- `tool-cognitive-forge` → rename pending ("Cognitive Forge" on
  Do-Not-Use list).

### Structural defects

- **Workspace `Cargo.toml` unification** — per 2026-04-18 audit,
  workspace declares only 8 of ~70+ crates as members. Other crates
  are treated as standalone workspaces (hence 23 stray
  `Cargo.lock` files). Unifying would consolidate targets and
  resolve profile inheritance.
- **Monorepo `.gitignore` deduplication** — the "Asymmetric Storage
  Protocol: Enforce Tier-1 Quarantine" block is duplicated four
  times. Normalise to a single copy.
- **Large binaries** — candidates for build-time fetch or removal:
  `vendor-maxmind/GeoLite2-City.mmdb` (63.5 MB),
  `tool-cognitive-forge/engine/llamafile` (66 MB),
  `tool-cognitive-forge/engine/weights/qwen2.5-coder-1.5b.gguf`
  (15 MB), and ISO / IMG artefacts in `os-infrastructure/`,
  `os-network-admin/`, `os-totebox/` (tracking status TBD).

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
