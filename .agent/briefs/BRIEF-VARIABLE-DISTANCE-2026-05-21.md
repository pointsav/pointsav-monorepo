# BRIEF — Variable Co-location Distance & Two-Stage Ranking

> **Settled specification** · 2026-05-21 · consolidates the round-by-round draft.
> Parent: `BRIEF-CENTRE-BUBBLE-RING-2026-05-21.md`.
> Companions (memory): `gis-variable-distance-model`, `gis-site-selection-reframe`.
> Provenance: six Opus agents over three rounds — GIS · CBRE · coding (§9).

---

## 1. The model

A co-location is described by **three orthogonal axes**:

- **Tier** — retailer-category *composition*: which anchor categories are
  co-located (hypermarket / hardware / warehouse / lifestyle). Tier is
  composition **only** — not distance, not demand.
- **Distance rank** — a geometric compactness measure, ranked relative to peer
  co-locations. **Stage 1** of ranking.
- **Demand rank** — catchment demand, ranked. **Stage 2**, layered on Stage 1.

The variable-distance model replaces the fixed 1 km / 3 km radius toggle (retired
— radius is a clustering parameter, not a view control).

---

## 2. Membership — what *is* a co-location

Co-locations are formed by **tight-first two-pass DBSCAN**:
- **Pass 1** — proximity graph at `TAU_TIGHT = 1.0 km`; lock tight nuclei.
- **Pass 2** — proximity graph at `TAU_LOOSE = 3.0 km` on the remainder; split
  any component whose max pairwise diameter exceeds the cap (`split_greedy`).

- **Hard membership cap = 3.0 km** max pairwise diameter — **uniform, not
  per-country**. Do **not** widen to 4 km: a 4 km cap merges genuinely distinct
  retail nodes (Sherwood Park B + C, §6). The cap defines the *unit of
  observation*; a percentile cannot un-merge a bad boundary.
- **1.0 km = `tight_intact` flag** — every member within 1 km of every other.
  A quality/composition flag, **not** a second membership pass.
- The cap is uniform because membership is a *format* limit — "is this one
  shopping destination." Market differences (dense EU vs sprawling NA) are
  absorbed by the **relative rank** (§3), not by per-country caps.

---

## 3. Stage 1 — geometric rank

- **`span_km`** = max pairwise diameter of the co-location's members (the
  geometric compactness measure).
- **`tight_intact`** = bool (all members within 1 km).
- **`dist_rank_in_tier`** = inverted percentile of `span_km` — tighter ⇒ better
  — computed **within tier**, shrinkage-blended across the co-location's
  **Country** and **continent** (NA / EU): `w = n / (n + K)`, `K ≈ 20–30`.
  Within-tier so the rank discriminates *inside* a composition bucket rather
  than re-encoding the tier.

---

## 4. Stage 2 — demand rank

Demand is layered **on top of** Stage 1 — it does not replace geometry.
`demand_rank_in_tier` = inverted percentile of a demand measure, within tier.

- **Target measure:** demand inside the O-D-derived **primary / secondary
  catchment** — ring-fencing the people who *actually frequent* the
  co-location's retailers. Primary ≈ origins supplying 60–70 % of trips;
  secondary ≈ the next band to ~85–90 %. A *frequency* split — only O-D /
  mobility data can build it.
- **Interim measure** (pre-mobility-data): ambient population / spend in the
  existing **35 km / 150 km** catchment rings — what `synthesize-od-study.py`
  already produces.
- Per-cluster **`demand_basis`** flag (`"od-primary-secondary"` vs
  `"catchment-35-150"`). Observed-O-D and interim clusters are ranked in
  **separate pools**. The UI must never label an ambient ring "catchment" or
  "trade area" — caption it as an interim population proxy with a method badge.

**Data status:** the US already has a true per-cluster primary/secondary
catchment — `lodes-work-summary-us.jsonl` bands `total_work_reach_35km/_150km`
(needs joining into `clusters-meta.json`). ES has MITMA. CA — including
Sherwood Park — has none yet; it runs on the interim fallback. (LODES is
work-commute O-D — a proxy; a true retail-visitor catchment needs more data.)

