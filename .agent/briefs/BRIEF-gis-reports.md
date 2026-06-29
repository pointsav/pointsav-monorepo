---
artifact: brief
schema: foundry-brief-v1
status: active
brief-id: project-gis-gis-reports
owner: project-gis
created: 2026-06-24
updated: 2026-06-25
---

# BRIEF: GIS Reports — 4-Page Location Intelligence Print Report

## Scope

Upgrade the existing 2-page print report (stats + vector map snapshot) to a 4-page
location intelligence report. Operator-confirmed page order:

| Page | Content | Status |
|---|---|---|
| 1 | Cluster stats panel | **Shipped** |
| 2 | Vector map snapshot — tier legend below map | **Shipped + legend added 2026-06-25** |
| 3 | Wikipedia context (2-col if metro) · anchor matrix · trade area table · co-location table · nearby tenancy | **Shipped + major layout overhaul 2026-06-25** |
| 4 | Satellite aerial · header · category legend | **Shipped + header/legend improved 2026-06-25** |

---

## Background

The current `printOnePager()` function (~line 908 of `index.html`):
1. Sets print date header
2. Flies to cluster center at `calcPrintZoom(span_km)` (formula: `clamp(round(14.5 − log₂(max(1, span))), 11–15)`)
3. Waits for `map.once('idle')` → captures `map.getCanvas().toDataURL()` → injects into `<img id="print-map-img">`
4. Calls `window.print()`

Pages are separated by CSS `break-before: page` on `.print-only` divs after the stats aside.

---

## Phase 1 — Page 3: Retailer Table + Regional Market Context

### Data sources

**Nearby retailers (non-anchor):**
- Overpass API public endpoint: `https://overpass-api.de/api/interpreter`
- Query: `[out:json]; node(around:{radius_m},{lat},{lng})["shop"]; out center;`
- Filter: normalize POI name → remove entries whose `brand` or `name` fuzzy-matches any key
  in the existing `CHAIN_FAMILIES` config in index.html. Show remainder.
- Columns: **Name · Category (`tags.shop`) · ≈ Distance (km)** (haversine from cluster center,
  client-side)
- Sort: ascending by distance
- Row cap: top 40 entries (dense urban areas return 500+; cap keeps page to 1–2 print pages)
- Latency: 200–800 ms typical for 3 km radius
- Rate limit: public Overpass ~1 req/sec; one request per user print action — well within limits

**Regional Market context (from cluster props + `regional-markets.json`):**

Confirmed field names from `clusters-meta.json` schema (verified 2026-06-24):

| Field in cluster props | Meaning |
|---|---|
| `p.rm` | RM ID (e.g., `rm_us_goose_creek`) |
| `p.mkt` | Settlement name (e.g., `"Goose Creek, SC"`) |
| `p.mrgn` | Metro region (e.g., `"Charleston-North Charleston Metro Area"`) |
| `p.iso` | ISO country code (e.g., `"US"`) |
| `p.cont` | Continent (`"NA"` / `"EU"`) |
| `p.t` | Tier (1 / 2 / 3) |
| `p.td` | Tier description (e.g., `"Hypermarket + Hardware"`) |

Additional RM fields from `www/data/regional-markets.json` (keyed by `rm_id`):
- `cluster_count` — total co-locations in this RM
- `best_tier` — highest tier present in the RM
- `metro_market` — metro area name (alternate to `mrgn`)

The `regional-markets.json` file is 2,986 records and must be loaded/fetched at print time
unless it is already in memory. Implementation decision: either `fetch('/data/regional-markets.json')`
on demand, or maintain a small `rmIndex` map loaded at app startup (file is ~300 KB).

### HTML structure (page 3)

