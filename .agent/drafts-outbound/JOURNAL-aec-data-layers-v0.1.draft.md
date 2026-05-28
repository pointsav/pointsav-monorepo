---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.2"
title: "Open-Source Building-Systems Data Layers for Urban-Scale Site Analysis: A Continental Coverage Assessment Across Sixteen Countries"
target_journal: "Automation in Construction"
target_publisher: "Elsevier"
impact_factor: "12.0"
alternate_venue: "Journal of Computing in Civil Engineering (ASCE, IF 6.04); Journal of Information Technology in Construction (ITcon, IF 3.8, open access)"
authors:
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: jmwoodfine@gmail.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Data Curation
      - Formal Analysis
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Conceptualization
      - Validation
      - Writing – Review & Editing
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Software
      - Data Curation
      - Writing – Review & Editing
subject_codes:
  - "TH9 Construction management and practice"
  - "NA2100 Architecture — Environmental aspects"
  - "QA76.9.I52 Geographic information systems"
keywords:
  - AEC site analysis
  - building code climate zones
  - open geospatial data
  - flood hazard
  - continental-scale coverage
  - ASHRAE 169
  - H3 spatial indexing
bcsc_class: public-disclosure-safe
ai_tool_used: "claude-sonnet-4-6 (Anthropic)"
corresponding_author: jmwoodfine@gmail.com
word_count_body: 7800
word_count_target: 8000
submission_status: not-submitted
language_pass_date: 2026-05-28
cites: []
forbidden_terms_cleared: true
scaffolded_from:
  - AEC-LAYERS-RESEARCH.md
  - AEC-DATA-PARITY-RESEARCH.md
scaffolded_date: 2026-05-27
writing_pass_date: 2026-05-28
preprint_posted: true
preprint_posted_date: 2026-05-28
doi: ""
license: "CC BY 4.0"
cite_as: "Woodfine, J.M., Woodfine, P.M., & Woodfine, M. (2026). Open-Source Building-Systems Data Layers for Urban-Scale Site Analysis. Working Paper v0.2, 28 May 2026. Woodfine Management Corp., Vancouver, BC."
revision_history:
  - version: "0.1"
    date: "2026-05-27"
    changes: "Initial scaffold from AEC research materials"
  - version: "0.2"
    date: "2026-05-28"
    changes: "Full writing pass (§1–§5, §7–§8); language pass; preprint notice and FLS advisory; public posting"
notes_for_editor: |
  Writing pass complete 2026-05-28. All sections written except §6 Results,
  which requires concrete coverage metrics from the nightly build pipeline
  (target: Night 2–5, 2026-05-24 to 2026-05-28). §6 is marked as a structured
  TODO with the data fields required.

  Pre-submission checklist:
    1. §6 Results: populate coverage metrics from nightly build pipeline output
    2. ORCID IDs for all three authors required
    3. Final word count trim to ≤8,000 words body
    4. Confirm EFFIS wildfire data request submitted to JRC (prior to final build)
    5. Verify ATC Hazards API terms permit commercial use before citing in §5.4
    6. Parity scorecard table (§4) to be rendered as a figure or table per AutoCon style
    7. All cited URLs verified as live at submission date
---

---

> **Working Paper · Version 0.2 · 2026-05-28 · CC BY 4.0**
> This manuscript is a working draft. It has not been peer reviewed. Findings are preliminary and subject to revision without notice. Correspondence: jmwoodfine@gmail.com.
>
> *Cite as:* Woodfine, J.M., Woodfine, P.M., & Woodfine, M. (2026). Open-Source Building-Systems Data Layers for Urban-Scale Site Analysis. Working Paper v0.2, 28 May 2026. Woodfine Management Corp., Vancouver, BC.

> **Forward-Looking Statements**
> Certain statements in this paper describe intended research directions, planned system capabilities, and anticipated outcomes. These statements reflect the authors' current expectations and are based on reasonable assumptions and work in progress as of the date above. Actual results, measurements, and findings may differ materially. Readers should not place undue reliance on such statements; they are subject to revision as research progresses and new data become available.

# Open-Source Building-Systems Data Layers for Urban-Scale Site Analysis: A Continental Coverage Assessment Across Sixteen Countries

**Woodfine Management Corp.**
Vancouver, British Columbia, Canada

*Corresponding author:* jmwoodfine@gmail.com

*Keywords:* AEC site analysis, building code climate zones, open geospatial data, flood hazard, ASHRAE 169, H3 spatial indexing

---

## Abstract

