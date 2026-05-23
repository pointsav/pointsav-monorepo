# BRIEF — Master Build Specification

> **Consolidated, supersession-free build spec · 2026-05-22.**
> This is the **executable plan** — pick it up and run the new build at any time.
> It consolidates the settled decisions from the four research briefs:
> `BRIEF-CENTRE-BUBBLE-RING-2026-05-21` · `BRIEF-VARIABLE-DISTANCE-2026-05-21` ·
> `BRIEF-CATEGORY-TAXONOMY-2026-05-22` · `BRIEF-REGIONAL-MARKETS-2026-05-22` ·
> `BRIEF-HOLISTIC-REVIEW-2026-05-22`. Those remain the research record
> (rationale, agent reports, supersessions).
> **This file is what the build follows.** Where the two disagree, this file wins.
>
> **Status: Phase 19 IMPLEMENTATION COMPLETE (2026-05-23); Phase 19 build
> scheduled tonight (PID 2507282, 05:00 UTC 2026-05-24).** Phase 18 produced
> 5,702 clusters (T1=1,157/T2=4,283/T3=262). Phase 19 adds sport as 7th category,
> geometric T2→T3 split, n≥4 T1 rule, London fix; expected output tonight:
> T1=1,157 / T2=2,889 / T3=1,656 (committed `a2c974e4`, pwoodfine).
> AEC layers building across 5 nights — see §10. Stage 6 needed for commits
> `a2c974e4`, `9886d9fa`, `e1792934`, `34a48183`, `382d5576`.
>
> **Remaining work:**
> - ✓ fix `nodes` circle-radius expression (`score_final` → `tier`) — DONE
> - ✓ §8 artifact-first TOPIC/GUIDE/DESIGN drafts — DISPATCHED `fe5148fd`
> - §4 frontend: BentoBox two-view (`showMarketDetail`); retire ring-merge
>   panel (`showMergedGroupPanel`); retire `drillIntoCluster` legacy path;
>   retire layer3-radius layers
> - §3 step 11: `generate-rm-topics.py` (one TOPIC per RM)
> - §3 step 2/3: `region_engine.py` Nominatim override delete + `metro-markets.json`
> - layer1-locations.pmtiles rebuild (new chains post-Phase-18)
> - §7.1: economic-indicator block for RM records (gates wiki leg)
> - §10 AEC: Nights 2–5 scripts to be written before each window; see §10

---

## 0. Execution handoff

To run the build from this spec (fresh session — start here):

1. **§8 first — artifact-first.** Produce the `TOPIC-*` / `GUIDE-*` methodology
   drafts → project-editorial and `DESIGN-*` → project-design.
2. **Then §3 — the 12-step pipeline.** New scripts: `taxonomy.py`,
   `build-regional-markets.py`, `build-demand-ranking.py`, `generate-rm-topics.py`;
   rewrites: `build-clusters.py`, `generate-top400.py`.
3. **§7 is additive** — the economic-indicator + provenance layer for the RM
   record / wiki leg; it does **not** gate the geometric build.

**S0 already done — committed, live in the Alberta sim** (`simulate-dbscan-ab.py`
+ `www/index.html`): two-pass tight-first DBSCAN · corrected composition tier
rule · `span_km` / `dist_rank_in_tier` · centre-dot removed · hysteresis zoom.
**S0 remaining:** create `taxonomy.py`; prototype `build-regional-markets.py` and
the `generate-*` rewrites on Alberta; `cross-check-taxonomy.py`; the §6 data fixes.

**S2** = the production multi-country rebuild — a **heavy build; run only in the
overnight window** (start after 05:00 UTC, finish before 16:00 UTC) per the
standing rule. All §6 build-gating decisions are settled.

---

## 1. The data model (settled)

A **co-location** is a cluster of retailer category-anchors formed by **two-pass
tight-first DBSCAN**. Its geometry is its **centroid** (the mid-point) — never an
anchor pin. It has three orthogonal axes:

- **Tier** — retailer-category **composition only** (no distance, no demand).
- **Distance rank** — geometric compactness, Stage 1 of ranking.
- **Demand rank** — catchment demand, Stage 2, layered on Stage 1.

A co-location belongs to one **Regional Market** (a municipality) which may
nest under one **Metro Market** (a major metro).

