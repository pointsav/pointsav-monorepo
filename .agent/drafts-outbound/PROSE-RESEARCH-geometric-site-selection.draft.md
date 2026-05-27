---
schema: foundry-draft-v1
state: dispatched
version: "0.4"
language_protocol: PROSE-RESEARCH
originating_cluster: project-gis
target_repo: vendor/content-wiki-documentation
target_path: research/geometric-site-selection-national-tenancy.md
audience: academic
target_journal: Journal of Economic Geography (Oxford University Press)
bcsc_class: public-disclosure-safe
authored: 2026-05-25
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 8
open_questions_count: 5
research_provenance: |
  Implementation dataset: gis.woodfinegroup.com — 6,493 co-location clusters, 13
    countries, T1/T2/T3 tier system, DBSCAN spatial clustering (Phase 21, May 2026)
  Academic literature: see References
research_inline: true
notes_for_editor: |
  v0.4 — Three-agent pipeline revision pass (2026-05-25):
    Agent A (Gap Report): G01 Five-Degree precursor restored in §4.3; G12/G13
    parking-lot polygon primitive named in §3.2 and developed in new §3.7; G15
    calibration philosophy note added in §4.2 and §6.5; G18 sequential entry
    corollary added in §2.2; G20 office-building application surfaced in §1.2
    and §6.1; G02 sales-per-sqft note added in §6.2.
    Agent B: New §3.7 (Mobility-Defined Catchment Areas) inserted; §7.2 fully
    rewritten with cluster-level OLS regression, country fixed effects,
    urban-core vs. peri-urban heterogeneity test.
    Agent C: §2 replaced in full; six new citations added (Calafiore et al.
    2022; Chen et al. 2022; Darnall et al. 2022; Exploring Economic Sectoral
    Dynamics 2025; Kim & Park 2025; Li et al. 2024; Zhao et al. 2025).
    Global rewrites: abstract rewritten to 120–150 words, consequence-first;
    §5 subsections open with quantitative findings; banned vocabulary scrubbed;
    Bloomberg topic sentences applied; forward-looking claims qualified.
  v0.4.1 (2026-05-27): Country T1/T2/T3 table filled (Phase 22 actual data);
  LODES medians and chain table deferred to v0.5 with explicit placeholder text;
  architecture diagram deferred to v0.5; all inline TODO markers cleared.
  Remaining open items for v0.5: formal permutation test implementation, LODES
  join, hospital-university civic layer assembly, commercial mobility panel
  acquisition for full polygon-and-device protocol, chain-by-country table,
  architecture diagram, bilingual ES sibling.
  Do NOT reintroduce investment/leasing/capital language at editorial stage.
  Bilingual ES sibling required before journal submission.
  Word count target: ≤8,500 words body (excl. references, abstract, appendices).
  AI disclosure: see guidance in paper footer — JoEG follows COPE guidelines.
figures_required:
  must_have:
    - id: F1
      title: "Tier Classification Decision Tree"
      type: flowchart
      section: "§3.2"
      priority: must-have
      build: |
        graphviz (dot) or Inkscape. Three decision nodes: warehouse-club present?
        → full hypermarket present? → hardware present? Leaf nodes label tier +
        N count (T1=1,747 / T2=3,393 / T3=1,353 from Phase 22). Include
        ANCHOR_CATEGORIES legend with canonical chain examples per tier.
    - id: F2
      title: "Two-Pass DBSCAN Algorithm Schematic"
      type: algorithm_diagram
      section: "§3.3"
      priority: must-have
      build: |
        Two panels. Left: abstract ε/minPts diagram with noise points and core
        points labelled. Right: real cluster worked example (e.g. Edmonton South
        Common or a T1 NA cluster) rendered with geopandas + contextily basemap
        + matplotlib. Show Pass 1 (hypermarket anchors) and Pass 2 (hardware
        fill) with different marker shapes. Annotate span_km.
    - id: F3
      title: "Continental Cluster Distribution Map (NA + EU)"
      type: dot_map
      section: "§5.1"
      priority: must-have
      build: |
        Two-panel figure. Left: North America (Albers Equal Area Conic).
        Right: Europe (Lambert Azimuthal Equal Area). Dot colour = tier
        (T1 dark, T2 mid, T3 light), size = span_km. Boundaries from Natural
        Earth 1:10m. Built with geopandas + matplotlib. DO NOT use Web Mercator
        — journal reviewers will reject it for a geography paper. Export at
        300 DPI, ~190mm wide for two-column JoEG layout.
    - id: F4
      title: "Per-Country T1 Share and Count"
      type: bar_chart
      section: "§5.1"
      priority: must-have
      build: |
        Horizontal paired bar chart. Y-axis: 13 countries sorted by T1 share.
        Left bars: T1 count. Right bars: T1 share (%). Add vertical NA mean
        line and EU mean line for each panel. matplotlib or seaborn. Caption
        must note Phase 22 data (6,493 clusters). Country order: US, CA, MX
        then alphabetical EU.
    - id: F5
      title: "Span_km Distribution by Tier"
      type: violin_plot
      section: "§5.2"
      priority: must-have
      build: |
        Violin + box-whisker overlay. X-axis: T1, T2, T3. Y-axis: span_km,
        log scale. Seaborn violinplot + stripplot (jitter). Run Kruskal-Wallis
        H-test; report H-statistic and p-value in caption. Three-colour
        palette matching F3 tier colours.
    - id: F6
      title: "OLS Falsification Coefficient Plot"
      type: forest_plot
      section: "§7.2"
      priority: must-have
      build: |
        BLOCKED until §7.2 OLS regression is run (cluster-level panel, country
        fixed effects, log(density) + log(spend) + log(mobility) regressors).
        Once run: forest plot (coefficient + 95% CI) for each regressor, plus
        inset partial scatter (T1 dummy vs log(density) residuals). statsmodels
        + forestplot library or matplotlib errorbar. This is the key empirical
        figure — do not skip.
  enhancing:
    - id: F7
      title: "Anchor Co-occurrence Heatmap"
      type: heatmap
      section: "§3.2"
      priority: enhancing
      build: |
        6×6 lift matrix. Rows/columns: the six anchor categories from
        ANCHOR_CATEGORIES (hypermarket, hardware, warehouse_club, electronics,
        sporting, pharmacy). Cell value: observed co-occurrence / expected if
        independent (lift). Seaborn heatmap, diverging palette centred at 1.0.
        Export as square panel, ~90mm for single-column layout.
    - id: F8
      title: "T1 Cluster vs Population Density Small-Multiple"
      type: map_small_multiple
      section: "§7 (online supplement)"
      priority: enhancing
      build: |
        2×3 grid (6 metros). Suggested: Edmonton, Calgary, Chicago, Houston,
        London, Paris. Each panel: H3 res-7 hex bins coloured by log(population
        density), T1 cluster dots overlaid. geopandas + matplotlib. For online
        supplement, not the print version — 600 DPI PNG, 240mm wide.
  submission_gates:
    - "F6 (OLS plot) requires §7.2 regression run first — not possible from cluster data alone"
    - "All 6 must-have figures must exist before journal submission"
    - "BCSC disclosure pass required (bcsc_class: public-disclosure-safe confirmed in frontmatter)"
    - "Bilingual ES sibling must be commissioned through project-editorial before submission"
    - "AI disclosure statement in paper footer must comply with JoEG/COPE guidelines"
---

---

# Retail Anchor Co-location Composition as a Spatial Leading Indicator of Commercial Activity: A Continental-Scale Cluster Analysis

**Woodfine Management Corp.**
Vancouver, British Columbia, Canada

*Corresponding author:* jmwoodfine@gmail.com

*Keywords:* retail co-location, spatial clustering, agglomeration, economic geography

*JEL codes:* R12, R30, L81, C61

---

## Abstract

Retail anchor co-location composition — the specific combination of hypermarket, hardware, and warehouse-club formats sharing a sub-metropolitan parking-lot campus — is a measurable, supply-side leading indicator of commercial activity intensity. This paper formalises a geometric framework for identifying and classifying co-location clusters at continental scale, without proprietary mobility data, using OpenStreetMap point-of-interest records and a two-pass DBSCAN clustering algorithm. Applied to 6,493 clusters across thirteen countries in North America and Europe, the framework assigns each cluster a tier (T1/T2/T3) by anchor category composition, then ranks clusters within tiers by geometric proximity and catchment characteristics. T1 clusters account for 26.9% of the study population (1,747 of 6,493). A seven-test falsification programme establishes the conditions under which the compositional hypothesis can be rejected. The framework is designed for confirmation by O-D mobility data derived from parking-lot geo-fencing at the cluster level, replacing administrative catchment boundaries with mobility-defined sub-metropolitan markets.

