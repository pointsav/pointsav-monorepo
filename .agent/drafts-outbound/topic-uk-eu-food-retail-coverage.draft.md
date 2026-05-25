---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-uk-eu-food-retail-coverage.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 4
research_suggested_count: 0
open_questions_count: 1
research_provenance: |
  Derived from GIS Sprint 10 Phase D + F (UK + EU food fill). 12 new chain
  configurations added. Wikidata identifiers verified against the canonical
  brand entities. Aldi Süd / Aldi Nord asymmetry documented. Aldi-NL coverage
  gap surfaced and resolved in Sprint 11 via name_query fallback.
research_inline: false
notes_for_editor: |
  Bloomberg-register polish required. Per Sprint 9 brand-family taxonomy,
  Food family is data-layer only — visible on the map but NOT in the co-location
  scoring. This distinction is commercially important; do not soften it.
---

# UK and European Food Retail Coverage

The co-location index distinguishes between retail chains that participate in the cluster scoring algorithm — anchors, hardware, warehouse — and chains that appear on the map as supporting context but do not affect cluster grades. The latter category is the Food family. This article documents the United Kingdom and European Union food-retail coverage as it stands after the May 2026 expansion.

## Why Food is Data-Only

The co-location methodology measures the convergence of large-format anchor stores: hypermarkets, hardware retailers, warehouse clubs. Adding a grocery chain to the scoring algorithm would dilute the signal — most urban areas have many grocers, so a "co-location" of one anchor and one grocer would tell us little about commercial density. Keeping Food data-layer-only preserves the index as a measure of large-format convergence while still showing field operators the broader retail context around an anchor.

A Tesco store appears on the map as a green dot. It does not contribute to any cluster grade. It does appear in the inspector panel of the surrounding cluster as part of the All Locations layer.

## United Kingdom Coverage

Three chains anchor the UK food layer as of May 2026:

| Chain | Wikidata | Approx. Stores | Notes |
|---|---|---|---|
| Tesco | Q487494 | 3,300 | Largest UK grocer. Sub-formats Extra, Superstore, Metro, Express are handled via the post-ingest sub-entity pass. |
| Sainsbury's | Q152096 | 1,400 | Local + Superstore sub-formats. |
| Lidl GB | Q151954 | 960 | Discount segment. Wikidata identifier is the global Lidl entity; country bounding box confines results to GB. |

These join the prior UK retail coverage of B&Q (hardware), IKEA (anchor), and Costco (anchor + warehouse). The Food layer roughly triples the on-map dot density across the United Kingdom while leaving the cluster grades unchanged.

## European Union Coverage

Five Lidl country instances and four Aldi country instances were added in May 2026:

| Lidl | Wikidata | Approx. Stores | | Aldi | Wikidata | Approx. Stores |
|---|---|---|---|---|---|---|
| Lidl Germany | Q151954 | 3,250 | | Aldi Germany | Q41171 + name_query | 4,200 |
| Lidl France | Q151954 | 1,600 | | Aldi UK | Q41171 + name_query | 1,000 |
| Lidl Netherlands | Q151954 | 440 | | Aldi Netherlands | Q125054 + name_query | ~480 |
| Lidl Austria | Q151954 | 260 | | Aldi Poland | Q41171 + name_query | 280 |
| Lidl Portugal | Q151954 | 270 | | | | |

Pre-existing European Lidl coverage (Spain, Greece, Italy, Poland, the Nordic aggregate) is unchanged.

The Aldi entries warrant a footnote. Aldi operates as two corporate entities — Aldi Süd (Wikidata Q41171) and Aldi Nord (Wikidata Q125054) — that split European geography. In Germany, both entities operate under the Aldi brand on a north-south split. In the Netherlands and other Nordic-adjacent markets, only Aldi Nord operates. In the United Kingdom and Poland, only Aldi Süd operates. The OpenStreetMap brand:wikidata tag on individual store records is inconsistent — many stores carry one identifier, the other, or neither. To reach acceptable coverage in markets where the Wikidata tag is sparse, the ingest configuration falls back to a name-based query ("Aldi") confined to the country's bounding box. This produces store counts close to the operator's published expectations.

## Coverage Gaps Surfaced

Three coverage observations, surfaced for operator review:

**Aldi Netherlands undercoverage.** The first ingest pass returned three records via the Q125054 Wikidata query, well below the expected ~480 stores. Sprint 11 swapped the configuration to force the name_query fallback, restoring coverage to several hundred records.

**Tesco sub-format leak.** Tesco operates Express stores in transport-hub formats (railway stations, motorway service areas). A small number of these may be tagged with non-store amenity tags in OpenStreetMap and dropped during the fuel-station and pharmacy filter pass. Field-team review of urban Tesco coverage is encouraged.

**EU food-family expansion is not exhaustive.** Carrefour France, Auchan, Mercadona Spain, and other major European grocers are not yet ingested in their home countries. Adding them is mechanical (one chain configuration each) but was not in the May 2026 sprint scope.

## Why This Matters

For a Woodfine customer evaluating a development opportunity in the United Kingdom or Continental Europe, the co-location grade alone tells one story: how many large-format anchors converge on this site. The Food layer adds a second story: what is the surrounding daily-trip retail density. A cluster with a Prime grade and a high local Food density behaves differently in the field from a cluster with the same grade and a sparse Food density. Keeping the two layers analytically separate but visually present is the design intent.

## See Also

- [Retail Brand Family Taxonomy](topic-retail-brand-family-taxonomy.md)
- [Retail Co-location Methodology](topic-co-location-methodology.md)
- [Adding a New Chain to the GIS Pipeline](guide-gis-adding-a-chain.md)
