# AEC Regulatory and Land-Use Layers — US-First Scoping Report
> **Created:** 2026-05-20 | **Scope:** US-first; integration path to PMTiles served alongside existing cluster/catchment/mobility layers
> **Companion:** AEC-LAYERS-RESEARCH.md (master plan), AEC-WEATHER-LAYERS-RESEARCH.md

---

## Summary Table

| Layer | Source | License | Tier | Complexity |
|---|---|---|---|---|
| Zoning polygons | National Zoning Atlas (NZA) | Research/non-commercial (unclear for prod) | 1 | High — partial coverage, no bulk download yet |
| Zoning + FAR/setbacks (parcel-level) | Regrid Standardized Zoning | Commercial license required | 1 | Medium — API/tiles available, paid |
| FEMA NFHL Flood Zones | FEMA / NFHL | Public domain | 1 | Medium — ~12 GB national GDB; county/state downloads; WFS available |
| NWI Wetlands | USFWS | Public domain | 1 | Medium — state GeoPackage downloads; tippecanoe → PMTiles |
| USGS Seismic (Ss/S1/PGA) | USGS Earthquake Hazards | Public domain | 1 | Low — point-query REST API; no polygon tiles needed |
| EPA Radon Zones | EPA | Public domain | 1 | Low — county-level; 3,142 features; easy tippecanoe |
| NPS Historic Register (points + polygons) | NPS / ArcGIS Hub | Public domain | 1 | Low — ArcGIS FeatureServer; GeoJSON export |
| CAL FIRE FHSZ (CA only) | CAL FIRE / OSFM | Public domain | 1 | Low — CA Geoportal GeoJSON download |
| USFS WUI 2020 | USDA Forest Service / Silvis Lab | Public domain | 1 | Medium — national GDB ~1–2 GB; tippecanoe |
| EPA EJSCREEN / Brownfields | EPA FRS | Public domain | 2 | Medium — monthly GDB; national; census block group |
| EPA Superfund SEMS sites | EPA FRS | Public domain | 2 | Low — point data; FRS GeoJSON; easy |
| USFWS Critical Habitat | USFWS / NOAA | Public domain | 2 | Medium — complex polygons; ArcGIS Hub GeoJSON |
| FAA Digital Obstacle File (DOF) | FAA AeroNav | Public domain | 2 | Low — point data; updated every 56 days |
| FAA UAS Facility Maps | FAA UDDS | Public domain | 2 | Low — ArcGIS Hub; polygon download |
| FAA Airport Noise Contours (Part 150) | Per-airport / FAA | Public domain (per airport) | 2 | High — no national dataset; per-airport only |
| BTS National Transportation Noise Map | BTS / DOT | Public domain | 2 | Medium — WMS tiles available; raster |
| USGS Landslide Susceptibility (90 m) | USGS ScienceBase | Public domain | 2 | Medium — national GeoTIFF raster; needs COG/tile conversion |
| USFS Wildfire Hazard Potential (WHP) | USDA Forest Service | Public domain | 2 | Medium — 270 m or 30 m GeoTIFF raster |
| USGS Design Maps API (ASCE 7) | USGS | Public domain | 2 | Low — point-query only; no polygon layer to tile |
| Energy code adoption (IECC/IBC year) | ICC / BCAP | Tabular, no GIS | 3 | High — no spatial download; must join to state/county boundaries |
| State/local historic districts | State SHPOs | Mixed | 3 | High — 50 separate sources; no national aggregate |
| FAR/height restrictions by parcel (national) | No national dataset | N/A | 3 | Very High — parcel data is locally held |
| HUD noise assessment zones | HUD (derived tool, not dataset) | N/A | 3 | High — no downloadable polygon layer |
| Railroad noise/vibration (FRA) | FRA / BTS | Public domain | 3 | High — modeled raster; BTS WMS only |

---

## 1. Zoning

### 1a. National Zoning Atlas (NZA) — Cornell / Land Use Atlas Inc.

**What it shows:** Digitized municipal zoning district polygons with standardized land-use classification attributes (residential density, commercial, industrial, mixed-use). Fundamental for site feasibility: allowable uses, minimum lot size, building height, FAR, density, and permitted uses by district.

**Why AEC professionals need it:** First question at any development site is "what is it zoned?" This layer answers that without downloading the local ordinance PDF.

