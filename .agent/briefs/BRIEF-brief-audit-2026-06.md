---
artifact: brief
schema: foundry-brief-v1
brief-id: project-marketing-brief-audit-2026-06
title: "BRIEF audit — project-marketing — 2026-06"
status: active
owner: project-marketing
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-marketing has 4 active BRIEFs but 3 are confirmed cross-archive contamination (project-knowledge, project-data, project-intelligence); the 1 legitimate active BRIEF (artifact-style-guide) appears to be editorial-scope; and the archive has no BRIEF covering its own core mission (app-mediakit-marketing / SEO / UX work documented in NEXT.md).

Active: 4 / Total: 8

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
| `BRIEF-artifact-style-guide.md` | active | fix-schema | Missing required frontmatter fields: brief-id, schema (foundry-brief-v1). Owner is set to 'project-editorial' — if this BRIEF originated from project-editorial contamination it may belong there, but if it is legitimately project-marketing scope the owner field should name an individual or role within project-marketing. README active-briefs table is also empty and does not list this BRIEF. |
| `BRIEF-knowledge-platform-master.md` | active | migrate-to-archive → briefs/archive/ | Contaminated — frontmatter declares 'cluster: project-knowledge'; content tracks app-mediakit-knowledge, not project-marketing. Belongs in clones/project-knowledge/.agent/briefs/. Should be archived here (status: archived + contamination_note) and the correct copy confirmed or created in project-knowledge. Missing brief-id, schema, owner, created fields. |
| `BRIEF-os-totebox-ppn-build-out.md` | active | migrate-to-archive → briefs/archive/ | Contaminated — frontmatter declares 'archive: project-data'; content tracks service-people, service-extraction, and JOURNAL-totebox-orchestration work in project-data scope. Belongs in clones/project-data/.agent/briefs/. Missing brief-id, title, owner fields. NEXT.md for this archive confirms it was copied here from project-data during bulk-copy contamination. |
| `BRIEF-project-intelligence-active-work.md` | active | migrate-to-archive → briefs/archive/ | Contaminated — author declared as 'totebox@project-intelligence'; body tracks memory pressure incident, service-content, Doorman circuit breaker, and OLMo training pipeline — all project-intelligence scope. Belongs in clones/project-intelligence/.agent/briefs/. Missing brief-id, schema, title, owner fields. Companion BRIEFs (slm-substrate-master, slm-learning-loop) are also project-intelligence artifacts. |
| `BRIEF-cross-platform-release.md` | archived | archive | Already archived with contamination_note. Belongs to project-console. Can remain in archive/ subdirectory or be migrated there. Currently sitting in the root briefs/ directory rather than briefs/archive/ — should be moved for cleanliness. |
| `BRIEF-dev-env-mcp-expansion.md` | archived | fix-schema | Archived but missing title, brief-id, schema, owner fields. Content references BRIEF-slm-substrate-master and BRIEF-slm-learning-loop which are project-intelligence artifacts — this BRIEF may itself be contamination from a Command Session or project-intelligence. At minimum needs title and brief-id before archival record is clean. |
| `BRIEF-phase-fg-institutional-redesign.md` | archived | archive | Properly archived with status_note. Author is 'totebox@project-editorial' — this is editorial scope contamination. Should be moved to briefs/archive/ to keep the root briefs directory clean. No schema violations that block archival. |
| `BRIEF-project-console-master.md` | archived | archive | Properly archived with contamination_note and correct_archive: project-console. Should be moved to briefs/archive/ to keep the root briefs directory clean. No blocking schema violations given its archived status. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- README.md active-briefs table is empty — none of the 4 active BRIEFs are listed in the table
- brief-id field absent from all 8 BRIEFs (required per foundry-brief-v1 schema)
- schema: foundry-brief-v1 field absent from BRIEF-artifact-style-guide.md, BRIEF-knowledge-platform-master.md, BRIEF-project-intelligence-active-work.md, BRIEF-dev-env-mcp-expansion.md
- owner field absent from BRIEF-knowledge-platform-master.md, BRIEF-os-totebox-ppn-build-out.md, BRIEF-project-intelligence-active-work.md, BRIEF-dev-env-mcp-expansion.md
- created field absent from BRIEF-knowledge-platform-master.md
- title field absent from BRIEF-os-totebox-ppn-build-out.md, BRIEF-project-intelligence-active-work.md, BRIEF-dev-env-mcp-expansion.md
- BRIEF-cross-platform-release.md, BRIEF-phase-fg-institutional-redesign.md, BRIEF-project-console-master.md are archived but sit in briefs/ root instead of briefs/archive/
- NEXT.md is heavily contaminated with project-data, project-editorial, project-knowledge, and project-gis content — the project-marketing section is buried at the bottom; systemic Stage 6 sync-local contamination noted in NEXT.md itself

