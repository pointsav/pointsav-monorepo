---
plan: AEC-NIGHTLY-BUILD-PLAN
created: 2026-05-23
updated: 2026-05-23 (parity research incorporated)
author: Jennifer Woodfine (project-gis Totebox)
status: ACTIVE — queued across 5 nights from 2026-05-24
companion: AEC-LAYERS-RESEARCH.md, AEC-INFRASTRUCTURE-LAYERS-RESEARCH.md, AEC-DATA-PARITY-RESEARCH.md
---

# AEC Layers — Staged Nightly Build Plan

Five-night rollout of AEC/site-conditions layers for gis.woodfinegroup.com.
Each night runs at 05:00 UTC (22:00 PDT) as a `nohup` background job.

**Canada note:** Canada is NOT Phase 18. Köppen-Geiger is global (covers CA, MX, ES
simultaneously with US). NSRDB solar covers CA in the same API calls. The
Canada-specific layer (NRCan NECB climate zones) is lightweight and runs Night 2
alongside ASHRAE. Canada gets full AEC coverage on Night 2+3 — not next phase.

**EU parity upgrade (from parity research 2026-05-23):** EU flood data is
**regulatory-grade**, not just AQUEDUCT proxy. EU Floods Directive 2007/60/EC
requires all member states to publish hazard maps. Per-country shapefiles for
GB/FR/ES/IT run Night 5 alongside FEMA; smaller ISOs ingest via INSPIRE WFS.

**EFFIS wildfire action required before Night 5:** Submit formal data request at
https://forest-fire.emergency.copernicus.eu/applications/data-and-services
Without approval, GWIS FWI raster serves as fallback.

**CONABIO blocker:** Mexico eco-regions and Köppen-García are CC BY-NC — blocked
for commercial use. Use INEGI climate raster for NMX climate zones; Resolve 2017
(CC BY 4.0) for eco-regions.

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
| 5 | Download NRCan NECB HOT2000 climate zone via MapServer REST | `work/aec/necb-zones-ca.geojson` | 5 min |
| 6 | ogr2ogr → tippecanoe → layer8-necb-zones-ca.pmtiles | gateway tiles/ | 5 min |
| 7a | EU climate zones: download national lookup tables (FR/ES/IT/DE/GR/PT/FI/PL/SE) + GISCO LAU2 boundaries | `work/aec/eu-lau2-raw.gpkg` | 20 min |
| 7b | Build-by-join: produce per-country zone GeoJSON; merge to layer8-eu-climate-zones.pmtiles | gateway tiles/ | 15 min |
| 8 | NREL NSRDB API: sample GHI at all US + CA + MX cluster centroids | `work/aec/ghi-us-ca-mx.json` | 60–90 min (rate limited) |
| 9 | Patch ghi_kwh_m2_yr + ashrae_zone + necb_zone + eu_climate_zone into clusters-meta.json | gateway www/data/ | 2 min |

**Note on EU climate zones (build-by-join pipeline):**
- FR RE2020: 8 zones (H1a–H3) via département lookup (Arrêté 4 août 2021) + GISCO boundaries → Etalab 2.0
- ES CTE DB-HE: 12 zones (A3–E1) via municipal lookup from Annex B + INE municipios → Ministerio attribution
- IT DPR 412/1993: 6 zones (A–F) via ENEA GradiGiorni + ISTAT comuni → CC BY
- DE GEG 2023 / TRY 2017: 15 TRY regions via BBSR portal (account required) + DWD CDC raster → DWD attribution
- GR KENAK: 4 zones (A–D) via regulation lookup + ELSTAT municipalities → ELSTAT free
- PT SCE/REH-RECS: 3 winter + 3 summer via DGEG + SNIG/DGT boundaries → free attribution
- FI SFS-EN ISO 15927-4: 4 zones via municipality lookup + Maanmittauslaitos → CC BY 4.0
- PL WT 2021: 5 zones (I–V) via MDPI 2024 geospatial model → CC BY
- SE BBR: 4 zones via Boverket Klimatdatabas API → CC0/CC BY
- GB/NL/DK/NO: Single-reference countries — no zone polygon needed (NL/DK/NO) or HadUK-Grid (GB)