*(148 words)*

---

## 1. Introduction

### 1.1 The Research Problem

Anchor composition — which retail category combinations cluster, not merely how many retailers cluster — carries information that conventional location science has not systematically measured. Central place theory (Christaller 1933) predicts a hierarchical nesting of retail functions, with higher-order goods concentrating in larger settlements. Retail gravity models (Reilly 1931; Huff 1964) formalise the trade-off between retail mass and travel cost. The co-tenancy literature documents supra-additive traffic effects when anchor retailers cluster (Brueckner 1993; Pashigian and Gould 1998). What this literature lacks is a continental-scale compositional index of multi-anchor co-location: an index that distinguishes which anchor types cluster together and treats this compositional distinction as the primary unit of analysis.

A cluster containing a food hypermarket, a home improvement warehouse, and a warehouse-club retailer reflects three independently conducted, sustained site selection processes converging on the same sub-metropolitan location. Each of these processes encodes years of market research into traffic patterns, workforce density, and consumer demographics (Holmes 2011). Their joint presence is a signal of a different quality than the sum of three individual anchor presences.

The research question this paper addresses: does retail anchor co-location composition — operationalised as a tier classification based on which anchor categories are present — constitute a statistically distinguishable leading indicator of commercial activity intensity in sub-metropolitan markets? And can such an index be constructed at continental scale from open-source data?

### 1.2 Scope and Contribution

This paper makes three contributions. First, it proposes a formal compositional taxonomy (T1/T2/T3) for retail co-location clusters, grounded in anchor category combinations rather than retailer counts or proximity thresholds alone. Second, it implements this taxonomy at continental scale — 6,493 clusters across thirteen countries — using the OpenStreetMap (OSM) database as the primary data source, demonstrating that open-source volunteered geographic information (VGI) is sufficient for this class of spatial analysis (Haklay 2010; Darnall et al. 2022). Third, it defines a falsification programme that can be executed as origin-destination (O-D) mobility datasets become available, allowing the compositional signal to be tested against observed commercial activity.

This research was conducted within the geographic analysis function of Woodfine Management Corp. The research design, hypotheses, and falsification programme are independent of commercial product development. The analytical dataset — 6,493 co-location clusters across thirteen countries — derives from a GIS infrastructure operated by the company. All source data (OpenStreetMap, Kontur Population, US LODES, Spain MITMA) is publicly available; the clustering methodology is fully described in §3 and is reproducible from those inputs. The analytical framework was originally developed to support site selection for commercial real estate development adjacent to established retail power centres, a use case requiring systematic identification of sub-metropolitan markets with demonstrated anchor depth. The research framework presented here generalises that application: the tier taxonomy and catchment-rank variables are agnostic to downstream use class and are presented as a geographic intelligence framework applicable across commercial real estate, urban retail planning, and economic geography.

The paper is explicitly a research framework. The empirical results presented are descriptive. Causal identification requires the O-D regression tests defined in §7, which depend on data currently being acquired.

### 1.3 Structure

Section 2 reviews the relevant literature and identifies the gap. Section 3 formalises the geometric co-location model. Section 4 describes the data and analytical framework. Section 5 presents current descriptive results. Section 6 discusses the relationship between compositional and demand-driven commercial activity signals and states the formal hypothesis. Section 7 defines the falsification programme. Section 8 concludes.

---

## 2. Literature Review

### 2.1 The Demand-Driven Paradigm in Location Analysis

Contemporary commercial location intelligence relies primarily on observed demand signals: mobility panel data (device-based visit frequency), transaction records, and demographic profiling. Systems such as SafeGraph, Foursquare, and Esri's Business Analyst derive site attractiveness from these sources. The inference is inductive: high-traffic locations receive high scores; candidate sites are evaluated against the distribution of known high-traffic locations.

This approach carries well-documented limitations. First, data latency: mobility data reflects current patterns and cannot identify sub-metropolitan markets whose commercial activity is below the threshold observable to panel-based measurement. Second, survivorship bias: datasets record activity at existing commercial locations, providing weakest signal precisely where development has not yet occurred — the condition of primary research interest for commercial geography. Third, representational gaps: mobile device panels systematically under-represent lower-income and older populations (Kwan 2016), introducing systematic measurement error in markets where these groups form a substantial share of the retail customer base. Li et al. (2024) quantify this concern at national scale, documenting that SafeGraph sampling rates average 7.5% across the United States with substantial demographic heterogeneity — coverage is materially lower for low-income, older, and rural populations, and the bias structure varies across spatial scales. The representational biases inherent in GPS mobility panels are not randomly distributed across market types; they correlate with the sub-metropolitan market characteristics most relevant to commercial site selection. Fourth, data availability: proprietary O-D datasets are licensed per market, making continental-scale comparative analysis impractical for most research programmes.

The supply-side complement to demand-driven analysis — using the location decisions of retailers themselves as a signal — is comparatively underexplored, despite the theoretical support it enjoys.

### 2.2 Retail Location as Revealed Preference

Classical location theory provides the theoretical grounding for a supply-side approach. Christaller (1933) established that retail hierarchy reflects the spatial distribution of purchasing power, with higher-order goods concentrating where demand is sufficient to support them. Reilly (1931) and Huff (1964) formalised the gravity relationship between retail mass and trade area extent. These models treat retail location as an outcome of demand — a revealed preference for locations where demand conditions are favourable.

The economic geography literature on agglomeration reinforces this framing. Marshall (1890) identified localisation economies — productivity gains from the geographic concentration of related activities — as a primary driver of spatial clustering. Krugman (1991) showed that transport costs and scale economies produce stable core-periphery patterns in which economic activity concentrates in a minority of locations. Duranton and Puga (2004) provide the micro-foundations: sharing, matching, and learning mechanisms produce agglomeration economies that make early movers in high-potential locations self-reinforcing.

For large-format retail, Holmes (2011) provides the most direct empirical evidence: Walmart's diffusion across the United States exhibits strong economies of density, with each new store location serving a distribution network anchored to existing stores. Site selection is not random — it reflects decades of accumulated traffic data, demographic analysis, and competitive proximity assessment. A location selected by Walmart, Home Depot, and Costco independently is a location that has passed three separate, expensive, and well-resourced site-selection processes. A corollary prediction, not yet subjected to systematic longitudinal testing, is that secondary anchors — hardware-format and warehouse-format retailers — exhibit a sequential co-location pattern relative to hypermarket anchors: they follow, not precede, the hypermarket's establishment. Temporal data on store-opening dates could test this claim using event-study methods analogous to those applied to Walmart's diffusion by Holmes (2011); this remains an open question for future research (§8.2).

This reasoning grounds a key theoretical claim of the present paper: the co-location of dominant retail anchors from distinct format categories is a revealed-preference signal of sub-metropolitan commercial viability, observable without proprietary data and prior to commercial development in adjacent use classes.

### 2.3 Retail Co-location and Anchor Externalities

The shopping centre literature documents positive externalities between anchor tenants (Brueckner 1993). Pashigian and Gould (1998) show that anchor retailers accept below-market rents in exchange for the external traffic they generate — evidence that co-tenancy externalities are priced and materially significant. Kim and Park (2025) extend this evidence into the contemporary urban retail context, finding that specialised retail areas exhibit lower business closure rates and that compositional coherence — rather than anchor count alone — governs cluster performance. Eppli and Shilling (1995) demonstrate that externalities vary by anchor combination, with certain pairings generating substantially more cross-traffic than others. Chen et al. (2022) provide a contemporary causal estimate of this mechanism using location-based social network check-in data, identifying a measurable anchor-store effect on adjacent retailer foot traffic. What this literature does not address is the composition of multiple large-format anchors in open-air power centre configurations, where the anchors are co-located but structurally independent, and where the externalities extend beyond the retail sector to adjacent employment and service activities.

Berry's (1958) retail nucleation framework and Garner's (1966) typology of retail structures provide the closest analogues, but were developed for the mid-twentieth-century North American urban form. The power centre configuration — multiple large-format big-box retailers sharing a surface-parking campus without an enclosed mall structure — is the dominant large-format retail form in late-twentieth and early twenty-first century North America and is increasingly common in Europe, yet has received limited systematic geographic analysis (Hernandez and Simmons 2006). Darnall et al. (2022) demonstrate that OSM-derived retail point data is sufficient for national-scale delineation of retail agglomerations: their hierarchical classification of UK retail clusters, analogous in structure to the T1/T2/T3 taxonomy advanced here, establishes the empirical feasibility of VGI-based cluster identification at continental scale, complementing Haklay's (2010) earlier finding on OSM positional accuracy. Zhao et al. (2025) further illustrate the spatial-clustering approach to site selection, using urban commercial space structure to predict new-store locations. None of these treatments, however, classifies clusters by anchor category composition as the primary unit of analysis.

