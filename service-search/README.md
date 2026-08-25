# ⚙️ SERVICE-SEARCH: SOVEREIGN INVERTED INDEX
**Vendor:** PointSav Digital Systems™
**Standard:** Leapfrog 2050 (Totebox Archives & Asymmetric Storage)
**Tier:** 5 (Service Logic)

---

## I. ARCHITECTURAL MANDATE
To maintain DARP compliance, the Totebox Archive cannot rely on centralized, memory-heavy databases (like Elasticsearch) for data retrieval. `service-search` is the **one** search service for the monorepo — other `service-*` and the workbench log into it over HTTP rather than each embedding their own index.

## II. THE DUAL-STATE ARCHITECTURE
This component permanently separates physical storage from lightning-fast retrieval.

1. **The Forge (`forge` binary):** A point-in-time Rust binary that scans the configured roots, builds the indexes, writes them to `index_path`, then **terminates to release RAM**. Peak indexing memory is paid by a short-lived process, never by the running server.
2. **The Strike (`strike` binary):** A lightweight, read-only HTTP service. It loads the persisted indexes (mmaps Tantivy; reads the small trigram sidecar) and answers `GET /search?q=…` in milliseconds at a few MB of RSS, without ever re-opening the original documents.

## III. THE ENGINE — one search, three parts, no overlap
Per `BRIEF-workplace-comprehensive-search` (operator-ratified 2026-07-16):

| Part | Role | Owned? |
|---|---|---|
| **`moonshot-index`** (`TrigramIndex`) | Trigram substring **correctness floor** — the no-silent-miss guarantee a token index structurally cannot make. | **Owned** |
| **Tantivy** (`vendor-tantivy`, MIT) | **BM25 ranking** + on-disk persistence + the stored body used for snippets and content verification. | Vendored |
| **`service-search`** (this crate) | **Fuses** them: ranked hits ∪ guaranteed hits, two bands (filenames / contents), never dropping a trigram-verified hit for the sake of ranking. | Owned |

**Memory model:** the Strike holds only the trigram postings + filenames (small) and mmaps Tantivy. The content copy lives once, in Tantivy's stored `body` field; trigram content candidates are verified by reading that field back — never from a RAM copy, never from a second filesystem pass.

## IV. USAGE
```bash
# 1. Forge the index (run on demand or by timer); exits when done.
forge config.toml

# 2. Serve queries read-only.
strike config.toml
#    GET /search?q=<query>  → JSON { filenames[], contents[], files_indexed, roots_indexed }
#    GET /healthz           → "ok"
```
See `config.example.toml`. The `_command` → `/srv/foundry` root (83 GB) is deliberately excluded — index real project roots only.

## V. STATUS
v1 (2026-07-16): Forge + Strike + fusion working end-to-end; two bands + coverage line. Later phases (per the BRIEF): `gix` git-history rail, scope chips, index-health dot, incremental reindex, opt-in semantic rerank. **OS integration is an `os-totebox` handoff** — `service-search` runs once inside an `os-*` bundle (the Doorman precedent); wiring it there is that archive's scope.