**Disk delta:** ~15–25 MB new PMTiles (ASHRAE + NECB + EU climate zones)
**Total est.:** ~3 hrs

**Countries covered after Night 2:** US (ASHRAE 169 + solar), CA (NECB + solar),
MX (solar), EU 8 countries (regulatory-grade code zones)

---

## Night 3 — 05:00 UTC 2026-05-26

**Script:** `build-aec-koppen-ecozones.sh` (create this session)

**What runs:**

| Step | Action | Output | Est. |
|---|---|---|---|
| 1 | Download Beck et al. 2018 Köppen-Geiger 1km GeoTIFF | `work/aec/kg2018.tif` (~50 MB) | 10 min |
| 2 | GDAL polygonize by class code | `work/aec/koppen-raw.gpkg` | 30–45 min |
| 3 | Simplify polygons (mapshaper or ogr2ogr -simplify) | `work/aec/koppen-simplified.geojson` | 20 min |
| 4 | tippecanoe z3–z9 → layer9-koppen-global.pmtiles | gateway tiles/ | 30 min |
| 5 | Download Resolve Ecoregions 2017 (CC BY 4.0, ~150 MB) | `work/aec/Ecoregions2017.zip` | 15 min |
| 6 | Unzip + tippecanoe → layer13-ecoregions-global.pmtiles | gateway tiles/ | 20 min |
| 7 | Download EEA Biogeographical Regions 2016 | `work/aec/eea-biogeographic-regions.shp` | 5 min |
| 8 | tippecanoe → layer14-biogeographic-eu.pmtiles | gateway tiles/ | 5 min |
| 9 | EPA Level III Ecoregions (US) | `work/aec/us-ecoregions-l3.shp` | 5 min |
| 10 | tippecanoe → layer14-ecoregions-us.pmtiles | gateway tiles/ | 5 min |
| 11 | PVGIS API: sample GHI at all EU cluster centroids | `work/aec/ghi-eu.json` | 30–60 min |
| 12 | Patch ghi_kwh_m2_yr + koppen_class + ecoregion_name for EU/global clusters into clusters-meta.json | gateway www/data/ | 2 min |

**Resolve Ecoregions 2017:**
- Direct download: `https://storage.googleapis.com/teow2016/Ecoregions2017.zip`
- License: CC BY 4.0 (commercial OK with attribution to Dinerstein et al. 2017)
- 846 ecoregions globally; covers all 16 ISOs; one ecoregion per retail site location

**Disk delta:** ~50–100 MB new PMTiles (Köppen + Resolve + EEA Biogeo + EPA L3 US)
**Total est.:** ~3–4 hrs

**Countries covered after Night 3:**
ALL 13 ISO countries get Köppen zone via global layer.
ALL 16 ISOs get Resolve ecoregion.
EU clusters get EEA Biogeographical Region.
US clusters get EPA Level III ecoregion (precision override).
EU clusters get solar GHI added. CA/US from Night 2 already complete.

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
| 6 | Mexico seismic (CENAPRED CFE-2015 / SGM) via Atlas Nacional → merge with ESHM20 | `work/aec/seismic-mx.tif` or skip if portal down | 20 min |
| 7 | GWL_FCS30 global wetland (CC BY 4.0, Zenodo) — sample at cluster centroids | `work/aec/wetlands-sample.json` | 30 min |
| 8 | Sample PGA at each cluster centroid → patch seismic_pga_g + wetland_class in clusters-meta.json | gateway | 5 min |

**GWL_FCS30 wetland (Night 4 addition):**
- URL: https://zenodo.org/records/7340516
- License: CC BY 4.0
- 30m global wetland, 8 classes — best open NWI equivalent globally
- Cluster centroid sample only (no PMTiles layer this round — add to Night 5 if disk allows)

**CENAPRED note:** Mexico seismic portal is intermittent. Try
`http://www.atlasnacionalderiesgos.gob.mx/` — if down, skip and use ESHM20
southern extent as MX proxy (ESHM20 covers ~MX border region). Contact
anr.administracion@cenapred.unam.mx for offline shapefile bundles.

**Disk delta:** ~50–200 MB new PMTiles
**Total est.:** ~2–3 hrs

