I'm one strand of the Phase 1 research swarm (cartography), so I'll leave the shared Phase task as-is and return my findings as my output. The research is complete.

---

# Cartography & Visual Design Recommendations — Woodfine GIS (gis.woodfinegroup.com)

Data-dense tiered point/cluster map: thousands of T1/T2/T3 colored dots + proximity rings on OpenFreeMap **positron** (light) basemap, rendered via MapLibre GL JS + PMTiles.

---

## A. Color encoding & palette (3-tier categorical)

**1. A 3-tier categorical color set is the wrong encoding for what is actually ordinal data. Switch to an ordered (sequential or quasi-sequential) ramp, not arbitrary hues.**
*Rationale:* T1/T2/T3 co-location tiers are **ranked** (T1 = strongest co-location, T3 = weakest). Categorical palettes (distinct hues with no implied order) tell the reader "these are different kinds of thing," which contradicts the data's meaning. Cartographic convention is that ranked classes get an ordered visual variable — value/lightness or saturation — so the eye reads the hierarchy without consulting the legend. ColorBrewer separates "qualitative" (unordered categories) from "sequential" (ordered low→high) precisely for this reason.
*Change:* Keep three discrete swatches but make them a **single-hue or two-hue sequential ramp** where T1 is the darkest/most saturated and T3 the lightest — e.g., a 3-class ColorBrewer sequential like Blues (`#deebf7 / #9ecae1 / #3182bd` reversed so T1=darkest) or Reds. The viewer then perceives "darker = higher tier" pre-attentively.

**2. If you keep hue-based categories for brand/recognition reasons, adopt a colorblind-safe set — the Okabe-Ito palette, not an off-the-shelf ColorBrewer qualitative ramp.**
*Rationale:* ColorBrewer's qualitative palettes are explicitly weak for colorblind safety — only "Dark2", "Paired", and "Set2" survive at 3 classes, and the project audience (institutional/investor) makes accessibility part of the credibility story. The Okabe-Ito palette is the gold standard (Nature Methods / Wong 2011; default in Wilke's *Fundamentals of Data Visualization*), engineered to stay distinguishable across protanopia, deuteranopia, and tritanopia.
*Change:* For three tiers use three Okabe-Ito colors with maximum mutual separation — Blue `#0072B2` (T1), Orange `#E69F00` (T2), Bluish-green `#009E73` (T3) — but note these are unordered, so pair them with a size cue (rec. 6) to restore the rank reading. **Best of both worlds:** combine a sequential lightness ramp *and* a size ramp so tier is double-encoded (redundant coding is the single most robust accessibility technique).

**3. Add a white/light halo (stroke) around every dot regardless of palette choice.**
*Rationale:* WCAG SC 1.4.11 (Non-text Contrast) requires ≥ 3:1 for graphical objects against adjacent colors. On positron's near-white land (`~#fafafa`) a light dot fails; against gray roads/labels even darker dots can drop below 3:1 at edges. The cartographic remedy when colors can't all clear 3:1 is "borders, spacing, or halos." A halo also separates overlapping dots from each other (see overplotting).
*Change:* `circle-stroke-width: 1–1.5`, `circle-stroke-color: #ffffff` (or a dark halo if you move to a dark basemap). This is `circle-stroke-*` in MapLibre — cheap, GPU-rendered.

**4. Verify every tier color against positron land AND against gray features, targeting ≥ 3:1.**
*Rationale:* Markers are non-text graphical objects → 3:1 minimum (not the 4.5:1 text threshold). Yellow (`#F0E442`) and other light Okabe-Ito colors fail on white; this is why halos are mandatory rather than optional.
*Change:* Run each final hex through WebAIM's contrast checker against `#fafafa`; any tier under 3:1 keeps the halo (rec. 3) and/or darkens. Document the resulting ratios in your DATA/methodology copy — it reinforces the credibility narrative.

---

## B. Overplotting of thousands of dots at low zoom

