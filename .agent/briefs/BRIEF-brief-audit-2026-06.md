---
artifact: brief
schema: foundry-brief-v1
brief-id: project-knowledge-brief-audit-2026-06
title: "BRIEF audit — project-knowledge — 2026-06"
status: active
owner: project-knowledge
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-knowledge has 1 legitimate active BRIEF (knowledge-platform-master) plus 2 contamination BRIEFs from project-data and project-intelligence that inflated the active count to 3; all BRIEFs except the master are missing required schema/brief-id/owner frontmatter fields, and the README active-briefs table is unpopulated.

Active: 3 / Total: 5

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
| `/srv/foundry/clones/project-knowledge/.agent/briefs/BRIEF-knowledge-platform-master.md` | active | fix-schema | Missing required frontmatter fields: schema (foundry-brief-v1), brief-id, owner. Also README.md active-briefs table is empty and does not list this BRIEF. |
| `/srv/foundry/clones/project-knowledge/.agent/briefs/BRIEF-dev-env-mcp-expansion.md` | archived | fix-schema | Status is canonical (archived) but missing required frontmatter fields: schema, brief-id, title, owner. Additionally the BRIEF covers slm-mcp-server expansion work which belongs to project-intelligence, not project-knowledge — already git mv'd to archive/ but schema is still incomplete. |
| `/srv/foundry/clones/project-knowledge/.agent/briefs/BRIEF-os-totebox-ppn-build-out.md` | active | migrate-to-archive → clones/project-knowledge/.agent/briefs/archive/BRIEF-os-totebox-ppn-build-out.md | Frontmatter declares archive: project-data — this is a contamination BRIEF from project-data, not project-knowledge scope. Should be git mv'd to archive/ with status: archived and contamination note, then re-homed in clones/project-data/.agent/briefs/. Also missing schema, brief-id, title, owner fields. |
| `/srv/foundry/clones/project-knowledge/.agent/briefs/BRIEF-project-intelligence-active-work.md` | active | migrate-to-archive → clones/project-knowledge/.agent/briefs/archive/BRIEF-project-intelligence-active-work.md | Content covers service-slm, Doorman, DataGraph, apprenticeship queue — all project-intelligence scope. This BRIEF was contaminated into project-knowledge. Should be git mv'd to archive/ with status: archived + contamination note, and re-homed in clones/project-intelligence/.agent/briefs/. Also missing schema, brief-id, title, owner fields. |
| `/srv/foundry/clones/project-knowledge/.agent/briefs/archive/BRIEF-project-intelligence-master.md` | relocated | rename-status | Status value 'relocated' is non-canonical. The five permitted values are: active, reference, archived, superseded, stub. Should be changed to 'archived' with the contamination_note retained. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- README.md active-briefs table is empty — no rows despite 3 active BRIEFs present
- BRIEF-knowledge-platform-master.md missing: schema:foundry-brief-v1, brief-id, owner
- BRIEF-dev-env-mcp-expansion.md missing: schema:foundry-brief-v1, brief-id, title, owner
- BRIEF-os-totebox-ppn-build-out.md missing: schema:foundry-brief-v1, brief-id, title, owner
- BRIEF-project-intelligence-active-work.md missing: schema:foundry-brief-v1, brief-id, title, owner
- archive/BRIEF-project-intelligence-master.md uses non-canonical status value 'relocated' (permitted: active|reference|archived|superseded|stub)

## New BRIEFs needed

_No new BRIEFs identified as needed._

## Work log

2026-06-12 command@claude-code: Automated audit run. 3 active, 5 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `/srv/foundry/clones/project-knowledge/.agent/briefs/BRIEF-knowledge-platform-master.md` — Missing required frontmatter fields: schema (foundry-brief-v1), brief-id, owner. Also README.md active-briefs table is empty and does not list this BRIEF.
- [ ] fix-schema: `/srv/foundry/clones/project-knowledge/.agent/briefs/BRIEF-dev-env-mcp-expansion.md` — Status is canonical (archived) but missing required frontmatter fields: schema, brief-id, title, owner. Additionally the BRIEF covers slm-mcp-server expansion work which belongs to project-intelligence, not project-knowledge — already git mv'd to archive/ but schema is still incomplete.
- [ ] migrate-to-archive: `/srv/foundry/clones/project-knowledge/.agent/briefs/BRIEF-os-totebox-ppn-build-out.md` — Frontmatter declares archive: project-data — this is a contamination BRIEF from project-data, not project-knowledge scope. Should be git mv'd to archive/ with status: archived and contamination note, then re-homed in clones/project-data/.agent/briefs/. Also missing schema, brief-id, title, owner fields.
- [ ] migrate-to-archive: `/srv/foundry/clones/project-knowledge/.agent/briefs/BRIEF-project-intelligence-active-work.md` — Content covers service-slm, Doorman, DataGraph, apprenticeship queue — all project-intelligence scope. This BRIEF was contaminated into project-knowledge. Should be git mv'd to archive/ with status: archived + contamination note, and re-homed in clones/project-intelligence/.agent/briefs/. Also missing schema, brief-id, title, owner fields.
- [ ] rename-status: `/srv/foundry/clones/project-knowledge/.agent/briefs/archive/BRIEF-project-intelligence-master.md` — Status value 'relocated' is non-canonical. The five permitted values are: active, reference, archived, superseded, stub. Should be changed to 'archived' with the contamination_note retained.

