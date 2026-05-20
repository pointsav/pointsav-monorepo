# AEC Infrastructure, Sustainability, and Social Layers — US-First Scoping Report
> **Created:** 2026-05-20 | **Scope:** US-first; integration path to PMTiles served alongside existing cluster/catchment/mobility layers
> **Companion:** AEC-LAYERS-RESEARCH.md (master plan), AEC-REGULATORY-LAYERS-RESEARCH.md, AEC-WEATHER-LAYERS-RESEARCH.md

---

## Summary Table

| Layer | Source Agency | Dataset Name | License | Tier | Complexity |
|---|---|---|---|---|---|
| Transmission lines | HIFLD/EIA | Electric Power Transmission Lines | Public domain | T1 | Low — direct GeoJSON download |
| Electric substations | HIFLD | Electric Substations | Public domain | T1 | Low — direct GeoJSON download |
| Power plants | HIFLD/EIA | Power Plants | Public domain | T1 | Low — direct GeoJSON download |
| Utility service territories | EIA | Electric Retail Service Territories | Public domain | T1 | Low — GeoJSON, polygon |
| eGRID carbon intensity | EPA | eGRID 2023 Subregion Shapefiles | Public domain | T2 | Low — polygon, ~24 subregions |
| Water system service areas | EPA | CWS Service Area Boundaries | Public domain | T1 | Medium — modeled gaps, join required |
| Water treatment plants | HIFLD | Water Treatment Plants | Public domain | T1 | Low — point dataset |
| Wastewater capacity | EPA | CWNS 2022 Sewersheds | Public domain | T2 | Medium — ML-estimated sewersheds |
| Broadband availability | FCC | National Broadband Map BDC | Public domain | T1 | Medium — H3 hex + state CSV chunks |
| Transit feeds | MobilityDatabase | GTFS feeds (6,000+ agencies) | Varies (mostly open) | T2 | High — per-agency GTFS, no single national file |
| Highway network | FHWA | HPMS (Federal-Aid System) | Public domain | T2 | Medium — state-by-state shapefile |
| Freight network | BTS/NTAD | National Rail + Intermodal Network | Public domain | T3 | Medium — NTAD geodatabase |
| NLCD land cover | MRLC/USGS | NLCD 2021 (30m GeoTIFF) | Public domain | T1 | High — ~3–8 GB CONUS raster, tile conversion needed |
| Impervious surface | MRLC/USGS | NLCD 2021 Impervious Surface | Public domain | T1 | High — same as land cover |
| Tree canopy | MRLC/USGS | NLCD 2021 Tree Canopy | Public domain | T2 | High — raster, tile conversion needed |
| Lidar/DEM 10m | USGS | 3DEP 10m DEM (COG/GeoTIFF) | Public domain | T2 | High — large file, COG available |
| Lidar/DEM 1m | USGS | 3DEP 1m DEM (COG/GeoTIFF) | Public domain | T3 | High — patchy coverage, very large tiles |
| Urban heat island | USGS | Annual SUHI Intensity 1985–present | Public domain | T2 | Medium — raster; CONUS regions, COG |
| PM2.5 concentration | WashU/EPA | SatPM2.5 / AQS Design Values | Public domain | T2 | Medium — raster (0.01°), GIS derivation required |
| AirNow stations | EPA | AirNow Monitoring Sites (live) | Public domain | T3 | Low — point feature service, ArcGIS Hub |
| EJSCREEN | EPA | EJScreen v2.3 (block group) | Public domain | T1 | Low — GDB/CSV download, 12 EJ indicators |
| CDC SVI | CDC/ATSDR | SVI 2022 (census tract) | Public domain | T1 | Low — shapefile download, 16 variables |
| Opportunity Zones | HUD | Qualified Opportunity Zones | Public domain | T1 | Low — GeoJSON/shapefile, 8,764 tracts |
| DOT Equity Index | USDOT | ETC Explorer (census tract) | Public domain | T2 | Low — ArcGIS Hub GeoJSON |
| Food desert index | USDA ERS | Food Access Research Atlas | Public domain | T2 | Medium — tabular join to Census tracts |
| Building footprints | Microsoft | USBuildingFootprints (~130M) | ODbL | T1 | Medium — per-state GeoJSON; PMTiles via VIDA |
| Building footprints (global) | Overture Maps | Buildings theme (2.6B) | ODbL | T1 | Low — PMTiles available on S3 |
| Brownfields | EPA | ACRES Brownfields Properties | Public domain | T2 | Low — point shapefile/GeoJSON |
| USDA rural eligible areas | USDA RD | Rural Development Eligibility | Public domain | T2 | Low — polygon, web service |
| EDA distressed communities | EIG / EDA | Distressed Communities Index | CC BY | T2 | Low — CSV join to ZCTA shapefile |
| Hospitals | HIFLD | Hospitals (point) | Public domain | T1 | Low — direct download |
| Schools | NCES EDGE | Public School Locations 2024–25 | Public domain | T1 | Low — shapefile, ArcGIS Hub |

