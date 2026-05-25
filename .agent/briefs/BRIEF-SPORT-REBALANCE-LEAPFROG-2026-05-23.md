# BRIEF — Sport Category · Tier Rebalance · Leapfrog 2030

> Synthesis brief · 2026-05-23 · captures four-agent research session (2026-05-22).
> **STATUS: Phase 19 IMPLEMENTED** — committed `a2c974e4` (2026-05-23, Peter Woodfine).
> Scheduled to build tonight at 05:00 UTC 2026-05-24 (PID 2507282).
> Companion: `BRIEF-CATEGORY-TAXONOMY-2026-05-22.md` — original category rationale.

---

## Implementation status (2026-05-23)

| Item | Status | Commit |
|------|--------|--------|
| `sport` category added to taxonomy.py | ✓ DONE | a2c974e4 |
| n≥4 T1 rule added to tier_of() | ✓ DONE | a2c974e4 |
| Sport BRAND_FILL: Decathlon EU×12+CA, REI, Bass Pro, Cabela's | ✓ DONE | a2c974e4 |
| Geometric T2→T3 split (span<1.25km ∧ mc≤2) | ✓ DONE | a2c974e4 |
| London splitter fix (split_greedy_tight diameter guard) | ✓ DONE | a2c974e4 |
| costco-uk re-ingest (30 clean warehouse records) | ✓ DONE | a2c974e4 |
| phase19-rebuild.sh (ingest 16 chains → rebuild cluster/tiles) | ✓ DONE | a2c974e4 |
| Tonight's build scheduled (PID 2507282, 05:00 UTC 2026-05-24) | ✓ SCHEDULED | — |
| DE lifestyle chains (XXXLutz / Höffner / Segmüller) | DEFERRED Phase 20 | — |
| Meijer US / Bodega Aurrera MX | DEFERRED Phase 19 or 20 | — |
| `market_position_pctile` field | DEFERRED | — |

---

## 1. Problem statement

Phase 18 delivered 5,702 clusters. Tier distribution is structurally unbalanced:

| Tier | Count | Share |
|------|-------|-------|
| T1   | 1,157 | 20.3% |
| T2   | 4,283 | 75.1% |
| T3   | 262   | 4.6%  |

Target: approximately **30% / 50% / 20%**. T2 dominates; T3 is near-zero.

**Root cause**: 90.2% of all T2 clusters (3,863 / 4,283) carry the single
combo `{hypermarket, hardware}` — the globally dominant retail-strip pattern.
With only 4 retail categories (hypermarket / hardware / price_club / lifestyle),
the combinatoric space is too narrow. Adding a fifth retail anchor category
opens enough new T1 and T3 paths to rebalance.

---

## 2. Decisions reached

Three decisions were made by the operator after reviewing agent reports:

### Decision 1 — Add `sport` as 7th category (5th retail anchor)

**Confirmed 2026-05-23 after Round 2 agent analysis: Option 2 (sport as T2/T3
enhancer only; T1 rule unchanged). Option 3 (sport enables T1) was rejected.**

Rejection rationale for Option 3:
- Option 3 would expand EU T1 by **728%** (144 → ~1,192) vs NA T1 by only 26%
  (1,013 → ~1,280), making T1 cross-market incomparable.
- Flip candidates ({Hyper+HW+Sport}) have median span 1.397km / mc=3 — demonstrably
  weaker geometry than current T1 (median span 2.085km / mc=5).
- EU T1 undercount is a **price_club gap** (Costco sparse in EU), not a sport gap.
  Option 3 papers over the Costco gap rather than fixing it.

**Sport** (Decathlon-class destination sporting goods, ≥3,000 sqm) is the
correct addition as T2/T3 enhancer:

- Decathlon alone has ~2,000 stores across 13+ EU countries; it is the most
  frequently co-located non-anchor with hypermarkets in EU retail parks.
- REI (US, ~180 stores) and Bass Pro/Cabela's (US, ~200 stores combined)
  fill the same slot in the North American market.
- Sport unlocks significant T3 volume (`{hypermarket, sport}`, `{hardware, sport}`)
  without inflating T1.

**T1 rule UNCHANGED:**
```
T1 = has_hyper ∧ has_hw ∧ (has_pc ∨ has_life)   ← unchanged
T2 = has_hyper ∧ n_retail ≥ 2                    (unchanged)
T3 = n_retail ≥ 2 ∧ ¬has_hyper                  (unchanged)
```

Sport adds new T3 combos ({hypermarket, sport}, {hardware, sport}) and contributes
to T2 via {hypermarket, hardware, sport}. Does NOT enable T1.

**Category is same across NA and EU; only BRAND_FILL differs per country.**
This is the invariant the whole taxonomy is built on.

**`sport` NAICS code:** 451110 (Sporting Goods Stores — large-format subset).

### Decision 2 — Fix the London splitter bug

`split_greedy_tight()` in `build-clusters.py` (lines 130–154) fails on dense
city-wide graphs. Cluster `co_gb_n5150596_w011882` contains 487 members
spanning 22.07 km — this is the whole of central London being treated as one
co-location unit.

