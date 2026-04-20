# STANDARDS.md
# Compliance and Classification Standards Reference
**Version:** 1 · April 19, 2026
**Governing regulatory frameworks: SOC3 and DARP**

---

## Governing Frameworks

### SOC3 — Service Organization Control 3
Public-facing attestation of Trust Services Criteria (TCC) compliance.
Five criteria: Security · Availability · Processing Integrity · Confidentiality · Privacy.
YAML snapshots from service-content are the primary SOC3 audit artifacts.
Every verification transaction in service-verification/ledger is SOC3 evidence.

### DARP — Data Access and Retention Protocol
PointSav's internal standard. Core requirement: **data must be searchable without proprietary
software**. The LadybugDB + flat-file architecture satisfies DARP by design.
DARP is baked into graph geometry as four structural invariants — see ARCHITECTURE.md §4.
All Data Marketplace products must carry a DARP compliance chain.

---

## Chart of Accounts Standards Stack

### Records and Information Management

| Standard | Scope | Role in CoA |
|---|---|---|
| **ARMA GARP (2025 update)** | 8 governance principles: Accountability, Transparency, Integrity, Protection, Compliance, Availability, Retention, Disposition | Governs the Information & Technology / Records macro-domain. The 8 principles map to governance properties on every CoA node. WORM + SHA-256 satisfies principles 3, 4, 6, and 8 by design. |
| **ISO 15489-1:2016** | Records management — concepts and principles | Defines what constitutes a record, classification, retention |
| **ISO 30301:2019** | Management systems for records | System-level requirements |
| **MoReq2010** | 13 modules, ~345 metadata elements | Informs leaf-level CoA structure |

### Compliance Management

| Standard | Scope | Role in CoA |
|---|---|---|
| **ISO 37301:2021** | Compliance management systems (the only certifiable CMS MSS) | 10-clause HLS maps to Risk, Compliance & Controls macro-domain. Frozen spine changes require ISO 37301 §5 review. |
| **COBIT 2019** | IT governance — 40 objectives, 5 domains | Information & Technology macro-domain |
| **NIST CSF 2.0** | Cybersecurity — 6 Functions, 22 Categories, 106 Subcategories | Cybersecurity sub-domain |
| **SOC 2 Trust Services Criteria** | 5 criteria, 61 criteria points | Security and Privacy sub-domains |

### Financial Reporting

| Standard | Scope | Role in CoA |
|---|---|---|
| **IFRS Accounting Taxonomy** | ~3,000+ XBRL elements around financial statements | Foundation of Financial macro-domain. All financial CoA nodes carry XBRL bindings. |
| **US-GAAP Taxonomy** | XBRL equivalent for GAAP | Alternative financial binding |

### Sustainability and ESG

| Standard | Scope | Role in CoA |
|---|---|---|
| **GRI Universal Standards 2021** | 3 universal + ~40 sector + 200/300/400-series topic standards | Sustainability macro-domain |
| **ESRS** | 2 cross-cutting + 10 topical EU standards | EU sustainability sub-domains |
| **ISSB S1/S2** | TCFD pillars: Governance, Strategy, Risk, Metrics | Sustainability disclosure structure |
| **SASB** | 77 industry standards, 5 dimensions | Industry-specific sustainability classification |

### Entity and Identity

| Standard | Scope | Role |
|---|---|---|
| **GLEIF / ISO 17442** | Legal Entity Identifier | Entity & Governance entity IDs |
| **ISO 20275** | Entity Legal Forms (~3,000 jurisdiction-specific forms) | Legal form classification |
| **Schema.org Organization** | Web-readable entity schema | Data product discoverability |

---

## Archetype Standards Stack

Five-layer dual-labeled stack. Every archetype node carries two labels: canonical URI from an
approved standard + emergent community label from the Leiden pipeline.

| Layer | Standard | Classes |
|---|---|---|
| L0 — Upper Ontology | BFO (ISO/IEC 21838-2) | ~30 universal primitives |
| L1 — Standards Canonical | FIBO + GICS + NAICS + Schema.org + GLEIF + ArchiMate 3.2 | ~3,000–5,000 URI-identified classes |
| L2 — Enterprise Types | DKA-native + gist + REA + OntoUML meta | ~150 classes, ~100 properties |
| L3 — Role/Shape | RACI + FCA SMF + Jungian 12 brand archetypes | Multi-attachable |
| L4 — Emergent Labels | Leiden pipeline output | Continuous, `skos:closeMatch` to L1–L2 |

### The L2 Three-Layer Foundation

**gist (Semantic Arts, CC-BY 4.0):** ~100–135 classes, ~100–140 properties. 20+ years commercial
traction at Goldman Sachs, Morgan Stanley, P&G, Schneider Electric. OWL 2, aligned to BFO via
gistBFO and to Schema.org. The domain-neutral kernel.