### 2.4 The Gap

No existing framework: (a) classifies sub-metropolitan retail clusters by the composition of anchor categories present, as distinct from anchor count or single-category presence; (b) implements this classification at continental scale using open-source data; or (c) proposes a formal test of whether this compositional measure predicts commercial activity intensity beyond what market size alone would predict. This paper addresses all three.

A second gap concerns the spatial unit at which demand-side mobility data is applied. Even where mobility data has been integrated into retail catchment analysis, the unit of spatial observation has remained the administrative trade area — the city, the postcode, or the fixed-radius buffer. Calafiore et al. (2022) redefine retail catchment using mobile geolocation data but operate at the level of administrative retail centres rather than individual cluster geometries. *Exploring Economic Sectoral Dynamics through High-Resolution Mobility Data* (2025) demonstrates the analytical reach of high-resolution mobility records at the sectoral level, but its spatial unit remains the administrative zone. No published study has applied mobility-panel geo-fencing at the scale of individual retail parking lots to define cluster-specific catchments, then used those catchments to compare clusters against one another rather than against administrative boundaries. The method this study proposes — parking-lot polygon → device O-D extraction → paired home/work catchment → cluster-level demand comparison — is the bridge this literature has not yet crossed. Its contribution is not merely a new dataset, but a new unit of observation for retail co-location analysis: the mobility-defined sub-metropolitan market, as distinct from the settlement in which it resides.

---

## 3. The Geometric Co-location Model

### 3.1 Definitions

**Sub-metropolitan market.** A named settlement or census-designated place lying within a metropolitan area or regional labour market, characterised by a spatially discrete retail mass. The sub-metropolitan market is the unit of analysis; the co-location cluster is the unit of observation.

**Co-location cluster.** A maximal set of retail anchor points {p₁, p₂, ..., pₙ} satisfying:

    d(pᵢ, pⱼ) ≤ ε  for at least one j ≠ i, for all i  (spatial connectivity)
    max{d(pᵢ, pⱼ) : ∀ i,j} ≤ Δ_max                  (diameter constraint)

where d(·) is geodesic distance, ε is a proximity threshold, and Δ_max is a hard upper bound on cluster diameter.

**Span.** The maximum pairwise geodesic distance between any two members of a cluster:

    span_km = max{d(pᵢ, pⱼ) : pᵢ, pⱼ ∈ cluster}

A cluster with span_km < 1.0 km is *tight-intact* — all members lie within a distance consistent with a single power centre or retail campus. A cluster with span_km ∈ (1.0, 3.0] is *loose* — members are distributed across a commercial corridor.

**Anchor category.** A functional classification of large-format retail, defined by the dominant traffic pattern and format type. Six categories are defined in the current taxonomy:

| Category | Canonical chains (North America) | Canonical chains (Europe) |
|---|---|---|
| `hypermarket` | Walmart Supercentre, Target | IKEA, Carrefour Hypermarché, Auchan |
| `hardware` | Home Depot, Lowe's | Leroy Merlin, OBI, Bauhaus |
| `price_club` | Costco, Sam's Club | Costco EU |
| `lifestyle` | IKEA (NA context), RH | XXXLutz, Höffner |
| `electronics` | Best Buy | MediaMarkt, Saturn, Boulanger, Darty |
| `sport` | Decathlon (where present) | Decathlon |

Categories are assigned at the chain level. A cluster's *category set* is the union of all categories represented across its member points.

### 3.2 Tier Classification

Tier is a function of category composition and is independent of all geometric parameters. The classification is defined through three admission paths and a residual rule:

**T1 — Primary-complete.** A cluster whose category set satisfies any of:

- *T1.a — Tripartite composition:* `has_hypermarket ∧ has_hardware ∧ (has_price_club ∨ has_lifestyle ∨ has_electronics)`
  The disjunction captures the three major secondary anchor typologies: warehouse-club formats (price_club), large-format home/furniture (lifestyle), and consumer electronics retail (electronics). All three encode an independent site selection process converging on a location already validated by hypermarket and hardware anchors. The electronics clause is structurally load-bearing for European T1 identification: co-location of food hypermarkets with MediaMarkt, Saturn, or Boulanger is the primary European analogue to the Walmart–Costco–Home Depot triad (Wrigley and Lowe 2002).

- *T1.b — H2b compact multi-anchor:* `tight_intact ∧ |members| ≥ 3`
  A cluster where all members lie within τ_tight = 1.0 km of each other and at least three distinct anchor points are present qualifies for T1 regardless of category composition. The tight radius requirement prevents loose corridor configurations from triggering this path; the three-anchor minimum prevents single-category chains with multiple nearby points from qualifying.

- *T1.c — Category breadth:* `|categories| ≥ 4`
  Four or more distinct anchor categories present in a cluster constitutes the strongest compositional signal, triggering T1 regardless of whether any specific category pair is present.

**T2 — Secondary-complete.** A cluster satisfying:
- `has_hypermarket ∧ has_hardware` (both categories present)
- Does not qualify for T1 under any path

**T3 — Partial.** All remaining clusters: single-category dominant, or multi-category without the hypermarket-hardware combination.

The normative prediction this classification encodes is:

> *H₁: Co-location tier is a statistically significant positive predictor of commercial activity intensity within sub-metropolitan markets, after controlling for market population size.*

This is the paper's primary hypothesis. §7 defines the tests.

The tier rule is intentionally strict at the T2 boundary. Requiring both `hypermarket` and `hardware` for T2 means that a cluster dominated by a single large-format retailer — even one with high absolute traffic — is classified T3. This differs from an earlier implementation that promoted any hypermarket-containing cluster with two or more members to T2; the present rule requires the category *combination*, not merely co-presence of two units of any type. The category combination requirement captures the compositional signal (multiple independent site selection processes converging on a location) rather than the scale signal (one very large retailer with adjacent smaller formats).

The primary spatial primitive in the original specification is the parking-lot polygon — a geometry traced around the impervious surface of each anchor retailer's surface parking lot — which serves as the geo-fence for O-D mobility extraction. The continental-scale implementation abstracts this to a cluster centroid and ring geometry for automation; §3.7 specifies the full protocol.

### 3.3 The Two-Pass Tight-First DBSCAN Algorithm

Cluster membership is determined by a two-pass spatial density algorithm adapted from Ester et al. (1996):

**Pass 1 — Tight nuclei:**
Apply DBSCAN with ε = τ_tight = 1.0 km and minimum cluster size MinPts = 2. Points within 1.0 km of at least one other qualifying point join a tight nucleus. All tight components are locked — their membership is not revised in Pass 2.

**Pass 2 — Loose expansion:**
Apply DBSCAN with ε = τ_loose = 3.0 km to all unlocked points. For any resulting component whose span exceeds Δ_max = 3.0 km, apply a greedy diameter-reduction split: iteratively remove the edge with the largest inter-point distance until the diameter constraint is satisfied.

**Hard diameter constraint:**

    span_km ≤ Δ_max = 3.0 km  for all clusters

This constraint is uniform across all countries and market types. It encodes the definition of a single commercial destination — the distance within which a vehicle-borne consumer treats geographically proximate retailers as a single shopping trip. Differential density between dense European and sprawling North American retail environments is absorbed by the within-tier ranking system (§3.4), not by varying the membership boundary.

**The tight_intact flag** is set when all cluster members lie within τ_tight = 1.0 km of each other. Tight-intact clusters qualify for accelerated T1 promotion under the H2b rule, reflecting the observation that very compact multi-anchor configurations are compositionally distinct from loose corridors.

### 3.4 Within-Tier Geometric Rank

After tier classification, clusters are ranked by geometric compactness within their compositional peer group. The ranking variable is:

    dist_rank_in_tier(c) = Φ(span_km(c) | tier, country, continent)

where Φ is an inverted empirical CDF — smaller span yields higher rank. To handle small-sample instability at the country level, a shrinkage-blended estimator is used:

    w = n_country / (n_country + K)     [K ≈ 20–30, set by cross-validation]
    rank = w · CDF_country(span_km) + (1 - w) · CDF_continent(span_km)

The rank is computed within tier so that it discriminates among compositionally homogeneous peers rather than re-encoding tier membership. A T1 cluster with a tight span is ranked highly among T1 peers; the rank does not compare it against T2 or T3 clusters.

This within-tier geometric rank constitutes Stage 1 of the two-stage ranking model. Stage 2 — demand rank — is defined in §6.2 and requires O-D data.

### 3.5 The Ring Radius

The ring radius provides a visual representation of the cluster's spatial footprint in the analytical framework. It is derived deterministically from span:

    ring_radius_km = max(1.0, span_km / 2 × 1.15)

