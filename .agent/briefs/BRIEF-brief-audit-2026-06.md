---
artifact: brief
schema: foundry-brief-v1
brief-id: project-editorial-brief-audit-2026-06
title: "BRIEF audit — project-editorial — 2026-06"
status: active
owner: project-editorial
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-editorial has 2 legitimate active BRIEFs (artifact-style-guide and knowledge-platform-master) out of 9 total files; 3 files are contamination from other archives not yet remediated, the README active-briefs table is empty, and nearly all BRIEFs are missing brief-id and schema frontmatter fields.

Active: 2 / Total: 9

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
| `BRIEF-artifact-style-guide.md` | active | fix-schema | Missing required frontmatter fields: brief-id and schema. Has artifact, status, title, owner, created, updated — but brief-id (e.g. project-editorial-artifact-style-guide) and schema: foundry-brief-v1 are absent. This is the primary editorial reference BRIEF and the most important file in the directory; fix is low-effort. |
| `BRIEF-cross-platform-release.md` | archived | migrate-to-archive → briefs/archive/BRIEF-cross-platform-release.md | Contaminated copy from project-console (bulk-.agent/-copy event per contaminated_note). Uses schema: foundry-plan-v1 (wrong type). correct_archive: project-console is declared in frontmatter. Status is already archived; physical move to briefs/archive/ would enforce the boundary and remove it from the active directory listing. |
| `BRIEF-dev-env-mcp-expansion.md` | archived | fix-schema → briefs/archive/BRIEF-dev-env-mcp-expansion.md | Status is correctly archived but missing brief-id, schema, title, and owner from required frontmatter. No artifact: brief field present. Should be corrected before archive to ensure it is a valid historical record, then moved to briefs/archive/. |
| `BRIEF-knowledge-platform-master.md` | active | fix-schema | Active BRIEF covering app-mediakit-knowledge platform state. Missing required frontmatter: brief-id, schema, and owner. Has artifact, status, title, updated, and supersedes. Cluster: project-knowledge (not project-editorial) — this BRIEF tracks knowledge-platform work and is legitimately referenced from project-editorial as cross-archive context, but ownership should be clarified. Add brief-id: project-editorial-knowledge-platform-master, schema: foundry-brief-v1, owner: project-editorial. |
| `BRIEF-os-totebox-ppn-build-out.md` | active | migrate-to-archive → briefs/archive/BRIEF-os-totebox-ppn-build-out.md | Frontmatter declares archive: project-data — this BRIEF belongs to project-data, not project-editorial. Content covers service-people, service-extraction, and os-totebox build-out, all of which are project-data scope. This is a contamination from the bulk-.agent/-copy event. Should be archived here and the canonical copy confirmed in clones/project-data/.agent/briefs/. |
| `BRIEF-phase-fg-institutional-redesign.md` | archived | migrate-to-archive → briefs/archive/BRIEF-phase-fg-institutional-redesign.md | Status is correctly archived (content absorbed into BRIEF-artifact-style-guide.md §13 per status_note). Missing brief-id, schema, owner. Physical move to briefs/archive/ will clean the active directory. No schema fix needed urgently since it is already archived. |
| `BRIEF-project-console-master.md` | archived | migrate-to-archive → briefs/archive/BRIEF-project-console-master.md | Contaminated copy from project-console (contaminated_note present). correct_archive: project-console declared. Status is correctly archived. Move to briefs/archive/ to remove it from the working directory. Owner field not present; schema is foundry-brief-v1. |
| `BRIEF-project-intelligence-active-work.md` | active | archive | This BRIEF belongs to project-intelligence: author is totebox@project-intelligence, companion BRIEFs (BRIEF-slm-substrate-master.md, BRIEF-slm-learning-loop.md) are in project-intelligence scope, and all content covers service-slm, Doorman, OLMo, and apprenticeship queue — project-intelligence domain. This is contamination from the bulk-.agent/-copy event. Set status: archived with contamination note; move to briefs/archive/ and verify canonical copy exists in clones/project-intelligence/.agent/briefs/. Missing brief-id, schema, owner. |
| `archive/BRIEF-project-intelligence-master.md` | relocated | rename-status | Status value 'relocated' is not in the canonical five-value enum (active | reference | archived | superseded | stub). The file has already been git mv'd here with a contamination_note and relocated_to pointer. Change status to archived to restore canonical governance. Missing brief-id and schema fields. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- README.md active-briefs table is empty — no rows present despite 2 legitimate active BRIEFs (BRIEF-artifact-style-guide.md and BRIEF-knowledge-platform-master.md); table must be kept current at every session per brief-discipline.md
- BRIEF-artifact-style-guide.md missing brief-id field
- BRIEF-artifact-style-guide.md missing schema: foundry-brief-v1 field
- BRIEF-knowledge-platform-master.md missing brief-id field
- BRIEF-knowledge-platform-master.md missing schema: foundry-brief-v1 field
- BRIEF-knowledge-platform-master.md missing owner field
- BRIEF-dev-env-mcp-expansion.md missing artifact: brief field
- BRIEF-dev-env-mcp-expansion.md missing brief-id field
- BRIEF-dev-env-mcp-expansion.md missing schema: foundry-brief-v1 field
- BRIEF-dev-env-mcp-expansion.md missing title field
- BRIEF-dev-env-mcp-expansion.md missing owner field
- BRIEF-os-totebox-ppn-build-out.md missing brief-id field
- BRIEF-os-totebox-ppn-build-out.md missing owner field
- BRIEF-project-intelligence-active-work.md missing brief-id field
- BRIEF-project-intelligence-active-work.md missing schema: foundry-brief-v1 field
- BRIEF-project-intelligence-active-work.md missing owner field
- archive/BRIEF-project-intelligence-master.md uses non-canonical status value 'relocated' — must be renamed to 'archived'
- 3 BRIEFs in active directory are contamination from other archives (project-console, project-data, project-intelligence) — bulk-.agent/-copy event not yet fully remediated in this archive