---

## 1. Electrical Infrastructure

### 1.1 HIFLD Electric Power Transmission Lines

**What it shows / AEC relevance:** All transmission lines 69kV and above in the contiguous US. Essential for large-format retail, industrial, and data-center site selection — shows proximity to high-capacity power corridors and identifies last-mile distribution challenges.

**Source:** HIFLD / EIA Energy Atlas
**URLs:**
- Primary: https://hifld-geoplatform.hub.arcgis.com/datasets/geoplatform::transmission-lines-1
- Mirror: https://atlas.eia.gov/datasets/geoplatform::transmission-lines

**License:** Public domain (DHS/EIA)
**Format:** GeoJSON, Shapefile, KML, CSV, GeoPackage, PMTiles — all available directly from ArcGIS Hub export
**US Coverage:** National, CONUS + AK/HI; some gaps in privately-reported segments
**Update frequency:** Annually (EIA cycle)
**File size:** ~50–80 MB as GeoJSON (line features)
**Download method:** Direct bulk download from HIFLD Hub (no API key required)
**PMTiles:** Available natively from HIFLD Hub export
**Integration complexity:** LOW — download GeoJSON, serve as PMTiles; style by voltage class (69kV / 115kV / 230kV / 345kV / 500kV / 765kV)
**Known quality issues:** Some line routing is schematic rather than exact. Voltage attributes sometimes missing for distribution-tier lines.
**Priority: Tier 1**

---

### 1.2 HIFLD Electric Substations

**What it shows / AEC relevance:** Point locations of all electric substations nationally. Used to identify transformer capacity proximity and switchgear locations. Critical for EV charging buildout, large-anchor tenant power planning.

**Source:** HIFLD
**URL:** https://hifld-geoplatform.hub.arcgis.com/datasets/755e8c8ae15a4c9abfceca7b2e95fb9a_0
**License:** Public domain
**Format:** GeoJSON, Shapefile, CSV, PMTiles
**US Coverage:** National; ~70,000+ point records
**Update frequency:** Annually
**Integration complexity:** LOW — point layer; cluster at low zoom, show on click at z12+
**Priority: Tier 1**

---

### 1.3 HIFLD Power Plants

**Source:** HIFLD / EIA
**URL:** https://atlas.eia.gov (search "Power Plants") / HIFLD Hub
**License:** Public domain
**Notes:** ~11,000 generating units; fuel type, nameplate capacity (MW), operator attributes. Useful for grid mix context and carbon intensity attribution.
**Integration complexity:** LOW — point layer
**Priority: Tier 1** (ship with transmission lines as a set)

---

### 1.4 EIA Electric Retail Service Territories (Utility Boundaries)

**What it shows / AEC relevance:** Polygon boundaries showing which electric utility serves each parcel. Critical for permitting — hook-up fees, interconnection standards, and DR program eligibility all depend on which IOU/co-op/municipality the site falls under.

**Source:** EIA / HIFLD (derived from Form EIA-861 annual filing)
**URLs:**
- https://atlas.eia.gov/datasets/f4cd55044b924fed9bc8b64022966097
- https://hifld-geoplatform.opendata.arcgis.com/datasets/electric-retail-service-territories
- Supplemental (nested hierarchy): https://github.com/IMMM-SFA/electricity_entity_boundaries

**License:** Public domain
**Format:** GeoJSON, Shapefile, KML; WMS/WFS available
**US Coverage:** National; ~3,300 utility service territories
**Update frequency:** Annually (mirrors EIA-861 cycle)
**File size:** ~30–50 MB as GeoJSON
**Integration complexity:** LOW-MEDIUM — polygons are complex in some metro areas; dissolve/simplify for display, keep utility name + EIA ID attributes
**Priority: Tier 1**

---

### 1.5 EPA eGRID Carbon Grid Intensity

**What it shows / AEC relevance:** Emission rates (lbs CO2/MWh) by eGRID subregion and balancing authority. Required for LEED Energy + Atmosphere credits, ASHRAE 189.1 compliance, and corporate Scope 2 emissions reporting for retail tenants.

**Source:** EPA
**URL:** https://www.epa.gov/egrid/egrid-mapping-files
**License:** Public domain
**Format:** Shapefile (subregion polygons); ~24 subregions nationally
**Most recent data:** eGRID2023 (released January 2025)
**File size:** Small — <5 MB total shapefile
**Integration complexity:** LOW — simple polygon layer; join tabular eGRID data to shapefile by subregion code; display annual CO2 lbs/MWh as choropleth
**Priority: Tier 2**

---

## 2. Water and Sewer

### 2.1 EPA Community Water System Service Area Boundaries

