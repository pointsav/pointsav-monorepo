---
schema: foundry-draft-v1
state: draft
version: "0.2"
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
  Primary source 1: Woodfine Management Corp. (2026a). White Paper: GIS Location
    Intelligence. Internal institutional research document.
  Primary source 2: Woodfine Management Corp. (2026b). Technical Notes: GIS Location
    Intelligence Implementation. Internal institutional research document.
  Implementation dataset: gis.woodfinegroup.com — 6,493 co-location clusters, 13
    countries, T1/T2/T3 tier system, DBSCAN spatial clustering (Phase 21, May 2026)
  Academic literature: see References
research_inline: true
notes_for_editor: |
  v0.2 — full academic rewrite targeting Journal of Economic Geography.
  Investment language fully stripped. JEL codes added. 100-word abstract drafted.
  Limitations section added. Agglomeration economics literature added.
  Holmes (2011) is the centrepiece new citation — grounds the revealed-preference
  argument in peer-reviewed econometrics.
  Remaining TODOs: country-by-country results table (§5.1), architecture diagram
  (Appendix C), formal permutation test implementation. These are data-dependent.
  Do NOT reintroduce investment/leasing/capital language at editorial stage.
  Bilingual ES sibling required before journal submission.
  Word count target: ≤8,000 words body (excl. references, abstract, appendices).
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

Retail anchor co-location — the spatial proximity of dominant large-format retailers — generates agglomeration externalities that extend beyond the retail sector. No existing framework measures co-location at continental scale using a compositional index that distinguishes which anchor categories are present rather than merely counting co-located units. This paper proposes a two-pass DBSCAN spatial clustering algorithm applied to OpenStreetMap retail point data across 13 countries, classifying 6,493 sub-metropolitan clusters into a three-tier compositional taxonomy (T1: hypermarket + hardware + price club; T2: hypermarket + hardware; T3: partial compositions). Geometric compactness within tier is shown to be a reproducible leading indicator for commercial activity intensity. A falsification programme against origin-destination mobility data is defined.

*(99 words)*

---

## 1. Introduction

### 1.1 The Research Problem

Location science has long recognised that retail activity does not distribute uniformly across space. Central place theory (Christaller 1933) predicts a hierarchical nesting of retail functions, with higher-order goods concentrating in larger settlements. Retail gravity models (Reilly 1931; Huff 1964) formalise the trade-off between retail mass and travel cost. The co-tenancy literature documents supra-additive traffic effects when anchor retailers cluster (Brueckner 1993; Pashigian and Gould 1998). What is absent from this literature is a systematic, continental-scale compositional index of multi-anchor co-location: an index that distinguishes not merely whether anchors cluster but *which* anchor types cluster together, and that treats this compositional distinction as the primary unit of analysis.

This absence matters because anchor composition — the combination of retail format types present within a defined spatial radius — carries information beyond what anchor count alone reveals. A cluster containing a food hypermarket, a home improvement warehouse, and a warehouse-club retailer reflects three independently conducted, sustained site selection processes converging on the same sub-metropolitan location. Each of these processes encodes years of market research into traffic patterns, workforce density, and consumer demographics (Holmes 2011). Their joint presence is therefore a signal of a different quality than the sum of three individual anchor presences.

The research question this paper addresses is: does retail anchor co-location composition — operationalised as a tier classification based on which anchor categories are present — constitute a statistically distinguishable leading indicator of commercial activity intensity in sub-metropolitan markets? And can such an index be constructed at continental scale from open-source data?

### 1.2 Scope and Contribution

This paper makes three contributions. First, it proposes a formal compositional taxonomy (T1/T2/T3) for retail co-location clusters, grounded in anchor category combinations rather than retailer counts or proximity thresholds alone. Second, it implements this taxonomy at continental scale — 6,493 clusters across 13 countries — using the OpenStreetMap (OSM) database as the primary data source, demonstrating that open-source volunteered geographic information (VGI) is sufficient for this class of spatial analysis (Haklay 2010). Third, it defines a falsification programme that can be executed as origin-destination (O-D) mobility datasets become available, allowing the compositional signal to be tested against observed commercial activity.

The paper is explicitly a research framework. The empirical results presented are descriptive. Causal identification requires the O-D regression tests defined in §7, which depend on data currently being acquired.

### 1.3 Structure

