---
> *Entries truncated for performance (683 lines → last 200 kept — see git history).*

## B — Backlog (queued for future sessions)

### B1 — TOPIC: Co-location Ranking System (full update)
- **Status:** SUPERSEDED / ARCHIVED 2026-05-31 — confirmed committed in media-knowledge-projects as `topic-co-location-ranking-system.md` + `.es.md`; source draft archived to `.agent/drafts-outbound/archived/`

### B2 — TOPIC: POI Data Schema
- **Status:** SUPERSEDED / ARCHIVED 2026-05-31 — confirmed committed in media-knowledge-documentation as `architecture/poi-data-schema.md` + `.es.md` (last_edited: 2026-05-25); source draft archived to `.agent/drafts-outbound/archived/`

### B3 — GUIDE: Adding a Chain
- **Status:** DISPATCHED sprint 11/13 — at project-editorial; appendix added

### B4 — GUIDE: Adding a Country
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B5 — TEXT: Canada/Walmart Supercentre + Hospital Coverage
- **Status:** STAGED in drafts-outbound/ (text-gis-canada-walmart-hospital-coverage.draft.md)

### B6 — DESIGN-RESEARCH: Bento Merged Zones Disclosure
- **Status:** IMPLEMENTED 21cf18df (2026-05-17) — merged-ring UX shipped in index.html (Union-Find groupOverlappingClusters, showMergedGroupPanel). Editorial draft still at project-design for write-up.

### B7 — DESIGN-RESEARCH: Location Intelligence UX
- **Status:** STAGED in drafts-outbound/ — at project-design

### B8 — DESIGN-RESEARCH: Ring Retailer Click UX
- **Status:** STAGED in drafts-outbound/ — at project-design

### B9 — DESIGN-RESEARCH: Tier Naming Accessibility
- **Status:** STAGED in drafts-outbound/ — at project-design

### B10 — DESIGN-RESEARCH: Zoom Prefetch Pattern
- **Status:** STAGED in drafts-outbound/ — at project-design

### B11 — TEXT: Nordic/UK Coverage Release
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B12 — TEXT: UK/EU Coverage Release
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B13 — TOPIC: Regional Name Resolution Architecture
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B14 — TOPIC: Co-location Tier Nomenclature
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B15 — TOPIC: GIS as BIM Substrate
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B16 — TOPIC: UK/EU Food Retail Coverage
- **Status:** STAGED in drafts-outbound/ — at project-editorial

---

## C — Data Artifacts (pipeline outputs, not editorial)

### Archetype models + scripts (2026-06-03; commit `aec2187e`)

| Artifact | File | Status |
|---|---|---|
| archetype-vwh.geojson (Urban Fringe) | `work/` → gateway www/data/ | DONE (2026-06-03; **7,028 features**; Retail-density model; `qualify_vwh` admits ≥2-cat OR lone STRONG/BROAD; composition-score tier T1=747/T2=2,732/T3=3,549; served at `?v=20260603d`) |
| archetype-pks.geojson (Commuter) | `work/` → gateway www/data/ | DONE (2026-06-03; **5,977 features**; geometric airport-led park-and-ride; regional airports + outer rail belt; NA map-cell coverage 96→957; tier T1=1,317/T2=3,183/T3=1,477; served at `?v=20260603d`) |
| sim_spread.py (harness) | `app-orchestration-gis/tools/` | DONE (2026-06-03; clusters once + evaluates qualify/tier rules instantly; used to tune both archetypes) |
| ingest-osm-parking.py | `app-orchestration-gis/` | DONE — scaffold (2026-06-03; park_ride + structured parking → `cleansed-civic-parking.jsonl`; runs June 4 overnight; BUILT/PARTIAL/GREENFIELD filter) |
| ingest-osm-parcel-depot.py | `app-orchestration-gis/` | DONE — scaffold (2026-06-03; `post_depot`+`office=logistics` → `parcel-depot-osm.jsonl`; VWH enrichment; runs June 4) |
| run-overnight-ingests.sh | `app-orchestration-gis/` | DONE (2026-06-03; crontab June 4 05:00 UTC; parking + parcel depots + 20 new VWH brand chains) |

### Earlier data artifacts

