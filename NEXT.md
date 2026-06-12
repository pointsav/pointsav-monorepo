# NEXT — project-gis

Deferred, blocked, and follow-up items. Attribution: `[YYYY-MM-DD role@engine]`.

---

## PKS Commuter archetype — overnight data tasks

- [x] **Fix crontab** `[2026-06-11 totebox@claude-sonnet-4-6]` DONE 2026-06-12
  All 3 entries updated: project-orgcharts → project-gis. Nightly rebuild now runs calibrated
  build-vwh-clusters.py and build-pks-clusters.py from project-gis.

- [x] **Park-and-ride ingest for US/CA/DE/FR/IT/PL/NO** `[2026-06-11 totebox@claude-sonnet-4-6]` DONE
  17,721 park_ride records. Result: 6,649 features T1=462/T2=2,451/T3=3,736.

- [x] **EU car rental ingests** `[2026-06-11 totebox@claude-sonnet-4-6]` DONE
  hertz-eu=687, avis-eu=741, budget-eu=130, europcar-eu=1,021, budget-us=278, alamo-us=110, national-us=2, sixt-eu=246.
  All in CAR_RENTAL_CHAINS in build-pks-clusters.py.

- [x] **Hotel chain YAMLs for PKS** `[2026-06-11 totebox@claude-sonnet-4-6]` DONE
  ibis-eu=708, premier-inn-gb=817, travelodge-gb=580, motel-one-de=24,
  holiday-inn-express-us=2,021, hampton-us=240, courtyard-us=1,020, b-and-b-hotels-eu=797. Total: 6,207.
  `hotel` enrichment class added to _enrich_classes() + tier_pks() + build() in build-pks-clusters.py.

- [x] **PKS co-location calibration converged + deployed** `[2026-06-11 totebox@claude-sonnet-4-6]` DONE
  5 sim iterations + production build. Final: 6,953 clusters (T1=9.9%, T2=38.2%, T3=51.9%).
  Key insight: park-and-ride (23,117 records) is the discrete geographic anchor; EPS_LOOSE=2.5km
  prevents rail network collapse. Scripts: analyze-parkade-colocation.py + sim-pks-colocation.py.
  Gateway deployed: /srv/foundry/deployments/gateway-orchestration-gis-1/www/data/archetype-pks.geojson
- [x] **PKS `hotel` property added to GeoJSON output** `[2026-06-12 totebox@claude-sonnet-4-6]` DONE
  `build-pks-clusters.py` line 604 was missing `"hotel": "hotel" in cats` in the properties dict.
  Hotel enrichment affected tier logic but was not written to the output file. Added, re-ran, redeployed.
  Final: hotel=1,221 / car_rental=1,311 of 6,953 clusters.
- [x] **VWH/PKS gateway redeployment after nightly-rebuild overwrite** `[2026-06-12 totebox@claude-sonnet-4-6]` DONE
  Nightly rebuild at 05:00 UTC Jun 12 ran from project-orgcharts (old uncalibrated scripts) and
  overwrote gateway: VWH=7,028 (wrong), PKS=6,215 (wrong). Redeployed correct work/ files:
  VWH=6,368 (T1=852/T2=1,327/T3=4,189; retail_contamination=3,048 ✓)
  PKS=6,953 (T1=691/T2=2,658/T3=3,604; hotel=1,221 ✓)

- [ ] **PKS opportunity scoring (next step)** `[2026-06-11 totebox@claude-sonnet-4-6]`
  With T1/T2/T3 co-locations established, add opportunity class per cluster:
  - DEVELOP: commercial enrichment thin (T3 or single enrich) → parkade absent or underbuilt
  - EXPAND: co-location present (T2), rental + hotel signal → capacity may need expansion
  - SATURATED: T1 hub with full commercial ecosystem → supply likely meets demand
  Implement in build-pks-clusters.py as second-pass property after tier assignment.

- [x] **VWH co-location calibration converged + deployed** `[2026-06-11 totebox@claude-sonnet-4-6]` DONE
  Profile (analyze-vwh-colocation.py): 10,338 hardware anchors; hypermarket 73.9% contamination
  signal; auto_parts 51.2%; CBD filter NOT viable (73.6% >30km from metro refs).
  Sim (sim-vwh-colocation.py): 1 iteration; 1,555 clusters; hardware validation 73.4% PASS.
  Production build-vwh-clusters.py updated: qualify_vwh + tier_vwh replaced with group-collapse.
  Final: 6,368 clusters (T1=13.4%, T2=20.8%, T3=65.8%). Deployed to gateway.
  Key insight: T3-heavy is correct for VWH (hardware-alone = thin T3; true trade hubs are rare).
  retail_contamination flag added (47.9% of clusters have hypermarket <1km — informational).

- [ ] **VWH retail_contamination UX (next step)** `[2026-06-11 totebox@claude-sonnet-4-6]`
  47.9% of VWH clusters are flagged retail_contamination (hypermarket <1km).
  Decision: display all clusters on map, use flag to differentiate in infobox (e.g. "Mixed-use site"
  badge vs "Trade hub" for clean VWH sites). Requires UX design pass in project-design.

---

## Artifact registry cleanup

- [ ] **Remove stale J1 v0.1 EN stub from JOURNAL/** `[2026-06-12 totebox@claude-sonnet-4-6]`
  `JOURNAL/JOURNAL-retail-colocation-v0.1.draft.md` is superseded by v0.5 in same directory.
  ES sibling `JOURNAL-retail-colocation-v0.1.es.draft.md` is intentionally still v0.1 (ES version not yet updated to v0.5 content).
  Removal: `git rm JOURNAL/JOURNAL-retail-colocation-v0.1.draft.md` (file is git-tracked — confirmed).

---

## Stage 6 and promotion

- [ ] **Promote commit 91b354ff** `[2026-06-11 totebox@claude-sonnet-4-6]`
  Run from Command Session: `~/Foundry/bin/promote.sh`
  (CLAUDE.md + .gitignore + 10 .agent/ tracked deletions)

---

## Editorial dispatch

- [ ] **B19/B20/B21 relay** `[2026-06-11 totebox@claude-sonnet-4-6]`
  Files confirmed in drafts-outbound/; outbox msg-id project-gis-20260609-editorial-dispatch-b19-b20-b21 present.
  Now registered as A22 (TOPIC-location-intelligence-archetypes EN+ES), A23 (GUIDE-gis-aec-pipeline-repair),
  A24 (GUIDE-gis-nightly-build-operations). Command Session needs to pick up and relay to project-editorial.

---

## Infrastructure

- [ ] **app-orchestration-gis scripts version control** `[2026-06-11 totebox@claude-sonnet-4-6]`
  Scripts in gitignored `pointsav-monorepo/app-orchestration-gis/` are unversioned.
  Options: (a) git init the subdirectory as a standalone repo, or
           (b) promote key scripts to the tracked `app-orchestration-gis/` at repo root.
  Affects: build-pks-clusters.py (PKS tier changes this session), nightly-rebuild.sh, etc.
