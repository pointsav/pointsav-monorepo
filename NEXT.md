# NEXT — project-gis

Deferred, blocked, and follow-up items. Attribution: `[YYYY-MM-DD role@engine]`.

---

## PKS Commuter archetype — overnight data tasks

- [ ] **Fix crontab** `[2026-06-11 totebox@claude-sonnet-4-6]`
  Run as mathew: `crontab -l | sed 's|project-orgcharts|project-gis|g' | crontab -`
  Fixes entries 1 (nightly-rebuild.sh) and 3 (build-aec-global.sh) from project-orgcharts → project-gis.
  Entry 2 (run-overnight-ingests.sh June 4) is past — ignore.

- [ ] **Park-and-ride ingest for US/CA/DE/FR/IT/PL/NO/IS** `[2026-06-11 totebox@claude-sonnet-4-6]`
  Script: `ingest-osm-parking.py`; all countries in TILE_GRIDS; US takes ~12 tiles.
  Run after 05:00 UTC (10pm Vancouver):
  ```
  cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
  nohup python3 ingest-osm-parking.py --countries US CA DE FR IT PL NO IS >> ingest.log 2>&1 &
  ```
  Then re-run build-pks-clusters.py. Expected: ~6,500-7,000 features, T1~15% T2~40% T3~45%.

- [ ] **EU car rental ingests** `[2026-06-11 totebox@claude-sonnet-4-6]` (Fable recommendation)
  Priority order:
  1. Generic `car-rental-osm.yaml` (`amenity=car_rental`, all 18 countries) — single biggest EU fix
  2. `sixt-eu.yaml` — expand existing sixt-de.yaml bbox from DE to EU
  3. `budget-us.yaml` (Q1004913), `national-us.yaml`, `alamo-us.yaml` — NA airport counter brands
  4. `avis-eu.yaml`, `hertz-eu.yaml` — EU footprints of existing NA brands
  After: re-run build-pks-clusters.py; expected ~100-300 more airports upgrading to T1.

- [ ] **Hotel chain YAMLs for PKS** `[2026-06-11 totebox@claude-sonnet-4-6]` (new category, post car-rental)
  Brands: ibis-eu (Q920166, incl. Ibis Budget), b-and-b-hotels-eu (Q794939),
  premier-inn-gb (Q2108626), travelodge-gb, motel-one-de (Q866334),
  holiday-inn-express-us (Q5880423), hampton-us (Q5646184), courtyard-us (Q1053170).
  Add `hotel` enrichment class to build-pks-clusters.py after YAMLs ingested.

---

## Stage 6 and promotion

- [ ] **Promote commit 91b354ff** `[2026-06-11 totebox@claude-sonnet-4-6]`
  Run from Command Session: `~/Foundry/bin/promote.sh`
  (CLAUDE.md + .gitignore + 10 .agent/ tracked deletions)

---

## Editorial dispatch

- [ ] **B19/B20/B21 relay** `[2026-06-11 totebox@claude-sonnet-4-6]`
  Files confirmed in drafts-outbound/; outbox msg-id project-gis-20260609-editorial-dispatch-b19-b20-b21 present.
  Command Session needs to pick up and relay to project-editorial.

---

## Infrastructure

- [ ] **app-orchestration-gis scripts version control** `[2026-06-11 totebox@claude-sonnet-4-6]`
  Scripts in gitignored `pointsav-monorepo/app-orchestration-gis/` are unversioned.
  Options: (a) git init the subdirectory as a standalone repo, or
           (b) promote key scripts to the tracked `app-orchestration-gis/` at repo root.
  Affects: build-pks-clusters.py (PKS tier changes this session), nightly-rebuild.sh, etc.