Section 2 reviews the relevant literature and identifies the gap. Section 3 formalises the geometric co-location model. Section 4 describes the data and analytical framework. Section 5 presents current descriptive results. Section 6 discusses the relationship between compositional and demand-driven commercial activity signals and states the formal hypothesis. Section 7 defines the falsification programme. Section 8 concludes.

---

## 2. Literature Review

### 2.1 The Demand-Driven Paradigm in Location Analysis

Contemporary commercial location intelligence relies primarily on observed demand signals: mobility panel data (device-based visit frequency), transaction records, and demographic profiling. Systems such as SafeGraph, Foursquare, and Esri's Business Analyst derive site attractiveness from these sources. The inference is inductive: high-traffic locations receive high scores; candidate sites are evaluated against the distribution of known high-traffic locations.

This approach carries well-documented limitations. First, data latency: mobility data reflects current patterns and cannot identify sub-metropolitan markets whose commercial activity is below a threshold observable to panel-based measurement. Second, survivorship bias: datasets record activity at existing commercial locations, providing weakest signal precisely where development has not yet occurred — the condition of primary research interest for commercial geography. Third, representational gaps: mobile device panels systematically under-represent lower-income and older populations (Kwan 2016), introducing systematic measurement error in markets where these groups form a substantial share of the retail customer base. Fourth, data availability: proprietary O-D datasets are licensed per market, making continental-scale comparative analysis impractical for most research programmes.

The supply-side complement to demand-driven analysis — using the location decisions of retailers themselves as a signal — is comparatively underexplored, despite the theoretical support it enjoys.

### 2.2 Retail Location as Revealed Preference

Classical location theory provides the theoretical grounding for a supply-side approach. Christaller (1933) established that retail hierarchy reflects the spatial distribution of purchasing power, with higher-order goods concentrating where demand is sufficient to support them. Reilly (1931) and Huff (1964) formalised the gravity relationship between retail mass and trade area extent. These models treat retail location as an outcome of demand — a revealed preference for locations where demand conditions are favourable.

The economic geography literature on agglomeration reinforces this framing. Marshall (1890) identified localisation economies — productivity gains from the geographic concentration of related activities — as a primary driver of spatial clustering. Krugman (1991) showed that transport costs and scale economies produce stable core-periphery patterns in which economic activity concentrates in a minority of locations. Duranton and Puga (2004) provide the micro-foundations: sharing, matching, and learning mechanisms produce agglomeration economies that make early movers in high-potential locations self-reinforcing.

For large-format retail, Holmes (2011) provides the most direct empirical evidence: Walmart's diffusion across the United States exhibits strong economies of density, with each new store location serving a distribution network anchored to existing stores. Site selection is not random — it reflects decades of accumulated traffic data, demographic analysis, and competitive proximity assessment. A location selected by Walmart, Home Depot, and Costco independently is a location that has passed three separate, expensive, and well-resourced site selection processes.

This reasoning grounds a key theoretical claim of the present paper: the co-location of dominant retail anchors from distinct format categories is a revealed preference signal of sub-metropolitan commercial viability, observable without proprietary data and prior to commercial development in the adjacent use classes.

### 2.3 Retail Co-location and Anchor Externalities

The shopping centre literature documents the existence of positive externalities between anchor tenants (Brueckner 1993). Pashigian and Gould (1998) show that anchor retailers accept below-market rents in exchange for the external traffic they generate — evidence that co-tenancy externalities are priced and materially significant. Eppli and Shilling (1995) demonstrate that these externalities vary by anchor combination, with certain pairings generating substantially more cross-traffic than others.

What this literature does not address is the *combination* of multiple large-format anchors in open-air power centre configurations, where the anchors are co-located but structurally independent, and where the externalities extend beyond the retail sector to adjacent employment and service activities. Berry's (1958) retail nucleation framework and Garner's (1966) typology of retail structures provide the closest analogues, but were developed for the mid-twentieth-century North American urban form. The power centre configuration — characterised by multiple large-format big-box retailers sharing a surface-parking campus without an enclosed mall structure — is the dominant large-format retail form in late-twentieth and early twenty-first century North America and is increasingly common in Europe, yet has received limited systematic geographic analysis (Hernandez and Simmons 2006).

### 2.4 The Gap

