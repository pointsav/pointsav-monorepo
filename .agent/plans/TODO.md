# project-gis — Master TODO
> **Last updated:** 2026-05-21 (session 2)
> This is the canonical session-to-session work queue. Update when items are started, completed, or reprioritized.

---

## STANDING RULE — Large builds run overnight only

**Server is UTC. Vancouver PDT (May–Nov) = UTC-7.**

| Window | Vancouver | UTC |
|---|---|---|
| Start after | 10:00pm PDT | 05:00 UTC |
| Finish before | 9:00am PDT | 16:00 UTC |

Schedule with: `echo "cd <dir> && python3 <script> > /tmp/<log>.log 2>&1" | at 05:00`

**Scripts that must follow this rule:** `build-tiles.py`, `build-mobility-tiles.py`, `build-data-tiles.py`, `synthesize-od-study.py`, `ingest-lodes.py`, `build-catchment-polygons.py`, `ingest-kontur.py`

---

## BLOCKED — Waiting on operator manual download

### O1 — UK ONS ODWP01EW commute flows
- **Source:** https://www.nomisweb.co.uk/datasets/wu03ew (MSOA-level, ~77 MB)
- **Save to:** `/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-mobility/`
- **When done:** Write ingest script + run mobility tile rebuild

### O2 — France INSEE FD_MOBPRO21 commute flows
- **Source:** https://www.insee.fr/ — FD_MOBPRO21 (commune-level)
- **Save to:** `/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-mobility/`
- **When done:** Write ingest script + run mobility tile rebuild

### O3 — Germany BA Pendler commute flows
- **Source:** Bundesagentur für Arbeit — Pendler XLSX (Kreis-level)
- **Save to:** `/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-mobility/`
- **When done:** Write ingest script + run mobility tile rebuild

---

## RESUME HERE — Next session start

### R1 — Bubble/ring overlap bug (unresolved as of 2026-05-21 end)
- **Symptom:** At z≥9 (retail level), proximity rings appear but cluster bubble nodes do NOT disappear
- **Confirmed:** Server sends `no-cache` headers; browser is getting fresh file
- **Latest fix deployed:** `setRetailLevel()` now sets both `visibility:'none'` AND `circle-opacity:0` on nodes/nodes-halo before showing rings
- **Next step:** User tests after hard-refresh (Ctrl+Shift+R). If still broken, open F12 → Console and check for JS errors during zoom-in
- **Code location:** `setRetailLevel()` ~line 1149; `showOverview()` ~line 1076; `drillIntoCluster()` ~line 2285

---

## PENDING — Open decisions (operator sign-off required)

### D1 — Path C tier composition (T1 qualification)
- **Decision:** Add HW∧HM (hypermarket anchor AND high mobility) as a T1-qualifying path?
- **Impact:** ~+199 US T1 clusters; brings US T1 to ~475
- **Trade-off:** Makes T1 less purely "regional draw" and more "high mobility regardless of format"
- **Owner:** Jennifer (operator decision)
- **Plan:** `DBSCAN-TRIANGULATION-REDESIGN-2026-05-20.md` §11 (OD3)

### D2 — Vignette opacity final value
- **Decision:** 0.20 / 0.22 / 0.25 — validate on Alberta sim first
- **Plan:** `VISUAL-DESIGN-SYSTEM-2026-05-20.md` §16 (OD-V1)

### D3 — 35km ring solid vs. dashed
- **Decision:** Solid = "real trade area" / dashed = "estimated"
- **Note:** MITMA-measured variant already uses solid; generalise or keep dashed for non-MITMA?
- **Plan:** `RING-HIERARCHY-DESIGN-2026-05-20.md` §11 (OD-R2)

### D4 — Data horizon arc label copy
- **Decision:** `Data horizon — 150 km` vs `Regional boundary` vs nothing
- **Plan:** `RING-HIERARCHY-DESIGN-2026-05-20.md` §11 (OD-R3)

### D5 — Retire `layer3-radius.pmtiles` timing
- **Decision:** Same commit as ring redesign vs. separate cleanup commit
- **Risk:** Live regression if retired before `proximity-circle-src` confirmed stable
- **Plan:** `RING-HIERARCHY-DESIGN-2026-05-20.md` §11 (OD-R1)

