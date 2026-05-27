---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.1"
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
word_count_body: 0
word_count_target: 8000
submission_status: not-submitted
cites: []
forbidden_terms_cleared: false
scaffolded_from:
  - AEC-LAYERS-RESEARCH.md
  - AEC-DATA-PARITY-RESEARCH.md
scaffolded_date: 2026-05-27
notes_for_editor: |
  Scaffolded from AEC-LAYERS-RESEARCH.md (2026-05-20, revised) and
  AEC-DATA-PARITY-RESEARCH.md (2026-05-23, complete). All section bodies
  are TODO stubs — this draft requires a full writing pass.

  Contribution angle:
    First systematic open-license assessment of AEC-relevant geospatial data layers
    (building code climate zones, flood hazard, seismic PGA, solar GHI, wildfire,
    soil, urban heat island, eco-regions) across 16 countries in North America
    and Europe, with a reproducible H3 res-7 spatial-indexing pipeline. Demonstrates
    that regulatory-grade data coverage is achievable for the majority of evaluated
    layers across studied countries using only open-license sources.

  Key findings from research:
    - US: all four Tier 1 layers have complete regulatory-grade open data
    - EU: national climate-zone layers constructable via LAU2 polygon join
      (no pan-EU harmonised shapefile; 8 ISOs have regulatory-grade lookup tables)
    - EU floods: Floods Directive 2007/60/EC → per-country regulatory shapefiles
      for GB/FR/ES/IT/DE (replaces AQUEDUCT proxy for EU)
    - Canada: NECB HOT2000 climate zones open; no national flood hazard layer
      until FHIMP 2024–2028 complete
    - Mexico: CONABIO CC BY-NC blocks precision eco-regions/climate; use INEGI raster
    - Eco-regions: Resolve 2017 CC BY 4.0 as global baseline (846 ecoregions)
    - ASCE 7 wind/snow maps copyrighted — ATC Hazards API point-lookup only
    - Solargis CC BY-SA ShareAlike conflict with EUPL-1.2 → use NSRDB+PVGIS only

  Pre-submission checklist:
    1. Write all section bodies (currently stubs)
    2. forbidden_terms_cleared: run language pass; set to true when clean
    3. ORCID IDs for all three authors required
    4. Build and run the actual data pipeline (Phase 17/18 from AEC-LAYERS-RESEARCH.md)
       so results section has concrete measurement data, not projected estimates
    5. Add parity scorecard table from AEC-DATA-PARITY-RESEARCH.md §7 as a Results table
    6. Word count target: ≤8,000 words body
    7. JoEG-style? No — AutoCon style: tables, figures, technical precision
---

---

# Open-Source Building-Systems Data Layers for Urban-Scale Site Analysis: A Continental Coverage Assessment Across Sixteen Countries

**Woodfine Management Corp.**
Vancouver, British Columbia, Canada

*Corresponding author:* jmwoodfine@gmail.com

*Keywords:* AEC site analysis, building code climate zones, open geospatial data, flood hazard, ASHRAE 169, H3 spatial indexing

---

## Abstract

TODO — 150–250 words. Structure: (1) The problem: AEC professionals evaluating commercial development sites lack a systematic comparison of open-license regulatory-grade geospatial data across jurisdictions. (2) Method: We assess eight AEC-relevant data layer categories across sixteen countries using a common H3 res-7 spatial indexing pipeline, categorising each source by regulatory grade, license, and practical integration method. (3) Result: Quantify coverage rates and identify gaps. (4) Significance: First systematic coverage assessment enabling reproducible open-data AEC layer pipelines at continental scale.

*(TARGET: 148 words)*

---

## 1. Introduction

### 1.1 The Research Problem

TODO — Architects and engineers evaluating commercial development sites require regulatory-grade geospatial data inputs: building code climate zones, flood hazard designations, seismic design categories, solar irradiance for energy calculations, and landscape eco-regions for stormwater compliance. These data exist across dozens of national and sub-national agencies, with inconsistent licensing, format, and spatial resolution. No systematic comparative assessment of open-license coverage across North America and Europe exists.

### 1.2 Scope and Contributions