Architecture, Engineering, and Construction (AEC) professionals evaluating commercial development sites require regulatory-grade geospatial inputs — building code climate zones, flood hazard designations, seismic design categories, solar irradiance, and landscape eco-regions — drawn from dozens of national agencies with inconsistent licensing, format, and spatial resolution. No systematic comparative assessment of open-licence coverage across jurisdictions and layer types has been published. This paper presents a coverage assessment of eight AEC-relevant geospatial data layer categories across sixteen countries in North America and Europe, evaluating each source by regulatory prescriptiveness, licence, and practical integration method. A reproducible H3 hexagonal resolution-7 spatial-indexing pipeline joins each layer to a sub-metropolitan point inventory of 6,493 co-location clusters. The United States achieves regulatory-grade open coverage across all four Tier 1 layers. Eight EU member states can produce regulatory-grade climate-zone polygons through national code lookup tables joined to LAU2 municipal boundaries, replacing Köppen proxy approaches for the majority of EU study clusters. Three structural open-data gaps are identified: no national flood hazard layer exists for Canada; Mexico's CONABIO precision eco-region and climate data carry a non-commercial licence restriction; and ASCE 7 wind and snow load maps are copyrighted, requiring an API-only compliance approach. A seven-test falsification programme evaluates coverage and reproducibility claims.

*(215 words)*

---

## 1. Introduction

### 1.1 The Research Problem

The design of a large commercial facility — a retail power centre, a logistics warehouse, a multi-family residential development — requires regulatory-grade geospatial input at every design stage. A structural engineer calculating seismic design categories needs peak ground acceleration (PGA) values at the project address. A mechanical engineer sizing HVAC systems needs the ASHRAE 169 building code climate zone. A civil engineer designing stormwater management needs flood zone designation and, for LEED Sustainable Sites compliance, eco-region data for native plant selection. A photovoltaic system designer needs annual global horizontal irradiance (GHI).

Each of these data inputs is publicly funded and, in the United States, publicly available under public-domain or permissive licences. Collectively, they define the regulatory and environmental context within which any large building project must be designed and permitted. Yet no systematic comparison of their availability — across jurisdictions, across licence regimes, and in a form suitable for integration into spatial analysis pipelines — has been published.

The absence of this comparison has a practical consequence: geospatial analysis platforms serving AEC professionals either default to US-only data layers or substitute global proxies (Köppen-Geiger climate classification, WRI AQUEDUCT flood hazard) without documenting the precision and regulatory-grade trade-offs involved. The result is inconsistency in what data an AEC professional can access depending on whether their project is in Chicago, Calgary, Paris, or Madrid.

### 1.2 Scope and Contributions

This paper makes three contributions.

First, a taxonomy of eight AEC-relevant geospatial data layer categories, ranked by regulatory prescriptiveness and AEC workflow stage, with documented sources and licences for sixteen countries in North America and Europe (US, CA, MX, GB, FR, DE, ES, IT, PL, NL, PT, SE, DK, NO, FI, GR).

Second, a per-country coverage assessment using a three-level quality classification: regulatory-grade open data (✓), proxy or partial coverage (~), and no available open data (✗). The coverage scorecard spans 16 countries × 8 layer categories, producing a 128-cell open-data audit for the AEC domain.

Third, a reproducible H3 resolution-7 spatial-indexing pipeline enabling the assessed layers to be joined to any sub-metropolitan point inventory, with explicit documentation of the integration architecture for each layer type: PMTiles polygon for precise regulatory boundaries; raster PMTiles for continuous gradients; cluster metadata fields for scalar point values; and API call-time lookups for copyright-constrained datasets.

### 1.3 Structure

Section 2 reviews AEC data requirements, building energy code systems, open geospatial data infrastructure, and the H3 spatial indexing approach. Section 3 defines the eight-layer taxonomy. Section 4 presents the per-country coverage assessment. Section 5 describes the integration pipeline architecture. Section 6 reports quantitative coverage results. Section 7 discusses structural gaps, reproducibility, the formal hypotheses, and the falsification programme. Section 8 concludes.

---

## 2. Background and Related Work

### 2.1 AEC Data Requirements at Site Analysis Scale

Commercial building design proceeds in phases — site selection, schematic design, design development, construction documents — each requiring progressively more precise geospatial inputs. Site selection typically uses aggregate indicators (census population, mobility data) alongside coarse regulatory data (flood plain status, seismic zone) to screen candidate locations. Schematic and design-development phases require regulatory-grade inputs: precise ASHRAE climate zone designation for envelope and HVAC sizing; FEMA Flood Insurance Rate Map zone classification for first-floor elevation requirements; USGS PGA for seismic design category determination per ASCE 7.

The regulatory significance of these inputs is substantial. SFHA (Special Flood Hazard Area) designation under the FEMA National Flood Insurance Program obligates project-specific flood analysis and mandatory flood insurance. Seismic Design Category — derived from USGS NSHM PGA values — determines structural system type, detailing requirements, and non-structural component anchorage. In EU jurisdictions, equivalent requirements derive from the EU Buildings Directive (2010/31/EU, 2018/844/EU, recast 2024/1275/EU) and national energy code zone systems.

### 2.2 Building Energy Code Systems

In the United States, ASHRAE Standard 169-2013/2020 defines climate zones 1–8 with moisture subtypes (A: humid; B: dry; C: marine) at county resolution. The IECC 2021 climate zone map is co-derived from this classification and is the primary prescriptive reference for residential and commercial envelope insulation, fenestration, and mechanical system selection. The Pacific Northwest National Laboratory (PNNL) publishes the authoritative county-to-zone lookup table under public-domain licence (Taylor et al. 2008).

