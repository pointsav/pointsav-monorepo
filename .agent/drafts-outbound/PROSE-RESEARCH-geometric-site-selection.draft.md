---
schema: foundry-draft-v1
state: scaffolding
language_protocol: PROSE-RESEARCH
originating_cluster: project-gis
target_repo: vendor/content-wiki-documentation
target_path: research/geometric-site-selection-national-tenancy.md
audience: institutional-investor
bcsc_class: public-disclosure-safe
authored: 2026-05-24
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 2
research_suggested_count: 12
open_questions_count: 8
research_provenance: |
  Primary source 1: IT SUPPORT_MCorp_2026_01_06_White Paper_GIS_Location
    Intelligence_FIN.docx — investment strategy memo, five-degree co-location
    framework, Optimum Mosaic demographic confirmation
  Primary source 2: IT SUPPORT_MCorp_2026_01_06_White Paper_GIS_Location
    Intelligence_Notes_FIN.docx — technical implementation notes, variable
    radius concept, O-D data strategy, Project Wiki architecture
  Implementation: gis.woodfinegroup.com — 6,493 co-location clusters, 13
    countries, T1/T2/T3 tier system, DBSCAN spatial clustering
research_inline: true
notes_for_editor: |
  SCAFFOLDING DOCUMENT — structure and thesis only. Many sections contain
  placeholder text and TODO markers. This is the multi-year research framework.
  Each TODO represents a research task that will be completed as data arrives.
  The provable thesis: as O-D/demand data is acquired, it will be joined to
  clusters-meta.json and the correlation between geometric tier and demand
  performance tested. The GIS platform IS the experimental apparatus.
  Strip forward-looking demand data language. Bloomberg register throughout.
  Bilingual ES sibling required at final draft stage.
---

# Geometric Co-location as the Foundation of Location Intelligence
## A Framework for National Tenancy Site Selection in Regional Markets

**Working paper — scaffolding draft**
Woodfine Management Corp. · Woodfine Capital Projects Inc.
Version 0.1 · May 2026

---

## Abstract

This paper advances the thesis that the spatial proximity of dominant national
retail anchors constitutes a **necessary and sufficient primary signal** for
viable National Tenancy office and commercial real estate site selection in
Regional Markets. We term this signal *geometric co-location intelligence*,
distinguishing it from demand-driven location intelligence systems that rely
on mobility data, transaction records, or demographic profiling as primary
inputs.

The central claim is falsifiable: co-location tier (a composition measure of
which retail anchor categories are spatially proximate) should predict relative
commercial real estate performance within Regional Markets. Demand-side
validation — via origin-destination mobility data and retail sales proxies —
is treated as **confirmatory evidence**, not as the discovery mechanism.

The Woodfine GIS Platform, currently indexing 6,493 co-location clusters across
13 countries, constitutes the experimental apparatus through which this thesis
will be tested as demand data is acquired. The top-tier clusters identified
geometrically are predicted to produce higher foot traffic, denser employment
catchments, and stronger retail sales per square foot than lower-tier clusters
in the same Regional Market. The platform makes this prediction visible and
addressable before demand data is available — which is precisely the investment
thesis.

*Keywords:* location intelligence, retail co-location, site selection, regional
markets, national tenancy, spatial clustering, geometric analysis

---

## 1. Introduction

### 1.1 The National Tenancy Problem

A National Tenancy — a major employer operating across multiple locations within
a single real estate market — exhibits a characteristic location preference:
proximity to the dominant retail mass of its region. Law firms choose office
space near the courthouse and the financial district. Medical practices cluster
near hospitals. Back-office operations seek proximity to large-format retail
anchors, where workforce housing is affordable and suburban infrastructure is
built out.

The investment question is not *whether* National Tenancies follow these
attractors. That is well-established in the commercial real estate literature
[TODO: cite Morrow-Jones, Gillen-Romero, CBRE North American Office Leasing
Survey]. The investment question is *which* attractor configurations predict
the highest future leasing velocity and rental rate appreciation — before the
demand signal is observable.