---

---

## READY — Next sprint work

### P17-V — Phase 17: Visual / DBSCAN redesign (Alberta sim → production)
Research complete (three plan files committed 4cfd19f5). Implementation sequence per `RING-HIERARCHY-DESIGN-2026-05-20.md` §12 and `VISUAL-DESIGN-SYSTEM-2026-05-20.md` §17:

| Step | Task | Prereq | Effort |
|---|---|---|---|
| V1 | **Alberta sim — three-ring hierarchy** | — | ~40 lines JS; `RING-HIERARCHY-DESIGN` §10 |
| V2 | **Alberta sim — retailer-first visual** (hull + dots + centroid glyph) | members_detail now in GeoJSON ✓ | ~60 lines JS |
| V3 | **Palette conflict fixes** (live map dot colors C1–C4) | Operator colour approval | `VISUAL-DESIGN-SYSTEM` §1–§4 |
| V4 | **Ring semantics** — `ringKmForTier(tier)`; retire `currentRadius`; remove `.radius-selector` buttons | Operator ring approval | `RING-HIERARCHY-DESIGN` §6 |
| V5 | **Ring weight stratification** — 2px/1.5px/1px hierarchy | — | Paint property change only |
| V6 | **150km ring repaint** — remove blur; `#64748B` slate; `[6,4]` dash; 1px/0.5 opacity | — | `RING-HIERARCHY-DESIGN` §4 |
| V7 | **Vignette mask** — `data-horizon-mask` layer; `makeMaskGeoJSON()`; 0.22 opacity | D2 sign-off | `RING-HIERARCHY-DESIGN` §4; `VISUAL-DESIGN-SYSTEM` §9 |
| V8 | **Data horizon arc label** — italic annotation at 150km north point | D4 sign-off | `RING-HIERARCHY-DESIGN` §4 |
| V9 | **Transition model** — stagger S3 reveal; 150ms selection eases | — | `VISUAL-DESIGN-SYSTEM` §7 |
| V10 | **BentoBox upgrade** — tabular numerics; type scale; state-bound legend | — | `VISUAL-DESIGN-SYSTEM` §11–§13 |
| V11 | **Button/toggle rename** — `Show trade area`; persistent pill; `catchmentActive` → `dataLayerActive` | — | `RING-HIERARCHY-DESIGN` §7 |
| V12 | **DBSCAN build-clusters.py rewrite** | Alberta sim operator sign-off | `DBSCAN-TRIANGULATION-REDESIGN-2026-05-20.md` |
| V13 | **clusters-meta.json schema** — add `members[]`, `mc`, `tight`, `span_m` fields | V12 | After DBSCAN rewrite |
| V14 | **Mobility per-cluster pre-clip** — `build-mobility-tiles.py` per-cluster files | Overnight build; D5 | `RING-HIERARCHY-DESIGN` §8 |
| V15 | **Retire `layer3-radius.pmtiles`** — remove `radius-fill`/`radius-line` layers | V4 stable | D5 sign-off |

**MVP polish (do these first, cheapest leverage):** V5 (ring weights), V9 (transitions), V10 tabular numerics only.

### P17-A — Phase 17: AEC Tier 1 layers (regulatory + environmental)
Priority order based on effort/value ratio. All US-only, all public domain, all PMTiles-viable.

