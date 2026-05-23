---
plan: AEC-LAYERS-RESEARCH
created: 2026-05-19
updated: 2026-05-20
author: Jennifer Woodfine (research agent + Totebox synthesis)
status: REVISED — eco-region verdict corrected; country scoping added
---

# AEC Map Layers — Research Plan

Research question: what map layers should be added to gis.woodfinegroup.com to serve
Architecture, Engineering, and Construction (AEC) professionals evaluating co-location
sites for new building development?

---

## Part 0 — Country Scoping: Start with the US

**Revised guidance (2026-05-20):** Do not attempt to build AEC layers for all 13
countries simultaneously. Data availability and quality is uneven. Scope Phase 17
to the US, validate the pattern, then extend to Canada in Phase 18.

### Data coverage by country — Phase 17 candidates

| Layer | US | CA | MX | ES |
|---|---|---|---|---|
| Building code climate zones | ✓ ASHRAE/IECC, county-level, public domain | ✓ NECB degree-day, NRCan | Partial — NMX-C-460 zones exist; digitized availability unclear | ✗ — CTE zones A–E exist but no unified open shapefile |
| Flood hazard | ✓ FEMA NFHL, nationwide, public domain | Partial — provincial maps, inconsistent | Partial — CENAPRED; limited open download | Partial — SNCZI viewer, WMS only |
| Seismic | ✓ USGS NSHM 2023, public domain | ✓ NRCan 2015, public domain | ✓ CENAPRED, public domain | ✓ ESHM20, CC BY 4.0 |
| Solar GHI | ✓ NREL NSRDB, public domain | ✓ NREL NSRDB covers CA | ✓ NREL NSRDB covers MX | ✓ PVGIS (JRC), free |

**Phase 17: US only.** 282 of 443 T1 clusters are in the US. All four Tier 1 layers
have complete, open, nationally consistent US data. The US pattern validates the
architecture before tackling the more fragmented EU and MX data environments.

**Phase 18: Add Canada.** NRCan publishes NECB climate zones and seismic hazard
as open data. 32 T1 clusters.

**Phase 19+: Mexico and Spain.** Flood and energy-code zone data gaps need to be
resolved first — likely via WRI AQUEDUCT (global flood proxy) and Köppen lookup
tables (code proxy for ES/MX).

---

## Part 1 — Eco-regions vs. Climate Regions for AEC

### Revised verdict (2026-05-20): eco-regions ARE relevant — for landscape architecture

Initial assessment dismissed eco-regions entirely. That was wrong. The correct
distinction is by AEC discipline, not by AEC as a whole.

**Eco-regions are NOT relevant for:**
- Structural engineers (seismic, wind, snow loads are climate-zone derived, not eco-zone)
- MEP engineers (HVAC sizing uses ASHRAE climate zones)
- Building energy code compliance (IECC/ASHRAE 90.1 reference climate zones, not biomes)

**Eco-regions ARE relevant for — landscape architecture (LA):**

Landscape architects on commercial development projects (the scale of Walmart/IKEA/Costco
co-locations) reference eco-regions for every site. Specifically:

1. **Native plant selection.** EPA Level III/IV Ecoregions define what is native and
   adapted for a given site. Selecting non-native plants increases irrigation demand,
   maintenance cost, and invasive species risk. LEED BD+C requires a native/adaptive
   plant analysis referencing the local ecoregion.

2. **Stormwater bioretention design.** Rain gardens, bioswales, and constructed wetlands
   use native plant palettes specific to the ecoregion. A bioretention cell in EPA
   Ecoregion 34 (Central Appalachians) uses different plant species than one in
   Ecoregion 10 (High Plains). Getting this wrong means plant mortality and costly
   replacement.

3. **LEED Sustainable Sites credits (SSc2, SSc3, SSc4).** Stormwater management,
   heat island reduction, and light pollution reduction credits require demonstrating
   that plant selections are native/adaptive to the ecoregion. Eco-region boundary
   is the reference.

4. **SITES v2 certification.** The Sustainable SITES Initiative (GBCI) uses EPA
   Level III Ecoregions as the reference for native habitat and soils requirements.
   SITES is increasingly required for large commercial sites pursuing sustainability
   credentials.