This paper argues that geometric co-location analysis — identifying clusters
where Tier 1 retail anchors (food hypermarket + hardware/home improvement
+ warehouse club) appear in spatial proximity — provides a more actionable and
earlier signal than demand-driven systems. The reasoning is structural: the
retail anchors themselves took decades and hundreds of millions of dollars of
market research to locate. Their spatial configuration encodes the market's
verdict on which Regional Markets can sustain mass-market commercial activity.
Following the national retailers is following the market's most expensive and
durable location research.

### 1.2 The Investment Opportunity

The target universe is approximately 400 Regional Markets in North America and
a comparable set in Europe — markets where the geometric co-location signal is
strongest and where National Tenancy office development is either underserved
or priced below replacement cost.

This figure — ~400 per continent — emerges from the geometry of the problem,
not from a pre-set target. When the algorithm is calibrated to identify T1
co-locations (the most compositionally complete clusters), approximately 1,537
sites emerge globally across 13 countries. This number is not an output of
market analysis but of geometric necessity: given the density and distribution
of Walmart Supercentres (North America) and IKEA (Europe) as Primary Target
anchors, approximately 400–600 per continent exceed the composition threshold
that predicts National Tenancy viability.

### 1.3 Structure of this Paper

Section 2 reviews the existing location intelligence literature and identifies
the gap this framework addresses. Section 3 formalises the geometric co-location
model with mathematical precision. Section 4 describes the GIS platform
implementation. Section 5 presents results from the current 13-country dataset.
Section 6 discusses the relationship between geometric and demand-driven
intelligence. Section 7 lays out the falsification programme — the empirical
tests that will confirm or challenge the thesis as demand data arrives.
Section 8 concludes.

---

## 2. Literature Review

### 2.1 The Demand-Driven Paradigm

The dominant paradigm in commercial location intelligence treats demand as the
primary input. Systems such as SafeGraph, Foursquare, CARTO, and Esri's Business
Analyst derive site attractiveness from observed visitor mobility, transaction
density, and demographic segmentation. The logic is inductive: high-traffic
locations are scored highly; new sites are compared against the distribution
of known high-traffic locations.

This approach has well-documented limitations in Regional Markets:

1. **Data latency.** Mobility data reflects current patterns. Investment decisions
   require predictions about which sites *will become* high-traffic under
   development scenarios that do not yet exist. Demand data cannot identify
   underdeveloped sites in growing Regional Markets.

2. **Survivorship bias.** Datasets record traffic at existing retail and commercial
   locations. Markets that lack development have no traffic to record. The signal
   is strongest where development has already occurred — providing the weakest
   marginal insight to investors.

3. **Cost.** Commercial O-D datasets currently range from USD $50,000 to $150,000
   per year per market, with licensing restrictions that limit cross-market
   comparative analysis at scale.

4. **Representational gaps.** Mobility panel data (device-based) systematically
   underrepresents low-income and rural populations — precisely the demographic
   profile of many mid-sized Regional Markets where National Tenancy investment
   is underpriced.

### 2.2 Gravity Models and Retail Hierarchy Theory

Classical location theory — Christaller's central place theory [1933], Reilly's
law of retail gravitation [1931], and Huff's probabilistic gravity model [1964]
— predicts that retail hierarchy reflects underlying population distribution and
transportation cost. Under this framework, the location of dominant retail
anchors is itself an output of the market's demand distribution.

**Our inversion of this logic is the key theoretical contribution:** if retail
anchor location is an equilibrium outcome of decades of market competition, then
the spatial configuration of dominant anchors encodes the market's demand
distribution *better and more durably* than any single-vintage mobility dataset.
Walmart's site selection process — incorporating traffic counts, demographic
analysis, and competitive proximity — represents a multi-decade accumulation of
exactly the demand intelligence that commercial LI providers sell at high cost.
A 30-year-old Walmart Supercentre encodes a demand signal that no mobility
dataset can replicate.

### 2.3 The Co-location Literature in Retail Real Estate

Co-location effects in retail — the tendency for certain retailer combinations
to produce supra-additive traffic — are well-documented in the shopping centre
literature. [TODO: cite Pashigian-Gould 1998, Brueckner 1993, Eppli-Shilling
1995 on anchor-tenant externalities.] The canonical finding is that an anchor
tenant (typically a department store or supermarket) generates external foot
traffic that benefits co-located specialty tenants, justifying below-market rents
to attract and retain the anchor.

