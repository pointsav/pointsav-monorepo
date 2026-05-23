# GIS UX Redesign — Consolidated Research
**Drafted:** 2026-05-21 from five Opus deep-think sessions.
**Status:** Research complete. Partial implementation started (Alberta sim only).
**Companion plans:** `RING-HIERARCHY-DESIGN-2026-05-20.md`, `VISUAL-DESIGN-SYSTEM-2026-05-20.md`, `DBSCAN-TRIANGULATION-REDESIGN-2026-05-20.md`

---

## 1. Key finding: basemap is already correct

The map already uses OpenFreeMap Positron (light, key-free). The "dark basemap" premise in earlier sessions is wrong — the basemap is correct. What was wrong: dark-era overlay colors (`#0A3070`, `#1E40AF`, `#3B6FB5`, `#64748B`) sitting on the light base, tuned for a canvas they no longer have.

**Action:** keep Positron. Optionally set `#map { background: #F7F9FA }` to match Woodfine canvas pre-tile.

---

## 2. Woodfine color integration — WCAG findings

Contrast ratios vs `woodfine-canvas #F7F9FA`:

| Tier | Token | Hex | Contrast | WCAG 3:1 non-text |
|---|---|---|---|---|
| T1 Regional | woodfine-blue | `#164679` | 8.0:1 | Pass |
| T2 District | woodfine-green | `#54924E` | 3.0:1 | Pass (borderline — stroke needed) |
| T3 Local | institutional gold | `#EAB308` | **1.7:1** | **FAIL** |
| T4 Fringe | institutional maroon | `#991B1B` | 6.4:1 | Pass |

**Critical:** `#EAB308` gold **fails WCAG 3:1** as a bare fill on the light canvas. Two-part remedy:
1. Use `#B58A00` (ochre, 3.2:1) for badge backgrounds and fill polygons where gold is needed.
2. For map dots: keep bright `#EAB308` fill but add a mandatory 1.5px `#111827` slate stroke (anchor already has 2.5px stroke — extend to all member-dot categories).

**T3 Local production change needed:** TIER_COLORS currently has `#EAB308` for T3 fill. For map circles/fills, use `#B58A00`. Badge in panel uses dark text (`#111827`) for gold — acceptable since badge is UI not map symbol.

### Member dot palette migration (dark-era → Woodfine)

| Category | Dark-era | Woodfine | Hex |
|---|---|---|---|
| anchor | `#FACC15` / `#EAB308` | `#EAB308` + **`#111827` 2.5px stroke** | existing architecture |
| hardware | `#D97706` | woodfine-amber | `#B54708` |
| warehouse | `#0891B2` / `#22D3EE` | woodfine-cyan | `#0E7490` |
| lifestyle | `#E879F9` | deep magenta | `#A21CAF` |

**Universal rule:** every `circle` symbol gets `circle-stroke-width: 1.5`, `circle-stroke-color: #FFFFFF`. White stroke separates dot from basemap regardless of fill contrast.

### Co-location ring opacity on light base
Dark-era rings were tuned for dark canvas and look washed-out on light. Raise `line-opacity` to `0.85–0.90` for co-location rings on the light base.

---

## 3. Fonts: Woodfine brand not yet loaded

The map declares `"Inter"` (line ~27) which is never fetched — silently falls back to system sans. Neither Nunito Sans nor Barlow Condensed are currently loaded. **Self-host both (OFL licensed):**

```css
@font-face {
  font-family: "Nunito Sans"; font-weight: 400 800; font-display: swap;
  src: url("lib/fonts/nunito-sans-var.woff2") format("woff2-variations");
}
@font-face {
  font-family: "Barlow Condensed"; font-weight: 600; font-display: swap;
  src: url("lib/fonts/barlow-condensed-600.woff2") format("woff2");
}
```

Rationale: self-hosting avoids Google Fonts CDN dependency (GDPR/privacy), removes a render-blocking cross-origin round trip, and actually delivers the brand typeface for the first time.

---

## 4. Interaction model — state machine

**Source: mapuipatterns.com "Feature selection" + Placer.ai convention**

### States

```
mapState = {
  selection: 'S0' | 'S2' | 'S4',   // S0=none, S2=cluster, S4=store
  data: 'OFF' | 'S3',               // S3=catchment (sub-state of S2)
  zoomBand: 'overview' | 'metro' | 'street'
}
```

| State | Panel | Map |
|---|---|---|
| **S0** overview | "Select a node" hint | T1/T2/T3 nodes visible, T4 hidden |
| **S2** cluster selected | Full BentoBox | selected cluster ring + proximity ring |
| **S3** catchment (sub-state of S2) | BentoBox + active catchment sublayer | 35km + 150km rings + heatmap + vignette |
| **S4** store selected | BentoBox unchanged + bottom strip | cluster ring stays + dot selected |

