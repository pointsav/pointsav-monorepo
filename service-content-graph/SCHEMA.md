# SCHEMA.md
# LadybugDB Graph Schema
**Version:** 1 · April 19, 2026
**Database:** service-content/graph/knowledge.db (LadybugDB — MIT, Kùzu fork)
**Every node carries the RF2 universal envelope: (id, effectiveTime, active, moduleId)**

---

## Design Principles

- No source of truth — duplicates are valid, expected, and self-heal over time
- Nodes are never deleted — set `active = false` with reason if superseded or disputed
- Every node carries `verified` (boolean) and `confidence` (float 0.0–1.0) properties
- All timestamps in UTC ISO 8601
- SHA-256 addressing: `id = SHA-256(canonical_serialization(node))` for DARP I1 compliance

---

## Node Tables

### Document
One node per /ledger + /assets file pair.

```cypher
CREATE NODE TABLE Document(
  id              STRING,     -- SHA-256(filepath + date)
  domain          STRING,     -- "corporate" | "projects" | "documentation" | custom
  doc_type        STRING,     -- inferred from content
  date            DATE,
  source_path     STRING,     -- path to /source file
  ledger_path     STRING,     -- path to /ledger .yaml
  assets_path     STRING,     -- path to /assets .md
  contact_name    STRING,
  sha256_hash     STRING,     -- SHA-256 of /source for WORM integrity
  ingest_date     DATE,
  -- RF2 universal envelope
  effectiveTime   DATE,       -- when this version became effective
  active          BOOLEAN DEFAULT true,
  moduleId        STRING,     -- namespace (e.g. "woodfine-corporate")
  -- Verification
  verified        BOOLEAN DEFAULT false,
  confidence      FLOAT DEFAULT 0.0,
  PRIMARY KEY(id)
)
```

### Chunk
Retrievable unit with vector embedding. The atomic unit of retrieval.

```cypher
CREATE NODE TABLE Chunk(
  id             STRING,   -- Document.id + "_chunk_" + seq
  document_id    STRING,
  content        STRING,
  chunk_type     STRING,   -- "prose" | "qa_pair" | "metric_block" | "asset_list"
  token_count    INT64,
  embedding      FLOAT[768],   -- nomic-embed-text-v1.5, Matryoshka 768-dim
  seq_in_doc     INT64,        -- position in document for ordering
  -- RF2 envelope
  effectiveTime  DATE,
  active         BOOLEAN DEFAULT true,
  moduleId       STRING,
  -- Verification
  verified       BOOLEAN DEFAULT false,
  confidence     FLOAT DEFAULT 0.0,
  PRIMARY KEY(id)
)
```

### Entity
Person or organization. Canonical identity resolved by service-people.

```cypher
CREATE NODE TABLE Entity(
  id              STRING,   -- canonical slug (service-people resolves duplicates)
  name            STRING,   -- canonical name
  entity_type     STRING,   -- "person" | "organization" | "regulator" | "property"
  organization    STRING,
  role            STRING,
  jurisdiction    STRING,
  source_url      STRING,   -- LinkedIn URL or other verification source
  -- RF2 envelope
  effectiveTime   DATE,
  active          BOOLEAN DEFAULT true,
  moduleId        STRING,
  -- Verification
  verified        BOOLEAN DEFAULT false,
  confidence      FLOAT DEFAULT 0.0,
  PRIMARY KEY(id)
)
```

### Metric
Quantitative value extracted from /ledger YAML. Deterministic path (SYS-ADR-07).

```cypher
CREATE NODE TABLE Metric(
  id           STRING,   -- Document.id + "_" + key
  document_id  STRING,
  key          STRING,   -- e.g. "min_investment_threshold"
  value        STRING,   -- stored as string, typed on retrieval
  unit         STRING,   -- "CAD" | "days" | "count" | "%" | null
  domain       STRING,
  -- RF2 envelope
  effectiveTime DATE,
  active       BOOLEAN DEFAULT true,
  moduleId     STRING,
  PRIMARY KEY(id)
)
```

### Domain
Operational area. Configured from seeds/domains.csv.

```cypher
CREATE NODE TABLE Domain(
  id          STRING,
  label       STRING,   -- "corporate" | "projects" | "documentation" | custom
  description STRING,
  PRIMARY KEY(id)
)
```

### Glossary
Term definition extracted from corpus. One SET per domain.

```cypher
CREATE NODE TABLE Glossary(
  id             STRING,
  term           STRING,
  definition     STRING,   -- synthesized from usage context in /assets
  domain_id      STRING,   -- which domain this belongs to
  first_seen     DATE,
  document_count INT64,
  PRIMARY KEY(id)
)
```

### Topic
Subject cluster. One SET per domain. Maps 1:1 to a future wiki page.

```cypher
CREATE NODE TABLE Topic(
  id          STRING,
  label       STRING,   -- synthesized subject heading
  domain_id   STRING,   -- which domain this belongs to
  description STRING,   -- brief synthesized description
  wiki_status STRING DEFAULT 'not_generated',
              -- "not_generated" | "draft" | "approved" | "published"
  first_seen  DATE,
  last_active DATE,
  PRIMARY KEY(id)
)
```

### Archetype
Synthesized organizational identity pattern — dual-labeled.

