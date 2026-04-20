# ROADMAP.md — Workplace✦Presentation

> The full seven-phase plan. Each phase is a discrete, shippable milestone.
> Commission prompts for each phase are included — paste into Claude Code when ready.

---

## Phase 1 — Fork from memo

**Goal:** Inert shell that compiles and launches. No presentation features yet.

**Inherited unchanged:** Rust IPC (4 commands), Tauri config pattern, Cargo workspace opt-out, webkit shim (system-wide — already applied), EUPL-1.2 headers, PointSav gold chrome tokens, download-deps.sh, embed-fonts.sh, Makefile.

**New:** three-pane HTML shell (navigator | canvas | code-view placeholder), app identity (product name, icon, window title).

**Done when:** `make dev` launches a window titled "Workplace Presentation" showing three visibly distinct panes. No console errors. Compiled binary size roughly matches memo's.

**Commission prompt:** see `NEXT.md`.

---

## Phase 2 — Blank slide canvas

**Goal:** A single blank **US Letter landscape (11″ × 8.5″)** slide rendered in the centre pane. Click to add a text box. Type. Enter creates a new slide. Arrow keys navigate between slides.

**Canvas specification:**
- Logical dimensions: 1100 × 850 (100 dpi of 11″ × 8.5″). Math stays clean.
- Physical dimensions in CSS: `aspect-ratio: 11 / 8.5;` — scales to fit the centre pane with letterboxing where needed.
- Default typography: Source Sans 3, 24pt body, 40pt title.
- Default layout: Blank. Layout gallery does not appear at launch.

**New:**
- `src/js/schema.js` — `newDocument()` returns one blank US-Letter-landscape slide with empty `elements: []`. Three layouts registered: `title`, `content`, `blank`. Default = `blank`.
- `src/js/canvas.js` — renders the active slide. Click empty canvas → insert text box at click position. Text box is a `<div contenteditable>` with absolute positioning.
- Slide element model: `{ id, type: 'text' | 'image' | 'shape', x, y, width, height, content, style }`
- Keyboard: Enter on a blank canvas with no selection → new slide. Left/Right arrow keys → previous/next slide. Escape → deselect.
- `src/js/editor.js` — document state, active slide index, dirty flag.

**Done when:** Launch app → one blank US Letter landscape slide. Click → text box appears. Type "Revenue" → it's there. Press Enter twice → new slide appears. Arrow keys move between them. Ctrl+S saves (dialog — file write wired in Phase 5).

**Commission prompt:**
```
Phase 2 task: blank slide canvas with text-box-on-click.

Read CLAUDE.md, NEXT.md, and ROADMAP.md Phase 2 section first.

Build src/js/schema.js, src/js/canvas.js, src/js/editor.js per the
ROADMAP specification. Use the slide element model exactly as
specified. Wire the canvas into src/index.html's centre pane
(replacing the Phase 1 placeholder).

Aspect ratio: US Letter landscape (11 x 8.5 inches) — NOT 16:9.
See ADR-PR-09 for rationale (target audience prints on US Letter).
Canvas renders at 1100 x 850 logical units, scaled to fit the centre
pane with letterboxing.

Layouts: three registered — title, content, blank. Default = blank.
No layout gallery on launch. Startup is one empty blank slide.

Default typography: Source Sans 3, 24pt body, 40pt title.

Do not build the slide navigator yet — that is Phase 3.
Do not build the code view yet — that is Phase 4.
Do not wire save-to-disk yet — that is Phase 5. Ctrl+S can log to
console for now.

Commit as: feat(canvas): blank US Letter landscape slide with text-box-on-click
```

---

## Phase 3 — Slide navigator

**Goal:** Left pane shows live thumbnails of all slides. Click to jump. Drag to reorder. Right-click for duplicate/delete/new.

**New:**
- `src/js/slides.js` — thumbnail renderer (uses the same DOM as the canvas, scaled down via CSS transform), drag-and-drop reorder, context menu
- Navigator shows slide number beneath each thumbnail
- Active slide highlighted with PointSav gold left border (4px)
- Ctrl+M inserts new blank slide after the active one
- Ctrl+D duplicates the active slide

