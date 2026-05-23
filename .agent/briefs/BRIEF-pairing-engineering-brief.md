---
schema: foundry-plan-v1
archive: project-console
topic: pairing-engineering-brief
created: 2026-05-21
updated: 2026-05-23
status: archived
cites: []
---

> **ARCHIVED 2026-05-23.** Pre-implementation research. Pairing Phases 1–2 shipped
> (d6267e39, 30874995). Phase 3+4 implementation guide consolidated into
> `BRIEF-pairing-phase3-4.md`. §5 (terminal graphics) and §8 Phase 3–4 notes are the
> sections still referenced by the new brief.

# Engineering Brief — Zero-Jargon Device Pairing & Terminal Graphics for os-console

> Research by Opus engineering agent, 2026-05-21.
> Scope: `os-console` pairing implementation + ratatui terminal graphics.
> Companion plans: `os-console-platform.md`, `leapfrog-2030-coding.md`, `pairing-system-design.md`, `pairing-ux-design.md`.

## 0. Crate version caveat

Crate versions below are stated from engineering knowledge as of Jan 2026 training cutoff. Every version marked `[verify]` must be confirmed against crates.io with `cargo add` before adding to `Cargo.toml`. The `Cargo.lock` already resolves `reqwest`, `base64`, `base64ct`, and `sha2` — the HTTP + hashing substrate is in-tree.

---

## 1. Current state (verified from source)

Paths relative to `pointsav-monorepo/`:

| File | Relevant fact |
|---|---|
| `system-gateway-mba/src/auth.rs` | `compute_fingerprint(&PublicKey) -> String` → `"SHA256:<base64>"` |
| `system-gateway-mba/src/db.rs` | SQLite at `~/.local/share/proof/proof.db`; `users` table keyed on `fingerprint` |
| `system-gateway-mba/src/bin/proofctl.rs` | `proofctl user add/list/disable/rotate-key`; loads `.pub` via `russh::keys::load_public_key` |
| `os-console/src/mba_client.rs` | russh client; `authenticate_publickey` with `PrivateKeyWithHashAlg::new(Arc::new(key), None)` |
| `os-console/src/main.rs` | One-shot MBA probe (5 s timeout) **before** the TUI starts; result frozen for the session |
| `app-console-keys/src/chassis.rs` | `PairingInfo { fingerprint, host, port }`; `render_pairing_screen` prints the `proofctl` command |

**Three problems with today's flow:**

1. **Jargon.** The pairing screen literally prints a shell command with `--tenant` and `--key-file <path/to/your/id_ed25519.pub>`.
2. **No live transition.** `main.rs` probes MBA once, then *drops the tokio runtime*. The screen says "restart os-console." No path from "waiting" to "paired" without a restart.
3. **No HTTP client in the chassis.** `app-console-keys` has no `reqwest` dependency.

---

## 2. Recommended pairing approach

### 2.1 Decision: server-issued pairing code + QR

**A server-issued random code (not derived from the key) is the MVP, with an in-terminal QR code as progressive enhancement.**

- A code *derived* from the public key cannot be short without becoming guessable. A server-issued code is an opaque rendezvous token.
- **Flow (A) — Claim (recommended default):** os-console posts a request carrying its public key; server returns a short code; user reads the code to the admin; admin types `proofctl pair approve <code>`; server already holds the public key.
- **Flow (B) — Invite:** admin pre-generates an invite code; user types it into os-console; os-console submits its key bound to that invite. Build (A) first; (B) is a thin variant.
- QR code is pure UX polish over flow (A): encodes the same approve URL/code. Never the *only* channel — the 8-char code is always present as text.

### 2.2 Why not key-derived codes

`bs58(fingerprint)[..8]` ≈ 47 bits, but deterministic from a public value — grindable. TOTP-style needs a pre-shared secret — the bootstrapping problem we are removing. Server-issued random codes have no offline attack surface.

### 2.3 Code format

- **Alphabet:** Crockford base32 (`0-9A-Z` minus `I L O U`) — unambiguous, case-insensitive, phone-friendly.
- **Length:** 8 symbols = 40 bits of entropy. Format: `XXXX-XXXX` (e.g. `K7Q2-9XMT`).
- **Generation:** `rand` → 5 random bytes → Crockford base32 → 8 chars.
- **Lifecycle:** generated server-side on `POST /v1/pair/request`; `expires_at = now + 600s`; single-use; deleted on approve/deny/expiry.