## New BRIEFs needed

- **project-marketing core mission BRIEF**: The archive has zero legitimate project-marketing BRIEFs. manifest.md describes app-mediakit-marketing (a Rust server with WordPress leapfrog architecture, v0.0.1 MVP shipped 2026-05-06). NEXT.md marketing section tracks GSC/sitemap/SEO/UX audit work, mobile improvements, and contact form. This ongoing product work warrants a persistent multi-session BRIEF to track decisions locked (SEO stack, UX sprint results, mobile improvement backlog, JSON-LD schema choices) and carry-forward items (operator-gated GSC/Bing actions, real contact form server work).

## Work log

2026-06-12 command@claude-code: Automated audit run. 4 active, 8 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `BRIEF-artifact-style-guide.md` — Missing required frontmatter fields: brief-id, schema (foundry-brief-v1). Owner is set to 'project-editorial' — if this BRIEF originated from project-editorial contamination it may belong there, but if it is legitimately project-marketing scope the owner field should name an individual or role within project-marketing. README active-briefs table is also empty and does not list this BRIEF.
- [ ] migrate-to-archive: `BRIEF-knowledge-platform-master.md` — Contaminated — frontmatter declares 'cluster: project-knowledge'; content tracks app-mediakit-knowledge, not project-marketing. Belongs in clones/project-knowledge/.agent/briefs/. Should be archived here (status: archived + contamination_note) and the correct copy confirmed or created in project-knowledge. Missing brief-id, schema, owner, created fields.
- [ ] migrate-to-archive: `BRIEF-os-totebox-ppn-build-out.md` — Contaminated — frontmatter declares 'archive: project-data'; content tracks service-people, service-extraction, and JOURNAL-totebox-orchestration work in project-data scope. Belongs in clones/project-data/.agent/briefs/. Missing brief-id, title, owner fields. NEXT.md for this archive confirms it was copied here from project-data during bulk-copy contamination.
- [ ] migrate-to-archive: `BRIEF-project-intelligence-active-work.md` — Contaminated — author declared as 'totebox@project-intelligence'; body tracks memory pressure incident, service-content, Doorman circuit breaker, and OLMo training pipeline — all project-intelligence scope. Belongs in clones/project-intelligence/.agent/briefs/. Missing brief-id, schema, title, owner fields. Companion BRIEFs (slm-substrate-master, slm-learning-loop) are also project-intelligence artifacts.
- [ ] archive: `BRIEF-cross-platform-release.md` — Already archived with contamination_note. Belongs to project-console. Can remain in archive/ subdirectory or be migrated there. Currently sitting in the root briefs/ directory rather than briefs/archive/ — should be moved for cleanliness.
- [ ] fix-schema: `BRIEF-dev-env-mcp-expansion.md` — Archived but missing title, brief-id, schema, owner fields. Content references BRIEF-slm-substrate-master and BRIEF-slm-learning-loop which are project-intelligence artifacts — this BRIEF may itself be contamination from a Command Session or project-intelligence. At minimum needs title and brief-id before archival record is clean.
- [ ] archive: `BRIEF-phase-fg-institutional-redesign.md` — Properly archived with status_note. Author is 'totebox@project-editorial' — this is editorial scope contamination. Should be moved to briefs/archive/ to keep the root briefs directory clean. No schema violations that block archival.
- [ ] archive: `BRIEF-project-console-master.md` — Properly archived with contamination_note and correct_archive: project-console. Should be moved to briefs/archive/ to keep the root briefs directory clean. No blocking schema violations given its archived status.
- [ ] Create BRIEF for: project-marketing core mission BRIEF