```cypher
CREATE NODE TABLE Archetype(
  id               STRING,
  label            STRING,   -- emergent community label from Leiden
  canonical_uri    STRING,   -- standards-anchored URI (FIBO/GICS/NAICS/gist)
  canonical_label  STRING,   -- e.g. "Real Estate Management & Development"
  archetype_layer  INT64,    -- 0-4 (see LAYERS.md)
  description      STRING,
  first_seen       DATE,
  last_updated     DATE,
  document_count   INT64,
  PRIMARY KEY(id)
)
```

### ChartOfAccounts
CoA classification node. The compliance backbone.

```cypher
CREATE NODE TABLE ChartOfAccounts(
  id           STRING,
  category     STRING,   -- e.g. "Stakeholder & Reporting"
  macro_domain STRING,   -- one of the 8 macro-domains
  parent_id    STRING,   -- null for top-level categories
  description  STRING,
  source       STRING,   -- "frozen_spine" | "periphery"
  xbrl_element STRING,   -- XBRL binding where applicable
  PRIMARY KEY(id)
)
```

### Theme
Active or archived temporal trend. Shows where data is moving over time.

```cypher
CREATE NODE TABLE Theme(
  id          STRING,
  label       STRING,
  description STRING,
  active      BOOLEAN DEFAULT true,
  first_seen  DATE,
  last_seen   DATE,
  PRIMARY KEY(id)
)
```

---

## Relationship Tables

```cypher
-- Layer 0 → Layer 1
CREATE REL TABLE HAS_CHUNK(FROM Document TO Chunk)
CREATE REL TABLE AUTHORED_BY(FROM Document TO Entity)
CREATE REL TABLE REFERENCES_ENTITY(FROM Document TO Entity)
CREATE REL TABLE CHUNK_REFERENCES_ENTITY(FROM Chunk TO Entity, mention_count INT64)
CREATE REL TABLE RESPONDS_TO(FROM Chunk TO Chunk)          -- Q&A pair linkage
CREATE REL TABLE CONTAINS_METRIC(FROM Chunk TO Metric)
CREATE REL TABLE SUPERSEDES(FROM Document TO Document)     -- inferred from file tree
CREATE REL TABLE CO_OCCURS(FROM Entity TO Entity, count INT64, last_seen DATE)

-- Layer 1 → Layer 2
CREATE REL TABLE CLASSIFIED_AS(FROM Document TO ChartOfAccounts, confidence FLOAT)
CREATE REL TABLE EXPRESSES_ARCHETYPE(FROM Document TO Archetype, confidence FLOAT)

-- Layer 2 → Layer 3
CREATE REL TABLE BELONGS_TO_DOMAIN(FROM Document TO Domain)
CREATE REL TABLE CHUNK_IN_DOMAIN(FROM Chunk TO Domain)
CREATE REL TABLE TOPIC_IN_DOMAIN(FROM Topic TO Domain)
CREATE REL TABLE GLOSSARY_IN_DOMAIN(FROM Glossary TO Domain)
CREATE REL TABLE DEFINES_TERM(FROM Chunk TO Glossary)
CREATE REL TABLE RELATES_TO_TOPIC(FROM Chunk TO Topic, relevance_score FLOAT)

-- Layer 3 → Layer 4
CREATE REL TABLE TOPIC_HAS_THEME(FROM Topic TO Theme)
CREATE REL TABLE DOCUMENT_TAGGED_THEME(FROM Document TO Theme)
CREATE REL TABLE CHUNK_TAGGED_THEME(FROM Chunk TO Theme)
```

---

## Vector Index (NaviX HNSW)

LadybugDB ships with the NaviX HNSW vector index inherited from Kùzu. Configured on
the Chunk table's `embedding` column for semantic similarity retrieval.

For DuckPGQ fallback (if using DuckDB instead):
```sql
INSTALL vss;
LOAD vss;
CREATE INDEX chunk_embedding_idx ON Chunk USING HNSW (embedding)
WITH (metric = 'cosine', ef_construction = 128, M = 16);
```

---

## Hybrid Query Pattern

```cypher
// Find semantically similar Chunks, then expand to Documents, Entities, CoA, Themes
MATCH (c:Chunk)
WHERE vector_cosine_similarity(c.embedding, $query_embedding) > 0.7
WITH c ORDER BY vector_cosine_similarity(c.embedding, $query_embedding) DESC LIMIT 20
MATCH (d:Document)-[:HAS_CHUNK]->(c)
OPTIONAL MATCH (d)-[:CLASSIFIED_AS]->(coa:ChartOfAccounts)
OPTIONAL MATCH (d)-[:DOCUMENT_TAGGED_THEME]->(t:Theme WHERE t.active = true)
OPTIONAL MATCH (c)-[:CHUNK_REFERENCES_ENTITY]->(e:Entity WHERE e.verified = true)
RETURN d, c, coa, collect(DISTINCT t) AS themes, collect(DISTINCT e) AS entities
ORDER BY similarity DESC LIMIT 5
```

---

## DARP Dual-Engine Commutation Test (I4)

The schema must produce identical answer sets from both LadybugDB (Cypher) and a
SPARQL engine (Oxigraph) reading the same data exported as Parquet + Turtle.

These commutation tests run as CI assertions on every commit. Divergence = build failure.

Sample canonical test queries (to be defined during trial):
1. "Return all Documents classified under Stakeholder & Reporting with confidence > 0.7"
2. "Return all Entities co-occurring with Victoria Johnson"
3. "Return the complete SUPERSEDES chain for the most recent investor relations document"
4. "Return all active Themes in the Corporate domain"
5. "Return all Chunks relating to the minimum investment threshold Metric"