**5. At low zoom, do not render raw points — aggregate. Use server-side cluster tiles (you already have PMTiles) with a count-driven proportional symbol, and only "explode" to individual dots past a zoom threshold.**
*Rationale:* MapLibre's own large-data guide is explicit: thousands of points "quickly become cluttered and difficult to read"; the fix is clustering to reduce rendered features, and for massive datasets "convert GeoJSON to vector tiles." Raw 13k+ dots at country zoom is both a legibility failure (the map reads as noise) and a performance cost. Because your data is already in PMTiles vector tiles, the cleanest path is to pre-aggregate per zoom in the tile pipeline rather than rely on runtime GeoJSON `cluster: true` (which only works on GeoJSON sources, not vector tiles).
*Change:* In the tile build, generate aggregated cluster features for low zooms (a count attribute per grid/cluster cell), styled as a proportional circle; set the individual-dot layer's `minzoom` so it only appears once dots can be told apart (typically z7–z9 for retail networks). Set point-source `maxzoom ~12` per MapLibre guidance to avoid over-detailing.

**6. Scale the aggregate symbols by AREA (√count), classified into ~3 discrete sizes, not continuous linear radius.**
*Rationale:* Proportional symbols are the correct primitive for counts/totals and are geography-independent (small dense markets stay visible — unlike choropleth where small areas vanish). But readers systematically *underestimate* area differences, and the error grows with magnitude. Axis Maps' recommendation is to use "only a few discrete symbol sizes (small/medium/large)" — the accuracy gain outweighs the lost detail. Scale by area so a cluster of 100 isn't drawn 10× the radius of a cluster of 10.
*Change:* `circle-radius` driven by `["interpolate", ["linear"], ["sqrt", ["get","count"]], …]`, bucketed to ~3 size stops. This also lets the low-zoom symbol double-encode tier (color) and magnitude (size).

**7. Make dot size zoom-responsive with a MapLibre `interpolate(["zoom"]…)` expression rather than a fixed radius.**
*Rationale:* A radius that reads well at z10 is a smear at z4 and a pinprick at z14. MapLibre interpolates `circle-radius` continuously by zoom when it's a **paint** property (paint camera expressions re-evaluate on fractional zoom; layout expressions only re-evaluate at integer zoom — so keep radius in paint).
*Change:*
```js
"circle-radius": ["interpolate", ["exponential", 1.5], ["zoom"],
  4, 2,   8, 4,   12, 7,   16, 11]
```
Exponential base > 1 grows dots faster at higher zoom where separation exists; tune per region density (NA vs Europe toggle).

**8. Set `circle-sort-key` so T1 draws on top of T2/T3 where dots overlap.**
*Rationale:* MapLibre resolves overlap by sort key. Without it, draw order is data order and the most important tier can hide behind weaker ones — the opposite of the intended hierarchy.
*Change:* `"circle-sort-key": ["match", ["get","tier"], "T1", 3, "T2", 2, 1]` (higher = drawn last/on top for `circle` layers).

**9. For the proximity rings / catchments, use fill opacity and an outline rather than solid fills, and consider cap on how many render at once.**
*Rationale:* Overlapping translucent symbols are the accepted partial fix for symbol congestion — transparency lets stacked features show through. Solid catchment fills at scale create a muddy blanket that defeats the point-data reading.
*Change:* `fill-opacity ~0.12–0.18` with a 1px stroke at the tier color; gate ring rendering to the active selection / viewport rather than all 10,739 catchments.

---

## C. Legend & on-map affordances likely missing

**10. Add a persistent legend that encodes BOTH variables you're using: tier color and (if adopted) symbol size.**
*Rationale:* Good legends place symbol-left / label-right, evenly spaced and aligned, with clear wording — "the symbols you chose may be excellent, but if the layer names describing them aren't clear, the symbols won't work." A tiered co-location map with no legend forces the investor/agent audience to guess what blue vs orange means.
*Change:* In the left glass panel, a compact legend: three color swatches labeled "T1 — strongest co-location / T2 / T3" with a one-line plain-language definition of a tier. If you adopt proportional cluster symbols, add a **nested-circle size legend** (concentric circles, smallest on top, value labels along the tops) — it's the most space-efficient legend layout, ideal for your panel.

**11. Add a dynamic/contextual legend and a visible data-vintage + source line on the map face.**
*Rationale:* Web legends benefit from screen-specific spacing/alignment rules; and for a credibility-driven product, on-map attribution (basemap © OpenFreeMap/OSM, your data sources + vintage) is both a cartographic norm and a trust signal. You already have a "Data" modal — surface a one-line summary on the map itself.
*Change:* Persistent small-print attribution bottom-left; legend updates to reflect active filters (e.g., when only T1 is toggled on, dim the T2/T3 legend rows).