**Done when:** Three-slide deck. Thumbnails update live as canvas edits happen. Drag slide 3 above slide 1 → order changes. Right-click slide 2 → Duplicate → slide 3 is now a copy of 2.

**Commission prompt:**
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

Commit as: feat(slides): navigator with thumbnails and reorder
```

---

## Phase 4 — Split-screen code view

**Goal:** Toggle right pane open via View menu or Ctrl+/. Shows the HTML source of the active slide. Edits in either pane sync to the other on blur.

**New:**
- `src/js/codeview.js` — renders active slide's HTML as plain text in a `<textarea>` or contenteditable pre (no highlighting in Phase 4 — see NEXT.md deferred decision)
- Edits in code view: on blur, parse text as HTML, replace active slide's element DOM, re-render canvas and thumbnail
- Edits in canvas: on blur of any element, regenerate HTML text, replace code view contents
- Invalid HTML in code view: show a subtle warning strip at the top of the pane; keep the last valid state until the user fixes it

**Done when:** Toggle split view. Type "Revenue" in canvas → appears in code pane on blur. Edit `<div>Revenue</div>` to `<div>Net Revenue</div>` in code → canvas updates on blur. Break the HTML deliberately → warning strip; canvas keeps the last valid render.

**Commission prompt:**
```
Phase 4 task: split-screen code view.

Read CLAUDE.md and ROADMAP.md Phase 4 section.

Build src/js/codeview.js per the specification. Wire it into
src/index.html's right pane.

Toggle mechanism: View menu item "Split Code View" and keyboard
shortcut Ctrl+/. When active, right pane takes 50% of the horizontal
space; when inactive, canvas takes 100%.

Sync strategy: blur-driven, not keystroke-driven. On canvas element
blur, regenerate the active slide's HTML string and replace code
pane contents. On code pane blur, parse contents with
DOMParser('text/html'), validate it has a single root element,
replace slide's element DOM if valid, show warning strip if not.

No syntax highlighting in this phase. Ship as plain monospace text
first. See NEXT.md open decisions for the highlighter question.

Commit as: feat(codeview): split-screen HTML source with blur sync
```

---

## Phase 5 — Save as self-contained .html

**Goal:** File → Save writes a single `.html` file that opens standalone in any browser and runs as a slideshow with arrow keys.

**New:**
- `src/js/export.js` — assembles the saved file:
  - DOCTYPE, html, head with title and `<meta name="workplace-presentation-document">` containing JSON metadata
  - `<style>` block with slide layout CSS + base64 `@font-face` rules for embedded fonts
  - `<script>` block with the minimal slideshow runtime (~2KB — arrow keys, F for fullscreen, Escape to exit)
  - `<body>` with one `<section class="slide">` per slide
  - SHA-256 hash of the body, written back into the meta tag before final write
- Wire `save_file` IPC command from main.rs to the File → Save menu
- Ctrl+S triggers save
- Ctrl+Shift+S triggers Save As (file picker dialog)
- Dirty flag: window title shows "• Workplace Presentation — filename.html" when unsaved changes exist

**Done when:** Save a 3-slide deck as `demo.html`. Open `demo.html` directly in Firefox. Arrow keys navigate. F goes fullscreen. Esc exits. File opens in any text editor and is human-readable. SHA-256 verifies.

**Commission prompt:**
```
Phase 5 task: save as self-contained single-file .html

Read CLAUDE.md and ROADMAP.md Phase 5 section.

Build src/js/export.js per the specification. Wire File → Save and
File → Save As menu items, plus Ctrl+S and Ctrl+Shift+S.

Use the existing save_file IPC command in src-tauri/src/main.rs
unchanged. The JavaScript calls Tauri's invoke('save_file', { path, contents }).

Embed fonts by reading base64 data from src/js/font-data.js (populated by
scripts/embed-fonts.sh). Include only fonts actually used in the deck —
inspect each slide's text elements for font-family, build a set, embed
that set.

The slideshow runtime in the saved file must be independent of the
editor. When the file is opened in a browser, only the runtime loads.
No editor code leaks into the saved artefact.

SHA-256: compute over the <body>...</body> content after final
assembly, write hash into the meta tag. Use the Web Crypto API
(crypto.subtle.digest).