5. **Invasive species avoidance.** Eco-region designation tells an LA which species
   are invasive in that region and must be avoided even if otherwise commonly planted.

6. **Xeriscape / water-efficient landscaping.** State-level water-budget calculations
   for landscaping (CA MWELO, TX WaterIQ) reference eco-region or EPA-derived zones.

### Recommended data source: EPA Level III Ecoregions

- **Source:** US EPA Level III Ecoregions shapefile. Public domain.
  https://www.epa.gov/eco-research/level-iii-and-iv-ecoregions-continental-united-states
- **Coverage:** US and Canada (Level II+ covers North America)
- **Global equivalent:** WWF Biomes (too coarse for site-level LA) or
  Resolve Ecoregions 2017 (Dinerstein et al., CC BY 4.0 — best global option at
  ~850 ecoregions; resolveconservation.org)
- **For Phase 17 (US):** EPA Level III (85 ecoregions in contiguous US). Level IV
  (967 ecoregions) is more precise for site-level work but also larger.
- **Size:** EPA Level III shapefile is ~5 MB → ~8–15 MB as PMTiles polygon.
- **UI framing:** Label as "Landscape Ecoregion" not "Eco-region" to be clear about
  its design use. Display in BentoBox: "Ecoregion: 34 — Central Appalachians (EPA Level III)"

### Summary table — revised

| Classification | Relevant for | Code-prescriptive | Recommended |
|---|---|---|---|
| EPA Level III/IV Ecoregions | Landscape architects (planting, LEED SS, SITES) | No (but LEED/SITES reference) | **Yes — Tier 1D for US** |
| WWF Biomes | Too coarse for site work | No | No |
| Resolve Ecoregions 2017 | Global LA equivalent to EPA | No | Tier 2 (global) |
| ASHRAE 169 / IECC | Structural, MEP, energy code | Yes (US) | **Yes — Tier 1A** |
| Köppen-Geiger | Global code proxy | No (correlates) | **Yes — Tier 2** |

### The actionable classification systems for AEC:

**1. ASHRAE 169 / IECC Climate Zones (US)**

The single most critical climate layer for US commercial building design.
- Zone number (1–8, with moisture subtype A/B/C) is the mandatory input to:
  - ASHRAE 90.1 envelope U-factor and SHGC compliance tables
  - IECC insulation R-value prescriptive tables
  - ACCA Manual N/J climate design condition selection
  - HVAC sizing calculations
  - LEED BD+C energy prerequisite documentation
- Defined at the county level. PNNL (Pacific Northwest National Laboratory) publishes
  the authoritative county-to-zone lookup table: public domain.
- Map polygon data: US DOE Building Energy Codes Program publishes a shapefile
  of IECC climate zone boundaries. Public domain.
- Source: https://www.energycodes.gov/prototype-building-models (see climate zones map)
- Size: <5 MB as PMTiles polygon.
- **An architect cannot begin envelope or HVAC design without this zone number.**

**2. Köppen-Geiger Climate Classification (global)**

The international scientific climate classification standard. Not code-prescriptive
in itself but maps predictably to national building energy codes worldwide.
- Use as global proxy where ASHRAE 169 does not apply (EU, MX, CA):
  - Spain CTE: zones A–E approximate to Köppen B/C subtypes by province
  - Germany GEG: Testreferenzjahr regions approximate to Köppen C subtypes
  - Canada NECB: degree-day zones correlate with Köppen D/C subtypes
  - Mexico NMX: climate zones A–E map from ASHRAE-like criteria but separately defined
- Beck et al. 2018 global 1-km raster. License: CC BY 4.0.
- Source: https://doi.org/10.1038/sdata.2018.214
  Download: https://figshare.com/articles/dataset/Present_and_future_K_ppen-Geiger_climate_classification_maps_at_1-km_resolution/6396641
- Size: ~50 MB raster (1 km); ~20–80 MB as simplified PMTiles polygon.

**3. National code climate zones (EU/CA lookup table — no unified map)**