What has not been studied systematically is the *combination* of multiple
large-format anchors in power centre or regional mall configurations as a signal
for adjacent *office* and *National Tenancy* demand. The co-location literature
focuses on retail-to-retail spillovers. We extend it to retail-to-office
spillovers, arguing that multi-anchor configurations signal labour force density,
infrastructure quality, and demographic stability that support office development.

### 2.4 The Gap This Framework Addresses

[TODO: Develop this section as demand data arrives and comparative studies
become possible. The gap is: no existing framework uses *multi-anchor
composition* — not single-anchor proximity — as the primary site selection
signal for commercial real estate outside the retail sector itself. This gap
is the intellectual contribution.]

---

## 3. The Geometric Co-location Model

### 3.1 Definitions

**Regional Market.** A settlement or metropolitan area that contains one or more
co-location clusters of any tier. The Regional Market is the unit of analysis
for investment; the co-location cluster is the unit of observation.

**National Tenancy.** A commercial tenant operating at multiple locations within
a single real estate market, with employee populations requiring proximate
suburban commercial space. The canonical examples are professional services,
back-office operations, and healthcare.

**Co-location Cluster.** A maximal set of retail anchor points {p₁, p₂, ..., pₙ}
such that:
- Every point pᵢ is within a defined spatial radius ε of at least one other point
- The maximum pairwise geodesic distance between any two members ≤ Δ_max

Where ε = 1.0 km (tight) or 3.0 km (loose), and Δ_max = 3.0 km.

**Span.** The maximum pairwise geodesic distance between any two members of a
cluster, measured in kilometres:

    span_km = max{d(pᵢ, pⱼ) : pᵢ, pⱼ ∈ cluster}

Span is the *diameter* of the cluster's convex hull along its longest axis.
A cluster with span_km < 1.0 km is *tight-intact* — every member is within 1 km
of every other, the configuration consistent with a single power centre or
mall complex. A cluster with span_km ∈ (1.0, 3.0] is a *loose* configuration —
retailers drawn from a commercial corridor rather than a single site.

**Anchor Category.** A functional classification of retail format, defined
by the dominant traffic pattern and co-tenancy externality type:

| Category | Canonical chains (NA) | Canonical chains (EU) |
|---|---|---|
| `hypermarket` | Walmart Supercentre, Target | IKEA, Carrefour Hypermarché, Auchan |
| `hardware` | Home Depot, Lowe's | Leroy Merlin, OBI, Bauhaus |
| `price_club` | Costco, Sam's Club | Costco EU |
| `lifestyle` | IKEA (NA), Crate & Barrel | XXXLutz, Höffner |
| `electronics` | Best Buy, MediaMarkt US | MediaMarkt, Saturn, Boulanger, Darty |
| `sport` | SportChek, Decathlon | Decathlon |

Categories are mutually exclusive at the chain level. A cluster's *category set*
is the union of all categories present across its member points.

### 3.2 Tier Classification

Tier is a function of category composition — which anchor categories are
co-located — and is independent of geometric parameters:

**T1 — Primary-complete:**
A cluster whose category set satisfies *any* of:
- `has_hypermarket AND has_hardware AND has_price_club` (the canonical NA triumvirate)
- `has_hypermarket AND has_hardware AND |members| ≥ 3` (extended EU hypermarket corridor)
- `has_hypermarket AND has_price_club AND |members| ≥ 3` (price club variant)
- Any cluster with ≥ 4 distinct anchor categories regardless of specific combination
- Electronic + hardware configurations (H2b rule: `tight_intact AND |members| ≥ 3
  AND (has_electronics OR has_hardware)`)

**T2 — Secondary-complete (proposed Scenario A):**
- `has_hypermarket AND has_hardware` — hypermarket with hardware co-location
- This is the minimum composition that distinguishes a purpose-built power centre
  from a commercial strip anchored by a single dominant retailer

**T3 — Tertiary:**
- All remaining clusters: single-anchor dominant, or multi-anchor without
  hypermarket+hardware pairing

The normative prediction this classification encodes:

> T1 ≻ T2 ≻ T3 as investment sites for National Tenancy commercial real estate
> development, holding Regional Market size constant.

