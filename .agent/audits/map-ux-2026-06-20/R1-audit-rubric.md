The research is complete. The rubric below is my return value.

---

# Audit Rubric: Web-Based Interactive Mapping Application

> Reusable, prescriptive checklist. Each check has a pass/fail target and, where the field defines one, a numeric threshold. Annotations marked **[gis]** call out the specific finding for `gis.woodfinegroup.com` so the rubric doubles as an immediate scorecard.

---

## 1. Load & Runtime Performance

**Field-data targets (measure at 75th percentile, mobile + desktop):**

- **LCP ≤ 2.5s** good / 2.5–4.0s needs-improvement / >4.0s poor. For a map app the LCP element is usually the canvas first paint or the glass panel — ensure it is not blocked behind the 27MB eager fetch.
- **INP ≤ 200ms** good / 200–500ms needs-improvement / >500ms poor. Pan/zoom/click must respond within 200ms; long main-thread tasks parsing 19MB JSON will blow this.
- **CLS ≤ 0.1** good / 0.1–0.25 needs-improvement / >0.25 poor. Reserve fixed dimensions for the map container, legend, and panels so late-loading data does not shift layout.

**Payload & budget checks:**

- **Critical-path (compressed) ≤ 170KB**; hard ceiling ~250–300KB compressed JS on CPU-constrained devices. **[gis] FAIL — MapLibre alone is 785KB uncompressed; brotli it (~200KB) and confirm it loads, but the 201KB inline `index.html` defeats HTML caching.**
- **Code-split into chunks ≤ 50KB each.** **[gis] FAIL — ~95% of app logic is inline in one 3,659-line file. Extract to a hashed, cacheable `app.js`; the inline pattern forces re-download of all logic on every HTML change and prevents the JS budget from ever being met.**
- **No eager mega-payloads.** Defer/stream anything not needed for first interaction. **[gis] CRITICAL — 27MB eager load (clusters-meta.json 19MB + archetype GeoJSON 6.4MB + rm-top400 1.2MB). Convert clusters-meta + archetypes to PMTiles/vector tiles so the client downloads only the current viewport, not the whole dataset.**
- **No N+1 fetch storms.** **[gis] CRITICAL — 10,739 catchment-cell JSON files fetched one-per-click. Pack into a single PMTiles layer or a server-side range-request archive; or batch + cache. This is both an INP killer and an SEO/UX liability.**
- **Compression:** Brotli first, gzip fallback, on every text asset (JS/JSON/GeoJSON/CSS/SVG). Pre-compress static `.br`/`.gz` and set `Vary: Accept-Encoding`. Verify the 19MB JSON ships brotli (it should drop ~85–90%).
- **Cache headers:** content-hashed static assets → `Cache-Control: public, max-age=31536000, immutable`. Tiles/data with a stable URL → long max-age; HTML → short or `no-cache` with revalidation. **[gis] Verify PMTiles, fonts, and basemap sprites carry immutable long-cache headers.**
- **Lazy vs eager:** load basemap + viewport tiles eagerly; load catchments, ecoregions, koppen, commuter layers only when their toggle is on. **[gis] FAIL — heavy layers appear bundled into the initial load rather than fetched on toggle.**
- **Third-party dependency:** `positron` basemap is on an external CDN (OpenFreeMap). Audit its TTFB, uptime, and CORS; for an investor-facing credibility story, consider self-hosting the basemap so a third-party outage cannot dark the map.

---

## 2. Map Rendering & Cartography

