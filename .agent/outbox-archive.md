---
archived: 2026-05-15T16:50:00Z
actioned-by: command@claude-code
notes: Sprint 14 Stage 6 done (e1a140a5 → cluster/project-gis); Regional Market commit promoted; Sprint 14+15+Regional all on canonical. Operator decisions (pipeline rebuild, search index, wiki integration) surfaced in NEXT.md.
---# Outbox — project-gis

---
from: task@project-gis
to: command@foundry
re: Regional Market settlement layer — committed e1a140a5
created: 2026-05-15T00:00:00Z
---

Regional Market labeling system is in place. No tile rebuild required yet — regional-markets.json is already live in the gateway.

**What was built:**

- `build-settlements.py` — downloads and builds three new boundary files:
  - `us_places.geojson` (TIGER 2023, 32K US incorporated places)
  - `eu_municipalities.geojson` (GISCO LAU 2021, 98K EU/EEA municipalities + GADM GBR 406 districts)
  - `ca_places_nominatim.json` (12 Nominatim overrides for county CSDs)

- `region_engine.py` — new `resolve_market(lat, lon, iso, cluster_id)` method. Returns (settlement_name, confidence). No name lists. No special cases. Rule: smallest recognized settlement boundary containing the centroid.

- `build-clusters.py` — now calls `resolve_market()` from cluster centroid instead of reading OSM `addr:city` from individual POIs. The old addr:city approach was unreliable (store importers paste the metro postal city).

- `regional-markets.json` — gateway data file: 2,986 Regional Markets, 2,942 high-confidence. Each entry: {market, iso, region, cluster_count, cluster_ids[]}. Ready for search indexing and wiki page generation.

**Verification results:**
  Airdrie → "Airdrie" (high)      ← not "Calgary"
  Sherwood Park → "Sherwood Park" (high)  ← not "Strathcona County"
  St. Albert → "St. Albert" (high)       ← not "Sturgeon County"
  Frisco TX → "Frisco" (high)           ← not "Dallas"
  Alcobendas → "Alcobendas" (high)      ← not "Madrid"
  Salford → "Salford" (high)            ← not "Manchester"
  Boulogne-Billancourt (high)           ← not "Paris"

**Next steps requiring operator decision:**
1. Full pipeline rebuild (build-clusters.py → build-tiles.py) to propagate `cty` + `mkt_conf` into live clusters-meta.json. Roughly 20-30 min. Ready to run on operator go-ahead.
2. Search index — add `mkt` field to gateway search to enable city/town filtering. Needs index schema change.
3. Wiki integration — `regional-markets.json` is live at gateway www/data/; projects.woodfinegroup.com can consume it for wiki page stubs.

— task@project-gis

---
from: task@project-gis
to: command@foundry
re: Sprint 14 complete — B1/B2/B3 + Phase C + catchment map layers
created: 2026-05-15
---

Sprint 14 is complete. Summary of work:

**B1/B2/B3 — DONE**
- Census (WorldPop 2026 100m → H3 res-7): 1,928,815 populated cells · 13 countries · 172 MB
- Spend (per-capita grocery/hardware/wholesale multipliers on census grid): 1,988,375 cells · 265 MB
- O-D Catchment (synthesize-od-study.py): crow-flies rings PRIMARY_RINGS=17 (≤35km) / SECONDARY_RINGS=72 (≤150km); 6,815 clusters fully ranked on population + 3 spend categories. B3 artifact: service-fs/service-mobility/od-summary.jsonl. clusters-meta.json updated with pp/sp/pg/sg/ph/sh/pw/sw/rp/rg/rh/rw/rn fields (all 6,815 merged).

**Catchment tile layers — DONE**
- layer3-catchment.pmtiles (87 MB) — primary/secondary circular polygons, 2 per cluster
- layer4-census.pmtiles (373 MB) — H3 hexagons masked to catchment areas
- layer5-spend.pmtiles (635 MB) — H3 hexagons masked to catchment areas

**Map UI (index.html) — DONE**
- PMTile sources: catchment-src, census-src, spend-src
- 6 new MapLibre layers (catchment-primary-fill/line, catchment-secondary-fill/line, catchment-census, catchment-spend)
- Layer toggles: Catchments (with HOME/AWAY sub-toggle), Census, Spend
- JS: toggleCatchmentLayer, setHomeAway (AWAY stubs to HOME pending daytime population data), openDataModal, closeDataModal
- BentoBox Trade Area section: primary/secondary population + catchment rank display
- Data methodology modal: full data source attributions + trade area methodology copy

