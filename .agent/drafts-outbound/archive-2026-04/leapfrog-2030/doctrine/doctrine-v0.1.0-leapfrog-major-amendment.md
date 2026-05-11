---
schema: foundry-draft-v1
state: refined-pending-master-commit
originating_cluster: master-workspace
target_repo: ~/Foundry
target_path: DOCTRINE.md
audience: vendor-public + foundry-internal
bcsc_class: forward-looking-disclosure-controlled
language_protocol: PROSE-DOCTRINE
authored: 2026-04-30
authored_by: master @ /srv/foundry (session 642a40302788fe84)
authored_with: claude-opus-4-7-1m
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
references:
  - vendor/pointsav-monorepo/service-slm/docs/yoyo-training-substrate-and-service-content-integration.md
  - conventions/four-tier-slm-substrate.md
  - conventions/three-ring-architecture.md
  - conventions/apprenticeship-substrate.md
  - conventions/llm-substrate-decision.md
  - conventions/cluster-wiki-draft-pipeline.md
research_done_count: 28
research_suggested_count: 8
open_questions_count: 3
research_provenance: cluster-research-iter-24 + master-web-validation
research_inline: true
notes_for_editor: |
  This is the constitutional document for the leapfrog-2030 architectural moment.
  Claims #43, #44, #45 originate from project-slm Task iter-24 research; claims
  #46-#52 + #40 amendment originate from this Master synthesis session. Bloomberg-
  grade register requested. Bilingual overview not required (DOCTRINE.es.md has
  its own strategic-adaptation pattern). When refining, preserve claim numbering
  exactly — these are referenced from convention files, INVENTIONS.md, TOPICs,
  GUIDEs. Insertion point: §III claim portfolio, after existing claim #42 row.
---

# DOCTRINE.md v0.1.0 — Leapfrog-2030 MAJOR Amendment (proposed)

