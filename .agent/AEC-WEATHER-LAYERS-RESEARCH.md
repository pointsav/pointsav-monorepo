# AEC Weather & Climate Layers — Research Plan
> **Created:** 2026-05-20 | **Scope:** US-first; Phase 17 target
> **Companion:** AEC-LAYERS-RESEARCH.md (master plan)

---

## Summary Table

| Layer | Source | License | Tier | Phase | Complexity |
|---|---|---|---|---|---|
| HDD / CDD | NOAA NCEI 1991–2020 Normals | Public domain | **1** | 17 | Low — CSV nearest-station snap |
| ASHRAE Design Temps | DOE EPW + ladybug | Public domain | **1** | 17 | Medium — EPW parse per cluster |
| NOAA Atlas 14 Precip Freq | NOAA PFDS REST API | Public domain | **1** | 17 | Low — on-demand API |
| Frost Depth / AFI | NRCS SDA API (SSURGO) + NOAA Normals | Public domain | **2** | 17 | Medium — SSURGO SDA query |
| Prevailing Wind / Wind Rose | ERA5 U10/V10 monthly | Copernicus free commercial | **2** | 17–18 | Medium — NetCDF → H3 aggregation |
| Hail Risk | NOAA SPC Storm Events CSV | Public domain | **2** | 17 | Low — kernel density from CSV |
| Tornado Risk | NOAA SPC F1+ probability grid | Public domain | **1** | 17 | Low — pre-computed raster |
| Hurricane Track Density | NOAA IBTrACS AT+EP | Public domain | **2** | 17 | Medium — track density KDE |
| Atmospheric Corrosivity | NOAA coastline proximity proxy | Public domain | **3** | 18 | Low proxy / Medium MICAT |

**Integration architecture:** All Phase 17 layers integrate via **clusters-meta.json H3 fields** — no new PMTiles infrastructure required for Phase 17. Estimated payload: ~350 bytes/cluster × 13,657 clusters ≈ 4.8 MB addition.

---

## Layer 1 — Heating Degree Days / Cooling Degree Days (HDD/CDD)

### What it shows
Annual HDD (base 65°F) and CDD (base 65°F) calculated from 30-year normals. Fundamental HVAC sizing input — every mechanical engineer needs this. Also feeds ASHRAE 90.1 energy budget compliance and LEED EA credit calculations.

### Data Source
- **Agency:** NOAA National Centers for Environmental Information (NCEI)
- **Dataset:** Climate Normals 2006–2020 (or 1991–2020) — `normal_ann.csv`
- **URL:** `https://www.ncei.noaa.gov/data/normals-annual/2006-2020/access/`
- **Fields:** `HDD6510` (annual HDD, base 65°F × 10), `CDD6510`
- **Coverage:** ~9,000 ASOS/COOP stations; continental US + AK + HI
- **Vintage:** 2006–2020 normals published 2022; 1991–2020 also available
- **License:** US federal government work, public domain

### Integration
- Download `normal_ann.csv` (~5 MB)
- For each cluster centroid: find nearest station within 75 km (H3 k-ring or scipy KDTree)
- Divide raw values by 10 (stored as tenths), add `hdd`, `cdd` fields to clusters-meta.json
- Flag clusters > 75 km from nearest station as `hdd_approx: true`
- Phase 18: replace with ERA5 2m temperature gridded derivation for CA/MX/EU

### BentoBox display
"Heating Degree Days: 4,820 / Cooling Degree Days: 1,340 (NOAA 2006–2020)"

---

## Layer 2 — ASHRAE Design Temperatures

### What it shows
ASHRAE Fundamentals Handbook Table 1 design conditions: 99%/99.6% winter dry-bulb, 1%/2% summer dry-bulb, 1% summer wet-bulb, mean coincident wet-bulb (MCWB), dehumidification dew points. The literal inputs to HVAC load calculations per ASHRAE 90.1 and ANSI/ASHRAE Standard 183.

### Data Source
**Primary (open):**
- **DOE EnergyPlus Weather Files (EPW):** `https://energyplus.net/weather`
- ~3,000 US stations; EPW format encodes design conditions in header records
- `ladybug` Python library (MIT) parses EPW design condition headers cleanly
- **License:** Public domain (US DOE)

**Authoritative:**
- ASHRAE Handbook Fundamentals Table 1 — proprietary ASHRAE publication ($)
- For open pipeline, DOE EPW is the permissible proxy (same underlying station data)

