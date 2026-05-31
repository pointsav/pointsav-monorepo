# NEXT — app-workplace-http-prototype

## Hot items

- [x] SSE real-time file events — `/api/files/events`; inotify watcher; index +
      memo auto-refresh on change (fab7a2f6)
- [x] nginx 9200 proxies prototype root — `http://10.8.0.9:9200/` is now the
      daily-use URL; prototype binds on 9110 internally
- [x] Workbench merge — SPA + backend at `/workbench/`; config.toml roots;
      /files/ and 9210 standalone untouched (ab75fa69)
- [x] Jennifer confirmed `/workbench` working — /files/ retired (301 → /workbench)
- [x] Surface toolbar — 8 surfaces in workbench header; `/` redirects to
      `/workbench`; home page retired (4fc7b341)
- [x] File tree truncation — 75-item cap + "Show N more…" button (6b4940a6)
- [x] Pin stability audit — path-based unpin; apiPath guard; pinFolder
      validation (6b4940a6)
- [x] File tree timestamps — always shows date + time; 5-mode sort cycle
      (A-Z / Date / Time / Mod / Size) in sidebar toolbar (6afbbb8a)
- [ ] Retire `app-privategit-workbench` service (9210) — pending: switch
      `/_api/edit/` nginx proxy from 9210 → 9110/workbench + update config.toml
      url_prefix values to match SPA browse paths (_sandbox-jennifer, _command, _clones)
- [ ] Add systemd unit `local-workplace-http-prototype.service` so the binary
      survives session exit (currently launched manually)
- [ ] Auto-refresh tree on background file changes — SSE watcher; clear
      `dataset.loaded` + re-render expanded dirs on "changed" event
- [ ] File-click delay — split `Promise.all` into independent fetches; show
      viewer from `viewerP` immediately without waiting for `editorP`
- [ ] Existing broken pins (apiPath="") — user needs to unpin + re-pin from
      file tree; no automatic migration

## Stage 2 — Proforma

- [ ] Add `/proforma` route + `proforma.html` surface
- [ ] File format: `.json` (Proforma JSON schema per `tool-proforma`)
- [ ] SYS-ADR-07: all formula evaluation in Rust, never through slm

## Stage 3 — Presentation

- [ ] Add `/presentation` route + `presentation.html`
- [ ] File format: `.json` slide deck

## Stage 4–8 — Schedule / Code / PDF / GIS / BIM

- [ ] Scaffold per BRIEF §5

## Infrastructure

- [ ] Add systemd unit `local-workplace-http-prototype.service`
- [ ] Add to `app-privategit-workbench` config.toml as a linked surface
      (optional convenience shortcut)