There is no unified EU climate zone map equivalent to ASHRAE 169. Each member
state maintains its own energy regulation with its own zone system. A lookup
table mapping Köppen subtypes → national zone approximations is the practical
implementation. Stored as a static JSON in the gateway, applied client-side
when a cluster is selected. No new PMTiles layer needed.

### Summary table

| Classification | AEC use | Code-prescriptive | Geographic scope | Recommended? |
|---|---|---|---|---|
| WWF Biomes / EPA Ecoregions | None | No | Global | No |
| ASHRAE 169 / IECC | Energy code, HVAC | Yes (US) | US (county) | **Yes — Tier 1** |
| Köppen-Geiger | Code proxy, communication | No (but correlates) | Global | **Yes — Tier 2** |
| National code zones (ES/DE/CA/MX) | Code-prescriptive | Yes (national) | Country-specific | Yes — lookup table |

---

## Part 2 — Full AEC Layer Inventory

### 2.1 ASHRAE 169 / IECC Climate Zones

- **AEC actionability:** Critical. Required input for every US energy-code calculation.
- **Source:** US DOE Building Energy Codes Program / PNNL county lookup table.
- **License:** Public domain (US federal government).
- **Global availability:** US-only. Köppen serves as proxy elsewhere.
- **Format:** County polygon shapefile → PMTiles. Join TIGER county polygons with
  PNNL ASHRAE 169-2013 county table (downloadable as CSV from energycodes.gov).
- **Size:** ~3–5 MB as PMTiles polygon.
- **Integration:** PMTiles polygon layer. On cluster click, query climate zone from
  polygon centroid intersection. Display as "Climate Zone: 4A (Mixed-Humid)" in
  BentoBox inspector. Toggle: "Building Code Climate" in layer controls.

### 2.2 FEMA Flood Zones (US)

- **AEC actionability:** Very high. Special Flood Hazard Area (SFHA) designation
  (Zone A, AE, VE) can block development, mandate flood insurance, impose first-floor
  elevation requirements, and increase construction cost by 5–25%.
- **Source:** FEMA National Flood Hazard Layer (NFHL). Public domain.
  https://msc.fema.gov/portal/availabilitySearch
- **License:** Public domain (US federal government).
- **Global availability:** US-only. WRI AQUEDUCT covers global flood hazard (see §2.11).
- **Format:** GeoPackage/shapefile → PMTiles polygon. NFHL full download is ~30 GB;
  extract SFHA-only zones (ZONE_SUBTY = AE, A, VE, etc.) for a practical ~200–500 MB subset.
  Alternatively, use state-by-state NFHL downloads and process incrementally.
- **Size (SFHA-only):** ~200–500 MB as PMTiles polygon.
- **Integration:** PMTiles polygon, toggled separately. Show flood zone designation
  in BentoBox inspector. Flag: "⚠ SFHA — verify with local floodplain manager."

### 2.3 Seismic Hazard

- **AEC actionability:** High for clusters in Pacific Coast (US/MX), Greece, Italy,
  Portugal. Peak Ground Acceleration (PGA) at 2% probability in 50 years determines
  ASCE 7 seismic design category. Critical for structural system selection.
- **Sources:**
  - US: USGS National Seismic Hazard Model (NSHM) 2023. Public domain.
    https://www.usgs.gov/programs/earthquake-hazards/science/2023-us-nshm
  - Global: GSHAP (Global Seismic Hazard Assessment Program). Public domain.
    http://www.seismo.ethz.ch/static/gshap/
  - EU: ESHM20 (European Seismic Hazard Model 2020). CC BY 4.0.
    https://doi.org/10.12686/a15
- **License:** Public domain (USGS/GSHAP); CC BY 4.0 (ESHM20).
- **Format:** Raster (PGA in g) → PMTiles raster tiles at z3–z10.
- **Integration:** Raster overlay with color ramp (green=low → red=high). BentoBox
  shows PGA value in g units for selected cluster centroid.

### 2.4 Wind Load Zones

- **AEC actionability:** High. Design wind speed (mph/m/s) determines ASCE 7 wind
  load categories, cladding design pressures, and roof uplift calculations.
- **IMPORTANT — licensing issue:** ASCE 7 wind speed maps are copyrighted by the
  American Society of Civil Engineers. Redistribution as a GIS layer requires a license.
  **Do not digitize or redistribute ASCE 7 maps directly.**