### Integration
- Download all US EPWs (bulk zip per climate region, ~1.5 GB total)
- Parse `DESIGN CONDITIONS` header record: fields 99% HDB, 1% CDB, MCWB
- Nearest-EPW-station snap per cluster centroid (same KDTree as HDD/CDD)
- Add fields: `ashrae_hdb99`, `ashrae_hdb996`, `ashrae_cdb1`, `ashrae_mcwb1`
- Phase 18: global EPW coverage is reasonable (ladybug-tools has 4,500+ EPWs)

---

## Layer 3 — NOAA Atlas 14 Precipitation Frequency

### What it shows
Rainfall intensity by return period (2-, 5-, 10-, 25-, 50-, 100-, 500-year) and duration (5-min to 60-day). The regulatory standard for stormwater infrastructure design in the US — referenced by ASCE 7, local grading ordinances, and every civil engineer's drainage report.

### Data Source
- **Agency:** NOAA Precipitation Frequency Data Server (PFDS)
- **API:** `https://hdsc.nws.noaa.gov/cgi-bin/hdsc/buildout.py?type=pf&units=us&series=pds&lon={lon}&lat={lat}`
- Returns JSON with frequency/duration matrix
- **Coverage:** Contiguous US + AK + HI + Puerto Rico (volumes 1–11; some western regions pre-date Atlas 14 → use Atlas 2)
- **License:** Public domain
- **No bulk download needed:** per-point API sufficient for 13,657 cluster centroids

### Integration
- Call PFDS API once per cluster (rate-limit: 1 req/s, ~3.8 hours for all clusters — run async)
- Cache JSON response per cluster in `work/atlas14-cache/`
- Extract 24-hr 100-year PF value (civil design standard): add `precip_100yr_24hr_in` field
- Phase 17: store 10-yr and 100-yr 24-hr; Phase 18: add 500-yr for critical facilities
- Note: Atlas 14 does not cover Canada or Europe → ECCC IDF for CA, ECA&D for EU (Phase 18)

---

## Layer 4 — Frost Depth / Air Freezing Index (AFI)

### What it shows
Maximum seasonal soil frost penetration depth (inches) — drives foundation minimum depth per IBC 1809.5 and IRC Table R301.2(1). AFI (degree-days below 32°F) is the engineering calculation basis.

### Data Sources

**AFI (derivable, public domain):**
- NOAA Normals `TMIN` and `TMAX` → compute degree-days below 32°F
- Or: NRCS published AFI grids (older, but pre-computed)

**Frost depth (SSURGO via NRCS SDA API):**
- **URL:** `https://sdmdataaccess.nrcs.usda.gov/Tabular/post.rest`
- SSURGO map unit component table: `breindbdep_l` (restrictive layer upper depth)
- Alternative: `freezedepth` soil climate table where available
- **Coverage:** Conterminous US, ~36% Alaska; Hawaii no frost
- **License:** Public domain (USDA)

### Integration
- Query SDA REST API per cluster with spatial join to map unit polygon
- Fall back to AFI-derived estimate where SSURGO coverage absent
- Add `frost_depth_in` (median SSURGO) and `afi` (NOAA normals derivation) fields
- BentoBox: "Design Frost Depth: 36 in (SSURGO / NRCS)"

---

## Layer 5 — Prevailing Wind / Wind Rose

### What it shows
Monthly and annual prevailing wind direction and speed distribution. Used by mechanical engineers (stack effect, natural ventilation), landscape architects (windbreak planting), and civil engineers (snow drift loading, erosion). ASHRAE 90.1 Appendix C references prevailing wind for natural ventilation compliance paths.

### Data Source
- **ERA5 Hourly Reanalysis** — U and V components of wind at 10 m
- **Provider:** Copernicus Climate Change Service (C3S) / ECMWF
- **API:** CDS API (`cdsapi` Python package), 0.25° × 0.25° grid
- **License:** Copernicus License v2 — free for commercial use with attribution
- **Coverage:** Global, 1940–present
- **Volume:** Monthly means at 0.25°: ~10 GB/decade; cluster-point extraction is trivial

### Integration
- Extract 10-year monthly means (U10, V10) at each cluster centroid via CDS API
- Derive: prevailing direction (most frequent 30° bin), mean speed, max gust percentile
- Add `wind_prevailing_deg`, `wind_mean_ms`, `wind_p95_ms` to clusters-meta.json
- Phase 18: generate full wind rose histograms (16-bin) stored as JSON arrays for chart rendering

---

## Layer 6 — Hail Risk

