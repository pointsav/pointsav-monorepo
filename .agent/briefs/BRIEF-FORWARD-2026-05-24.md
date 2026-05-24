# BRIEF — Forward · Consolidated Project State and Roadmap

> **Date:** 2026-05-24
> **Replaces as daily reference:** the BUILD-SPEC, CENTRE-BUBBLE-RING, and all
> individual phase notes. The four research briefs (`BRIEF-VARIABLE-DISTANCE`,
> `BRIEF-CATEGORY-TAXONOMY`, `BRIEF-REGIONAL-MARKETS`, `BRIEF-HOLISTIC-REVIEW`)
> remain the authoritative rationale record; this file is what the build follows
> from here.
>
> **Where this disagrees with an older brief, this file wins.**

---

## §1 — Current verified state

| Item | Value |
|---|---|
| Last canonical commit | `f3856f96` — fix(gis): electronics category loop |
| Total co-locations | **6,493** |
| T1 Regional | **1,537** |
| T2 District | **3,090** |
| T3 Local | **1,866** |
| Electronics clusters | 1,061 (verified 2026-05-24) |
| layer2-clusters.pmtiles | 47.7 MB |
| Live URL | gis.woodfinegroup.com (nginx, TLS) |

**Phase 21 verified (2026-05-24):**
- Electronics pills rendering correctly (MediaMarkt DE/AT/NL/ES/GR/PL, Saturn DE,
  Boulanger FR, Darty FR, MediaWorld IT)
- Lifestyle pills rendering (XXXLutz AT/DE, Höffner DE)
- No regression in SE/FR (mediamarkt-se = 0 OSM records; xxxlutz-fr = 0 after
  cross-border filter; xxxlutz-se = 0 Overpass failure) — expected gaps, not bugs

**Known OSM / data gaps (Phase 22 backlog, not blocking):**
- mediamarkt-se, xxxlutz-se — OSM coverage gap; re-check in ≥60 days
- xxxlutz-fr — 27 stores all cross-border-filtered; revisit polygon filter logic
- xxxlutz CZ/SK/HU/SI/HR/BG/RO/CH — ISO_TO_CONTINENT not yet wired

---

## §2 — Settled architecture (do not reopen)

### 2.1 Data model

A **co-location** is a cluster of retailer category-anchors formed by **two-pass
tight-first DBSCAN**. Its geometry is its **centroid** (the mid-point) — never
an anchor pin. Three orthogonal axes:

- **Tier** — retailer-category composition only. No distance. No demand.
- **Distance rank** — geometric compactness, Stage 1 of ranking.
- **Demand rank** — catchment demand, Stage 2, layered on Stage 1.

A co-location belongs to one **Regional Market** (a municipality) which may
nest under one **Metro Market** (a major metro).

### 2.2 Tier rule — composition only

```
T1 Regional:
  H1  = (hypermarket ∧ hardware ∧ (price_club ∨ lifestyle))  [tight group]
  H2a = n_retail ≥ 3 distinct categories [any three retail anchors]
  H2b = tight_intact ∧ n_retail ≥ 3     [tight AND three categories — Phase 20]
  n≥4 = any four retail anchor categories

T1 geometric demotion → T3:  span_km < 1.25 ∧ member_count ≤ 2
T2 District:  hypermarket ∧ n_retail ≥ 2
T3 Local:     n_retail ≥ 2 ∧ ¬has_hypermarket
```

Sport does **not** enable T1. Sport adds: new T3 `{hypermarket,sport}`,
`{hardware,sport}`; new T2 `{hypermarket,hardware,sport}`.

### 2.3 Category taxonomy — 8, fixed (Phase 21)

**Retail anchors (6):**
`hypermarket` · `hardware` · `price_club` · `lifestyle` (IKEA) ·
`electronics` (MediaMarkt-class, large-format CE, ≥3,000 sqm) ·
`sport` (Decathlon-class, ≥3,000 sqm)