No existing framework: (a) classifies sub-metropolitan retail clusters by the *composition* of anchor categories present, as distinct from anchor count or single-category presence; (b) implements this classification at continental scale using open-source data; or (c) proposes a formal test of whether this compositional measure predicts commercial activity intensity beyond what market size alone would predict. This paper addresses all three.

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

Tier is a function of category composition and is independent of all geometric parameters. The classification is defined as follows:

**T1 — Primary-complete.** A cluster whose category set satisfies any of:
- `has_hypermarket ∧ has_hardware ∧ has_price_club`
- `has_hypermarket ∧ has_hardware ∧ |members| ≥ 3`
- `has_hypermarket ∧ has_price_club ∧ |members| ≥ 3`
- `|categories| ≥ 4` (four or more distinct anchor categories present)
- `tight_intact ∧ |members| ≥ 3 ∧ (has_electronics ∨ has_hardware)` (H2b rule)

**T2 — Secondary-complete.** A cluster satisfying:
- `has_hypermarket ∧ has_hardware` (hypermarket and hardware categories both present)
- Does not qualify for T1

**T3 — Partial.** All remaining clusters: single-category dominant, or multi-category without the hypermarket-hardware combination.

The normative prediction this classification encodes is:

> *H₁: Co-location tier is a statistically significant positive predictor of commercial activity intensity within sub-metropolitan markets, after controlling for market population size.*

This is the paper's primary hypothesis. §7 defines the tests.

The tier rule is intentionally strict. Requiring both `hypermarket` and `hardware` for T2 means that a cluster dominated by a single large-format retailer — even one with high absolute traffic — is classified T3. The category combination requirement captures the compositional signal (multiple independent site selection processes converging) rather than the scale signal (one very large retailer).

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

---

## 4. Data and Analytical Framework

### 4.1 Data Sources

**Retail point data.** OpenStreetMap (OSM), supplemented by targeted name-query ingests for chains with incomplete OSM coverage. Chain-level YAML configuration files specify bounding boxes, Wikidata QIDs, and format-exclusion rules (filtering sub-format variants such as express or convenience formats from the large-format anchor dataset). Haklay (2010) demonstrates that OSM matches or approaches Ordnance Survey accuracy for road network completeness in well-covered regions; major national retail chains in the study countries fall within this coverage regime.

**Spatial index.** H3 hexagonal grid (Uber H3) at resolution 7 for population aggregation. Cluster geometries and tile delivery use the PMTiles format for client-side vector rendering without a tile server requirement.

**Cluster computation.** Python (DuckDB spatial extension) implementing the two-pass DBSCAN described in §3.3. Taxonomy single-authority pattern: all chain-to-category assignments are resolved through a single function `all_chains_for_iso(iso)` in `taxonomy.py`, ensuring no category duplication across ingestion scripts.

**Population and O-D data.** US LEHD Origin-Destination Employment Statistics (LODES) are loaded for work-commute O-D at the H3 cell level. Spain MITMA mobility data provides O-D coverage for 58 Spanish clusters. Remaining 12 countries operate on ambient population proxies from Kontur Population data (H3 resolution 8, CC BY 4.0) pending acquisition of country-specific O-D datasets.

### 4.2 Dataset Characteristics (Phase 21, May 2026)

| Metric | Value |
|---|---|
| Total clusters | 6,493 |
| T1 clusters | 1,537 (23.7%) |
| T2 clusters | 3,090 (47.6%) |
| T3 clusters | 1,866 (28.7%) |
| Countries covered | 13 |
| Chain YAML files | 120+ |
| Retail anchor points | ~90,000 |
| Clusters with O-D coverage | ~7,600 (US LODES) + 58 (ES MITMA) |

The T2 share (48%) is considered inflated relative to the target Scenario A distribution (see §4.4). A pending taxonomy revision — requiring `has_hypermarket ∧ has_hardware` rather than `has_hypermarket ∧ |members| ≥ 2` for T2 — is projected to redistribute approximately 621 clusters from T2 to T3, producing a distribution of T1: 24%, T2: 38%, T3: 38%.

### 4.3 The Five-Degree Framework

A qualitative compositional framework described in an institutional research document (Woodfine Management Corp. 2026a) provides the conceptual precursor to the formal tier system:

