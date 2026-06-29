---
artifact: brief
schema: foundry-brief-v1
status: active
brief-id: project-gis-delivery-rearchitecture
owner: project-gis
parent: project-gis-map-ux-audit
created: 2026-06-20
updated: 2026-06-20
---

# Delivery Re-Architecture

Engineering BRIEF for the delivery and front-end-architecture work behind
`gis.woodfinegroup.com`. This is the single largest technical-diligence risk
surfaced by the 2026-06-20 map UX/tech audit
(`BRIEF-gis-map-ux-audit-2026-06-20.md`): the data asset and co-location model
are differentiated, but the delivery layer reads as a talented-solo prototype.
A technical reviewer opening the network panel sees the failure in roughly ten
minutes — a 27 MB eager payload, a 10,739-request fetch storm on the first node
click, and ~95% of the application logic inline in one 201 KB `index.html`.

This BRIEF scopes five findings into a sequenced engineering plan: **F3**
(eager payload), **F4** (N+1 catchment fetch), **F10** (inline-JS extraction),
**F31** (external basemap single point of failure), and **F9** (metro-view
404s). It does not cover the cartographic, scorecard, or mobile findings —
those route to `project-design` and to the separate White-Space & Cannibalization
BRIEF.

## Scope and deployment posture

- **Owner:** project-gis (Totebox). All work is engineering owned here; the
  doc-gap items derived from the audit route to `project-editorial` /
  `project-design` separately.
- **Edit target — localhost first.** Every change in this BRIEF ships to the
  localhost `www/` working copy first
  (`/srv/foundry/deployments/gateway-orchestration-gis-1/www/`), is re-rendered
  and re-screenshotted in the browser-in-the-loop harness, and only then is
  proposed for production. **Production (push-to-prod, nginx config, the public
  `gis.woodfinegroup.com` vhost) is Command-scope and is not touched by this
  archive.** Any header, compression, or range-serving change that requires an
  nginx directive is staged as a recommendation in the outbox for Command, with
  the exact directive written out; this BRIEF does not edit production nginx.
- **Build provenance.** The data artifacts (`clusters-meta.json`,
  `archetype-*.geojson`, `catchment-cells/`, the `tiles/*.pmtiles` layers) are
  produced by the build pipeline under `app-orchestration-gis/`. Any change to
  their packaging is a change to the build scripts that emit them, captured in
  `guide-gis-pipeline-rebuild` and run on the overnight build window per the
  overnight-builds policy (start after 22:00 Vancouver / 05:00 UTC).

## Current state (verified 2026-06-20)

Measured against the live working copy at
`/srv/foundry/deployments/gateway-orchestration-gis-1/www/`:

| Asset | Size / count | Load trigger | Mechanism |
|---|---|---|---|
| `data/clusters-meta.json` | 19.5 MB | eager (page load) | `fetch('data/clusters-meta.json')` |
| `data/archetype-vwh.geojson` | 3.1 MB | eager | `fetch('data/archetype-vwh.geojson?v=…')` |
| `data/archetype-pks.geojson` | 3.5 MB | eager | `fetch('data/archetype-pks.geojson?v=…')` |
| `data/rm-top400.json` | 143 KB | eager | `fetch('data/rm-top400.json')` |
| `data/catchment-cells/*.json` | 10,739 files | one per node click | `fetch(\`data/catchment-cells/${clusterId}.json\`)` |
| `index.html` | 205 KB (≈3,659 lines, ~95% inline JS) | eager | inline `<script>` |
| `lib/maplibre-gl.js` | 803 KB uncompressed | eager | external `<script src>` |
| basemap style + glyphs + sprite | external | eager | `https://tiles.openfreemap.org/styles/positron` |
| `tiles/*.pmtiles` | self-served, range-requested | viewport | PMTiles protocol (correct) |

The eager total (clusters-meta + two archetypes + rm-top400 + MapLibre, before
compression) is ~27 MB of data plus ~1 MB of library. The PMTiles layers are
the one part already done correctly — viewport-only, range-served. The fix is
to bring the JSON/GeoJSON payloads and the catchment cells up to the same
standard.