```rust
// system-gateway-mba/src/pairing.rs
const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

pub fn new_code() -> String {
    let mut bytes = [0u8; 5];
    rand::rng().fill_bytes(&mut bytes);
    let mut acc: u64 = 0;
    for b in bytes { acc = (acc << 8) | b as u64; }
    let mut out = String::with_capacity(9);
    for i in (0..8).rev() {
        let idx = ((acc >> (i * 5)) & 0x1f) as usize;
        out.push(CROCKFORD[idx] as char);
        if i == 4 { out.push('-'); }
    }
    out
}

pub fn normalize(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| match c.to_ascii_uppercase() {
            'I' | 'L' => '1', 'O' => '0', c => c,
        }).collect()
}
```

### 2.4 Security analysis

| Vector | Mitigation |
|---|---|
| Code guessing / brute force | 40-bit space, 10-min TTL, rate-limit `pair/approve` ~5/min/IP; lock code after 5 bad attempts |
| Replay of an approved code | Single-use; row deleted on approve |
| MITM swapping the public key | TLS on the pairing endpoint (see Risks §8). The key in the request *is* the credential |
| Malicious user pairing as someone else | Admin sees `username`, `tenant`, fingerprint at approve time and explicitly approves |
| Pending-request table DoS | TTL sweep on every write; cap pending rows per source IP |
| Code shoulder-surfed | Short TTL + single-use + admin approval |

**The code is a rendezvous token, not a credential** — state this in code comments so nobody "optimizes" it into an auto-approve secret.

---

## 3. Server side — `system-gateway-mba`

### 3.1 HTTP surface

Pairing needs an **HTTP** surface (client polls it; the russh auth channel is not pollable). **Recommend `tiny_http`** over `axum`+`tokio` — `system-gateway-mba` is currently synchronous (`rusqlite`, no tokio); `tiny_http` adds one thread, no async runtime. Bind on a dedicated pairing port (e.g. **9201**) — port 9092 is live `service-proofreader`.

### 3.2 Endpoints

```
POST /v1/pair/request
  body  : { username, tenant, public_key, fingerprint }
  resp  : { request_id, code, expires_at }

GET  /v1/pair/status/{request_id}
  resp  : { state: pending|approved|denied|expired }
  long-poll: hold up to 25 s, return early on state change

POST /v1/pair/approve    (admin-only, localhost-bound for MVP)
  body  : { code }   -> resolves code, calls db::add_user(...), deletes row

POST /v1/pair/deny       (admin-only)
  body  : { code }
```

The public key travels in the **request**, so `approve` only needs the code. `add_user` is reused verbatim.

### 3.3 SQLite schema (extend existing `migrate()` in `db.rs`)

```sql
CREATE TABLE IF NOT EXISTS pairing_requests (
    request_id   TEXT PRIMARY KEY,        -- uuid v4
    code         TEXT UNIQUE NOT NULL,    -- normalized, no dash
    username     TEXT NOT NULL,
    tenant       TEXT NOT NULL CHECK(tenant IN ('pointsav','woodfine')),
    fingerprint  TEXT NOT NULL,
    public_key   TEXT NOT NULL,           -- full authorized_keys line
    role         TEXT NOT NULL DEFAULT 'editor',
    state        TEXT NOT NULL DEFAULT 'pending'
                 CHECK(state IN ('pending','approved','denied','expired')),
    attempts     INTEGER NOT NULL DEFAULT 0,
    created_at   TEXT NOT NULL,
    expires_at   TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_pairing_code ON pairing_requests(code);
```

Add a `sweep_expired(conn)` helper; call at the top of every handler.

### 3.4 Admin interface

**MVP:** no push. `proofctl pair list` shows pending requests; `proofctl pair approve <code>` calls the same DB path. HTTP `approve`/`deny` endpoints are **localhost-bound only** — admin runs `proofctl` on the gateway box; that *is* the auth boundary.

New `proofctl` subcommands: `pair list`, `pair approve <code>`, `pair deny <code>`, `pair invite --tenant <t> [--username <u>]`.

---

## 4. Client side — os-console

### 4.1 The blocking problem

`main.rs` does a one-shot probe then `drop(rt)`. **Fix: background poll thread + channel.** Spawn a `std::thread` owning a blocking `reqwest::blocking::Client` that long-polls `pair/status` and sends a `PairingEvent` over an `mpsc::Sender`. The 16 ms render loop does a non-blocking `try_recv()` each frame. No async runtime in the chassis.

