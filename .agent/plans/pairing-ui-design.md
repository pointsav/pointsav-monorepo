---
schema: foundry-plan-v1
archive: project-console
topic: pairing-ceremony-ui-design
created: 2026-05-21
status: draft
cites: []
---

# Pairing Ceremony & Holistic TUI Design — UI Design Brief

> Research by Opus UI design agent, 2026-05-21.
> Scope: visual design of pairing screens + holistic TUI design language for os-console.
> Companion plans: `pairing-ux-design.md`, `pairing-engineering-brief.md`, `os-console-platform.md`.

---

## 1. Codebase grounding

Read from source before drafting:

| File | Relevant observation |
|---|---|
| `app-console-keys/src/chassis.rs` | `render_pairing_screen` → currently prints `proofctl user add` shell command + raw `SHA256:` fingerprint. `PairingInfo { fingerprint, host, port }` is a static struct — replaced by `PairingState` enum in Phase 1 |
| `app-console-keys/src/widgets/status_bar.rs` | `MbaStatus` enum: `Active` (green), `Inactive(String)` (yellow), `Pending` (blue). Status bar renders `jennifer@woodfine │ MBA LINK ACTIVE` |
| `app-console-keys/src/widgets/fkey_strip.rs` | Active tab: black-on-cyan. Inactive tabs: grey |
| `os-console/src/main.rs` + `mba_client.rs` | Fire-once 5 s pre-TUI MBA connection. **Must become a retrying in-TUI loop before the pairing ceremony can animate** |
| `os-console-platform.md` | MBA peer-to-peer model, F11 `app-console-system`, Kitty graphics mandated platform-wide |

---

## 2. Part 1 — Pairing Ceremony Screen Design

### 2.1 The four-act ceremony

#### Act 1 — Invitation (Screen: Welcome)

**Layout:** single centred panel, no F-key strip (dimmed), no active content area.

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│                                                                 │
│                        ◈  os-console                           │
│                                                                 │
│                  Welcome to your workspace.                     │
│                                                                 │
│      Let's connect this computer. It takes about a minute.     │
│      You'll show a short code to your administrator,           │
│      they'll approve it, and you're in.                         │
│                                                                 │
│                    ──────────────────                           │
│                    Enter  Get started                           │
│                                                                 │
│                         ?  Help                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
  Not connected                                    os-console 0.1
```

Color: background dark (`#1a1a2e` or terminal default); workspace glyph `◈` in cyan; "os-console" in bold white; body text in `Color::Gray`; `Enter` label in cyan; status bar: `▲ Not connected` in amber.

#### Act 2 — Code Display (Screen: Your Connection Code)

Hero layout. QR image (Kitty/Sixel) or Unicode QR occupies left ≈ 60% of content area; code and instructions on right.

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  ┌─── Your Connection Code ──────────────────────────────────┐  │
│  │                                                           │  │
│  │   [ QR CODE — large, scannable               ]           │  │
│  │                                                           │  │
│  │   Can't scan?  Read this code instead:                   │  │
│  │                                                           │  │
│  │          ┌─────────────────────┐                         │  │
│  │          │   K7Q2  ·  9XMT    │                         │  │
│  │          └─────────────────────┘                         │  │
│  │                                                           │  │
│  │   Show this to your administrator. As soon as they       │  │
│  │   approve it, this screen moves on by itself.            │  │
│  │                                                           │  │
│  │                   Enter  I've shared the code →          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
  ◌ Waiting for approval                           os-console 0.1
```

**Code pill styling:** `K7Q2  ·  9XMT` in a bordered box; `Modifier::BOLD`; `Color::Cyan` text on dark background. The center-dot separator `·` is grey (de-emphasized). Monospace, wide characters if terminal supports.

**QR area (80×24 terminal):** allocate approximately 20×20 characters for the QR block. At 8px module-dimensions on a Kitty-capable terminal, this is scannable. Unicode `Dense1x2` fallback must also fit; if terminal width < 60 cols, drop QR entirely and promote the code pill to hero.

#### Act 3 — Waiting (Screen: Waiting for approval)

Calm, ambient, proof-of-life.

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│                                                                 │
│                 Waiting for your administrator…                 │
│                                                                 │
│                        ◌   ◍   ◌                               │
│                                                                 │
│      We've let your administrator know. They'll approve        │
│      this computer when they get a moment.                     │
│                                                                 │
│      • This usually takes a minute or two.                     │
│      • You can leave this screen open — we'll catch the        │
│        approval the moment it happens.                         │
│                                                                 │
│              R  Show my code again                             │
│              ?  Help          Q  I'll come back later          │
│                                                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
  ◌ Waiting for approval                           os-console 0.1
```