**Civic anchors (2, descriptor-only — never gate tier):**
`medical` (hospital, ≥150 beds) · `education` (university, enrolment threshold)

**Single authority for the category list: `taxonomy.py → all_chains_for_iso()`.**
`build-clusters.py`, `build-tiles.py`, and `index.html` must not carry
independent lists. The Phase 21 bug (`build-clusters.py` category tuple missing
`"electronics"`) was fixed in `f3856f96` — this is the canonical failure mode
to guard against.

### 2.4 Membership and distance

- Hard membership cap: **3.0 km max pairwise diameter**, uniform, not per-country.
- `tight_intact` = True when all members within ≤1.0 km of each other.
- `span_km` = max pairwise diameter (the distance measure).
- `dist_rank_in_tier` = inverted percentile of `span_km` within tier,
  shrinkage-blended Country + continent.
- Two-stage rank, **lexicographic** — never a weighted blend:
  `(tier, dist_rank_in_tier, demand_rank_in_tier)`.

### 2.5 Regional Market and Metro Market

- **Regional Market (RM)** = the incorporated municipal polygon the centroid
  falls in (CA GADM CSD / US TIGER place / MX municipio / EU GISCO LAU).
  Contains 1..N co-locations. Sherwood Park → Strathcona County — correct,
  accepted. No sub-municipal override — the CA Nominatim override is deleted.
- Dense-metro RM with > ~8 co-locations → geometric self-clustering into
  **Districts** (~8km cut, single-linkage) for wiki presentation only.
  No schema change, no breadcrumb level, no map layer.
- **Metro Market** = MSA/CBSA (US) / CMA (CA) polygon, from a published
  CBRE + Oxford Economics list. Context only, nullable, not shown in ring zoom.

### 2.6 Frontend authority model

- **Single `View` state object** — `{mode: OVERVIEW|RETAIL, selectedClusterId,
  dataLayer, simActive, region}`. One `setView(patch)` mutator. One `applyView()`
  that sets **all** layer visibilities. No other code calls `setLayoutProperty`.
  This closes the R1 bubble/ring race by construction.
- **Bubble ↔ ring swap** = one `setView()` call, one render frame.
- **Ring radius** = from `ring_radius_km` field in cluster props (1.0 km if
  T1-tight, else 3.0 km). Index.html never hard-codes radii.
- **BentoBox pill order (canonical):**
  anchor (alpha) → hardware → price_club → lifestyle → electronics → sport → civic footnote
- **Ring UX (current, post-5960ae18):**
  first click → ring-overview z12/z11 (T1/T2-T3), 1200 ms;
  second click same ring → street level z14/z13, 800 ms.

---

## §3 — In-flight: AEC Site Conditions build (Nights 2–5)

Night 1 = Phase 19 rebuild — COMPLETE (2026-05-24 09:10Z).

| Night | UTC | Script | Output | Status |
|---|---|---|---|---|
| **2** | 2026-05-25 05:00 | `build-aec-climate-solar.sh` | layer8-ashrae, layer8-necb, layer8-eu-climate; GHI patch on clusters-meta | **SCHEDULED** (crontab set 2026-05-24) |
| **3** | 2026-05-26 05:00 | `build-aec-koppen-ecozones.sh` | layer9-koppen, ecoregions | script NOT YET WRITTEN |
| **4** | 2026-05-27 05:00 | `build-aec-seismic.sh` | layer10-seismic, wetlands | script NOT YET WRITTEN |
| **5** | 2026-05-28 05:00 | `build-aec-flood.sh` | layer11-flood, wildfire | script NOT YET WRITTEN |

⚠ **Night 2 note:** `NREL_API_KEY` unset → solar GHI step skipped. Climate zone
tiles build regardless. Register at `developer.nrel.gov` if GHI needed.

⚠ **Before Night 5:** verify ≥35 GB free. Submit EFFIS wildfire data request:
`https://forest-fire.emergency.copernicus.eu/applications/data-and-services`
(GWIS FWI raster is the automatic fallback).

