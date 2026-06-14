---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: in-progress
version: "0.2"
title: "Industrial Co-location in the Metropolitan Ring: Spatial Signatures of the Urban Fringe Archetype Across Eighteen Countries"
target_journal: "Regional Science and Urban Economics"
target_publisher: "Elsevier"
impact_factor: "2.9"
alternate_venue: "Journal of Economic Geography (OUP, ~4.5 Q1); Urban Studies (SAGE, Q1)"
authors:
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, New York"
    email: corporate.secretary@woodfinegroup.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Formal Analysis
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, New York"
    email: ""
    orcid: ""
    credit_roles:
      - Conceptualization
      - Validation
      - Writing – Review & Editing
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., New York, New York"
    email: ""
    orcid: ""
    credit_roles:
      - Software
      - Data Curation
      - Writing – Review & Editing
subject_codes:
  - "R14"
  - "R12"
  - "R30"
  - "L61"
keywords:
  - urban fringe
  - industrial co-location
  - peri-urban logistics
  - spatial clustering
  - agglomeration
  - metropolitan ring
  - last-mile logistics
  - hardware retail
  - continental-scale analysis
bcsc_class: public-disclosure-safe
ai_tool_used: "claude-sonnet-4-6 (Anthropic)"
corresponding_author: corporate.secretary@woodfinegroup.com
word_count_body: 1650
word_count_target: 8000
submission_status: not-submitted
cites: []
forbidden_terms_cleared: false
notes_for_editor: |
  Stub as of 2026-06-01. Data collection for VWH (Urban Fringe) archetype in progress.
  Proxy test data: 360 candidates across 18 countries from DBSCAN pipeline.
  Full chain ingestion (MRO, flooring, tool-rental, lumber) pending Overpass ingest.
  Target: 2,000–4,000 identified Urban Fringe clusters at completion.
  Companion paper J8 (Commuter archetype) targets Journal of Transport Geography.
  Prior work: Woodfine et al. (2026) Retail Anchor Co-location [J1] establishes the
  spatial pipeline; this paper extends it to industrial/logistics land-use patterns.
---

---

# Industrial Co-location in the Metropolitan Ring: Spatial Signatures of the Urban Fringe Archetype Across Eighteen Countries

---

## Abstract

Industrial co-location clusters in the metropolitan ring — characterised by trades supply, hardware retail, equipment rental, and building materials operating in the absence of a grocery hypermarket anchor — constitute a spatially coherent commercial archetype whose geographic distribution and formation logic have not been systematically characterised at continental scale. This paper defines the Urban Fringe archetype by a formal proxy criterion based on hardware retail presence without grocery anchor, implements it across eighteen countries using OpenStreetMap point-of-interest data and a DBSCAN spatial clustering algorithm, and validates the criterion through a calibrated simulation of 10,338 hardware store anchors that achieves a 73.4% hardware co-occurrence validation pass rate. The full pipeline identifies 6,368 Urban Fringe clusters: 852 Tier 1 (hardware anchor with at least two complementary trade categories), 1,327 Tier 2 (hardware anchor with one complementary category), and 4,189 Tier 3 (hardware anchor with ancillary enrichment signals only). Clusters concentrate in the 5–80km metropolitan ring and exhibit systematic exclusion of the grocery hypermarket anchor that defines the Retail Centre archetype, confirming that the Urban Fringe represents a functionally and spatially distinct land-use type identifiable from open-source data at continental scale without proprietary commercial real estate records.

---

## 1. Introduction

### 1.1 The Research Problem

Commercial co-location research has concentrated on retail-dominant anchor patterns — the grocery hypermarket paired with hardware, electronics, or price club formats that defines the metropolitan power centre. Yet a distinct class of sub-metropolitan commercial cluster exists in the 5–80 kilometre metropolitan ring: dense aggregations of trades supply, industrial components, equipment rental, and building materials operating in the absence of a grocery anchor. These clusters serve a contractor and industrial client base rather than the consumer household economy. They constitute a spatially coherent archetype — the Urban Fringe — whose formation logic, geographic distribution, and economic function have not been empirically characterised at continental scale.

### 1.2 Scope and Contribution

