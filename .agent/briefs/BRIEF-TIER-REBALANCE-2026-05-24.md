# BRIEF — T2/T3 Tier Rebalancing: Scenario A

> **Decision-ready analysis** · 2026-05-24
> Companion: `BRIEF-FORWARD-2026-05-24.md` (Phase 21), `BRIEF-VARIABLE-DISTANCE-2026-05-21.md`
> Provenance: agent analysis on 6,493-cluster Phase 21 dataset (clusters-meta.json + build-clusters.py audit)

---

## 1. Problem statement

Current global distribution (Phase 21, 6,493 clusters):

| Tier | Count | Share |
|---|---|---|
| T1 | 1,537 | 24% |
| T2 | 3,090 | 48% |
| T3 | 1,866 | 29% |

T2 is inflated. Nearly half of all identified co-locations carry T2 status.
The core issue: the current `tier_of()` rule in `taxonomy.py` promotes any cluster with
`has_hyper` and ≥2 members to T2, regardless of whether a hardware / home-improvement
anchor is present. This means a hypermarket + lifestyle store = T2, which overstates
investability for office tenancy purposes.

---

## 2. The hidden geometric downgrade rule

`build-clusters.py` lines 317–318 contain a post-classification downgrade:

```python
if tier == 2 and span < 1.25 and len(members) <= 2:
    tier = 3
```

This silently demotes **1,133 clusters** (small T2 → T3) without this being reflected in
`taxonomy.py`'s tier logic. It is a geometric patch on top of a composition rule.
This is a design smell: tier is defined as composition-only, but geometric shape is
being used to correct it. Scenario A should make this downgrade unnecessary.

---

## 3. Scenario A — proposed rule change

**Trigger:** `has_hyper AND has_hw` required for T2.

Change in `taxonomy.py → tier_of()`:

```python
# Current:
if has_hyper and n >= 2:
    return 2

# Scenario A:
if has_hyper and has_hw:          # requires hardware co-location
    return 2
if has_hyper:                     # hyper without hw → T3
    return 3
return 3
```

**Effect on distribution:**

| Tier | Current | Scenario A | Delta |
|---|---|---|---|
| T1 | 1,537 (24%) | 1,537 (24%) | unchanged |
| T2 | 3,090 (48%) | 2,469 (38%) | −621 |
| T3 | 1,866 (29%) | 2,487 (38%) | +621 |

The 621 clusters that move T2→T3 are `has_hyper` but lack a hardware anchor
(typically: hypermarket + lifestyle, or hypermarket + price_club without hw).

Result is a near-equal T2/T3 split (38%/38%) with T1 at 24%. This is a healthier
distribution: T2 now credibly signals a Tier 1 + Tier 2 composition pairing, not
merely "has a hypermarket."

**The geometric downgrade rule becomes redundant.** Under Scenario A, small
hyper-only clusters drop to T3 by composition, not geometry. The
`span < 1.25 and len <= 2` patch can be removed from `build-clusters.py`,
making tier purely composition-derived as originally intended.

---

## 4. Does Scenario A allow removal of the variable distance model?

**Short answer: No — but it strengthens it.**

The variable distance model operates at two levels:

1. **Membership** — DBSCAN at TAU_TIGHT=1.0 km / TAU_LOOSE=3.0 km. Scenario A does
   not touch this. Membership is geometric regardless of tier rule.

2. **Within-tier ranking** — `dist_rank_in_tier` (inverted percentile of `span_km`).
   This ranks clusters *inside* a tier bucket. Scenario A makes the tier buckets
   cleaner (T2 now means hyper+hw, not just hyper), so `dist_rank_in_tier` now
   discriminates among a more compositionally homogeneous peer group. This is an
   improvement, not a removal.

**What Scenario A does eliminate:**
- The geometric patch (`span < 1.25 and len <= 2`) which was a workaround for the
  over-broad T2 definition.
- The need for UI caveats that "some T2 clusters are only marginally T2."

**What Scenario A does not eliminate:**
- `span_km` as the geometric compactness measure within a tier.
- `dist_rank_in_tier` as Stage 1 ranking.
- The tight-first DBSCAN membership algorithm.
- The 3.0 km cap on cluster diameter.

**Conclusion:** Scenario A purifies the tier definition. The variable distance model
remains necessary for ranking. The two are orthogonal: tier = composition,
rank = geometry.

---

## 5. EU analysis

EU-specific pre/post counts (estimated from agent analysis):

| Tier | EU Current | EU Scenario A |
|---|---|---|
| T1 | 516 (19%) | 516 |
| T2 | 1,590 (59%) | ~1,200 (44%) |
| T3 | 622 (23%) | ~1,012 (37%) |

EU T2 is proportionally larger than NA because EU retail formats separate
hardware from food (Leroy Merlin / Brico Depôt are not co-located with
Carrefour as frequently as Home Depot is with Walmart NA). Scenario A
correctly identifies these as T3 unless the hw anchor is genuinely present.

---

## 6. Recommended implementation path

1. **taxonomy.py change** — 3-line diff above. No rebuild until overnight window.
2. **Remove geometric downgrade** from `build-clusters.py` lines 317–318.
3. **Verify** on a 100-cluster sample (EU + NA) that T2 assignments are now
   exclusively `has_hyper + has_hw`.
4. **Phase 22 rebuild** via `phase22-rebuild.sh` (to be written):
   - Runs `build-clusters.py` with updated taxonomy
   - Runs `build-tiles.py --layer 2`
   - Target: overnight window, ≥05:00 UTC to avoid peak hours

---

## 7. Open questions

- **Category research** — Are there EU-specific cases where `has_hw` should be
  defined more broadly (e.g., large-format garden centre as hw-equivalent)?
  Noted for the next category analysis session.
- **Electronics as hw-equivalent?** The `electronics` category (Phase 21) anchors
  MediaMarkt / Saturn / Boulanger. Should `has_electronics` qualify for T2 when
  `has_hw` is absent? This is a research question, not a code change yet.
- **T3 heterogeneity** — After Scenario A, T3 contains: (a) hyper-only, (b) hw-only,
  (c) price_club-only, and (d) lifestyle-only clusters. These are qualitatively
  different. A T3 subcategory system may be warranted in a future phase.