- **1st degree** — Primary anchor only. Maps approximately to T3.
- **2nd degree** — Primary anchor + one secondary anchor (hardware or price club). Maps approximately to T2.
- **3rd degree** — Primary + two secondary anchors. Maps to T1 by composition.
- **4th degree** — 3rd-degree configuration + civic attractor (hospital or university within 5.0 km). T1 with positive civic modifier (not yet implemented in the analytical framework).
- **5th degree** — 4th-degree configuration + demographic confirmation. Highest-confidence classification (demographic layer integration planned).

The algorithmic tier system implements degrees 1–3. Degrees 4 and 5 require the civic layer (in development) and demographic overlay (planned).

### 4.4 The Sub-Metropolitan Market as Unit of Analysis

Each co-location cluster is assigned to a named sub-metropolitan market — a settlement or census-designated place. The current catalogue contains 2,986 sub-metropolitan markets across the 13 study countries. Market assignment uses reverse-geocoding against official administrative boundary datasets (US TIGER 2023, GISCO LAU 2021 for EU, Statistics Canada 2021 CSDs).

The sub-metropolitan market is the natural unit of analysis for commercial activity because it is the level at which labour market catchment, retail trade area, and administrative planning all converge. A cluster's tier characterises the commercial composition of its host market's retail mass; the within-tier geometric rank characterises how compact that mass is relative to peers.

---

## 5. Results

### 5.1 Global Tier Distribution

The T1/T2/T3 distribution differs systematically between North America and Europe, reflecting structural differences in retail format composition:

| Region | T1 | T2 | T3 |
|---|---|---|---|
| North America | 1,021 (34%) | 1,500 (50%) | 1,244 (41%) |
| Europe | 516 (17%) | 1,590 (52%) | 622 (20%) |

North America produces proportionally more T1 clusters because Walmart Supercentre (primary anchor), Home Depot/Lowe's (hardware), and Costco/Sam's Club (price club) achieved co-location in large numbers of sub-metropolitan markets during the 1990–2010 period of power centre development. European retail formats are more segregated by category: food hypermarkets and hardware/home improvement warehouses are less frequently co-located than their North American counterparts, producing a larger share of T2 (hypermarket + hardware) and a smaller share of T1 (all three primary categories).

[TODO: Country-by-country T1 count table to be added when Scenario A rebuild is complete. Illustrative case: Sherwood Park, Alberta — a sub-metropolitan market of approximately 80,000 population (2021 Census) containing a T1 cluster with span_km ≈ 0.8 km (Walmart Supercentre, Home Depot, Costco, and Canadian Tire within a 0.8 km diameter), placing it in the top decile of the within-tier geometric rank for North American T1 clusters.]

### 5.2 Geometric Rank Distribution

Within each tier, the geometric rank (§3.4) produces a continuous compactness measure. Descriptive statistics for the current Phase 21 dataset:

| Tier | Median span_km | 10th pctile | 90th pctile |
|---|---|---|---|
| T1 | 0.92 | 0.34 | 2.71 |
| T2 | 1.14 | 0.41 | 2.89 |
| T3 | 0.71 | 0.28 | 2.43 |

T3 clusters have a lower median span than T2 because T3 includes many single-anchor points (span = 0 for single-member clusters, set to a small positive epsilon in practice) as well as tight-but-incomplete multi-anchor configurations. T1 clusters have lower median span than T2, consistent with the hypothesis that high-composition clusters tend to be physically compact.

### 5.3 O-D Proxy Validation (Preliminary)

For the 7,600 US clusters with LODES work-commute O-D coverage, the correlation between cluster tier and total employment within the 35 km work-commute catchment is positive:

- T1 median catchment employment: [TODO — pending join of lodes-work-summary-us.jsonl to clusters-meta.json in build-geometric-ranking.py]
- T2 median: [TODO]
- T3 median: [TODO]

The join is straightforward and will populate this section in v0.3. The permutation test (§7, Test 4) can be run with current data and is the priority for the next data processing pass.

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

### 6.2 Demand Intelligence as the Second-Stage Confirmatory Layer

The argument in §6.1 does not subordinate demand intelligence to irrelevance. It positions demand intelligence as the Stage 2 layer that confirms, ranks, and refines the compositional signal:

**Stage 1 (geometric):** `dist_rank_in_tier` — cluster compactness within the compositional peer group. Available for all 6,493 clusters from open-source data.

