
# service-content — Architecture (post-rebuild)

> The Ring 2 knowledge runtime for Foundry tenants. Per-tenant graph and vector
> store with temporal mutations and an MCP server interface. Replaces the legacy
> file-watcher implementation.

**Status**: rebuild blueprint for project-data Task. The existing
`src/main.rs` (legacy file-watcher with hardcoded deployment paths) is
deprecated and replaced.

**Doctrine alignment**: claims #44 (graph-grounded apprenticeship), #46
(MCP-as-substrate-protocol), #47 (seed taxonomy), #48 (customer-owned graph IP),
#54 (substrate-without-inference base case).

---

## §1 — What this service is

service-content is the **per-tenant knowledge runtime** for the Foundry
substrate. It holds:

- A property graph (entities, relationships, properties) per tenant
- A vector store (embeddings for semantic search) per tenant
- The seed taxonomy (Archetypes, Chart of Accounts, Domains, Themes) per tenant
- An audit log of all mutations (composes with service-fs WORM ledger)

It exposes itself as an MCP server. Other services (service-slm Doorman,
service-extraction, service-input, customer extensions) consume it as MCP
clients.

It does NOT generate text. It does NOT call LLMs. It is a Ring 2 data service.
AI-grounding requests come from the Doorman.

## §2 — Why the rebuild

The current `src/main.rs` is a legacy file-watcher with several structural
problems:

- Hardcoded path `/home/mathew/deployments/woodfine-fleet-deployment/...` (a
  deployment path that may not exist)
