# RESEARCH.md
# Research Synthesis — Market, Architecture, Competitive
**Version:** 1 · April 19, 2026
**Sources:** DKA Defensibility and Design Study + DKA Architecture Blueprint by 2030
**Classification:** PointSav Digital Systems — Internal / Strategic

---

## Executive Summary

The Derivative Knowledge Architecture occupies a real and currently unoccupied competitive niche:
an embedded graph + compliance-anchored ontology + auto-generated wiki product for the $30K–$150K
mid-market ACV band. Palantir ($10M+ TCVs) and Glean ($65K median, $350K+ TCO) simultaneously
vacate this band. Three structural market forces align by 2030: EU Data Act full enforcement
(January 2027), Kùzu's October 2025 Apple acquisition creating an MIT-licensed orphan the
community is actively forking, and Canada's Bill C-27 dying (January 2025), leaving the field
open for a sovereignty-first architecture.

**Three claims in the DKA specification need reframing before public defense:**

1. "Two human inputs" → recast as "two inputs plus a standard-library pack of industry mappings"
2. "Derivative" metaphor → anchor to Taylor-style P&L attribution, not differential geometry
3. "Self-healing taxonomy" → achievable for L1–L3 mid-layers; NOT yet possible for formal-ontology L2 spine

---

## Part I: The Three Exploitable Market Gaps

**Gap 1 — Persistent auto-generated wiki (the biggest gap).**
Nothing currently builds a persistent, human-browsable wiki from enterprise unstructured content
backed by an editable ontology with continuous auto-update. SamurAIGPT's `llm-wiki-agent` is
hobbyist-local. Palantir renders objects as apps, not pages, and requires $207K–$415K Forward-
Deployed Engineers. Glean, Confluence, and Notion summarize at query time rather than materializing
compounding pages. LazyGraphRAG and LightRAG are retrieval mechanisms, not knowledge products.

**Gap 2 — Mid-market price band ($25K–$150K ACV).**
The band between Confluence AI ($18/user) and Glean ($50+/user, $60K+ floor) is structurally
empty. Post-LazyGraphRAG, indexing 10,000 documents costs approximately $10. The DKA cost
structure — embedded graph on a $7/month VM, yo-yo GCP compute for batch processing — makes
a $25K–$150K ACV product arithmetically possible. No equivalent product exists.

**Gap 3 — Vertical out-of-the-box ontologies.**
Every buyer of a knowledge management product today faces $200K+ in custom ontology engineering
(ZipRecruiter: $107K average ontology engineer salary, up to $220K+ senior; boutique ontology
engagements $150K–$500K; full enterprise programs $500K–$3M in year one). DKA's pre-packaged
compliance-anchored CoA + archetype libraries eliminate this cost entirely.

---

## Part II: The Competitive Cost Reality

| Platform | Cost model | Effective entry point |
|---|---|---|
| Palantir Foundry | $10M–$100M+ TCV + 1–3 FDEs at $207K–$415K each for 6–18 months | $10M+ per customer |
| Glean | $50+/user/mo, 100-seat minimum, $65K median deal, 10% mandatory support | $350K–$480K/yr TCO |
| Microsoft GraphRAG | $47,000 in API indexing costs for 100,000 documents before first query | Opaque — compute + Azure |
| LazyGraphRAG | ~$10 for 10,000 documents (700× cheaper than GraphRAG) | Retrieval only, not a product |
| Memgraph Enterprise | $25,000/yr minimum for 16 GB RAM | Server license only |
| TigerGraph | $730/mo to $186,880/mo | Server license only |
| DKA | $7/month VM + ~$200–300 per batch GCP run + Claude API per query | $25K–$150K ACV sustainable |

---

## Part III: What Hyperscalers Structurally Cannot Build

Seven structural constraints — not commercial decisions, but architectural incompatibilities with
hyperscaler business models:

1. **Data on customer hardware** — EU Data Act Art. 25 forcing cloud-switching (January 2027)
2. **100% marketplace revenue share** — AWS Data Exchange takes 3% + per-GB + per-grant-hour
3. **Self-generating taxonomy** — cannibalizes their Professional Services revenue (Snowflake PS: ~6-7% of revenue at negative gross margin)
4. **Cloud switching on 2-month notice** — EU Data Act, DMA investigations into AWS/Azure (opened November 2025)
5. **SMB long-tail pricing** — hyperscaler discounting requires commitment tiers SMBs cannot meet
6. **CLOUD-Act-immune sovereignty** — Microsoft admitted in French court they cannot guarantee EU data residency
7. **Forkable OSS** — doing so evaporates their proprietary-service moat (Redshift/BigQuery/DynamoDB)

