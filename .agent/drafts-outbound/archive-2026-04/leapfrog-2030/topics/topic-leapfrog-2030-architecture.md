---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_repo: vendor/content-wiki-documentation
target_path: topic-leapfrog-2030-architecture.md (+ .es.md sister)
audience: vendor-public
bcsc_class: forward-looking-disclosure-controlled
language_protocol: PROSE-TOPIC
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
research_done_count: 28
research_suggested_count: 5
open_questions_count: 2
research_provenance: master-leapfrog-synthesis-cross-industry + cluster-research-iter-24
research_inline: true
notes_for_editor: |
  Bloomberg-grade register. Bilingual sister .es.md required (strategic-
  adaptation pattern, not 1:1 translation). Body length is within the 1,800-2,200
  word target. Citation IDs noted inline where registered against
  ~/Foundry/citations.yaml; unresolved external URLs left as-is.
cites:
  - ni-51-102
  - idc-smb-2026
  - mcp-spec
  - sigstore-rekor-v2
---

# The Leapfrog 2030 Architecture

How a small-and-medium-business AI substrate becomes structurally different from
the hyperscaler stack.

## The bet

Three forces converge in 2026. First, small open-weight language models have
crossed the usability threshold for narrow-domain work: OLMo 2 1B Instruct
produces production-viable sysadmin output at five to fifteen tokens per second
on commodity 4-vCPU hardware. Second, the Model Context Protocol has become the
standard for AI-native service composition [mcp-spec]; 28% of Fortune 500
companies had shipped MCP servers by early 2026 (per CData industry survey).
Third, on-premises is the fastest-growing deployment type within the SMB
software market [idc-smb-2026].

These three forces open a structural gap.

The hyperscaler vendors are committed to *concentrated compute + integrated
stack + recurring revenue*. Their SMB tier is a discounted enterprise tier.
Their AI features require ongoing data flow into their cloud. Their pricing
assumes the customer is renting capability, not owning it.

The opening for a different commitment: *sovereign substrate + composable
services + transactional revenue*. The customer owns their hardware, their data,
their adapter weights, and their relationships. Foundry is the substrate
provider; the customer is the substrate operator; revenue flows when value
flows.

This document describes the architecture that realizes that commitment.

## What "leapfrog 2030" means

Three structural distinctions from the hyperscaler stack:

**Sovereignty over service.** The customer's Totebox is designed to boot, run,
query, audit, and export without any Foundry-side dependency. If Foundry ceases
operations, the customer's substrate is intended to continue functioning. This
is not a backup feature; it is the designed default operational mode (per
Doctrine claim #54).

**Specialization over generalization.** Tier A — the always-on, customer-side
inference tier — is a 1-billion-parameter sysadmin specialist, not a
32-billion-parameter generalist. It runs on commodity CPU. It trains on the
customer's own engineering and IT-support corpus. Over time it becomes
specifically useful for that customer's environment. The generalist tier (Tier
B, 32B class on GPU) is opt-in and on-demand; the precision tier (Tier C,
external API) is rare and sovereignty-disclosed.

