---
mailbox: outbox
owner: task@project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-gis

---
from: task@project-gis
to: command@claude-code
re: Phase 2.7 diff harness — PAUSE required before Stage 6
created: 2026-05-16T04:10:00Z
updated: 2026-05-16T04:25:00Z
status: operator-pending
actioned_by: command@claude-code
actioned_at: 2026-05-16
note: read by Command; decisions A/C remain; Decision B resolved (synthesize-od DONE); updated findings below
msg-id: project-gis-20260516-phase2-diff-pause
---

Phase 2 geometric ranking engine has been written and run against the Phase 1
cluster build. **synthesize-od-study.py has now completed (9,234/9,234 clusters).
Findings updated below. Operator acknowledgment required before Stage 6.**

---

### Updated Contingency Table (post synthesize-od): rank_v2 × tier (V3)

```
   v2\tier        T1        T2        T3        T4 (Fringe)
      v2=?         2       268       381          2259   (new Phase 1 EU chains, no prior rank)
      v2=1        29       644      1493          2554
      v2=2         5       100       516           872
      v2=3         0         4        45            62
```

Totals: T1=36  T2=1,016  T3=2,435  T4=5,747

---

### Test matrix — final results

| Location | Expected | Actual V3 | Notes |
|---|---|---|---|
| Mountain View (Walmart+Costco) | T2 District | **T2 District** ✓ | rank_pp=0.194 ≤ P25; rank_sp=0.258 |
| Sherwood Park (Costco+HomeDepot) | T3 Local | **T3 Local** ✓ | rank_pp=0.447 ≤ P50; hc_count=1; E2 closed |
| Mercadona Pamplona (×2) | T1 Regional | **T1 Regional** ✓ | Makro-ES warehouse co-tenant; rank_pp≈0.033 |
| Mississauga Vaughan (Walmart+Costco) | T1 Regional | **T4 Fringe** | rank_sp=0.359 — structural issue (see below) |
| London/UK Tesco clusters | T2 District | **T2 District** ✓ | Tesco+Home Depot combos pass T2 |

---

### Decision B — RESOLVED

synthesize-od-study.py completed 2026-05-16T04:19:37Z. All 9,234 clusters now have
catchment data. Mercadona/Tesco/Sainsbury's EU clusters are fully ranked. No
operator action needed on Decision B.

---

### Decision A — REVISED: rank_sp gate structural issue for CA/UK

The earlier framing ("relax P20 to P25") was incorrect. The actual finding is:

**No Canadian or UK cluster can achieve T1 Regional under the current rank_sp gate,
regardless of threshold, because the gate is contaminated by a cross-border effect.**