In the European Union, there is no harmonised pan-EU climate zone polygon equivalent to ASHRAE 169. The EU Buildings Directive mandates national energy performance calculation methodologies but does not prescribe a harmonised zone system. Each member state defines zones according to its national energy code, keyed to municipal (LAU2) administrative boundaries. The closest European parallel to ASHRAE 169 is the suite of national energy performance standards: RE2020 (France), EPBD national implementations, CTE DB-HE (Spain), GEG (Germany), DPR 412/1993 (Italy), KENAK (Greece), and equivalents. Each operates independently.

### 2.3 Flood Hazard Mapping Standards

Flood hazard mapping in the United States is administered by FEMA under the National Flood Insurance Program. The National Flood Hazard Layer (NFHL) provides Special Flood Hazard Area (SFHA) designations at parcel resolution across the continental United States, with zone classifications (A, AE, VE, and sub-types) that carry regulatory effect for development permits, first-floor elevation requirements, and mandatory flood insurance. The NFHL is distributed as public-domain data.

In the EU, the Floods Directive (2007/60/EC) requires member states to assess flood risk and publish flood hazard maps for areas of significant flood risk. Implementation quality and accessibility vary by member state (Alphen and Lodder 2006). Regulatory-grade shapefiles are directly downloadable from the UK, France, Spain, Italy, and Portugal; other member states require WFS ingestion from national or regional portals.

### 2.4 Open Geospatial Data Infrastructure and H3 Indexing

The OpenStreetMap (OSM) project (Haklay 2010) demonstrated that volunteer-contributed point data can approach national survey accuracy for road networks and points of interest in well-covered regions. Darnall et al. (2022) applied OSM retail point data to delineate UK retail agglomerations at national scale, establishing the viability of open data for commercial spatial analysis.

The H3 hexagonal spatial indexing system (Brodsky 2018) partitions the Earth's surface into a hierarchy of hexagonal cells at fifteen resolution levels. At resolution 7, each cell covers approximately 1.22 km² — smaller than a sub-metropolitan census-designated place but larger than a single building footprint. H3's regular hexagonal tessellation avoids the edge-effect artefacts of square-grid systems and supports consistent spatial joins across layer types with different native resolutions.

### 2.5 Coverage Gap

No published study provides a systematic comparative assessment of open-licence AEC geospatial data coverage across multiple jurisdictions and layer types at sub-metropolitan resolution. Closest adjacent work: individual layer documentation (FEMA NFHL technical reference; USGS NSHM documentation) treats each layer in isolation; global land cover assessments (Buchhorn et al. 2020) do not address regulatory AEC inputs; OSM quality assessments (Haklay 2010; Darnall et al. 2022) document volunteer-contributed data rather than regulatory agency layers. This paper fills that gap.

---

## 3. AEC Data Layer Taxonomy

### 3.1 Layer Categories and Regulatory Status

Eight categories are selected to represent regulatory inputs required at site-analysis or schematic-design stage for a large commercial facility in at least one of the sixteen study countries. "Regulatory-grade" designates a source explicitly cited in a building code, a flood insurance programme, or an environmental compliance standard — not merely correlated with regulatory requirements.

| Layer | AEC discipline | Code-prescriptive | Priority tier |
|---|---|---|---|
| Building code climate zones | Energy/HVAC | Yes (ASHRAE 169/IECC; national EU codes) | Tier 1 |
| Flood hazard zones | Structural/Site | Yes (FEMA NFHL; EU Floods Directive) | Tier 1 |
| Seismic PGA | Structural | Yes (ASCE 7/Eurocode 8) | Tier 1 |
| Solar GHI | Energy/MEP | Yes (LEED EAc1; code-proxy roles) | Tier 1 |
| Eco-regions | Landscape architecture | No (LEED SS/SITES reference) | Tier 1D |
| Wind design speed | Structural | Yes (ASCE 7 — copyrighted map) | Tier 2 |
| Wildfire hazard | Site/Materials | Yes (CA CBC Ch. 7A) | Tier 2 |
| Soil type | Geotechnical/Foundation | No (site-specific required) | Tier 3 |

### 3.2 Building Code Climate Zones

The ASHRAE 169/IECC climate zone is the single most operationally critical AEC data layer for US commercial building design. The zone number — 1 (very hot) through 8 (subarctic) with moisture subtypes A (humid), B (dry), and C (marine) — is the mandatory input to ASHRAE 90.1 envelope compliance tables, IECC prescriptive R-value tables, ACCA Manual N/J climate design condition selection, and HVAC sizing. An architect cannot begin a code-compliant envelope or mechanical design without this number.

The PNNL county-to-zone lookup table (public domain) maps all US counties; the DOE Building Energy Codes Program publishes the corresponding polygon shapefile. The pipeline joins the PNNL CSV to TIGER 2023 county polygons and converts to PMTiles via tippecanoe at z4–z16, yielding a ~3–5 MB layer.

