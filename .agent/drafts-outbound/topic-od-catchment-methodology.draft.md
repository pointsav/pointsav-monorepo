---
schema: foundry-draft-v1
type: TOPIC
slug: topic-od-catchment-methodology
title: "O-D Catchment Methodology: Primary and Secondary Trade Areas"
language_protocol: project-editorial
status: draft
created: 2026-05-14
author: task@project-gis
destination: content-wiki-documentation
open_questions:
  - "Should the 35km primary radius be published as a fixed parameter or explicitly flagged as provisional?"
  - "How should we describe HOME vs AWAY when AWAY data is not yet available?"
research_trail:
  method: "Derived from synthesize-od-study.py implementation; H3 ring parameters from verify-data-radius.py precedent"
  sources: ["WorldPop 2026 100m raster aggregated to H3 res-7", "clusters-meta.json 6,815 centroids"]
  assumptions: "35km primary radius is provisional — subject to refinement once O-D validation data becomes available"
  gaps: "AWAY (daytime/workplace population) layer deferred pending external data source"
  date: 2026-05-14
cites: []
---

# O-D Catchment Methodology: Primary and Secondary Trade Areas

## Summary

The Woodfine Location Intelligence platform defines trade areas for each co-location
cluster using an Origin-Destination (O-D) model based on crow-flies distance rings
over a hexagonal spatial grid. Each cluster is assigned two catchment zones that
determine which census and spend data is attributed to it.

## Spatial Framework

Trade areas are computed using the H3 global hexagonal grid at resolution 7.
Each H3 res-7 cell covers approximately 5.16 km² with a center-to-center spacing
of approximately 2.11 km. The grid is continuous and consistent worldwide, enabling
direct comparison between clusters across all 13 countries in the current dataset.

## Catchment Zone Definitions

**Primary catchment:** All H3 res-7 cells whose centre point falls within 35 km
(crow-flies) of the cluster centroid. This zone represents the immediate trade area
where the majority of regular shopping trips originate.

**Secondary catchment:** All H3 res-7 cells whose centre point falls between 35 km
and 150 km (crow-flies) of the cluster centroid. This zone captures the wider regional
draw, including occasional shoppers and cross-town trips.

The 35 km primary boundary is a provisional parameter based on established retail
geography conventions. It is subject to refinement once empirical origin-destination
data becomes available.

The 150 km outer boundary aligns with the platform's data collection radius, ensuring
that every cell contributing to a cluster's catchment has been ingested and verified.

## Distance Method

All distances are calculated as the crow-flies (great-circle) distance using the
Haversine formula. No drive-time routing is used. This approach is:

- Reproducible without map routing infrastructure
- Consistent across urban and rural geographies
- Computationally efficient over millions of H3 cells
- Suitable as a baseline before empirical O-D data is available

H3 ring traversal is used to identify candidate cells efficiently (17 rings ≈ 35 km;
72 rings ≈ 150 km at res-7), with haversine as the definitive distance measure.

## HOME and AWAY Perspectives

The platform distinguishes two perspectives on catchment population:

**HOME:** Population counts derived from residential data (WorldPop 2026).
Represents where people live within each catchment zone. This is the default
view and is fully implemented.

**AWAY:** Population counts representing daytime or workplace population.
Because workplace distribution differs from residential distribution — concentrated
in commercial districts and employment centres rather than dispersed across
suburbs — the AWAY view produces differently shaped catchments for clusters in
or near urban employment cores. This perspective is planned; the data source is
pending.

## One Cell, Multiple Clusters

A single H3 cell may fall within the catchment of multiple co-location clusters.
This is intentional: trade areas are not exclusive territories. A household
within 35 km of two competing clusters contributes to both clusters' primary
catchment populations. This reflects the competitive retail landscape accurately
and is foundational to the cross-cluster comparison methodology.

## Application

Catchment zone membership is the basis for:
- Population aggregation (census data by zone)
- Spend aggregation (grocery, hardware, wholesale spend by zone)
- Cross-cluster competitive ranking (see: Catchment Ranking Methodology)

The catchment polygons displayed on the map are generated from the same
35 km / 150 km crow-flies radii and are visualised in two distinct colours
to distinguish primary from secondary zones.