TODO — Three contributions:
1. A taxonomy of eight AEC-relevant geospatial data layer categories, ranked by regulatory prescriptiveness and typical AEC workflow stage
2. A per-country coverage assessment across 16 countries (US, CA, MX, GB, FR, DE, ES, IT, PL, NL, PT, SE, DK, NO, FI, GR) evaluating regulatory grade, open license, and integration method for each layer type
3. A reproducible H3 res-7 spatial-indexing pipeline enabling the assessed layers to be joined to any sub-metropolitan point inventory

### 1.3 Structure

§2 reviews AEC data requirements and related work. §3 defines the eight layer taxonomy. §4 presents the per-country coverage assessment. §5 describes the integration pipeline. §6 quantifies coverage results. §7 discusses gaps and licensing constraints. §8 concludes.

---

## 2. Background and Related Work

TODO — Cover:
- Central place theory and site selection (Christaller 1933; Huff 1964) — connection to commercial development siting
- Building code climate zone mapping: ASHRAE 169-2013/2020 (PNNL/DOE); IECC 2021
- EU Buildings Directive (2010/31/EU, 2018/844/EU) and national energy code zone systems
- EU Floods Directive 2007/60/EC — regulatory flood hazard mapping requirement
- OpenStreetMap and related open POI data for site inventory
- H3 spatial indexing (Brodsky 2018, Uber) for consistent multi-layer joins
- Gap: no systematic comparative assessment of open-license AEC data coverage at continental scale

---

## 3. AEC Data Layer Taxonomy

### 3.1 Layer categories and regulatory status

| Layer | AEC discipline | Code-prescriptive | Priority tier |
|---|---|---|---|
| Building code climate zones | Energy/HVAC | Yes (US: ASHRAE 169/IECC) | Tier 1 |
| Flood hazard zones | Structural/Site | Yes (SFHA/Floods Directive) | Tier 1 |
| Seismic PGA | Structural | Yes (ASCE 7/Eurocode 8) | Tier 1 |
| Solar GHI | Energy/MEP | Yes (LEED EAc1; code proxies) | Tier 1 |
| Eco-regions | Landscape architecture | No (LEED SS/SITES reference) | Tier 1D |
| Wind design speed | Structural | Yes (ASCE 7 — copyrighted) | Tier 2 |
| Wildfire hazard | Site/Materials | Yes (CA CBC Ch. 7A) | Tier 2 |
| Soil type | Geotechnical/Foundation | No (site-specific required) | Tier 3 |

TODO — develop each category.

### 3.2 Building code climate zones

TODO — ASHRAE 169/IECC (US, county-level, public domain). EU national systems: no pan-EU harmonised shapefile. Pipeline: national code zone lookup table → join to GISCO LAU2 polygons. Eight EU ISOs with regulatory-grade lookup tables enumerated. Four single-reference countries (NL/DK/NO/GB) do not require a zone polygon.

### 3.3 Flood hazard

TODO — FEMA NFHL (US, SFHA-only subset). EU Floods Directive 2007/60/EC: per-country regulatory shapefiles. JRC Global River Flood Hazard Maps v2.1 as global fallback. Per-country download URLs and licenses documented.

### 3.4 Seismic PGA

TODO — USGS NSHM 2023 (US, public domain). ESHM20 European Seismic Hazard Model 2020 (CC BY 4.0). GSHAP global. Raster PMTiles at z3–z10 serving model.

### 3.5 Solar GHI

TODO — NREL NSRDB (US, public domain); PVGIS JRC (EU, free, no ShareAlike); Solargis CC BY-SA licensing issue documented. H3 field approach vs. tile approach.

### 3.6 Eco-regions

TODO — Resolve Ecoregions 2017 CC BY 4.0 (global baseline, 846 ecoregions). EPA Level III (US precision, 85 zones, public domain). EEA Biogeographical Regions 2016 (EU regulatory reference, Habitats Directive). CONABIO Mexico: CC BY-NC blocks commercial use.

---

## 4. Per-Country Coverage Assessment

TODO — Present the parity scorecard table from AEC-DATA-PARITY-RESEARCH.md §7.
Rows = 16 ISOs; columns = 8 layer types; cells = ✓ regulatory-grade / ~ proxy / ✗ no open data.