---

## F3 — 27 MB eager payload → viewport-only tiles

**Problem.** On first load the client downloads the entire dataset before the
map is interactive: `clusters-meta.json` (19.5 MB) and the two archetype
GeoJSONs (6.6 MB combined). The map cannot answer "is it slow because of my
connection or because it's broken?" — both read the same to a reviewer. At the
audit's field-data targets (LCP ≤ 2.5 s, INP ≤ 200 ms) this fails on both:
parsing a 19 MB JSON blob is a multi-hundred-millisecond main-thread task, and
nothing renders behind it.

**Target.** Initial transfer for first interaction ~1–3 MB compressed. The
client downloads only what is in the current viewport at the current zoom; the
EU constellation is not paid for while looking at North America, and vice versa.

**Approach (two phases).**

1. **Interim — region-split + brotli (low effort, ships this week).** Split
   `clusters-meta.json` and the archetype GeoJSONs into per-region files
   (`clusters-meta-na.json`, `clusters-meta-eu.json`, archetype-*-na/eu), and
   load only the region the NA/EU toggle currently shows. The off-screen region
   loads on toggle, not at startup. In parallel, pre-compress every text asset
   to brotli (`.br`) — a 19 MB JSON of repeated keys and numeric strings
   typically drops 85–90%, so even un-split the wire cost falls to ~2–3 MB.
   This is a build-script change (emit split + `.br` siblings) plus a small
   front-end change (region-aware fetch). It does **not** yet hit the
   viewport-only target but defuses the ten-minute-diligence failure and
   demonstrates intent. The brotli pre-compression also needs an nginx
   `Vary: Accept-Encoding` + pre-compressed-static directive — **staged for
   Command**, written out, not applied here.

2. **Structural — clusters + archetypes into PMTiles.** Move the cluster
   centroids/metadata and the archetype polygons into PMTiles vector layers
   alongside the existing `tiles/*.pmtiles`, sourced via the same self-served
   PMTiles protocol the map already uses. The cluster dots become a
   vector-tile source with `minzoom`/`maxzoom` tuned and per-feature properties
   (tier, span_km, anchor count, confidence flag) carried in the tile so the
   panel can read them without a separate JSON. This is the only approach that
   actually reaches the ~1–3 MB target, because the client then downloads tiles
   for the current bounding box only. Generation is a `tippecanoe` step added to
   the build pipeline (centroids as points; archetypes as polygons with
   `--drop-densest-as-needed` and `maxzoom` tuned the way `layer3-catchment`
   already is). The front end swaps the GeoJSON sources for `pmtiles://` vector
   sources; the click handler reads feature properties from the tile.

**Sequencing note.** The metadata that drives the scorecard (F5, separate
BRIEF) and the proportional-symbol cartography (F2/F23, project-design) both
want per-feature properties available at the dot. Tiling clusters-meta is the
shared substrate for all three — so this is done as a clean property schema in
the tile, not a minimal port. Coordinate the property names with the scorecard
BRIEF before the build runs.

**Rough effort.** Interim: ~0.5 day front end + ~0.5 day build script.
Structural: ~3–4 days (tippecanoe schema design, build-script integration,
front-end source swap, click-handler rewrite to read tile properties,
re-screenshot verification). Overnight build slot for the tile generation.

---

## F4 — 10,739-file N+1 catchment fetch on click → one archive

**Problem.** Clicking a node fires `fetch(\`data/catchment-cells/${clusterId}.json\`)`
against a directory of 10,739 individual JSON files. There is no loading state,
no optimistic selection, and no cache — re-clicking the same node re-fetches.
On a metro view where several clusters are visible and a user clicks through
candidates, this is a fetch storm that blows the INP budget and, on any cluster
whose file is missing, produces the silent 404s seen in F9. 10,739 files is also
a deployment and integrity liability: every overnight rebuild has to write,
sync, and (in prod) serve all of them.

