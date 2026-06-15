---
artifact: brief
schema: foundry-brief-v1
title: BRIEF — Workplace Desktop Environment
status: active
created: 2026-05-27
cluster: project-workplace
language_protocol: PROSE-ARCHITECTURE
source_briefs:
  - BRIEF-app-workplace-architecture.md (§10 + §11)
  - BRIEF-tui-desktop-architecture.md (§5, §11, §12, §13, §14)
supersedes: BRIEF-workplace-desktop-suite.md (partial; OS-layer concerns)
related:
  - BRIEF-workplace-architecture.md
  - BRIEF-workplace-roadmap.md
---

# BRIEF — Workplace Desktop Environment

## 1. Mission

`os-workplace` is the sovereign native desktop operating environment.
This BRIEF governs the compositor, deployment shapes, session model, MBA
connectivity topology, clipboard contract, and GPU target matrix.
Application delivery (the `app-workplace-*` suite) is in
`BRIEF-workplace-software-suite.md`.

The workbench is the single visible program in the dock. Individual tool surfaces
(Memo, Proforma, Schedule, etc.) are renderer processes spawned per file schema and
invisible to the user. See §2 below.

---

## 2. Document-Centric Chassis Model

**There is one program in the dock: `app-workplace-launcher`.**

The user never "launches Memo" or "launches Proforma." They open a file, and the
launcher identifies the file schema and spawns the correct renderer process, which
paints its tool surface (toolbar, content area, status bar) inside the workbench window.
When the file is closed, the renderer process is reaped.

### File schema → tool surface mapping

| File | Schema identifier | Tool surface activated |
|---|---|---|
| `*.html` (with `<meta name="wp-type" content="memo">`) | Memo schema | Memo toolbar — rich text, autosave, export |
| `*.json` with `"proforma_version"` root key | Proforma v1.0 schema | Spreadsheet toolbar — formula bar, grid, XLSX exhaust |
| `*.json` with `"schedule_version"` root key | Schedule schema | Gantt toolbar — WBS, CPM, baseline, PMXML exhaust |
| `*.json` with `"presentation_version"` root key | Presentation schema | Slide toolbar — deck editor, presenter mode, HTML export |
| `*.geojson` / `*.json` with `"type": "FeatureCollection"` | GeoJSON RFC 7946 | GIS toolbar — MapLibre viewport, layer editor, PMTiles cache |
| `*.pdf` | ISO 32000 | PDF toolbar — pdfium viewer, annotation sidecar `.json` |
| `*.ifc` | IFC-SPF ISO 16739-1:2024 | BIM toolbar — @thatopen/components viewer, BCF issues, IDS validation |
| Code files (`.rs`, `.ts`, `.py`, etc.) | Extension-based | IDE toolbar — CodeMirror 6, LSP, integrated terminal |

### Why separate processes under one window

- **seL4 CNode isolation preserved.** Each renderer is a separate OS process with its own capability set. A BIM WASM panic cannot reach proforma data.
- **Crash containment.** If the IFC renderer crashes on a 200 MB model, the open memo is unaffected. The workbench surfaces a restart banner.
- **Per-app audit identity.** `system-mba` reads `SO_PEERCRED` per renderer PID, not per workbench PID — gateway audit trail records which file type made each request.
- **User experience.** One dock entry. No "which app do I need to open this file?" One consistent three-pane navigator (file tree / preview / tool surface) regardless of file type.

### Analogy

Chrome is one icon in the dock; it spawns a separate renderer process per tab. Users see one program; the OS sees many isolated processes. `app-workplace-workbench` follows the same pattern with file schemas substituting for URLs.

---

## 4. Three Deployment Shapes

One monorepo; three complementary shapes:

| Shape | Engineering name | Session model | Primary user |
|---|---|---|---|
| **Graphical Desktop** | `os-workplace` | niri-fork-graphical → Tauri v2 + WebKitGTK | Office workers, BIM/GIS users |
| **TUI Desktop** | `os-tui` | Kitty → `pointsav-tui-shell` → ratatui + ratatui-pixels | SSH/headless/remote/sovereignty-maximal |
| **Developer Desktop** | `os-developer` | niri-fork-developer → Tauri v2, stripped WM chrome | Developers building PointSav |

All three shapes share: BFS attribute store · substrate-handle broker ·
`DESK:*` namespace · `software.pointsav.com` distribution ($1 Apache 2.0
/ $19 FSL perpetual, ratified 2026-05-22).

---

## 5. Compositor Decision

