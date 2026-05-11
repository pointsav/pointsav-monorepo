---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/reverse-flow-substrate.md
audience: foundry-internal + vendor-public
bcsc_class: forward-looking-disclosure-controlled
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 52
research_done_count: 11
research_suggested_count: 5
open_questions_count: 3
research_provenance: master-web-research-iab-openrtb + snowflake-marketplace + brave-bat
research_inline: true
---

# Reverse-Flow Substrate

The same Doorman gateway, audit ledger, and per-tenant moduleId that enforce
inbound discipline are intended to also enforce outbound commercial flows. Two
reverse flows are planned as first-class: Data Marketplace and Ad Exchange.
Both are opt-in per tenant; structurally disabled by default; and require
per-record cryptographic provenance.

This convention codifies Doctrine claim #52 (ratified v0.1.0). It is the
operational form for the intended Phase 5 implementation of `service-marketplace`
and `service-ad-exchange`.

## §1 — The two reverse flows

### Flow A — Data Marketplace

The customer's accumulated graph, audit ledger, and adapter weights are saleable
assets. The marketplace gateway (`service-marketplace`, Ring 2) is intended to
expose per-tenant inventory to external buyers.

**Planned saleable inventory categories**:

- **Graph-derived patterns**: anonymized aggregate queries against the per-tenant
  graph (e.g., "monthly distribution of customer order patterns" for a
  restaurant, "matter-type frequency by quarter" for a law firm)
- **Adapter weights**: per-tenant LoRA adapters trained on the customer's
  specific data — saleable to other tenants in the same vertical (with explicit
  permission)
- **Curated datasets**: the customer's content-wiki content, tagged with consent
  and license terms
- **Audit-proven training data**: verdict-signed corpus tuples sold as training
  data to AI labs (with cryptographic provenance)

**Planned inventory shape** (aligned with IAB Data Transparency standard):

```
Listing: {
  listing_id: "ULID",
  tenant: "module_id",
  category: "graph-pattern" | "adapter-weights" | "curated-dataset" | "training-corpus",
  description: "...",
  schema: { fields: [...] },
  sample_records: 5-10,
  consent_terms: "...",
  provenance_hash: "sha256:...",
  pricing: { model: "per-record" | "per-query" | "flat", currency: "USD|crypto", amount: ... },
  audit_chain: ["audit_id_1", "audit_id_2", ...]
}
```

### Flow B — Ad Exchange

The customer is planned to operate as both seller and buyer in a
standards-compliant (IAB OpenRTB 2.6+) ad exchange. The ad exchange gateway
(`service-ad-exchange`, Ring 2) is intended to implement IAB OpenRTB 2.6+ for
real-time bidding.

**As seller**: the customer's first-party audience (with per-record consent and
Tier A intent classification) is real-time-bid inventory. Buyers (advertisers)
bid for impressions. The customer's audience is addressable via opaque
tenant-scoped identifiers — no cross-tenant matching; per-tenant moduleId
isolation is preserved.

**As buyer**: the customer's tenant adapter may classify their audience intent.
The ad-exchange gateway may target external campaigns at the per-impression
boundary on behalf of the customer (e.g., a restaurant advertising a new menu
item to local audiences via partner networks). The customer's budget controls
the bid.

## §2 — Per-tenant opt-in discipline

Reverse flows are structurally disabled by default. To enable:

