---
artifact: brief
schema: foundry-brief-v1
title: TUI Desktop — Merged TUI/GUI Desktop Paradigm
status: archived
archived: 2026-05-27
synthesised_into:
  - BRIEF-workplace-desktop-environment.md
created: 2026-05-25
author: command@claude-code
gateway: project-software
cites:
  - BRIEF-app-workplace-architecture
  - BRIEF-sovereign-os-family-master-plan
  - doctrine-claim-1-air-gap
  - doctrine-claim-2-100-year-readability
  - doctrine-claim-34-two-bottoms
  - doctrine-claim-43-single-boundary-compute
related_briefs:
  - BRIEF-app-workplace-architecture.md
agents: 10 competing Opus research agents (WP-TUI-01..10)
---

# BRIEF — TUI Desktop Architecture
## Merged TUI/GUI Desktop Paradigm for os-workplace

> Synthesis of WP-TUI-01..10: prior art historian, non-developer UX analyst,
> terminal capabilities researcher, browser-failure analyst, rendering architect,
> application framework designer, shell-as-WM theorist, sovereignty architect,
> workflow cognitive scientist, synthesis agent.
> Written 2026-05-25, Command Session, claude-sonnet-4-6.
>
> **Relationship to `BRIEF-app-workplace-architecture.md`:** The niri-fork + Tauri v2
> graphical shape (that BRIEF, §1–§11) remains canonical for apps requiring WebView
> (app-workplace-bim, app-workplace-gis) and the graphical desktop. The TUI Desktop
> is a **second, complementary deployment shape** — sovereignty-maximal, SSH-accessible,
> and suitable for low-bandwidth / headless contexts. Both shapes share the BFS
> attribute store, substrate-handle broker, and `software.pointsav.com` distribution.

---

## 1. Definition

**The TUI Desktop is a sovereign-by-construction desktop environment in which the
GPU-accelerated terminal emulator is the display server, a Zellij-derived multiplexer
is the window manager, and ratatui applications (extended with the Kitty graphics
protocol for pixel-accurate raster content) are the apps.**

Mouse, clipboard, and inter-app data transfer are first-class — routed through the
multiplexer, not through escape sequences leaked between apps. One OS process per app
preserves the seL4 CNode capability boundary. No browser engine. No Wayland client
diversity. No network surface in the delivery channel.

---

## 2. Prior Art Survey

**What worked, what failed, and the contribution to the TUI Desktop.**

| System | Date | What it proved | Why it failed | Contribution |
|---|---|---|---|---|
| **Blit** (Bell Labs, Rob Pike) | 1982 | Mouse-in-terminal is viable; text + mouse + windows are not competing paradigms | Hardware ~$10k/unit; tethered to host | Mouse-in-terminal is 44 years old — not a research problem |
| **Plan 9 / rio / Acme** (Pike) | ~1992 | WM = shell; window is a namespace; every visible string is callable | AT&T licensing; no GPU/DRM stack; no POSIX apps | WM compresses to 2000 lines when the protocol substrate is uniform |
| **Oberon System** (Wirth & Gutknecht) | 1987 | Desktop as live, editable, executable text surface | Single-language ecosystem | "Everything visible is callable" — no separate launcher needed |
| **NeWS** (Sun, Gosling) | 1987 | Downloadable PostScript widgets; server-side rendering | Lost to X11: open + simple beats elegant + proprietary | Cautionary tale — architecture alone does not win adoption |
| **BeOS / Haiku** | 1995– | BFS queryable typed attributes; Finder-as-query, no separate DB | Be Inc. collapse | Filesystem-as-structured-store removes every app's catalogue |
| **TermKit** (Wittens) | 2011 | Structured pipes between tools; rich terminal output | Reinvented the browser (Node+WebKit); single maintainer | Pipes can carry richer types — but adding WebKit added all browser failure modes |
| **Notty** | 2013 | Terminal protocol carrying structured (non-ANSI) data | No emulator adopted it; chicken-and-egg | Protocol extension requires emulator-and-app co-launch from day one |
| **Carbonyl** | 2023 | Full Chrome rendered in terminal at 60fps via Sixel | Novelty, not workflow | Removes the "terminals can't draw" objection entirely |
| **Kitty graphics protocol** (Kovid Goyal) | 2018 | Pixel-accurate images, animation, z-index in terminal cells | — | Unblocks GUI-grade widgets (charts, PDFs, Gantt bars) in TUI |
| **Zellij** | 2020– | Multiplexer-as-desktop-shell; Wasm plugins; floating panes | No app launcher, no persistent desktop, no MIME clipboard | Extensibility (Wasm) + floating panes = proto-desktop WM |

**Key synthesis:** Prior attempts failed when they brought the browser in (TermKit).
They succeeded or remained viable when they stayed in the terminal. NeWS demonstrates
that architectural superiority without ecosystem co-launch loses to open + simple.
The TUI Desktop must co-launch emulator, multiplexer, and app framework together.

---

## 3. Non-Developer UX Requirements

**What non-developers need — what currently fails, and why.**

Modern terminals (xterm SGR 1006, WezTerm, Kitty, Alacritty) fully support mouse at
the wire level. The blockers are per-app implementation gaps, not protocol limitations.

**What breaks (per app, not per protocol):**
- **Right-click context menu.** SGR reports button-3; no TUI toolkit ships a standard context-menu widget.
- **Double/triple-click text selection.** Inside a TUI app, the emulator hands clicks to the app and OS-level selection breaks.
- **Drag-and-drop between panes.** No terminal protocol covers cross-pane drag; emulators only report drag within one surface.
- **Clipboard.** OSC 52 is the solution (xterm, WezTerm, Kitty, Alacritty, Windows Terminal ≥1.18) but historically broken by tmux defaults and GNOME Terminal until 2023.

**The desktop as workflow staging (Malone 1983 / Kidd 1994 / Boardman & Sasse 2004):**
Non-developers use the desktop NOT as a file browser but as a **cognitive offloading
surface** — a staging area for unfinished task intent. Piles encode work state that
hierarchical folders destroy (Malone). Location is meaning (Kidd). The desktop is the
only surface visible across applications (Boardman). Median dwell-time of "desktop files"
is under two weeks (Ravasio et al. 2004) — in-flight artifacts, not archive.

**Five non-negotiables for non-developer adoption:**
1. Right-click anywhere → context menu within 100ms; mouse-navigable; dismissable by click-away. No prefix key.
2. Ctrl+C / Cmd+C copies to OS clipboard end-to-end (OSC 52 + multiplexer passthrough); Ctrl+V pastes.
3. A persistent visible staging surface holding in-progress work items, backed by BFS `DESK:staged` attribute query — survives reboot.
4. Alt+Tab / Super+Tab cycles apps with a visible switcher overlay; status bar shows running apps with click-to-switch.
5. Thumbnail previews for PDF, images, and common files (Kitty graphics protocol) plus a working "Open With" that hands off to graphical apps when needed.

---

## 4. Why Browser-as-Desktop Failed (and What to Avoid)

**Structural failure pattern — common across ChromeOS, Electron, and web-based IDEs.**

| Failure | Root cause | TUI Desktop avoids it by |
|---|---|---|
| ChromeOS needs Android + Linux escape valves | Browser cannot host professional offline apps; network is load-bearing in delivery channel | Binaries are local; offline is the default; no escape valve needed |
| Electron phones home despite `connect-src 'none'` | Chromium telemetry/OCSP/font CDNs run outside page CSP — the host process is outside policy | ratatui binary has no browser engine; zero network syscalls provable via `strace` |
| Web IDEs collapse without network | Every keystroke = round-trip; offline = zero | Terminal + multiplexer run local; PPN is optional, not load-bearing |
| DOM/CSS fights app rendering | Generic layout model is wrong for spreadsheets, Gantt bars, IFC models — every serious app abandons DOM for WebGL | Cell grid IS the rendering model; Kitty graphics IS the pixel layer — no second rendering model |

**Anti-patterns the TUI Desktop must never introduce:**
- A runtime with a network surface outside application policy
- An update channel that requires outbound connectivity at run time
- A generic layout engine imposed on top of application-domain rendering
- A shared-runtime TCB larger than ~5 MLOC (Chromium is 35 MLOC; Kitty is ~65 kLOC)
- Any dependency on a vendor account or vendor-operated CDN

**What to copy from Electron's commercial success:** Electron won because HTML/CSS/JS
was existing developer muscle memory. The TUI Desktop's equivalent: POSIX shell +
ANSI sequences + plain-text files + pipes. Zero runtime network surface; no engine-CVE
inheritance; no telemetry channel outside policy.

---

## 5. Three-Layer Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│ L0 — Terminal Emulator (Kitty)                                    │
│   GPU pipeline (OpenGL/Vulkan) · KMS/DRM framebuffer             │
│   Kitty graphics image store · SGR mouse · OSC 52 (policy-gated) │
│   [v1: runs inside Wayland niri-fork]                            │
│   [v2: direct-KMS, no compositor required]                       │
└─────────────────┬────────────────────────────────────────────────┘
                  │ PTY + escape sequences (display bytes only)
┌─────────────────▼────────────────────────────────────────────────┐
│ L1 — Multiplexer-WM (pointsav-tui-shell, Zellij fork)            │
│   Layout: floating-default (non-devs) / tiling opt-in (Super+T) │
│   Focus · SGR mouse routing to panes · status bar               │
│   Fuzzy launcher (Super+D) · persistent named layouts           │
│   OSC 52 clipboard broker (per-pane policy; hard read-deny)     │
│   OSC 5522 drag-and-drop broker                                 │
│   Substrate-handle transfer broker (Unix domain socket)         │
│   Notification bus · "Desk" pane (BFS attribute queries)        │
│   Control IPC: $XDG_RUNTIME_DIR/foundry-wm.sock (MessagePack)  │
└─────────────────┬────────────────────────────────────────────────┘
                  │ UDS IPC (handle broker, notifications, DnD)