**The strategic position:** DKA is not a better knowledge graph. It is an irrevocable one.

---

## Part IV: The Post-Kùzu Database Landscape

Apple acquired Kùzu October 10, 2025 and archived the GitHub repository. The MIT license on
v0.11.3 remains valid. Three community forks:

**LadybugDB (MIT) — Recommended primary:**
- v0.15.2 (March 2026), monthly releases
- CLA removed at v0.12.0 (contributors retain copyright — truly forkable)
- Bolt-protocol support, WASM bindings, NaviX HNSW vector index built-in
- Arun Sharma (ex-Facebook/Google)
- Real production exposure (CocoIndex, GitNexus)

**RyuGraph (MIT) — Recommended backup:**
- v25.9.2 (December 2025)
- Adds full-text search + vector search on top of Kùzu base
- Akon Dey (ex-Dgraph CEO)
- Enterprise-flavored

**Other options evaluated and rejected:**
- FalkorDB: SSPL license (not OSI-approved) + Redis subprocess (not truly embedded)
- Memgraph CE: moved to BSL 1.1 (not OSI-approved)
- Neo4j CE: GPLv3 (not embeddable into proprietary code)
- SurrealDB: BSL-1.1 (already relicensed once)
- Apache AGE: PostgreSQL extension, not embedded
- Oxigraph: MIT/Apache but RDF-only, no Cypher/GQL

**DuckPGQ note:** DuckDB + SQL/PGQ extension is the correct DARP I4 commutation partner
(second engine for CI testing), but missing full Kleene-star path queries for enterprise KG
workloads. Use as commutation test engine, not primary.

**GQL Standard (ISO/IEC 39075:2024):** The first new ISO database query language since SQL
in 1987. Writing DKA queries against the Cypher/GQL intersection ensures forward portability.

---

## Part V: Cross-Industry Leapfrog Patterns

### From Finance: Layered Derivative Computation

Options Greeks form a genuine layered derivative stack. DKA's correct framing: each layer L_k is
a summarization operator Φ_k applied to L_{k-1}. Update variance grows with operator norm. Cadence
scales with observed volatility, not layer depth. Every L5 output carries an attribution trace
decomposable back to L0 base facts — auditable as trading P&L is auditable.

The framing that does NOT survive peer review: treating DKA layers as literal differential
geometry derivatives. The base is discrete, not continuous.

### From Scientific Publishing: OpenAlex Pattern

OpenAlex's 2024 rebuild auto-generated a 4-level, ~4,500-topic taxonomy by Leiden-clustering
1.7 billion citation links, then using GPT-3.5 to label clusters, then training a classifier to
route new works. This is the clearest open-source precedent for DKA's layer-on-layer computation.

SPECTER2 (Ai2, 2023): adapter-based, task-specific embeddings on citation graph outperform
single-purpose embeddings on 11 of 16 downstream tasks.

The DOI/Handle System: prefix/suffix resolver with content negotiation — permanence without
central control. Model for DKA's entity identifier scheme.

### From Legal: Temporal Validity as First-Class Edges

Akoma Ntoso/LegalDocML (OASIS, 2018): `<lifecycle>` elements encode temporal validity, supporting
retroactivity and ultractivity at the schema level. ELI/ECLI: persistent URI scheme with
`{jurisdiction}/{year}/{point-in-time}/{version}` template. CELLAR: ~2.7M legal works as RDF.

**MeSH precedent:** Does NOT retroactively re-index MEDLINE. Retroactive re-tagging costs more
than the benefit. DKA adopts temporal fragmentation as a feature.

### From Medicine: openEHR Two-Level Methodology

openEHR's two-level methodology is the single most valuable import for DKA's L2 Archetypes:
tiny invariant Reference Model + large Archetype Model expressed in ADL 2, compiled via templates
into Operational Templates. DKA adaptation: Party/Role/Product/Place/Time/Event reference model
(domain-neutral, not clinical). Git/PR-based archetype governance instead of CKM central review.

SNOMED CT RF2 versioning quadruple `(id, effectiveTime, active, moduleId)` — proves bitemporal
versioning works at planetary scale. Every DKA node carries this envelope.

ICD-11: Foundation Component (ontology) + Linearizations (views). One underlying graph, many
use-case-specific outputs. Maps directly to DKA's Domains-vs-Themes architecture.