- **Vector tiles over raw GeoJSON** for any large dataset (client downloads only visible tiles). **[gis] Good for PMTiles layers; extend the same treatment to clusters-meta and archetypes (currently raw JSON/GeoJSON).**
- **Source `maxZoom` tuned** (~12 for point sources) and **layer `minZoom` set** so tiles aren't requested where features are invisible.
- **Coordinate precision ≤ 6 decimals** (~1cm) in all GeoJSON; strip unused properties; simplify geometry (Mapshaper). Cuts payload with no visible loss.
- **Marker / label declutter:** enable clustering (`clusterMaxZoom` ~14, `clusterRadius` ~50px) or symbol collision so T1/T2/T3 dots don't overplot at low zoom. Use zoom-dependent label visibility so labels appear progressively, not all at once.
- **Color scale integrity:** for any choropleth/tier coloring, use a perceptually ordered scale (sequential light→dark for magnitude; categorical only for unordered classes). Ensure T1/T2/T3 colors are distinguishable for color-vision deficiency (don't rely on hue alone — add size/shape/label).
- **Legend present and adjacent to the map,** plain-language class labels, matches the rendered colors exactly. **[gis] Confirm the tier dots + every toggled layer (koppen, ecoregions, etc.) each have a legend entry.**
- **Scale bar and/or zoom indicator** visible; **projection** appropriate (Web Mercator is standard for MapLibre but note area distortion if catchment areas are quoted).
- **Basemap quality:** confirm `positron` (muted, low-chroma) lets the data layers dominate — correct choice for data-viz; verify glyphs/sprites load and labels aren't clipped at retina DPR.

---

## 3. Interaction UX

- **Pan/zoom responsiveness** within the 200ms INP budget; momentum/inertia feel native on touch.
- **Selection feedback:** clicking a cluster gives immediate visual selection state + an opening detail panel; never a silent 10,739-file fetch with no spinner. **[gis] Add an optimistic loading state on cluster click.**
- **Search/discovery:** chain search should debounce, show result counts, support empty/no-match messaging, and fly-to the selected result. Confirm keyboard submit and clear affordances.
- **Filters:** tier filters (T1/T2/T3) and layer toggles should show active state, be reversible, and indicate when a filter yields zero results.
- **Mobile touch targets ≥ 24×24 CSS px (WCAG 2.5.8 AA); aim for 44×44 (iOS) / 48×48dp (Android).** Apply to tier dots, toggles, search, and the bottom-panel controls. **[gis] Audit the colored tier dots and glass-panel controls at mobile width.**
- **Drawer/sheet pattern:** the mobile bottom panel should support partial/expanded states, not occlude the map, and be dismissable. Left glass panel should collapse on small screens.
- **Empty, loading, and error states** for every async surface (search, catchment fetch, layer load). No blank panels.
- **Loading affordance for the 27MB initial load** — a skeleton or progress indicator so the map doesn't appear frozen.

---

## 4. Accessibility (WCAG 2.2 AA)

- **Keyboard operability (SC 2.1.1):** map pannable/zoomable via keyboard; all controls (search, toggles, tier filters) reachable by Tab in logical order; Enter activates, Space opens menus; **no keyboard traps.** **[gis] Canvas-based MapLibre is typically a keyboard black hole — verify or add keyboard handlers.**
- **Focus management & logical flow:** after an action, move focus to the next logical element (search → results; cluster select → detail panel). Visible focus indicator with **≥3:1 contrast.**
- **Screen-reader support:** wrap map in `role="region"` + descriptive `aria-label`; announce dynamic changes via `aria-live="polite"` (selection, zoom level, result counts); errors via an alert region.
- **Text/non-visual alternative:** provide an accessible **list or data-table view** of the same clusters/rankings (the rm-top400 data is ideal for this). A canvas map is inaccessible without it — and this also helps SEO (see §5).
- **Contrast:** text ≥4.5:1, UI components & graphical objects ≥3:1. Verify glass-panel text over translucent backgrounds and tier-dot colors against `positron`.
- **Color independence:** never encode tier by color alone — pair with label/shape.
- **Reduced motion:** honor `prefers-reduced-motion` — disable fly-to easing/inertia animation for users who set it.

---

## 5. SEO / Shareability / Structured Data

- **`<title>`, meta description, canonical** present and descriptive.
- **Open Graph + Twitter Card** (`og:title`, `og:description`, `og:image`) so shared links render a preview — important for an investor story. Use a static map/screenshot as `og:image`.
- **JSON-LD structured data** in `<script type="application/ld+json">`: `Dataset` (name, description, keywords, publisher, license, spatialCoverage) for the location-intelligence corpus; `Place`/`GeoCoordinates` or `GeoShape` for notable markets. **[gis] CRITICAL GAP — "No JSON-LD" today. Add `Dataset` + `Organization` (PointSav) markup; rich results lift CTR 20–30% and this directly supports the credibility narrative.**
- **Sitemap.xml** + robots.txt; ensure non-JS content is crawlable.
- **Deep-linkable map state in the URL** (center/zoom/region/active chain/tier/layers as query or hash params) so a specific view is shareable and bookmarkable. **[gis] Verify NA/Europe toggle, selected chain, and active layers are reflected in the URL.**
- **Crawlable fallback content:** because the app is a canvas, search engines see little — the accessible list/table view from §4 doubles as indexable HTML.

---

## 6. Trust / Credibility

- **Data provenance & attribution:** cite every source (WorldPop, OSM, OpenFreeMap, Kontur, TIGER, GISCO, etc.) with vintage/date. OpenFreeMap/OSM attribution is a license requirement, not optional. **[gis] Surface a "Data" modal — the artifact registry shows a `text-gis-data-methodology-dialog` draft; ship it.**
- **Methodology disclosure:** explain O-D catchment model, ring radii, tier definitions, spend multipliers in accessible language — links to TOPIC docs. Critical for a site-selection audience making capital decisions.
- **Provisional/confidence framing:** label estimates as estimates; note coverage gaps (e.g., OSM-sparse countries). Aligns with disclosure discipline.
- **Error handling visible to users:** failed tile/data/basemap loads should degrade gracefully with a message, not a blank or frozen map. Confirm behavior when the external basemap CDN is unreachable.
- **Freshness signal:** show data "last updated" date; investors and site-selectors both weight recency.

---

## Priority fixes for gis.woodfinegroup.com (highest blast-radius first)

1. **Eliminate the 27MB eager load** → PMTiles for clusters-meta + archetypes; viewport-only fetch. *(Perf §1, the dominant problem.)*
2. **Kill the 10,739-file N+1 catchment fetch** → single tiled/range-request archive + loading state. *(Perf §1 / UX §3.)*
3. **Extract inline JS → hashed, code-split, brotli'd, immutable-cached files.** *(Perf §1.)*
4. **Add JSON-LD (`Dataset` + `Organization`) + OG tags + an accessible list/table view.** *(SEO §5 / A11y §4 — one accessible HTML surface satisfies both.)*
5. **Keyboard + screen-reader pass on the canvas map and panels.** *(A11y §4.)*
6. **Ship the data-provenance/methodology modal.** *(Trust §6.)*

---

## Sources consulted

- [web.dev — Web Vitals (LCP/INP/CLS thresholds)](https://web.dev/articles/vitals)
- [web.dev — How Core Web Vitals thresholds were defined](https://web.dev/articles/defining-core-web-vitals-thresholds)
- [web.dev — Setting performance budgets with webpack](https://web.dev/articles/codelab-setting-performance-budgets-with-webpack)
- [MapLibre GL JS — Optimising performance for large GeoJSON datasets](https://maplibre.org/maplibre-gl-js/docs/guides/large-data/)
- [MapLibre GL JS — Performance Optimization Techniques (DeepWiki)](https://deepwiki.com/maplibre/maplibre-gl-js/5.2-performance-optimization-techniques)
- [MapLibre GL JS — Tile Management (DeepWiki)](https://deepwiki.com/maplibre/maplibre-gl-js/2.4-tile-management)
- [Nielsen Norman Group — Interactive UX Maps 101](https://www.nngroup.com/videos/interactive-ux-maps/)
- [Nielsen Norman Group — 10 Usability Heuristics](https://www.nngroup.com/articles/ten-usability-heuristics/)
- [W3C WAI — Understanding SC 2.5.8 Target Size (Minimum)](https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html)
- [W3C WAI — Understanding SC 2.5.5 Target Size](https://www.w3.org/WAI/WCAG21/Understanding/target-size.html)
- [Minnesota IT — Accessibility Guide for Interactive Web Maps (PDF)](https://mn.gov/mnit/assets/Accessibility%20Guide%20for%20Interactive%20Web%20Maps_tcm38-403564.pdf)
- [BOIA — Interactive Maps and Accessibility: 4 Tips](https://www.boia.org/blog/interactive-maps-and-accessibility-4-tips)
- [Minnesota IT — Making Maps Accessible to Screen Readers](https://mn.gov/mnit/media/blog/?id=38-645700)
- [Felt — Choropleth maps: color-coding without misleading](https://felt.com/blog/choropleth-maps)
- [Datawrapper Academy — Customizing your choropleth map](https://academy.datawrapper.de/article/118-customizing-your-choropleth-map)
- [arXiv — Zoomless Maps: external labeling for dense point sets](https://arxiv.org/pdf/2008.13556)
- [Simon Hearne — Caching header best practices](https://simonhearne.com/2022/caching-header-best-practices/)
- [MDN — Cache-Control header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cache-Control)
- [Cloudflare — Content compression (Brotli/Gzip)](https://developers.cloudflare.com/speed/optimization/content/compression/)
- [SALT.agency — Beginner's guide to JSON-LD Schema for SEOs](https://salt.agency/blog/json-ld-structured-data-beginners-guide-for-seos/)
- [Google Search Central — Understanding Core Web Vitals](https://developers.google.com/search/docs/appearance/core-web-vitals)