┌─────────────────▼────────────────────────────────────────────────┐
│ L2 — TUI Apps (one OS process each; one seL4 CNode each)         │
│   ratatui 0.29+ + ratatui-pixels (Kitty graphics adapter)       │
│   crossterm SGR mouse → DesktopEvent dispatch                   │
│   BFS attribute queries for staged items / recent docs          │
│   App-to-app transfer: BfsHandle via broker (not OSC 52)        │
└──────────────────────────────────────────────────────────────────┘
```

**Layer boundaries:** L0 is the only GPU client. L1 and L2 see cells; pixels are a
side-channel L0 honours. Apps talk to the WM via the **Unix socket control plane** —
never via terminal escape sequences, which are lossy, untyped, and collide with
app rendering.

**Rendering budget (AArch64, 60fps):**
- Gantt chart (400×300px): ~2ms encode + ~1ms upload + 0.3ms place = **~3.3ms/pane**
- Mouse click-to-pixel latency: **~10ms** (comparable to Wayland ~8ms)
- Max simultaneously-animated panes: **~30** (CPU-bound, ratatui diff ~0.4ms/pane)
- Max Kitty-graphics panes at 60fps: **8–12** (PTY bandwidth-bound, ~80MB/s aggregate)

---

## 6. Application Framework

**`ratatui` + `ratatui-pixels` + `tui-desktop-widgets` — all EUPL 1.2, monorepo.**

### Framework baseline

**ratatui 0.29+** (MIT, AArch64-clean, zero C deps via crossterm). Chosen over
notcurses (C linking + EUPL redistribution friction) and raw crossterm (duplicates
widget composition).

**`ratatui-pixels` (new crate, `tools/tui-desktop/ratatui-pixels/`):**

```rust
pub trait KittyGraphicsBackend {
    fn upload(&mut self, id: ImageId, png: &[u8]) -> Result<()>;
    fn place(&mut self, id: ImageId, cell: (u16, u16), z: i32);
    fn delete(&mut self, id: ImageId);
}
```

Falls back to Sixel (xterm, foot) → Unicode half-block (any terminal). Capability
detection via DA1 `CSI ? c` probe at startup.

### Mouse event model

```rust
pub enum DesktopEvent {
    Click { btn, cell, modifiers, count },  // count enables double-click
    Drag  { from, to, btn },
    Hover { cell },                         // 16ms throttle
    ContextMenu { cell },                   // right-click synonym
}
```

SGR decode (crossterm) → `HitTester` walks frame widget rect tree → `WidgetId` →
`WidgetController` dispatch. Latency budget: **<8ms input-to-redraw** on AArch64.

### Clipboard

`tui-clipboard` crate (new). OSC 52 write only by default. Security hard rules:
never echo write payload back to TTY; strip C0/C1 from incoming paste; cap at 64KiB;
require explicit user gesture — never programmatic clipboard read.

### Drag-and-drop IPC

`OSC 5522` private-use protocol brokered via `pointsav-tui-shell`:
```
offer:  OSC 5522 ; offer ; <uuid> ; <mime-list> ; <size> ST
accept: OSC 5522 ; accept ; <uuid> ; <chosen-mime> ST
```
Payload >64KB bypasses PTY — broker negotiates a direct Unix socket between pane PIDs.
PTY carries handshake only. Mirrors `wl_data_device` semantics without Wayland.

### New widgets (not in ratatui today)

| Widget | Crate | Gap filled |
|---|---|---|
| `IconGrid` (file picker) | `tui-desktop-widgets` | Grid view + Kitty thumbnails |
| `Toast` | `tui-desktop-widgets` | Overlay z-order + auto-dismiss timer |
| `Button` | `tui-desktop-widgets` | Mouse hit-test + `on_click` + modal focus trap |
| `SplitPane` | `tui-desktop-widgets` | Drag handle consuming `Drag` events |
| `Image` | `tui-desktop-widgets` | Wraps `ratatui-pixels` |
| `Gauge` sub-cell | `tui-desktop-widgets` | Unicode 1/8 blocks + 60Hz tick |

All in `tools/tui-desktop/tui-desktop-widgets/` (EUPL 1.2).

---

## 7. Multiplexer as Window Manager

**`pointsav-tui-shell` — Zellij fork.**

### What Zellij already provides

Tiling + floating panes, SGR mouse routing, Wasm plugin host (`wasmtime`,
`PluginRunner`), status/tab bar plugins, persistent sessions, MessagePack control
IPC (`~/.cache/zellij/<session>/zellij`). **2–3 engineer-months from a minimal
desktop shell.**

### What the fork adds

| Feature | Implementation |
|---|---|
| `Super+D` fuzzy launcher | New Wasm plugin; reads `app-workplace-*` manifest |
| "Desk" pane | BFS `DESK:*` attribute queries; four zones (Recent / Active / Staged / Notifications) |
| OSC 52 clipboard broker | Per-pane policy; **hard read-deny** by default; explicit grant per app identity |
| OSC 5522 DnD broker | Offer/accept handshake; Unix socket pairs for large payloads |
| Floating-default mode | Stacking for non-developers; `Super+T` flips to tiling |
| Named layout persistence | `--layout desk` reconstructs full session on restart |
| `foundry-wm.sock` | MessagePack ops: `open_pane`, `focus`, `move`, `float`, `list_panes`, `request_handle`, `post_notification` |

### The Plan 9 lesson applied

When the substrate is uniform (one multiplexer owns layout + clipboard + IPC), window
management compresses to ~2000 lines of Rust. The complexity budget saved versus
building a Wayland compositor is ~300 kLOC. Open + simple + co-launched beats
elegant + proprietary (see §2 NeWS entry).

### Non-developer screen (bookkeeper's default layout)

One maximised floating pane running the bookkeeping TUI; status bar bottom showing
running apps with click-to-switch; `Super+Tab` cycles panes; `Super+D` shows fuzzy
launcher overlay. Tiling is invisible until the user runs `Super+T`. This matches the
stacking-default mandate in `BRIEF-app-workplace-architecture.md §10.3`.

---

## 8. Desktop as Workflow Staging Area

**The "Desk" pane — cognitive science basis.**

Three peer-reviewed findings anchor the design:
- **Malone (1983)** "How Do People Organize Their Desks?" (ACM TOIS): office workers *pile* (temporal, reminding) not *file* (categorical, retrieval). Piles encode unfinished task intent; filing destroys the signal.
- **Kidd (1994)** "The Marks Are on the Knowledge Worker": physical location is meaning — filing removes a document from working cognition.
- **Boardman & Sasse (2004)** "Stuff Goes Into the Computer and Doesn't Come Out": the desktop is the only surface visible across applications — hence the forced piling behaviour.

**Three non-negotiable cognitive functions:**
1. **Cross-application staging without filing** — user groups heterogeneous artefacts by current task, recalls by task not path. → BFS `DESK:staged=true` query.
2. **Spatial persistence across restart** — layout, staging pile, and active-task list identical on re-attach. → Zellij named-layout + BFS attribute store.
3. **Visible committed toolset** — apps "always kept open" reappear in the same screen positions each session. → `zellij --layout desk` at session start.

**"Desk" pane — four zones:**

| Zone | BFS query | Behaviour |
|---|---|---|
| Recent | `DESK:opened_at` per-app top-5 | Auto-populated, no user action |
| Active tasks | `DESK:flagged=true` | Any app can flag; cross-app; click to open |
| Staging pile | `DESK:staged=true` | User explicitly stages via `:desk` command or `OSC 5522` drop |
| Notifications | `DESK:notify=true, unread=true` | Dismissible; survives restart |

"I left that invoice on the desktop" resolves to `bfs query DESK:staged=true` at
login. File location is irrelevant; the attribute is the desktop.

---

## 9. Sovereignty Analysis

**On balance: MORE sovereign than Tauri v2. One structural regression, hard-mitigated.**

| Dimension | TUI Desktop | Tauri v2 | Verdict |
|---|---|---|---|
| Network surface | ratatui: no TLS/DNS/HTTP/JS; air-gap via `strace -e trace=network` on child | WebKitGTK: TLS + HTTP3 + QUIC reachable from any RCE bypassing CSP | **TUI wins** — structural, not policy |
| Air-gap proof (Doctrine #1) | Single `seccomp`/`strace` rule; packet capture trivially clean | Must trust CSP inside 2M-LOC engine | **TUI wins** |
| Shared-TCB blast radius | Kitty 0-day reaches all TUI apps sharing one emulator window | WebKit 0-day reaches one Tauri app | **TUI regression** |
| seL4 CNode boundary | One OS process per app = one CNode; Kitty is display server, not process boundary | One Tauri process per app | **Equal** (with one-window-per-app rule) |
| OSC 52 exfiltration | Any child can silently overwrite clipboard if emulator allows write | No equivalent channel | **TUI regression** |

**Hard requirements (non-configurable in `pointsav-tui-shell` defaults):**
1. OSC 52 read: **hard-deny** for all panes by default; explicit per-app grant only.
2. **One Kitty window per app-workplace-* instance** — never share an emulator window between apps (prevents shared-TCB blast radius and Kitty graphics ID collision).
3. Terminal emulator treated as a **vendored, audited substrate** — distributed via `software.pointsav.com` as a signed binary, not user-replaceable.

---

## 10. Relationship to niri-fork + Tauri v2 Shape

The TUI Desktop is **not** a replacement. Two deployment shapes, one monorepo:

| Shape | Primary use case | Stack |
|---|---|---|
| **Graphical (Tauri)** | BIM/GIS workflows; WebView-required apps; full desktop | niri-fork compositor → Tauri v2 → WebKitGTK |
| **TUI Desktop** | Sovereignty-maximal; SSH-accessible; low-bandwidth; server/headless | Kitty → `pointsav-tui-shell` → ratatui + `ratatui-pixels` |

Both share: BFS attribute store · substrate-handle broker · `DESK:*` namespace ·
`software.pointsav.com` distribution ($1 Apache 2.0 / $19 FSL, per ratified substrate 2026-05-22).

---

## 11. Ship Path

### v0.1 — Minimum viable TUI Desktop (parallel to app-workplace-* roadmap)

1. Package Kitty as a signed `os-workplace` component binary (AArch64 + x86_64) on `software.pointsav.com`.
2. Fork Zellij → `pointsav-tui-shell`; add fuzzy launcher, OSC 52 broker (hard read-deny), `OSC 5522` DnD broker, floating-default, named-layout persistence, `foundry-wm.sock`.
3. Publish `ratatui-pixels` and `tui-desktop-widgets` in monorepo (`tools/tui-desktop/`).
4. Port two existing ratatui surfaces (proofreader CLI, bookkeeping TUI) to mount as panes — first end-to-end demo.
5. Ship "Desk" pane backed by BFS `DESK:*` attribute queries.

**Does not block the app-workplace-* roadmap.** Runs on a parallel crate cluster.

### Moonshot path

| Phase | What changes |
|---|---|
| **v2** | Kitty runs direct-KMS (`kitty +launch --no-wayland`); Wayland removed from sovereign deployments |
| **v3** | Terminal emulator as seL4 native task; multiplexer and each ratatui app as separate CNodes; no Linux desktop stack; full formal-verification substrate |

v3 is the structural endpoint no hyperscaler desktop can match: formally-verified
inter-process isolation at the display-server boundary, with no browser engine,
no Electron dependency, and no vendor account requirement — by construction.

---

---

## 12. Windows/macOS Extension — Office Worker Deployment

> Research basis: five competing agents (WP-WIN-01..05: platform compatibility,
> office UX analyst, distribution/BFS architect, SSH remote scenario analyst,
> competitor leapfrog analyst). Written 2026-05-25, Command Session, claude-sonnet-4-6.

### 12.1 Two Deployment Models

The TUI Desktop as specified in §1–§11 assumes a Linux host. Extending it to office
workers on Windows or macOS requires choosing between two structurally different
deployment models.

**Model A — Native Packaged App (primary; office-worker-first)**

Bundle WezTerm + `pointsav-tui-shell` + ratatui app binaries into a single
installable artifact. The user double-clicks `PointSav Bookkeeper.app` or
`PointSav Bookkeeper.exe` — they never see a terminal emulator.

- **macOS L0:** WezTerm (more packageable than Kitty; native Dock + menu bar
  integration; full KGP support; ships as `.dmg`). Kitty remains the preferred L0
  on Linux for its GPU pipeline and reference KGP implementation.
- **Windows L0:** WezTerm (only viable option — Kitty has no native Windows binary;
  Windows Terminal lacks KGP entirely). WezTerm ships a signed `.exe` installer.
- **No WSL2 required.** WSL2 requires admin rights, triggers "why am I installing
  Linux?" confusion, and conflicts with common corporate hypervisors (Hyper-V,
  VirtualBox). Fatal for non-technical office workers.

**Model B — Remote Session via WireGuard PPN (secondary; server-operator model)**

Linux server runs the full Kitty + `pointsav-tui-shell` stack. Office workers connect
via WireGuard PPN (already in PointSav design per SYS-ADR-16), then direct TCP to
`pointsav-tui-shell`. The client needs only WezTerm on macOS/Windows to render
KGP sequences over the tunnel.

- 5–10 user shared Linux server: viable at ~$50–100/mo (GCP e2-medium), one Linux
  user account per office worker. Standard Unix file permissions provide isolation.
- **Not a Citrix replacement.** Local printer integration, file drag-drop from
  macOS Finder or Windows Explorer, and audio are not available without custom
  bridges. Suitable for keyboard-driven data entry (bookkeeping, property
  management); unsuitable for traditional office peripheral workflows.
- **Mosh: incompatible.** Mosh strips terminal control sequences for reliability;
  the Kitty graphics protocol (`OSC _G`) does not survive Mosh transit. Use
  WireGuard + direct TCP instead of SSH+Mosh.
- **Kitty graphics state does not survive Zellij detach/reattach.** Images are
  uploaded to the client terminal's local cache. On re-attach (a new client
  connection), the cache is empty. Apps must re-upload images on every render;
  do not assume `ImageId` persistence across sessions.

---

### 12.2 Platform Compatibility Matrix

Five terminals tested against the TUI Desktop's three critical protocols:

| Feature | Linux (Kitty) | macOS (Kitty) | macOS (WezTerm) | Windows (WezTerm) | Windows Terminal |
|---|---|---|---|---|---|
| Kitty graphics protocol | ✅ Full | ✅ Full | ⚠️ Partial | ⚠️ Partial | ❌ None |
| OSC 52 clipboard | ✅ | ✅ | ✅ | ✅ | Write-only |
| SGR 1006 extended mouse | ✅ | ✅ | ✅ | ✅ | ⚠️ Limited hover |
| GPU acceleration | ✅ OpenGL | ✅ Metal | ✅ | ❌ Software | N/A |
| Native app packaging | AppImage | ✅ .dmg | ✅ .dmg | ✅ .exe | Built-in |
| First-class deployment target | ✅ primary | ✅ Model A | ✅ Model A | ✅ Model A | ❌ not viable |

**WezTerm KGP limitations (macOS + Windows):** z-index (layered images) and frame
animation are not implemented as of WezTerm 20250123. The `ratatui-pixels`
`KittyGraphicsBackend::place()` call must degrade gracefully when
`z_index_available = false` (flat render order; all images at z=0).

**macOS Terminal.app:** does not support OSC 52 or Kitty graphics — not usable
as L0 for any TUI Desktop feature set.

---

### 12.3 WezTerm as L0 on macOS/Windows

WezTerm (MIT licence, Rust, maintained by Wez Furlong) is the deployment terminal for
both macOS and Windows Model A packaging. Reasons it is preferred over Kitty for
cross-platform packaging:

- Ships a signed native macOS `.dmg` (`.app` bundle with Dock icon, menu bar) and
  a signed Windows `.exe` installer — directly packageable without a wrapper.
- Lua configuration baked at build time: `wezterm.lua` can embed the
  `pointsav-tui-shell` launch command, window title, font, colour scheme, and key
  bindings so the user never edits a config file.
- Full SGR 1006 mouse and OSC 52 clipboard on all platforms (same as Kitty).
- KGP image upload and chunked transfer work; z-index and animation are the only
  gaps (§12.8.2 below documents the mitigation).

The `tui-desktop-launcher` script (§12.8.4) is the entry point for both platforms:
it opens WezTerm with the pre-baked config and starts `pointsav-tui-shell` in the
foreground. The `.dmg` / `.exe` installer points to this launcher, not to WezTerm
directly.

---

### 12.4 iTerm2 Compatibility — OSC 1337 Backend

iTerm2 (macOS) uses its own image protocol (`OSC 1337 ; File=...`), entirely
separate from Kitty's `OSC _G`. They are not forwards-compatible. A user running
the TUI Desktop inside iTerm2 will see blank space where images should appear —
silent failure, no error.

**Detection:** `$TERM_PROGRAM=iTerm.app` is set by iTerm2 at startup.

**Required addition to `ratatui-pixels`:** an `iTerm2Backend` implementing
`KittyGraphicsBackend` via `OSC 1337`. This extends the existing capability
fallback chain to:

```
Kitty (_G protocol)
  → WezTerm (flat KGP, no z-index)
  → iTerm2 (OSC 1337)
  → Sixel (foot, xterm with sixel)
  → Unicode half-block (any terminal)
```

Capability detection at startup via DA1 (`CSI ? c`) probe plus `$TERM_PROGRAM`
environment variable. The selected backend is stored as
`pointsav_tui_shell::graphics::Backend` and passed to every app via the
`foundry-wm.sock` `env_caps` message at pane spawn.

---

### 12.5 Office Worker UX Gaps and Fixes

The "comfortable" threshold for a non-developer office worker has four criteria:
(1) double-click to install — no terminal required; (2) muscle memory intact — OS
keyboard shortcuts not overridden; (3) recognizable patterns — files, apps, search
visible without a manual; (4) the system looks legitimate — signed, no security
warnings. The TUI Desktop as specified in §1–§11 meets 0 of 4 criteria on
Windows/macOS without the additions below.

**UX gaps ranked by severity:**

| Gap | Windows | macOS | Fix |
|---|---|---|---|
| Installation requires terminal literacy | Fatal | Fatal | Model A packaging (§12.1) |
| Gatekeeper unsigned-app warning | — | Fatal | Notarize .dmg ($99/yr; §12.7) |
| `Super+D` = Show Desktop (Windows) | Direct collision | — | Windows keybinding profile: `Ctrl+Space` for launcher |
| `Cmd+Space` = Spotlight (macOS) | — | Overlap | macOS keybinding profile: `Cmd+P` for launcher |
| No onboarding — blank screen on first launch | High | High | Onboarding pane (§12.8.6) |
| DESK pane is query-only — no spatial file tree | Medium | Medium | Add Files sidebar alongside DESK; both modes available |
| Status bar not visually afforded as interactive | Medium | Medium | Hover highlight + tooltip "Click to switch" |
| No Exposé / tiled overview | — | Low | F3 toggle deferred to v0.2 |

**Keyboard profile rule:** the OS keybinding layer (§12.8.5) must never override
`Alt+Tab` on Windows or `Cmd+Tab` on macOS — these are the deepest muscle-memory
shortcuts on each platform. New launcher and pane-management shortcuts are additive.

**Spatial file access:** the DESK pane (§8) must coexist with a traditional folder
tree, not replace it. Research (Malone 1983; confirmed by office worker agent) shows
that query-based staging is valued but only when spatial navigation is also available.
The Files sidebar is a narrow `tui-desktop-widgets::FileTree` widget (existing tree
rendering; no new protocol).

---

### 12.6 BFS Attribute Store — Cross-Platform SQLite Architecture

Raw filesystem attributes (`xattr` / NTFS ADS) are unreliable as the backing store
for `DESK:*` queries on Windows and macOS:

- **macOS:** iCloud Drive and AirDrop strip `user.*` extended attributes. Time
  Machine preserves them; USB copy preserves them.
- **Windows (NTFS ADS):** OneDrive, Google Drive, and Dropbox all delete alternate
  data streams — documented as an intentional security measure to prevent malware
  hiding in ADS. The `xattr` crate (crates.io) has no Windows support.

**Recommendation: always-SQLite sidecar database.**

```
Linux/macOS:  ~/.local/share/pointsav/desk-metadata.db
Windows:      %APPDATA%\PointSav\desk-metadata.db
```

```sql
CREATE TABLE IF NOT EXISTS file_attributes (
    file_path    TEXT    PRIMARY KEY,
    file_inode   INTEGER,             -- change detection (Linux/macOS)
    file_mtime   INTEGER,             -- stale check
    desk_staged  INTEGER DEFAULT 0,   -- BOOLEAN
    desk_flagged INTEGER DEFAULT 0,   -- BOOLEAN
    desk_opened_at INTEGER,           -- Unix timestamp
    content_hash TEXT,                -- SHA-256; enables move tracking
    modified_at  INTEGER NOT NULL
);
CREATE INDEX idx_desk_staged  ON file_attributes(desk_staged);
CREATE INDEX idx_desk_flagged ON file_attributes(desk_flagged);
```

Configuration:
- `PRAGMA journal_mode = WAL;` — concurrent reads from Desk pane + writes from apps.
- `PRAGMA synchronous = NORMAL;` — durable enough for staging metadata; not
  financial records.
- Cleanup task runs at session start: `DELETE FROM file_attributes WHERE
  NOT EXISTS (SELECT 1 FROM stat(file_path))` removes orphaned entries for
  deleted files.
- Optional cache marker: on Linux/macOS only, also write
  `user.pointsav.desk-synced=1` xattr as a fast validity hint. Never read
  `DESK:*` values from xattr — SQLite is the truth.

The `desk-metadata` crate (§12.8.3) wraps this schema and exposes
`DeskStore::set(path, key, value)`, `DeskStore::query(predicate)`, and
`DeskStore::evict_orphans()`. Estimated implementation: ~1,200 LOC Rust
(`rusqlite 0.31+` crate).

---

### 12.7 Distribution Requirements

| Platform | Primary format | Secondary | Signing requirement | Annual cost | Tooling |
|---|---|---|---|---|---|
| **Linux** | Binary on `software.pointsav.com` + AppImage | Homebrew tap | Ed25519 (existing pipeline) | $0 | Current `bin/deploy-binary.sh` |
| **macOS** | `.dmg` (notarized) | Homebrew cask | Developer ID cert + `xcrun notarytool` | $99 Apple Dev | Tauri bundle + `codesign` + `stapler` |
| **Windows** | `.exe` NSIS installer | winget manifest | Authenticode OV cert | €250–350 | Tauri NSIS + `signtool` |

**macOS notarization process (per release, ~30 min):**
```bash
codesign -s "Developer ID Application: PointSav Digital Systems" \
  --timestamp --options=runtime PointSavBookkeeper.app