| Artifact | File | Status |
|---|---|---|
| O-D Summary (B3) | `service-fs/service-mobility/od-summary.jsonl` | DONE (2026-05-15) |
| Catchment Data | `work/catchment-data.json` | DONE (2026-05-15) |
| Catchment Polygons | `work/catchment-polygons.geojson` → `layer3-catchment.pmtiles` (87 MB) | DONE (2026-05-15) |
| Census Catchment | `work/census-catchment.geojson` → `layer4-census.pmtiles` (373 MB) | DONE (2026-05-15) |
| Spend Catchment | `work/spend-catchment.geojson` → `layer5-spend.pmtiles` (635 MB) | DONE (2026-05-15) |
| DATA-MANIFEST.md | project root | DONE (2026-05-12) |
| clusters-meta.json | gateway www/data/ | DONE (2026-05-22; 5,702 clusters; T1=1,157/T2=4,283/T3=262; Phase 18; 570bda53) |
| regional-markets.json | gateway www/data/ | DONE (2026-05-15; 2,986 Regional Markets, 2,942 high-conf) |
| us_places.geojson | deployments/boundaries/ | DONE (2026-05-15; TIGER 2023, 32K US places) |
| eu_municipalities.geojson | deployments/boundaries/ | DONE (2026-05-15; GISCO LAU 2021 + GADM GBR, 98.6K entries) |
| ca_places_nominatim.json | deployments/boundaries/ | DONE (2026-05-15; 12 county-CSD overrides, e.g. Sherwood Park) |

---

### Phase 13 Re-Ingests + Mobility Update (2026-05-17)

| Artifact | File | Status |
|---|---|---|
| tesco-uk re-ingest | `service-business/tesco-uk.jsonl` | DONE (2026-05-17; 784→3,872 records; name_query partial; Phase 13) |
| sainsburys-uk re-ingest | `service-business/sainsburys-uk.jsonl` | DONE (2026-05-17; 672→1,903 records; name_query partial; Phase 13) |
| tiendas-3b-mx re-ingest | `service-business/tiendas-3b-mx.jsonl` | DONE (2026-05-17; 151→247 records; name_query Tiendas 3B; Phase 13) |
| MITMA ES mobility_source | clusters-meta.json (58 ES clusters) | DONE (2026-05-17; build-mobility-tiles.py; Phase 13) |

---

### Phase 15 Chain Ingests (2026-05-18)

| Artifact | File | Status |
|---|---|---|
| wegmans-us JSONL | `service-business/wegmans-us.jsonl` | DONE (2026-05-18; 114 records; Q1182328; name_query; 4952dfaf) |
| winco-foods-us JSONL | `service-business/winco-foods-us.jsonl` | DONE (2026-05-18; 145 records; Q2584339; name_query; 4952dfaf) |
| sprouts-us JSONL | `service-business/sprouts-us.jsonl` | DONE (2026-05-18; 450 records; Q7580917; name_query; 4952dfaf) |
| build-tiles CHAIN_FAMILY fix | `build-tiles.py` | DONE (2026-05-19; wegmans/winco/sprouts/whole-foods/chedraui/asda/morrisons/heb brand_family fixed) |
| layer2-clusters.pmtiles rebuild | gateway tiles/ | DONE (2026-05-19; 13,657 clusters; 76.7 MB) |

---

### Phase 12 Chain Ingests (2026-05-17)

| Artifact | File | Status |
|---|---|---|
| ASDA-UK JSONL | `service-business/asda-uk.jsonl` | DONE (2026-05-17; 1,051 records; Q297410; 3b367a9f) |
| Morrisons-UK JSONL | `service-business/morrisons-uk.jsonl` | DONE (2026-05-17; 620 records; Q922344; 3b367a9f) |
| H-E-B JSONL | `service-business/heb-us.jsonl` | DONE (2026-05-17; 301 records; Q1665088; name_query fallback; 3b367a9f) |
| whole-foods-us ALPHA promo | config.py | DONE (2026-05-17; promoted from GENERIC_FOOD; 528 records pre-existing; 3b367a9f) |
| chedraui-mx ALPHA promo | config.py | DONE (2026-05-17; promoted from generic; 249 records pre-existing; 3b367a9f) |

---

### Phase 16 Chain Ingests + Infrastructure (2026-05-19)

