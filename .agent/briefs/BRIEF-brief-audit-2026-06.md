---
artifact: brief
schema: foundry-brief-v1
brief-id: project-system-brief-audit-2026-06
title: "BRIEF audit — project-system — 2026-06"
status: active
owner: project-system
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-system has 5 active BRIEFs at the soft cap, all missing one or more required frontmatter fields (brief-id and owner are universally absent), the README active-briefs table is completely unpopulated, two BRIEFs use the wrong schema value (foundry-draft-v1 instead of foundry-brief-v1), one archived BRIEF uses a non-canonical status value ('relocated'), and NEXT.md references a BRIEF-VM-ARCHITECTURE.md that does not exist.

Active: 5 / Total: 6

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
| `.agent/briefs/BRIEF-artifact-style-guide.md` | active | fix-schema | Missing required frontmatter fields: brief-id, schema. The owner field is present but set to 'project-editorial' (an archive name, not a role noun — violates §2.2 of the style guide it governs). No body sections in canonical order (Context → Scope → Decisions locked → Decisions open → Work log → Carry-forward) — structured as numbered paragraphs instead. |
| `.agent/briefs/BRIEF-knowledge-platform-master.md` | active | fix-schema | Missing required frontmatter fields: brief-id, schema, owner, created. Has non-standard fields (cluster, supersedes, verdict_source) but lacks the canonical required set. The cluster field suggests this BRIEF may belong to project-knowledge archive rather than project-system. |
| `.agent/briefs/BRIEF-os-totebox-ppn-build-out.md` | active | fix-schema | Missing required frontmatter fields: brief-id, owner. Contains archive: project-data which conflicts with its location in the project-system archive — likely a misrouted BRIEF from a prior archive contamination event. The NEXT.md brief-redistribution item names this file as one of 6 project-intelligence briefs that need Command relocation. |
| `.agent/briefs/BRIEF-ostotebox-phase1-deployment.md` | active | fix-schema | Missing required frontmatter fields: brief-id, owner. Uses schema: foundry-draft-v1 instead of foundry-brief-v1 — this is the draft artifact schema, not the brief schema. The author field names a totebox session identity which is not a valid owner value. |
| `.agent/briefs/BRIEF-substrate-phd-thesis-2026-05-27.md` | active | fix-schema | Missing required frontmatter fields: brief-id, owner. Uses schema: foundry-draft-v1 instead of foundry-brief-v1. Was named in NEXT.md brief-redistribution item as one of 6 project-intelligence BRIEFs needing relocation to clones/project-intelligence/.agent/briefs/ — may be misrouted. Updated field value is '2026-05-27' (same as created), suggesting no updates since initial creation. |
| `.agent/briefs/BRIEF-phase-fg-institutional-redesign.md` | archived | keep-active | Status is correctly set to archived; content was absorbed into BRIEF-artifact-style-guide.md §13. No action needed on the status itself. |
| `.agent/briefs/archive/BRIEF-project-intelligence-master.md` | relocated | rename-status | Status 'relocated' is not in the canonical five-value enum (active | reference | archived | superseded | stub). Should be changed to 'superseded' with a superseded_by field pointing to the canonical location in project-intelligence archive. |

## Consolidation opportunities

**Suggested: "os-totebox — PPN Deployment Master BRIEF"**
- Files: .agent/briefs/BRIEF-os-totebox-ppn-build-out.md, .agent/briefs/BRIEF-ostotebox-phase1-deployment.md
- Reason: Both BRIEFs cover the os-totebox deployment lifecycle in project-system. BRIEF-os-totebox-ppn-build-out.md is the session work log; BRIEF-ostotebox-phase1-deployment.md is the Phase 1 build plan. They share the same scope (os-totebox boot, service-stack, PPN deployment). Phase 1 milestone is now COMPLETE per NEXT.md (2026-06-12), so the phase-1-deployment BRIEF could be archived with its decisions carried forward into the ppn-build-out BRIEF.

## Missing governance

