---
artifact: brief
schema: foundry-brief-v1
title: BRIEF — Workplace Development Roadmap
status: active
created: 2026-05-31
cluster: project-workplace
language_protocol: PROSE-ARCHITECTURE
purpose: >
  What to code next, in what order. The iteration planning document.
  Updated after each session; session log carries the living history.
supersedes:
  - BRIEF-workplace-http-prototype.md
  - BRIEF-workbench-refactoring-roadmap.md
  - BRIEF-workplace-software-suite.md (§5, §7–§10)
related: BRIEF-workplace-architecture.md
---

# BRIEF — Workplace Development Roadmap

## 1. HTTP Prototype — Current State

**Crate:** `app-workplace-http-prototype`
(`pointsav-monorepo/app-workplace-http-prototype/`)

**Stack:** Rust + axum HTTP server; frontend assets embedded via `rust-embed`; single
binary; fully offline-capable on the PPN.

**Access:** `http://10.8.0.9:9200` via WireGuard PPN (nginx proxy; prototype binds
`0.0.0.0:9110`; configurable via `WORKPLACE_PROTO_PORT`).

**File storage:** `/home/jennifer/workbench/` (configurable via `WORKPLACE_PROTO_WORKSPACE`).
No database; files are the state. TOML config (`config.toml`) with roots model:
`sandbox-jennifer` (writable), `_command`, `_clones`.

**Routing:**
```
/             surface selector (open existing file or create new)
/workbench/   app-privategit-workbench SPA (3575-line VS Code-style editor)
/memo         Memo surface
/proforma     Proforma surface
/presentation Presentation surface
/schedule     Schedule surface
/code         Code surface
/pdf          PDF surface
/gis          GIS surface
/bim          BIM surface
/api/files    file listing + CRUD endpoints (axum JSON API)
/api/events   SSE endpoint — inotify watcher; broadcasts file changes
```

**CSP note:** The prototype intentionally does NOT enforce `connect-src 'none'`. That
sovereignty rule applies to the native Tauri apps only. SYS-ADR-07 still applies:
Proforma, Schedule, and GIS data must not transit `service-slm` inference even in
the prototype.

**The prototype IS the design specification.** Every UX decision validated here feeds
directly into `BRIEF-workplace-architecture.md` and the native app implementations.
A document created in the prototype opens without conversion in the native app.

---

## 2. Stage Build Plan

Each stage = one surface reaching **usable threshold** (Jennifer can do real work with
it: open, create, edit, save). Jennifer requests the next stage when the current one
is usable.