The bug: the greedy partitioner seeds from the densest node and assigns all
reachable nodes to that seed. When the tight-pass graph is densely connected
(as in central London where stores are within 1 km of each other in every
direction), no partition is ever forced.

Fix approach: after the tight-pass graph is built, apply a diameter constraint.
If a component's diameter exceeds `MAX_CLUSTER_DIAMETER_KM` (suggest 8 km),
subdivide using spatial k-means or a second-pass DBSCAN with a tighter radius
before handing off to the greedy splitter.

### Decision 3 — TAU_LOOSE stays at 3.0 km

No distance change. The 3 km radius is correct for retail-park-scale grouping.
The tier rebalance is achieved through category expansion, not distance tuning.

---

## 3. UI clarification — cluster mode vs retailer zoom

**Cluster mode** (zoomed out to co-location level):
- Show **only T1 / T2 / T3 cluster centroids** — the tier bubbles.
- Do not show individual store rings in this mode.
- No change from current behavior for the cluster dots themselves.
- **Filter**: if any non-tier clusters or ungrouped store markers are currently
  visible in cluster mode, suppress them. Only T1/T2/T3 entries should appear.

**Retailer zoom** (zoomed in past the cluster threshold):
- Show **all rings**, including stores that are not part of any T1/T2/T3 cluster.
- This is a change: previously, non-clustered stores may not have been shown.
- A store with no co-location partner still has a valid 1-ring or 3-ring
  catchment and should be visible at retailer zoom.

This creates a clean two-mode UX: the map at scale shows only meaningful
multi-anchor co-locations; zooming in reveals the full retailer geography.

---

## 4. Agent findings — EU price_club structural gap

Nine EU countries have no Costco-class B2C membership warehouse:
- **IT, GR, AT, DK, NO, FI**: no qualifying price_club format at all.
- **DE, FR, PL, ES, NL**: limited Metro/Makro (B2B only — already rejected) or
  Selgros DE (300–5,000 sqm variety, B2B-oriented, ~100 DE stores).

**Selgros DE** was investigated as a price_club candidate:
- Selgros is ~50% B2B trade by revenue; no membership fee; no B2C-primary model.
- Agent verdict: does not qualify as price_club. Not recommended for ingestion.
- The price_club gap in EU is structural and should not be papered over with
  B2B formats — sport is the correct T1 enabler for EU markets.

**Action**: No price_club additions this phase. Revisit if Costco expands in EU.

---

## 5. Agent findings — EU lifestyle chain expansion

German furniture chains that pass the ≥15,000 sqm lifestyle threshold:

| Chain | Footprint | OSM | Recommended |
|-------|-----------|-----|-------------|
| XXXLutz (XXXL brand) | 40+ DE stores, ~35,000 sqm avg | Q636293 present | **YES** |
| Höffner | 38,000 sqm per store, ~15 DE stores | Mapped | **YES** |
| Segmüller | 40,000+ sqm, ~10 DE stores (Bavaria/BW) | Mapped | **YES** |
| Porta | 25,000 sqm avg, ~40 DE stores | Partial | Borderline |

These would add ~60–80 lifestyle anchors to DE, enabling new T1/T2 combos
where IKEA is absent. **Deferred to Phase 20** — sport category comes first;
lifestyle expansion for DE can be done separately once sport baseline is stable.

---

## 6. Agent findings — cluster volume analysis

Mathematical floor for 30%/50%/20% distribution with existing 5,702 clusters:

- T1 floor = 31% is near-impossible without relaxing the T1 rule structure.
- Closest achievable with existing categories: ~33% / 46% / 20%, requiring
  DE+GB rule promotion and ~600 new standalone T3 clusters.
- Sport category is the cleaner path: estimated +150–300 new T1 clusters
  (EU retail parks where Decathlon + hypermarket + hardware co-locate),
  +400–600 new T3 clusters (`{hypermarket, sport}` + `{hardware, sport}`).
- Net effect: shifts T2 share down to ~65–68%, T3 share up to ~12–15%,
  T1 share up to ~20–22%. Not a perfect 30/50/20 but significantly more
  balanced than current 20/75/5.

**The 30/50/20 target is aspirational, not a hard constraint.**

---

## 7. Agent findings — Leapfrog 2030 roadmap

Strategic positioning: "First global, open-data, auditable trade-area dataset
at sub-metro grain in 17 countries" — the open-data differentiator over
CBRE/Placer.ai/SafeGraph/CoStar.

Suggested expansion roadmap:

| Milestone | Category | Year |
|-----------|----------|------|
| Sport anchors (Decathlon / REI / Bass Pro) | Phase 19 | 2026 |
| EV charging (Shell/BP Pulse co-location) | New layer | 2027 |
| DE lifestyle chains (XXXLutz / Höffner / Segmüller) | Phase 20 | 2026 |
| Continuous RGS (regional gravity scores) | Compute layer | 2027–28 |
| `market_position_pctile` field | Cluster metadata | 2026 |
| 30-country expansion (LATAM + SEA + ANZ) | Coverage | 2028–29 |