**12. Add interaction affordances: hover/focus highlight, and a "zoom to see individual locations" cue at low zoom.**
*Rationale:* When aggregation hides individual points, users need to know dots will resolve on zoom (standard cluster-map convention). Cooperative collision handling and clear cluster-vs-point styling prevent the "is this one store or fifty?" confusion.
*Change:* Distinct styling for aggregate clusters (e.g., count label inside) vs individual dots; cursor `pointer` + a subtle stroke-width bump on hover via `feature-state`.

---

## D. Basemap choice trade-offs

**13. Positron is the correct default — keep it — because it is purpose-built for point data overlays.**
*Rationale:* CARTO/Stamen designed Positron as a light-gray basemap specifically good for point data, with a deliberately muted feature hierarchy so overlays dominate. The general rule — light backgrounds pair with dark/saturated overlays, dark backgrounds with light overlays — favors positron given your tier colors are mid-to-dark hues. This is the right call.
*Change:* None to the default; but self-host or cache the style/sprites/glyphs rather than depending on the OpenFreeMap external CDN for an investor-facing product (single point of failure + latency on the credibility demo). PMTiles you already self-serve; do the same for the basemap glyphs/sprite at minimum.

**14. Offer dark and satellite as optional modes, not the default — each has a specific job.**
*Rationale:* Dark Matter is "good for polygon/line data" and gives better contrast for *light* overlays — useful if you add line/flow layers (commuter/mobility). Satellite/hybrid adds real-world context (rooftops, parking, big-box footprints) that resonates strongly with retail real-estate site selection — a powerful "show me the actual site" affordance. But satellite imagery is visually noisy and tanks dot contrast, so it's a toggle, not a base.
*Change:* Add a 3-way basemap switch (Positron default / Satellite-hybrid / Dark). On satellite, auto-bump dot halos to dark + thicker (rec. 3) since the imagery is busy and mostly mid-tone.

**15. If you switch a layer to dark, invert the dot halo, don't just keep white.**
*Rationale:* Halo contrast logic flips with background luminance; a white halo on dark is fine for light dots but a dark halo separates light dots from dark basemap better. Redundant-coding accessibility must hold in every mode.
*Change:* Make halo color a function of active basemap (white on positron/satellite-light areas, dark on Dark Matter).

---

## E. Retina / crispness

