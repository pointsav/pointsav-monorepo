---
artifact: brief
schema: foundry-brief-v1
brief-id: project-workplace-brief-audit-2026-06
title: "BRIEF audit — project-workplace — 2026-06"
status: active
owner: project-workplace
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-workplace has 3 active BRIEFs (within the 5-BRIEF soft cap) with well-structured content and correct status values, but every BRIEF in the archive — all 10 files — is missing the three required frontmatter fields (brief-id, owner, updated), and the README.md active-briefs table is unpopulated.

Active: 3 / Total: 10

## Scope
Review all existing BRIEFs in this archive against the Phase A governance spec.
Out of scope: content quality review; this is structural + schema compliance only.

## Decisions locked
- 2026-06-12: Governance spec ratified in conventions/brief-discipline.md

## Decisions open
- [ ] Operator review of recommended actions below
- [ ] Actions executed by next Totebox session for this archive

## Action table

| File | Current status | Recommended action | Reason |
|------|---------------|-------------------|--------|
| `BRIEF-workplace-architecture.md` | active | fix-schema | Missing required frontmatter fields: brief-id, owner, updated. schema: foundry-brief-v1 is present. |
| `BRIEF-workplace-desktop-environment.md` | active | fix-schema | Missing required frontmatter fields: brief-id, owner, updated. schema: foundry-brief-v1 is present. |
| `BRIEF-workplace-roadmap.md` | active | fix-schema | Missing required frontmatter fields: brief-id, owner, updated. schema: foundry-brief-v1 is present. |
| `BRIEF-app-workplace-architecture.md` | archived | fix-schema | Missing required frontmatter fields: brief-id, owner, updated AND missing schema: foundry-brief-v1. Oldest BRIEF in the archive; archived 2026-05-27. |
| `BRIEF-leapfrog-2030-audit-and-vision.md` | archived | fix-schema | Missing required frontmatter fields: brief-id, owner, updated. |
| `BRIEF-tui-desktop-architecture.md` | archived | fix-schema | Missing required frontmatter fields: brief-id, owner, updated. |
| `BRIEF-workbench-refactoring-roadmap.md` | archived | fix-schema | Missing required frontmatter fields: brief-id, owner, updated. |
| `BRIEF-workplace-desktop-suite.md` | archived | fix-schema | Missing required frontmatter fields: brief-id, owner, updated. |
| `BRIEF-workplace-http-prototype.md` | archived | fix-schema | Missing required frontmatter fields: brief-id, owner, updated. |
| `BRIEF-workplace-software-suite.md` | archived | fix-schema | Missing required frontmatter fields: brief-id, owner, updated. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- brief-id field absent from all 10 BRIEF files
- owner field absent from all 10 BRIEF files
- updated field absent from all 10 BRIEF files
- schema: foundry-brief-v1 absent from BRIEF-app-workplace-architecture.md
- README.md active-briefs table is empty — 3 active BRIEFs (BRIEF-workplace-architecture.md, BRIEF-workplace-desktop-environment.md, BRIEF-workplace-roadmap.md) are not listed

## New BRIEFs needed

_No new BRIEFs identified as needed._

## Work log

2026-06-12 command@claude-code: Automated audit run. 3 active, 10 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `BRIEF-workplace-architecture.md` — Missing required frontmatter fields: brief-id, owner, updated. schema: foundry-brief-v1 is present.
- [ ] fix-schema: `BRIEF-workplace-desktop-environment.md` — Missing required frontmatter fields: brief-id, owner, updated. schema: foundry-brief-v1 is present.
- [ ] fix-schema: `BRIEF-workplace-roadmap.md` — Missing required frontmatter fields: brief-id, owner, updated. schema: foundry-brief-v1 is present.
- [ ] fix-schema: `BRIEF-app-workplace-architecture.md` — Missing required frontmatter fields: brief-id, owner, updated AND missing schema: foundry-brief-v1. Oldest BRIEF in the archive; archived 2026-05-27.
- [ ] fix-schema: `BRIEF-leapfrog-2030-audit-and-vision.md` — Missing required frontmatter fields: brief-id, owner, updated.
- [ ] fix-schema: `BRIEF-tui-desktop-architecture.md` — Missing required frontmatter fields: brief-id, owner, updated.
- [ ] fix-schema: `BRIEF-workbench-refactoring-roadmap.md` — Missing required frontmatter fields: brief-id, owner, updated.
- [ ] fix-schema: `BRIEF-workplace-desktop-suite.md` — Missing required frontmatter fields: brief-id, owner, updated.
- [ ] fix-schema: `BRIEF-workplace-http-prototype.md` — Missing required frontmatter fields: brief-id, owner, updated.
- [ ] fix-schema: `BRIEF-workplace-software-suite.md` — Missing required frontmatter fields: brief-id, owner, updated.
