# DBSCAN Triangulation Redesign — Cluster + Tier Architecture

**Owner:** project-gis Totebox session (architecture); operator (gate decisions).
**Drafted:** 2026-05-20 from two deep-think Opus sessions.
**Status:** Architecture approved by operator; implementation not yet started.
**Relationship to prior plan:** This document supersedes the _geometric/IoU_ sections of
`tier-scoring-overhaul-2026-05-16.md`. The taxonomy (4-class), tier names (G1), bento
layout, and EU ingest backlog from that plan all remain valid and carry forward here.

---

## 1. Executive summary

The current system is anchor-centric: every cluster is seeded by one anchor store, a
Haversine ring is drawn from that anchor's pin, and tier is decided by what falls inside.
This creates three structural problems: (a) the cluster center is the anchor pin even when
the anchor sits at the periphery of the actual commercial zone; (b) two nearby anchors
produce overlapping clusters that require a post-hoc IoU dedup pass; (c) the 1km/3km
distinction is a UI toggle rather than a first-class analytical concept.

The replacement is a **DBSCAN proximity graph model**: pool all qualifying retailers as
equal nodes, connect nodes within a proximity threshold τ, extract connected components,
compute the geometric centroid of each component, and draw the catchment ring from that
centroid. Tier is determined by the composition of the component (which classes of
retailer converge) and the tightness of the convergence (whether the component holds
together at τ_tight = 1km vs τ_loose = 3km). Non-overlap is a geometric invariant — each
store belongs to exactly one component — so the entire IoU dedup pass in
`build-geometric-ranking.py` becomes dead code.

Simultaneously, the tier targets are revised: ~1,000 T1 clusters per region (NA and EU
separately), ~3,000–4,000 total tiered clusters per region, with all remaining
co-locations rendered as a non-tiered "All Retail Proximity Zones" rings layer that
retires the T4/Fringe bucket.

---

## 2. Why the current model is wrong geometrically

| Problem | Current behaviour | New behaviour |
|---|---|---|
| Cluster center | Anchor pin — even if anchor is at the edge of the commercial zone | Unweighted centroid of all participating stores |
| Overlap | Two nearby anchors → two overlapping clusters → IoU dedup pass patches it | Each store belongs to one component; overlap is structurally impossible |
| Ring origin | Drawn from anchor pin | Drawn from centroid (build-radius.py auto-follows; no code change needed) |
| 1km/3km | UI toggle — "which question did you mean?" | Tier discriminator — T1 requires τ_tight, T2/T3 allow τ_loose |
| T4 / "lonely stores" | Anchor with no qualifying secondary; counted, shown, confusing | Isolated graph node; rendered as a ring-layer singleton; not a tier |

---

## 3. Algorithm: anchor-centric rings → DBSCAN proximity graph

### Current algorithm (build-clusters.py lines 236–274)

```
for each region:
    for each anchor_cid:                         # O(A) anchor instances
        for each store of that anchor:
            query 3km ring via 0.1° grid
            evaluate_tier(what's inside ring)
            centroid = mean of commercial stores  # computed but NOT used as ring origin
            emit cluster keyed on anchor pin
```

### New algorithm

```
for each region:
    nodes = all qualifying stores (anchor ∪ hardware ∪ warehouse)
                                                 # canonicalized via CHAIN_FAMILIES
    build proximity graph: edge if haversine(a,b) ≤ TAU_LOOSE_KM (3km)
                                                 # using existing query_grid_with_dist
    components = union_find(nodes, edges)        # same pattern as frontend groupOverlappingClusters
    for each component:
        sub_groups = split_to_diameter(component, TAU_LOOSE_KM)
                                                 # prevents chain-runs; greedy, deterministic
        for each sub_group:
            centroid = unweighted mean(member lat/lon)
            tight_intact = component holds under TAU_TIGHT_KM (1km) edges only
            tier = evaluate_tier_geometric(sub_group, classes_present, tight_intact)
            emit one cluster; geometry = centroid
```

Key properties:
- Iteration count = number of spatial components, strictly ≤ number of anchors.
- Complexity stays O(N) — graph build via existing 0.1° grid; Union-Find near-linear;
  diameter-split O(m²) per component where m << 10 in practice.
- The diameter-split step is **mandatory** — without it, a chain of stores each 2.9km
  apart merges into one giant multi-km cluster with a meaningless centroid.
