---
schema: foundry-draft-v1
type: TOPIC
slug: topic-trade-area-data-sources
title: "Trade Area Data Sources: Population and Spend"
language_protocol: project-editorial
status: draft
created: 2026-05-14
author: task@project-gis
destination: content-wiki-documentation
open_questions:
  - "Should the per-capita spend multipliers be published openly or treated as proprietary model parameters?"
  - "Currency normalisation — should we convert all spend to USD/EUR for cross-country comparison?"
research_trail:
  method: "Derived from ingest-spend.py SPEND_MULTIPLIERS constants and bin-census-h3.py implementation"
  sources: ["WorldPop 2026 100m raster (worldpop.org)", "BLS Consumer Expenditure Survey (US proxy)", "StatCan Household Expenditures (CA proxy)", "Eurostat Household Budget Survey (EU proxy)"]
  assumptions: "Spend multipliers are annual per-capita estimates; regional currency not normalised in v1"
  gaps: "Mexico spend multipliers are in MXN — cross-country spend comparison requires FX normalisation"
  date: 2026-05-14
cites: []
---

# Trade Area Data Sources: Population and Spend

## Population Data

Population estimates are sourced from the **WorldPop 2026 100-metre population
grid** (worldpop.org). WorldPop produces modelled population estimates derived
from census microdata, satellite imagery, and dasymetric redistribution
techniques. The 100m resolution places population at the sub-block level,
enabling precise trade area delineation.

### Processing Pipeline

The raw WorldPop GeoTIFF rasters are processed as follows:

1. **Spatial filter:** Only grid cells within 150 km of at least one co-location
   cluster centroid are retained. This reduces data volume by approximately 80%
   while preserving all cells relevant to catchment computation.

2. **H3 aggregation:** Retained cells are assigned to their containing H3 res-7
   hexagon and population values are summed. H3 res-7 cells have an average area
   of 5.16 km², producing a hexagonal grid suitable for both display and computation.

3. **Output:** `census-h3-res7.jsonl` — one record per H3 cell with fields
   `{h3, lat, lon, pop, iso}`.

### Countries Covered

USA, Canada, Mexico, Great Britain, Germany, France, Netherlands, Austria,
Portugal, Greece, Denmark, Iceland, Poland — 13 countries as of the current
pipeline version.

## Spend Data

Spend estimates are synthesised by applying **annual per-capita expenditure
multipliers** by retail category to the population grid. The multipliers are
proxies derived from national household expenditure surveys:

| Country | Grocery (p.a.) | Hardware (p.a.) | Wholesale (p.a.) | Currency |
|---------|---------------|-----------------|------------------|----------|
| USA | $3,500 | $1,200 | $1,500 | USD |
| Canada | C$3,200 | C$1,100 | C$1,300 | CAD |
| Mexico | MX$18,000 | MX$3,500 | MX$2,500 | MXN |
| Great Britain | £2,800 | £850 | £900 | GBP |
| Germany | €2,900 | €950 | €1,000 | EUR |
| France | €3,100 | €900 | €1,000 | EUR |
| Netherlands | €2,700 | €1,000 | €1,100 | EUR |
| Austria | €3,000 | €950 | €1,000 | EUR |
| Portugal | €2,400 | €600 | €700 | EUR |
| Greece | €2,200 | €500 | €600 | EUR |
| Denmark | €3,500 | €1,200 | €1,100 | EUR |
| Iceland | €4,000 | €1,500 | €1,500 | EUR |
| Poland | PLN8,000 | PLN2,000 | PLN2,500 | PLN |

**Important:** Multipliers are expressed in local currency. Cross-country spend
comparisons require FX normalisation, which is not applied in the current
pipeline version. Rankings are therefore most meaningful within a single country
or within the eurozone.

### Retail Categories

- **Grocery:** Supermarkets, hypermarkets, food co-ops, and food sections of
  general merchandise retailers.
- **Hardware:** Home improvement, building materials, garden centres.
- **Wholesale:** Members-only warehouse clubs, cash-and-carry.

### Processing Pipeline

Spend values are computed at the H3 res-7 level by multiplying each cell's
aggregated population by the per-capita multipliers for its country. Output:
`cleansed-spend-h3-res7.jsonl` — one record per H3 cell with fields
`{h3, pop, spend_grocery, spend_hardware, spend_wholesale, currency}`.

## Catchment Aggregation

For each co-location cluster, primary and secondary catchment zones are defined
by crow-flies distance rings (see: O-D Catchment Methodology). Population and
spend for all H3 cells within each zone are summed to produce the cluster's
trade area statistics.

These aggregated values are the basis for cross-cluster competitive ranking.
