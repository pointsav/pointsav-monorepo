# NEXT.md — project-console (cluster/project-console branch)

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Monorepo coding roadmap: `pointsav-monorepo/NEXT.md`.

Last updated: 2026-06-01 (Session 41 — Stage 6 complete; Phase 8 work queue updated).

---

## Stage 6 — complete (2026-06-01)

All three Phase 6/7/8 commits cherry-picked onto canonical via Command Session 40.
`HEAD = origin/main = 371e968c`. Prior staging-mirror push and promote-queue items superseded.

## Phase C/D/E — complete 2026-05-31

- [x] Phase C — EmailCartridge (F3): `app-console-email` converted to lib; inbox list +
  read + compose/send; `service-email` backend at `email_endpoint` (9093); workspace member;
  plain mode supported; registered in `os-console/src/main.rs`
- [x] Phase D — SlmCartridge (F9): `app-console-slm` converted to lib; Doorman health
  dashboard + entity count; 10s background poll + R manual refresh; `?` help overlay;
  workspace member; registered at F9
- [x] Phase E — Orchestration wiring: `mba_client.rs` audited (clean); `orchestration_host`
  + `email_endpoint` + `plain_mode` added to `ConsoleConfig`; zero `app-orchestration-command`
  references; `BRIEF-os-console-platform.md` §5 updated with full peer-field table
- [x] BRIEF consolidation: `BRIEF-project-console-master.md` created; 4 BRIEFs absorbed/
  superseded; `BRIEF-os-console-platform.md` port note fixed (9080 correct)

## Phase 6 — complete 2026-05-31

- [x] Offline detection: background health poll against `{slm_endpoint}/readyz` (30s interval); `tick()` drains; `self.offline` flag
- [x] `/new` blocked when offline → `ContentState::Error` "AI unavailable — Doorman offline"
- [x] Offline indicator in Input hint bar: `[⚠ AI OFFLINE — /new disabled]`
- [x] `/search <query>` command → `ContentState::SearchResults`; spawns thread; `GET {content_endpoint}/v1/search?q=...`; j/k navigate; Esc back
- [x] `content_endpoint` field added to `ConsoleConfig` (default `http://127.0.0.1:9081`)
- [x] `app-console-content/src/search.rs` new module; `cargo check` 0 errors

## Phase 7 — complete 2026-05-31

- [x] `app-console-content/src/pdf.rs`: `render_page(path, page_idx)` via `pdfium-render` 0.8 (binds libpdfium at runtime; A4 res 1240px); graceful error if libpdfium absent
- [x] `ContentState::PdfView` — bg render channel + cached `StatefulProtocol`; `/pdf <path>` command
- [x] `on_pdf_key`: j/k/PgUp/PgDn page nav (re-renders in bg thread); Esc to exit
- [x] `render_pdf_view`: Kitty/Sixel pixel render via `ratatui-image` 9; text fallback on terminals without graphics protocol
- [x] `Cartridge::set_graphics_caps(kitty, sixel, font_size)` trait method; chassis calls after probe
- [x] `image` dep trimmed to `default-features = false` (drops rav1e/avif; matches app-console-keys); `cargo check` 0 warnings

## Phase 8 — continuing (Session 41)

- [x] `/audit` verdict-log viewer — F12 InputCartridge; Ctrl-A opens `AuditLog` state; `audit::query_recent(200)` reads local `ingest_log` SQLite; j/k scroll; Esc back; status colour-coded (3c9e6c89, 2026-05-31)
- [ ] **Phase 8A** — 24-bit truecolor application (`TerminalCaps.truecolor` already probed; extend `set_graphics_caps` signature; use `Color::Rgb` vs `Color::Indexed` fallback in render paths)
- [ ] **Phase 8B** — OSC 8 hyperlinks on search result titles, audit log paths, protocol name spans (post-render injection via crossterm)
- [ ] **Phase 8C** — Session persistence — auto-save draft content to `~/.local/share/proof/content_session.db`; restore on reconnect (rusqlite pattern from `app-console-input/src/audit.rs`)
- [ ] **Phase 8D** — Multi-tab editing (`Ctrl-w n` new tab, `Ctrl-w h/l` navigate, `Ctrl-w c` close; `ContentState::MultiDraft` variant + tab bar render)
- [ ] **F2 People cartridge — BLOCKED.** `service-people` has no HTTP API. Contract requested
  from project-data via outbox `project-console-20260531-service-people-contract`. Build F2
  once project-data ships/defines the endpoint. Leave `app-console-people` Reserved until then.

## Post-Stage 6 — operator infra (Command/operator gated)

- [ ] **GCE firewall port 2222** — required for external MBA → `pairing-server` connections
- [ ] **pairing-server systemd unit** — unit file EXISTS at
  `infrastructure/systemd/ppn/local-ppn-pairing.service` (ExecStart: `/usr/local/bin/service-ppn-pairing`);
  binary name discrepancy flagged to Command (unit says `service-ppn-pairing`; Cargo provides
  `pairing-server` from `system-gateway-mba` and `ppn-pairing-server` from `service-ppn-pairing`
  — awaiting Command confirmation of which binary to install)
- [ ] **Peter SSH key + proofctl user add** — post-Stage 6; Peter needs SSH key committed to authorized_keys
- [ ] **Tag v0.1.0 on pointsav-monorepo** — triggers GitHub Actions release (os-console + pairing-server + proofctl)

## Completed (archive reference)

- [x] Phase 1–5 coding complete; Phases 1+2 on canonical; Phases 3–5 on canonical via Stage 6
- [x] NEXT.md contamination cleared by Command 2026-05-28 (was project-infrastructure content)
- [x] Stage 6 complete 2026-06-01 (Command Session 40 — 3 commits cherry-picked to canonical; HEAD = origin/main = 371e968c)