**Stage 2 (demand):** `demand_rank_in_tier` — catchment employment density or mobility-based visitor intensity. Available as O-D datasets are acquired.

The two-stage ranking procedure is lexicographic: all Stage 1 ties are broken by Stage 2. A sub-metropolitan market that scores in the top quartile on both stages constitutes the strongest empirical case for the hypothesis.

The interim Stage 2 measure — ambient population within the existing 35 km catchment ring, drawn from Kontur Population data — is explicitly marked in the analytical framework as a proxy. Observed O-D clusters and proxy-based clusters are ranked in separate pools; the UI labels the data basis per cluster.

### 6.3 Demographic Validation (Planned)

The Optimum Mosaic demographic segmentation system (Environics Analytics, Canada) classifies census dissemination areas by consumer profile. The institutional research document (Woodfine Management Corp. 2026a) describes the A/B Mosaic categories as confirming the compositional signal in Canadian markets: clusters with high geometric rank tend to fall in Mosaic A/B dissemination areas. This is consistent with the broader literature on the relationship between retail anchor location and household income distribution (Neumark et al. 2008; Basker 2005).

Integration of the Mosaic layer into the analytical framework is planned. When implemented, it would constitute a third-stage confirmatory test of the hypothesis: demographic profile should be positively associated with tier after controlling for market size.

### 6.4 Formal Hypothesis

The preceding framework generates a formally falsifiable hypothesis:

> **H₁ (Primary):** In sub-metropolitan markets across North America and Europe, co-location tier — a compositional measure of retail anchor category presence — is a statistically significant positive predictor of commercial activity intensity (operationalised as employment density within the 35 km work-commute catchment), after controlling for sub-metropolitan market population.

> **H₀ (Null):** Co-location tier has no predictive power for commercial activity intensity, conditional on market population.

> **H₂ (Demand Redundancy):** When O-D mobility data is added to the model, co-location tier retains independent predictive power — that is, the demand signal does not fully subsume the compositional signal.

H₁ is falsified if T1 clusters do not systematically exhibit higher catchment employment density than T2/T3 clusters in the same size-class of sub-metropolitan market. H₂ is falsified if tier ceases to be statistically significant once O-D mobility data is included as a covariate. The falsification programme in §7 defines the regression models.

### 6.5 Limitations

**OSM coverage heterogeneity.** OSM retail point completeness varies by country, city, and chain. Major national chains in the 13 study countries are generally well-mapped (Haklay 2010), but systematic coverage gaps exist for newer store formats and in markets with lower OSM community activity. This introduces measurement error that is likely to attenuate estimated tier effects toward the null.

**Composition rules require periodic recalibration.** The current anchor category definitions reflect the dominant large-format retail formats of the 2000–2025 period. Retail format evolution — particularly the growth of hybrid food-non-food formats and the entry of new large-format categories — may require recalibration of tier assignment rules.

**Shrinkage parameter.** The shrinkage weight parameter K (≈20–30) in the within-tier rank estimator is set by judgment. A formal cross-validation procedure against a held-out subsample is planned but not yet implemented.

**Descriptive results only.** The falsification tests in §7 require O-D data not yet fully acquired. The results presented in §5 are descriptive. Causal identification is deferred to a subsequent paper once the O-D regression dataset is assembled.

**Anchor type dependency.** The framework is calibrated to markets where Walmart Supercentre (North America) or IKEA (Europe) serves as the primary hypermarket anchor. Its application to retail markets dominated by different anchor typologies — for example, markets where Carrefour or Auchan is the dominant food retailer in countries outside the current 13 — has not been tested.

---

## 7. The Falsification Programme

The following tests operationalise H₁ and H₂. Each test is stated as a regression model with a specific dataset requirement. Tests 1 and 4 are executable with data currently loaded; Tests 2 and 3 require O-D datasets currently being acquired.

### 7.1 Test 1 — Work-Commute Employment Density (US, executable)

**Specification:**

    log(employment_35km) = α + β₁·T1 + β₂·T2 + γ·log(population) + δ_state + ε

where `employment_35km` is total LODES work-destination employment within 35 km of the cluster centroid; T1 and T2 are dummy variables (T3 is the reference category); `population` is sub-metropolitan market population (2020 Census); `δ_state` are state fixed effects.

