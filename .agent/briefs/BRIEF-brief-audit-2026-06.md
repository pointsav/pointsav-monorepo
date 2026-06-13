---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-brief-audit-2026-06
title: "BRIEF audit — project-intelligence — 2026-06"
status: archived
owner: project-intelligence
created: 2026-06-12
updated: 2026-06-13
archived: 2026-06-13
status_note: Core findings actioned — governance commit 6de9ce0b (2026-06-12); carry-forward items absorbed into BRIEF-slm-learning-loop.md §16
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

project-intelligence has 4–5 active BRIEFs (one, BRIEF-slm-substrate-master, is declared superseded by BRIEF-project-intelligence-master but still carries status: active); every BRIEF is missing multiple required frontmatter fields (schema, brief-id, owner); the README.md active-briefs table is empty; and AI-AUDIT-baseline-2026-05-31.md lacks the BRIEF- filename prefix and foundry-brief-v1 schema.

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
| `BRIEF-slm-substrate-master.md` | active | rename-status | BRIEF-project-intelligence-master.md explicitly lists this in its supersedes: field and declares itself PRIMARY PLAN OF RECORD. The substrate master predates the master BRIEF and covers a subset of the same domain. It should carry status: superseded, not status: active, so it does not count against the soft cap. |
| `BRIEF-project-intelligence-active-work.md` | active | fix-schema | Missing required frontmatter fields: schema (must be foundry-brief-v1), brief-id (must be project-intelligence-<slug>), title, and owner. Body does not follow the required section order (Context → Scope → Decisions locked → Decisions open → Work log → Carry-forward). |
| `BRIEF-project-intelligence-master.md` | active | fix-schema | Missing required frontmatter fields: brief-id (must be project-intelligence-<slug>) and owner. Schema field is present. This is the primary plan of record and should be the first BRIEF corrected. |
| `BRIEF-slm-learning-loop.md` | active | fix-schema | Missing required frontmatter fields: schema (must be foundry-brief-v1), brief-id (must be project-intelligence-<slug>), and owner. The updated field is present and current (2026-06-12). |
| `BRIEF-dev-env-mcp-expansion.md` | archived | fix-schema | Missing required frontmatter fields: schema, brief-id, title, and owner. Even archived BRIEFs are permanent git-tracked artifacts and should carry complete frontmatter per brief-discipline.md. |
| `AI-AUDIT-baseline-2026-05-31.md` | active | fix-schema | This file does not follow the BRIEF-*.md naming convention, uses the wrong schema (no schema field; frontmatter is gemini-cli authored and lacks foundry-brief-v1 structure), and is missing brief-id, title, owner, and updated fields. It should be renamed to BRIEF-ai-audit-baseline-2026-05-31.md and its frontmatter brought into compliance, or superseded by a properly formed BRIEF if its content has been absorbed elsewhere. |
| `BRIEF-vm-hardening-and-consolidation.md` | archived | keep-active | Already archived with a contamination note and absorbed_into reference. Frontmatter is incomplete (missing schema, brief-id, owner) but the BRIEF is correctly retired. No action required beyond noting the missing fields in a future fix-schema pass. |
| `BRIEF-substrate-phd-thesis-2026-05-27.md` | archived | fix-schema | Uses schema: foundry-journal-v1 and artifact_type: JOURNAL rather than artifact: brief / schema: foundry-brief-v1. Correctly marked archived. Missing brief-id and owner. If this is intended as an archived BRIEF it needs the correct schema; if it is a JOURNAL artifact it should be routed to project-editorial drafts-outbound, not stored in .agent/briefs/. |

## Consolidation opportunities

**Suggested: "project-intelligence Master BRIEF — Sovereign AI Platform for SMBs (with integrated work log)"**
- Files: BRIEF-project-intelligence-active-work.md, BRIEF-project-intelligence-master.md
- Reason: Both files cover project-intelligence session state and active work planning. BRIEF-project-intelligence-master.md is the declared plan of record (§15 open items, §2 live state, §16 sprint plan). BRIEF-project-intelligence-active-work.md duplicates live state (§mem memory incident, §0 resolved items, §2 next items) that belongs in the master BRIEF's work log. The active-work BRIEF should be folded into the master as a Work log section and then archived.

## Missing governance