**Data source:**
- National Zoning Atlas: https://www.zoningatlas.org/
- Mercatus Center state downloads: https://www.mercatus.org/state-and-regional-zoning-atlas-datasets
- S3 data bucket (select state GeoJSONs): https://statezoningatlasdata.s3.amazonaws.com/index.html

**Format:** GeoJSON + CSV per state; national interactive tile layer on ArcGIS (not downloadable as national file)

**US coverage:** Partial and expanding. As of 2025, completed or substantially complete states include Connecticut, New Hampshire, Vermont, Rhode Island, Massachusetts, Virginia, Colorado, Montana, Utah, Hawaii, and parts of Middle Tennessee. Texas in progress. The NZA estimates 33,000+ jurisdictions nationally; completed atlas covers roughly 4,000+ jurisdictions today. No national bulk download offered; more information on third-party data access expected in 2026.

**Update frequency:** Ad hoc as state teams complete jurisdictions; biannual improvement cycles for completed states.

**Download method:** State GeoJSON files from Mercatus Center page or S3 bucket for completed states. No national download.

**Integration complexity:** Medium-high. Each state GeoJSON can be processed with tippecanoe into PMTiles. Attribute standardization across state teams is consistent (NZA methodology enforces it). Gap: roughly 85–90% of US jurisdictions not yet in dataset. **Do not use as sole zoning data source for production.**

**File sizes:** Per-state GeoJSONs typically 5–50 MB; no national file published.

**Existing PMTiles:** None. The ArcGIS tile service exists but is not downloadable as PMTiles.

**Priority:** Tier 1 — use what exists now; supplement with Regrid below.

---

### 1b. Regrid Standardized Zoning (Commercial)

**What it shows:** Zoning district polygons with 26 standardized fields including zoning code, standardized type/subtype, setbacks (front/rear/side), FAR, lot coverage limits, and permitted use indicators. Parcel-level linkage available.

**Why AEC professionals need it:** Covers 6,000+ municipalities with spatial data, updated monthly. This is the practical national answer to the NZA coverage gap.

**Data source:**
- Regrid: https://regrid.com/zoning
- API and tile documentation: https://regrid.com/api
- AEC sector page: https://regrid.com/aec

**Format:** API (JSON), vector tile service, bulk download (licensed). PMTiles not natively offered but the tile service is MVT-compatible.

**US coverage:** 2,500+ counties; zoning boundaries covering 127.9 million parcel boundaries; 13,900+ municipalities. Not 100% of US jurisdictions — rural areas with no zoning simply absent.

**Update frequency:** Monthly.

**Download method:** Paid API or licensed bulk delivery. Enterprise pricing required for bulk zoning tiles. No open/free tier for standardized zoning product.

**Integration complexity:** Medium. If licensed, tile service can be proxied or data converted to PMTiles. The 26-field schema is the best-standardized commercially available product for AEC use.

**License:** Commercial; contact Regrid for pricing. Academic/non-profit discount available ("Data with Purpose" program).

**Priority:** Tier 1 — recommended as production-grade zoning source pending NZA national coverage.

---

### 1c. Zoning Preemption Map

**Data source:** No authoritative GIS dataset exists. BCAP, APA, and EIG track this in tabular/web form.
- BCAP: https://bcapcodes.org/code-status/
- EIG housing reform tracker: https://eig.org/

**Integration complexity:** Very high — requires manual encoding of legislative data against state polygons, updated as laws change (124 pro-housing laws enacted first half of 2025).

**Priority:** Tier 3 — editorial/tooltip content rather than a tile layer.

---

## 2. Development Restrictions

### 2a. Height Restrictions / FAR / Setbacks (Parcel Level)

No national open GIS dataset exists. These parameters are embedded in municipal zoning codes and not published as a unified national spatial layer. Regrid Standardized Zoning (§1b) is the closest commercial aggregation.

**US coverage:** Highly fragmented. Some cities publish parcel-level GIS with zoning overlays (NYC PLUTO is the gold standard; Boston, Chicago, LA have similar). No federal agency maintains this.

**Priority:** Tier 1 via Regrid (covered above); Tier 3 as a standalone national open-data layer.

---

## 3. Environmental Regulatory

### 3a. FEMA National Flood Hazard Layer (NFHL)

**What it shows:** Flood insurance rate map (FIRM) zones: AE (100-year floodplain), X (500-year), VE (coastal), floodway, etc. Foundational layer for flood risk assessment.

