---
artifact: brief
schema: foundry-brief-v1
brief-id: project-design-brief-audit-2026-06
title: "BRIEF audit — project-design — 2026-06"
status: active
owner: project-design
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-design has one legitimate active BRIEF (app-privategit-design), but four contaminated project-intelligence BRIEFs remain at root level instead of being moved to archive/ as the README documents they should be; all BRIEFs are missing required schema/brief-id/title/owner frontmatter fields.

Active: 1 / Total: 5

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
| `/srv/foundry/clones/project-design/.agent/briefs/BRIEF-app-privategit-design.md` | active | fix-schema | Missing required frontmatter fields: schema (foundry-brief-v1), brief-id (project-design-app-privategit-design), title (in body H1 but not frontmatter), owner. Also updated value '2026-06-08-b' is a non-standard date suffix — should be ISO 8601 date only. |
| `/srv/foundry/clones/project-design/.agent/briefs/BRIEF-slm-learning-loop.md` | active | migrate-to-archive → /srv/foundry/clones/project-design/.agent/briefs/archive/BRIEF-slm-learning-loop.md | Belongs to project-intelligence (SLM/DPO training pipeline). README archived-briefs table already lists it as a contamination artifact to redistribute to project-intelligence, but the file was never physically moved to archive/. Status should be changed to 'archived' and file git-mv'd to briefs/archive/. |
| `/srv/foundry/clones/project-design/.agent/briefs/BRIEF-slm-substrate-master.md` | active | migrate-to-archive → /srv/foundry/clones/project-design/.agent/briefs/archive/BRIEF-slm-substrate-master.md | Belongs to project-intelligence (Yo-Yo VM + Doorman + tier routing). README archived-briefs table already lists it as contamination to redistribute to project-intelligence, but the file was never physically moved to archive/. Status should be changed to 'archived' and file git-mv'd to briefs/archive/. |
| `/srv/foundry/clones/project-design/.agent/briefs/BRIEF-dev-env-mcp-expansion.md` | archived | migrate-to-archive → /srv/foundry/clones/project-design/.agent/briefs/archive/BRIEF-dev-env-mcp-expansion.md | Status is correctly 'archived' but file remains at briefs/ root level rather than briefs/archive/. Content covers slm-mcp-server MCP expansion — project-intelligence scope. Should be git-mv'd to archive/ to match the README archived table intent. Also missing schema, brief-id, title, owner frontmatter. |
| `/srv/foundry/clones/project-design/.agent/briefs/AI-AUDIT-baseline-2026-05-31.md` | active | migrate-to-archive → /srv/foundry/clones/project-design/.agent/briefs/archive/AI-AUDIT-baseline-2026-05-31.md | Not a BRIEF (missing BRIEF- filename prefix). Belongs to project-intelligence (audit of service-slm/service-content/app-orchestration-slm). README archived table lists it as contamination. Status should change to 'archived', file renamed to BRIEF-ai-audit-baseline-2026-05-31.md and git-mv'd to archive/. Missing schema, brief-id, title, owner frontmatter. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- BRIEF-app-privategit-design.md: missing frontmatter field schema (required: foundry-brief-v1)
- BRIEF-app-privategit-design.md: missing frontmatter field brief-id (required: project-design-app-privategit-design)
- BRIEF-app-privategit-design.md: missing frontmatter field title (title present in H1 body but not in frontmatter)
- BRIEF-app-privategit-design.md: missing frontmatter field owner
- BRIEF-app-privategit-design.md: updated field value '2026-06-08-b' is non-standard — ISO 8601 date only required
- BRIEF-slm-learning-loop.md: missing frontmatter fields schema, brief-id, title, owner
- BRIEF-slm-learning-loop.md: file at briefs/ root but README documents it as archived/moved to archive/ — file never physically relocated
- BRIEF-slm-substrate-master.md: missing frontmatter fields schema, brief-id, title, owner
- BRIEF-slm-substrate-master.md: file at briefs/ root but README documents it as archived/moved to archive/ — file never physically relocated
- BRIEF-dev-env-mcp-expansion.md: missing frontmatter fields schema, brief-id, title, owner
- BRIEF-dev-env-mcp-expansion.md: status is 'archived' but file remains at briefs/ root level instead of briefs/archive/
- AI-AUDIT-baseline-2026-05-31.md: non-canonical filename — missing BRIEF- prefix required by brief-discipline.md
- AI-AUDIT-baseline-2026-05-31.md: missing frontmatter fields schema, brief-id, title, owner
- README.md: active-briefs table is accurate (lists only BRIEF-app-privategit-design.md) but archived-briefs table references files not yet moved to archive/ — divergence between README state and actual filesystem state

## New BRIEFs needed

- **BRIEF-design-token-intake-pipeline**: Inbox shows 3 pending DESIGN-TOKEN-CHANGE dispatches from project-documents, project-orgcharts, and project-workplace — each requiring master_cosign verification and commit to pointsav-design-system. Session context shows this is a recurring cross-archive workflow with multi-session continuity requirements (blocked drafts, cosign tracking, Stage 6 pending). A BRIEF would track the token intake queue, cosign state, and pending Stage 6 promotes across sessions, replacing ad-hoc tracking in session-context.md.

## Work log

2026-06-12 command@claude-code: Automated audit run. 1 active, 5 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `/srv/foundry/clones/project-design/.agent/briefs/BRIEF-app-privategit-design.md` — Missing required frontmatter fields: schema (foundry-brief-v1), brief-id (project-design-app-privategit-design), title (in body H1 but not frontmatter), owner. Also updated value '2026-06-08-b' is a non-standard date suffix — should be ISO 8601 date only.
- [ ] migrate-to-archive: `/srv/foundry/clones/project-design/.agent/briefs/BRIEF-slm-learning-loop.md` — Belongs to project-intelligence (SLM/DPO training pipeline). README archived-briefs table already lists it as a contamination artifact to redistribute to project-intelligence, but the file was never physically moved to archive/. Status should be changed to 'archived' and file git-mv'd to briefs/archive/.
- [ ] migrate-to-archive: `/srv/foundry/clones/project-design/.agent/briefs/BRIEF-slm-substrate-master.md` — Belongs to project-intelligence (Yo-Yo VM + Doorman + tier routing). README archived-briefs table already lists it as contamination to redistribute to project-intelligence, but the file was never physically moved to archive/. Status should be changed to 'archived' and file git-mv'd to briefs/archive/.
- [ ] migrate-to-archive: `/srv/foundry/clones/project-design/.agent/briefs/BRIEF-dev-env-mcp-expansion.md` — Status is correctly 'archived' but file remains at briefs/ root level rather than briefs/archive/. Content covers slm-mcp-server MCP expansion — project-intelligence scope. Should be git-mv'd to archive/ to match the README archived table intent. Also missing schema, brief-id, title, owner frontmatter.
- [ ] migrate-to-archive: `/srv/foundry/clones/project-design/.agent/briefs/AI-AUDIT-baseline-2026-05-31.md` — Not a BRIEF (missing BRIEF- filename prefix). Belongs to project-intelligence (audit of service-slm/service-content/app-orchestration-slm). README archived table lists it as contamination. Status should change to 'archived', file renamed to BRIEF-ai-audit-baseline-2026-05-31.md and git-mv'd to archive/. Missing schema, brief-id, title, owner frontmatter.
- [ ] Create BRIEF for: BRIEF-design-token-intake-pipeline