**What it shows / AEC relevance:** Polygon boundaries for ~50,000 community water systems. Go/no-go infrastructure layer for any ground-up development — hook-up feasibility and fee zones set by water system boundary.

**Source:** EPA Office of Water (Version 3, released March 2026)
**URL:** https://www.epa.gov/ground-water-and-drinking-water/community-water-system-service-area-boundaries
**Download:** https://www.epa.gov/waterdata/waters-geospatial-data-downloads
**License:** Public domain
**Format:** File geodatabase / GeoJSON; also ESRI web service
**US Coverage:** National; combines state-submitted boundaries + EPA-modeled infill where states haven't published. Some states (CA, TX) have high-quality official data; rural areas may be modeled approximations.
**Update frequency:** Annually; modeled boundaries updated as state data improves
**Key attribute:** PWSID — links to SDWIS for water quality, violations, population served
**Integration complexity:** MEDIUM — coordinate system conversion needed; modeled vs. official source distinction should be surfaced to users; some boundary topology gaps at state borders
**Known quality issues:** Modeled boundaries (30–40% of systems) are approximations derived from census block allocation, not legal service area maps.
**Priority: Tier 1**

---

### 2.2 HIFLD Water Treatment Plants

**Source:** HIFLD
**URL:** https://hifld-geoplatform.hub.arcgis.com (search "Water Treatment Plants")
**License:** Public domain
**Notes:** ~17,000+ point records; owner type, capacity (MGD), treatment type attributes.
**Integration complexity:** LOW — point layer
**Priority: Tier 1**

---

### 2.3 EPA CWNS 2022 Sewersheds (Wastewater Treatment Capacity)

**What it shows / AEC relevance:** Geographic catchment areas (sewersheds) for 17,544 POTWs, with capacity and population served. Critical for industrial/commercial sewer availability analysis; capacity constraints are a common development killer.

**Source:** EPA Clean Watersheds Needs Survey 2022
**URL:** https://www.epa.gov/cwns/clean-watersheds-needs-survey-cwns-2022-report-and-data
**ArcGIS:** https://www.arcgis.com/home/item.html?id=54e4cd333df54eff9c9133100f165f72
**License:** Public domain
**Notes:** First national sewershed dataset — combines official submissions with ML-estimated boundaries. Coverage: 270M people / 17,544 facilities.
**Update frequency:** Every 4 years (CWNS cycle); next ~2026
**Integration complexity:** MEDIUM — ML-estimated sewersheds need flagging; join to CWNS tabular data for capacity fields
**Priority: Tier 2**

---

## 3. Broadband / Telecom

### 3.1 FCC National Broadband Map — Fixed Availability

**What it shows / AEC relevance:** ISP-reported availability at the location level. Shows which addresses have access to speeds ≥25/3 Mbps, ≥100/20 Mbps, ≥1 Gbps. Critical for data center, medical office, and call-center siting; required for BEAD program grant eligibility analysis.

**Source:** FCC Broadband Data Collection
**URL:** https://broadbandmap.fcc.gov/data-download
**License:** Public domain (BDC data); Fabric requires free license agreement
**Format:**
- Fixed broadband: CSV by state + technology type
- Mobile coverage: ESRI Shapefile or GeoPackage with H3 resolution-9 hexagons
- API spec: https://www.fcc.gov/sites/default/files/bdc-public-data-api-spec.pdf

**US Coverage:** National; 50 states + territories; June 2025 Fabric update current
**Update frequency:** Semi-annual (BDC filing cycle: June + December)
**File size:** Very large — full national fixed availability is multiple GB across state CSVs
**PMTiles:** No official PMTiles; ArcGIS has a June 2024 aggregated layer; for self-serve: aggregate fixed CSVs to H3 res-8 and generate PMTiles
**Integration complexity:** MEDIUM-HIGH — no single national download; per-state assembly; H3 aggregation needed for map display at low zoom
**Known quality issues:** ISP-reported data is self-certified; overclaiming is a documented problem
**Priority: Tier 1**

---

## 4. Transportation / Multimodal

### 4.1 GTFS Transit Feeds

**What it shows / AEC relevance:** Bus/rail/ferry stop locations, routes, headways, and service frequency. Workers-access analysis, TDM plan requirements, TOD bonus eligibility, and parking demand modeling.

**Source:** MobilityDatabase (successor to TransitFeeds, launched February 2024)
**URL:** https://mobilitydatabase.org/feeds
**License:** Varies by agency — most are open; most require attribution
**Format:** GTFS ZIP bundles; API for discovery
**US Coverage:** 1,000+ US agencies; no single national file
**Update frequency:** Daily feed polls
**Supplemental:** FTA NTD 2023 GTFS weblinks at https://catalog.data.gov/dataset/2023-ntd-annual-data-general-transit-feed-specification-gtfs-weblinks
**Integration complexity:** HIGH — no single national file; feed URLs change; agencies with no GTFS have no data
**Practical approach for v1:** Ingest the ~400 largest US agencies (covering ~90% of ridership) via FTA NTD list; generate stops + routes PMTiles statically; refresh quarterly
**Priority: Tier 2**

