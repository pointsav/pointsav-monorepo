---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-regional-name-resolution-architecture.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 10 Phase B + C (region_engine.py integration of GADM 4.1
  admin-2 / admin-3 boundaries). All boundary files are open-data, no API keys:
  Census TIGER, Statistics Canada 2021, GADM 4.1 (UC Davis), Eurostat GISCO, Natural
  Earth. Data quality lessons from Sprint 9–11 are captured in the Engineering Notes
  section.
research_inline: false
notes_for_editor: |
  Bloomberg-register polish required. This is a methodology document; precision
  matters. Keep the engineering notes as a footnote section — they document
  data-quality decisions a customer reviewer should be able to audit.
---

# Regional Name Resolution Architecture

The co-location map labels each cluster with a human-readable regional name — a North American Metropolitan Area, a European NUTS-3 region, a Mexican municipio, a Canadian Census Subdivision. The name is not a single field on the source data; it is the output of a layered offline reverse-geocoding pipeline. This article documents the data sources, the lookup order, and the post-processing that produces the names visible at gis.woodfinegroup.com.

## The Five Boundary Layers

Each cluster anchor's coordinates are tested against five boundary GeoJSON files in a country-specific order:

| Layer | Source | Coverage | Granularity |
|---|---|---|---|
| `us_cbsa.geojson` | US Census Bureau TIGER GENZ2023 | United States | Core-Based Statistical Areas (Metro + Micropolitan) |
| `ca_cma.geojson` | Statistics Canada 2021 Census | Canada | Census Metropolitan Areas |
| `ca_csd.geojson` | GADM 4.1 admin-3 (UC Davis Open Data) | Canada | Census Subdivision proxies (municipalities) |
| `mx_municipio.geojson` | GADM 4.1 admin-2 (UC Davis Open Data) | Mexico | Municipios |
| `eu_nuts3.geojson` | Eurostat GISCO 2021 | EU + UK + EFTA + Western Balkans | NUTS-3 regions |
| `fallback_ne_admin1.geojson` | Natural Earth 10m | Global | Admin-1 (states / provinces) |

All files load once at engine initialisation. Spatial indexes (Shapely STRtrees) accelerate point-in-polygon lookups to O(log N) per query.

## Country-Specific Routing

The engine routes each cluster's anchor coordinates by ISO country code:

- **United States**: CBSA lookup. If hit, format the CBSA name (strip state suffix, append " Metro Area" if absent).
- **Canada**: Census Subdivision lookup first (admin-3 — captures Strathcona County, Spruce Grove, Sherwood Park separately from Edmonton). If a CSD hits, also look up the surrounding CMA. When both hit and differ, the result is composed: "Strathcona County, Edmonton". When only the CSD hits, return the CSD alone. When only the CMA hits, return the CMA alone.
- **Mexico**: Municipio lookup (admin-2). On hit, return the municipio name with Spanish-text post-processing applied. On miss, fall through to the global Natural Earth fallback (state level).
- **European Union, United Kingdom, EFTA, Western Balkans**: NUTS-3 lookup. The composite "Nordics" identifier (used for Sweden / Norway / Denmark / Finland / Iceland aggregated bboxes) routes here too.
- **Fallback**: Natural Earth admin-1 for any country not covered by the layered files. Returns state or province names.

Each layer has a tolerance built into its STRtree query: when a point falls just outside any polygon — for instance, a coastal store on a fjord edge — the engine accepts the nearest polygon within ~15 km (0.15° latitude). This prevents legitimate stores in coastal complications from falling through to the fallback layer.

## Post-Processing the Raw Names

Boundary files carry source-language names with concatenated affixes that are not human-readable. Three transformations clean them.

**CamelCase splitter.** GADM 4.1 admin-2 and admin-3 names are stored without word separators. "StrathconaCounty" must become "Strathcona County"; "DivisionNo.11" stays as-is. A regular expression inserts a space between any lowercase letter followed by an uppercase letter.

**Spanish preposition splitter.** Mexican municipio names occasionally carry preposition concatenation: "Bocadel Río", "Apetatitlánde Antonio Carvajal", "Acapulcode Juárez". A second regular expression detects the prepositions *de*, *del*, *la*, *las*, *el*, *los* glued to a preceding lowercase character and inserts a space before the preposition. Matches consume the preposition to prevent overlapping substitutions.

**Period normaliser.** "Gustavo A.Madero" reads better as "Gustavo A. Madero". A third pass inserts a space after a period that precedes a letter.

A separate explicit-override dictionary (`_REGION_CLEAN`) handles cases that fall outside the regular-expression scope: Greek names transliterated to English, Finnish "X-niemi" suffixes simplified, Polish "miasto" prefixes stripped, Belgian bilingual names normalised. This dictionary holds roughly 200 entries as of mid-2026.

## Mexican Display Overrides

Some Mexican municipio names are technically correct but not the form a Spanish-speaking reader expects on a map. The engine keeps a small display-override dictionary that maps INEGI Zona Metropolitana names to their popular short forms — "Zona Metropolitana del Valle de México" becomes "Ciudad de México", "Zona Metropolitana de Guadalajara" becomes "Guadalajara". These overrides apply only to the legacy ZM file (`mx_metro.geojson`); the GADM municipio file uses the `NAME_2` field directly.

## Numbers

After the layered routing and post-processing, the engine produces approximately 1,200 unique region names across the operational footprint as of May 2026. By country: 671 distinct US Metropolitan Areas, 245 Canadian regions (CSDs and CMAs combined), 104 Mexican Municipios, and several hundred European NUTS-3 regions. Each region name appears on the map in cluster pop-ups and the Bento inspector panel.

## Engineering Notes

The Mexican Zona Metropolitana boundary file from INEGI is preferred to the GADM admin-2 substitute but is distributed as a 3.3 GB shapefile that exceeds practical sprint-window download budgets. The GADM admin-2 municipio file (~7.5 MB GeoJSON) is the pragmatic open-data substitute. When the INEGI file is loaded directly, the engine prefers it over the GADM substitute.

The Canadian Census Subdivision file from Statistics Canada is the preferred source for sub-CMA granularity, but the official 2021 download URL has been moved without redirect since the engine was first wired. GADM 4.1 admin-3 (UC Davis) provides the same level of granularity from a stable mirror; "DivisionNo.X" entries that GADM marks as generic are dropped from the post-processed output.

Cross-border bbox contamination — OSM records returned by a Canada-wide bounding-box query that geographically belong to United States stores near the border — is filtered at ingest time using a country-polygon containment check (Sprint 11). This is independent of the region-name lookup but worth noting: the region engine sees only records that have already passed country-polygon containment, so its results are not polluted by border crossings.

## See Also

- [Retail Co-location Methodology](topic-co-location-methodology.md)
- [Cluster Deduplication Threshold](topic-cluster-deduplication-threshold.md)
- [Adding a Country to the GIS Pipeline](guide-gis-adding-a-country.md)