Commit as: feat(export): save as self-contained single-file .html
```

---

## Phase 6 — F5 slideshow mode

**Goal:** Press F5 → fullscreen slideshow using the same runtime embedded in saved files.

**New:**
- `src/js/slideshow.js` — the runtime. Single file. Used in two places: (a) inlined into saved files by `export.js`; (b) loaded by the editor when F5 is pressed to show the current in-memory deck.
- Fullscreen via `document.documentElement.requestFullscreen()`
- Arrow keys / PageUp / PageDown / Space navigate
- F toggles fullscreen
- Escape exits slideshow mode and returns to editor
- B blacks the screen; W whites it (standard PowerPoint conventions)

**Done when:** Edit a 3-slide deck. Press F5. Fullscreen. Right-arrow advances. Escape returns to edit mode with the deck intact.

**Commission prompt:**
```
Phase 6 task: F5 slideshow mode.

Read CLAUDE.md and ROADMAP.md Phase 6 section.

Build src/js/slideshow.js as a self-contained runtime that accepts a
deck object (array of slides) and a starting index. Expose a single
entry point: startSlideshow(deck, startIndex).

Same file is used by the editor (F5 in-memory) and by exported HTML
files (embedded in <script>). Keep the runtime under ~100 lines and
dependency-free — it must work when inlined into a saved file
with no other scripts present.

Keyboard: ArrowRight / ArrowDown / PageDown / Space → next. ArrowLeft
/ ArrowUp / PageUp → previous. F → fullscreen toggle. Escape → exit.
B → black screen toggle. W → white screen toggle.

Update export.js to inline slideshow.js contents into the saved file's
<script> block during Phase 5 export assembly.

Commit as: feat(slideshow): F5 fullscreen mode with shared runtime
```

---

## Phase 7 — Print pipeline

**Goal:** File → Print produces a handout with one slide per page, using Paged.js (same library memo uses for documents). Because the canvas is already US Letter landscape, there is no aspect-ratio conversion — slides print 1:1 at native size.

**New:**
- `src/js/print.js` — builds a print view: each slide becomes one `@page` at US Letter landscape
- CSS print stylesheet handles page breaks between slides
- Triggered by File → Print and Ctrl+P
- Opens a print dialog; user exports to PDF via the OS print dialog

**Done when:** 5-slide deck prints to PDF as 5 US Letter landscape pages, each page showing one slide at 1:1. No editor chrome in the PDF. Text and spacing match what the user sees on canvas.

**Commission prompt:**
```
Phase 7 task: print to PDF handout.

Read CLAUDE.md and ROADMAP.md Phase 7 section.

Build src/js/print.js per the specification. Reuse the Paged.js
library already present in src/js/vendor/paged.polyfill.js (downloaded
by scripts/download-deps.sh).

Print stylesheet: one @page per slide, landscape orientation, full
bleed by default. User can adjust margins via OS print dialog.

Wire File → Print menu and Ctrl+P. Open native print dialog via
window.print() after Paged.js has laid out the pages.

Do not embed fonts in the print output — the browser loads them from
the @font-face rules already in the document.

Commit as: feat(print): one-slide-per-landscape-page handout via Paged.js
```

---

## Post-Phase-7 backlog (not scheduled)

Logged in `CLEANUP_LOG.md` as they arise. Representative items:

- **Two-Column layout** (deferred from initial ship — 3 layouts at launch are Title/Content/Blank)
- **16:9 slide option** (deferred — US Letter landscape is the ship default per ADR-PR-09)
- Insert image from disk (Insert menu)
- Basic shapes — rectangle, ellipse, line
- Additional layouts: Comparison, Section Header, Quote, Image-with-caption
- Design themes (colour palette + font pair presets)
- Speaker notes pane (View → Notes)
- Slide transitions (subtle fade-in on next, no animation gimmicks — we are not PowerPoint)
- Syntax highlighting in code view (see NEXT.md — revisit if feedback requests)
- Ruler + guides on canvas for alignment
- Undo/redo stack
- Import from .pptx (future — significant undertaking, requires parsing Open XML)

None of these block Phase 1–7. Do not build them until commissioned.