### 4.2 Client state machine

```
Probing MBA --probe fails--> Unpaired --[P]--> AwaitingApproval
   |--probe ok--> Paired (run)              (POST pair/request)
AwaitingApproval --status=approved--> Paired (run)  [no restart]
AwaitingApproval --denied/expired--> Failed --[P]--> AwaitingApproval
```

Per-state screens (see pairing-ux-design.md for full copy):
- **Unpaired:** plain text, "Press P to request access." No shell command, no flags.
- **AwaitingApproval:** big centered code `K7Q2-9XMT`, QR block, plain instructions.
- **Paired:** flips `mba_status` to `Active`, drops into normal cartridge view. No restart.
- **Failed:** "That request expired. Press P to try again."

### 4.3 HTTP wiring

- Add `reqwest = { version = "0.12", features = ["blocking", "json"] }` and `serde_json`. Both already in `Cargo.lock`.
- **Timeout discipline (per cluster memory):** long-poll client read timeout 30 s (server holds 25 s). `pair/request` and `pair/approve` use a fast 30 s client. **Do not reuse the 300 s proofread client.**

### 4.4 Poll-thread sketch

```rust
pub enum PairingEvent { Pending, Approved, Denied, Expired, Error(String) }

pub fn spawn_status_poll(base: String, request_id: String, tx: mpsc::Sender<PairingEvent>) {
    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build().expect("client");
        loop {
            let url = format!("{base}/v1/pair/status/{request_id}");
            match client.get(&url).send().and_then(|r| r.json::<StatusResp>()) {
                Ok(s) => match s.state.as_str() {
                    "approved" => { let _ = tx.send(PairingEvent::Approved); break; }
                    "denied"   => { let _ = tx.send(PairingEvent::Denied);   break; }
                    "expired"  => { let _ = tx.send(PairingEvent::Expired);  break; }
                    _          => { let _ = tx.send(PairingEvent::Pending); }
                },
                Err(e) => {
                    let _ = tx.send(PairingEvent::Error(e.to_string()));
                    std::thread::sleep(Duration::from_secs(3));
                }
            }
        }
    });
}
```

Render loop integration — one non-blocking line per frame:
```rust
while let Ok(ev) = pair_rx.try_recv() { self.apply_pairing_event(ev); }
```

### 4.5 Reading the public key

os-console already loads the secret key via `load_secret_key` (`mba_client.rs`). The public key for the request body is `key.public_key()` serialized to the `ssh-ed25519 AAAA...` OpenSSH line. `compute_fingerprint` already gives the `SHA256:` string. No new key handling.

---

## 5. Terminal graphics in ratatui

### 5.1 Kitty graphics protocol — how it works

Images are transmitted via an APC escape sequence: `ESC _ G <key>=<value>,... ; <base64 payload> ESC \`. Payload is base64 PNG (`f=100`) / RGBA (`f=32`) / RGB (`f=24`); large images chunked. The **unicode-placeholder method** (`U+10EEEE` placeholders + `a=p,U=1`) is the ratatui-friendly variant and is what `ratatui-image` uses.

### 5.2 Recommended crate: `ratatui-image`

`ratatui-image` `[verify — confirm version for ratatui 0.29]` provides:
- `StatefulImage` widget rendered into a ratatui `Rect`
- Automatic **protocol detection** (Kitty → Sixel → iTerm2 → Unicode half-block fallback)
- Handles the cursor/cell-skip dance
- Takes an `image::DynamicImage`

**Do not hand-roll Kitty escape sequences.** `viuer` is stdout-oriented (wrong shape for in-frame ratatui composition) — use it only for a non-ratatui admin CLI.

### 5.3 QR code generation

**`qrcode` `[verify ~0.14]`** — outputs both a `DynamicImage` **and** a built-in Unicode/string renderer (`unicode::Dense1x2`). One crate covers the image path *and* the no-graphics fallback. **Recommended.**

### 5.4 What to encode in the QR

A **compact pairing URL**, not the raw key: `pair://approve?c=K7Q29XMT&h=gateway.host&p=9201`. A raw SSH key is 68+ chars → denser QR, harder to scan. The key is already on the server from `pair/request` — the QR only carries the rendezvous code.

### 5.5 QR rendering strategy

Option B (image via Kitty/Sixel) with Option A (Unicode) as automatic fallback — both through one path: `ratatui-image` + `qrcode`.

