---
schema: foundry-draft-v1
artifact_type: BRIEF
language_protocol: PROSE-RESEARCH
audience: editorial-and-technical
bcsc_class: no-disclosure-implication
version: "1.0"
title: "Regional Markets Intelligence System — Reference Brief"
date: 2026-05-30
---

# Regional Markets Intelligence System — Reference Brief

## Executive Summary

The Regional Markets Intelligence System is a continental-scale geographic
analysis framework that identifies sub-metropolitan retail markets defined by
the convergence of large-format retail anchors, civic infrastructure, and
demographic catchment. The current dataset spans 6,493 co-location clusters
across eighteen countries in North America and Europe, classified into three
compositional tiers (T1, T2, T3) and aggregated into 4,436 named Regional
Markets. The system combines retail point-of-interest records, population
rasters, modelled consumer spend, mobility-derived catchments, and four
climate-and-ecology data layers into a single ranked index. This document is
the canonical reference for downstream wiki articles, methodology guides, and
the Top-400 Regional Markets list surface.

---

## 1. Dataset Scope

The current build covers 6,493 co-location clusters across eighteen countries
on two continents.

| Region | Countries |
|---|---|
| North America | United States, Canada, Mexico |
| Europe — west and south | Spain, Italy, Greece, France, Germany, Portugal, Netherlands, Austria |
| Europe — Nordic | Sweden, Norway, Denmark, Finland, Iceland |
| Europe — central / east | Poland, United Kingdom |

Cluster counts by tier: **T1 = 1,746** (Regional anchors), **T2 = 2,726**
(District anchors), **T3 = 2,021** (Local anchors). The build pipeline draws
on four primary data sources.

**OpenStreetMap (ODbL licence).** Retail chain locations filtered by Wikidata
QID via the Overpass API. The current ingest covers more than sixty chains
spanning hypermarkets, hardware superstores, warehouse clubs, electronics
retailers, sporting-goods stores, and pharmacies.

**Overture Maps Foundation (CDLA Permissive 2.0).** Civic anchor locations
extracted from the Places dataset using the `taxonomy.primary` field, which
replaced the deprecated `categories.primary` field in the November 2025
release. Current coverage includes 27,833 medical and 28,846 higher-education
records across the eighteen countries.

**Kontur Population 2023 (CC BY 4.0).** A global H3 resolution-8 population
hex grid covering all eighteen countries; aggregated to H3 resolution-7
(≈1.22 km² per cell) for catchment calculations.

**WorldPop 100-metre raster (2026 release, CC BY 4.0).** Used in combination
with per-country spend multipliers from BLS (United States), Statistics
Canada, and Eurostat household budget surveys to model grocery, hardware, and
wholesale spend potential at the catchment level.

The clustering methodology is a two-pass DBSCAN: a first pass identifies
hypermarket and full-anchor cores, a second pass adds peripheral hardware
and warehouse-club anchors within a span constraint. The full algorithm is
described in §2 below.

---

## 2. Co-location Tier System

Each cluster is assigned one of three tiers based on the composition of
retail anchors present within the cluster boundary.

| Tier | Label | Composition rule |
|---|---|---|
| **T1** | Regional | Hypermarket + hardware + warehouse-club (or full equivalent across three independent anchor categories) |
| **T2** | District | Hypermarket + hardware (two independent anchor categories) |
| **T3** | Local | Any single qualifying anchor category |

The tier rule is compositional rather than count-based. A site with four
co-located hypermarket banners and no hardware or warehouse-club anchor
remains T3, because the compositional signal that distinguishes regional
draw from local convenience is the presence of *independent* anchor
categories, not the count of stores within a single category.

**Geometric span ranking within tiers.** Within each tier, clusters are
ordered by `span_km` — the diameter of the smallest enclosing circle that
contains all member anchors. Compact clusters (`span_km` below 2.5) rank
ahead of dispersed clusters. The cap is applied to prevent the algorithm
from extending an arterial corridor into a single notional cluster. A
distance-rank value is published per cluster within its tier.