**Recommendation: fork `niri` (GPL-3.0, Smithay-based, Rust, AArch64-clean).**
Two build profiles (`niri-fork-graphical` / `niri-fork-developer`) from one
codebase; 95% shared source tree.

### Weighted reasoning matrix

| Criterion | Weight | Sway-unmodified | niri-fork |
|---|---:|---:|---:|
| Engineering cost to v1 (lower is better) | 25% | 9 | 7 |
| AArch64 maturity | 15% | 9 | 9 |
| Rust-language coherence with app stack | 15% | 3 | 9 |
| EUPL 1.2 outbound compatibility | 10% | 9 | 7 |
| Workplace-policy invariance | 15% | 3 | 7 |
| Onboarding cost (tiling-capable, not default) | 10% | 6 | 8 |
| seL4 retargeting cost | 10% | 6 | 8 |
| **Weighted total** | **100%** | **6.60** | **7.85** |

**niri-fork wins.** The 2–4 EM cost premium over Sway-unmodified buys workplace-policy
invariance (compositor enforces "only `app-workplace-*` clients") and Rust-coherence
with the app stack — both structural moats unavailable from Sway.

**License:** GPL-3.0 is in the EUPL 1.2 outbound compatibility appendix. The compositor
ships as a separable GPL-3.0 binary; `app-workplace-*` apps connect over Wayland IPC
and remain EUPL 1.2. The workplace shell layer is a new EUPL 1.2 component, not a
relicense of the niri fork.

**Engineering timeline (one senior Rust engineer, AArch64-fluent):**
- v0.1 (rebrand + KDL strip + `workplace-shell-v1` protocol): 2–4 EM, ~3 calendar months
- v0.3 (launcher IPC wired, F-row vocabulary enforced, mode-bar live): +3 EM, ~6 months
- v0.5 (`os-workplace config` CLI + L0/L1/L2 TOML reload): +3 EM, ~9 months

**Fallback:** Sway-unmodified if v0.1 slips by >2 EM.

---

## 6. TUI Desktop Architecture

Three layers; one PTY column top to bottom.

```
┌──────────────────────────────────────────────────────────────┐
│ L0 — Kitty (GPU pipeline; OpenGL/Vulkan; KMS/DRM)            │
│   Kitty graphics protocol (KGP) · SGR mouse · OSC 52        │
│   [v1: runs inside niri-fork Wayland]                        │
│   [v2: direct-KMS, no compositor required]                   │
└───────────────────┬──────────────────────────────────────────┘
                    │ PTY + escape sequences
┌───────────────────▼──────────────────────────────────────────┐
│ L1 — pointsav-tui-shell (Zellij fork)                        │
│   Floating-default layout · SGR mouse routing               │
│   OSC 52 clipboard broker (hard read-deny policy)           │
│   OSC 5522 drag-and-drop broker                             │
│   Fuzzy launcher (Super+D) · persistent named layouts       │
│   Substrate-handle broker (Unix domain socket)              │
│   Control IPC: $XDG_RUNTIME_DIR/foundry-wm.sock             │
└───────────────────┬──────────────────────────────────────────┘
                    │ UDS IPC (handle broker, notifications, DnD)
┌───────────────────▼──────────────────────────────────────────┐
│ L2 — ratatui apps (one OS process each; one seL4 CNode each) │
│   ratatui 0.29+ · ratatui-pixels · tui-desktop-widgets      │
│   crossterm SGR mouse → DesktopEvent dispatch               │
│   BFS attribute queries for staged items / recent docs      │
└──────────────────────────────────────────────────────────────┘
```

**Key constraints:**
- OSC 52 read: hard-deny at L1 (no TUI app may read the system clipboard directly)
- One Kitty window per `app-workplace-*` instance
- Apps talk to the WM via UDS control plane — never via escape sequences
- Kitty graphics state does not survive Zellij detach/reattach; apps re-upload on every render

**Windows/macOS extension:**
- Model A (primary): WezTerm packaged app — WezTerm + `pointsav-tui-shell` + ratatui
  binaries bundled into a single `.dmg` / `.exe`; user never sees a terminal
- Model B (secondary): remote session via WireGuard PPN; suitable for keyboard-driven
  data entry only; no drag-drop / audio / printer integration
- macOS notarization required ($99/yr Apple Developer); Windows OV cert (~€250–350)

---

## 7. Developer Desktop

`os-developer` = `niri-fork-developer` (same codebase as graphical compositor, different
build profile) + stripped chrome. Developers building PointSav dogfood it; same binaries
as customer deployments.