- README.md active-briefs table is empty — no rows; all active BRIEFs are unlisted
- brief-id field absent from all BRIEFs: BRIEF-project-intelligence-active-work.md, BRIEF-project-intelligence-master.md, BRIEF-slm-learning-loop.md, BRIEF-slm-substrate-master.md, BRIEF-dev-env-mcp-expansion.md, AI-AUDIT-baseline-2026-05-31.md, BRIEF-vm-hardening-and-consolidation.md, BRIEF-substrate-phd-thesis-2026-05-27.md
- owner field absent from all BRIEFs (same 8 files)
- schema field absent from: BRIEF-project-intelligence-active-work.md, BRIEF-slm-learning-loop.md, BRIEF-slm-substrate-master.md, BRIEF-dev-env-mcp-expansion.md, AI-AUDIT-baseline-2026-05-31.md, BRIEF-vm-hardening-and-consolidation.md
- AI-AUDIT-baseline-2026-05-31.md does not follow BRIEF-*.md filename convention
- BRIEF-slm-substrate-master.md carries status: active despite being declared superseded by BRIEF-project-intelligence-master.md
- BRIEF-substrate-phd-thesis-2026-05-27.md uses schema: foundry-journal-v1 instead of schema: foundry-brief-v1 — wrong artifact type stored in briefs/

## New BRIEFs needed

- **Tier B GPU restoration and zone strategy**: BRIEF-slm-learning-loop.md §14 (2026-06-12 audit) identifies restoring Tier B GPU as the single blocker for DPO pairs and training. This is a multi-session infrastructure decision (zone selection, yoyo-batch reprovisioning, fallback policy) that is orthogonal to the learning loop BRIEF's training pipeline scope and warrants its own trackable artifact.
- **Phase 4b DataGraph sweep ledger bug and enrichment pipeline correctness**: BRIEF-slm-learning-loop.md §14 (2026-06-12 audit) identifies two high-priority code faults — SHA ledger written on 202-queue ACK instead of enrichment success (permanently poisoning 400 commits) and SLM_YOYO_WEIGHTS_GCS_BUCKET unset. These are distinct from the training pipeline design (learning loop BRIEF) and from infrastructure (Tier B BRIEF), and have enough open decisions (ledger fix, GCS bucket setup, training budget floor) to warrant a dedicated BRIEF for multi-session tracking.

## Work log

2026-06-12 command@claude-code: Automated audit run. 4 active, 8 total BRIEFs reviewed.

## Carry-forward

- [ ] rename-status: `BRIEF-slm-substrate-master.md` — BRIEF-project-intelligence-master.md explicitly lists this in its supersedes: field and declares itself PRIMARY PLAN OF RECORD. The substrate master predates the master BRIEF and covers a subset of the same domain. It should carry status: superseded, not status: active, so it does not count against the soft cap.
- [ ] fix-schema: `BRIEF-project-intelligence-active-work.md` — Missing required frontmatter fields: schema (must be foundry-brief-v1), brief-id (must be project-intelligence-<slug>), title, and owner. Body does not follow the required section order (Context → Scope → Decisions locked → Decisions open → Work log → Carry-forward).
- [ ] fix-schema: `BRIEF-project-intelligence-master.md` — Missing required frontmatter fields: brief-id (must be project-intelligence-<slug>) and owner. Schema field is present. This is the primary plan of record and should be the first BRIEF corrected.
- [ ] fix-schema: `BRIEF-slm-learning-loop.md` — Missing required frontmatter fields: schema (must be foundry-brief-v1), brief-id (must be project-intelligence-<slug>), and owner. The updated field is present and current (2026-06-12).
- [ ] fix-schema: `BRIEF-dev-env-mcp-expansion.md` — Missing required frontmatter fields: schema, brief-id, title, and owner. Even archived BRIEFs are permanent git-tracked artifacts and should carry complete frontmatter per brief-discipline.md.
- [ ] fix-schema: `AI-AUDIT-baseline-2026-05-31.md` — This file does not follow the BRIEF-*.md naming convention, uses the wrong schema (no schema field; frontmatter is gemini-cli authored and lacks foundry-brief-v1 structure), and is missing brief-id, title, owner, and updated fields. It should be renamed to BRIEF-ai-audit-baseline-2026-05-31.md and its frontmatter brought into compliance, or superseded by a properly formed BRIEF if its content has been absorbed elsewhere.
- [ ] fix-schema: `BRIEF-substrate-phd-thesis-2026-05-27.md` — Uses schema: foundry-journal-v1 and artifact_type: JOURNAL rather than artifact: brief / schema: foundry-brief-v1. Correctly marked archived. Missing brief-id and owner. If this is intended as an archived BRIEF it needs the correct schema; if it is a JOURNAL artifact it should be routed to project-editorial drafts-outbound, not stored in .agent/briefs/.
- [ ] Consolidate BRIEF-project-intelligence-active-work.md + BRIEF-project-intelligence-master.md → "project-intelligence Master BRIEF — Sovereign AI Platform for SMBs (with integrated work log)"
- [ ] Create BRIEF for: Tier B GPU restoration and zone strategy
- [ ] Create BRIEF for: Phase 4b DataGraph sweep ledger bug and enrichment pipeline correctness