xcrun notarytool submit PointSavBookkeeper.dmg \
  --apple-id open.source@pointsav.com \
  --team-id <TEAM_ID> --wait
xcrun stapler staple PointSavBookkeeper.dmg
```
Without notarization, macOS 12+ blocks launch entirely ("cannot check for malicious
software"). There is no workaround acceptable to a non-technical office worker.

**Windows Authenticode (OV certificate):** SmartScreen "unrecognized app" warning
appears for the first ~30 downloads of an OV-signed binary as Microsoft's reputation
service accumulates evidence. EV certificate (~€500–700/yr) bypasses the throttle
immediately. Recommendation: ship OV first; upgrade to EV at v1.0 if early users
report friction.

**Homebrew / winget:** these are secondary — additive for technical staff at the same
office, not required for the office-worker target. Homebrew tap requires a PR to the
`homebrew-core` repo (~2 week review). winget requires a PR to
`microsoft/winget-pkgs` (~2–3 weeks).

---

### 12.8 Architecture Additions

Six new components are required to support the Windows/macOS office-worker deployment.
None of these block the existing §11 v0.1 ship path on Linux.

#### 12.8.1 `ratatui-pixels` — iTerm2 Backend

New struct `iTerm2Backend` implementing `KittyGraphicsBackend`:

```rust
pub struct iTerm2Backend<W: Write> {
    writer: W,
}

impl<W: Write> KittyGraphicsBackend for iTerm2Backend<W> {
    fn upload(&mut self, id: ImageId, png: &[u8]) -> Result<()> {
        // OSC 1337 ; File=inline=1;width=auto:<base64(png)> BEL
        write!(self.writer, "\x1b]1337;File=inline=1;width=auto:{}\x07",
               BASE64.encode(png))?;
        Ok(())
    }
    fn place(&mut self, _id: ImageId, _cell: (u16, u16), _z: i32) {
        // iTerm2 places inline at upload site; no separate place call
    }
    fn delete(&mut self, _id: ImageId) {
        // no deletion in OSC 1337; images are ephemeral
    }
}
```

Detection: `std::env::var("TERM_PROGRAM").as_deref() == Ok("iTerm.app")`.

#### 12.8.2 `ratatui-pixels` — Flat-Compositing Mode

Extend `KittyGraphicsBackend` with a capability query:

```rust
pub trait KittyGraphicsBackend {
    fn upload(&mut self, id: ImageId, png: &[u8]) -> Result<()>;
    fn place(&mut self, id: ImageId, cell: (u16, u16), z: i32);
    fn delete(&mut self, id: ImageId);
    fn z_index_available(&self) -> bool { true }  // WezTerm: false
}
```

All widget code using `place()` must check `z_index_available()` and fall back to
z=0 flat compositing (paint order = insertion order). No z-index assumptions in
`tui-desktop-widgets`.

#### 12.8.3 `desk-metadata` Crate

Location: `tools/tui-desktop/desk-metadata/` (EUPL 1.2).

```rust
pub struct DeskStore {
    conn: rusqlite::Connection,
}

impl DeskStore {
    pub fn open(data_dir: &Path) -> Result<Self>;
    pub fn set(&self, path: &Path, key: DeskKey, value: DeskValue) -> Result<()>;
    pub fn get(&self, path: &Path, key: DeskKey) -> Result<Option<DeskValue>>;
    pub fn query(&self, predicate: DeskPredicate) -> Result<Vec<PathBuf>>;
    pub fn evict_orphans(&self) -> Result<usize>;
}

pub enum DeskKey { Staged, Flagged, OpenedAt, ContentHash }
pub enum DeskPredicate { Staged, Flagged, OpenedAfter(i64) }
```

Replaces all direct `xattr` calls in `pointsav-tui-shell` and `tui-desktop-widgets`.

#### 12.8.4 `tui-desktop-launcher`

Platform-aware entry point for the packaged `.app` / `.exe`.

- **macOS:** shell script embedded in `.app/Contents/MacOS/`; opens WezTerm with
  `--config-file $BUNDLE/Contents/Resources/wezterm.lua` and passes
  `pointsav-tui-shell --layout desk` as the program.
- **Windows:** batch file in the NSIS installer; runs
  `wezterm.exe --config-file %APPDATA%\PointSav\wezterm.lua`.
- `wezterm.lua` is pre-baked (PointSav colour scheme, font, window title
  `"PointSav Desktop"`, key bindings per OS profile). Users do not edit it.

#### 12.8.5 OS Keybinding Layer in `pointsav-tui-shell`

New config section `[keybindings]` with an `os_profile` field:

```toml
[keybindings]
os_profile = "auto"   # "auto" | "linux" | "macos" | "windows"
```

Auto-detection: `cfg!(target_os = "macos")` / `cfg!(target_os = "windows")`.

| Action | Linux default | macOS profile | Windows profile |
|---|---|---|---|
| Fuzzy launcher | `Super+D` | `Cmd+P` | `Ctrl+Space` |
| Tiling toggle | `Super+T` | `Cmd+T` | `Ctrl+T` |
| Close pane | `Super+W` | `Cmd+W` | `Ctrl+W` |
| App switcher | `Super+Tab` | `Cmd+\`` | `Alt+\`` |
| New pane | `Super+N` | `Cmd+N` | `Ctrl+N` |

Hard rules: never bind `Alt+Tab` (Windows) or `Cmd+Tab` (macOS) — these are
OS-level shortcuts that cannot be captured inside a terminal window anyway.

#### 12.8.6 Onboarding Pane

A `WelcomePane` ratatui widget shown on first launch (guarded by
`~/.local/share/pointsav/onboarding.json` completion flag).

Three steps:
1. **Welcome** — two-sentence description; PointSav logo (Kitty graphics if
   available, Unicode art fallback). Press Enter to continue.
2. **Import** — import from QuickBooks / Excel (CSV drop zone using `OSC 5522`
   DnD broker) or skip. Pressing `i` opens the FileTree widget filtered to `.csv`.
3. **Your desktop** — loads the `desk` named layout with a visible status bar,
   Files sidebar, and DESK staging pane. Welcome pane closes; writes completion flag.

Estimated effort: ~600 LOC in `tui-desktop-widgets::onboarding`.

---

### 12.9 Leapfrog Positioning by User Type

The felt advantage over the incumbent tool must be demonstrable within 15–20 minutes
of first launch. One concrete demo per user type:

| User type | Incumbent | Felt leapfrog claim | Demo action |
|---|---|---|---|
| **Bookkeeper (macOS)** | QuickBooks ($600/yr subscription) | "Your books live on your machine. $0 licensing. Auditor verifies with one command — no login required." | Export ledger to plain YAML; run `sha256sum ledger.yaml`; email hash to auditor |
| **Property manager (Windows)** | Excel + Dropbox | "All tenant records in one flat file. Works offline. Fits on a USB drive." | Disconnect WiFi; post a rent payment; show committed entry; copy to USB |
| **HR manager (macOS)** | HR SaaS platform | "Personnel records never leave your laptop. Every change is cryptographically signed." | Show a personnel record's signature; run `verify` in the HR TUI |

Language that resonates with office workers: "your files, your machine, no login,"
"works on a plane," "your auditor doesn't need an account."

Language to avoid in office-worker-facing copy: "formally verified microkernel,"
"WORM-compliant ledger," "seL4 CNode," "BFS attribute store." These are
implementation vocabulary — correct but alienating at first contact.

---

### 12.10 What Does Not Change

The Windows/macOS extension is an additive layer. Everything in §1–§11 remains
canonical:

- The three-layer architecture (L0/L1/L2) is unchanged. WezTerm is an alternative
  L0, not a replacement architecture.
- `pointsav-tui-shell` (Zellij fork) runs identically on all platforms. The OS
  keybinding layer is a config option, not a code fork.
- The `DESK:*` BFS namespace is unchanged. `desk-metadata` is the storage backend;
  the query API surface is identical.
- The sovereignty verdict (§9) is unchanged and strengthened on Windows: a ratatui
  binary under WezTerm has the same zero-network-syscall provability as on Linux.
  WezTerm itself has a smaller TCB than Kitty (~40 kLOC vs ~65 kLOC); the
  sovereignty posture is equivalent.
- The `software.pointsav.com` distribution model ($1 Apache 2.0 / $19 FSL) is
  unchanged. macOS and Windows add platform-specific signing overhead but not a
  different pricing or licensing model.
- The moonshot path (§11 v2 direct-KMS, v3 seL4 native) remains Linux-only — this
  is correct and intentional. Windows/macOS are deployment targets for v0.1 and
  v0.2; they are not the sovereignty-maximal endpoint.

---

*§12 authors: WP-WIN-01..05 research agents | 2026-05-25*

---

## 13. The Developer Desktop — Stripped DE for Developers

> **Deployment shape name:** `os-developer` (engineering wordmark) / "Developer Desktop"
> (vendor-facing wordmark).
> **Position in the three-shape family:** the middle shape — keeps a graphical Wayland
> session for the Tauri v2 application suite while stripping every desktop subsystem a
> developer would not personally launch from a keystroke.

### 13.1 Concept: the third deployment shape

PointSav ships three complementary deployment shapes that share a sovereignty posture
but serve different use patterns:

| Shape | Engineering name | Session model | Primary user |
|---|---|---|---|
| Graphical Desktop | `os-workplace` | Wayland + Tauri v2 WebView | Office workers, BIM/GIS users |
| TUI Desktop | `os-tui` | Kitty + pointsav-tui-shell + ratatui | SSH/headless/remote operators |
| **Developer Desktop** | **`os-developer`** | **Wayland + Tauri v2, stripped WM chrome** | **Developers building PointSav** |

The Developer Desktop is not a fork of the Graphical Desktop and not a graphical layer
over the TUI Desktop. It is a distinct image whose compositor and chrome are assembled
from a minimal set of keyboard-first, auditable, Rust-where-possible components that
satisfy exactly what a working developer needs — and stops there.

The engineering prefix `os-*` is consistent with `os-workplace`, `os-console`, and
`os-orchestration` in the monorepo taxonomy. The `app-developer-*` namespace is reserved
for surface crates if needed.

**Relationship to the Graphical Desktop:** the Developer Desktop's compositor
(`niri-fork-developer`) and the Graphical Desktop's compositor (`niri-fork-graphical`)
share 95 percent of the niri upstream source tree. They are two build profiles of one
fork repository, not two separate codebases. What differs is documented in §13.6.

**Relationship to the TUI Desktop:** the terminal emulator (kitty), the session greeter
(`greetd` + `tuigreet`), the audio stack (PipeWire + WirePlumber), and the package
format (`pointsav-pkg`) are shared. A developer running the Developer Desktop dogfoods
the same kitty build and the same pointsav-tui-shell that ships to TUI Desktop customers.

### 13.2 Five design principles

These five principles are ordered by primacy. A later principle never overrides an
earlier one; when they appear to conflict, the earlier principle wins and the resolution
is documented as a TOML opt-in, not a default behavior change.

**1. Keyboard completeness over mouse discoverability.**
Every workflow — launch, focus, workspace switch, screenshot, lock, logout, secret reveal,
Doorman tier change, network status — has a `Super+*` keybind. Mouse and trackpad are
accelerators, never gates. Test: any UI surface whose primary action requires a mouse
fails the Developer Desktop review.

**2. Auditable surface over rich integration.**
The Developer Desktop ships fewer components, each with a readable Rust or single-purpose
C surface, instead of a single integrated DE stack. GNOME lifecycle services (online
accounts, search providers, Evolution Data Server, GNOME Software) are excluded so the
TCB and the network surface remain enumerable on one page. Test: `ss -tunlp` on a fresh
boot fits in a 25-line terminal.

**3. One process per app, capability-scoped.**
Every Tauri v2 app, every shell helper, every daemon runs as its own `systemd --user`
unit with an explicit capability set encoded in TOML. No shared D-Bus session grants
arbitrary services arbitrary peers. Test: kill any single daemon; confirm only its
dependent surface degrades and no other app crashes.

**4. Rust where it touches the user; C/Wayland-protocol where it touches the kernel.**
Status bar, launcher, notification daemon, secret broker, clipboard manager, screen
locker, polkit agent — all Rust, all maintained under `pointsav-monorepo`. Compositor,
input device handling, kernel drivers — vendored upstream (niri, libinput, kernel). We
do not write a new compositor; we fork niri because it is already Rust (Smithay). Test:
every dependency above the kernel that the developer interacts with daily compiles under
`cargo build --workspace`.

**5. The same binary the customer downloads is the same binary the developer dogfoods.**
No "dev mode" branch. `os-developer.img` enters the `software.pointsav.com` release
manifest at v0.3. The developers building PointSav use the Developer Desktop. Test:
`os-developer.img` is in the release manifest and the build host is demonstrably one of
them (build-host attestation chain, §13.12).

### 13.3 Prior art — who tried this and what stopped them short

