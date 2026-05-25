---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: master
target_repo: pointsav-design-system
target_path: research/
target_filename: zoom-tier-reveal-pattern.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: DESIGN-RESEARCH
authored: 2026-04-30T04:05:00Z
authored_by: master-claude (workspace v0.1.94 session)
authored_with: opus-4-7
research_done_count: 4
research_suggested_count: 5
open_questions_count: 3
research_provenance: sub-agent
research_inline: true
references:
  - https://gis.woodfinegroup.com  # live reference implementation
  - https://maplibre.org/maplibre-gl-js/docs/  # MapLibre paint/layout filter docs
  - conventions/cluster-design-draft-pipeline.md
notes_for_designer: |
  This is a foundational design-system research entry — establishes the
  pattern that downstream map components (markers, clusters, drawer
  triggers) compose against. Sibling to the brand-family-swatch component;
  the cluster-centroid ring variant of brand-family-swatch is the visual
  manifestation of this pattern at zoom < threshold.

  No DTCG token output expected; this is a pattern-and-rule research entry,
  not a component recipe. Refined output goes to research/.
---

# Design Research — Zoom-Tier Reveal Pattern

## Pattern statement

A map view that holds many features benefits from a **zoom-tier reveal
pattern** — at low zoom levels, only aggregate representations (cluster
centroids) render; as the user zooms past a threshold, individual features
reveal. This is not pure clustering; the aggregates carry meaningful data
(family distribution, cluster grade) that survives the threshold crossing.

## Why this matters

Industry-standard map clustering (MapLibre native cluster source, Mapbox
clusterRadius) collapses many points into a count-only marker — "73 stores
in this area" — losing all per-feature semantics. Zoom-tier reveal, by
contrast, preserves the family-distribution semantics at every zoom level:

- **At zoom < 8.5**: cluster centroid rings render, each ring showing the
  brand-family distribution for that cluster (1 Department + 2 Hardware +
  1 Warehouse Club rendered as concentric / pie ring).
- **At zoom ≥ 8**: individual anchor swatches render. Cluster centroids
  fade out via opacity transition (tile-zoom-blend interval 8.0 → 8.5).

The cluster centroid is **first-class data**, not a rendering optimisation.
This aligns with leapfrog invention #1 (cluster-grade as first-class
entity) from the v0.1.94 list.

## Threshold selection

The threshold zoom level is determined by:

1. **Cluster spread** — at what zoom does the average cluster occupy ≥1
   tile (i.e., individual anchors are visually separable)?
2. **Density signal** — at what zoom does the cluster centroid carry more
   information than 4 individual swatches at the same screen position?
3. **Customer story** — at what zoom does the customer's narrative shift
   from "global / regional pattern" to "this specific cluster"?

The current GIS surface uses **zoom 8 - 8.5** as the transition band. Below
8: cluster centroids only. Above 8.5: individual anchors only. Between:
both render with crossfade opacity.

## Composition with other patterns

- **Country filter chips** — when a country chip is selected, the flyTo
  zoom level is set just below the threshold (e.g., zoom 4 for US-extent,
  6 for ES-extent), keeping the cluster-centroid view active and allowing
  the user to zoom in to a specific corridor.
- **Side drawer** — clicking a cluster centroid flyTo's to zoom 13 (well
  above threshold), revealing individual anchors and opening the drawer
  for the cluster context (not for any single anchor).
- **Brand-family swatch** — the cluster-centroid ring variant is the
  multi-family aggregate representation at zoom < threshold; the inline
  chip / map marker variant is the per-anchor representation at zoom ≥
  threshold.

## Implementation rules (MapLibre)

```javascript
map.addLayer({
  id: 'cluster-centroids',
  type: 'circle',
  source: 'co-locations',
  maxzoom: 8.5,                    // hide above 8.5
  paint: {
    'circle-opacity': [
      'interpolate', ['linear'], ['zoom'],
      8.0, 1.0,
      8.5, 0.0
    ]
    // ... family-ring rendering via paint expressions
  }
});

map.addLayer({
  id: 'individual-anchors',
  type: 'circle',
  source: 'places',
  minzoom: 8.0,                    // show above 8.0
  paint: {
    'circle-opacity': [
      'interpolate', ['linear'], ['zoom'],
      8.0, 0.0,
      8.5, 1.0
    ]
    // ... family-swatch rendering via paint expressions
  }
});
```

The 8.0 → 8.5 crossfade band is the key UX detail. Hard threshold (e.g.,
both layers swap at zoom 8 exactly) creates a visible "snap" that breaks
spatial continuity.

## Accessibility considerations

- **Reduced motion** — `prefers-reduced-motion: reduce` collapses the
  crossfade to instant swap. The user still sees both representations,
  just without the transition animation.
- **Screen-reader semantics** — the map is announced as a region; the
  cluster centroids and individual anchors are announced as named
  features at their respective zoom levels. Zoom-changes do not produce
  auditory events; the user controls disclosure via interaction (click /
  Tab through markers).
- **Keyboard navigation** — keyboard-arrow zoom on the map respects the
  threshold; pressing `+` past zoom 8 reveals individual anchors in the
  Tab order.

## Research trail

### Done

- [https://gis.woodfinegroup.com] reference implementation observed; 12 cluster centroids visible at zoom < 8.5; 41 individual anchors visible at zoom ≥ 8; crossfade transition operational.
- [https://maplibre.org/maplibre-gl-js/docs/] MapLibre `minzoom` / `maxzoom` layer property documented; `interpolate` paint expression for opacity confirmed as the recommended crossfade mechanism.
- Sonnet sub-agent retail-mapping-UI-research dispatched 2026-04-30 returned: industry-standard clustering (Mapbox / Leaflet.markercluster / Supercluster) collapses to count-only markers; this pattern is novel in preserving family-distribution semantics across the zoom threshold.
- Operator demo confirmed at v0.1.94: 12-corridor view at zoom 2.6 shows cluster centroids legibly; clicking a centroid flies to zoom 13 with individual anchors.

### Suggested

- Validate the 8.0 → 8.5 crossfade band on production data at scale (target: 500+ corridors, 5,000+ anchors); the band may need to widen at higher density.
- Measure threshold-tier UX with eye-tracking — does the crossfade band cause split-attention?
- Compare with industry-convergent clustering (MapLibre clustering / Supercluster) on identical data; document the visual / semantic differences.
- Audit the 8.0 lower / 8.5 upper bounds against country-extent zoom levels — does any country (Spain, Mexico) sit awkwardly close to the threshold by default?
- Test zoom-tier reveal on `prefers-reduced-motion` users — does instant swap produce a usable experience?

### Open questions

- Three-tier reveal (cluster centroid → cluster expansion → individual anchor) for very dense clusters (100+ anchors) — needed, or does the 2-tier pattern hold?
- Cluster centroid click behaviour: flyTo zoom 13 (current) vs flyTo zoom = threshold + 1 (just past the reveal point)? Latter preserves more spatial context.
- Threshold per layer — should each map layer have its own threshold, or is a single global threshold sufficient? Decision pending Phase-2 mobility-layer composition work.

## Provenance

Reference implementation deployed via workspace v0.1.94 (2026-04-30) at
`gateway-orchestration-gis-1`. Pattern is foundational research for
downstream map components and aligns with leapfrog invention #1
(cluster-grade as first-class entity) from the v0.1.94 list.