| # | Layer | Source | Est. effort | Plan file |
|---|---|---|---|---|
| 1 | EPA Radon Zones (county polygons) | EPA ArcGIS REST → TIGER counties join | 1 day | AEC-REGULATORY-LAYERS-RESEARCH.md §8 |
| 2 | NPS Historic Register (points + polygons) | NPS ArcGIS Hub GeoJSON export | 1 day | AEC-REGULATORY-LAYERS-RESEARCH.md §4a |
| 3 | CAL FIRE FHSZ (CA fire severity zones) | CA Geoportal GeoJSON | 0.5 day | AEC-REGULATORY-LAYERS-RESEARCH.md §5b |
| 4 | HIFLD Transmission Lines + Substations + Power Plants | HIFLD Hub PMTiles export | 0.5 day | AEC-INFRASTRUCTURE-LAYERS-RESEARCH.md §1 |
| 5 | EIA Electric Retail Service Territories (utility boundaries) | EIA Energy Atlas GeoJSON | 0.5 day | AEC-INFRASTRUCTURE-LAYERS-RESEARCH.md §1.4 |
| 6 | Overture Maps Buildings PMTiles | Public S3 URL — zero preprocessing | 0.5 day | AEC-INFRASTRUCTURE-LAYERS-RESEARCH.md §8.2 |
| 7 | HUD Opportunity Zones | HUD Hub GeoJSON → PMTiles | 0.5 day | AEC-INFRASTRUCTURE-LAYERS-RESEARCH.md §6.2 |
| 8 | CDC SVI 2022 (social vulnerability index) | CDC direct shapefile → PMTiles | 1 day | AEC-INFRASTRUCTURE-LAYERS-RESEARCH.md §6.1 |
| 9 | EPA EJSCREEN v2.3 (environmental justice) | EPA FTP GDB → PMTiles | 1 day | AEC-INFRASTRUCTURE-LAYERS-RESEARCH.md §5.8 |
| 10 | FEMA NFHL Flood Zones | FEMA REST / state SHPs → PMTiles | 3 days | AEC-REGULATORY-LAYERS-RESEARCH.md §3a |
| 11 | USFS WUI 2020 (wildland-urban interface) | USDA RDS GDB → PMTiles | 1 day | AEC-REGULATORY-LAYERS-RESEARCH.md §5a |
| 12 | USFWS NWI Wetlands | USFWS state GeoPackages → PMTiles | 2 days | AEC-REGULATORY-LAYERS-RESEARCH.md §3b |
| 13 | USGS Seismic Design Maps | Click-to-query popup (REST API, no PMTiles) | 1 day | AEC-REGULATORY-LAYERS-RESEARCH.md §9a |

### P17-B — Phase 17: AEC weather fields (clusters-meta.json enrichment)
All delivered as new fields in clusters-meta.json, not tile layers. Run `synthesize-od-study.py`-style batch script.

| # | Field | Source | Est. effort | Plan file |
|---|---|---|---|---|
| 1 | HDD / CDD | NOAA NCEI 1991–2020 Normals CSV | 0.5 day | AEC-WEATHER-LAYERS-RESEARCH.md §1 |
| 2 | ASHRAE 99% / 1% design temps | DOE EPW files + ladybug | 1 day | AEC-WEATHER-LAYERS-RESEARCH.md §2 |
| 3 | 100-yr / 10-yr 24-hr precipitation | NOAA PFDS REST API (per-cluster) | 1 day | AEC-WEATHER-LAYERS-RESEARCH.md §3 |
| 4 | Tornado risk F1+ annual probability | NOAA SPC GeoTIFF raster sample | 0.5 day | AEC-WEATHER-LAYERS-RESEARCH.md §7 |
| 5 | Hail events per decade | NOAA SPC Storm Events CSV → KDE | 1 day | AEC-WEATHER-LAYERS-RESEARCH.md §6 |
| 6 | Design frost depth | NRCS SDA API (SSURGO) | 1 day | AEC-WEATHER-LAYERS-RESEARCH.md §4 |
| 7 | Prevailing wind direction + mean speed | ERA5 U10/V10 (requires CDS API registration) | 1 day | AEC-WEATHER-LAYERS-RESEARCH.md §5 |
| 8 | Hurricane track density | NOAA IBTrACS AT+EP basins → KDE | 1 day | AEC-WEATHER-LAYERS-RESEARCH.md §8 |
| 9 | Corrosivity proxy (coastal distance) | NOAA GSHHS coastline → proximity raster | 0.5 day | AEC-WEATHER-LAYERS-RESEARCH.md §9 |

**Prerequisite for ERA5:** Register at https://climate.copernicus.eu — CDS API registration is a manual step.

