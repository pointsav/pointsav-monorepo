---
schema: foundry-draft-v1
type: TOPIC
slug: topic-catchment-ranking-methodology
title: "Catchment Ranking Methodology: Cross-Cluster Comparison"
language_protocol: project-editorial
status: draft
created: 2026-05-14
author: task@project-gis
destination: content-wiki-documentation
open_questions:
  - "Should rank dimensions be published as separate fields or combined into a single composite rank?"
  - "What is the appropriate display format for catchment rank in the BentoBox — percentile or ordinal?"
research_trail:
  method: "Derived from rank-catchments logic in synthesize-od-study.py"
  sources: ["catchment-data.json (pipeline output)", "clusters-meta.json 6,815 entries"]
  assumptions: "No weights applied in v1; all rank dimensions are independent; combined primary+secondary used"
  gaps: "FX normalisation required before cross-country spend rankings are meaningful"
  date: 2026-05-14
cites: []
---

# Catchment Ranking Methodology: Cross-Cluster Comparison

## Purpose

Each co-location cluster on the platform is ranked against all other clusters
in the dataset using trade area population and spend potential data. Rankings
provide a quantitative basis for comparing the commercial significance of
different retail nodes.

## Ranking Dimensions

Clusters are ranked independently on four dimensions:

| Rank field | Basis |
|---|---|
| Population rank | Combined primary + secondary catchment population |
| Grocery rank | Combined primary + secondary grocery spend potential |
| Hardware rank | Combined primary + secondary hardware spend potential |
| Wholesale rank | Combined primary + secondary wholesale spend potential |

Rank 1 indicates the highest value; rank N indicates the lowest, where N is
the total number of clusters in the dataset (currently 6,815).

## Combined Zone Approach

Rankings use the sum of primary and secondary zone values rather than either
zone alone. This reflects the commercial reality that a retailer's trade area
encompasses both proximate and regional shoppers. Using the combined total:

- Avoids penalising clusters in lower-density primary zones that serve large
  regional populations
- Avoids over-rewarding clusters in high-density areas with limited regional reach
- Aligns with standard retail geography practice for total trade area analysis

## No Weighting in Current Version

The four ranking dimensions are independent and unweighted. No composite score
is computed. This is intentional: the pipeline is designed to verify data
integrity and geometric correctness before introducing weighting models.

A future version will apply category-specific weights to produce a composite
catchment rank. Candidate weighting approaches include:

- Chain-type weighting (hardware spend weighted more heavily for hardware co-locations)
- Store-count normalisation (spend per anchor chain rather than total spend)
- Density adjustment (spend potential per km² of primary catchment)

## Currency and Cross-Country Comparisons

Spend values are in local currency (USD, CAD, MXN, GBP, EUR, PLN). Within
the eurozone and within individual countries, spend rankings are directly
comparable. Cross-currency comparisons — for example, ranking a US cluster
against a German cluster by grocery spend — require FX normalisation, which
is not applied in the current version.

Population rankings are currency-neutral and fully comparable across all 13
countries.

## Display

Catchment rank is displayed in the cluster detail panel alongside population
and spend totals. The displayed format is: "Rank N of M" where M is the total
number of clusters. Percentile display is planned for a future version.