New clusters-meta.json fields per night:

| Field | Night |
|---|---|
| `ashrae_zone` · `necb_zone` · `eu_climate_zone` · `ghi_kwh_m2_yr` | 2 |
| `koppen_class` · `ecoregion_name` · `ecoregion_biome` · `epa_l3_ecoregion` | 3 |
| `seismic_pga_g` · `wetland_class` | 4 |
| `flood_hazard` · `wildfire_hazard` | 5 |

Full AEC detail (sources, URLs, per-country coverage table):
`.agent/AEC-NIGHTLY-BUILD-PLAN.md` (canonical runbook).

---

## §4 — Prioritised build roadmap

### Track A — Data model fixes (prerequisite for S1 sign-off)

Status audit (2026-05-24): A1, A2, A3, A5 were already implemented in Phase 19.
The research briefs that prescribed them predate the implementation. Outstanding
items are A4 and the three pipeline scripts below.

| ID | Change | File | Status |
|---|---|---|---|
| **A1** | Feature geometry = centroid, not anchor pin | `build-clusters.py:440` | **DONE** — `[clon, clat]` centroid; `seed_lat/lon` retains anchor |
| **A2** | Emit `tier`, `tight_intact`, `span_km` | `build-clusters.py` | **DONE** — all three fields in schema |
| **A3** | Emit `ring_radius_km` | `build-clusters.py` | **DONE** — `1.0` if tight_intact else `3.0` |
| **A4** | Sync `build-tiles.py build_clusters_meta()` to §2 schema | `build-tiles.py:344` | **OUTSTANDING** — drop `rank_2km`; lon/lat = centroid |
| **A5** | Two-pass DBSCAN: one feature = one co-location (graph component) | `build-clusters.py` | **DONE** — centroid-based `cluster_id`; dedup anchors; Phase 19 |

Outstanding pipeline scripts:
- **`build-geometric-ranking.py`** — compute `dist_rank_in_tier` (inverted
  percentile, shrinkage-blended country+continent). Currently 0.0 placeholder.
- **`build-regional-markets.py`** (NEW) — group co-locations by `regional_market`
  → rebuild `regional-markets.json` (~3,011 RMs). Point-in-polygon via
  RegionEngine.
- **Alberta S0 prototype** — validate the full pipeline on AB-only subset before
  any global S2 overnight build. Sherwood Park must resolve to 3 co-locations
  in Strathcona County. S1 operator sign-off gates S2.

**`taxonomy.py` single-authority rule:** `all_chains_for_iso()` is the sole loop
in `build-clusters.py` (migrated 2026-05-24, commit pending). The Phase 21 bug
class (hardcoded category tuple missing a category) is now closed.

### Track B — Frontend single-authority refactor (after A1–A3 land)

Changes to `www/index.html`. Can be built and tested on Alberta sim before S2.

| ID | Change |
|---|---|
| **B1** | Replace 1km/3km toggle with `ring_radius_km` from cluster props; delete `.radius-selector`, `setRadius()`, `currentRadius` |
| **B2** | Single zoom-state authority: `View` object + `setView()` + `applyView()`. Delete scattered `setLayoutProperty` calls. Closes R1 bubble/ring race. |
| **B3** | Feature geometry at centroid (not anchor pin) for ring/bubble rendering |
| **B4** | Delete `setRetailLevel()` and the "all-locations viewport-wide" paint path. Scope retailer dots to inside selected co-location's ring only. |
| **B5** | Delete `showMergedGroupPanel()` and the 3km Union-Find merge. Honest overlapping rings, one per co-location. |
| **B6** | BentoBox two-view: `showMarketDetail()` (RM parent, 2+ co-location markets) + co-location detail. 3-level breadcrumb: Overview → Regional Market → Co-location. |
| **B7** | Bubble sizing by tier (T1 largest), not `score_final`. Rings: solid layer T1, dashed T2–T3. Delete "All Locations" bento toggle. Delete "DBSCAN Sim" button or debug-flag it. |