**Countries covered after Night 4:**
US, CA seismic (USGS + NRCan). EU all ISOs (ESHM20). MX conditional.
Global wetland class added to cluster metadata.

---

## Night 5 — 05:00 UTC 2026-05-28

**Script:** `build-aec-flood.sh` (create night-of)

**⚠ Pre-Night-5 actions:**
1. Verify ≥35 GB free: `df -h /srv/foundry`
2. Confirm EFFIS data request submitted (URL above) — or proceed with GWIS FWI fallback
3. Check CENAPRED MX seismic was ingested Night 4 — else retry before flood build

**What runs:**

| Step | Action | Output | Est. |
|---|---|---|---|
| 1 | Download WRI AQUEDUCT 3.0 1-in-100yr riverine GeoTIFF (global, ~5 GB) | `work/aec/aqueduct-100yr.tif` | 60–90 min |
| 2 | Clip to bbox of each ISO country cluster set | `work/aec/aqueduct-clipped/` | 30 min |
| 3 | Raster PMTiles z3–z8 | `layer11-flood-global.pmtiles` (~500 MB–1 GB) | 45 min |
| 4 | Download FEMA NFHL state GDBs (US-only, SFHA filter) | `work/aec/fema-sfha/` | 120–180 min (30+ GB download) |
| 5 | Merge SFHA features → tippecanoe → layer12-fema-sfha-us.pmtiles | gateway tiles/ | 45 min |
| 6 | UK EA Flood Map for Planning (OGL) — Flood Zones 2 & 3 GeoPackage | `work/aec/ea-flood-zones-gb.gpkg` | 30 min |
| 7 | FR Géorisques Zonages Inondation 2020 (TRI, Etalab 2.0) | `work/aec/georisques-flood-fr.shp` | 20 min |
| 8 | ES SNCZI Zonas Inundables T100 (free, 1.02 GB) | `work/aec/snczi-flood-es.shp` | 40 min |
| 9 | IT IdroGEO PAI (IODL 2.0/CC BY) | `work/aec/idrogeo-flood-it.shp` | 20 min |
| 10 | Merge EU regulatory shapefiles → tippecanoe → layer12-flood-eu-regulatory.pmtiles | gateway tiles/ | 30 min |
| 11 | GWIS FWI wildfire raster — or EFFIS if approved | `work/aec/wildfire-global.tif` | 20 min |
| 12 | tippecanoe → layer15-wildfire-global.pmtiles | gateway tiles/ | 20 min |
| 13 | Canada Future Flood Susceptibility 2024 (OGL-Canada) + AQUEDUCT | `work/aec/flood-ca.tif` | 30 min |
| 14 | Sample flood depth + wildfire hazard at cluster centroids → patch flood_hazard + wildfire_hazard in clusters-meta.json | gateway | 5 min |

**EU regulatory flood notes:**
- GB EA: https://environment.data.gov.uk/dataset/04532375-a198-476e-985e-0579a0a11b47 → OGL v3; most comparable to FEMA NFHL
- FR Géorisques: https://www.georisques.gouv.fr/donnees/bases-de-donnees/zonages-inondation-rapportage-2020 → Etalab 2.0
- ES SNCZI: https://www.miteco.gob.es/en/cartografia-y-sig/ide/descargas/agua/descargas_agua_snczi.html → MITECO attribution
- IT IdroGEO: https://idrogeo.isprambiente.it/app/page/open-data → IODL 2.0
- DE/PL/GR/NL/SE/DK/NO/FI/PT: INSPIRE WFS → EU Floods Directive reporting layer; ingest via INSPIRE geoportal

**Canada flood:** No national FEMA-equivalent. Use NRCan Future Flood Susceptibility 2024
(XGBoost-modelled national proxy, OGL-Canada) + AQUEDUCT as background.
URL: https://open.canada.ca/data/en/dataset/c00f95a3-7bab-4d28-b9cc-b30f06b5afd2

**Disk delta:** ~1–2 GB (FEMA largest; 30+ GB temp space needed)
**Total est.:** ~7–9 hrs

---

## After Night 5 — index.html + BentoBox wiring

