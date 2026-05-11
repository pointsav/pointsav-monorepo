---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/tier-zero-customer-side-sovereign-specialist.md
audience: foundry-internal + vendor-public
bcsc_class: forward-looking-disclosure-controlled
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 49
research_done_count: 7
research_suggested_count: 3
open_questions_count: 1
research_provenance: master-empirical-tier-a-1b-validation + idc-smb-2026 + cross-industry
research_inline: true
---

# Tier 0 Customer-Side Sovereign Specialist

The Tier 0 customer deployment is a sovereign specialist running on the
customer's own hardware with no required cloud dependency.

This convention codifies Doctrine claim #49 (ratified v0.1.0).

## §1 — The Tier 0 reference unit

The reference Tier 0 deployment is a **Totebox** — a small-form-factor x86 or
ARM appliance with the Foundry stack:

- service-fs (Ring 1 WORM ledger; deterministic; approximately 100 MB binary)
- service-content (Ring 2 knowledge runtime; SQLite-graph and vector store;
  approximately 80 MB binary plus per-tenant graph data)
- service-slm Doorman (Ring 3 boundary; approximately 10 MB binary)
- Tier A 1B sysadmin specialist (OLMo 2 1B Instruct Q4 GGUF; approximately
  600 MB)
- slm-cli TUI (operator interface; approximately 5 MB binary)
- service-input, service-extraction, service-egress (approximately 20 MB each)

Total disk footprint: approximately 1 GB. Memory: 2–4 GB working set. CPU: 2–4
vCPU is sufficient for Tier A inference at 5–15 tokens/second alongside
concurrent Ring 1 and Ring 2 operations.

No GPU is required. No cloud dependency is required. No data egresses without
explicit per-record consent (per claim #52 and #53).

## §2 — Hardware reference designs

| Customer scale | Hardware | Intended TCO |
|---|---|---|
| 5-employee SMB | Mini-PC (Intel NUC class; ~$300–500) | $0/month operating |
| 30-person firm | Slightly larger appliance (~$800–1,500) | $0/month + optional Tier B (~$200/mo amortized) |
| 300-person firm or regional hospital | Multi-Totebox cluster + GPU box (~$5,000–15,000 hardware) | $0/month + Tier B + optional Tier C |

Foundry's intended commercial focus is the first three rows. The fourth row
(larger enterprise) exists but is not the primary target.

## §3 — Why "specialist" not "generalist"

Tier A on the Totebox is the OLMo 2 1B Instruct sysadmin specialist (per claim
#40 amendment). It is purpose-routed to:

- Sysadmin and IT-support questions
- Mechanical edits (commit messages, schema validation, frontmatter
  normalization)
- Routine queries about the customer's own audit ledger and graph
- Short-output tasks (approximately 200 tokens generation or less)

It is not the generalist that handles editorial, bilingual, or long-form
reasoning. Those tasks route to Tier B (Yo-Yo or customer GPU box) when
available, or are gracefully unavailable when Tier B is offline (per claim #54).

The specialist trains on the customer's own engineering corpus and TUI
interactions (per claim #45). Over time, the specialist is intended to become
tuned to that customer's environment — their systemd units, their seed taxonomy,
their workflow vocabulary.

## §4 — Why no GPU is required

Empirical evidence (workspace VM, 2026-04-30):
- OLMo 2 1B Instruct Q4_K_M (893 MB on disk)
- 4 vCPU, approximately 7 tokens/second real generation rate after Doorman overhead
- Approximately 6.3 seconds end-to-end for 40-token responses
- Quality: production-viable for sysadmin-class queries

This is fast enough for human-conversational pace. The customer operator types
a question; the specialist responds in seconds. No GPU acquisition, no driver
maintenance, no thermal management. The hardware is the same class as a
customer's other internal appliances (NAS, backup server, point-of-sale).

## §5 — Why "sovereign"

The customer's substrate operates without:

- Foundry-side servers (everything runs on customer hardware)
- AllenAI continuing to ship OLMo (existing GGUF files work indefinitely)
- Anthropic, Google, or OpenAI APIs (Tier C is opt-in; not required)
- Internet connectivity (the substrate works fully offline; updates pull when
  available)
- Cloud subscriptions (no cloud account required)

Per claim #54 (Substrate-Without-Inference Base Case), even the AI tier itself
is optional. The Totebox is the customer's property in the strongest sense.

## §6 — Market context

Per the IDC SMB 2026 Digital Landscape report [idc-smb-2026], on-premises is
the fastest-growing deployment type within the SMB software market. Per
Techaisle SMB 2026 predictions, the 2026 battleground is "the Corporate Brain —
a private, sovereign, persistent data fabric."

Foundry's claim is structural fit to this stated demand. Hyperscaler vendors
are structurally unable to deliver this because their economics require ongoing
data flow and recurring license revenue. Foundry's economics (per claim #53)
make sovereign deployment the default.

## §7 — Composition with other claims

- Claim #16 (Three-Ring Architecture)
- Claim #34 (Two-Bottoms Sovereign Substrate)
- Claim #40 amendment (Tier A purpose-routed specialist)
- Claim #43 (Single-Boundary Compute Discipline; Doorman is local)
- Claim #46 (MCP-as-Substrate; tools compose locally)
- Claim #47 (Seed Taxonomy as SMB Bootstrap; per-tenant graph)
- Claim #48 (Customer-Owned Graph IP)
- Claim #54 (Substrate-Without-Inference Base Case)

## §8 — Tier B and Tier C as opt-ins

**Tier B** (Yo-Yo or customer GPU box):
- Optional per tenant
- Customer chooses: PointSav-arranged Yo-Yo capacity or a customer-owned GPU box
- Used for editorial, bilingual, long-form, and reasoning tasks
- Routes through the customer's local Doorman (claim #43)

**Tier C** (external API):
- Optional per tenant; off by default
- Sovereignty-disclosed (the customer is informed when an external API is used)
- Allowlist-gated (specific labels: citation-grounding, initial-graph-build,
  entity-disambiguation)
- Most customers are intended to operate without Tier C entirely

## §9 — The first customer Totebox

The first customer Totebox (intended for Phase 7 of the leapfrog roadmap) is
Woodfine's. The customer-first-ordering convention means PointSav dogfoods the
same shape it ships to Woodfine. The workspace VM is the first Tier 0 instance.
Subsequent customer Toteboxes follow the same provisioning pattern.

## Provenance

Research reviewed: IDC SMB 2026 Digital Landscape report [idc-smb-2026];
Techaisle SMB 2026 predictions; empirical Tier A swap (OLMo 2 1B Q4 → 7
tokens/second on workspace VM CPU); Salesforce Agentforce SMB pricing;
Toast IQ AI pricing; Epic Cosmos AI pricing; Praxis EMR
sovereignty-on-prem-without-AI pattern (rural hospital precedent).

Suggested next research: (1) customer Totebox hardware reference designs
(mini-PC, SBC, embedded x86; TCO analysis per design); (2) first-customer-Totebox
provisioning runbook; (3) per-vertical Tier B utilization estimates.

**OQ #1 — Customer Totebox network model.** Does the Totebox connect to the
internet for updates only, or does it accept inbound connections (e.g., for
marketplace listings)? Operational decision gating Phase 7 implementation.

## References

- `DOCTRINE.md` claim #49
- Companion: all leapfrog claims (#43–#54) compose to enable Tier 0
- Empirical: workspace v0.1.96 Tier A swap validation