### What it shows
Historical hail event frequency and maximum recorded hail diameter. Drives roofing material selection (Class 4 IR shingles, TPO membrane spec), skylight glazing spec, HVAC condensing unit protection, and insurance pricing. ICC 500 storm shelter design also references hail.

### Data Source
- **NOAA Storm Prediction Center Storm Events Database**
- **URL:** `https://www.spc.noaa.gov/wcm/#data` → `storm_data_*.csv` (annual)
- Filter: `EVENT_TYPE = "Hail"`, `MAGNITUDE >= 1.0 inch` (F1+ threshold)
- **Coverage:** 1950–2024; quality improves post-1990 (use 1990–2024 window)
- **License:** Public domain (NOAA)
- **Size:** ~2 GB total all years; filtered hail records ~80 MB

### Integration
- Download 1990–2024 Storm Events CSVs
- Kernel density estimation (scipy.stats.gaussian_kde) on hail event centroids
- Normalize to events/decade/100km² — add `hail_events_per_decade` and `hail_max_in` fields
- Note: European ESWD hail data has restrictive license — defer EU to Phase 18+ negotiation

---

## Layer 7 — Tornado Risk

### What it shows
Probability of F1+ tornado strike per year (per unit area). Directly referenced by ICC 500 (community storm shelters), FEMA P-361 (safe rooms), and increasingly by insurance underwriters for SFRC / hardened construction requirements.

### Data Source
- **NOAA SPC Tornado Probability Maps** — pre-computed annual probability grids
- **URL:** `https://www.spc.noaa.gov/new/research/tornado_risk/` (GeoTIFF rasters)
- F1+, F2+, F3+ annual probability at 80 km² grid cells
- **License:** Public domain
- **Size:** ~5 MB per raster

**Supplementary:**
- NOAA SPC Tornado Tracks shapefile (1950–2024): `https://www.spc.noaa.gov/gis/svrgis/`
- Enables track-density KDE as quality cross-check

### Integration
- Download F1+ annual probability GeoTIFF
- Sample raster value at each cluster centroid (rasterio point_query)
- Add `tornado_prob_f1` (annual probability, 0–1 float) to clusters-meta.json
- BentoBox: "Tornado Risk: 0.12% / year (F1+)" with qualitative band (Low/Moderate/High/Extreme)

---

## Layer 8 — Hurricane Track Density

### What it shows
Historical hurricane and tropical storm track frequency (tracks/100 km²/decade). Relevant to wind load design (ASCE 7-22 Chapter 26 wind speed maps already encode this) but useful for explicit client communication on catastrophe risk, insurance costs, and hardened envelope specification (PGT, Simonton impact windows, etc.).

### Data Source
- **NOAA IBTrACS** (International Best Track Archive for Climate Stewardship)
- **URL:** `https://www.ncei.noaa.gov/products/international-best-track-archive`
- Files: `IBTrACS.NA.list.v04r01.csv` (North Atlantic) + `IBTrACS.EP.list.v04r01.csv` (East Pacific, covers Pacific Mexico)
- **License:** Public domain
- **Size:** ~15 MB for NA basin, 1842–2024

### Integration
- Filter: `USA_WIND >= 34 kt` (tropical storm+), `SEASON >= 1970` (satellite era)
- Kernel density on track point locations (50 km bandwidth)
- Add `hurricane_track_density` (tracks/decade/100km²) to clusters-meta.json
- Coverage note: applies to US Gulf/Atlantic coast, Pacific Mexico, Puerto Rico
- Midwest/interior clusters will naturally receive near-zero values

---

## Layer 9 — Atmospheric Corrosivity

### What it shows
ISO 9223 corrosivity category (C1–C5) — the rate at which unprotected steel and zinc corrode, driven by airborne chloride (marine) and sulfur dioxide (industrial) deposition. Determines structural steel coating spec, fastener material (SS vs. hot-dip galvanized), and facade cladding system selection.

### Data Sources

**Phase 17 proxy (fast, open):**
- Coastal proximity: NOAA GSHHS coastline + GDAL proximity raster
- Industrial SO₂: EPA NEI (National Emissions Inventory) facility point data
- Combined: distance-to-coast + downwind-of-facility composite score → rough C1–C4 proxy
- Accuracy: adequate for high-level site screening; not per-ISO-certified

**Phase 18 (rigorous):**
- **MICAT Project** (European ISO 9223 corrosivity map): covers EU; license unclear
- **ISO CORRAG network** data: station-based; proprietary
- **NACE / AMPP** corrosion maps: commercial