### Track C — Demand layer (Stage 2, after geometric S1 sign-off)

Not yet started. Implement `build-demand-ranking.py` and the O-D ingests.

| ID | Item |
|---|---|
| **C1** | UK: ONS Census 2021 ODWP01EW — 7,200 MSOAs → H3 res-7 |
| **C2** | FR: INSEE RP2021 FD_MOBPRO — 35,000 communes → H3 res-7 |
| **C3** | DE: BA Statistik Pendler Gemeinde — Kreis ~400 → H3 res-7 |
| **C4** | `build-demand-ranking.py` — Huff gravity model on WorldPop × catchment; `demand_rank_in_tier` field |
| **C5** | Final ranking: `(tier, dist_rank_in_tier, demand_rank_in_tier)` lexicographic |
| **C6** | `generate-top400.py` rewrite — demand-layer product; NA slice + EU slice; RM column |

Research complete for UK/FR/DE — see `work/od-data-research-uk-fr-de.md`.
US LODES and ES MITMA are already ingested. Paid mobility providers (Advan,
Huq, Spectus, Replica, StreetLight) evaluated and deferred to Stage 2+ —
see `memory/paid-mobility-providers.md`.

### Track D — Phase 22 chain expansion (parallel to other tracks)

| Chain | Status |
|---|---|
| Best Buy US/CA | Counter-factual first: does it co-locate with existing anchors? |
| XXXLutz CZ/SK/HU/SI/HR/BG/RO/CH | Add to `ISO_TO_CONTINENT`; per-country YAMLs |
| mediamarkt-se | OSM gap; re-check ≥2026-07-24 |
| xxxlutz-se | Overpass query failure; re-check ≥2026-07-24 |
| xxxlutz-fr | Cross-border filter revisit |
| Meijer US | From chain-coverage-audit.md |
| Bodega Aurrera MX | From chain-coverage-audit.md |

---

## §5 — Holistic review — additive, does not block geometric build

Four-agent cross-check (urban-land · CRE · GIS · economics), 2026-05-22.
Unanimous: the geometric build (§2–§4 Track A+B) is sound — freeze it.
The additive layer (below) gates the wiki/editorial leg only.

**7.1 Economic-indicator block** — each RM record needs (from open data, dated + sourced):
- Population + 5-yr CAGR; median household income; estimated retail spend potential.
- Built-up-area share (Kontur/WorldPop/GHSL) — maturity vs opportunity axis.
- Anchor supply per 10,000 residents — saturation / white-space signal.
- Parent Metro Market economic context.

**7.2 Framing discipline:**
- Tier = anchor composition, NOT market grade. Highest tiers are often most saturated.
- Municipality is the publishing unit, not the functional economy.
- Do not surface demand rank outside US/ES until O-D data is broader.
- State the methodology line unmissably: the platform sizes and locates markets,
  it does not price real estate.

**7.3 Input-provenance layer (required before S2 canonical publish):**
Make `DATA-MANIFEST.md` a pipeline output: dated OSM extract, per-chain ingest
method + count, boundary-file vintages, `taxonomy.py` version.
Convert flagship `name_query` ingests to structured IDs (Esselunga, Sklavenitis,
Continente, Billa Plus, Albert Heijn XL) — non-reproducible ingest under a
flagship slot is the top integrity risk.

---

## §6 — Editorial / publication pipeline

All drafts dispatched to project-editorial under commit `fe5148fd` (2026-05-16):

