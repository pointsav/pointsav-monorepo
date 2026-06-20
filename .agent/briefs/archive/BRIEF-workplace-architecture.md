---
artifact: brief
schema: foundry-brief-v1
title: BRIEF — Workplace Architecture (Source of Truth)
status: active
created: 2026-05-31
cluster: project-workplace
language_protocol: PROSE-ARCHITECTURE
purpose: >
  Drift-check reference and ratified architecture source of truth for project-workplace.
  Contains the canonical app inventory, IPC contract, deployment shapes, and 6 non-negotiable
  drift-check rules that every proposed change must satisfy.
supersedes:
  - BRIEF-leapfrog-2030-audit-and-vision.md
  - BRIEF-workplace-software-suite.md
  - BRIEF-workplace-desktop-environment.md (§2, §4, §9 — OS environment detail remains in that file)
related: BRIEF-workplace-roadmap.md
---

# BRIEF — Workplace Architecture (Source of Truth)

## 1. Mission + Competitive Moat

The `app-workplace-*` suite delivers a 2030-class native workbench for office and
technical workflows that hyperscalers cannot replicate.

**Three sovereignty pillars:**
- **Sovereignty by construction** — proof-based security (seL4), air-gapped capability
  sets, local-first data. No telemetry channel outside policy.
- **Leapfrog UI/UX** — hybrid TUI/GUI paradigm (the "Desk" surface) achieving
  clipboard/device parity with macOS/Windows at terminal-tier performance.
- **AI as Accelerant** — audited AI integration (Doorman) that is local-first (Tier A
  OLMo) and strictly user-initiated for Tier B/C. User data never leaks; AI remains
  a controllable tool, not a dependency.

**Why hyperscalers cannot compete:** they are structurally compelled to build telemetry,
cloud-subscription dependencies, and browser-engine reliance into their products.

- **Zero-Trust Surface** — no Chromium, no Electron, no vendor-account requirements,
  no telemetry-channel-outside-policy.
- **Provable Auditability** — every app's capability set is visible, live-revokable, and
  auditable from a single keystroke (`pointsav-launch --capability-mode`).
- **Architecture as Moat** — one OS process per app per CNode (seL4), with a
  formal-verification path that no vendor-OS can retrofit.

---

## 2. Ratified Architecture — Option (j)

**`app-workplace-launcher` (suite identity shell) + 9 separate Tauri v2 desktop
processes. 7/8 Opus agent consensus. Weighted score: 8.43.**

### Veto summary

Every option carrying a browser-delivered leg is vetoed by `connect-src 'none'`
sovereignty enforcement. The browser leg's DNS resolution, TLS handshake, OCSP/CT
logging, and vendor telemetry sit outside any page CSP and cannot be proved offline.
Options (b), (c), (e), (f), (h), (i) all vetoed. Option (g) (unified workbench) vetoed
on Leapfrog 2030 grounds: collapses 9 app capability domains into one seL4 CNode set,
voluntarily discards the kernel's primary value (per-process formal-verification isolation).

Options (a) — separate apps, no launcher — and (j) — launcher + 9 apps — are the
surviving options. (j) wins by 0.18 on Security/Sovereignty (+0.50), Economic (+1.00),
and UI/UX (+1.00) sub-dimensions.

**Claim #2 — 100-year readability.** Canonical formats are all plain-text or
open-standard: `.html` (Memo), `.json` (Proforma, Schedule, Presentation, GIS),
`.ifc` (BIM, ISO 16739-1:2024), `.pdf` (PDF viewer). No proprietary binary format
is ever the fiduciary record. Export to legacy formats (XLSX, PPTX, MPX) is permitted
as exchange exhaust; these are never the source of truth.

---

## 3. App Inventory

| App | Priority | Registry state | Canonical format | Next action |
|---|---|---|---|---|
| `app-workplace-launcher` _(branded: **Workbench**)_ | **P1 — ships first** | Not in registry | — (shell, no document type) | Create crate; three-pane navigator; file-schema dispatch; IPC contract |
| `app-workplace-code` | Parallel track | Not in registry | code files (`.rs`, `.ts`, `.py`, etc.) | VS Code analogue; CodeMirror 6; LSP; integrated terminal; does not block Suite Sprint 1 |
| `app-workplace-memo` | **P2 — first renderer** | Scaffold-coded (47 files) | `.html` (self-contained, base64 fonts) | Tauri v1→v2; workbench handshake; autosave |
| `app-workplace-pdf` | P3 | Not in registry | `.pdf` (ISO 32000) | Create crate; pdfium-render; annotation sidecar `.json` |
| `app-workplace-proforma` | P4 | Active (local-only CLAUDE.md) | `.json` (v1.0 schema — `docs/schema.md`) | Resolve "local-only" status; Tauri v1→v2; workbench handshake |
| `app-workplace-schedule` | P4.5 | Not in registry | `.json` (versioned schema) | Create crate; egui Gantt widget; WBS + CPM; MS Project muscle memory; MPX/PMXML import+export; construction + employee scheduling — **NOT a calendar** |
| `app-workplace-presentation` | P5 | Active (Phase 5) | `.json` (slide schema) + `.html` export | Tauri v1→v2; workbench handshake; presenter display |
| `app-workplace-gis` | P6 | Not in registry | `.geojson` (RFC 7946) | Create crate; MapLibre GL JS; OpenFreeMap PMTiles tile cache |
| `app-workplace-bim` | P7 — gated | Reserved-folder (2 files) | `.ifc` (ISO 16739-1:2024) — immutable | Awaiting xeokit quote; MPL-only v0 possible |