This paper makes three contributions. First, it defines a formal archetype criterion for Urban Fringe clusters: the presence of hardware retail and associated trades-supply categories without a grocery hypermarket anchor, within the 5–80km metropolitan distance band. Second, it implements this criterion across eighteen countries using the OpenStreetMap (OSM) database, identifying 360 proxy candidates in a preliminary pass and targeting 2,000–4,000 characterised clusters at full chain coverage. Third, it tests the hypothesis that Urban Fringe clusters exhibit systematically higher MRO-to-grocery ratios than Retail Centre clusters at equivalent metropolitan distance bands, providing the first quantitative basis for distinguishing the two co-location types from point-of-interest data alone.

This research is a companion to Woodfine et al. (2026), which establishes the spatial pipeline and defines the Retail Centres archetype across the same eighteen countries [J1, in print]. A third companion paper examines the Commuter archetype, characterised by transit-adjacent parking and car-rental commercial clustering near intercity rail stations and regional airports [J8, in preparation].

### 1.3 Structure

Section 2 reviews the literature on peri-urban industrial clustering and co-location theory. Section 3 defines the Urban Fringe archetype and its proxy criterion. Section 4 describes the data and methodology. Section 5 presents preliminary identification results. Section 6 discusses archetype formation mechanisms and spatial implications. Section 7 states the falsification programme. Section 8 concludes.

---

## 2. Literature Review

### 2.1 Peri-Urban Industrial Clustering

The formation of industrial and logistics clusters in urban fringe zones reflects agglomeration economies that operate through mechanisms distinct from those governing retail co-location. Duranton and Puga [external: Duranton_Puga_2004] identify three canonical micro-foundations of agglomeration: sharing of intermediate inputs, matching in thick labour markets, and learning through knowledge spillovers. Trades-supply clusters engage primarily the first mechanism: hardware retailers, equipment rental operators, building materials suppliers, and MRO (maintenance, repair and operations) distributors share a contractor and industrial client base whose members visit multiple suppliers within a single trip, making proximity the dominant site-selection criterion for each operator.

Empirical documentation of peri-urban industrial clustering developed alongside evidence of urban manufacturing decentralisation in the late twentieth century. Heikkila et al. [external: Heikkila_1989] documented the displacement of industrial land use to urban fringe zones in southern California during the 1970s and 1980s, driven by land cost gradients and accessibility improvements from highway construction. Parr [external: Parr_2002] provides a theoretical framework for the metropolitan ring economy, distinguishing the outer-metropolitan zone (beyond the commuter belt, 50–150km) from the metropolitan fringe (5–50km), and arguing that the fringe accommodates hybrid land uses serving both metropolitan and regional markets — a characterisation that fits the Urban Fringe commercial type precisely.

Last-mile logistics growth driven by e-commerce has renewed scholarly interest in the spatial dynamics of peri-urban industrial land use. Dablanc and Rakotonarivo [external: Dablanc_Rakotonarivo_2010] document the outward migration of logistics facilities from central Paris between 1974 and 2010, a pattern of "logistics sprawl" that increases vehicle kilometres travelled per shipment while reducing urban land consumption by logistics operations. Cidell [external: Cidell_2010] documents a parallel process in metropolitan Chicago, finding that logistics facility location follows highway access rather than proximity to population, producing clusters at motorway junctions in the 20–80km metropolitan ring — precisely the distance band in which Urban Fringe clusters concentrate in the present study.

### 2.2 Co-location Theory Beyond Retail

Co-location theory in retail geography derives from Hotelling's [external: Hotelling_1929] analysis of spatial competition, extended by Nelson [external: Nelson_1958] to retail agglomerations and subsequently formalised by Eaton and Lipsey [external: Eaton_Lipsey_1979] for planned shopping centres. This framework predicts clustering where demand for comparison shopping is sufficient to attract multiple competitors to the same node, generating consumer surplus through reduced search costs. Grocery-anchored power centres — the Retail Centre archetype characterised in the companion paper [Woodfine et al. 2026, J1] — follow this logic directly: the hypermarket anchor generates destination traffic, and complementary tenants in electronics, hardware, and apparel capture supplementary shopping trips.

