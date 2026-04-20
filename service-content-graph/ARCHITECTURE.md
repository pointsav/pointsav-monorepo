# ARCHITECTURE.md
# Derivative Knowledge Architecture — Full Specification
**Version:** 7 · April 19, 2026
**Regulatory framework:** SOC3 and DARP
**See also:** LAYERS.md · SERVICES.md · SCHEMA.md · STANDARDS.md · RESEARCH.md

---

## 1. What We Are Building

A six-layer derivative knowledge graph pipeline that converts institutional documents into a
continuously growing, self-maintaining knowledge base, and generates four commercial outputs:
Content Generation, Wiki, Data Marketplace, and Ad Exchange. An optional distributed verification
layer sits above the outputs and increases the commercial value of data products.

The system runs permanently on commodity hardware (Laptop-A, 4 GB RAM). Batch computation
happens on transient GCP Cloud Run GPU nodes that spin up, process, and tear down. The complete
knowledge base is portable on a USB drive.

**Phase 1 scope:** The 2.5 GB corpus is already converted to /ledger and /assets by Gemini API.
Extraction is complete and out of scope. Phase 1 loads existing data into the graph and validates
the full pipeline. Trial run on 3 files first.

---

## 2. The Seven-Layer Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  VERIFICATION LAYER — OPTIONAL                                              │
│  Switched on separately as a paid feature                                   │
│  Distributed human workforce via os-console from any location               │
│  Generates questionnaires FROM graph nodes · verifiers cross-reference      │
│  externally via their own browser · confidence scores update graph nodes    │
│  Higher verification = higher-confidence data = premium marketplace pricing │
│  See VERIFICATION.md                                                        │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
┌──────────────────────────────────────▼──────────────────────────────────────┐
│  LAYER 5 — OUTPUT AND UTILITY                                               │
│                                                                             │
│  CONTENT          WIKI              DATA              AD                    │
│  GENERATION       Self-generating   MARKETPLACE       EXCHANGE              │
│  Drafts,          knowledge base    Structured data   Audience segments     │
│  reports,         from Topic nodes  products licensed from TOTAL data       │
│  analysis via     Diátaxis layout   via DCAT/ODRL     graph via OpenRTB 2.6 │
│  Claude API       IAB Content Tax   SOC3 attested     IAB Audience Tax      │
│                   v3.0 on pages     DARP compliant    NOT wiki content       │
│                                                                             │
│  All L5 outputs require operator approval before publication (SYS-ADR-19)  │
│  All citation-grounded: every claim linked to L0 source (Pydantic+instructor│
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ synthesized from
┌──────────────────────────────────────▼──────────────────────────────────────┐
│  LAYER 4 — THEMES (Fourth Derivative)                                       │
│  Analytical output showing WHERE DATA IS MOVING OVER TIME                  │
│  Not a static label — a temporal pattern detected across weekly batches     │
│  Active: currently recurring patterns · Archived: fading — never deleted   │
│  Theme timeline = built-in trend analysis for the archive                  │
│  High commercial value: premium Ad Exchange segments · marketplace product  │
│  Re-evaluated every weekly batch                                            │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ computed from
┌──────────────────────────────────────▼──────────────────────────────────────┐
│  LAYER 3 — DOMAINS · GLOSSARIES · TOPICS (Third Derivative)                │
│                                                                             │
│  Domain (unlimited — Woodfine defaults: Corporate/Projects/Documentation)  │
│    └── Glossary nodes: terms defined by co-occurrence in this domain        │
│          Each domain has its OWN Glossary set                               │
│    └── Topic nodes: subject clusters — one Topic = one future Wiki page     │
│          Each domain has its OWN Topic set                                  │
│                                                                             │
│  ⚠ THESE ARE INDEX NODES — NOT WIKI PAGE CONTENT                           │
│  Topic node = label + description + Chunk IDs + Entity IDs + Metric IDs    │
│              + IAB Content Taxonomy v3.0 code (applied at L5, not here)    │
│  Glossary node = term + definition + source Chunk IDs                      │
│  Wiki page content is generated FROM Topic nodes at Layer 5                │
│  "The Topic node is the recipe. The wiki page is the dish."                │
│                                                                             │
│  Temporary Classification bucket for unresolved documents (GS1 GPC pattern)│
│  Update: Domains 12+ months · Glossaries/Topics weekly                     │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ computed from
┌──────────────────────────────────────▼──────────────────────────────────────┐
│  LAYER 2 — ARCHETYPES · CHART OF ACCOUNTS (Second Derivative)              │
│                                                                             │
│  CHART OF ACCOUNTS — FROZEN COMPLIANCE SPINE (~30 top-level categories)    │
│  ISO 37301 · IFRS/XBRL · GRI · ESRS · ISSB · SASB · COBIT · NIST CSF      │
│  SOC2 · ARMA GARP (2025) · ISO 15489 · MoReq2010                           │
│  Human sign-off required for any change to frozen spine                    │
│                                                                             │
│  SELF-HEALING PERIPHERY: crosswalks · synonyms · leaf bindings             │
│  "Healing" = raising a reviewable PR — never auto-committing               │
│                                                                             │
│  ARCHETYPES — FIVE-LAYER DUAL-LABELED STACK                                │
│  L0: BFO upper ontology (~30 universal primitives)                         │
│  L1: Standards-anchored canonical (FIBO · GICS/NAICS · Schema.org · GLEIF) │
│  L2: DKA-native enterprise domain types                                     │
│  L3: Role/shape archetypes (RACI · FCA SMF · Jungian 12)                  │
│  L4: Emergent community labels (Leiden pipeline, skos:closeMatch to L1–L2) │
│                                                                             │
│  L2 FOUNDATION: gist (kernel, CC-BY) + REA (economic semantics, ISO 15944) │
│                + OntoUML (meta-validator for rigidity/identity errors)      │
│  Target: ~150 classes, ~100 properties                                      │
│                                                                             │
│  Human override: edit seeds/coa.csv or seeds/archetypes.csv → propagates   │
│  Update: Archetypes 24+ months · CoA spine 18-24 months                   │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ computed from
┌──────────────────────────────────────▼──────────────────────────────────────┐
│  LAYER 1 — KNOWLEDGE GRAPH (First Derivative)                              │
│  Entities · Chunks · Metrics · Documents + all relationships               │
│  Hybrid retrieval: LadybugDB graph traversal + vector index (HNSW)         │
│  No single source of truth — duplicates self-heal over time                │
│  Nodes never deleted — marked superseded or confidence=0 if disputed       │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ derived from
┌──────────────────────────────────────▼──────────────────────────────────────┐
│  LAYER 0 — BASE ASSETS — GROUND TRUTH (service-fs)                         │
│  /source: original .docx .pdf .xlsx (WORM · SHA-256 sealed)               │
│  /ledger: structured .yaml (2.5 GB — complete for Phase 1)                 │
│  /assets: prose .md    (2.5 GB — complete for Phase 1)                     │
│  Immutable after write · Legal record · Never queried directly for         │
│  intelligence — use Layer 1 for all retrieval                              │
│  RF2 envelope on every node: (id, effectiveTime, active, moduleId)         │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Layer 5 Detail — Four Outputs

### Content Generation
Claude API synthesizes drafts, reports, memos, analysis from assembled graph context.
Every claim citation-grounded to L0 source via Pydantic v2 + instructor structured output.
Operator approves before use. No AI-generated content commits without human authorization.

### Wiki
Self-generating knowledge base organized by Domain → Topics → Wiki pages.
- Topic node (L3) is the index pointer — label + Chunk IDs + source metadata
- Wiki page (L5) is Claude API synthesis from the Topic's contributing Chunks
- IAB Content Taxonomy v3.0 applied to wiki PAGES for contextual advertising
- Diátaxis structure: Reference (auto-generated), Explanation (synthesized), Tutorial (human)
- No wiki page published without operator approval (SYS-ADR-19)

### Data Marketplace
Structured data products licensed from derivative layers (L2–L4). Never sells L0 base assets.
- DCAT v3 (machine-readable catalog) · ODRL 2.2 (rights and usage terms)
- SOC3 attestation + DARP compliance chain on every product
- Buyers: analysts, researchers, institutional data consumers
- Higher verification confidence = premium pricing tier

### Ad Exchange
Audience segment monetization from the **total data graph** — NOT from wiki content.
- IAB Audience Taxonomy v1.1 describes data segments (not content)
- OpenRTB 2.6 real-time bidding protocol
- W3C DPV 2.0 + ODRL + ISO/IEC TS 27560:2023 + GPC + GPP consent stack (NOT IAB TCF)
- No PII in any segment — segments describe entity/pattern clusters
- PropertyArchive IoT → "commercial building operators" · CorporateArchive → "LP investors"
- Archive owner earns CPM revenue share from segment purchases

**The critical IAB distinction:**
- IAB Content Taxonomy v3.0 → what a wiki PAGE is about → applied at L5 wiki output
- IAB Audience Taxonomy v1.1 → what a DATA SEGMENT represents → applied at L5 Ad Exchange
- These are separate taxonomies with separate purposes. Never conflate them.

---

## 4. DARP as Graph Geometry — The Four Invariants

DARP (PointSav's Data Access and Retention Protocol) requires: data searchable without proprietary
software, retained and dispositioned per schedule. These become enforceable when baked into the
graph schema itself — not as policy metadata.

**G = (V, E, P, S) is geometrically DARP-compliant if and only if:**

**I1 — Content-addressable addressing.** Every node, edge, and property blob:
`id(x) = SHA-256(canonical_serialization(x))`
The graph is a Merkle DAG. Any subgraph verifies without a trust anchor.

**I2 — Self-describing serialization.** Every element serializable to at least one royalty-free
format implemented by ≥3 independent open libraries in ≥3 languages. Survivors of the 25-year
readability test: Parquet, Arrow IPC, JSON-LD, Turtle, WARC.

**I3 — Open-standard schema.** Schema S expressible as RDF: RDFS/OWL for classes, SHACL for
structural constraints, SKOS for hierarchies, PROV-O for provenance between layers, DCAT v3 for
catalog metadata. Schema ships as Turtle files alongside the data.

**I4 — Query-language equivalence (the commutation square).** At least two ISO/W3C-standardized
query languages and two independent open engines produce identical answer sets from the same data.
Example: DuckDB (SQL/PGQ) + Oxigraph (SPARQL 1.1) on Parquet + Turtle. **Testable as a CI
invariant** — canonical queries run against both engines on every commit. Divergence = build fail.

**Every DKA node and edge carries the RF2 universal versioning envelope:**
`(id, effectiveTime, active, moduleId)` — from SNOMED CT's Release Format 2 — plus
Datomic's assertion/retraction op-bit plus XTDB2 bitemporal dimensions (system_time, valid_time)
plus FHIR's `meta` block (`profile[]`, `security[]`, `tag[]`). This makes retention a schema
constraint — the storage engine refuses to write a node without a disposition-schedule reference.

---

## 5. The Derivative Metaphor — Correct Framing

Each DKA layer L_k is a **summarization operator Φ_k applied to L_{k-1}**, parameterized by a
volatility-scaled update cadence τ_k. Under small perturbations of L_{k-1}:

`δL_k ≈ Σ (∂Φ_k/∂x_i)·δx_i`

Higher-order operators (Themes) have larger operator norms on their inputs and therefore larger
update variance per unit base-layer change — directly analogous to Gamma > Delta sensitivity in
options pricing. Update cadence scales with observed volatility, not layer depth.

**The framing that does NOT survive peer review:** treating DKA layers as literal mathematical
derivatives in the differential geometry sense — the base is discrete, not continuous.

**The framing that does survive:** Taylor-style P&L attribution — every downstream claim is
traceable back through layers to L0 base facts, exactly as trading P&L is decomposed into Greeks.

---

## 6. The Yo-Yo Compute Model

```
Laptop-A (permanent — Linux Mint, 4 GB RAM)
│  service-fs · service-content (LadybugDB) · service-people
│  service-slm (Doorman) · service-verification · service-marketplace · service-state→merged into service-content
│
│  sanitised /ledger + /assets payload
│  SSH tunnel outbound only
▼
GCP Cloud Run GPU (transient — spins up for job, tears down on completion)
  SkyPilot 0.11+ manages spot instance lifecycle + auto-recovery
  Gemma 4 26B A4B (Apache 2.0 — self-hosted, pay compute only)
  text-embedding-005 (Google API — $0.006/MTok)
  Generates: graph delta · derivative layers (L1–L4) · questionnaires for service-verification
  Checkpoints to GCS every N minutes (atomic tmp-then-replace)
  SIGTERM handler flushes in-memory state within 2-minute GCP grace window
  Jobs idempotent by (input_hash, job_version)
│
│  graph delta + embeddings inbound
│  re-hydrated by service-slm with canonical entity IDs
▼
Laptop-A — LadybugDB updated — GCP node tears down
│
│  query time — no GCP node needed
▼
service-slm Role 2 → Claude API → L5 outputs (wiki · content gen · marketplace products)
```

**GPU cost estimates:**

| GPU | VRAM | Est. throughput | Est. time (2.5 GB) | Approx. cost |
|---|---|---|---|---|
| NVIDIA L4 | 24 GB | ~80 tok/s | ~85 hrs | ~$340 |
| NVIDIA A100 40GB | 40 GB | ~200 tok/s | ~35 hrs | ~$280 |
| NVIDIA A100 80GB | 80 GB | ~350 tok/s | ~20 hrs | ~$200 |

Trial run on 3 files calibrates actual tok/s before committing to full corpus spend.

---

## 7. The No Source of Truth Principle

The graph has no single authoritative record. This is intentional and by design.

- The same entity may appear under multiple name variants
- The same document may be classified to multiple CoA categories
- These are not errors — they are the natural state of unverified data
- Over time, Leiden community detection identifies co-occurring variants
- The system proposes canonical resolution; humans confirm via Verification
- Until confirmed, duplicates remain as separate nodes with `verified = false`
- Higher-confidence data commands premium pricing in the Marketplace
- Nodes are never deleted — only marked `active = false` with a reason

---

## 8. The Commercial Flywheel

```
More source data ingested
         ↓
More Topic nodes (L3) and Archetype clusters (L2)
         ↓
More wiki pages published (L5) + more audience segments defined
         ↓
More ad inventory + more data products
         ↓
More revenue to archive owner
         ↓
Incentive to ingest more data and invest in verification
         ↓
Higher-confidence graph → premium marketplace pricing
```

---

## 9. What Hyperscalers Structurally Cannot Build

1. **Data on customer hardware** — incompatible with consumption-metered storage + egress revenue
2. **100% marketplace revenue share** — AWS Data Exchange takes 3% + per-GB + per-grant-hour
3. **Self-generating taxonomy** — cannibalizes their Professional Services revenue line
4. **Cloud switching on 2-month notice without egress charges** — EU Data Act Art. 25 (force January 2027)
5. **SMB long-tail pricing** — hyperscaler discounting requires commitment tiers SMBs cannot meet
6. **CLOUD-Act-immune sovereignty** — Microsoft admitted in French court they cannot guarantee this
7. **Forkable OSS** that eliminates their proprietary service moat — doing so evaporates their lock-in

**The strategic position:** DKA is not a better knowledge graph. It is an irrevocable one. That is
the property hyperscalers cannot offer at any price, because offering it destroys the reason
customers rent from them in the first place.

---

## 10. Three Leapfrog Opportunities (2030)

**Leapfrog #1: The RF2 universal versioning envelope on every node.**
`(id, effectiveTime, active, moduleId)` + Datomic op-bit + XTDB2 bitemporality + FHIR meta.
No hyperscaler knowledge graph offers this natively. Compliance becomes a type-system property,
not a metadata overlay. Storage billed per-assertion-history, not per-byte — incompatible with
hyperscaler pricing models.

**Leapfrog #2: openEHR-inspired two-level L2 with gist + REA + OntoUML.**
Tiny stable Reference Model (Party/Role/Product/Place/Time/Event) + composable Archetype Model
compiled to runtime Operational Templates (SHACL + JSON Schema + GraphQL). Zero ontology
engineers required at deployment. Eliminates Professional Services revenue that hyperscalers depend
on. No canonical open-source property-graph REA implementation exists — DKA will be the first,
filling a 40-year gap in the literature.

**Leapfrog #3: Bootable portable sovereignty (OVA + IPFS CAR + OCI Artifacts).**
Ship as a bootable virtual appliance. Distribute graph payloads as content-addressed `.car` archives
with identical CIDs on both sides of an air-gap. Distribute updates as signed OCI Artifacts with
SLSA attestations. W3C DID + VC 2.0 identity layer. Hyperscalers cannot match this — platform
stickiness is their business model. A CLOUD-Act-immune bootable ejectable appliance is the exact
product their shareholders are structurally forbidden from building.

---

## 11. Architecture Decisions Confirmed

All confirmed decisions are tracked in `docs/DECISIONS.md`.

The most critical for Phase 1:
- LadybugDB (MIT, Kùzu fork) as primary graph DB — confirmed
- Three fixed domains for Woodfine — confirmed
- DARP as geometric property (four invariants) — confirmed
- SOC3 and DARP as governing regulatory frameworks — confirmed
- service-state merged into service-content — confirmed
- service-marketplace = one service, two modes — confirmed
- Verification is a feature switched on separately — confirmed
- 100% open source — confirmed