---

### 4.2 FHWA Highway Performance Monitoring System (HPMS)

**What it shows / AEC relevance:** Road classification (Interstate/NHS/Principal Arterial/etc.), AADT (annual average daily traffic), number of lanes, pavement type. Essential for traffic impact study context, TDM assessment, and access road classification.

**Source:** FHWA
**URLs:**
- Shapefiles: https://www.fhwa.dot.gov/policyinformation/hpms/shapefiles.cfm
- BTS Geospatial: https://geodata.bts.gov/datasets/c199f2799b724ffbacf4cafe3ee03e55

**License:** Public domain
**Format:** Shapefile by state; also national GeoJSON via BTS Geospatial Catalog
**US Coverage:** Federal-Aid highway system nationally; non-federal roads excluded
**Update frequency:** Annual (states submit to FHWA); 2022 is most current public shapefile
**File size:** National assembled ~500 MB shapefile
**Integration complexity:** MEDIUM — state-by-state assembly; coordinate system normalization; large line dataset benefits from attribute filtering before tiling
**Priority: Tier 2**

---

### 4.3 BTS National Transportation Atlas Database (NTAD) — Freight

**What it shows / AEC relevance:** Class I freight rail lines (BNSF, UP, CSX, NS, CN, CPKC), intermodal terminals, ports. Required for distribution center and industrial siting analysis; proximity to Class I rail is a top criterion for industrial tenants.

**Source:** Bureau of Transportation Statistics
**URL:** https://www.bts.gov/ntad (quarterly updates; Spring 2025 current)
**License:** Public domain
**Format:** File Geodatabase, Shapefile, GeoJSON, KML, CSV — all formats available
**Download:** https://geodata.bts.gov — direct catalog download
**Integration complexity:** MEDIUM — multi-layer geodatabase; extract rail + intermodal layers specifically; GDB requires ogr2ogr for non-ESRI workflows
**Priority: Tier 3**

---

### 4.4 Walk Score / Bike Score API

**Source:** Walk Score (www.walkscore.com)
**License:** Commercial API — requires license for business use; free tier with rate limits
**Format:** JSON API response per lat/lon (no bulk tile download)
**Notes:** Returns Walk Score (0–100), Transit Score, and Bike Score for any coordinate. Not bulk-downloadable — must be queried point-by-point and cached. Alternative: OpenStreetMap-derived walkability scoring (free but less calibrated).
**Integration complexity:** MEDIUM — API call per cluster centroid; cache results; no PMTiles path
**Priority: Tier 3**

---

## 5. Environmental / Climate

### 5.1 NLCD 2021 Land Cover

**What it shows / AEC relevance:** 30m raster land cover classification for CONUS (21 classes). Foundation for stormwater runoff models (TR-55, SWMM), impervious surface area calculations, vegetation clearing estimates, and LEED site assessment.

**Source:** MRLC Consortium (USGS/EPA/USFS/NOAA)
**URL:** https://www.mrlc.gov/data
**License:** Public domain (US government, unrestricted)
**Format:** Cloud-Optimized GeoTIFF (COG); also via Google Earth Engine (USGS/NLCD_RELEASES/2021_REL/NLCD)
**File size:** CONUS national GeoTIFF: approximately 3–8 GB per product
**US Coverage:** CONUS; separate AK product; HI limited
**Update frequency:** NLCD releases every 2–3 years; 2021 released 2023; Annual NLCD collection now available for 2016–2023
**FTP alternate:** https://gaftp.epa.gov/aqmg/nlcd/
**PMTiles path:** Reclassify → vectorize (gdal_polygonize) → simplify (mapshaper) → tippecanoe → PMTiles; or serve COG via TiTiler
**Integration complexity:** HIGH — raster to PMTiles pipeline; full CONUS pipeline ~4–8 hours on typical VM
**Priority: Tier 1** (simplified 8-class version; ship impervious surface first)

---

### 5.2 NLCD 2021 Impervious Surface

Same download location as land cover (§5.1). Continuous 0–100% raster at 30m. For stormwater modeling more precise than binary classification. Separate download (~3 GB GeoTIFF).

**Priority: Tier 1** (ship with land cover; raster serve via TiTiler or derive polygons at >30% impervious threshold)

---

### 5.3 NLCD Tree Canopy Cover

Same MRLC source. 30m raster, 0–100% canopy cover. Used for urban heat island mitigation analysis, LEED Sustainable Sites credits, and tree preservation ordinance compliance assessment.

**Priority: Tier 2**

---

### 5.4 USGS 3DEP Digital Elevation Models

**What it shows / AEC relevance:** Terrain elevation for site grading analysis, stormwater drainage basin delineation, floodplain context, and cut/fill estimates.

