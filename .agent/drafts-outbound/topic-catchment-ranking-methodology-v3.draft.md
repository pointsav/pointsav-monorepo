---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-catchment-ranking-methodology-v3.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-16
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 17 Phase 2 implementation. Pure-predicate tier engine
  (build-geometric-ranking.py, commit b7a7cdf1). Percentile thresholds from G6
  operator decision (2026-05-16). Civic classification from ingest-osm-civic.py
  and SCORING-METHODOLOGY.md V3. IoU formula derived from standard lens-area
  closed-form. All threshold values are current-fact as of Sprint 17.
research_inline: false
notes_for_editor: |
  Bloomberg-register polish required. Percentile-relative language throughout
  (G10: self-bounded; no forward-looking superlatives). Pair with the earlier
  topic-catchment-ranking-methodology.md which covers the V2 score-based system.
  This document describes V3 (predicate gates). Both should remain on the wiki;
  the V2 document needs a "superseded by V3" note added.
---

# Pure-Predicate Catchment Ranking Methodology

The co-location tier system assigns each cluster to one of four tiers — Regional, District, Local, or Fringe — using binary predicate gates rather than a composite score. A cluster must pass every gate in a tier's gate set to qualify for that tier; partial scores do not accumulate. This methodology is described here as implemented in Sprint 17 (May 2026).

## Why Predicate Gates Replace Composite Scores

The prior system (V2, April–May 2026) assigned tiers by summing a base score, count bonus, diversity bonus, civic depth term, and overlap penalty. The resulting composite score was interpretable in isolation but difficult to explain to a non-technical audience: a cluster could reach Tier 2 via a high diversity bonus even if it lacked the population catchment and civic infrastructure that the tier was intended to signal.

Binary gates make the qualification criteria explicit and verifiable. A Regional cluster must have national-scale population reach, a specific anchor composition, regional hospital access, and spatial independence from stronger clusters. None of these requirements are satisfied by proxy.

## Population Catchment Ranks

Catchment population is computed by `synthesize-od-study.py` using a crow-flies H3 grid at resolution 7 (cell width approximately 2.1 km). Two zones are defined for each cluster:

- **Primary zone**: all H3 cells within 35 km of the cluster anchor
- **Secondary zone**: all H3 cells between 35 km and 150 km of the cluster anchor

Population totals for each zone are drawn from WorldPop 2026 100 m rasters aggregated to H3 resolution 7.

Clusters are then ranked within their ISO country on each of eight axes: primary population, secondary population, primary grocery spend, secondary grocery spend, primary hardware spend, secondary hardware spend, primary wholesale spend, and secondary wholesale spend. The rank is expressed as a fraction: rank 1 in a country of 500 clusters yields a value of 0.002; rank 50 yields 0.100. Lower values indicate higher relative reach within the country. A cluster with a primary-population rank of 0.10 is in the top 10% of its country by primary trade-area population.

Spend estimates are derived from per-capita household spending surveys (BLS for the United States, StatCan for Canada, Eurostat HBS for EU member states, INEGI for Mexico) applied to WorldPop grid cells, stratified by grocery, hardware, and wholesale category shares.

## Tier Gate Definitions

### Tier 1 — Regional

A cluster qualifies as Regional if all five of the following conditions are true:

1. **Composition**: The cluster contains a Warehouse anchor (Costco, Sam's Club, Makro, or equivalent) and a Hypermarket anchor (Walmart, Target, Mercadona, Tesco, Sainsbury's, or equivalent); or it contains a Lifestyle anchor (IKEA) and a Hypermarket anchor.
2. **Primary catchment**: The cluster's primary-population rank within its country is in the top 10% (rank value ≤ 0.10).
3. **Secondary catchment**: The cluster's secondary-population rank within its country is in the top 20% (rank value ≤ 0.20).
4. **Civic — regional hospital**: At least one hospital classified as "regional" by the OSM-derived civic classification is present within the 5 km civic ring around the cluster anchor.
5. **Spatial independence**: The Intersection over Union (IoU) between this cluster's 3 km disk and the 3 km disk of any cluster in the same country with a higher primary-population rank does not exceed 0.10.

### Tier 2 — District

A cluster qualifies as District if all five of the following conditions are true:

1. **Composition**: The cluster contains a Hypermarket anchor and a Hardware anchor (Home Depot, Lowe's, Leroy Merlin, or equivalent) or a Warehouse anchor.
2. **Primary catchment**: The cluster's primary-population rank within its country is in the top 25% (rank value ≤ 0.25).
3. **Spend reach**: The cluster's rank within its country on at least one of grocery spend, hardware spend, or wholesale spend is in the top 25% (rank value ≤ 0.25).
4. **Civic — hospital present**: At least one hospital classified as "regional" or "district" is present within the 5 km civic ring.
5. **Spatial independence**: The IoU between this cluster's 3 km disk and the 3 km disk of any Regional cluster in the same country does not exceed 0.25.

### Tier 3 — Local

A cluster qualifies as Local if all three of the following conditions are true:

1. **Composition**: The cluster contains a Hardware or Warehouse anchor.
2. **Primary catchment**: The cluster's primary-population rank within its country is in the top 50% (rank value ≤ 0.50).
3. **Civic — any hospital**: At least one hospital of any classification is present within the 5 km civic ring.

### Tier 4 — Fringe

A cluster that does not qualify for Regional, District, or Local is classified as Fringe. A Fringe cluster may still contain significant co-tenancy; the Fringe classification indicates that one or more required conditions for Local or above were not met.

## Overlap Measurement

The spatial independence gate uses the closed-form intersection-over-union formula for two equal-radius circles:

```
lens_area = 2r² · arccos(d/2r) − (d/2) · √(4r² − d²)
IoU = lens_area / (2·π·r² − lens_area)
```

where d is the haversine distance between cluster centroids and r = 3.0 km (the secondary co-location radius). Two clusters whose centroids are further than 6 km apart have IoU = 0 by definition.

## Civic Classification

Hospital and university tier assignments are produced by `ingest-osm-civic.py` from OpenStreetMap data. Hospitals are classified as `regional` (major general hospitals with emergency departments), `district` (secondary hospitals and specialist centres), or `clinic` (general practice and walk-in clinics). Universities are classified as `regional` (large research universities), `small` (community colleges and specialist schools), or `excluded` (non-degree institutions). Clinics and excluded institutions do not contribute to Regional or District civic gates.

## Threshold Summary

| Threshold | Symbol | Value |
|---|---|---|
| T1 primary catchment | P10 | top 10% within country |
| T1 secondary catchment | P20 | top 20% within country |
| T2 primary catchment / spend | P25 | top 25% within country |
| T3 primary catchment | P50 | top 50% within country |
| T1 IoU limit | — | ≤ 0.10 |
| T2 IoU limit | — | ≤ 0.25 |
| Civic ring radius | — | 5 km (tertiary ring) |
| IoU disk radius | — | 3 km (secondary co-location radius) |

Thresholds are intentionally coarse. The P10/P25/P50 values were chosen to distinguish nationally significant clusters from local nodes, not to produce a precise rank. Refinement is planned for a future sprint as additional catchment data becomes available.

## See Also

- [O-D Catchment Methodology](topic-od-catchment-methodology.md)
- [Trade Area Data Sources](topic-trade-area-data-sources.md)
- [Co-Location Tier Nomenclature](topic-co-location-tier-nomenclature.md)
- [Retail Co-location Methodology](topic-co-location-methodology.md)
