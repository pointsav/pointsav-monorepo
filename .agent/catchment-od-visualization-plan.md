# Plan: Catchment Visualization & O-D Study — Sprint 15+

Synthesized from Opus agent research sessions (2026-05-15).
Covers: data architecture, UI/UX design, Spain MITMA implementation, provider demo strategy.

---

## 1. The Vision

When a user clicks a co-location cluster and zooms in, a **Catchment** toggle appears in the
cluster detail panel. When ON:

- Two soft-shaded zones appear behind the cluster as a background fill:
  - **Inner zone (≤35 km):** warmer, more opaque — primary visitor draw area
  - **Outer zone (35–150 km):** cooler, very subtle — extended reach
- These represent where people who visit the retailers in this cluster **live (HOME)** or **work (WORK)**
- Sub-layers within the catchment show **Population**, **Spend**, and **Trade Area rank**
- The same methodology runs across all 6,815 clusters in 13 countries → **apples-to-apples comparison**

---

## 2. Data Architecture

### Two-field O-D model (what we build now with free data)

| Field | Method | Source | Countries |
|---|---|---|---|
| `home_pop_modeled_huff` | Huff gravity × residential population | WorldPop 2026 (already in pipeline) | All 13 |
| `work_pop_modeled_commute` | Census commuting matrix → H3 res-7 | LEHD LODES (US), ONS WU03 (UK), INSEE Mobilités Pro (FR), Destatis Pendlerstatistik (DE), StatCan Place of Work, INE (ES), CBS (NL), Nordic national matrices | 12/13 (Mexico weakest) |

Future fields (when licensed panel data arrives):
- `home_visitors_measured` — device-level, from SafeGraph/Veraset/Placer
- `work_visitors_measured` — device-level

**Rule:** Never mix `_modeled_` and `_measured_` in the same visual field.

### Spain MITMA (Priority #1 — free, real MNO data)

Spain's Ministry of Transport (MITMA) published anonymized aggregated OD matrices derived from
Orange and Vodafone network data. Free download, country-wide, ~2.5 km grid resolution.

- **Source:** https://www.mitma.gob.es/ministerio/proyectos-singulares/estudio-de-movilidad-con-big-data
- **Data:** Daily OD matrices (origin zone → destination zone × hour × purpose), 2020-ongoing
- **Format:** CSV/ZIP by province
- **Granularity:** ~3,500 transport analysis zones (TAZ); convert to H3 res-7 via centroid join
- **License:** Creative Commons CC BY 4.0 — commercially usable
- **Value:** The only country in our 13 where we have free MNO-backed OD signal
- **Use case:** For Spanish clusters, compute `home_visitors_mitma` and `work_visitors_mitma`
  fields that are measured, not modeled → demonstrate dashed→solid border transition in one country

**Ingest script:** `ingest-mitma.py` (see §5)
**Output:** `service-fs/service-mobility/mitma-od-es.jsonl` — one record per MITMA zone pair

### Catchment centroid files (required for heatmap layer)

Pre-compute one GeoJSON file per cluster: H3 res-7 cell centroids (points, not polygons) within
150 km, with `pop`, `spend_grocery`, `spend_hardware`, `spend_wholesale` properties.

- **Location:** `gateway-orchestration-gis-1/www/data/catchment-cells/<cluster_id>.json`
- **Script:** `build-catchment-centroids.py` (new)
- **Rationale:** Lazy-loaded on cluster click; ~10–15k points per cluster, <500 KB gzipped;
  cached client-side for last 5 clicks. Drives the heatmap layer without ever rendering hex polygons.

---

## 3. UI/UX Design

### No hexagons on the map

H3 is the computation substrate only. The visual output uses:

1. **Envelope:** existing `layer3-catchment.pmtiles` circle rings, styled with soft fills +
   blurred outer line (`line-blur` interpolated by zoom). Looks like a natural catchment, not a ring.
2. **Interior data:** MapLibre native **heatmap layer** driven by H3 cell centroids (points).
   Heatmap's Gaussian blur shader makes the hex grid completely disappear.

### Color palette

| Zone | HOME color | WORK color (planned) |
|---|---|---|
| Inner 35 km | `#f97316` orange, 18% opacity | `#0ea5e9` cyan, 18% opacity |
| Outer 150 km | `#fdba74` peach, 8% opacity | `#7dd3fc` sky, 8% opacity |
| Boundary (estimated) | dashed line, `#c2410c`, 1.5px | dashed line, `#0369a1` |
| Boundary (measured) | **solid** line — promoted when real data lands | solid |

Warm = HOME (evening, residential). Cool = WORK (daylight, office). Both stay clear of the
existing `#0a3070` navy cluster markers.

### Toggle interaction

