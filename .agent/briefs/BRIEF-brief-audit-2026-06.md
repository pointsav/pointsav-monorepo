---
artifact: brief
schema: foundry-brief-v1
brief-id: project-orgcharts-brief-audit-2026-06
title: "BRIEF audit — project-orgcharts — 2026-06"
status: active
owner: project-orgcharts
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

No BRIEF-*.md files exist at the top level of project-orgcharts; the archive is clean after a 2026-06-04 contamination sweep that moved six foreign BRIEFs to archive/; active work tracked only in NEXT.md and session-start.md.

Active: 0 / Total: 0

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
| (none) | | | |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- README.md active-briefs table lists 'none' but does not use the canonical schema format (no brief-id column, no status column) — acceptable for empty state but should be updated when first BRIEF is created
- audit-foundry-wide-2026-05-16.md in briefs/ is a non-brief research document; it is registered in README.md as a non-brief exception, which is compliant, but the file lacks foundry-brief-v1 frontmatter that would distinguish it from a BRIEF at a glance

## New BRIEFs needed

- **Org-chart authoring — JW series milestone tracker**: NEXT.md tracks 76+ uncommitted commits, multiple pending Stage 6 signals, JW7+JW9 REVIEW milestones gating wiki leg, and Bencal naming conflict — this is multi-session in-progress work that would benefit from a durable BRIEF rather than accumulating exclusively in NEXT.md
- **Design token backfill — pointsav-design-system**: NEXT.md references A3/A4 DESIGN-TOKEN-CHANGE drafts awaiting project-design, and token-olive class backfill pending; this cross-archive design-system work spans multiple sessions and sessions need durable context beyond NEXT.md checkboxes

## Work log

2026-06-12 command@claude-code: Automated audit run. 0 active, 0 total BRIEFs reviewed.

## Carry-forward

- [ ] Create BRIEF for: Org-chart authoring — JW series milestone tracker
- [ ] Create BRIEF for: Design token backfill — pointsav-design-system