Not a nightly build. Separate coding session to add:

1. **MapLibre layer groups** for each new PMTiles layer (ASHRAE, NECB, EU climate zones,
   Köppen, seismic, flood-regulatory, flood-global, ecoregions, wildfire)
2. **Layer controls panel** — new "Site & Hazard" collapsible group
3. **BentoBox inspector** — new "Site Conditions" section per cluster:
   - Climate Zone: 4A — Mixed-Humid (ASHRAE 169) / RE2020 H1b (FR) / CTE A3 (ES) / etc.
   - Solar: 1,640 kWh/m²/yr (NSRDB/PVGIS)
   - Ecoregion: Temperate Broadleaf Forests (Resolve 2017)
   - Seismic PGA: 0.12g
   - Flood Exposure: Zone X / Moderate / EA Zone 2 (GB)
   - Wetland Class: Freshwater Emergent (GWL_FCS30) or None
4. **Wind/snow:** ATC Hazards API point-lookup on cluster click (no tile layer)

---

## Layer naming convention

| PMTiles file | What | Coverage |
|---|---|---|
| `layer8-ashrae-zones-us.pmtiles` | ASHRAE 169 / IECC climate zones | US county level |
| `layer8-necb-zones-ca.pmtiles` | NRCan NECB HOT2000 climate zones | Canada |
| `layer8-eu-climate-zones.pmtiles` | National building code zones (build-by-join) | FR/ES/IT/DE/GR/PT/FI/PL/SE (8 regulatory + 1 raster) |
| `layer9-koppen-global.pmtiles` | Köppen-Geiger 2018 classification | Global (all 16 ISOs) |
| `layer10-seismic-na.pmtiles` | USGS + NRCan PGA raster | US + Canada |
| `layer10-seismic-eu.pmtiles` | ESHM20 PGA raster | EU countries + MX proxy |
| `layer11-flood-global.pmtiles` | WRI AQUEDUCT 1-in-100yr | Global (proxy / CA background) |
| `layer12-fema-sfha-us.pmtiles` | FEMA NFHL SFHA zones | US (regulatory precision) |
| `layer12-flood-eu-regulatory.pmtiles` | EU Floods Directive shapefiles | GB/FR/ES/IT + INSPIRE WFS ISOs |
| `layer13-ecoregions-global.pmtiles` | Resolve Ecoregions 2017 (CC BY 4.0) | Global all 16 ISOs |
| `layer14-biogeographic-eu.pmtiles` | EEA Biogeographical Regions 2016 | EU (Habitats Directive ref) |
| `layer14-ecoregions-us.pmtiles` | EPA Level III Ecoregions | US (precision override) |
| `layer15-wildfire-global.pmtiles` | GWIS FWI or EFFIS (if approved) | Global / EU |

---

## clusters-meta.json new fields (added incrementally)

| Field | Source | Night added |
|---|---|---|
| `ghi_kwh_m2_yr` | NREL NSRDB (US/CA/MX) / PVGIS (EU) | 2 + 3 |
| `ashrae_zone` | PNNL county table | 2 |
| `necb_zone` | NRCan HOT2000 MapServer | 2 |
| `eu_climate_zone` | Build-by-join from national codes + GISCO LAU2 | 2 |
| `koppen_class` | Beck 2018 (global) | 3 |
| `ecoregion_name` | Resolve 2017 (global) | 3 |
| `ecoregion_biome` | Resolve 2017 (global) | 3 |
| `epa_l3_ecoregion` | EPA Level III (US only) | 3 |
| `seismic_pga_g` | USGS/NRCan/ESHM20/CENAPRED | 4 |
| `wetland_class` | GWL_FCS30 30m global (CC BY 4.0) | 4 |
| `flood_hazard` | WRI AQUEDUCT + FEMA (US) + EA (GB) + Géorisques (FR) + SNCZI (ES) | 5 |
| `wildfire_hazard` | GWIS FWI or EFFIS | 5 |

---

## Country coverage matrix (after each night)