- **Recommended approach:**
  - US: Use ATC Hazards by Location API (free, point lookup).
    https://hazards.atcouncil.org/ — returns Vult design wind speed at any lat/lon.
    Integrate as a client-side API call for selected cluster, not a served tile layer.
  - Global: Global Wind Atlas (DTU/World Bank), mean wind speed at 100m.
    CC BY 4.0. https://globalwindatlas.info/
    Useful for orientation and renewable energy assessment, not code design values.
- **Integration:** ATC API lookup on cluster selection → display "Design Wind Speed:
  115 mph (ASCE 7-22)" in BentoBox inspector. No PMTiles layer needed for US.

### 2.5 Snow Load Zones

- **AEC actionability:** High in northern US, Canada, Nordics, alpine regions.
  Ground snow load (Pg) drives roof structural design.
- **IMPORTANT — licensing issue:** ASCE 7 snow load maps also copyrighted by ASCE.
  **Same restriction as wind.** Use ATC Hazards API for point lookups.
- **Canada:** NBC snow load data is available through NBCC commentaries.
  National Research Council Canada data not freely redistributable as GIS.
- **Global proxy:** ERA5 reanalysis maximum annual snow depth (Copernicus/ECMWF).
  License: Copernicus Data License (free for commercial use with attribution).
  https://cds.climate.copernicus.eu/
- **Integration:** ATC API for US; ERA5 raster for international context.

### 2.6 Solar Irradiance / GHI

- **AEC actionability:** High for PV system sizing, passive solar design, LEED EAc1
  (renewable energy) credits, and daylighting design.
- **Sources:**
  - US: NREL NSRDB (National Solar Radiation Database). Public domain.
    https://nsrdb.nrel.gov/ — annual mean GHI at 4km resolution.
  - EU: PVGIS (Photovoltaic Geographical Information System, JRC).
    Free, no redistribution restrictions for derived data.
    https://re.jrc.ec.europa.eu/pvg_tools/en/
  - Global (commercial): Solargis annual mean GHI. CC BY-SA 4.0.
    **License risk:** CC BY-SA ShareAlike clause may be incompatible with EUPL-1.2.
    **Recommendation:** Use NSRDB (US) + PVGIS (EU) to avoid ShareAlike exposure.
- **Integration (recommended):** Do not create a PMTiles raster layer. Instead,
  sample GHI at each cluster centroid during `synthesize-od-study.py` and store
  as `ghi_kwh_m2_yr` in clusters-meta.json. Zero new tile infrastructure.
  Display in BentoBox: "Solar: 1,850 kWh/m²/yr."

### 2.7 WRI AQUEDUCT Global Flood Hazard

- **AEC actionability:** Medium-high. Global riverine and coastal flood hazard at
  return periods 2–1000 years. Covers all 13 current ISOs including EU and MX
  where FEMA data does not apply.
- **Source:** WRI AQUEDUCT Floods 3.0. CC BY 4.0.
  https://www.wri.org/data/aqueduct-floods-hazard-maps
- **License:** CC BY 4.0. Commercially permissible with attribution.
- **Format:** GeoTIFF raster per return period. Large total (~50 GB for all layers);
  use 1-in-100-year return period subset (~5–10 GB → ~1–3 GB as raster PMTiles).
- **Integration:** Raster PMTiles overlay at z3–z10. BentoBox: "Flood hazard:
  Moderate (1-in-100yr)."

### 2.8 Wildfire Hazard Zones

- **AEC actionability:** Medium-high for California, Pacific NW, interior West US,
  Spain, Portugal, Greece. WUI (Wildland-Urban Interface) designation can impose
  ignition-resistant construction requirements under CBC Chapter 7A (CA) and
  similar codes.
- **Sources:**
  - US: USFS Wildfire Hazard Potential (WHP) 2023. Public domain.
    https://www.fs.usda.gov/rds/archive/catalog/RDS-2015-0047-4
  - CA: CAL FIRE Fire Hazard Severity Zone maps. Public domain.
    https://osfm.fire.ca.gov/divisions/community-wildfire-preparedness-and-mitigation/wildland-hazards-building-codes/fire-hazard-severity-zones-maps/
  - EU: JRC/EFFIS (European Forest Fire Information System). Public domain.
    https://effis.jrc.ec.europa.eu/