**Anchor categories.** Six anchor categories are recognised in the current
build: `hypermarket`, `hardware`, `warehouse_club`, `electronics`,
`sporting_goods`, and `pharmacy`. Hypermarket, hardware, and warehouse-club
are weighted as tier-determining; electronics, sporting-goods, and pharmacy
are recognised as supporting anchors and contribute to the descriptive
fields but do not alter tier classification.

---

## 3. Regional Markets

A Regional Market is a named settlement — city, municipality, county, or
equivalent administrative unit — that contains one or more co-location
clusters. The Regional Market is the editorial unit of the system: each
Regional Market corresponds to a single article surface, regardless of how
many clusters it contains.

**Total count: 4,436 Regional Markets.** Of these, **2,327 are in North
America** and **2,109 are in Europe**. The breakdown reflects both the
relative size of the retail-anchor population and the resolution of
administrative boundaries: European municipalities are smaller and more
numerous, so Regional Markets in Europe more often resolve to a single
city rather than a county-equivalent.

**Schema fields per Regional Market.**

| Field | Type | Notes |
|---|---|---|
| `rm_id` | string | Stable identifier, persistent across rebuilds |
| `market` | string | Display name; resolves from the administrative-boundary join |
| `iso` | string | ISO 3166-1 alpha-2 country code |
| `centroid` | `[lon, lat]` | Geographic centroid of constituent cluster centroids |
| `best_tier` | string | Highest tier present among constituent clusters (T1 > T2 > T3) |
| `cluster_count` | integer | Number of co-location clusters within the Regional Market |

Regional Markets serve as the spatial join key for downstream layers:
demographic catchment, modelled spend, mobility origin-destination summaries,
and the climate-and-ecology layers described in §6.

---

## 4. Top 400 Composite Ranking

The Top 400 Regional Markets list is a composite ranking that combines
tier composition, civic infrastructure, distance from primary metropolitan
nodes, and confidence in the underlying chain data. The list is produced
separately for North America and Europe, yielding two ranked surfaces of
400 markets each.

**Composite score formula.**

```
score = tier_score × civic_multiplier × metro_distance_multiplier × confidence_factor

where:
  tier_score = (T1 × 4) + (T2 × 2) + (T3 × 1)
  civic_multiplier = 1.5  if any medical or higher-education anchor present, else 1.0
  metro_distance_multiplier = clamp(dist_km / 50, 0.5, 2.0)
  confidence_factor = 1.0  for high-confidence chain coverage
                      0.7  for low-confidence chain coverage
```

**Rationale.** The composite score identifies markets that combine
supply-side anchor strength, civic infrastructure, and suburban character.
The tier weighting (4 / 2 / 1) reflects the compositional hierarchy: a
single T1 cluster contributes more than two T2 clusters because the
presence of three independent anchor categories is a stronger signal than
the presence of two. The civic multiplier rewards the presence of medical
or academic anchors that indicate a functioning sub-metropolitan service
centre rather than a pure retail strip. The metro-distance multiplier is
configured to reward markets that sit at functional distance from the
nearest primary metropolitan node — close enough to share regional supply
chains, far enough to serve a distinct catchment.

The clamp at 0.5 prevents the formula from penalising markets that sit
within the metropolitan urbanised area itself (Berlin, Houston) where the
co-location signal remains meaningful even in zero-distance terms. The
clamp at 2.0 prevents extreme distances from dominating the score.

The Top 400 list is the entry surface for editorial coverage: each
Regional Market in the list is a candidate for a dedicated wiki article.

---

## 5. Civic Infrastructure Layer

The civic infrastructure layer adds medical and higher-education anchor
presence to the cluster member data. The source is the Overture Maps
Foundation Places dataset, queried for the `healthcare` and
`higher_education` primary categories.