**Ambient animation:** three dots cycling: `◌ ◌ ◌` → `◍ ◌ ◌` → `◌ ◍ ◌` → `◌ ◌ ◍` → repeat. Cycle: 1.8 s. Color: `Color::Blue` (the Pending accent — mirrors the status bar). *Slow* movement reads as patience; fast spinners read as anxiety.

**Rotating reassurance copy:** ~8 s per line, rotate through 3–4 messages so the screen breathes. All messages calm, non-alarmist.

**Time escalation:** after ~3 min, append one calm line in `Color::Gray` (slightly dimmed): `"Your administrator may be away — your code is still valid."` No colour change, no warning glyph — escalation is editorial, not visual.

#### Act 4 — Connected (Screen: You're connected)

~1.5 s success animation, then auto-advance.

**Frame 0 (250ms):** blank, dark background — a breath before the reveal.

**Frame 1 (400ms):** large `✓` appears, centered, in `Color::Green`. Slightly oversized via padding.

**Frame 2 (350ms):** `✓` holds. "You're connected." fades in below (render text, was absent). `Color::White`, `Modifier::BOLD`.

**Frame 3 (500ms hold):** body text appears: "This computer is now linked to your workspace. Everything's ready." `Color::Gray`.

**Frame 4 (auto-advance):** F-key strip un-dims left-to-right, one key per frame at 60 fps. Status bar transitions: `▲ Not connected` → `● Connecting…` (Blue, `Pending`) → `● Connected · MBA LINK ACTIVE` (Green, `Active`). TUI enters normal chassis operation (F4 ContentCartridge).

**One-time tooltip:** first connection only, a small dimmed overlay appears at the status bar for ~5 s: `"Green dot = connected to your workspace. You're all set."` Dismiss on any keypress.

---

## 3. Part 2 — Holistic TUI Design Language

### 3.1 Design philosophy

**World-class TUI references studied:**

| App | What to steal |
|---|---|
| **btop** | Dense information at speed; accent color as the only pop of colour in a dark field; the "alive" feeling of a constantly updating display |
| **lazygit** | Modular pane vocabulary; colour as semantic signal (green=safe, yellow=changed, red=danger) used consistently; keyboard-driven with visible labels |
| **Helix (hx)** | High-information status line; mode-awareness in the bar; clean, no chrome |
| **Zellij** | Pane borders as navigation vocabulary; the "active pane" is always obvious |
| **Charm / Wish** | TUI can feel like a product, not a tool; bold use of colour + brand; Lipgloss's block/padding model is transferable to ratatui |
| **Broot** | Single-purpose, focused; the whole screen is one action |

**Guiding constraint:** the design must degrade to a 16-colour xterm with no graphics support and still be usable and recognisably the same product.

### 3.2 Colour palette

True-colour primary palette (24-bit):

| Role | Hex | 256-colour fallback | Usage |
|---|---|---|---|
| Background | `#1a1a2e` | `Color::Black` | Terminal background (may be transparent if terminal allows) |
| Surface | `#16213e` | `Color::DarkGray` | Pane backgrounds, inactive areas |
| Border | `#0f3460` | `Color::DarkGray` | All borders — same value, low contrast |
| Border active | `#e94560` | `Color::Red` | Active pane / focused widget border |
| Accent cyan | `#00b4d8` | `Color::Cyan` | Primary accent: active F-key tab, links, cursor, CTAs |
| Text primary | `#e0e0e0` | `Color::White` | Body copy, labels |
| Text secondary | `#808080` | `Color::Gray` | De-emphasized labels, hints |
| Success green | `#06d6a0` | `Color::Green` | MBA LINK ACTIVE, success states, `✓` |
| Warning amber | `#ffb703` | `Color::Yellow` | `▲ Not connected`, time escalation |
| Pending blue | `#4895ef` | `Color::Blue` | `◌ Connecting…`, waiting animation, MBA Pending |
| Error red | `#e94560` | `Color::LightRed` | **Reserved for data loss only** — never for "connection failed" |
| Code pill bg | `#0f3460` | `Color::DarkGray` | Background of the `K7Q2 · 9XMT` display |