## New BRIEFs needed

_No new BRIEFs identified as needed._

## Work log

2026-06-12 command@claude-code: Automated audit run. 2 active, 9 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `BRIEF-artifact-style-guide.md` — Missing required frontmatter fields: brief-id and schema. Has artifact, status, title, owner, created, updated — but brief-id (e.g. project-editorial-artifact-style-guide) and schema: foundry-brief-v1 are absent. This is the primary editorial reference BRIEF and the most important file in the directory; fix is low-effort.
- [ ] migrate-to-archive: `BRIEF-cross-platform-release.md` — Contaminated copy from project-console (bulk-.agent/-copy event per contaminated_note). Uses schema: foundry-plan-v1 (wrong type). correct_archive: project-console is declared in frontmatter. Status is already archived; physical move to briefs/archive/ would enforce the boundary and remove it from the active directory listing.
- [ ] fix-schema: `BRIEF-dev-env-mcp-expansion.md` — Status is correctly archived but missing brief-id, schema, title, and owner from required frontmatter. No artifact: brief field present. Should be corrected before archive to ensure it is a valid historical record, then moved to briefs/archive/.
- [ ] fix-schema: `BRIEF-knowledge-platform-master.md` — Active BRIEF covering app-mediakit-knowledge platform state. Missing required frontmatter: brief-id, schema, and owner. Has artifact, status, title, updated, and supersedes. Cluster: project-knowledge (not project-editorial) — this BRIEF tracks knowledge-platform work and is legitimately referenced from project-editorial as cross-archive context, but ownership should be clarified. Add brief-id: project-editorial-knowledge-platform-master, schema: foundry-brief-v1, owner: project-editorial.
- [ ] migrate-to-archive: `BRIEF-os-totebox-ppn-build-out.md` — Frontmatter declares archive: project-data — this BRIEF belongs to project-data, not project-editorial. Content covers service-people, service-extraction, and os-totebox build-out, all of which are project-data scope. This is a contamination from the bulk-.agent/-copy event. Should be archived here and the canonical copy confirmed in clones/project-data/.agent/briefs/.
- [ ] migrate-to-archive: `BRIEF-phase-fg-institutional-redesign.md` — Status is correctly archived (content absorbed into BRIEF-artifact-style-guide.md §13 per status_note). Missing brief-id, schema, owner. Physical move to briefs/archive/ will clean the active directory. No schema fix needed urgently since it is already archived.
- [ ] migrate-to-archive: `BRIEF-project-console-master.md` — Contaminated copy from project-console (contaminated_note present). correct_archive: project-console declared. Status is correctly archived. Move to briefs/archive/ to remove it from the working directory. Owner field not present; schema is foundry-brief-v1.
- [ ] archive: `BRIEF-project-intelligence-active-work.md` — This BRIEF belongs to project-intelligence: author is totebox@project-intelligence, companion BRIEFs (BRIEF-slm-substrate-master.md, BRIEF-slm-learning-loop.md) are in project-intelligence scope, and all content covers service-slm, Doorman, OLMo, and apprenticeship queue — project-intelligence domain. This is contamination from the bulk-.agent/-copy event. Set status: archived with contamination note; move to briefs/archive/ and verify canonical copy exists in clones/project-intelligence/.agent/briefs/. Missing brief-id, schema, owner.
- [ ] rename-status: `archive/BRIEF-project-intelligence-master.md` — Status value 'relocated' is not in the canonical five-value enum (active | reference | archived | superseded | stub). The file has already been git mv'd here with a contamination_note and relocated_to pointer. Change status to archived to restore canonical governance. Missing brief-id and schema fields.