**Now priority**: `market_position_pctile` — a percentile-rank field on each
cluster within its Regional Market, enabling "this Walmart+Home Depot is in
the top 12% of co-locations in its metro" — high signal for site-selection use.

---

## 8. Missing NA retailers (not blocked on sport)

| Chain | Country | Gap | Records |
|-------|---------|-----|---------|
| Meijer | US (Midwest) | 240 stores; biggest hypermarket gap in US | Not yet ingested |
| Bodega Aurrera (large-format) | MX | Walmart MX hypermarket arm | Not yet ingested |

These can be ingested independently; not blocked on sport category decisions.
Recommend Phase 19 or 20 depending on session capacity.

---

## 9. Implementation plan — Phase 19

### Step 1 — Sport category definition in taxonomy.py

1. Add `"sport"` to `CATEGORIES` dict (between `lifestyle` and `medical`):
   ```python
   "sport": {
       "label": "Sport / Outdoor Anchor",
       "naics": "451110",
       "description": "Destination sporting-goods anchor (Decathlon-class, ≥3,000 sqm).",
   },
   ```

2. Add `"sport"` to `_RETAIL_CATS`:
   ```python
   _RETAIL_CATS = {"hypermarket", "hardware", "price_club", "lifestyle", "sport"}
   ```

3. Add `has_sport` to `tier_of()`:
   ```python
   has_sport = "sport" in retail
   if has_hyper and has_hw and (has_pc or has_life or has_sport):
       return 1
   ```

4. Add `BRAND_FILL["sport"]` with per-country brand fills (see §9 Step 2).

5. Add DISPLAY_NAMES entries for each sport chain.

### Step 2 — Sport chain YAML files and ingestion

**EU — Decathlon** (brand:wikidata=Q509349):
- `decathlon-fr.yaml`, `decathlon-de.yaml`, `decathlon-gb.yaml`,
  `decathlon-es.yaml`, `decathlon-it.yaml`, `decathlon-nl.yaml`,
  `decathlon-be.yaml`, `decathlon-pl.yaml`, `decathlon-pt.yaml`
- Per-country bbox filters; `name_query: "Decathlon"` fallback.

**NA — REI** (brand:wikidata=Q860698):
- `rei-us.yaml` — ~180 stores, US only.

**NA — Bass Pro Shops / Cabela's** (brand:wikidata=Q4866375 / Q606290):
- `bass-pro-shops-us.yaml` + `cabelas-us.yaml`
- Same taxonomy slot `sport` — these are co-located in many US markets.

**Ingest command** (per chain):
```bash
python3 ingest-osm.py --chain decathlon-fr
```

### Step 3 — Fix London splitter bug in build-clusters.py

Location: `build-clusters.py` lines 130–154 (`split_greedy_tight()`).

Add `MAX_CLUSTER_DIAMETER_KM = 8.0` constant at the top of the file.

After tight-pass DBSCAN assigns components, before the greedy splitter runs:
- For any component with diameter > `MAX_CLUSTER_DIAMETER_KM`, run a second
  spatial k-means pass with k = ceil(diameter / MAX_CLUSTER_DIAMETER_KM).
- This is a pre-split before `split_greedy_tight()` — not a replacement.

Alternative: simpler fix — change `split_greedy_tight()` to use a
distance-capped BFS: any node whose great-circle distance from the seed
centroid exceeds `MAX_CLUSTER_DIAMETER_KM / 2` cannot be assigned to that seed.

### Step 4 — UI filter (index.html)

In cluster mode (zoomed to cluster level):
- Only render markers where `tier ∈ {1, 2, 3}`.
- If any store markers without a cluster assignment are currently rendered at
  cluster zoom level, filter them out.

In retailer zoom (zoomed past cluster threshold):
- Render all rings, including stores with no cluster assignment.
- This is an extension of the existing ring rendering, not a replacement.

### Step 5 — Rebuild and deploy

```bash
cd pointsav-monorepo/app-orchestration-gis
python3 build-clusters.py   # includes taxonomy.py changes
python3 build-tiles.py
python3 nightly-rebuild.sh  # or equivalent
```

Deploy to gateway; update artifact-registry.md; commit via commit-as-next.sh.

---

## 10. Open questions (not blocking Phase 19)

1. **`market_position_pctile`**: Add to clusters-meta.json now, or as a separate
   Phase 20 compute step?
2. **Meijer / Bodega Aurrera**: Phase 19 or 20?
3. **DE lifestyle chains (XXXLutz / Höffner / Segmüller)**: Phase 20 — confirm.
4. **London bug fix validation**: After fix, how many clusters should central London
   produce? Roughly 10–20 distinct retail-park clusters expected.
5. **EV charging as future layer**: AEC research already done (see
   `AEC-INFRASTRUCTURE-LAYERS-RESEARCH.md`) — can this be a Phase 21 scope?