```
Stage 1: Memo — [x] base complete (2026-05-28) + [x] Session 1 enhancements (2026-06-03)
  crate created; axum 0.7 + rust-embed 8; surface selector + memo.html
  Base toolbar: B I H1 H2 H3 • Save Open▾; Ctrl+S save; 30s auto-save
  Session 1 (commit 3768ba89, Stage 6 pending):
    toolbar: U, S̶, P, 1., ≡L/≡C/≡R, clr, ☀/🌙 theme toggle
    wp-theme localStorage (shared with proforma); html.light CSS block
    word count in status bar (debounced 50ms): filename · N words · N chars
    paste sanitization: allow-list strip; insertHTML into undo stack
    crash recovery: memo-crash:<path> localStorage draft; recovery banner
    placeholder CSS (:empty::before); ol styling; extended keyboard shortcuts
  Memo improvement sessions 2–4 still ahead (Outline, Find/Replace, Export)

Stage 2: Proforma — [x] complete + enhanced (2026-06-02)
  proforma-v1.0 JSON schema; custom grid; /proforma route; commit a444266b
  Click-to-select, double-click/keypress to edit; Tab/Enter/Esc/Arrow nav
  Formula evaluator: =SUM/AVERAGE/AVG/MIN/MAX/COUNT(range), cell refs, arithmetic
  AutoSum Σ button (toolbar + Alt+=) — detects run above/left, inserts SUM formula
  en-CA locale numbers; add/remove row+col; auto-save 3s
  proforma-v2.0 schema: entity/date/analyst metadata; editable col labels; per-col format
  Light/dark theme toggle (☀/🌙); wp-theme localStorage key shared across surfaces

Stage 3: Presentation — [ ] not started
  Open an existing .json slide deck
  Create a new presentation
  Add/remove slides; add text and image to a slide; reorder slides
  Save
  Technology: Custom slide canvas (HTML/CSS/JS)

Stage 4: Schedule — [ ] not started
  Open an existing .json schedule
  Create a new schedule
  Add tasks with names and durations; indent/outdent for WBS hierarchy
  Connect tasks with finish-to-start dependencies
  View as Gantt bars on a date axis
  Save
  NOT a calendar — no appointment or event concept
  Muscle memory: MS Project; secondary Primavera P6
  Technology: Custom Gantt + WBS

Stage 5: Code — [ ] not started
  Browse workspace file tree
  Open a code file (any extension)
  Edit with syntax highlighting
  Save
  Technology: CodeMirror 6 (MIT)

Stage 6: PDF — [ ] not started
  Open a .pdf file from the workspace
  Page navigation (prev/next/jump to page)
  Zoom in/out
  Technology: PDF.js (Apache 2.0)

Stage 7: GIS — [ ] not started
  Open a .geojson file
  Render features on a MapLibre GL map viewport
  Layer list (show/hide)
  Technology: MapLibre GL JS + OpenFreeMap tiles

Stage 8: BIM — [x] schema complete (2026-06-02)
  Proper W3C DTCG format: $schema URI, flat $value/$type tokens, $extensions for
  non-token data (visibility, project metadata); backward compat for old files
  Element-styles editor: color swatch + visible toggle per IFC type (8 defaults)
  Live DTCG JSON preview with syntax colouring (incl. $extensions key)
  Ctrl+S + auto-save 3s; /api/bim/files + /api/bim/create; bim/ workspace dir auto-created
  IFC viewer follow-up: @thatopen/components 3D render + element tree panel
```

After all 8 stages reach usable threshold: begin **native-parity pass** — align each
HTTP surface with the corresponding native Tauri app spec in
`BRIEF-workplace-architecture.md`.

---

## 3. Session Log (newest on top)

