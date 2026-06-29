---
artifact: brief
schema: foundry-brief-v1
status: active
brief-id: project-gis-map-ux-audit
owner: project-gis
created: 2026-06-20
updated: 2026-06-24
---

# BRIEF — gis.woodfinegroup.com Map UX/Tech Audit (swarm, browser-in-the-loop)

Master record of the 2026-06-20 multi-agent browser-in-the-loop audit of the live retail
co-location map, benchmarked against hyperscaler / competitor mapping platforms across eight
expert personas. Supersedes nothing; feeds the implementation backlog below.

## Method (reproducible)

- **Vision:** headless Playwright Chromium (cached, non-snap) renders the **localhost** site;
  screenshots are `Read` by vision-capable agents. Full how-to in memory
  `reference-browser-in-the-loop.md`. Harnesses: `~/sandbox/wiki-harness/{gshot,cshot}.mjs`;
  Range-capable server `~/sandbox/gis-visual-audit/rangeserver.mjs` (PMTiles need byte-serving).
- **Evidence:** 7 our-site states + 8 competitor maps captured to `~/sandbox/gis-visual-audit/`
  (`ours/`, `competitors/`; index in `.agent/audits/map-ux-2026-06-20/screenshot-index.txt`).
- **Research swarm (Phase 1):** audit rubric, competitor teardown, cartographic reference —
  `.agent/audits/map-ux-2026-06-20/R1..R3*.md`.
- **Persona swarm (Phase 2):** 8 Opus personas (Costco siting, GIS prof ×2 [methodology +
  cartography], Goldman IC banker, Madison-Ave strategist, graphic designer, staff front-end,
  principal UI/UX) → 93 findings (33 P0 / 43 P1 / 17 P2) → deduped synthesis
  (`.agent/audits/map-ux-2026-06-20/SYNTHESIS.md`, `personas.json`).

## Headline diagnosis

The **data asset + co-location model are genuinely differentiated, but the delivery and
interaction layer read as a talented-solo prototype, not a decision-grade / fundable product.**
Four cross-cutting themes: (a) delivery architecture fails technical diligence in 10 minutes
(27 MB eager payload, 10,739-file N+1 catchment fetch on click, ~95% logic inline in one 201 KB
`index.html`); (b) cartographic illegibility (overplotting at low zoom, **no on-map legend** —
raised by all 8 personas, ordinal tiers wrongly encoded as unordered hues, no dot halos on a
near-white basemap); (c) the moat is invisible — a node click shows **geometric crow-flies rings
with no scorecard** (no population/spend/competitors/score); (d) mobile is effectively broken
(panel ~60 % of viewport). Two deepest analytical hits: catchments are circles not observed/
drive-time trade areas (the mobility data is already on disk), and "QUALITY TIERS" + the
`span_km/2 × 1.15` radius conflate co-location *composition* with *quality/strength*.

## Prioritized backlog (F1–F34)

Persona keys: COSTCO · PROF-GIS · CARTO · GS · BRAND · DESIGN · FE · UX.