- **Integration:** PMTiles polygon (FHSZ categories) or raster (WHP continuous).
  Display in BentoBox: "Wildfire: High Hazard Severity Zone (CAL FIRE)."

### 2.9 Soil Type / Bearing Capacity

- **AEC actionability:** Medium. USCS soil classification and bearing capacity
  affect foundation type selection (slab-on-grade vs. deep pile, etc.) and
  geotechnical report scope. Useful at site screening stage.
- **Sources:**
  - US: USDA Web Soil Survey (SSURGO). Public domain.
    https://websoilsurvey.nrcs.usda.gov/ — 100 GB+ total; practical as API.
    Use USDA SoilDataMart REST API for point lookups.
  - Global: FAO/IIASA World Soil Information (ISRIC) — SoilGrids 2.0. CC BY 4.0.
    https://soilgrids.org/ — 250m resolution raster.
- **Integration:** Point API lookup for US (USDA); H3 field from SoilGrids for
  global. Store in clusters-meta.json as `soil_class`. Low new-tile cost.

### 2.10 Urban Heat Island (UHI)

- **AEC actionability:** Medium. UHI intensity affects HVAC cooling load, outdoor
  comfort design, and LEED Sustainable Sites credits (heat island reduction).
- **Source:** NASA SEDAC Global Surface UHI Explorer. CC BY 4.0.
  https://sedac.ciesin.columbia.edu/data/set/sdei-global-uhi-2013
  Landsat-derived summer daytime LST difference (urban vs. rural).
- **Integration:** H3 res-7 sampled field in clusters-meta.json. BentoBox:
  "Urban Heat: +3.2°C vs. rural baseline."

### 2.11 Groundwater Depth

- **AEC actionability:** Medium. Shallow groundwater increases basement waterproofing
  requirements, affects foundation type, and can require dewatering during construction.
- **Source:** Gleeson et al. 2011 global water table depth raster. CC BY 4.0.
  https://doi.org/10.1038/nature10552
  Also: Fan et al. 2013 (higher resolution, 1km). Available via PANGAEA.
- **Integration:** H3 field in clusters-meta.json. BentoBox: "Groundwater: ~2.4m depth."

### 2.12 Permafrost Zones

- **AEC actionability:** Low for current coverage (US/CA/MX/EU). Relevant only
  for Alaska, northern Canada, Scandinavia — none of which are current anchor-chain
  markets. Defer unless coverage expands to Russia, Alaska, or Nordic rural clusters.
- **Source:** IPA (International Permafrost Association) global map.
- **Recommendation:** Deprioritize. Add note to revisit if Nordic T1 cluster count
  grows significantly.

---

## Part 3 — Prioritized Implementation Roadmap

### Tier 1 — Highest AEC value; open data; immediately buildable (US only, Phase 17)

**Tier 1A: ASHRAE 169 / IECC Climate Zones (US)**
- Source: DOE energycodes.gov + PNNL county table
- License: Public domain
- Format: PMTiles polygon (county boundaries + zone attribute)
- Build: Join TIGER county shapefile with PNNL ASHRAE 169-2013 CSV → tippecanoe
- Size: ~3–5 MB PMTiles
- UI: Toggle "Building Code Climate" → zone label on cluster click in BentoBox
- Estimated build time: 2–3 hours (data fetch + tippecanoe + MapLibre integration)

**Tier 1B: FEMA NFHL — SFHA-only subset (US)**
- Source: FEMA MSC NFHL state downloads
- License: Public domain
- Format: PMTiles polygon (SFHA zones only)
- Build: Download NFHL state GDBs → filter SFHA → merge → tippecanoe
- Size: ~200–500 MB PMTiles
- UI: Toggle "Flood Zone" → zone label (A/AE/VE/X) + warning flag in BentoBox
- Estimated build time: 4–6 hours (data download is the bottleneck)