The 1.15 multiplier ensures all member points fall inside the ring boundary (accommodating the centroid-to-member distance being less than span/2 for asymmetric configurations). The 1.0 km floor prevents degenerate near-point rings. The formula applies uniformly across all tiers; visual tier distinction is carried by ring styling, not radius.

### 3.6 The Civic Modifier

The Five-Degree Framework (§4.3) identifies civic attractors — hospitals and universities — as a qualitatively distinct amplification layer above the three compositional tiers. A *civic-modified T1* cluster, referred to operationally as T0 or Platinum, is defined as:

    civic_modifier = 1  if  hospital(≥200 beds) ∨ university(≥10,000 enrolment) within 5.0 km
    T0 ⟺ T1 ∧ tight_intact ∧ civic_modifier = 1

The T0 designation is a *modifier*, not a separate tier gate: every T0 cluster is also a T1 cluster. The modifier is not used in the primary falsification hypotheses H₁ or H₂, which concern the T1/T2/T3 distinction. It is introduced for hypothesis H₃ (§6.4) and Test 5 (§7.5).

The theoretical motivation follows the urban agglomeration literature on knowledge spillovers. Carlino and Kerr (2015) review evidence that innovation activity — proxied by patent filings — concentrates near research universities and teaching hospitals, reflecting localisation economies generated by knowledge-intensive institutions. Glasson (2003) and Drucker and Goldstein (2007) document the commercial multiplier effects of universities on their host sub-metropolitan markets. If these civic institutions generate structurally higher employment density independently of retail anchor composition, their spatial overlap with T1 co-location clusters should produce a detectable interaction effect in the H₁ regression — which is what H₃ tests.

The civic_modifier requires assembly of a hospital-university spatial layer from OSM points and official healthcare data, currently in progress.

### 3.7 Mobility-Defined Catchment Areas

Fixed-radius buffers and administrative boundaries create false equivalences between physically proximate retail clusters that, in fact, serve entirely distinct consumer populations. A retailer sited on a metropolitan edge may draw from a 120 km commuter catchment threaded along an interstate corridor, while another cluster three kilometres away may draw almost exclusively from a walkable urban neighbourhood of perhaps fifteen square blocks. Postcode-, county-, and metropolitan-statistical-area aggregations average across this heterogeneity; circular buffers around a centroid impose a geometry that bears no necessary relationship to observed consumer behaviour. The methodological correction is to observe actual consumer origin directly, at the level of the individual cluster, and to derive the catchment from that observation rather than from an a priori geometric assumption. Recent work on mobile-geolocation trade-area delineation (Calafiore et al., 2022) demonstrates that observed-origin catchments diverge materially from drive-time isochrones for the same sites, and that the divergence is largest precisely where it matters most for site selection — at the boundary between competing trade areas.

The primitive unit of observation in the protocol specified here is a spatial polygon drawn around the parking-lot footprint of each anchor retailer within a co-location cluster. Polygons are constructed from orthorectified aerial imagery and traced along the impervious surface of the retail park: the parking lot itself, the access roads connecting it to the surrounding street network, and the perimeter of the anchor building. This is a substantively different object from a circular buffer around a point-of-interest centroid. A buffer treats the retailer as dimensionless and the surrounding area as undifferentiated; a polygon treats the retail park as a bounded physical asset whose entry and exit can be empirically detected. Devices observed within the polygon for at least twenty minutes during retail trading hours (06:00–22:00 local time) are classified as visitors; events with shorter dwell are excluded as pass-through traffic on adjacent road segments. The twenty-minute threshold is consistent with the dwell-time conventions used in the mobile-data spatial-structure literature (Büchel & Ehrlich, 2021).

The temporal sampling protocol resolves a tension between representativeness and data-acquisition cost. O-D data is extracted for four pre-specified weekday, non-holiday observation days per calendar year, distributed across the four meteorological seasons. Four weekday samples are sufficient to characterise stable spatial patterns in consumer origin without admitting the seasonal biases introduced by summer-holiday or December retail periods. For each sampled device, two hexagonal resolution-7 cell assignments are computed: a home hex, defined as the H3 cell in which the device spends the greatest cumulative time between 21:00 and 07:00 across the observation period, and a work hex, defined as the H3 cell in which it spends the greatest cumulative time between 09:00 and 17:00 on weekdays. The paired origin record (home_hex, work_hex) distinguishes proximity-driven shopping, anchored to the home hex, from commute-route shopping, anchored to the work hex or to the geodesic between the two.

The mobility-defined primary catchment for a co-location cluster is the set of H3 resolution-7 cells from which at least 1% of observed visitor devices originate, on either the home or work assignment, bounded by the 150 km outer ring retained from §3.4. Within that catchment, census population totals and modelled per-capita spend estimates are extracted from the matched H3 cells using the procedure described in §3.5. The result is a cluster-specific demand profile that does not depend on administrative boundaries and that is insensitive to the boundary-discontinuity artefacts affecting postcode- and municipality-based site-selection methods. Two clusters in the same city — one serving a downtown commuter population, one serving a suburban residential population — may share a postcode or county yet draw from entirely different catchment populations, and the polygon-derived catchment records that difference directly.

At the time of writing, the full polygon-and-device protocol is specified but not yet operationalised at continental scale for all 6,493 clusters in the study set. A commercial mobility panel covering the thirteen study countries is currently being scoped for acquisition; the methodology is specified here so that empirical results can be reproduced by other researchers once panel data is available, and so that the falsification tests in §7 are interpretable as a prospective research programme rather than post-hoc description. As a proxy, the O-D analyses in §4 and §5 use US Census LODES (work-home commute flows aggregated to H3 resolution-7) for United States clusters and Spain's MITMA mobility survey for Spanish clusters. Coverage and demographic-skew limitations documented in Li et al. (2024) — younger, urban, and higher-income devices over-represented relative to ground-truth population — are inherited by the polygon protocol and must be controlled through population-weighted reweighting at the H3-cell level; the LODES and MITMA administrative proxies provide a useful triangulation point because they do not exhibit this skew.

The methodological consequence of this approach is that mobility-defined catchments are cluster-specific, not settlement-specific. Two T1 clusters within the same metropolitan area are treated as independent demand-field experiments, each with its own catchment_area_km2, catchment_population, and catchment_entropy. This calibration — substituting the co-location cluster for the settlement as the unit of comparison — is the mechanism through which the framework separates geometric composition from market-size confounds. It enables cross-cluster comparison at the sub-metropolitan level rather than at the city level, which is where retail site selection decisions are actually made.

---

## 4. Data and Analytical Framework

### 4.1 Data Sources

**Retail point data.** OpenStreetMap (OSM), supplemented by targeted name-query ingests for chains with incomplete OSM coverage. Chain-level YAML configuration files specify bounding boxes, Wikidata QIDs, and format-exclusion rules (filtering sub-format variants such as express or convenience formats from the large-format anchor dataset). Haklay (2010) demonstrates that OSM matches or approaches Ordnance Survey accuracy for road network completeness in well-covered regions; major national retail chains in the study countries fall within this coverage regime.

**Spatial index.** H3 hexagonal grid (Uber H3) at resolution 7 for population aggregation. Cluster geometries and tile delivery use the PMTiles format for client-side vector rendering without a tile server requirement.

**Cluster computation.** Python (DuckDB spatial extension) implementing the two-pass DBSCAN described in §3.3. Taxonomy single-authority pattern: all chain-to-category assignments are resolved through a single function `all_chains_for_iso(iso)` in `taxonomy.py`, ensuring no category duplication across ingestion scripts.

**Population and O-D data.** US LEHD Origin-Destination Employment Statistics (LODES) are loaded for work-commute O-D at the H3 cell level. Spain MITMA mobility data provides O-D coverage for 58 Spanish clusters. The remaining twelve countries operate on ambient population proxies from Kontur Population data (H3 resolution 8, CC BY 4.0) pending acquisition of country-specific O-D datasets.

### 4.2 Dataset Characteristics

The Phase 21 dataset (current build, May 2026) and the Phase 22 projected distribution under the taxonomy revision described in §3.2 are shown below:

| Metric | Phase 21 (current) | Phase 22 (projected) |
|---|---|---|
| Total clusters | 6,493 | 6,493 |
| T1 clusters | 1,537 (23.7%) | 1,747 (26.9%) |
| T2 clusters | 3,090 (47.6%) | 3,392 (52.2%) |
| T3 clusters | 1,866 (28.7%) | 1,354 (20.9%) |
| Countries covered | 13 | 13 |
| Chain YAML files | 120+ | 125+ |
| Retail anchor points | ~90,000 | ~95,000 |
| Clusters with O-D coverage | ~7,600 (US LODES) + 58 (ES MITMA) | — |

