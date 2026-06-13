---
artifact: brief
schema: foundry-brief-v1
brief-id: project-gis-brief-audit-2026-06
title: "BRIEF audit — project-gis — 2026-06"
status: active
owner: project-gis
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-gis has 8 active BRIEFs against a soft cap of 5; three are out-of-domain (project-knowledge, project-data, PhD thesis substrate), all but one are missing required frontmatter fields, and the README.md active-briefs table is empty.

Active: 8 / Total: 9

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
| `BRIEF-artifact-style-guide.md` | active | migrate-to-archive → clones/project-editorial/.agent/briefs/ | Owner declared as project-editorial; content is cross-archive editorial style guidance with no GIS-specific scope. Living in project-gis is a scope violation. Also missing required fields: schema, brief-id. Should move to project-editorial where it belongs. |
| `BRIEF-gis-nightly-rebuild-aec-2026-06-12.md` | active | fix-schema | Missing required frontmatter fields: brief-id (uses non-standard 'name:' field instead), title, owner, updated. Schema is present. Brief-id should be: project-gis-gis-nightly-rebuild-aec. |
| `BRIEF-knowledge-platform-master.md` | active | migrate-to-archive → clones/project-knowledge/.agent/briefs/ | Declares cluster: project-knowledge — this is a project-knowledge BRIEF that was created in the wrong archive. Missing required fields: schema, brief-id, owner, created, updated. Should be migrated to project-knowledge where it belongs. |
| `BRIEF-location-intelligence-archetypes-2026-06-01.md` | active | fix-schema | Missing required frontmatter fields: brief-id (uses non-standard 'name:' field instead), title, owner, updated. Core GIS domain BRIEF — correct archive. Needs schema repair. |
| `BRIEF-os-totebox-ppn-build-out.md` | active | migrate-to-archive → clones/project-data/.agent/briefs/ | Declares archive: project-data explicitly — this BRIEF belongs to project-data, not project-gis. Also missing required fields: title, brief-id, owner, updated. |
| `BRIEF-ostotebox-phase1-deployment.md` | active | fix-schema | Uses schema: foundry-draft-v1 instead of foundry-brief-v1 (wrong schema for a BRIEF). Missing required fields: brief-id, owner. Content is os-totebox infrastructure work with no GIS-specific scope; consider whether it belongs in project-gis or should migrate to the archive owning os-totebox work. |
| `BRIEF-phase-fg-institutional-redesign.md` | archived | keep-active | Status is already canonical 'archived' — correct. Content absorbed into BRIEF-artifact-style-guide.md §13 as documented in status_note. No action needed. |
| `BRIEF-pks-fable-analysis-2026-06-11.md` | active | fix-schema | Missing required frontmatter fields: brief-id (uses non-standard 'slug:' instead), owner, updated. Core GIS domain BRIEF (PKS archetype calibration) — correct archive. Needs schema repair. |
| `BRIEF-substrate-phd-thesis-2026-05-27.md` | active | migrate-to-archive → clones/project-intelligence/.agent/briefs/ | PhD thesis substrate BRIEF (J2 — Trustworthy Systems/ASPLOS) with no GIS-specific scope; declares schema: foundry-draft-v1 (wrong for a BRIEF). Missing required fields: brief-id, owner. Belongs with the systems/intelligence domain work, not GIS. |

## Consolidation opportunities

**Suggested: "os-totebox Infrastructure — PPN Build-Out and Phase 1 Boot Milestone"**
- Files: BRIEF-os-totebox-ppn-build-out.md, BRIEF-ostotebox-phase1-deployment.md
- Reason: Both BRIEFs document os-totebox infrastructure build-out work (PPN service stack and Phase 1 boot milestone respectively). They share the same domain, author session, and decision space. After migrating os-totebox-ppn-build-out.md to project-data, the Phase 1 deployment BRIEF should either follow or be merged into it as a sub-section.

## Missing governance