- Hardcoded SLM endpoint at port 8082 (pre-Doorman; bypasses claim #43)
- No graph database — flat JSON files
- No vector search — keyword classification only
- No MCP server — the file-watcher pattern is not composable
- Retired naming ("PointSav Semantic Watcher", "Schema Expansion Routing")

The rebuild is structural, not cosmetic. The directory structure, the data
shape, and the wire contract all change.

## §3 — Target file layout

```
service-content/
├── README.md
├── README.es.md
├── ARCHITECTURE.md            (this document)
├── DEVELOPMENT.md
├── CLAUDE.md
├── NEXT.md
├── Cargo.toml
├── src/
│   ├── main.rs                MCP server bootstrap; binds 9101
│   ├── mcp_server.rs          MCP tool/resource exposure
│   ├── graph/
│   │   ├── mod.rs
│   │   ├── store.rs           graph storage (SQLite-graph short-term)
│   │   ├── query.rs           Cypher-subset query engine
│   │   ├── mutate.rs          atomic multi-op mutations
│   │   └── temporal.rs        append-only versioning
│   ├── vector/
│   │   ├── mod.rs
│   │   ├── store.rs           sqlite-vec embedding store
│   │   ├── search.rs          k-NN search
│   │   └── embed.rs           Doorman-mediated embedding generation
│   ├── tenant.rs              moduleId isolation
│   ├── seed/
│   │   ├── mod.rs
│   │   ├── pack_loader.rs     install/update/contribute Vertical Pack
│   │   ├── archetypes.rs
│   │   ├── chart_of_accounts.rs
│   │   ├── domains.rs
│   │   └── themes.rs
│   ├── api/
│   │   ├── context.rs         POST /v1/graph/context (Doorman pre-inference)
│   │   ├── mutate.rs          POST /v1/graph/mutate (Doorman post-verdict)
│   │   ├── query.rs           POST /v1/graph/query (deterministic ops)
│   │   ├── search.rs          POST /v1/graph/search (vector)
│   │   └── export.rs          POST /v1/graph/export (transfer bundle)
│   ├── audit.rs               composes with service-fs ledger
│   └── error.rs
├── seeds/
│   └── default/               default pack (Foundry universal floor)
├── packs/                     installed Vertical Seed Packs (per-tenant)
├── tests/
└── docs/
```

## §4 — Graph store choice

**Short-term (Phase 3)**: SQLite-graph. Reasons:

- Pure Rust via `rusqlite`; no separate database process
- Fits Tier 0 hardware footprint (Totebox)
- Cypher subset implementable as a Rust query layer
- File-based; trivially backed up and transferred
- Aligns with claim #54 (substrate-without-inference); SQLite operates without
  any external dependency

**Long-term (Phase 4+ optional)**: LadybugDB if it stabilizes (community fork
of Kuzu post-Apple acquisition, October 2025; MIT license; Rust SDK).

The rebuild scaffolds against an abstract `GraphStore` trait so the backend can
be swapped without rewriting the consumer logic.

## §5 — Vector store choice

`sqlite-vec` (SQLite extension via `sqlite-vec-rs`). Reasons:

- Same SQLite database as the graph; co-located indexes
- Pure Rust; no separate vector DB process
- Sufficient for SMB-scale (tens of thousands of vectors per tenant)
- Foundry-tenant scale (Woodfine corpus) is within this range

For larger deployments (regional hospital scale), a `qdrant` side-car becomes
feasible without changing the API surface.

## §6 — MCP server interface

Tools exposed:

```
graph_query(module_id, query_terms, max_depth, max_nodes) → subgraph
graph_mutate(module_id, operations) → applied_count
vector_search(module_id, query_embedding | query_text, top_k) → matches
temporal_query(module_id, entity_id, at_time) → historical_state
seed_get(module_id, category) → entities
seed_update(module_id, category, entity, change) → ok
pack_install(module_id, pack_name, version) → diff
pack_contribute(module_id, pack_name, additions) → contribution_id
export_bundle(module_id, scope) → bundle_url
```

Resources exposed:

```
graph://{module_id}/{entity_id}              → entity (read-only)
graph://{module_id}/{entity_id}/relations    → relations (read-only)
seed://{module_id}/archetypes/{archetype_id} → seed entity (read-only)
ledger://{module_id}/{audit_id}              → audit entry (read-only)
```

## §7 — Per-tenant moduleId isolation

The `module_id` is the tenant scope. Every tool call requires `module_id`.
Refuses cross-tenant traversal. Enforced at:

- HTTP request layer (URL or header)
- Database query layer (every query has a `module_id` WHERE clause)
- Audit ledger entry (every event records `module_id`)

Two tenants on the same Totebox (e.g., a multi-tenant scenario) operate fully
isolated. The same code serves both with no cross-tenant leakage.

## §8 — Composition with service-slm Doorman

```
[Doorman receives /v1/chat/completions]
    ↓
[Doorman calls graph_query MCP tool]
    ↓ (via local socket or 127.0.0.1:9101)
[service-content returns subgraph + context_summary]
    ↓
[Doorman injects context into system prompt]
    ↓
[Doorman dispatches to Tier A / B / C]
    ↓
[Doorman receives response]
    ↓
[Verdict signing path (when applicable)]
    ↓
[Doorman calls graph_mutate MCP tool with parsed entities/edges]
    ↓
[service-content applies mutations atomically; writes audit]
    ↓
[Doorman writes audit ledger entry; returns response to caller]
```

Each step is observable. The audit ledger covers the full chain.

## §9 — Composition with service-extraction (ingest path)

```
[New file lands in service-input]
    ↓
[service-input writes to service-fs WORM ledger]
    ↓
[service-extraction reads from service-fs]
    ↓
[service-extraction tokenizes + deterministic parsing]
    ↓
[service-extraction calls Doorman (Tier B for AI extraction) — IF available]
    ↓
[Doorman calls service-content graph_query for grounding]
    ↓
[Doorman dispatches; receives entities]
    ↓
[Doorman calls service-content graph_mutate to insert entities]
    ↓
[service-content writes to graph + audit]
```

When the Doorman returns no inference (claim #54 base case):
service-extraction's deterministic-only output writes to the graph via a direct
`graph_mutate` call. The graph still grows, without AI enrichment.

## §10 — The seed taxonomy as graph nodes

When a Vertical Seed Pack is installed (`pack_install`), each entity becomes a
graph node with type `Archetype`, `ChartOfAccount`, `Domain`, or `Theme`.
Properties include the `gravity_keywords` array. Relations connect entities to
documents and to each other.

A document classification (during service-extraction) becomes:

```
[document_node] —[classified-as]→ [coa_node]
              —[matches-archetype]→ [archetype_node]
              —[in-domain]→ [domain_node]
              —[supports-theme]→ [theme_node]
```

Subsequent `graph_query` calls return these relationships as grounding context.

## §11 — Audit ledger composition with service-fs

service-content does not store audit entries itself. Every mutation writes
through `audit_ledger.append()`, which calls service-fs's ledger via MCP. The
WORM property is preserved at service-fs. service-content holds a denormalized
read-cache for graph query performance.

## §12 — Implementation phasing

| Phase | Effort | Outcome |
|---|---|---|
| 3a | 1 week (project-data Task) | MCP server skeleton + SQLite-graph store + basic graph_query/graph_mutate |
| 3b | 1 week | Vector store + vector_search; embedding via Doorman |
| 3c | 1 week | Seed pack loader + temporal queries + per-tenant isolation tests |
| 3d | 1 week | Doorman integration; audit ledger composition; export_bundle for transfer |

Total: approximately 4 weeks Task time. Composes with project-slm's Doorman MCP
gateway work (Phase 4) — the two clusters coordinate via outbox.

## §13 — Migration from legacy

The existing `service-content/seeds/*.json` files are preserved (reference for
the default pack). The existing legacy `src/main.rs` is deleted; the existing
`content-compiler/src/main.rs` is folded into service-extraction's
deterministic-extraction path. The `ontology/*.csv` files become the
`gravity_keywords` source for the default pack.

Customer-tenant data (currently in `service-fs/data/...` deployment paths)
imports into the new graph at first-boot of the rebuilt service. A one-shot
migration script handles this; it runs in the deployment-instance scope, not
workspace tier.

## Provenance

Research reviewed: direct code read of legacy `service-content/src/main.rs`
(confirmed pre-Doorman bypass and hardcoded paths); direct code read of
`content-compiler/src/main.rs` (deterministic; good baseline for
service-extraction); direct read of seed JSON files (confirmed taxonomy
structure); KuzuDB → LadybugDB migration research; SQLite-graph patterns
(rusqlite + custom Cypher subset); sqlite-vec (vector store as SQLite
extension); Microsoft GraphRAG patterns; Apprenticeship Substrate §7B
(capture-on-completion alignment); service-fs WORM ledger MCP integration
(already deployed v0.1.23); MCP specification 2025-11-25; Foundry workspace
v0.1.85 §7C Brief Queue Substrate; Three-Ring Architecture Ring 2 placement.

Suggested next research: (1) Cypher subset specification (which SQL operations
support which Cypher patterns); (2) vector embedding source — which model, how
often, embedded by what; (3) graph schema versioning and migration patterns;
(4) multi-tenant graph performance benchmarking on Totebox-class hardware.

**OQ #1 — Embedding model selection.** `vector_search` needs an embedding
model. Options: same OLMo 2 1B model (consistency); a smaller dedicated
embedding model (e.g., bge-small or sentence-transformers in GGUF); rely on
the Doorman to provide embeddings. Pending project-data Task investigation
during Phase 3a.

**OQ #2 — Graph snapshot frequency.** How often does service-content checkpoint
a graph snapshot for the transfer-bundle? Options: on every mutation (heavy);
periodic timer; on operator command. Pending operational decision.

## References

- `DOCTRINE.md` claims #44, #46, #47, #48, #54
- Companion: `conventions/knowledge-graph-grounded-apprenticeship.md`
- Companion: `conventions/mcp-substrate-protocol.md`
- Companion: `conventions/seed-taxonomy-as-smb-bootstrap.md`
- Companion: `conventions/customer-owned-graph-ip.md`
- Reference: existing `vendor/pointsav-monorepo/service-content/seeds/`
- Cluster scope: project-data Task
