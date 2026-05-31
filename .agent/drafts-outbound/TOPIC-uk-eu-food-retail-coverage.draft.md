---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC-*
title: "UK and EU Food Retail Coverage"
slug: topic-uk-eu-food-retail-coverage
status: staged
destination: media-knowledge-documentation
bcsc_class: no-disclosure-implication
paired_with: TOPIC-uk-eu-food-retail-coverage.es.draft.md
research_done_count: 6
research_suggested_count: 1
open_questions_count: 0
research_provenance: service-business/ JSONL files (per-chain record counts); config.py (ALPHA_HYPERMARKET EU, REGION_CONFIG); taxonomy.py Phase 17+18; Phase 13/15/16/18 session cleanup-log entries; Wikidata QIDs per chain
research_inline: true
created: 2026-05-31
---

# UK and EU Food Retail Coverage

The co-location platform draws on OpenStreetMap point-of-interest data, enriched
with Wikidata entity identifiers, to build its inventory of retail locations. This
article documents which grocery and food retail chains are covered in the United
Kingdom and the European Union, how many locations each chain contributes to the
dataset, and where coverage remains sparse.

## Coverage Summary

As of Phase 18 (2026-05-22), the platform covers food retail locations across
18 countries. The United Kingdom, Germany, and France have the deepest chain
coverage, reflecting both the density of major retail formats in those markets
and the completeness of OSM mapping for large-format stores. Iceland (3 clusters),
Norway (10), and Finland (55) have the thinnest coverage, consistent with smaller
market sizes and sparser OSM mapping in those geographies.

## United Kingdom

The United Kingdom was the subject of a focused re-ingest campaign in Phase 13
(2026-05-17) that substantially expanded coverage.

**Tesco** (Wikidata Q487494) is the largest UK grocery chain by store count and
contributes 3,872 records to the platform. Tesco operates hypermarkets,
supermarkets, and convenience formats; the platform captures primarily the
large-format hypermarket and superstore locations relevant to co-location
analysis. Phase 13 expanded Tesco coverage from 784 to 3,872 records through
a name-query enrichment pass against OSM.

**Sainsbury's** (Q153417) contributes 1,903 records following Phase 13 expansion
from 672 records.

**ASDA** (Q297410) contributes 1,051 records, ingested in Phase 12 (2026-05-17).
ASDA operates predominantly in the large supermarket and hypermarket format range
relevant to T1 and T2 cluster formation.

**Morrisons** (Q922344) contributes 620 records, ingested in Phase 12.

**Wickes** (Q7998350) contributes 236 records. Wickes is a DIY and home
improvement retailer (Travis Perkins Group) classified as an anchor hardware
format. Its presence in a cluster contributes to the hardware anchor requirement
for T1 and T2 designation.

## Germany

Germany has the largest cluster count of any European country in the platform,
with 722 clusters (227 T1, 338 T2, 157 T3) as of Phase 23+Change B.

The German market is characterised by a concentration of large-format grocery
and DIY operators. Long-established chains — Aldi, Lidl, Rewe, and Edeka — were
ingested in earlier build phases and provide broad coverage. Phase 18 (2026-05-22)
added two significant chains.

**Kaufland** (Q685967, Schwarz Group) contributes 253 records in Germany
(additional records in Poland — see below). Kaufland operates full-line
hypermarkets, typically 3,000–12,000 m², qualifying as T1 or T2 anchors.

**Globus** (Q528681, Globus Holding) contributes 125 records. Globus operates
large-format hypermarkets in Germany, classified as ALPHA_HYPERMARKET EU.

## France

France has 624 clusters (247 T1, 161 T2, 216 T3). French hypermarket coverage
is anchored by Carrefour (ingested in earlier phases). Phase 18 added:

**Intermarché Hyper** (Q2029154, Les Mousquetaires) — 56 records of the
hypermarket-format stores.

**Géant Casino** (Q2901839, Casino Group) — 10 records. Géant Casino is the
hypermarket division of Groupe Casino.

**Bricomarché** (Q2896882, Les Mousquetaires) — 497 records. Bricomarché is
a DIY hardware format contributing to the hardware anchor layer.

**Brico Dépôt** (Q3007003, Kingfisher) — 137 records. Kingfisher's French
DIY chain.

## Other European Markets

**Austria:** Interspar (Q1364056, SPAR Austria) contributes 85 records of the
Interspar hypermarket format. Billa+ (Q806085) contributes records for the
expanded Billa hypermarket format.

**Netherlands:** Albert Heijn XL (no Wikidata entity; brand of Ahold Delhaize)
contributes 43 records. Jumbo Foodmarkt (Q14716185) contributes 8 records of
the large-format flagship stores.

**Poland:** Kaufland (Q685967) contributes 253 records. E.Leclerc (Q1273376)
contributes 36 records.

**Italy:** Esselunga (Q1377048) contributes 259 records. Esselunga operates
large-format supermarkets and hypermarkets in northern Italy.

**Greece:** Sklavenitis (Q7536996) contributes 406 records. Location data was
sourced via Greek-language name query (Σκλαβενίτης) in OSM, reflecting the
importance of using native-language queries for non-Latin-script markets.

**Portugal:** Continente (Q5164541, Sonae) contributes 57 records.

**Nordic markets (SE/DK/NO/FI):** IKEA (per-country entities: Q690498 SE,
etc.) and Bauhaus (Q532716) provide the primary large-format anchor coverage.
Føtex (Q3093871, Salling Group) contributes 103 DK records. Bauhaus is present
in SE (ingested earlier), DK (20 records), NO (2 records), and FI (6 records).

## Coverage Gaps

Several geographies within the platform's 18-country scope have known coverage
gaps.

**Iceland** has only 3 clusters, reflecting limited OSM mapping of retail
locations and a small market. No large-format chains with sufficient OSM presence
have been identified for ingest.

**Atlantic Iberia** (western Spain and Portugal beyond Lisbon) and **southern
Italy** have thinner T1 coverage than their population size would suggest,
reflecting gaps in OSM completeness for large-format stores in those areas.

**Scotland** has sparser coverage than England and Wales, primarily due to ASDA
and Morrisons density being lower in Scottish markets.

The Aldi and Lidl discount formats present across Europe are recorded in OSM
but are not classified as ALPHA_HYPERMARKET anchors because their store sizes
fall below the full-hypermarket threshold used for T1/T2 anchor designation.
They contribute to T3 cluster formation as general retail members.

## Data Provenance and Attribution

All location records are sourced from OpenStreetMap, which makes its data
available under the Open Database Licence (ODbL). Wikidata entity identifiers
used to match OSM records to canonical chain identities are CC0. Chain record
counts reflect OSM completeness at the time of each phase's ingest run and will
vary as OSM contributors add or update entries.

---

*Data provenance:* JSONL chain files in `service-business/` (per-chain records);
Phase 12–18 ingest commits (2026-05-17 through 2026-05-22). OSM ODbL; Wikidata CC0.