### From Supply Chain: REA and the 5W+H Envelope

REA (McCarthy 1982, ISO/IEC 15944-4): every economic fact is an Event with `:PROVIDES`/
`:RECEIVES` to Agents and `:INCREMENTS`/`:DECREMENTS` to Resources. GL accounts become saved
queries. No canonical open-source property-graph REA implementation exists — DKA will be the first.

GS1 GPC Brick 99999999: formal temporary classification bucket with change-request workflow.
DKA adopts this literally as the Temporary Classification node pattern.

GS1 EPCIS 2.0 (ISO/IEC 19987): 5W+H event envelope (What/When/Where/Why/Who/How). Universal
event model applicable to business, audit, and IoT events in DKA.

WoT Thing Description (W3C Rec December 2023): every entity simultaneously a graph node, a
digital twin submodel, and a live JSON-LD-self-describing API.

---

## Part VI: DARP as Graph Geometry

DARP is defined formally as four geometric invariants. Any graph G = (V, E, P, S) is
geometrically DARP-compliant if and only if:

**I1:** Every element ID is `SHA-256(canonical_serialization(element))` — Merkle DAG structure.
**I2:** Every element serializable to at least one royalty-free format with ≥3 open implementations.
**I3:** Schema S expressible as OWL+SHACL+SKOS+PROV-O+DCAT, shipped as Turtle alongside data.
**I4:** Two ISO/W3C query languages + two independent engines produce identical answer sets — testable as CI invariant.

Precedents: Wikidata (JSON+RDF dual dumps), OpenStreetMap (OSM-XML+PBF), SNOMED CT (RF2 TSV),
Internet Archive (WARC), Human Genome Project (FASTA). All used the same pattern: multiple
independent encodings, multiple independent readers, royalty-free specs, dated stable releases.

---

## Part VII: The Universal Business Geometry — What Survives Scrutiny

**The strong claim** ("one universal enterprise ontology exists") is **not defensible.**
O'Leary (IEEE IS, 2000), ISO/IEC 21838 (explicitly standardizes multiple upper ontologies),
and 40 years of ontology research confirm this.

