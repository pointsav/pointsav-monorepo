---
artifact: brief
schema: foundry-brief-v1
brief-id: project-software-brief-audit-2026-06
title: "BRIEF audit — project-software — 2026-06"
status: active
owner: project-software
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

All 5 BRIEFs present in project-software are project-intelligence scope contamination (M-17); none cover the archive's actual mission (software.pointsav.com storefront and distribution substrate), and all are missing multiple required frontmatter fields including schema, brief-id, and owner.

Active: 4 / Total: 5

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
| `BRIEF-dev-env-mcp-expansion.md` | archived | fix-schema | Missing required frontmatter fields: schema, brief-id, title, owner, updated. Also covers project-intelligence scope (slm-mcp-server, service-slm crates) — not the project-software mission (software.pointsav.com storefront). Likely M-17 scope contamination. Status is already archived, so fix-schema is the minimum; operator should confirm whether this belongs in project-intelligence briefs/ instead. |
| `BRIEF-project-intelligence-active-work.md` | active | migrate-to-archive → project-intelligence | Entirely project-intelligence scope: covers Doorman circuit breaker, service-content memory hardening, GIS python3 incident, apprenticeship queue drain — none of which belong to project-software (storefront/marketplace/wallet). Missing required frontmatter: schema, brief-id, title, owner. Appears to be M-17 scope contamination. Should be migrated to clones/project-intelligence/.agent/briefs/ and removed from this archive. |
| `BRIEF-project-intelligence-master.md` | active | migrate-to-archive → project-intelligence | Entirely project-intelligence scope: covers service-slm, service-content, app-orchestration-slm, OLMo tier routing, DataGraph, Yo-Yo GPU infrastructure. Missing required frontmatter: brief-id, owner. This is the primary plan-of-record for project-intelligence engineering — it should live in that archive, not project-software. M-17 scope contamination. |
| `BRIEF-slm-learning-loop.md` | active | migrate-to-archive → project-intelligence | Entirely project-intelligence scope: covers OLMo LoRA training pipeline, DPO corpus quality, Yo-Yo enrichment cycles, apprenticeship substrate — unrelated to software distribution. Missing required frontmatter: schema, brief-id, owner. M-17 scope contamination. Should live in clones/project-intelligence/.agent/briefs/. |
| `AI-AUDIT-baseline-2026-05-31.md` | active | migrate-to-archive → project-intelligence | Not named BRIEF-*.md (convention violation). Covers project-intelligence ecosystem audit (service-slm, service-content, app-orchestration-slm). Missing required frontmatter: schema, brief-id, title (in frontmatter), owner, updated. M-17 scope contamination. Should be renamed BRIEF-ai-audit-baseline-2026-05-31.md and migrated to clones/project-intelligence/.agent/briefs/. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- README.md active-briefs table is empty — none of the 4 active BRIEFs are listed in it
- All 4 active BRIEFs are missing required frontmatter field: schema (foundry-brief-v1)
- All 4 active BRIEFs are missing required frontmatter field: brief-id (<archive>-<slug>)
- All 4 active BRIEFs are missing required frontmatter field: owner
- BRIEF-dev-env-mcp-expansion.md missing: updated field
- BRIEF-project-intelligence-active-work.md missing: title field
- BRIEF-slm-learning-loop.md missing: brief-id, schema, owner fields
- AI-AUDIT-baseline-2026-05-31.md does not follow BRIEF-*.md naming convention
- All 5 files cover project-intelligence domain scope — none cover project-software mission (software.pointsav.com storefront, marketplace, wallet); likely M-17 scope contamination

## New BRIEFs needed

- **software-distribution-substrate**: The archive mission is software.pointsav.com — app-privategit-source (port 9201), app-privategit-marketplace (port 9202), tool-wallet. Session context shows active editorial work (6 TOPIC drafts staged 2026-06-12) and Stage 6 promotion pending. Multi-session tracking is warranted; no BRIEF exists for this core domain.
- **crypto-license-payment-architecture**: Polygon USDC payment flow, Ed25519 license key issuance, BIP-32 HD address derivation, and flat-file receipt system are active engineering concerns surfaced in session-context. Decisions locked include pricing model, USDC-only, FSL vs Apache 2.0 tiers. A BRIEF is needed to preserve these decisions across sessions.

## Work log

2026-06-12 command@claude-code: Automated audit run. 4 active, 5 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `BRIEF-dev-env-mcp-expansion.md` — Missing required frontmatter fields: schema, brief-id, title, owner, updated. Also covers project-intelligence scope (slm-mcp-server, service-slm crates) — not the project-software mission (software.pointsav.com storefront). Likely M-17 scope contamination. Status is already archived, so fix-schema is the minimum; operator should confirm whether this belongs in project-intelligence briefs/ instead.
- [ ] migrate-to-archive: `BRIEF-project-intelligence-active-work.md` — Entirely project-intelligence scope: covers Doorman circuit breaker, service-content memory hardening, GIS python3 incident, apprenticeship queue drain — none of which belong to project-software (storefront/marketplace/wallet). Missing required frontmatter: schema, brief-id, title, owner. Appears to be M-17 scope contamination. Should be migrated to clones/project-intelligence/.agent/briefs/ and removed from this archive.
- [ ] migrate-to-archive: `BRIEF-project-intelligence-master.md` — Entirely project-intelligence scope: covers service-slm, service-content, app-orchestration-slm, OLMo tier routing, DataGraph, Yo-Yo GPU infrastructure. Missing required frontmatter: brief-id, owner. This is the primary plan-of-record for project-intelligence engineering — it should live in that archive, not project-software. M-17 scope contamination.
- [ ] migrate-to-archive: `BRIEF-slm-learning-loop.md` — Entirely project-intelligence scope: covers OLMo LoRA training pipeline, DPO corpus quality, Yo-Yo enrichment cycles, apprenticeship substrate — unrelated to software distribution. Missing required frontmatter: schema, brief-id, owner. M-17 scope contamination. Should live in clones/project-intelligence/.agent/briefs/.
- [ ] migrate-to-archive: `AI-AUDIT-baseline-2026-05-31.md` — Not named BRIEF-*.md (convention violation). Covers project-intelligence ecosystem audit (service-slm, service-content, app-orchestration-slm). Missing required frontmatter: schema, brief-id, title (in frontmatter), owner, updated. M-17 scope contamination. Should be renamed BRIEF-ai-audit-baseline-2026-05-31.md and migrated to clones/project-intelligence/.agent/briefs/.
- [ ] Create BRIEF for: software-distribution-substrate
- [ ] Create BRIEF for: crypto-license-payment-architecture