S3 is always a sub-state of S2 — catchment cannot outlive cluster selection. This is the structural fix for the "BentoBox confused" problem: one panel owner at a time.

### Key transitions
- **T4:** click cluster → S2, `flyToCluster()` (zoom-in-only, edge-test, panel padding)
- **T5:** second click on selected cluster → deselect → S0
- **T7:** click map background → deselect → S0
- **T8:** Show trade area toggle → S3
- **T10:** click store dot inside ring → S4 (BentoBox unchanged; bottom strip fills)
- **T13:** Escape / Back button → S0 full reset

### No-popup rule (implemented)
The only remaining `maplibregl.Popup` (sim click handler, lines ~2124–2127) has been removed. Sim clicks now route to `#inspector`. `maplibregl.Popup` should appear zero times in the file.

### `flyToCluster()` — zoom-in-only
```js
function flyToCluster(coords) {
  const z = map.getZoom();
  const targetZoom = Math.max(z, RETAIL_ZOOM_THRESHOLD);
  const pt = map.project(coords);
  const { width, height } = map.getCanvas();
  const margin = 0.20;
  const offEdge = pt.x < width*margin  || pt.x > width*(1-margin)
               || pt.y < height*margin || pt.y > height*(1-margin);
  if (offEdge || targetZoom > z) {
    map.flyTo({ center: coords, zoom: targetZoom, duration: 800, essential: true,
                padding: { left: PANEL_WIDTH } });
  }
}
```
Key rules: never zoom out; `padding.left` keeps cluster in the visible (un-panelled) map slice; don't move if already in frame.

---

## 5. Zoom architecture

| Band | Zoom | Visible tiers |
|---|---|---|
| overview | z < 7 | T1/T2/T3 only |
| metro | 7 ≤ z < 10 | T1/T2/T3 |
| street | z ≥ 10 | All tiers inc. T4 |

**T4 split — `nodes-singleton` layer** (defer to production rebuild after sign-off):
```js
map.addLayer({
  id: 'nodes-singleton', type: 'circle', source: 'data',
  minzoom: 10,
  filter: ['==', ['get', 'tier'], 4],
  paint: {
    'circle-radius': ['interpolate',['linear'],['zoom'], 10, 3, 14, 6],
    'circle-color': '#94A3B8',
    'circle-stroke-width': 1, 'circle-stroke-color': '#fff',
    'circle-opacity': 0.6,
  },
});
```
T4 singletons are not clickable into a BentoBox — they route to `showElementDetail()`.

**Add `minzoom:9` to `all-locations`** — individual store dots should not paint at overview zoom (currently relies on a prefetch hack).

---

## 6. BentoBox redesign — implemented items

**Implemented this session:**
- Badge size: `42px → 13px` inline chip. Layout changed from centered billboard to `display:flex; align-items:center; gap:10px;` with badge chip + composition text side-by-side.
- Ranking: replaced "National Rank #N" with "— in Top 400 NA/EU" and "— in Top 400 [Country]". Top 400 data does not exist yet; placeholders shown until dataset is built.
- TIER_COLORS: migrated to Woodfine tokens (`#164679`/`#54924E`/`#EAB308`/`#991B1B`).
- Popup removed: sim click handler now populates `#inspector` directly.

**Still pending (production sprint, after Alberta sign-off):**
- Two-state BentoBox (S2 cluster vs S4 store) — State A / State B replace-in-place
- `data-label` "National Rank" → removed (done)
- Bottom strip for store selection (S4)
- BentoBox data-horizon copy (per `RING-HIERARCHY-DESIGN.md` §4)
- Tabular numerics + type scale (V10 in TODO.md P17-V)

---

## 7. Mobile bottom sheet — Google Maps pattern

**Pattern:** non-modal bottom sheet, three snap points. No dark scrim. Map stays live behind the sheet.

| Snap | Height | Content |
|---|---|---|
| Peek | `min(180px, 22vh)` | Cluster name + tier chip + 1 summary line |
| Half | `52vh` | Name + metrics row + anchor chains list + action buttons |
| Full | `94vh` (6vh sliver retained) | Everything + full chain table + catchment detail + ranking |

**Key implementation detail:** on snap change, `map.easeTo({ padding: { bottom: <visible sheet height> }, duration: 220 })` so the tapped cluster recentres into the visible slice above the sheet.

**Breakpoint:** `768px` viewport width → switch side panel ↔ bottom sheet. One shared content model, two containers.

**Desktop:** keep existing 360px right-side panel (→ widen to 380–400px in production sprint). Internal sections collapse/expand.

**Do not copy from Google Maps:** hero imagery, star ratings, consumer action buttons ("Directions/Save/Share"), five-item bottom nav bar, fully modal full-state. Our tool is institutional — lead with figures and tables, keep a 6vh map sliver always visible, use professional verbs ("Catchment view", "Compare", "Export").