**Target.** A node click resolves its catchment in one request, with immediate
visual feedback. Re-clicks are instant from cache.

**Approach.**

1. **Pack catchment cells into one PMTiles layer keyed by cluster id.** The
   per-cell H3 geometry already exists; pack all cells into a single
   `layer-catchment-cells.pmtiles` with each feature tagged by its `clusterId`.
   The click handler filters the tiled source by `clusterId` instead of
   fetching a file. This collapses 10,739 files to one range-served archive and
   removes the file-path failure mode entirely (a missing cluster is an empty
   filter result, not a 404). This is the preferred end state and parallels the
   F3 structural approach (same PMTiles substrate, same build step).

   *Alternative if cells must stay JSON short-term:* a single range-request
   archive (one concatenated file + a byte-offset index keyed by `clusterId`,
   served with `Range` requests — the same capability the PMTiles serving
   already relies on) gives the one-request property without a tippecanoe pass.
   Either way the directory of 10,739 files is retired.

   *Fallback for environments without range serving:* a batched endpoint
   `GET /catchment?cluster=<id>` returning the cells for one cluster — but this
   needs a dynamic backend the current static-file deployment does not have, so
   it is the least-preferred option and noted only for completeness.

2. **Optimistic select + loading skeleton.** On click, immediately render the
   selected-state on the dot (stroke/fill change) and a skeleton in the panel,
   then resolve the catchment. The map never appears frozen; the user always
   sees that the click registered. This is a front-end-only change and is
   independent of which packaging approach (1) takes, so it ships first.

3. **LRU cache.** Keep an in-memory LRU (e.g. last ~50 clusters' catchment
   geometry) so re-clicks and back-and-forth comparison are instant. Front-end
   only; ships with (2).

**Sequencing note.** (2) optimistic-select + skeleton and (3) LRU are
front-end-only and ship immediately on the localhost copy — they make the
*current* fetch feel responsive and remove the "did it work?" ambiguity while
the packaging work lands. (1) the PMTiles/range packaging is the durable fix and
is done with the F3 structural build pass so both run in one overnight slot.

**Rough effort.** Optimistic-select + skeleton + LRU: ~0.5–1 day front end.
PMTiles packaging of cells: ~1.5–2 days (build step + click-handler rewrite +
verification), bundled with F3 structural.

---

## F10 — ~95% logic inline in 201 KB index.html → modular app.js

**Problem.** `index.html` is ~205 KB / ~3,659 lines with the overwhelming
majority of the application logic inline in `<script>`. This is the first thing
technical diligence flags — it is a key-person/maintainability signal and it
defeats HTTP caching: any one-line copy change forces the browser to re-download
the entire application, and the JS can never be `Cache-Control: immutable`
because it has no stable hashed URL.

**Target.** A small, short-cache `index.html` (markup, meta, the JSON-LD/OG tags
F21 will add) referencing a content-hashed, brotli-compressed `app.<hash>.js`
served `Cache-Control: public, max-age=31536000, immutable`. Repeat visits and
content edits no longer re-download the application logic.

**Approach.**

1. **Extract, do not rewrite.** Move the inline script to `app.js` verbatim as a
   first commit — a mechanical lift, not a refactor, so behavior is provably
   unchanged against the re-screenshot harness. Verify byte-for-byte behavioral
   equivalence before any cleanup.
2. **Content-hash + compress.** Emit `app.<contenthash>.js` (+ `.br`) from the
   build; `index.html` references the hashed name. Hashing is what makes the
   immutable long-cache safe. The immutable header is an nginx directive —
   **staged for Command**, written out.
3. **Modularize incrementally (optional, later).** Once extracted, the file can
   be split into modules (map init, data loading, panel/scorecard, search,
   layer toggles) behind a tiny bundler step. This is deferred — the extraction
   alone captures the caching and diligence wins; modularization is a quality
   follow-up, not a blocker.

**Sequencing note.** This is the lowest-risk, highest-signal item and is done
**first**, before the F3/F4 structural work, because (a) it is a mechanical
extraction with a clean equivalence check, and (b) the F3/F4 front-end changes
(source swaps, click-handler rewrite, optimistic-select) are far easier to make
and review in `app.js` than buried in a 3,659-line HTML file. Extract first; then
edit.