```html
<div id="print-retail-page" class="print-only">
  <div class="print-rm-header">
    <h2>[cluster display_name or anchor_label] — Nearby Retailers</h2>
    <p class="print-rm-context">
      Regional Market: <strong>[p.mkt]</strong> · [p.mrgn] · [cluster_count] co-location(s)
    </p>
    <p class="print-ring-note">Retailers within [ring_radius_km] km radius · Ring shown on maps above</p>
  </div>
  <table class="print-retail-table">
    <thead><tr><th>Name</th><th>Category</th><th>≈ Distance</th></tr></thead>
    <tbody><!-- filled by JS --></tbody>
  </table>
  <p class="print-attribution">Retailer data © OpenStreetMap contributors, ODbL · Overpass API</p>
</div>
```

### Async flow integration

`printOnePager()` must orchestrate 3 async tasks before calling `window.print()`:
1. Page 2 snapshot — already exists (fly → idle → toDataURL); **no change**
2. Page 3: Overpass query → parse + render table
3. Page 4: Satellite map render → toDataURL → inject

Tasks 2 and 3 can run concurrently after page 2 snapshot completes.

```js
// Sketch — exact implementation at coding time
map.once('idle', async () => {
    // Page 2 capture (existing)
    document.getElementById('print-map-img').src = map.getCanvas().toDataURL('image/png');

    // Page 3 + 4 in parallel
    const [retailers, satImg] = await Promise.allSettled([
        fetchOverpassRetailers(coords, ring_radius_km * 1000),
        renderSatelliteSnapshot(coords, zoom)
    ]);
    
    renderRetailerTable(retailers.value || []);
    if (satImg.value) document.getElementById('print-sat-img').src = satImg.value;
    
    window.print();
});
```

### Degradation

- Overpass 429 or timeout (>5 s): show "Retailer data unavailable (network)" in `<tbody>`.
- Satellite render failure: hide `#print-sat-page`; print 3 pages instead of 4.

---

## Phase 2 — Page 4: Satellite Aerial View

### Basemap

**ESRI World Imagery** — confirmed feasible, no API key for moderate use.

MapLibre inline style (no external style.json):
```js
const SAT_STYLE = {
    version: 8,
    sources: {
        esri_sat: {
            type: 'raster',
            tiles: ['https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}'],
            tileSize: 256,
            attribution: '© Esri, DigitalGlobe, Earthstar Geographics, CNES/Airbus DS, USDA, USGS, AeroGRID, IGN'
        }
    },
    layers: [{ id: 'esri_sat', type: 'raster', source: 'esri_sat' }]
};
```

Note: ESRI tile URL uses `{y}` before `{x}` (row/col ordering, not standard XYZ).

### Ring overlay on satellite

Re-use existing `makeCircleGeoJSON(center, radiusKm)` (already in index.html) to add a
white 3 px ring layer over satellite. White is more legible than amber on satellite imagery.

```js
satMap.addSource('print-ring', { type: 'geojson', data: makeCircleGeoJSON(coords, ring_radius_km) });
satMap.addLayer({ id: 'print-ring-line', type: 'line', source: 'print-ring',
    paint: { 'line-color': '#FFFFFF', 'line-width': 3, 'line-opacity': 0.9 } });
```

### POI overlay on satellite

Re-add `all-locations` source from `pmtiles://tiles/layer1-locations.pmtiles` on the satellite
map instance. Set visible. Use same circle paint as main map `all-locations` layer but increase
opacity to 0.9 for print contrast. Keep default radius.

### Satellite render flow