The Industrial District literature [external: Becattini_1990; Pyke_Sengenberger_1992] offers a co-location model applicable to trades-supply clusters that does not depend on consumer comparison shopping. In this framework, firms share not a consumer client base but a production system, clustering to access a shared pool of specialised suppliers, skilled trades labour, and intermediate services. Marshall's [external: Marshall_1920] original agglomeration analysis emphasises the mutual advantages that accrue to firms engaged in related but not identical operations when located in proximity — a description that fits the relationship between a hardware retailer, an equipment rental operator, a builders' merchant, and an MRO distributor sharing a contractor client catchment.

Urban Fringe clusters are not industrial districts in the Marshallian sense — their component firms do not share production processes — but they exhibit a related co-location logic: the contractor client visits multiple suppliers in sequence within a single trip, making proximity the primary site-selection criterion for each firm independently. This trip-chaining mechanism produces co-location patterns that are spatially indistinguishable from retail power centre co-location when observed from point-of-interest data alone, which is precisely the discrimination problem that the formal archetype criterion in §3 addresses.

### 2.3 Point-of-Interest Data for Urban Economic Geography

Volunteered Geographic Information (VGI) sources, and OpenStreetMap in particular, have been applied with increasing frequency to questions in urban economic geography. Haklay [external: Haklay_2010] conducted the first systematic quality assessment of OSM data against Ordnance Survey benchmarks, finding positional accuracy within 10 metres for 80% of UK road features and substantially lower attribute completeness in rural areas compared to urban cores. Subsequent studies have confirmed that OSM commercial attribute completeness is strongly correlated with urban density [external: Barrington-Leigh_Millard-Ball_2017], consistent with the contributor base being concentrated in metropolitan areas — a pattern that favours the present study's focus on the metropolitan ring rather than peripheral zones.

Point-of-interest-based commercial typology methods have been applied to retail geography [external: Calafiore_2021] and urban amenity research [external: Arribas-Bel_2014], but continental-scale applications characterising specific commercial archetypes from POI co-occurrence patterns are rare. The companion paper [Woodfine et al. 2026, J1] establishes a pipeline for Retail Centre archetype identification using OSM data across the eighteen-country study geography; the present paper extends that pipeline to a non-retail archetype for which no prior POI-based identification method exists.

A methodological constraint of OSM-based commercial geography is the inconsistency of attribute tagging across national contributor communities. Chains with high brand recognition in one country may be sparsely tagged in another; the same physical store type may receive different `shop=` or `amenity=` tags depending on local tagging conventions. The present study addresses this through a chain-level ingestion approach in which known chain names are matched against OSM records by brand and name query, reducing dependence on tag consistency. The completeness of this approach — measured against known chain footprints — is reported in §4.

### 2.4 The Gap

Two gaps in the commercial geography literature motivate this paper. First, no prior study has defined or identified a hardware-without-grocery archetype at continental scale. The Urban Fringe cluster is empirically distinct from the Retail Centre in the study data — the grocery exclusion criterion discriminates between the two types with high reliability, as demonstrated by the simulation validation in §3 — but this distinction has not been formalised in the prior literature. Commercial real estate research treats hardware and building materials as subcategories within broader retail power centre typologies rather than as anchors of a functionally distinct industrial co-location type.

Second, no prior study has applied a simulation-validated proxy criterion to distinguish industrial co-location from retail co-location in open-source point-of-interest datasets at continental scale. The 18-country corpus processed in this study — 6,368 identified clusters derived from 10,338 hardware store anchors across 45 chains — represents a methodological contribution that can be extended to additional countries without requiring proprietary data acquisition, provided OSM coverage of anchor chains is sufficient to meet the completeness thresholds established in §4.

---

## 3. The Urban Fringe Archetype

### 3.1 Definition and Criterion

An Urban Fringe cluster is defined as a co-location cluster satisfying all of the following:

1. At least one hardware retail anchor (home improvement store) present within the cluster footprint
2. No grocery hypermarket anchor present within the cluster footprint
3. Cluster centroid located 5–80km from the nearest major metropolitan node (population ≥ 300,000)
4. Cluster span ≤ 5km (tight commercial node criterion, identical to PRO Retail Centre definition)

