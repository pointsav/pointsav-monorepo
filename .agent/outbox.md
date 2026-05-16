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
status: pending
msg-id: project-gis-20260516-phase2-diff-pause
---

Phase 2 geometric ranking engine has been written and run against the Phase 1
cluster build. Diff harness output below. **Operator acknowledgment required
before Stage 6 and build-tiles.py run.**

**Pipeline state:** 9,234 clusters after dedup. Per-ISO ranks loaded from
catchment-data.json (6,794 of 9,234 clusters have catchment data; 2,440
new Phase 1 clusters from Mercadona/Tesco/Sainsbury's have no catchment
data yet — will be populated on next synthesize-od-study.py run).

---

### Contingency Table: rank_v2 × tier (V3)

```
   v2\tier        T1        T2        T3        T4 (Fringe)
      v2=0         0       133       243          2534     (no catchment data = Fringe)
      v2=1        14       289      1211          3206
      v2=2         3        76       497           917
      v2=3         0         3        43            65
```

---

### Mountain View — EXPECTED demotion confirmed

`c_walmart_us_37x401__122x11` (Mountain View, CA, US):
- V2 rank: T0 Border (rank_v2=0) — was never V2 Apex, so no demotion
- V3 tier: **T2 District** ✓
- pred: `["T2:composition", "T2:rank_pp", "T2:rank_spend", "T2:civic_hospital", "T2:iou"]`
- rank_pp_iso: 0.1765 (top 18% in US by primary population)
- hc_count_regional: 2

Mountain View is correctly T2 District under V3. The anticipated demotion is confirmed.

---

### Test matrix results

| Location | Expected | Actual V3 | Notes |
|---|---|---|---|
| Mountain View | T2 District | **T2 District** ✓ | Confirmed demotion |
| Mississauga (Walmart+Costco at 43x544) | T1 Regional | **T2 District** | rank_sp=0.22 — just above P20 threshold; passes all other T1 gates |
| Sherwood Park (Costco+HomeDepot) | T1 Regional | **T3 Local** | rank_pp=0.44; composition lacks Hypermarket co-tenant for T1/T2 |
| Madrid Salamanca (Mercadona) | T1 Regional | **T4 Fringe** | No catchment data for new Mercadona clusters; needs synthesize-od rerun |
| Anderlecht (IKEA-nl) | T2 District | **T4 Fringe** | rank_pp=1.0 — no catchment data for EU IKEA-nl cluster |
| Camden, NJ (Walmart) | T3 Local | **T4 Fringe** | rank_pp=0.82; no Warehouse co-tenant; fails T3 |

---

### Operator decisions needed

**Decision A — T1 rank_sp threshold: keep P20 or relax to P25?**
Mississauga Walmart+Costco has rank_sp=0.22 (22nd percentile), missing T1 by 2
percentage points. Relaxing to P25 would promote this cluster to T1 Regional.
Recommendation: relax to P25 (rank_sp is a secondary population gate; P20 was
conservative; P25 is more consistent with P25 used for T2 primary gate).

**Decision B — Catchment data gap for new Phase 1 chains: run synthesize-od first?**
Mercadona, Tesco, Sainsbury's clusters have no catchment data and cannot be tiered
by the geometric engine. These will all score Fringe until synthesize-od-study.py
is re-run. synthesize-od-study.py takes ~3–4 hours on this hardware.
Recommendation: run synthesize-od before build-tiles.py; accept temporary Fringe
assignment if a partial deploy is acceptable.

**Decision C — Stage 6 scope: Phase 1 only, or Phase 1+2 together?**
Phase 1 code (4-class taxonomy, BentoBox Layout A/B, per-tier civic counts) is
committed and tested. Phase 2 engine (build-geometric-ranking.py) is committed
but has the above known gaps. Options:
  (i) Stage 6 Phase 1 only: run build-tiles.py now; defer Phase 2 tier field until
      catchment data gap is resolved. Bento will use V2 rank_3km for colors.
  (ii) Stage 6 Phase 1+2 together: run synthesize-od first (~4h), then full pipeline.
Recommendation: Stage 6 Phase 1 only now; Phase 2 Stage 6 after catchment rerun.

---

**This message requires operator acknowledgment before build-tiles.py is run
and before Stage 6. Do not proceed to build-tiles.py automatically.**

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