```rust
use qrcode::QrCode;
use image::{DynamicImage, Luma};

pub fn qr_image(url: &str) -> anyhow::Result<DynamicImage> {
    let code = QrCode::new(url.as_bytes())?;
    let img = code.render::<Luma<u8>>().quiet_zone(true)
        .module_dimensions(8, 8).build();
    Ok(DynamicImage::ImageLuma8(img))
}

pub fn qr_unicode(url: &str) -> anyhow::Result<String> {
    let code = QrCode::new(url.as_bytes())?;
    Ok(code.render::<qrcode::render::unicode::Dense1x2>().quiet_zone(true).build())
}
```

Chassis render decision:
```rust
match self.picker {
    Some(picker) => {
        let proto = picker.new_resize_protocol(qr_image(&url)?);
        frame.render_stateful_widget(StatefulImage::default(), qr_rect, &mut proto);
    }
    None => frame.render_widget(Paragraph::new(qr_unicode(&url)?), qr_rect),
}
```

**Always render the 8-char code as large text next to the QR** — a no-camera admin just reads it. The QR is never the only channel.

### 5.6 Terminal detection

Prefer `ratatui-image`'s `Picker::from_query_stdio()`. **SSH caveat (critical for os-console):** when serving via the russh SSH server, graphics must survive the russh channel. **Default to the Unicode fallback over SSH**; the local-PTY path (`run_local`) can trust `from_query_stdio()` fully.

### 5.7 Rich TUI capabilities (beyond QR)

- **`ratatui::widgets::canvas::Canvas`** — built in; draws `Line`/`Rectangle`/`Circle`/`Points` with `Marker::Braille` (2×4 sub-cell dots), `HalfBlock`, `Dot`.
- **Half-block pixel art** — `▀`/`▄` with independent fg/bg = two color pixels per cell.
- **Animation** — the chassis loop already polls at 16 ms = ~60 fps. Animate from `started: Instant` via `elapsed()`. ratatui only writes changed cells, so one animated region is nearly free.
- **Gradient text** — split a string into one `Span` per character, assign each an interpolated `Color::Rgb`. ~20-line helper, no crate needed.

---

## 6. Crate dependency table

| Crate | Version (est.) | Used in | Purpose |
|---|---|---|---|
| `reqwest` | `0.12` `[verify]` (in `Cargo.lock`) | `app-console-pair` | Blocking HTTP for pair/request, status long-poll |
| `serde_json` | `1` `[verify]` | both | JSON bodies |
| `serde` | `1` (already in monorepo) | both | Derive request/response structs |
| `rand` | `0.10` `[verify]` (optional in os-console today) | `system-gateway-mba` | Pairing code entropy |
| `uuid` | `1` `[verify]`, feature `v4` | `system-gateway-mba` | `request_id` |
| `tiny_http` | `0.12` `[verify]` | `system-gateway-mba` | Minimal blocking HTTP server |
| `qrcode` | `0.14` `[verify]` | `app-console-pair` | QR generation — image **and** Unicode |
| `image` | `0.25` `[verify]` | `app-console-pair` | `DynamicImage` carrier |
| `ratatui-image` | `[verify for ratatui 0.29]` | `app-console-pair` | Kitty/Sixel/Unicode image widget + protocol detection |
| `colorgrad` | `[verify]` *(optional)* | `app-console-keys` | Gradient stops for gradient text |

**Crates deliberately NOT used:** `bs58`/`nanoid`/`ulid` (code gen is a 13-line function); `viuer` (stdout-oriented); `axum`+`tokio` in `system-gateway-mba` (drags an async runtime into a synchronous crate).

**New crate recommendation:** put client-side pairing logic in a **new `app-console-pair` crate**, not in `app-console-keys`. Keeps the base chassis crate's dependency surface lean; isolates the heavier image stack. The chassis renders a `&PairingState` it is handed; it does not pull `reqwest` itself.

---

## 7. New file layout

```
system-gateway-mba/
├── src/
│   ├── pairing.rs          # new: code gen, normalize, PairingRequest struct
│   ├── pairing_db.rs       # new: pairing_requests table CRUD + sweep_expired
│   └── pairing_http.rs     # new: tiny_http route handlers
app-console-pair/           # new crate
└── src/
    ├── lib.rs              # PairingState enum, spawn_status_poll, qr helpers
    └── client.rs           # reqwest POST pair/request
app-console-keys/
└── src/
    └── chassis.rs          # updated: PairingState integration, try_recv in render loop
os-console/
└── src/
    └── main.rs             # updated: on probe failure → enter pairing flow
```