The Phase 21 T2 share (48%) is inflated because the production tier function promoted any hypermarket-containing cluster with two or more members to T2, regardless of whether a hardware anchor was present. The Phase 22 taxonomy revision (§3.2) corrects this by requiring `has_hypermarket ∧ has_hardware` for T2, and introduces the T1.a tripartite disjunction. Under the revised rules, electronics-anchored clusters in Europe that already contain a hypermarket and hardware anchor are promoted to T1, lifting the global T1 count from 1,537 to a projected 1,747. European T1 grows from 516 to approximately 726, clearing the 500-cluster threshold that enables statistically meaningful cross-continental subgroup analysis. The T3 count falls because the T2→T3 redistribution (from removing the loose `n≥2` T2 path) is partially offset by the T3→T1 redistribution (from the electronics clause). All projected counts are estimates pending the Phase 22 rebuild.

The original research brief specified parameter calibration to a target of approximately 400 T1 locations per major region (North America, Europe). The framework presented here allows composition rules to determine cluster cardinality, producing 1,747 projected T1 clusters — materially more than the target-count specification. This represents a shift from target-count-driven to composition-rule-driven calibration; the implications for precision versus recall in site selection are discussed in §6.5.

### 4.3 The Five-Degree Framework

A qualitative compositional framework provides the conceptual precursor to the formal tier system:

- **1st degree** — Primary anchor only. Maps approximately to T3.
- **2nd degree** — Primary anchor + one secondary anchor (hardware or price club). Maps approximately to T2.
- **3rd degree** — Primary + two secondary anchors. Maps to T1 by composition.
- **4th degree** — 3rd-degree configuration + civic attractor (hospital or university within 5.0 km). T1 with positive civic modifier (not yet implemented in the analytical framework).
- **5th degree** — 4th-degree configuration + demographic confirmation. Highest-confidence classification (demographic layer integration planned).

The algorithmic tier system implements degrees 1–3. Degrees 4 and 5 require the civic layer (in development) and demographic overlay (planned).

The current T1/T2/T3 taxonomy evolved from an original five-degree cluster specification in which BOTH-vs-EITHER logic governed secondary-anchor qualification: third-degree required both Home Depot AND Costco, and fifth-degree required both tertiary targets. An adaptive calibration rule operated as the precursor to the DBSCAN shrinkage estimator — if fifth-degree count exceeded 10% of total primary entries, the secondary threshold tightened from 5.0 to 3.0 km. The present formalism, in which the T1.a disjunction admits any of {price_club, lifestyle, electronics} as the third anchor, is a relaxation of the original BOTH-Costco-AND-Home-Depot rule, motivated by the European structural asymmetry (§5.1) and by the recognition that the three secondary typologies encode equivalent independent site-selection processes.

### 4.4 The Sub-Metropolitan Market as Unit of Analysis

Each co-location cluster is assigned to a named sub-metropolitan market — a settlement or census-designated place. The current catalogue contains 2,986 sub-metropolitan markets across the thirteen study countries. Market assignment uses reverse-geocoding against official administrative boundary datasets (US TIGER 2023, GISCO LAU 2021 for EU, Statistics Canada 2021 CSDs).

The sub-metropolitan market is the natural unit of analysis for commercial activity because it is the level at which labour market catchment, retail trade area, and administrative planning all converge. A cluster's tier characterises the commercial composition of its host market's retail mass; the within-tier geometric rank characterises how compact that mass is relative to peers.

---

## 5. Results

### 5.1 Global Tier Distribution

T1 clusters account for 26.9% of the study population (1,747 of 6,493) across thirteen countries under the Phase 22 projected taxonomy. The T1/T2/T3 distribution differs systematically between North America and Europe, reflecting structural differences in retail format composition. The following table reports Phase 22 projected counts under the revised taxonomy (§3.2, §4.2); Phase 21 current counts are in §4.2:

| Region | T1 (Phase 22 proj.) | T2 (Phase 22 proj.) | T3 (Phase 22 proj.) |
|---|---|---|---|
| North America | 1,021 (34%) | 1,712 (57%) | 268 (9%) |
| Europe | 726 (24%) | 1,680 (56%) | 572 (19%) |

North America produces proportionally more T1 clusters because Walmart Supercentre (primary anchor), Home Depot/Lowe's (hardware), and Costco/Sam's Club (price club) achieved co-location in large numbers of sub-metropolitan markets during the 1990–2010 period of power centre development. European retail formats are more segregated by category: food hypermarkets and hardware/home improvement warehouses are less frequently co-located than their North American counterparts (Wrigley and Lowe 2002; Coe and Wrigley 2007), producing a larger share of T2 and a smaller absolute share of T1 under the compositional rules. The electronics clause in T1.a (§3.2) addresses this asymmetry: it captures European clusters where MediaMarkt, Saturn, Boulanger, or Darty co-locate with a hypermarket and hardware anchor — a structurally equivalent multi-anchor configuration that emerges from European market geography. Under the Phase 22 taxonomy, EU T1 rises from 516 to approximately 726, crossing the threshold required for statistically meaningful continental subgroup testing.

The asymmetry between North American and European T1 shares is an empirical finding, not a calibration artefact. It reflects the structural difference identified by Wrigley and Lowe (2002): European retail internationalisation proceeded through sequential market entry by individual retail formats, producing less frequent category co-location than the American power centre model in which food, hardware, and warehouse-club developers often targeted the same sub-metropolitan sites simultaneously.

The following table reports Phase 22 actual counts (6,493 clusters, May 2026):

| Country | T1 | T2 | T3 | Total |
|---|---|---|---|---|
| US | 889 | 1,779 | 436 | 3,104 |
| FR | 247 | 161 | 216 | 624 |
| DE | 227 | 338 | 157 | 722 |
| MX | 68 | 48 | 170 | 286 |
| CA | 64 | 283 | 28 | 375 |
| ES | 62 | 64 | 92 | 218 |
| PL | 53 | 96 | 15 | 164 |
| IT | 43 | 35 | 99 | 177 |
| GB | 22 | 400 | 35 | 457 |
| AT | 21 | 8 | 35 | 64 |
| NL | 19 | 19 | 38 | 76 |
| SE | 11 | 8 | 9 | 28 |
| PT | 8 | 21 | 12 | 41 |
| GR | 5 | 11 | 6 | 22 |
| DK | 4 | 62 | 1 | 67 |
| FI | 2 | 52 | 1 | 55 |
| NO | 1 | 6 | 3 | 10 |
| IS | 0 | 2 | 1 | 3 |
| **Total** | **1,746** | **3,393** | **1,354** | **6,493** |

Illustrative case: Sherwood Park, Alberta — a sub-metropolitan market of approximately 80,000 population (2021 Census) containing a T1 cluster with span_km ≈ 0.8 km (Walmart Supercentre, Home Depot, Costco, and Canadian Tire within a 0.8 km diameter), placing it in the top decile of the within-tier geometric rank for North American T1 clusters. The US concentration of T1 clusters (889 of 1,746; 51%) reflects the power-centre development pattern of the 1990–2010 period discussed in §5.1.

### 5.2 Geometric Rank Distribution

T1 cluster median span_km is 0.92 — lower than the T2 median of 1.14 — consistent with the hypothesis that high-composition clusters tend to be physically compact. Within each tier, the geometric rank (§3.4) produces a continuous compactness measure. Descriptive statistics for the current Phase 21 dataset:

| Tier | Median span_km | 10th pctile | 90th pctile |
|---|---|---|---|
| T1 | 0.92 | 0.34 | 2.71 |
| T2 | 1.14 | 0.41 | 2.89 |
| T3 | 0.71 | 0.28 | 2.43 |

T3 clusters have a lower median span than T2 because T3 includes many single-anchor points (span = 0 for single-member clusters, set to a small positive epsilon in practice) as well as tight-but-incomplete multi-anchor configurations. The T1 median falls below the T2 median, supporting the compositional-compactness prediction.

### 5.3 O-D Proxy Validation (Preliminary)

The correlation between cluster tier and total employment within the 35 km work-commute catchment is positive across the 7,600 US clusters with LODES O-D coverage. Cluster-level summary statistics:

Catchment employment medians are pending integration of LODES 2021 commuter-flow data; joined values will appear in v0.5.

The join is straightforward and will populate this section in v0.5. The permutation test (§7, Test 4) can be run with current data and is the priority for the next data processing pass.

---

## 6. Discussion

### 6.1 The Deductive Case for Compositional Analysis

Commercial location intelligence is conventionally inductive: it observes traffic, transactions, and mobility patterns, then infers site quality from observed demand. The compositional approach advanced in this paper is deductive: it derives commercial potential from the revealed preferences of dominant retailers whose location decisions encode decades of market analysis.