**Persistent Close (X)** required in addition to swipe gesture — NN/g flags gesture-only dismissal as accessibility failure.

### Measurements
| Property | Value |
|---|---|
| Sheet corner radius | 16px |
| Sheet shadow | `0 -2px 16px rgba(22,70,121,0.12)` |
| Drag handle bar | 32×4px, radius 2px, `#C4CED6` |
| Drag handle touch target | 48px tall × full width |
| Snap animation | 220ms ease-out (transform + easeTo padding) |
| Touch targets | ≥44×44px |
| Cluster name | Nunito Sans 18px/600 `#164679` |
| Table cell | Nunito Sans 13px/400 `#2A3744` |

---

## 8. Award-winning GIS UX — pattern inventory

From prior session research (mapuipatterns.com + Google Maps + Placer.ai + CBRE):

| Pattern | Description | Status |
|---|---|---|
| Info Panel | Click cluster → side panel populates. No popup bubble. | Implemented |
| List-and-Details | Click cluster → panel + map fly-to. Panel owns state. | Partially implemented |
| Feature State | Selection via `setFeatureState` not filter-swapping. | Pending production |
| Progressive Disclosure | Tier visibility by zoom band. T4 `minzoom:10`. | Pending production |
| Panel Padding | `map.flyTo({padding:{left:PANEL_WIDTH}})`. Cluster stays in visible slice. | Pending production |
| Catchment as bounded context | Data renders only inside 150km ring, gated to `cluster_id`. | Implemented (filter-gated) |
| Vignette mask | Dims outside-150km to "data ends here". | Pending (RING-HIERARCHY §4) |
| Deselect on second click | Click selected cluster → back to S0. | Pending production |
| Background click dismisses | Click empty map → deselect → S0. | Pending production |
| Escape key | Full reset → S0. | Pending production |
| Back navigation | Cluster → Region → Overview breadcrumb. | Partially implemented |

---

## 9. Open decisions carried from this session

| # | Decision | Default |
|---|---|---|
| OD-U1 | `#B58A00` ochre vs `#EAB308` gold for T3 map fills | Ochre for fills; bright gold for panel badge with dark text |
| OD-U2 | Font self-hosting timing (Nunito Sans + Barlow Condensed) | Before next deployment sprint |
| OD-U3 | T4 `nodes-singleton` layer — same sprint as ring redesign? | Yes, bundle with V4 ring semantics |
| OD-U4 | Mobile bottom sheet — same sprint as BentoBox redesign? | Separate sprint after desktop stabilises |
| OD-U5 | Top 400 dataset construction — when? | Phase 18 (after AEC layers) |

---

## 10. Implementation checklist (ordered, post-Alberta sign-off)

1. `nodes-singleton` layer, `minzoom:10`, T4 filter — retire T4 from `nodes` layer
2. `all-locations` `minzoom:9`
3. `setFeatureState` selection model — retire `nodes-highlight` from selection path
4. `flyToCluster()` zoom-in-only with panel padding
5. Deselect on second click (T5) + background click (T7) + Escape (T13)
6. Two-state BentoBox — S4 bottom strip + `renderPanel(state)` single owner
7. `#B58A00` ochre for T3 map fills + white stroke for all member-category dots
8. Font self-hosting (Nunito Sans variable + Barlow Condensed 600)
9. Catchment ring opacity raise (0.85–0.90 on light base)
10. Mobile bottom sheet (768px breakpoint, 3 snap points, `map.easeTo({padding})`)

---

## References
- [Feature selection — Map UI Patterns](https://mapuipatterns.com/feature-selection/)
- [Google Maps rolling out full sheet redesign on Android (9to5Google)](https://9to5google.com/2025/04/24/google-maps-sheet-redesign-android/)
- [Bottom Sheets: Definition and UX Guidelines (NN/g)](https://www.nngroup.com/articles/bottom-sheet/)
- [Bottom sheets – Material Design 3 specs](https://m3.material.io/components/bottom-sheets/specs)
- [WebAIM: Contrast and Color Accessibility](https://webaim.org/articles/contrast/)
- [Dataviz: the perfect map style for dashboards — MapTiler](https://www.maptiler.com/news/2026/02/dataviz-the-perfect-map-style-for-dashboards/)
- [Popup content in side panel — ArcGIS API for JavaScript](https://developers.arcgis.com/javascript/3/jssamples/popup_sidepanel.html)
- [PaddingOptions – MapLibre GL JS](https://maplibre.org/maplibre-gl-js/docs/API/type-aliases/PaddingOptions/)
- [Location Intelligence Analytics Platform — Placer.ai](https://www.placer.ai/products/analytics)

— consolidated 2026-05-21 from five Opus sessions (award-winning GIS UX, Woodfine color system, interaction model + zoom arch, Google Maps mobile patterns, visual design system)