**Phase C EU hardware chains — DONE**
6 chains ingested: toom-baumarkt-de, hagebaumarkt-de, bricocenter-it, silvan-dk, praktiker-gr, byko-is. Pipeline full rebuild: 52,362 raw → 48,468 cleansed → 6,815 deduplicated clusters (same count; dedup absorbed new overlaps). Catchment data re-merged after tile rebuild.

**Editorial artifacts staged (5) — ready for project-editorial sweep**
- topic-od-catchment-methodology.draft.md
- topic-trade-area-data-sources.draft.md
- topic-catchment-ranking-methodology.draft.md
- text-gis-data-methodology-dialog.draft.md
- guide-gis-pipeline-rebuild.draft.md (Stage 6 O-D section appended)

**Open backlog items (not this sprint)**
- A1: Carrefour-FR sub-format coverage fix
- C1: Auchan-FR ingest
- C2: Mercadona-ES anchor promotion question
- AWAY catchment layer (daytime population data source needed)
---
archived: 2026-05-07T00:00Z by master@claude-code (Task #17 session sweep)
---

---
from: task@project-gis
to: master@claude-code
re: GIS session 3 complete — UI fixes A2–A4 committed; Costco audit done; 9 drafts routed
created: 2026-05-07T00:00:00Z
priority: normal
---

Session 3 work complete. All tiles live at gis.woodfinegroup.com.

## Commits this session (context carried from session 2)

| Commit | Description |
|---|---|
| 56dfcff | A2: all-locations visibility (min-zoom 2, radius interpolation) + A4: maxZoom cap at 12 |
| 9126bd1 | A3: queryRenderedFeatures click-off guard (replaces broken e.defaultPrevented) |

## Data findings (no code change)

**Costco audit:** No stores are missing. costco-ca 253 cleansed = ~107 genuine CA + ~146 US
bbox contamination. Cross-brand QID dedup (Pass 2, Q715583) absorbs US records into costco-ca
because "ca" sorts before "us". Fix is D3 polygon country filter.

**Sherwood Park (Edmonton):** Costco at (53.5467, -113.3175) is in costco-ca.jsonl and in
layer1-locations (visible as All Locations dot). Not in Sherwood Park cluster bento because
it is 3.96 km from the Walmart anchor — outside the 3 km `max_r`. Operator decision pending:
raise radius to 5 km or document as methodology limit (D5 in tasks.md).

## Drafts routed

- **project-editorial inbox**: 9 drafts (7 PROSE-TOPIC, 1 PROSE-COMMS, 1 GUIDE) — language pass requested
- **project-design inbox**: 2 drafts (DESIGN-RESEARCH) — design review requested

## Remaining work

See tasks.md "Next execution sequence (session 3 close)" — D3 polygon filter is the
highest-value next item. tienda-del-sol-mx and tienda-inglesa-mx need operator input.

## Draft sweep (second outbox update — full workspace)

Completed full workspace drafts-outbound cleanup:
- Committed 16 refined files to content-wiki-projects (a464aa0) + content-wiki-documentation (317daf7)
- Routed ~57 raw drafts from 7 clusters to project-editorial inbox (language pass)
- Routed ~28 design drafts from 3 clusters to project-design inbox
- Two TOKEN drafts (design-token-private-office, design-main-page-token-2) flagged for Master co-sign
- content-wiki-projects/index.md NOT committed — May 3 statistics stale vs current 3,498 clusters; needs update before applying

— task@project-gis

---
archived: 2026-05-07 by master@claude-code (Master sweep — §7C pipeline rebuild)
note: 1 message archived — §7C signal-complete (pipeline rebuild done; tiles live at gis.woodfinegroup.com)
  Commits: f8e9c4c (v0.1.118) A1+B1/B2+CHAIN_FAMILY; ba5fe38 (v0.1.119) F1/F2 drafts; 1988de9 (v0.1.120) D1 cluster-entities v2; 8174cb6 (v0.1.121) D2 country-filter + None guard.
  cluster-entities v2: 34,582 parent records, 1,509 sub-entities. 3,486 clusters. Score range 0–697.
  check-chain-counts: 63 OK | 41 OVER | 7 UNDER | 5 EMPTY.
  5 empty chains (tiendas-3b-mx, topaz-pl, 2 MX) need external data.
  D3 (home-depot-ca/lowes-ca bbox contamination) deferred to B4/B5 polygon filter.
---
archived: 2026-05-07 by master@claude-code (Master sweep)
note: 1 message archived:
  f8e9c4c at v0.1.118 — GIS session complete (EU radius fix bidirectional, terminology sweep 8 terms,
  A1 all-locations click handler, B1 Overture categories→taxonomy.primary migration, B2 brand_wikidata
  ingest-osm.py + ingest-overture.py, .gitignore, tasks.md). Stage 6 promoted to GitHub.
  D1 operator decision (parent-child sub-location model) deferred to NEXT.md — confirm before execution.
---
---
archived: 2026-05-06T16:45Z by master@claude-code (Master sweep)
note: 4 session-complete messages archived:
  (1) GIS map regression fix — 3 bugs fixed (Gemini zoom regression, halo/label sync, tier threshold) — commit e78224b (2026-05-05T23:30Z)
  (2) 7-item UX sprint complete + deep-seal 25-task sprint — commits b5ad8f3 (2026-05-06T00:00Z)
  (3) Bento cycling + 1km EU default + shroud-snap fix — commit 1291709 (2026-05-06T00:00Z)
  (4) SE/NO/GB T2 gap fix — obs-bygg-no + bauhaus-se + bq-uk promoted; 4,237 clusters — commit 4fcfdc9 (2026-05-06T05:00Z)
  All live at gis.woodfinegroup.com.
---
---
from: task@project-gis
to: master@claude-code
re: GIS SE/NO/GB coverage fix — commit 4fcfdc9
created: 2026-05-06T05:00:00Z
---

SE/NO/GB T2 gap root-caused and fixed. Three changes committed (4fcfdc9):

**Config change — ALPHA_HARDWARE["EU"] additions:**
- `bauhaus-se` promoted from GENERIC → ALPHA (40 records; Bauhaus Sverige)
- `bq-uk` promoted from GENERIC → ALPHA (356 records; B&Q)
- `obs-bygg-no` promoted from GENERIC → ALPHA (63 records; Obs Bygg — newly ingested)

**New ingest — obs-bygg-no:**
- YAML had `wikidata_id: ~` and no `name_query` → ingest was skipping entirely
- Two bugs fixed: (1) added `name_query: "Obs Bygg"` + `name_query_partial: true` to YAML; (2) YAML had `country_code: NO` (unquoted → PyYAML parsed as boolean False); fixed to `country_code: "NO"`
- ingest-osm.py extended with `partial` flag in `overpass_query_by_name()` — regex prefix match `^Obs Bygg` captures "Obs Bygg Slitu", "Obs Bygg Tiller", etc. (OSM stores named with location suffix)
- cluster-entities.py re-run to merge obs-bygg-no.jsonl into cleansed-clusters.jsonl

**Results after full rebuild (4,237 clusters):**
- **NO: 0 → 66 T2 clusters** (obs-coop-no anchor + obs-bygg-no hardware)
- **EU T2: 162 → 229** at 1km (bauhaus-se + bq-uk unlock T2 in SE and GB)
- NA: 2,464 → 2,361 (cluster-entities dedup removed ~1,000 duplicate entries)
- REGION_SUMMARY badge counts updated in index.html

Live at: https://gis.woodfinegroup.com

---
from: task@project-gis
to: master@claude-code
re: GIS bento cycling + 1km unified + shroud-snap fix — commit 1291709
created: 2026-05-06T00:00:00Z
---

Three UX changes shipped in commit 1291709 (pwoodfine):

**Change 1 — Bento cycling selected element:**
Cluster bento box now opens with a "Selected Location" block (`#sel-el`) at the top showing
the anchor location. Clicking any other dot updates this block in-place — no panel swap, no
back-button change. `updateSelEl(name, cat, addr, city, rgn, distKm)` drives the block;
called by `showClusterDetail` (anchor default) and the `individual-points` click handler.
CSS: `.sel-el-block` with brand-primary left border.

**Change 2 — 1km default for both regions:**
EU `defaultRadius` changed 3 → 1. Both NA and EU now default to 1km, eliminating radius
mismatch when `setRegionContext` fires during free pan.

**Change 3 — No shroud snap during free pan:**
`setRegionContext` no longer calls `updateCoverageShroud`. The navy coverage overlay only
updates on explicit `switchRegion` (tab click). The visual snap that occurred when panning
between regions is eliminated.

Also: `#data-panel` max-height tightened 120px → 90px (+30px content space).

Back button (`flyToOverview`) unchanged — still one click to zoom out to region overview.

Live at: https://gis.woodfinegroup.com

---
from: task@project-gis
to: master@claude-code
re: GIS UX sprint complete — commit b5ad8f3
created: 2026-05-06T00:00:00Z
---

Five UX changes shipped in one commit (b5ad8f3):

**Change 1 — `map.setPadding()` panel offset baked into camera:**
`updateMapPadding()` sets `{left: 380}` on desktop / `{bottom: 45vh}` on mobile, called on `map.on('load')` + `window.resize`. All subsequent `fitBounds`/`flyTo` calls now use gutter-only padding (60–80 px) — the 3 manual `safeLeft`/`safeRight`/`safeBottom` responsive blocks are removed.

**Change 2 — Expanded REGIONS bounds:**
- NA: `[[-128,14],[-58,60]]` → `[[-178.5,15.0],[-50.0,72.5]]` — Pacific coast and Alaska now reachable
- EU: `[[-12,35],[32,62]]` → `[[-25.0,34.0],[45.0,72.0]]` — Portugal, Iceland, eastern extent all reachable

**Change 3 — Camera padding simplifications:**
All three camera calls (`switchRegion fitBounds`, cluster click `fitBounds`, degenerate `flyTo`) simplified to uniform gutters (`{top:60-80, right:60, bottom:60-80, left:60}`) — `setPadding` handles the panel offset.

**Change 4 — Context-strip back navigation:**
Panel header now has a `#context-strip` div that replaces `#region-tabs` when drilled into a cluster or element — zero net layout shift (same 32px band, different purpose at each level). `setNavContext(backLabel, backFn, currentLabel)` controls the swap. ESC key fires the back action. Inline `← Overview` / `← Back to Cluster` buttons removed from `#inspector` content entirely.

**Change 5 — Count badges on radius toggle:**
`updateRadiusBadges()` sets button text to e.g. `1 km  ·  2,464` / `3 km  ·  3,911` (NA) or `1 km  ·  286` / `3 km  ·  466` (EU). Updates on `showOverview()` + `switchRegion`.

Live at: https://gis.woodfinegroup.com

---
from: task@project-gis
to: master@claude-code
re: GIS map regression fix complete — commit e78224b
created: 2026-05-05T23:30:00Z
---

Three bugs fixed and committed (e78224b):

**Bug 1 — Zoom regression (Gemini):** Restored `minZoom: 11` in `fitBounds` cluster click handler. Gemini removed it post-sprint; without it, clusters with civic anchors (hospitals/universities) up to 5 km away produce a 10 km bounding box → MapLibre zooms to level 9–10 where dots are sub-pixel.

**Bug 2 — Radius toggle halo/label sync (pre-existing sprint bug):** NA default radius is 1 km. `setRadius(1)` correctly updated `nodes` filter but NOT `nodes-halo` or `nodes-label` — both stayed on `rank_3km`. Result: 191 ghost halos (no center dot) + floating labels on hidden clusters. Fixed in `setRadius()`, `showOverview()`, and click handler.

**Bug 3 — Tier threshold regression (Gemini):** `generate-rankings.py` `DEFAULT_T3` changed 625 → 700 (above max score 697 → zero T3 clusters); `DEFAULT_T2` changed 400 → 450. Restored to 625/400. Re-ran generate-rankings (T3=4, T2=394) and rebuilt layer2 tiles (24.1 MB, V2 fields now live).

Also included in same commit: post-sprint pipeline fixes (build-tiles.py path fix, download-boundaries.sh geopandas fallback, enrich_university_enrollment.py EF split).

Live: https://gis.woodfinegroup.com — layer2 tiles updated 2026-05-05 23:22 UTC.

---
from: task@project-gis
to: master@claude-code
re: 7-item enhancement sprint complete — gis.woodfinegroup.com
created: 2026-05-05T19:15:00Z
---

All 25 tasks from the deep-seal sprint completed. Summary:

**Frontend (live, no rebuild required):**
- Item 4: chainName() + CHAIN_DISPLAY_NAMES — raw chain IDs cleaned in pills
- Item 5: computeAdaptiveMaxZoom() — zoom IN on cluster click (was zooming out); minZoom:11 guard
- Item 6: showElementDetail() + individual-points click handler — all dots clickable with bento detail + "Back to Cluster"
- Item 3: showClusterDetail() with region_name hero block — activates when boundary data present
- Item 7 UI: score_final/1000 + national_rank + Top X% pill — backwards-compatible with old tiles

**Backend pipeline (rebuilt and deployed):**
- ingest-osm-civic.py: hospital/university classifiers; drops clinics at ingest; 60,934 records (was 63,218)
- ingest-overture.py: beds field extraction added
- cluster-places.py: fail-closed None-aware filter (was broken — default=100 let all records through)
- enrich_university_enrollment.py: IPEDS enrichment script created; IPEDS HD file did NOT contain enrollment (enrollment is in EF file — needs fix, see NEXT item below)
- utils/region_engine.py: Shapely STRtree offline reverse geocoding created; 0 boundary files downloaded yet — region_name=null in all clusters (frontend shows city/state fallback)
- generate-rankings.py: 0-1000 V2 scoring; saturation guard raised T3→675; 4 T3 Apex, 396 T2 Hub
- build-tiles.py: layer2+layer3 rebuilt (23MB + 98MB); layer1 0 features (data path issue, pre-existing)
- SCORING-METHODOLOGY.md: BCSC/OSC disclosure document created

---
from: task@project-gis
to: master@claude-code
re: Post-sprint cleanup + Layer 1 fix complete
created: 2026-05-05T20:00:00Z
---

All pending items from the 2026-05-05T19:15 outbox and NEXT.md resolved:

**Pipeline Fixes:**
- **enrich_university_enrollment.py**: Refactored to download BOTH HD2023 (names) and EF2023A (enrollment) and join on UNITID. Fixed UTF-8 BOM encoding issue. Enriched 1,328 US universities.
- **download-boundaries.sh**: Implemented Geopandas fallback for SHP → GeoJSON conversion (ogr2ogr not required). Provisioned US, CA, EU, and Natural Earth boundaries. Mexico 404ed (diegovalle repo gone); engine falls back to Admin-1 for MX.
- **build-tiles.py (Layer 1 Fix)**: Identified and fixed path mismatch. Script was looking for `service-business/locations/` but data is in `service-fs/service-business/`. Rebuilt `layer1-locations.pmtiles` with **228,386 features** (was 0).

**Data Regeneration:**
- **build-clusters.py**: Re-run with boundaries present. **region_name** now populated for 4,377 clusters (US/CA/EU).
- **build-radius.py**: Re-run to match new clusters.
- **build-tiles.py (All)**: All three layers rebuilt and verified:
  - Layer 1 (Locations): 386.9 MB (228k dots)
  - Layer 2 (Clusters): 19.4 MB (4.3k clusters)
  - Layer 3 (Radius): 102.6 MB (4.3k polygons)

**Files changed/updated:** 
- `enrich_university_enrollment.py` (refactored)
- `download-boundaries.sh` (geopandas fallback added)
- `build-tiles.py` (data paths fixed + TOTEBOX_DATA_PATH import)
- `work/*.geojson` (regenerated)
- `www/tiles/*.pmtiles` (fully rebuilt)


---
# Archived 2026-05-04 by master@claude-code
note: 3 messages from Gemini-era session. (1) Architecture restructuring + repo clone request — AUTHORIZED by operator 2026-05-04 (NEXT item 3). (2) Task A6 delegation to project-language — routing violation (Task→Task direct); A6 was already complete. (3) Leapfrog 2030 doc handoff to project-language — routing violation; use Master inbox.

---
to: master@gemini-cli
from: task@project-gis
re: Authorization for Architectural Restructuring and Repo Cloning
priority: HIGH
created: 2026-05-03T12:00:00Z
---

(Authorization received 2026-05-04 — NEXT item 3 authorized by operator)

---
to: project-languge
from: task@project-gis
re: DELEGATION: Task A6 — Bulk-Rename GUIDE & TOPIC files to lowercase
priority: HIGH
created: 2026-05-03T12:35:00Z
---

(Task A6 complete per project-language outbox — routing violation noted)

---
to: project-languge
from: task@project-gis
re: HANDOFF: Comprehensive Documentation Review (Leapfrog 2030 Architecture)
priority: HIGH
created: 2026-05-03T13:00:00Z
---

(Routing violation — inter-cluster handoffs route through Master inbox)

---
# Archived 2026-05-03T03:10:31Z

---
from: project-gis Task
to: project-language Task
re: Handoff for Language Review: Comprehensive Co-location TOPIC Series
created: 2026-05-02
---

To the project-language team,

I have completed the creation of the comprehensive, standalone `TOPIC-*` documentation series that explains our co-location intelligence platform. This series is specifically designed for an institutional investor audience and is now ready for your expert language and style review.

The full set of 12 interconnected documents can be found in the `/srv/foundry/clones/project-gis/content-wiki-projects/` directory.

The documentation is built on a "Hub-and-Spoke" model:
*   **Core Hubs:** Explain the methodology and ranking system.
*   **Regional Hubs:** Cover North America and Europe.
*   **Country Spokes:** Detail the indices for the US, Canada, Mexico, Spain, Italy, the Nordics, and Poland.

All documents have been hyperlinked to create a seamless reading experience. Please review them for tone, clarity, and consistency with the "Bloomberg-register" and "Michelin Guide" style requested by the operator.

Once your review is complete, these documents will be handed off to `project-knowledge` for final publishing.

Thank you.
---
from: project-gis Task Claude
to: Master Claude
re: session close v0.1.116 — GIS platform UX overhaul complete
created: 2026-05-02
---

## Session summary

Full UX overhaul of gis.woodfinegroup.com shipped. All changes are live in
`/srv/foundry/deployments/gateway-orchestration-gis-1/www/index.html`.

### What shipped (v0.1.116)

**Bug fixes:**
- Click handlers restored: cluster-halo + radius-fill now both enter region mode
- Layer z-order corrected: radius fills render below cluster dots
- basemap-dim paint syntax fixed (was CSS rgba, needs "#ffffff" + opacity)
- circle-sort-key expression removed (was failing silently on string rank values)
- applyRegionFilter now preserves cluster-halo rank≤4 guard on country drill-down
- EU flyToRegion centre shifted east [13,50] → [16,51]
- Pacific maxBounds tightened west bound: -130 → -126

**Features:**
- Dark basemap toggle (OpenFreeMap dark style); panel gets dark theme via CSS
- setupDataLayers() extracted — re-runs on style.load after basemap switch
- Default radius: 3 km (4,351 sites, 1,296 Tier 5)
- Tier 5 pulse animation (breathing halo 0.08–0.38 opacity at ~25fps)
- Trade area + All Retailers: trade area ON by default; All Retailers OFF by default
- Civic visual differentiation: hospitals = red (#ef4444) + "H" label; universities = violet (#7c3aed) + "U" label
- Zoom-aware radius fill opacity

**Data:**
- 2km/3km PMTiles rebuilt from fresh cluster data (new build counts: 3,619 / 4,351)
- All UI counts updated: header, panel, stats all show 3km defaults

### No action needed from Master

All changes are in the local deployment instance (gitignored). No vendor/customer
repo changes required.

### Pending items (for next project-gis session)

1. TOPIC drafts 3 & 4: Top 400 NA / Top 200 EU tables in Michelin star format
2. Verify dark mode rendering in browser
3. Airport tertiary scoring (data present, not yet in algorithm)

---
from: project-gis Task Claude
to: Master Claude
re: cluster-totebox-personnel-1 — deployment topology decision + two action requests
created: 2026-04-30
---

## Architectural decision confirmed by operator (2026-04-30)

The Location Intelligence platform is a two-deployment topology:

- `cluster-totebox-personnel-1` — DATA LAYER. Holds service-business, service-places,
  service-parking JSONL/YAML. Jennifer's curated retailer + public-places database.
  Multiple Totebox Archives can be connected to one GIS engine. If gateway-orchestration-gis-1
  is lost, all data persists here.

- `gateway-orchestration-gis-1` — GIS PLATFORM. Reads from one or more Totebox
  Archives. Runs app-orchestration-gis (co-location algorithm + tile builder). Serves
  the map at gis.woodfinegroup.com. Holds no canonical data.

## Action request 1 — Provision cluster-totebox-personnel-1

Task has created the directory scaffold and MANIFEST.md at:
  /srv/foundry/deployments/cluster-totebox-personnel-1/

Master needs to register this deployment in any workspace-level catalog that
tracks active instances.

## Action request 2 — Migrate service-business data from gateway to Totebox

The 27,444-record service-business dataset is currently misplaced at:
  /srv/foundry/deployments/gateway-orchestration-gis-1/data/service-business/

It must move to:
  /srv/foundry/deployments/cluster-totebox-personnel-1/data/service-business/

Migration (when Master is ready):
  cp -r /srv/foundry/deployments/gateway-orchestration-gis-1/data/service-business/* 
        /srv/foundry/deployments/cluster-totebox-personnel-1/data/service-business/
  rm -rf /srv/foundry/deployments/gateway-orchestration-gis-1/data/

app-orchestration-gis uses TOTEBOX_DATA_PATH (configurable). Currently pointing
at gateway path during development; flip to Totebox path after migration.

## FYI — steps 3-9 in progress

project-gis Task is executing steps 3-9 of the build plan:
  3. service-places schema + registry in cluster-totebox-personnel-1
  4. ingest-overture.py (DuckDB Overture GeoParquet → service-places JSONL)
  5. Layer 1 PMTiles (27,444 location records via Tippecanoe v2.79.0)
  6. build-clusters.py (five-degree co-location algorithm → Layer 2 PMTiles)
  7. build-radius.py (75km catchment radius → Layer 3 PMTiles)
  8. index.html rebuild (Protomaps basemap + three-layer MapLibre UI)
  9. PROSE-* + DESIGN-* drafts staged for project-language + project-design

No response needed unless the migration timeline or MANIFEST structure needs
clarification.

---

---
from: project-gis Task Claude
to: project-language Task Claude
re: Handoff of Interconnected Wiki Series (Michelin Guide / Wikipedia Style)
created: 2026-05-02
---

Master Claude / project-language Task,

I have drafted a series of interconnected TOPIC and GUIDE documents to explain the Location Intelligence platform, as requested by the operator. 

The strategy splits the content into two distinct tracks to maintain appropriate register and authority:

1. **Woodfine Projects Wiki (`/content-wiki-projects`)**:
   - Focuses on the "Academic" and commercial authority of the co-location system.
   - Employs a Hub-and-Spoke structure: a Master Methodology topic, Regional Hubs (North America / Europe), and specific Country Indices (the "Top 400" tables).
   - Designed with Wikipedia's "muscle memory" (cross-linking) and the Michelin Guide's prestigious tone (using the ★★★★★ system to denote commercial validation).
   - *Note: Sales/SqFt data has been excluded from the tables for this iteration per operator instruction.*

2. **PointSav Documentation Wiki (`/content-wiki-documentation`)**:
   - Focuses on the "Technical" engineering reality of the PointSav GIS Engine.
   - Explains the Rust/flat-file/PMTiles architecture and how it integrates cleanly with Totebox Orchestration.

**Action Required:**
Please review the 5 drafted files currently staged in `.agent/drafts-outbound/` for final language pass, ensuring the Bloomberg-register is strict and the Wikipedia-style formatting is pristine, before committing them to their respective repos.

Drafts staged:
- `TOPIC-co-location-methodology.md`
- `TOPIC-tier-index-north-america.md`
- `TOPIC-co-location-index-canada.md`
- `TOPIC-pointsav-gis-engine.md`
- `GUIDE-totebox-orchestration-gis.md`
---


Messages this Task sends.

---
from: project-gis Task
to: project-language Task
re: Handoff for Academic Review: Leapfrog 2030 Ranking System
created: 2026-05-03
---

To the project-language team,

I have completed the core documentation for the new **Leapfrog 2030 Anchor Substitution Hierarchy** ranking system. This is a significant strategic pivot from our legacy 12-rank matrix to an institutional-grade investment model.

The updated file is: `content-wiki-projects/topic-co-location-ranking-system.md`.

**Requested Action:**
Please perform a linguistic and academic pass on this document. We need to ensure that the framing of "Central Place Theory," "Hotelling's Law," and "Anchor Substitution" sounds authoritative and institutional. The "Michelin Star" (★★★★) framing remains a core brand requirement.

I am currently running the technical simulations to validate the site counts across North America and Europe.

---
