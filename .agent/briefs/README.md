# Briefs — project-workplace

`BRIEF-*.md` files are permanent git-tracked artifacts. Never delete — supersede via
`status: archived` or `git mv` to `briefs/archive/`. See `conventions/brief-discipline.md`.

## Active briefs

| File | brief-id | Title | Status | Updated |
|------|----------|-------|--------|---------|
| `BRIEF-location-intelligence-archetypes-2026-06-01.md` | A18 | Location Intelligence Archetypes (PRO / VWH / PKS) | active | 2026-06-13 |
| `BRIEF-pks-fable-analysis-2026-06-11.md` | A25 | PKS Commuter Archetype — Fable Model Analysis | active | 2026-06-11 |
| `BRIEF-gis-nightly-rebuild-aec-2026-06-12.md` | A26 | GIS Nightly Rebuild + AEC Layer Infrastructure | active | 2026-06-13 |

**Note — contamination (Command Session action required):** 10 non-GIS BRIEFs are present in this
directory from a prior project-knowledge contamination event. These must NOT be deleted.
Command Session to `git mv` them to their correct archive `briefs/` directories.
Contaminated files: `BRIEF-artifact-style-guide.md`, `BRIEF-brief-audit-2026-06.md`,
`BRIEF-crypto-license-payment-architecture.md`, `BRIEF-dev-env-mcp-expansion.md`,
`BRIEF-os-totebox-ppn-build-out.md`, `BRIEF-ostotebox-phase1-deployment.md`,
`BRIEF-phase-fg-institutional-redesign.md`, `BRIEF-software-distribution-substrate.md`,
`BRIEF-substrate-phd-thesis-2026-05-27.md`, `BRIEF-totebox-transformation.md`

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