**Source:** USGS 3D Elevation Program
**URLs:**
- 1m DEM: https://data.usgs.gov/datacatalog/data/USGS:77ae0551-c61e-4979-aedd-d797abdcde0e
- 10m DEM: https://data.usgs.gov/datacatalog/data/USGS:4f34caac-f28f-4ea0-8d82-eafb2b8f9a5d
- AWS Open Data (lidar): https://registry.opendata.aws/usgs-lidar/
- LidarExplorer: https://apps.nationalmap.gov/lidar-explorer/

**License:** Public domain
**Format:** COG GeoTIFF (migrated 2020); lidar point clouds as LAZ
**US Coverage:** 1m — approximately 60% of CONUS as of 2024; expanding annually. 10m — complete national coverage.
**PMTiles:** Not directly applicable (raster elevation); serve via TiTiler as hillshade COG for display; extract contours at 2m interval for vector tiles
**Integration complexity:** HIGH for engineering use; MEDIUM for display (hillshade layer from 10m COG via TiTiler is straightforward)
**Priority: Tier 2** (10m hillshade as base context; Tier 3 for 1m engineering-grade)

---

### 5.5 Urban Heat Island — USGS Annual SUHI Intensity

**What it shows / AEC relevance:** Surface Urban Heat Island (SUHI) intensity: difference between urban LST and surrounding non-urban LST, annually from 1985 to 2020+ for ~50 US regions. Informs cool-roof requirements, shade design, and LEED Heat Island Reduction credits.

**Source:** USGS
**URL:** https://data.usgs.gov/datacatalog/data/USGS:656e232fd34e7ca10833f968
**License:** Public domain
**Format:** GeoTIFF raster by region/year
**Supplemental:** Landsat Collection 2 Surface Temperature: https://www.usgs.gov/landsat-missions/landsat-collection-2-surface-temperature (national, 30m)
**Supplemental:** NASA MODIS LST at 1km monthly resolution: https://neo.gsfc.nasa.gov/view.php?datasetId=MOD_LSTD_M
**Integration complexity:** MEDIUM — pre-packaged SUHI rasters need georeferencing and color ramp
**Priority: Tier 2**

---

### 5.6 EPA / WashU PM2.5 Concentration (5-Year Average Raster)

**What it shows / AEC relevance:** Ambient fine particulate matter exposure. Key LEED v4 EQ prerequisite and WELL Building Standard indicator. Used in environmental justice reviews and health impact assessments.

**Sources:**
1. **EPA PM2.5 Annual Design Values 2013–2022** (ArcGIS layer, monitor-based): https://www.arcgis.com/home/item.html?id=1412feebf14e4413a4a595afb9ef3a03
2. **Washington University SatPM2.5** (best for raster): https://sites.wustl.edu/acag/datasets/surface-pm2-5/ — satellite-derived, 0.01° × 0.01° (~1km), North America, 2000–2023, CC BY
3. **Global GHAP** (alternative, GEE): https://gee-community-catalog.org/projects/ghap/ — 1km, 2017–2022

**Recommended dataset:** WashU SatPM2.5 — highest resolution, freely downloadable as NetCDF, R code provided for GeoTIFF conversion
**License:** CC BY (WashU); public domain (EPA monitor data)
**File size:** NetCDF annual files ~50–200 MB; compute 5-year average in GDAL/Python
**Integration complexity:** MEDIUM — NetCDF → GeoTIFF → COG via gdal_translate; serve via TiTiler; color ramp to WHO AQG categories (5/10/15/25 µg/m³)
**Priority: Tier 2**

---

### 5.7 EPA AirNow Monitoring Stations

**Source:** EPA / AirNow
**URLs:**
- ArcGIS Hub live feature service: https://www.arcgis.com/home/item.html?id=2d718d2733a74d1689d72b922c0ac4f4
- AQS static monitor list: https://hub.arcgis.com/maps/EPA::u-s-epa-oaqps-airdata-air-quality-monitors-us
**Notes:** ~5,000 US monitoring stations; AQI index, pollutant type, status. Better as a click-through information layer than a spatial analysis layer.
**Priority: Tier 3**

---

### 5.8 EPA EJSCREEN v2.3

**What it shows / AEC relevance:** 12 environmental burden indicators + 6 socioeconomic indicators at census block group level. Required for LEED v4.1 Community Impact category, DOT Justice40 compliance documentation, and community benefit agreement scoping.