**Why AEC professionals need it:** NFHL determines whether NFIP flood insurance is required, triggers elevation certificates, drives foundation design (IBC Section 1612), and affects stormwater management design.

**Data source:**
- FEMA Map Service Center: https://www.fema.gov/flood-maps/national-flood-hazard-layer
- REST service: https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer
- WMS: https://hazards.fema.gov/femaportal/wps/portal/NFHLWMS
- ArcGIS Hub: https://gis-fema.hub.arcgis.com/datasets/ae38b6f94eaf4abf97f986fa01921e13/api?layer=28

**Format:** Shapefile (county/state via Map Service Center); KMZ national; REST/GeoJSON/PBF via API; WMS tile service available.

**US coverage:** Over 90% of US population. Some rural and low-risk areas lack effective DFIRMs.

**Update frequency:** Continuous; new and revised data added rolling. Latest effective study date: July 2025.

**Download method:** State or county-level shapefile via FEMA Map Service Center. National bulk download ~12 GB uncompressed. REST API supports GeoJSON and PBF queries by bounding box.

**Integration complexity:** Medium. The WMS is immediately consumable. For PMTiles: download national GDB (~12 GB), extract `S_FLD_HAZ_AR` polygon layer, run tippecanoe. Estimated 200–400 MB output PMTiles at zoom 0–14 with moderate simplification.

**Existing PMTiles:** None published nationally. FEMA ArcGIS REST service (PBF format) can be proxied as alternative.

**License:** Public domain.

**Priority:** Tier 1.

---

### 3b. USFWS National Wetlands Inventory (NWI)

**What it shows:** Wetland and deepwater habitat polygons classified by Cowardin system (palustrine, estuarine, riverine, lacustrine, marine). Critical for Section 404 Army Corps permitting.

**Why AEC professionals need it:** Any work in or adjacent to jurisdictional wetlands requires Army Corps Section 404 permitting. NWI is the first-pass screen before site delineation.

**Data source:**
- USFWS NWI: https://www.fws.gov/program/national-wetlands-inventory
- State downloads: https://www.fws.gov/program/national-wetlands-inventory/download-state-wetlands-data
- NOAA Digital Coast mirror: https://coast.noaa.gov/digitalcoast/data/nwi.html

**Format:** File Geodatabase or GeoPackage by state; Shapefile by HUC8 watershed. All compressed ZIP.

**US coverage:** National; mapped for all 50 states + territories. Rural/remote areas may have older vintage data.

**Update frequency:** Biannual — May and October each year.

**Download method:** State-by-state GeoPackage or GDB downloads from USFWS. No single national bulk file. Massachusetts ~215 MB; larger states (TX, CA) 500 MB–1 GB+ per state.

**Integration complexity:** Medium. Process each state GeoPackage with ogr2ogr to GeoJSON, then tippecanoe to PMTiles. Wetland polygons are dense in coastal/riparian areas — apply `--drop-densest-as-needed` at low zooms. National PMTiles estimated 300–800 MB depending on zoom range.

**Existing PMTiles:** None. USFWS Wetlands Mapper uses their own ArcGIS tile service.

**License:** Public domain.

**Priority:** Tier 1.

---

### 3c. EPA Superfund / SEMS / Brownfields / EJSCREEN

**What it shows:**
- **SEMS:** Point locations of all active and archived Superfund sites (NPL listed, proposed, removed, archived).
- **Brownfields:** EPA-tracked brownfield properties receiving assessment/cleanup funding.
- **EJSCREEN:** Environmental justice indicators at census block group level — 13 environmental indicators crossed with 6 demographic indicators.

**Why AEC professionals need it:** Superfund and brownfield proximity affects Phase I ESA findings, lender requirements, and remediation cost contingency. EJSCREEN is increasingly required for federal grant applications and HUD-funded development.

**Data source:**
- EPA FRS Geospatial Download: https://www.epa.gov/frs/geospatial-data-download-service
- EJSCREEN ArcGIS Hub (tract level): https://hub.arcgis.com/datasets/448f514d14204df7b4641e96a3fee52e
- EJSCREEN historical archive (Zenodo): https://zenodo.org/records/14767363

**Format:** SEMS/Brownfields — point shapefiles via FRS; updated monthly. EJSCREEN — file geodatabase (.gdb) or CSV at block group; GDB download from EPA FTP.

**US coverage:** National.