**Rule:** red is reserved for data-loss risk only. A declined pairing is amber + grey, not red. "Something went wrong" is amber. Red in os-console means "you may lose data if you continue."

### 3.3 Typographic scale (ANSI)

ratatui has no font control — typography is expressed through weight (`Modifier::BOLD`), case (`str::to_uppercase()`), spacing (padding via `Block`), and color contrast.

| Level | How | Example |
|---|---|---|
| Display | `BOLD` + `Color::White` + surrounding whitespace | "You're connected." |
| Heading | `BOLD` + `Color::White` | Pane titles in `Block::title()` |
| Body | `Color::White` (normal weight) | Instructions, descriptions |
| Secondary | `Color::Gray` | Hints, de-emphasized labels |
| Code / data | `Color::Cyan` + bordered pill | Connection code, fingerprint (operator view) |
| Keyboard label | `Color::Cyan` key + `Color::Gray` description | `R  Show my code again` |

Keyboard labels always follow the pattern: `<key>  <plain description>` — two spaces between; key in cyan, description in grey.

### 3.4 12-unit layout grid

All layout uses ratatui's `Layout::new` with percentage or fixed constraints. Treat 12 as the grid unit.

- Content areas: 8/12 (≈67%) width, centred, leaving 2/12 margin each side
- Code pill: 6/12 (50%) of content area, centred
- QR + code split: QR 7/12, code + instructions 5/12
- Status bar: always 1 row, full width
- F-key strip: always 3 rows, full width (title + tabs + separator)

At narrow terminals (< 80 cols), collapse to single-column; QR drops first, then side-by-side layouts stack.

### 3.5 Border-as-focus vocabulary

| State | Border style | Border color |
|---|---|---|
| Default / inactive pane | `Borders::ALL`, rounded if supported | `#0f3460` / `Color::DarkGray` |
| Active / focused pane | `Borders::ALL` | `#00b4d8` / `Color::Cyan` (accent) |
| Modal / ceremony screen | `Borders::ALL` + title | `#00b4d8` / `Color::Cyan` |
| Error state | `Borders::ALL` | `#ffb703` / `Color::Yellow` (amber — not red) |

**Rounded corners** (`border_type = BorderType::Rounded`): use where terminal supports UTF-8 box-drawing. Fallback: `BorderType::Plain`.

### 3.6 Motion budget

The chassis loop already runs at ~60 fps (16 ms poll). All animations must fit within that budget.

| Animation | Duration | Trigger |
|---|---|---|
| Waiting dot cycle | 1.8 s loop | During `AwaitingApproval` state |
| Success checkmark reveal | 400 ms | On `PairingEvent::Approved` |
| F-key strip ignition | 500 ms (one key per ~42 ms) | Frames 3–4 of connected sequence |
| Status bar transition | 1 frame | Instant — state flip |
| Rotating reassurance copy | 8 s per message | During `AwaitingApproval` state |

**Reduced-motion:** if the terminal environment variable `REDUCE_MOTION=1` is set (or a future config option), skip frame-by-frame animations — show final state directly. The dot cycling becomes a static `◍ Waiting…`.

### 3.7 Redesigned status bar

Current: `jennifer@woodfine │ MBA LINK ACTIVE` (hard-coded phrasing).

**Proposed:**

```
● Connected · MBA LINK ACTIVE    jennifer@woodfine         os-console 0.1
```

- Left: link state glyph + word + technical label (de-emphasised after the human word)
- Centre: user identity (pushed to centre with `Layout::Horizontal` spacers)
- Right: app name + version (always dim)

**Glyphs by state:**
- `●` (filled circle) in `Color::Green` — Connected / MBA LINK ACTIVE
- `◌` (open circle) in `Color::Blue` — Connecting… / Pending
- `▲` (triangle) in `Color::Yellow` — Not connected / Inactive