---

## 4. Canonical File Format Registry

All formats are open, flat, and standard. No proprietary binary format is ever the
fiduciary record. Export to legacy formats is permitted as exchange exhaust only.

| Tool surface | Canonical format | Standard / schema | Export / import |
|---|---|---|---|
| Memo | `.html` | Self-contained; base64 fonts; browser-readable forever | `.pdf` (print exhaust) |
| Proforma | `.json` | v1.0 schema at `app-workplace-proforma/docs/schema.md`; SHA-256 audit chain | `.xlsx` / `.csv` (legacy exhaust); `.pdf` (print exhaust) |
| Schedule | `.json` | Versioned schema (to define); FIDIC §20.1 baselines via git tags | `.pmxml` / `.mpx` import+export; `.pdf` export; TaskJuggler DSL export |
| Presentation | `.json` | Versioned slide schema (to define) | `.html` (self-contained export); `.pdf` (print exhaust) |
| GIS layers | `.geojson` | RFC 7946 | PMTiles (`.pmtiles`) for read-only tile cache; `.pdf` export |
| PDF viewer | `.pdf` | ISO 32000 | Annotation sidecar as `.json` |
| BIM | `.ifc` | ISO 16739-1:2024 — immutable per `bim-product-family.md` | BCF 3.0; IDS 1.0; SVG floor plans; glTF cache |

**Rule: SYS-ADR-07 applies to all structured formats.** Proforma, Schedule, and GIS
data must not transit `service-slm` inference. The formula engine, scheduler, and
projection engine are all native Rust — never routed through AI inference.

---

## 5. Three Surfaces Disambiguation

Three distinct products sharing the word "workbench." Do not conflate them.

