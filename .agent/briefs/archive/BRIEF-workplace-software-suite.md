---
artifact: brief
schema: foundry-brief-v1
title: BRIEF — Workplace Software Suite
status: archived
superseded_by:
  - BRIEF-workplace-architecture.md
  - BRIEF-workplace-roadmap.md
created: 2026-05-27
cluster: project-workplace
language_protocol: PROSE-ARCHITECTURE
source_briefs:
  - BRIEF-app-workplace-architecture.md (§1–§9 + §11)
  - BRIEF-workplace-desktop-suite.md (superseded)
supersedes: BRIEF-workplace-desktop-suite.md (full; app-delivery concerns)
related: BRIEF-workplace-desktop-environment.md
---

# BRIEF — Workplace Software Suite

## 1. Mission

The `app-workplace-*` suite is a document-centric workbench environment, branded to
users as **Workbench**. There is **one program in the dock** — `app-workplace-launcher`.
When the user opens a file, the launcher spawns the appropriate renderer process for
that file's schema (Memo toolbar for `.html`, Gantt toolbar for a schedule `.json`,
spreadsheet toolbar for a proforma `.json`). Individual renderer processes are invisible
to the user; Workbench is the only surface they ever see. Each renderer remains a
separate OS process underneath, preserving seL4 CNode isolation and crash containment
per app type.

All canonical file formats are open, flat, and standard: `.html`, `.json`, `.pdf`,
`.ifc`, `.md`. No proprietary binary format is ever the fiduciary record.

This BRIEF governs what gets built, in what order, how tools connect to each other and
to OS services, and how they distribute. Desktop environment (compositor, TUI shape, MBA
topology, clipboard daemon) is in `BRIEF-workplace-desktop-environment.md`.

**Jennifer Woodfine (`jwoodfine`) is actively developing `app-privategit-workbench`,
the HTTP browser-based three-column developer tool (file tree / viewer / code), in
`project-development`. That product is separate from this suite — see §6 for the
detailed product distinction.**

**HTTP prototype:** While native Tauri builds require a macOS host, an HTTP prototype
(`app-workplace-http-prototype`) provides daily-use access to all 8 surfaces in a
browser today, running on the VM via WireGuard PPN. UX decisions validated there feed
directly into this BRIEF. See `BRIEF-workplace-http-prototype.md`.

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
is ever the fiduciary record. Export to legacy formats (XLSX, PPTX, MPX) is
permitted as exchange exhaust; these are never the source of truth.

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

## 3.5 Canonical File Format Registry

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

## 4. IPC Model

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
- ~~`MbaSocketFd { fd }`~~ — **removed** (per §11.7 of architecture BRIEF; MBA is an OS-tier contract; see below)

### MBA contract (separate)

`MbaSocketFd` is not in the launcher IPC contract. Apps connect directly to
`/run/foundry/system-mba.sock` at startup. The launcher holds no WireGuard FD and
passes none to children. Full MBA contract spec: `BRIEF-workplace-desktop-environment.md` §7.

**Moonshot note:** the launcher IPC contract is designed as a stable OS-level ABI from
day 1; when the launcher is absorbed into `os-workplace` as a system service, the
`app-workplace-*` binaries connect to the same socket at the same path under a system
service, unchanged. This absorption applies only to the launcher; `app-workplace-code`
(coding IDE) is a user-tier application and is not a candidate for OS absorption.

---

## 5. Tauri v1 → v2 Migration (Sprint 0)

Applies to `app-workplace-memo`, `app-workplace-proforma`, `app-workplace-presentation`.
Must complete before the launcher `Hello` handshake can be validated against a real child.

- `tauri migrate` CLI for config migration (top-level `tauri` key → `app`;
  `allowlist` → `src-tauri/capabilities/`)
- CSP: `connect-src 'none'` everywhere; `ipc:` pseudo-origin is local-only (not a
  network relaxation — Tauri v2 IPC requirement)
- Budget: 10–20 engineer-days total across three apps (2–5 days each per Software Architect)

---

## 6. Three surfaces sharing the "workbench" vocabulary — disambiguation

Three distinct products. Do not conflate them. Naming is now resolved.

