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


