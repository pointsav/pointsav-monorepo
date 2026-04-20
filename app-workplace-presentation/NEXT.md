# NEXT.md — What to do right now

> The live action list. Update at the end of every session. Newest items at top.

---

## Immediate — Phase 3: Slide navigator

Phase 2 shipped a blank canvas with text-box-on-click and keyboard navigation. Next is the left pane — live thumbnails, click-to-jump, drag-to-reorder, right-click context menu. No Phase 4 (code view) or Phase 5 (save-to-disk) yet.

### Paste this into Claude Code to start Phase 3:

```
Phase 3 task: slide navigator with live thumbnails and reordering.

Read CLAUDE.md and ROADMAP.md Phase 3 section.

Build src/js/slides.js per the specification. Wire it into
src/index.html's left pane (replacing the Phase 1 placeholder).

Thumbnail rendering strategy: clone the slide DOM, scale with CSS
transform. Do not maintain separate thumbnail DOMs — one source of
truth.

Drag-and-drop: use native HTML5 drag-and-drop (dragstart, dragover,
drop). No library.

Context menu: vanilla JS. Right-click opens a small positioned div
with Duplicate / Delete / New Slide After.

Ask before deviating from this list.

Commit as: feat(slides): navigator with thumbnails and reorder

---

End-of-phase housekeeping (run before concluding the session):

When the work above is verified running, perform these file updates
in a single separate commit before ending the session:

1. CHANGELOG.md — under [Unreleased] → Added, append a bullet for
   Phase 3 describing what shipped (slides.js, live thumbnail
   navigator, drag-and-drop reorder, right-click context menu with
   duplicate/delete/new-slide-after).

2. NEXT.md — replace the "Immediate — Phase 3" block with the
   Phase 4 commission prompt from ROADMAP.md (split-screen code
   view). Append the same end-of-phase housekeeping block to that
   prompt, updated for Phase 4. Move the Phase 3 record into the
   session log at the bottom with today's date.

3. CLAUDE.md — update the "Last updated" line at the top to today's
   date. Update "Current phase" to Phase 4. No other changes.

4. Commit the doc updates as:
   docs: update tracking for Phase 3 completion

Ask before deviating from this list.
```

---

## After Phase 3 is verified running

Claude Code performs the end-of-phase housekeeping above as part of the Phase 3 session. This file will already contain the Phase 4 commission prompt when the next session begins — no manual edit required.

---

## Resolved decisions — 2026-04-19

- **Slide aspect ratio:** **US Letter landscape (11″ × 8.5″, 1.294:1).** Not 16:9. Rationale: target audience (bankers, asset managers) prints every deck on US Letter paper. The deck *is* the handout; projector use is secondary. Tradeoff: black bars on modern 16:9 projectors — acceptable. See ADR-PR-09.
- **Default fonts:** Source Sans 3, 24pt body / 40pt title. Locked.
- **Slide layouts at ship:** Three only — **Title / Content / Blank**. **Blank is the startup default** (proforma discipline: no template pops up at launch). Two-Column moves to post-Phase-7 backlog.
- **Code view:** Ships in Phase 4 as planned. Plain monospace text, no syntax highlighting. The code view is an institutional feature — it lets a banker see their own file's raw HTML as proof of ownership. Highlighter deferred; revisit if feedback requests it.

---

## Deferred — track, do not action

- macOS build verification. Owner's iMac is on 10.13 High Sierra. Linux Mint is the primary dev target. macOS builds happen when the iMac is upgraded.
- Tauri v2 migration. Coordinated across all three workplace apps. Not this project.
- Real icon artwork. Currently using a solid gold square as `icon-source.png`. A commissioned grid-of-cells motif (proforma) and document-with-fold motif (memo) are the family direction; presentation gets a slide-stack motif in PointSav gold when commissioned.

---

## Session log