**Source:** EPA
**URL (FTP):** https://gaftp.epa.gov/EJScreen/
**ArcGIS Hub (tract-level percentiles):** https://hub.arcgis.com/datasets/448f514d14204df7b4641e96a3fee52e
**License:** Public domain
**Format:** File geodatabase (GDB) + CSV; block-group or census-tract resolution
**Most recent:** EJScreen v2.3 (July 2024)
**US Coverage:** National; ~220,000 block groups
**Update frequency:** Annually
**File size:** ~200–400 MB GDB national
**Integration complexity:** LOW-MEDIUM — join tabular to Census geometry; 12 EJ indicators color-mappable directly. Key indicators for AEC: PM2.5, air toxics, ozone, traffic proximity, wastewater discharge, RMP proximity, Superfund proximity, underground storage tanks.
**Priority: Tier 1**

---

## 6. Social and Community

### 6.1 CDC/ATSDR Social Vulnerability Index (SVI) 2022

**What it shows / AEC relevance:** 16 census-tract variables aggregated into 4 themes (socioeconomic, household composition, minority status/language, housing type/transportation) and an overall SVI score. Used for community benefit agreement scoping, FEMA resilience grant applications, and LEED Social Equity pilot credits.

**Source:** CDC/ATSDR GRASP program
**URL:** https://atsdr.cdc.gov/place-health/php/svi/svi-data-documentation-download.html
**ArcGIS Hub:** https://hub.arcgis.com/maps/414c0b43a0ec4adc829d5815bc621750
**License:** Public domain
**Format:** Shapefile + CSV; also ArcGIS Online feature layer
**Most recent:** SVI 2022 (released May 2024); 2000/2010/2014/2016/2018/2020 archives available
**New in 2022:** ZCTA-level dataset added in addition to census tract
**US Coverage:** All census tracts nationally
**Update frequency:** Every 2 years (aligned with ACS)
**File size:** ~300 MB national shapefile
**Integration complexity:** LOW — well-documented, ready-to-use shapefile; four aggregate theme scores immediately usable for color ramps
**Priority: Tier 1**

---

### 6.2 HUD Qualified Opportunity Zones

**What it shows / AEC relevance:** 8,764 census tracts designated as Opportunity Zones under the 2017 Tax Cuts and Jobs Act. Investors in QOZ-eligible projects receive capital gains tax deferrals and reductions. Essential for deal feasibility screening.

**Source:** HUD / IRS
**URLs:**
- HUD Open Data: https://hudgis-hud.opendata.arcgis.com/datasets/opportunity-zones
- Data.gov Shapefile: https://catalog.data.gov/dataset/opportunity-zones-16322
- Interactive map: https://opportunityzones.hud.gov/resources/map

**License:** Public domain
**Format:** GeoJSON, Shapefile, KML, CSV — all available from HUD Open Data hub
**US Coverage:** All 50 states + DC + territories; 8,764 census tracts
**Update frequency:** Static — OZ designations set in 2018 (unless legislation changes)
**File size:** Small — <20 MB
**Integration complexity:** LOW — binary yes/no overlay; display as semi-transparent fill at zoom ≥ 8
**Priority: Tier 1**

---

### 6.3 USDOT Equitable Transportation Community (ETC) Explorer

**What it shows / AEC relevance:** Census-tract scoring across 5 domains: transportation insecurity, climate/disaster risk, environmental burden, health vulnerability, and social vulnerability. Identifies Justice40 communities for federal grant eligibility.

**Source:** US Department of Transportation
**URL:** https://equity-data.dot.gov/datasets/usdot-equitable-transportation-community-etc-explorer
**Portal:** https://equity-data.dot.gov/
**License:** Public domain
**Format:** GeoJSON, CSV, KML from ArcGIS Hub
**Coverage:** National census tracts; based on 2020 Census
**Update frequency:** Last major update 2023
**Integration complexity:** LOW — ArcGIS Hub export; census tract polygons
**Priority: Tier 2**

---

### 6.4 USDA Food Access Research Atlas

**What it shows / AEC relevance:** Identifies census tracts qualifying as low-income + low-access food areas ("food deserts") using multiple distance thresholds (0.5/1/10/20 mi). Relevant for grocery-anchored retail feasibility, USDA grant eligibility, and community benefit documentation.

**Source:** USDA Economic Research Service
**URL:** https://www.ers.usda.gov/data-products/food-access-research-atlas/download-the-data
**License:** Public domain
**Format:** Excel/CSV (tabular); geometry requires join to Census tract boundaries
**Most current:** Based on 2019 data; not yet updated to 2020 Census tracts (known gap)
**Integration complexity:** MEDIUM — tabular join to Census tract shapefile required; 2010-era tracts vs. 2020 tracts mismatch; recommend using 2020 Census crosswalk
**Priority: Tier 2**

---

## 7. Land Cover and Terrain

See Section 5.1–5.4 for NLCD 2021 land cover, impervious surface, tree canopy, and USGS 3DEP. Those sections cover terrain fully.

---

## 8. Building Stock

### 8.1 Microsoft US Building Footprints

**What it shows / AEC relevance:** ~130M ML-detected building footprints for all 50 states. Used for site context, shadow analysis, density calculations, and identifying existing building stock for redevelopment assessment.

