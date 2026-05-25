# Ring Hierarchy & Interaction Design

**Owner:** project-gis Totebox session (research); operator (gate decisions).
**Drafted:** 2026-05-20 from three parallel Opus deep-think sessions.
**Status:** Research complete. No implementation started.
**Relationship to prior plans:**
- `DBSCAN-TRIANGULATION-REDESIGN-2026-05-20.md` — upstream; this plan assumes the
  DBSCAN centroid model and the T1/T2/T3 tier definitions from that document.
- `tier-scoring-overhaul-2026-05-16.md` — BentoBox layouts and tier names (G1) carry
  forward; ring interaction supersedes the old 1km/3km toggle model.

---

## 1. The three-ring hierarchy

All three rings share the same center: the **DBSCAN cluster centroid**.

| Ring | Radius | When visible | Meaning |
|---|---|---|---|
| **Co-location ring** | 1km (T1 Regional) or 3km (T2/T3) | On cluster selection, z≥9 | "These stores all fall within this distance of each other" |
| **Primary catchment** | 35km | Data layer ON only | Local trade area — the primary draw basin |
| **Regional catchment** | 150km | Data layer ON only | Regional draw; hard data horizon — nothing rendered outside |

The ring size of the co-location ring **encodes tier** without text: T1 = tight 1km ring (solid stroke), T2/T3 = loose 3km ring (dashed stroke). This is why the 1km/3km UI toggle can be retired.

---

## 2. Co-location ring — visual spec

| Property | T1 Regional (1km) | T2 District / T3 Local (3km) |
|---|---|---|
| `line-color` | `#0A3070` deep navy | `#1E40AF` indigo / `#3B6FB5` |
| `line-width` | `interpolate zoom 10→2.0, 14→3.0` | `interpolate zoom 10→1.5, 14→2.25` |
| `line-dasharray` | **solid** | **`[3, 2]` dashed** |
| `line-opacity` | 0.90 | 0.70 |
| `fill-opacity` | 0.04 (faint tint, click target) | 0.04 |
| `minzoom` | 9 | 7 |
| `maxzoom` | none | none |

**Encoding rationale:** solid = tight/hard containment. Dashed = loose/corridor association. Color alone fails for colour-blind users at small sizes; the dash pattern carries the tight/loose signal pre-attentively.

**Relationship to member-store hull:** the hull (visual footprint of where stores actually sit) renders inside the co-location ring. One strong stroke (ring) + one soft fill (hull). Never two strong outlines nested — that is the confusing case. Hull: tier-navy fill at 0.10 opacity, hairline or no stroke.

---

## 3. Primary catchment ring (35km)

