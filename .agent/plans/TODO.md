# project-gis — Master TODO
> **Last updated:** 2026-05-20
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

## PENDING — Open decisions

### D1 — Path C tier composition (T1 qualification)
- **Decision:** Add HW∧HM (hypermarket anchor AND high mobility) as a T1-qualifying path?
- **Impact:** ~+199 US T1 clusters; brings US T1 to ~475
- **Trade-off:** Makes T1 less purely "regional draw" and more "high mobility regardless of format"
- **Owner:** Jennifer (operator decision)

---

---

## READY — Next sprint work

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
| Phase 16: ingest-kontur.py written + validated | session (2026-05-19) — **NOT YET COMMITTED** |
| Phase 16: od-study rerun with Kontur (1,928,815 cells, T1=443) | session (2026-05-19) |
| AEC research: weather/regulatory/infrastructure — 3 plan files | session (2026-05-20) |