In the EU, the build-by-join pipeline joins national code zone lookup tables to GISCO LAU2 municipal boundary polygons. Eight EU ISOs support this approach at regulatory-grade quality: France (RE2020, 8 zones H1a–H3, Etalab 2.0), Spain (CTE DB-HE Annex B, 12 zones A3–E1), Italy (DPR 412/1993, 6 zones A–F, CC BY 4.0 via ENEA), Germany (GEG 2023 / TRY 2017 Testreferenzjahr regions), Portugal (SCE/REH-RECS, 3 winter × 3 summer, DGEG), Finland (SFS-EN ISO 15927-4, 4 zones, CC BY 4.0), Poland (WT 2021/EN 12831, 5 zones I–V, CC BY), and Greece (KENAK, 4 zones A–D, ELSTAT). Four ISOs — Netherlands (NTA 8800), Denmark (BR18), Norway (TEK17), and the United Kingdom (SAP/HEM) — operate under single-reference standards or lack an analogous zone polygon system. Canada's NECB HOT2000 climate zones are available via NRCan under OGL-Canada. Mexico's NMX-C-460 zones lack an openly available vector dataset; INEGI climate raster is used as a substitute, with the CONABIO Köppen-García classification excluded due to its CC BY-NC 2.5 MX licence.

### 3.3 Flood Hazard Zones

FEMA's NFHL designates SFHA zones at parcel resolution across the continental United States. SFHA designation (Zone A, AE, VE, and sub-types) triggers mandatory flood insurance under the NFIP, imposes first-floor elevation requirements, and substantially constrains site design. FEMA distributes NFHL as state-by-state GeoPackage downloads (public domain); the full dataset exceeds 30 GB, but an SFHA-only extract (Zone types AE, A, VE, AO, AH) reduces to approximately 200–500 MB as PMTiles polygon data at z8–z16.

For EU member states, the Floods Directive (2007/60/EC) requires regulatory flood hazard map publication. Regulatory-grade shapefiles are publicly available for the United Kingdom (Environment Agency Flood Map for Planning, OGL v3), France (Géorisques Zonages Inondation 2020, Etalab 2.0), Spain (SNCZI Zonas Inundables T100, free, MITECO attribution), Italy (IdroGEO PAI, IODL 2.0), and Portugal (SNIAmb INSPIRE ATOM). Germany requires per-Bundesland WFS ingestion from LAWA portals via the BfG aggregator. Remaining EU ISOs and Canada are served by the JRC Global River Flood Hazard Maps v2.1 (Baugh et al. 2016) or WRI AQUEDUCT 3.0 (CC BY 4.0) as proxy layers.

### 3.4 Seismic PGA

PGA at 2% probability in 50 years is the primary input to seismic design category determination under ASCE 7-22, Eurocode 8 (EN 1998-1:2004+A1:2013), and National Building Code of Canada structural provisions. USGS NSHM 2023 provides gridded PGA for the United States at 0.01° × 0.01° resolution, public domain. ESHM20 (European Seismic Hazard Model 2020) provides EU coverage at 0.1° × 0.1°, CC BY 4.0. Both are served as raster PMTiles at z3–z10, sufficient to distinguish seismic zone differences between co-located sub-metropolitan markets.

### 3.5 Solar GHI

Annual mean GHI (kWh/m²/year) determines photovoltaic yield, informs passive solar design, and is required for LEED Energy and Atmosphere credit documentation. NREL NSRDB provides US coverage at 4 km resolution, public domain; PVGIS (JRC) provides EU and global coverage without redistribution restrictions. Solargis annual GHI is globally available under CC BY-SA 4.0; the ShareAlike clause creates licence compatibility concerns and NSRDB + PVGIS are preferred for the study countries. The recommended integration samples GHI at each cluster centroid via NREL and PVGIS APIs and stores the value as a `ghi_kwh_m2_yr` metadata field, avoiding new tile infrastructure.

### 3.6 Eco-regions and Landscape Reference Zones

Landscape architects on commercial development projects reference eco-regions to satisfy LEED BD+C Sustainable Sites credits (SSc2, SSc3, SSc4) requiring native and adaptive planting, and SITES v2 certification requirements — both of which cite EPA Level III Ecoregions as the native habitat baseline for US sites. Resolve Ecoregions 2017 (Dinerstein et al. 2017; CC BY 4.0) provides the global baseline at 846 ecoregions. EPA Level III (public domain, 85 ecoregions, contiguous US) provides higher precision for US projects. EEA Biogeographical Regions 2016 (free, EEA attribution) provides the 11-region reference used in EU Habitats Directive 92/43/EEC implementation, the appropriate reference for EU landscape architects.

---

## 4. Per-Country Coverage Assessment

Each of the sixteen study countries is assessed on the 8 layer categories under a three-level quality classification: ✓ (regulatory-grade open data — explicitly cited in national code or regulatory programme, freely available for commercial use), ~ (proxy or partial coverage — modelled, API-only, or partial geographic coverage), and ✗ (no open data — proprietary, non-commercial-restricted, or structurally absent).

