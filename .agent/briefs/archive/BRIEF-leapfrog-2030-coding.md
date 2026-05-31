---
schema: foundry-plan-v1
archive: project-console
title: "Leapfrog 2030 — os-console Coding Roadmap"
created: 2026-05-20
updated: 2026-05-23
status: superseded
superseded_by: BRIEF-project-console-master.md
superseded: 2026-05-31
authors: [totebox@project-console, claude-sonnet-4-6]
doctrine_anchors: [claim-45, claim-49, claim-54, SYS-ADR-07, SYS-ADR-10, SYS-ADR-19]
supersedes: "~/.claude/plans/can-you-make-a-deep-naur.md (standalone proofreader approach)"
companion: BRIEF-os-console-platform.md
---

# Leapfrog 2030 — os-console Coding Roadmap

## Purpose

Phased coding plan for `os-console` + `app-console-*` cartridge system.
**Chassis-first:** Phase 1 builds `app-console-keys` chassis. The proofreader
workflow (formerly standalone) becomes Phase 3 — a cartridge of the chassis.

Read `BRIEF-os-console-platform.md` for architecture. This document is the
implementation sequence only.

---

## Phase 0 — Spike `[COMPLETE 2026-05-17]`

Committed: `feat: Session 1 — russh + ratatui spike; SSH TUI skeleton on port 2222`

- [x] `pointsav-monorepo/Cargo.toml` workspace created
- [x] `app-console-content/Cargo.toml`: ratatui 0.30, crossterm 0.28, russh 0.60, rand 0.10, tokio, anyhow
- [x] `app-console-content/src/main.rs` — russh 0.60 SSH server + ratatui frame render
- [x] `app-console-content/src/auth.rs`, `db.rs` — SSH fingerprint auth + SQLite users table
- [x] `app-console-content/src/bin/proofctl.rs` — admin CLI stub
- [x] `cargo build` green
- [x] **Gate passed:** ratatui frame renders over SSH port 2222 via `ssh -p 2222 -i ~/.ssh/google_compute_engine mathew@localhost`

---

## Phase 1 — Chassis `[COMPLETE 2026-05-21]`

`app-console-keys` lib crate + `os-console` bin created; Phase 0 spike refactored into cartridge architecture.
Commits: af462797, 480dd105 (session 5, 2026-05-21)

- [x] Create `app-console-keys/` lib crate
  - `Cartridge` trait (fkey, title, render, handle_event, is_installed)
  - `FKey` enum (F1–F12)
  - `AppConsoleKeys` builder (register cartridges, run event loop)
  - F-key tab strip widget (ratatui; active highlighted; greyed when not installed)
  - Status bar widget (`MBA LINK INACTIVE` placeholder; session duration)
  - Profile-based config: `~/.config/os-console/config.toml` with `local` profile
- [x] Create / expand `os-console/` bin crate
  - Move russh SSH server from `app-console-content/src/main.rs` behind `#[cfg(feature = "ssh-server")]`
  - Default `main.rs`: crossterm PTY, local terminal mode
  - Register `ContentCartridge` at F4; register `InputCartridge` at F12 (stub)
- [x] Convert `app-console-content/src/main.rs` → `app-console-content/src/lib.rs`
  - Export `ContentCartridge` implementing `Cartridge` trait
  - Existing auth.rs + db.rs stay (will move to system-gateway-mba in Phase 2)
- [x] Update `Cargo.toml` workspace to include new crates
- [x] `cargo build` green for all three profiles:
  - `cargo build` (local PTY mode, default)
  - `cargo build --features ssh-server`
  - `cargo build --release`

**Gate: PASSED** — F-key tab strip visible; F4 (Content) active; F-key navigation switches between placeholder panes.

---

## Phase 2 — Auth + MBA `[COMPLETE 2026-05-21]`

`system-gateway-mba` fleshed out; `proofctl` CLI; MBA LINK ACTIVE gate confirmed.
Commit: 0b8088c4 (session 5, 2026-05-21)

- [x] Expand `system-gateway-mba/` crate (Scaffold-coded → flesh out)
  - Move `auth.rs`, `db.rs`, SQLite `users` schema from `app-console-content/`
  - Server-side MBA verifier: verify SSH public key fingerprint on incoming connection
  - `proofctl` binary: `user add / list / disable / rotate-key`
  - Immutable audit log of all connection attempts