```
2026-06-03/04 — Totebox@claude-code: Workbench drag-drop + undo (Sessions 5/6)
  Drag-to-move: POST /move added to app-privategit-workbench (port 9210); binary deployed.
  Frontend: wireDragOnItem() wired in wireItem(); .dragging/.drag-over/.drag-over-editor CSS;
  doWbMoveFile() calls /_api/edit/move; drag-to-open on #viewer pane calls openFile().
  Bug fix: handler was in wrong service (port 9110 prototype, not port 9210 write service);
  /_clones/ draggable guard removed (all _clones/project-* roots are writable=true).
  Undo: moveHistory[] stack (cap 10); showWbToast(msg, undoFn) upgraded with 6s Undo button;
  undoLastMove() reverses last move via doWbMoveFile(from, name, origDir, isUndo=true);
  Ctrl+Z shortcut gated on !isEditing (won't intercept text editor undo).
  Commits: d451dcd2 + 7870683f + 6866eb3a (all Stage 6 pending).
  Plan saved (deferred): Memo save location chooser at /home/jennifer/.claude/plans/.

2026-06-03 — Totebox@claude-code: Memo Session 1 — toolbar + theme + word count + paste + crash recovery
  Plan created by 3 Opus Explore agents (codebase inventory + shared infrastructure + web research).
  Toolbar completions: U (underline), S̶ (strikethrough), P (normal text), 1. (ordered list),
  ≡L/≡C/≡R (alignment), clr (clear formatting). Extended updateToolbarState() for all 9 new buttons.
  Light/dark toggle: anti-flash <script>, wp-theme key shared with proforma, html.light CSS overrides.
  Word count: live debounced 50ms in status bar — "filename · N words · N chars".
  Paste sanitization: DOMParser + allow-list strips inline styles and disallowed tags.
  Crash recovery: localStorage draft saved every 2s; recovery banner on next load if draft is newer.
  Placeholder: :empty::before CSS renders the data-placeholder attribute.
  Keyboard: Ctrl+Shift+7 (OL), Ctrl+Shift+X (S̶), Ctrl+E (≡C), Ctrl+Shift+L/R, Ctrl+\ (clr).
  Build verified clean. Prototype restarted. Commit 3768ba89 (Stage 6 pending).

2026-06-02/03 — Totebox@claude-code: Proforma enhancements + BIM DTCG schema fix
  BIM schema fixed to proper W3C DTCG: $schema URI, flat tokens, $extensions for
  visibility + project metadata. Backward compat: old bim-workspace-v1.0 still loads.
  Colorizer: $extensions highlighted (gold); hex color regex fixed (was missing quotes).
  Proforma schema v2.0: adds entity/date/analyst metadata subbar; editable column header
  labels (click to rename); per-column format badge (cycles T→$→%→# on click); currency
  display uses accounting negatives (1,234). Rust skeleton updated to v2.0 on create.
  Proforma light/dark toggle: ☀/🌙 button in toolbar; html.light CSS overrides (white
  background, dark text, blue selection); wp-theme localStorage key for future shared use.
  Jennifer feedback: dark mode was hard to see — prompted the light mode toggle.
  Proforma formula functions: AVERAGE, AVG, MIN, MAX, COUNT added to evalExpr via
  evalRange() helper. AutoSum Σ button (toolbar) + Alt+= shortcut: detects numeric run
  above (column sum) or left (row sum); inserts =SUM() formula automatically; falls back
  to opening edit with = if no adjacent numbers.
  Commits: dfb07944 (BIM DTCG, promoted 5aa88c3f), 8d8049c6 (proforma v2.0, promoted
  4a7e3499), 683fc671 (theme toggle, Stage 6 pending), 3ffaa8f6 (formula/AutoSum, Stage 6 pending).

2026-06-02 — Totebox@claude-code: Leapfrog 2030 Phase 1+2 — keyboard + design tokens
  Competing Opus agents audited http://10.8.0.9:9200 across design coherence, 2030 vision,
  and keyboard-first power user mechanics. Implemented Phases 1+2 in one session.
  Phase 1 — Keyboard quick wins:
    memo.html: Ctrl+B/I/U (bold/italic/underline) + Ctrl+Alt+1/2/3/0 (headings/para)
    workbench: Ctrl+1..9 surface switching (Files=1...BIM=9)
    proforma: Ctrl+Z/Y undo/redo (50-entry ring buffer); Ctrl+C/V clipboard; Ctrl+D fill-down
      with relative formula ref rewriting
  Phase 2 — Design system tokenization:
    style.css: --wp-* foundation (16 palette tokens, 7-step spacing, 6-step type, z-index map)
    Graphite bronze --wp-accent: #c89a4a replaces VS Code derivative #007acc
    .wp-btn / .wp-btn--primary / .wp-btn--ghost / .wp-btn--danger unified button system
    workbench: bronze rail on active surface (inset 0 3px 0 0 #c89a4a); light-mode updated
  Committed 6ae5e97c. Stage 6 pending.
  Phase 3 (stack pane model, Desk surface, status line) deferred — multi-session scope.

2026-06-01 — Totebox@claude-code: Stage 8 BIM schema — bim-workspace-v1.0 DTCG format
  Schema: $schema/project(title+ifc-file)/element-styles using W3C DTCG $value/$type.
  File ext: .bim.json stored in workspace bim/ dir. 8 default IFC element types.
  bim.html: two-panel layout — left = element styles table (color swatch + visible),
  right = live DTCG JSON preview with syntax colouring. Ctrl+S + 3s auto-save.
  Backend: /bim route, /api/bim/files GET, /api/bim/create POST.
  Workbench: renderBimSurface() panel wired. Stage 6 pending.
  Also: BRIEF Stage 2 status corrected (was "not started"; complete since a444266b).

2026-05-31 — Totebox@claude-code: Light/dark theme toggle
  Added ☀/🌙 toggle button to sidebar toolbar (next to A-Z sort). CSS override block
  (~140 rules) appended to <style>; dark is default; html.light class activates light
  palette. localStorage persistence (wb-theme). Anti-flash <script> in <head> applies
  class before render. Markdown viewer stays white in both modes (was already light).
  Code/editor shifts to #f8f8f8 + VS Code Light syntax colours in light mode.
  Committed cb44f3b1; deployed 2026-05-31T20:09Z.

2026-05-31 — Totebox@claude-code: SSE file-watch reload — proper fix
  Root cause: inotify watcher only covered writable roots; absolute paths from inotify
  didn't match tab.path (root-relative); mtime was hardcoded to 0. Fix: watch ALL roots
  including _clones/; convert absolute → root-relative path in watcher event loop; emit
  real mtime. Frontend polling reduced 4s → 30s safety net (SSE is now primary).
  Binary replaced at /srv/foundry/cargo-target/jennifer/release/; started from crate dir
  so config.toml is found; "workbench roots: 3" confirmed at startup.
  Commits: c7efdd1c (source). Deployed 2026-05-31T19:30Z.

2026-05-31 — Session consolidation: five active BRIEFs consolidated into three.
  BRIEF-workplace-architecture.md (new): vision + app inventory + IPC + drift check.
  BRIEF-workplace-roadmap.md (this file): prototype stages + refactoring + distribution.
  BRIEF-workplace-desktop-environment.md (updated): naming drift fixed.
  Archived: leapfrog-2030, workbench-refactoring-roadmap, software-suite, http-prototype.

2026-05-30 — Totebox@claude-code: SSE live file-change notifications
  Path-aware broadcast from put_file; /workbench/events endpoint; EventSource IIFE
  in workbench frontend; dirty-tab banner; 5s reconnect. Also: sidebar Recent section
  (global MRU list below Pinned; 10 default / 50 cap; right-click to pin or remove);
  file tree timestamps + sort-by-name/date/size; file tree truncation + pin stability.
  Commits: 962df2d2, c0748993 and nearby.

2026-05-30 — Totebox@claude-code: Workbench merge into prototype at /workbench/
  app-privategit-workbench SPA (3575-line VS Code-style editor) + full backend ported
  into app-workplace-http-prototype as src/workbench.rs. Nested at /workbench/; base href
  injected at serve time. TOML config (config.toml) with roots model: sandbox-jennifer
  (writable), _command, _clones. AppState extended with roots/weasyprint/max_bytes;
  watcher extended to writable roots. /files/ nginx path and standalone workbench at 9210
  untouched pending Jennifer's confirmation. Files tile added to surface selector → /workbench.
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
  Commit acc10fd7.
```