| Property | Value |
|---|---|
| `fill-color` | `#f97316` warm orange |
| `fill-opacity` | 0.10 |
| `line-color` | `#c2410c` burnt orange |
| `line-width` | 1.5 (constant) |
| `line-dasharray` | `[6, 4]` (long dash — distinct period from co-location's `[3,2]`) |
| `line-opacity` | 0.75 |
| `minzoom` | 4 |
| `maxzoom` | add fade-to-0 by z13 (`interpolate zoom z12→base, z13→0`) |
| MITMA-measured variant | **solid** line (already implemented) |

The 35km ring **gets a fill** — it is the canvas for census/spend heatmap data. The orange fill (faint) visually answers "this is the data zone." The heatmap renders on top.

---

## 4. Regional catchment ring (150km) — the data horizon

Critical design decision: the 150km ring is simultaneously (a) a visual circle, (b) a hard data boundary, and (c) a signal that data ends here. It must read as a **limit/horizon**, not as a soft gradient.

| Property | Value |
|---|---|
| `line-blur` | **0** — remove the existing blur. A crisp line. |
| `line-dasharray` | `[6, 4]` — limit-of-data dash (survey/boundary convention) |
| `line-color` | `#64748B` neutral slate (desaturated from current warm orange) |
| `line-width` | `interpolate zoom 3→2.5, 8→3.5` |
| `line-opacity` | 0.85 |
| `fill-opacity` | none for the disc — see vignette mask below |
| `minzoom` | 3 |
| `maxzoom` | none (must stay visible even when bigger than viewport — explains the data edge) |

**Palette rule (hard):** blue family = store containment (co-location rings). Orange = consumer reach (35km). Slate = data limit (150km). No crossover between families.

### Vignette mask (data-horizon-mask layer)

A "donut" fill that covers everything *outside* the 150km circle:
- Geometry: world bounding polygon with the 150km circle as a hole, built client-side from `makeCircleGeoJSON(centroid, 150)`
- `fill-color`: `#0b1220` (near-black slate, matches basemap dark furniture)
- `fill-opacity`: 0.45
- Rendered above heatmaps, below cluster rings and pins
- Visible only when data layer is ON

Effect: world outside 150km dims to a vignette; the catchment becomes a "lit stage." The 150km ring's crisp dashed edge is the transition from lit to dimmed. The user cannot mistake where data ends.

### Arc label

A single `symbol` label anchored on the north point of the 150km arc:
- Text: `Data horizon — 150 km`
- `text-size`: 10, slate color, white halo
- One instance, not repeated around the circle

### BentoBox copy

Under population stats:
> *"Population and spend are measured within 150 km of the cluster centroid. The outer ring marks the data horizon — figures do not include demand beyond it."*

---

## 5. State machine

| State | Co-location ring | 35km | 150km | Heatmap | Vignette |
|---|---|---|---|---|---|
| S0 — Overview, no selection | hidden | hidden | hidden | hidden | hidden |
| S1 — Retail level, no selection | all visible (merged at 3km) | hidden | hidden | hidden | hidden |
| S2 — Cluster selected, data OFF | selected cluster only | hidden | hidden | hidden | hidden |
| S3 — Cluster selected, data ON | stays visible | appears | appears | appears | appears |
| S4 — Switch cluster (data OFF) | snaps to new cluster | hidden | hidden | hidden | hidden |
| S5 — Switch cluster (data ON) | snaps to new | re-centers | re-centers | reloads | re-centers |
| S6 — Zoom out to overview, data OFF | disappears (hard cut) | — | — | — | — |
| S6 — Zoom out, data ON | stays visible (selection retained) | stays | stays | stays | stays |

**Key rules:**
- Clicking a cluster never automatically shows 35/150km rings — those require explicit data-layer opt-in.
- Co-location ring is the persistent anchor through the whole drill-in session.
- One selection at a time — selecting cluster B clears A's rings before drawing B's.
- No fade animations on ring appear/disappear. Hard show/hide reads as "state changed."

---

## 6. Retiring the 1km/3km button pair

**The button pair is removed as a radius selector.** Ring size is derived per cluster from tier:
```
tier === 1  →  1km ring (solid)
tier > 1   →  3km ring (dashed)
```

The global `currentRadius` variable becomes a pure function: `ringKmForTier(tier)`.

**The vacated slot is repurposed as the persistent "Trade area" control** — a pill in the bottom-left map control stack, mirroring the BentoBox toggle. Disabled/greyed when no cluster is selected. Tooltip: *"Select a cluster to view its trade area."*

**Optional (follow-up, not blocking):** a "Zoom to: [co-location] [35km] [150km]" link row inside the BentoBox catchment block. Uses existing `catchmentBounds(lon, lat, km)` + `fitBounds`. ~15 lines. Not a new control — just links inside the bento.

**Retire `layer3-radius.pmtiles`** and its `radius-fill`/`radius-line` layers — they are dead code. The live ring path is the dynamic `proximity-circle-src` GeoJSON. Flag for cleanup-log.

---

## 7. The "data layer" toggle — redesign

**Button label:** `Show trade area` (active: `Hide trade area`).

Rationale: "Show Data" is vague. "Show Catchment" is internal jargon. "Trade area" is standard retail-site-intelligence vocabulary and implies both the rings and the data inside them.

**Toggle, not multi-state.** 35km and 150km are one nested object — atomic toggle. Splitting them would allow viewing population with no data horizon, which is an analytical misread.

**Layers toggled ON by `toggleDataLayer()`:**
- `catchment-inner-line`, `catchment-inner-fill` (35km)
- `catchment-outer-line`, `catchment-outer-fill` (150km, restyled per §4)
- active heatmap sublayer
- `data-horizon-mask` vignette
- `catchment-sublayers` BentoBox panel
- triggers `loadCatchmentCentroids(selectedClusterId)`

**Layers toggled OFF:**
- all of the above → `visibility: none`
- `catchment-centroids-src` → cleared to empty FeatureCollection
- `proximity-line`/`proximity-fill` visibility restored

The co-location ring is **never** touched by this toggle.

---

## 8. Data clipping at 150km

**Key finding from the clipping research:** the problem is narrower than expected.

| Layer | Current state | Action needed |
|---|---|---|
| `catchment-heatmap-pop` | Already per-cluster pre-clipped (loaded from `catchment-cells/<id>.json`) | None |
| `catchment-heatmap-spend` | Already per-cluster pre-clipped | None |
| `catchment-heatmap-work` | **Global PMTiles (164 MB) — renders across entire country** | Extend per-cluster pre-clip |
| `catchment-heatmap-home-measured` | **Global PMTiles — unbounded** | Extend per-cluster pre-clip |
| `catchment-inner/outer-fill` | Filtered by `cluster_id` against PMTiles — only selected cluster renders | None |

**MapLibre `clip` layer type does NOT work here** — it only masks `model`/`symbol` content, not `fill`/`line`/`circle`/`heatmap`. Rejected.

**`setFilter()` by distance does NOT work** — MapLibre expression language has no geodesic distance function. Rejected.

**Recommended: Option C — extend per-cluster pre-clipping to the two mobility layers.**

- Build pipeline: `build-mobility-tiles.py` emits `www/data/mobility-cells/<cluster_id>.json` (H3 centroid points with `visits_norm`, scoped to 150km radius). Mirrors the existing `catchment-cells/` pattern.
- `index.html`: add `mobility-cells-src` GeoJSON source; `loadMobilityCells(clusterId)` alongside `loadCatchmentCentroids()`; repoint the two mobility heatmap layers from global PMTiles to the new GeoJSON source.
- Performance win: sub-MB per-cluster fetch replaces 164 MB global PMTiles range-request.

**Interim (if build-pipeline change is out of scope):** add the vignette mask (§4) as a visual focus — it dims the out-of-bounds area even though the heatmap data technically still renders there. Must be disclosed in the data modal: *"The visual boundary represents the 150 km data horizon; mobility data outside this zone is suppressed for clarity."*

**The 150km boundary ring should be a separate visible layer** (crisp slate dashed stroke per §4). Under pre-clipping, the heatmap simply ends and a soft heatmap edge is not a legible boundary. The explicit ring + vignette together make the horizon unambiguous.

---

## 9. Ring behavior by zoom level

| Ring | minzoom | maxzoom | Notes |
|---|---|---|---|
| Co-location 1km (T1) | 9 | none | Below z9 → overview state, hidden |
| Co-location 3km (T2/T3) | 7 | none | Lowered to z7 so visible at data-layer flyTo z8.5 |
| 35km primary | 4 | fade to 0 by z13 | Off-screen at street level; fade before MapLibre draws single arc across viewport |
| 150km regional | 3 | none | Must stay visible even when bigger than viewport — explains data edge |

**"150km fills the viewport at z7 over Alberta — correct?** Yes, for a single selected cluster. The 150km ring is a per-selection object, never drawn for all clusters simultaneously. A single 150km circle at z7 is the honest answer to "how far does this centre draw."

**At z14 (street level):** only the co-location ring and hull survive on-screen. 35km and 150km are off-screen; fade their `line-opacity` to 0 before their `maxzoom` rather than letting MapLibre draw a stray arc fragment across the viewport.

---

## 10. Alberta simulation test path (rings)

**Fully client-side, zero Python, zero PMTiles rebuild.**

The existing `sim-ab-dbscan.geojson` has DBSCAN centroid coordinates and `tier` per cluster feature. `makeCircleGeoJSON(center, radiusKm)` already exists in `index.html`.

Approach: add `sim-ring-src` GeoJSON source + 3 ring line layers (co-loc / 35km / 150km). Wire into the existing sim click handler to call `showSimRings(coords, tier)`. On sim toggle OFF: clear the source.

Estimated new JS in `index.html`: **35–45 lines**. No Python, no pipeline changes.

This lets the Alberta sim validate the full three-ring hierarchy — tier-derived 1/3km sizing, 35km, 150km — cluster-by-cluster before any production `catchment.pmtiles` work begins.

---

## 11. Open decisions (not yet locked)

| # | Decision | Options | Note |
|---|---|---|---|
| OD-R1 | Retire `layer3-radius.pmtiles` timing | Same commit as ring redesign vs. separate cleanup | Risk: live regression if retired before `proximity-circle-src` is confirmed stable |
| OD-R2 | 35km solid vs dashed | Solid = "real trade area" / dashed = "estimated" | The MITMA-measured variant already uses solid; generalise |
| OD-R3 | Data horizon label copy | `Data horizon — 150 km` vs `Regional boundary` vs nothing | Operator preference |
| OD-R4 | Mobility per-cluster build timing | Same sprint as ring redesign vs. overnight build scheduled separately | Tier-B dependency: `build-mobility-tiles.py` is a significant pipeline addition |
| OD-R5 | Vignette opacity | 0.45 was proposed — operator may find it too dark or too light | Validate on Alberta sim first |

---

## 12. Phased implementation order (after Alberta sign-off)

1. **Alberta sim** — add `sim-ring-src` + 3 ring layers + `showSimRings()` in `index.html`. Research/validation only; do not merge to main until operator approves visual.
2. **Ring semantics** — make `updateProximityRings()` derive radius per cluster; delete `currentRadius` global + `.radius-selector` + `setRadius()` radius-button code.
3. **Retire `layer3-radius.pmtiles`** — remove `radius-fill`/`radius-line` layers from `index.html`; note in cleanup-log.
4. **150km ring repaint** — remove `line-blur`; add `line-dasharray [6,4]`; change color to `#64748B`; add `minzoom` to inner/outer catchment layers.
5. **Vignette mask** — `data-horizon-mask` layer + `makeMaskGeoJSON()` helper.
6. **Data horizon label** — single arc symbol at north point of 150km ring.
7. **Button / toggle rename** — `catchmentActive` → `dataLayerActive`; label `Show trade area`; persistent map control pill.
8. **BentoBox copy** — add data-horizon explanation line under population stats.
9. **Mobility clipping** (overnight build) — `build-mobility-tiles.py` per-cluster files; repoint heatmap layers to GeoJSON source; retire global PMTiles sources.

---

## 13. References

**Code (verified 2026-05-20):**
- `www/index.html` — `makeCircleGeoJSON` ~L1345, `updateProximityRings` ~L773, `setRadius` ~L775, `.radius-selector` HTML ~L291-293, `toggleCatchmentMaster`/`hideCatchment` ~L502-544, `loadCatchmentCentroids` ~L614-630, `catchment-inner/outer-*` layers ~L1858-1916, `catchment-heatmap-*` layers ~L1860-2010, `radius-fill`/`radius-line` ~L1725-1745 (dead code)
- `build-mobility-tiles.py` — source for new per-cluster mobility cell build
- `www/data/catchment-cells/` — existing per-cluster heatmap pattern to mirror for mobility

**Prior session plans:**
- `.agent/plans/DBSCAN-TRIANGULATION-REDESIGN-2026-05-20.md` — upstream cluster model
- `.agent/plans/tier-scoring-overhaul-2026-05-16.md` — tier names (G1), BentoBox layouts

**Artifact registry updates needed:** when any phase above is implemented, update the `C-table` data artifact entries and relevant TOPIC/DESIGN-RESEARCH drafts (A3, A4, B7, B8 in `.agent/rules/artifact-registry.md`).

— drafted 2026-05-20 from three Opus deep-think sessions (visual spec, clipping, interaction model)