| Artifact | File | Status |
|---|---|---|
| layer3-catchment.pmtiles rebuild | gateway tiles/ | DONE (2026-05-19; 30MB vs 1.7GB bad build; max-zoom 8, drop-densest, simplification 8) |
| Kontur Population downloads | deployments/cluster-totebox-personnel-1/service-fs/service-census/kontur-raw/ | DONE (2026-05-19; 13 countries; 523MB; CC BY 4.0) |
| esselunga-it JSONL | `service-business/esselunga-it.jsonl` | DONE (2026-05-19; 259 records; name_query fallback; Q1377048) |
| sklavenitis-gr JSONL | `service-business/sklavenitis-gr.jsonl` | DONE (2026-05-19; 406 records; Greek name_query Σκλαβενίτης; Q7536996) |
| billa-plus-at JSONL | `service-business/billa-plus-at.jsonl` | DONE (2026-05-19; 139 records; name_query fallback; Q806085) |
| continente-pt JSONL | `service-business/continente-pt.jsonl` | DONE (2026-05-19; 57 records; name_query fallback; Q5164541) |
| albert-heijn-xl-nl JSONL | `service-business/albert-heijn-xl-nl.jsonl` | DONE (2026-05-19; 43 records; name_query "Albert Heijn XL"; no wikidata) |
| config.py Phase 16 update | `config.py` | DONE (2026-05-19; 5 chains → ALPHA_HYPERMARKET EU + REGION_CONFIG anchors + ANCHOR_DISPLAY_NAMES) |
| US LODES full ingest | `service-fs/service-mobility/lodes-work-od-us.jsonl` | DONE (2026-05-20; 50 states/AK skipped; 684,334 H3 cells; 7,577 US clusters; 5.3GB) |
| layer6-mobility-work.pmtiles rebuild | gateway tiles/ | DONE (2026-05-20; 164MB; full US LODES + MITMA ES; 49af6829) |
| ingest-kontur.py | `app-orchestration-gis/ingest-kontur.py` | DONE (2026-05-20; committed 49af6829; sqlite3 stdlib, no fiona; 13 countries) |

---

### Phase 17 EU Taxonomy Audit + Rebuild (2026-05-22)