- For deterministic builds, the greedy split seeds on the highest-class store, then
  absorbs nearest qualifying neighbours within τ. Tie-break: chain_id alphabetical.

### Centroid method

**Unweighted arithmetic mean of member lat/lon.** At 1–3km scale the planar mean
differs from the spherical centroid by centimetres; the 3D Cartesian formula is
unnecessary. The function already exists in build-clusters.py lines 344–353; it is
currently computed but not used as the ring origin — it just needs to become
authoritative.

Do NOT use circumcenter (undefined for N≠3, diverges for collinear stores) or
convex-hull centroid (adds complexity, negligible accuracy gain at this scale).

Size-weighting deferred to v2 — no floor-area field in JSONL records today.

---

## 4. Does the 1km/3km toggle survive?

**No. It collapses into the tier definition.**

Changing τ changes which connected components *exist* — you cannot toggle it at view
time. A 1km graph and a 3km graph are different graphs with different node memberships
and different centroids. "1km or 3km view?" becomes "T1 or T2/T3?" — the distance
distinction is preserved, just promoted from a UI control into the first-class tier
definition.

What survives as a UI control: a **ring-radius display preference** (draw the visual
catchment ring at 1/3/5km) — cosmetic map styling, decoupled from tier membership.
This must not be confused with the old toggle.

---

## 5. Revised tier architecture

### Tier definitions

| Tier | Name (G1) | Proximity gate | Composition | Civic | Signal |
|---|---|---|---|---|---|
| **T1** | Regional | All members mutually ≤ **1km** (`tight_intact = True`) | ≥3 chains, ≥2 _distinct owners_, Hypermarket ∧ (Hardware ∨ Warehouse) | hospital ≤ 5km | Proven power node — competitive / defensive site play |
| **T2** | District | Connected ≤ **3km**; fails 1km test | Hypermarket ∧ (Hardware ∨ Warehouse) | hospital ≤ 5km | Established corridor — densification play |
| **T3** | Local | Connected ≤ **3km** | ≥2 distinct chains, any class | none | Multi-anchor adjacency, not yet a district |
| **Rings** | All Retail Proximity Zones _(not a tier)_ | ≥1 qualifying store, any τ | ≥1 qualifying store, incl. singletons | — | Complete retail substrate; tiers are the curated subset |

### T4 / "Fringe" is retired

T4 = "anchor with no qualifying secondary" was never a real tier; it was the absence of
one. Under the new model, isolated-graph-node stores become ring-layer singletons — they
exist, are shown, but are not part of the tier ranking. The T4 bucket (8,540 entries
today) dissolves entirely. The T4 / Fringe name from the 2026-05-16 plan is no longer
needed.

### Distinct-owner counting

The composition predicate uses **distinct ultimate owners**, not distinct chains. Sam's
Club + Walmart share an owner (Walmart Inc.); counting them as two distinct anchors for
a T1 composition claim is double-counting one real-estate decision. Add `CHAIN_OWNERS`
dict to config.py:

```python
CHAIN_OWNERS = {
    "sams-club-us": "walmart-inc",
    "walmart-us":   "walmart-inc",
    "walmart-ca":   "walmart-inc",
    "walmart-mx":   "walmart-inc",
    # etc.
}
```

The "≥3 chains, ≥2 distinct owners" rule prevents a single operator's three banners
in one plaza from reading as a multi-anchor T1.

---

## 6. Count targets and how to reach them

### Targets

| Tier | NA target | EU target | Current (global) |
|---|---|---|---|
| T1 Regional | ~950–1,050 | ~900–1,000 | 435 (global) |
| T2 District | ~1,400 | ~1,500 | 1,602 |
| T3 Local | ~1,300 | ~1,000 | 3,080 |
| **Tiered total** | **~3,650** | **~3,400** | 5,117 |
| Rings layer (all) | full graph | full graph | ~8,540+ singletons |

Per-region tiered totals (~3,650 NA, ~3,400 EU) are both inside the operator's
3,000–4,000 band. Global tiered ≈ 7,050. The rings layer covers ~9,000–12,000
components total.

### How T1 reaches ~1,000/region (no gate-loosening)

**Lever A — Delete the non-compositional gates** _(biggest single jump, costs nothing)_

The `rank_pp_iso ≤ p10` percentile gate and `iou_equal_circles` IoU filters in
`build-geometric-ranking.py` currently discard ~75% of compositionally-valid T1s.
Both gates are artifacts of the anchor-centric model:
- IoU exists to clean up overlapping anchor-instance clusters — structurally impossible
  under DBSCAN; it is dead code.
