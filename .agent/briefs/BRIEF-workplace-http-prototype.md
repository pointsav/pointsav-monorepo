---
artifact: brief
schema: foundry-brief-v1
title: BRIEF — Workbench HTTP Prototype
status: archived
superseded_by: BRIEF-workplace-roadmap.md
created: 2026-05-28
cluster: project-workplace
language_protocol: PROSE-ARCHITECTURE
related: BRIEF-workplace-software-suite.md
---

# BRIEF — Workbench HTTP Prototype

## 1. Mission

The HTTP prototype is a UX laboratory and daily-use feedback mechanism. It provides
access to all 8 Workbench tool surfaces in a browser — running on the VM, accessible
via WireGuard PPN — while the sovereign native Tauri apps are being engineered on a
macOS host.

Jennifer uses the prototype for real work, 8+ hours per day. Her direct experience
surfaces what is working, what is wrong, and what to build next. Each session targeting
a new stage is driven by that feedback. The BRIEF is the continuity artifact: updated
after each session to record what was built, what Jennifer observed, and what the next
stage is.

The prototype is NOT the production delivery. It is the design specification. Every UX
decision validated here feeds directly into `BRIEF-workplace-software-suite.md` and the
native app implementations.

---

## 2. Why now

- Native Tauri builds require a macOS build host with Xcode. That host is not yet in the
  build pipeline for this cluster.
- The HTTP prototype runs on the existing VM (`10.8.0.9`) and is reachable from any
  browser on the WireGuard PPN — no special toolchain, no install.
- Iteration loop is fast: edit Rust + JS assets → rebuild binary → Jennifer refreshes.
- Prototype and native apps share canonical file formats. A `.html` memo created in the
  prototype opens identically in `app-workplace-memo` native when that ships. No
  migration cost; the formats ARE the contract.

---

## 3. Architecture

**Crate:** `app-workplace-http-prototype`
(location: `pointsav-monorepo/app-workplace-http-prototype/`)

**Stack:**
- Rust + axum HTTP server; single binary
- Frontend assets embedded via `rust-embed` — no CDN; fully offline-capable on the PPN
- Port: 9110 (default; configurable via `WORKPLACE_PROTO_PORT` env var)
- Access: `http://10.8.0.9:9200` via WireGuard PPN (nginx proxy; prototype binds on `0.0.0.0:9110` internally)

**File storage:**
- Workspace directory configurable via `WORKPLACE_PROTO_WORKSPACE` env var
- Default: `/home/jennifer/workbench/`
- Files are stored at their canonical paths with their canonical extensions
- No database; files are the state

**Routing:**
- `/` — surface selector (open existing file or create new)
- `/memo` — Memo surface
- `/proforma` — Proforma surface
- `/presentation` — Presentation surface
- `/schedule` — Schedule surface
- `/code` — Code surface
- `/pdf` — PDF surface
- `/gis` — GIS surface
- `/bim` — BIM surface
- `/api/files` — file listing + CRUD endpoints (axum JSON API)

**CSP note:** The prototype intentionally does NOT enforce `connect-src 'none'`. That
sovereignty rule applies to the native Tauri apps only. The prototype may load
MapLibre GL JS tiles from OpenFreeMap, PDF.js, and other bundled/CDN assets as needed
for rapid iteration. SYS-ADR-07 still applies: Proforma, Schedule, and GIS data must
not transit `service-slm` inference even in the prototype.

---

## 4. Tool surfaces — build priority order

Each surface has two thresholds:
- **Usable:** Jennifer can do real work with it (open, create, edit, save)
- **Native-parity:** HTTP prototype and native Tauri app are equivalent in daily use