### 1.1 Categories — 7, fixed (Phase 19)
`hypermarket` · `hardware` · `price_club` · `lifestyle` (IKEA) · `sport`
(Decathlon-class, ≥3,000 sqm) — retail anchors;
`medical` · `education` — civic anchors. Civic categories are **not** in the
tier gate (descriptor / demand-stage only). Sport is a T2/T3 enhancer — it
**does not enable T1** (see §1.2). No other categories.

### 1.2 Tier rule — composition only (Phase 19)
- **T1 Regional** = `(hypermarket ∧ hardware ∧ (price_club ∨ lifestyle))` **OR**
  `n_retail ≥ 4` (any four retail anchor categories)
- **T1 geometric override → T3**: `span_km < 1.25 ∧ member_count ≤ 2` demotes
  to T3 (compact 2-anchor clusters — a quality signal, not a Regional cluster)
- **T2 District** = `hypermarket ∧ n_retail ≥ 2` (catches hardware, lifestyle,
  sport combinations)
- **T3 Local** = `n_retail ≥ 2 ∧ ¬has_hyper`
- singletons → not a co-location. **Three tiers.** Sport adds new T3 combos
  `{hypermarket, sport}`, `{hardware, sport}` and new T2 `{hypermarket, hardware,
  sport}` — it does NOT enable T1 on its own.

### 1.3 Membership & distance
- **Hard membership cap = 3.0 km** max pairwise diameter, uniform — *not*
  per-country. **1.0 km = `tight_intact`** flag (quality, not a second gate).
- `span_km` = max pairwise diameter. `dist_rank_in_tier` = inverted percentile
  of `span_km`, **within tier**, shrinkage-blended Country + continent.
- Two-stage rank, **lexicographic**: `(tier, dist_rank_in_tier, demand_rank_in_tier)`
  — never a weighted blend.

### 1.4 Regional Market & Metro Market
- **Regional Market** = the incorporated municipal polygon the centroid falls in
  (CA GADM admin-3 CSD · US TIGER place · MX municipio · EU GISCO LAU). The
  **product unit**. Contains 1..N co-locations. Rural co-locations resolve to
  their containing municipality. **No sub-municipal override** — Sherwood Park
  resolves to "Strathcona County" and that is correct and accepted.
- **Metro Market** = the MSA/CBSA (US) / CMA (CA) polygon, **filtered to a
  published list** (CBRE Econometric Advisors + Oxford Economics). Context only,
  nullable, never shown in the co-location/ring zoom.

---

## 2. Schema — per co-location

Emitted by `build-clusters.py` → `clusters.geojson` → `clusters-meta.json` → PMTiles.

| Field | Meaning |
|---|---|
| `cluster_id` | centroid-derived: `co_{iso}_{clat5}_{clon5}` — anchor-independent |
| geometry | the **centroid** (mid-point); keep anchor pin as `seed_lat/lon` |
| `tier` | 1/2/3 canonical (composition rule §1.2) |
| `tier_descriptor` | human composition string |
| `span_km` · `tight_intact` | geometric measure + ≤1 km flag |
| `dist_rank_in_tier` · `dist_pctile` | Stage-1 geometric rank |
| `ring_radius_km` | 1.0 if T1-tight else 3.0 (frontend reads this, never hard-codes) |
| `demand_rank_in_tier` · `demand_basis` | Stage-2 (`demand_basis`: `od-...` vs `catchment-35-150`) |
| `regional_market` | slug `rm_{iso}_{settlement}` — the RM join key |
| `market_name` · `market_region` | RM display + metro context string |
| `metro_market` | published-list metro, `""` if none |
| `mkt_conf` | geocode precision (high/medium/low) — **never a ranking** |
| `members[]` · `member_count` | member stores; each member carries its `category` |

`regional-markets.json`: one record per RM — `rm_id, market, iso, region,
mkt_conf, cluster_count, cluster_ids[], centroid, best_tier, metro_market`.

---

## 3. Build sequence — pipeline

Run in order. New files marked **(NEW)**; rewrites **(REWRITE)**.