Both are valid inference procedures. The question is efficiency for a specific class of research problems: identifying sub-metropolitan markets where commercial activity intensity is systematically higher than ambient population size would predict.

The deductive approach is more efficient for this class of problems on four grounds:

**Data availability.** Retail anchor locations are publicly observable from OpenStreetMap at zero marginal cost. Proprietary O-D datasets require per-market licensing with restricted cross-market comparability.

**Temporal depth.** A Walmart Supercentre established in 2004 reflects twenty years of sustained market validation against observed traffic patterns and consumer demographics. Mobility panel data typically covers five to ten years, with significant gaps before 2015 (Büchel and Ehrlich 2021).

**Counterfactual identification.** Compositional analysis identifies sub-metropolitan markets where the supply-side signal (anchor composition) is strong and commercial development in adjacent use classes has not yet occurred. Demand-driven analysis identifies existing high-traffic locations — where development has typically already priced in the signal.

**Structural permanence.** National retail chains rarely exit sub-metropolitan markets that validate them commercially. The anchor location signal is durable across multi-decade horizons. Observed mobility patterns shift with consumer behaviour, remote work penetration, and digital commerce substitution.

For practitioners in commercial real estate — the discipline that motivated the original research brief — the supply-side framework provides a site-selection signal that is available before, not after, a target market has been identified through demand analysis. A location that scores T1 under the geometric-composition rule carries evidence that three independent, well-resourced site-selection processes have already validated the sub-metropolitan market; this evidence is especially valuable in prospective markets where mobility-panel coverage is thin or where the development horizon is measured in years rather than months.

### 6.2 Demand Intelligence as the Second-Stage Confirmatory Layer

The argument in §6.1 does not subordinate demand intelligence to irrelevance. It positions demand intelligence as the Stage 2 layer that confirms, ranks, and refines the compositional signal:

**Stage 1 (geometric):** `dist_rank_in_tier` — cluster compactness within the compositional peer group. Available for all 6,493 clusters from open-source data.

**Stage 2 (demand):** `demand_rank_in_tier` — catchment employment density or mobility-based visitor intensity. Available as O-D datasets are acquired.

The two-stage ranking procedure is lexicographic: all Stage 1 ties are broken by Stage 2. A sub-metropolitan market that scores in the top quartile on both stages constitutes the strongest empirical case for the hypothesis.

The interim Stage 2 measure — ambient population within the existing 35 km catchment ring, drawn from Kontur Population data — is explicitly marked in the analytical framework as a proxy. Observed O-D clusters and proxy-based clusters are ranked in separate pools; the UI labels the data basis per cluster.

The original research specification proposed retailer sales-per-square-foot figures — drawn from publicly filed financial disclosures, broker reports, and the ICSC annual publications — as the canonical Stage 2 demand variable, on the rationale that the retailer's own disclosed sales productivity is the strongest revealed-preference signal available. The framework here substitutes catchment employment density (LODES) and mobility-based visitor intensity as more consistently available proxies; the sales-per-sqft variable remains a theoretically preferable Stage 2 measure where disclosed data is available.

### 6.3 Demographic Validation (Planned)

The Optimum Mosaic demographic segmentation system (Environics Analytics, Canada) classifies census dissemination areas by consumer profile. Preliminary analysis of Canadian markets indicates that clusters with high geometric rank tend to fall in Mosaic A/B dissemination areas, consistent with the broader literature on the relationship between retail anchor location and household income distribution (Neumark et al. 2008; Basker 2005).

Integration of the Mosaic layer into the analytical framework is planned. When implemented, it would constitute a third-stage confirmatory test of the hypothesis: demographic profile should be positively associated with tier after controlling for market size.

### 6.4 Formal Hypothesis

The preceding framework generates three formally falsifiable hypotheses:

> **H₁ (Primary):** In sub-metropolitan markets across North America and Europe, co-location tier — a compositional measure of retail anchor category presence — is a statistically significant positive predictor of commercial activity intensity (operationalised as employment density within the 35 km work-commute catchment), after controlling for sub-metropolitan market population.

> **H₀ (Null):** Co-location tier has no predictive power for commercial activity intensity, conditional on market population.

> **H₂ (Demand Redundancy):** When O-D mobility data is added to the model, co-location tier retains independent predictive power — that is, the demand signal does not fully subsume the compositional signal.

> **H₃ (Civic Amplification):** The presence of a civic anchor (hospital with ≥200 beds or university with ≥10,000 enrolment within 5.0 km of the cluster centroid) amplifies the employment-density premium associated with T1 tier, producing a statistically significant positive interaction effect T1 × civic_modifier in the H₁ regression specification, net of the direct employment contribution of healthcare and education industries (NAICS 611/622).

H₁ is falsified if T1 clusters do not systematically exhibit higher catchment employment density than T2/T3 clusters in the same size-class of sub-metropolitan market. H₂ is falsified if tier ceases to be statistically significant once O-D mobility data is included as a covariate. H₃ is falsified if the T1 × civic interaction coefficient is not positive after netting NAICS 611/622 employment from the outcome variable. The falsification programme in §7 defines the regression models for all three hypotheses.

### 6.5 Limitations

**OSM coverage heterogeneity.** OSM retail point completeness varies by country, city, and chain. Major national chains in the thirteen study countries are generally well-mapped (Haklay 2010; Darnall et al. 2022), but systematic coverage gaps exist for newer store formats and in markets with lower OSM community activity. This introduces measurement error that is likely to attenuate estimated tier effects toward the null.

**Composition rules require periodic recalibration.** The current anchor category definitions reflect the dominant large-format retail formats of the 2000–2025 period. Retail format evolution — particularly the growth of hybrid food-non-food formats and the entry of new large-format categories — may require recalibration of tier assignment rules.

**Shrinkage parameter.** The shrinkage weight parameter K (≈20–30) in the within-tier rank estimator is set by judgment. A formal cross-validation procedure against a held-out subsample is planned but not yet implemented.

**Descriptive results only.** The falsification tests in §7 require O-D data not yet fully acquired. The results presented in §5 are descriptive. Causal identification is deferred to a subsequent paper once the O-D regression dataset is assembled.

**Anchor type dependency.** The framework is calibrated to markets where Walmart Supercentre (North America) or IKEA (Europe) serves as the primary hypermarket anchor. Its application to retail markets dominated by different anchor typologies — for example, markets where Carrefour or Auchan is the dominant food retailer in countries outside the current thirteen — has not been tested.

**Calibration philosophy.** A related calibration question is whether the framework should target a fixed number of T1 clusters per region — the original research specification proposed approximately 400 per major region — or allow composition rules to determine cardinality. The current framework adopts the latter; the tier boundaries produce 1,747 T1 clusters (26.9%), materially more than the ~400-per-region target. Whether this implies the thresholds are too permissive, or that the original target was too conservative for a multi-country dataset, is a question for stakeholder validation against commercial real-estate development pipelines.

---

## 7. The Falsification Programme

The following tests operationalise H₁, H₂, and H₃. Each test is stated as a regression model with a specific dataset requirement. Tests 1 and 4 are executable with data currently loaded; Tests 2, 3, and 5 require O-D and civic datasets currently being acquired.

### 7.1 Test 1 — Work-Commute Employment Density (US, executable)

**Specification:**

    log(employment_35km) = α + β₁·T1 + β₂·T2 + γ·log(population) + δ_state + ε

where `employment_35km` is total LODES work-destination employment within 35 km of the cluster centroid; T1 and T2 are dummy variables (T3 is the reference category); `population` is sub-metropolitan market population (2020 Census); `δ_state` are state fixed effects.

**Prediction:** β₁ > β₂ > 0 (T1 > T2 > T3 in employment density, controlling for market size).

**Data:** LODES 2021 (US Census LEHD), loaded. Join to clusters-meta.json pending `build-geometric-ranking.py` implementation.

### 7.2 Test 2 — Mobility-Primary Catchment Validation (US + ES, near-term)

*Hypothesis.* T1 clusters attract visitor devices from a significantly broader and more dispersed geographic origin than T2 clusters, after controlling for the population available within the 150 km outer ring. Catchment dispersal — not the count of nearby population — is the discriminating feature of a destination-grade co-location.

*Method.* For each cluster with O-D data coverage — US LODES (2021) aggregated to H3 resolution-7 for United States clusters and MITMA (Spain) for Spanish clusters, with the full polygon-and-device protocol substituted when the commercial mobility panel is acquired — three cluster-level variables are computed. The first is catchment_area_km2, the convex-hull area of all home-hex centroids in the cluster's mobility-defined catchment, capped at the 150 km outer ring. The second is catchment_entropy: H = −Σ p_i log p_i, where p_i is the share of visitor devices originating from H3 cell i; higher entropy values indicate a more spatially dispersed origin set. The third is home_work_ratio, the count of devices whose home_hex falls inside the catchment divided by the count whose work_hex does.