### P17-C — National Zoning Atlas (completed states only)
- Download available state GeoJSONs from https://statezoningatlasdata.s3.amazonaws.com/index.html
- Run tippecanoe → PMTiles for each completed state
- Note: ~15% of US jurisdictions only; supplement with Regrid quote (commercial)

### P17-D — Regrid Standardized Zoning licensing evaluation
- Contact: https://regrid.com/aec — request enterprise pricing
- Covers 6,000+ municipalities, 127.9M parcels; FAR, setbacks, permitted uses
- Decision needed before any zoning tile work beyond NZA

---

## BACKLOG — Future phases

### Phase 18 — Canada + Mexico + EU weather extension
- ERA5 global for HDD/CDD (CA/MX/EU)
- ECCC IDF curves (CA precipitation frequency)
- ECA&D precipitation frequency (EU)
- Kontur → census-h3-res7.jsonl already covers all 13 countries

### Phase 18 — Additional AEC Tier 2 layers
- EPA eGRID carbon intensity by subregion
- EPA CWNS 2022 Sewersheds
- FCC Broadband H3 aggregated PMTiles
- USDOT ETC Explorer (equity index)
- USGS 3DEP 10m DEM hillshade (TiTiler)
- WashU SatPM2.5 raster (TiTiler)
- GTFS transit stops + routes (top-400 US agencies)
- USDA Food Access Research Atlas
- EPA CWNS Sewersheds

### UK/FR/DE mobility (pending operator downloads O1–O3)
- UK ONS ODWP01EW → write `ingest-ons-odwp01ew.py`
- France INSEE FD_MOBPRO21 → write `ingest-insee-mobpro.py`
- Germany BA Pendler → write `ingest-ba-pendler.py`
- Rebuild layer6/layer7 mobility PMTiles after each ingest

### Chain ingests — queued candidates
- ALDI US (large potential; OSM coverage may be thin)
- Trader Joe's US (already in config? verify coverage)
- Marks & Spencer UK food halls
- Carrefour FR (verify vs. existing Carrefour)

---

## DONE THIS PHASE (archive reference)

| Item | Commit / date |
|---|---|
| Phase 15: wegmans/winco/sprouts ingest | c5662554 (2026-05-18) |
| Phase 16: esselunga/sklavenitis/billa-plus/continente/albert-heijn-xl ingest | session (2026-05-19) |
| Phase 16: config.py EU anchors update | session (2026-05-19) |
| Phase 16: layer3-catchment.pmtiles rebuild (1.7GB → 30MB) | session (2026-05-19) |
| Phase 16: Kontur Population migration (WorldPop deleted, disk 32G→7G) | session (2026-05-19) |
| Phase 16: ingest-kontur.py written + validated | 49af6829 (2026-05-20) |
| Phase 16: od-study rerun with Kontur (1,928,815 cells, T1=443) | session (2026-05-19) |
| AEC research: weather/regulatory/infrastructure — 3 plan files | 6a600f81 (2026-05-20) |
| DBSCAN redesign research → DBSCAN-TRIANGULATION-REDESIGN-2026-05-20.md | 4cfd19f5 (2026-05-21) |
| Ring hierarchy research → RING-HIERARCHY-DESIGN-2026-05-20.md | 4cfd19f5 (2026-05-21) |
| Visual design system research → VISUAL-DESIGN-SYSTEM-2026-05-20.md | 4cfd19f5 (2026-05-21) |
| Alberta sim delta palette fix (slate/green/rose/violet) | 4cfd19f5 (2026-05-21) |
| sim-ab members_detail bug fixed; GeoJSON regenerated | 4cfd19f5 (2026-05-21) |
| GIS UX redesign research (5 agents) → GIS-UX-REDESIGN-2026-05-21.md | session (2026-05-21) |
| TIER_COLORS migrated to Woodfine tokens (#164679/#54924E/#EAB308/#991B1B) | session (2026-05-21) |
| BentoBox badge 42px→13px compact chip; ranking → Top 400 NA/EU format | session (2026-05-21) |
| Sim popup removed; sim clicks route to #inspector panel | session (2026-05-21) |
| Zoom transition: removed maxzoom:9 (clean zoomend swap); opacity+visibility dual-kill on nodes | session (2026-05-21) |