**16. Your vector layers are already retina-correct — the real win is confirming MapLibre renders at the device pixel ratio and that any RASTER assets (sprites, satellite tiles) are @2x.**
*Rationale:* PMTiles vector layers are resolution-independent — MapLibre rasterizes them at runtime to `container * devicePixelRatio`, so dots/text are automatically crisp on retina. The crispness risk is only in **raster** assets: the sprite sheet (custom icons), and any raster basemap (satellite). MapLibre appends `@2x` for raster tiles when `devicePixelRatio ≥ 2`; for the sprite you must ship a `sprite@2x.png` + `sprite@2x.json`.
*Change:* (a) Ship a `@2x` sprite for any custom marker icons; (b) if you add satellite, use a provider that serves @2x/512px HiDPI tiles; (c) leave `pixelRatio` unset so MapLibre uses `devicePixelRatio` (don't pin it to 1). For static export/screenshots used in the investor deck, render with `pixelRatio: 2`.

**17. Prefer GPU `circle` layers over DOM `Marker` elements for the dots (you likely already do via vector tiles — confirm).**
*Rationale:* DOM markers add a node per point and "slow down rendering and interactions" at hundreds–thousands; symbol/circle layers are vector and far more efficient. With thousands of dots, DOM markers would also blur/jank on retina.
*Change:* Ensure all tier dots are MapLibre `circle` (or `symbol`) layers bound to the PMTiles source, not `new maplibregl.Marker()`. Reserve DOM markers for the single selected/active site only.

---

## F. Accessible contrast for dot colors on positron (consolidated)

**18. Treat 3:1 (SC 1.4.11) as the floor for each dot color vs the positron land color, and enforce it with the halo + size redundancy rather than color alone.**
*Rationale:* Markers are graphical objects (3:1), not text (4.5:1). But contrast against a *single* background isn't enough on a multi-tone basemap (white land, gray roads, blue water, green parks) — a dot can clear 3:1 on land and fail over water. Halos guarantee a constant local 3:1 boundary regardless of what's underneath, and double-encoding (color + size, rec. 6) means colorblind users and low-contrast situations still convey tier.
*Change:* Mandatory white halo (rec. 3) + Okabe-Ito or sequential ramp (recs 1–2) + size-by-tier where feasible. Don't rely on hue alone to distinguish T1/T2/T3 — that's the single biggest accessibility failure mode for this map type.

---

### Net priority order (highest impact first)
1. **Aggregate at low zoom** (recs 5–6) — fixes the dominant legibility + 27MB-payload problem.
2. **Halos on dots** (rec 3) — one-line change, fixes contrast everywhere.
3. **Switch tier colors to ordered + colorblind-safe + double-encode size** (recs 1–2, 6).
4. **Add a real legend** (recs 10–11).
5. **Zoom-responsive radius + sort-key** (recs 7–8).
6. **Self-host basemap assets + offer satellite/dark toggles** (recs 13–15).
7. **@2x sprite + confirm circle-layers-not-DOM-markers** (recs 16–17).

---

## Sources consulted

- [ColorBrewer 2.0](https://colorbrewer2.org/) — sequential vs qualitative classification; colorblind-safe filter
- [RColorBrewer reference manual (qualitative palette limits / colorblind safety)](https://cran.r-universe.dev/RColorBrewer/doc/manual.html)
- [Okabe-Ito palette — gold standard for categorical CVD-safe color (Vizcept)](https://vizcept.com/blog/okabe-ito-palette-guide)
- [Okabe-Ito hex code reference (ConceptViz)](https://conceptviz.app/blog/okabe-ito-palette-hex-codes-complete-reference)
- [Axis Maps — Proportional Symbols guide](https://www.axismaps.com/guide/proportional-symbols)
- [Wikipedia — Proportional symbol map](https://en.wikipedia.org/wiki/Proportional_symbol_map)
- [Penn State GEOG 486 — Choosing Symbols for Maps](https://courses.ems.psu.edu/geog486/node/893)
- [MapLibre GL JS — Optimising performance for large GeoJSON / large data](https://maplibre.org/maplibre-gl-js/docs/guides/large-data/)
- [MapLibre GL JS — Create and style clusters](https://maplibre.org/maplibre-gl-js/docs/examples/create-and-style-clusters/)
- [MapLibre Style Spec — Expressions (interpolate / zoom / exponential)](https://maplibre.org/maplibre-style-spec/expressions/)
- [MapLibre Style Spec — Layers (circle-radius paint vs layout, sort-key)](https://maplibre.org/maplibre-style-spec/layers/)
- [MapLibre Native — Symbol placement & collision detection](https://deepwiki.com/maplibre/maplibre-native/3.3-symbol-placement-and-collision-detection)
- [MapLibre GL JS — MapOptions (pixelRatio / devicePixelRatio)](https://maplibre.org/maplibre-gl-js/docs/API/type-aliases/MapOptions/)
- [MapLibre GL JS issue #141 — retina/HiDPI raster tiles (@2x behavior)](https://github.com/maplibre/maplibre-gl-js/issues/141)
- [MapTiler — 256 vs 512 vs HiDPI/Retina tiles](https://docs.maptiler.com/guides/maps-apis/maps-platform/difference-between-256x256-512x512-and-hidpiretina-rasterized-tiles/)
- [CARTO Documentation — Basemaps (Positron vs Dark Matter use cases)](https://docs.carto.com/carto-user-manual/maps/basemaps)
- [Stamen — Introducing Positron & Dark Matter](https://stamen.com/introducing-positron-dark-matter-new-basemap-styles-for-cartodb-d02172610baa/)
- [W3C WAI — Understanding SC 1.4.3 Contrast (Minimum)](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum)
- [WebAIM — Contrast and Color Accessibility](https://webaim.org/articles/contrast/)
- [Make Things Accessible — WCAG 2.2 AA contrast (3:1 non-text)](https://www.makethingsaccessible.com/guides/contrast-requirements-for-wcag-2-2-level-aa/)
- [Map Library — Color contrast strategies for map accessibility](https://www.maplibrary.org/9529/7-color-contrast-strategies-for-map-accessibility/)
- [Esri ArcGIS Blog — Cartographic design: Legends](https://www.esri.com/arcgis-blog/products/product/mapping/cartographic-design-legends)
- [Map Library — Techniques for effective map legends (nested-circle layout)](https://www.maplibrary.org/1451/techniques-for-creating-effective-map-legends/)