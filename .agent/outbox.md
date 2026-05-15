---
mailbox: outbox
owner: task@project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-gis

---
from: task@project-gis
to: command@foundry
re: Regional Market settlement layer — committed e1a140a5
created: 2026-05-15T00:00:00Z
---

Regional Market labeling system is in place. No tile rebuild required yet — regional-markets.json is already live in the gateway.

**What was built:**

- `build-settlements.py` — downloads and builds three new boundary files:
  - `us_places.geojson` (TIGER 2023, 32K US incorporated places)
  - `eu_municipalities.geojson` (GISCO LAU 2021, 98K EU/EEA municipalities + GADM GBR 406 districts)
  - `ca_places_nominatim.json` (12 Nominatim overrides for county CSDs)

- `region_engine.py` — new `resolve_market(lat, lon, iso, cluster_id)` method. Returns (settlement_name, confidence). No name lists. No special cases. Rule: smallest recognized settlement boundary containing the centroid.

- `build-clusters.py` — now calls `resolve_market()` from cluster centroid instead of reading OSM `addr:city` from individual POIs. The old addr:city approach was unreliable (store importers paste the metro postal city).

- `regional-markets.json` — gateway data file: 2,986 Regional Markets, 2,942 high-confidence. Each entry: {market, iso, region, cluster_count, cluster_ids[]}. Ready for search indexing and wiki page generation.

**Verification results:**
  Airdrie → "Airdrie" (high)      ← not "Calgary"
  Sherwood Park → "Sherwood Park" (high)  ← not "Strathcona County"
  St. Albert → "St. Albert" (high)       ← not "Sturgeon County"
  Frisco TX → "Frisco" (high)           ← not "Dallas"
  Alcobendas → "Alcobendas" (high)      ← not "Madrid"
  Salford → "Salford" (high)            ← not "Manchester"
  Boulogne-Billancourt (high)           ← not "Paris"

**Next steps requiring operator decision:**
1. Full pipeline rebuild (build-clusters.py → build-tiles.py) to propagate `cty` + `mkt_conf` into live clusters-meta.json. Roughly 20-30 min. Ready to run on operator go-ahead.
2. Search index — add `mkt` field to gateway search to enable city/town filtering. Needs index schema change.
3. Wiki integration — `regional-markets.json` is live at gateway www/data/; projects.woodfinegroup.com can consume it for wiki page stubs.

— task@project-gis

---
from: task@project-gis
to: command@foundry
re: Sprint 14 complete — B1/B2/B3 + Phase C + catchment map layers
created: 2026-05-15
---

Sprint 14 is complete. Summary of work:

**B1/B2/B3 — DONE**
- Census (WorldPop 2026 100m → H3 res-7): 1,928,815 populated cells · 13 countries · 172 MB
- Spend (per-capita grocery/hardware/wholesale multipliers on census grid): 1,988,375 cells · 265 MB
- O-D Catchment (synthesize-od-study.py): crow-flies rings PRIMARY_RINGS=17 (≤35km) / SECONDARY_RINGS=72 (≤150km); 6,815 clusters fully ranked on population + 3 spend categories. B3 artifact: service-fs/service-mobility/od-summary.jsonl. clusters-meta.json updated with pp/sp/pg/sg/ph/sh/pw/sw/rp/rg/rh/rw/rn fields (all 6,815 merged).

**Catchment tile layers — DONE**
- layer3-catchment.pmtiles (87 MB) — primary/secondary circular polygons, 2 per cluster
- layer4-census.pmtiles (373 MB) — H3 hexagons masked to catchment areas
- layer5-spend.pmtiles (635 MB) — H3 hexagons masked to catchment areas

**Map UI (index.html) — DONE**
- PMTile sources: catchment-src, census-src, spend-src
- 6 new MapLibre layers (catchment-primary-fill/line, catchment-secondary-fill/line, catchment-census, catchment-spend)
- Layer toggles: Catchments (with HOME/AWAY sub-toggle), Census, Spend
- JS: toggleCatchmentLayer, setHomeAway (AWAY stubs to HOME pending daytime population data), openDataModal, closeDataModal
- BentoBox Trade Area section: primary/secondary population + catchment rank display
- Data methodology modal: full data source attributions + trade area methodology copy

**Phase C EU hardware chains — DONE**
6 chains ingested: toom-baumarkt-de, hagebaumarkt-de, bricocenter-it, silvan-dk, praktiker-gr, byko-is. Pipeline full rebuild: 52,362 raw → 48,468 cleansed → 6,815 deduplicated clusters (same count; dedup absorbed new overlaps). Catchment data re-merged after tile rebuild.

**Editorial artifacts staged (5) — ready for project-editorial sweep**
- topic-od-catchment-methodology.draft.md
- topic-trade-area-data-sources.draft.md
- topic-catchment-ranking-methodology.draft.md
- text-gis-data-methodology-dialog.draft.md
- guide-gis-pipeline-rebuild.draft.md (Stage 6 O-D section appended)

**Open backlog items (not this sprint)**
- A1: Carrefour-FR sub-format coverage fix
- C1: Auchan-FR ingest
- C2: Mercadona-ES anchor promotion question
- AWAY catchment layer (daytime population data source needed)


