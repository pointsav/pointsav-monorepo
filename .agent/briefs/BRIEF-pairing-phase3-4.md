---
artifact: brief
name: BRIEF-pairing-phase3-4
topic: Pairing Ceremony — Phase 3 (Kitty/Sixel QR) and Phase 4 (F11 operator panel)
status: active
created: 2026-05-23
---

# BRIEF: Pairing Phase 3+4

## Context

Phases 1+2 shipped (d6267e39, 30874995). Phase 2 renders Unicode Dense1x2 QR via
`qrcode 0.14`. Phase 3 upgrades to pixel-perfect QR on capable terminals. Phase 4
adds the F11 in-TUI approval panel so operators approve without typing `proofctl pair approve`.

Research sources (archived): `BRIEF-pairing-engineering-brief.md` §5, `BRIEF-pairing-ui-design.md`
§2 Act 2, `BRIEF-pairing-ux-design.md` §6, `BRIEF-pairing-system-design.md` §7.

---

## Phase 3 — Kitty/Sixel pixel-perfect QR

**Crates to add** (verify versions against crates.io before adding):
- `image = "0.25"` — DynamicImage carrier
- `ratatui-image` — **must verify compat with ratatui 0.30** before adding (may have breaking changes)

**New helper in `app-console-keys/src/qr.rs`:**

```rust
use image::{DynamicImage, Luma};
use qrcode::QrCode;

pub fn qr_image(content: &str) -> anyhow::Result<DynamicImage> {
    let code = QrCode::new(content.as_bytes())?;
    let img = code.render::<Luma<u8>>().quiet_zone(true).module_dimensions(8, 8).build();
    Ok(DynamicImage::ImageLuma8(img))
}
```

**`chassis.rs` render decision** (extend `render_awaiting_approval`):

```rust
match self.picker {
    Some(ref mut picker) => {
        let proto = picker.new_resize_protocol(qr_image(&pair_url)?);
        frame.render_stateful_widget(StatefulImage::default(), qr_rect, &mut proto);
    }
    None => {
        // Dense1x2 fallback — already ships from Phase 2
        frame.render_widget(Paragraph::new(qr_unicode(&pair_url)?), qr_rect);
    }
}
```

**`Picker` initialisation** — add to chassis init alongside `pair_endpoint`:
```rust
let picker = Picker::from_query_stdio().ok(); // None on query failure or SSH path
```

**SSH path critical rule:** when `run_with_bytes` (russh SSH server path), force `picker = None`
— APC escape sequences must survive the russh channel AND the remote terminal must support the
protocol. `from_query_stdio()` is only reliable on the local-PTY path.

**QR content:** unchanged from Phase 2 — `PAIR:<code_no_dash>` (e.g. `PAIR:K7Q29XMT`).

**Gate:** pixel-accurate QR scanned from phone on kitty terminal, Linux Mint. Unicode Dense1x2
fallback unchanged and still present (8-char code pill always rendered regardless of QR tier).

---

## Phase 4 — F11 `app-console-system` operator panel

**New crate:** `app-console-system` lib crate, registered at F11 in `os-console/src/main.rs`.

```rust
AppConsoleKeys::new()
    .cartridge(ContentCartridge::new())
    .cartridge(InputCartridge::new())
    .cartridge(SystemCartridge::new())   // F11 — new
    .run()
```

**Data source:** `GET http://127.0.0.1:9201/v1/pair/pending` returns JSON array of pending
requests, each: `{ request_id, code, username, tenant, created_at }`. Poll every 5 s via
background thread (same `std::thread` + `mpsc` pattern as `spawn_status_poll` in pairing.rs).

**Panel layout:**

```
┌─ Pending Connections ───────────────────────── 1 waiting ─┐
│  Code       User       Tenant     Requested                │
│  ──────────────────────────────────────────────            │
│  K7Q2-9XMT  jennifer   woodfine   2 min ago    ← cursor    │
│                                                            │
│  Enter  Approve    D  Decline    ?  View fingerprint       │
└────────────────────────────────────────────────────────────┘
```

**Keybindings:**
- `j`/`k` — navigate list
- `Enter` — `POST http://127.0.0.1:9201/v1/pair/approve` `{"code": "<code>"}`
- `D` — `POST http://127.0.0.1:9201/v1/pair/deny` `{"code": "<code>"}`
- `?` — expand row to show fingerprint (operator cross-check)

No `proofctl pair approve` typing required. One keystroke.

**Status bar badge:** when pending count > 0, append `  · N request(s) waiting` in amber
to the status bar. Expose count via shared `Arc<AtomicUsize>` updated by the poll thread;
chassis reads it each render frame.

**Confirmation copy after approve:**
```
jennifer approved. Their console will transition automatically.
```
No jargon. No fingerprint in the confirmation (it was available via `?` for the cross-check).

**Gate:** operator presses `Enter` on a pending request in the F11 TUI → user's
`AwaitingApproval` screen transitions to `Connected` with no restart and no `proofctl` required.

---

## Color + copy rules (from BRIEF-pairing-ceremony.md)

- Code pill: `Color::Cyan` text, dark bordered background
- Waiting animation: `Color::Blue` (Pending accent)
- Approved: `Color::Green` checkmark
- Status bar badge: `Color::Yellow` (amber) — not red (red = data loss only)
- Vocabulary enforced: "Connection Code", "administrator", "Connect this computer"
- Never expose: SSH, key, keyfile, fingerprint, hash, ed25519, proofctl (in user-facing copy)

---

## Open questions

- `ratatui-image` version compatibility with ratatui 0.30: verify before Phase 3 starts
- APC escape passthrough over russh `run_with_bytes`: test with real Kitty client over SSH before
  relaxing the "force Unicode on SSH" rule
- Badge count location in status bar: left / centre / right? (status bar layout in `fkey_strip.rs`)
