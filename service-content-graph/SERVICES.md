# SERVICES.md
# Service Definitions and File Tree
**Version:** 1 · April 19, 2026
**Principle: Minimum services. A service is only added if it owns distinct data,
has a distinct outbound connection pattern, or has a distinct lifecycle.**

---

## File Tree (Laptop-A)

```
Laptop-A (Linux Mint, 4 GB RAM — substitute Totebox Archive)
│
├── service-fs/
│     ├── data/service-fs/
│     │     ├── source/               ← /source (original files, WORM, SHA-256)
│     │     └── ledger/               ← append-only transaction log (CSV)
│     ├── data/service-content/       ← LadybugDB graph database files
│     ├── data/service-people/        ← identity ledger
│     ├── data/service-slm/           ← doorman transaction log
│     ├── data/service-verification/  ← questionnaire + response ledger
│     └── data/service-marketplace/  ← data product catalog + ad segments
│
├── service-content/                  ← Knowledge Graph (Layers 1–4) + state
│     ├── graph/
│     │     └── knowledge.db          ← LadybugDB: graph + vector index (NaviX HNSW)
│     ├── seeds/
│     │     ├── coa.csv               ← Chart of Accounts seed (human-editable)
│     │     ├── archetypes.csv        ← Archetype overrides (human-editable)
│     │     └── domains.csv           ← Domain names and descriptions (human-editable)
│     ├── state/
│     │     ├── sync.db               ← SQLite: ingestion state, dedup, hash tracking
│     │     └── snapshots/            ← YAML exports (SOC3 audit trail)
│     └── derivative-engine/          ← orchestration scripts (dbt + Dagster)
│
├── service-people/                   ← Identity resolution feeder
│     ├── canonical.json              ← name → entity ID mappings
│     ├── variants.json               ← name variant ledger
│     └── organizations.json          ← organization index
│
├── service-slm/                      ← Doorman (no local model — two outbound roles)
│     ├── outbound/                   ← sanitised payloads pending send
│     ├── inbound/                    ← received graph deltas
│     └── log/                        ← doorman transaction log (CSV)
│
├── service-extraction/               ← [Phase 2 — NOT IN SCOPE for Phase 1]
│
├── service-verification/             ← Distributed verification (optional feature)
│     ├── questionnaires/             ← pending tasks generated from graph nodes
│     ├── responses/                  ← submitted verifier answers
│     └── ledger/                     ← verification transaction log (CSV, append-only)
│
└── service-marketplace/              ← Data Marketplace + Ad Exchange (one service, two modes)
      ├── products/                   ← DCAT v3-described data products
      ├── segments/                   ← IAB Audience Taxonomy segment definitions
      └── ledger/                     ← transaction log (CSV)
```

---

## service-content (includes state — formerly service-state)

**Owns:** The complete knowledge graph (Layers 1–4). Also owns sync state and audit
snapshots. service-state was merged into service-content because state tracking is
internal to graph management — keeping it separate adds a process boundary around
a concern service-content fully owns.

**What it holds:**
- `graph/knowledge.db` — LadybugDB property graph + NaviX HNSW vector index
- `seeds/` — human-editable CSV overrides for CoA, archetypes, domains
- `state/sync.db` — SQLite tracking which files are ingested, hash deduplication
- `state/snapshots/` — periodic YAML exports for SOC3 audit trail

**CSV override pattern:**
Operator edits CSV in `seeds/` → watchdog (debounced, 10-second polling fallback)
→ dbt seed → downstream layer recomputation → CSV value wins → provenance recorded.
Layer tables store `(value, source ∈ {auto, human}, confidence, reason)`.

**Snapshot cadence:** YAML snapshot generated after every batch job. Each snapshot
is dated and immutable after write — the SOC3 audit trail. Adding new data over
time is tracked through snapshots so graph growth history is fully auditable.

---

## service-slm — Doorman (Two Outbound Roles, No Local Model)

**No local model runs on Laptop-A.** service-slm is the orchestration and sanitisation
gateway. Two outbound roles, same protocol: sanitise outbound, compute externally,
re-hydrate inbound.