| # | Surface | Muscle memory | Canonical format | HTTP technology |
|---|---------|--------------|-----------------|-----------------|
| 1 | **Memo** | Word / Pages | `.html` (self-contained) | TipTap or ProseMirror (MIT, embeddable) |
| 2 | **Proforma** | Excel | `.json` (proforma v1.0 schema) | Handsontable Community (MIT) or custom grid |
| 3 | **Presentation** | PowerPoint / Keynote | `.json` (versioned slide schema) | Custom slide canvas (HTML/CSS/JS) |
| 4 | **Schedule** | MS Project / Primavera P6 | `.json` (versioned schema) | Custom Gantt + WBS — **NOT a calendar** |
| 5 | **Code** | VS Code | code files (`.rs`, `.ts`, `.py`, etc.) | CodeMirror 6 (MIT) |
| 6 | **PDF** | Adobe / Preview | `.pdf` (ISO 32000) | PDF.js (Apache 2.0) |
| 7 | **GIS** | QGIS | `.geojson` (RFC 7946) | MapLibre GL JS + OpenFreeMap tiles |
| 8 | **BIM** | AutoCAD / Revit | `.ifc` (ISO 16739-1:2024) | @thatopen/components |

**Schedule note:** Schedule is a Gantt / CPM / WBS scheduling tool for construction
projects and employee scheduling. It is NOT a calendar. It does not show appointments
or events. It shows tasks, dependencies, durations, and critical paths. Muscle memory
target is MS Project; secondary is Primavera P6.

**SYS-ADR-07 (hard rule):** Proforma formula evaluation, Schedule CPM calculation,
and GIS projection transforms must NOT transit `service-slm` inference. These are
structured data transforms implemented in Rust — never routed through AI.

---

## 5. Stage build plan

Each stage = one surface reaching **usable threshold**. Stages are worked sequentially
by default; Jennifer requests the next stage when the current one is usable for her.

Current stage status is updated in this section after each session.

```
Stage 1: Memo
  - Open an existing .html file from the workspace directory
  - Create a new .html document
  - Edit with basic formatting: bold, italic, headings (H1–H3), unordered list
  - Save (overwrite in place)
  - Status: [x] complete — crate created 2026-05-28; cargo build clean

Stage 2: Proforma
  - Open an existing .json proforma file
  - Create a new proforma
  - Edit: add/remove rows and columns; enter values; basic formula (SUM)
  - Save
  - Status: [x] complete — proforma-v1.0 JSON schema; /proforma route; commit a444266b 2026-06-01

Stage 3: Presentation
  - Open an existing .json slide deck
  - Create a new presentation
  - Add/remove slides; add text and image to a slide; reorder slides
  - Save
  - Status: [ ] not started

Stage 4: Schedule
  - Open an existing .json schedule
  - Create a new schedule
  - Add tasks with names and durations; indent/outdent for WBS hierarchy
  - Connect tasks with finish-to-start dependencies
  - View as Gantt bars on a date axis
  - Save
  - NOT a calendar — no appointment or event concept
  - Status: [ ] not started

Stage 5: Code
  - Browse workspace file tree
  - Open a code file (any extension)
  - Edit with syntax highlighting
  - Save
  - Status: [ ] not started

Stage 6: PDF
  - Open a .pdf file from the workspace
  - Page navigation (prev/next/jump to page)
  - Zoom in/out
  - Status: [ ] not started

Stage 7: GIS
  - Open a .geojson file
  - Render features on a MapLibre GL map viewport
  - Layer list (show/hide)
  - Status: [ ] not started

Stage 8: BIM
  - Open an .ifc file
  - Render 3D model with orbit camera (@thatopen/components)
  - Element tree panel
  - Status: [ ] not started
```

After all 8 stages reach usable threshold: begin **native-parity pass** — align each
HTTP surface with the corresponding native Tauri app spec in
`BRIEF-workplace-software-suite.md`.

---

## 6. Jennifer's feedback loop

Jennifer is the operator and primary user of the prototype. The feedback loop:

1. Jennifer uses the prototype for real work at `http://10.8.0.9:9200`
2. When a surface is usable enough, or when something needs fixing, she opens a session
   and says: **"work on the next stage of [surface]"** or **"fix [specific behaviour]"**
3. The session targets one surface at a time; BRIEF stage status is updated at session end
4. Feedback that changes a UX decision is reflected in `BRIEF-workplace-software-suite.md`
   (via edit to the relevant section) and noted in this BRIEF's session log below

**Session log** (newest on top):