```
[Cluster panel — when cluster is selected]

  Catchment  [◑ ON]               ← master toggle (replaces current toggleCatchment())
  ┌──────────────────────────────┐
  │  ◉ Population                │  ← radio group: mutually exclusive
  │  ○ Spend                     │    (overlapping heats are unreadable)
  │  ○ Trade Area rank           │
  └──────────────────────────────┘
  Reach: [ HOME  |  WORK (planned) ]  ← segmented control; WORK disabled until data
  Vintage: Estimated · Gravity v1  ⓘ  ← click → Data methodology dialog
```

**Master toggle OFF:** envelope + sub-layers hide; sub-layer selection remembered.
**Master toggle ON, no sub-layer:** envelope only (two-zone fill).
**Master toggle ON + sub-layer:** envelope + heatmap under the envelope outline.

### Layer stack order (bottom to top)

```
base map (OpenFreeMap positron)
catchment-outer-fill          ← NEW: cool/warm 8% fill, filter by cluster_id
catchment-inner-fill          ← NEW: warm/cool 18% fill
catchment-heatmap             ← NEW: pop / spend / rank; lazy-loaded centroids
catchment-outer-line-blur     ← NEW: line with line-blur for soft edge
catchment-inner-line          ← NEW: dashed (estimated) or solid (measured)
radius-fill / radius-line     existing 1km/3km proximity rings
all-locations                 existing individual store dots
nodes-halo / nodes            existing cluster bubbles
nodes-highlight               existing chain-match amber ring
proximity-fill / proximity-line existing tier ring
individual-points             existing on-click retailer dots
nodes-label                   existing text labels
```

### Estimated-data disclosure

- **Dashed outer boundary** = gravity model estimate. Promotes to solid when measured data arrives.
- **"Vintage" row** in cluster panel: `Estimated · Gravity model v1 (Huff proxy)` + ⓘ link
- **Data methodology dialog** (already planned as artifact A4): add "What this is not" paragraph
- **BCSC language throughout:** "estimated HOME catchment", "WORK catchment planned", "projected population"

---

## 4. Heatmap implementation (MapLibre)

```javascript
// Population sub-layer — driven by lazy-loaded centroid GeoJSON
map.addLayer({
  id: 'catchment-heatmap-pop',
  type: 'heatmap',
  source: 'catchment-centroids',  // GeoJSON points, loaded per cluster on click
  layout: { visibility: 'none' },
  paint: {
    'heatmap-weight': [
      'interpolate', ['linear'], ['get', 'pop'],
      0, 0,  5000, 0.4,  50000, 1.0,
    ],
    'heatmap-intensity': [
      'interpolate', ['linear'], ['zoom'],
      6, 0.6,  10, 1.2,  14, 2.0,
    ],
    'heatmap-color': [
      'interpolate', ['linear'], ['heatmap-density'],
      0,   'rgba(253,186,116,0.0)',
      0.2, 'rgba(253,186,116,0.35)',
      0.5, 'rgba(249,115,22,0.55)',
      1.0, 'rgba(194,65,12,0.75)',
    ],
    'heatmap-radius': [
      'interpolate', ['linear'], ['zoom'],
      6, 14,  10, 32,  14, 60,
    ],
    'heatmap-opacity': [
      'interpolate', ['linear'], ['zoom'],
      6, 0.7,  14, 0.8,  // override default fade-out at high zoom
    ],
  },
});
```

Spend sub-layer: same structure, swap `'get', 'pop'` → `'get', 'spend_total'`, use green ramp.

**Clipping to catchment boundary:** MapLibre does not support layer-level clip masks. Strategy:
- Heatmap intensity naturally falls off outside dense areas
- The envelope fill at the outer boundary does the visual segmentation
- For sprint-1: acceptable without PostGIS clipping
- For sprint-2+: pre-filter centroids server-side to cluster-specific catchment

---

## 5. Sprint Plan

### Sprint 15 (now — UI wiring, ~4 days)

| # | Task | File | Notes |
|---|---|---|---|
| 1 | Add `catchment-outer-fill` + `catchment-inner-fill` layers from `layer3-catchment.pmtiles` | `index.html` | Warm/cool fills; `filter: cluster_id == selected` |
| 2 | Add `catchment-outer-line` (`line-blur`) + `catchment-inner-line` (`line-dasharray`) | `index.html` | Dashed = estimated |
| 3 | Add Catchment master toggle + Reach segmented control + radio in cluster panel | `index.html` | Replaces `toggleCatchment()` |
| 4 | Build `build-catchment-centroids.py` — per-cluster centroid JSON files | new script | Writes to `www/data/catchment-cells/` |
| 5 | Add `catchment-heatmap-pop` + `catchment-heatmap-spend` layers + lazy-load on click | `index.html` | Per-cluster JSON fetch + cache |
| 6 | "Estimated · Gravity v1" vintage row + link to data dialog | `index.html` | Extend `showClusterDetail()` |