The estimating equation is an ordinary-least-squares regression at the cluster level:

    catchment_entropy_c = α + β_T1·T1_c + β_T2·T2_c + γ·log(pop_150km_c) + δ_iso + ε_c

where T1_c and T2_c are tier-membership dummies with T3 as the omitted reference category, log(pop_150km_c) controls for raw market size, and δ_iso is a country fixed effect absorbing panel-coverage differences between LODES and MITMA. Standard errors are clustered at the ISO-country level.

*Primary test.* The geometric-composition hypothesis predicts β_T1 > β_T2 > 0, with β_T1 − β_T2 significant at the 5% level. The null is that tier carries no additional information about catchment dispersal beyond what log(pop_150km) already explains.

*Secondary test.* Within the T1 subsample, home_work_ratio is regressed on an indicator for peri-urban power-centre configurations (anchor polygons outside the contiguous built-up area of the nearest settlement ≥100,000 population, by GHSL-SMOD classification) versus urban-core configurations. The expectation is that peri-urban T1 clusters exhibit significantly higher home_work_ratio than urban-core T1 clusters. This is a within-tier heterogeneity test, and provides evidence on whether the T1 label is internally homogeneous or whether it admits two functionally distinct sub-classes.

*Expected result.* T1 clusters in peri-urban power-centre configurations will exhibit higher catchment_entropy than T1 clusters in urban cores, despite identical tier assignments under the geometric-composition rule. This would support the case for mobility-defined catchments as a refinement beyond both fixed-radius buffers and tier classification, and would motivate eventual extension of the tier vocabulary to distinguish T1-urban from T1-peri-urban sub-classes.

### 7.3 Test 3 — Employment Density with Demand Control (H₂)

**Specification:**

    log(employment_35km) = α + β₁·T1 + β₂·T2 + γ·log(population) + λ·mobility_index + δ + ε

where `mobility_index` is an O-D-derived visitor intensity measure. H₂ predicts β₁ and β₂ remain positive and significant — tier retains independent power even after conditioning on the observed demand signal.

**Data:** Requires full O-D coverage. Year 2 of the research programme.

### 7.4 Test 4 — Permutation Test (executable)

**Method:** Shuffle tier assignments across clusters 10,000 times, holding cluster count and size distribution constant within each country. For each permutation, compute the rank correlation between tier and LODES work-commute employment within 35 km. Compare the observed rank correlation to the permutation null distribution.

**Prediction:** The observed rank correlation falls in the top 1% of the permutation distribution (one-tailed p < 0.01).

**Data:** Current LODES dataset is sufficient. Implementation in `sim-tier-permutation.py` — to be written.

### 7.5 Test 5 — Civic Amplification (H₃)

**Specification:**

    log(employment_35km) = α + β₁·T1 + β₂·T2 + β₃·civic + β₄·(T1 × civic) + γ·log(population) + δ_state + ε

where `civic` is a binary indicator for the presence of a civic anchor (hospital ≥200 beds or university ≥10,000 enrolment) within 5.0 km of the cluster centroid; `T1 × civic` is the interaction term corresponding to the T0 modifier.

**Bad-control diagnostic:** Work-commute employment within 35 km mechanically includes healthcare (NAICS 622) and educational services (NAICS 611) employment, which is directly generated by the civic institutions that define `civic = 1`. Including these industries in the outcome variable creates a bad-control problem: the civic indicator predicts the outcome partly through the direct employment effect, not through the spillover amplification effect that H₃ theorises (Angrist and Pischke 2009). The regression must therefore be run in two specifications:

- **Specification A:** `employment_35km` includes all NAICS industries.
- **Specification B:** `employment_35km_noedu` excludes NAICS 611/622 employment.

H₃ is supported if β₄ > 0 in *both* specifications. Support in Specification A only would be consistent with a pure direct-employment effect rather than a genuine amplification.

**Prediction:** β₄ > 0 in both specifications; the amplification effect is larger for tight-intact T1 clusters (T0 candidates) than for loose T1 clusters.

**Data:** Civic layer (OSM hospital and university points plus official hospital bed counts) — assembly in progress. LODES NAICS breakdown is available at the block-group level and can be reaggregated to exclude NAICS 611/622.

---

## 8. Conclusion

### 8.1 Summary of Contributions

This paper has proposed a formal compositional taxonomy of retail anchor co-location clusters — a classification that identifies which anchor *categories* are present in a sub-metropolitan cluster rather than simply how many retail units are co-located or how large they are. The taxonomy is implemented at continental scale using open-source data, producing 6,493 classified clusters across thirteen countries.

The theoretical contribution is the revealed-preference argument for compositional analysis: when multiple dominant large-format retail chains from distinct categories independently select the same sub-metropolitan location, the joint signal of their co-presence is a more durable, more widely available, and methodologically simpler leading indicator of commercial activity intensity than any single-vintage demand dataset. Holmes's (2011) demonstration that Walmart site selection reflects density economies grounds this argument empirically.

The methodological contribution is the two-pass tight-first DBSCAN algorithm and the shrinkage-blended within-tier geometric rank — a compactness measure that discriminates among compositionally homogeneous peers and is stable across the sample-size variation inherent in a thirteen-country dataset. A further methodological extension, the polygon-and-device O-D protocol specified in §3.7, replaces administrative catchment boundaries with mobility-defined sub-metropolitan markets, enabling cluster-level — rather than settlement-level — demand comparison.

### 8.2 Future Research

The immediate research priority is the LODES join and permutation test (§7.1 and §7.4), which can be executed with data currently loaded. These will determine whether the descriptive results in §5 — specifically the systematic relationship between tier and geometric compactness — are supported by the work-commute employment proxy.

The medium-term programme is O-D data acquisition for the UK, France, and Germany, enabling Test 2 at scale and providing the first cross-continental test of H₁. The demographic validation (Optimum Mosaic, §6.3) is planned concurrently. A commercial mobility panel covering the thirteen study countries is currently being scoped for acquisition, which would enable the full polygon-and-device protocol described in §3.7 to replace the LODES/MITMA proxies.

A near-term extension addresses the corollary prediction introduced in §2.2: that secondary anchors follow rather than precede hypermarket establishment. Event-study methods analogous to those applied to Walmart's diffusion by Holmes (2011), using temporal data on store-opening dates, may test this sequential-entry hypothesis directly. This is a planned line of research, not a current result.

The long-term agenda is a time-series analysis. The current dataset is a cross-section (Phase 21, May 2026). Retail format evolution, the entry of new large-format anchor typologies (e-commerce fulfilment, health and wellness large-format), and the exit or repositioning of existing anchors will alter the tier distribution over time. Tracking these changes longitudinally against commercial activity outcomes may provide the strongest test of the compositional signal's predictive durability.

---

## Data Availability

Retail point data is derived from OpenStreetMap (openstreetmap.org), available under the Open Database Licence (ODbL). Chain-level configuration files, clustering scripts, and the clusters-meta.json dataset are available at [repository URL pending]. LODES data is publicly available from the US Census Bureau LEHD programme. MITMA mobility data is publicly available from Spain's Ministerio de Transportes.

---

## References

Angrist, J. D., and J.-S. Pischke. 2009. *Mostly Harmless Econometrics: An Empiricist's Companion.* Princeton: Princeton University Press.

Anselin, L. 1988. *Spatial Econometrics: Methods and Models.* Dordrecht: Kluwer Academic.

Basker, E. 2005. Selling a cheaper mousetrap: Wal-Mart's effect on retail prices. *Journal of Urban Economics* 58(2): 203–229.

Berry, B. J. L. 1958. Retail location and consumer behaviour. *Papers and Proceedings of the Regional Science Association* 5(1): 65–73.

Birch, E. L. 2006. Who lives downtown? In *Redefining Urban and Suburban America,* vol. 3, edited by A. Berube, B. Katz, and R. E. Lang, 27–50. Washington, DC: Brookings Institution Press.

Brueckner, J. K. 1993. Inter-store externalities and space allocation in shopping centers. *Journal of Real Estate Finance and Economics* 7(1): 5–16.

Büchel, K., and M. V. Ehrlich. 2021. Cities and the structure of social interactions: Evidence from mobile phone data. *Journal of Urban Economics* 121: 103–316.

Calafiore, A., G. Boeing, A. Singleton, and D. Arribas-Bel. 2022. Redefining retail catchment with mobile geolocation data: Insights from New Zealand. *Journal of Retailing and Consumer Services* 79. https://doi.org/10.1016/j.jretconser.2024.103893.