| Artifact | File | Status |
|---|---|---|
| taxonomy.py Phase 17 | `app-orchestration-gis/taxonomy.py` | DONE (2026-05-22; af434817; 12 zero-cost EU hypermarket chains activated) |
| config.py Phase 17 | `app-orchestration-gis/config.py` | DONE (2026-05-22; af434817; ALPHA_HYPERMARKET EU + REGION_CONFIG anchors sync'd) |
| generate-rm-topics.py | `app-orchestration-gis/generate-rm-topics.py` | DONE (2026-05-22; af434817; 225 lines; generates TOPIC drafts per Regional Market) |
| layer2-clusters.pmtiles Phase 17 | gateway tiles/ | DONE (2026-05-22; 37.7 MB; 5,273 clusters; two-pass DBSCAN §2 schema) |
| clusters-meta.json Phase 17 | gateway www/data/ | DONE (2026-05-22; 11 MB; T1=1,136/T2=3,865/T3=272; PL 0→17 T1; IT 3→8; ES 25→29; SE 8→4) |
| ikea-se.yaml / ikea-dk.yaml / ikea-no.yaml / ikea-fi.yaml | deployments/service-business/ | DONE (2026-05-22; per-country YAMLs replacing ikea-nordics multi_country; proper bbox+polygon filter) |
| ikea-se.jsonl | deployments/service-business/ | DONE (2026-05-22; 16 records; clean SE only; Lithuanian contamination removed) |
| ikea-dk.jsonl | deployments/service-business/ | DONE (2026-05-22; 5 records; clean DK only; 7 UK/SE ghost stores removed) |
| ikea-no.jsonl | deployments/service-business/ | DONE (2026-05-22; 5 records; Oslo/Trondheim/Slependen/Sørlandet/Åsane) |
| ikea-fi.jsonl | deployments/service-business/ | DONE (2026-05-22; 4 records; Espoo/Vantaa/Tampere/Kuopio) |
| layer2-clusters.pmtiles Nordic IKEA fix | gateway tiles/ | DONE (2026-05-22; 37.7 MB; 5,274 clusters; DK+NO+FI each gain first T1) |
| clusters-meta.json Nordic IKEA fix | gateway www/data/ | DONE (2026-05-22; 11 MB; T1=1,136/T2=3,866/T3=272; Odense DK T1; Oslo NO T1; Tampere FI T1) |

---

### Phase 18 Chain Ingests (2026-05-22)

| Artifact | File | Status |
|---|---|---|
| kaufland-pl JSONL | `service-business/kaufland-pl.jsonl` | DONE (2026-05-22; 253 records; Q685967; PL T1=17 unchanged — joins existing clusters) |
| foetex-dk JSONL | `service-business/foetex-dk.jsonl` | DONE (2026-05-22; 103 records; Q3093871 Salling Group) |
| wickes-uk JSONL | `service-business/wickes-uk.jsonl` | DONE (2026-05-22; 236 records; Q7998350 Travis Perkins hardware) |
| bauhaus-dk JSONL | `service-business/bauhaus-dk.jsonl` | DONE (2026-05-22; 20 records; Q532716) |
| bauhaus-no JSONL | `service-business/bauhaus-no.jsonl` | DONE (2026-05-22; 2 records; Q532716; OSM sparse in NO) |
| interspar-at JSONL | `service-business/interspar-at.jsonl` | DONE (2026-05-22; 85 records; Q1364056 SPAR Austria; Q1473279 rejected = Turmöl fuel) |
| jumbo-nl JSONL | `service-business/jumbo-nl.jsonl` | DONE (2026-05-22; 8 records; Q14716185 Jumbo Foodmarkt large-format) |
| leclerc-pl JSONL | `service-business/leclerc-pl.jsonl` | DONE (2026-05-22; 36 records; Q1273376) |
| bricomarch-fr JSONL | `service-business/bricomarch-fr.jsonl` | DONE (2026-05-22; 497 records; Q2896882 Les Mousquetaires hardware) |
| brico-depot-fr JSONL | `service-business/brico-depot-fr.jsonl` | DONE (2026-05-22; 137 records; Q3007003 Kingfisher hardware) |
| bauhaus-fi JSONL | `service-business/bauhaus-fi.jsonl` | DONE (2026-05-22; 6 records; Q532716) |
| globus-de JSONL | `service-business/globus-de.jsonl` | DONE (2026-05-22; 125 records; Q528681 Globus Holding) |
| geant-casino-fr JSONL | `service-business/geant-casino-fr.jsonl` | DONE (2026-05-22; 10 records; Q2901839 Casino Group) |
| intermarche-hyper-fr JSONL | `service-business/intermarche-hyper-fr.jsonl` | DONE (2026-05-22; 56 records; Q2029154 Les Mousquetaires) |
| taxonomy.py Phase 18 | `app-orchestration-gis/taxonomy.py` | DONE (2026-05-22; 570bda53; 14 chains added across PL/DK/GB/AT/NL/FR/DE/FI) |
| layer2-clusters.pmtiles Phase 18 | gateway tiles/ | DONE (2026-05-22; 40.8 MB; 5,702 clusters; T1=1,157/T2=4,283/T3=262) |
| clusters-meta.json Phase 18 | gateway www/data/ | DONE (2026-05-22; 570bda53; +17 T2 from FR hardware bricomarch+brico-depot; +4 T1/+19 T2 from globus-de) |

---

### Phase 11 Analysis Artifacts (2026-05-17)

| Artifact | File | Status |
|---|---|---|
| T1 threshold sweep (pre-IoU) | `work/sim-1a-results.txt` | DONE (2026-05-17; 5 thresholds; NA@P=0.20: 476 pre-IoU, 245 post-IoU) |
| T1 civic/composition sensitivity | `work/sim-1b-results.txt` | DONE (2026-05-17; civic B: NA=278@P=0.20; Path C adds 302 clusters) |
| T1 IoU + spatial coverage | `work/sim-1c-results.txt` | DONE (2026-05-17; IoU=0.10@P=0.20: NA=226 EU=57; 37 US states at 0) |
| Chain count audit | `work/chain-count-audit.txt` | DONE (2026-05-17; 91 OK, 35 OVER, 14 UNDER, 1 EMPTY) |
| Chain coverage audit | `work/chain-coverage-audit.md` | DONE (2026-05-17; gap candidates per country; see Section 2) |
| OD data research (UK/FR/DE) | `work/od-data-research-uk-fr-de.md` | DONE (2026-05-17; ONS ODWP01EW + INSEE FD_MOBPRO + BA Pendler all viable) |
| Kontur integration plan | `work/kontur-integration-plan.md` | DONE (2026-05-17; H3 res-8 available; CC BY 4.0; HDX download) |
| Storage report | `work/storage-report.md` | DONE (2026-05-17; root 65%; stale backups 35M removable) |
| SafeGraph export | `export-safegraph.py` | DONE adbb5d42 (2026-05-17; --sample 100 verified) |

---

## Maintenance

- When an artifact is dispatched to project-editorial/project-design, update status to DISPATCHED + commit hash
- When an artifact is returned (approved/rejected), update status
- Add new artifacts here at the time they are planned — do not wait until staging