1. **`taxonomy.py`** (NEW) — single declarative taxonomy: `CATEGORIES` (6,
   NAICS-keyed) · `BRAND_FILL[category][country]` with #1/#2 slots (per-country
   table, `BRIEF-CATEGORY-TAXONOMY` §10) · `THRESHOLDS` (hospital beds = 150;
   university enrolment) · `DISPLAY_COUNTRIES` (17, grouped NA / UK / Nordics /
   Continental Europe) · `category_of()` · `tier_of()` · `slots_for()`.
2. **`region_engine.py`** — **delete** the Nominatim override (override branch,
   `cluster_id` param of `resolve_market()`, `ca_places_nominatim.json` load).
   Add **`resolve_metro()`** — MSA/CBSA/CMA lookup gated on the published list.
3. **`metro-markets.json`** (NEW) — committed catalogue of CBRE + Oxford
   Economics metros with their MSA/CBSA/CMA polygon IDs.
4. **`ingest-osm-civic.py`** — **attribute-first** civic filter: NAICS gate
   (622110 hospital / 611310 university) → bed-count / enrolment threshold →
   name only as tiebreak. Wire ETER (EU) + HESA (UK) enrolment.
5. **`build-clusters.py`** (REWRITE) — two-pass tight-first DBSCAN; geometry =
   centroid; emit the full §2 schema; centroid-based `cluster_id`; import
   `taxonomy.py`; call `resolve_market()` + `resolve_metro()` on each centroid.
6. **`build-geometric-ranking.py`** — Stage 1: compute `dist_rank_in_tier`
   (percentile within tier, shrinkage-blended). **Strip the population / spend /
   IoU demand gates out of the tier predicates** — tier is composition only.
7. **`build-regional-markets.py`** (NEW) — group co-locations by
   `regional_market` → rebuild `regional-markets.json` on the new co-locations.
8. **`build-demand-ranking.py`** (NEW) — Stage 2: `demand_rank_in_tier`. Interim
   = 35/150 km catchment population; target = O-D primary/secondary catchments
   (US LODES summary already exists; ES MITMA). Per-cluster `demand_basis`.
9. **`build-tiles.py`** — `build_clusters_meta()` carries every §2 field into
   `clusters-meta.json`; rebuild `layer2-clusters.pmtiles`.
10. **`generate-top400.py`** (REWRITE) — **demand-layer** product. Sort
    `(tier, dist_rank_in_tier, demand_rank_in_tier)`, slice `[:400]` for NA and
    for Europe (= UK+Nordics+Continental). Each row carries the Regional Market
    column. Drop the Apex/Hub/Core/Valid star bands.
11. **`generate-rm-topics.py`** (NEW) — one TOPIC per Regional Market →
    `.agent/drafts-outbound/` → project-editorial. An RM with > ~8 co-locations
    is partitioned into District sections by geometric self-clustering (§6.4).
12. **`index.html`** — frontend, §4.

---

## 4. Frontend — `www/index.html`

- **Single `View` / `setView()` / `applyView()` authority** — every layer's
  visibility derived from `View`; no scattered `setLayoutProperty`. This closes
  the R1 bubble/ring race by construction.
- **Bubble ↔ ring** — bubbles (tier-sized) at global zoom, rings at local;
  per-co-location radius from `ring_radius_km`; **no centre dot**; **hysteresis**
  — rings persist on zoom-out down to z 6.
- **BentoBox** — two views: `showMarketDetail()` (Regional Market parent —
  hero + co-location list; conditional, only for 2+ co-location markets) and
  the co-location detail view. 3-level breadcrumb Overview → Regional Market →
  Co-location.
- **One rank** — the **geometric `dist_rank`** (within tier / country), shown as
  a single minimal chip. **Not Top 400** — Top 400 is a demand-layer product.
  Delete the 4-slot placeholder rank grid.
- **Retire** the 1 km/3 km radius toggle, the ring-merge / `showMergedGroupPanel`,
  the dead `layer3-radius` layers, the "All Locations" bento toggle.
- Non-anchor retailers (Lidl/Aldi/discount grocery) off the default map —
  anchors-only default layer + optional co-tenant toggle.

---

## 5. S0 / S2 staging

