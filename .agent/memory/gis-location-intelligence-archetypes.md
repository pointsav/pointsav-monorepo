---
name: gis-location-intelligence-archetypes
description: Location Intelligence three-archetype model (PRO/VWH/PKS) + data-collection learnings for the GIS pipeline
metadata:
  type: project
---

**Location Intelligence** is the umbrella concept for three GIS co-location archetypes.
Codes ratified 2026-06-01: **PRO / VWH / PKS**.

- **PRO** — Professional Centres: the existing T1/T2/T3 grocery-anchored retail clusters (the base map product, NOT a toggle overlay).
- **VWH** — Vertical Warehouse: 3–6 story urban logistics / light-manufacturing zones. Proxy = hardware cluster with NO hypermarket, 5–80km from metro. Enriched by auto_parts/paint/mro_industrial/flooring/tool_rental/lumber/plumbing/electrical/welding.
- **PKS** — Parking Structures: 3–9 story car parks at regional airports or intercity rail stations 15–150km from a major metro, co-located with a T1/T2 cluster. Defining commercial signal = car_rental.

**Map UI:** VWH/PKS are toggle overlays in `index.html` layer control under the Layers section (below Cluster Bubbles/Köppen/Ecoregions). State vars `vwhActive`/`psActive`; functions `toggleVwhLayer`/`togglePsLayer`. They copy the rm-stars fade pattern (`applyLiOverlayStyle()` ghosts cluster bubbles to 10% when active). GeoJSON at `www/data/archetype-vwh.geojson` + `archetype-pks.geojson`. **Operator wants the BentoBox left untouched** — overlays only, no inspector changes.

**Enrichment design (critical):** the 10 VWH/PKS chain categories attach to clusters as an enrichment pass in `build-clusters.py` (`enrich_with_vwh`, `ENRICH_CATS`, `ENRICH_RADIUS_KM=5.0`), exactly like hospitals/universities — within 5km, marked `_enrich`, emitted with `category_of()`. They are NOT in `_RETAIL_CATS`, so they **never gate PRO tier or change cluster geometry** (verified: tier counts identical after enrichment, 6,493 clusters unchanged).

**Data-collection learnings:**
- OSM `brand:wikidata` uses **brand** QIDs, not company/holding QIDs. Research-sourced Wikidata IDs are often the parent company (returns ~0 or tiny counts). Fixes found live: AutoZone Q4826087 (not Q2241044), O'Reilly Q7071951, Sherwin-Williams Q48881, Würth Q679750, Enterprise Q17085454 (not Q2283517, which gave 20 records). **Always add `name_query` fallback to every chain YAML** — it triggers when wikidata returns exactly 0 and recovers the chain.
- `multi_country: true` only skips the per-record country filter; it still queries ONE bbox. To sweep all EU, add `query_bbox: EU` (added an "EU" bbox to ingest-osm.py COUNTRY_BBOX = (35,-10,71.5,31.5)). This doubled Würth/Europcar and recovered Hilti.
- Overpass public servers (kumi/private.coffee) **throttle by IP after heavy use** (~40 ingests). Put a working endpoint first in OVERPASS_URLS; the 406-ing overpass-api.de last. Heavy `way` queries (airports, industrial landuse) over large bboxes get queued/dropped. **US/CA airport queries must be bbox-tiled.** Transit infrastructure ingest (airports+railway) is genuinely an overnight job — run detached with `setsid` + retry-on-throttle backoff.

See [[gis-build-spec]], [[gis-regional-markets]], [[feedback-overnight-builds]].