**Rough effort.** Extraction + hashing + compression: ~1 day including
verification. Incremental modularization (deferred): ~2–3 days when scheduled.

---

## F31 — External OpenFreeMap CDN basemap → self-host / cache

**Problem.** The basemap style, glyphs, and sprite load from
`https://tiles.openfreemap.org/styles/positron` — an external CDN outside our
control. If OpenFreeMap is slow or unreachable during an investor demo, the map
goes dark with no graceful state. For a credibility-facing site this is a single
point of failure on the most visible surface. (The vector *tiles* are already
self-served via PMTiles; only the basemap is external.)

**Target.** The positron basemap renders from assets we control. OpenFreeMap
outage does not dark the map.

**Approach.**

1. **Self-host the positron style JSON + glyphs + sprite.** Fetch the positron
   style JSON, the glyph PBF ranges, and the sprite image/JSON, vendor them
   under `www/lib/basemap/positron/` (or a `tiles/basemap/` path consistent with
   the existing self-served layout), and rewrite the style's `glyphs`/`sprite`/
   source URLs to local paths. The positron *raster/vector source* itself is the
   larger question — if it points back to OpenFreeMap's tile endpoint, vendoring
   the style alone still leaves a tile dependency; evaluate whether to cache a
   bounded tile set for the demo extents or accept tile-source dependency while
   removing the style/glyph/sprite SPOF. Keep positron as the default; the
   audit also notes satellite/dark as optional toggles (separate, lower
   priority).
2. **Graceful failure.** Pair with the F9 global error handler so a basemap load
   failure shows a message, not a frozen canvas.

**License note.** OpenFreeMap/OSM attribution is a license obligation, not
optional — preserve the attribution control when self-hosting (this also serves
F14 provenance). Confirm the positron style's license permits self-hosting the
glyphs/sprite before vendoring.

**Sequencing note.** Independent of F3/F4/F10; can ship any time. Best paired
with F9 (shared error-handling surface). The licensing confirmation is the long
pole, not the engineering.

**Rough effort.** Style/glyph/sprite vendoring + URL rewrite + verification:
~1 day. Bounded tile-set caching (if pursued): +1–2 days and storage budget
review.

---

## F9 — 404s on metro views → root-cause + global error handling

**Problem.** Metro-zoom views produce 404s on the core drill-down happy path —
the most likely cause is the F4 catchment fetch hitting a `clusterId` with no
corresponding file in `catchment-cells/`, but it may also be a tile-asset path
generated for a layer/zoom that has no tile. Today these fail silently: blank
tiles or an empty panel with no message. A live defect on the primary
interaction is the worst possible thing for a reviewer to find.

**Target.** No silent failures. Every failing asset path is either fixed at the
source or handled with an explicit, honest state ("no data at this zoom").

**Approach.**

1. **Root-cause from the network panel.** Reproduce at metro zoom in the
   browser-in-the-loop harness, capture the failing request URLs, and classify:
   (a) catchment-cell file missing for a clicked `clusterId` — fixed
   structurally by F4 (tiled source → empty filter, not 404); (b) tile path
   requested where no tile exists — fix by setting correct `minzoom`/`maxzoom`
   on the layer so MapLibre doesn't request absent tiles; (c) a genuinely
   missing build artifact — fix in the pipeline.
2. **Global error handler.** Add a single handler for failed data/tile/basemap
   loads that surfaces a graceful state instead of a blank or frozen map, and a
   "no data at this zoom" empty state for the legitimate case where a layer has
   no features in view. Front-end only.

**Sequencing note.** The root-cause (1) is done **early** — it is diagnostic and
informs the F4 packaging design (knowing exactly which `clusterId`s lack cells
shapes the tile build). Class (a) failures are then largely resolved as a side
effect of F4. The global error handler (2) ships with the F4 optimistic-select
work since they touch the same click/load path, and is shared with F31.