**System76 COSMIC** (2024–2026, Rust, Smithay-based compositor) is the closest prior art
and the strongest external validation that the approach is viable. COSMIC ships:
simultaneous tiling + floating per workspace, RON config files, Rust throughout, no GNOME
dependency. Gaps relative to the Developer Desktop: targets general consumers as well as
developers (no Doorman-aware launcher, no per-app capability visibility, no
sovereignty-first network audit), and its aesthetic skews toward the consumer polish of
Pop!_OS rather than the auditable-tool signal PointSav needs.

**Regolith Linux** (i3/sway inside a GNOME session) is the most-installed
"developer-without-rice" distribution as of 2026. It keeps `gnome-settings-daemon` for
hardware support (HiDPI, suspend/resume, Bluetooth) while stripping GNOME Shell. Gaps:
X11-first (i3) or Wayland with sway (fractional scaling broken on sway), no sovereignty
posture, no Doorman integration.

**Omarchy** (DHH, 2024–2026) is an opinionated Arch + Hyprland distribution explicitly
aimed at macOS refugees. It ships a curated default aesthetic, a one-line install, and
sensible keybind defaults. Gaps: Hyprland (C, animation-first, plugin-churn); no
sovereignty posture; no business-application stack.

**The gap all three leave open:** none of them makes the developer's capability surface
visible and live-revokable from the keyboard. That is the Developer Desktop's
distinguishing move, not the window manager choice.

### 13.4 Component stack — INCLUDED vs EXCLUDED