- The p10 percentile gate was a thinning mechanism to compensate for over-generation —
  also unnecessary when each store belongs to exactly one component.

Removing them recovers real clusters that always passed the composition test.
Estimated effect: NA 348 → ~700–800; EU 87 → ~250.

**Lever B — Finish the EU hardware-chain ingest backlog** _(gating dependency for EU T1)_

The config.py TODO block (lines 124–145) lists chains with zero ingested records due
to sparse OSM brand:wikidata tagging: OBI (~640 stores, DE+IT+PL), toom (~340 DE),
Hagebau (~350 DE), Bauhaus DE/ES, Brico Dépôt, Bricocenter, gamma-nl, karwei-nl.

EU is not structurally smaller than NA — it is **under-ingested**. Once each country's
market-leading hardware chain is ingested and promoted to ALPHA_HARDWARE, EU T1 becomes
reachable at the 1,000-cluster target.

Estimated effect post-ingest: EU ~250 → ~700–900.

**Lever C — Breadth rule: ≥3 chains across ≥2 classes** _(replaces rigid pair gate)_

Replace the rigid `(Warehouse ∧ Hypermarket)` pair requirement with:
`≥3 distinct chains (by owner), ≥2 classes, must include Hypermarket ∧ (Hardware ∨ Warehouse)`.

This _raises_ the bar (a lone Hypermarket+Warehouse pair no longer auto-qualifies)
while correctly capturing genuine 3-anchor power centres that the old binary rule missed.
Net NA effect: ~+100–150.

**Rejected levers:**
- Relax τ to 3km for T1 — a 3km-spanning cluster is a District, not a Power Centre;
  that is exactly what T2 is for. This would manufacture fake signal.
- Lifestyle/electronics as qualifying secondaries (Best Buy, Target) — informative as
  an info-card enrichment but not a gate criterion.
- Count Walmart sub-banners separately — textbook over-count; CHAIN_FAMILIES already
  canonicalizes them.

### Honest flag on EU numbers

The EU T1 ≈ 1,000 target is **contingent on completing Lever B**. Until
OBI/Bauhaus/toom/Hagebau et al. are ingested, EU T1 realistically lands ~400–500.
Do not tune τ or percentiles to hit the number with today's data. Report EU T1 ≈ 1,000
as planned/intended (BCSC posture) until the data lands.

---

## 7. The rings layer

### What it is

The full DBSCAN graph output **before tier filtering** — every connected component
including singletons. It is `clusters.geojson` itself; the T1/T2/T3 set is a filtered
subset of it.

### What it is not

Not a T0. Not "Unranked." Calling it that implies it sits on the same ranking ladder as
T1–T3, which invites "why is my site T0 and not T3?" It is the complete retail
substrate; tiers are the curated highlights.

### UI treatment

- **Layer name:** "All Retail Proximity Zones" (bilingual per workspace rules)
- **Visual style:** thin, low-opacity neutral-grey rings; no tier colour; no rank label
- **Tier rings render on top** in their tier colours
- **User-facing explanation** (for the "Data" modal, artifact A4):
  > "Every place two qualifying stores sit near each other is shown as a ring. Tiers 1–3
  > are the subset our methodology ranks as investment-grade co-locations. A ring without
  > a tier badge is a real co-location that did not meet tier criteria — most often a
  > single anchor, or anchors too dispersed to form a district."

### Pipeline output

A new `layer7-proximity-zones.pmtiles` built from the full untiered component set.
`layer2-clusters.pmtiles` filtered to `tier ∈ {1,2,3}` only.

`build-clusters.py` currently drops anchor-only components at line 273:
`if not any(ranks.values()): continue` — remove this to emit all components.

---

## 8. Exact code changes

### config.py

```python
# Add alongside existing SECONDARY_RADIUS_KM = 3.0
TAU_TIGHT_KM       = 1.0   # T1 proximity threshold (component must hold under this)
TAU_LOOSE_KM       = 3.0   # T2/T3 proximity threshold (= current SECONDARY_RADIUS_KM)
COMPONENT_SPLIT_KM = 3.0   # Max component diameter before greedy split

# New — distinct-owner collapse for composition predicate
CHAIN_OWNERS: dict = {
    "sams-club-us": "walmart-inc",
    "walmart-us": "walmart-inc",
    "walmart-ca": "walmart-inc",
    "walmart-mx": "walmart-inc",
    # ... extend for all sub-banner relationships
}
```

