# NEXT — app-workplace-http-prototype

## Hot items

- [ ] Restart `local-workplace-http-prototype.service` (or `cargo run`)
      and smoke-test Stage 1 Memo end-to-end from browser at
      `http://10.8.0.1:9110`

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