**Source:** Microsoft / Bing Maps
**URL:** https://github.com/microsoft/USBuildingFootprints
**License:** ODbL (Open Database License) — attribution required; compatible with commercial use
**Format:** GeoJSON per state (50 files); also GeoParquet and PMTiles via VIDA/source.coop
**US Coverage:** National; generated from Bing Maps imagery 2014–2023
**PMTiles:** YES — VIDA (source.coop) publishes a combined Google+Microsoft footprint dataset in PMTiles:
- https://source.coop/vida/google-microsoft-open-buildings — 2.5B global footprints; PMTiles per country
**Integration complexity:** MEDIUM — per-state assembly or use VIDA PMTiles directly; display at z≥12 only
**Priority: Tier 1** (PMTiles already exist; integration is straightforward)

---

### 8.2 Overture Maps Buildings

**What it shows / AEC relevance:** 2.6B global building footprints with richer attribution than Microsoft alone (height, building type, name where available from OSM merge). US coverage is complete.

**Source:** Overture Maps Foundation
**URL:** https://docs.overturemaps.org/guides/buildings/
**AWS Open Data:** https://registry.opendata.aws/overture/
**PMTiles:** https://overturemaps-tiles-us-west-2-beta.s3.amazonaws.com/ (public S3 URL; use current release tag e.g. 2024-11-13/buildings.pmtiles)
**License:** ODbL (OSM-derived) + CDLA Permissive 2.0 (Microsoft-derived)
**Format:** GeoParquet (primary); PMTiles (beta, available on S3)
**Update frequency:** Quarterly releases
**Integration complexity:** LOW — PMTiles already published; point MapLibre source at S3 URL; filter to US bbox. Height attribute available where sourced from OSM with `building:levels` tag.
**Priority: Tier 1**

---

## 9. Brownfields and Opportunity

### 9.1 EPA ACRES Brownfields Properties

**What it shows / AEC relevance:** Point locations of EPA Brownfields grant-funded assessment and cleanup sites. Phase I/II ESA context; remediation cost indicator; state brownfields tax credit eligibility.

**Source:** EPA / ACRES database
**URL:** https://catalog.data.gov/dataset/acres-brownfields-properties
**EPA FRS Geospatial (monthly):** https://www.epa.gov/frs/geospatial-data-download-service
**License:** Public domain
**Format:** Shapefile, KMZ, GeoJSON; also Cleanups in My Community geodatabase
**US Coverage:** ~26,000 brownfields properties nationally (grant-funded only; excludes all known contaminated sites)
**Update frequency:** Monthly (FRS geospatial); ACRES quarterly
**Integration complexity:** LOW — point layer; add Superfund NPL sites from FRS for completeness
**Priority: Tier 2**

---

### 9.2 EIG Distressed Communities Index (DCI)

**What it shows / AEC relevance:** Composite score for economic distress at ZIP code level across 7 indicators. Useful for EDA grant eligibility screening and community benefit analysis.

**Source:** Economic Innovation Group (non-profit research)
**URL:** https://eig.org/dci-hub/
**Upjohn alternate:** https://www.upjohn.org/major-initiatives/promise-investing-community/data-and-resources/economic-distress/distressed-communities-file-download
**License:** CC BY
**Format:** CSV by ZCTA (ZIP); join to Census ZCTA shapefile
**Integration complexity:** LOW — CSV join to ZCTA TIGER shapefile; 5-tier distress classification color-mappable directly
**Priority: Tier 2**

---

### 9.3 USDA Rural Development Eligible Areas

**What it shows / AEC relevance:** Polygon boundaries of areas eligible for USDA rural lending programs. Critical for rural healthcare, broadband, and community facilities projects with USDA financing.

**Source:** USDA Rural Development GIS Group
**URL:** https://catalog.data.gov/dataset/usda-rural-development-property-eligibility-sfh-mfh
**License:** Public domain
**Format:** Web service (ESRI MapServer); polygon download via data.gov
**US Coverage:** National; boundaries reflect 2020 Census effective FY2024
**Update frequency:** Decennial census cycle
**Integration complexity:** LOW — polygon layer; download from data.gov, convert to GeoJSON, generate PMTiles
**Priority: Tier 2**

---

## 10. Health and Schools

### 10.1 HIFLD Hospitals

**What it shows / AEC relevance:** Point locations of ~7,500+ US hospitals with bed count, trauma level, helipad, type (general/children/VA/etc.). Proximity analysis for healthcare campus siting, medical office development, and emergency access documentation.

**Source:** HIFLD
**URL:** https://hifld-geoplatform.hub.arcgis.com (search "Hospitals")
**License:** Public domain
**Format:** GeoJSON, Shapefile, CSV, PMTiles — all available from HIFLD Hub
**Update frequency:** Annually
**Integration complexity:** LOW — point layer; filter by TRAUMA level and TYPE
**Priority: Tier 1**