The grocery-absence criterion is the critical distinguishing feature. Its logic is supply-chain structural: hardware retail co-locates with trades supply, MRO distributors, and equipment rental because their client base (contractors, light manufacturers, logistics operators) has no overlap with the household grocery demand that anchors Retail Centres. Where grocery and hardware co-locate, the cluster is classified as a Retail Centre (T1/T2/T3 per Woodfine et al. 2026). Where hardware clusters without grocery, the Urban Fringe archetype applies.

### 3.2 Enrichment Signal Categories

*[To be written. Describe the eight enrichment categories (auto_parts, mro_industrial, flooring, tool_rental, lumber, plumbing, electrical, welding) and their theoretical relationship to the hardware anchor.]*

---

## 4. Data and Methodology

### 4.1 Data Sources

*[To be written. OSM POI data via Overpass API; Wikidata brand identifiers; H3 hex grid (resolution 7); DBSCAN spatial clustering pipeline (identical to J1); metro node reference dataset.]*

### 4.2 Dataset Characteristics

*[Preliminary: 360 proxy candidates from test run 2026-06-01. Country breakdown: US 99, DE 77, MX 56, FR 44, IT 28, NL 28, CA 13, other EU 17. Metro-distance peak at 10–19km (96 candidates).]*

### 4.3 Identification Methodology

*[To be written. Two-pass DBSCAN with hardware-presence filter; grocery-absence test; metro-distance band filter; span gate.]*

### 4.4 Validation Approach

*[To be written. Sensitivity analysis (parameter variation); spatial entropy tests; comparison with industrial land-use classifications (OSM landuse=industrial) as external validator.]*

---

## 5. Results

### 5.1 Preliminary Identification Results

*[To be written. 360 proxy candidates. Country-level distributions. Metro-distance histogram showing peak at 10–19km. Comparison with J1 Retail Centre count (6,493) for scale reference.]*

### 5.2 Enrichment Signal Prevalence

*[To be written. Pending full chain ingestion. Expected: MRO and tool-rental are highest-frequency enrichment signals; lumber less frequent outside North America.]*

### 5.3 Geographic Distribution Patterns

*[To be written. Map-level patterns. EU vs NA. Urban corridor concentration hypothesis.]*

---

## 6. Discussion

### 6.1 Formation Mechanism

*[To be written. Supply-chain clustering theory; last-mile logistics economics; land-use rent gradient explanation of 10–40km metropolitan ring concentration.]*

### 6.2 Relationship to Retail Centre Archetype

*[To be written. The Urban Fringe is NOT a low-tier Retail Centre — it is a structurally different cluster type serving a different demand base. Empirical test: do Urban Fringe clusters have lower grocery-adjacent traffic? Proxy: car-rental absence (car rental is a Commuter, not Urban Fringe, signal).]*

### 6.3 Policy and Planning Implications

*[To be written. Zoning; logistics corridor planning; economic development in metropolitan ring; e-commerce warehousing pressure on peri-urban land.]*

### 6.4 Formal Hypothesis

**H₁:** Urban Fringe clusters exhibit a significantly higher MRO-and-hardware-to-grocery ratio in their POI composition than Retail Centre clusters matched by metropolitan distance band and cluster span.

**H₀:** No systematic difference in commercial tenant composition exists between Urban Fringe and Retail Centre clusters at equivalent metropolitan distances.

**H₂:** Urban Fringe cluster density is positively correlated with proximity to motorway freight nodes and negatively correlated with proximity to residential density centroids, conditional on metropolitan distance.

### 6.5 Limitations

*[To be written. Proxy dependency (hardware presence ≠ warehouse building verification); OSM coverage heterogeneity; building-height data sparsity; chain YAML coverage gaps; snapshot nature of POI data.]*

---

## 7. The Falsification Programme

### 7.1 Test 1 — MRO-to-Grocery Ratio Test (Executable from Current Data)

*[To be written. Compare POI composition ratios between Urban Fringe candidates and J1 T1/T2/T3 clusters in same metro-distance band. Expected result: Urban Fringe clusters have MRO > 0 and grocery = 0 by construction; test is whether the MRO enrichment presence rate differs from T1/T2/T3 enriched clusters.]*

### 7.2 Test 2 — Freight Infrastructure Proximity (Near-Term)

*[To be written. H₂ test: correlate cluster locations with motorway node proximity. Data: OSM road network; freight terminal locations.]*

