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

