# NEXT.md — project-knowledge (Totebox)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-16

---

## Blocked — Command Session (route via outbox)

- [ ] **check --strict gate** — F2/F3 dead links at project-editorial must resolve first
- [ ] **Nginx vhost SSE proxy** — add `/_api/edit/events` block to nginx vhost for live reload:
      `proxy_buffering off; proxy_read_timeout 3600s; proxy_set_header Connection ''; proxy_http_version 1.1;`
      (VM sysadmin scope)

## Active (Totebox scope)

- [x] **Cargo check + test gate** — 200 tests, 0 failed; cargo check OK; 14.9 GB available [2026-06-16 totebox@claude-code]
- [x] **Stage 6 READY signal** — sent to Command (msg-id: command-20260617-stage-6-ready-app-mediakit-knowledge-bin); sub-clone canonical HEAD `3d90e76d` [2026-06-16 totebox@claude-code]
- [x] **Design artifacts D1a–D1e** — dispatched to project-design via send_mailbox_message (msg-id: command-20260616-design-artifacts-dispatch-knowledge-plat) [2026-06-16 totebox@claude-code]
- [x] **D2 orgcharts relay** — D2a–D2m relayed to project-design via send_mailbox_message (msg-id: command-20260616-cross-archive-relay-d2a-d2m-orgchart-wor); GUIDE-orgchart-authoring.draft.md flagged for project-editorial [2026-06-16 totebox@claude-code]
- [x] **Sprint H** — ActivityPub wiring: FederationConfig + AppState.activitypub_outbox_url + on_article_saved() wired into content-dir file watcher; committed `2c0ed559`; Stage 6 READY pending [2026-06-16 totebox@claude-code]
- [ ] **F-series tracking** — F1–F7 content repair requests sent to project-editorial 2026-06-14; track responses; update artifact-registry.md Status column when returned [2026-06-16 totebox]

## Completed (this session, 2026-06-16)

- [x] **Sprint D** — home page peer-band (`aside.peer-band`) in `home_handlers.rs`; CSS in `style.css`; peers threaded into `home_chrome()` [2026-06-16 totebox@claude-code]
- [x] **Defect 2** (footnotes CSS) — `sup/sub/footnotes` CSS prophylactically added to `style.css` [2026-06-16 totebox@claude-code]
- [x] **M13 /openapi.json** — `GET /openapi.json` route added → 301 redirect to `/openapi.yaml` [2026-06-16 totebox@claude-code]
- [x] **Sprint E** — `audience` + `aliases` fields added to `render::Frontmatter` (correct) + `walker::Frontmatter`; audience chips in `wiki_chrome`; `resolve_alias_slug()` + 301 alias redirect [2026-06-16 totebox@claude-code]
- [x] **Sprint F** — engine version in `shell_footer()`; `.peer-strip` cross-instance nav in `wiki_chrome`; doc-header CSS gap noted (already in HTML, was missing CSS — `.site-footer__trademark` CSS structure confirmed) [2026-06-16 totebox@claude-code]
- [x] **Sprint G** — `search_complete` returns `{title,slug,lede}`; `search_page` accepts `?category=` + `?status=` filters (post-search) [2026-06-16 totebox@claude-code]

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
