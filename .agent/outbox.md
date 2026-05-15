---
mailbox: outbox
owner: task@project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-gis

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