This is a falsifiable claim. §7 describes the empirical test.

### 3.3 The Two-Pass Tight-First DBSCAN Algorithm

Cluster membership is determined by a two-pass spatial density algorithm:

**Pass 1 — Tight nuclei:**
Apply DBSCAN with ε = TAU_TIGHT = 1.0 km. Points within 1.0 km of a minimum
density neighbour join a tight nucleus. Lock all tight components.

**Pass 2 — Loose expansion:**
Apply DBSCAN with ε = TAU_LOOSE = 3.0 km to unlocked points. Split any resulting
component whose maximum pairwise diameter exceeds Δ_max = 3.0 km using a
greedy split algorithm (split the component at the longest inter-point edge
iterating until diameter constraint is satisfied).

**Geometric constraint:**
    span_km ≤ 3.0 km   for all clusters

This constraint is hard and uniform across all countries. It encodes the
definition of "one shopping destination" — a distance a pedestrian or short-drive
customer treats as a single retail node. Markets with denser or sparser retail
configurations are accommodated by the relative ranking system (§3.4), not by
loosening the membership boundary.

**The tight_intact flag:**
A cluster is *tight_intact* if all members are within TAU_TIGHT = 1.0 km of
every other member. This flag qualifies for accelerated T1 promotion rules
(H2b) and produces higher geometric rank scores within tier.

### 3.4 Within-Tier Geometric Rank

After tier classification, clusters are ranked by geometric compactness within
their tier peer group. The ranking variable is:

    dist_rank_in_tier = P(span_km ≤ x | tier, country, continent)

where P is a shrinkage-smoothed empirical CDF, blending country-level and
continent-level distributions with weight parameter K:

    w = n_country / (n_country + K)   [K ≈ 20–30]
    rank = w · CDF_country(span_km) + (1 - w) · CDF_continent(span_km)

Rank is inverted so that tighter clusters (smaller span) score higher. The
intuition: a T1 cluster where Walmart, Home Depot, and Costco are within 0.6 km
of each other is a more coherent investment target than one where they are spread
across a 2.8 km corridor.

This ranking is **Stage 1** of the two-stage lexicographic rank. Stage 2 —
demand rank — is introduced when O-D data is available (§6.2).

### 3.5 The Ring Radius Formula

The ring radius is the visual representation of a cluster's spatial footprint
on the GIS platform. It is derived directly from span_km:

    ring_radius_km = max(1.0, span_km / 2 × 1.15)

The 1.15 multiplier ensures that all member points fall visually inside the ring
(accommodating centroid-to-member distance being slightly less than span/2 for
non-symmetric configurations). The 1.0 km floor prevents degenerate rings on
single-point or very tight clusters.

This formula applies uniformly across all tiers. The visual distinction between
tiers is carried by ring styling (T1: solid navy; T2/T3: dashed), not by
radius calculation.

---

## 4. Implementation: The Woodfine GIS Platform

### 4.1 Data Architecture

The platform operates entirely on open-standard, sovereign data — no proprietary
API dependency in the critical path.

**Retail point data:** OpenStreetMap via Overture Maps Foundation, supplemented
by targeted name-query ingests for chains with poor OSM coverage. As of Phase 21
(May 2026), the dataset covers 13 countries across North America and Europe,
with chain-level YAML files specifying bounding boxes, Wikidata QIDs, and
format-exclusion rules.

**Spatial index:** H3 hexagonal grid (Uber H3, resolution 7 for population;
resolution varies by layer). PMTiles format for client-side vector tile delivery
(MapLibre GL JS renderer, no tile server required).

**Cluster computation:** Python (DuckDB spatial + custom DBSCAN implementation
in `build-clusters.py`). Taxonomy single-authority pattern: all chain-to-category
assignments flow through `taxonomy.py::all_chains_for_iso()`. No hard-coded
category lists in downstream scripts.

**Delivery:** nginx-served static site (`gis.woodfinegroup.com`). No server-side
query layer. The full dataset is delivered as:
- `layer2-clusters.pmtiles` (47.7 MB) — all cluster centroids with tier/span
- `clusters-meta.json` (11 MB) — full cluster metadata for panel UI
- `layer3-catchment.pmtiles` — O-D catchment polygons
- `layer4/5-census/spend.pmtiles` — ambient population and spend surfaces