This document amends the Foundry Doctrine from v0.0.14 to **v0.1.0** with
twelve new claims (#43–#54) and one amendment to the existing claim #40.
The MAJOR bump is warranted because:

1. Three claims (#43, #46, #51) restructure how the substrate composes — they
   are not additive features but invariants that retroactively re-frame
   claims #16 (Three-Ring Architecture) and #32 (Apprenticeship Substrate).
2. Two claims (#48, #49) re-charter the customer relationship — the
   per-tenant graph becomes customer intellectual property, and Tier 0
   becomes a sovereign customer-side product, not a SaaS delivery.
3. One claim (#52, the Reverse-Flow Substrate) introduces a wholly new
   commercial mode (data marketplace and ad exchange) that the prior doctrine
   did not contemplate.
4. The claim #40 amendment (purpose-based vs size-based tier routing) changes
   the operational semantics of the Four-Tier SLM Substrate Ladder.

Together, these moves represent Foundry's deliberate divergence from the
hyperscaler AI-integration pattern and the formal commitment to a sovereign,
small-and-medium-business-first architecture. v0.1.0 is the version stamp on
that commitment.

---

## Amendment to claim #40 — Four-Tier SLM Substrate Ladder, purpose-routed

The original claim #40 (ratified v0.0.14, 2026-04-29) framed tier
differentiation as size-based: Tier A is OLMo 7B, Tier B is OLMo 32B, Tier C
is external API, with routing by `SLM_BRIEF_TIER_B_THRESHOLD_CHARS=500`. This
amendment re-charters the ladder as **purpose-based**:

| Tier | Model | Purpose | Lifetime |
|---|---|---|---|
| **A** | OLMo 2 1B Instruct (or successor) | Sysadmin / IT-support / mechanical edit / commit-msg / schema-validate | Always-on (CPU; 600 MB Q4_K_M) |
| **B** | OLMo 2 32B Instruct (or successor) | Editorial / bilingual / long-form / reasoning | On-demand (GPU; auto-wake on queue depth) |
| **C** | External API (Anthropic / Gemini / OpenAI) | Citation grounding / initial graph build / entity disambiguation | Rare; sovereignty-disclosed; allowlist-gated |

**Routing key changes from prompt size to `task_type`.** Each task type maps
to a default tier; an explicit `X-Foundry-Tier-Hint` header overrides the
default. The existing `SLM_BRIEF_TIER_B_THRESHOLD_CHARS` heuristic is
deprecated.

**The training trajectories diverge.** Tier A trains on the engineering
JSONL corpus (currently 355 events); it is intended to produce
`PointSav-OLMo-Sysadmin-1B`. Tier B trains on the prose-edit and
design-edit corpus; it is intended to produce `PointSav-OLMo-Editorial-32B`.
Tier C never trains.

**Rationale**: empirical evidence from the 2026-04-30 Tier A swap — OLMo 7B
Q4 on this VM at 0.02 tokens/second, replaced by OLMo 2 1B Q4 at
approximately 7 tokens/second, a 125× speedup with production-viable sysadmin
output — demonstrated that size-based routing produces a Tier A with no
distinct purpose. It becomes a slower Tier B. The amendment gives Tier A a
first-class identity: the always-on, narrow, sovereign specialist that runs on
customer hardware without a GPU.

This amendment composes with the `conventions/four-tier-slm-substrate.md`
v0.1.0 update (separate file).

---

## §III claim portfolio — additions

### Claim #43 — Single-Boundary Compute Discipline

> The Doorman (`service-slm`) is the single boundary point for all AI inference
> compute in every Foundry deployment. No process, session, or service accesses
> an inference tier (local, Yo-Yo, or external API) except through the Doorman.
> Bearer tokens, API keys, and compute endpoint URLs for all inference tiers
> live exclusively in the Doorman's configuration. Bypass attempts are audit
> violations, not convenience shortcuts. Enforcement is structural — firewall
> rules, UID-owner iptables on Tier A localhost, and bearer-only-in-Doorman —
> not policy-only.

**What it unlocks**: complete audit ledger coverage; cryptographic-grade
provenance for every AI inference call; minimal secret surface; unified cost
control across all tiers; a legally admissible audit record consistent with
BCSC continuous-disclosure posture [ni-51-102].

**What distinguishes this approach**: existing LLM gateway products (LiteLLM,
Portkey, Helicone) implement preferred-path routing — customers can choose to
use them; bypass is permitted by configuration. Foundry's claim is
exclusive-path with structural enforcement. The bearer token is structurally
inaccessible outside the Doorman process boundary, making the audit guarantee
structural rather than procedural.

**Cross-industry analogs**: ServiceNow CMDB (single source of truth for IT
asset state); Splunk Universal Forwarder (single path for log aggregation);
Kubernetes service mesh sidecars (mandatory traffic enforcement).

**Operational form**: `conventions/single-boundary-compute-discipline.md`.

---

### Claim #44 — Knowledge-Graph-Grounded Apprenticeship

> service-slm consults the service-content per-tenant knowledge graph before
> routing every substantive inference request. The atomic training tuple
> becomes (query, graph-context, response, verdict). Training on graph-grounded
> tuples produces adapters that generate graph-coherent responses: references
> to entities that exist in the graph, relationships consistent with the graph,
> and structured outputs that extend the graph on write-back. The datagraph
> and the adapter co-evolve — each accepted graph mutation increases the
> grounding quality of the next inference; each training cycle produces an
> adapter better at querying the graph.

**What it unlocks**: 30–40% hallucination reduction (per published GraphRAG
literature). Auditable answers (cited entity IDs traceable to graph nodes).
Measurable grounding quality over time (percentage of model responses citing
known graph entities, tracked in the audit ledger).

**What distinguishes this approach**: published GraphRAG patterns (Microsoft
Research, 2024) treat the graph as static input — built once from documents,
with no model feedback path. Foundry's claim is dynamic: the model's accepted
output updates the graph; the training loop rewards graph-coherent generation.
The co-evolution loop has no direct analog in published GraphRAG work through
early 2026.

**Cross-industry analogs**: Glean and Perplexity workspace context (answers
cite graph nodes); ServiceNow CMDB-as-gatekeeper (IT decisions query CMDB
first).

**Operational form**: `conventions/knowledge-graph-grounded-apprenticeship.md`
and an extension to `conventions/apprenticeship-substrate.md` §8 (the
`graph_context` field on the JSONL schema).

---

### Claim #45 — TUI-as-Corpus-Producer

> Every terminal interaction with service-slm through the System Administrator
> TUI (`slm-cli`) is a curated training corpus contribution. Sysadmin
> interactions are uniquely high-quality training signal: the ground truth is
> verifiable (the system either responds to administrative action correctly
> or it does not); the interaction domain is narrow (Totebox Archive
> operations, Foundry conventions, customer archive specifics); the operator
> providing feedback is the domain expert. The TUI implements a lightweight
> `/feedback [good|bad|refine]` mechanism producing (query, response, verdict)
> DPO triples. The IT-support adapter is expected to reach production quality
> with 200–500 high-quality interaction triples — an order of magnitude less
> data than code-generation tasks require, because the domain is narrow and
> ground truth is unambiguous.

**What it unlocks**: customer-facing corpus contribution that delivers value
while producing training signal. Every TUI session may improve the customer's
specific deployment adapter. This closes the customer-value and model-quality
loop in a form customers can understand and participate in.

**What distinguishes this approach**: existing AI sysadmin products treat the
operator as a passive beneficiary — the corpus contribution loop is not
visible to them. Foundry's claim is that the operator is an active corpus
contributor whose feedback is intended to improve their specific deployment's
model. This is the customer-owned-substrate principle (claim #28,
Designed-for-Breakout) applied to AI training data.

**Operational form**: `conventions/tui-corpus-producer.md` and an extension to
`conventions/apprenticeship-substrate.md` §7D (TUI verdict capture).

---

### Claim #46 — MCP-as-Substrate-Protocol

> Every Foundry Ring 1 and Ring 2 service exposes a Model Context Protocol
> (MCP) server interface as its primary external contract. The Doorman
> (`service-slm`) is the MCP gateway: it consumes content, extraction, fs,
> people, and other Ring services as MCP tools, dispatches to inference
> tiers, and exposes a unified `/v1/chat/completions` interface to operator
> surfaces. Customer-specific extensions plug in as additional MCP servers
> without modifying core services. MCP is the substrate-level wire contract
> for service composition; bespoke per-service wire formats are deprecated.

**What it unlocks**: industry-standard service composition [mcp-spec]. Customer
extensions are pluggable without core modification. Auditable tool-call
boundaries. Inter-service contract testing via MCP conformance suites. Foundry
MCP servers are interoperable with non-Foundry MCP clients (Claude Desktop,
IDE extensions, customer-built agents).

**What distinguishes this approach**: by early 2026, 28% of Fortune 500
companies had implemented MCP servers in their AI stacks (per CData Software
industry survey); 75% of API gateway vendors and 50% of iPaaS vendors are
projected to ship MCP features during 2026. Foundry's claim is that MCP is not
a feature layer but the *substrate protocol* — the same gateway that enforces
inference boundaries (claim #43) is the MCP entry point for service
composition.

**Cross-industry references**: MCP is described in the industry as the
"USB-C for AI-native applications" — a convergence point that
Foundry treats as a substrate commitment rather than a product option.

**Operational form**: `conventions/mcp-substrate-protocol.md`.

---

### Claim #47 — Seed Taxonomy as SMB Bootstrap

> Every Foundry tenant deployment provisions a four-part seed taxonomy as
> the bootstrap of its knowledge graph: **Archetypes** (5–7 role-by-cognitive-
> pattern identities), **Chart of Accounts** (5–10 industry-specific business
> profiles), **Domains** (3–5 macro categories of work — Corporate, Projects,
> Documentation as the universal default), **Themes** (time-bound initiatives
> that age out). Each entity carries `gravity_keywords` — explainable keyword
> anchors for classification, not embedding vectors. The taxonomy is hand-
> tunable, compact, and auditable. Customers customize it as they discover their
> own business reality. Industry-specific seed packs (claim #50) provide
> starter taxonomies for first-day operation.

**What it unlocks**: SMB onboarding in hours rather than weeks. No
ontology-engineering staff required. Classifications remain explainable (a
customer can read why an entity was categorized where it was). The taxonomy
travels with the customer's graph as their intellectual property (claim #48).

**What distinguishes this approach**: enterprise ontologies (Salesforce Data
Cloud's data-model-objects, ServiceNow CMDB classes, Microsoft Dataverse
tables) require enterprise-scale configuration teams and are optimized for
completeness across all possible customers. Foundry's seed is structurally the
inverse — start with 5–10 entities per category; evolve as you discover. The
`gravity_keywords` mechanism provides explainability that embedding-based
classification systems cannot match: a misclassification is correctable by
editing a keyword list, not by retraining a model.

**Empirical origin**: the existing Woodfine seed (5 Archetypes, 4 COA
profiles, 3 Domains, 4 Themes at `service-content/seeds/`) demonstrated the
pattern in production. Cross-industry mapping (a 5-employee restaurant, a
mid-size law firm, a regional hospital, a real estate firm) confirmed the form
is universal even as the substance differs.

**Operational form**: `conventions/seed-taxonomy-as-smb-bootstrap.md`.

---

### Claim #48 — Customer-Owned Graph IP

> The per-tenant knowledge graph held in service-content is the customer's
> intellectual property, not a side-effect of using the Foundry platform.
> Every node, edge, and mutation is owned by the tenant. Foundry has no
> claim to aggregate or resell tenant graph data without explicit per-tenant
> opt-in (claim #52, the Reverse-Flow Substrate, provides the consent and
> monetization mechanism for opt-in cases). The graph travels with the
> customer if they exit Foundry — the export format is open (graph, ontology,
> and audit ledger in JSONL and Cypher dump). The training adapters trained
> on the customer's tenant data are also the customer's property; per-tenant
> LoRA weights are portable artefacts, not Foundry assets.

**What it unlocks**: the customer relationship inverts. Foundry is a substrate
provider, not a data consumer. The customer may move their graph elsewhere;
the contract is on substrate quality, not lock-in. This creates a commercial
alignment: Foundry succeeds when the customer's graph becomes more valuable,
not when it becomes more locked-in.

**What distinguishes this approach**: the hyperscaler SaaS pattern shapes
customer data to the vendor's ontology; exit is a multi-year project; the
vendor's EULA retains aggregate-data rights. Foundry's claim is that the graph
is open from day one — the export is a routine operation, not a legal event.

**Cross-industry reference**: Snowflake's secure data-sharing approach points
in this direction but is priced at enterprise scale. Foundry's claim is the
same principle applied to SMB deployments.

**Operational form**: `conventions/customer-owned-graph-ip.md`.

---

### Claim #49 — Tier 0 Customer-Side Sovereign Specialist

> The Tier 0 customer deployment is a sovereign specialist running on the
> customer's own hardware with no required cloud dependency. The reference
> Tier 0 unit is the **Totebox**: a small-form-factor x86 or ARM appliance
> hosting service-fs (WORM ledger), service-content (knowledge runtime),
> service-slm (Doorman plus Tier A 1B sysadmin specialist), `slm-cli` (TUI
> operator surface), and service-input, service-extraction, and service-egress
> (Ring 1 and Ring 2 services). No GPU is required. No data egresses without
> explicit per-record consent (claim #52 mechanism). Tier B and Tier C are
> opt-in: the customer routes editorial and heavy work to a customer-owned GPU
> box, to PointSav-arranged Yo-Yo capacity, or declines those tiers entirely.

**What it unlocks**: the minimum viable AI-native stack for an SMB customer is
approximately $300–$500 hardware and $0/month operating cost, indefinitely. A
mid-size law firm or regional hospital deploys at estimated $5,000–$15,000
hardware, with optional Tier B amortized monthly. Comparable enterprise
document-management implementations (iManage, NetDocuments) carry
$50,000–$500,000 in implementation costs plus ongoing licensing. The cost
differential is structural, not negotiated.

**What distinguishes this approach**: the 2026 SMB market reports indicate
on-premises is the fastest-growing deployment type in SMB software [idc-smb-2026].
Hyperscaler vendors are structurally unable to serve this demand — their
economics require ongoing data flow and recurring license revenue. Foundry's
claim is that the substrate runs without those flows.

**Cross-industry comparison**: existing on-premises EHR vendors (Praxis EMR,
CPSI) have sovereignty but lack AI-native architecture. Existing AI vendors
(cloud-based legal-tech, restaurant-management AI) have AI capabilities but
require cloud egress. Foundry's claim is the structural intersection of
sovereignty and AI-native operation.

**Operational form**: `conventions/tier-zero-customer-side-sovereign-specialist.md`.

---

### Claim #50 — Vertical Seed Packs Marketplace

> Foundry intends to distribute industry-specific seed packs as starter taxonomies
> for Tier 0 customer deployments. Each pack is a curated bundle of Archetypes,
> Chart of Accounts profiles, Domain categories, and starter Themes tuned
> to a specific industry vertical. Reference packs are planned to include:
> `pack-restaurant-smb` (5-employee restaurant shape),
> `pack-law-firm-mid` (50–300 lawyer shape),
> `pack-hospital-regional` (small/rural hospital shape),
> `pack-real-estate-mid` (mid-firm shape — Woodfine reference).
> Customers install a pack, customize from there, and may contribute refinements
> back to the marketplace. The marketplace is intended to be curated, versioned,
> and permissively licensed.

**What it unlocks**: same-day onboarding for new tenants. Industry-specific
defaults matched to the customer's reality before they need to think about
ontology engineering. A community of customers potentially contributing pack
refinements over time.

**What distinguishes this approach**: hyperscaler vertical solutions are top-down,
monolithic, and priced at enterprise scale. Foundry's seed packs are intended
to be bottom-up, modular, and marketplace-distributed. The pack is compact
enough that a customer can read and understand the entire taxonomy in
approximately 30 minutes — a property that enterprise ontologies are not
designed to meet.

**Operational form**: `conventions/vertical-seed-packs-marketplace.md`.

---

### Claim #51 — Code-for-Machines First

> Every Foundry inter-service contract, audit record, configuration, and
> ontology is machine-readable as a primary surface. Inter-service
> communication is MCP (claim #46). Audit ledger is JSONL. Seed taxonomies
> are JSON. Doctrine and conventions are markdown with structured
> frontmatter. Per-tenant configuration is YAML. Human-facing surfaces (TUI,
> web UI, mobile) are skins on machine-first APIs — they consume the
> same MCP servers any other client would. There is no "human-only" data
> surface; everything is machine-mutable.

**What it unlocks**: every operator surface is an MCP client; every customer
integration is an MCP server; audit and observability are consistent because
the data is structured at every layer; documentation travels with the
machine-readable artefacts. Foundry deployments compose without integration
projects.

**What distinguishes this approach**: existing enterprise software treats human
UIs as primary and machine APIs as secondary — often retrofitted after the
fact. Foundry's claim inverts this. Machine APIs are primary; human UIs are
derived. Every new Foundry feature lands as MCP first and TUI or web second,
and never as a human-only surface.

**Operational form**: `conventions/code-for-machines-first.md`.

---

### Claim #52 — Reverse-Flow Substrate

> The same Doorman gateway, audit ledger, and per-tenant moduleId that
> enforce inbound discipline (claims #43, #46) also enforce outbound
> commercial flows. Two reverse flows are first-class:
>
> **Flow A — Data Marketplace.** The customer's accumulated graph, audit
> ledger, and adapter weights are saleable assets. The marketplace gateway
> (`service-marketplace`, Ring 2) is intended to expose per-tenant inventory
> to external buyers under explicit per-record consent and provenance
> signature. The customer retains majority of revenue; Foundry is planned to
> take a transaction percentage. Listings are intended to be vertical-specific:
> anonymized order patterns (restaurant), matter-type aggregations (law firm),
> care-quality patterns (hospital, de-identified per HIPAA),
> property/tenant patterns (real estate).
>
> **Flow B — Ad Exchange.** The customer is planned to operate as both seller
> and buyer in a standards-compliant (IAB OpenRTB 2.6+) ad exchange. As
> seller: their first-party audience (with consent and Tier A classification
> of intent) is real-time-bid inventory. As buyer: their adapter-trained
> intent model may target external campaigns at the per-impression boundary.
> The ad exchange gateway (`service-ad-exchange`, Ring 2) is intended to
> handle bid, impression, and billing reconciliation with the audit ledger as
> authoritative record.
>
> Both flows are opt-in per tenant; structurally disabled by default; require
> per-record cryptographic provenance from the audit ledger; and respect the
> per-tenant moduleId isolation that prevents cross-tenant leakage.

**What it unlocks**: SMB customers who hold data assets — customer-loyalty
patterns, referral-network insights, anonymized care-quality data, tenant-flow
data — gain a potential first-class revenue channel. The customer keeps the
data and the revenue; Foundry is intended to capture a transaction percentage
and substrate utility. This is planned to invert the SaaS economic model: the
customer is a potential revenue partner, not a license-payer.

**What distinguishes this approach**: existing data marketplace and CDP vendors
target enterprise sellers and buyers; the SMB seller has no practical path into
those markets today. Foundry's claim is that the sovereign Tier 0 deployment
is the data infrastructure — participation in data and ad markets is intended
to be a tenant configuration toggle, not a multi-month integration project.

**Cross-industry references**: IAB OpenRTB 2.6 (the ad exchange standard);
Snowflake Marketplace (the enterprise pattern Foundry intends to adapt for SMB);
Brave's BAT model (direct-payment-to-rights-holder validation at consumer scale).

**Operational form**: `conventions/reverse-flow-substrate.md`.

---

### Claim #53 — Direct-Payment Settlement

> Payment for data marketplace transactions and ad exchange revenue
> (per claim #52 Reverse-Flow Substrate) is intended to flow directly from
> buyer to tenant (the customer), not from buyer to platform to tenant.
> Foundry's share is planned to be taken as a transaction fee at settlement
> time, not as a recurring subscription. The settlement gateway
> (`service-settlement`, Ring 2) is intended to support two settlement rails:
> traditional banking (Stripe Connect or equivalent custodial-account model)
> and direct cryptocurrency (operator-chosen network; Sigstore Rekor anchored
> receipts). The audit ledger records the transaction; the customer's account
> receives the payment; Foundry's transaction fee is deducted at settlement
> with a full audit trail. Tenants are planned to opt into either or both rails.

**What it unlocks**: the customer's economic relationship with Foundry is
planned to invert. Foundry is paid only when the customer earns; the customer
is the recipient, not the payer. This composes with claim #48
(Customer-Owned Graph IP): if the data is the customer's IP, the revenue
from selling it should accrue to the customer, with Foundry taking a service
fee on the transaction, not a subscription fee on access.

**What distinguishes this approach**: existing data marketplaces intermediate
the payment — the marketplace collects from the buyer and remits to the seller
after fees and payout-cycle delays. Foundry's intended model is direct
settlement: the buyer pays the seller through the settlement gateway acting as
a routing layer, not a custody layer; Foundry deducts its fee at the moment of
transaction. This removes custody-of-funds risk, float, and payout-cycle
delays for the customer. Brave's BAT model demonstrates that direct-payment-
to-rights-holder works at scale (approximately 60 million monthly active users
in 2026).

**Why direct-payment matters for SMB**: a small business has no treasury
operations; 30–60-day marketplace payout cycles impose operational overhead
and counterparty risk. A regional hospital may have compliance preferences for
direct payment flows. Direct-payment settlement is the structurally simpler
model for the customer; Foundry assumes the platform complexity of integrating
banking and crypto rails to deliver that simplicity.

**Operational form**: `conventions/direct-payment-settlement.md`.

---

### Claim #54 — Substrate-Without-Inference Base Case

> The Totebox Archive must remain operationally functional and freely
> transferable even when service-slm cannot run any inference. Tier A
> unavailable, Tier B unreachable, Tier C disabled — the base-case
> substrate (service-fs WORM ledger, service-content graph and vector
> store, service-input/extraction/egress ingest pipeline, slm-cli
> TUI in deterministic-only mode) continues to provide full data
> sovereignty operations: query, audit, keyword and vector search,
> export, and transfer of ownership. AI inference is value-add, not
> load-bearing. The "freely transferable" property requires that
> nothing of structural importance to the customer's business depend
> on Foundry, on AllenAI continuing to ship OLMo, on Anthropic, Google,
> or OpenAI APIs remaining available, or on any external provider.

**What it unlocks**: the Totebox is genuinely the customer's property
in the strongest sense. It may be:
- Transferred to a new operator without re-training, re-licensing, or
  Foundry involvement
- Sold (with data) without any SaaS handover
- Inherited or transferred at the legal level (estate planning, business
  sale, acquisition) without service interruption
- Operated for years as a deterministic information system if the
  customer chooses not to enable any AI tier

**What distinguishes this approach**: existing sovereign-data products are
typically sovereign at the data layer but require platform availability for
operations (for example, a backup vendor's offline backups still need their
restore tooling). Foundry's claim is full operational sovereignty: the
Totebox boots, runs, queries, audits, and exports without any Foundry-side
dependency. The slm-cli TUI in deterministic-only mode is a complete sysadmin
interface — the operator can do everything except ask AI questions.

**Cross-claim alignment**:
- Claim #16 (Three-Ring Architecture; Ring 3 structurally optional) —
  this claim makes the optionality a customer-facing commercial commitment
- Claim #34 (Two-Bottoms Sovereign Substrate) — the substrate is designed
  to boot on customer hardware without infrastructure dependency on Foundry
- Claim #48 (Customer-Owned Graph IP) — the IP only matters if the
  customer can operate it independently
- Claim #49 (Tier 0 Customer-Side Sovereign Specialist) — Tier 0 is the
  AI-enabled mode; this claim defines the AI-disabled mode that always works

**The deterministic-only mode in slm-cli**: when no inference tier is
available, the TUI:
- Disables natural-language chat (returns "AI tier unavailable; use slash
  commands for deterministic operations")
- Keeps all slash commands operational (`/status`, `/audit`, `/graph`,
  `/search`, `/export`, `/help`)
- Shows a clear UI indication of mode ("AI-disabled — operating
  deterministically")
- Continues to capture corpus events for the audit ledger (so that when
  AI returns, training resumes from the captured baseline)

**Operational form**: `conventions/substrate-without-inference-base-case.md`.

---

## Implementation sequencing

Per the rebuild roadmap (indexed at
`~/Foundry/.claude/drafts-outbound/leapfrog-2030/INDEX.md`), doctrine
ratification at v0.1.0 is Phase 1 — the constitutional moment. Phases 2–7
implement the rebuild over an intended 4–8 weeks. The rebuild is paced by:

- project-data Task (cluster scope: service-content, service-extraction,
  service-input, service-fs, service-people rebuilds; absorbs
  service-content per the operator decision 2026-04-30T05:30Z)
- project-slm Task (cluster scope: Doorman MCP gateway, brief shape v2,
  slm-cli TUI, auto-wake Yo-Yo, adapter loader, first LoRA cycle)
- Master scope (workspace-tier doctrine ratification, deployment instances,
  verdict-signing in sweep cadence, cross-cluster broadcast)
- New project-marketplace cluster (proposed; cluster scope:
  service-marketplace, service-ad-exchange; reverse-flow gateway services;
  separate from project-data because the commercial logic is its own domain)

## Versioning rules applied

Per `~/Foundry/CLAUDE.md` §7:
- This is a doctrine MAJOR bump: 0.0.14 → **0.1.0**
- Companion convention files are versioned to match (each carries
  `doctrine_version: 0.1.0` in frontmatter)
- Workspace `MANIFEST.md` `doctrine_version` field updates
- Tag v0.1.0 (annotated, SSH-signed per §3)
- CHANGELOG.md receives a chapter entry (MAJOR = chapter)

## Research trail

### Done (28 items)

The full research trail spans project-slm Task iter-24 work and Master
synthesis. Detailed citations are preserved in source documents:

- Three-Ring Architecture composition (claim #16, ratified v0.0.4)
- Apprenticeship Substrate (claim #32, ratified v0.0.13 with §7B amendment)
- Four-Tier SLM Substrate Ladder (claim #40, ratified v0.0.14 — amended here)
- LLM substrate decision (OLMo L3 fully-open per `conventions/llm-substrate-decision.md`)
- service-slm trainer scoping (10,837-word document by project-slm Task iter-24)
- KuzuDB acquisition by Apple (October 2025); LadybugDB community fork status
- MCP 2026 roadmap (OAuth 2.1, audit, gateway patterns) [mcp-spec]
- Salesforce SMB pricing 2026
- Toast POS knowledge-management gaps
- iManage and NetDocuments enterprise legal DMS pricing
- Epic and Cerner EHR market share and AI capabilities
- IDC SMB 2026 Digital Landscape report [idc-smb-2026]
- Techaisle SMB 2026 predictions ("Corporate Brain")
- IAB OpenRTB 2.6 specification
- Snowflake Marketplace and LiveRamp data-exchange patterns
- ServiceNow CMDB single-source-of-truth pattern
- Splunk Universal Forwarder log-aggregation pattern
- Kubernetes Istio/Linkerd service mesh enforcement
- Microsoft GraphRAG research (2024)
- Glean and Perplexity workspace-context patterns
- Praxis EMR ambient-documentation pattern (rural and regional hospitals)
- Empirical Tier A swap (OLMo 7B 0.02 t/s → OLMo 2 1B 7 t/s on workspace VM)
- Direct code reads of service-content, service-extraction, service-input
- Existing Woodfine seed taxonomy (Archetypes, COA, Domains, Themes)
- Doorman audit ledger format (foundry-audit-ledger-v1)
- Engineering corpus capture rate (40–60 tuples/day across 9 clusters)
- §7C Brief Queue Substrate operational state (queue, drain, reaper)
- Yo-Yo manual fast-path operational state (L4 spot approximately $0.18/hr)
- Apprenticeship corpus state (16 prose-edit and 4 shadow-capture tuples)

### Suggested (8 items)

Future research the next leg should pursue:

1. LadybugDB conformance testing on real Cypher workloads (vs SQLite-graph)
2. IAB OpenRTB SDK availability in Rust ecosystem (for service-ad-exchange)
3. HIPAA Safe Harbor de-identification techniques for hospital data marketplace listings
4. Snowflake Marketplace technical integration patterns (for data marketplace UX precedent)
5. OLMo 2 1B QLoRA training on engineering corpus — validation cycle cost on L4
6. Per-vertical seed pack starter content authoring (4 packs × approximately 2 hours each)
7. MCP authentication patterns (OAuth 2.1 + tenant scoping) for marketplace gateway
8. Tier 0 hardware reference designs (mini-PC vs SBC vs embedded x86) — TCO analysis

### Open questions (3 items)

Questions that gate Phase 2+ implementation:

**OQ #1 — Marketplace cluster scope.** Should service-marketplace and
service-ad-exchange live in a new `project-marketplace` cluster, be absorbed
into project-data (sibling to service-content), or be held as project-slm
scope (sibling to Doorman audit-ledger logic)?
*(Operator decision; routes the rebuild assignment.)*

**OQ #2 — Foundry transaction percentage.** What share of marketplace
revenue does Foundry retain? The pricing model affects customer commercial
appeal and Foundry recurring revenue.
*(Operator decision; documented in `conventions/reverse-flow-substrate.md`
once chosen.)*

**OQ #3 — First marketplace launch tenant.** Woodfine is the dogfood
tenant; do they participate in the data marketplace as the launch case, or
does Foundry recruit a separate launch tenant for the marketplace gateway?
*(Operator and Woodfine decision; gates Phase 4 marketplace launch.)*

---

## What this MAJOR amendment does NOT change

- Existing claims #1–#42 remain in force (claim #40 amended, not retired)
- Existing convention files referenced from this amendment continue to
  apply: `three-ring-architecture.md`, `compounding-substrate.md`,
  `economic-model.md`, `bcsc-disclosure-posture.md`, `apprenticeship-substrate.md`,
  `meta-repo-pattern.md`, `cluster-wiki-draft-pipeline.md`,
  `model-tier-discipline.md`, `customer-first-ordering.md`,
  `root-files-discipline.md`, `language-protocol-substrate.md`
- Operational mailbox protocol (§VI), versioning rules (§VII), and
  layer scope (§V) are unchanged
- Existing deployments continue to operate; the rebuild is incremental,
  not a flag day

---

## Companion documents (separate drafts in this batch)

Each new claim has a dedicated convention file in
`~/Foundry/.claude/drafts-outbound/leapfrog-2030/conventions/`:

1. `convention-single-boundary-compute-discipline.draft.md` (claim #43)
2. `convention-knowledge-graph-grounded-apprenticeship.draft.md` (claim #44)
3. `convention-tui-corpus-producer.draft.md` (claim #45)
4. `convention-mcp-substrate-protocol.draft.md` (claim #46)
5. `convention-seed-taxonomy-as-smb-bootstrap.draft.md` (claim #47)
6. `convention-customer-owned-graph-ip.draft.md` (claim #48)
7. `convention-tier-zero-customer-side-sovereign-specialist.draft.md` (claim #49)
8. `convention-vertical-seed-packs-marketplace.draft.md` (claim #50)
9. `convention-code-for-machines-first.draft.md` (claim #51)
10. `convention-reverse-flow-substrate.draft.md` (claim #52)
11. `convention-direct-payment-settlement.draft.md` (claim #53)
12. `convention-substrate-without-inference-base-case.draft.md` (claim #54)

Plus the amendment to existing `conventions/four-tier-slm-substrate.md` for
the purpose-routing semantics change to claim #40.

---

## Provenance

This major amendment synthesizes project-slm Task iter-24 research (claims
#43, #44, #45) and Master Claude leapfrog-synthesis session (workspace v0.1.96,
2026-04-30, claims #46–#54, plus the #40 amendment). The empirical Tier A swap
(OLMo 2 1B Q4 at approximately 7 tokens/second on workspace VM CPU, replacing
OLMo 7B at 0.02 tokens/second) is documented in workspace v0.1.96 CHANGELOG.
External research sources are enumerated in the Research trail above; citation
IDs against `~/Foundry/citations.yaml` are noted inline where registered.