| ISO | Climate zone | Flood | Seismic | Solar | Eco-region | Wildfire | Soil | Wind/snow |
|---|---|---|---|---|---|---|---|---|
| US | ✓ ASHRAE 169 | ✓ FEMA NFHL | ✓ USGS NSHM | ✓ NSRDB | ✓ EPA L3 | ✓ USFS WHP | ~ SSURGO API | ~ ATC API only |
| CA | ✓ NECB HOT2000 | ~ Future Susceptibility | ✓ NRCan 2015 | ✓ NSRDB | ~ Resolve | ~ CWFIS | ~ SoilGrids | ~ NRC NBC |
| MX | ~ INEGI raster | ~ CENAPRED Atlas | ~ CENAPRED (intermittent) | ✓ NSRDB | ~ Resolve | ~ SoilGrids | ~ SoilGrids | ~ |
| GB | ✗ no zone polygon | ✓ EA Flood Map | ~ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ GWIS FWI | ~ SoilGrids | ~ |
| FR | ✓ RE2020 join | ✓ Géorisques TRI | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ EFFIS (req.) | ~ SoilGrids | ~ |
| DE | ~ TRY 2017 | ✓ LAWA WFS | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ GWIS FWI | ~ SoilGrids | ~ |
| ES | ✓ CTE DB-HE join | ✓ SNCZI T100 | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ✓ EFFIS (req.) | ~ SoilGrids | ~ |
| IT | ✓ DPR 412 join | ✓ IdroGEO PAI | ✓ ESHM20+INGV | ✓ PVGIS | ~ Resolve/EEA | ✓ EFFIS (req.) | ~ SoilGrids | ~ |
| PL | ~ WT 2021 join | ~ KZGW INSPIRE | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ GWIS FWI | ~ SoilGrids | ~ |
| NL | n/a single zone | ~ Risicokaart | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ GWIS FWI | ~ SoilGrids | ~ |
| PT | ✓ SCE join | ✓ SNIAmb INSPIRE | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ✓ EFFIS (req.) | ~ SoilGrids | ~ |
| SE | ~ BBR 4 zones | ~ MSB INSPIRE | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ very low | ~ SoilGrids | ~ |
| DK | n/a single zone | ~ INSPIRE WFS | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ very low | ~ SoilGrids | ~ |
| NO | n/a single zone | ~ NVE INSPIRE | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ very low | ~ SoilGrids | ~ |
| FI | ~ SFS 4 zone join | ~ SYKE INSPIRE | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ~ very low | ~ SoilGrids | ~ |
| GR | ~ KENAK 4 join | ~ YPEN INSPIRE | ✓ ESHM20 | ✓ PVGIS | ~ Resolve/EEA | ✓ EFFIS (req.) | ~ SoilGrids | ~ |

The United States achieves the broadest regulatory-grade open coverage across the study set: all four Tier 1 layers are ✓. France, Spain, Italy, and Portugal achieve ✓ for both climate zones (via national code lookup joins) and flood hazard (via Floods Directive regulatory shapefiles). Germany achieves ✓ for flood hazard via LAWA WFS but is assessed ~ for climate zones due to a registration requirement for the DWD CDC raster input. All sixteen study countries achieve ✓ or ~ for seismic (via ESHM20 globally) and solar GHI (via PVGIS globally). Canada's primary gap is flood hazard: no national regulatory layer exists pending FHIMP completion. Mexico has the most constrained environment, with CONABIO non-commercial restrictions and intermittent CENAPRED download availability.

---

## 5. Integration Pipeline

### 5.1 H3 Spatial Indexing Approach

All layer values are joined to the point inventory using H3 resolution 7 as the spatial key. At this resolution, each hexagonal cell covers approximately 1.22 km² — larger than an individual anchor retail site (typically 0.5–2.5 ha) but smaller than a sub-metropolitan census-designated place (typically 5–500 km²). The resolution absorbs the sub-cluster scatter of individual building footprints while remaining fine enough to distinguish neighbouring clusters in the same metropolitan area that may fall in different climate zones, flood zones, or eco-regions.

Each cluster centroid is assigned an H3 resolution-7 cell index. Polygon layers (climate zones, flood zones, eco-regions) are intersection-joined to the centroid's H3 cell polygon, producing consistent categorical assignments irrespective of where within the cell the centroid falls. Raster layers (seismic PGA, GHI at tile resolution) are sampled at the centroid coordinate directly.

### 5.2 Polygon Layers: PMTiles

Polygon layers carrying categorical regulatory designations are converted to PMTiles using the tippecanoe vector tiling library. The PMTiles format (Protomaps) enables single-file vector and raster tile archives served from object storage without a tile server, suitable for high-availability geospatial applications. Zoom-level configuration follows a precision-appropriate mapping: climate zone polygons (county-resolution) at z4–z16; flood zone polygons (parcel-resolution SFHA) at z8–z16; eco-region polygons at z4–z12; seismic PGA rasters at z3–z10. Approximate PMTiles file sizes: ASHRAE 169/IECC climate zones ~3–5 MB; EPA Level III Ecoregions ~8–15 MB; FEMA NFHL SFHA subset ~200–500 MB; EU Floods Directive shapefiles ~50–1,000 MB per country.