---

### 10.2 NCES Public School Locations

**What it shows / AEC relevance:** Point locations of all ~130,000 public K–12 schools with enrollment, grade range, locale type (urban/suburban/rural/town). School proximity is a standard criterion in residential and community development siting; required for traffic impact analysis near schools.

**Source:** NCES EDGE Program
**URL:** https://nces.ed.gov/programs/edge/geographic/schoollocations
**ArcGIS Hub:** https://data-nces.opendata.arcgis.com/datasets/nces::public-school-locations-current/about
**License:** Public domain
**Format:** Shapefile, GeoJSON, CSV — all available from ArcGIS Hub
**Vintage:** 2024–2025 school year; updated annually in fall
**US Coverage:** All 50 states + DC + territories
**File size:** ~30 MB national GeoJSON
**Integration complexity:** LOW — point layer with rich attributes; displayable immediately
**Priority: Tier 1**

---

## 11. Integration Architecture Notes

### PMTiles generation pipeline (recommended for raster-derived layers)

For NLCD, DEM, PM2.5, and urban heat island layers:
1. Download COG GeoTIFF from source
2. Reproject to EPSG:4326 via `gdalwarp -t_srs EPSG:4326`
3. For land cover: reclassify → gdal_polygonize → simplify (mapshaper) → tippecanoe → PMTiles
4. For continuous rasters (PM2.5, impervious %): serve directly via TiTiler as a raster tile service with color ramp configuration
5. Alternatively: use rio-cogeo + TiTiler deployed as FastAPI on existing GCS infrastructure

### Serving recommendation by path

| Path | Layers |
|---|---|
| Direct PMTiles download (ready-made) | Overture Buildings, VIDA Google+MS Buildings |
| HIFLD Hub direct PMTiles export | Transmission lines, substations, power plants, hospitals, water treatment plants |
| GeoJSON → tippecanoe → PMTiles (simple) | Utility territories, SVI, EJSCREEN, Opportunity Zones, NCES schools, USDA rural, DCI |
| Raster → TiTiler (serve COG) | NLCD land cover, impervious, DEM hillshade, PM2.5, urban heat island |
| Multi-source assembly required | FCC broadband (state CSVs → H3 → PMTiles), GTFS transit (per-feed assembly), HPMS (state-by-state) |

### File size budget estimate (PMTiles, after optimization)

| Layer | Estimated PMTiles size |
|---|---|
| Transmission lines | 15–25 MB |
| Utility territories | 20–40 MB |
| SVI (census tracts) | 80–120 MB |
| EJSCREEN (block groups) | 150–200 MB |
| NLCD land cover (simplified vector) | 400–700 MB |
| Buildings (US subset of Overture) | 3–8 GB (zoom-clamped) |
| FCC broadband (H3 agg) | 200–400 MB |

Total estimated tile storage for full T1 layer set: approximately 5–10 GB, consistent with existing pipeline.

---

## 12. Prioritized Rollout Plan

### Tier 1 — Ship First (high value, low complexity, data ready)

1. HIFLD Transmission Lines + Substations + Power Plants (one download session, three layers)
2. EIA Utility Service Territories (single GeoJSON download)
3. EPA CWS Water System Service Areas (national GDB → GeoJSON → PMTiles)
4. EPA EJSCREEN v2.3 (FTP download, block-group GDB → PMTiles)
5. CDC SVI 2022 (direct shapefile download → PMTiles)
6. HUD Opportunity Zones (HUD Hub GeoJSON → PMTiles)
7. HIFLD Hospitals + NCES School Locations (two point layers, trivial)
8. Overture Maps Buildings PMTiles (S3 URL, zero preprocessing)
9. NLCD 2021 impervious surface — simplified vector version
10. FCC Broadband — H3 res-8 aggregated PMTiles (medium effort but high demand)

### Tier 2 — Next Quarter

11. EPA eGRID Subregion carbon intensity shapefiles
12. EPA CWNS 2022 Sewersheds
13. USDOT ETC Explorer (census tract equity index)
14. USDA Food Access Research Atlas (tabular join to Census tracts)
15. EPA Brownfields ACRES (point layer)
16. EIG Distressed Communities Index (ZCTA join)
17. USDA Rural Eligible Areas
18. FHWA HPMS road network (state assembly → PMTiles)
19. Urban heat island raster (USGS SUHI → TiTiler)
20. WashU SatPM2.5 5-year average raster (COG → TiTiler)
21. USGS 3DEP 10m DEM hillshade (TiTiler serve)
22. GTFS transit stops + routes (top-400 agencies)

### Tier 3 — Future

23. NLCD Tree Canopy Cover
24. BTS Freight Rail + Intermodal network
25. USGS 3DEP 1m lidar (where available)
26. Walk Score API (per-cluster centroid query, not a map layer)
27. EPA AirNow station points (supplemental)
