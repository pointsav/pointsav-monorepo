# NEXT.md — project-gis (Totebox)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-17

---

## Active (Totebox scope)

- [ ] **Post-overnight build verification** — after build completes, run verification checks:
      PKS T1 count (expect > 692 with park_ride data), park_ride coverage (expect > 0/7,045),
      EU seismic cluster count (expect > 0 after Step 8 logic fix), flood layer freshness
      (layer11 newer than 2026-06-10). Commands in plan file. [2026-06-17 totebox@claude-code]
- [ ] **EU seismic (EFEHR)** — maps.efehr.org NXDOMAIN; Step 8 OR→two-if bug fixed 2026-06-17
      (build-aec-seismic.sh); ESHM20 tarball from gitlab still produces 1-feature metadata
      GeoJSON (Step 7 issue); EU seismic PMTiles will remain sparse until tarball issue resolved.
      [2026-06-17 totebox@claude-code]
- [ ] **F-series tracking** — F1–F7 content repair requests sent to project-editorial 2026-06-14;
      track responses; update artifact-registry.md Status column when returned
      [2026-06-16 totebox]

## Blocked — Command Session (route via outbox)

- [ ] **Stage 6 READY** — 3 script fixes (overnight path + seismic EU join + AQUEDUCT threshold)
      committed tonight; await Stage 6 from Command. [2026-06-17 totebox@claude-code]
- [ ] **push-to-prod.sh gis** — after post-overnight verification passes; Command Session only.
      [2026-06-17 totebox@claude-code]
- [ ] **check --strict gate** — F2/F3 dead links at project-editorial must resolve first
      [2026-06-17 command@claude-code]

## Completed (Sessions 84+)

- [x] **overnight-aec-builds.sh path fix** — ingest-osm-parking.py was called from wrong dir;
      now uses `../pointsav-monorepo/app-orchestration-gis/ingest-osm-parking.py` [2026-06-17 totebox@claude-code]
- [x] **build-aec-seismic.sh EU join fix** — Step 8 `or` condition split into two separate `if`
      guards; EU vector join was skipping all clusters [2026-06-17 totebox@claude-code]
- [x] **build-aec-flood.sh AQUEDUCT threshold fix** — lowered from 100MB to 85MB; S3 file is
      92MB, causing every clean download to fail validation [2026-06-17 totebox@claude-code]
- [x] **PKS Phase 5b** — 7,045 clusters (T1=692/T2=2,665/T3=3,188); MX=177; false-US removed
      [2026-06-16 totebox]
- [x] **park-and-ride anchor ingest** — 23,117 records [2026-06-16 totebox]
- [x] **EU/US car rental + hotel chain ingests** [2026-06-16 totebox]
- [x] **PKS archetype rebalanced** — Fable analysis + mode-group collapse [2026-06-15 totebox]
- [x] **VWH retail_contamination badge** — showArchetypeDetail() badge for 3,048/6,368 clusters
      [2026-06-13 totebox]
- [x] **AEC wetland VRT fix** — 408 GWL_FCS30 5°-tiles assembled; gdal_translate removed (9c041f65)
      [2026-06-16 totebox]
- [x] **AEC wildfire numpy fix** — pure Python GDAL API (2ea45b07) [2026-06-16 totebox]
- [x] **ashrae_zone producer script** — build-ashrae-zone.py; 6,493/6,493 populated (dce0d157)
      [2026-06-16 totebox]
- [x] **PKS opportunity_class field** — SATURATED/EXPAND/DEVELOP per BRIEF §10.12 (2ea45b07)
      [2026-06-16 totebox]
- [x] **EFEHR seismic API (sample-eshm20-api.py)** — maps.efehr.org NXDOMAIN; main build script
      uses gitlab.seismo.ethz.ch tarball (unaffected); sample script is dev-only [2026-06-16 totebox]
- [x] **NEXT.md contamination (M-17)** — project-knowledge + project-intelligence content removed;
      GIS-only content restored [2026-06-17 totebox@claude-code]
