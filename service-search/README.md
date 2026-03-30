# ⚙️ SERVICE-SEARCH: SOVEREIGN INVERTED INDEX
**Vendor:** PointSav Digital Systems™
**Standard:** Leapfrog 2050 (3D Asset Tokens & Asymmetric Storage)
**Tier:** 5 (Service Logic)

---

## I. ARCHITECTURAL MANDATE
To maintain DARP compliance, the Totebox Archive cannot rely on centralized, memory-heavy databases (like Elasticsearch) for data retrieval. `service-search` utilizes the Tantivy Rust library to forge a static, binary **Inverted Index**.

## II. THE DUAL-STATE ARCHITECTURE
This component permanently separates physical storage from lightning-fast retrieval.

1. **The Forge (Indexing):** A point-in-time Rust binary that scans the `/assets` (Markdown) and `/ledger` (YAML) directories upon ingestion. It maps every word to an exact byte-coordinate in a highly compressed `/search-index/` binary file, then terminates to release RAM.
2. **The Strike (Querying):** A lightweight, read-only interface that queries the binary index, returning file paths (e.g., `service-minutebook/ledger/resolution_01.yaml`) in microseconds without ever opening the original documents.
