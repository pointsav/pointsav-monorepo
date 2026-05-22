# service-content Architecture Analysis — 2026-05-14

> Authored: 2026-05-14 Opus agent via task@project-intelligence
> Status: Active — drives Sprint 1 implementation decisions

---

## TL;DR

The current `main.rs` is the **legacy watcher** that `service-content/ARCHITECTURE.md` explicitly calls deprecated and scheduled for deletion. It is the production binary. The rebuild plan in the same document describes an inverted (PUSH) architecture that would fix the Ring 2/Ring 3 coupling problem — but the code hasn't caught up to its own architecture document.

**Most critical defect:** Ring 2 ingest completely halts when Ring 3 (Doorman) is unavailable. The three-ring "Ring 2 functional without AI" principle is aspirational, not real. Fix takes ~30 LOC.

---

## 1. Ring 2 / Ring 3 Coupling — The Core Problem

Current flow (PULL — wrong):
```
service-content → POST /v1/extract → Doorman → Tier B
                                              ← {entities[]}
  → upsert_entities(graph)
  → write SEMANTIC_*.json
```

When Ring 3 is unavailable: Ring 2 writes NOTHING. The graph does not grow. The Community Tier (Rings 1+2, AGPL, no AI) is broken by construction.

### Sprint 1 fix (30 LOC) — deterministic pass

Before calling Doorman, write a `Source` node to the graph:
```rust
graph_store.upsert_entities(module_id, &[GraphEntity {
    entity_name: worm_id.to_string(),
    classification: "Source".to_string(),
    module_id: effective_module_id.to_string(),
    confidence: 1.0,
    ...
}]);
```
Graph grows regardless of Ring 3. The entity gets enriched when Doorman eventually delivers extraction results via PUSH.

### Strategic migration — invert to PUSH (5 sprints)

| Sprint | Change | LOC delta |
|---|---|---|
| 1 | Deterministic Source node write before Doorman call | +30 |
| 2 | Schema: add `node_type`, `source_worm_id` to Entity; add `RelatedTo` writes | +150 |
| 3 | Doorman-side queue + PUSH to `/v1/graph/mutate`; delete PULL path from service-content | -120 / +80 in Doorman |
| 4 | Move `/v1/draft/generate` to Doorman; delete from service-content | -120 |
| 5 | Persistent `processed_ledgers` → graph query; 114-file migration script | -20 / +30 |

Net: service-content gets smaller; Doorman gains the queue that properly belongs to Ring 3.

---

## 2. Graph Store Defects

### RelatedTo table: declared, never populated (graph.rs:66-72)
The graph is a node-only store. No edges exist. Everything ARCHITECTURE.md §8 promises about linked nodes is unimplemented.

### Entity ID collision (graph.rs:100-104)
```rust
format!("{}__{}", module_id, entity.entity_name.to_lowercase().replace(' ', "_"))
```
- Hyphens, accents, double-spaces, punctuation bypass dedup
- No alias table, no canonicalisation

### `created_at` is mislabelled (graph.rs:117)
Overwrites on every upsert — tracks *last seen*, not *first seen*.

### No source provenance
No `worm_id` on the `Entity` schema. Cannot trace "what evidence supports this entity?"

### Module ID injection (graph.rs:100, http.rs:99-108, main.rs:167-170)
`__taxonomy__` can be overwritten by:
- A CORPUS file carrying `"module_id": "__taxonomy__"` (the per-file override at main.rs:167-170)
- Unauthenticated `POST /v1/graph/mutate` with any module_id

Fix: validate module_id against `[a-z0-9-]{1,64}`, reject `__` prefix.

---

## 3. HTTP Surface Defects

