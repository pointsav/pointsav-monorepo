# Briefs — project-gis

`BRIEF-*.md` files are permanent git-tracked artifacts. Never delete — supersede via
`status: archived` or `git mv` to `briefs/archive/`. See `conventions/brief-discipline.md`.

## Active briefs

| File | brief-id | Title | Status | Updated |
|------|----------|-------|--------|---------|
| BRIEF-pks-fable-analysis-2026-06-11.md | pks-fable-analysis | PKS Commuter Archetype — Fable Rebalance | active | 2026-06-11 |
| BRIEF-gis-nightly-rebuild-aec-2026-06-12.md | gis-nightly-rebuild-aec | GIS Nightly Rebuild + AEC Layer Infrastructure | active | 2026-06-19 |

## Artifact routing

When a BRIEF graduates to a deliverable, it routes here:

| Artifact type | Destination |
|---|---|
| CODE-* | pointsav-monorepo sub-clone; Stage 6 READY to Command |
| TOPIC-* | project-editorial drafts-outbound |
| DESIGN-* | project-design drafts-outbound |
| GUIDE-* | Command Session (woodfine/* customer tier) |

## Contamination note (2026-06-19)

| File | Archived date | Notes |
|------|--------------|-------|
| archive/BRIEF-app-workplace-architecture.md | 2026-06-14 | Relocated from project-gis (wrong archive) |

## History

- 2026-06-19: Contamination repaired — README previously showed project-knowledge content
  (knowledge-platform-master BRIEF); restored to GIS-only content.
- 2026-06-13: README overwritten by project-intelligence rebase (M-17 contamination).
  20 non-GIS BRIEFs relocated by Command (commit: actioned 2026-06-18).
