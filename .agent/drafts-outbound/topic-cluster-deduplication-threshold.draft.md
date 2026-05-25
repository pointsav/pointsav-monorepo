---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-cluster-deduplication-threshold.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Session 8 deduplication threshold calibration work (2026-05-08).
  Edmonton case study: Home Depot at -113.4171 / 53.5946 and Costco at -113.4169 / 53.5958
  — 20 m apart, same commercial zone, correctly deduplicated at 0.15 km threshold.
  Threshold reduced from 0.5 km after field observation that zones 200–500 m apart
  (genuinely separate strip malls) were being silently removed at the higher setting.
  Raw cluster count change: 7,594 → 6,422 (1,172 duplicates removed).
research_inline: false
notes_for_editor: |
  Bloomberg-register polish required.
  Keep the distance comparisons precise — these are literal implementation parameters.
  Do not soften "silently removed" — the suppression without user notification is the point.
---

# Cluster Deduplication Threshold

The co-location index pipeline produces one cluster per anchor store — every Walmart, IKEA, Home Depot, and Costco generates its own candidate cluster centred on that store's coordinates. When two anchors occupy the same commercial zone, the result is two overlapping clusters representing the same trade area. The deduplication threshold resolves that redundancy.

## The Same-Parking-Lot Problem

Large commercial zones frequently host two or more anchor-category stores within metres of each other. A Home Depot and a Costco sharing a parking lot in suburban Edmonton, for example, sit roughly 20 metres apart. Without deduplication, both stores produce clusters with nearly identical catchment geometry, co-tenants, and scores. The map displays two concentric rings covering the same zone — neither wrong in isolation, but together misleading about the density of distinct commercial nodes in that corridor.

## Threshold Selection: 0.15 km

The deduplication step removes any cluster whose anchor store falls within a fixed radius of a higher-ranked cluster anchor already confirmed for retention. The threshold is set at **0.15 km (150 metres)**. At this distance, only stores that genuinely share a single parking lot or immediate building complex are collapsed. Anchors in adjacent strip malls separated by a service road (typically 200–500 metres apart) are treated as distinct nodes and retained.

An earlier implementation used a 0.50 km threshold. Field review of the Edmonton metropolitan area identified several cases where legitimate, separately operated zones were being silently removed: a Walmart-anchored node and a Home Depot–anchored node in neighbouring commercial blocks, serving different residential catchments, were treated as duplicates and one was suppressed. The 0.50 km threshold proved too coarse for dense suburban corridors in Canadian and North American markets.

## Ranking the Winner

When two anchors fall within the threshold distance, the retained cluster is the one with the higher count of co-tenants within the 3 km catchment radius (`count_3km`). Ties break on the 1 km count (`count_1km`). This ensures that the cluster representing the fuller commercial zone — more stores, broader multi-purpose draw — survives, regardless of which anchor happened to be processed first.

## Pipeline Effect

After applying the 0.15 km threshold, the June 2026 pipeline run produced 6,422 retained clusters from 7,594 candidates — 1,172 same-zone duplicates removed. The reduction is concentrated in dense commercial corridors where multiple anchor formats (hardware, warehouse club, hypermarket) co-locate in close proximity. Tier distribution and national rankings are assigned after deduplication runs, so the counts reflect deduplicated zones only.

## See Also

- [Retail Co-location Methodology](topic-co-location-methodology.md)
- [North American Tier Index](topic-tier-index-north-america.md)