**Coverage.** 27,833 medical records and 28,846 higher-education records
across the eighteen countries. North American coverage is denser than
European coverage because the underlying Overture taxonomy reflects the
density of structured business directories in each region; an enhancement
to improve European civic coverage is in preparation.

**Encoding.** Civic presence is encoded as a binary flag per cluster: if
any cluster member is classified as medical or higher-education, the
cluster carries `civic = True`. The Regional Market inherits the civic
flag from any constituent cluster. The civic flag is the input to the
1.5× civic multiplier in the composite score described in §4.

The civic layer is conceptually distinct from the retail layer. A
hospital adjacent to a hypermarket-and-hardware cluster does not turn
T2 into T1 — the tier classification is anchor-composition only. The
civic flag operates orthogonally as a market-quality signal.

---

## 6. AEC Data Layers

The AEC (architecture, engineering, construction) data layers add
climate, regulatory, and ecological context to each Regional Market and
to the surrounding development envelope. Four layers are currently
delivered; two further layers are in preparation.

**Delivered layers.**

| Layer | Source | Coverage |
|---|---|---|
| ASHRAE 169-2013 climate zones | ASHRAE standard, US extent | 94.4% of US Regional Markets |
| EU regulatory energy climate zones | Per-country building-energy regulations | Variable — Germany and France near 100%, Spain and United Kingdom partial |
| Köppen-Geiger climate class | Beck et al. 2018 global raster (CC BY 4.0) | 100% of all Regional Markets |
| WWF Ecoregions 2017 | World Wildlife Fund global vector (CC BY 4.0) | 99.5% of all Regional Markets |

The ASHRAE 169-2013 layer informs envelope-design assumptions for
United States projects. The EU regulatory layer captures member-state
specific climate zone definitions used in national building-energy
codes; the differences between Germany's `Klimazone` system and France's
`zone climatique` are preserved rather than normalised. The Köppen-Geiger
layer is the universal cross-country reference for climate classification.
The WWF Ecoregions layer adds biome and ecoregion context that is
relevant to landscape architecture, native-planting specifications, and
ecological-impact assessments.

**Layers in preparation.**

| Layer | Source | Status |
|---|---|---|
| Seismic peak ground acceleration | USGS (United States) and EFEHR (Europe) | Re-run scheduled for 1 June 2026 |
| Flood hazard | FEMA (United States) and EU JRC | Build scheduled for 31 May 2026 |

The seismic and flood layers are intended to complete the AEC envelope
for each Regional Market: climate-driven envelope loads, seismic design
category, and flood-zone designation in one combined record.

---

## 7. POI Data Schema

The platform operates two record classes within its location data layer:
service-business records (retail chains) and service-places records
(civic anchors).

**Service-business records.** Each record represents a single retail
chain location and is identified by a `chain_id` linking to a chain
configuration file and by a `brand_wikidata` field holding the Wikidata
QID for the brand. The Wikidata QID is the canonical cross-source chain
identifier because it is brand-level rather than name-level; two
storefronts with different local-language spellings but the same QID
belong to the same chain.

**Service-places records.** Civic anchors — hospitals, universities,
airports — ingested from Overture Maps using `taxonomy.primary` as the
category filter. Service-places records use a `category_id` key
(`hospital`, `university`, `airport`) in place of `chain_id`.

**Shared core fields.**

| Field | Type | Notes |
|---|---|---|
| `location_name` | string | Display name with category fallback |
| `brand_wikidata` | string or null | Wikidata QID; null for civic places with no brand identity |
| `street_address` | string or null | Freeform address from OSM `addr:*` or Overture addresses |
| `city` | string or null | Locality |
| `region` | string or null | Province, state, or NUTS-3 region |
| `iso_country_code` | string | ISO 3166-1 alpha-2 country code |
| `latitude`, `longitude` | float | WGS 84, 7 decimal places |
| `naics_code` | string | NAICS classification |
| `source` | string | `osm` or `overture` |
| `confidence` | float | OSM fixed 0.85; Overture from dataset |

