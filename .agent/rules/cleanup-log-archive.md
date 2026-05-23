# Cleanup Log Archive — project-knowledge/pointsav-monorepo

Archived session entries from `cleanup-log.md`. Newest on top.
Active config sections remain in `cleanup-log.md`.

---

> *Entries truncated for performance (820 lines removed — see git history).*

  `sync/`) plus `main.rs` and `config.rs`; `templates/` (4 HTML
  files); `static/` (13 KB `wiki.js` + 19 KB `style.css`);
  `tests/fixtures/architecture/` with 2 markdown fixtures;
  `.gitignore` (46 B). `Cargo.toml` and `README.md` were
  overwritten (93 B → 1,470 B; 751 B → 8,243 B). A garbage
  top-level directory literally named `{src` — containing a
  four-level chain of brace-expansion artefacts from how the zip
  was originally created (quoted `mkdir` blocked shell expansion)
  — was removed before any git operation. Nothing staged or
  committed in the extraction step itself.
- **Open follow-ups from the extraction (not acted on this
  session):**
  - `README.es.md` (403 B scaffold) is now out of sync with the
    new 8,243 B English README — CLAUDE.md §6 bilingual-pair rule
    in violation until a refresh pass lands. Editorial work;
    track as open item rather than inline.
  - `.gitkeep` at project root is redundant now that `src/` has
    real files; remove at next commit touching this project.
  - Registry row (`app-mediakit-knowledge` under `app-mediakit`)
    currently reads "Scaffold-coded, 4 files" — state remains
    Scaffold-coded per §8 (never run end-to-end) but file count
    and notes need updating.
  - Source-side disposition of
    `content-wiki-documentation/app-mediakit-knowledge.zip`
    undecided: delete from the sibling repo (cross-repo move,
    separate commit there), or retain as an archive. Not
    recorded in this repo's `handoffs-outbound.md` since the
    direction is inbound, not outbound.
- **BIM product family handoff landed — four project directories
  created, rules extension added.** The zip
  `/home/mathew/Documents/pointsav-bim-handoff.zip` (44 KB, 10
  files) was unpacked into a `/tmp` staging area and 9 files were
  placed into the monorepo:
  - Four new project directories each with `CLAUDE.md` +
    `RESEARCH.md`: `app-console-bim/`, `app-orchestration-bim/`,
    `app-workplace-bim/`, `service-bim/`.
  - One new `.claude/rules/` file:
    `.claude/rules/bim-product-family.md` (9,238 B) — a new
    *category* of rules file (product-family rules), outside the
    four named in `~/Foundry/CLAUDE.md` §10. Surfaced to Master
    Claude as a potential §10 extension.
  - Joint research file placed as `RESEARCH.md` in **both**
    `app-console-bim/` and `app-orchestration-bim/` — intentional
    duplication for Task Claude — BIM to rationalise during its
    cleanup pass, not prematurely.
  - `RESEARCH-BIM-MARKET.md` not placed in the monorepo (already
    present in `content-wiki-documentation/` at repo root,
    byte-identical; per `repo-layout.md` sibling-repo rule, market
    research belongs in content-wiki only).
  - `CLAUDE-root-additions.md` held back — it describes patches to
    a monorepo root `CLAUDE.md` that does not exist. Zip retained
    at source path; Master Claude applies when the root CLAUDE.md
    is created.
- **Registry drift closed (four rows without directories).** The
  2026-04-22 bootstrap registered the four BIM dirs as
  Reserved-folder with "1 file (RESEARCH.md)" notes, but
  `git ls-tree` showed no trace on any branch. The rows were
  aspirational; the directories were never created. This session
  creates them for the first time. Registry rows updated to
  reflect the actual contents (2 files each). State remains
  Reserved-folder (§8: Scaffold-coded requires a `Cargo.toml`
  skeleton; these are research-phase, no code yet).
- **Cross-repo BIM handover outbox entry opened.** Single
  consolidated entry in `handoffs-outbound.md` headed "BIM
  material → content-wiki-documentation", labelled as a **pattern
  variant: raw-material handover, not a file move** — source files
  remain in the monorepo permanently. Destination Root Claude
  transforms the material into proper wiki topics per its own
  repo-layout. Detection pattern for closure:
  `"receive BIM material from pointsav-monorepo"` in the
  destination repo's git log.
- **Surfaced for Master Claude (workspace-scope follow-ups):**
  1. **Root `CLAUDE.md` for `pointsav-monorepo` is missing.**
     Required per §10 to wire the `.claude/rules/*` files into
     Claude sessions. `CLAUDE-root-additions.md` in the handoff
     zip (location:
     `/home/mathew/Documents/pointsav-bim-handoff.zip` →
     `CLAUDE-root-additions.md`, 1,594 B) describes four targeted
     additions (`.claude/rules/bim-product-family.md` reference,
     four BIM dirs in Repo structure, canonical-name guards,
     IFC/F12 rules). Apply when the root CLAUDE.md is first
     drafted.
  2. **§10 canonical list may need to grow.**
     `bim-product-family.md` is a fifth type of `.claude/rules/`
     file beyond the four listed in §10. Decision: enumerate,
     generalise, or name as a subcategory.
  3. **`cluster-bim` clone provisioning pending.** Per §9, Master
     Claude provisions clones. A future Task Claude — BIM needs
     `~/Foundry/clones/cluster-bim/` with feature branch
     `cluster/bim` and a `PROJECT-CLONES.md` row before it can
     activate the four BIM projects.
  4. **Stale paths in existing outbox entries.** The two prior
     entries in `handoffs-outbound.md` use
     `/home/mathew/Foundry/factory-pointsav/...` paths (non-
     existent on disk) and helper `~/Foundry/tool-commit-as-next.sh`
     (§7 canonical is `~/Foundry/bin/commit-as-next.sh`). A
     destination Root Claude running the prescribed commands
     verbatim would hit failures. Needs correction before
     pickup. This session's new BIM entry uses correct paths.

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
