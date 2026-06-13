# Briefs — project-editorial

`BRIEF-*.md` files are permanent git-tracked artifacts. Never delete — supersede via
`status: archived` or `git mv` to `briefs/archive/`. See `conventions/brief-discipline.md`.

## Active briefs

| File | brief-id | Title | Status | Updated |
|------|----------|-------|--------|---------|
| `BRIEF-artifact-style-guide.md` | — | Artifact Style Guide — Internal/External Voice Discipline | active (master) | 2026-06-12 |
| `BRIEF-brief-audit-2026-06.md` | project-editorial-brief-audit-2026-06 | BRIEF audit — project-editorial — 2026-06 | active | 2026-06-12 |

## Artifact routing
When a BRIEF graduates to a deliverable, it routes here:

| Artifact type | Destination |
|---|---|
| CODE-* | monorepo sub-clone; Stage 6 READY to Command |
| TOPIC-* | project-editorial drafts-outbound |
| DESIGN-* | project-design drafts-outbound |
| JOURNAL-* | project-editorial drafts-outbound |
| GUIDE-* | Command Session (woodfine/* customer tier) |

## Archived briefs

BRIEFs with `status: archived` or `status: superseded` are listed here or moved to
`briefs/archive/`.

| File | Archived date | Notes |
|------|--------------|-------|
| `BRIEF-phase-fg-institutional-redesign.md` | 2026-06-11 | Content absorbed into BRIEF-artifact-style-guide.md §13 |
| `BRIEF-dev-env-mcp-expansion.md` | 2026-06-09 | Archived (unrelated to editorial scope) |
| `BRIEF-cross-platform-release.md` | 2026-06-08 | Archived + contaminated (project-console) |
| `BRIEF-project-console-master.md` | 2026-06-08 | Archived + contaminated (project-console) |
| `archive/BRIEF-project-intelligence-master.md` | — | In archive/ subdirectory |

## Contaminated briefs

BRIEFs copied from other archives during a past bulk-copy event. `status: contaminated`.
Not active guidance for this archive. Retained per hard rule (briefs are permanent).

| File | contaminated_from | Notes |
|------|-------------------|-------|
| `BRIEF-knowledge-platform-master.md` | project-knowledge | Knowledge Platform master BRIEF |
| `BRIEF-os-totebox-ppn-build-out.md` | project-data | PPN build-out scoping |
| `BRIEF-project-intelligence-active-work.md` | project-intelligence | Intelligence active work tracker |