### 2026-04-20 — Phase 2 complete: blank slide canvas
- Added `src/js/schema.js` (document model with `newDocument()`, `newSlide()`, `newElement()`, three layouts registered and Blank as the startup default), `src/js/canvas.js` (renders active slide at 1100×850 logical units, CSS-transform scaling so the centre pane letterboxes cleanly on any window size, click-to-insert text box with viewport→logical coord translation, contenteditable blur commits content), `src/js/editor.js` (state, active-slide index, dirty flag, keyboard wiring, `insertTextBox`, `markDirty`).
- Wired the three scripts into `src/index.html` at the end of `<body>` in order (schema → canvas → editor). Emptied the Phase 1 canvas-area placeholder. Added `data-status` hooks on status-bar spans so the slide counter can be updated without brittle `firstElementChild` lookups.
- Extended `src/styles/app.css` with `.slide-stage`, `.slide-canvas` (white sheet with transform-origin top-left and a dark drop shadow), and `.slide-element` (24pt Source Sans 3 default, PointSav-gold focus ring). Flipped `#canvas-area` overflow from `auto` to `hidden` so a fractional-pixel fit miscalc can't trigger scrollbars.
- Keyboard model chosen: inside a focused text box, Enter commits and blurs (PowerPoint title-cell behaviour); outside any editable, Enter adds a new slide after the active one. This matches the "Enter twice → new slide" cadence in the ROADMAP — first Enter commits, second Enter creates. Shift+Enter inside a text box inserts a line break as usual. Arrow keys navigate slides only when no contenteditable has focus.
- Verified with `make dev` on Linux Mint 22. Blank US Letter landscape slide renders letterboxed on the dark desktop, click inserts a text box, typing works, Enter commits and then Enter adds a new slide, arrow keys navigate, status bar counter updates.
- Environment note: needed `source ~/.bashrc` to load nvm and `PKG_CONFIG_PATH` (webkit 4.0→4.1 shim) — the terminal had opened as a login shell and skipped the rc file. Not a project issue; shell-config nuance.
- One new deferred item logged in CLEANUP_LOG.md: element selection, deletion, and move/resize (post-Phase-7, alongside undo/redo).
- Not committed this session (skipped by user request).

### 2026-04-19 — Phase 1 complete: shell forked from memo
- Forked unchanged from app-workplace-memo: `src-tauri/src/main.rs` (four IPC commands), `scripts/download-deps.sh`, `scripts/embed-fonts.sh`, `docs/licence-header.txt`.
- Adapted with app-identity changes only: `src-tauri/Cargo.toml` (package name + `[workspace]` opt-out verified), `src-tauri/tauri.conf.json` (productName, identifier `com.pointsav.workplace.presentation`, window title), `package.json` name field.
- PointSav gold chrome tokens copied from memo `src/styles/app.css`; memo-specific layout rules removed.
- `src/index.html` scaffolded as three-pane shell: 200px left pane (navigator placeholder), flex centre pane (canvas placeholder), right pane hidden by default (code view placeholder). Flat menubar (File / Home / Insert / Design / Slide Show / View). Status bar showing "Slide 1 of 1 · 100%".
- Icons generated from placeholder gold `icon-source.png` via `npx tauri icon`.
- `make setup` and `make dev` verified. Window launches titled "Workplace Presentation". Three panes visibly distinct. No console errors.
- Committed as: `chore(init): fork from app-workplace-memo — Phase 1 shell`.

### 2026-04-19 — Project scaffolded + design decisions locked
- Created repo scaffold: CLAUDE.md, NEXT.md, ROADMAP.md, CLEANUP_LOG.md, ARCHITECTURE.md, DEVELOPMENT.md, README.md (bilingual), LICENCE, CHANGELOG.md, Makefile, package.json, .gitignore, src-tauri/Cargo.toml, src-tauri/tauri.conf.json, src-tauri/src/main.rs.
- Four design decisions resolved: aspect ratio = US Letter landscape (ADR-PR-09), fonts = Source Sans 3 24/40pt, three ship layouts (Title/Content/Blank), code view ships plain without highlighter.
- No source frontend code written yet. That is Phase 1 work.
- Phase 1 commission prompt drafted.