| Type | Artifact | Status |
|---|---|---|
| TOPIC | O-D Catchment Methodology | at project-editorial |
| TOPIC | Trade Area Data Sources | at project-editorial |
| TOPIC | Catchment Ranking Methodology | at project-editorial |
| TEXT | Data Methodology Dialog (gis.woodfinegroup.com) | at project-editorial |
| GUIDE | Pipeline Rebuild (Phase 1/2) | at project-editorial |
| TOPIC | POI Data Schema | at project-editorial |
| GUIDE | Adding a Chain (appendix added) | at project-editorial |
| DESIGN-RESEARCH | BentoBox Merged Zones — implemented 2026-05-17 | at project-design |
| DESIGN-RESEARCH | Location Intelligence UX | at project-design |
| DESIGN-RESEARCH | Ring Retailer Click UX | at project-design |
| DESIGN-RESEARCH | Tier Naming Accessibility | at project-design |
| DESIGN-RESEARCH | Zoom Prefetch Pattern | at project-design |

**Gates:** RM TOPIC drafts (`generate-rm-topics.py`) gate on Track A + economic-indicator block (§5.7.1).
AEC Site Conditions TOPIC gates on Night 2+ tile delivery.

---

## §7 — Standing rules

| Rule | Detail |
|---|---|
| **Overnight builds** | Start after 05:00 UTC (22:00 Vancouver PDT); finish before 16:00 UTC. All heavy builds (rebuild.sh, phase-rebuild, AEC nights). |
| **Commit** | `~/Foundry/bin/commit-as-next.sh "<message>"` — staged files only, no `git add -A`. |
| **Promote** | `~/Foundry/bin/promote.sh` from Command Session. From Totebox: `FOUNDRY_COMMAND_SESSION=1 ~/Foundry/bin/promote.sh`. After rebase, force-push staging mirrors with `FOUNDRY_CONFIRM_DESTRUCTIVE=1`. |
| **Category loop** | Any script loading chains must use `taxonomy.py → all_chains_for_iso()`. Never maintain an independent tuple. The Phase 21 bug (`f3856f96`) is the canonical failure mode. |
| **Tier is composition** | Do not reopen distance-based tier gate discussions. Closed 2026-05-22. |
| **NREL API key** | Needed for solar GHI in AEC Night 2+. Register at `developer.nrel.gov`. Without it, climate zones build but GHI is skipped. |
| **EFFIS wildfire** | Submit formal data request before 2026-05-27. GWIS FWI is the automatic fallback. |

---

## §8 — Research-record cross-reference

| Topic | Source |
|---|---|
| Zoom / bubble / ring, R1 bug, Track A/B | `.agent/briefs/BRIEF-CENTRE-BUBBLE-RING-2026-05-21.md` |
| Tier=composition, span_km, two-stage rank | `.agent/briefs/BRIEF-VARIABLE-DISTANCE-2026-05-21.md` |
| 8 categories, per-country BRAND_FILL | `.agent/briefs/BRIEF-CATEGORY-TAXONOMY-2026-05-22.md` |
| Regional Market, Metro Market, RM wiki | `.agent/briefs/BRIEF-REGIONAL-MARKETS-2026-05-22.md` |
| Holistic review — economic + provenance layer | `.agent/briefs/BRIEF-HOLISTIC-REVIEW-2026-05-22.md` |
| Sport as 6th retail anchor — T2/T3 rationale | `.agent/briefs/BRIEF-ADD-SPORT-CATEGORY-2026-05-23.md` |
| T2→T3 rebalance, London fix, sport agents | `.agent/briefs/BRIEF-SPORT-REBALANCE-LEAPFROG-2026-05-23.md` |
| AEC 5-night runbook (full sources + URLs) | `.agent/AEC-NIGHTLY-BUILD-PLAN.md` |
| UK/FR/DE O-D data sources | `work/od-data-research-uk-fr-de.md` |
| 150-chain taxonomy census | `work/taxonomy-census-2026-05-22.md` |
| Edmonton / Sherwood Park calibration | `work/edmonton-area-colocations.md` |
| Phase 22 chain gap analysis | `work/chain-coverage-audit.md` |
| Paid mobility provider evaluation | `memory/paid-mobility-providers.md` |
| Build master (pre-Phase-20) | `.agent/briefs/BRIEF-BUILD-SPEC-2026-05-22.md` |