Work the hardware-ingest TODO backlog (Lever B): ingest OBI/Bauhaus/toom/Hagebau/
Brico Dépôt/Bricocenter/gamma-nl/karwei-nl, then promote each country's market-leader
from GENERIC_HARDWARE to ALPHA_HARDWARE.

The 4-class taxonomy from `tier-scoring-overhaul-2026-05-16.md §4` carries forward
unchanged (ALPHA_HYPERMARKET / ALPHA_WAREHOUSE / ALPHA_LIFESTYLE / ALPHA_HARDWARE).

### build-clusters.py

**Primary rewrite — replace lines 236–353.**

Remove:
- The `for anchor_cid in region_chains[region]['anchor']: for pri in locs_by_cid[anchor_cid]:` double-loop (lines 236–239)
- `if not any(ranks.values()): continue` (line 273) — must emit singletons for rings layer

Add:
- Pool all qualifying stores (anchor ∪ hardware ∪ warehouse) into one node list per region
- Build proximity graph at TAU_LOOSE_KM using `query_grid_with_dist` (reuse as primitive)
- Union-Find connected components (port pattern from frontend `groupOverlappingClusters`)
- Diameter-split: within each component, greedy partition so no sub-group has diameter > TAU_LOOSE_KM
- For each sub-group: `centroid = mean(member lat/lon)` — promote from vestigial to authoritative
- Compute `tight_intact: bool` — does the sub-group stay connected under TAU_TIGHT_KM edges only?
- Compute `n_distinct_chains`, `n_distinct_owners` (via CHAIN_OWNERS), `classes_present`
- Emit `geometry = [centroid_lon, centroid_lat]` (was anchor pin)

