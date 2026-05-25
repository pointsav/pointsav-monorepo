---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: text-gis-uk-eu-coverage-release.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TEXT
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 10 (UK + EU food coverage) and Sprint 11 (region
  granularity + bbox contamination correction). Bloomberg release-note format
  per POINTSAV-Project-Instructions.md. Companion to topic-uk-eu-food-retail-
  coverage.draft.md and topic-regional-name-resolution-architecture.draft.md.
research_inline: false
notes_for_editor: |
  Bloomberg-register polish required. Strict word budget — release note format,
  not a feature article. Verify all numbers against the topic articles before
  publication.
---

# Woodfine GIS — May 2026 Coverage Update

The Woodfine Location Intelligence platform expanded its operational footprint and refined its boundary resolution in the May 2026 release. Changes are live at gis.woodfinegroup.com.

## United Kingdom and Continental Europe

Twelve grocery chain instances were added across the United Kingdom and continental Europe. The platform now ingests Tesco, Sainsbury's, and Lidl in the United Kingdom; Lidl in Germany, France, the Netherlands, Austria, and Portugal; and Aldi in Germany, the United Kingdom, the Netherlands, and Poland. These join the existing Lidl coverage in Spain, Greece, Italy, Poland, and the Nordic countries.

The new chains are classified within the platform's Food brand family, which appears on the map as supporting context but does not contribute to the co-location cluster grades. The distinction preserves the index as a measure of large-format anchor convergence while giving field operators a fuller picture of daily-trip retail density around each cluster. Approximately 15,000 grocery store records were added to the May 2026 dataset.

## Regional Name Granularity

The platform's offline reverse-geocoding engine now resolves Canadian and Mexican coordinates to municipal granularity rather than metropolitan-area-only. Where prior versions returned a single name for the Edmonton Census Metropolitan Area, the May 2026 release distinguishes Edmonton, Strathcona County (which contains Sherwood Park), Spruce Grove, Leduc County, Sturgeon County, and Fort Saskatchewan as separate cluster regions. Mexican clusters now resolve to municipios — Cuauhtémoc, Coyoacán, Iztapalapa, Boca del Río — rather than the surrounding state names.

Boundary data was sourced from the Global Administrative Areas project's 2022 admin-2 and admin-3 shapefiles, supplementing the existing Statistics Canada and Eurostat boundary layers. Total distinct regional names produced by the engine grew from approximately 1,015 in April 2026 to approximately 1,200 in May 2026.

## Cross-Border Data Correction

A polygon-containment filter was added to the OpenStreetMap ingest stage in May 2026. The filter drops records whose coordinates fall outside the country polygon declared on the chain configuration. Prior to the filter, queries for Canadian retail chains returned cross-border records from the United States — Home Depot stores in Detroit appearing as Canadian rows, Walmart stores in Buffalo appearing as Canadian rows — because OpenStreetMap data does not always carry an explicit country tag.

The correction reduced the Canadian retail record count by approximately 900 stores, a 25% decrease in raw data volume but with no decrease in coverage of legitimate Canadian stores. Cluster counts for the Canadian footprint dropped from 712 to 483 as the phantom border-zone clusters were removed; the corresponding 234-cluster increase in the United States footprint reflects records correctly re-attributed.

## Tier Label Refresh

The cluster grade labels visible in the map's inspector panel were renamed in May 2026 from a numeric scheme (T0, T1, T2, T3) with technical descriptors to a Plain-English scheme: Prime, Strong (with a parenthetical specialty marker), Core (with specialty marker), and Emerging. The methodology is unchanged. Underlying tier numbers remain available to analytical users via the API and to screen-reader users via the inspector panel's ARIA label.

## What Is Next

The platform's preferred boundary file for Mexico — the National Institute of Statistics and Geography's Zona Metropolitana shapefile — exceeds practical sprint download budgets at 3.3 GB. The Global Administrative Areas substitute is the operational source as of May 2026; a future release will integrate the official file when bandwidth and processing time permit. Coverage of Belgium, Luxembourg, Ireland, and Switzerland is anticipated in subsequent quarters, subject to operator priority.

For methodology details, see [Retail Co-location Methodology](topic-co-location-methodology.md), [Regional Name Resolution Architecture](topic-regional-name-resolution-architecture.md), [UK and European Food Retail Coverage](topic-uk-eu-food-retail-coverage.md), and [Co-Location Tier Nomenclature](topic-co-location-tier-nomenclature.md).