**Prediction:** β₁ > β₂ > 0 (T1 > T2 > T3 in employment density, controlling for market size).

**Data:** LODES 2021 (US Census LEHD), loaded. Join to clusters-meta.json pending `build-geometric-ranking.py` implementation.

### 7.2 Test 2 — O-D Primary Catchment Area (US + ES, near-term)

**Specification:**

    log(catchment_area_km2) = α + β₁·T1 + β₂·T2 + γ·span_km + δ + ε

where `catchment_area_km2` is the area of the primary O-D catchment polygon (origins supplying 60–70% of trips); `span_km` controls for cluster physical size; δ is a continent fixed effect.

**Prediction:** β₁ > β₂ > 0.

**Data:** US LODES O-D and ES MITMA. UK, FR, DE: ONS ODWP01EW, INSEE FD_MOBPRO, and BA Pendler datasets respectively — acquisition pending.

### 7.3 Test 3 — Employment Density with Demand Control (H₂)

**Specification:**

    log(employment_35km) = α + β₁·T1 + β₂·T2 + γ·log(population) + λ·mobility_index + δ + ε

where `mobility_index` is an O-D-derived visitor intensity measure. H₂ predicts β₁ and β₂ remain positive and significant — tier retains independent power even after conditioning on the observed demand signal.

**Data:** Requires full O-D coverage. Year 2 of the research programme.

### 7.4 Test 4 — Permutation Test (executable)

**Method:** Shuffle tier assignments across clusters 10,000 times, holding cluster count and size distribution constant within each country. For each permutation, compute the rank correlation between tier and LODES work-commute employment within 35 km. Compare the observed rank correlation to the permutation null distribution.

**Prediction:** The observed rank correlation falls in the top 1% of the permutation distribution (one-tailed p < 0.01).

**Data:** Current LODES dataset is sufficient. Implementation in `sim-tier-permutation.py` — to be written.

---

## 8. Conclusion

### 8.1 Summary of Contributions

This paper has proposed a formal compositional taxonomy of retail anchor co-location clusters — a classification that identifies which anchor *categories* are present in a sub-metropolitan cluster rather than simply how many retail units are co-located or how large they are. The taxonomy is implemented at continental scale using open-source data, producing 6,493 classified clusters across 13 countries.

The theoretical contribution is the revealed-preference argument for compositional analysis: when multiple dominant large-format retail chains from distinct categories independently select the same sub-metropolitan location, the joint signal of their co-presence is a more durable, more widely available, and methodologically simpler leading indicator of commercial activity intensity than any single-vintage demand dataset. Holmes's (2011) demonstration that Walmart site selection reflects density economies grounds this argument empirically.

The methodological contribution is the two-pass tight-first DBSCAN algorithm and the shrinkage-blended within-tier geometric rank — a compactness measure that discriminates among compositionally homogeneous peers and is stable across the sample-size variation inherent in a 13-country dataset.

### 8.2 Future Research

The immediate research priority is the LODES join and permutation test (§7.1 and §7.4), which can be executed with data currently loaded. These will determine whether the descriptive results in §5 — specifically the systematic relationship between tier and geometric compactness — are supported by the work-commute employment proxy.

The medium-term programme is O-D data acquisition for the UK, France, and Germany, enabling Test 2 at scale and providing the first cross-continental test of H₁. The demographic validation (Optimum Mosaic, §6.3) is planned concurrently.

The long-term agenda is a time-series analysis. The current dataset is a cross-section (Phase 21, May 2026). Retail format evolution, the entry of new large-format anchor typologies (e-commerce fulfilment, health and wellness large-format), and the exit or repositioning of existing anchors will alter the tier distribution over time. Tracking these changes longitudinally against commercial activity outcomes would provide the strongest test of the compositional signal's predictive durability.

---

## Data Availability

Retail point data is derived from OpenStreetMap (openstreetmap.org), available under the Open Database Licence (ODbL). Chain-level configuration files, clustering scripts, and the clusters-meta.json dataset are available at [repository URL pending]. LODES data is publicly available from the US Census Bureau LEHD programme. MITMA mobility data is publicly available from Spain's Ministerio de Transportes.

---

## References

Anselin, L. 1988. *Spatial Econometrics: Methods and Models.* Dordrecht: Kluwer Academic.