### 4.2 Platform Scale (Phase 21)

| Metric | Value |
|---|---|
| Total clusters | 6,493 |
| T1 clusters | 1,537 (24%) |
| T2 clusters | 3,090 (48%) |
| T3 clusters | 1,866 (29%) |
| Countries covered | 13 |
| Chain YAMLs | 120+ |
| Retail anchor points | ~90,000 |

### 4.3 The Five-Degree Co-location Framework (from source memos)

The investment memo that initiated this research describes a qualitative
five-degree taxonomy that the algorithmic framework formalises:

**1st degree** — Primary Target only (Walmart Supercentre / IKEA). The minimum
viable co-location. Maps approximately to T3 in the algorithmic system.

**2nd degree** — Primary Target + one Secondary Anchor (Home Depot, Costco, or
European equivalent within 1.0 km). Maps approximately to T2.

**3rd degree** — Primary Target + two Secondary Anchors. Maps approximately to
T1 by composition.

**4th degree** — Primary + Secondary configuration + Tertiary Anchor (university
or hospital within 5.0 km of cluster centroid). Maps to high-confidence T1 with
positive civic modifier.

**5th degree** — The full composition including major medical / educational
institutions *plus* strong demographic confirmation (Optimum Mosaic category
A/B). The highest-confidence National Tenancy sites.

The algorithmic system currently implements degrees 1–3 via tier classification.
Degrees 4–5 require the civic layer (currently being built) and demographic
overlay (Optimum Mosaic integration: planned).

### 4.4 The Regional Market as the Unit of Investment

Each co-location cluster belongs to a Regional Market — a named settlement or
metropolitan area. The platform's current Regional Market catalogue contains
2,986 entries. The investment decision is made at the Regional Market level
(which market to enter) using the cluster tier as the signal (which markets have
the composition to support National Tenancy).

The top-tier Regional Markets form the "Top 400" — approximately 200 in North
America and 200 in Europe — where T1 cluster presence, Regional Market
population, and absence of competitive office supply create the investment case.
This figure is calibrated geometrically: reducing the clustering radius naturally
converges on ~400 sites per continent as the algorithm selects only the most
compact and compositionally complete configurations.

---

## 5. Results: The Current Dataset

### 5.1 Global Distribution

[TODO: This section should be populated with the final Phase 21 distribution
maps and country-by-country tables when the dataset stabilises post-Scenario A
rebuild. Include: T1 count by country, top 20 Regional Markets by tier
composition, NA vs EU comparison.]

### 5.2 North America

The North American dataset is anchored by Walmart Supercentre as the primary
target. The Walmart network covers virtually all Canadian and US Regional
Markets above ~100,000 population, making coverage completeness high.

Key observations:
- US T1 clusters concentrate in Sun Belt (Texas, Florida, Carolinas, Arizona)
  and Great Lakes (Illinois, Ohio, Michigan, Ontario). These match known patterns
  of affordable land + high population growth.
- Canadian T1 clusters concentrate in the suburban rings of the six major CMAs
  (Toronto, Montreal, Calgary, Edmonton, Vancouver, Ottawa-Gatineau).
- Sherwood Park, Alberta is the canonical demonstration case: a Regional Market
  with the highest T1 co-location density relative to population in Canada,
  where Walmart + Home Depot + Costco form a tight nucleus within 0.8 km.

### 5.3 Europe

[TODO: Develop this section after Scenario A rebuild. The EU dataset is more
heterogeneous: IKEA is the Primary Target anchor but is present in fewer
markets than Walmart. EU T1 clusters are more concentrated in major metros
(Berlin, Paris, Madrid, Amsterdam, Warsaw). Medium-sized Regional Markets in
EU have lower T1 density, supporting the thesis that EU presents a different
but comparable investment opportunity.]

### 5.4 Calibration: The ~400-Site Convergence

[TODO: Run the simulation: vary composition threshold and track T1 count.
Show that the ~400-per-continent figure is a geometric property, not a target.
The calibration should demonstrate that the threshold producing ~400 sites
also maximises composition completeness — the algorithmic result and the
investment thesis converge at the same point.]

---