### Sprint 16 (Spain MITMA — real measured data for ES clusters, ~3 days)

| # | Task | File | Notes |
|---|---|---|---|
| 7 | `ingest-mitma.py` — download + parse MITMA CSV zones, map to H3 res-7, output JSONL | new script | Destination zones matching ES cluster ringfences |
| 8 | `synthesize-od-study.py` — extend to consume MITMA data for ES clusters | existing script | Add `home_visitors_mitma` field for ES; keep `home_pop_modeled_huff` as fallback |
| 9 | `build-data-tiles.py` — add `is_measured: true` flag for ES cells with MITMA backing | existing script | Drives dashed→solid border in frontend |
| 10 | Frontend: dashed→solid border promotion logic based on `is_measured` | `index.html` | First cluster with solid border = proof of concept |

### Sprint 17 (commute WORK data — US LEHD LODES, ~3 days)

| # | Task |
|---|---|
| 11 | `ingest-lodes.py` — download LODES v8 OD files (NAICS 44-45 retail), map to H3 res-7 |
| 12 | Extend `synthesize-od-study.py` to produce `work_pop_modeled_commute` for US clusters |
| 13 | Enable WORK tab in Reach segmented control for US clusters |

### Sprint 18 (Compare mode + demo)

| # | Task |
|---|---|
| 14 | Compare mode: pin two cluster catchments simultaneously (orange + cyan envelope pair) |
| 15 | Comparable-cohort row in cluster panel: "2.4M HOME pop · Rank 12 of 142 similar clusters" |
| 16 | Demo deck: 6 clusters (DFW, Toronto, Mexico City, London, Madrid, Reykjavík) |

---

## 6. Data Provider Demo Strategy

**Frame:** "Here is the product. Here is where your data slots in. The dashed border goes solid."

**Demo flow:**
1. Zoom to Dallas–Fort Worth → catchment toggle ON → dashed-border warm rings appear
2. Toggle Population heat → smooth orange gradient, no hexagons visible
3. Toggle to Spend → green gradient
4. Show "Estimated · Gravity v1" vintage label + click to methodology dialog
5. Show a Spain cluster → border is **solid** (MITMA-backed) → "this is what your data produces everywhere"
6. Compare mode: DFW vs Houston side-by-side catchments, same panel columns

**Specific ask per provider:**

| Provider | Ask |
|---|---|
| SafeGraph / Advan | Neighborhood Patterns CBG-level visitor home origins for ~100K POIs in 6,815 clusters |
| Veraset | Movement panel aggregated to H3 res-7 home + work cells per POI |
| Placer.ai | True Trade Area 70% threshold per anchor POI — directly substitutes the catchment envelope |
| Spain MITMA | Already free — integrate sprint 16 |

**Cost expectations (enterprise, 13-country scope):** $200K–$800K/year. US-only: $30K–$100K/year.
Spain MITMA: $0 (CC BY 4.0).

---

## 7. Disclosure Framework (BCSC posture)

All forward-looking claims use: estimated / projected / planned / intended / may / target.

| Visual | Meaning | BCSC language |
|---|---|---|
| Dashed border | Gravity-model estimate | "Estimated HOME catchment (gravity model)" |
| Solid border | Measured mobility data | "Measured HOME catchment" |
| WORK tab disabled | Data not yet integrated | "WORK catchment planned" |
| Vintage row | Source + method | "Estimated · Gravity model v1 (Huff proxy)" |

DATA-MANIFEST.md and the "Data" methodology dialog (artifact A4) must include:
- "Catchment polygons are *estimated* via a Huff gravity model. They are not derived from measured visitor data."
- "Spain clusters use MITMA-aggregated mobile network data (CC BY 4.0); these catchments are *measured*."
- WorldPop 2026 vintage, BLS/StatCan/Eurostat spend proxy attribution

---

## 8. Key Sources

- Spain MITMA Big Data study: https://www.mitma.gob.es/ministerio/proyectos-singulares/estudio-de-movilidad-con-big-data
- Spain open data portal (MITMA OD downloads): https://datos.gob.es/es/catalogo?theme_id=transport
- LEHD LODES v8: https://lehd.ces.census.gov/data/
- Advan/SafeGraph via Dewey (academic only): https://www.deweydata.io/
- Veraset academic: https://www.veraset.com/solutions/universities-research
- Spectus Social Impact: https://cuebiq.com/social-impact/
- Placer.ai True Trade Area API: https://docs.placer.ai/reference/post_v1-reports-true-trade-area
- MapLibre heatmap example: https://maplibre.org/maplibre-gl-js/docs/examples/create-a-heatmap-layer/
- CARTO H3 isochrones: https://carto.com/blog/unlock-trade-area-analysis-at-scale-with-h3-isochrones/

---

*Last updated: 2026-05-15. Owner: project-gis Totebox session.*
