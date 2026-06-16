# NEXT.md — project-knowledge (Totebox)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-16

---

## Blocked — Command Session (route via outbox)

- [ ] **check --strict gate** — F2/F3 dead links at project-editorial must resolve first

## Active (Totebox scope)

- [ ] **Nginx vhost SSE proxy** — add `/_api/edit/events` block for live reload:
      `proxy_buffering off; proxy_read_timeout 3600s; proxy_set_header Connection ''; proxy_http_version 1.1;`
- [ ] **Design artifacts D1a–D1e** — DESIGN-* files in `.agent/drafts-outbound/`; dispatch to project-design
- [ ] **D2 orgcharts relay** — D2a–D2m in drafts-outbound originated from project-orgcharts;
      relay to project-design or confirm project-orgcharts has canonical copies before removing
- [ ] **F-series tracking** — F1–F7 content repair requests sent to project-editorial 2026-06-14;
      track responses; update artifact-registry.md Status column when returned

## Completed (Sessions 84+)

- [x] **Stage 6 — sub-clone + archive** — promoted Session 86 (→ d0abd9ad) [2026-06-16 command]
- [x] **Post-Stage-6 TOML + binary rebuild** — instance/canonical_url added; binary rebuilt; all 3 instances healthy [2026-06-16 command]
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