**The weak claim** ("a small convergent vocabulary of ~100–200 concepts covers 80%+ of
cross-industry enterprise structure") **is defensible.** Convergence across five independent
traditions is remarkable (gist ~130 classes, FIBO Foundations ~150, REA 5+15, Silverston 7
macro-patterns, Hay ~10 entities).

**The DKA's defensible reframing:** "2 inputs + a curated library of ~20–40 industry packs
over a universal ~100-class upper ontology (gist + REA + OntoUML)." Minimum realistic input
count: **5–7** (CoA seed, industry/vertical picklist, org shape, primary work-unit noun, role
taxonomy, document taxonomy seed, fiscal convention). Inputs 5–7 carry strong defaults so users
experience "a handful of decisions, not hundreds."

**gist wins on pragmatics:** CC-BY open license, ~100–135 classes learnable in weeks, OWL 2,
20+ years traction at Goldman Sachs, Morgan Stanley, P&G, Schneider Electric, active Semantic
Arts council.

**REA fills gist's gap:** Resource-Event-Agent duality and stockflow primitives. GL accounts become
saved queries. Geerts/Poels OWL formalization (BIS 2007, LNCS 4439).

**Cyc as the cautionary tale:** 40+ years, ~25M rules, a person-century of engineering, OpenCyc
withdrawn 2017, Lucid.ai collapsed. Over-claiming is fatal.

---

## Part VIII: Self-Healing Taxonomy — Honest Scope

**Production-viable now (L1–L3 mid-layers):**
- OntoRAG (ABB, arXiv:2506.00664, May 2025): one-shot pipeline, 85% comprehensiveness vs vector RAG
- LazyGraphRAG (Microsoft, Nov 2024): $10/10K docs, 700× cheaper than full GraphRAG
- LightRAG (EMNLP 2025): ~99% retrieval token reduction
- TaxoLLaMA, TaxoAdapt, Chain-of-Layer, TaxoInstruct: 2024–2025 state-of-the-art taxonomy induction

**NOT yet possible (L2 formal ontology):**
AGM-minimal revision for DL/OWL remains NP-hard. No system in April 2026 demonstrates
autonomous closure on formal-ontology consistency repair under LLM-proposed changes.

**The four algorithms DKA must invent** (no published end-to-end precedent):
1. **OCTH** — Ontology-Constrained Taxonomy Healer: L3 repair under L2 OntoUML constraints with ASP-based SHACL repair
2. **REA-Aware Property Graph Schema Inference** — mine REA duality pairs from unlabeled transactional data (40-year literature gap)
3. **Universal-Vocabulary Drift Detector** — detect when instance edge-distributions diverge from L2 kind
4. **Catastrophic-Forgetting-Free Ontology Evolution with Provenance** — extend IncLoRA with axiom-level provenance

---

## Part IX: Consent Architecture — Why Not IAB TCF

IAB TCF v2.3 became mandatory February 28, 2026:
- €1,575/yr CMP registration + €1,575/yr vendor registration
- External runtime dependency: GVL fetch from consensu.org (IAB-controlled infrastructure)
- Joint GDPR controllership liability (Belgian Market Court €250K fine upheld May 2025)
- TC String propagates across the ad-tech supply chain — antithetical to sovereignty

**DKA consent stack: W3C DPV 2.0 + ODRL + ISO 27560 + GPC + GPP**
- $0 cost, self-hostable, no external runtime dependencies
- Consent as RDF triples in the graph (`<user> dpv:hasConsent <receipt>; dpv:hasPurpose dpv:Marketing`)
- IAB GPP (Apache-2.0 spec, no IAB membership) handles per-state US signaling
- Klaro! (BSD-3) for user-facing consent banner
- EU upgrade path: add TCF as GPP section when justified

**The no-PII shortcut:** Under GDPR Recital 26 + EDPB Guidelines 04/2025, true k-anonymized
aggregates (l-diversity + differential privacy) fall outside GDPR scope entirely. CCPA
§1798.140(m) similarly excludes properly deidentified data. DKA's "no PII in any data product"
is both a moral principle and a consent-regime escape hatch.

---

## Part X: The Three Leapfrog Opportunities by 2030

### Leapfrog #1: RF2 Universal Versioning Envelope

`(id, effectiveTime, active, moduleId)` + Datomic op-bit + XTDB2 bitemporality + FHIR meta
on EVERY node and edge. No hyperscaler knowledge graph offers this natively. Storage would be
billed per-assertion-history, not per-byte — incompatible with their pricing model.

### Leapfrog #2: openEHR Two-Level L2 with gist + REA + OntoUML

Tiny Reference Model (~30 classes: Party/Role/Product/Place/Time/Event) + composable Archetype
Model compiled to runtime Operational Templates (emitting SHACL + JSON Schema + GraphQL).
Zero ontology engineers at deployment. Directly eliminates hyperscalers' Professional Services
revenue line. No canonical open-source property-graph REA implementation exists — DKA builds it.

### Leapfrog #3: Bootable OVA + IPFS CAR + OCI Artifacts

Bootable virtual appliance with LadybugDB + vector index + OLMo 3 pre-baked. Graph payloads as
content-addressed `.car` archives (identical CIDs on both sides of air-gap). Signed OCI Artifacts
with SLSA attestations for versioned updates. W3C DID + VC 2.0 + zcap-ld for identity and
capability-based access control. CLOUD-Act-immune. Hyperscalers cannot build this because platform
stickiness is their business model.

---

## Key Numbers for the Go-to-Market Narrative

| Metric | Value | Source |
|---|---|---|
| GraphRAG indexing cost (100K docs) | $47,000 | Microsoft/Beyond Key, March 2025 |
| LazyGraphRAG indexing cost (10K docs) | ~$10 | Microsoft Research, Nov 2024 |
| DKA full corpus run (2.5 GB) | ~$200–300 | Calculated from GPU benchmarks |
| Palantir FDE median salary | $207.5K–$215K | Levels.fyi, April 2026 |
| Glean annual TCO (mid-market) | $350K–$480K | GoSearch/Vendr buyer data |
| Ontology engineer average salary | $107K | ZipRecruiter |
| Boutique ontology engagement | $150K–$500K | Market rate |
| DKA base monthly compute | $7 | Commodity cloud VM |
| DKA target ACV | $25K–$150K | Mid-market band |
| Memgraph Enterprise floor | $25,000/yr | Public pricing |
| MinIO archived | Feb 13, 2026 | GitHub archive notice |
| Kùzu archived by Apple | Oct 10, 2025 | GitHub archive + EU DMA filing |
| EU Data Act full enforcement | Jan 12, 2027 | Regulation 2023/2854 |
| Canada Bill C-27 died | Jan 6, 2025 | Parliamentary prorogation |
| AWS QLDB discontinued | Jul 31, 2025 | AWS announcement |