**Tier 1C: Solar GHI — H3 field ingest (global)**
- Source: NREL NSRDB (US) + PVGIS API (EU)
- License: Public domain / free for commercial use
- Format: No new PMTiles. Point-sample at cluster centroids → clusters-meta.json field
- Build: Script to call NREL/PVGIS API per cluster centroid, store `ghi_kwh_m2_yr`
- Size: Zero tile infrastructure; ~200KB addition to clusters-meta.json
- UI: BentoBox field "Solar: X kWh/m²/yr"
- Estimated build time: 3–4 hours (API rate limits may slow US ingest)

**Tier 1D: EPA Level III Ecoregions (US) — Landscape Architecture**
- Source: US EPA Level III Ecoregions shapefile, public domain
  https://www.epa.gov/eco-research/level-iii-and-iv-ecoregions-continental-united-states
- License: Public domain
- Format: PMTiles polygon
- Build: Download EPA shapefile → tippecanoe → layer-ecoregion-us.pmtiles
- Size: ~5 MB source → ~8–15 MB PMTiles
- UI: Toggle "Landscape Ecoregion" in layer controls. BentoBox: "Ecoregion: 34 — Central Appalachians"
- Note: Label clearly as landscape-architecture reference, not regulatory boundary
- Estimated build time: 1–2 hours (small file, straightforward pipeline)

### Tier 2 — High value; moderate integration complexity

**Tier 2A: Köppen-Geiger (global PMTiles polygon)**
- Source: Beck et al. 2018, Figshare CC BY 4.0
- Build: Download 1-km raster → polygonize by class → simplify → tippecanoe
- Size: ~30–80 MB PMTiles
- Note: Polygonization of 1-km global raster is compute-intensive; use GDAL
  `gdal_polygonize` or pre-simplified SHP from published sources
- Estimated build time: 6–8 hours (raster processing)

**Tier 2B: Seismic PGA (US + EU raster PMTiles)**
- Source: USGS NSHM 2023 (US) + ESHM20 (EU, CC BY 4.0)
- Build: Merge rasters → convert to PNG raster tiles → tippecanoe or GDAL tiler
- Size: ~50–200 MB PMTiles
- Estimated build time: 4–6 hours

**Tier 2C: WRI AQUEDUCT 1-in-100yr Flood (global)**
- Source: WRI AQUEDUCT 3.0, CC BY 4.0
- Build: Download 1-in-100yr riverine + coastal GeoTIFF → raster PMTiles
- Size: ~1–3 GB PMTiles (raster); accept lower zoom ceiling (z8 max)
- Estimated build time: 6–8 hours (download + processing)

### Tier 3 — Specialized; US/EU-only or complex licensing

| Layer | Blocker | Action |
|---|---|---|
| Wind load (ASCE 7) | ASCE copyright | ATC API point-lookup only |
| Snow load (ASCE 7) | ASCE copyright | ATC API point-lookup only |
| USDA SSURGO soil | 100 GB+ raw | REST API point-lookup only |
| Wildfire (USFS WHP) | US-only broad coverage | PMTiles polygon Phase 18 |
| CAL FIRE FHSZ | CA-only | Bundle with wildfire layer |
| Urban Heat Island | Medium value | H3 field in clusters-meta.json |
| Groundwater depth | Low precision at res-7 | H3 field, low priority |
| Permafrost | No current market coverage | Defer |

---

## Part 4 — Technical Integration Architecture

### Layer type decision matrix

| Layer | Serve as | Rationale |
|---|---|---|
| ASHRAE/IECC zones | PMTiles **polygon** | Regulatory boundaries must be precise; H3 aggregation blurs county edges |
| FEMA flood zones | PMTiles **polygon** | SFHA boundary is a legal regulatory line; H3 would destroy precision |
| Köppen-Geiger | PMTiles **polygon** | Categorical classification; polygon is natural representation |
| Seismic PGA | PMTiles **raster** | Continuous gradient; raster is efficient at ~200 MB |
| AQUEDUCT flood | PMTiles **raster** | Continuous depth values; raster appropriate |
| GHI solar | **clusters-meta.json field** | Point value per cluster; no tile layer needed |
| Wind/snow (US) | **ATC API call** | Copyright blocks redistribution; real-time API is compliant |
| Soil class | **clusters-meta.json field** | USDA API point lookup; store result |
| UHI | **clusters-meta.json field** | NASA sampled value per cluster centroid |