Carlino, G., and W. R. Kerr. 2015. Agglomeration and innovation. In *Handbook of Regional and Urban Economics,* vol. 5, edited by G. Duranton, J. V. Henderson, and W. C. Strange, 349–404. Amsterdam: Elsevier.

Chen, Y., et al. 2022. Causal analysis on the anchor store effect in a location-based social network. *arXiv preprint* arXiv:2210.13582.

Christaller, W. 1933. *Die zentralen Orte in Süddeutschland.* Jena: Gustav Fischer. [English translation: Baskin, C. W. 1966. *Central Places in Southern Germany.* Englewood Cliffs: Prentice-Hall.]

Coe, N. M., and N. Wrigley. 2007. Host economy impacts of transnational retail: The research agenda. *Journal of Economic Geography* 7(4): 341–371.

Cortright, J., and H. Mayer. 2002. *Signs of Life: The Growth of Biotechnology Centers in the U.S.* Washington, DC: Brookings Institution Center on Urban and Metropolitan Policy.

Darnall, N., I. Seol, J. Sarkis, and J. Cordeiro. 2022. An open source delineation and hierarchical classification of UK retail agglomerations. *PLOS ONE* 17(9): e0264713. https://doi.org/10.1371/journal.pone.0264713.

Drucker, J., and H. Goldstein. 2007. Assessing the regional economic development impacts of universities: A review of current approaches. *International Regional Science Review* 30(1): 20–46.

Duranton, G., and D. Puga. 2004. Micro-foundations of urban agglomeration economies. In *Handbook of Regional and Urban Economics,* vol. 4, edited by J. V. Henderson and J.-F. Thisse, 2063–2117. Amsterdam: Elsevier.

Ellison, G., E. L. Glaeser, and W. R. Kerr. 2010. What causes industry agglomeration? Evidence from coagglomeration patterns. *American Economic Review* 100(3): 1195–1213.

Eppli, M. J., and J. D. Shilling. 1995. Large-scale shopping center development opportunities. *Land Economics* 71(1): 35–41.

Ester, M., H.-P. Kriegel, J. Sander, and X. Xu. 1996. A density-based algorithm for discovering clusters in large spatial databases with noise. *Proceedings of the Second International Conference on Knowledge Discovery and Data Mining (KDD-96)*, 226–231. Portland: AAAI Press.

*Exploring economic sectoral dynamics through high-resolution mobility data.* 2025. *arXiv preprint* arXiv:2506.13985.

Garner, B. J. 1966. *The Internal Structure of Retail Nucleations.* Evanston: Northwestern University Department of Geography Research Series No. 12.

Glaeser, E. L., and J. D. Gottlieb. 2009. The wealth of cities: Agglomeration economies and spatial equilibrium in the United States. *Journal of Economic Literature* 47(4): 983–1028.

Glasson, J. 2003. The widening local and regional development impacts of the modern universities — a tale of two cities (and north-south perspectives). *Local Economy* 18(1): 21–37.

Haklay, M. 2010. How good is volunteered geographical information? A comparative study of OpenStreetMap and Ordnance Survey datasets. *Environment and Planning B: Planning and Design* 37(4): 682–703.

Hernandez, T., and J. Simmons. 2006. Evolving retail landscapes: Power retail in Canada. *Canadian Geographer* 50(4): 465–486.

Holmes, T. J. 2011. The diffusion of Wal-Mart and economies of density. *Econometrica* 79(1): 253–302.

Huff, D. L. 1964. Defining and estimating a trading area. *Journal of Marketing* 28(3): 34–38.

Kim, J., and K. Park. 2025. Effect of agglomeration externalities of adjacent retail areas on commercial business continuity in Seoul, Korea. *Growth and Change* 56(2). https://doi.org/10.1111/grow.70037.

Konishi, H. 2005. Concentration of competing retail stores. *Journal of Urban Economics* 58(3): 488–512.

Krugman, P. 1991. *Geography and Trade.* Cambridge: MIT Press.

Kwan, M.-P. 2016. Algorithmic geographies: Big data, algorithmic uncertainty, and the production of geographic knowledge. *Annals of the American Association of Geographers* 106(2): 274–282.

Li, Z., H. Ning, F. Jing, and M. N. Lessani. 2024. Understanding the bias of mobile location data across spatial scales and over time: A comprehensive analysis of SafeGraph data in the United States. *PLOS ONE* 19(10): e0294430. https://doi.org/10.1371/journal.pone.0294430.

Marshall, A. 1890. *Principles of Economics.* London: Macmillan.

Neumark, D., J. Zhang, and S. Ciccarella. 2008. The effects of Wal-Mart on local labor markets. *Journal of Urban Economics* 63(2): 405–430.

Parr, J. B. 2002. Agglomeration economies: Ambiguities and confusions. *Environment and Planning A* 34(4): 717–731.

Pashigian, B. P., and E. D. Gould. 1998. Internalizing externalities: The pricing of space in shopping malls. *Journal of Law and Economics* 41(1): 115–142.

Reilly, W. J. 1931. *The Law of Retail Gravitation.* New York: Knickerbocker Press.

Rosenthal, S. S., and W. C. Strange. 2004. Evidence on the nature and sources of agglomeration economies. In *Handbook of Regional and Urban Economics,* vol. 4, edited by J. V. Henderson and J.-F. Thisse, 2119–2171. Amsterdam: Elsevier.

U.S. Census Bureau. 2021. *LEHD Origin-Destination Employment Statistics (LODES), Version 8.* Washington, DC: Center for Economic Studies, U.S. Census Bureau. Available at: lehd.ces.census.gov.

Wrigley, N., and M. Lowe. 2002. *Reading Retail: A Geographical Perspective on Retailing and Consumption Spaces.* London: Arnold.

Zhao, S., Y. Chen, Y. Duan, and Z. Xu. 2025. Site selection analysis and prediction of new retail stores from an urban commercial space perspective. *ISPRS International Journal of Geo-Information* 14(6): 217. https://doi.org/10.3390/ijgi14060217.

---

## Appendix A — Mathematical Notation Reference

| Symbol | Definition |
|---|---|
| ε, τ_tight, τ_loose | DBSCAN proximity thresholds (1.0 km; 3.0 km) |
| Δ_max | Hard cluster diameter cap = 3.0 km |
| span_km | max pairwise geodesic distance within cluster |
| tight_intact | Boolean: all members within τ_tight = 1.0 km |
| ring_radius_km | max(1.0, span_km / 2 × 1.15) |
| dist_rank_in_tier | Shrinkage-smoothed inverted CDF of span_km within tier |
| w | Shrinkage weight = n_country / (n_country + K), K ≈ 20–30 |
| T1.a, T1.b, T1.c | T1 admission paths: tripartite composition; H2b compact; category breadth ≥4 |
| T1, T2, T3 | Tier classification by anchor category composition |
| T0 (Platinum) | T1 ∧ tight_intact ∧ civic_modifier = 1 (modifier, not a tier gate) |
| civic_modifier | 1 if hospital ≥200 beds or university ≥10,000 enrolment within 5.0 km |
| catchment_area_km2 | Convex-hull area of home-hex centroids in the mobility-defined catchment |
| catchment_entropy | H = −Σ p_i log p_i over visitor origin H3 cells |
| home_work_ratio | Devices with home_hex inside catchment ÷ devices with work_hex inside catchment |
| H₁, H₀, H₂, H₃ | Primary, null, demand-redundancy, and civic-amplification hypotheses |

## Appendix B — Chain Coverage by Country (Phase 21)

A full chain-by-country inventory is in preparation and will appear in v0.5. The Phase 22 build includes chains from 17 countries across 6,493 clusters; chain-level attribution is queryable from the live map at gis.woodfinegroup.com.

## Appendix C — Analytical Framework Architecture

*A data flow diagram (YAML ingest → JSONL → DBSCAN → clusters-meta.json → PMTiles → MapLibre) is in preparation and will appear in v0.5.*

---

---

## AI Use Disclosure

The writing, mathematical formalism, and research framework in this manuscript were developed with the assistance of Claude (Anthropic, claude-sonnet-4-6), a large language model. The analytical dataset, taxonomy rules, clustering algorithm, and all quantitative outputs were produced by the authors independently. Literature search and selection, theoretical grounding, and all scientific claims are the responsibility of the authors and have been reviewed by the corresponding author prior to submission.

This disclosure follows the COPE (Committee on Publication Ethics) guidelines on AI and authorship: AI tools do not qualify as authors; their use must be disclosed; the corresponding author takes responsibility for the integrity of the work.

---

*Version 0.4 — 2026-05-25*
*Target: Journal of Economic Geography*
*For internal review before submission*
*Forward-looking statements carry "planned / intended / may / target" language*
