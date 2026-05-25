# BRIEF — Centre / Bubble / Ring Co-location Model

> **Status:** DRAFT for operator review | **Date:** 2026-05-21
> **Author:** project-gis Totebox Session, synthesizing four Opus analysis agents
> (UI design · UX design · system architecture · GIS data-layer)
> **Supersedes nothing — unifies:** `RING-HIERARCHY-DESIGN-2026-05-20.md`,
> `VISUAL-DESIGN-SYSTEM-2026-05-20.md`, `DBSCAN-TRIANGULATION-REDESIGN-2026-05-20.md`,
> `GIS-UX-REDESIGN-2026-05-21.md`, `TODO.md` (R1, D1–D5, V1–V15).

---

## 1. Purpose

Define one coherent model for how a **co-location** is rendered across zoom
levels on gis.woodfinegroup.com, and identify every gap between that model and
the code as it stands. This BRIEF is the decision document; implementation is
gated on the operator decisions in §7.

---

## 2. The target model (operator, 2026-05-21)

1. **Co-location** = a cluster of Tier-defined retailer categories that come
   together within a threshold distance. Each co-location has a **mid-point** —
   the geometric centre of the participating retailers. That mid-point **is**
   the co-location's *"centre"*.

2. **Global zoom** (zoomed out) — the "co-location zoom": each co-location
   renders as a **bubble**, sized and coloured by **Tier 1-2-3**. Bubbles sit
   on the co-location mid-point.

3. **Local zoom** (zoomed in): the bubble is replaced by a **ring**, centred on
   the same mid-point. The mid-point — the *"centre"* — carries a small dot or
   nothing at all. Rings carry **Tier 1-2-3-4**. Ring **radius varies per
   co-location** — some 1 km, some 3 km.

4. The **"all retailers"** view at local zoom is **not a user option** and is
   removed from the bento box.

---

## 3. Headline finding

**The target model already exists in working code — in the Alberta DBSCAN
simulation, not in production.** `showSimDetail()` (`www/index.html:2214`)
draws exactly one co-location ring per cluster, at the DBSCAN centroid, with
`coKm = tier===1 ? 1 : 3`. The production path (`drillIntoCluster`,
`setRetailLevel`, `updateProximityRings`) still runs the legacy logic:
merge-rings, a global radius toggle, and an always-on "all retailers" layer.

The work is therefore largely **promotion of the sim path to production +
removal of legacy machinery**, plus three data-model fixes (§5).

---

## 4. Current state — what is actually built

### 4.1 Bubbles (global zoom) — `nodes` layer
- Bubble **radius** interpolates `score_final`, **not tier** — two Tier-1
  co-locations render at different sizes. (`index.html:1833-1836`)
- Bubble **colour** is tier-keyed, but reads a `tier` field that **does not
  exist in the data** (see §5.1) — it resolves to the `?? 4` fallback.
- `nodes` filter `tier<4` excludes Tier 4 from bubbles — correct for the
  "bubbles = 1-2-3" requirement.

### 4.2 Rings (local zoom) — `proximity-line` / `proximity-fill`
- Ring colour/weight/opacity keyed on **`rank`** (3 classes: 3 / 2 / else),
  not **`tier`** (target needs 4 classes). T3 and T4 collapse together.