| Endpoint | Defect |
|---|---|
| `GET /healthz` | Always returns OK — never checks graph, Doorman, disk |
| `POST /v1/graph/mutate` | No auth, no module_id validation |
| `POST /v1/draft/generate` | Ring 2 generating text via Ring 3 — architectural violation; move to Doorman |
| `POST /v1/draft/generate` | Response extraction uses wrong JSON pointer (`/choices/0/message/content`); only fallback `content` path works |
| All CSV POST endpoints | `std::fs::read_to_string` blocks async runtime; should use `tokio::fs` or `spawn_blocking` |

---

## 4. Extraction Schema Gaps

Current: `[Person, Company, Project, Account, Location]` with 3 string vectors.

Missing:
- `Property / Asset / Unit` (currently collapsed into Location)
- `Document / Contract / Lease` (the corpus artifact itself should be a node)
- `Event / Transaction / Payment`
- **Relationships** — the schema asks for entities only; no edges between them
- Temporal qualification (effective-from / effective-to)
- Source provenance (worm_id linkage)
- Per-field confidence (single hardcoded 0.95 for all fields)

---

## 5. Concrete Defects — Prioritised

### Critical
1. `main.rs:293` — `fs::write(...).unwrap()` panics on disk-full/permission error
2. `main.rs:102` — `processed_ledgers` RAM-only Vec; resets on restart (114-file backlog active)
3. No module_id validation (taxonomy namespace injection risk)
4. `RelatedTo` table never populated — graph is node-only
5. Entity dedup via name collision (hyphen/accent/punctuation bypass)
6. Dual-write without transaction (`SEMANTIC_*.json` + LadybugDB can desync)
7. File modification after processing is silently ignored
8. 250ms fixed debounce: slow writers → JSON parse failure → file silently lost (file already marked processed at main.rs:137)
9. `unwrap()` on startup dir creation (main.rs:47, 48, 53)

### High
10. `/healthz` always OK (http.rs:84-86)
11. No structured logging / metrics — no request_id, no counters
12. Hardcoded default `base_dir` to customer deployment path (main.rs:24)
13. No file-size cap before `fs::read_to_string` (OOM on large file)
14. CSV POST endpoints: blocking I/O in async handlers

### Medium
15. `delete_by_classification` always returns `Ok(0)` — count not propagated
16. `/v1/draft/generate` uses wrong JSON pointer (only fallback works)
17. `module_id` per-file override allows arbitrary string injection

---

## 6. Minimum for Production Continuity

| # | Change | File:line |
|---|---|---|
| 1 | Persist `processed_ledgers` as disk-backed set | main.rs:102 |
| 2 | Replace `unwrap()` with error handling on startup + write paths | main.rs:47,48,53,293 |
| 3 | Real `/healthz` (graph ping with timeout) | http.rs:84-86 |
| 4 | Validate module_id; reject `__` prefix | http.rs:99-108, main.rs:167-170 |
| 5 | Cap CORPUS file size before read | main.rs:159 |
| 6 | Retry-with-backoff on JSON parse failure, OR atomic write from Ring 1 | main.rs:136-160 |
| 7 | Deterministic Source-node write before calling Doorman | main.rs:198 |
| 8 | Graceful shutdown on SIGTERM | main.rs:127-149 |
| 9 | HashSet instead of Vec for processed_ledgers | main.rs:102 |

---

## 7. What the Architecture Doc Says vs What Exists

Per `service-content/ARCHITECTURE.md`:
- Current `main.rs` is explicitly labelled **deprecated legacy** and "deleted" in the rebuild plan (§2, §13)
- `service-extraction` (deterministic parser, Ring 2, ADR-07) is documented but not in this binary
- MCP server with `temporal_query`, `pack_install`, etc. is documented but not present
- `POST /v1/graph/query` (Cypher subset), `/v1/graph/search` (kNN), `/v1/graph/export` — documented, not present
- "Doorman calls graph_mutate MCP tool" (§8) — the PUSH direction is already the documented intent

The rebuild plan is the north star. Sprint 1 (deterministic Source node) is the one change that makes the current legacy code survivable until the rebuild lands.