- README.md active-briefs table is empty — all 5 active BRIEFs are unlisted; table must be populated to comply with brief-discipline.md
- BRIEF-artifact-style-guide.md: brief-id field absent
- BRIEF-artifact-style-guide.md: schema field absent
- BRIEF-knowledge-platform-master.md: brief-id field absent
- BRIEF-knowledge-platform-master.md: schema field absent
- BRIEF-knowledge-platform-master.md: owner field absent
- BRIEF-knowledge-platform-master.md: created field absent
- BRIEF-os-totebox-ppn-build-out.md: brief-id field absent
- BRIEF-os-totebox-ppn-build-out.md: owner field absent
- BRIEF-ostotebox-phase1-deployment.md: brief-id field absent
- BRIEF-ostotebox-phase1-deployment.md: owner field absent
- BRIEF-ostotebox-phase1-deployment.md: schema is foundry-draft-v1, should be foundry-brief-v1
- BRIEF-substrate-phd-thesis-2026-05-27.md: brief-id field absent
- BRIEF-substrate-phd-thesis-2026-05-27.md: owner field absent
- BRIEF-substrate-phd-thesis-2026-05-27.md: schema is foundry-draft-v1, should be foundry-brief-v1
- archive/BRIEF-project-intelligence-master.md: non-canonical status value 'relocated' (valid values: active | reference | archived | superseded | stub)

## New BRIEFs needed

- **VM Architecture — VM-* naming and os-* product lineup**: NEXT.md line 10 explicitly references 'BRIEF-VM-ARCHITECTURE.md' as the authoritative source for VM-* naming conventions mirroring the os-* product lineup. The file does not exist. This is a dangling reference in the active working document, implying a BRIEF was intended but never created. VM naming discipline is a multi-session architectural decision that warrants its own durable BRIEF.

## Work log

2026-06-12 command@claude-code: Automated audit run. 5 active, 6 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `.agent/briefs/BRIEF-artifact-style-guide.md` — Missing required frontmatter fields: brief-id, schema. The owner field is present but set to 'project-editorial' (an archive name, not a role noun — violates §2.2 of the style guide it governs). No body sections in canonical order (Context → Scope → Decisions locked → Decisions open → Work log → Carry-forward) — structured as numbered paragraphs instead.
- [ ] fix-schema: `.agent/briefs/BRIEF-knowledge-platform-master.md` — Missing required frontmatter fields: brief-id, schema, owner, created. Has non-standard fields (cluster, supersedes, verdict_source) but lacks the canonical required set. The cluster field suggests this BRIEF may belong to project-knowledge archive rather than project-system.
- [ ] fix-schema: `.agent/briefs/BRIEF-os-totebox-ppn-build-out.md` — Missing required frontmatter fields: brief-id, owner. Contains archive: project-data which conflicts with its location in the project-system archive — likely a misrouted BRIEF from a prior archive contamination event. The NEXT.md brief-redistribution item names this file as one of 6 project-intelligence briefs that need Command relocation.
- [ ] fix-schema: `.agent/briefs/BRIEF-ostotebox-phase1-deployment.md` — Missing required frontmatter fields: brief-id, owner. Uses schema: foundry-draft-v1 instead of foundry-brief-v1 — this is the draft artifact schema, not the brief schema. The author field names a totebox session identity which is not a valid owner value.
- [ ] fix-schema: `.agent/briefs/BRIEF-substrate-phd-thesis-2026-05-27.md` — Missing required frontmatter fields: brief-id, owner. Uses schema: foundry-draft-v1 instead of foundry-brief-v1. Was named in NEXT.md brief-redistribution item as one of 6 project-intelligence BRIEFs needing relocation to clones/project-intelligence/.agent/briefs/ — may be misrouted. Updated field value is '2026-05-27' (same as created), suggesting no updates since initial creation.
- [ ] rename-status: `.agent/briefs/archive/BRIEF-project-intelligence-master.md` — Status 'relocated' is not in the canonical five-value enum (active | reference | archived | superseded | stub). Should be changed to 'superseded' with a superseded_by field pointing to the canonical location in project-intelligence archive.
- [ ] Consolidate .agent/briefs/BRIEF-os-totebox-ppn-build-out.md + .agent/briefs/BRIEF-ostotebox-phase1-deployment.md → "os-totebox — PPN Deployment Master BRIEF"
- [ ] Create BRIEF for: VM Architecture — VM-* naming and os-* product lineup