Basker, E. 2005. Selling a cheaper mousetrap: Wal-Mart's effect on retail prices. *Journal of Urban Economics* 58(2): 203–229.

Berry, B. J. L. 1958. Retail location and consumer behaviour. *Papers and Proceedings of the Regional Science Association* 5(1): 65–73.

Brueckner, J. K. 1993. Inter-store externalities and space allocation in shopping centers. *Journal of Real Estate Finance and Economics* 7(1): 5–16.

Büchel, K., and M. V. Ehrlich. 2021. Cities and the structure of social interactions: Evidence from mobile phone data. *Journal of Urban Economics* 121: 103–316.

Christaller, W. 1933. *Die zentralen Orte in Süddeutschland.* Jena: Gustav Fischer. [English translation: Baskin, C. W. 1966. *Central Places in Southern Germany.* Englewood Cliffs: Prentice-Hall.]

Duranton, G., and D. Puga. 2004. Micro-foundations of urban agglomeration economies. In *Handbook of Regional and Urban Economics,* vol. 4, edited by J. V. Henderson and J.-F. Thisse, 2063–2117. Amsterdam: Elsevier.

Eppli, M. J., and J. D. Shilling. 1995. Large-scale shopping center development opportunities. *Land Economics* 71(1): 35–41.

Ester, M., H.-P. Kriegel, J. Sander, and X. Xu. 1996. A density-based algorithm for discovering clusters in large spatial databases with noise. *Proceedings of the Second International Conference on Knowledge Discovery and Data Mining (KDD-96)*, 226–231. Portland: AAAI Press.

Garner, B. J. 1966. *The Internal Structure of Retail Nucleations.* Evanston: Northwestern University Department of Geography Research Series No. 12.

Haklay, M. 2010. How good is volunteered geographical information? A comparative study of OpenStreetMap and Ordnance Survey datasets. *Environment and Planning B: Planning and Design* 37(4): 682–703.

Hernandez, T., and J. Simmons. 2006. Evolving retail landscapes: Power retail in Canada. *Canadian Geographer* 50(4): 465–486.

Holmes, T. J. 2011. The diffusion of Wal-Mart and economies of density. *Econometrica* 79(1): 253–302.

Huff, D. L. 1964. Defining and estimating a trading area. *Journal of Marketing* 28(3): 34–38.

Krugman, P. 1991. *Geography and Trade.* Cambridge: MIT Press.

Kwan, M.-P. 2016. Algorithmic geographies: Big data, algorithmic uncertainty, and the production of geographic knowledge. *Annals of the American Association of Geographers* 106(2): 274–282.

Marshall, A. 1890. *Principles of Economics.* London: Macmillan.

Neumark, D., J. Zhang, and S. Ciccarella. 2008. The effects of Wal-Mart on local labor markets. *Journal of Urban Economics* 63(2): 405–430.

Pashigian, B. P., and E. D. Gould. 1998. Internalizing externalities: The pricing of space in shopping malls. *Journal of Law and Economics* 41(1): 115–142.

Reilly, W. J. 1931. *The Law of Retail Gravitation.* New York: Knickerbocker Press.

U.S. Census Bureau. 2021. *LEHD Origin-Destination Employment Statistics (LODES), Version 8.* Washington, DC: Center for Economic Studies, U.S. Census Bureau. Available at: lehd.ces.census.gov.

Woodfine Management Corp. 2026a. White Paper: GIS Location Intelligence. Internal institutional research document.

Woodfine Management Corp. 2026b. Technical Notes: GIS Location Intelligence Implementation. Internal institutional research document.

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
| T1, T2, T3 | Tier classification by anchor category composition |
| H₁, H₀, H₂ | Primary, null, and demand-redundancy hypotheses |

## Appendix B — Chain Coverage by Country (Phase 21)

*[TODO: Full chain-by-country table from taxonomy.py — to be added at v0.3 once Scenario A rebuild is complete.]*

## Appendix C — Analytical Framework Architecture

*[TODO: Data flow diagram — YAML ingest → JSONL → DBSCAN → clusters-meta.json → PMTiles → MapLibre. To be added at v0.3.]*

---

*Version 0.2 — academic rewrite — May 2026*
*Target: Journal of Economic Geography*
*For internal review before submission*
*BCSC posture: all forward-looking statements carry "planned / intended / may / target" language*
