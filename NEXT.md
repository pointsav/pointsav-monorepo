# NEXT.md — project-workplace

Open items, deferred work, and follow-ups. Attribution: `[YYYY-MM-DD role@engine]`.
Backlog (lower priority) at `.agent/next-backlog.md`.

---

## Hot items

- [ ] **Stage 6**: commits `923b5171` (sub-clone .agent/ untrack) + `7152333f` (live reload) +
  `f00e676a` (cargo fmt) + `ec305edc` (clippy strip_prefix) + `646462ec` (folder ZIP, Session 8)
  unpromoted — Command Session signal received; run `bin/promote.sh` + binary ledger entry for
  `app-privategit-workbench`. [2026-06-09 totebox@claude-code]

- [ ] **Nginx vhost (operator/Command)**: add SSE proxy block for live reload:
  `location /_api/edit/events { proxy_buffering off; proxy_read_timeout 3600s;`
  `proxy_set_header Connection ''; proxy_http_version 1.1; }` [2026-06-09 totebox@claude-code]

- [x] **Stage 6**: `d451dcd2` + `7870683f` + `6866eb3a` (workbench drag+undo) — promoted 810a2277. [2026-06-04]

- [x] **Stage 6**: `683fc671` + `3ffaa8f6` (proforma) — promoted. [2026-06-03]

- [x] **Stage 6**: `3768ba89` (Memo Session 1) — promoted. [2026-06-03]

- [x] **HTTP prototype Stage 1 — Memo**: complete. [2026-05-28]

- [x] **HTTP prototype Stage 2 — Proforma**: complete + enhanced (v2.0 schema, metadata,
  column labels/formats, theme toggle, AutoSum, AVERAGE/MIN/MAX/COUNT). [2026-06-02]

- [x] **BIM schema — W3C DTCG compliance**: fixed $schema URI, flat tokens, $extensions.
  [2026-06-02]

- [x] **Selection bug — org-charts SVG wireBox**: fixed in project-orgcharts 705a86d9.
  [2026-06-01]

- [ ] **HTTP prototype Stage 3 — Presentation**: add `/presentation` route +
  `presentation.html`; slide-based JSON schema; add/remove/reorder slides; text + image
  per slide; save. Technology: custom HTML/CSS/JS canvas. [2026-06-03 totebox@claude-code]

- [ ] **Workbench drag-drop — cross-archive moves**: moving a file between two different
  `_clones/project-*` roots uses `fs::copy` + `rm` (non-atomic). Consider alerting
  the user if the move crosses a git boundary (not git-tracked). [2026-06-04 totebox@claude-code]

- [ ] **Memo save location chooser**: plan ready at `/home/jennifer/.claude/plans/`; not
  started. [2026-06-04 totebox@claude-code]

---

## Active development — Wave 1

- [ ] `app-workplace-launcher` (P1): create crate; three-pane navigator; file-schema
  dispatch; IPC contract (`workplace-launcher.sock`; CBOR; `Hello` + `OpenDocument` +
  `Heartbeat` + `Quit`). [2026-05-27 totebox@claude-code]

- [ ] `app-workplace-memo` (P2): Tauri v1 → v2 migration; launcher `Hello` handshake;
  autosave; macOS `x86_64-apple-darwin` build clean.

- [ ] `app-workplace-presentation` (P5): Tauri v1 → v2 migration; launcher handshake;
  presenter display mode.

---

## Deferred / blocked

- [ ] macOS prerequisites walkthrough for Jennifer (Rust, Node.js, Xcode CLT, first
  `app-workplace-launcher` build) — awaiting Jennifer to have a Mac available.
  [2026-05-28 totebox@claude-code]

- [ ] `app-workplace-schedule` (P4.5): create crate; not in registry. Reserve-folder →
  Scaffold-coded when Schedule JSON schema spec is written. See BRIEF §8 for Day-1
  feature set (10 items). NOT a calendar. [2026-05-27 totebox@claude-code]

- [ ] `app-workplace-proforma`: resolve "local-only CLAUDE.md" status; Tauri v1 → v2.

- [ ] `app-workplace-pdf` (P3): create crate; pdfium-render; annotation sidecar.

- [ ] `app-workplace-gis` (P6): create crate; MapLibre GL JS; OpenFreeMap PMTiles.

- [ ] `app-workplace-bim` (P7 — gated): awaiting xeokit commercial licence quote from
  Creoox. [2026-05-27 totebox@claude-code]

- [ ] `app-workplace-code`: not in registry; parallel track; does not block Sprint 1.

- [ ] BRIEF-workplace-desktop-environment.md: section numbering gap §2 → §4 (skips §3).
  Minor; low priority. [2026-05-27 totebox@claude-code]

- [ ] Schedule JSON schema spec (`app-workplace-schedule/docs/schema.md`) — TBD when
  crate is created.

- [ ] Presentation JSON schema spec — TBD when crate is created.

- [ ] macOS notarization certificate — needed for Wave 1 distribution via
  project-software. [2026-05-27 totebox@claude-code]

- [ ] Pairing-server port for `system-gateway-mba`: TBD.