**Rough effort.** Root-cause + classification: ~0.5 day. Layer minzoom/maxzoom
fixes: ~0.5 day. Global error handler + empty state: ~0.5–1 day.

---

## Recommended sequencing (overall)

Ordered to front-load the low-risk, high-signal work and to batch the build-step
changes into single overnight slots.

1. **F10 extraction first.** Lift inline JS to `app.<hash>.js` verbatim; verify
   behavioral equivalence. Everything downstream is easier to edit and review in
   `app.js`. (~1 day)
2. **F9 root-cause + F4 front-end feel.** Diagnose the metro 404s; ship
   optimistic-select, loading skeleton, LRU cache, and the global error handler.
   These are front-end-only and remove the "is it broken?" ambiguity
   immediately. (~1–1.5 days)
3. **F3 interim region-split + brotli.** Build-script emits split + `.br`
   payloads; front end loads current region only. Stage the nginx
   `Vary`/pre-compressed-static directive for Command. Cuts the eager wire cost
   to ~2–3 MB this week. (~1 day)
4. **F31 basemap self-host.** Vendor positron style/glyphs/sprite (pending
   license confirmation); pair with the F9 error handler. (~1 day)
5. **F3 + F4 structural (one overnight build slot).** Tile clusters-meta,
   archetypes, and catchment cells into PMTiles with a coordinated property
   schema; swap front-end sources; rewrite the click handler to read tile
   properties and filter cells by `clusterId`. This reaches the ~1–3 MB
   viewport-only target and retires the 10,739-file directory. (~4–5 days +
   overnight generation)
6. **F10 modularization (deferred).** Split `app.js` into modules behind a small
   bundler step when scheduled. Quality follow-up, not a blocker. (~2–3 days)

Total to "passes ten-minute technical diligence": items 1–4, roughly one focused
week on the localhost copy. Items 5–6 are the durable architecture and run on
the overnight build cadence.

## Cross-references and dependencies

- **Master backlog:** `BRIEF-gis-map-ux-audit-2026-06-20.md` (F3, F4, F9, F10,
  F31 rows; "Doc gaps → routing" names this BRIEF).
- **Synthesis:** `.agent/audits/map-ux-2026-06-20/SYNTHESIS.md` §2 (F3/F4/F9/F10/F31)
  and §"Priority fixes" 1–3.
- **Rubric thresholds:** `.agent/audits/map-ux-2026-06-20/R1-audit-rubric.md` §1
  (LCP/INP/CLS targets, payload budgets, the explicit gis FAIL/CRITICAL calls
  that this BRIEF closes).
- **Competitor precedent:** `R2-competitor-teardown.md` — Mapbox
  style-optimized vector tiles and Kepler.gl Arrow/GPU aggregation as the
  industry remedies for exactly the eager-load and N+1 problems (gap-analysis
  items 1, 7, 8).
- **Property-schema coordination:** the F3 cluster tile schema is shared with
  the Site Scorecard work (F5, project-design) and proportional-symbol
  cartography (F2/F23, project-design). Settle property names before the build.
- **Build pipeline:** changes to data packaging are changes to
  `app-orchestration-gis/` build scripts, documented in
  `guide-gis-pipeline-rebuild` and run on the overnight window (after 22:00
  Vancouver) per the overnight-builds policy.
- **Scope boundary:** all changes ship to the localhost `www/` copy first and
  are re-screenshot-verified; **production (push-to-prod, nginx, the public
  vhost) is Command-scope.** nginx directives required by F3 (brotli `Vary` +
  pre-compressed static) and F10 (immutable long-cache on hashed assets) are
  staged for Command in the outbox with the exact directives written out, not
  applied from this archive.

## Forward-looking note (BCSC posture)

The targets and sequencing above describe work this archive **plans/intends** to
implement on the localhost copy; they are not claims of delivered capability or
of public-site performance. Effort figures are rough estimates intended for
planning. Any production-facing performance outcome is contingent on the
Command-scope deployment steps and is not asserted here.