**Five design principles (ordered by primacy):**
1. Keyboard completeness — every workflow has a `Super+*` keybind; mouse is an accelerator, never a gate
2. Auditable surface — fewer components, each readable Rust or single-purpose C; `ss -tunlp` on fresh boot fits in 25 lines
3. One process per app — each Tauri v2 app and daemon is its own `systemd --user` unit with explicit capability TOML
4. Rust where it touches the user — status bar, launcher, notification daemon, clipboard manager, screen locker all compile under `cargo build --workspace`
5. Same binary the customer downloads — `os-developer.img` enters `software.pointsav.com` release manifest at v0.3; build-host attestation chain verifies

**Include / exclude decisions:** niri-fork-developer (compositor), greetd + tuigreet
(greeter), Kitty (terminal), waybar or swaybar (status bar), mako (notifications),
swaylock-rs (screen locker), xdg-desktop-portal-wlr (screenshare), secret-service-rs
(secrets), no GNOME lifecycle services, no online accounts, no Evolution Data Server,
no display manager chooser, no dock, no desktop icons.

**Prior art validated:** System76 COSMIC (Smithay-based Rust, simultaneous tiling +
floating) is the closest external validation. Gap: no Doorman-aware launcher, no
per-app capability visibility. niri-fork-developer fills that gap.

---

## 8. Clipboard Daemon — `pointsav-clipboard-daemon`

Solves the six clipboard failure modes (ANSI escape leakage, Wayland-source-dies
clipboard loss, Ctrl+C collision, X11 bridge gap, OSC 52 over SSH, F12 CWD).

| Property | Value |
|---|---|
| Crate | `pointsav-monorepo/tools/clipboard-daemon/` |
| Protocol | `ext-data-control-v1` preferred → `wlr-data-control-unstable-v1` fallback |
| IPC | D-Bus: `com.pointsav.Clipboard` at `/com/pointsav/Clipboard/1` |
| Storage | In-memory ring buffer (256 entries, 16 MiB cap) + `sled`-backed WAL |
| Est. size | ~4,800 LOC Rust |
| ratatui clipboard std | `wl-clipboard-rs::copy::copy_multi` (multi-MIME) — never raw OSC 52 write |

**Persistence:** daemon binds `ext-data-control-v1` focus-free, eagerly drains all
MIME types into ring buffer on every `selection` event, and re-creates the selection
with itself as source when the original source disconnects. F2 (Wayland-source-dies) closed.

**Security:** compositor allow-list patch (~150 LOC on Smithay's `SelectionHandler`
allow-callback) — only `pointsav-clipboard-daemon`, `pointsav-launch`, and signed
PointSav binaries may bind the focus-free global.

---

## 9. MBA Connectivity Topology (Ratified: Topology D)

Two OS-level IPC contracts per station:

| Contract | Socket path | Owner | Scope |
|---|---|---|---|
| Launcher contract | `$XDG_RUNTIME_DIR/workplace-launcher.sock` | `app-workplace-launcher` → future `os-workplace` system service | App lifecycle, file picker, recent docs, focus, UI handoffs |
| MBA contract | `/run/foundry/system-mba.sock` | `system-mba.service` (systemd unit; OS-tier; never user-tier) | Per-app audit identity, WireGuard egress, Doorman routing |

**Topology D: `system-mba` OS-tier daemon (8-agent Opus consensus; weighted score 8.25).**

`system-mba` reads `SO_PEERCRED` on each local UDS connection, resolves PID to binary
path via `/proc/<pid>/exe`, signs an Ed25519 audit header `(station, app, user,
timestamp, request-hash)` before forwarding over `wg0`. Apps hold no signing key, no
`CAP_NET_ADMIN`, no WireGuard FD — only a connected `AF_UNIX` endpoint.

**Jennifer-without-Console test:** `system-mba.service` is a systemd unit started before
any UI process. Apps connect directly; Console state is irrelevant. Launcher crash does
not affect MBA or any connected app. This test eliminates Topologies A and B.

**Minimum-viable `system-mba` UDS contract:**
- `OpenStream { target_service, optional_app_hint }` — app → daemon; daemon returns connected stream
- `Health` — tunnel state + last-handshake timestamp
- Wire format: length-prefixed CBOR; `Version: u8` byte at frame start
- Contract docs at `vendor/pointsav-monorepo/system-mba/docs/UDS-CONTRACT.md`

---

## 10. What `os-workplace` Must Provide