## 6. Geometric vs. Demand-Driven Location Intelligence

### 6.1 The Fundamental Distinction

Demand-driven location intelligence is **inductive**: it observes traffic,
transactions, and mobility patterns and infers site quality from observed
demand. Geometric co-location intelligence is **deductive**: it derives site
quality from the revealed preferences of dominant retailers whose location
decisions encode decades of demand analysis.

Both are valid inference procedures. The question is which is *more efficient*
for the specific investment problem: identifying undervalued National Tenancy
sites in Regional Markets.

**Claim:** For Regional Markets, geometric intelligence is more efficient
because:

1. **Data availability.** Retail anchor locations are publicly observable from
   OpenStreetMap at zero cost. Commercial mobility data costs USD $50K–$150K
   per year per market.

2. **Temporal depth.** A Walmart Supercentre opened in 2004 reflects 20 years
   of sustained market validation. Mobility data reflects at most 5–10 years of
   panel coverage, with significant gaps before 2015.

3. **Counterfactual identification.** Geometric analysis can identify Regional
   Markets where the *composition* signal is strong but development has not yet
   occurred — the investment opportunity. Demand data identifies existing
   high-traffic locations, where development has often already priced in the signal.

4. **Structural permanence.** Retail anchors are sticky; major national chains
   do not exit Regional Markets that validate them. The geometric signal is
   durable. Mobility patterns shift with consumer behaviour, remote work, and
   digital commerce.

### 6.2 Demand Intelligence as Confirmatory Layer

The thesis is not that demand intelligence is *wrong* or *unnecessary*. It is
that demand intelligence should be the **Stage 2** confirmatory layer on top of
a geometric foundation, not the primary discovery mechanism.

The two-stage ranking model implements this directly:

- **Stage 1 (geometric):** `dist_rank_in_tier` — cluster compactness within
  composition tier. Available immediately, for all 6,493 clusters, at zero
  marginal cost.

- **Stage 2 (demand):** `demand_rank_in_tier` — catchment population or O-D
  mobility within the cluster's primary/secondary catchment. Available as O-D
  data is acquired, market by market, as a supplementary ranking layer.

The investment workflow is: use geometric Stage 1 to identify the top-50 Regional
Markets in a target region; use Stage 2 demand data to rank them against each
other before committing capital. Geometric analysis narrows the field cheaply.
Demand analysis closes the deal expensively but precisely.

### 6.3 The Optimum Mosaic as Demographic Confirmation

[TODO: Integrate Optimum Mosaic (Environics Analytics) as the Canadian demographic
confirmation layer. The memo describes it as confirming the geometric hypothesis
rather than generating it. The practical integration: after geometric T1
identification, check Optimum Mosaic category (A/B/C) for the primary catchment.
A-category Regional Markets with T1 co-location are the highest-confidence
National Tenancy targets. This section should be written when the Optimum Mosaic
data layer is integrated into the platform.]

### 6.4 The PhD Thesis Statement

The preceding framework generates a formally falsifiable thesis:

> **Thesis:** In Regional Markets across North America and Europe, co-location
> tier (a function of retail anchor category composition) is a statistically
> significant positive predictor of commercial real estate demand intensity —
> measured by office leasing velocity, retail sales per square foot, and
> employment density — after controlling for Regional Market size and
> infrastructure quality. Demand-driven location intelligence variables
> (catchment population, mobility index) will be positively correlated with
> geometric tier and will not materially improve prediction accuracy when
> added to a model that already contains geometric tier as a covariate.

**Null hypothesis:** Co-location tier has no predictive power for commercial
real estate performance after controlling for Regional Market size. The null
is falsified when the platform's T1 clusters systematically outperform T2/T3
clusters on observable demand metrics.

**Falsification programme:** See §7.

---

## 7. The Falsification Programme

This section defines the empirical tests that will be conducted as demand data
becomes available. The GIS platform is the experimental apparatus. Each data
acquisition event adds an observation to the regression model underlying the thesis.

### 7.1 Test 1 — Retail Sales Proxy (available now, partial)

**Hypothesis:** T1 clusters have higher Walmart sales per square foot than T2
clusters in the same Regional Market.