```js
async function renderSatelliteSnapshot(coords, zoom) {
    return new Promise((resolve) => {
        const container = document.createElement('div');
        container.style.cssText = 'position:absolute;width:1200px;height:900px;left:-9999px;top:0';
        document.body.appendChild(container);
        
        const satMap = new maplibregl.Map({
            container, style: SAT_STYLE,
            center: coords, zoom,
            preserveDrawingBuffer: true
        });
        
        const cleanup = () => { satMap.remove(); container.remove(); };
        const timeout = setTimeout(() => { cleanup(); resolve(null); }, 12000);
        
        satMap.on('load', () => {
            // Add ring + POI layers
            satMap.addSource('print-ring', { type: 'geojson', data: makeCircleGeoJSON(coords, ring_radius_km) });
            satMap.addLayer({ id: 'print-ring-line', type: 'line', source: 'print-ring',
                paint: { 'line-color': '#FFFFFF', 'line-width': 3 } });
            // Add all-locations PMTiles source + layer
            // ... (mirror main map source/layer config)
            satMap.flyTo({ center: coords, zoom, essential: true });
        });
        
        satMap.once('idle', () => {
            clearTimeout(timeout);
            const img = satMap.getCanvas().toDataURL('image/png');
            cleanup();
            resolve(img);
        });
    });
}
```

Memory note: satellite map instance peaks at ~100–150 MB; destroyed after capture, before
`window.print()` opens. Acceptable for on-demand print flow.

### HTML structure (page 4)

```html
<div id="print-sat-page" class="print-only">
  <div class="print-map-caption">
    Aerial view · zoom [Z] · [span_km] km span · © Esri, DigitalGlobe, Earthstar Geographics
  </div>
  <img id="print-sat-img" class="print-map-img">
</div>
```

---

---

## Tenant Rep / National Retailer Requirements (2026-06-25)

Research summary: what tenant reps and national retailer expansion teams need in a site selection report.

### What we provide today (mapped to tenant rep priorities)

| Metric | Priority | Status |
|---|---|---|
| Trade area population (site ring + regional market) | Critical | ✅ pp / pg fields; shown in Trade Area table (page 3) |
| Estimated consumer spend (modelled) | Critical | ✅ sp / sg fields; shown in Trade Area table (page 3) |
| Co-tenancy / anchor composition | Critical | ✅ members array; anchor matrix bubbles + co-location table |
| Cluster tier + Top-400 market rank | High | ✅ t + rm-top400.json; shown page 1 + co-location table |
| Competitive landscape (nearby tenancy) | High | ✅ Overpass OSM retailers within ring; page 3 table (5 categories) |
| Regional Market / Metro Area context | High | ✅ metro_name / rm_type; co-location table scoped to RM or Metro Area |
| Wikipedia market context | Medium | ✅ two Wikipedia entries (local + metro), two-column layout |
| Satellite aerial view | Medium | ✅ page 4, ESRI World Imagery |
| Climate / site conditions | Low–Medium | ✅ AEC block page 1 (Köppen, flood, seismic) |
| Commuter worker population | Medium | ✅ partial — LODES (US), MITMA (ES); shown on-screen; not yet on print page 3 |

### What is missing — Phase 3 roadmap

| Item | Priority | Notes |
|---|---|---|
| **Drive-time isochrones (15/30 min)** | Very High | Single most-requested metric in NA retail site selection. Open routing: Valhalla (Docker, ~10 GB). Population/spend within drive-time is more defensible than crow-flies radius. |
| **Age + income demographic breakdown** | High | Census tract / DA level. US: ACS 5-year; CA: StatCan NHS; EU: Eurostat. Pre-join to H3 cells. Unlocks proper demographic profile block on page 1 / page 3. |
| **Traffic volume proxy** | Medium | AADT is proprietary (HERE, TomTom). Near-term substitute: OSM road classification at centroid (motorway/trunk/primary/secondary) — already queryable. Coarse but removes a blank field. |
| **Commuter pop on print (page 3)** | Medium | LODES / MITMA commuter data is on-screen. Add "Daytime worker catchment" row to Trade Area table when mobility data is present. |
| **Parking area estimate** | Low–Medium | Derivable via OSM `amenity=parking` polygons near centroid. Physical-site grounding the satellite page lacks. |
| **"Prepared by" branding block** | High UX | Tenant reps hand this to their clients. A placeholder "Prepared by: [Firm Name]" line on page 1 header makes the report presentation-ready. Add to Phase 3. |
| **Parking stall count / GLA** | Future | Not derivable from OSM reliably. Requires commercial data (CoStar, CBRE). Post Phase 3. |
| **Real traffic counts (AADT)** | Future | Proprietary. HERE/TomTom API. Post Phase 3. |
| **Lease rates / vacancy** | Future | Requires CoStar / local BID data. Post Phase 3. |