**S0 — Alberta test bed (build now, no overnight job).** Prototype every
algorithm in `simulate-dbscan-ab.py` + the sim frontend, plus `taxonomy.py` and
`cross-check-taxonomy.py`. Already partly live: two-pass tight-first DBSCAN,
corrected tier rule, `span_km`/`dist_rank`, centre-dot removal, hysteresis zoom.
Remaining S0: create `taxonomy.py`; retarget the sim to import it; prototype
`build-regional-markets.py` + the `generate-*` rewrites on Alberta.

**S2 — production (overnight build, start after 05:00 UTC).** The
`build-clusters.py` rewrite + the full §3 pipeline across all 17 countries;
cluster-ID migration (anchor-pin IDs → centroid IDs — invalidates persisted
IDs); full `layer2`/`clusters-meta`/PMTiles rebuild; `regional-markets.json`
rebuilt; Top 400 + RM TOPIC drafts generated.

**Tier projection at current data coverage** (`work/tier-projection.py`):
**T1 = 1,044 · T2 = 3,609 · T3 = 1,421 · total 6,074 co-locations.**

---

## 6. Operator decisions

### Settled — 2026-05-22 (operator)
1. **Tiers — three (T1 / T2 / T3). No T4/Fringe.** A co-location is ≥2 retail
   categories; below that it is not a co-location. Confirmed.
2. **Headline geometric rank — emit both, display within-tier.** The build emits
   `dist_rank_in_tier` *and* a within-country variant (cheap, schema-neutral);
   the BentoBox shows within-tier. Display swappable later with no rebuild.