| ID | Title | Sev | Area | Effort | Raised | Quick | Recommendation (short) |
|---|---|---|---|---|---|---|---|
| F1 | No on-map legend | P0 | carto | S | 8 | **Y** | Persistent panel+mobile legend; swatches match dot colors; plain-language tier defs; ring=catchment note |
| F2 | Overplotting at low zoom | P0 | carto | M | 8 | N | Pre-aggregate to proportional cluster symbols <~z7; explode at z7–9; circle-sort-key T1 on top |
| F3 | 27 MB eager payload | P0 | perf | L | 5 | N | clusters-meta(19MB)+archetypes→viewport PMTiles; interim region-split → ~1–3 MB |
| F4 | 10,739-file N+1 catchment fetch | P0 | perf | L | 5 | N | Pack into one PMTiles/range archive; optimistic select + skeleton; LRU cache |
| F5 | Click gives rings, no decision data | P0 | data/ux | L | 5 | N | Scorecard: pop+households (WorldPop), spend, co-located chains, explainable score+drivers |
| F6 | Crow-flies rings, not observed trade areas | P0 | carto/trust | L | 5 | N | Drive-time isochrones / O-D origins (LODES/MITMA/layer6 on disk); else relabel "straight-line bands" |
| F7 | Mobile panel dominates (~60%) | P0 | mobile | M | 7 | N | Draggable 3-state bottom sheet default PEEK; map ~70–80%; fix zoom overlap; ≥44px targets |
| F8 | Categorical hues for ordinal tiers + no halo (fails 3:1) | P0/P1 | carto/a11y | S | 7 | **Y** | Ordered/CVD-safe ramp (T1 darkest) + `circle-stroke #fff 1–1.5px` everywhere |
| F9 | 404s on metro views (live defect) | P0 | trust/data | M | 3 | N | Fix failing catchment/tile asset paths; global error handler + "no data at this zoom" |
| F10 | ~95% logic inline in 201 KB index.html | P0 | code | M | 2 | N | Extract to hashed brotli `app.js` immutable-cached; small short-cache HTML |
| F11 | Ring radius `span_km/2×1.15` has no demand meaning | P0 | data | M | 1 | N | Derive from distance-decay/drive-time/threshold; document or drop 1.15 + 1.0km floor |
| F12 | "QUALITY TIERS" conflate composition w/ strength | P0 | data | M | 2 | N | Rename "Co-location depth (anchor count)" OR compute real score (pop×spend×access) |
| F13 | No uncertainty shown (confidence flag exists) | P0 | trust | M | 1 | N | Encode confidence (opacity/hollow); per-estimate error in Method modal |
| F14 | No provenance/vintage on map face (OSM attribution = license) | P0/P1 | trust | S | 5 | **Y** | "Data: WorldPop 2026, OSM, Kontur CC-BY — updated YYYY-MM"; wire DATA modal |
| F15 | No candidate comparison/shortlist | P1 | ux | L | 1 | N | Compare tray → side-by-side table (pop, spend, competitors, tier, score) |
| F16 | No white-space/cannibalization analysis | P1 | carto/data | L | 2 | N | Quantify Union-Find ring overlap; surface white-space on chain select |
| F17 | Chain search flat (no counts/fly-to/context) | P1 | ux | M | 4 | N | On select fly-to+filter, show "#### locations", sections, keyboard, clear, aria-live |
| F18 | Overlays stack illegibly | P1 | carto/ux | M | 4 | N | Per-layer legend+opacity; translucent fills+strokes; distinct encodings; "N in view"; reset |
| F19 | No first-run orientation | P1 | ux | M | 2 | N | Dismissible coachmarks (define cluster/tiers; "click a dot"); localStorage |
| F20 | No value-prop headline | P1 | brand | S | 2 | **Y** | One hero line e.g. "Where the right retailers cluster — and where the white space is" |
| F21 | No JSON-LD/OG/Twitter cards | P1 | seo | S | 3 | **Y** | JSON-LD Dataset+Org(PointSav); OG hero image (EU frame); crawlable rm-top400 list |
| F22 | Inverted hierarchy at metro zoom (POIs shout) | P1 | carto | M | 2 | N | Demote raw POIs; promote cluster+active catchment; loudest red = selected only |
| F23 | No proportional symbol scaling | P1 | carto | M | 1 | N | `circle-radius` by √count, ~3 buckets, zoom-interpolated |
| F24 | No shareable per-site report/PDF | P1/P2 | ux | M/L | 3 | N | One-click branded one-pager (scorecard, catchment, demographics, competitors) |
| F25 | Spend chains 3 estimations, no error; MAUP at H3-7 | P1 | data | M | 1 | N | State per-capita assumption + MAUP in Method; avoid false-precision $ |
| F26 | No keyboard/screen-reader path | P1 | a11y | M | 1 | N | `role=region`+aria-label; aria-live; rm-top400 list as non-visual alternative |
| F27 | No deep-linkable URL state | P2 | ux | S | 3 | **Y** | Serialize center/zoom/region/chain/tier/layers to hash; "copy link to view" |
| F28 | DBSCAN params invisible; 3,765 = false precision | P1 | data | S | 1 | **Y** | Surface eps/minPts/IoU + sensitivity note in Method (sims swing NA 226–476) |
| F29 | Web-Mercator distortion vs km radii | P1/P2 | carto | S/M | 2 | partial | Compute radii/areas geodesically (turf); note projection caveat |
| F30 | "Regional Market = ≥1 co-location" too permissive | P1 | data | S | 1 | **Y** | Raise floor or rename "settlements with co-location presence"; explicit Top-400 criterion |
| F31 | External OpenFreeMap CDN = SPOF | P2 | trust | M | 4 | N | Self-host/cache positron style+glyphs+sprite; keep default, offer satellite/dark |
| F32 | Panel type/spacing flat; icons inconsistent | P2 | brand/ux | M | 2 | partial | Type scale (hero metric 2×), 8px system, ≥4.5:1, one icon family, strong toggle states |
| F33 | PointSav (developer) invisible | P2 | brand | S | 2 | **Y** | "Engineered by PointSav" footer + "X chains · Y countries · updated <date>" |
| F34 | Differentiated views (EU frame, catchment) buried | P2 | brand | S/M | 2 | partial | EU constellation as OG/splash; tease catchment with real number on ring |

## Quick localhost wins (S-effort, highest credibility/hour) — Phase 5 candidates