- AArch64 native bootable image; NetBSD compatibility bottom today; seL4 native as moonshot (Doctrine claim #34)
- Wayland + GTK + WebKitGTK 4.1 stack (Tauri v2 Wayland client requirement)
- Per-station WireGuard PPN identity at `10.42.20.x` (per master plan §B address plan)
- `pairings.yaml` entries per station; per-app TLS certs for gateway audit identity
- Per-app data-directory isolation: `~/.local/share/workplace/<app>/`; mapped to per-app seL4 CNode sets on native bottom
- Freedesktop `.desktop` integration; launcher and each `app-workplace-*` app appear in OS app grid independently
- `service-slm` Doorman endpoint reachable over WireGuard tunnel (Doctrine claim #43 single-boundary compute)

---

## 11. Onboarding Model

**"Tiling-capable, not tiling-default"** (UX Researcher + SMB Survey independent convergence).

| Phase | Default mode | Trigger | What changes |
|---|---|---|---|
| Day 0 | Stacking layout, mouse-first | Boot | Arrow keys work; single-click focuses; `Super+T` flips to tiling |
| Week 2 | Stacking + focus-promotion hints | `os-workplace promote --next` | Mode-bar adds "tiling-ready — `Super+T` to lock in" |
| Week 4 | Tiling layout, keyboard-first | User opts in | Compositor defaults to scroll-strip tiling; mouse resize remains |
| Month 3 | Tiling + plumb-by-selection | User opts in | Acme-style Button-3 plumb; `Super+Shift+s` Send palette muscle memory |

Each promotion is opt-in, reversible (`os-workplace promote --rollback`). L1 deployment
policy can lock any user at any phase. The universal F-row keyboard vocabulary (50-line
spec at `vendor/pointsav-os-workplace/conventions/keyboard-vocabulary.md`) applies to
every `app-workplace-*` app.

---

## 12. AArch64 GPU Target Matrix

| Target | Mesa driver | Vulkan | Status |
|---|---|---|---|
| Qualcomm Snapdragon X-series (Adreno) | freedreno | Turnip | **Primary** — Mesa 25.1 default; production-grade |
| ARM Mali Valhall G610-class | Panfrost | PanVK | **Secondary** — Vulkan 1.1 conformant April 2025; strongest open-driver story |
| Rockchip RK3588 (Panfrost) | Panfrost | PanVK | Tertiary eval-only (low-cost SMB hardware) |
| Apple M1/M2 (Asahi) | Asahi | Asahi | Tertiary eval-only (Mac-resident developers) |

Out of scope for v1: Apple M3/M4/M5 (Asahi GPU in progress); NVIDIA Jetson; Apple Silicon for customer-facing deployments.

---

## 13. Ship Path

### Graphical Desktop v0.1
1. Fork niri → `niri-fork-graphical`; strip KDL config; add `workplace-shell-v1` Wayland protocol
2. Wire launcher IPC (`workplace-launcher.sock`) and MBA daemon (`system-mba.service`)
3. Ship `app-workplace-launcher` + `app-workplace-memo` as first end-to-end demo

### TUI Desktop v0.1 (parallel track; does not block Graphical)
1. Package Kitty as signed `os-workplace` component on `software.pointsav.com`
2. Fork Zellij → `pointsav-tui-shell`; add OSC 52 broker (hard read-deny), fuzzy launcher, `foundry-wm.sock`
3. Publish `ratatui-pixels` + `tui-desktop-widgets` in monorepo (`tools/tui-desktop/`)
4. Port two existing ratatui surfaces (proofreader CLI, bookkeeping TUI) as first demo

### Developer Desktop v0.1
1. Build `niri-fork-developer` profile from same niri fork codebase
2. Assemble stripped component stack (greetd, Kitty, waybar, mako, swaylock-rs, xdg-portal-wlr)
3. Enter `software.pointsav.com` release manifest at v0.3

### Moonshot path
| Phase | What changes |
|---|---|
| v2 | Kitty direct-KMS; Wayland removed from TUI/sovereign deployments |
| v3 | Terminal emulator as seL4 native task; each ratatui app as separate CNode |
| Launcher absorption | `app-workplace-launcher` becomes a native `os-workplace` system service; `workplace-launcher.sock` contract unchanged (already an OS-level ABI) |

---

## 14. Open Questions

- Pairing-server port for `system-gateway-mba`: TBD — check pairing-server deployment
- v0.5 BFS attribute substrate decision: Haiku BFS port vs Btrfs/ZFS xattrs + indexed query daemon (deferred to v0.5 ratification)
- `app-workplace-code` (coding IDE) priority and scheduling within project-workplace (parallel track; does not block app-suite Sprint 1)