**Update frequency:** FRS points: monthly. EJSCREEN: annual (current version 2.32, updated September 2024).

**Download method:** FRS: `https://www.epa.gov/frs/geospatial-data-download-service` — prepackaged monthly GDB. EJSCREEN: EPA GAFTP FTP server at `https://gaftp.epa.gov/EJScreen/`.

**Integration complexity:** Low for Superfund/brownfield points. Medium for EJSCREEN polygons (census block group boundaries + attribute join; ~200 MB GeoJSON nationally).

**License:** Public domain.

**Priority:** Tier 2.

---

### 3d. USFWS Critical Habitat (ESA)

**What it shows:** Polygon boundaries of designated critical habitat for federally threatened and endangered species under the Endangered Species Act. Separate NOAA dataset covers marine/anadromous species.

**Why AEC professionals need it:** Any federal nexus (permit, funding, federal land) triggers Section 7 consultation. Private projects face Section 9 take prohibition issues if habitat is disturbed.

**Data source:**
- USFWS: https://gis-fws.opendata.arcgis.com/maps/794de45b9d774d21aed3bf9b5313ee24
- ArcGIS Hub: https://hub.arcgis.com/datasets/9d8de5e265ad4fe09893cf75b8dbfb77
- NOAA (marine): https://www.fisheries.noaa.gov/resource/map/national-esa-critical-habitat-mapper

**Format:** GeoJSON, KML, Shapefile, GeoPackage via ArcGIS Hub; WFS/REST service.

**US coverage:** National; ~700+ species with designated critical habitat. Coverage patchy by species range.

**Update frequency:** Updated as new critical habitat designations are finalized through federal rulemaking (irregular).

**Download method:** ArcGIS Hub export to GeoJSON; national dataset in a single download.

**Integration complexity:** Medium. Polygons range from simple to very complex (watershed-scale). Tippecanoe at zoom 0–12 recommended.

**License:** Public domain.

**Priority:** Tier 2.

---

## 4. Historic Preservation

### 4a. National Register of Historic Places (NPS)

**What it shows:** Point and polygon locations of all properties listed on the National Register of Historic Places — approximately 100,000 properties including individual buildings, structures, sites, objects, and historic districts.

**Why AEC professionals need it:** NRHP listing triggers Section 106 review for any federal undertaking. Historic Tax Credits (20% federal) require NRHP eligibility. Secretary of Interior Standards govern rehabilitation.

**Data source:**
- NPS Data Downloads: https://www.nps.gov/subjects/nationalregister/data-downloads.htm
- ArcGIS Hub (points): https://public-nps.opendata.arcgis.com/datasets/nps::national-register-of-historic-places-points/explore
- ArcGIS Hub (polygons): https://hub.arcgis.com/datasets/nps::national-register-of-historic-places-polygons/about
- NPS MapServer: https://mapservices.nps.gov/arcgis/rest/services/cultural_resources/nrhp_locations/MapServer

**Format:** ArcGIS Feature Service (GeoJSON export available); shapefile download from NPS data downloads page.

**US coverage:** National; ~100,000 listed properties. Restricted properties (some archaeological sites) excluded from public download.

**Update frequency:** Rolling as new listings are approved by NPS (typically monthly).

**Download method:** NPS OpenData ArcGIS Hub — GeoJSON export of points (~100K features) and polygons (~15K features for properties >10 acres).

**Integration complexity:** Low. Points: trivial tippecanoe. Polygons: small dataset, straightforward. Both fit in a single PMTiles file under 50 MB.

**License:** Public domain.

**Priority:** Tier 1.

---

### 4b. State and Local Historic Districts

