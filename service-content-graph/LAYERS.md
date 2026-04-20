# LAYERS.md
# The Seven Layers — Detailed Reference
**Version:** 1 · April 19, 2026
**See also:** ARCHITECTURE.md · SCHEMA.md

---

## Layer 0 — Base Assets (service-fs)

**What it is:** The immutable legal record. Ground truth for the entire system.

**Contents:**
- `/source` — original files (.docx, .pdf, .xlsx) — WORM, SHA-256 sealed
- `/ledger` — structured .yaml metadata extracted from source (2.5 GB for Phase 1)
- `/assets` — prose .md extracted from source (2.5 GB for Phase 1)

**Rules:**
- Write-once, read-many (WORM). Nothing changes after write.
- SHA-256 sealed: if a single character changes, the checksum fails
- Never queried directly for intelligence — Layer 1 is the query layer
- Every record carries the RF2 versioning envelope: `(id, effectiveTime, active, moduleId)`
- Raw original file always vaulted before any processing begins (chain of custody, SYS-ADR-07)

**Update cadence:** Never changes after write.

**Who owns it:** service-fs

---

## Layer 1 — Knowledge Graph (First Derivative)

**What it is:** Relationships between base facts — the extractable intelligence from Layer 0.

**Contents:** Document nodes · Chunk nodes · Entity nodes · Metric nodes + all relationships

**Key design rules:**
- No single source of truth — duplicates are valid and expected
- Duplicates self-heal over time as more data arrives and patterns emerge
- Nodes are never deleted — marked `active = false` with reason if superseded or disputed
- Hybrid retrieval: LadybugDB graph traversal + NaviX HNSW vector index

**How it is built:**
Gemma 4 on GCP reads /ledger + /assets, extracts structured facts, maps relationships, computes
embeddings, and returns a graph delta. service-slm re-hydrates the delta with canonical entity IDs
from service-people and writes to LadybugDB.

**Update cadence:** Every batch job run. New nodes added; existing nodes never removed.

**Who owns it:** service-content (LadybugDB)

---

## Layer 2 — Archetypes + Chart of Accounts (Second Derivative)

**What it is:** The structural classification system — the governance and taxonomy backbone.

Two distinct but related components:

### Chart of Accounts

The CoA is not an accounting chart of accounts. It is an enterprise document classification
taxonomy anchored to compliance standards that apply across all industries and jurisdictions.

**The frozen compliance spine (~30 top-level categories):**

| Macro-Domain | Example Categories | Anchoring Standards |
|---|---|---|
| Entity & Governance | Org Context, Leadership, Strategy | ISO 37301, COBIT |
| Risk, Compliance & Controls | ERM, Obligations, Audit, Incidents | ISO 37301, SOC2, NIST CSF |
| Financial | Position, Performance, Cash Flows, Tax | IFRS/XBRL, GAAP |
| Operations & Products | Products, Customers, Supply Chain, Assets | SASB |
| People & Human Capital | Workforce, Health & Safety, DEI | GRI 400-series |
| Information & Technology | Records, Data Governance, Cybersecurity | ISO 15489, ARMA GARP, NIST CSF |
| Sustainability | Climate, Pollution, Biodiversity, Conduct | GRI, ESRS, ISSB S1/S2 |
| Stakeholder & Reporting | Engagement, Disclosures, Regulatory | XBRL, SOC3, DARP |

**The self-healing periphery:** crosswalks between GRI/ESRS/ISSB, synonym resolution, leaf-level
XBRL bindings, data-quality lineage repair. Healing means **raising a reviewable PR, not
committing**. Full autonomous mutation violates ISO 37301 §5 and SOC2 CC1.

**Temporary Classification bucket:** Any document that cannot be auto-classified goes here (GS1
GPC Brick 99999999 pattern). Formal graph node with change-request workflow. Never silently
dropped.

**Human override:** edit `seeds/coa.csv` → watchdog → downstream recomputation → CSV wins.

**Update cadence:** Frozen spine: 18-24 months (human sign-off required). Periphery: auto-heals
continuously.

### Archetypes

Discovered via community detection, then matched to approved standards. Each archetype node
carries two labels simultaneously: a canonical URI from an approved standard (stable,
compliance-auditable) and an emergent community label derived from data (descriptive,
self-updating).

**Five-layer stack:**

| Layer | Contents | Cadence |
|---|---|---|
| L0 — Upper Ontology | BFO (ISO/IEC 21838-2) — ~30 universal primitives | Rarely |
| L1 — Standards-Anchored Canonical | FIBO + GICS/NAICS + Schema.org + GLEIF + ArchiMate | Quarterly, immutable IDs |
| L2 — Enterprise Domain Types | DKA-native ObjectTypes derived from L1 + corpus | Monthly |
| L3 — Role/Shape Archetypes | RACI + FCA SMF + Jungian 12 brand archetypes | Weekly |
| L4 — Emergent Community Labels | Leiden pipeline output, `skos:closeMatch` to L1–L2 | Continuous |

**L2 foundation:** gist (CC-BY, ~100–135 classes) as the domain-neutral kernel + REA
(Resource-Event-Agent, ISO/IEC 15944-4) as the economic-semantics module + OntoUML as
meta-validator catching rigidity and identity errors. Target: ~150 classes, ~100 properties.

**Leiden promotion pipeline:** Raw clusters → embeddings → hierarchical Leiden at 4 resolution
levels → centroid + TF-IDF signature → alignment against L1 standards → accept (≥τ_high) /
queue for review (τ_low–τ_high) / register emergent-only (< τ_low). Promotion to archetype
requires ≥3 consecutive weekly runs with ≥75% Jaccard overlap and ARI ≥ 0.6.