**REA (Resource-Event-Agent, McCarthy 1982; ISO/IEC 15944-4:2007):** Every economic fact is an
Event with `:PROVIDES`/`:RECEIVES` edges to Agents and `:INCREMENTS`/`:DECREMENTS` edges to
Resources. GL accounts become views (saved queries), not stored entities. No canonical
open-source property-graph REA implementation exists in the literature — DKA will be the first.

**OntoUML/UFO meta-validator:** Applies Kind/Phase/Role/Mixin stereotypes as annotations on
gist/REA to catch rigidity and identity errors at schema design time.

Target L2 size: ~150 classes, ~100 properties.

---

## Data Marketplace Standards

| Standard | Role |
|---|---|
| **DCAT v3** (W3C) | Machine-readable catalog entry per data product |
| **ODRL 2.2** (W3C) | Licensing terms: who can use data, for what purpose, for how long |
| **Dublin Core (DCMI)** | Lightweight metadata on every exported dataset |
| **Schema.org Dataset** | SEO-discoverable structured markup for public catalog |
| **XBRL** (already in CoA) | Financial data products pre-tagged for institutional platforms |

---

## Ad Exchange Standards

**The critical distinction: two separate IAB taxonomies for two separate purposes.**

| Standard | Applied to | Purpose |
|---|---|---|
| **IAB Content Taxonomy v3.0** | Wiki PAGES (Layer 5 output) | What the page is about. Enables contextual advertising on wiki pages. 1,500+ categories, 4-tier hierarchy. |
| **IAB Audience Taxonomy v1.1** | DATA SEGMENTS (Ad Exchange) | What the data segment represents (demographic, interest, purchase-intent). NOT applied to content. |
| **IAB Data Transparency Standard (DTS)** | Audience segments | 20-field "nutrition label" for segment quality disclosure |
| **OpenRTB 2.6** | Ad Exchange bids | Real-time bidding protocol |

### Consent Stack — NOT IAB TCF

IAB TCF v2.3 is rejected for DKA: €3,150+/yr fees, external runtime dependency (GVL fetch from
consensu.org), joint GDPR controllership liability per Belgian Market Court ruling (May 2025).

**DKA consent stack:**

| Standard | Role |
|---|---|
| **W3C DPV 2.0** (Data Privacy Vocabulary) | Consent as RDF triples: purpose, legal basis, retention period |
| **ODRL 2.2** | Access policies on data products |
| **ISO/IEC TS 27560:2023** | Machine-readable consent receipt structure |
| **Global Privacy Control (GPC)** | Universal opt-out, legally recognized in 12 US states |
| **IAB Global Privacy Platform (GPP)** | Per-state US sections (Apache-2.0 spec, no IAB membership) |
| **Klaro!** (BSD-3) | Self-hostable CMP for admin console GDPR/ePrivacy banner |

EU launch path: add IAB TCF as a GPP section when EU expansion justifies the cost. The DPV stack
remains the canonical source of truth internally.

---

## ARMA GARP Eight Principles (2025 Update)

Relevant to the Records & Information macro-domain and to service-verification audit design.

1. **Accountability** — Senior leadership oversight of information governance
2. **Transparency** — Recordkeeping processes documented and verifiable
3. **Integrity** — Records authentic, reliable, and unaltered (maps to WORM/SHA-256)
4. **Protection** — Appropriate security throughout lifecycle
5. **Compliance** — Records meet legal, regulatory, and organizational requirements
6. **Availability** — Records accessible when needed (maps to DARP requirement)
7. **Retention** — Records kept as long as required, then disposed per schedule
8. **Disposition** — Secure and appropriate disposal (maps to WORM append-only model)

---

## Sovereign Identity Standards

| Standard | Role |
|---|---|
| **W3C DID 1.0** (Recommendation July 2022) | `did:web` for org IDs · `did:key` for ephemeral contexts · `did:peer` for pairwise |
| **W3C VC 2.0** (Recommendation May 2025) | Verifiable Credentials — SD-JWT or BBS selective disclosure |
| **zcap-ld** (W3C CCG) | Linked Data Capabilities — unforgeable delegated tokens replacing ACL |
| **SLSA v1.0** (Supply-chain Levels for Software Artifacts) | Provenance attestations on every OCI Artifact release |
| **Sigstore / Cosign** (Apache 2.0) | Keyless signing for OCI Artifacts with Rekor transparency |

---

## Portability Standards

| Standard | Role |
|---|---|
| **OVA/OVF** (DMTF) | Bootable VM appliance packaging — OS + engine + seeded data |
| **IPFS CAR** (Content Addressable aRchives) | Graph payload distribution — CIDs identical on both sides of air-gap |
| **OCI Artifacts** (OCI v1.1, 2024) | Versioned updates: schemas, policies, model weights |
| **GQL** (ISO/IEC 39075:2024) | ISO graph query language standard — future-proofing query portability |