- README.md active-briefs table is empty — all 8 active BRIEFs are absent from the table; must be populated
- BRIEF-artifact-style-guide.md: missing schema and brief-id fields
- BRIEF-gis-nightly-rebuild-aec-2026-06-12.md: missing brief-id (uses 'name:' instead), title, owner, updated
- BRIEF-knowledge-platform-master.md: missing schema, brief-id, owner, created, updated
- BRIEF-location-intelligence-archetypes-2026-06-01.md: missing brief-id (uses 'name:' instead), title, owner, updated
- BRIEF-os-totebox-ppn-build-out.md: missing title, brief-id, owner, updated
- BRIEF-ostotebox-phase1-deployment.md: uses schema: foundry-draft-v1 instead of foundry-brief-v1; missing brief-id, owner
- BRIEF-pks-fable-analysis-2026-06-11.md: missing brief-id (uses 'slug:' instead), owner, updated
- BRIEF-substrate-phd-thesis-2026-05-27.md: uses schema: foundry-draft-v1 instead of foundry-brief-v1; missing brief-id, owner
- Active BRIEF count (8) exceeds soft cap of 5 — archive cleanup required before opening new BRIEFs

## New BRIEFs needed

- **PKS Opportunity Scoring**: NEXT.md item (2026-06-11) describes a multi-decision work item — DEVELOP/EXPAND/SATURATED classification — that requires design decisions about scoring thresholds, property schema, and UX integration. Multi-session tracking scope that is orthogonal to the existing PKS calibration BRIEF.
- **GIS Governance Layer — pairings.yaml + cron-audit.sh**: NEXT.md Phase 5 items (5a–5d) describe new governance infrastructure across pairings.yaml, .owner files for active gateways, and a new bin/cron-audit.sh validation script. This crosses Command/Totebox boundary and needs a BRIEF to track decisions and carry-forward across sessions.

## Work log

2026-06-12 command@claude-code: Automated audit run. 8 active, 9 total BRIEFs reviewed.

## Carry-forward

- [ ] migrate-to-archive: `BRIEF-artifact-style-guide.md` — Owner declared as project-editorial; content is cross-archive editorial style guidance with no GIS-specific scope. Living in project-gis is a scope violation. Also missing required fields: schema, brief-id. Should move to project-editorial where it belongs.
- [ ] fix-schema: `BRIEF-gis-nightly-rebuild-aec-2026-06-12.md` — Missing required frontmatter fields: brief-id (uses non-standard 'name:' field instead), title, owner, updated. Schema is present. Brief-id should be: project-gis-gis-nightly-rebuild-aec.
- [ ] migrate-to-archive: `BRIEF-knowledge-platform-master.md` — Declares cluster: project-knowledge — this is a project-knowledge BRIEF that was created in the wrong archive. Missing required fields: schema, brief-id, owner, created, updated. Should be migrated to project-knowledge where it belongs.
- [ ] fix-schema: `BRIEF-location-intelligence-archetypes-2026-06-01.md` — Missing required frontmatter fields: brief-id (uses non-standard 'name:' field instead), title, owner, updated. Core GIS domain BRIEF — correct archive. Needs schema repair.
- [ ] migrate-to-archive: `BRIEF-os-totebox-ppn-build-out.md` — Declares archive: project-data explicitly — this BRIEF belongs to project-data, not project-gis. Also missing required fields: title, brief-id, owner, updated.
- [ ] fix-schema: `BRIEF-ostotebox-phase1-deployment.md` — Uses schema: foundry-draft-v1 instead of foundry-brief-v1 (wrong schema for a BRIEF). Missing required fields: brief-id, owner. Content is os-totebox infrastructure work with no GIS-specific scope; consider whether it belongs in project-gis or should migrate to the archive owning os-totebox work.
- [ ] fix-schema: `BRIEF-pks-fable-analysis-2026-06-11.md` — Missing required frontmatter fields: brief-id (uses non-standard 'slug:' instead), owner, updated. Core GIS domain BRIEF (PKS archetype calibration) — correct archive. Needs schema repair.
- [ ] migrate-to-archive: `BRIEF-substrate-phd-thesis-2026-05-27.md` — PhD thesis substrate BRIEF (J2 — Trustworthy Systems/ASPLOS) with no GIS-specific scope; declares schema: foundry-draft-v1 (wrong for a BRIEF). Missing required fields: brief-id, owner. Belongs with the systems/intelligence domain work, not GIS.
- [ ] Consolidate BRIEF-os-totebox-ppn-build-out.md + BRIEF-ostotebox-phase1-deployment.md → "os-totebox Infrastructure — PPN Build-Out and Phase 1 Boot Milestone"
- [ ] Create BRIEF for: PKS Opportunity Scoring
- [ ] Create BRIEF for: GIS Governance Layer — pairings.yaml + cron-audit.sh