**Human override:** edit `seeds/archetypes.csv` → propagates.

**Update cadence:** 24+ months.

---

## Layer 3 — Domains · Glossaries · Topics (Third Derivative)

**What it is:** Operational classification with vocabulary and subject clusters.

**Critical clarification: These are INDEX NODES — not content.**

### Domains

- Unlimited in number
- **Woodfine fixed defaults:** Corporate · Projects · Documentation
- **Other deployments:** configure any number in `seeds/domains.csv`
- Domains emerge from content clustering; operator provides initial names

### Glossaries (one SET per domain)

Each domain has its **own** Glossary node set — not a shared global Glossary.

```
Glossary node {
  id:            STRING
  term:          STRING       -- e.g. "Reporting Issuer"
  definition:    STRING       -- extracted from corpus usage context
  domain_id:     STRING       -- which domain this Glossary belongs to
  source_chunks: STRING[]     -- Chunk IDs where the term was defined
  first_seen:    DATE
  document_count: INT64
}
```

Glossary nodes are extracted terms with definitions — they are NOT wiki pages. Each Glossary node
becomes one Glossary entry at Layer 5.

### Topics (one SET per domain)

Each domain has its **own** Topic node set — not a shared global Topic list.

```
Topic node {
  id:            STRING
  label:         STRING       -- synthesized subject heading
  domain_id:     STRING       -- which domain this Topic belongs to
  description:   STRING       -- brief synthesized description
  chunk_ids:     STRING[]     -- contributing Chunk IDs from Layer 1
  entity_ids:    STRING[]     -- related Entity IDs
  metric_ids:    STRING[]     -- related Metric IDs
  wiki_status:   STRING       -- "not_generated" | "draft" | "approved" | "published"
  first_seen:    DATE
  last_active:   DATE
}
```

Topic nodes are structured pointers — they are NOT wiki pages. Each Topic node corresponds to
one future Wiki page. **The Topic node is the recipe. The wiki page is the dish.**

**How a Topic becomes a Wiki page (the two-step at Layer 5):**

```
Topic node (L3)
  contains: label + description + chunk_ids + entity_ids
        ↓
service-slm Role 2 assembles context:
  reads contributing Chunks from Layer 1 (the actual text)
  reads related Entities and Metrics
  reads active Themes referencing this Topic
  builds context window (~2,000–4,000 tokens)
        ↓
Claude API (L5 generation):
  synthesizes prose wiki page
  Pydantic + instructor enforces citation grounding
  every claim linked to L0 source: {"claim": "...", "support": ["L0-asset-id:char-offset"]}
  unsupported claims flagged or dropped
        ↓
Operator approves (SYS-ADR-19)
        ↓
Wiki page published
```

**Update cadence:** Domains: 12+ months. Glossaries/Topics: weekly batch.

---

## Layer 4 — Themes (Fourth Derivative)

**What it is:** Analytical output showing where data is moving over time.

Themes are NOT static labels. A Theme is detected when Topic clusters co-occur with high frequency
across documents in a recent time window. The Theme timeline is the system's built-in trend
detection.

- **Active Theme:** Currently recurring pattern (high co-occurrence in recent batches)
- **Archived Theme:** Previously active, now fading — flagged `active = false`, never deleted

**What Themes feed:**
- Content Generation: Claude is briefed on active Themes for context
- Wiki pages: pages note "this topic is currently active in [N] recent documents"
- Ad Exchange: active Themes create premium audience segments
- Data Marketplace: Theme timelines are a high-value data product

**Commercial value:** Theme history is a documented institutional trend record. Archived Themes
with date ranges are valuable for ESG analysts, researchers, and institutional data buyers.

**Update cadence:** Re-evaluated every weekly batch.

---

## Verification Layer — Optional

**What it is:** Distributed human-in-the-loop confidence scoring. A switchable paid feature.

**Position:** Above Layer 5 outputs. The archive functions without it. Higher verification produces
higher-confidence data, which commands premium pricing in the Data Marketplace and Ad Exchange.

**Key design:** Questionnaires generated FROM graph nodes by Gemma 4 during the GCP batch job.
Distributed to verifiers via os-console. Verifiers use their own browser for external lookups (air-
gapped protocol — system never touches LinkedIn or external services). Results update confidence
scores on LadybugDB nodes.

**See:** VERIFICATION.md for full specification.

---

## Layer 5 — Output and Utility (Fifth Derivative)

**What it is:** Four commercial outputs generated FROM derivative layers. Never generated directly
from Layer 0 base assets.

See ARCHITECTURE.md §3 and MARKETPLACE.md for full specifications.

**The Diátaxis wiki structure:**

| Page type | Source | Generation | Human step |
|---|---|---|---|
| Reference | Entity/Metric nodes (L1) | Fully auto-generated | Approval before publish |
| Explanation | Topic + Archetype (L2–L3) | Claude API synthesis | Approval before publish |
| How-To | Theme + Topic (L3–L4) | Claude API synthesis | Approval before publish |
| Tutorial | n/a | Human-authored only | Full authorship |

**Self-healing scope by layer (summary):**

| Layer | Self-healing | Human required |
|---|---|---|
| L1 Knowledge Graph | Fully automatic | None |
| L2 Archetype periphery | Automatic via Leiden | Review on ambiguous alignment |
| L2 CoA frozen spine | NOT automatic | Sign-off on any change |
| L2 CoA periphery | Automatic (crosswalks, synonyms) | Review on conflict |
| L3 Domains | Automatic on new CSV row | One row addition |
| L3 Glossaries + Topics | Fully automatic weekly | None |
| L4 Themes | Fully automatic weekly | None |
| L5 Wiki pages | Staged as draft overlays | Approval before publish (SYS-ADR-19) |