### Integration
- Phase 17: compute coastal_dist_km per cluster (GDAL proximity), bin to C1/C2/C3 categories
- Add `corrosivity_iso9223_proxy` and `coastal_dist_km` fields
- Flag as "approximate" in BentoBox — this is a screening tool, not an engineered assessment
- Phase 18: replace with gridded ISO 9223 data if open source emerges

---

## BentoBox "Weather & Climate" Section Layout

Proposed panel layout for the cluster detail BentoBox:

```
┌─────────────────────────────────────────────────────┐
│  WEATHER & CLIMATE                                   │
├─────────────────┬───────────────────────────────────┤
│ HDD / CDD       │  4,820 / 1,340  (NOAA 2006–2020) │
│ ASHRAE 99% DB   │  14°F winter design                │
│ ASHRAE 1% DB    │  94°F summer design / 76°F WB      │
├─────────────────┴───────────────────────────────────┤
│ 100-yr 24-hr Rain   3.2 in  (NOAA Atlas 14)        │
│ Design Frost Depth  36 in   (NRCS SSURGO)           │
│ Prevailing Wind     SW  9.2 mph annual mean (ERA5)  │
├─────────────────────────────────────────────────────┤
│ HAZARD                                               │
│ Tornado Risk   Moderate  0.12%/yr F1+ (NOAA SPC)   │
│ Hail Events    1.8 / decade ≥ 1" (NOAA SPC)        │
│ Hurricane      Low (0.02 tracks/decade, IBTrACS)    │
│ Corrosivity    C2 Moderate (coastal proxy)           │
└─────────────────────────────────────────────────────┘
```

---

## clusters-meta.json Field Schema

New fields added per cluster (~350 bytes × 13,657 = ~4.8 MB addition):

```json
{
  "hdd": 4820,
  "cdd": 1340,
  "hdd_approx": false,
  "ashrae_hdb99": 14.2,
  "ashrae_hdb996": 11.0,
  "ashrae_cdb1": 94.0,
  "ashrae_mcwb1": 76.2,
  "precip_100yr_24hr_in": 3.2,
  "precip_10yr_24hr_in": 2.1,
  "frost_depth_in": 36,
  "afi": 580,
  "wind_prevailing_deg": 225,
  "wind_mean_ms": 4.1,
  "wind_p95_ms": 9.8,
  "hail_events_per_decade": 1.8,
  "hail_max_in": 2.0,
  "tornado_prob_f1": 0.0012,
  "hurricane_track_density": 0.02,
  "corrosivity_proxy": "C2",
  "coastal_dist_km": 38.4
}
```

---

## Legal / Attribution Table

| Source | Attribution required |
|---|---|
| NOAA NCEI Normals | "NOAA National Centers for Environmental Information" |
| DOE EnergyPlus EPW | "U.S. Department of Energy" |
| NOAA PFDS Atlas 14 | "NOAA Hydrometeorological Design Studies Center" |
| NRCS SSURGO | "USDA Natural Resources Conservation Service" |
| ERA5 | "Copernicus Climate Change Service (C3S) / ECMWF" |
| NOAA SPC Storm Events | "NOAA Storm Prediction Center" |
| NOAA IBTrACS | "NOAA National Centers for Environmental Information" |
| NOAA GSHHS | "NOAA / NGDC" |

---

## Risk Items

| Risk | Mitigation |
|---|---|
| PFDS API rate limit | Async with 1 req/s throttle; cache responses |
| ERA5 CDS API registration required | Register at climate.copernicus.eu before Phase 17 |
| SSURGO coverage gaps (~10%) | Fall back to AFI-derived frost depth estimate |
| Atlas 14 western US volumes (1,2,3) older vintage | Flag vintage in BentoBox tooltip |
| Hail KDE smoothing over orographic terrain | Use county-level floor for mountainous clusters |
| Hurricane: inland clusters get near-zero (correct) | No action needed — by design |
| Corrosivity proxy inaccuracy near industrial sites | Label as "screening estimate" in UI |

---

## Phase Plan

| Phase | Deliverables | Estimated effort |
|---|---|---|
| **17 (US)** | HDD/CDD, ASHRAE DBs, Atlas 14 100-yr, Tornado prob, Frost depth, Hail events | 2–3 sessions |
| **17 (US)** | Prevailing wind (ERA5), Hurricane density, Corrosivity proxy | 1 session |
| **18 (CA/MX/EU)** | ERA5 global for HDD/CDD, ASHRAE EPWs global, ECCC IDF (CA), Corrosivity EU | 2 sessions |
| **18** | Wind rose full histograms → JSON chart data | 1 session |
| **19** | ISO 9223 rigorous if open data source identified | TBD |