**Do not remove `MBA LINK ACTIVE`** — it is the operator-visible technical label and stays. The human word (`Connected`) comes first; the technical label follows on the same line, dimmed (`Color::DarkGray`).

### 3.8 TUI graphics fallback tiers

| Tier | Protocol | Terminals | Quality |
|---|---|---|---|
| 1 | Kitty graphics (unicode-placeholder method) | Kitty, WezTerm, Ghostty, Konsole | Pixel-perfect; composes with ratatui layout |
| 2 | Sixel | xterm (sixel build), foot, WezTerm, contour, mlterm | Good; wider reach |
| 3 | iTerm2 inline images | iTerm2, WezTerm | Good on macOS |
| 4 | Unicode `Dense1x2` half-block | Every Unicode terminal | Functional; the guaranteed baseline |

Detection: `ratatui-image`'s `Picker::from_query_stdio()` at startup. **Over SSH (`run_with_bytes` path): default to Tier 4** — APC escapes must survive the russh channel and the user's local terminal must support the protocol; query latency over SSH is too high to probe reliably.

**Rule: the 8-char code plus Tier 4 Unicode QR is the floor. os-console must never require a graphics protocol for core function.**

---

## 4. Part 3 — Engineering prerequisites for the UI

Eight items surfaced from codebase review that must be resolved before the full pairing ceremony can function as designed:

1. **MBA must become a retrying in-TUI loop** — currently a one-shot 5 s probe before the TUI starts, runtime dropped. The animated `AwaitingApproval` → `Paired` live transition requires polling inside the event loop via `mpsc::Receiver`. (Tracked in `pairing-engineering-brief.md` §4.1.)

2. **`ratatui-image` compatibility with ratatui 0.29** — must be confirmed with `cargo add` before Phase 3 starts. If incompatible, ship Unicode-only (Tier 4) until ratatui version is bumped.

3. **`PairingInfo` → `PairingState` refactor** — static struct is not enough; the chassis needs a state enum to drive per-state rendering. (Tracked in `pairing-engineering-brief.md` §4.2.)

4. **Pairing port allocated** — suggest 9201; confirm against cluster port map before implementing `tiny_http` server. Must not collide with 9092 (`service-proofreader`).

5. **Success animation timing** — the 1.5 s connected sequence requires `Instant::elapsed()` in the chassis event loop. The chassis already has `started: Instant`; extend it with a per-animation state.

6. **F-key strip ignition** — sequential key reveal requires per-key activation state in `fkey_strip.rs`. Currently the strip renders all tabs in one pass. Add `active_up_to: FKey` field to the strip to support progressive reveal.

7. **Status bar glyph** — the `▲ / ◌ / ●` glyphs must render in 16-colour terminals without the rounded Unicode corners. Confirm `▲` is in CP437 for Windows Terminal compatibility; fallback to `!` / `~` / `*` for pure ASCII.

8. **`run_with_bytes` graphics default** — confirm that `ratatui-image`'s `Picker` is initialised differently for the SSH path vs. local PTY path. SSH: force Tier 4. Local PTY: `from_query_stdio()`.

---

## 5. Open questions (operator decisions required)

1. **`MBA LINK ACTIVE` visibility** — keep de-emphasised in normal user view, or fully hide behind an operator/advanced mode? Recommendation: keep, de-emphasised.
2. **Background colour** — `#1a1a2e` assumes a terminal that respects the background color escape. On some terminals (notably tmux with a global bg) the surface color will bleed. Confirm acceptable or use `Color::Reset` (terminal default) as background.
3. **`REDUCE_MOTION` mechanism** — env var, config key in `config.toml`, or auto-detect from `$TERM`? Recommendation: `config.toml` key `reduce_motion = false`.
4. **QR content** — is `pair://` a custom URI scheme (opens nothing on most phones) or `https://` to a future web-approve page? Recommendation: `https://` to `gateway.pointsav.com/pair/approve?c=K7Q29XMT`; fallback to the short code readout if the URL is unavailable.
5. **Sound** — success bell on Screen 4? Default: disabled. Config key: `sound = false`.