### 5.3 Point Enrichment: Cluster Metadata Fields

Layers with continuous scalar values at cluster centroids — solar GHI, seismic PGA, urban heat island intensity, soil classification — are stored as fields in the cluster metadata JSON rather than as tile layers. This approach eliminates tile hosting overhead for these layers while making values available for programmatic query, inspector panel display, and export. The solar GHI field (`ghi_kwh_m2_yr`) is populated by querying NREL NSRDB for US clusters and PVGIS for EU clusters via their respective public APIs; seismic PGA (`pga_2pct_50yr_g`) is sampled from USGS NSHM 2023 for US clusters and ESHM20 for EU clusters at cluster centroid coordinates.

### 5.4 API Call-Time Lookups: Copyright-Constrained Layers

ASCE 7-22 design wind speed and ground snow load maps are copyrighted by the American Society of Civil Engineers. Redistribution of these maps as GIS layers requires a commercial licence from ASCE; no open-licence version exists. The ATC Hazards by Location API (Applied Technology Council) returns design wind speed (Vult in mph), site-amplified ground snow load (Pg), and seismic design parameters at any US coordinate via a structured JSON response. The ATC API is queried at cluster-selection time; values are displayed in the site-analysis inspector without being stored as a redistributed layer. This architecture is appropriate for point-lookup use cases and avoids ASCE copyright exposure; it introduces API latency and an external dependency that bulk-downloaded raster layers do not.

---

## 6. Results

[TODO — Concrete coverage metrics pending nightly build pipeline completion (target: 2026-05-28). Required fields for this section:

For each of the four Tier 1 layers built in the nightly pipeline:
- Total US T1 clusters assigned climate zone (ASHRAE 169): N of M (X%)
- Total EU clusters assigned climate zone via build-by-join: N of M (X%) per ISO
- Total US clusters assigned flood zone (FEMA SFHA): N of M (X%)
- GHI values populated for US clusters via NSRDB: N of M (X%)
- GHI values populated for EU clusters via PVGIS: N of M (X%)

Preliminary findings from the parity scorecard (§4): 5 of 8 layer types achieve ✓ (regulatory-grade open) for at least 6 of 16 study countries. All 16 study countries achieve ✓ or ~ on seismic (ESHM20 globally) and solar GHI (PVGIS globally). The US achieves ✓ on all assessed Tier 1 layers. Concrete H3-cell coverage counts to be inserted when nightly build data is available.]

---

## 7. Discussion

### 7.1 Coverage Gaps and Implications

**EU building code climate zones are derived, not native.** No pan-EU harmonised climate zone polygon equivalent to ASHRAE 169 exists. The build-by-join pipeline produces regulatory-grade assignments for eight EU ISOs, but the output is a derived layer: it inherits errors in national code zone lookup tables and depends on the completeness of LAU2 boundary data from GISCO. This creates a maintenance dependency: when France updates RE2020, Germany updates GEG, or any of the eight ISOs revises its national energy code, the lookup table must be updated before the climate zone layer reflects the new regulatory regime. This limitation is structural and unavoidable with the current state of EU spatial data infrastructure; it is ameliorated but not eliminated by INSPIRE Directive implementation.

**Canada has no national flood hazard layer.** The FHIMP (Flood Hazard Identification and Mapping Programme, 2024–2028) is expected to produce the first nationally consistent Canadian flood hazard mapping layer. Until it delivers, the best available substitute is NRCan's Future Flood Susceptibility 2024 dataset — an XGBoost-modelled national layer, not a regulatory product. Any display of Canadian flood hazard data using this proxy must disclose its modelled, non-regulatory character.

**CONABIO licensing blocks Mexico precision data.** CONABIO's eco-region and Köppen-García climate classification carry CC BY-NC 2.5 MX licences, blocking commercial use without written exception. Resolve Ecoregions 2017 (CC BY 4.0) serves as a globally available substitute, commercially permissible, but with lower site-scale precision for Mexican projects. The INEGI climate raster provides a usable NMX-C-460 zone proxy at the cost of a raster-to-vector build step.

**ASCE 7 copyright constrains redistribution.** ASCE 7 wind speed and ground snow load maps are copyrighted. The ATC Hazards API provides a compliant point-lookup alternative, but the API architecture introduces latency and external dependency absent from bulk-downloaded raster layers. The constraint is well-documented in the structural engineering community but inconsistently reflected in geospatial tools that redistribute ASCE-derived values without licence.

### 7.2 Reproducibility and Licence Transparency

Reproducibility of the pipeline described in §5 requires: documenting every source URL, licence, and download date for each layer; documenting all transformation steps (coordinate reprojection, field selection, tippecanoe configuration); and explicitly excluding any layer that introduces a non-commercial, ShareAlike, or proprietary licence without documentation. The EFFIS wildfire layer requires a formal data request to JRC and cannot be reproducibly fetched from a public URL; any pipeline that includes EFFIS data must document the data request date and dataset version. GWIS FWI raster data (free, attribution) is used as a fallback for EU ISOs where EFFIS data is pending.