3. **Pre-freeze data-quality fixes — do all three before S2:** split the
   `ikea-nordics` blob into SE/DK/NO/FI (else all four Nordic lifestyle slots
   wrongly read empty); drop stale `lowes-ca` (1 store — Lowe's left Canada);
   re-ingest `coop-forum-se` (1 store, severe under-ingest).

4. **Dense-metro Regional Market granularity — geometric self-clustering.**
   No external layer meets the global test: CBRE/Oxford publish no usable
   sub-metro geography (CoStar-licensed, US-only retail, broker-drawn), and no
   global sub-city administrative boundary exists (the §11 conclusion repeats).
   **Rule:** a Regional Market TOPIC is published whole *unless* it holds more
   than ~8 co-locations — then its co-locations are partitioned into **District
   sections** by single-linkage agglomerative clustering of their own centroids
   (~8 km cut). It is a **wiki-presentation rule** living in `generate-rm-topics.py`
   (§3 step 11) — no schema change, no map layer, no breadcrumb level, no change
   to `region_engine.py`. Districts named geometrically (ordinal + quadrant),
   never curated. Calibrate N and the cut on Chicago/Madrid in S0. Gates the
   wiki leg only.

### Deferred — gates the demand stage, not this build
5. **Interim demand scalar** — population only vs population + spend. Decide when
   `build-demand-ranking.py` (Stage 2 / the demand layer) is built.

---

## 7. Holistic review — the record needs a decision layer

Four-agent holistic cross-check (urban-land · CRE · GIS · economics), 2026-05-22.
**Unanimous: the geometric build (§1–§5) is sound — freeze it, do not re-open
it.** All four found the same mission-level gap: the design measures
**geometry + supply** (where big-box retail already agglomerated — a *lagging*
indicator); the mission — the authoritative public Regional Market repository —
requires **geometry + supply + demand-side economics + provenance.** The fix is
**additive** — it does not block the geometric build — but the **wiki/editorial
leg must not ship a Regional Market TOPIC without it.**

### 7.1 The Regional Market record needs an economic-indicator block
Today an RM record is a retail-cluster index entry. Each RM record needs a
standard economic profile — open data (ACS / StatCan / Eurostat), dated + sourced:
- Population + **5-yr population CAGR** (trend, not just level).
- Median household income + trend; estimated retail spend potential.
- **Built-up-area share** (from Kontur/WorldPop/GHSL rasters already ingested) —
  the maturity-vs-opportunity axis the tier cannot provide.
- **Anchor supply per 10,000 residents** — the saturation / white-space signal,
  derivable from data already in hand. Highest-leverage single addition.
- Parent Metro Market economic context, attributed.

### 7.2 Framing discipline — what the wiki / UI must say plainly
- **Tier = anchor-category composition, NOT market grade or opportunity.** The
  highest tiers are often the *most saturated*. Co-location count is a
  saturation signal as much as an opportunity signal. State this unmissably.
- **The municipality is the publishing unit, not the functional economy.**
  Catchment/demand figures are functional-ring measures — label them
  "catchment, not municipal."
- **Do not surface the demand rank outside US/ES** — it is interim ambient
  population elsewhere; badge the method on every figure.
- **Do not fake what cannot be sourced** — the platform sizes and locates
  markets; it does not price real estate. State the line in the methodology TOPIC.

### 7.3 Input-provenance layer — required before S2
Make `DATA-MANIFEST.md` a **pipeline output**: dated OSM extract, per-chain
ingest method (`wikidata` vs `name_query`) + count, boundary-file vintages,
`taxonomy.py` version. Convert the country-flagship `name_query` ingests
(Esselunga, Sklavenitis, Continente, Billa Plus, Albert Heijn XL) to structured
IDs — a non-reproducible ingest under a flagship slot is the top integrity risk.
Publish a per-country coverage-and-confidence statement. The platform earns
"authoritative" by being the **most transparent** sub-metro dataset, not the broadest.

### 7.4 Stays settled — do not re-open
6 categories · 3 tiers · two-pass DBSCAN · centroid geometry · `span_km` ·
municipal RM resolution · override deletion · district self-clustering. The
geometric build (§3 steps 1–12) freezes as written. §7.1–7.3 are an **additive**
economic + provenance layer, scoped to the RM record and the wiki leg.

---

## 8. Build start — artifact-first

The platform's product *is* the public repository — the wiki and the guides.
**The build begins by completing the editorial and design artifacts** routed to
the other `project-*` repos (drafts → `.agent/drafts-outbound/`; bilingual
EN+ES for TOPIC/GUIDE per workspace CLAUDE.md §14; `foundry-draft-v1`
frontmatter + five research-trail fields):

- **TOPIC-\*** → project-editorial — methodology TOPICs: co-location method ·
  category taxonomy · Regional Market & Metro Market definition · the two-stage
  ranking · the coverage-and-confidence statement (§7.3) · the unit-choice
  methodology note (§7.2). Then, after the build runs, the per-Regional-Market TOPICs.
- **GUIDE-\*** → project-editorial — operational guides: reading the platform ·
  adding a chain/country · deployment.
- **DESIGN-\*** → project-design — the BentoBox two-view redesign · the
  bubble/ring visual system.
- **CODE / SCRIPT / CONFIG / DATA-\*** — committed directly in this archive (the §3 pipeline).

---

## 9. Research-record cross-reference

| Topic | Research brief |
|---|---|
| Zoom / bubble / ring, R1 bug, View authority | `BRIEF-CENTRE-BUBBLE-RING-2026-05-21.md` |
| Tier=composition, span_km, two-stage rank, DBSCAN | `BRIEF-VARIABLE-DISTANCE-2026-05-21.md` |
| 7 categories, per-country BRAND_FILL, tier projection | `BRIEF-CATEGORY-TAXONOMY-2026-05-22.md` |
| Holistic cross-check — economic/provenance layer, framing discipline | `BRIEF-HOLISTIC-REVIEW-2026-05-22.md` |
| Regional Market, Metro Market, wiki, Top 400, BentoBox, override | `BRIEF-REGIONAL-MARKETS-2026-05-22.md` |
| Sport as 7th category — T2/T3 only rationale, BRAND_FILL | `BRIEF-ADD-SPORT-CATEGORY-2026-05-23.md` |
| Tier rebalance (T2→T3 split, n≥4 T1), sport agent analysis | `BRIEF-SPORT-REBALANCE-LEAPFROG-2026-05-23.md` |
| AEC layers — nightly build plan (full detail) | `.agent/AEC-NIGHTLY-BUILD-PLAN.md` |
| AEC parity research — EU regulatory flood, eco-regions, CONABIO | `.agent/AEC-DATA-PARITY-RESEARCH.md` |

Artifacts: `work/taxonomy-census-2026-05-22.md` (150-chain census) ·
`work/tier-projection.py` (tier counts) · `work/edmonton-area-colocations.md`.

---

## 10. AEC / Site-Conditions Build Plan

Five-night staged rollout of AEC and site-conditions layers for gis.woodfinegroup.com.
All nights fire at **05:00 UTC** (22:00 Vancouver PDT). Scripts live in
`pointsav-monorepo/app-orchestration-gis/`.
Full detail including step-by-step download sources and per-country URL list is in
`.agent/AEC-NIGHTLY-BUILD-PLAN.md` (canonical) and `.agent/AEC-DATA-PARITY-RESEARCH.md`
(source + licence table). This section is the BRIEF-level backup.

### 10.1 Schedule

| Night | UTC date | Script | What | Status |
|---|---|---|---|---|
| 1 | 2026-05-24 05:00 | `nightly-rebuild.sh` + `phase19-rebuild.sh` | Sport chains + London fix + geometric T2→T3 | **SCHEDULED** PID 2507282 |
| 2 | 2026-05-25 05:00 | `build-aec-climate-solar.sh` | ASHRAE 169 (US) + NECB (CA) + EU regulatory climate zones + NSRDB/PVGIS solar GHI | script to write |
| 3 | 2026-05-26 05:00 | `build-aec-koppen-ecozones.sh` | Köppen-Geiger 2018 global + Resolve Ecoregions 2017 (CC BY 4.0) + EEA Biogeo EU + EPA L3 US + PVGIS EU solar | script to write |
| 4 | 2026-05-27 05:00 | `build-aec-seismic.sh` | USGS NSHM (US) + NRCan (CA) + ESHM20 EU seismic PGA + GWL_FCS30 global wetland | script to write |
| 5 | 2026-05-28 05:00 | `build-aec-flood.sh` | WRI AQUEDUCT global + FEMA NFHL SFHA (US) + EU regulatory flood (EA/Géorisques/SNCZI/IdroGEO + INSPIRE WFS) + GWIS/EFFIS wildfire | script to write |

⚠ **Before Night 5:** verify ≥35 GB free (`df -h /srv/foundry`); submit EFFIS
wildfire data request at
`https://forest-fire.emergency.copernicus.eu/applications/data-and-services`
(GWIS FWI raster is the automatic fallback if not approved in time).

### 10.2 New PMTiles layers (gateway `tiles/`)

| Layer file | Content | Coverage | Night |
|---|---|---|---|
| `layer8-ashrae-zones-us.pmtiles` | ASHRAE 169 / IECC county zones | US | 2 |
| `layer8-necb-zones-ca.pmtiles` | NRCan NECB HOT2000 zones | CA | 2 |
| `layer8-eu-climate-zones.pmtiles` | National building-code zones (build-by-join) | FR/ES/IT/DE/GR/PT/FI/PL/SE | 2 |
| `layer9-koppen-global.pmtiles` | Beck et al. 2018 Köppen-Geiger 1km | All 16 ISOs | 3 |
| `layer13-ecoregions-global.pmtiles` | Resolve Ecoregions 2017 (CC BY 4.0) | All 16 ISOs | 3 |
| `layer14-biogeographic-eu.pmtiles` | EEA Biogeographical Regions 2016 | EU (Habitats Directive ref) | 3 |
| `layer14-ecoregions-us.pmtiles` | EPA Level III ecoregions | US precision | 3 |
| `layer10-seismic-na.pmtiles` | USGS NSHM 2023 + NRCan 2015 PGA | US + CA | 4 |
| `layer10-seismic-eu.pmtiles` | ESHM20 PGA raster | EU all ISOs | 4 |
| `layer11-flood-global.pmtiles` | WRI AQUEDUCT 3.0 1-in-100yr riverine | Global (CA background) | 5 |
| `layer12-fema-sfha-us.pmtiles` | FEMA NFHL SFHA zones | US regulatory precision | 5 |
| `layer12-flood-eu-regulatory.pmtiles` | EU Floods Directive shapefiles (per-country) | GB/FR/ES/IT + INSPIRE WFS | 5 |
| `layer15-wildfire-global.pmtiles` | GWIS FWI or EFFIS (if approved) | Global | 5 |

Total estimated disk delta after Night 5: ~1.35 GB.

### 10.3 New clusters-meta.json fields

| Field | Source | Night |
|---|---|---|
| `ashrae_zone` | PNNL county→zone CSV | 2 |
| `necb_zone` | NRCan HOT2000 MapServer | 2 |
| `eu_climate_zone` | Build-by-join national codes + GISCO LAU2 | 2 |
| `ghi_kwh_m2_yr` | NREL NSRDB (US/CA/MX) / PVGIS (EU) | 2+3 |
| `koppen_class` | Beck 2018 global | 3 |
| `ecoregion_name` · `ecoregion_biome` | Resolve 2017 global | 3 |
| `epa_l3_ecoregion` | EPA Level III (US only) | 3 |
| `seismic_pga_g` | USGS/NRCan/ESHM20/CENAPRED | 4 |
| `wetland_class` | GWL_FCS30 30m global (CC BY 4.0) | 4 |
| `flood_hazard` | AQUEDUCT + FEMA (US) + EA/Géorisques/SNCZI/IdroGEO (EU) | 5 |
| `wildfire_hazard` | GWIS FWI or EFFIS | 5 |

### 10.4 Country coverage (regulatory-grade where bold)

| ISO | Code zone | Solar GHI | Seismic | Flood | Ecoregion |
|---|---|---|---|---|---|
| US | **ASHRAE 169** | NSRDB | **USGS NSHM** | **FEMA NFHL** + AQUEDUCT | EPA L3 + Resolve |
| CA | **NECB** | NSRDB | **NRCan** | Future Flood Susceptibility + AQUEDUCT | Resolve |
| MX | INEGI raster (CONABIO CC BY-NC blocked) | NSRDB | CENAPRED (conditional) | AQUEDUCT | Resolve |
| GB | HadUK-Grid ref | PVGIS | ESHM20 | **EA Flood Map OGL** | Resolve + EEA Biogeo |
| FR | **RE2020** | PVGIS | ESHM20 | **Géorisques TRI 2020** | Resolve + EEA Biogeo |
| DE | TRY 2017 | PVGIS | ESHM20 | LAWA WFS | Resolve + EEA Biogeo |
| ES | **CTE DB-HE** | PVGIS | ESHM20 | **SNCZI T100** | Resolve + EEA Biogeo |
| IT | **DPR 412/1993** | PVGIS | ESHM20 | **IdroGEO PAI** | Resolve + EEA Biogeo |
| PL | WT 2021 | PVGIS | ESHM20 | INSPIRE WFS | Resolve + EEA Biogeo |
| NL/DK/NO/FI/GR/PT/SE | National code (where applicable) | PVGIS | ESHM20 | INSPIRE WFS | Resolve + EEA Biogeo |

**Bold** = regulatory-grade (not modelled proxy).

### 10.5 Parity decisions

- **EU flood is regulatory-grade** for GB/FR/ES/IT (direct national shapefiles).
  All remaining EU ISOs ingest via EU Floods Directive INSPIRE WFS. This matches
  FEMA-quality for those markets — significantly above AQUEDUCT-only.
- **Canada has no FEMA equivalent** — FHIMP national programme runs 2024–2028.
  Using NRCan Future Flood Susceptibility 2024 (XGBoost model, OGL-Canada) as proxy.
- **CONABIO (MX eco-regions/climate) is CC BY-NC** — blocked for commercial use.
  Substitute: INEGI climate raster for NMX-C-460 zone join; Resolve 2017 for eco-regions.
- **EFFIS wildfire** requires formal data request — GWIS FWI raster is the automatic
  fallback. Both paths are handled in `build-aec-flood.sh`; the script chooses based
  on presence of the EFFIS file at run time.

### 10.6 After Night 5 — frontend wiring (separate session)

Not a nightly build. Requires an interactive coding session on `index.html`:

1. MapLibre source + layer entries for all 13 new PMTiles layers
2. **"Site & Hazard" collapsible group** in the layer controls panel
3. **BentoBox "Site Conditions" section** per cluster — displaying:
   - Climate Zone (ASHRAE / RE2020 / CTE / Köppen class)
   - Solar GHI (kWh/m²/yr)
   - Ecoregion (Resolve 2017 biome name)
   - Seismic PGA (g)
   - Flood Exposure (zone label per source)
   - Wetland Class (GWL_FCS30 or None)
4. **Wind/snow:** ATC Hazards API point-lookup on cluster click (no tile layer needed)