---

## 4. Jennifer's Feedback Loop

Jennifer is the operator and primary user of the prototype. The feedback loop:

1. Jennifer uses the prototype for real work at `http://10.8.0.9:9200`
2. When a surface is usable enough, or when something needs fixing, she opens a session
   and says: **"work on the next stage of [surface]"** or **"fix [specific behaviour]"**
3. The session targets one surface at a time; BRIEF stage status is updated at session end
4. Feedback that changes a UX decision is reflected in `BRIEF-workplace-architecture.md`
   (edit the relevant section) and noted in §3 Session Log above

---

## 5. app-privategit-workbench — Improvement Phases

`app-privategit-workbench` is owned by `project-development` (Jennifer's scope). Work
is tracked here because the prototype hosts it at `/workbench/` and this cluster's
sessions drive the daily iteration. Handoffs to project-development go via outbox.

### Immediate: File-watch reload (inbox 2026-05-31, from project-orgcharts)

**Bug:** When a document is open in Workbench via `/_api/edit/document?path=...` in an
iframe, external writes (Claude Code, text editor) are invisible until the file is
closed and reopened. Workbench serves its in-memory cached version, not the disk state.

**Workaround applied in project-orgcharts:** `doSave()` in drag-editor HTML writes
through to disk via `PUT /_api/edit/file` (with `X-Foundry-Editor: 1`) in addition to
the `postMessage({type:'wb-save-content'})`. Keeps disk and memory in sync for
drag-editor saves. Does not fix the reverse direction (Claude Code → disk → Workbench
still requires close/reopen).

**Fix options (pick one):**
1. **File-watch reload (preferred)** — when a document is open via `/_api/edit/document`,
   watch the underlying file with inotify `IN_MODIFY`; push a reload signal to the iframe
   via SSE or WebSocket. iframe reloads from disk automatically when Claude Code saves.
2. **Reload-from-disk on focus/refresh** — on window focus event or user refresh,
   re-fetch via `GET /_api/edit/file`; compare mtime to cached version; if newer,
   reload the iframe. Simpler than inotify; slightly less responsive.

### Phase 1: Frontend Componentization (medium-term)
- [ ] Transition `index.html` monolith to a structured component framework (SolidJS/WebComponents)
- [ ] Implement a reactive state store to replace manual DOM manipulations
- [ ] Decouple File Tree, Viewer, and Editor panes into independent, state-reactive components

### Phase 2: Shell/Renderer Decoupling (medium-term)
- [ ] Implement `app-workplace-launcher` as the unified orchestrator (see architecture BRIEF §8)
- [ ] Define JSON-Schema-based Toolbar Manifests for each tool surface
- [ ] Wire the IPC handshake (`OpenDocument`, `ToolbarManifestInjection`) between Shell and Renderer

### Phase 3: Institutional Hardening (longer-term)
- [ ] Formalize `system-mba` UDS contract for renderer process authentication and audit logging
- [ ] Implement structured error handling and localized logging surfaced to the Workbench Shell
- [ ] Integrate per-app capability set visibility

---

## 6. Tauri v1 → v2 Migration (Sprint 0 — native apps)

Applies to `app-workplace-memo`, `app-workplace-proforma`, `app-workplace-presentation`.
Must complete before the launcher `Hello` handshake can be validated against a real child.

- `tauri migrate` CLI for config migration (top-level `tauri` key → `app`;
  `allowlist` → `src-tauri/capabilities/`)
- CSP: `connect-src 'none'` everywhere; `ipc:` pseudo-origin is local-only (not a
  network relaxation — Tauri v2 IPC requirement)
- Budget: 10–20 engineer-days total across three apps (2–5 days each)

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
Signing: Apple Developer certificate + notarization; local dev builds skip signing.

### Long-term: AArch64 Linux (native `os-workplace` image)

The macOS near-term ship is the correct Day 1 answer for reaching customers before
`os-workplace` is production-ready. Long-term canonical target is AArch64 Linux as a
native component of the `os-workplace` image. Both are real; different timelines.

---

## 8. app-workplace-schedule — Product Scope (P4.5)

Gantt / CPM / resource-scheduling for construction PMs and SMB project managers.

**MS Project market window:** Plan 5 end-of-sale 2026-05-01 (past); Project Online
retires 2026-09-30. Structural market window open for AArch64-native, EUPL 1.2,
perpetual-licence Gantt tool with MS Project key bindings + MPX/PMXML round-trip.

**Canonical format: `.json`** — versioned schema; plain text; git-diffable; 100-year-
readable; Ed25519-signable baselines as git-tagged commits for FIDIC §20.1
contemporaneous-record requirements. No retroactive baseline edits; UI requires an
explicit "new baseline" action. TaskJuggler DSL (`.wpsched`) is export/import only —
never the fiduciary record.

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

**Renderer:** native egui Gantt widget (rectangles, 90° elbow arrows, `resvg` for
PDF/PNG export; no WebView dependency). Never DHTMLX or Bryntum (commercial, breaks
EUPL substrate).

**Keyboard default:** MS Project bindings. P6 binding profile as selectable preference.

**SYS-ADR-07 hard rule:** schedule data is structured; must not transit `service-slm`.

---

## 9. Open Questions

- Pairing-server port for `system-gateway-mba`: TBD — check pairing-server deployment
- xeokit commercial license quote from Creoox: operator action, gates `app-workplace-bim`
  Scaffold-coded → Active
- macOS notarization certificate: needed for Wave 1 distribution
- `app-workplace-code` priority scheduling: parallel track; does not block Suite Sprint 1
- v0.5 BFS attribute substrate decision: Haiku BFS port vs Btrfs/ZFS xattrs + indexed query
  daemon (deferred to v0.5 ratification)
- `/files/` nginx path (port 9210 standalone workbench): pending Jennifer's confirmation
  on retirement