### Trade Area table structure (current vs. ideal)

**Current** (page 3, shipped 2026-06-25):
| Scope | Population | Est. Annual Spend |
|---|---|---|
| X km ring — Trade Area | pp | sp |
| RM or Metro Area name | pg | sg |

**Ideal addition (Phase 3):**
Add "Daytime worker catchment" row using LODES/MITMA `ph` field where available.

---

## Open Questions

| # | Question | Notes |
|---|---|---|
| OQ1 | **ESRI TOS for client-facing commercial product** | "Moderate use" is informal. MapTiler Satellite (single app-level key, 100K views/month free tier) is the cleaner commercial path. Revisit at client-billing stage. |
| OQ2 | **Overpass hosting threshold** | Public Overpass ~1 req/sec. Self-hosting (Docker, ~250 GB OSM) recommended if volume exceeds ~200 prints/day. |
| OQ3 | **Retailer table scope** | `["shop"]` only, or extend to `["amenity"~"restaurant\|cafe\|pharmacy\|fuel"]`? Current plan: `["shop"]` only for clarity. |
| OQ4 | **Page 3 row cap** | 40 rows (dense urban areas return 500+). Cap strategy: take closest 40. |
| OQ5 | **Ring colour on satellite** | White (3 px) recommended — amber risks blending with building/terrain. |
| OQ6 | **RM data loading strategy** | Fetch `regional-markets.json` on demand at print time, OR load at app startup into `window.rmIndex`. File is ~300 KB — startup load is simpler. |

---

## Implementation Checklist

Phase 1 (retailer table + RM context):
- [ ] Add `fetchOverpassRetailers(coords, radiusM)` function
- [ ] Add `filterOurBrands(poiList)` using `CHAIN_FAMILIES` config
- [ ] Add `haversineKm(a, b)` (or re-use existing distance function if present)
- [ ] Add `renderRetailerTable(poiList, rmData)` → writes to `<tbody>` in `#print-retail-page`
- [ ] Load/fetch RM data keyed by `p.rm` at print time
- [ ] Add `<div id="print-retail-page">` HTML block + CSS after `#print-map-page`
- [ ] Update `printOnePager()` to gate `window.print()` on Overpass completion
- [ ] Test: dense T1 cluster (urban), sparse T3 cluster (suburban), Overpass timeout path

Phase 2 (satellite map):
- [ ] Add `renderSatelliteSnapshot(coords, zoom, ring_radius_km)` function
- [ ] Add `<div id="print-sat-page">` + `<img id="print-sat-img">` HTML block
- [ ] Add CSS `break-before: page` on `#print-sat-page`
- [ ] Run both phases concurrently with `Promise.allSettled()`
- [ ] Test: satellite tiles load in time; ring visible; POIs visible on satellite; toDataURL succeeds
- [ ] Verify 4-page output in browser print preview (not just `window.print()` side-effects)

---

## Research trail

- `foundry-draft-v1`: no — this is an engineering BRIEF, not an editorial draft
- Satellite basemap research: exploration agents 2026-06-24 (ESRI World Imagery confirmed feasible,
  no API key, tile URL `…/World_Imagery/MapServer/tile/{z}/{y}/{x}`)
- Overpass API research: exploration agents 2026-06-24 (public endpoint, 200–800 ms, `["shop"]` query)
- Dual MapLibre feasibility: exploration agents 2026-06-24 (~100–150 MB peak, acceptable for on-demand print)
- clusters-meta schema verified: 2026-06-24 via `/www/data/clusters-meta.json`
- regional-markets schema verified: 2026-06-24 via `/www/data/regional-markets.json`