### Zoom level recommendations

- ASHRAE/Köppen polygon: visible z4–z16 (county scale readable at z6+)
- FEMA flood polygon: visible z8–z16 (parcel-scale, too noisy below z8)
- Seismic raster: z3–z10 (regional pattern; fine detail unnecessary)
- AQUEDUCT raster: z3–z8 (coarse risk zones only)

### UI placement

- **Layer controls panel** (left sidebar): New collapsible group "Site & Hazard"
  - Toggles: Building Code Climate | Flood Zone | Seismic | Solar
- **BentoBox inspector** (cluster detail panel): New section "Site Conditions"
  - Climate Zone: 4A — Mixed-Humid (ASHRAE 169 / IECC 2021)
  - Flood Exposure: Zone X (Minimal Hazard)
  - Seismic PGA: 0.12g (moderate)
  - Solar: 1,640 kWh/m²/yr
  - Wind: 115 mph design (ASCE 7-22) [ATC API]

### Data pipeline integration point

- GHI, soil, UHI, wind/snow API values: add to `synthesize-od-study.py` as
  a post-processing step that enriches clusters-meta.json entries
- PMTiles layers: built by new `build-aec-tiles.py` script following the
  same pattern as `build-tiles.py`

---

## Part 5 — Open Risk Items

| Risk | Severity | Mitigation |
|---|---|---|
| ASCE 7 wind/snow map copyright | High — blocks redistribution | ATC Hazards API point-lookup only; do not digitize ASCE maps |
| Solargis CC BY-SA vs. EUPL-1.2 ShareAlike conflict | Medium — needs legal review | Use NSRDB (US public domain) + PVGIS (EU, no ShareAlike) |
| FEMA NFHL 30 GB download | Medium — operational | Process state-by-state; SFHA filter reduces to ~500 MB |
| No unified EU building-code climate zone map | Medium — gap in coverage | Köppen → national lookup table as interim; flag in UI |
| ESHM20 seismic (CC BY 4.0) vs. USGS (public domain) merge | Low | CC BY 4.0 permits commercial use; merge is fine with attribution |
| SoilGrids 250m resolution too coarse for parcel decisions | Low | Disclose resolution in UI; it's for orientation, not geotechnical spec |

---

## Phase 17 Proposed Build Sequence

```
Week 1 — ASHRAE/IECC Climate Zone PMTiles + BentoBox integration
  Day 1: Download PNNL county table + TIGER counties → join → GeoJSON
  Day 2: tippecanoe → layer8-ashrae-zones.pmtiles
  Day 3: MapLibre layer + BentoBox inspector field
  Day 4: Style, test all 8 zones + moisture subtypes
  Day 5: Commit + deploy

Week 2 — Solar GHI field ingest
  Day 1–2: Script NREL NSRDB API calls for US cluster centroids
  Day 3: Script PVGIS API calls for EU/MX/CA cluster centroids
  Day 4–5: Populate ghi_kwh_m2_yr in clusters-meta.json + BentoBox display

Week 3 — Köppen-Geiger PMTiles (global)
  Day 1–2: Download Beck et al. raster → GDAL polygonize
  Day 3–4: Simplify + tippecanoe → layer9-koppen.pmtiles
  Day 5: MapLibre layer + style (26 Köppen classes → colour palette)

Week 4 — FEMA NFHL SFHA layer (US, state-by-state)
  Day 1–3: Download + filter NFHL → merge SFHA features
  Day 4: tippecanoe → layer10-flood-sfha.pmtiles
  Day 5: MapLibre layer + BentoBox warning flag
```

---

## Maintenance

- Revisit ASCE wind/snow approach when ATC API terms are confirmed for commercial use
- Re-evaluate Solargis licensing if NSRDB/PVGIS coverage proves insufficient
- Add Tier 2 layers (seismic, AQUEDUCT) in Phase 18 after Phase 17 lanes ship
- Update Köppen vintage when Beck et al. publish next release (projected 2025–2026)
