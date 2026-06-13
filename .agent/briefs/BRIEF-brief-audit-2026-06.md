---
artifact: brief
schema: foundry-brief-v1
brief-id: project-data-brief-audit-2026-06
title: "BRIEF audit — project-data — 2026-06"
status: active
owner: project-data
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-data has 4 BRIEFs (1 archived, 3 active); 2 of the active BRIEFs are scope-contamination from project-intelligence and need redistribution per NEXT.md; all BRIEFs are missing multiple required frontmatter fields and the README active-briefs table is entirely empty.

Active: 3 / Total: 4

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
| `BRIEF-dev-env-mcp-expansion.md` | archived | fix-schema | Missing required frontmatter fields: schema, brief-id, title, owner, updated. Status is canonical (archived) so no rename needed, but the schema is incomplete for an archived artifact — should be corrected before any future reference. |
| `BRIEF-os-totebox-ppn-build-out.md` | active | fix-schema | Missing required frontmatter fields: brief-id (should be project-data-os-totebox-ppn-build-out), title, owner. Has artifact, schema, archive, created, updated. Also the active-briefs table in README.md is empty and must be updated to include this BRIEF. |
| `BRIEF-project-intelligence-active-work.md` | active | migrate-to-archive → clones/project-intelligence/.agent/briefs/BRIEF-project-intelligence-active-work.md | This BRIEF belongs to project-intelligence, not project-data. NEXT.md §Command actions pending explicitly lists it as one of six project-intelligence briefs contaminated into this archive that need redistribution to project-intelligence. Also missing schema, brief-id, title, owner fields. |
| `BRIEF-project-intelligence-master.md` | active | migrate-to-archive → clones/project-intelligence/.agent/briefs/BRIEF-project-intelligence-master.md | This BRIEF belongs to project-intelligence, not project-data. NEXT.md §Command actions pending lists it as contaminated into this archive (referenced indirectly via BRIEF-slm-substrate-master which this supersedes, plus author field shows totebox@project-intelligence). Missing brief-id and owner fields. Should be relocated and status set to archived in project-data after the copy is confirmed. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- README.md active-briefs table is empty — no rows despite 3 active BRIEFs present (BRIEF-os-totebox-ppn-build-out.md, BRIEF-project-intelligence-active-work.md, BRIEF-project-intelligence-master.md)
- BRIEF-dev-env-mcp-expansion.md: missing brief-id field
- BRIEF-dev-env-mcp-expansion.md: missing title field
- BRIEF-dev-env-mcp-expansion.md: missing owner field
- BRIEF-dev-env-mcp-expansion.md: missing updated field
- BRIEF-dev-env-mcp-expansion.md: missing schema field
- BRIEF-os-totebox-ppn-build-out.md: missing brief-id field
- BRIEF-os-totebox-ppn-build-out.md: missing title field
- BRIEF-os-totebox-ppn-build-out.md: missing owner field
- BRIEF-project-intelligence-active-work.md: missing schema field
- BRIEF-project-intelligence-active-work.md: missing brief-id field
- BRIEF-project-intelligence-active-work.md: missing title field
- BRIEF-project-intelligence-active-work.md: missing owner field
- BRIEF-project-intelligence-master.md: missing brief-id field
- BRIEF-project-intelligence-master.md: missing owner field

## New BRIEFs needed

- **JOURNAL programme — multi-paper research pipeline**: NEXT.md has an active §JOURNAL programme section with 6+ open items across J1, J3, J4, J7 (language passes, OLS regression, AEC coverage metrics, §4-§8 stubs). These span multiple sessions and are too substantial for NEXT.md alone. A dedicated BRIEF-journal-programme.md would carry decisions locked (e.g. J7 hold lifted, committed body), open items (OLS regression, AEC build blockers, ORCID requirement), and carry-forward context — reducing session startup overhead.

## Work log

2026-06-12 command@claude-code: Automated audit run. 3 active, 4 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `BRIEF-dev-env-mcp-expansion.md` — Missing required frontmatter fields: schema, brief-id, title, owner, updated. Status is canonical (archived) so no rename needed, but the schema is incomplete for an archived artifact — should be corrected before any future reference.
- [ ] fix-schema: `BRIEF-os-totebox-ppn-build-out.md` — Missing required frontmatter fields: brief-id (should be project-data-os-totebox-ppn-build-out), title, owner. Has artifact, schema, archive, created, updated. Also the active-briefs table in README.md is empty and must be updated to include this BRIEF.
- [ ] migrate-to-archive: `BRIEF-project-intelligence-active-work.md` — This BRIEF belongs to project-intelligence, not project-data. NEXT.md §Command actions pending explicitly lists it as one of six project-intelligence briefs contaminated into this archive that need redistribution to project-intelligence. Also missing schema, brief-id, title, owner fields.
- [ ] migrate-to-archive: `BRIEF-project-intelligence-master.md` — This BRIEF belongs to project-intelligence, not project-data. NEXT.md §Command actions pending lists it as contaminated into this archive (referenced indirectly via BRIEF-slm-substrate-master which this supersedes, plus author field shows totebox@project-intelligence). Missing brief-id and owner fields. Should be relocated and status set to archived in project-data after the copy is confirmed.
- [ ] Create BRIEF for: JOURNAL programme — multi-paper research pipeline
