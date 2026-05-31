---
schema: foundry-draft-v1
state: draft-pending-design-review
originating_cluster: project-gis
target_repo: vendor/pointsav-design-system
target_path: research/
target_filename: DESIGN-RESEARCH-zoom-prefetch-pattern.md
audience: internal-design
bcsc_class: internal
language_protocol: DESIGN-RESEARCH
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 4
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  Derived from GIS Sprint 9 Phase 2 (zoom-transition redesign). Operator constraint:
  "Eliminate the delay; this is a coding issue not a resources issue. Don't rewrite,
  iterate." Sprint 9 audit identified layer1 PMTile fetched on-demand at threshold
  cross as the bottleneck. Pre-warm pattern eliminates the cold-start paint stall.
  Implementation: index.html:362, 638-668, 1218-1244.
research_inline: false
notes_for_editor: |
  Route to project-design for design-system integration.
  Open question retained for downstream review: should the prefetch trigger be
  zoom-rate-aware (only prefetch on monotonic zoom-in, not on rapid pan)?
---

# DESIGN RESEARCH: Two-Stage Tile Prefetch Pattern

**Surface:** Co-location map with zoom-gated regimes (Co-location Level vs Retail Level).
**Component:** Layered tile mounting / visibility orchestration.
**Status:** Shipped GIS Sprint 9 (`7e92013`); reusable for any layered-tile UI.

---

## The Problem

A map UI with two visual regimes — overview cluster bubbles below threshold, individual retailer dots above — needs to switch atomically when the user crosses the zoom threshold. The naive approach (toggle layer visibility on `zoomend`) introduces a cold-start paint stall: tiles begin fetching only after visibility is set, leaving the user staring at a blank zone for the duration of the network request (~500–1200 ms on a typical CDN).

Three observed failure modes:

1. **Cold tile fetch at threshold**: visibility=visible triggers a wave of tile requests; user watches the blank zone fill in tile-by-tile.
2. **GeoJSON layers paint instantly while raster waits**: when both source types share the regime change, the GeoJSON paints first, raster lags — visible as "skeleton then dots."
3. **Cluster bubbles persist while raster loads**: `map.once('idle')` waits for the slowest source; the obsolete bubbles linger longer than necessary.

## The Decision

Mount the heavier tile-source layer earlier than its visual reveal, with paint suppressed via opacity 0. Reveal at threshold cross by bumping opacity to its design value.

### Three orchestration moves

**1. Pre-warm threshold (`RETAIL_PREFETCH_ZOOM`).**

A constant set two zoom levels below the regime threshold:

```js
const RETAIL_ZOOM_THRESHOLD  = 9;   // regime change here
const RETAIL_PREFETCH_ZOOM   = 7;   // warm the layer here
```

When `z >= RETAIL_PREFETCH_ZOOM`, the heavy tile layer is mounted with `visibility: visible` AND `circle-opacity: 0`. The browser begins fetching tiles immediately; the user sees nothing different. The two-zoom-level head start typically completes the network round-trip before the user reaches threshold.

**2. Reveal-by-opacity at regime cross.**

When `z >= RETAIL_ZOOM_THRESHOLD`, the regime-change handler bumps `circle-opacity` to its design value:

```js
map.setLayoutProperty('all-locations', 'visibility', 'visible');
map.setPaintProperty('all-locations', 'circle-opacity', 0.70);
```

Because the layer is already in the layer stack and tiles are already fetched (or in-flight), the paint is immediate. No second round-trip.

**3. Visibility-flip ordering: GeoJSON before raster.**

Within the regime-change handler, the order matters. GeoJSON layers (in-memory; instant paint) render first; raster/tile layers (network-bound) reveal afterward:

```js
// GeoJSON rings — instant paint
if (map.getLayer('proximity-fill')) map.setLayoutProperty('proximity-fill', 'visibility', 'visible');
if (map.getLayer('proximity-line')) map.setLayoutProperty('proximity-line', 'visibility', 'visible');
updateProximityRings();
// Raster dots — opacity bump (layer already mounted via prefetch)
if (map.getLayer('all-locations')) {
    map.setLayoutProperty('all-locations', 'visibility', 'visible');
    map.setPaintProperty('all-locations', 'circle-opacity', 0.70);
}
```

Users see rings paint immediately while raster streams in. Perceived responsiveness improves even when raster is genuinely slow.

## Reusable Pattern

**When a UI has two zoom regimes with a tile-source-backed component:**

1. Define `THRESHOLD` (regime change) and `PREFETCH_ZOOM` (typically `THRESHOLD - 2`).
2. On zoomend, when `z >= PREFETCH_ZOOM` and source is not yet visible, set `visibility: visible` + `opacity: 0`.
3. On regime change, bump opacity to design value. Do not re-toggle visibility.
4. Within the regime-change handler, reveal in-memory layers first; tile-backed layers second.
5. Reduce the inertia timeout for hiding the obsolete regime (5000 ms → 2000 ms in Sprint 9) so a slow tile fetch doesn't visually clobber the active regime longer than necessary.

## Open Question

Should the prefetch trigger be **zoom-rate-aware**? Currently any `z >= PREFETCH_ZOOM` triggers a mount, including rapid pans through the prefetch range. On a slow network, this can fire repeated tile fetches as the user scrubs zoom. Mitigation considered: debounce on rate-of-change, only prefetch when zoom is monotonically increasing for >300 ms. Not implemented in Sprint 9; flag for next editor.

## Research Trail

### Done
1. Bottleneck audit (Sprint 9 Audit 1, 2026-05-08) identified PMTiles fetched on-demand at threshold cross as the dominant latency source.
2. MapLibre layer lifecycle reviewed: tiles begin fetching when a layer with that source becomes visible AND the source's tile range covers the viewport.
3. Three-iteration approach validated against the operator's "iterate not rewrite" constraint — no architectural changes; additive only.
4. Live measurement on gis.woodfinegroup.com confirmed the pre-warm eliminates the cold-start stall in regional zoom (z=8–10).

### Suggested for next editor
1. Investigate zoom-rate-aware prefetch (open question above).

## Implementation Reference

| File | Line range | Purpose |
|---|---|---|
| `pointsav-monorepo/app-orchestration-gis/www/index.html` | 362–367 | Constants: THRESHOLD + PREFETCH_ZOOM |
| `pointsav-monorepo/app-orchestration-gis/www/index.html` | 638–668 | `setRetailLevel()` reveal sequence |
| `pointsav-monorepo/app-orchestration-gis/www/index.html` | 1218–1244 | zoomend prefetch hook |

## See Also

- DESIGN-RESEARCH-tier-naming-accessibility.draft.md (Sprint 9 tier rebrand)
- DESIGN-RESEARCH-bento-merged-zones-disclosure.draft.md (Sprint 9 dedup transparency)