| | `app-workplace-launcher` | `app-workplace-code` | `app-privategit-workbench` |
|---|---|---|---|
| **Product name** | **Workbench** (the suite; one program in the dock) | Code surface | PrivateGit Workbench |
| **What** | Document-centric launcher; spawns renderer processes for each file schema | Native desktop coding IDE; VS Code analogue | HTTP browser-based three-column developer tool |
| **Stack** | Tauri v2; IPC socket; file-schema dispatch | Tauri v2 + WebView; CodeMirror 6; LSP; integrated terminal | HTTP server; browser-accessible; file tree / viewer / code panes |
| **Persona** | All Workbench users (it IS the product) | Developers building PointSav software | Totebox Orchestration community users |
| **Owner** | `project-workplace` (this archive) | `project-workplace` (this archive) | `project-development` (Jennifer's scope) |
| **State** | Not in registry; create crate; P1 | Not in registry; parallel track | Actively in development; Jennifer is the operator |
| **Jennifer's role** | Primary end user | Not involved | **Active developer** |
| **Registry** | Needs new crate at `app-workplace-launcher/` | Needs new crate at `app-workplace-code/` | Managed by project-development |

This cluster does NOT own `app-privategit-workbench`. Handoffs route to project-development's outbox.

---

## 7. Distribution

### Near-term: macOS x86_64 via project-software

Wave 1 entries to add to `binary-targets.yaml` in project-software:
- `app-workplace-launcher` — platforms: `[x86_64-apple-darwin]`
- `app-workplace-memo` — platforms: `[x86_64-apple-darwin]`
- `app-workplace-presentation` — platforms: `[x86_64-apple-darwin]`

Wave 2 (after foundation matures):
- `app-workplace-proforma`, `app-workplace-pdf`, `app-workplace-gis`, `app-workplace-schedule`

Build platform: macOS (Intel or Apple Silicon). Not cross-compiled from Linux.
Signing: Apple Developer certificate + notarization for distribution; local dev builds skip signing.

### Long-term canonical: AArch64 Linux (native `os-workplace` image)

The macOS near-term ship is the correct Day 1 answer for reaching customers before
`os-workplace` is production-ready. The long-term canonical target is AArch64 Linux as
a native component of the `os-workplace` image. Both are real; different timelines.

Connectivity: configurable endpoint; WireGuard PPN default `10.42.20.x`; Doorman SLM (9092); proofreader (9097).

---

## 8. app-workplace-schedule — Product Scope (P4.5)

`app-workplace-schedule` is a Gantt / CPM / resource-scheduling app for construction
PMs and SMB project managers. Slots at Priority 4.5.

**MS Project market window:** Plan 5 end-of-sale 2026-05-01 (past); Project Online
retires 2026-09-30. Structural market window open for AArch64-native, EUPL 1.2,
perpetual-licence Gantt tool with MS Project key bindings + MPX/PMXML round-trip.

**Canonical format: `.json`** — versioned schema; plain text; git-diffable;
100-year-readable (Doctrine claim #2); Ed25519-signable baselines as git-tagged commits
for FIDIC §20.1 contemporaneous-record requirements. No retroactive baseline edits; UI
requires an explicit "new baseline" action. TaskJuggler DSL (`.wpsched`) is a
human-readable export/import format only — never the fiduciary record.

**Day-1 feature set (10 items):**
1. Gantt chart view (bars on date axis)
2. WBS hierarchy with indent/outdent (Alt+Shift+Right/Left)
3. Finish-to-start dependency with lag/lead time
4. Drag-to-reschedule task bars (cascade through dependencies)
5. PDF export (landscape, configurable date range)
6. MPX import + PMXML import + PMXML export (contract delivery format)
7. WBS-based critical-path calculation (CPM — longest dependency chain)
8. Baseline snapshot (planned-vs-actual stacked bars)
9. Print to paper (11×17 clean; sticky-notes-on-wall replacement)
10. Resource = subcontractor named-only assignment

**Renderer:** native egui Gantt widget (rectangles, 90° elbow arrows, `resvg` for PDF/PNG
export; no WebView dependency). Fallback only: vendored frappe-gantt (MIT, 50KB, SVG)
replaced in v1. Never DHTMLX or Bryntum (commercial, breaks EUPL substrate).

**Keyboard default:** MS Project bindings. P6 binding profile as selectable preference.

**SYS-ADR-07 hard rule:** schedule data is structured; must not transit `service-slm`
inference (same as proforma formula evaluation).

---

## 9. Wave 1 Exit Criteria

1. Launcher + `app-workplace-memo` handshake end-to-end (`Hello` + `OpenDocument` + `Heartbeat`)
2. Document opens via launcher's shared file picker
3. Memo opens, edits, saves, and re-opens a document correctly
4. macOS `x86_64-apple-darwin` build clean for launcher + Memo
5. Smoke test: connect Memo to WireGuard PPN endpoint via `system-mba.service`; Doorman responds

---

## 10. Open Questions

- Pairing-server port for `system-gateway-mba`: TBD — check pairing-server deployment
- xeokit commercial license quote from Creoox: operator action, gates `app-workplace-bim` Scaffold-coded → Active
- macOS notarization certificate: needed for Wave 1 distribution
- `app-workplace-code` priority scheduling: parallel track; does not block Suite Sprint 1