**Spatial deduplication.** Records within a 100-metre radius per chain
are deduplicated, retaining the record with the most complete address
fields. A second pass at 25 metres across different `chain_id` values
sharing the same `brand_wikidata` QID identifies sub-format or
co-branded stores; these are candidates for the parent-child
sub-location model in which a primary store collapses ancillary
services (pharmacy, fuel, optical) into a `sub_entities` list.

The parent-child model follows the SafeGraph `parent_placekey` pattern
and the Placekey `What@Where` standard, where co-located sub-businesses
share the `Where` suffix while differing on the `What` prefix. Placekey
integration is planned for a later iteration of the ingest pipeline.

---

## 8. Catchment Model

The catchment model assigns each cluster a primary and secondary trade
area defined by crow-flies radius from the cluster centroid.

| Ring | Radius | Role |
|---|---|---|
| Primary | 35 km | Local-residence trade area |
| Secondary | 150 km | Regional draw |

Catchment population and spend are calculated by intersecting these
rings with H3 resolution-7 hexagons (≈1.22 km² per cell) populated from
Kontur Population 2023 and modelled spend from WorldPop combined with
per-country household-budget multipliers (BLS for the United States,
Statistics Canada, Eurostat for European countries).

**Ranking dimensions.** Each cluster receives four independent ranks
across the dataset:

| Rank field | Basis |
|---|---|
| Population rank | Combined primary + secondary catchment population |
| Grocery rank | Combined primary + secondary grocery spend potential |
| Hardware rank | Combined primary + secondary hardware spend potential |
| Wholesale rank | Combined primary + secondary wholesale spend potential |

Combined primary + secondary totals are used rather than either ring
alone, reflecting the commercial reality that a trade area encompasses
both proximate and regional shoppers. The four ranks are independent
and unweighted in the current version; a composite catchment rank with
chain-type weighting is intended for a later iteration.

**Cross-currency comparisons.** Spend values are denominated in local
currency (USD, CAD, MXN, GBP, EUR, PLN, SEK, NOK, DKK). Within the
eurozone and within individual countries, spend rankings are directly
comparable. Cross-currency comparisons require FX normalisation, which
is planned but not applied in the current version. Population rankings
are currency-neutral and fully comparable across all eighteen countries.

**Mobility-derived catchments.** A mobility-defined catchment layer
sits alongside the radius-based model. For United States clusters, the
US LODES origin-destination employment dataset provides a
worker-commute catchment per H3 cell. For Spain, the Ministerio de
Transportes MITMA mobility dataset provides a parallel surface. The
mobility surfaces are intended to replace administrative catchment
boundaries with empirically observed sub-metropolitan markets where
data permits. Commercial parking-lot geo-fenced mobility data is in
acquisition planning for a follow-on phase.

---

## 9. Key Findings

The Top-15 Regional Markets in each continental list illustrate the
shape of the composite score distribution.

**North America — Top 15 by composite score.**

