# NEXT.md — project-console (cluster/project-console branch)

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Monorepo coding roadmap: `pointsav-monorepo/NEXT.md`.

Last updated: 2026-05-28 (Command Session — replaced erroneous project-infrastructure content).

---

## Stage 6 — monorepo sub-clone

- [ ] **Push staging mirrors** — `git push --force-with-lease origin-staging-j main` +
  `origin-staging-p main`. Authorized by Command Session 2026-05-28 (inbox msg
  `command-20260528-console-answers`). project-proforma commits on staging-j confirmed
  safe in canonical. **Includes Phases C/D/E + BRIEF consolidation commits from this session.**
- [ ] **Write promote-queue.jsonl** after staging push — Command will merge to canonical.

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

## Next coding phase

- [ ] **Phase 8 — Polish** — OSC 8 hyperlinks; truecolor; multi-tab editing; session persistence; `/audit` verdict-log viewer; F2 People cartridge

## Stage 6 — monorepo sub-clone

- [ ] **GCE firewall port 2222** — required for external MBA → `pairing-server` connections
- [ ] **pairing-server systemd unit** — deploy on VM alongside SSH; unit file missing from `infrastructure/`
- [ ] **Peter SSH key + proofctl user add** — post-Stage 6; Peter needs SSH key committed to authorized_keys
- [ ] **Tag v0.1.0 on pointsav-monorepo** — triggers GitHub Actions release (os-console + pairing-server + proofctl)

## Completed (archive reference)

- [x] Phase 1–5 coding complete; Phases 1+2 on canonical; Phases 3–5 awaiting Stage 6
- [x] NEXT.md contamination cleared by Command 2026-05-28 (was project-infrastructure content)