---

## 8. Implementation phases

**Phase 1 — MVP pairing, no graphics (2-3 days):**
1. `pairing.rs` in `system-gateway-mba` (code gen, normalize)
2. Extend `db.rs` migrate with `pairing_requests` + `sweep_expired`
3. `tiny_http` server: `pair/request`, `pair/status` long-poll, `pair/approve`, `pair/deny`
4. `proofctl pair list/approve/deny`
5. New `app-console-pair` crate: structs, poll thread, `PairingState` enum
6. `app-console-keys`: replace static `PairingInfo` with `PairingState`; rewrite `render_pairing_screen` (no shell command, no flags); wire `try_recv()` into the loop; flip to `set_mba_active()` on `Approved` — no restart
7. `main.rs`: on probe failure, enter pairing flow instead of freezing

**Exit criteria:** non-technical user launches, presses `P`, reads `K7Q2-9XMT` to an admin, admin runs `proofctl pair approve K7Q2-9XMT`, console transitions to active, no restart, no jargon.

**Phase 2 — QR, Unicode-only (1-2 days):**
8. Add `qrcode`; render pairing URL with `Dense1x2` renderer beside the code

**Phase 3 — Pixel-perfect graphics (2-3 days):**
9. Add `image` + `ratatui-image`; `Picker::from_query_stdio()` at startup
10. `StatefulImage` when available, else Phase 2 fallback
11. Validate over the russh SSH-server path; default to Unicode over SSH

**Phase 4 — Admin experience + polish (future):**
12. `proofctl pair watch`; optional Slack/email notifier
13. Admin TUI cartridge (F11 `app-console-system`) for one-keystroke approval
14. Animated MBA-link indicator, gradient title text

---

## 9. Risks and unknowns

| Risk | Severity | Notes |
|---|---|---|
| `proof_endpoint` is plain HTTP today | High | Pairing request carries the public key; must be TLS before any non-loopback deployment |
| `ratatui-image` compatibility with ratatui 0.29 | Medium | Must `[verify]`. Phase 2 (Unicode QR) still ships if incompatible; Phase 3 slips |
| Graphics over the russh SSH channel | Medium | APC escapes must pass through russh untouched; default to Unicode on SSH path |
| `system-gateway-mba` gaining an HTTP server | Medium | `tiny_http` adds one thread to a synchronous crate — confirm acceptable |
| Port allocation | Low | 9092 is live `service-proofreader`; pairing HTTP needs its own port (suggest 9201) |
| `rand` 0.10 API churn | Low | 0.9→0.10 changed `thread_rng()`→`rng()`; sketch uses the newer API |
| Long-poll connection limits | Low | Each waiting client holds one `tiny_http` thread ≤25 s; size the pool |
| QR scannability from a terminal render | Low | Encoding the short code URL keeps QR low-version with large modules; test on real phones |

---

## 10. Terminal compatibility

- **Kitty graphics:** Kitty, WezTerm, Ghostty, recent Konsole
- **Sixel:** xterm (sixel build), foot, WezTerm, contour, mlterm, recent Windows Terminal
- **iTerm2 protocol:** iTerm2, WezTerm
- **Unicode `Dense1x2` half-block QR:** every terminal with a Unicode font — **the guaranteed-everywhere baseline; must always be present.** os-console must never *require* a graphics protocol — the 8-char code plus Unicode QR is the floor.
- **Over SSH:** default conservative (Unicode) on the `run_with_bytes` path

---

## 11. One-paragraph summary

Replace the jargon `proofctl user add ... --key-file` instruction screen with a server-issued 8-character pairing code (`XXXX-XXXX`, Crockford base32, 10-minute single-use). os-console posts a pairing request (carrying its own public key) to a new `tiny_http` surface on `system-gateway-mba`, displays the returned code in large text, and a background thread long-polls for approval so the console flips to ACTIVE live with no restart. The admin approves with `proofctl pair approve <code>` — no key files, no flags. Layer a QR code on top: encode the short approve URL, render it with `qrcode`'s Unicode half-block renderer everywhere and, where the terminal supports Kitty/Sixel, upgrade to a pixel-perfect image via `ratatui-image`. Build the code flow first (Phases 1–2); graphics are a clearly-separable enhancement (Phases 3–4) that never become a hard dependency.
