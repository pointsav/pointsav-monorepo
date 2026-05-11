---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/customer-owned-graph-ip.md
audience: foundry-internal + vendor-public
bcsc_class: forward-looking-disclosure-controlled
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 48
research_done_count: 5
research_suggested_count: 3
open_questions_count: 1
research_provenance: master-cross-industry-saas-lock-in-comparison
research_inline: true
---

# Customer-Owned Graph IP

The per-tenant knowledge graph held in service-content is the customer's
intellectual property, not a side-effect of using the Foundry platform.

This convention codifies Doctrine claim #48 (ratified v0.1.0).

## §1 — The principle

Every node, edge, mutation, and audit-ledger entry scoped to a tenant's
`module_id` is owned by that tenant. Foundry has no claim to aggregate, resell,
or use tenant graph data outside the explicit per-tenant opt-ins defined in
claim #52 (Reverse-Flow Substrate) and claim #53 (Direct-Payment Settlement).
The training adapters trained on the customer's tenant data are also the
customer's property; per-tenant LoRA weights are portable artefacts.

## §2 — What "ownership" means operationally

- **Export at any time**: `slm-cli /export` produces a complete bundle (graph,
  audit ledger, adapters, seed taxonomy) suitable for transfer to another
  Totebox, another vendor, or independent operation
- **No vendor approval required**: export is a routine operation, not a legal
  event
- **No format lock-in**: the export is open (Cypher dump for graph, JSONL for
  audit, GGUF or safetensors for adapters)
- **No aggregate license**: Foundry does not retain rights to use the customer's
  data to train cross-tenant models without explicit per-tenant opt-in
- **Transfer of ownership** (composition with claim #54): the customer may sell,
  gift, or transfer the Totebox and bundle without Foundry's involvement

## §3 — Why this inverts the SaaS pattern

Hyperscaler SaaS pattern:
- Customer data is shaped to the vendor's ontology
- Exit is a multi-month migration project requiring vendor cooperation
- Vendor retains aggregate-data rights in the EULA
- Subscription pricing means the customer effectively rents their own data shape

Foundry pattern:
- Customer data is shaped by their own seed taxonomy (claim #47)
- Export is a single-command operation
- Foundry has no aggregate-data rights without per-tenant opt-in
- Transaction-fee pricing (claim #53) means the customer pays Foundry only when
  they earn; access is unmetered

## §4 — Composition with other claims

- Claim #34 (Two-Bottoms Sovereign Substrate): the substrate does not depend on
  Foundry; this claim makes the data on the substrate the customer's property
- Claim #54 (Substrate-Without-Inference Base Case): the customer may operate
  independently; this claim makes them the legal owner
- Claim #52 (Reverse-Flow Substrate): the customer owns the data; their
  monetization decisions are theirs
- Claim #53 (Direct-Payment Settlement): revenue from monetizing customer-owned
  IP goes to the customer
- Claim #47 (Seed Taxonomy as SMB Bootstrap): the customer's customizations to
  the seed taxonomy are their additions to their IP

## §5 — Practical operator-facing form

In the slm-cli TUI, the operator sees:
- A "your data" framing in the welcome screen and status bar
- `/export` prominently listed in `/help` output
- An audit log that includes "data left this Totebox" events when the operator
  initiates marketplace listings or transfers

## Provenance

Research reviewed: Salesforce, iManage, and Epic SaaS lock-in patterns;
Snowflake Secure Data Sharing approach (enterprise-priced precedent);
estate-planning for digital assets — legal precedents; Foundry's existing
Sigstore Rekor anchoring (Doctrine Invention #7); Brave BAT
direct-payment-to-rights-holder model.

Suggested next research: (1) customer-facing data-ownership disclosure language
(legal review; composes with BCSC posture); (2) export bundle format
specification; (3) multi-jurisdiction ownership transfer mechanics.

**OQ #1 — Foundry-side training data.** Does Foundry retain rights to use
customer-owned graph data for improving Foundry's general adapters (e.g., the
engineering corpus that accumulates from all clusters)? The current behavior is
that engineering corpus is PointSav-tenant data (PointSav is dogfood); Woodfine
corpus stays Woodfine-tenant. The policy requires explicit codification.
Pending operator decision.

## References

- `DOCTRINE.md` claim #48
- Companion: `conventions/two-bottoms-sovereign-substrate.md` (claim #34)
- Companion: `conventions/substrate-without-inference-base-case.md` (claim #54)
- Companion: `conventions/reverse-flow-substrate.md` (claim #52)
- Companion: `conventions/direct-payment-settlement.md` (claim #53)
