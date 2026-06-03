---
name: gis-location-intelligence-archetypes
description: Location Intelligence three-archetype model (PRO/VWH/PKS) + data-collection learnings for the GIS pipeline
metadata:
  type: project
---

**Location Intelligence** is the umbrella concept for three GIS co-location archetypes.
Codes ratified 2026-06-01: **PRO / VWH / PKS**.

- **PRO** — Professional Centres: the existing T1/T2/T3 grocery-anchored retail clusters (the base map product, NOT a toggle overlay).
- **VWH** — Urban Fringe: the light-industrial fringe near demand (JIT depots, dry-clean, plumbers, painters). The early proxy model (hardware-no-hypermarket) was replaced — see Production models below.
- **PKS** — Commuter: park-and-ride development thesis. The early model (car parks co-located with a cluster, car_rental signal) was replaced by a geometric airport-led model — see below.

**Production models (2026-06-03 — `build-vwh-clusters.py` / `build-pks-clusters.py`, independent of retail clusters; commit `aec2187e`):**
- **VWH = Retail-density.** `qualify_vwh(cats)` admits ≥2 distinct trade-supply categories OR any lone STRONG/BROAD store (hardware/mro/tool_rental/electrical/plumbing/lumber/flooring/welding); drops lone WEAK (auto_parts/paint). Tier `tier_vwh(cats,n)` = `|cats| + 2·|cats∩STRONG| + (hardware?1) + min(n,8)` → T1≥10/T2≥5/T3<5. **7,028 features.**
- **PKS = geometric airport-led park-and-ride.** Candidate = sized regional airport (park-and-fly, ≤600km from a metro ref) OR outer commuter-rail-belt station (15–110km ring, connected toward core, ≤4 stops from line end). **Airports LEAD because they are the geographic-spread lever — rail clusters in corridors; airports blanket the map** (rail-only = 96 NA map cells; airport-led = 957, ≈10×). Tier `tier_pks_geo(metro_d, inward, iso_km, outward, is_airport)`. **5,977 features.** Both target ~Retail density (~6,500).
- **Planned (June 4 overnight):** parking layer → BUILT/PARTIAL/GREENFIELD "no-parkade-yet" filter for PKS; 4 new VWH categories (builders'/self-storage/trade-counter/parcel-depot) for genuine co-locations.

**Cache-busting is MANDATORY on data redeploy:** the map fetches `archetype-*.geojson?v=<token>`; the browser caches geojson as fresh, so changing the file alone does NOT update the live map (stale-cache trap — caused a "not updating" report). Bump the `?v=` token in `index.html` on every rebuild + redeploy. Panel counts are PER-REGION (NA/EU tab), not global totals — clarify when a number looks low.

**Map UI (current):** Retail / Urban Fringe / Commuter are a **radio group** (mode switch), not independent overlays. The BentoBox is **mode-aware** (T1/T2/T3 counts per active archetype, drill-on-click, archetype detail panel) and was given a full mobile **detent sheet** (peek/half/full via `window.SHEET`) + footbar hardening. Sources `uf-src`/`cm-src` load `data/archetype-{vwh,pks}.geojson?v=<token>`. (The earlier "leave the BentoBox untouched / overlays only" guidance is superseded.)

**Enrichment design (critical):** the 10 VWH/PKS chain categories attach to clusters as an enrichment pass in `build-clusters.py` (`enrich_with_vwh`, `ENRICH_CATS`, `ENRICH_RADIUS_KM=5.0`), exactly like hospitals/universities — within 5km, marked `_enrich`, emitted with `category_of()`. They are NOT in `_RETAIL_CATS`, so they **never gate PRO tier or change cluster geometry** (verified: tier counts identical after enrichment, 6,493 clusters unchanged).

**Data-collection learnings:**
- OSM `brand:wikidata` uses **brand** QIDs, not company/holding QIDs. Research-sourced Wikidata IDs are often the parent company (returns ~0 or tiny counts). Fixes found live: AutoZone Q4826087 (not Q2241044), O'Reilly Q7071951, Sherwin-Williams Q48881, Würth Q679750, Enterprise Q17085454 (not Q2283517, which gave 20 records). **Always add `name_query` fallback to every chain YAML** — it triggers when wikidata returns exactly 0 and recovers the chain.
- `multi_country: true` only skips the per-record country filter; it still queries ONE bbox. To sweep all EU, add `query_bbox: EU` (added an "EU" bbox to ingest-osm.py COUNTRY_BBOX = (35,-10,71.5,31.5)). This doubled Würth/Europcar and recovered Hilti.
- Overpass public servers (kumi/private.coffee) **throttle by IP after heavy use** (~40 ingests). Put a working endpoint first in OVERPASS_URLS; the 406-ing overpass-api.de last. Heavy `way` queries (airports, industrial landuse) over large bboxes get queued/dropped. **US/CA airport queries must be bbox-tiled.** Transit infrastructure ingest (airports+railway) is genuinely an overnight job — run detached with `setsid` + retry-on-throttle backoff.

See [[gis-build-spec]], [[gis-regional-markets]], [[feedback-overnight-builds]].
