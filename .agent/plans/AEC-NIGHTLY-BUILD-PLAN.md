---
plan: AEC-NIGHTLY-BUILD-PLAN
created: 2026-05-23
author: Jennifer Woodfine (project-gis Totebox)
status: ACTIVE — queued across 5 nights from 2026-05-24
companion: AEC-LAYERS-RESEARCH.md, AEC-INFRASTRUCTURE-LAYERS-RESEARCH.md
---

# AEC Layers — Staged Nightly Build Plan

Five-night rollout of AEC/site-conditions layers for gis.woodfinegroup.com.
Each night runs at 05:00 UTC (22:00 PDT) as a `nohup` background job.

**Canada note:** Canada is NOT Phase 18. Köppen-Geiger is global (covers CA, MX, ES
simultaneously with US). NSRDB solar covers CA in the same API calls. The
Canada-specific layer (NRCan NECB climate zones) is lightweight and runs Night 2
alongside ASHRAE. Canada gets full AEC coverage on Night 2+3 — not next phase.

---

## Night 1 — 05:00 UTC 2026-05-24 (already scheduled, PID 2507282)

**Scripts:** `nightly-rebuild.sh` → `phase19-rebuild.sh`

Expected output:
- T1=1,157 / T2=2,889 / T3=1,656 (geometric split applied)
- 16 sport chains ingested (Decathlon EU×12+CA, REI, Bass Pro, Cabela's)
- Cluster rebuild reflects: London fix + costco-uk cleanup + n≥4 T1 rule

**AEC:** None tonight.

---

## Night 2 — 05:00 UTC 2026-05-25

**Script:** `build-aec-climate-solar.sh` (create this session)

**What runs:**

| Step | Action | Output | Est. |
|---|---|---|---|
| 1 | Download PNNL ASHRAE 169-2013 county→zone CSV | `work/aec/pnnl-ashrae-county.csv` | 2 min |
| 2 | Download TIGER 2023 county polygons (GeoPackage) | `work/aec/tiger-counties-us.gpkg` | 5 min |
| 3 | Join PNNL table to TIGER → filter contiguous US → GeoJSON | `work/aec/ashrae-zones-us.geojson` | 3 min |
| 4 | tippecanoe → layer8-ashrae-zones-us.pmtiles | gateway tiles/ | 5 min |
| 5 | Download NRCan NECB climate zone shapefile | `work/aec/necb-zones-ca.shp` | 3 min |
| 6 | ogr2ogr → GeoJSON → tippecanoe → layer8-necb-zones-ca.pmtiles | gateway tiles/ | 5 min |
| 7 | NREL NSRDB API: sample GHI at all US + CA cluster centroids | `work/aec/ghi-us-ca.json` | 60–90 min (rate limited) |
| 8 | Patch ghi_kwh_m2_yr into clusters-meta.json | gateway www/data/ | 2 min |

**Disk delta:** ~10–15 MB new PMTiles
**Total est.:** ~2 hrs

**Countries covered after Night 2:** US (ASHRAE 169 + solar), CA (NECB + solar)

---

## Night 3 — 05:00 UTC 2026-05-26

**Script:** `build-aec-koppen.sh` (create this session)

**What runs:**

| Step | Action | Output | Est. |
|---|---|---|---|
| 1 | Download Beck et al. 2018 Köppen-Geiger 1km GeoTIFF | `work/aec/kg2018.tif` (~50 MB) | 10 min |
| 2 | GDAL polygonize by class code | `work/aec/koppen-raw.gpkg` | 30–45 min |
| 3 | Simplify polygons (mapshaper or ogr2ogr -simplify) | `work/aec/koppen-simplified.geojson` | 20 min |
| 4 | tippecanoe z3–z9 → layer9-koppen-global.pmtiles | gateway tiles/ | 30 min |
| 5 | PVGIS API: sample GHI at all EU cluster centroids | `work/aec/ghi-eu.json` | 30–60 min |
| 6 | Patch ghi_kwh_m2_yr for EU clusters into clusters-meta.json | gateway www/data/ | 2 min |

**Disk delta:** ~30–80 MB new PMTiles
**Total est.:** ~3–4 hrs

**Countries covered after Night 3:**
ALL 13 ISO countries get Köppen zone via global layer.
EU clusters get solar GHI added.
CA/US from Night 2 already complete.

---

## Night 4 — 05:00 UTC 2026-05-27

**Script:** `build-aec-seismic.sh` (create night-of)

**What runs:**

| Step | Action | Output | Est. |
|---|---|---|---|
| 1 | Download USGS NSHM 2023 PGA raster (2% in 50yr, CONUS) | `work/aec/usgs-pga-us.tif` | 15 min |
| 2 | Download NRCan 2015 seismic hazard raster (CA) | `work/aec/nrcan-pga-ca.tif` | 10 min |
| 3 | Reproject both to EPSG:4326 + merge | `work/aec/seismic-na.tif` | 5 min |
| 4 | GDAL → raster PMTiles z3–z9 | `layer10-seismic-na.pmtiles` | 20 min |
| 5 | ESHM20 (EU seismic CC BY 4.0) → raster PMTiles | `layer10-seismic-eu.pmtiles` | 30 min |
| 6 | Sample PGA at each cluster centroid → patch seismic_pga_g in clusters-meta.json | gateway | 5 min |

**Disk delta:** ~50–200 MB new PMTiles
**Total est.:** ~2–3 hrs

**Countries covered after Night 4:**
US, CA, ES, IT, GR, PT seismic data (high-seismicity markets prioritized).

---

## Night 5 — 05:00 UTC 2026-05-28

**Script:** `build-aec-flood.sh` (create night-of)

**What runs:**

| Step | Action | Output | Est. |
|---|---|---|---|
| 1 | Download WRI AQUEDUCT 3.0 1-in-100yr riverine GeoTIFF (global, ~5 GB) | `work/aec/aqueduct-100yr.tif` | 60–90 min |
| 2 | Clip to bbox of each ISO country cluster set | `work/aec/aqueduct-clipped/` | 30 min |
| 3 | Raster PMTiles z3–z8 | `layer11-flood-global.pmtiles` (~500 MB–1 GB) | 45 min |
| 4 | Download FEMA NFHL state GDBs (US-only, SFHA filter) | `work/aec/fema-sfha/` | 120–180 min (30+ GB download) |
| 5 | Merge SFHA features → tippecanoe → layer12-fema-sfha-us.pmtiles | gateway tiles/ | 45 min |
| 6 | Sample flood depth at cluster centroids → patch flood_hazard in clusters-meta.json | gateway | 5 min |

**Disk delta:** ~1–2 GB (FEMA is largest layer; raw downloads need 30 GB temp space)
**Total est.:** ~5–7 hrs

**⚠ Disk check before Night 5:** Verify ≥35 GB free (FEMA download + temp).
Check with: `df -h /srv/foundry`

---

## After Night 5 — index.html + BentoBox wiring

Not a nightly build. Separate coding session to add:

1. **MapLibre layer groups** for each new PMTiles layer (ASHRAE, NECB, Köppen, seismic, flood)
2. **Layer controls panel** — new "Site & Hazard" collapsible group
3. **BentoBox inspector** — new "Site Conditions" section per cluster:
   - Climate Zone: 4A — Mixed-Humid (ASHRAE 169)
   - Solar: 1,640 kWh/m²/yr (NSRDB)
   - Seismic PGA: 0.12g
   - Flood Exposure: Zone X / Moderate
4. **Wind/snow:** ATC Hazards API point-lookup on cluster click (no tile layer)

---

## Layer naming convention

| PMTiles file | What | Coverage |
|---|---|---|
| `layer8-ashrae-zones-us.pmtiles` | ASHRAE 169 / IECC climate zones | US county level |
| `layer8-necb-zones-ca.pmtiles` | NRCan NECB degree-day zones | Canada |
| `layer9-koppen-global.pmtiles` | Köppen-Geiger 2018 classification | Global (all 13 ISOs) |
| `layer10-seismic-na.pmtiles` | USGS + NRCan PGA raster | US + Canada |
| `layer10-seismic-eu.pmtiles` | ESHM20 PGA raster | EU countries |
| `layer11-flood-global.pmtiles` | WRI AQUEDUCT 1-in-100yr | Global proxy |
| `layer12-fema-sfha-us.pmtiles` | FEMA NFHL SFHA zones | US (regulatory precision) |

---

## clusters-meta.json new fields (added incrementally)

| Field | Source | Night added |
|---|---|---|
| `ghi_kwh_m2_yr` | NREL NSRDB (US/CA) / PVGIS (EU) | 2 + 3 |
| `ashrae_zone` | PNNL county table | 2 |
| `necb_zone` | NRCan | 2 |
| `koppen_class` | Beck 2018 (global) | 3 |
| `seismic_pga_g` | USGS/NRCan/ESHM20 | 4 |
| `flood_hazard` | WRI AQUEDUCT + FEMA (US) | 5 |

---

## Country coverage matrix (after each night)

| ISO | Köppen | Solar GHI | Code Zone | Seismic | Flood |
|---|---|---|---|---|---|
| US | N2 | N2 | N2 (ASHRAE) | N4 (USGS) | N5 (FEMA + AQUEDUCT) |
| CA | N3 | N2 | N2 (NECB) | N4 (NRCan) | N5 (AQUEDUCT) |
| GB | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| FR | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| DE | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| ES | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| IT | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| MX | N3 | N3 (NSRDB) | N3 (Köppen proxy) | — | N5 (AQUEDUCT) |
| PL | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| NL | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| PT | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| SE | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| DK | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| NO | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| FI | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |
| GR | N3 | N3 (PVGIS) | N3 (Köppen proxy) | N4 (ESHM20) | N5 (AQUEDUCT) |

N2 = Night 2, N3 = Night 3, N4 = Night 4, N5 = Night 5.
US and CA get code-prescriptive zones (ASHRAE/NECB); all others get Köppen proxy.