Key findings to frame:
- US: broadest regulatory-grade open coverage across all four Tier 1 layers
- EU: heterogeneous; GB/FR/ES/IT have regulatory-grade flood data; DE requires WFS ingest
- Canada: NECB climate zones open; flood hazard gap until FHIMP 2024–2028
- Mexico: CONABIO licensing blocks precision eco-regions; INEGI raster as substitute
- Nordic ISOs (SE/DK/NO/FI): single-climate-zone countries; flood and seismic low risk

---

## 5. Integration Pipeline

### 5.1 H3 spatial indexing approach

TODO — Describe H3 res-7 (~1.22 km² average cell area) as the spatial join substrate. Each cluster centroid is assigned an H3 cell; all polygon layers (climate zones, flood zones, eco-regions) are intersection-joined to determine the H3 cell's regulatory designation. Point/raster layers (solar GHI, seismic PGA, soil) are sampled at the cluster centroid directly.

### 5.2 Polygon layers: PMTiles serving

TODO — Layer type decision matrix: which layers serve as PMTiles polygon vs. raster vs. clusters-meta.json field vs. API call-time lookup. Zoom level recommendations. Build pipeline: download → filter → tippecanoe → PMTiles.

### 5.3 Point enrichment: clusters-meta.json fields

TODO — Solar GHI, soil class, urban heat island, groundwater depth stored as H3-keyed fields in the cluster metadata JSON. Zero new tile infrastructure. BentoBox inspector display format.

### 5.4 API call-time lookups: copyright-constrained layers

TODO — ASCE 7 wind/snow (copyrighted): ATC Hazards API point-lookup only; no PMTiles redistribution. Architecture: client-side API call on cluster selection.

---

## 6. Results

TODO — Concrete measurements. For each layer built:
- Build time and pipeline steps
- PMTiles file size
- Coverage percentage across study-area clusters
- Example values from representative T1 clusters in each country

Population the parity scorecard numbers. Count ✓/~/✗ per ISO per layer type.

---

## 7. Discussion

### 7.1 Coverage gaps and their implications

TODO — Key structural gaps:
1. EU: no pan-EU harmonised climate-zone polygon. Build-by-join from national code lookup tables adds maintenance burden; each table needs updating when national codes revise.
2. Canada: no national flood hazard layer (FHIMP 2024–2028). Future Flood Susceptibility 2024 (XGBoost, modelled) is disclosed as proxy.
3. Mexico: CONABIO CC BY-NC blocks commercial precision data for eco-regions and climate. INEGI raster + Resolve Ecoregions 2017 as substitutes. Gap is licensing-structural, not data-structural.
4. ASCE 7 wind/snow: Copyright prevents redistribution as a GIS layer. ATC API as compliant workaround introduces latency and external dependency.

### 7.2 Reproducibility and license transparency

TODO — All layer sources, licenses, and integration methods published. Reproducibility commitment: pipeline rebuild from listed sources should produce byte-compatible output. Copyright-constrained layers explicitly excluded from the open-data pipeline.

### 7.3 Formal Hypotheses

> **H₁ (Coverage Hypothesis).** For the sixteen countries studied, a minimum of six of eight AEC data layer types can be populated using open-license sources (CC BY, OGL, public domain, or equivalent) for the majority (≥50%) of countries in the study set.

> **H₀ (Null).** Fewer than six of eight layer types achieve open-license coverage for ≥50% of the study countries; the remaining layer types require proprietary data or API-only access that limits reproducibility.

> **H₂ (Regulatory Grade).** For the United States — the most data-rich jurisdiction in the study — all four Tier 1 layers (building code climate zones, flood hazard, seismic PGA, solar GHI) are available as regulatory-grade open-license sources suitable for use in permit applications and code-compliance documentation.

### 7.4 Falsification Programme

TODO — Seven tests:

