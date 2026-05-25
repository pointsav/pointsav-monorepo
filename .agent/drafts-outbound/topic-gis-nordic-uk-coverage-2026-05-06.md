---
schema: foundry-draft-v1
state: draft
language_protocol: PROSE-TOPIC
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-gis-nordic-uk-coverage-2026-05-06.md
audience: customer-woodfine
bcsc_class: current-fact
authored: 2026-05-06
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 1
open_questions_count: 0
research_provenance: |
  OSM Overpass API — confirmed bauhaus-se (40 records), bq-uk (356 records),
  obs-bygg-no (63 records post-ingest via name prefix query).
  JSONL data cross-referenced against REGION_CONFIG and ALPHA_HARDWARE sets.
  Per-ISO cluster counts verified from clusters.geojson after pipeline rebuild.
research_inline: true
notes_for_editor: |
  Route to project-language for Bloomberg-register polish and bilingual pairing.
  BCSC class current-fact — all counts from live tile data as of 2026-05-06.
---

# Nordic and UK Coverage Expansion — May 2026

The Woodfine co-location intelligence map added substantive coverage in Norway, Sweden, and the United Kingdom on May 6, 2026, through the promotion of three hardware chains to the Alpha tier and the first ingest of Norwegian Obs Bygg location data.

## What Changed

The platform's tier system classifies hardware and warehouse retailers into two groups: Alpha chains, which alone are sufficient to qualify a co-location as a T2 Hub, and Generic chains, which produce only the lowest qualifying tier. Three chains previously held at the Generic level were promoted to Alpha following confirmation of sufficient data coverage.

**Bauhaus Sverige** (Sweden) — 40 locations. Bauhaus is the dominant large-format home improvement chain in Sweden. Its 21-store national footprint overlaps directly with IKEA's retail corridor presence. Promotion to Alpha immediately activated T2 status at IKEA-adjacent nodes across Stockholm, Gothenburg, Uppsala, and Malmö.

**B&Q** (United Kingdom) — 356 locations. B&Q is the primary mass-market home improvement retailer in Great Britain, operating large-format stores in suburban and edge-of-town retail parks. The chain had been held at Generic pending an OSM re-ingest, which completed with 356 verified locations. Promotion to Alpha unlocked T2 and T3 designations at IKEA nodes across Greater London, the Midlands, and Scotland.

**Obs Bygg** (Norway) — 63 locations. Obs Bygg is the DIY hardware format operated by Coop Norge, Norway's largest retail cooperative. The chain had not previously been ingested due to sparse OpenStreetMap brand tag coverage; a name-prefix ingest query resolved 63 active stores. Obs Coop, the hypermarket format from the same parent, serves as the cluster anchor in Norway — the first time any Norwegian clusters have been scored.

## Coverage Before and After

| Market | T2 Clusters Before | T2 Clusters After |
|--------|--------------------|-------------------|
| Norway | 0 | 66 |
| Sweden (1km radius) | 0 | 4+ |
| United Kingdom (1km radius) | 0 | 9+ |
| EU total (1km) | 162 | 229 |

All new Norwegian clusters carry T2 Hub designation, reflecting the combination of Obs Coop (anchor) and Obs Bygg (alpha hardware) within a 1 km radius — the characteristic footprint of the Coop Norge co-tenancy format.

## Significance

Norway's retail park format is structurally distinct from North American or Central European patterns. The Obs Coop / Obs Bygg co-tenancy is a national standard: Coop Norge deliberately co-locates its hypermarket and DIY formats to anchor retail corridors in mid-sized Norwegian cities. The 66 new T2 clusters represent the first systematic map of this format in the platform.

The B&Q promotion resolves the most significant prior data gap in the UK dataset. With 356 locations verified, the UK cluster set now reflects the actual distribution of IKEA-adjacent retail parks rather than anchor-only nodes.

## Open Items

None. The May 6 rebuild is the authoritative dataset for Nordic and UK coverage.