| Rank | Market | ISO | T1 | T2 | T3 | Civic | Score | Nearest metro / dist |
|---|---|---|---|---|---|---|---|---|
| 1 | Ontario | US | 10 | 19 | 22 | Yes | 300.0 | Toronto 187 km |
| 2 | Texas | MX | 3 | 6 | 47 | Yes | 143.4 | Austin 96 km |
| 3 | Québec | US | 13 | 3 | 6 | Yes | 110.3 | Montréal 82 km |
| 4 | Jacksonville, FL | US | 2 | 9 | 5 | Yes | 93.0 | Jacksonville 166 km |
| 5 | El Paso, TX | US | 5 | 4 | 3 | Yes | 93.0 | Albuquerque 368 km |
| 6 | Columbus, OH | US | 8 | 5 | 0 | Yes | 68.6 | Cincinnati 55 km |
| 7 | Springfield, IL | US | 1 | 7 | 4 | Yes | 66.0 | St. Louis 170 km |
| 8 | Franklin, NC | US | 3 | 4 | 2 | Yes | 66.0 | Cincinnati 153 km |
| 9 | Columbia, MO | US | 4 | 2 | 1 | Yes | 63.0 | Charlotte 191 km |
| 10 | Aurora, CO | US | 4 | 2 | 0 | Yes | 60.0 | Denver 252 km |
| 11 | Manchester, CT | US | 3 | 3 | 0 | Yes | 54.0 | Baltimore 176 km |
| 12 | Houston, TX | US | 10 | 11 | 7 | Yes | 51.8 | Houston 14 km |
| 13 | Fayetteville, NC | US | 1 | 6 | 1 | Yes | 51.0 | Atlanta 204 km |
| 14 | Colorado Springs, CO | US | 2 | 4 | 1 | Yes | 48.8 | Denver 96 km |
| 15 | Madison, MS | US | 2 | 3 | 2 | Yes | 48.0 | Nashville 126 km |

The North American list is dominated by markets that combine multiple
T1 and T2 clusters with civic infrastructure and a substantial buffer
from the nearest primary metro. Houston ranks twelfth despite carrying
the highest absolute T1 count in the list because the small
metro-distance multiplier (Houston centroid distance 14 km) compresses
its score. Several entries marked as state names (Ontario, Québec,
Texas) reflect resolution at the state or province level in the
underlying administrative-boundary join; these are flagged for
sub-resolution in a later refinement pass.

**Europe — Top 15 by composite score.**

| Rank | Market | ISO | T1 | T2 | T3 | Civic | Score | Nearest metro / dist |
|---|---|---|---|---|---|---|---|---|
| 1 | Nürnberg | DE | 4 | 0 | 1 | Yes | 51.0 | Munich 150 km |
| 2 | Berlin, Stadt | DE | 12 | 4 | 5 | Yes | 45.8 | Berlin 2 km |
| 3 | Dresden, Stadt | DE | 2 | 2 | 2 | Yes | 42.0 | Leipzig 103 km |
| 4 | Κοινότητα Πυλαίας | GR | 5 | 4 | 13 | Yes | 39.7 | Athens 32 km |
| 5 | Łódź | PL | 2 | 1 | 3 | Yes | 39.0 | Warsaw 119 km |
| 6 | Gdańsk | PL | 2 | 2 | 1 | Yes | 39.0 | Warsaw 290 km |
| 7 | Magdeburg | DE | 2 | 2 | 0 | Yes | 36.0 | Leipzig 103 km |
| 8 | Bydgoszcz | PL | 2 | 2 | 0 | Yes | 36.0 | Warsaw 223 km |
| 9 | Poznań | PL | 2 | 2 | 0 | Yes | 36.0 | Wrocław 143 km |
| 10 | Nice | FR | 3 | 0 | 0 | Yes | 36.0 | Toulon 123 km |
| 11 | Lublin | PL | 3 | 0 | 0 | Yes | 36.0 | Warsaw 153 km |
| 12 | Tampere | FI | 1 | 3 | 0 | Yes | 30.0 | Helsinki 160 km |
| 13 | Palma | ES | 2 | 0 | 2 | Yes | 30.0 | Barcelona 206 km |
| 14 | Mannheim | DE | 3 | 0 | 2 | Yes | 29.5 | Frankfurt 70 km |
| 15 | Würzburg | DE | 2 | 0 | 2 | Yes | 29.1 | Frankfurt 97 km |

The European list shows a different shape. German cities dominate the
top of the list (Nürnberg, Berlin, Dresden, Magdeburg, Mannheim,
Würzburg) reflecting the depth of full-anchor hypermarket-and-hardware
combinations in the German retail landscape. Polish cities (Łódź,
Gdańsk, Bydgoszcz, Poznań, Lublin) form a second cluster, supported by
the recent ingest of Kaufland and Leclerc in Poland. The single Greek
entry (Κοινότητα Πυλαίας in metropolitan Thessaloniki) carries an
exceptionally large T3 component (13 local anchors) within a compact
geographic envelope.