1. **Coverage count test.** Count layers with open-license sources per country. H₁ falsified if <6 of 8 achieve ≥50% country coverage.
2. **Regulatory-grade test (US).** Confirm ASHRAE 169/IECC, FEMA NFHL, USGS NSHM 2023, and NREL NSRDB are each usable in US permit documentation. H₂ falsified if any is rejected by a relevant US authority as non-authoritative.
3. **License compatibility test.** Confirm no CC BY-SA or CC BY-NC layer is included in the open pipeline without documented exception or substitution. Falsified if any such layer is found without documentation.
4. **Reproducibility test.** Re-run build pipeline from listed sources on a clean environment; compare PMTiles output checksums. Falsified if outputs differ due to undisclosed proprietary data dependency.
5. **EU climate zone completeness test.** Confirm eight EU ISOs with documented lookup tables produce correct zone assignments for a test set of 50 municipal centroids from official national publications. Falsified if assignment error rate >5%.
6. **H3 join accuracy test.** For US ASHRAE 169 zones: confirm H3 res-7 cell-to-zone assignments for 100 T1 cluster centroids match PNNL county table within county boundary precision. Falsified if >2% of assignments deviate.
7. **Copyright constraint test.** Confirm that no ASCE 7 wind or snow map content is reproduced in any served tile layer. Falsified if an ASCE-copyrighted map is found in PMTiles output.

### 7.5 Limitations

TODO — Key limitations:
- Study covers 16 countries; findings may not generalise to other geographies
- EU climate-zone layers are derived (lookup join + LAU2 polygons), not native national publications; accuracy depends on lookup table completeness
- Canada flood proxy (Future Flood Susceptibility 2024) is modelled, not regulatory; disclose in UI
- PMTiles file sizes for flood and seismic layers are substantial (FEMA NFHL SFHA ~200–500 MB, AQUEDUCT 1-in-100yr ~1–3 GB); hosting cost scales with coverage expansion
- EFFIS wildfire data requires formal data request; not direct download
- Seismic data for Mexico is intermittently available; pipeline stability not guaranteed

---

## 8. Conclusion

TODO — Summarise: (1) the coverage assessment result; (2) the build-by-join EU pattern as the structural contribution; (3) the three open licensing gaps (CONABIO, ASCE 7, EFFIS) and their documented workarounds; (4) the H3 res-7 pipeline as a reproducible substrate for multi-jurisdiction AEC layer integration.

---

## Acknowledgements

No external funding received. The authors thank the open-data programmes of the US Department of Energy, FEMA, USGS, NREL, the European Environment Agency, the Joint Research Centre, and national statistical agencies for the datasets on which this work depends.

---

## AI Use Disclosure

This paper was developed using Claude Sonnet 4.6 (Anthropic). The data layer inventory, coverage assessment methodology, and license analysis were developed with AI assistance under human editorial direction. All factual claims regarding data availability and licensing have been independently verified by the authors. The model used is identified per COPE guidelines (2024).

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

All data sources used in this study are publicly available. Source URLs and licenses are documented in the manuscript. The build pipeline code will be made available upon acceptance.

---

## References

TODO — Build from AEC-LAYERS-RESEARCH.md and AEC-DATA-PARITY-RESEARCH.md source references. Key citations:

Beck, H.E., N.E. Zimmermann, T.R. McVicar, N. Vergopolan, A. Berg, and E.F. Wood. 2018. "Present and future Köppen-Geiger climate classification maps at 1-km resolution." *Scientific Data* 5: 180214. https://doi.org/10.1038/sdata.2018.214

Brodsky, Isaac. 2018. "H3: Uber's Hexagonal Hierarchical Spatial Index." Uber Engineering Blog. https://www.uber.com/en-US/blog/h3/

Christaller, Walter. 1933. *Die zentralen Orte in Süddeutschland.* Jena: Gustav Fischer.

Dinerstein, Eric, et al. 2017. "An Ecoregion-Based Approach to Protecting Half the Terrestrial Realm." *BioScience* 67 (6): 534–545. https://doi.org/10.1093/biosci/bix014

Gleeson, T., Y. Wada, M.F.P. Bierkens, and L.P.H. van Beek. 2012. "Water balance of global aquifers revealed by groundwater footprint." *Nature* 488: 197–200.

Huff, David L. 1964. "Defining and Estimating a Trading Area." *Journal of Marketing* 28 (3): 34–38.

[TODO: add FEMA, USGS, PNNL, NREL, Eurocodes, ESHM20, WRI AQUEDUCT, JRC, EFFIS citations]

---

*Version 0.1 — scaffolded 2026-05-27*
*Target: Automation in Construction (Elsevier)*
*For internal review before external distribution*
*Forward-looking statements carry "planned / intended / may / target" language per bcsc-disclosure-posture.md*
*All section bodies marked TODO require a full writing pass before submission*