**Role 1 — Yo-Yo Bridge (graph build):**
1. Reads /ledger + /assets from service-fs
2. Sanitises payload: strips PII, coordinates, sensitive identifiers
3. Sends to GCP provisional node via SSH tunnel (outbound only)
4. GCP: Gemma 4 extracts entities, maps relationships, synthesizes derivative layers, embeds Chunks
5. Receives graph delta inbound from GCP
6. Re-hydrates delta with canonical entity IDs from service-people
7. Writes to service-content (LadybugDB)
8. GCP node tears down

**Role 2 — Content Generation Bridge (query time, no GCP node needed):**
1. Embeds operator query via text-embedding-005
2. Hybrid search: LadybugDB vector (NaviX HNSW) + graph traversal
3. Assembles context: Chunks + Entities + Metrics + Themes + CoA classification + active Topics
4. Sends assembled context to Claude API (Anthropic) outbound
5. Claude API returns L5 output (wiki page, report, analysis)

**The Doorman discipline applies to both roles equally.** Target changes (GCP node OR
Claude API). Protocol does not change.

---

## service-people — Identity Resolution Feeder

**Owns:** Canonical entity identity resolution. Entity nodes live in service-content's
LadybugDB graph. service-people manages the resolution logic that maps name variants
to canonical IDs before graph write.

**What it is NOT:** service-people does not own entity nodes. It is a feeder service.

**Verification integration:** Confidence updates to entity nodes flow through
service-verification → service-people → service-content graph node update.

---

## service-verification — Distributed Verification (Optional Feature)

**Status:** Feature that is switched on separately. Archive runs without it. Enabling
it unlocks premium marketplace pricing.

**Owns:** The questionnaire lifecycle — generation, distribution, response collection,
confidence scoring.

**Questionnaire generation:** Gemma 4 on GCP batch node generates questionnaires from
graph nodes with `confidence < threshold` or `verified = false`. Questionnaires are
queued in service-verification.

**Verifier access:** Via os-console from any location. One questionnaire at a time.
Verifier uses their own personal browser for external lookup. The system never touches
LinkedIn or external services directly (air-gapped protocol preserved).

**Example LinkedIn workflow:**
1. System shows: "Entity: Victoria Johnson · Extracted company: Woodfine Management Corp."
2. Verifier opens LinkedIn in personal browser
3. Finds correct profile
4. Copies LinkedIn URL
5. Pastes URL into os-console response field and submits
6. Graph node updated: `verified = true · confidence = 0.85 · source_url = [URL]`

**Confidence model:**

| Responses | Agreement | Result |
|---|---|---|
| 1 | Confirmed | `confidence = 0.65` — partially verified |
| 2 | Both agree | `confidence = 0.85` — verified |
| 2 | Disagree | `status = disputed` — queued for review |
| 3+ | 2/3 agree | `confidence = 0.90+` — high confidence |
| Any | + URL evidence | Additional +0.05 |

**Payment:** Polygon USDC primary (~$0.002/tx), Lightning Network (LNbits) backup.
See MARKETPLACE.md for micropayment details.

**SOC3 audit trail:** Every verification transaction logged in `service-verification/ledger/`:
`(questionnaire_id, verifier_id, timestamp, response, url_evidence, confidence_before, confidence_after)`
Append-only CSV — never deleted.

---

## service-marketplace — Data Marketplace + Ad Exchange (One Service, Two Modes)

**Why one service:** Both modes read the same derivative data (L2–L4). Both require
the same consent/privacy layer. Both use IAB taxonomy standards. The operational
difference is a protocol difference, not a service boundary.

See MARKETPLACE.md for full specification.

---

## service-extraction — Phase 2 Only

Converts /source → /ledger + /assets using Gemma 4 on GCP node.
NOT IN SCOPE for Phase 1. The 2.5 GB corpus already exists in /ledger and /assets.

---

## RAM Envelope (Laptop-A, 4 GB)

| Component | RAM |
|---|---|
| Linux Mint baseline | ~1.2 GB |
| Python orchestrator | ~150 MB |
| LadybugDB buffer pool (capped at 1 GB) | ~1.0 GB |
| service-state (SQLite) | ~100 MB |
| Headroom | ~550 MB |
| Local inference model | Not feasible on Laptop-A — 0 MB |
