# NEXT.md — project-gis (Totebox)
# NEXT.md — project-knowledge (Totebox)

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
## Blocked — Command Session (route via outbox)

- [ ] **Stage 6 — sub-clone** (`pointsav-monorepo/`) — 8 commits pending:
      d6e0ff08 da07781e a51f201b 7879b8ed 50eda4b6 35fe9a2f 9cc1a80c 4de90703
      msg-id: command-20260615-stage-6-pending-project-knowledge-sub-cl
- [ ] **Stage 6 — archive** — 4 ops commits: 298ba52a 9eda459f 9fb431cb f11197ee
- [ ] **Post-Stage-6 TOML additions** (Command, `/etc/local-knowledge/*.toml`):
      `instance = "documentation"/"projects"/"corporate"` + `canonical_url` under `[site]`
- [ ] **Binary rebuild + service restart** after Stage 6:
      `~/Foundry/bin/deploy-binary.sh app-mediakit-knowledge`
      `sudo systemctl restart local-knowledge-{documentation,projects,corporate}`
- [ ] **check --strict gate** — F2/F3 dead links at project-editorial must resolve first

## Active (Totebox scope)

- [ ] **Design artifacts D1a–D1e** — DESIGN-* files in `.agent/drafts-outbound/`; dispatch to project-design
- [ ] **D2 orgcharts relay** — D2a–D2m in drafts-outbound originated from project-orgcharts;
      relay to project-design or confirm project-orgcharts has canonical copies before removing
- [ ] **F-series tracking** — F1–F7 content repair requests sent to project-editorial 2026-06-14;
      track responses; update artifact-registry.md Status column when returned

## Completed (Sessions 84+)

- [x] Phase 9: WCAG 2.2 focus outline, sitemap/i18n repairs, defects 1/4/8 — committed E6+E7
- [x] Phase 7 ActivityPub scaffold — committed E5 (6d554ec6)
- [x] Sprint C 7-category IA — 9cc1a80c
- [x] GET /images/{*path} route — da07781e
- [x] Sub-clone CLAUDE.md identity fixed (was project-console contamination) — a51f201b
- [x] Archive identity corrected in manifest + CLAUDE.md — 9fb431cb
- [x] .agent/briefs/ gitignore exclusion removed; BRIEFs now tracked — f11197ee
- [x] D1b draft renamed with DESIGN- prefix — 9eda459f
- [x] Master BRIEF updated (Sprint 0 row; 2026-06-15 work log) — 298ba52a
- [x] 12-agent external audit — F1–F7 content repair requests dispatched to project-editorial
- [x] check --strict: F2/F3 dead links identified