| ISO | Köppen | Solar GHI | Code Zone | Seismic | Flood | Ecoregion |
|---|---|---|---|---|---|---|
| US | N3 | N2 | N2 (ASHRAE) | N4 (USGS) | N5 (FEMA + AQUEDUCT) | N3 (EPA L3 + Resolve) |
| CA | N3 | N2 | N2 (NECB) | N4 (NRCan) | N5 (Future Susceptibility + AQUEDUCT) | N3 (Resolve) |
| MX | N3 | N2 (NSRDB) | N2 (INEGI raster, NMX proxy) | N4 (CENAPRED — conditional) | N5 (AQUEDUCT) | N3 (Resolve) |
| GB | N3 | N3 (PVGIS) | N2 (HadUK ref, no zone polygon) | N4 (ESHM20) | N5 (**EA regulatory OGL**) | N3 (Resolve + EEA Biogeo) |
| FR | N3 | N3 (PVGIS) | N2 (**RE2020 regulatory**) | N4 (ESHM20) | N5 (**Géorisques regulatory**) | N3 (Resolve + EEA Biogeo) |
| DE | N3 | N3 (PVGIS) | N2 (TRY 2017 raster join) | N4 (ESHM20) | N5 (LAWA WFS) | N3 (Resolve + EEA Biogeo) |
| ES | N3 | N3 (PVGIS) | N2 (**CTE DB-HE regulatory**) | N4 (ESHM20) | N5 (**SNCZI regulatory**) | N3 (Resolve + EEA Biogeo) |
| IT | N3 | N3 (PVGIS) | N2 (**DPR 412 regulatory**) | N4 (ESHM20+INGV) | N5 (**IdroGEO regulatory**) | N3 (Resolve + EEA Biogeo) |
| PL | N3 | N3 (PVGIS) | N2 (WT 2021 join) | N4 (ESHM20) | N5 (INSPIRE WFS) | N3 (Resolve + EEA Biogeo) |
| NL | N3 | N3 (PVGIS) | n/a single climate | N4 (ESHM20) | N5 (INSPIRE WFS) | N3 (Resolve + EEA Biogeo) |
| PT | N3 | N3 (PVGIS) | N2 (**SCE regulatory**) | N4 (ESHM20) | N5 (SNIAmb INSPIRE) | N3 (Resolve + EEA Biogeo) |
| SE | N3 | N3 (PVGIS) | N2 (BBR API) | N4 (ESHM20) | N5 (MSB INSPIRE) | N3 (Resolve + EEA Biogeo) |
| DK | N3 | N3 (PVGIS) | n/a single climate | N4 (ESHM20) | N5 (INSPIRE WFS) | N3 (Resolve + EEA Biogeo) |
| NO | N3 | N3 (PVGIS) | n/a single climate | N4 (ESHM20) | N5 (NVE INSPIRE) | N3 (Resolve + EEA Biogeo) |
| FI | N3 | N3 (PVGIS) | N2 (SFS 4 zones) | N4 (ESHM20) | N5 (SYKE INSPIRE) | N3 (Resolve + EEA Biogeo) |
| GR | N3 | N3 (PVGIS) | N2 (KENAK 4 zones) | N4 (ESHM20) | N5 (YPEN INSPIRE) | N3 (Resolve + EEA Biogeo) |

N2 = Night 2, N3 = Night 3, N4 = Night 4, N5 = Night 5.
**Bold** = regulatory-grade (not modelled proxy).

---

## Parity notes

- **EU flood is regulatory-grade** for GB/FR/ES/IT (direct national shapefiles) and all
  smaller ISOs (EU Floods Directive INSPIRE WFS). This matches FEMA for those markets.
- **Canada has no FEMA equivalent** (FHIMP programme runs 2024–2028). Future Flood
  Susceptibility (XGBoost model) is the national proxy available today.
- **Mexico** has the weakest coverage: CENAPRED seismic is intermittent; CONABIO
  eco-regions/climate are CC BY-NC (blocked for commercial use). INEGI raster + Resolve
  used instead.
- **Wildfire:** EFFIS is the best EU layer but requires a formal data request. GWIS FWI
  raster is the fallback. Submit EFFIS request before Night 5.
- **Ecoregions:** Three-tier system — Resolve 2017 globally, EPA L3 for US precision,
  EEA Biogeographical Regions for EU regulatory reference (Habitats Directive frame).