```
2026-06-01 — Totebox@claude-code: Stage 2 Proforma complete — commit a444266b
  proforma-v1.0 JSON schema (schema/title/rows/cols/cells map). New asset
  proforma.html: dark-theme spreadsheet grid, column headers (A/B/C…) + row numbers,
  click-to-select, double-click or keypress to edit via absolute-positioned overlay,
  Tab/Enter/Escape/Arrow navigation, Delete clears cell, F2 opens edit, Ctrl+S saves.
  Formula evaluator: =SUM(range), cell refs (=A1), arithmetic (=A1+B2*C3). Numbers
  right-aligned with en-CA locale formatting. Add/remove row and column buttons.
  Auto-save 3 sec after last edit. Backend: /proforma route, /api/proforma/files GET,
  /api/proforma/create POST; proforma/ dir auto-created at startup.
  Workbench: renderProformaSurface() panel wired — file list + new proforma button.
  Build clean; all endpoints verified (create, read, save round-trip).
  Stage 6 pending.

2026-05-30 — Totebox@claude-code: Workbench merge into prototype at /workbench/
  app-privategit-workbench SPA (3575-line VS Code-style editor) + full backend
  ported into app-workplace-http-prototype as src/workbench.rs. Nested at
  /workbench/; base href injected at serve time. TOML config (config.toml)
  with roots model: sandbox-jennifer (writable), _command, _clones. AppState
  extended with roots/weasyprint/max_bytes; watcher extended to writable roots.
  /files/ nginx path and standalone workbench at 9210 untouched pending
  Jennifer's confirmation. Files tile added to surface selector → /workbench.
  Commit ab75fa69.

2026-05-30 — Totebox@claude-code: SSE real-time file events + nginx consolidation
  nginx 9200 now proxies prototype root (was intranet static dashboard); prototype is
  the daily-use tool at http://10.8.0.9:9200. SSE endpoint /api/files/events added:
  notify crate (inotify) watches workspace; broadcasts "changed" on any file event;
  index.html + memo.html connect via EventSource and re-fetch file list automatically.
  Broadcast also fires on save_file + create_file API calls. VM IP corrected to 10.8.0.9.
  Commit fab7a2f6.

2026-05-28 — Totebox@claude-code: Stage 1 Memo complete — app-workplace-http-prototype
  crate created; axum 0.7 + rust-embed 8; surface selector + memo.html; 8-item toolbar
  (B I H1 H2 H3 • Save Open▾); contenteditable editor; Ctrl+S save; 30s auto-save; dirty
  title indicator; /api/files CRUD; path validation; memo/ subdir auto-created on startup.
  Viewer-bar overflow menu also added to app-privategit-workbench (collapsing Print/Copy/
  Reload/NewTab into ⋯ dropdown). Build: cargo build clean.

2026-05-28 — Totebox@claude-code: BRIEF created; Stage 1 (Memo) is the first build target
```

---

## 7. Relationship to native apps

| Property | HTTP prototype | Native Tauri apps |
|---|---|---|
| File formats | Identical (`.html`, `.json`, `.pdf`, `.ifc`, `.geojson`) | Same |
| Sovereignty | Relaxed CSP; prototype only | `connect-src 'none'`; full sovereignty |
| seL4 isolation | None | One CNode per app |
| Distribution | No binary; VM-hosted; WireGuard PPN only | `software.pointsav.com`; Apple-notarized |
| Purpose | UX laboratory; design specification | Sovereign production delivery |
| Retirement | When Wave 1 native apps ship to Jennifer's Mac | Indefinite |

A document created in the prototype opens without conversion in the native app. The
prototype IS the format spec; whatever file it writes must be readable by the native app
from day 1.

---

## 8. Non-goals

- No `connect-src 'none'` sovereignty enforcement (prototype only)
- No seL4 CNode isolation per surface (prototype only)
- No Apple notarization or code-signing
- No production multi-user support; single operator (Jennifer) on the WireGuard PPN
- Not a replacement for the native apps — retired when Wave 1 ships to Jennifer's Mac
- Not a public web service; accessible only via WireGuard PPN