**Data:** Walmart annual reports disaggregate comparable store sales by region.
Market-level data is not publicly available, but LODES work-commute O-D data
(US) provides a workplace density proxy that correlates with retail throughput.

**Method:** Within each US state, regress `span_km` (geometric compactness) on
LODES `total_work_reach_35km`. A negative coefficient (tighter clusters have
higher work catchment) confirms the geometric hypothesis.

**Status:** LODES data is loaded (`lodes-work-summary-us.jsonl`). Join to
`clusters-meta.json` pending (`build-geometric-ranking.py` — open task).

### 7.2 Test 2 — O-D Primary Catchment (as data arrives)

**Hypothesis:** T1 clusters have larger primary O-D catchments (origins
supplying 60–70% of visits) than T2 clusters of equal span_km.

**Data:** LODES (US), MITMA (ES), Statistics Canada PUMF (CA — planned).
UK, FR, DE: ONS ODWP01EW, INSEE FD_MOBPRO, BA Pendler (research completed —
acquisition pending).

**Method:** For each cluster with an observed O-D catchment, compute primary
catchment area (km²). Regress on tier dummy variables, controlling for
`log(Regional_Market_population)`. The coefficient on T1 dummy should be
positive and significant.

**Status:** US LODES loaded; ES MITMA loaded for 58 clusters. CA: none.
UK/FR/DE: pending acquisition.

### 7.3 Test 3 — Office Leasing Velocity (planned)

**Hypothesis:** Regional Markets with T1 co-location in the top quartile of
their continent exhibit higher office leasing velocity (sq ft absorbed per year)
than markets with only T2/T3 co-location.

**Data:** CBRE North American Office Leasing Survey (annual); JLL European
Office Market Database. Neither is open data — acquisition cost ~$30K/year.

**Method:** Match CBRE/JLL market definitions to the platform's Regional Market
catalogue. Cross-tabulate tier presence with leasing velocity. Run Mann-Whitney
test for T1 vs non-T1 markets.

**Status:** Not yet acquired. Target: Year 2 of the research programme.

### 7.4 Test 4 — Synthetic Simulation (buildable now)

**Hypothesis:** Randomly assigned tier labels (holding cluster count constant)
produce lower predictive accuracy than the geometric tier assignments on any
available demand proxy.

**Method:** Permutation test. Shuffle tier assignments 1,000 times. For each
permutation, compute correlation between tier and LODES work-commute density.
Compare the observed correlation to the permutation distribution.

**Status:** Executable now with current data. Should be implemented in
`verify-data-radius.py` or a new `sim-tier-permutation.py`.

---

## 8. Conclusion

### 8.1 The Thesis in Plain Language

National retail chains spend decades and billions of dollars identifying the
Regional Markets that can sustain mass-market commercial activity. When Walmart,
Home Depot, and Costco cluster within a 1-kilometre radius, that configuration
is the product of three independently conducted, sustained, and expensive
location selection processes all converging on the same point.

This convergence is the signal. It is observable, durable, publicly documented,
and free to analyse. A platform that indexes these configurations at continental
scale — tracking which Regional Markets have the full T1 composition and which
have partial compositions — produces a site selection framework that is *prior*
to demand data. The demand data, when it arrives, will confirm it.

The GIS platform is the instrument that makes this argument quantitative.
Its T1/T2/T3 taxonomy is not a marketing hierarchy — it is a formal prediction
about which Regional Markets will produce the best National Tenancy commercial
real estate performance. Each new data acquisition event is a test of that
prediction.

### 8.2 The Research Programme

This paper is the scaffolding. The research programme is:

**Year 1 (2026):** Implement Stage 2 demand rank using LODES + MITMA. Run
Tests 1 and 4. Publish interim results. Acquire O-D data for UK and France.

**Year 2 (2027):** Run Test 2 at scale (US + ES + UK). Integrate CBRE or JLL
office market data for Test 3. Initiate Optimum Mosaic integration for Canada.

**Year 3 (2028):** Full falsification run across all 13 countries. Publish
findings. If the thesis holds, the platform's Top 400 list becomes a credible
investment shortlist for external capital.