**Data source:** No national dataset. Each SHPO and local historic district commission maintains its own data. Some state geoportals aggregate (e.g., NC HPO: https://www.hpo.nc.gov/survey-and-national-register/gis-maps-and-data).

**US coverage:** Approximately 2,500 local historic districts nationally. Coverage maps exist for fewer than 20 states.

**Integration complexity:** Very high — 50+ separate data collection efforts.

**Priority:** Tier 3 — editorial tooltip noting the gap; link to each state SHPO.

---

## 5. Wildland-Urban Interface / Fire

### 5a. USFS WUI 2020 Dataset

**What it shows:** Housing density and wildland vegetation data used to classify areas as WUI Interface (housing intermixed with vegetation), WUI Intermix (scattered housing within wildland vegetation), or non-WUI. Uses census housing data (1990–2020) and NLCD vegetation.

**Why AEC professionals need it:** IRC Chapter R327 and CBC Chapter 7A impose fire-resistive construction requirements based on WUI location. Insurers, lenders, and AHJs all reference WUI designation.

**Data source:**
- USDA Forest Service RDS Archive: https://www.fs.usda.gov/rds/archive/catalog/RDS-2015-0012-4
- Silvis Lab (University of Wisconsin): http://silvis.forest.wisc.edu/data/wui-change/
- USDA AgroData Commons: https://agdatacommons.nal.usda.gov/articles/dataset/Wildland_Urban_Interface_2020_Map_Service_/25973179
- ArcGIS REST: https://apps.fs.usda.gov/arcx/rest/services/EDW/EDW_WUI_2020_01/MapServer/0

**Format:** File Geodatabase (national) or Shapefile (by state). Data includes 1990/2000/2010/2020 housing density and 1992/2001/2011/2019 NLCD vegetation.

**US coverage:** Conterminous US (CONUS). Alaska and Hawaii not included.

**Update frequency:** Decennial — aligned with census. 2020 edition is current; next expected ~2031.

**Download method:** Bulk GDB download from RDS Archive (national). State-level shapefiles also available. File size for national GDB: expect 1–3 GB for CONUS.

**Integration complexity:** Medium. Tippecanoe at zoom 0–12 with WUI class attribute retained. Estimated PMTiles: 150–400 MB.

**License:** Public domain (USDA).

**Priority:** Tier 1.

---

### 5b. CAL FIRE Fire Hazard Severity Zones (California)

**What it shows:** State Responsibility Area (SRA) and Local Responsibility Area (LRA) fire hazard severity zones: Moderate, High, or Very High. Effective April 1, 2024 (SRA); recommended March 24, 2025 (LRA).

**Why AEC professionals need it:** FHSZ designation in California directly dictates CBC Chapter 7A requirements — fire-resistive roofing, exterior siding, vent screens, decking, etc. Required disclosure on property sales.

**Data source:**
- California State Geoportal: https://gis.data.ca.gov/datasets/CALFIRE-Forestry::california-fire-hazard-severity-zones-fhsz
- OSFM rollout hub: https://fire-hazard-severity-zones-rollout-calfire-forestry.hub.arcgis.com/
- ArcGIS MapServer: https://services.gis.ca.gov/arcgis/rest/services/Environment/Fire_Severity_Zones/MapServer

**Format:** GeoJSON, GeoTIFF, CSV, KML, ZIP shapefile — all from California Geoportal.

**US coverage:** California only.

**Download method:** Direct GeoJSON or ZIP download from California State Geoportal. SRA and LRA are separate layers; both needed for complete coverage.

**Integration complexity:** Low. CA-only makes the dataset manageable. Estimated <20 MB PMTiles.

**License:** Public domain (State of California).

**Priority:** Tier 1.

---

### 5c. USFS Wildfire Hazard Potential (WHP)

**What it shows:** Index of wildfire hazard potential at 270 m resolution (national) or 30 m resolution (Wildfire Risk to Communities edition). Continuous raster 0 (very low) to 5 (very high). Incorporates fuel load, moisture, fire weather, and topography.

**Data source:**
- USFS FireLab: https://research.fs.usda.gov/firelab/products/dataandtools/wildfire-hazard-potential
- Wildfire Risk download portal: https://wildfirerisk.org/download/
- ImageServer (REST): https://apps.fs.usda.gov/fsgisx01/rest/services/RDW_Wildfire/RMRS_WRC_WildfireHazardPotential/ImageServer

**Format:** GeoTIFF raster (270 m national; 30 m state-level).

**US coverage:** Conterminous US at 270 m; state-level 30 m available.

**Update frequency:** Irregular — 2020 is current edition.

**Integration complexity:** Medium. Raster data requires Cloud-Optimized GeoTIFF (COG) conversion for efficient HTTP range serving. For raster PMTiles, use tippecanoe raster mode.

**License:** Public domain (USDA).

**Priority:** Tier 2.

---

## 6. Airport / FAA

### 6a. FAA Digital Obstacle File (DOF)

**What it shows:** All known man-made obstructions that penetrate FAR Part 77 surfaces — towers, buildings, cranes, smokestacks, antennas. Attributes include structure type, height AGL, height AMSL, lighting, marking, accuracy code.

**Why AEC professionals need it:** New construction near airports requires FAA OE/AAA filing (Form 7460-1) if it penetrates Part 77 surfaces.

**Data source:**
- FAA AeroNav: https://www.faa.gov/air_traffic/flight_info/aeronav/digital_products/dof/
- ArcGIS Hub: https://adds-faa.opendata.arcgis.com/datasets/e202ff4e4cf943bda02ff63c0c44c9b7_0/about
- OE/AAA GIS tools: https://oeaaa.faa.gov/oeaaa/external/gisTools/gisAction.jsp

**Format:** Shapefile / CSV from AeroNav; GeoJSON from ArcGIS Hub. Updated every 56 days.

**US coverage:** National including territories, plus parts of Canada/Mexico/Caribbean.

**Update frequency:** Every 56 days (FAA aeronautical cycle).

**Integration complexity:** Low. Point data; straightforward tippecanoe. Estimated PMTiles <10 MB.

**License:** Public domain.

**Priority:** Tier 2.

---

### 6b. FAA UAS Facility Maps

**What it shows:** Maximum altitudes (feet AGL) at which the FAA may authorize Part 107 UAS (drone) operations in controlled airspace near airports. Grid-cell polygons covering Class B/C/D and Class E surface areas.

**Why AEC professionals need it:** Construction cranes near airports require LAANC authorization. Architects conducting drone-based site documentation need this.

**Data source:**
- FAA UDDS: https://udds-faa.opendata.arcgis.com/
- FAA UAS Facility Maps page: https://www.faa.gov/uas/commercial_operators/uas_facility_maps
- FAA UAS ArcGIS Hub: https://uas-faa.opendata.arcgis.com/

**Format:** Shapefile, GeoJSON, KMZ — multiple formats via UDDS.

**Integration complexity:** Low. Polygon grid cells; tippecanoe straightforward. Estimated <50 MB.

**License:** Public domain.

**Priority:** Tier 2.

---

### 6c. Airport Noise Contours (FAA Part 150 DNL)

**What it shows:** DNL contours at 65, 70, and 75 dB around airports. Determine noise-compatible land use per FAA AC 150/5020-1.

**Data source:** No national dataset. FAA Part 150 studies are per-airport submissions. Approximately 40 major airports have published DNL contours. Individual airport environmental pages host their own GIS files.

**Integration complexity:** Very high for national coverage.

**Recommendation:** Use BTS Transportation Noise Map (§7a) as proxy for metro-scale noise footprint. Aggregate top 50 airport Part 150 studies for Tier 3.

**Priority:** Tier 3 as dedicated layer; incorporated into BTS Transportation Noise Map at Tier 2.

---

## 7. Noise / Vibration

### 7a. BTS National Transportation Noise Map

**What it shows:** Modeled noise exposure (dBA DNL) for aviation, highway, and passenger rail at national scale. Derived from FAA AEDT model (aviation), FHWA Traffic Noise Model inputs (road), and Amtrak/commuter rail schedules.

**Why AEC professionals need it:** First-pass assessment of transportation noise environment at any US location. HUD noise assessment requires determination of whether DNL exceeds 65 dB.

**Data source:**
- BTS Geospatial: https://www.bts.gov/geospatial/national-transportation-noise-map
- Interactive map: https://maps.dot.gov/BTS/NationalTransportationNoiseMap/
- Supporting datasets (ROSAP): https://rosap.ntl.bts.gov/view/dot/77530

**Format:** WMS tile service (aviation and road noise rasters). Raster download via ROSAP/NTAD.

**US coverage:** National.

**Integration complexity:** Medium for raster. WMS endpoint can be added directly to MapLibre GL as raster source. For PMTiles, convert raster GeoTIFF to Cloud-Optimized GeoTIFF (COG).

**License:** Public domain.

**Priority:** Tier 2.

---

## 8. Radon

### 8a. EPA Radon Zone Map

**What it shows:** All 3,142 US counties classified into three zones: Zone 1 (predicted average indoor radon >4 pCi/L), Zone 2 (2–4 pCi/L), Zone 3 (<2 pCi/L). Developed 1993.

**Why AEC professionals need it:** IRC Appendix F (adopted in many jurisdictions) and ASTM E1709 require passive radon mitigation systems in new construction in Zone 1 areas, and recommend them in Zone 2.

**Data source:**
- EPA Radon Map: https://www.epa.gov/radon/epa-map-radon-zones
- ArcGIS viewer: https://gispub.epa.gov/radon/
- State-specific PDFs: https://www.epa.gov/radon/epa-maps-radon-zones-and-supporting-documents-state

**Format:** ArcGIS MapServer (queryable); PDF for static reference. No direct GeoJSON/Shapefile national download — query the EPA ArcGIS REST endpoint for all 3,142 county features.

**US coverage:** Complete — all 3,142 US counties.

**Update frequency:** Static since 1993; no planned updates (state supplemental maps offer finer resolution).

**Download method:** Query EPA ArcGIS REST service with national bounding box to export GeoJSON. Alternatively, join county-zone CSV to Census TIGER county boundaries.

**Integration complexity:** Low. 3,142 polygons; trivial tippecanoe. Estimated PMTiles: <5 MB.

**License:** Public domain.

**Priority:** Tier 1 — high value, trivially easy.

---

## 9. Seismic

### 9a. USGS Seismic Design Maps API

**What it shows:** Site-specific seismic design parameters per ASCE/SEI 7-22 — Ss, S1, PGA, Fa, Fv, SMS, SM1, SDS, SD1, and derived Seismic Design Category (SDC).

**Why AEC professionals need it:** IBC Section 1613 and ASCE 7 Chapter 11 require SDC determination for every building. SDC drives structural system selection, detailing, and cost.

**Data source:**
- USGS Design Maps tool: https://earthquake.usgs.gov/hazards/designmaps/usdesign.php
- USGS Web Services: https://earthquake.usgs.gov/ws/designmaps/
- ASCE Hazard Tool: https://www.asce.org/publications-and-news/asce-hazard-tool/about
- Example API call: `https://earthquake.usgs.gov/ws/designmaps/asce7-22.json?latitude=34.05&longitude=-118.25&siteClass=D&title=Example`

**Format:** REST API — JSON response. Input: latitude, longitude, site class, reference document.

**US coverage:** Conterminous US, Alaska, Hawaii, Puerto Rico, US territories.

**Update frequency:** Aligned with ASCE 7 edition releases (7-22 is current).

**Integration complexity:** Low for a point-query widget. **This is not a polygon layer** — most naturally surfaced as a "Click for seismic values" tool in the UI. Fire the USGS API on map click and display Ss, S1, SDC in a popup. No PMTiles needed.

**For a polygon layer:** USGS publishes national seismic hazard raster grids (PGA, Ss, S1 at 0.05° resolution) as GeoTIFFs from the National Seismic Hazard Model project: https://www.usgs.gov/programs/earthquake-hazards/national-seismic-hazard-model-project. These can be served as COG or raster PMTiles.

**License:** Public domain.

**Priority:** Tier 2 — point-query widget is low effort and high value. Raster tile layer is optional supplemental.

---

### 9b. USGS National Landslide Susceptibility Map

**What it shows:** Landslide susceptibility classifications for CONUS, Alaska, Hawaii, and Puerto Rico at 90 m resolution. Uses slope-relief threshold models.

**Why AEC professionals need it:** Landslide and liquefaction susceptibility affect geotechnical design, grading permits, and insurance in hillside and seismic areas. IBC Section 1803 requires investigation in high-susceptibility areas.

**Data source:**
- USGS ScienceBase: https://www.sciencebase.gov/catalog/item/65ccea5bd34ef4b119cb3bac
- USGS Landslide Hazards: https://www.usgs.gov/programs/landslide-hazards
- Interactive map: https://www.usgs.gov/tools/us-landslide-inventory-and-susceptibility-map

**Format:** GeoTIFF raster (90 m) from ScienceBase.

**US coverage:** CONUS + AK + HI + PR (90 m version).

**Update frequency:** Research product; 2024 publication, not regularly updated.

**Integration complexity:** Medium. 90 m raster → Cloud-Optimized GeoTIFF (COG) for HTTP range serving. Same raster pipeline as WHP (§5c).

**License:** Public domain (USGS).

**Priority:** Tier 2.

---

## 10. Code Jurisdictions

### 10a. ICC Building Code Adoption by State

**Data source:**
- ICC Code Adoption Map: https://www.iccsafe.org/adoptions/code-adoption-map/IECC
- BCAP Code Status: https://bcapcodes.org/code-status/
- ACEEE State Policy Database: https://database.aceee.org/state/residential-codes
- ICC adoption lookup: https://codeadoptions.iccsafe.org/

**Format:** Web-only interactive maps; no GIS download.

**Integration complexity:** High for a GIS layer. Data only in ICC's web database. Building a tile layer requires screenscraping or manually encoding the ICC adoption table, joining to state/county Census boundaries, and maintaining updates.

**Recommendation:** Surface as a tooltip/popup attribute joined to state boundaries. The ICC interactive map can be deep-linked from a popup. BCAP's OCEAN database is the most comprehensive tabular source for the energy code.

**License:** ICC data — not freely redistributable. BCAP tabular — freely available.

**Priority:** Tier 3 as tile layer; Tier 2 as editorial integration in popup content.

---

## 11. Engineering Team Recommendations

### PMTiles-ready today (single sprint)

| Dataset | Source | Est. PMTiles Size | Preprocessing |
|---|---|---|---|
| EPA Radon Zones (county polygons) | EPA ArcGIS REST → GeoJSON | <5 MB | Join zone attribute to TIGER counties |
| NPS Historic Register Points | NPS ArcGIS Hub | <15 MB | Direct GeoJSON export |
| NPS Historic Register Polygons | NPS ArcGIS Hub | <20 MB | Direct GeoJSON export |
| CAL FIRE FHSZ (CA only) | CA Geoportal GeoJSON | <20 MB | Download and tippecanoe |
| FAA Digital Obstacle File (DOF) | FAA ArcGIS Hub | <10 MB | GeoJSON export |
| FAA UAS Facility Maps | FAA UDDS | <50 MB | GeoJSON export |

### Requires moderate preprocessing (1–3 engineering days)

| Dataset | Source | Preprocessing |
|---|---|---|
| FEMA NFHL Flood Zones | FEMA REST / county-state SHPs | Collect state SHPs, merge, tippecanoe |
| USFWS NWI Wetlands | USFWS state GeoPackages | Download all states, merge, tippecanoe with simplification |
| USFS WUI 2020 | USDA RDS GDB | Extract polygon layer from GDB, tippecanoe |
| USFWS Critical Habitat | USFWS ArcGIS Hub GeoJSON | Download, tippecanoe with attribute filter |
| EPA EJSCREEN | EPA GAFTP GDB | Extract GDB, join to census geometries, tippecanoe |

### Does not produce PMTiles (point-query APIs or raster)

| Dataset | Integration pattern |
|---|---|
| USGS Design Maps (Ss/S1/SDC) | Click-to-query popup: fire USGS REST API on map click |
| BTS Transportation Noise Map | WMS raster source in MapLibre GL |
| USFS WHP / Landslide Susceptibility | Cloud-Optimized GeoTIFF served via HTTP range requests |

### Requires commercial licensing

| Dataset | Vendor | Coverage | Key fields |
|---|---|---|---|
| Regrid Standardized Zoning | Regrid | 6,000+ municipalities, 127.9M parcels | FAR, setbacks, permitted uses, lot coverage |

### No viable national open dataset (gaps)

- Height restrictions / FAR at parcel level (open): no national dataset; parcel data is locally maintained
- Airport noise contours (national): no national aggregation; per-airport Part 150 studies only
- State/local historic districts: 50 separate SHPO sources; no national aggregate
- Code adoption year by jurisdiction: tabular only; no GIS download from ICC

### Recommended Tier 1 sprint sequence

1. EPA Radon Zones — join to TIGER counties, tippecanoe, PMTiles. **1 day.**
2. NPS Historic Register (points + polygons) — ArcGIS Hub GeoJSON export, tippecanoe. **1 day.**
3. CAL FIRE FHSZ — CA Geoportal download, tippecanoe. **0.5 day.**
4. FEMA NFHL Flood Zones — script FEMA MSC state downloads, merge, tippecanoe. **3 days.**
5. USFWS NWI Wetlands — download all state GeoPackages, merge, tippecanoe. **2 days.**
6. USFS WUI 2020 — download national GDB, extract, tippecanoe. **1 day.**
7. USGS Seismic Design Maps — implement click-to-query popup widget via REST API. **1 day.**
8. NZA state GeoJSONs (completed states) — download from Mercatus/S3, tippecanoe. **1 day.**
9. Regrid Standardized Zoning — evaluate licensing, negotiate contract. Engineering spike to integrate tile service. **5+ days.**
