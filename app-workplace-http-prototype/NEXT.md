# NEXT — app-workplace-http-prototype

## Hot items

- [x] SSE real-time file events — `/api/files/events`; inotify watcher; index +
      memo auto-refresh on change (fab7a2f6)
- [x] nginx 9200 proxies prototype root — `http://10.8.0.9:9200/` is now the
      daily-use URL; prototype binds on 9110 internally
- [x] Workbench merge — SPA + backend at `/workbench/`; config.toml roots;
      /files/ and 9210 standalone untouched (ab75fa69)
- [ ] Jennifer tests `/workbench` surface; confirm before retiring /files/ and 9210
- [ ] Add systemd unit `local-workplace-http-prototype.service` so the binary
      survives session exit (currently launched manually)

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