- [x] `app-console-keys/` — MBA client
  - Read pairing from `pairings.yaml` / local config
  - Connect to `system-gateway-mba` on target os-* peer
  - Poll connection state; expose as `MbaStatus` enum
  - Status bar: `MBA LINK ACTIVE` / `MBA LINK INACTIVE <reason>` / `MBA LINK PENDING`
- [x] Session identity in status bar: `username@tenant | tier`

**Gate: PASSED** — `jennifer@woodfine | MBA LINK ACTIVE` confirmed over local TUI.

---

## Phase 3 — app-console-content as full cartridge `[COMPLETE 2026-05-21]`

ContentCartridge full proofread workflow. Commit: a020a2cd (session 5, 2026-05-21)

**Critical:** Doorman is at `http://localhost:9080`. Response field: `.content`.

- [x] `tui-textarea` integration (paste input for proofread text)
- [x] Protocol picker: 18 GenreTemplate variants via `nucleo` fuzzy filter
- [x] HTTP client to `service-proofreader /v1/proofread` (300s timeout; spinner during wait)
- [x] Status bar feedback during pipeline stages (poll `/v1/health/ready`)
- [x] `similar::TextDiff` → `Vec<Suggestion>` with severity from `findings`
- [x] `syntect` 24-bit colorization for diff panes
- [x] `tui-scrollview` for long documents
- [x] Per-suggestion verdict keybindings (`a`/`r`/`e`/`A`/`R`)
- [x] POST `/v1/verdict` on session complete → corpus event (closes apprenticeship loop)

**Gate: PASSED** — Full proofread workflow over local TUI; feature-equivalent to former web UI.

---

## Phase 4 — app-console-input (F12 / The Anchor) `[COMPLETE 2026-05-21]`

F12 InputCartridge. Commit: ce6c6621 (session 5, 2026-05-21)

- [x] Expand `app-console-input/` lib crate (Scaffold-coded → flesh out)
- [x] F12 global intercept: any F12 keypress routes through `InputCartridge` regardless
  of which cartridge is currently active — enforced in `app-console-keys` event dispatcher
- [x] File path input widget + confirm dialog (SYS-ADR-10 gate)
- [x] POST to `service-input` on Totebox Archive (Ring 1 boundary service); 30s timeout
- [x] `service-input` response: classification + routing target
- [x] Audit trail: local SQLite log of all ingest events (timestamp, file path,
  classification, routing target)
- [x] F12 cannot be bypassed from other panes

**Gate: PASSED** — F12 at any pane → Input Machine modal → file path entry → POST to `service-input` → audit log entry.

---

## Pairing Ceremony — interleaved workstream `[Phases 1+2 COMPLETE 2026-05-22]`

Not in the original phase sequence; added as a separate workstream between Phase 4 and Phase 5.
See `BRIEF-pairing-ceremony.md` for full detail; `BRIEF-pairing-phase3-4.md` for next steps.

- Phase 1 (d6267e39, 2026-05-22): server-issued 8-char Crockford code; `pairing-server` (tiny_http port 9201); `proofctl pair list/approve/deny`; `PairingState`/`PairingEvent` enums; background `spawn_status_poll` thread; zero-jargon TUI screens in chassis
- Phase 2 (30874995, 2026-05-22): `qrcode 0.14` Dense1x2 half-block QR beside code pill on wide terminals; narrow fallback; QR encodes `PAIR:<code>`
- **Phase 3 pending:** `ratatui-image` Kitty/Sixel pixel-perfect QR; local-PTY `Picker::from_query_stdio()`; default Unicode over russh
- **Phase 4 pending:** F11 `app-console-system` cartridge; pending-pair list; Enter approve / D deny in-TUI

---

## Phase 5 — Draft mode `[COMPLETE 2026-05-24]`

Commits: `6422c2a8` + `5118ce77` (2026-05-24)

Add `/new` command to ContentCartridge for AI-assisted draft generation.

- [x] `/new` command → fuzzy protocol picker
- [x] Entity context: `/search` → `service-content /graph/neighborhood/<id>` RAG fetch
- [x] Doorman Tier B request: `POST http://localhost:9080/v1/chat/completions`
  with RAG context + protocol scaffolding; response field: `.content`
