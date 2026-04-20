# MARKETPLACE.md
# Data Marketplace and Ad Exchange
**Version:** 1 · April 19, 2026
**Service:** service-marketplace (one service, two modes)
**Phase:** Designed in Phase 1 for correct data categorization. Production deployment Phase 3.

---

## Why One Service

Both modes read the same derivative data (Layers 2–4). Both require the same consent/privacy
layer. Both use IAB taxonomy standards. The operational difference is a protocol difference —
batch licensing vs. real-time bidding — not a service boundary. Keeping them as one service
reduces operational complexity and maintains minimum services principle.

---

## The Core Principle

**Base assets (Layer 0) are NEVER sold.** Base assets are the owner's immutable legal record.

**Derivatives (Layers 2–5) are what is sold.** The derivative layers are structured, standardized,
compliance-anchored outputs that can be packaged and licensed. The derivation chain from source
document through WORM seal through CoA classification through derivative synthesis is
machine-verifiable. No other commercial source offers this provenance depth.

---

## Mode 1: Data Marketplace

### What It Is

A structured data licensing platform where buyers purchase versioned, SOC3-attested,
DARP-compliant extracts from the derivative layers.

**What is sold:** L2–L4 derivative snapshots.
**Who buys:** Analysts, researchers, institutional data consumers, ESG platforms, real estate data
aggregators, financial data services.
**Why it is valuable:** The full chain of custody from source document through WORM seal through
CoA classification is machine-verifiable. Independently verified data (via service-verification)
commands premium pricing. No other source provides institutional-grade data with WORM provenance.

### The Four Generic Product Categories

These are not Woodfine-specific. They are the generic categories for any DKA deployment:

1. **Content Generation exports** — structured summaries from L3 Topics, versioned weekly
2. **Wiki snapshots** — versioned wiki page corpus for downstream indexing or licensing
3. **CoA-classified document index** — all documents mapped to compliance categories
4. **Theme timelines** — active/archived Theme history with document counts and date ranges

Higher-confidence data (from service-verification) commands premium pricing.
Unverified data is available at standard pricing with explicit confidence disclosure.

### Catalog and Licensing Standards

| Standard | Role |
|---|---|
| **DCAT v3** (W3C) | Machine-readable catalog entry: title, publisher, license, format, update frequency |
| **ODRL 2.2** (W3C) | Rights and usage terms: who can use data, for what purpose, for how long |
| **Dublin Core (DCMI)** | Lightweight discovery metadata on every exported dataset |
| **Schema.org Dataset** | SEO-discoverable structured markup for public catalog pages |
| **XBRL** (already in CoA) | Financial data products pre-tagged for institutional data platforms |

### DARP Compliance on Data Products

Every data product carries a DARP compliance chain:
- SHA-256 hash chain from L0 source to derivative output
- PROV-O provenance graph tracing every transformation step
- SOC3 attestation: Security, Availability, Processing Integrity, Confidentiality, Privacy
- Machine-verifiable via the DARP I4 commutation test (DuckDB SQL + Oxigraph SPARQL on same data)

---

## Mode 2: Ad Exchange

### What It Is

Real-time audience segment monetization from the **total data graph** — property, corporate, and
personnel archive data patterns. NOT advertising against wiki page content.

The wiki generates ad inventory via IAB Content Taxonomy contextual targeting (that is a separate,
simpler model). The Ad Exchange is about the DATA GRAPH ITSELF — the entity patterns, relationship
clusters, and operational behaviors that the graph represents.

### Why the Data is Valuable to Ad Exchange Networks

**Most ad exchange audience data is:**
- Derived from browser cookies (dying in post-cookie landscape)
- Self-reported (low accuracy)
- Based on behavioral inference (contested by privacy regulators)
- Not provenance-stamped (cannot prove data freshness or source)

**DKA data segments are:**
- Derived from institutional records with WORM provenance
- Cross-referenced by verified humans (service-verification)
- DARP-compliant (searchable without proprietary software)
- SOC3-attested (third-party audited)
- IAB Audience Taxonomy labeled (interoperable with all major DSPs)
- Privacy-safe by design (no PII, no behavioral tracking of individuals)
- First-party institutional data — the premium tier in the post-cookie landscape