Change:
- `cluster_id` (lines 453–456): currently `c_{anchor_cid}_{lat3}_{lon3}` keyed on anchor pin.
  Re-key on centroid (see Gotcha #1 below — this breaks all existing IDs).
- `primary_anchor` / `anchor_label`: with multiple anchors in a group, pick deterministic
  representative (highest class, then alphabetical chain_id); add `anchor_members` list field.
- `count_1km` / `count_3km` (lines 356–359): recompute relative to centroid.

Keep unchanged:
- `haversine_km`, `linear_score`, `load_cleansed_jsonl`, `build_grid`, `query_grid_with_dist`
- Civic logic (`count_distinct_institutions`, `count_distinct_by_tier`) — query from centroid now
- `RegionEngine` reverse-geocoding — feed centroid
- `CHAIN_FAMILIES` canonicalisation, `tier_descriptor` labels

### build-geometric-ranking.py

**Delete entirely:**
- `iou_equal_circles` function
- `T1:iou` and `T2:iou` predicate blocks (lines 81–92, 173–182, 213–217)
- `rank_pp_iso ≤ p10` percentile gate in `_eval_t1` (Lever A)
- `IoU_max(C, T1)` predicate in `_eval_t2`

**Rewrite `evaluate_tier_geometric(component)`:**
```python
def evaluate_tier_geometric(members, tight_intact, n_distinct_owners, classes_present, hc_count):
    has_hypermarket = "hypermarket" in classes_present
    has_hardware    = "hardware"    in classes_present
    has_warehouse   = "warehouse"   in classes_present
    has_hw_or_wh    = has_hardware or has_warehouse

    if (tight_intact
            and n_distinct_owners >= 3
            and len(classes_present) >= 2
            and has_hypermarket and has_hw_or_wh
            and hc_count >= 1):
        return 1

    if (has_hypermarket and has_hw_or_wh and hc_count >= 1):
        return 2

    if len({m["chain_id"] for m in members}) >= 2:
        return 3

    return None   # ring-layer singleton; not a tier
```

**Tier naming (from 2026-05-16 G1):** Regional / District / Local / Fringe (G1 locked).
Fringe is now unused (no T4); retain in config in case edge cases surface.

### build-radius.py

**No code change required.** It reads `feat.geometry.coordinates` and buffers from there.
Once `build-clusters.py` writes the centroid into `geometry.coordinates`, the ring is
automatically drawn from the centroid.

### New pipeline output

`build-tiles.py` needs a new build target: `layer7-proximity-zones.pmtiles` from the
full untiered component set (all components, grey/neutral styling). Existing
`layer2-clusters.pmtiles` filters to `tier ∈ {1,2,3}`.

---

## 9. Relationship to tier-scoring-overhaul-2026-05-16.md

| 2026-05-16 section | Status under this plan |
|---|---|
| §2 Operator decisions D1–D5 (taxonomy, composition, naming) | **Carries forward unchanged** |
| §3 Pure-predicate tier definitions (IoU + percentile gates) | **Superseded** — IoU dead code; percentile gate deleted (Lever A); predicate rewritten above |
| §4 Anchor taxonomy (4 classes) | **Carries forward unchanged** — ALPHA_HYPERMARKET / WAREHOUSE / LIFESTYLE / HARDWARE |
| §5 EU coverage matrix (Phase 1–6 ingest plan) | **Carries forward** — now also the mechanism for Lever B |
| §6 BentoBox inspector layouts (A and B) | **Carries forward** — centroid change is transparent to the UI |
| §7 Test matrix (6 reference locations) | **Partially superseded** — Mountain View demotion and Sherwood Park upgrade should be re-evaluated under the graph model |
| §8 G1–G15 operator decisions | **All carry forward** except G7 (IoU bounds — now dead code) |
| §9 Phased to-do | **Partially superseded** — Phase 2 rewrite is now the DBSCAN rewrite described here; Phases 1, 3–8 carry forward |

The new minimum viable ship is:
**Phase 1 (taxonomy)** + **DBSCAN rewrite of build-clusters.py** + **Phase 4 (bento).**
The IoU / percentile gate implementation from old Phase 2.3 / 2.2 is no longer needed.

---

## 10. Gotchas

1. **`cluster_id` instability — biggest regression risk.**
   Current IDs are keyed on the anchor pin (`c_{anchor_cid}_{lat3}_{lon3}`). Re-keying
   on the centroid changes almost every ID. Downstream artifacts keyed by `cluster_id`
   will orphan: `clusters-meta.json` (13,657 entries), per-cluster catchment cells
   (~300 MB), `od-summary.jsonl`, `catchment-data.json`, layer3/4/5/6 PMTiles, and any
   saved user state on gis.woodfinegroup.com.
   **Mitigation:** build a one-time `old_id → new_id` crosswalk by spatial nearest-match,
   OR accept a planned full downstream rebuild. Must be decided before implementation starts.

2. **Cluster count will drop materially.**
   Multiple anchors in one retail park (Walmart + Target + Costco within 1km) currently
   emit 3 clusters; they collapse to 1. Expect the 13,657 count to fall — possibly 20–40%
   in dense metros. The T1=435 figure is post-IoU; the new model produces de-duplicated
   clusters directly. Calibrate count-comparison expectations: the new and old counts are
   not the same denominator.

3. **Diameter-split is mandatory — not optional.**
   Without it, a chain of stores each 2.9km apart merges into one giant multi-km cluster
   with a meaningless centroid. Use a deterministic greedy rule (seed on highest-class
   store, absorb nearest qualifying neighbours within τ, repeat) so nightly rebuilds are
   reproducible.

4. **Store assigned to two adjacent groups (tie-breaking needed).**
   A hardware store equidistant between two hypermarkets must be assigned to exactly one.
   Deterministic rule: nearest centroid, then chain_id alphabetical.

5. **Dual-membership chains.**
   Home Depot and Costco appear in both `anchor` and `hardware`/`warehouse` sets. In the
   graph model, one store = one node — do not double-insert as both anchor node and
   hardware node or it self-loops.

6. **T4 definition shift.**
   Today T4 = "anchor, no qualifying secondary in ring." New: isolated graph node.
   Almost equivalent, but not identical — an anchor with only same-class neighbours
   (two Walmarts 500m apart) is connected in the graph but still T4 by composition.
   `evaluate_tier_geometric` must test composition diversity, not just graph degree.

7. **Civic count shift.**
   Hospitals/universities queried from the anchor pin today; querying from the centroid
   will slightly change `hc_count`/`he_count` for some clusters. Expected, not a bug.

8. **Overnight build policy.**
   Per project-gis policy, full pipeline rebuilds touching layer3–6 PMTiles must start
   after 05:00 UTC (10pm Vancouver PDT). The DBSCAN rewrite itself is cheap; the
   downstream tile + per-cluster-cell regeneration is the heavy part. Schedule accordingly
   using the `at 05:00` convention.

9. **Editorial artifacts need updating.**
   The methodology TOPIC drafts (A1 O-D Catchment, A3 Catchment Ranking) and the "Data"
   modal copy (A4 text-gis-data-methodology-dialog) all describe the anchor-ring model.
   These must be updated to describe the graph/centroid model and the rings layer.
   Route to `.agent/drafts-outbound/` → project-editorial.

---

## 11. Implementation checklist

- [ ] **C1** Decide cluster_id migration strategy (crosswalk vs full rebuild) before writing code.
- [ ] **C2** Add `TAU_TIGHT_KM`, `TAU_LOOSE_KM`, `COMPONENT_SPLIT_KM`, `CHAIN_OWNERS` to config.py.
- [ ] **C3** Rewrite `compute_clusters()` in build-clusters.py: graph build → Union-Find → diameter-split → centroid → `tight_intact` → `evaluate_tier_geometric`.
- [ ] **C4** Remove `if not any(ranks.values()): continue` — emit singletons for rings layer.
- [ ] **C5** Rewrite `evaluate_tier_geometric()` in build-geometric-ranking.py per §8.
- [ ] **C6** Delete `iou_equal_circles` + IoU predicate blocks + `rank_pp_iso ≤ p10` gate.
- [ ] **C7** Add `layer7-proximity-zones.pmtiles` build target in build-tiles.py (grey/neutral, full component set).
- [ ] **C8** Filter `layer2-clusters.pmtiles` to `tier ∈ {1,2,3}` only.
- [ ] **C9** Add `tight_intact`, `n_distinct_owners`, `classes_present`, `anchor_members` fields to clusters-meta.json emission.
- [ ] **C10** Verify build-radius.py auto-follows centroid (no code change expected; confirm with test run).
- [ ] **C11** Run full pipeline rebuild (overnight build; schedule at 05:00 UTC).
- [ ] **C12** Validate counts: NA T1 ~950–1,050, NA total tiered ~3,650; EU T1 ~400–500 pre-Lever-B, rising after ingest.
- [ ] **C13** Regression check: 6 reference clusters from 2026-05-16 §7 re-evaluated under new model.
- [ ] **C14** Update editorial artifacts (A1, A3, A4) — route to drafts-outbound → project-editorial.
- [ ] **C15** Update DATA-MANIFEST.md to reflect graph model + rings layer.
- [ ] **EU-L** Lever B ingest backlog: OBI → toom → Hagebau → Bauhaus DE/ES → Brico Dépôt → Bricocenter → gamma-nl → karwei-nl. Promote each country's market-leader to ALPHA_HARDWARE post-ingest.

---

## 12. Open decisions (not yet locked)

| # | Decision | Options | Constraint |
|---|---|---|---|
| OD1 | cluster_id migration | (a) spatial crosswalk old→new; (b) full downstream rebuild | Must decide before C3 |
| OD2 | CHAIN_OWNERS completeness | Draft in config.py; operator reviews before C3 | Affects T1 count directly |
| OD3 | EU Lever B priority order | Which hardware chain to ingest first (OBI most stores) | EU T1 target depends on this |
| OD4 | Rings layer on by default? | On by default (adds visual noise); off by default (users miss it) | UX decision; not a data decision |

---

## 13. References

**Scripts (verified 2026-05-20):**
- `app-orchestration-gis/build-clusters.py` — primary rewrite target (lines 140–160, 236–353, 453–456, 547)
- `app-orchestration-gis/config.py` — new constants + CHAIN_OWNERS + Lever B TODO backlog (lines 124–145)
- `app-orchestration-gis/build-geometric-ranking.py` — IoU/percentile deletions (lines 81–92, 173–182, 213–217)
- `app-orchestration-gis/build-radius.py` — no change; auto-follows centroid
- `app-orchestration-gis/build-tiles.py` — new layer7 target
- `www/index.html` — rings layer toggle (new); tier count, colours, bento (Phase 4 from 2026-05-16 plan)

**Prior plan:** `.agent/plans/tier-scoring-overhaul-2026-05-16.md` — taxonomy, UI, EU
ingest phases, operator decisions G1–G15 all carry forward.

**Artifact registry entries to update after implementation:** A1, A3, A4 (editorial);
C-table rows for all PMTile layers.

— drafted 2026-05-20 from two Opus deep-think sessions + operator design session
