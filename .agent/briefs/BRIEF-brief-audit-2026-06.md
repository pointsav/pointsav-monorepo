---
artifact: brief
schema: foundry-brief-v1
brief-id: project-infrastructure-brief-audit-2026-06
title: "BRIEF audit — project-infrastructure — 2026-06"
status: active
owner: project-infrastructure
created: 2026-06-12
updated: 2026-06-12
---

## Context
Automated BRIEF audit run 2026-06-12 (command@claude-code, Phase D governance sprint).
Applies governance rules from `conventions/brief-discipline.md` (committed 2026-06-12).

Two active BRIEFs are present and below the 5-BRIEF soft cap, but both have schema violations — one uses the wrong schema value and both are missing required frontmatter fields; four of five archived BRIEFs still carry status: active in their frontmatter despite physical placement in archive/.

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
| `BRIEF-substrate-phd-thesis-2026-05-27.md` | active | fix-schema | Uses schema: foundry-draft-v1 instead of the required schema: foundry-brief-v1. Missing required fields: brief-id (should be project-infrastructure-substrate-phd-thesis) and owner (has author instead). All other required frontmatter fields (artifact, title, status, created, updated) are present. |
| `BRIEF-totebox-transformation.md` | active | fix-schema | Missing five of the seven required frontmatter fields: schema, brief-id, title, owner, and updated. Frontmatter contains only artifact, status, created, supersedes, and cross-ref. The BRIEF body is also very large (700+ lines covering §1–§18) and has grown to encompass the PPN resource pool and customer virtualization layer — domains that could be split into a sibling BRIEF. |
| `archive/BRIEF-PPN-ARCHITECTURE.md` | active | rename-status | Physically in briefs/archive/ and listed as archived in README, but frontmatter still reads status: active. Should be updated to status: archived to match physical placement. |
| `archive/BRIEF-PPN-DEV-BOOTSTRAP.md` | active | rename-status | Physically in briefs/archive/ and listed as archived in README, but frontmatter reads status: active. Should be updated to status: archived. |
| `archive/BRIEF-OS-FAMILY.md` | active | rename-status | Has duplicate status: fields in frontmatter (status: archived followed by status: active due to copy-paste during archiving). The active value shadows the archived one. Remove the duplicate; keep status: archived. |
| `archive/BRIEF-LEAPFROG-2030.md` | active | rename-status | Physically in briefs/archive/ and listed as archived in README, but frontmatter reads status: active. Should be updated to status: archived. |
| `archive/BRIEF-VM-ARCHITECTURE.md` | archived | keep-active | Correctly marked status: archived and physically in archive/. No action needed. |

## Consolidation opportunities

_No consolidation opportunities identified._

## Missing governance

- BRIEF-totebox-transformation.md is missing required frontmatter fields: schema, brief-id, title, owner, updated
- BRIEF-substrate-phd-thesis-2026-05-27.md uses schema: foundry-draft-v1 instead of required schema: foundry-brief-v1
- BRIEF-substrate-phd-thesis-2026-05-27.md missing required fields: brief-id, owner
- Four archived BRIEFs in archive/ have status: active in frontmatter, contradicting their physical placement
- README.md frontmatter requirements table is incomplete — lists only artifact and status, omitting schema, brief-id, owner, created, updated from the required set per brief-discipline.md

## New BRIEFs needed

- **PPN resource pool and customer virtualization layer (service-vm-fleet, service-vm-host, service-vm-tenant)**: BRIEF-totebox-transformation.md has grown to 700+ lines and §§13–18 document a distinct, independently-archivable domain: the VM resource pool end-to-end test, customer virtualization auth layer, Laptop A/B node enrollment, placement policy, and os-network-admin control-plane role analysis. This work has multi-session carry-forward and open operator decisions (overlay transport choice: VXLAN vs nested WireGuard; IPAM datastore design) that warrant their own BRIEF rather than appending further to the transformation BRIEF.

## Work log

2026-06-12 command@claude-code: Automated audit run. 2 active, 7 total BRIEFs reviewed.

## Carry-forward

- [ ] fix-schema: `BRIEF-substrate-phd-thesis-2026-05-27.md` — Uses schema: foundry-draft-v1 instead of the required schema: foundry-brief-v1. Missing required fields: brief-id (should be project-infrastructure-substrate-phd-thesis) and owner (has author instead). All other required frontmatter fields (artifact, title, status, created, updated) are present.
- [ ] fix-schema: `BRIEF-totebox-transformation.md` — Missing five of the seven required frontmatter fields: schema, brief-id, title, owner, and updated. Frontmatter contains only artifact, status, created, supersedes, and cross-ref. The BRIEF body is also very large (700+ lines covering §1–§18) and has grown to encompass the PPN resource pool and customer virtualization layer — domains that could be split into a sibling BRIEF.
- [ ] rename-status: `archive/BRIEF-PPN-ARCHITECTURE.md` — Physically in briefs/archive/ and listed as archived in README, but frontmatter still reads status: active. Should be updated to status: archived to match physical placement.
- [ ] rename-status: `archive/BRIEF-PPN-DEV-BOOTSTRAP.md` — Physically in briefs/archive/ and listed as archived in README, but frontmatter reads status: active. Should be updated to status: archived.
- [ ] rename-status: `archive/BRIEF-OS-FAMILY.md` — Has duplicate status: fields in frontmatter (status: archived followed by status: active due to copy-paste during archiving). The active value shadows the archived one. Remove the duplicate; keep status: archived.
- [ ] rename-status: `archive/BRIEF-LEAPFROG-2030.md` — Physically in briefs/archive/ and listed as archived in README, but frontmatter reads status: active. Should be updated to status: archived.
- [ ] Create BRIEF for: PPN resource pool and customer virtualization layer (service-vm-fleet, service-vm-host, service-vm-tenant)