---

## 5. Combined ranking — lexicographic

- Headline order = lexicographic sort key
  **`(tier, dist_rank_in_tier, demand_rank_in_tier)`** within (tier × country).
- **Not** a weighted blend — a blend needs hand-tuned weights and lets demand
  mask bad geometry.
- Both ranks are emitted as **parallel readouts**; the frontend shows tier +
  compactness + demand side by side (a high-demand *corridor* and a high-demand
  *integrated node* must be distinguishable at a glance).

---

## 6. Sherwood Park — calibration case: **3 co-locations**

7 retail stores, 3 geographic nodes:

| Node | Stores | `span_km` | `tight_intact` |
|---|---|---|---|
| **A** (south ~53.511,−113.321) | Walmart + Canadian Tire | 0.53 km | true |
| **B** (centre ~53.544,−113.304) | Real Canadian Superstore + Home Depot + Costco | 1.68 km | false |
| **C** (north ~53.569,−113.285) | Walmart + Canadian Tire | 0.74 km | true |

- **3, not 2:** merging would require chaining across the ~3 km A↔B / B↔C
  bridges; A↔C is ~7 km — far past the cap. A and C are distinct tight nodes.
- **3, not "split B":** B is within the 3 km cap → one valid co-location. It
  self-identifies as loose via `tight_intact = false` + a poor `dist_rank` —
  the model working as intended, not a merge or a split.
- A 4 km cap would merge B + C → 2. This is the proof the cap stays at 3 km.

---

## 7. Implementation

- **Stage 1** = extend `build-geometric-ranking.py`. **Stage 2** = new
  `build-demand-ranking.py`. Both are post-clustering, pre-tile stages.
- Couples to the **DBSCAN rewrite** — production `build-clusters.py` is
  anchor-centric and would double-count; `span_km`/percentiles require the
  component model.
- **Schema** (into `clusters.geojson` / `clusters-meta.json` / PMTiles):
  canonical `tier`, `span_km`, `tight_intact`, `dist_rank_in_tier`,
  `demand_rank_in_tier`, `demand_basis`, `ring_radius_km`.

### S0 — Alberta test bed (build now, no overnight build)
- Two-pass tight-first DBSCAN in `simulate-dbscan-ab.py` (cap 3.0 km).
- Emit `span_km`, `tight_intact` per co-location.
- `dist_rank_in_tier` — Alberta pool (proves the mechanics; true distributions
  need S2's 13 countries).
- Frontend: data-driven `ring_radius_km`; distance-rank + tight/loose badge in
  the inspector; Back-button / sticky-ring nav.
- Regenerate `sim-ab-dbscan.geojson`, redeploy. Sherwood Park should read **3**.

### S2 — production (overnight build)
- DBSCAN rewrite of `build-clusters.py`; full multi-country percentile
  distributions; Stage 2 demand stage + O-D join; cluster-ID migration.

---

## 8. Open operator decisions (all small / deferrable)

1. **Tier purity** — move the population/spend gates currently inside
   `build-geometric-ranking.py` tier predicates out to Stage 2 (tier =
   composition only). Recommended: yes.
2. **Headline sort** — geometry-primary (recommended, ship now) vs CBRE's
   demand-led-gated-by-geometry. Schema-neutral — a one-function swap later.
3. **Interim demand scalar** — population `pp` (recommended) vs a
   population + spend blend.

---

## 9. Research provenance

Six Opus agents, three rounds, 2026-05-21:
- **Round 1** (GIS · CBRE · coding) — model design: geometric measure,
  membership cap, percentile mechanics, tier interaction.
- **Round 2** (GIS · CBRE · coding) — 3 vs 4 km cap, two-stage ranking
  composition, O-D primary/secondary catchments + interim fallback.

Full agent reports retained in the 2026-05-21 session transcript.