- [x] SSE consumer for streaming token output
- [x] Streaming render into draft pane at 60Hz
- [x] `/regenerate` — cancel + retry at same or higher tier
- [x] `/tier b|c` switching with cost-cap awareness
- [x] Draft accept → stage to `.agent/drafts-outbound/` with `foundry-draft-v1` frontmatter
  (5 mandatory research-trail fields, Doctrine claim #39)

**Gate: PASSED** — Draft mode functional; corpus event POST to `/v1/verdict`.

---

## Phase B — Cross-platform release `[NOT STARTED]` (est. 1–2 sessions)

macOS 10.13+ (High Sierra Intel floor) + current macOS (Apple Silicon + Intel universal) + Linux musl static.
Full spec: `BRIEF-cross-platform-release.md`.

- [ ] `rust-toolchain.toml` at monorepo root — pin stable 1.85
- [ ] `.cargo/config.toml` — `MACOSX_DEPLOYMENT_TARGET=10.13` for `x86_64-apple-darwin`
- [ ] Audit reqwest features — switch to `rustls-tls` (drop `native-tls` for macOS 10.13 compat)
- [ ] GitHub Actions `.github/workflows/release.yml` — 4-target matrix (musl, Intel, ARM, lipo universal)
- [ ] `TerminalCaps` runtime probe in `app-console-keys/src/chassis.rs` (kitty, sixel, truecolor booleans)

**Gate:** `cargo build --release` passes on Linux; GH Actions matrix green on all 4 targets.

**Blocked on:** operator confirmation of release trigger (`v*.*.*` tag push vs. push-to-main vs. manual dispatch).

---

## Phase C — Email cartridge F3 `[NOT STARTED]` (est. 2–3 sessions)

Full read + compose/send in one phase. Backend: `service-email` in `project-data` (NOT `service-email-egress`).

- [ ] Convert `app-console-email` stub → lib crate (remove main.rs stub; create lib.rs + cartridge.rs)
- [ ] Add `app-console-keys` path dep + reqwest + serde + tokio to `app-console-email/Cargo.toml`
- [ ] Add `app-console-email` to workspace `Cargo.toml` members
- [ ] Implement `EmailCartridge` with `Cartridge` trait:
  - Inbox list pane (j/k navigation, Enter to open)
  - Read pane (plain-text / HTML-stripped body, PgUp/PgDn scroll)
  - Compose pane (`tui-textarea` for To/Subject/Body; Ctrl+S to send)
  - Folder switcher inbox/sent/drafts (`[`/`]`)
  - State machine: `List → Read → Compose → Sending → List`
- [ ] Wire to `service-email` HTTP API (paths, auth headers, message/folder schema from `project-data`)
- [ ] Wire into `os-console/src/main.rs`: `.cartridge(EmailCartridge::new(&config))`

> **Cross-archive dependency:** `service-email` is in `project-data`. If its HTTP API is not
> yet stable at Phase C execution, implement against a local stub and write an outbox message
> to `project-data` requesting the final endpoint contract.

**Gate:** F3 tab visible in chassis; inbox renders; compose → send works against `service-email` dev instance.

---

## Phase D — SLM cartridge F9 `[NOT STARTED]` (est. 1–2 sessions)

Doorman health + Yo-Yo tier display. Backend: `http://localhost:9080` (Doorman).

- [ ] Convert `app-console-slm` stub → lib crate (create lib.rs + cartridge.rs)
- [ ] Add `app-console-keys` path dep + reqwest + serde + tokio to `app-console-slm/Cargo.toml`
- [ ] Add `app-console-slm` to workspace `Cargo.toml` members
- [ ] Implement `SlmCartridge` with `Cartridge` trait:
  - Doorman health panel: `GET localhost:9080/healthz` + model status
  - Yo-Yo tier display: active tier (A/B/C), model name, latency stats
  - Corpus panel: training queue depth (via apprenticeship substrate)
- [ ] Wire into `os-console/src/main.rs`: `.cartridge(SlmCartridge::new(&config))`

**Gate:** F9 tab visible; Doorman health reads from `localhost:9080`; tier label correct.

---

## Phase E — Orchestration wiring `[NOT STARTED]` (est. 1 session)

No new crate. `os-orchestration` is already the command hub via MBA. This is wiring only.

- [ ] Audit `os-console/src/mba_client.rs` — confirm targeting `os-orchestration` as primary peer
- [ ] Update `ConsoleConfig` / `config.example.toml`: rename `totebox_host` → `orchestration_host` where applicable; add note that this is the command hub, not a single archive
- [ ] Update `BRIEF-os-console-platform.md` §5 MBA section — document topology: `os-console → os-orchestration → Totebox Archives` (no `app-orchestration-command` crate — retired from roadmap)

**Gate:** `pairings.yaml` entry for `os-orchestration` reads correctly; MBA LINK ACTIVE against os-orchestration on GCE.

---

## Phase 6 — Offline mode + search `[NOT STARTED]` (est. 1 week)

- [ ] Offline detection: poll `/v1/health/ready`; switch to deterministic-only mode
- [ ] Disabled-state UX: greyed inference widgets; `/status` shows what is offline
- [ ] Tantivy search: `/search <query>` → `service-content` Tantivy index at port 9081
- [ ] SYS-ADR-10 enforced even in offline mode (F12 gate still active)

**Gate:** Doctrine claim #54 and SYS-ADR-10 compliant in offline mode.

---

## Phase 7 — PDF viewing `[NOT STARTED]` (est. 1 week)

Add PDF rendering. Pixel graphics only — no text-extraction fallback.

- [ ] Add `pdfium-render` dependency
- [ ] PDF page → RGB bitmap via pdfium
- [ ] Terminal capability probe at startup: Kitty → Sixel → error
- [ ] Kitty graphics protocol renderer (primary)
- [ ] Sixel fallback (iTerm2, mlterm, xterm)
- [ ] Hard error on unsupported terminals (Alacritty, basic xterm)
- [ ] PDF pane in ContentCartridge: page-by-page navigation (PgUp/PgDn, j/k)

**Terminal requirements:** kitty, iTerm2, Ghostty, WezTerm.
Alacritty: error (no graphics protocol support as of 2026).

**Gate:** PDF opens with pixel-accurate page rendering on kitty on Linux Mint.

---

## Phase 8 — Polish + additional cartridges `[NOT STARTED]` (est. 2 weeks)

- [ ] OSC 8 hyperlinks on TOPIC/protocol references, citations
- [ ] 24-bit truecolor detection + application throughout
- [ ] Multi-tab editing (`Ctrl-w n`, `Ctrl-w h/l`)
- [ ] Session persistence via SQLite (re-open last draft on reconnect)
- [ ] `/audit` verdict log viewer
- [ ] `/export` write buffer to file
- [ ] `app-console-people` (F2) — basic implementation
- [ ] `app-console-email` (F3) — basic implementation
- [ ] `app-console-system` (F11) — os-* service health dashboard + MBA pairing status

---

## Phase 9 — Operations `[NOT STARTED]` (est. 3–5 days)

- [ ] `local-proofreader-tui.service` systemd unit (replaces both web UI console units)
- [ ] Unit source at `infrastructure/local-proofreader/local-proofreader-tui.service`
- [ ] Prometheus metrics endpoint (separate HTTP port)
- [ ] Fail2ban rule for port 2222
- [ ] Key-rotation runbook in `guide-provision-node.md`
- [ ] Graceful shutdown: flush corpus WAL on SIGTERM
- [ ] Update deployment instance MANIFEST.md

**Gate:** `local-proofreader-tui.service active`. Monitoring live. Runbook written.

---

## Future phases (post-Phase 9)

- Phase 10: `app-console-bim` (F7) — BIM workflow cartridge
- Phase 11: `app-console-slm` (F9) — SLM adapter marketplace management
- Phase 12: `app-console-mesh` (F10) — PPN mesh management (replaces web F8 Terminal)
- Phase 13: os-console chassis ops on Linux Mint (MBA auto-reconnect, watchdog, auto-start)

---

## Key cargo dependencies

```toml
# Core TUI
ratatui = "0.30"
crossterm = "0.28"
tui-textarea = "0.7"
tui-scrollview = "0.5"

# SSH server (feature-gated)
[features]
ssh-server = ["dep:russh"]

[dependencies]
russh = { version = "0.60", optional = true }
russh-keys = "0.60"

# Storage
rusqlite = { version = "0.32", features = ["bundled"] }

# Diff + colorize
similar = "2.5"
syntect = "5.2"

# Fuzzy filter
nucleo = "0.5"

# PDF rendering
pdfium-render = "0.8"

# HTTP client
reqwest = { version = "0.12", features = ["json", "stream"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
rand = "0.10"
```