F1 legend · F8 dot halos + ordered ramp · F14 provenance line · F20 value-prop headline ·
F21 JSON-LD + OG cards · F27 deep-link URL state · F28/F30 Method-modal honesty edits ·
F33 "Engineered by PointSav" signature. (All edit the localhost `www/index.html` copy first.)

## Doc gaps → routing (Phase 4 follow-through)

- **TOPIC — Trade-Area Methodology** (revise A1): F6+F11+F29 — rewrite crow-flies→drive-time/observed; document radius derivation + geodesic caveat. → project-editorial.
- **TOPIC — Co-location Tiering & Scoring** (revise B1 + `gis-variable-distance-model`): F12+F5+F28 — rename QUALITY TIERS, define explainable score, publish DBSCAN params. → project-editorial.
- **TOPIC — Spend & Population Provenance** (revise A2): F25+F13+F14. → project-editorial.
- **TOPIC — Regional Market Definition** (revise `gis-regional-markets`): F30. → project-editorial.
- **TEXT — Data & Methodology Dialog** (verify A4 shipped not just dispatched): F14+F13.
- **DESIGN-RESEARCH — Cartographic Legibility System**: F2+F8+F18+F22+F23. → project-design.
- **DESIGN-RESEARCH/COMPONENT — Mobile Bottom-Sheet + First-Run**: F7+F19+F32 (pairs with `feedback-mobile-redesign-now`). → project-design.
- **DESIGN-RESEARCH — Site Scorecard / Compare / Export Inspector**: F5+F15+F24. → project-design.
- **BRIEF (here) — Delivery Re-Architecture**: F3+F4+F10+F31+F9 — PMTiles migration, inline-JS extraction, basemap self-host, metro-404 root-cause. Biggest diligence risk.
- **BRIEF (here) — White-Space & Cannibalization Model**: F16 — new analytic over Union-Find overlap.

## Next actions

1. **Phase 5 (operator go/no-go):** implement the S-effort quick wins on the localhost `www/` copy; re-screenshot to verify; keep prod (push-to-prod, nginx) Command-scope.
2. Spin the doc-gap items into the routed TOPIC/DESIGN-RESEARCH drafts.
3. Open the two engineering sub-BRIEFs (delivery re-architecture; white-space model).

---

## Implementation status & carry-on (updated 2026-06-22)

All work is on the **localhost deployment copy** `deployments/gateway-orchestration-gis-1/www/` (map = `index.html`;
research = `research*.html` + `lib/research-mobile.css`), served at `http://127.0.0.1:8900/` via the operator's
gcloud SSH tunnel. **Deployment-only** — the git-tracked source `pointsav-monorepo/app-orchestration-gis/www/`
is ~1,100 lines behind and NOT reconciled; a rebuild from git source would lose all of this. Nothing is public
(Command must run `push-to-prod.sh gis`). Vision harness: `~/sandbox/wiki-harness/gshot.mjs` + `cshot.mjs` against
`http://127.0.0.1:8900/`; range server `~/sandbox/gis-visual-audit/rangeserver.mjs` (see memory `reference-browser-in-the-loop`).
Wording spec: `~/sandbox/gis-visual-audit/COPY-SPEC.md`. Audit artifacts: `.agent/audits/map-ux-2026-06-20/`.

### DONE on localhost (verified)
- **Quick-wins (2026-06-21):** F1 legend, F8 white halos, F14 provenance→attribution, F21 JSON-LD/OG, F27 deep-link
  URL hash, F28/F30 method-modal honesty, F33 PointSav→home.pointsav.com link, F12 tier rename. F20 tagline (re-added).
- **Cartography (B1):** ordered CVD-safe Blues tier ramp (T1 `#08306B` / T2 `#3182BD` / T3 `#9ECAE1`) consistent
  across dots+legend+pills; proportional symbols (√count); low-zoom de-clutter (zoom-staged); demoted raw POIs at metro.
- **Mobile (F7):** draggable peek bottom-sheet (peek/half/full), compact 46px zoom buttons.
- **UX:** F17 chain search (counts/sections/aria), F19 first-run (now the branded card), F13 low-conf uncertainty render.
- **A11y/trust:** F26 ARIA + aria-live + crawlable Top-400 list + focus-visible; F29 rings already geodesic; F25 MAUP/spend caveat; F32 type/contrast.
- **Stretch:** F24 print one-pager, F5 scorecard summary line, F15 compare tray.
- **2026-06-22 build-out (6 phases):** (1) UF/Commuter bubbles unified to the Blues ramp (white strokes) — one tier
  reading across all 3 modes; (2) **Top-400 over ANY mode** — decoupled from the radio group, active bubbles recess
  to 0.45 (archetype opacity stored/restored in `_archetypeOrigOpacity`), `rm-stars` moved to top, amber legend row
  `#lg-top400`; (3) dropped "★ Regional Markets" label, toggle renamed **"Top 400 Regional Markets"**; (4) branded
  welcome card (`.cm-*`); (5) `/research` overhaul (sticky topbar ← Map + tabs + breadcrumb + prev/next; tokens synced;
  consolidated into `lib/research-mobile.css`); (6) overview tiers → 3-up `.tier-chip` grid, peek 214→176, trimmed BentoBox copy.
