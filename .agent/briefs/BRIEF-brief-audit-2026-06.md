---
artifact: brief
schema: foundry-brief-v1
brief-id: project-console-brief-audit-2026-06
title: "BRIEF audit — project-console — 2026-06"
status: active
owner: project-console
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-console briefs directory has zero legitimate active BRIEFs for its own work — all three active BRIEFs are contamination artifacts from project-knowledge, project-data, and project-intelligence; both archived native BRIEFs (cross-platform-release and project-console-master) have schema defects; the archive/BRIEF-project-intelligence-master.md uses the non-canonical status value 'relocated'; and the README active-briefs table is entirely unpopulated.

Active: 3 / Total: 7

## Scope
Review all existing BRIEFs in this archive against the Phase A governance spec.
Out of scope: content quality review; this is structural + schema compliance only.

## Decisions locked
- 2026-06-12: Governance spec ratified in conventions/brief-discipline.md

## Decisions open
- [x] Operator review of recommended actions below — completed; all actions approved as written
- [x] Actions executed by next Totebox session for this archive — completed 2026-06-12

## Action table

| File | Current status | Recommended action | Reason |
|------|---------------|-------------------|--------|
| `BRIEF-cross-platform-release.md` | archived | fix-schema | Uses schema: foundry-plan-v1 instead of foundry-brief-v1; missing artifact: brief field; missing brief-id and owner fields. Status is canonical (archived). |
| `BRIEF-dev-env-mcp-expansion.md` | archived | fix-schema | Missing schema, brief-id, title, and owner fields. Status is canonical (archived). Content belongs to workspace-level MCP expansion work, not project-console scope — but status is already archived so no action on scope. |
| `BRIEF-knowledge-platform-master.md` | active | migrate-to-archive → clones/project-knowledge/.agent/briefs/ | Contamination artifact: frontmatter declares cluster: project-knowledge and content is entirely about app-mediakit-knowledge. Belongs in project-knowledge, not project-console. Should be git mv'd to correct archive or archived here with contaminated_note. |
| `BRIEF-os-totebox-ppn-build-out.md` | active | migrate-to-archive → clones/project-data/.agent/briefs/ | Contamination artifact: frontmatter declares archive: project-data and all content concerns service-people, service-extraction, and os-totebox build-out work scoped to project-data. Should be relocated to project-data briefs directory. |
| `BRIEF-project-console-master.md` | archived | fix-schema | Missing brief-id and owner fields. Has artifact and schema fields. Status is canonical (archived). Contains the master os-console state tracker — historically significant even as archived. |
| `BRIEF-project-intelligence-active-work.md` | active | migrate-to-archive → clones/project-intelligence/.agent/briefs/ | Contamination artifact: author field declares totebox@project-intelligence and all content is about service-slm, apprenticeship queue, Doorman circuit breakers, and OLMo training — entirely project-intelligence scope. Should be relocated. |
| `archive/BRIEF-project-intelligence-master.md` | relocated | rename-status | Status value 'relocated' is non-canonical. Must be one of: active | reference | archived | superseded | stub. Given the file was git mv'd here from project-knowledge during contamination cleanup, status should be 'archived' with the existing contamination_note preserved. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- README.md active-briefs table is empty — no rows populated despite 3 active BRIEFs present in the directory
- BRIEF-cross-platform-release.md: missing artifact: brief field
- BRIEF-cross-platform-release.md: schema uses non-canonical foundry-plan-v1 instead of foundry-brief-v1
- BRIEF-cross-platform-release.md: missing brief-id field
- BRIEF-cross-platform-release.md: missing owner field
- BRIEF-dev-env-mcp-expansion.md: missing schema field
- BRIEF-dev-env-mcp-expansion.md: missing brief-id field
- BRIEF-dev-env-mcp-expansion.md: missing title field
- BRIEF-dev-env-mcp-expansion.md: missing owner field
- BRIEF-knowledge-platform-master.md: missing schema field
- BRIEF-knowledge-platform-master.md: missing brief-id field
- BRIEF-knowledge-platform-master.md: missing owner field
- BRIEF-os-totebox-ppn-build-out.md: missing brief-id field
- BRIEF-os-totebox-ppn-build-out.md: missing title field (only in body heading, not frontmatter)
- BRIEF-os-totebox-ppn-build-out.md: missing owner field
- BRIEF-project-console-master.md: missing brief-id field
- BRIEF-project-console-master.md: missing owner field
- BRIEF-project-intelligence-active-work.md: missing schema field
- BRIEF-project-intelligence-active-work.md: missing brief-id field
- BRIEF-project-intelligence-active-work.md: missing title field
- BRIEF-project-intelligence-active-work.md: missing owner field
- archive/BRIEF-project-intelligence-master.md: non-canonical status value 'relocated' (must be: active | reference | archived | superseded | stub)
- NEXT.md title reads 'project-data' instead of 'project-console' — contamination from bulk .agent/ copy
- manifest.md frontmatter is contaminated with project-knowledge YAML fields (cluster_name, datagraph_module_id blocks from project-knowledge schema)

## New BRIEFs needed

- **os-console active development state — phases 8–10**: BRIEF-project-console-master.md is archived (status: archived). Phases 8–10 (polish, operations, BIM/mesh/watchdog) have no active BRIEF tracking them. The archive has no legitimate active BRIEF for its own work. A new master BRIEF for the current development frontier is needed so sessions have a durable artifact for in-progress decisions.

## Work log

2026-06-12 command@claude-code: Automated audit run. 3 active, 7 total BRIEFs reviewed.
2026-06-12 totebox@claude-code: All carry-forward items executed. Contaminated BRIEFs archived in place; schema fixes applied; archive/BRIEF-project-intelligence-master.md status renamed to 'archived'; BRIEF-os-console-active-dev.md created; README populated; NEXT.md and manifest.md rewritten. Outbox sent to Command for BRIEF redistribution + Stage 6 Phase B promote.

## Carry-forward

- [x] fix-schema: `BRIEF-cross-platform-release.md` — schema changed to foundry-brief-v1; artifact: brief, brief-id, owner added. [2026-06-12 totebox@claude-code]
- [x] fix-schema: `BRIEF-dev-env-mcp-expansion.md` — schema, brief-id, title, owner added. [2026-06-12 totebox@claude-code]
- [x] migrate-to-archive: `BRIEF-knowledge-platform-master.md` — archived in place with contaminated_note; outbox to Command for redistribution to project-knowledge. [2026-06-12 totebox@claude-code]
- [x] migrate-to-archive: `BRIEF-os-totebox-ppn-build-out.md` — archived in place with contaminated_note; outbox to Command for redistribution to project-data. [2026-06-12 totebox@claude-code]
- [x] fix-schema: `BRIEF-project-console-master.md` — brief-id and owner added. [2026-06-12 totebox@claude-code]
- [x] migrate-to-archive: `BRIEF-project-intelligence-active-work.md` — archived in place with contaminated_note; outbox to Command for redistribution to project-intelligence. [2026-06-12 totebox@claude-code]
- [x] rename-status: `archive/BRIEF-project-intelligence-master.md` — status changed from 'relocated' to 'archived'. [2026-06-12 totebox@claude-code]
- [x] Create BRIEF for: os-console active development state — phases 8–10 — `BRIEF-os-console-active-dev.md` created. [2026-06-12 totebox@claude-code]
