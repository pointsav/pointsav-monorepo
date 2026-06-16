# NEXT.md — project-gis (Totebox)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-16

---

## Completed (Sessions 84+)

- [x] **PKS Phase 5b** — 7,045 clusters (T1=692/T2=2,665/T3=3,688); MX=177; false-US clusters removed [2026-06-16 totebox]
- [x] **park-and-ride anchor ingest** — 23,117 records [2026-06-16 totebox]
- [x] **EU/US car rental + hotel chain ingests** [2026-06-16 totebox]
- [x] **PKS archetype rebalanced** — Fable analysis + mode-group collapse [2026-06-15 totebox]
- [x] **VWH retail_contamination badge** — `showArchetypeDetail()` badge for 3,048/6,368 clusters [2026-06-13 totebox]
- [x] **A26 BRIEF updated** — §4 operator crontab checkboxes; first clean run 2026-06-13T05:48Z [2026-06-13 totebox]
- [x] **Root disk + BRIEF contamination outbox sent** — msg-id: project-gis-20260613-disk-brief-contamination [2026-06-13 totebox]

## Blocked — Command Session (route via outbox)

- [ ] **Stage 6 promotion** — ~17 commits ahead of canonical; Command runs `bin/promote.sh`
      msg-id: command-20260616-action-required-project-gis-stage-6-root
- [ ] **BRIEF contamination** — `git mv` 10 non-GIS BRIEFs to correct archives
      (msg-id: project-gis-20260613-disk-brief-contamination)
- [ ] **pairings.yaml + .owner files** for all gateways (A26 §4 Command scope)
- [ ] **cron-audit.sh** new validation script (A26 §4 Command scope)

## Active (Totebox scope)

- [ ] **AEC seismic script fixes** — CONUS 0-hit, EU ESHM20 0-hit, Zenodo tile URLs, VRT→TIF step
- [ ] **AEC flood URL fix** — AQUEDUCT 3.0 raster reference (bd17a348 URL)
- [ ] **gwl-fcs30-global.tif stub deletion** — work/aec/
- [ ] **ashrae_zone producer script** — no current script; Totebox scope
- [ ] **Nginx vhost SSE proxy** — add `/_api/edit/events` block for live reload:
      `proxy_buffering off; proxy_read_timeout 3600s; proxy_set_header Connection ''; proxy_http_version 1.1;`