### Concrete Segment Examples by Archive Type

**PropertyArchive with IoT data:**
- Graph contains: sensor readings, maintenance records, building occupancy patterns
- Audience segment: "commercial building operators managing multi-site HVAC systems"
- IAB Audience Taxonomy: "Business > Real Estate > Commercial Property Management"
- Advertisers: HVAC manufacturers, energy management software, facilities services

**CorporateArchive with investor/stakeholder data:**
- Graph contains: investor relations corpus, stakeholder communications, CoA-classified financials
- Audience segment: "participants in LP/reporting issuer structures in real estate"
- IAB Audience Taxonomy: "Business > Investing > Alternative Investments"
- Advertisers: financial services, legal services, professional development

**PersonnelArchive with professional network data:**
- Graph contains: verified entity nodes, organization relationships, role histories
- Audience segment: "decision-makers in development and construction"
- IAB Audience Taxonomy: "Business > Construction > Commercial Construction"
- Advertisers: construction materials, project management software, professional services

### IAB Taxonomy Application — The Critical Distinction

**IAB Content Taxonomy v3.0** → Applied to **WIKI PAGES** (L5 output)
- Describes what the page is about
- Used for contextual advertising alongside wiki content
- Applied at L5 when wiki page is published
- ~1,500+ categories, 4-tier hierarchy

**IAB Audience Taxonomy v1.1** → Applied to **DATA SEGMENTS** in the Ad Exchange
- Describes what the data segment represents (demographic, interest, purchase-intent)
- NOT applied to content
- 1,600+ standardized attribute nodes
- Three Tier 1 categories: Demographic, Interest-based, Purchase Intent

**These are separate taxonomies with separate purposes. Never conflate them.**

### Ad Exchange Protocol Stack

| Standard | Role |
|---|---|
| **IAB Audience Taxonomy v1.1** | Segment labeling — interoperable across all major DSPs |
| **IAB Data Transparency Standard (DTS)** | 20-field "nutrition label" per segment (quality disclosure) |
| **OpenRTB 2.6** | Real-time bid request/response protocol |
| **W3C DPV 2.0 + ODRL + ISO 27560 + GPC + GPP** | Consent management (NOT IAB TCF — see STANDARDS.md) |
| **PPID** | Publisher Provided Identifiers — privacy-safe audience IDs |

### What Is Never Used in Ad Targeting

- PII of any kind
- Financial position data
- Insider identities
- Any data subject to securities disclosure requirements
- Individual behavioral data or browsing history
- Any L0 base assets

---

## The Commercial Flywheel

```
More source data ingested
         ↓
More derivative nodes generated (L2–L4)
         ↓
More Topic nodes → more wiki pages published (L5)
         ↓
More ad inventory (wiki) + more audience segments (total graph)
         ↓
More revenue to archive owner
         ↓
Incentive to ingest more data and fund verification
         ↓
Higher verification throughput → higher confidence scores
         ↓
Premium pricing in marketplace → more revenue → more investment
```

---

## service-marketplace File Tree

```
service-marketplace/
├── products/
│   ├── catalog.json              ← DCAT v3 catalog (all data products)
│   └── [product-id]/
│       ├── definition.yaml       ← product spec: source layer, filter, format, frequency
│       ├── license.odrl          ← ODRL policy
│       └── snapshots/            ← versioned data exports
│
├── segments/
│   ├── definitions.yaml          ← IAB Audience Taxonomy segment definitions
│   └── active.json               ← currently active segments for exchange
│
└── ledger/
    ├── transactions.csv          ← all marketplace and exchange transactions
    └── revenue.csv               ← revenue share records
```

---

## Micropayment Infrastructure for Verifier Payments

(Shared with VERIFICATION.md — documented here for completeness)

**Primary:** Polygon USDC on Polygon PoS (~$0.002/tx, 0.2% fee rate).
**Backup:** Lightning Network via LNbits (self-hosted, MIT/BSD).
**Not viable:** Stripe Connect ($0.25+/payout), PayPal Payouts (25–50% fee on sub-$2 amounts).
**GNU Taler:** Promising but geographically limited as of April 2026.
**FinCEN/FINTRAC:** MSB registration required before enabling paid workforce.
