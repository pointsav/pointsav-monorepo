---
artifact: brief
name: BRIEF-pairing-ceremony
topic: Zero-jargon pairing ceremony (Phases 1–2)
status: superseded
superseded_by: BRIEF-project-console-master.md
superseded: 2026-05-31
created: 2026-05-22
updated: 2026-05-22
---

# BRIEF: Pairing Ceremony — Zero-Jargon MBA Pairing Flow

## Context

os-console users run a local TUI binary that must establish an MBA peer-to-peer link to
os-totebox before showing content. The original Phase 5 implementation showed a static
screen with `proofctl user add` instructions — correct but jargon-heavy. This brief covers
the replacement: a server-issued 8-character code the user shares with their administrator,
who approves via `proofctl pair approve <code>`.

## Phase 1 — MVP (d6267e39, 2026-05-22) ✓

**system-gateway-mba additions:**
- `src/pairing.rs` — Crockford base32 code gen (`new_code()`, `normalize()`); request/response structs
- `src/pairing_db.rs` — `pairing_requests` SQLite table; `insert_request`, `get_by_code`, `set_state`, `list_pending`, `sweep_expired`
- `src/pairing_http.rs` — `tiny_http` HTTP server port 9201; `POST /v1/pair/request`, `GET /v1/pair/status/{id}`, `POST /v1/pair/approve`, `POST /v1/pair/deny`, `GET /v1/pair/pending`
- `src/bin/pairing_server.rs` — standalone binary; deploy to VM alongside SSH server
- `src/bin/proofctl.rs` — `pair list`, `pair approve <code>`, `pair deny <code>`
- `src/db.rs` — `migrate()` extended: `pairing_requests` table + `idx_pairing_code` index

**app-console-keys additions:**
- `src/pairing.rs` — `PairingState` enum (Unpaired, AwaitingApproval, Approved, Denied, Expired, Error); `PairingEvent` enum; `post_pair_request()`; `spawn_status_poll()`
- `src/chassis.rs` — rewritten: `pairing_state: PairingState` + `pair_rx: Option<mpsc::Receiver<PairingEvent>>`; per-state render screens; `drain_pair_events()` (borrow-safe Vec collect pattern)
- `src/config.rs` — `pair_endpoint: String` default `http://127.0.0.1:9201`

**os-console/src/main.rs** — on MBA failure: `post_pair_request()` → `set_pairing_awaiting()` → `spawn_status_poll()` → `set_pair_rx(rx)`; `load_pubkey_line()` helper

**Vocabulary (zero-jargon, enforced in all render copy):**
- "Connection Code" (not: token, OTP, pairing code)
- "administrator" (not: operator, proofctl, tenant)
- "Connect this computer to your workspace" (not: pair, MBA, fingerprint)

## Phase 2 — Unicode QR beside code pill (30874995, 2026-05-22) ✓

- `app-console-keys/src/qr.rs` — `qr_unicode(content: &str) -> String` via `qrcode 0.14` Dense1x2 renderer
- `chassis.rs` AwaitingApproval: wide layout (inner width ≥ qr_col_w + 32) → QR left + code pill right; narrow → code-pill-only fallback
- QR encodes `PAIR:<code_no_dash>` (e.g. `PAIR:K7Q29XMT`); version 1 QR (21×21 modules)

## Pending

- **Phase 3** — `ratatui-image` Kitty/Sixel pixel-perfect QR; activates on protocol detection; Dense1x2 fallback unchanged
- **Phase 4** — F11 `app-console-system` cartridge: pending-pair list; `Enter` approve / `D` deny (no proofctl typing)

## Infrastructure (Command Session required)

- Deploy `pairing-server` binary to VM with systemd unit (`pairing-server 0.0.0.0:9201`)
- GCE firewall port 2222 open for external MBA connections
- GitHub PR: `cluster/project-proofreader → main` on pointsav/pointsav-monorepo