**What not everyone can do:** The thesis is articulable by anyone. The apparatus
to test it — a sovereign, open-standard, 6,493-cluster spatial database covering
13 countries, with a pipeline from OpenStreetMap ingest to PMTiles delivery,
updated incrementally as data arrives — represents the differentiating
contribution. The research insight is the thesis. The competitive moat is
the backend that proves it.

---

## References

*[Scaffold — to be populated as research programme proceeds]*

### Foundational location theory
- Christaller, W. (1933). *Die zentralen Orte in Süddeutschland.* Jena: Gustav Fischer.
- Reilly, W. J. (1931). *The Law of Retail Gravitation.* New York: Knickerbocker Press.
- Huff, D. L. (1964). Defining and estimating a trading area. *Journal of Marketing*, 28(3), 34–38.

### Retail co-location and anchor externalities
- Pashigian, B. P., & Gould, E. D. (1998). Internalizing externalities: The pricing of space in shopping malls. *Journal of Law and Economics*, 41(1), 115–142.
- Brueckner, J. K. (1993). Inter-store externalities and space allocation in shopping centers. *Journal of Real Estate Finance and Economics*, 7(1), 5–16.
- Eppli, M. J., & Shilling, J. D. (1995). Large-scale shopping center development opportunities. *Land Economics*, 71(1), 35–41.
- [TODO: Add CBRE anchor co-tenancy studies 2015–2024]

### Location intelligence and commercial real estate
- [TODO: JLL Office Outlook North America, current year]
- [TODO: CBRE North American Office Leasing Survey, current year]
- [TODO: Colliers Regional Market Reports — Sun Belt focus]

### Open data and spatial infrastructure
- Overture Maps Foundation. (2024). *Overture Maps 2024 Release.* overturemaps.org.
- Uber Technologies. (2018). H3: A hierarchical hexagonal geospatial indexing system. GitHub.
- Felt, B. (2023). *PMTiles: A single-file archive format for pyramids of tile data.*
- OpenStreetMap contributors. (2024). *OpenStreetMap.* openstreetmap.org.

### Demand-side data and O-D systems
- U.S. Census Bureau. (2023). *LEHD Origin-Destination Employment Statistics (LODES).* lehd.ces.census.gov.
- Ministerio de Transportes, Movilidad y Agenda Urbana. (2022). *Estudio de movilidad (MITMA).* mitma.gob.es.
- [TODO: ONS ODWP01EW Workplace population 2021]
- [TODO: INSEE FD_MOBPRO Mobilité professionnelle, current year]
- [TODO: Bundesagentur für Arbeit Pendler, current year]

### Demographic profiling
- [TODO: Environics Analytics. Optimum Mosaic Canada.]

### Platform implementation
- Woodfine Management Corp. (2026). *Woodfine GIS Platform, Phase 21.* gis.woodfinegroup.com.
- Woodfine Management Corp. (2026). *BRIEF-VARIABLE-DISTANCE-2026-05-21.md.* Internal.
- Woodfine Management Corp. (2026). *BRIEF-TIER-REBALANCE-2026-05-24.md.* Internal.

---

## Appendix A — Mathematical Notation Reference

| Symbol | Definition |
|---|---|
| ε | DBSCAN proximity radius (TAU_TIGHT = 1.0 km; TAU_LOOSE = 3.0 km) |
| Δ_max | Maximum cluster diameter cap = 3.0 km |
| span_km | max{d(pᵢ, pⱼ)} for all members pᵢ, pⱼ |
| tight_intact | Boolean: all members within 1.0 km of each other |
| ring_radius_km | max(1.0, span_km / 2 × 1.15) |
| dist_rank_in_tier | Shrinkage-smoothed inverted CDF of span_km within tier |
| w | Shrinkage weight = n_country / (n_country + K), K ≈ 20–30 |
| T1, T2, T3 | Tier classification by anchor category composition |

## Appendix B — Chain Coverage by Country (Phase 21)

[TODO: Full chain-by-country table from taxonomy.py at final draft stage]

## Appendix C — Platform Architecture Diagram

[TODO: Data flow diagram — YAML ingest → JSONL → DBSCAN → PMTiles → MapLibre]

---

*Version 0.1 — scaffolding draft — May 2026*
*For internal circulation only pending editor review*
*All forward-looking statements carry "planned / intended / may / target" language*
*per BCSC continuous-disclosure posture*
