---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-gis-as-bim-substrate.md
audience: cross-cluster-bridge
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 4
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  Derived from project-gis manifest "Bridge to BIM" language and Doctrine claim #41
  (City Code as Composable Geometry). Authored from project-gis side; consumed by
  project-bim. This is the substrate-side documentation: what the GIS dataset offers
  to a BIM composition pipeline.
research_inline: false
notes_for_editor: |
  This is a cross-cluster bridge document. Consumer is project-bim (the BIM cluster)
  rather than the customer wiki directly. Editorial pass should preserve the
  technical specificity; project-bim Task agents will ingest this to plan their
  city-code geometry composition.
---

# GIS as a BIM Substrate

Building Information Modelling occupies the building scale: structural geometry, material assemblies, mechanical systems, occupancy. A model is meaningful in isolation, but its commercial value emerges when it is sited — when the model is positioned in a real geography with real neighbours, real catchments, real regulatory context. The Foundry GIS substrate is designed to provide that siting context to BIM compositions in the project-bim cluster.

This article documents what the GIS dataset offers a BIM consumer, what fields are stable, and what extensions are anticipated.

## The Cluster Manifold

The primary GIS output is a manifold of approximately 6,400 deduplicated commercial co-location clusters across the United States, Canada, Mexico, the United Kingdom, and continental Europe. Each cluster carries a stable identifier and a fixed geographic position (latitude/longitude), the regional name resolved through the layered boundary engine, the tier classification, the categorical composition (which anchor types are present), and the count of stores within nested catchment radii (1 km, 2 km, 3 km).

For a BIM consumer, this manifold answers questions a model alone cannot: How densely commercial is the area within 3 km of this proposed building? Which anchor formats already serve the catchment? Where is the nearest equivalent existing site against which a model could be benchmarked?

## What the Cluster Properties Carry

Each cluster's properties record carries fields suitable for direct ingest by a city-code composer:

| Field | Type | BIM use |
|---|---|---|
| `cluster_id` | string | Stable join key |
| `latitude`, `longitude`, `centroid_lat`, `centroid_lon` | float | Anchor and centroid positions for siting |
| `region_name` | string | Resolved metro / municipal name; useful as a model parameter |
| `tier_descriptor` | string | "Prime" / "Strong (Retail)" / etc. — a single-word density signal |
| `count_1km`, `count_3km` | integer | Catchment density |
| `unique_brands` | integer | Distinct retail brands within catchment |
| `merged_zones` | array | Same-zone clusters that were consolidated; presented for transparency |
| `iso`, `state` | string | Jurisdiction codes |

Geometry-side, the cluster manifold is published as PMTiles with a layer schema that supports both individual store positions (Layer 1) and cluster envelopes with their proximity rings (Layer 2). A BIM consumer can either fetch the GeoJSON manifest at `/data/clusters-meta.json` for direct lat/lon access or read the PMTiles directly via byte-range requests for spatially indexed queries.

## Region Resolution Depth

The boundary engine resolves coordinates to one of five granularities, listed from most specific to most general: GADM admin-3 (Canadian Census Subdivision proxies, Mexican Municipios), GADM admin-2 (where admin-3 is unavailable), Eurostat NUTS-3 (European regions), Statistics Canada CMA / US Census CBSA (metropolitan areas), Natural Earth admin-1 (state/province global fallback). For a BIM composition that needs to anchor against a municipal jurisdiction — for instance, a building proposed inside the city limits of Strathcona County in Alberta — the GIS engine resolves to that level. For a composition that needs only a metropolitan reference frame, the same engine resolves the surrounding CMA. The resolution returned matches the granularity required by the consumer's query.

## Civic Context Layers

Beyond the cluster manifold, the GIS substrate ingests two civic layers that may be useful to BIM consumers exploring building programmes that depend on civic adjacency:

- **Hospital catalogue.** Approximately 28,000 hospital locations across the operational footprint, sourced from OpenStreetMap and cross-validated against national authority lists where available.
- **University catalogue.** Approximately 19,000 higher-education locations, similarly sourced.

Distance-to-nearest hospital and distance-to-nearest university are computed per cluster (within a 5 km practical limit; longer distances are capped). For a BIM consumer modelling a healthcare-adjacent or campus-adjacent programme, these distances are direct inputs.

## What Is Stable, What Is Likely to Change

Stable: cluster IDs, the manifold structure, the tier classification scheme, the regional name resolution algorithm, the catchment radii.

Likely to change in next sprint quarters: the size of the brand-family taxonomy (food and pharmacy families are expanding), the absolute store counts (OpenStreetMap coverage is improving year over year), the inclusion of additional countries (Belgium, Luxembourg, Ireland, Switzerland are absent from the May 2026 footprint). A BIM composition that joins on `cluster_id` will see growth but no deletion of existing identifiers; a composition that joins on `region_name` should be aware that text values may shift slightly with each region-engine refinement.

## Open Question

How should a BIM composition handle the case where a model is sited at coordinates that do not fall inside any existing cluster's catchment radius? Possible answers: (a) interpolate from the nearest cluster; (b) treat as out-of-coverage and decline to siting-rank; (c) trigger an Opportunity Lens evaluation per the PRODUCT_VISION.md leapfrog feature (when implemented). This is a design question for project-bim to resolve as it composes its city-code grammar.

## Suggested for Next Editor

1. The PRODUCT_VISION.md Opportunity Engine, if implemented, would supply BIM compositions with a "what could be" overlay alongside the current "what is" cluster manifold. A bridge article should be written when that feature reaches design-research stage in project-bim.

## See Also

- [Retail Co-location Methodology](topic-co-location-methodology.md)
- [Regional Name Resolution Architecture](topic-regional-name-resolution-architecture.md)
- project-bim cluster (cross-cluster reference)