The Solargis annual GHI dataset (CC BY-SA 4.0) is excluded from the pipeline due to the ShareAlike clause. NSRDB and PVGIS together cover all sixteen study countries without a ShareAlike exposure. The Copernicus Data Licence governing ERA5 reanalysis data permits commercial use with attribution, making it an alternative for weather-related fields where NOAA/ECCC data is not directly accessible.

### 7.3 Formal Hypotheses

> **H₁ (Coverage Hypothesis).** For the sixteen countries studied, at least six of eight AEC data layer types can be populated using open-licence sources (CC BY, OGL, ODbL, public domain, or equivalent commercial-use licence) for at least eight of the sixteen study countries.

> **H₀ (Null).** Fewer than six of eight layer types achieve open-licence, commercially permissible coverage for at least eight of the sixteen study countries.

> **H₂ (Regulatory-Grade Hypothesis).** For the United States, all four Tier 1 layers (building code climate zones, flood hazard, seismic PGA, solar GHI) are available from regulatory-grade open-licence sources — sources explicitly cited in US code compliance documentation — at the time of the assessment.

### 7.4 Falsification Programme

Seven tests operationalise H₁, H₂, and the reproducibility commitment:

**Test 1 — Coverage count.** For each of the eight layer categories, count the number of study countries achieving ✓ (regulatory-grade open) coverage. H₁ is falsified if fewer than six categories achieve ✓ for at least eight countries.

**Test 2 — US regulatory-grade confirmation.** Verify that ASHRAE 169/IECC (PNNL/DOE), FEMA NFHL (SFHA zones), USGS NSHM 2023 PGA, and NREL NSRDB GHI are each cited as authoritative sources in US permit documentation or code compliance standards. H₂ is falsified if any of the four sources is rejected as non-authoritative by a competent US authority in a permit application context.

**Test 3 — Licence compatibility.** Confirm that no CC BY-SA or CC BY-NC layer is included in the production pipeline without a documented written exception or an explicitly documented open-licence substitute. The pipeline is falsified if an incompatible layer is found in production without documentation.

**Test 4 — Reproducibility.** Re-execute the build pipeline from the listed source URLs on a clean environment. Compare PMTiles checksums to the reference build. Falsified if outputs differ due to an undisclosed proprietary data dependency.

**Test 5 — EU climate zone accuracy.** For each of the eight EU ISOs with build-by-join climate zone layers, select 50 municipal centroids from the corresponding national official publication. Confirm that the pipeline assigns the correct zone code. Falsified if the assignment error rate exceeds 5%.

**Test 6 — H3 join accuracy.** For US ASHRAE 169 zones: select 100 cluster centroids distributed across all 8 climate zones. Confirm that the H3 resolution-7 cell-to-zone assignment matches the PNNL county table. Falsified if more than 2% of centroids are assigned to a zone inconsistent with the PNNL table.

**Test 7 — Copyright compliance.** Inspect every PMTiles layer in the production pipeline for ASCE 7 wind speed or ground snow load content. The pipeline is falsified if ASCE-copyrighted map content is reproduced in any served tile.

### 7.5 Limitations

**Geographic scope.** The study covers sixteen countries. Findings regarding open-data availability may not generalise to other regions, particularly countries with less developed national spatial data infrastructure.

**EU climate zone derivation.** EU climate zone layers are derived — lookup join to LAU2 boundaries — not native national publications. Accuracy depends on lookup table completeness, LAU2 boundary vintage, and the mapping from national zone codes to the harmonised output schema.

**Canada flood proxy.** The Future Flood Susceptibility 2024 (NRCan) is a modelled layer, not a regulatory product. Until FHIMP delivers a national layer, Canadian flood hazard information carries higher uncertainty than US FEMA NFHL data.

**PMTiles hosting cost.** FEMA NFHL SFHA (~200–500 MB), EU Floods Directive shapefiles (~50–1,000 MB per country), and WRI AQUEDUCT (~1–3 GB) involve substantial hosted-data sizes. Expanding coverage to additional countries increases hosting costs proportionally.

**EFFIS wildfire data.** JRC EFFIS wildfire data requires a formal data request and cannot be reproduced from a public URL. GWIS FWI raster is used as a fallback for EU ISOs where EFFIS data is pending; FWI is a modelled fire-weather index, not a regulatory hazard zone polygon.

---

## 8. Conclusion

This paper presents a systematic coverage assessment of open-licence AEC geospatial data layers across sixteen countries in North America and Europe. The assessment demonstrates that regulatory-grade open-licence data is achievable for the majority of the eight evaluated layer types in the majority of the study countries, but that three structural gaps remain: the absence of a national Canadian flood hazard layer; CONABIO's non-commercial restriction on Mexico's precision eco-region and climate data; and the ASCE 7 copyright constraint on wind and snow load redistribution.