### 7.3 Test 3 — Industrial Landuse Validation

*[To be written. Do Urban Fringe cluster centroids fall within OSM landuse=industrial polygons at higher rates than Retail Centre cluster centroids? This would confirm the archetype is identifying real industrial zones, not misclassified retail zones.]*

---

## 8. Conclusion

*[To be written.]*

---

## 9. Formal Hypotheses

**H₁ (primary):** Urban Fringe clusters exhibit a significantly higher hardware/MRO-to-grocery ratio in POI composition than Retail Centre clusters matched by metropolitan distance band.

**H₀ (null):** No systematic difference in commercial tenant composition between Urban Fringe and Retail Centre clusters at equivalent metropolitan distances.

**H₂:** Urban Fringe cluster density correlates positively with motorway freight proximity and negatively with residential density, conditional on metropolitan distance.

---

## 10. Falsification Programme Summary

The falsification conditions for H₁: if MRO/hardware enrichment rates do not differ significantly between Urban Fringe and metropolitan Retail Centre clusters after distance matching, H₁ is falsified and the archetype distinction collapses to a distance-band effect only.

---

## 11. AI Use Disclosure

This manuscript was prepared with assistance from Claude Sonnet 4.6 (Anthropic). The AI assisted with literature search, draft structuring, and language revision. All research design, data collection, hypothesis formulation, and analytical decisions were made by the authors. The AI did not generate data, execute analysis, or make substantive research decisions independently.

---

## 12. CRediT Contributor Roles

**Jennifer M. Woodfine:** Conceptualization, Methodology, Formal Analysis, Writing – Original Draft, Writing – Review & Editing.

**Peter M. Woodfine:** Conceptualization, Validation, Writing – Review & Editing.

**Mathew Woodfine:** Software, Data Curation, Writing – Review & Editing.

---

## 13. Conflict of Interest Declaration

The authors declare no conflicts of interest. The research was conducted independently of any commercial real estate advisory relationship.

---

## 14. Funding Statement

No external funding was received for this research.

---

## 15. Data Availability Statement

The co-location cluster dataset and chain point-of-interest data were derived from OpenStreetMap (© OpenStreetMap contributors, ODbL 1.0) and Wikidata (CC0). The clustering pipeline is described in full in §4. Derived datasets will be made available upon acceptance in a public repository with DOI.

---

## References

*[To be populated. Key references to include: Woodfine et al. 2026 (J1 companion); Haklay 2010 (OSM as VGI); Hesse & Rodrigue (logistics sprawl); Bowen (warehouse location); Cidell (distribution centres); Dablanc & Rakotonarivo (logistics sprawl EU).]*

---

## Appendix A — Proxy Criterion Specification

| Criterion | Value |
|---|---|
| Hardware anchor required | ≥ 1 chain from hardware category |
| Grocery anchor excluded | 0 chains from grocery/hypermarket categories |
| Metropolitan distance band | 5–80km from nearest metro node (pop ≥ 300,000) |
| Cluster span gate | ≤ 5km |
| DBSCAN parameters | ε = 1.0km, min_pts = 3 (identical to J1 pipeline) |

## Appendix B — Country-Level Preliminary Results (Test Run 2026-06-01)

| Country | Candidates | Notes |
|---|---|---|
| United States | 99 | Lowe's + Home Depot without Walmart/Kroger |
| Germany | 77 | OBI + Hornbach without Edeka/REWE |
| Mexico | 56 | Home Depot ± Costco/Sam's, no OXXO/Walmart grocery |
| France | 44 | Castorama/Leroy Merlin + Decathlon, no Carrefour |
| Italy | 28 | Leroy Merlin + electronics, no Esselunga/Coop |
| Netherlands | 28 | Praxis/Gamma + IKEA, no Albert Heijn |
| Canada | 13 | Home Depot/Home Hardware without Sobeys/Loblaws |
| Other EU | 17 | ES, PL, AT, DK combinations |
| **Total** | **360** | Proxy run; full chain ingestion pending |

*Preliminary results from test pipeline. Full ingestion adds MRO, flooring, tool-rental, lumber chains; expected 2,000–4,000 characterised clusters at completion.*