| | `app-workplace-launcher` | `app-workplace-code` | `app-privategit-workbench` |
|---|---|---|---|
| **Product name** | **Workbench** (the suite; one program in the dock) | Code surface | PrivateGit Workbench |
| **What** | Document-centric launcher; spawns renderer processes for each file schema | Native desktop coding IDE; VS Code analogue | HTTP browser-based three-column developer tool |
| **Stack** | Tauri v2; IPC socket; file-schema dispatch | Tauri v2 + WebView; CodeMirror 6; LSP; integrated terminal | HTTP server (axum); browser-accessible; file tree / viewer / code panes |
| **Persona** | All Workbench users (it IS the product) | Developers building PointSav software | Totebox Orchestration community users |
| **Owner** | `project-workplace` (this archive) | `project-workplace` (this archive) | `project-development` (Jennifer's scope) |
| **State** | Not in registry; create crate; P1 | Not in registry; parallel track | Actively in development; currently hosted at `http://10.8.0.9:9200/workbench/` |
| **Jennifer's role** | Primary end user | Not involved | **Active developer** |

This cluster does NOT own `app-privategit-workbench`. Improvements tracked here because
the prototype hosts it; handoffs to project-development via outbox.

---

## 6. Document-Centric Chassis Model

**There is one program in the dock: `app-workplace-launcher`.**

The user never "launches Memo" or "launches Proforma." They open a file, and the
launcher identifies the file schema and spawns the correct renderer process, which
paints its tool surface (toolbar, content area, status bar) inside the launcher window.
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

- **seL4 CNode isolation preserved** — each renderer is a separate OS process with its
  own capability set. A BIM WASM panic cannot reach proforma data.
- **Crash containment** — if the IFC renderer crashes on a 200 MB model, the open memo
  is unaffected. The launcher surfaces a restart banner.
- **Per-app audit identity** — `system-mba` reads `SO_PEERCRED` per renderer PID; gateway
  audit trail records which file type made each egress request.
- **User experience** — one dock entry; one consistent three-pane navigator (file tree /
  preview / tool surface) regardless of file type.

---

## 7. Deployment Shapes (Triple-Shape Topology)

One monorepo; three complementary shapes sharing the same substrate (Kitty, ratatui,
BFS attribute store, audit ledger):

| Shape | Engineering name | Session model | Primary user |
|---|---|---|---|
| **Graphical Desktop** | `os-workplace` | niri-fork-graphical → Tauri v2 + WebKitGTK | Office workers, BIM/GIS users |
| **TUI Desktop** | `os-tui` | Kitty → `pointsav-tui-shell` → ratatui + ratatui-pixels | SSH/headless/remote/sovereignty-maximal |
| **Developer Desktop** | `os-developer` | niri-fork-developer → Tauri v2, stripped WM chrome | Developers building PointSav |

All three shapes share: BFS attribute store · substrate-handle broker ·
`DESK:*` namespace · `software.pointsav.com` distribution ($1 Apache 2.0
/ $19 FSL perpetual, ratified 2026-05-22).

See `BRIEF-workplace-desktop-environment.md` for compositor decision, TUI architecture,
developer desktop spec, clipboard daemon, and GPU target matrix.

---

## 8. IPC Model

### Launcher contract

**Socket:** `$XDG_RUNTIME_DIR/workplace-launcher.sock`
**Transport:** length-prefixed CBOR or MessagePack; versioned; no networking primitives
**Crates:** `interprocess` and/or `tokio-unix-ipc`
**Supervisor:** ~200-line bespoke Rust on `tokio::process::Command`

**Minimum-viable message set:**
- `Hello { app_id, pid, version }` — child → launcher on startup
- `OpenDocument { path }` — launcher → child (file-open routing from shared picker)
- `RecentDocument { path, app_id }` — child → launcher (local telemetry)
- `Quit` — launcher → child (graceful shutdown)
- `Heartbeat` — both directions (crash detection)

`MbaSocketFd` is NOT in the launcher IPC contract. Apps connect directly to
`/run/foundry/system-mba.sock` at startup.

**Moonshot note:** this contract is designed as a stable OS-level ABI from day 1. When
the launcher is absorbed into `os-workplace` as a system service, `app-workplace-*`
binaries connect to the same socket unchanged. `app-workplace-code` is user-tier only.

### MBA contract (separate)

`system-mba.service` is an OS-tier systemd unit. Apps connect directly; launcher crash
does not affect MBA. Full MBA contract spec: `BRIEF-workplace-desktop-environment.md` §9.

---

## 9. MBA Connectivity Topology — Topology D (Ratified)

**8-agent Opus consensus; weighted score 8.25.**

`system-mba` reads `SO_PEERCRED` on each local UDS connection, resolves PID to binary
path via `/proc/<pid>/exe`, signs an Ed25519 audit header
`(station, app, user, timestamp, request-hash)` before forwarding over `wg0`. Apps hold
no signing key, no `CAP_NET_ADMIN`, no WireGuard FD — only a connected `AF_UNIX` endpoint.

| Contract | Socket path | Owner |
|---|---|---|
| Launcher contract | `$XDG_RUNTIME_DIR/workplace-launcher.sock` | `app-workplace-launcher` → future OS system service |
| MBA contract | `/run/foundry/system-mba.sock` | `system-mba.service` (OS-tier; never user-tier) |

**Minimum-viable UDS contract:**
- `OpenStream { target_service, optional_app_hint }` — app → daemon; daemon returns connected stream
- `Health` — tunnel state + last-handshake timestamp
- Wire format: length-prefixed CBOR; `Version: u8` byte at frame start

---

## 10. Drift Check — 6 Non-Negotiables

Every proposed change to this cluster must clear all six tests before implementation.

| # | Check | Non-negotiable |
|---|---|---|
| 1 | **Network surface** | Does this change increase the network surface? Goal: zero network syscalls by default. All egress must be audited via `system-mba`. |
| 2 | **Capability visibility** | Is the app's capability set still visible via `pointsav-launch --capability-mode`? |
| 3 | **Canonical format** | Is the new file format open, flat, and standard (`.html`, `.json`, `.ifc`)? Does it avoid proprietary binaries? |
| 4 | **AI-safety** | Does this feature transit Doorman inference? If so, is it local-first (Tier A OLMo) or strictly user-initiated (Tier B/C)? SYS-ADR-07: structured data must NOT transit `service-slm`. |
| 5 | **State management** | Does the new component architecture use the reactive store, or is there residual imperative DOM manipulation? Goal: zero imperative DOM manipulation in new work. |
| 6 | **Isolation** | Does the component-shell still enforce the IPC contract? Is there any leaked state between panes? |

---

## 11. Wave 1 Exit Criteria

1. Launcher + `app-workplace-memo` handshake end-to-end (`Hello` + `OpenDocument` + `Heartbeat`)
2. Document opens via launcher's shared file picker
3. Memo opens, edits, saves, and re-opens a document correctly
4. macOS `x86_64-apple-darwin` build clean for launcher + Memo
5. Smoke test: connect Memo to WireGuard PPN endpoint via `system-mba.service`; Doorman responds