- All rings **solid** — no dash axis (`index.html:1716`, "incompatible with
  data-driven colour").
- Radius is a **single global `currentRadius`** (1 or 3 km), not per-co-location.
- At 3 km, `groupOverlappingClusters()` (Union-Find, `index.html:1248`)
  **merges overlapping clusters into one ring drawn at the mean of their
  centroids** — a synthetic mid-point belonging to no co-location. This
  directly contradicts target item 1.

### 4.3 The centre dot — `cluster-centroid`
- Drawn **only on the click path** (`drillIntoCluster:2311`), never when the
  user simply zooms in. So "centre marked by a dot" holds on one entry path
  and not the other.
- Uses the TIER_COLORS palette (navy/green/gold/maroon) while rings/bubbles
  use a blue ramp — two palettes for the same object.

### 4.4 The "all retailers" view
- The bento **"All Locations" toggle** (`index.html:1133`) only renders in the
  overview inspector — it is unreachable once drilled in.
- But `all-locations` (every store, every chain, viewport-wide) is
  **force-shown on every entry to local zoom** (`setRetailLevel:1176`,
  `drillIntoCluster:2327`, zoom-back `:2421`). "All retailers at local zoom" is
  the built-in, non-optional behaviour — not a toggle. Removing the bento
  button is half the job; the always-on paint must also be scoped to the
  selected co-location.

### 4.5 The radius selector
- The "1 km / 3 km" floating pill (`index.html:291-294`, `setRadius:773`) is a
  live global control that re-merges and re-draws every ring at a uniform
  radius — actively corrupting the per-co-location model.

---

## 5. Root-cause findings

### 5.1 DATA — no canonical `tier`, mid-point discarded, no stored radius
*(GIS data-layer agent)*

- **No `tier` field is emitted anywhere.** `build-clusters.py` writes
  `rank_1km` / `rank_3km` as a pair; `clusters-meta.json` never emits `tier`.
  `index.html` reads `c.tier ?? 4` and `p.tier ?? 4` — **tier rendering is
  currently running on the `?? 4` fallback constant.** `build-tiles.py:344`
  even reads a `rank_2km` key that `build-clusters.py` never produces.
- **The mid-point is computed and then thrown away.** `build-clusters.py:344-353`
  computes `centroid_lat/lon` as the unweighted mean of the anchor + commercial
  members — geometrically a correct mid-point. But `build-clusters.py:547`
  publishes the feature geometry at the **anchor store pin**, not the centroid.
  Bubbles and rings are off-centre by the anchor-to-centroid offset.
- **No per-co-location ring radius exists in the data.** Radius lives only as
  the client-side global `currentRadius`.
- A co-location today is **anchor-centric, not a graph cluster** — one feature
  per anchor store, with no dedup. True "one feature = one co-location"
  requires the DBSCAN rewrite (`DBSCAN-TRIANGULATION-REDESIGN`).

### 5.2 ARCHITECTURE — no single zoom-state authority
*(system architecture agent)*

- View state is an **implicit 4-tuple of globals** (`overviewActive`,
  `selectedClusterId`, `catchmentActive`, `simActive`) re-derived ad hoc by
  every handler. No state enum, no reducer.
- `nodes` visibility is written by **4** functions; `proximity-*` by **6**.
  There is no function whose job is "given state, set all layer visibilities."
- `setRetailLevel()` has a non-idempotent early-return (`if (!overviewActive)
  return`); `drillIntoCluster` duplicates its layer toggles inline but skips
  the guard. The `zoomend` handler — the de-facto state machine — has **no
  branch** for the plain drilled-in state.

### 5.3 R1 BUG — bubbles don't disappear at z≥9
*(system architecture agent — diagnosis)*

Not one bug; a **race between competing writers**:
- (a) `setRetailLevel`'s early-return aborts the whole bubble-hide if any other
  function set `overviewActive=false` first — but the ring-show path still runs.
- (b) Async `setData` re-tiling + late `sourcedata`/`idle`/`switchRegion`
  events can run `showOverview()`, which re-asserts `nodes` `visibility:visible`
  *after* the user is already at z≥9.
- (c) `circle-opacity:0` is used as a hide mechanism, but
  `applyClusterBubbleStroke`, `clearChainFilter`, `showOverview`, and
  `setRadius` all write `circle-opacity` back to `0.88`.

The "opacity=0 + visibility=none dual-kill" and the reverted `maxzoom:9` fix
each patched **one** writer; the others remained. **The bug is structural and
will not close without the single-authority refactor in §6.2.**

### 5.4 UX — the model is two mechanisms that disagree
*(UX design agent)*

The bubble and the ring are two unrelated render paths. The bubble sits at the
anchor pin; the ring is a separately generated GeoJSON circle at a possibly-
merged mid-point — so the ring frequently does **not** appear where the bubble
was. The swap is a hard cut with no cue that "this ring is that bubble." The
merge invents mid-points and forces a `showMergedGroupPanel` disambiguation
list that exists only to undo the merge.

---

## 6. Recommended approach

### 6.0 Staging strategy — Alberta first, then full re-build (operator, 2026-05-21)

The model is **proven on Alberta before any global re-build.** Alberta is the
test bed because the correct co-location model already lives in the Alberta
DBSCAN sim (§3) — it only needs the new UI/UX wired onto it.

| Stage | Scope | Gate |
|---|---|---|
| **S0 — Alberta test bed** | Run DBSCAN on Alberta only; render bubbles / rings / centre dot with the new model in the sim surface; validate co-location correctness *and* UI/UX. Cheap, fast, no overnight build. | — |
| **S1 — Operator sign-off** | Operator reviews Alberta: are the co-locations right? does the bubble→ring→centre interaction work? | **Blocks S2.** |
| **S2 — Full re-build** | Promote the validated model to production: Track A (all 13 countries, DBSCAN rewrite) + Track B (production `index.html`). Overnight builds. | S1 sign-off |

This means **Track B (frontend) is exercised on Alberta first** — the
single-authority refactor and the bubble/ring/centre rendering can be built and
tested against the Alberta sim data without touching the production pipeline.
Track A's quick-fix (A1–A4) and DBSCAN rewrite (A5) run **globally only at S2**,
after Alberta proves the model. Do **not** start the global re-build until S1.

Two tracks support this. Track A (data) and Track B (frontend) are independent;
both are needed for the full model, but both are validated on Alberta at S0
before going global at S2.

### 6.1 Track A — data model (build-clusters.py + build-tiles.py)

| # | Change | Effort |
|---|---|---|
| A1 | Publish feature geometry at the **centroid**, not the anchor pin (`build-clusters.py:547`). Keep the pin as `seed_lat/lon`. | 1-line + rebuild |
| A2 | Emit a **canonical `tier`** field (collapse `rank_1km`/`rank_3km`; recommend `tier = rank_3km`, plus a `tight_intact` flag). | small |
| A3 | Emit **`ring_radius_km`** per co-location (`1.0` if T1-tight else `3.0`). | small |
| A4 | `build-tiles.py` `build_clusters_meta()`: add `tier`, `ring_radius_km`; drop the bogus `rank_2km` read; make `lon/lat` = centroid. | small |
| A5 | *(Full model)* DBSCAN rewrite so one feature = one co-location, eliminating duplicate anchor clusters. | per `DBSCAN-TRIANGULATION-REDESIGN` |

A1–A4 are a **quick-fix bundle** that delivers "bubble/ring at the mid-point"
+ real tier data **without** waiting for DBSCAN. A5 is the full fix.
All builds are heavy → **overnight rule applies** (see TODO.md standing rule).

### 6.2 Track B — frontend (www/index.html)

**B1 — single zoom-state authority.** Introduce one `View` object
(`mode: OVERVIEW|RETAIL`, `selectedClusterId`, `dataLayer`, `simActive`,
`region`), one `setView(patch)` mutator, and one `applyView()` that derives
**every** layer's visibility from `View`. No other code calls
`setLayoutProperty(...'visibility'...)`. This **closes R1 by construction** —
`nodes` is hidden by exactly one statement evaluated from `View.mode`.

**B2 — delete `setRetailLevel`.** `RETAIL` always implies a selection. The
only way into `RETAIL` is `drillIntoCluster` → `setView({mode:'RETAIL',
selectedClusterId})`. The "all retailers, no selection" sub-state is removed
(target item 4).

**B3 — bubble→ring swap** = one `setView()` call → one `applyView()` → one
render frame. Drop the `circle-opacity:0` hide writes entirely. Keep the swap
instant (camera-threshold transitions read as instant); selection-driven
changes may carry a 150 ms ease.

**B4 — per-co-location radius.** Retire `currentRadius` as a render input;
`updateProximityRings` builds each circle with `ringKmForTier(tier)` (or reads
`ring_radius_km` from A3). Delete the `.radius-selector`, `setRadius()`, and
the dead `radius-fill`/`radius-line` layers.

**B5 — remove the merge.** One ring per co-location at its true mid-point.
Honest overlapping rings replace synthetic merged mid-points; `showMergedGroupPanel`
is deleted.

**B6 — visual spec.** Bubbles sized **by tier** (T1 largest), not `score_final`.
Rings carry **4 tier classes** — split `proximity-line` into a solid layer
(T1, 1 km) and a dashed layer (T2-4, 3 km), each filtered by tier/radius.
Centre dot: harmonise to the blue ramp; show on **both** zoom and click entry
paths; small (2.5–4 px) or suppressed for the tight T1 ring (operator, OD-7).

**B7 — bento cleanup.** Remove the "All Locations" toggle, the radius selector,
and the "DBSCAN Sim" button (debug-flag it). Scope `all-locations` store dots
to **inside the selected co-location's ring**, not viewport-wide. Replace the
overview "Layers" block with a Tier 1/2/3 legend + counts.

---

## 7. Operator decisions required

Implementation is blocked until these are settled. D1–D5 from `TODO.md` are
folded in.

| ID | Decision | Recommendation |
|---|---|---|
| **OD-1** | **Mid-point definition** — is the unweighted mean of anchor + *commercial* members (civic POIs excluded) the correct "centre"? | Accept as-is; it is a true geometric mid-point. |
| **OD-2** | **Zoom-in with no selection** — when a user zooms past z9 without clicking a bubble, the app should: (a) stay in overview / bubbles, (b) show bare retailer dots, no rings, (c) auto-select the nearest co-location and show its ring. | **(c)** — keeps the model coherent; `RETAIL` always has a selection. |
| **OD-3** | **Canonical tier** — when a cluster is T2 at 1 km but T3 at 3 km, which is "the tier"? | `tier = rank_3km`; carry `tight_intact` as a separate flag. |
| **OD-4** | **Ring radius source** — strictly tier-derived (T1→1 km, else→3 km), or data-driven from actual member spread? | Tier-derived now; revisit member-spread later (needs a new `member_span_km` field). |
| **OD-5** | **Tier 4** — target says rings carry T1-4 but bubbles carry T1-3. Confirm T4 co-locations get a ring at local zoom but **no** bubble at global zoom. | Confirm; `nodes` keeps `filter tier<4`. |
| **OD-6** | **Merged rings** — drop the 3 km merge (honest overlapping rings, one per co-location) vs keep it. | **Drop** — the merge invents mid-points and contradicts item 1. |
| **OD-7** | **Centre dot** — always-on dot / dot for T2-4 only / nothing. | Dot on all tiers for transition legibility; smaller for T1. |
| **OD-8** | **Sequencing** — *confirmed:* Alberta test bed (S0) → operator sign-off (S1) → global re-build (S2). See §6.0. | Operator-set 2026-05-21. |
| **D2** | Vignette opacity 0.20 / 0.22 / 0.25 | validate on Alberta sim |
| **D3** | 150 km / 35 km outer ring solid vs dashed | per `RING-HIERARCHY-DESIGN` |
| **D4** | Data-horizon arc label copy | — |
| **D1** | Path C T1 qualification (+~199 US T1) — independent of this BRIEF | separate track |

---

## 8. Provenance

Four Opus analysis agents, 2026-05-21, all read-only:
UI/visual design · UX/interaction design · system architecture · GIS data-layer.
Full per-agent reports retained in this session's transcript. Files examined:
`www/index.html` (2546 lines), `build-clusters.py`, `build-tiles.py`,
`config.py`, and the four prior plan files this BRIEF unifies.