Findings:
- Best CA T1 candidate: `c_walmart_ca_43x798__79x539` (Vaughan, ON)
  - Composition: Walmart + Costco + Canadian Tire + Home Depot ✓
  - rank_pp_iso = 0.0016 (top 0.16% in Canada — #1 by primary catchment) ✓
  - rank_sp_iso = 0.359 — far above P20, P25, P30 thresholds
  - pp = 6,359,233 / sp = 6,219,608 (total reach 12.6M people)
- P20 rank_sp cutoff in Canada corresponds to sp = 7,876,592 (cluster near Quebec City)
- The clusters with the best rank_sp in Canada are **border-town clusters** (Niagara,
  Windsor, Sarnia, Hamilton) whose 35-150km secondary ring extends into the US and
  captures dense US population. These border clusters dominate rank_sp, pushing even
  the GTA hubs to rank_sp=0.30-0.40.
- UK has the same problem: best rank_sp clusters are in Norfolk/Suffolk (whose
  secondary ring extends over continental Europe), not London or Manchester.
- CA clusters with rank_sp ≤ 0.20: 124 of 624 — but none have T1 composition
  AND rank_pp ≤ 0.10 simultaneously.

This is a methodology limitation, not a data gap. The rank_sp gate as designed
excludes dense national hubs in countries where border proximity inflates secondary
ranks for peripheral clusters.

**Three options for operator decision:**

**Option 1 — Remove rank_sp from T1:** T1 keeps rank_pp ≤ P10, civic, IoU, composition.
Secondary population reach is implicit in rank_pp (a 35-km primary ring of 6M people
already demonstrates national significance). This would enable Vaughan and likely
1-2 Toronto-area and 1-2 London-area clusters to reach T1.
Risk: slight over-tiering of dense but isolated rural hubs.

**Option 2 — Replace rank_sp with absolute threshold (sp ≥ 5,000,000):** A cluster
with 5M+ secondary-ring residents clearly has national-scale reach regardless of
cross-border effects. Vaughan (sp=6.2M) passes; border clusters pass too but only
if they also pass rank_pp ≤ P10 (which they don't — border clusters have rank_pp ≈ 0.5).
This is more principled than P20.

**Option 3 — Keep P20 (accept CA/UK T1 = 0):** The US and MX T1 counts (30 and 4)
are sufficient. CA and GB are both adequately served by T2 District. No change needed.

Recommendation: **Option 2** (absolute secondary threshold sp ≥ 5,000,000). It is
principled, addresses the cross-border contamination, and is explainable to end users:
"Tier 1 requires at least 5 million residents within the secondary trade area."

---

### Decision C — Stage 6 scope (unchanged)

Phase 1 code fully committed and tested. Phase 2 engine now has full catchment data.
Options:
  **(i) Stage 6 Phase 1+2 together** (recommended): run build-tiles.py now with
      current V3 tier assignments. T1=36, T2=1016, T3=2435. Pending Decision A/C
      threshold change (trivial code edit — 1 line in build-geometric-ranking.py).
  (ii) Stage 6 Phase 1 only: run build-tiles.py with V2 rank_3km for tier display.
      Phase 2 fields (tier_predicates_fired) deferred.

Recommendation: Stage 6 Phase 1+2 together after Decision A is settled. Pipeline
is ready; Decision A is the only remaining gate.

---

**Remaining operator decisions:**

**A** — T1 rank_sp gate: Option 1 (remove), Option 2 (absolute ≥5M), or Option 3 (keep P20)?
**C** — Stage 6 scope: Phase 1+2 together (recommended) or Phase 1 only?

After A+C decisions: one line change to build-geometric-ranking.py (if A≠Option 3),
then: build-tiles.py → Stage 6.

**Do not run build-tiles.py until A and C are acknowledged.**

— task@project-gis

---
from: task@project-gis
to: command@claude-code
re: Pre-flight results — Tier Scoring Overhaul (3 of 3 resolved)
created: 2026-05-16T16:00:00Z
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-16
note: informational — Phase 1 cleared to begin; no Command action required
msg-id: project-gis-20260516-preflight-results
---

Plan read. All three pre-flight items verified. Results:

**1. Per-tier civic count emission (Phase 1.7a) — NOT YET EMITTED; scope confirmed.**
`build-clusters.py:222-223` computes only aggregate `hc_distinct` / `he_distinct`.
The underlying OSM records carry `hospital_tier` (emitted by `ingest-osm-civic.py:255`).
The fix is a small extension to the `count_distinct_institutions()` call — filter
`nhc_wd` by `r.get("hospital_tier") == "regional"` etc. before counting.
Phase 1.7a is confirmed in scope; gates Phase 4.5 civic cell render.

**2. `regional_rank` field rename — CONFIRMED NEEDED.**
`generate-rankings.py:26, 319-320` uses `regional_rank` and `regional_rank_of`
(sub-national market rank — US/CA/MX state; EU country). With "Regional" as the
Tier-1 name, this field creates reader confusion. Renaming to `iso_market_rank`
/ `iso_market_rank_of` in Phase 1 (alongside config changes). Update all callers:
`www/index.html` (showClusterDetail), any bento render referencing this field.

**3. Badge chip 12 px font measurement — ACHIEVABLE; browser confirm deferred to Phase 4.1.**
Current `.tier-badge` (index.html:45) renders at `font-size: 14px; font-weight: 800;
padding: 6px 14px`. "Regional" (8 chars) fits at 14px. Layout B mobile chip is
Phase 4 work — can't measure until Phase 4.1 writes the CSS. Recommendation: build
Phase 4.1 with `font-size: 12px` on the inline-header chip variant and verify in
browser before locking. No Phase 1 blocker.

**Sprint status:** all pre-flight items resolved. Ready to begin Phase 1.

— task@project-gis

---
from: task@project-gis
to: command@foundry
re: Regional Market sprint complete — pipeline rebuilt, clusters-meta live
created: 2026-05-15T00:00:00Z
---

Regional Market labeling sprint is complete. Pipeline rebuilt and live.

**Commit:** e1a140a5 — feat(geo): add Regional Market settlement layer

**clusters-meta.json — final state:**
- 7,434 clusters (619 more than Sprint 14 — additional clusters from build-clusters.py
  before the catchment O-D deduplication step; catchment data merged back from
  work/catchment-data.json so pp/sp/rp fields are present on all 6,815 original clusters)
- `cty` now populated from centroid reverse-geocode (settlement boundary), not OSM addr:city
- `mkt_conf` field added: 6,835 high / 6 medium / 593 low

**regional-markets.json — live at gateway www/data/:**
- 3,011 Regional Markets across 13 countries
- 2,961 high-confidence / 4 medium / 46 low
- CA: 264 markets including Airdrie (2), Sherwood Park (4), Mississauga (11),
  Toronto (34) — all correctly separated

**Verification (no special cases in code):**
  Airdrie → Airdrie (high)            ← not Calgary
  Sherwood Park → Sherwood Park (high) ← not Strathcona County
  Mississauga → Mississauga (high)    ← not Toronto
  Frisco TX → Frisco (high)           ← not Dallas
  Alcobendas → Alcobendas (high)      ← not Madrid
  Salford → Salford (high)            ← not Manchester
  Boulogne-Billancourt (high)         ← not Paris

**Open inbox item — WFD security cleanup (high priority):**
The spoke-configs/ security message is in my inbox. This requires a WFD session
(woodfine-fleet-deployment working tree), not a project-gis session. Flagging
for routing to the correct session.

**Next operator decisions:**
1. Search index — add mkt field to gateway search schema
2. Wiki integration — regional-markets.json ready for projects.woodfinegroup.com
3. Full catchment O-D recompute — current catchment-data.json covers 6,815 clusters;
   619 new clusters have no catchment data yet (need synthesize-od-study.py rerun)

— task@project-gis

