---
artifact: brief
schema: foundry-brief-v1
brief-id: project-bim-brief-audit-2026-06
title: "BRIEF audit — project-bim — 2026-06"
status: active
owner: project-bim
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-bim has 2 legitimate active BRIEFs (both needing schema fixes) and 5 contaminated BRIEFs from other archives physically present in the directory — contamination is documented in README.md but the files have not been moved to briefs/archive/ yet.

Active: 2 / Total: 7

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
| `BRIEF-bim-objects-system.md` | active | fix-schema | Missing required frontmatter fields: schema (foundry-brief-v1), brief-id (e.g. project-bim-bim-objects-system), owner, created. Only has artifact, status, archive, updated. |
| `BRIEF-bim-website-pipeline.md` | active | fix-schema | Missing required frontmatter fields: schema (foundry-brief-v1), brief-id (e.g. project-bim-bim-website-pipeline), updated. Has artifact, status, topic, archive, created, owner but uses 'topic' instead of 'title' and lacks 'updated'. |
| `BRIEF-comprehensive-improvement-proposal.md` | absorbed | rename-status | Status 'absorbed' is non-canonical. Per brief-discipline.md the only valid values are active|reference|archived|superseded|stub. The brief has been absorbed into BRIEF-project-console-master.md; correct status is 'superseded'. Also belongs to project-console archive, not project-bim — contamination documented in README. |
| `BRIEF-knowledge-platform-master.md` | active | migrate-to-archive → briefs/archive/BRIEF-knowledge-platform-master.md | Contamination from project-knowledge archive (archive: project-knowledge in frontmatter). README documents this explicitly. Per discipline, never delete — should be git mv'd to briefs/archive/ to keep it out of the active count. Owning archive is project-knowledge. |
| `BRIEF-location-intelligence-archetypes-2026-06-01.md` | active | migrate-to-archive → briefs/archive/BRIEF-location-intelligence-archetypes-2026-06-01.md | Contamination from project-gis archive (author: totebox@project-gis in frontmatter). README documents this explicitly. Should be git mv'd to briefs/archive/ to remove from active count. Owning archive is project-gis. |
| `BRIEF-project-console-master.md` | active | migrate-to-archive → briefs/archive/BRIEF-project-console-master.md | Contamination from project-console archive (archive: project-console in frontmatter). README documents this explicitly. Should be git mv'd to briefs/archive/. Owning archive is project-console. |
| `BRIEF-yoyo-cloud-run-migration.md` | archived | migrate-to-archive → briefs/archive/BRIEF-yoyo-cloud-run-migration.md | Status is 'archived' (canonical) and belongs to project-intelligence (author: totebox@project-intelligence). README documents this as contamination. Should be git mv'd to briefs/archive/ per its own status and contamination status. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- BRIEF-bim-objects-system.md: missing schema:foundry-brief-v1 field
- BRIEF-bim-objects-system.md: missing brief-id field (required, e.g. project-bim-bim-objects-system)
- BRIEF-bim-objects-system.md: missing owner field
- BRIEF-bim-objects-system.md: missing created field
- BRIEF-bim-website-pipeline.md: missing schema:foundry-brief-v1 field
- BRIEF-bim-website-pipeline.md: missing brief-id field (required, e.g. project-bim-bim-website-pipeline)
- BRIEF-bim-website-pipeline.md: missing updated field
- BRIEF-bim-website-pipeline.md: uses 'topic' key instead of canonical 'title' key in frontmatter
- BRIEF-comprehensive-improvement-proposal.md: non-canonical status value 'absorbed' (valid values: active|reference|archived|superseded|stub)
- session-start.md is contamination from project-marketing (archive: project-marketing in frontmatter) — project-bim has no session-start.md of its own
- README.md active-briefs table lists only 2 briefs but does not note that contaminated BRIEFs are counted in total_count — table is accurate but contamination tracking comment could be clearer

## New BRIEFs needed

- **app-workplace-bim Wave 3 scaffold**: NEXT.md has a concrete multi-session item for app-workplace-bim Wave 3 (Tauri v1.7, Phase 1 AutoCAD muscle memory, IfcOpenShell subprocess, EUPL-1.2 licence) with no corresponding BRIEF. This is distinct from the BIM Objects system and the website pipeline — it is a new surface/crate with its own architecture decisions and phase boundaries that warrant multi-session tracking.

## Work log

2026-06-12 command@claude-code: Automated audit run. 2 active, 7 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `BRIEF-bim-objects-system.md` — Missing required frontmatter fields: schema (foundry-brief-v1), brief-id (e.g. project-bim-bim-objects-system), owner, created. Only has artifact, status, archive, updated.
- [ ] fix-schema: `BRIEF-bim-website-pipeline.md` — Missing required frontmatter fields: schema (foundry-brief-v1), brief-id (e.g. project-bim-bim-website-pipeline), updated. Has artifact, status, topic, archive, created, owner but uses 'topic' instead of 'title' and lacks 'updated'.
- [ ] rename-status: `BRIEF-comprehensive-improvement-proposal.md` — Status 'absorbed' is non-canonical. Per brief-discipline.md the only valid values are active|reference|archived|superseded|stub. The brief has been absorbed into BRIEF-project-console-master.md; correct status is 'superseded'. Also belongs to project-console archive, not project-bim — contamination documented in README.
- [ ] migrate-to-archive: `BRIEF-knowledge-platform-master.md` — Contamination from project-knowledge archive (archive: project-knowledge in frontmatter). README documents this explicitly. Per discipline, never delete — should be git mv'd to briefs/archive/ to keep it out of the active count. Owning archive is project-knowledge.
- [ ] migrate-to-archive: `BRIEF-location-intelligence-archetypes-2026-06-01.md` — Contamination from project-gis archive (author: totebox@project-gis in frontmatter). README documents this explicitly. Should be git mv'd to briefs/archive/ to remove from active count. Owning archive is project-gis.
- [ ] migrate-to-archive: `BRIEF-project-console-master.md` — Contamination from project-console archive (archive: project-console in frontmatter). README documents this explicitly. Should be git mv'd to briefs/archive/. Owning archive is project-console.
- [ ] migrate-to-archive: `BRIEF-yoyo-cloud-run-migration.md` — Status is 'archived' (canonical) and belongs to project-intelligence (author: totebox@project-intelligence). README documents this as contamination. Should be git mv'd to briefs/archive/ per its own status and contamination status.
- [ ] Create BRIEF for: app-workplace-bim Wave 3 scaffold