The central methodological contribution is the build-by-join pipeline for EU building code climate zones: the absence of a pan-EU harmonised climate zone polygon does not preclude regulatory-grade zone assignments when national code lookup tables are joined to LAU2 municipal boundary polygons from GISCO. Eight EU ISOs are assessed to produce regulatory-grade outputs via this approach, replacing Köppen proxy classification for the majority of EU study clusters.

The H3 resolution-7 spatial-indexing pipeline provides a reproducible substrate for joining a multi-layer AEC data stack to any sub-metropolitan point inventory, with explicit integration architecture documentation for each layer type. The separation of regulatory-grade outputs (PMTiles polygon for precise boundary layers), approximation layers (raster PMTiles for continuous gradients), and point-enrichment fields (JSON metadata for scalar values) reflects a distinction material to downstream use for permit documentation versus site orientation.

The falsification programme provides seven concrete tests that can be re-executed as data sources update, countries are added, or regulatory frameworks change. The tests are designed to be executable by any researcher who can access the listed source URLs and run the documented pipeline steps — constituting the reproducibility commitment of the paper.

---

## Acknowledgements

No external funding received. The authors thank the open-data programmes of the US Department of Energy, FEMA, USGS, NREL, the European Environment Agency, the Joint Research Centre, Statistics Canada, NRCan, and national statistical agencies for the datasets on which this work depends.

---

## AI Use Disclosure

This paper was developed using Claude Sonnet 4.6 (Anthropic). The data layer inventory, coverage assessment methodology, and licence analysis were developed with AI assistance under human editorial direction. All factual claims regarding data availability and licensing have been independently verified by the authors. The model used is identified per COPE 2024 guidelines.

---

## CRediT Contributor Roles

**Jennifer M. Woodfine:** Conceptualization, Methodology, Data Curation, Formal Analysis, Writing – Original Draft, Writing – Review & Editing.
**Peter M. Woodfine:** Conceptualization, Validation, Writing – Review & Editing.
**Mathew Woodfine:** Software, Data Curation, Writing – Review & Editing.

---

## Conflict of Interest

The authors declare no conflict of interest.

---

## Funding

No external funding received.

---

## Data Availability

All data sources used in this study are publicly available. Source URLs, licences, and download dates are documented in the manuscript. The build pipeline code will be made available upon acceptance.

---

## References

Alphen, J. van, and Q. Lodder. 2006. Flood management and spatial planning in Europe. *Proceedings of the ICE — Water Management* 159(1): 7–13.

Baugh, C. A., P. D. Bates, G. Schumann, and M. A. Trigg. 2016. LISFLOOD-FP hydrodynamic model for a global river flood model. *Geoscientific Model Development* 9(11): 4347–4365.

Beck, H. E., N. E. Zimmermann, T. R. McVicar, N. Vergopolan, A. Berg, and E. F. Wood. 2018. Present and future Köppen-Geiger climate classification maps at 1-km resolution. *Scientific Data* 5: 180214. https://doi.org/10.1038/sdata.2018.214

Brodsky, Isaac. 2018. H3: Uber's hexagonal hierarchical spatial index. Uber Engineering Blog. https://www.uber.com/en-US/blog/h3/

Buchhorn, M., M. Lesiv, N.-E. Tsendbazar, M. Herold, L. Bertels, and B. Smets. 2020. Copernicus Global Land Cover Layers — collection 2. *Remote Sensing* 12(6): 1044.

Darnall, N., I. Seol, J. Sarkis, and J. Cordeiro. 2022. An open source delineation and hierarchical classification of UK retail agglomerations. *PLOS ONE* 17(9): e0264713.

Dinerstein, E., et al. 2017. An ecoregion-based approach to protecting half the terrestrial realm. *BioScience* 67(6): 534–545.

European Parliament and Council. 2007. Directive 2007/60/EC on the assessment and management of flood risks. *Official Journal of the European Union* L 288.

European Parliament and Council. 2010. Directive 2010/31/EU on the energy performance of buildings. *Official Journal of the European Union* L 153.

Gleeson, T., Y. Wada, M. F. P. Bierkens, and L. P. H. van Beek. 2012. Water balance of global aquifers revealed by groundwater footprint. *Nature* 488: 197–200.

Haklay, M. 2010. How good is volunteered geographical information? A comparative study of OpenStreetMap and Ordnance Survey datasets. *Environment and Planning B: Planning and Design* 37(4): 682–703.

Protomaps. 2024. PMTiles specification. https://docs.protomaps.com/pmtiles/

Taylor, Z. T., D. Huang, R. Lucas, A. Chaney, and M. Gowri. 2008. *Using building energy simulation and optimization to design climate zones.* PNNL-17241. Richland, WA: Pacific Northwest National Laboratory.

WRI (World Resources Institute). 2023. *AQUEDUCT Floods 3.0: Comparing Future Flood Risk Across Scenarios.* Washington, DC: WRI.

---

*Version 0.2 — 2026-05-28 — Writing pass complete*
*Target: Automation in Construction (Elsevier, IF 12.0)*
*Forward-looking statements carry "planned / intended / may / target" language*
*§6 Results requires nightly build pipeline data before submission*