1. The operator authenticates as the tenant's primary identity
2. The operator runs `slm-cli /marketplace enable` (or `/ad-exchange enable`)
3. The TUI guides the operator through:
   - Inventory category selection (which kinds of inventory to expose)
   - Consent-record review (per claim #48, the operator affirms each consent term)
   - Pricing configuration
   - Settlement rail selection (per claim #53)
4. The configuration is signed by the operator's identity key and recorded in
   the audit ledger
5. service-marketplace begins exposing the listings; service-ad-exchange begins
   accepting bids

The TUI flow is the structural enforcement. There is no API path that enables
marketplace or ad-exchange without operator-signed consent.

## §3 — Audit ledger as proof-of-rights

Every transaction in either reverse flow is intended to record:

- The consent terms in force at transaction time
- The audit chain leading to the inventory (provenance from source data through
  to listing)
- The buyer's identity (or opaque token for ad-exchange impression buyers)
- The settlement event (when payment confirms; cross-references claim #53)

The audit ledger is the intended legally admissible record of: that the data
sold was sold with consent; that the consent in force matched the transaction's
terms; that the settlement happened according to agreed pricing; and that no
cross-tenant data leakage occurred. Per BCSC continuous-disclosure posture
[ni-51-102], this record is suitable for legal review.

## §4 — Per-tenant moduleId isolation in reverse flows

The same moduleId discipline that prevents cross-tenant graph traversal prevents
cross-tenant inventory leakage. service-marketplace listings are scoped by
tenant; a buyer cannot query "all restaurant inventory" without explicit
per-tenant grants. service-ad-exchange impression inventory is opaque-token-bound;
the buyer never sees the tenant's underlying audience.

This is the structural distinction from CDP and marketplace vendors that combine
audience data across tenants for advertiser targeting. Foundry's claim is that
combining is the customer's decision, not the platform's.

## §5 — Cross-industry inventory examples

| Vertical | Marketplace inventory | Ad exchange role |
|---|---|---|
| **Restaurant (5-employee)** | Anonymized order patterns; consenting customer base demographics | Seller: their audience to local CPG / restaurant supply; buyer: their customers via location-targeted ads |
| **Law firm (300-lawyer)** | Anonymized matter-type frequencies (e.g., "M&A activity in Toronto Q3 2026") | Seller: addressable audience to legal-tech ISVs; buyer: lateral-hire campaigns to law-school networks |
| **Regional hospital** | De-identified clinical patterns (HIPAA Safe Harbor); patient-outreach audiences (consented) | Seller: opt-in patient outreach segments; buyer: health-related advertising for own services (preventive screening reminders) |
| **Real estate firm (Woodfine)** | Property/tenant flow patterns; investor segment data | Seller: investor audience to financial publishers; buyer: tenant-prospecting campaigns |

The pattern is consistent: each customer holds data assets specific to their
vertical that are valuable to specific buyers. The marketplace is intended to
match them under consent, provenance, and per-tenant isolation.

## §6 — IAB OpenRTB compliance for ad exchange

The intended `service-ad-exchange` implementation aligns with IAB OpenRTB 2.6+
for the bid-stream interface:

- Bid requests carry impression-level targeting (no PII; opaque audience tokens)
- Bid responses carry creative and bid price
- Win notifications trigger settlement events (claim #53)
- Privacy signals (GDPR, CCPA, Canadian privacy framework) carried in the bid
  request per IAB privacy framework

This is intended to make Foundry-side audiences buyable by any IAB-compliant
DSP without bespoke integration.

## §7 — Snowflake Marketplace alignment for data marketplace

The intended `service-marketplace` listing format aligns with Snowflake's
secure-data-sharing patterns where applicable:

- Listings described by schema and sample
- Buyers may query before purchasing (limited free queries)
- Subscription and one-time purchase models supported
- Schema versioning preserves backwards compatibility for existing buyers

The structural distinction: Snowflake Marketplace is enterprise-priced and
assumes the seller has Snowflake infrastructure. Foundry's claim is that the
sovereign Tier 0 deployment is the marketplace infrastructure; participation is
a tenant-configuration toggle.

## §8 — Composition with other claims

- Claim #43 (Single-Boundary): all marketplace and ad-exchange traffic is
  intended to route through the Doorman; outbound auth and consent enforcement
  at the same boundary as inbound inference
- Claim #48 (Customer-Owned Graph IP): the inventory is the customer's IP;
  reverse flows are how that IP may earn
- Claim #46 (MCP-as-Substrate): service-marketplace and service-ad-exchange
  expose MCP servers; customer extensions plug in
- Claim #53 (Direct-Payment Settlement): payment flows directly to the customer;
  service-settlement is the Ring 2 settlement gateway

## Provenance

Research reviewed: IAB OpenRTB 2.6 specification and privacy framework; Snowflake
Marketplace technical patterns and secure data sharing; Brave BAT direct-payment
model (cross-claim #53 reference); LiveRamp connectivity model (audience
activation precedent); Salesforce Data Cloud Activations (what Foundry diverges
from); Adobe Real-Time CDP (similar comparison); HIPAA Safe Harbor
de-identification; cross-industry mapping (four verticals); IAB Data
Transparency standard alignment; per-tenant moduleId isolation in existing
service-content; BCSC continuous-disclosure audit-record requirements [ni-51-102].

Suggested next research: (1) IAB OpenRTB Rust SDK survey (which crates support
2.6+; project-slm Task scope during Phase 5); (2) Snowflake marketplace listing
UX patterns; (3) HIPAA Safe Harbor de-identification techniques for clinical
inventory listings; (4) per-vertical pricing-model templates; (5) cross-tenant
aggregation policy when tenants opt-in to combined listings.

**OQ #1 — Marketplace cluster scope.** Where do `service-marketplace` and
`service-ad-exchange` live operationally? Options: (a) new `project-marketplace`
cluster, (b) absorbed into project-data, (c) absorbed into project-slm. Pending
operator decision; routes Phase 5 dispatch.

**OQ #2 — First marketplace launch tenant.** Woodfine (dogfood) or external
launch tenant? Affects Phase 5 readiness criteria.

**OQ #3 — Cross-tenant aggregation governance.** When multiple tenants in the
same vertical opt into combined listings (e.g., ten restaurants aggregating
their patterns), what is the governance mechanism? This question is the inverse
of claim #50's pack-derivation question; both require operator decisions.

## References

- `DOCTRINE.md` claim #52
- Companion: `conventions/single-boundary-compute-discipline.md` (claim #43)
- Companion: `conventions/customer-owned-graph-ip.md` (claim #48)
- Companion: `conventions/direct-payment-settlement.md` (claim #53)
- External: IAB OpenRTB 2.6 specification
- External: Snowflake Marketplace documentation
- Cross-claim: Brave BAT model (claim #53 reference)