| Layer | INCLUDED | EXCLUDED and why |
|---|---|---|
| **Compositor** | `niri-fork-developer` (Rust, Smithay) — scrollable columns tuned for 80/100/132 col terminal widths; instant transitions; compile-time status bar; keybind table from TOML | Hyprland (C, plugin-churn, animation-first); Sway (i3-grid ≠ column model); KWin/Mutter (drags full DE) |
| **Status bar** | Compile-time module of `niri-fork-developer` — not a separate IPC client process. Modules: workspace pager, focused-column title, network state, PipeWire volume, battery (when present), clock (UTC + local), Doorman tier indicator, capslock, tray slot | `waybar` (GTK3, JSON-IPC, C++); `yambar` (good but second process); GNOME Shell bar |
| **Application launcher** | `pointsav-launch` — Rust; prefix-search over `.desktop` files + `$PATH` + `pointsav-launch.d/*.toml` recipes; Doorman-aware app metadata | `rofi` (C, X11-lineage); `fuzzel` (fine, but in-house gives Doorman recipe integration); `wofi` (GTK) |
| **Terminal** | `kitty` — same choice as TUI Desktop; one terminal lineage across all shapes; image protocol for `nnn` preview | `foot` (smaller, but kitty's KGP enables nnn image preview and the TUI Desktop tool surface); `alacritty` (no search); `gnome-terminal` |
| **Notifications** | `mako` — single C binary, ~3 kLOC, freedesktop spec, TOML criticality rules | `dunst` (X11 lineage, larger surface); `swaync` (GTK4 + JS config) |
| **Clipboard** | `wl-clipboard` (primitive) + `cliphist` (history daemon, BoltDB, `~/.local/share/cliphist/db`); `Super+V` → `cliphist list \| pointsav-launch --dmenu`; cleared on screen lock | `copyq`, `GPaste` (GTK/Qt — drag desktop libraries) |
| **Screen locker** | `gtklock` — Wayland-native, `ext-session-lock-v1`, PAM-integrated, CSS-themeable; the one deliberate GTK dependency (authentication MUST be visually distinct from any app window) | `swaylock` (works; no branded identity badge); `waylock` (Zig, less mature) |
| **Portal backend** | `xdg-desktop-portal-gnome` (primary — niri implements the Mutter D-Bus shim; provides FileChooser, Settings, ScreenCast, Notification, Inhibit); `xdg-desktop-portal-wlr` (secondary fallback for screencast) | `xdg-desktop-portal-hyprland` (Hyprland IPC-specific); wlr-only (Tauri v2 FileChooser + Settings break without the GNOME backend) |
| **Secrets** | `gnome-keyring-daemon --components=secrets,pkcs11,ssh` (storage); `pointsav-secret-broker` (Rust capability gateway; per-app TOML ACLs; v0.2) | KeePassXC as primary (not headless-friendly for automated secrets); plain libsecret with no broker (grants all secrets to all D-Bus peers) |
| **File manager** | None as a default desktop application. `nnn` ships in the base image, invoked via `Super+E` inside the active terminal column | Nautilus, Dolphin, Thunar, PCManFM-Qt (each drags its DE stack) |
| **Polkit agent** | `polkit-rs-agent` — ~400-line Rust agent; shows requesting binary path + capability set + signature status before prompting; default policy: no admin escalation from graphical session (drop to TTY) | `polkit-gnome` (GNOME drag); `lxqt-policykit` (Qt drag) |
| **Display manager** | `greetd` + `tuigreet` — reuses the TUI Desktop greeter binary; TUI surface (no GPU, no GTK, no theme engine) at the authentication boundary | `gdm` (GNOME drag); `sddm` (Qt drag); regreet GTK variant |
| **Network** | NetworkManager daemon + `nmtui` (keyboard config) + status-bar module via D-Bus; `Super+I` → `nmtui` in terminal column; **no `nm-applet`** (tray indicator only, not click target) | `iwd` standalone (insufficient: captive portals, 802.1X, WireGuard dialers all require NM); `connman` |
| **Audio** | PipeWire + WirePlumber; volume via `wpctl` keybinds (`XF86AudioRaiseVolume` / `XF86AudioMute`); status-bar module | `pavucontrol` / `pavucontrol-qt` (GTK/Qt — `wpctl` is the one-liner); PulseAudio (legacy) |
| **XWayland** | `xwayland-satellite` — per-app isolated rootless XWayland; each X11 app gets its own instance; XWayland is default-present but opt-in per app via `pointsav-launch.d/*.toml` `xwayland = forbidden \| allowed \| required` | Global rootless XWayland (all X11 apps share one instance — any X11 app can keylog any other); XWayland entirely absent (JetBrains IDEs still need it in 2026) |
| **Bluetooth** | `bluez` daemon; `bluetoothctl` CLI; connection state in status bar; **no GUI applet** | `blueman-applet` (GTK); `bluedevil` (KDE) |
| **Browser** | None bundled. Tauri WebView is the only WebKit surface. Developers install `firefox-developer-edition` via `pointsav-pkg`. | Bundling Firefox/Chromium (massive telemetry surface; violates principle 2) |
| **Session supervisor** | `pointsav-session` — ~600-line Rust supervisor; launches compositor, runs `~/.config/autostart/*.desktop`, then exits to inherit nothing (replaces what `gnome-session` provides) | `gnome-session` (drags D-Bus session bus activation for GNOME services) |

### 13.5 The 22 invisible DE services — KEEP vs STRIP

When a developer leaves GNOME or KDE for a bare WM, 22 services silently vanish. Ten
of those cause developer abandonment within two weeks. The table below records the
Developer Desktop decision for each.

| Service | Decision | Mechanism |
|---|---|---|
| Clipboard manager | **KEEP** | `cliphist` + `wl-clipboard` |
| Secret service (libsecret) | **KEEP** | `gnome-keyring-daemon` + `pointsav-secret-broker` |
| Screen share / portal | **KEEP** | `xdg-desktop-portal-gnome` (Mutter shim on niri) |
| File-type associations | **KEEP** | `xdg-mime` + `pointsav-launch.d/*.toml` recipes |
| Status bar | **KEEP** | Compile-time compositor module |
| Polkit agent | **KEEP** | `polkit-rs-agent` |
| Screen locker | **KEEP** | `gtklock` + swayidle |
| Display greeter | **KEEP** | `greetd` + `tuigreet` |
| XDG autostart contract | **KEEP** | `pointsav-session` supervisor |
| Notification daemon | **KEEP** | `mako` |
| NetworkManager | **KEEP** | Required for captive portals and 802.1X |
| PipeWire + WirePlumber | **KEEP** | Required for Tauri WebView audio and screencast |
| SSH agent | **KEEP** | systemd `--user` `ssh-agent.service` + `SSH_AUTH_SOCK` via `environment.d/` |
| GNOME online accounts | **STRIP** | Never installed; no cloud account integration |
| GNOME Tracker / indexer | **STRIP** | Network syscalls, write-to-disk during dev builds — unacceptable |
| Evolution Data Server | **STRIP** | No email or calendar integration in default image |
| GNOME Software / PackageKit | **STRIP** | `pointsav-pkg` is the package surface |
| Avahi / mDNS | **CONDITIONAL** | Off by default; opt-in for local service discovery |
| GeoClue / Location service | **STRIP** | No location portal; sovereignty violation |
| fwupd / LVFS | **CONDITIONAL** | Opt-in for laptop firmware updates; off on server-class hardware |
| Bluetooth GUI daemon | **STRIP** | `bluetoothctl` CLI; status-bar connection indicator only |
| GNOME Keyring GUI | **STRIP** | No seahorse; `secret-tool` CLI + `pointsav-secret-broker` |

Network surface after fresh boot (target): DHCP, NTP, one NetworkManager connectivity
check (disabled via `connectivity.uri=` in NM config for sovereignty-maximal installs).
Zero default outbound otherwise.

### 13.6 Two-compositor architecture

The niri upstream (Smithay, ~30 kLOC Rust) is forked once into `pointsav-compositor`.
Two build profiles produce two binaries:

| Property | `niri-fork-graphical` (Graphical Desktop) | `niri-fork-developer` (Developer Desktop) |
|---|---|---|
| IPC socket (`$NIRI_SOCKET`) | **Off** — socket not created | **On** — JSON-RPC; scriptable from shell |
| XWayland | Absent (stripped at build time) | `xwayland-satellite` per-app bridge |
| `wlr-layer-shell` | Restricted — PointSav-signed clients only | Open — any layer-shell client |
| Status bar | External `app-workplace-bar` (Tauri v2) via layer-shell | Compile-time Rust module; no external process |
| Animations | Optional easing for consumer polish | Off — instant transitions only |
| Column widths | Unconstrained (browser, BIM viewport) | Snap-to 80 / 100 / 132 col for terminal alignment |
| `xdg-decoration` | Forced CSD for Tauri v2 visual continuity | Server-side preferred; GTK4 apps may override |
| Corner radius | 8 px | 6 px |
| Window gaps | 8 px | 6 px |

Both binaries are built from the same `Cargo.toml` workspace; the differences are
feature flags and a compile-time config file (`config/developer.toml` vs
`config/graphical.toml`). Upstream niri commits are rebased onto the fork monthly; the
fork divergence is kept minimal by design.

### 13.7 Keyboard model — three OS profiles

The Developer Desktop ships keybind tables as TOML files in `/etc/os-developer/`. The
compositor reads the active profile at startup; the user selects a profile at first boot
and can switch via `pointsav-launch` → "Keyboard profile." No keybind requires editing a
config file.

**Philosophy:** `Super` is the WM modifier. `Ctrl` (or remapped `Cmd`) is the within-app
modifier. They are on different physical keys and never compete. i3/sway conventions win
on `Super`; macOS/Windows conventions layer onto `Ctrl`/`Alt` where muscle memory
conflicts arise.

#### Linux default profile (`/etc/os-developer/keybinds.linux.toml`)

| Binding | Action |
|---|---|
| `Super+Return` | Open default terminal (kitty) |
| `Super+Shift+Return` | Open terminal at git root of focused column |
| `Super+Space` | `pointsav-launch` application launcher |
| `Super+Shift+Space` | `pointsav-launch --capability-mode` |
| `Super+Q` | Close focused window |
| `Super+Shift+Q` | Quit application (kill systemd user unit) |
| `Super+E` | `nnn` in terminal column |
| `Super+V` | Clipboard history |
| `Super+L` | Lock screen (`gtklock`) |
| `Super+N` | Notification history |
| `Super+Shift+N` | Dismiss all notifications |
| `Super+I` | `nmtui` network manager in terminal |
| `Super+D` | Doorman tier picker (A / B / C for focused app) |
| `Super+M` | Network audit pane (outbound flows per app) |
| `Super+H` / `Super+L` (nav) | Focus left / right column |
| `Super+J` / `Super+K` | Focus down / up in column |
| `Super+Shift+H/J/K/L` | Move focused window |
| `Super+1`…`Super+9` | Switch to workspace N |
| `Super+Shift+1`…`Super+Shift+9` | Move focused window to workspace N |
| `Ctrl+Left` / `Ctrl+Right` | Cycle workspaces (wired in all three profiles) |
| `Super+Tab` | Toggle to last-focused window |
| `Super+S` / `Super+Shift+S` | Stash / recall scratchpad |
| `Super+P` | Screenshot region (`grim` + `slurp` + portal) |
| `Super+Shift+P` | Screenshot full screen |
| `Super+F` | Toggle fullscreen |
| `Super+Shift+F` | Toggle floating |
| `Super+R` | Reload compositor config |
| `Super+Backspace` | Power menu |
| `Super+Shift+Backspace` | Logout |
| `F12` | `pointsav-tui-shell` scratchpad (shared with TUI Desktop) |
| `XF86Audio*` | Volume / mute via `wpctl` |
| `XF86MonBrightness*` | Brightness (laptop only) |

#### macOS profile (`/etc/os-developer/keybinds.macos.toml`)

Differences from the Linux profile only:

| Binding | Action |
|---|---|
| `Super+Q` | Quit application (Cmd-Q — kill, not close) |
| `Super+W` | Close window (Cmd-W) |
| `Super+Tab` | Per-app switcher (last-focused per app) |
| `` Super+` `` | Cycle windows within current app |
| `Super+Shift+3` | Screenshot full screen |
| `Super+Shift+4` | Screenshot region |
| `Super+,` | Application preferences (delegates to focused app) |
| `Ctrl+Super+Q` | Lock screen (Ctrl-Cmd-Q macOS convention) |

#### Windows profile (`/etc/os-developer/keybinds.windows.toml`)

| Binding | Action |
|---|---|
| `Super` tap (no chord) | Open `pointsav-launch` (Start key muscle memory) |
| `Super+E` | `nnn` file browser |
| `Super+L` | Lock screen |
| `Super+Shift+D` | Doorman tier picker (Super+D = Show Desktop in Windows profile) |
| `Super+Left` / `Super+Right` | Snap window left / right |
| `Alt+Tab` / `Alt+Shift+Tab` | Window switcher |
| `Alt+F4` | Close window |
| `Ctrl+Alt+Delete` | Power menu |
| `PrintScreen` | Screenshot full screen |
| `Super+Shift+S` | Screenshot region |

### 13.8 Visual identity — "confidently restrained"

The Developer Desktop's aesthetic goal is the signal Linear, Vercel, and Raycast achieve:
few colors, deliberate spacing, considered typography, no decoration that does not carry
information. A developer's eye reads this as "made by serious people for serious work"
without being able to articulate why.

**Color palette** (custom, Nord-adjacent; not Catppuccin-as-default):

| Role | Hex | Notes |
|---|---|---|
| Background (chrome) | `#0F1419` | Deep blue-near-black; warmer than `#000`, cooler than Gruvbox |
| Surface (bar, launcher) | `#1A2026` | One step up |
| Border unfocused | `#2A3138` | ~30% visibility |
| Border focused | accent | See below |
| Text primary | `#E5E9F0` | Near-white, slightly cool |
| Text muted | `#7A8290` | Inactive bar modules, timestamps |
| Accent (precision) | `#5BBFA8` | Teal — professional sovereignty signal |
| Accent (distinctive) | `#D89A4E` | Warm amber — more memorable; brand decision |
| Error | `#C66060` | Desaturated |
| Warning | `#D4A85A` | Desaturated |
| Success | `#7AA88A` | Desaturated |

One accent ships as default; the other is one config line. Built-in theme alternatives
(one-command atomic switch covering bar + borders + launcher + notifications): Catppuccin
(all four variants), Nord, Gruvbox, Tokyo Night.

**Typography:**
- **UI font:** IBM Plex Sans — open-source, "engineered industrial product" signal.
  Bundled at `/usr/share/fonts/pointsav/IBMPlexSans-*.ttf`.
- **Monospace:** JetBrains Mono Nerd Font — OFL, Nerd-patched, ligatures on/off, ~60%
  of new dev setups 2024–2025. Bundled at `/usr/share/fonts/pointsav/JetBrainsMono-*.ttf`.
- Both fonts are in the image. No system-font fallback. The desktop looks identical on
  Arch, Ubuntu, and Fedora base systems.
- fontconfig: `hintnone`, `antialias=true`, `rgba=none` (grayscale only).
  `FREETYPE_PROPERTIES="cff:no-stem-darkening=0 autofitter:no-stem-darkening=0"` in
  `/etc/environment` — the single highest-leverage rendering improvement for macOS refugees
  (closes the perceived font-weight gap against macOS stem darkening).

**Window chrome:** 2px borders (accent focused; 30%-grey unfocused; 50% opacity inactive).
6px gaps outer = inner. 6px corner radius (consistent: bar, launcher, notifications,
windows). Subtle ambient shadow (~15% opacity) on floating windows only. No transparency,
no blur, no window-spawn animations. Workspace transitions: ≤100ms ease-out, opt-in only.

**Status bar:** full-width, top, 28px, solid fill. Left: workspace pager (numbers, no
glyphs). Center: focused-column title (faded). Right: Doorman tier, PPN state, network,
audio, battery, clock (`Mon 2026-05-25 14:32` UTC, ISO 8601). No PointSav logo in bar.

**First boot:** one kitty terminal open and tiled, bar fully configured, launcher hint
(`Super+Space`). No modal, no tour, no wizard. The defaults must be survivable indefinitely.
The one detail to get right: **typography rendering** — if the font is fuzzy or mismatched
between bar and terminal, the developer registers the desktop as broken in seconds, without
being able to say why. Bundled fonts + tuned fontconfig is the highest-leverage detail.

### 13.9 Portal and security stack

**Portal backend selection for niri:**

niri is Smithay-based, not wlroots. The correct backend is `xdg-desktop-portal-gnome`:
niri implements `org.gnome.Mutter.ScreenCast` and `org.gnome.Mutter.RemoteDesktop` D-Bus
interfaces as compatibility shims specifically so the GNOME portal backend works without
a GNOME session (documented in the niri wiki). Using wlr-only fails because
`xdg-desktop-portal-wlr` does not implement `FileChooser` or `Settings`, and Tauri v2
requires both: `FileChooser` for `<input type="file">` dialogs; `Settings` for
`prefers-color-scheme` in WebKit CSS.

Portal config (`~/.config/xdg-desktop-portal/niri-portals.conf`):

```
[preferred]
default=gnome
org.freedesktop.impl.portal.ScreenCast=gnome;wlr
org.freedesktop.impl.portal.Screenshot=gnome;wlr
```

**Screen locking — `ext-session-lock-v1`:** while a session-lock client holds the lock,
the compositor refuses to render any other surface or route input elsewhere. If the locker
crashes, the compositor holds a solid color — no X11-style "kill the locker to bypass"
attack is possible. This is a material security improvement over every X11-based locker.

```sh
swayidle -w \
  timeout 300 'gtklock -d' \
  timeout 600 'niri msg action power-off-monitors' \
  resume 'niri msg action power-on-monitors' \
  before-sleep 'gtklock -d'
```

**XWayland isolation:** `xwayland-satellite` (Rust, 2024) gives each X11 app its own
rootless XWayland instance, eliminating the cross-app keylogging property of shared
rootless XWayland. The opt-in model in `pointsav-launch.d/*.toml`:

```toml
# pointsav-launch.d/wireshark.toml
name = "Wireshark"
exec = "xwayland-satellite -- wireshark"
xwayland = "required"
caps = ["net:raw", "fs:read:/"]
```

Apps with `xwayland = "forbidden"` (the default) never touch XWayland.

**SSH agent** as systemd user service:

```ini
# ~/.config/systemd/user/ssh-agent.service
[Service]
Type=simple
Environment=SSH_AUTH_SOCK=%t/ssh-agent.socket
ExecStart=/usr/bin/ssh-agent -D -a $SSH_AUTH_SOCK
```

`~/.config/environment.d/ssh-auth-sock.conf`:
```
SSH_AUTH_SOCK=${XDG_RUNTIME_DIR}/ssh-agent.socket
```

`AddKeysToAgent yes` in `~/.ssh/config`. On machines running `pointsav-secret-broker`
(v0.2+), the broker injects keys from the capability-scoped vault on demand.

### 13.10 macOS migration path — 16 must-haves

| # | Need | Mechanism | Notes |
|---|---|---|---|
| 1 | **Fractional scaling** | niri `wp_fractional_scale_v1` | Critical — niri: yes. Sway: no. **Do not ship sway.** |
| 2 | **Font rendering** | FreeType stem darkening + `hintnone` + bundled fonts | `FREETYPE_PROPERTIES` in `/etc/environment`; biggest perceived rendering improvement |
| 3 | **Instant launcher** | `Super+Space` → `pointsav-launch` | Calculator, file search, clipboard, app launch, SSH hosts |
| 4 | **macOS keybind profile** | `/etc/os-developer/keybinds.macos.toml` | `Super+Q` quit, `Super+W` close, `Ctrl+Left/Right` workspace cycle |
| 5 | **`pbcopy`/`pbpaste`/`open` aliases** | Shell rc: `alias pbcopy=wl-copy pbpaste=wl-paste open=xdg-open` | Pre-configured in default zsh rc |
| 6 | **Terminal with shell integration** | kitty + starship + zsh completions | Ghostty acceptable and preferred for Mac refugees; kitty ships by default |
| 7 | **Screen capture keybinds** | `Super+Shift+3/4` → `grim` + `slurp` → `wl-copy` | Matches macOS Cmd+Shift+3/4 muscle memory |
| 8 | **Zoom/Meet screen share** | `xdg-desktop-portal-gnome` + PipeWire screencast | GNOME backend on niri is the correct choice (not wlr-only) |
| 9 | **Bluetooth audio** (AAC/LDAC) | PipeWire ≥1.0 + WirePlumber + `libfreeaptx`/`libldac` | AirPods spatial audio: not available on Linux |
| 10 | **Secrets / keychain** | `gnome-keyring-daemon` + libsecret | 1Password, Bitwarden, browsers, IDEs all integrate via `org.freedesktop.secrets` |
| 11 | **Compositor gestures** | libinput: three-finger swipe → workspace; four-finger → overview | Configured in niri gesture bindings |
| 12 | **Sleep/resume reliability** | `swayidle before-sleep 'gtklock -d'` + modern s0ix | Hardware-dependent; document known-good hardware list |
| 13 | **`sudo` via fingerprint** | `fprintd` + `pam_fprintd` | x86 fingerprint readers only; not available on Apple Silicon Secure Enclave |
| 14 | **Night shift** | `gammastep` + geoclue (non-MLS provider) | MLS sunset 2024; use BeaconDB or manual lat/lon |
| 15 | **AirPrint** | CUPS + `avahi-daemon` + IPP Everywhere (opt-in) | Avahi off by default (sovereignty posture) |
| 16 | **Honest "no" list** | First-boot documents: Touch ID on AS, AirDrop with Apple devices, iMessage, Continuity Camera, Handoff | Documented absence is better than silent failure |

macOS-only developer tools with no Linux equivalent: Xcode (iOS dev requires Mac),
Sketch, Things 3, iMessage, Proxyman's iOS-specific features. For PointSav's target
cohort — backend, infrastructure, web — only Xcode is a hard blocker.

### 13.11 Leapfrog — per-app capability visibility

The Developer Desktop's single provable superiority over macOS within 20 minutes:

**Every running application's capability set is visible, live, and revokable from the
keyboard — without quitting the app.**

On macOS, finding what `tccd` allowed, what `nesessionmanager` is doing, or which app is
using the microphone right now requires a paid third-party tool (Little Snitch, ~$60).
On the Developer Desktop it is two keystrokes, and it is on the first-boot tour.

**The 20-minute demo:**

1. Boot → greeter → login. (~2 min)
2. `Super+Return` → terminal → `pointsav-launch app-workplace-memo`. (~30 sec)
3. `Super+Shift+Space` → capability-mode launcher. The Memo entry shows:
   `caps: [fs:~/Documents/memos, net:none, secret:memo-encryption-key]`.
   No network. No filesystem outside its directory. Enforced, not advisory. (~1 min)
4. Launch Wireshark (XWayland): capability-mode shows `caps: [net:raw, fs:read:/]`.
   Compare the two side by side. (~3 min)
5. `Super+D` on Memo → Doorman tier picker → "Tier denied." Use the "summarize" feature:
   returns "no Doorman capability granted." Provable. (~2 min)
6. `Super+M` → network audit pane: every outbound TCP/UDP flow, per app, with the
   capability that authorized it. The whole network surface on one screen. (~3 min)
7. Revoke `net:tls:443` for Memo's Doorman capability. The button greys out in real time.
   Re-grant it — it comes back. (~3 min)
8. Compare: macOS cannot show this view without a paid tool. Here it is two keystrokes
   away on the first-boot tour. (~5 min)

**The sentence a developer says at minute 20:**
*"macOS cannot tell me what each app is allowed to do, and I cannot turn it off without
quitting the app. This can. I am keeping this."*

Sovereignty made felt, not abstract. The capability visibility derives from principle 3
(one process per app, capability-scoped) and the seL4-mandated CNode-per-app architecture
in `system-security`. The Developer Desktop is the surface that makes that architecture
legible to the human operating it.

### 13.12 Ship-path addendum

The Developer Desktop's milestones extend §11. The §11 TUI Desktop milestones are
unchanged; this section adds parallel tracks.

**Parallel with TUI Desktop:** kitty, `pointsav-tui-shell` (F12 scratchpad), `greetd` +
`tuigreet`, PipeWire + WirePlumber, `pointsav-pkg`, `polkit-rs-agent`, `mako`. Ship once,
serve both shapes.

**Parallel with Graphical Desktop:** `niri-fork-developer` and `niri-fork-graphical` are
two build profiles of the same `pointsav-compositor` repository. They ship as one effort
with two binaries.

**Sequential after TUI Desktop v0.1:** `pointsav-secret-broker`, Doorman tier picker,
capability-mode launcher, and network audit pane depend on the seL4 CNode capability
ledger in `system-security`. TUI Desktop v0.1 lands that primitive; Developer Desktop v0.1
is the graphical surface that follows.

#### v0.1 — minimum viable workstation

- `niri-fork-developer`: compile-time status bar, Linux-profile keybinds, 6px gaps,
  6px radius, custom PointSav palette
- `pointsav-launch` (text-only mode; capability-mode in v0.2)
- `mako`, `cliphist`, `wl-clipboard`, `gtklock`, `swayidle`
- `xdg-desktop-portal-gnome` + `xdg-desktop-portal-wlr`; `niri-portals.conf` wired
- `gnome-keyring-daemon` (secret-broker deferred to v0.2)
- NetworkManager + `nmtui`, PipeWire + WirePlumber, `bluez` (CLI)
- `polkit-rs-agent`, `greetd` + `tuigreet`, `pointsav-session`
- kitty, `nnn`, `xwayland-satellite`, zsh + starship, `wl-clipboard`, shell aliases
- Tauri v2 workplace suite + `pointsav-pkg`
- IBM Plex Sans + JetBrains Mono Nerd Font bundled; fontconfig tuned

**Test:** a single developer uses this as their daily-driver to build
`cargo build --workspace` against `pointsav-monorepo` and ship a signed release to
`software.pointsav.com` without touching another OS for 30 consecutive days.

#### v0.2 — the leapfrog

- `pointsav-secret-broker` with per-app TOML ACLs
- `xdg-desktop-portal-pointsav` (Settings + Secret interfaces)
- Capability-mode launcher (`Super+Shift+Space`)
- Doorman tier picker (`Super+D`)
- Network audit pane (`Super+M`)
- macOS and Windows keybind profiles
- 20-minute first-boot tour (guided; no modal; exits cleanly)

Once v0.2 ships, the §13.11 demo is runnable and the Developer Desktop becomes the
canonical PointSav engineering workstation. Principle 5 (dogfood what you ship) begins.

#### v0.3 — packaging and chain of custody

- `os-developer.img` enters `software.pointsav.com` release manifest ($1 Apache 2.0 /
  $19 FSL — same pricing as TUI Desktop, §11).
- **Build-host attestation:** `os-developer.img` is built on a machine running
  `os-developer.img`. The signed binary embeds a reference to the prior signed binary
  that built it. Chain of custody from version N to N−1 is a public Merkle log anchored
  in the same Sigstore Rekor instance used by the design substrate (DOCTRINE claims #33
  and #38).

### 13.13 What does not change

The Developer Desktop is additive. Nothing in §1–§12 is modified.

- **L1/L2/L3 architecture (§3):** the three-layer model applies to the TUI Desktop.
  The Developer Desktop's Tauri v2 apps are a parallel track, not a replacement.
- **BFS namespace and `DESK:*` attribute semantics (§4, §5):** unchanged. The
  `desk-metadata` SQLite crate (§12.6) is shared between the TUI Desktop DESK pane and
  any Developer Desktop file-association integration.
- **KGP and `ratatui-pixels` capability chain (§6, §12.3):** the Developer Desktop hosts
  `pointsav-tui-shell` in a kitty F12 scratchpad; all KGP rendering decisions apply
  unchanged inside that terminal context.
- **Sovereignty verdict (§9):** strengthened. The compile-in status bar, per-app
  XWayland isolation, network audit pane, and `ext-session-lock-v1` screen locker each
  close attack surface that generic WM setups leave open.
- **§11 TUI Desktop ship path:** unmodified. Developer Desktop is parallel, not a gate.
- **`software.pointsav.com` distribution model:** pricing, licensing, and Ed25519 signing
  pipeline are unchanged. The Developer Desktop adds a third signed image to the same
  distribution infrastructure.

---

*§13 authors: WP-DEV-01..10 research agents | 2026-05-25*

---

## 14. Clipboard Parity and Terminal Integration

The first thirteen sections specify what runs on the developer desktop, how it is composed,
and how it migrates from macOS. They do not yet answer the highest-friction question for
everyday work: when a user copies in one window and pastes in another, what happens?
PointSav ships three deployment shapes — `os-workplace`, `os-developer`, and `os-tui` —
and each carries a different mix of native Wayland apps, ratatui TUIs, XWayland
refugees, and remote SSH sessions. Without a unified clipboard and terminal-integration
layer, every shape leaks the underlying Linux substrate at the precise moment users are
most attentive: the paste.

The operator thesis is load-bearing: with AI in the loop, coders code less and reach for
GUI surfaces more, while office workers code more and reach for TUI surfaces more. Both
populations converge on the same requirement — clipboard parity with macOS, and a
terminal that behaves like a first-class OS citizen rather than an embedded curiosity.

### 14.1 The integration gap — six failure modes

| # | Failure mode | Shape | Today | Root cause |
|---|---|---|---|---|
| F1 | Office worker copies a row from the ratatui bookkeeper into Gmail | `os-workplace` | Paste yields ANSI escape sequences (`\e[38;5;…m`) | TUI writes styled pty text; Kitty's L1 selection copies SGR verbatim |
| F2 | Developer closes Rust error window after copying the stack trace | `os-developer` | Subsequent paste yields empty or stale content | Wayland clipboard dies with the source surface |
| F3 | macOS refugee fires Ctrl+C muscle memory in Kitty | all three | Running process receives SIGINT and dies | Ctrl+C is both "copy" (GUI convention) and "interrupt" (terminal convention) |
| F4 | X11 app (Wireshark, legacy IDE) copies → paste into Kitty | `os-developer` | Paste yields nothing or stale content | xwayland-satellite's per-app XWayland has its own X CLIPBOARD atom; no bridge to Wayland `wl-data-device` |
| F5 | Over SSH to Yo-Yo node, TUI app issues OSC 52 copy | `os-developer`, `os-tui` | Clipboard silently unchanged | OSC 52 disabled by default in stock terminals; plain `ssh` does not forward reliably |
| F6 | Operator presses F12 quake from inside a project directory | all three | Scratchpad appears in `$HOME`, not the project | F12 has no awareness of the focused window's CWD |

§14.2 through §14.8 close all six in order.

### 14.2 pointsav-clipboard-daemon

`pointsav-clipboard-daemon` is a single Rust binary, started by `pointsav-session` at
login before any client surface appears, and remains the canonical clipboard owner for
the lifetime of the session.

| Property | Value |
|---|---|
| Crate | `pointsav-clipboard-daemon` — `pointsav-monorepo/tools/clipboard-daemon/` |
| Protocol | `ext-data-control-v1` (preferred) → `wlr-data-control-unstable-v1` fallback |
| Wayland crates | `wayland-client 0.31`, `wayland-protocols-wlr` |
| IPC surface | D-Bus session bus — `com.pointsav.Clipboard` at `/com/pointsav/Clipboard/1` |
| Storage | In-memory ring buffer (256 entries, 16 MiB cap per entry) + `sled`-backed WAL at `$XDG_STATE_HOME/pointsav/clipboard/wal` |
| Estimated size | ~4,800 LOC Rust |

**Persistence mechanism.** Wayland's core clipboard requires a live source client to
serve data on paste. When a source surface closes, the data offer is destroyed and the
clipboard goes silent. The daemon solves this by binding `ext-data-control-v1`
focus-free, eagerly draining all advertised MIME types into its ring buffer on every
`selection` event, and re-creating the selection with itself as the source as soon as
the original source disconnects. From the paste consumer's perspective the source never
died. This is the mechanism `wl-copy --paste-once` and `cliphist` implement manually;
the daemon does it automatically for every clipboard change. F2 closes.

**Protocol preference order** (verified from `wl-clipboard`'s
`src/types/registry.c`):

```
ext_data_control_manager_v1      ← preferred (merged into wayland-protocols 2024)
zwlr_data_control_manager_v1     ← wlroots extension; all relevant compositors
wl_data_device_manager           ← core Wayland; focus-gated; daemon-incompatible fallback
```

**D-Bus interface:**

| Member | Type | Purpose |
|---|---|---|
| `History()` | method → `a(usstt)` | Ring buffer as `(id, preview, mime_primary, timestamp, flags)` |
| `Select(id)` | method | Re-offers ring buffer entry as current selection |
| `Delete(id)` | method | Removes from ring + WAL |
| `Sensitive` | signal | Emitted when fast-path PII detection flags an entry |
| `Selection` | signal | Emitted on every clipboard change |

`wl-copy`, `wl-paste`, and `cliphist` continue to work unchanged — they bind
`wlr-data-control` as ordinary clients. The daemon is a peer, not a replacement.

**Security.** `ext-data-control-v1` grants any client focus-free clipboard access.
The daemon patches `niri-fork-developer` to add a compositor allow-list: only
`pointsav-clipboard-daemon`, `pointsav-launch`, and signed PointSav binaries may bind
the global. All others fall back to the focus-gated core protocol. The allow-list is
~150 LOC against Smithay's `SelectionHandler` allow-callback.

Sensitive-content handling via `CLIPBOARD_STATE=sensitive` (the `x-kde-passwordManagerHint`
convention adopted by KeePassXC, Bitwarden, KWallet) and `pointsav-secret-broker`:
entries so flagged are held in memory for the immediate paste and never persisted to the
WAL. Paste outside the source-app family triggers a 30-second `gtklock`-style
confirmation modal via `polkit-rs-agent`.

**`cliphist` shim.** The image ships a `cliphist` binary that proxies `list`, `store`,
`decode` commands to the daemon over D-Bus. Existing picker scripts (fuzzel, rofi, wofi,
fzf) work without modification.

### 14.3 Keyboard unification — three-profile Ctrl+C model

| Profile | Copy | Paste | Interrupt | Who it targets |
|---|---|---|---|---|
| `linux-native` | `Ctrl+Shift+C` | `Ctrl+Shift+V` | `Ctrl+C` | Linux-native muscle memory |
| `unified-super` (default) | `Super+C` | `Super+V` | `Ctrl+C` | Clean split; Super becomes the safe OS-verb modifier |
| `macos-refugee` | `Super+C` | `Super+V` | `Ctrl+C` + one-time educational toast | macOS Cmd+C → Super+C remap |

The `unified-super` profile is the default for all three shapes. It resolves F3 by
removing the collision entirely: `Ctrl+C` is unambiguously SIGINT; `Super+C` copies.
The niri keybind-profile system (§13.7) already coordinates which `Super+*` chords are
available to the terminal; `Super+C` and `Super+V` are vacated from WM use specifically
to support this profile.

`/etc/pointsav/kitty/profiles/unified-super.conf`:

```
map super+c        copy_to_clipboard
map super+v        paste_from_clipboard
map ctrl+c         copy_or_interrupt
map ctrl+shift+c   copy_to_clipboard
map ctrl+shift+v   paste_from_clipboard
map shift+insert   paste_from_selection
```

`copy_or_interrupt` is a Kitty built-in: copies if a Kitty-layer text selection exists;
otherwise sends SIGINT. Combined with Kitty Keyboard Protocol (KKP) in ratatui apps
(`crossterm::event::PushKeyboardEnhancementFlags(DISAMBIGUATE_ESCAPE_CODES)`), the
factoring is clean: Kitty handles the terminal-layer selection; ratatui apps handle
their own semantic copy via OSC 52 / `wl-clipboard-rs` (§14.4).

Paste safety (`/etc/pointsav/kitty/conf.d/05-paste-safety.conf`):

```
paste_actions quote-urls-at-prompt,replace-dangerous-control-codes,confirm-if-large
```

`Shift+Insert` → `paste_from_selection` is preserved on all three profiles for
accessibility (no chord, no modifier math).

### 14.4 ratatui clipboard API standard — OSC 52 + wl-clipboard-rs + bracketed paste

Every PointSav ratatui app uses the same clipboard pattern via
`pointsav-ratatui-clipboard` (`pointsav-monorepo/crates/ratatui-clipboard/`).

**Two clipboard layers — both must work:**

| Layer | Trigger | What is copied | Mechanism |
|---|---|---|---|
| L1 — Terminal selection | Shift+drag or triple-click in Kitty | Raw screen characters — useful for ad-hoc grabs | Kitty intercepts; no app involvement. App must NOT capture Shift+modified mouse events. |
| L2 — Application copy | App's own keybinding (e.g. `Ctrl+Y`, `y`, `Enter` on selection) | Semantic payload — clean CSV, JSON, plain text | App constructs payload; emits via OSC 52 or `wl-clipboard-rs`. |

**Copy path (L2):**

```rust
// pointsav-ratatui-clipboard::copy_multi
pub fn copy_multi(plain: &str, csv: Option<&str>, html: Option<&str>) -> io::Result<()> {
    if is_local_wayland() {
        // wl-clipboard-rs::copy::copy_multi — multi-MIME, one wl_data_source
        let sources = build_mime_sources(plain, csv, html);
        wl_clipboard_rs::copy::copy_multi(Options::new(), sources)?;
    } else if is_kitty() {
        // kitty +kitten clipboard --mime text/plain --mime text/csv (OSC 5522)
        spawn_kitten_clipboard_multi(plain, csv, html)?;
    } else {
        // OSC 52 — plain text only, universal fallback
        emit_osc52(plain)?;
    }
    Ok(())
}
```

`wl-clipboard-rs::copy::copy_multi` (crate `wl-clipboard-rs 0.9`) is the canonical
multi-MIME path. A single `wl_data_source` advertises all formats simultaneously; the
paste consumer (Gmail, Excel, another TUI) picks the format it understands. `wl-copy`
cannot do this (single-format per invocation); `arboard` cannot do this (no multi-MIME
API). The crate was authored for exactly this use case — "terminal applications that
don't spawn Wayland surfaces."

**Paste path:** `crossterm::event::EnableBracketedPaste` at startup; handle
`Event::Paste(String)`. Multi-line paste arrives as one event with embedded newlines
preserved. F1 closes: the bookkeeper emits `text/csv` and `text/html` alongside
`text/plain`; Gmail picks `text/html` and renders a styled table row.

**Over SSH:** `kitten ssh` (aliased to `ssh` in the PointSav shell-integration layer)
forwards OSC 52 and OSC 5522 sequences from the remote Kitty session to the local
terminal. F5 closes. tmux workaround: `set-option -g set-clipboard on` (tmux ≥ 3.3).

**Standard MIME bundle for tabular data** (documented in
`conventions/clipboard-mime-bundle.md`):

```
text/plain             → TSV row (office-tool fallback)
text/csv               → RFC 4180 CSV
text/html              → <table><tr>…</tr></table> with minimal inline CSS
application/x-pointsav-row+json → round-trip back into a PointSav app
```

All four offered on every table-row copy. Consumers pick the best fit. The
`#[derive(PointsavRow)]` macro auto-generates all four serialisers from the app's row
struct.

### 14.5 X11 clipboard bridge — per-xwayland-satellite instance

`os-developer` runs one `xwayland-satellite` per X11 app (§13.3). Each instance has its
own X CLIPBOARD and PRIMARY atoms, isolated from the Wayland seat. Without a bridge,
F4 is permanent.

**`pointsav-x11-clip-bridge`** (`pointsav-monorepo/tools/x11-clip-bridge/`, ~500 LOC
Rust) is launched by the satellite manager for every XWayland instance it spawns:

| Phase | Action |
|---|---|
| Startup | Connect to per-instance X display via `x11rb`; subscribe to `XFixes` selection-owner-change events on CLIPBOARD + PRIMARY |
| X → Wayland | On X selection change, collect TARGETS; forward all MIME types to `pointsav-clipboard-daemon` via D-Bus `OfferFromX11(targets, owner_pid)` |
| Wayland → X | Subscribe to daemon `Selection` signal; when Wayland selection changes, claim X CLIPBOARD ownership in this instance; serve X `SelectionRequest` events from daemon ring buffer |
| Teardown | Exit when parent satellite exits |

The daemon is the single source of truth for all selection state. Multiple X instances
in flight simultaneously all federate on the same daemon D-Bus interface. Cross-instance
paste (Wireshark → Kitty → another X11 app) routes through:
`XWayland-A → bridge-A → daemon → bridge-B → XWayland-B`.

PRIMARY selection bridged identically via `com.pointsav.Clipboard.Primary` D-Bus
members. F4 closes.

**`xwl-launcher`** (a ~500 LOC Rust wrapper): wraps every X11 `Exec=` line in
`pointsav-launch.d/*.toml` to assign a fresh display number, spawn the satellite, spawn
the bridge, and run the app under `systemd-run --user --scope` with `DynamicUser=yes`
for cgroup audit. Display numbers allocated from `:10–:127` with file-locks in
`/run/user/$UID/xwl-satellite/`.

### 14.6 F12 quake terminal — CWD inheritance and latency target

F12 is the keystone affordance: one keystroke from any context to an interactive shell,
in the right directory.

**Architecture:**

```
F12 keypress
  → niri-fork-developer captures binding
  → pointsav-session queries niri IPC for focused-window PID
  → resolves /proc/<pid>/cwd → absolute path (Strategy A: Kitty OSC 7 via
      kitty @ ls; Strategy B: pstree deepest descendant; Strategy C: /proc/<pid>/cwd)
  → quake Kitty instance already running:
        kitty @ --to unix:@kitty-quake send-text --match "var:quake=1"
                " cd '<dir>'\n"
        niri IPC: move-window-to-workspace quake-instance → active-workspace
  → quake not yet running:
        spawn kitty --listen-on unix:@kitty-quake --class quake-term
              --directory '<dir>'
        niri window-rule: open-floating, top-left, 100% × 50%
```

| Metric | Target | Notes |
|---|---|---|
| F12 → terminal visible (warm) | < 100 ms | Toggle = one niri IPC call + one Wayland frame |
| F12 → terminal visible (cold) | < 400 ms | One Kitty spawn including font cache |
| `cd` injection complete | < 50 ms after visible | OSC 133 prompt-mark fires after `cd` |
| F12 again → hidden | < 50 ms | Reverse IPC call |

At 60 Hz, warm toggle is ~32 ms (1 IPC call + 1 vsync). The niri-fork-developer instant-
transition profile (§13.6) eliminates animation budget. F6 closes.

**Focus-tracker daemon** (`pointsav-session-helper --focus-track`): subscribes to niri's
event-stream IPC, records the last non-quake focused window PID in
`/run/user/$UID/quake-prev-focus.pid`. The CWD resolver reads this file synchronously
at F12 time, with a 30 ms fallback to `$HOME` if the file is absent or the path is
inaccessible.

**niri scratchpad status.** Native scratchpad (PR #2807) is unmerged upstream. Current
implementation uses `move-window-to-workspace scratch` via the community
`niri-scratchpad-rs` shim. When upstream merges, the config swaps from
`move-window-to-workspace scratch` to `move-window-to-scratchpad` — one line change,
no logic change.

### 14.7 Terminal-as-first-class integration points

The "terminal is a first-class OS citizen" principle has a concrete definition:
**every app that produces text — file paths, URLs, error messages, hashes — must be
able to hand that text to the terminal in one keystroke or one click, without a
copy-paste step.** The mechanisms below implement it.

| Affordance | Mechanism | Ship |
|---|---|---|
| `xdg-open` equivalent | `pointsav-open` (Rust, MIME-aware): text/code → Kitty + Helix or bat; image → swayimg; PDF → zathura; office formats → LibreOffice on `os-workplace`. Registered as system MIME handler at image-build time. | v0.1 |
| `open` / `pbcopy` / `pbpaste` shell aliases | `/etc/skel/.zshrc` ships `alias open=xdg-open pbcopy='wl-copy' pbpaste='wl-paste'`. macOS refugees use them without reconfiguration. | v0.1 |
| Clickable error lines | Shell integration (`/etc/skel/.zshrc` sources Kitty's bundled zsh integration: OSC 7 + OSC 133). Compiler errors in `rustc`, `cargo`, `ripgrep` are wrapped as OSC 8 hyperlinks; Kitty routes them via `hyperlink_alias` to `pointsav-open://edit?path=…&line=…`, which opens Helix at the exact line. | v0.1 |
| nnn → Kitty path drop | Wayland drag-and-drop via `wl-data-device`; nnn plugin emits `text/uri-list`; Kitty accepts the drop and emits the path at the shell cursor. | v0.1 |
| Notifications from terminal | `notify-send` → mako. `pointsav-notify` wrapper adds `app_id=com.pointsav.<crate>` for per-app mako rules. | Inherited §13 |
| `terminal://` URL scheme | `pointsav-open` registers as `terminal://` handler. A Tauri app wraps every path in `<a href="terminal://<path>">` — click spawns a Kitty window at that directory via `kitty @ launch --type=os-window --cwd=<path>`. | v0.1 |
| Quick preview (`pointsav-preview`) | `Super+Y` on selected path/URL → borderless niri layer-shell surface; content-type dispatch: text/code via tree-sitter, image inline (Kitty graphics protocol), markdown rendered. macOS Quick Look equivalent. | v0.2 |
| Drag-from-Kitty-selection | Kitty `mouse_map` binding: `shift+left press grabbed` initiates `wl_data_device` drag with `text/uri-list` when selection matches a filesystem path regex. | v0.2 |
| Find register | `pointsav-clipboard-daemon` virtual pasteboard `/com/pointsav/Clipboard/Named/find`. Neovim and Helix plugins write to it on `/` search; all apps that bind the scheme read from it on "find next." `Super+E` = "use selection for find" (macOS Cmd+E). | v0.2 |
| Shell integration marks in kitty | `Super+[` / `Super+]` → `scroll_to_prompt -1` / `+1`; `Super+Shift+A` → select last command output. Configured in `/etc/pointsav/kitty/conf.d/20-shell-integration.conf`. | v0.1 |

`pointsav-open` is the integration keystone. By routing `file://`, `https://`,
`terminal://`, and `pointsav://` through a single MIME-aware dispatcher, every app in
the session can hand any text-like object to any other app with a single keybind. The
gap between "developer tool produces output" and "I can act on it in my preferred
surface" collapses to zero clicks.

### 14.8 AI-aware clipboard layer (Doorman integration, v0.2)

The clipboard is the highest-value and highest-risk AI integration point. The design is
opt-in, local-first, and never takes automatic action.

**Tier dispatch:**

| Tier | Model | Latency | Default state | Purpose |
|---|---|---|---|---|
| A | Local OLMo-1B via `service-slm` | < 100 ms | OFF (opt-in) | Classification only: `code / error / number / table / sensitive / prose / other` |
| B | Yo-Yo node | < 2 s | OFF | On-demand transformation: "fix this error", "format as CSV" |
| C | External API via Doorman audit-routing | < 5 s | HARD OFF for clipboard | Only unlocked per-call by operator |

**Sensitive-content fast path (always ON, no AI model involved):**
Regex-and-heuristic detection for IBAN, SIN, SSN, credit-card Luhn, RSA/Ed25519 PEM
headers, AWS/GCP credential patterns. Runs on every copy event. Flagged entries are ring-
buffered but trigger the `polkit-rs-agent` confirmation modal before re-offering to any
other app. False positives: `Super+Shift+V` bypasses confirmation once.

**Classification (Tier A, opt-in):** when enabled, every copy event is dispatched to
`service-slm` via `POST /v1/audit_proxy` with `module_id=clipboard`. Classification
attaches to the ring-buffer entry as the `flags` field in `History()`. The clipboard
ring-buffer picker (`pointsav-launch --mode clipboard`, bound to `Super+V`) uses flags
to colour-code entries and surface contextual "Paste with..." actions: a `code` entry
offers "paste as quoted block" and "explain"; an `error` entry offers "find similar in
history" and "send to Doorman."

**Peripheral observability:** the niri-fork-developer status bar (§13.8) shows a
two-state clipboard-AI indicator (steady dot = classification on; 200 ms flash = AI
read event; red flash = sensitive content blocked). The indicator is 12 px wide and
placed adjacent to the Doorman tier badge.

**Privacy invariants (non-negotiable):**

- No AI model reads clipboard content without an explicit user action in the same turn.
- Tier A classification is fire-and-forget and read-only: the model receives the content
  and returns a label; it does not transform, store, or forward.
- Tier B/C transformations are discrete user-initiated events. Result staged in the
  daemon's `_transformed` ring slot; never auto-pasted.
- Every Tier B/C call emits a `foundry-audit-ledger-v1` entry at
  `$XDG_STATE_HOME/pointsav/audit-ledger/clipboard/<YYYY-MM>.jsonl`. SYS-ADR-07 and
  SYS-ADR-19 apply.
- First-boot tour presents a single explicit opt-in screen: "Allow the AI side panel to
  see what you copy?" Default: **Off.** Toggle available at any time via `Super+D` →
  clipboard scope.

### 14.9 What does not change

| Element | Status |
|---|---|
| Two-compositor architecture (niri-fork-graphical / niri-fork-developer) | Unchanged (§13.6) |
| Three-profile keybind system (linux / macos / windows) | Extended in §14.3; WM bindings unchanged |
| Kitty as L0 terminal across all three shapes | Unchanged |
| Tauri v2 as cross-shape GUI runtime | Unchanged (§13.5) |
| WezTerm packaging on Windows/macOS (os-workplace extension) | Unchanged (§12.2); OSC 52 write works; OSC 5522 and multi-MIME are Kitty/Wayland-only |
| KGP graphics compatibility matrix | Unchanged (§12.4) |
| SQLite BFS attribute layer | Unchanged (§12.6) — clipboard history lives at `$XDG_STATE_HOME/pointsav/clipboard/`, not a BFS namespace |
| Portal stack (xdg-desktop-portal-gnome on niri) | Unchanged (§13.9) — clipboard daemon does not interact with portals |
| Sovereignty verdict | Unchanged — service-slm classification is local OLMo; Tier B/C transformations are explicit user actions over Doorman with full audit trail |
| §11 TUI Desktop ship path | Unmodified — §14 is parallel, not a gate |

The clipboard daemon and the X11 bridge are net-new binaries. The keyboard profile,
ratatui clipboard crate, F12 CWD inheritance, and shell-integration affordances extend
components already specified in earlier sections. No part of §14 alters the substrate-
sovereignty posture of §13 or the ledger-as-source-of-truth posture of §12.

The leapfrog argument: macOS achieves clipboard parity by virtue of being a single
vendor controlling every surface. PointSav achieves the same parity by virtue of owning
the daemon (`pointsav-clipboard-daemon`) that mediates every surface. The single-vendor
advantage Apple holds at the OS layer is the single-daemon advantage PointSav holds at
the session layer. The user-visible result is identical; the architectural cost is one
Rust binary (~4,800 LOC) and one D-Bus name.

---

*§14 authors: CP-01..10 research agents | 2026-05-25*

---

## 15. Conclusion — os-workplace as the 2030 Sovereign Desktop

### 15.1 What we set out to build

This BRIEF opened with a question: can a ratatui TUI shell, rendered inside Kitty,
operating on a Linux Wayland compositor, match or exceed the productivity of a macOS
workstation running SaaS applications? Fourteen sections later the answer is yes — not
as a speculative claim but as a concrete architecture with named crates, specific binary
sizes, verified protocol behaviours, and a milestone-gated ship path.

The problem had three parts that the industry has not solved together:

1. **Sovereignty without regression.** Self-hosted file stores, SHA-256 sealed ledgers,
   and offline-first operation are not new ideas. They routinely fail because the
   sovereignty trade-off degrades the experience: slower load times, missing integrations,
   clipboard that does not work with Gmail, terminals that crash on paste. The existing
   art (NixOS, Guix, QubesOS) chooses correctness over comfort.

2. **TUI without abandonment.** Terminal interfaces remain the most composable, most
   auditable, and lowest-latency interactive surface available. But they have been
   abandoned as a primary mode for office workers — not because TUI is wrong but because
   every TUI ecosystem assumed a technical operator. pointsav-tui-shell and the ratatui
   widget library exist to close that gap.

3. **AI without capitulation.** The competitive pressure to wire applications directly
   to external inference APIs is real. Doing so trades sovereignty for convenience. The
   Doorman service-routing architecture and the three-tier compute model (Local OLMo →
   Yo-Yo GCE → External API) keep AI as an accelerant without making it a dependency
   or a surveillance vector.

os-workplace is the answer to all three simultaneously.

### 15.2 The three deployment shapes

The architecture resolves into three concrete deployment shapes, each owning a distinct
operator type, each sharing the same substrate.

| Shape | Name | Primary surface | Operator | Compositor |
|---|---|---|---|---|
| **TUI Desktop** | `os-tui` | Kitty + pointsav-tui-shell | Server operators, SSH sessions, headless deployments | None (TTY or remote terminal) |
| **Developer Desktop** | `os-developer` | niri-fork-developer + Kitty | Developers, migration targets from macOS | niri-fork-developer |
| **Graphical Workplace** | `os-workplace` | niri-fork-graphical + Tauri v2 apps | Office workers (bookkeeping, property management, HR) | niri-fork-graphical |

The three shapes share: one terminal lineage (Kitty), one widget surface (ratatui), one
clipboard daemon (pointsav-clipboard-daemon), one BFS DESK:\* attribute namespace, one
Doorman audit ledger, and one distribution channel (software.pointsav.com). The shapes
diverge only at the compositor and launcher layers. A practitioner operating in `os-tui`
can inspect the same sealed ledger records as a bookkeeper using `os-workplace` — the
file format is identical, the SHA-256 chain is the same, the audit trail is continuous.

The term **os-workplace** serves double duty: it names the graphical shape and functions
as the umbrella brand for the platform as a whole when addressing non-technical audiences.
Internally, all three shapes are branches of the same Rust monorepo and ship from the
same signed release artefact.

### 15.3 "We Own It" — the author vs vendor distinction

The critical distinction between os-workplace and a configured GNOME desktop or a
hardened macOS installation is not the component list. It is authorship.

A GNOME installation uses components assembled by Red Hat. A hardened macOS uses
components owned by Apple. In both cases the operator is a **consumer** of a platform
they did not author. Every policy decision — what constitutes a clipboard, how X11 and
Wayland interact, what the compositor permits — is made upstream and inherited.

os-workplace is a platform its operators **authored**. Specifically:

- `niri-fork-graphical` and `niri-fork-developer` are forks with PointSav-specific
  features compiled in (status bar, scratchpad shim, OS keybind profiles, capability
  IPC extensions). Upstream niri is a dependency, not the product.
- `pointsav-clipboard-daemon` owns the clipboard contract across all sessions and
  processes. Apple achieves clipboard parity by controlling every surface at the OS
  layer; PointSav achieves the same parity by controlling the daemon that mediates
  every surface at the session layer. The user-visible result is identical; the
  architectural cost is one Rust binary (~4,800 LOC) and one D-Bus name.
- `pointsav-tui-shell` owns the TUI surface. Kitty is the renderer; the shell is
  PointSav's. The distinction matters when the shell gains capability-mode display,
  Doorman tier-picker, and AI-clipboard opt-in — none of those require upstream
  cooperation.
- `pointsav-secret-broker` owns the secrets ACL. `gnome-keyring-daemon` stores
  credentials; the broker enforces per-app TOML policies. The keyring is infrastructure;
  the policy is ours.
- `desk-metadata` owns the BFS DESK:\* attribute store. SQLite-backed, cross-platform,
  cloud-resilient. xattr is used as an advisory cache marker only; truth lives in the
  database.

Authorship means that when a future compliance requirement appears — a new jurisdiction's
data-residency rule, an auditor's chain-of-custody demand — the answer is a configuration
change or a small crate addition, not a vendor negotiation.

### 15.4 Totebox Orchestration on the desktop

The Foundry operational model divides work between a **Command Session** (workspace
governance, cross-archive promotion) and **Totebox Sessions** (feature code, deployment
provisioning). The same topology appears on the desktop without any deliberate design
decision — it is a natural consequence of the compositor architecture.

| Foundry session role | Desktop equivalent | Compositor / surface |
|---|---|---|
| Command Session | os-workplace graphical session | niri-fork-graphical |
| Totebox Session | F12 quake terminal | niri-fork-developer via scratchpad |
| Stage 6 promotion | `bin/commit-as-next.sh` + `bin/promote.sh` | Runs inside quake terminal |
| Cross-archive handoff | Outbox message → inbox sweep | File in `$HOME/.config/pointsav/` |

The F12 quake terminal is the Totebox entry point on the desktop. It inherits CWD from
the focused graphical window (OSC 7 → `kitty @ ls` → `/proc/<pid>/cwd` fallback chain).
It is warm before the keystroke completes (<100 ms at 60 Hz from niri scratchpad). It
renders the same ratatui widgets as a full `os-tui` session. When a developer closes
the quake, the graphical session resumes without interruption — the compositor has no
knowledge that a Totebox was active.

This means a PointSav operator can sit at an `os-workplace` graphical session running
the Tauri v2 bookkeeping application, press F12, commit a ledger schema migration in
the quake terminal, press F12 again, and return to the bookkeeping application — all
without leaving the graphical session, all without a second physical machine. The
Totebox and the Command surface coexist in one desktop compositor instance.

The implication for customer deployments: Jennifer Woodfine operates `os-workplace` as
her primary desktop. Peter Woodfine connects via `os-developer` or `os-tui` over the
WireGuard PPN. The canonical ledger they both write to is the same BFS file store,
sealed by the same SHA-256 chain, auditable by the same `sha256sum` invocation.

### 15.5 The eleven-layer sovereignty stack

Sovereignty is not a feature toggle. It is an architectural property that either holds
at every layer or fails at the weakest one. The complete layer model:

| Layer | Component | Sovereign element |
|---|---|---|
| **L0** | Kitty (terminal emulator) | KGP renders images without a browser engine |
| **L1** | pointsav-tui-shell | Session manager, F12 quake, CWD resolver |
| **L2** | ratatui widget surface | No JavaScript, no DOM, no external font fetch |
| **L3** | niri-fork (compositor) | Wayland, no X11 global clipboard, no D-Bus leakage |
| **L4** | pointsav-clipboard-daemon | Owns clipboard contract; AI fast-path opt-in only |
| **L5** | Tauri v2 (GUI runtime) | IPC over Wayland portals; no Electron, no Node.js runtime |
| **L6** | desk-metadata (SQLite) | BFS DESK:\* attributes on local storage; no cloud sync required |
| **L7** | pointsav-secret-broker | Per-app TOML ACL; gnome-keyring as vault only |
| **L8** | Doorman (AI router) | Local OLMo by default; Tier B/C requires user action + audit log |
| **L9** | BFS ledger (WORM append) | SHA-256 sealed entries; auditor verifies with `sha256sum` |
| **L10** | build-host attestation | os-developer.img built on a machine running itself; Sigstore Rekor anchor |

A macOS workstation running SaaS accounting software fails at L4 (iCloud clipboard sync),
L6 (data lives on vendor servers), L7 (keychain accessible to Apple-signed processes),
L8 (AI features phone home by default), L9 (no immutable local ledger), and L10 (binary
signed by vendor; build environment opaque). os-workplace holds all eleven layers.

Holding L10 requires the build-host attestation milestone (v0.3): the release image is
built on a machine running the same release image, and the chain-of-custody Merkle log
is anchored to Sigstore Rekor — the same transparency log used for DOCTRINE claims
#33 and #38. This makes the sovereignty claim externally verifiable, not merely asserted.

### 15.6 Five compounding mechanisms

The substrate compounds. Five mechanisms reinforce each other across the three shapes:

**1. One terminal lineage (Kitty → L0 everywhere)**

`os-tui` runs Kitty as the compositor replacement. `os-developer` runs Kitty as the
primary terminal. `os-workplace` runs Kitty as the F12 quake terminal. Every developer
who has ever typed in `os-tui` is at home in the quake terminal of `os-workplace`. Every
ratatui widget developed for `os-tui` renders identically in the quake. There is no
"terminal mode" and "GUI mode" — there is one surface with two window managers layered
on top.

**2. One widget surface (ratatui across all apps)**

The Tauri v2 graphical applications and the pointsav-tui-shell applications share the
same ratatui component library. A widget written for the bookkeeping TUI is available
in the Developer Desktop's quake terminal without modification. A chart component that
renders in Kitty via KGP renders in the Tauri v2 window via the WebView bridge. One
widget, three rendering contexts.

**3. One clipboard daemon (pointsav-clipboard-daemon as the federation bus)**

The daemon mediates every paste event in the system: Wayland native, X11 via
xwayland-satellite bridge, SSH remote via OSC 52 / OSC 5522, and Tauri v2 apps via
D-Bus. A cell copied from the bookkeeping TUI pastes into Gmail as formatted HTML, into
the quake terminal as TSV, and into Excel (over OSC 52) as a tab-separated table. The
user does not think about MIME types. The daemon thinks about MIME types so they don't
have to.

**4. BFS DESK:\* namespace as the knowledge graph**

Every file touched by any application on any shape carries BFS DESK:\* attributes in
the SQLite sidecar. `DESK:staged`, `DESK:flagged`, `DESK:opened_at` are facts about
files that persist across reboots, across shapes, and across machines if the SQLite
database is replicated (which it can be, because it is a plain file). The DESK namespace
is the closest analogue to a corporate knowledge graph that does not require a server.

**5. Doorman audit ledger as DPO corpus**

Every AI call that transits Doorman — Tier A (local OLMo), Tier B (Yo-Yo GCE), or
Tier C (external API) — appends a `foundry-audit-ledger-v1` record. Over time, this
accumulates the operator's actual work patterns: what prompts they issued, what the
model returned, what they accepted or rejected. This is a direct-preference-optimisation
corpus for continued pretraining of the local OLMo model. The compounding effect:
the longer the platform is in use, the better the local model fits the operator's
domain. Hyperscalers cannot offer this — they aggregate across millions of users.
PointSav offers the inverse: a model that grows more useful per individual operator
the more it is used.

### 15.7 Three users, one desktop

The architecture was designed for three concrete operators. Every subsystem maps to at
least one of them.

**Jennifer Woodfine — bookkeeper, os-workplace**

Jennifer uses the Tauri v2 bookkeeping application as her primary surface. The ledger
is local, SHA-256 sealed, and offline-capable. She has never opened a terminal. When
her accountant asks for a trial balance as of a specific date, she exports a plain YAML
file with a `sha256sum` on the cover sheet. The accountant verifies the hash with a
command Jennifer can show her on a sticky note. No login required, no SaaS subscription,
no "accountant access" tier.

Her clipboard works exactly as it does on macOS: Super+C copies, Super+V pastes, a
formatted cell pastes into Gmail as a table. The unified-super keyboard profile and
the pointsav-clipboard-daemon deliver this without her knowing either exists.

**Peter Woodfine — developer, os-developer**

Peter connects via WireGuard PPN from his workstation. He runs `os-developer` with the
`macos-refugee` keybind profile (Super+Q quit, Super+W close, Cmd+Space launcher, F12
quake). He presses Super+Shift+Space to see the capability set enforced on every running
application — a feature that requires Little Snitch on macOS. He presses Super+D to
reroute any AI call between Local, Yo-Yo, and External tiers without quitting the
application. He presses Super+M to see every outbound TCP/UDP flow per process. He
can verify that the bookkeeping application Jennifer uses has made zero network calls
in the last 30 days by reading a file, not by trusting a vendor's privacy policy.

**The AI-midwifed operator — HR manager, os-workplace**

A third operator type — the HR manager who inherits the platform without choosing it —
completes the picture. She does not know what a compositor is. She double-clicks a
`.dmg` on macOS, or an `.exe` on Windows, and PointSav installs. The onboarding wizard
takes three steps: what PointSav does in two sentences, import from Excel/CSV, and here
is your desktop. Her keybind collisions (Cmd+Space = Spotlight on macOS; Super+D =
Show Desktop on Windows) are resolved by the OS keybind profile selected at first boot.
Her AI calls are off by default; she opts in at first-boot and sees the peripheral
observability dot in the status bar whenever AI is active.

For her, the leapfrog claim is not "sovereignty" — that word does not land. The claim
is: "your personnel records never leave your laptop; every change is signed; your old
system charged a $60 monthly fee for the privilege of storing your data on their servers."
That lands. The architecture delivers it.

### 15.8 The trust demonstrations

A leapfrog advantage only lands if it can be demonstrated in under 20 minutes to a
non-technical operator. Four concrete demonstrations, one per trust layer:

**Demo 1 — Immutable ledger (Jennifer, 5 minutes)**

```
$ sha256sum ~/Documents/pointsav/ledger/accounts-payable-2026.yaml
a7f3c8... ~/Documents/pointsav/ledger/accounts-payable-2026.yaml
```

Hand the printed hash to the auditor. The auditor runs the same command on their own
laptop with a copy of the file. If the hashes match, the file has not been altered since
export. No SaaS login, no audit portal, no vendor API. The SHA-256 command is available
on every platform that runs os-workplace.

**Demo 2 — Network silence (Peter, 3 minutes)**

```
$ strace -f -e trace=network -p $(pgrep app-bookkeeping) 2>&1 | head -20
[no output — bookkeeping application made zero network syscalls]
```

Alternatively: `Super+M` opens the network audit pane. The bookkeeping application row
shows 0 outbound connections. This is a live view, not a log review. The demonstration
takes under 30 seconds.

**Demo 3 — Capability mode (Peter, 7 minutes)**

```
Super+Shift+Space → capability-mode launcher
```

Every running application shows its enforced capability set:
```
app-bookkeeping
  caps: [fs:~/Documents/pointsav/ledger, net:none, secret:ledger-key]
app-browser
  caps: [fs:~/Downloads, net:any, secret:none]
```

The HR manager's personnel application shows `net:none`. The browser shows `net:any`.
The distinction is visible, legible, and requires no technical knowledge to interpret.
Little Snitch on macOS provides a similar view but costs $60/year and is opt-in.
On os-workplace it is a first-class compositor feature available in a single keystroke.

**Demo 4 — Migration (all operators, 2 minutes)**

```
$ cp -r ~/Documents/pointsav /Volumes/USB-Drive/pointsav-backup
```

The entire operational record — ledger, BFS metadata, clipboard history, audit log — is
a directory. It copies with `cp -r`. It restores with `cp -r`. No export wizard, no
vendor migration tool, no "contact support to transfer your account". The backup is
tested by the copy completing without error.

### 15.9 Ship path — from substrate to signed release

The path from current state to a fully attested, three-shape release follows eight
milestones. Each milestone is a testable, observable state.

**v0.1 — Substrate complete (parallel tracks)**

All three shapes boot and are usable by their primary operator. The pointsav-tui-shell
and ratatui application suite (bookkeeping, property management, HR) are functional on
`os-tui`. niri-fork-developer with keybind profiles and the quake terminal is functional
on `os-developer`. niri-fork-graphical with Tauri v2 app suite is installable on
`os-workplace`. Three tracks run in parallel; no shape is a gate for another.

Shared substrate delivered at v0.1: pointsav-clipboard-daemon, desk-metadata SQLite
crate, pointsav-secret-broker v0.1, greetd/tuigreet greeter, PipeWire audio, polkit-rs-
agent, xdg-desktop-portal-gnome on niri.

**v0.2 — Leapfrog features complete**

Capability-mode launcher (`Super+Shift+Space`), Doorman tier picker (`Super+D`), network
audit pane (`Super+M`), macOS/Windows keybind profiles with first-boot selector,
xdg-desktop-portal-pointsav (capability-enforcing portal shim), pointsav-secret-broker
v0.2 (per-app TOML ACL), onboarding wizard (three-step first-boot), AI-clipboard opt-in
with peripheral observability dot.

At v0.2 the 20-minute leapfrog demonstrations in §15.8 are all executable.

**v0.3 — Packaging and attestation**

os-tui, os-developer, and os-workplace enter the software.pointsav.com release manifest.
macOS `.dmg` (code-signed, notarized, $99/yr Apple Developer account) and Windows `.exe`
(Authenticode OV cert, ~€300/yr) for os-workplace client. Linux binary + AppImage for
all three shapes. Build-host attestation: os-developer.img built on a machine running
itself; Sigstore Rekor anchor matches DOCTRINE claims #33/#38 chain-of-custody standard.

**v1.0 — Three shapes in one signed release**

Single version number covers all three shapes. Release notes describe the delta across
all three. Ed25519 license key (existing software.pointsav.com substrate) gates each
shape independently: `os-tui` free; `os-developer` $1 (Apache 2.0); `os-workplace`
$19 (FSL). Polygon USDC payment as already ratified.

**v2 — Direct-KMS Kitty**

Kitty renders directly to KMS/DRM without a compositor on `os-tui` headless deployments.
This eliminates one software layer (the compositor) from the headless path and reduces
the cold-boot-to-first-prompt time. Direct-KMS is already supported in Kitty upstream;
the work is configuration and testing, not porting.

**v3 — seL4 native exploration**

A research milestone, not a product commitment. The seL4 microkernel provides formal
verification of isolation properties. If the Doorman audit model and the capability-
enforcement model mature to the point where formal verification adds meaningful external
credibility (regulatory audit, national-security certification), the `os-workplace`
security model can be re-implemented on seL4 as a separate track without disturbing the
Linux mainline. This is a planned/intended direction only; no code exists.

### 15.10 The 2030 competitive position

The competitive landscape by 2030 is not the same as 2024. Three structural shifts
are underway:

**AI commoditisation collapses the SaaS moat.** SaaS incumbents built defensibility
on data lock-in and UI familiarity. AI-midwifed interfaces remove the UI lock-in:
any application can be described in natural language. The remaining moat is data.
SaaS vendors hold data on their servers. os-workplace holds data in a local WORM-sealed
ledger. When a customer asks "show me every rent payment for unit 4B since 2019," the
answer comes from a local `grep` over a YAML file in under one second — no API call,
no pagination, no SaaS export wizard.

**Regulatory pressure increases data residency requirements.** Privacy legislation
across every jurisdiction PointSav targets (Canada, UK, EU, Australia) is tightening.
The compliance posture of SaaS vendors — "your data is stored in our ISO 27001-certified
datacentre" — is becoming insufficient. os-workplace's posture — "your data is stored
on your machine; here is the SHA-256 of every record" — is a structural compliance
advantage, not a marketing claim.

**Developer-desktop fatigue creates a migration window.** The macOS development
experience has degraded: Gatekeeper friction, notarisation requirements, ARM
incompatibilities, and the progressive closure of the BSD userland. The developer
migration to Linux has been underway since 2020 and is accelerating. `os-developer`
targets this migration window specifically: it arrives with the macOS keybind profile
pre-loaded, the `pbcopy`/`open` shell aliases in place, and the fractional scaling
that makes HiDPI screens usable on Wayland.

| Dimension | Proprietary SaaS | macOS + SaaS | os-workplace |
|---|---|---|---|
| Data residency | Vendor servers | Local + iCloud | Local only; SQLite on disk |
| Audit trail | Vendor log (opaque) | OS log + vendor log | SHA-256 WORM ledger, user-held |
| AI model transparency | Black box, phone-home | Black box, phone-home | Local OLMo default; full DPO corpus |
| Migration cost | High (export wizard + vendor lock-in) | Medium (Apple ecosystem coupling) | Zero (`cp -r`) |
| Licensing cost | Monthly subscription | Hardware + $600/yr QuickBooks | $19 perpetual (os-workplace) |
| Network requirement | Always-on | Partial (iCloud sync) | Zero (offline-first) |
| Clipboard parity | Via browser APIs | Native (single-vendor) | Native (pointsav-clipboard-daemon) |
| Capability visibility | None | None | First-class (`Super+Shift+Space`) |
| Build attestation | Opaque | Signed by Apple (trust Apple) | Sigstore Rekor (verifiable) |

The 2030 claim is not that os-workplace beats macOS on every dimension. It is that
os-workplace beats macOS on the dimensions that will determine regulatory fitness,
data trust, and migration readiness in the next five years — and does so at a price
point that regulated SMBs can afford without enterprise procurement cycles.

### 15.11 What this BRIEF does not resolve

Three open questions remain outside the scope of this document:

1. **Brand name.** The internal names `os-tui`, `os-developer`, `os-workplace` are
   engineering handles. The customer-facing brand has not been decided. The architecture
   is neutral to this decision — any brand can be substituted without changing a line
   of code.

2. **The graphical shape companion BRIEF.** `BRIEF-app-workplace-architecture.md` is
   the designated home for the niri-fork-graphical compositor details, the Tauri v2
   application architecture, the GTK4 policy, and the os-workplace installer design.
   This BRIEF covers the TUI surface and the shared substrate; the graphical shape
   detail belongs in the companion.

3. **The language question.** Every operator-facing string in pointsav-tui-shell,
   the Tauri v2 apps, the onboarding wizard, and the installer must pass through the
   project-editorial language protocol (Bloomberg standard, no AI-product marketing
   vocabulary, BCSC disclosure posture). The architecture is complete; the copy is not.

These are not blockers. They are sequenced after v0.1 ships.

---

The architecture described across fifteen sections is internally consistent, externally
verifiable, and buildable from existing open-source components without any proprietary
dependency. The sovereignty claim holds at all eleven layers. The leapfrog claim is
demonstrable in under twenty minutes to a non-technical operator. The compounding
substrate grows more valuable with each operator who uses it.

We own it.

---

*§15 authors: CONC-01..10 research agents | 2026-05-25*

---

*Author: command@claude-code via 10 WP-TUI + 10 CP + 10 CONC research agents | 2026-05-25*
*Companion: `BRIEF-app-workplace-architecture.md` (niri-fork + Tauri v2 graphical shape)*