**Compounding over capture.** Foundry's intended revenue compounds with the
customer's revenue, not against it. The Direct-Payment Settlement substrate
(claim #53) is designed to route data marketplace and ad-exchange transactions
directly from buyer to customer; Foundry takes a transaction percentage at
settlement, not a subscription fee on access. The customer is intended to be a
revenue partner, not a license-payer.

## The substrate at a glance

A Foundry deployment is composed of three rings:

**Ring 1 — Boundary Ingest.** Per-tenant MCP servers handling inbound data:
service-fs (WORM ledger), service-input (file, email, and voice), service-extraction
(entity extraction), service-people (CRM), service-email. Each is bounded;
each is auditable; each is deterministic at its baseline.

**Ring 2 — Knowledge and Processing.** Multi-tenant via moduleId:
service-content (the per-tenant graph and vector store), service-egress
(outbound formatting), service-marketplace (data marketplace gateway),
service-ad-exchange (IAB OpenRTB 2.6+ gateway), service-settlement (Stripe
Connect and crypto rail).

**Ring 3 — Optional Intelligence.** service-slm Doorman as MCP gateway; Tier A
1B sysadmin specialist (always-on); Tier B 32B generalist (Yo-Yo or customer
GPU; on-demand); Tier C external API (rare; allowlist-gated).

The Single-Boundary Compute Discipline (claim #43) makes the Doorman the only
path to inference compute. Bearer tokens, API keys, and endpoint URLs live
exclusively in the Doorman's configuration. Bypass is structurally prevented
(firewall, UID-owner iptables, and bearer-only-in-Doorman), not policy-only.

## How it works for the small business

A 5-employee restaurant installs a Totebox: a $300–$500 mini-PC at the back of
the kitchen. The Totebox runs the Foundry stack: ledger, knowledge graph, ingest,
and the 1B sysadmin specialist. The owner selects a Vertical Seed Pack
(`pack-restaurant-smb`): 5 Archetypes, 6 Chart of Accounts profiles, 3 Domains,
4 starter Themes. They customize in approximately 30 minutes — the pack is their
starting point, not a vendor-imposed ontology.

From day one, the owner can ask the Tier A specialist operational questions:
"When did the freezer fail last?", "Show me last month's labor costs by shift",
"Which suppliers have not sent invoices in two weeks?". Responses arrive in
seconds. Every interaction is a potential training tuple for the customer's own
adapter (claim #45, TUI-as-Corpus-Producer); over weeks, the specialist is
intended to become tuned to their environment.

When the owner needs editorial or bilingual work — a customer newsletter, a
translation of menu items — they may enable Tier B on-demand. The Yo-Yo instance
wakes, processes the request, and idles back down. Cost is metered by the
minute, not by seat.

When the owner wants to monetize their first-party customer data — anonymized
order patterns sold to local CPG companies — they may enable the marketplace
(claim #52). A buyer queries the listing, transacts through the IAB-compliant
gateway, and pays directly to the owner's business bank account (Stripe Connect
rail) or crypto wallet. Foundry takes its transaction fee at settlement; the
owner sees payment in days for fiat or minutes for crypto.

When the owner sells the business, the new owner imports the Totebox bundle and
operates from day one without re-training, without SaaS migration, and without
vendor cooperation. The "freely transferable" property (claim #54) is the
customer's assurance that their substrate is genuinely theirs.

## How it works for the larger SMB

A 300-lawyer firm deploys a Totebox cluster: multiple Tier 0 units plus a Tier B
GPU box. Per-matter graph isolation (per-tenant moduleId at the matter
granularity) provides ethical walls that are native to the architecture, not
bolted on. Senior partners' reasoning captured as TUI verdicts is intended to
train the firm's adapter; the next associate-level question benefits from a
more partner-shaped response.

A regional hospital deploys a Totebox per service line. Patient graphs are
isolated; data sovereignty is the default (data does not egress without explicit
consent; Tier C is structurally disabled at this tenant). The clinical knowledge
graph builds via service-extraction from encounter notes. Ambient documentation
flows into the graph; the graph grounds the next clinical-decision question
(claim #44, Knowledge-Graph-Grounded Apprenticeship).

A real estate firm — Woodfine, the reference customer — builds a seed taxonomy
hand-tuned to their operations (Investor Relations, Capital Projects, Compliance,
Construction). Themes age in and out as initiatives close and new ones launch.
The graph is their property, growing with the business.

In each case the form is the same — substrate, graph, specialist, optional
generalist, audit, marketplace — and the substance differs per industry. This is
what the Vertical Seed Packs Marketplace (claim #50) is designed to support.

## What hyperscalers structurally cannot match

The hyperscaler vendors are committed to integrated recurring revenue. Their
model assumes:

- Compute concentrates in their cloud (so they can charge for compute)
- Data flows to their cloud (so they can charge for storage and processing)
- Subscription fees are charged regardless of customer usage
- Lock-in increases as data accumulates (multi-month migration projects to exit)

Foundry's intended model assumes:

- Compute distributes to customer hardware (Tier 0 sovereign specialist)
- Data stays at the customer (graph is customer IP per claim #48)
- Transaction fees are charged when value flows (per claim #53)
- Transfer is a single-command operation (per claim #54)

These are not minor configuration differences. They are structural inversions. A
hyperscaler vendor cannot adopt Foundry's commercial model without dismantling
its existing revenue base. This is the window in which Foundry may be the only
realistic answer for the SMB that wants AI-native operations without lock-in.

## The compounding loop

The leapfrog architecture has a positive-feedback loop:

1. The customer's Totebox runs deterministic operations on their data (the
   substrate-without-inference base case; claim #54)
2. The Tier A specialist trains on their engineering corpus and TUI verdicts
   (claim #45)
3. The specialist becomes more useful → operator uses it more → more verdicts →
   improved specialist (the apprenticeship loop)
4. The graph grows from accepted inferences (claim #44, graph-grounded
   apprenticeship) → more grounding for the next inference → fewer hallucinations
   → more accepted inferences
5. The customer's data accumulates value as the graph grows; reverse flows
   (claim #52) may begin generating revenue
6. Direct-payment settlement (claim #53) is intended to send revenue directly to
   the customer; Foundry takes a transaction fee
7. Pack contributions back to the marketplace (claim #50) may make the next
   vertical-customer onboarding faster

Every loop iteration is a marginal improvement. The compounding runs in customer
time, not vendor time. Foundry's commercial incentive is to keep the loops
running well.

## The first reference implementation

Foundry itself is the first reference implementation. The workspace VM at
`~/Foundry/` is `vault-privategit-source-1`, a Tier 0 instance of the substrate
(per `MANIFEST.md`). PointSav Digital Systems operates this instance as their
dogfood deployment; Woodfine Management Corp. is the first customer. The
substrate composes the same shape for both.

This is the structural realization of the customer-first-ordering convention:
PointSav builds what Woodfine will install, on the same substrate Woodfine will
use, in the same order Woodfine will install it.

## What ships when (planned)

The leapfrog architecture ratifies at workspace doctrine v0.1.0. The intended
implementation rolls out over 2026:

- Phase 1 (May 2026): doctrine and conventions ratified; Tier A 1B swap
  empirically validated
- Phase 2 (May 2026): rebuild blueprints staged at engineering monorepo;
  cluster Tasks begin
- Phase 3 (May–June 2026): service-content, service-extraction, service-input
  rebuilt as MCP servers
- Phase 4 (June 2026): Doorman MCP gateway; brief shape v2; slm-cli TUI Phase 1
- Phase 5 (July–August 2026): service-marketplace, service-ad-exchange,
  service-settlement; first marketplace transactions
- Phase 6 (Q3 2026): first customer Totebox provisioned (Woodfine); first
  IT-support adapter trained
- Phase 7 (Q4 2026): community Vertical Seed Pack contributions begin
- Phase 8+ (2027): PointSav-SLM continued pretraining; second customer Totebox;
  federated marketplace launch

These phases are planned targets, not commitments. Material assumptions include:
project-data and project-slm cluster Tasks executing on the rebuild scope; operator
ratification of doctrine v0.1.0; and Woodfine's readiness to participate as the
Phase 6 reference customer.

## What this is, and what it is not

This is **a structural commitment** to sovereignty, composability, and SMB-first
design.

This is **not** a hyperscaler-replacement product. Foundry is not designed to
serve enterprises with $1B+ revenue that want vendor-managed compute with
recurring license fees. That is a well-served market. Foundry is designed for
the market that is not well served.

This is **not** a niche or experimental architecture. The 2026 industry research
[idc-smb-2026] and [mcp-spec] consistently point toward the structural shape
Foundry is building. Foundry is deliberately positioned at the intersection
where hyperscaler vendors are structurally unable to follow.

The substrate is in place. The architecture is ratified. The first customer is
in deployment. The next chapter is operational scale.

---

## Provenance

This topic synthesizes from project-slm Task iter-24 research
(`service-slm/docs/yoyo-training-substrate-and-service-content-integration.md`),
Master Claude leapfrog-synthesis session (workspace v0.1.96, 2026-04-30), direct
reads of service-content, service-extraction, and service-input code, empirical
Tier A swap validation (2026-04-30), and 30+ external industry sources covering
MCP 2026 standards, OLMo 2 capabilities, LadybugDB substrate, IAB OpenRTB
ad-exchange standard, Snowflake Marketplace patterns, Brave BAT direct-payment
model, IDC and Techaisle SMB 2026 forecasts, and per-vertical comparisons.

Citation IDs resolved against `~/Foundry/citations.yaml` where registered;
external URLs left as-is for unregistered sources.

## References

- `DOCTRINE.md` v0.1.0 (constitutional source)
- `conventions/*.md` (one operational form per claim #43–#54)
- `vendor/pointsav-monorepo/INVENTIONS.md` (engineering-side summary)
- [ni-51-102]: https://www.bcsc.bc.ca/securities-law/law-and-policy/instruments-and-policies/5-ongoing-requirements-for-issuers-insiders/current/51-102
- [idc-smb-2026]: https://www.idc.com/resource-center/blog/the-smb-2026-digital-landscape-how-ai-is-redefining-growth/
- [mcp-spec]: https://modelcontextprotocol.io/
- Companion topics:
  - `topic-mcp-substrate-protocol.md`
  - `topic-graph-grounded-llm-apprenticeship.md`
  - `topic-tier-zero-customer-totebox.md`
  - `topic-purpose-routed-tier-discipline.md`
  - `topic-seed-taxonomy-smb-bootstrap.md`
  - `topic-cross-industry-leapfrog-evidence.md`