**Pattern observation.** Across both continents, every entry in the
Top 15 carries the civic flag. The civic multiplier is therefore
operating as a near-binary qualifier at the top of the distribution —
markets without medical or academic anchors are functionally excluded
from the top tier of the composite score.

---

## 10. Forward-Looking Work

Work planned or intended for the next iterations of the system.

**Climate and hazard layer completion.** The seismic peak-ground-
acceleration layer and the flood-hazard layer are scheduled for build
on 31 May and 1 June 2026 respectively. Once delivered, every Regional
Market will carry a complete envelope record covering climate zone,
ecoregion, seismic design category, and flood-zone designation.

**OLS regression on span_km.** A cluster-level ordinary-least-squares
regression of `span_km` against catchment population density, modelled
spend, and mobility-derived activity is in preparation. Country fixed
effects and an urban-core versus peri-urban interaction term are
intended. The output will be the empirical falsification surface for
the compositional hypothesis at the heart of the framework.

**Per-country article surfaces.** Dedicated wiki articles for each of
the 400 Regional Markets in the Top-400 list (200 per continent) are
planned. The articles are intended to combine the data fields described
in this brief with locally-resolved narrative drawn from public
sources.

**Wikipedia API integration.** Live retrieval of Wikipedia summaries
for the named settlement underlying each Regional Market is intended,
to provide a single-pass context layer at article render time.

**FX normalisation for cross-country spend.** A foreign-exchange
normalisation pass on the modelled spend layer is planned, enabling
cross-currency catchment ranking and direct comparison of grocery,
hardware, and wholesale spend between countries.

**Commercial mobility panel acquisition.** Acquisition of a commercial
mobility panel (device-derived parking-lot visit records) is in
planning. This data, once acquired, will allow the system to replace
crow-flies catchment radii with empirically observed mobility
catchments at the cluster level.

---

## 11. Media-Knowledge-Project Notes

This brief is the canonical reference document for the
media-knowledge-documentation wiki. Downstream artefacts should treat
this document as the single source of truth for tier definitions,
Regional Market counts, composite-score formula, and AEC layer status.

**Document family.**

- **This brief** provides the system reference: schema, methodology,
  current dataset state, and forward-looking work.
- **The Top-400 Regional Markets articles** (one for North America,
  one for Europe) provide the list surface. Each list article enumerates
  the 400 ranked Regional Markets with score, tier composition, civic
  flag, and nearest-metro distance.
- **Per-Regional-Market articles** provide the detail surface. Each
  article describes a single Regional Market, drawing on the cluster
  member data, catchment population and spend, civic anchors, and AEC
  envelope.
- **The Regional Market Topic production guide** describes the
  workflow by which per-market articles are generated, reviewed, and
  published.

**Numerical fidelity.** All counts and percentages in this brief
reflect the current build state as of 30 May 2026. Subsequent rebuilds
will update the underlying figures; this brief is the canonical
snapshot against which downstream articles can be reconciled.

**Disclosure posture.** Forward-looking statements in this brief carry
"planned", "intended", or "scheduled" language. Delivered work is
stated as fact. The brief is suitable for editorial use without
further qualification of either category.

---

*Reference data current as of 30 May 2026. Sources: OpenStreetMap
contributors (ODbL); Overture Maps Foundation (CDLA Permissive 2.0);
Kontur Population 2023 (CC BY 4.0); WorldPop 2026 (CC BY 4.0); Beck et
al. 2018 Köppen-Geiger raster (CC BY 4.0); WWF Ecoregions 2017
(CC BY 4.0); US LODES (public domain); Spain MITMA mobility (open data).*
