---
artifact: brief
schema: foundry-brief-v1
brief-id: project-proforma-brief-audit-2026-06
title: "BRIEF audit — project-proforma — 2026-06"
status: active
owner: project-proforma
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-proforma has 2 active BRIEFs (well under the 5-BRIEF soft cap) and 1 archived BRIEF; all three files are missing the canonical schema and brief-id frontmatter fields, and the README.md active-briefs table is unpopulated.

Active: 2 / Total: 3

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
| `/srv/foundry/clones/project-proforma/.agent/briefs/BRIEF-proforma-engine.md` | archived | fix-schema | Missing required frontmatter fields: schema (foundry-brief-v1) and brief-id (<archive>-<slug>). The name field is present but brief-id is the canonical field per governance spec. Status and superseded_by are correctly set. |
| `/srv/foundry/clones/project-proforma/.agent/briefs/BRIEF-tearsheet-alt-re-v2.md` | active | fix-schema | Missing required frontmatter fields: schema (foundry-brief-v1) and brief-id (<archive>-<slug>). The name field is present but is not the canonical brief-id field. Body sections present and content is well-formed. |
| `/srv/foundry/clones/project-proforma/.agent/briefs/BRIEF-tool-proforma-leapfrog-2030.md` | active | fix-schema | Missing required frontmatter fields: schema (foundry-brief-v1) and brief-id (<archive>-<slug>). The name field is present but is not the canonical brief-id field. This is the master product BRIEF and is well-structured beyond the missing schema fields. |
| `/srv/foundry/clones/project-proforma/.agent/briefs/README.md` | active | fix-schema | Active-briefs table is empty (no rows) despite 2 active BRIEFs existing in the directory. BRIEF-tearsheet-alt-re-v2 and BRIEF-tool-proforma-leapfrog-2030 must be added to the table per governance spec requiring README.md to be kept current at every session. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- All BRIEF-*.md files are missing the schema: foundry-brief-v1 frontmatter field
- All BRIEF-*.md files use name: instead of the canonical brief-id: <archive>-<slug> field
- README.md active-briefs table has no entries despite 2 active BRIEFs present (BRIEF-tearsheet-alt-re-v2 and BRIEF-tool-proforma-leapfrog-2030)

## New BRIEFs needed

- **Stage 6 monorepo conflict resolution**: NEXT.md documents a multi-session blocker (cluster-branch conflict on cherry-pick of commits 72d4a635/6b2606bc, monorepo 7 commits ahead of origin/main, pwoodfine SSH alias broken) with no BRIEF tracking the open decisions and rollback path. This is orthogonal to the product-vision BRIEF and spans multiple sessions.

## Work log

2026-06-12 command@claude-code: Automated audit run. 2 active, 3 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `/srv/foundry/clones/project-proforma/.agent/briefs/BRIEF-proforma-engine.md` — Missing required frontmatter fields: schema (foundry-brief-v1) and brief-id (<archive>-<slug>). The name field is present but brief-id is the canonical field per governance spec. Status and superseded_by are correctly set.
- [ ] fix-schema: `/srv/foundry/clones/project-proforma/.agent/briefs/BRIEF-tearsheet-alt-re-v2.md` — Missing required frontmatter fields: schema (foundry-brief-v1) and brief-id (<archive>-<slug>). The name field is present but is not the canonical brief-id field. Body sections present and content is well-formed.
- [ ] fix-schema: `/srv/foundry/clones/project-proforma/.agent/briefs/BRIEF-tool-proforma-leapfrog-2030.md` — Missing required frontmatter fields: schema (foundry-brief-v1) and brief-id (<archive>-<slug>). The name field is present but is not the canonical brief-id field. This is the master product BRIEF and is well-structured beyond the missing schema fields.
- [ ] fix-schema: `/srv/foundry/clones/project-proforma/.agent/briefs/README.md` — Active-briefs table is empty (no rows) despite 2 active BRIEFs existing in the directory. BRIEF-tearsheet-alt-re-v2 and BRIEF-tool-proforma-leapfrog-2030 must be added to the table per governance spec requiring README.md to be kept current at every session.
- [ ] Create BRIEF for: Stage 6 monorepo conflict resolution