- Earlier: Top-400 ghost-shading bug fix.

### NOT complete / carry-on
- **[DONE 2026-06-22] Retail bubble/Top-400 parity + show-all gentle declutter.** Retail now shows all tiers at
  overview like UF/Commuter; then a paint-only declutter (white halos + circle-sort-key + zoom-graded alpha 0.78→1.0
  + ~18% low-zoom size taper) applied identically to all three modes — every dot kept (audit-defensible), legible,
  consistent; Top-400 recess(0.45)+amber overlay preserved; fixed `_restoreNodesIfClear` to restore staged opacity.
  Native clustering researched + REJECTED (hides individuals + strips tier/mc/cluster_id props). Also done same round:
  tagline → "Where retail, industrial, and transit anchors co-locate across North America and Europe" (the false
  "thirteen countries" swept from modal/JSON-LD/welcome — note JOURNAL itself is inconsistent 13↔18, flagged for
  project-editorial); sublabel "retail clusters"; Zero-Cookie posture (Disclaimer "Digital Infrastructure & Privacy
  Posture" + "∅ Zero Cookies" welcome badge); Data Policy routed to Command for factory-release-engineering.
- **[DONE 2026-06-22 session 3] Mobile sneak-peek + tagline round — all shipped to prod.**
  (a) Sneak-peek audit (Opus swarm): found mode label hidden at peek + sheet stuck at half after region switch.
  Fixed: `#bento-mode-label` moved into `.overview-peek-keep` block (f852fc7b); `delete panel.dataset.detent`
  before `setRadius` in `switchRegion` (same commit) → snaps to peek on region change.
  (b) Region tabs (NA/EU) hidden at peek → NA unclickable after Europe. Fixed: removed `.region-switch` from
  peek-hide CSS; compact tab styling (9px/3px 6px padding); PEEK_PX_OVERVIEW 176→182 (1c4b1fdb).
  (c) Peek height trimmed: stops at search bar — all `.data-block` hidden at peek (including `.overview-peek-keep`
  tier chips); PEEK_PX_OVERVIEW 182→152 (3f9b61c2).
  (d) Tagline iterated via 3× Opus agents: "Spatial measures of retail, industrial, and transit anchor clustering"
  restored (academic register; earlier iteration "Mapping clusters of retail, industrial, and transit" reverted by
  operator). CSS fix: `font-style: italic→normal` + `text-transform: none !important` (mixed case, not all-caps).
  Two-line break added via `<br>` after "transit" (3087fe56). All four changes pushed to prod individually.
- **deployment→git-source drift: RECONCILED** — git source and deployment copy are 0 lines diff (all session commits
  applied to both files; force-added via `git add -f` past the broad `pointsav-monorepo/` gitignore).
- **[DONE 2026-06-24] F15 compare badge bug fixed + F24 print upgraded to 2-page — PUSHED TO PROD.**
  (a) `removeFromCompare()` was missing the button-text sync that `addToCompare()` had → badge count
  never decremented on remove. Fixed: added `btn.textContent = compareSet.length ? \`＋ Compare (${compareSet.length})\` : '＋ Compare'`
  to `removeFromCompare()`.
  (b) Print was single-page; upgraded to 2-page: page 1 = stats panel, page 2 = map snapshot centred
  on cluster at auto-zoom (`calcPrintZoom`: `clamp(round(14.5 − log₂(max(1, span_km))), 11–15)`).
  Required: `preserveDrawingBuffer: true` added to MapLibre init (for `getCanvas().toDataURL()`);
  `#print-map-page` div placed AFTER stats in DOM (initial placement before stats produced 3 pages, not 2);
  `printOnePager()` rewritten as async: `flyTo → map.once('idle') → toDataURL → window.print()`.
  Pushed to prod 2026-06-24 via `push-to-prod.sh gis` (3.6 MB, nginx reloaded OK).
- **Deferred engineering (sub-BRIEFs, not auto):** F3 27MB→PMTiles, F4 N+1 catchment, F6 drive-time catchments,
  F16 white-space model, F31 basemap self-host, F10 inline-JS extraction.
- Editorial: the routed TOPIC/DESIGN-RESEARCH/TEXT drafts (A7 in artifact-registry) await project-editorial/project-design pickup.
