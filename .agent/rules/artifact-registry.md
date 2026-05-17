---
schema: foundry-artifact-registry-v1
project: project-gis
last_updated: 2026-05-17
---

# project-gis Artifact Registry

Persistent record of all editorial, design, and data artifacts being built
for this project. Updated as artifacts are staged, dispatched, or completed.
Companion to `tasks.md` (sprint history) and `outstanding-todo.md` (backlog).

Routing:
- TOPIC / GUIDE / TEXT / PROSE → `project-editorial`
- DESIGN-RESEARCH / DESIGN-TOKEN / COMPONENT → `project-design`
- DATA artifacts remain in this archive

---

## A — Active / In-Progress

### A1 — TOPIC: O-D Catchment Methodology
- **File:** `topic-od-catchment-methodology.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial
- **Destination:** project-editorial
- **Content:** Crow-flies O-D model; 35/150km ring rationale; H3 res-7; provisional language; HOME vs AWAY distinction

### A2 — TOPIC: Trade Area Data Sources
- **File:** `topic-trade-area-data-sources.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial
- **Destination:** project-editorial
- **Content:** WorldPop 2026 100m raster → H3 res-7 aggregation; 13 countries; per-capita spend multipliers; data vintage; BLS/StatCan/Eurostat proxies

### A3 — TOPIC: Catchment Ranking Methodology
- **File:** `topic-catchment-ranking-methodology.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial
- **Destination:** project-editorial
- **Content:** Combined primary+secondary rank dimensions; no-weights rationale; future weighting roadmap

### A4 — TEXT: Data Methodology Dialog
- **File:** `text-gis-data-methodology-dialog.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial
- **Content:** Copy for the "Data" button modal on gis.woodfinegroup.com; all data source attributions; methodology notes; link to DATA-MANIFEST.md

### A5 — GUIDE: Pipeline Rebuild (Phase 1/2 appended)
- **File:** `guide-gis-pipeline-rebuild.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial; Phase 1/2 sections appended
- **Destination:** project-editorial
- **Content:** Full rebuild procedure including Phase 1 taxonomy rebuild steps and Phase 2 build-geometric-ranking.py future pipeline

---

## B — Backlog (queued for future sessions)

### B1 — TOPIC: Co-location Ranking System (full update)
- **Status:** BACKLOG — existing draft dispatched; needs update for catchment rank fields
- **Destination:** project-editorial

### B2 — TOPIC: POI Data Schema
- **Status:** DISPATCHED sprint 13 (ba5fe38) — at project-editorial

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

| Artifact | File | Status |
|---|---|---|
| O-D Summary (B3) | `service-fs/service-mobility/od-summary.jsonl` | DONE (2026-05-15) |
| Catchment Data | `work/catchment-data.json` | DONE (2026-05-15) |
| Catchment Polygons | `work/catchment-polygons.geojson` → `layer3-catchment.pmtiles` (87 MB) | DONE (2026-05-15) |
| Census Catchment | `work/census-catchment.geojson` → `layer4-census.pmtiles` (373 MB) | DONE (2026-05-15) |
| Spend Catchment | `work/spend-catchment.geojson` → `layer5-spend.pmtiles` (635 MB) | DONE (2026-05-15) |
| DATA-MANIFEST.md | project root | DONE (2026-05-12) |
| clusters-meta.json | gateway www/data/ | DONE (2026-05-17; 13,164 clusters; T1=432/T2=1,594/T3=3,091/T4=8,047; P10=0.25+GateB Phase 14; full catchment ranks; US:279 ES:51 MX:34 CA:32 DE:14 FR:13 GB:9) |
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

### Phase 12 Chain Ingests (2026-05-17)

| Artifact | File | Status |
|---|---|---|
| ASDA-UK JSONL | `service-business/asda-uk.jsonl` | DONE (2026-05-17; 1,051 records; Q297410; 3b367a9f) |
| Morrisons-UK JSONL | `service-business/morrisons-uk.jsonl` | DONE (2026-05-17; 620 records; Q922344; 3b367a9f) |
| H-E-B JSONL | `service-business/heb-us.jsonl` | DONE (2026-05-17; 301 records; Q1665088; name_query fallback; 3b367a9f) |
| whole-foods-us ALPHA promo | config.py | DONE (2026-05-17; promoted from GENERIC_FOOD; 528 records pre-existing; 3b367a9f) |
| chedraui-mx ALPHA promo | config.py | DONE (2026-05-17; promoted from generic; 249 records pre-existing; 3b367a9f) |

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
