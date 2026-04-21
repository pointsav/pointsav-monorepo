# CLAUDE.md — Workplace✦Presentation

> Project memory for Claude Code. Auto-loads on every session launch from this repo root.
> Last updated: 2026-04-21
> Current phase: Phase 4 — split-screen code view (see NEXT.md for the commission prompt)

---

## What this project is

**Workplace✦Presentation** is the third app in the PointSav workplace family. It is a sovereign, offline-first desktop presentation editor that replaces PowerPoint. It outputs a single self-contained `.html` file as its native format — openable in any browser as a runnable slideshow, printable to PDF as a handout, editable by any text editor in fifty years.

Sibling apps:
- `app-workplace-memo` — document editor, HTML output ✅ running on Linux Mint
- `app-workplace-proforma` — spreadsheet editor, JSON output ✅ running on Linux Mint
- `app-workplace-presentation` — **this project**, presentation editor, HTML output

All three share engineering stack, chrome tokens, IPC patterns, and licence. UX evolves independently.

---

## Hard rules — do not negotiate

1. **Tauri v1.7 only.** Not v2. The owner's iMac runs macOS 10.13 High Sierra; v2 requires 10.15+. Migration to v2 happens in parallel across all three workplace apps when the constraint lifts — not piecemeal.
2. **EUPL-1.2 licence header on every source file.** European Union Public Licence. Use the header block in `docs/licence-header.txt`.
3. **CSP `connect-src 'none'`.** No component of this app reaches the network. Ever. Enforced in `src-tauri/tauri.conf.json`.
4. **The file is the product.** The `.html` file saved by this app must be complete, self-contained, and independently runnable. No accompanying files. No external dependencies. No proprietary wrappers. Base64-embed fonts. Inline all CSS and JavaScript. Metadata lives in a `<meta>` tag.
5. **Match memo's IPC pattern.** Rust IPC in `src-tauri/src/main.rs` exposes exactly four commands: `open_file`, `save_file`, `get_app_data_dir`, `read_font_file`. Copy unchanged from memo. Do not add more without explicit instruction.
6. **Cargo workspace opt-out.** `src-tauri/Cargo.toml` ends with an empty `[workspace]` table. This crate is not a member of the monorepo workspace. Standing pattern for all `app-workplace-*` crates.
7. **Blank canvas on startup.** One blank slide at **US Letter landscape (11″ × 8.5″, logical canvas 1100×850)**. See ADR-PR-09. Three layouts registered — Title, Content, Blank — with Blank as the default at launch. No preloaded template. No layout gallery at startup. No sample content. The demo motion requires a blank slide on first launch — same discipline as proforma's 26×50 blank grid.
8. **No framework churn.** Vanilla JS, no React, no Vue, no build step for the frontend. Plain HTML/CSS/JS served directly by Tauri from `src/`.
9. **PointSav gold (`#c8a96e`) is the accent colour.** Dark chrome. Match memo and proforma tokens — CSS custom properties defined in `src/styles/app.css`.

---

## Architecture snapshot

```
app-workplace-presentation/
├── CLAUDE.md              ← this file
├── NEXT.md                ← what to do right now
├── ROADMAP.md             ← the 7-phase plan
├── CLEANUP_LOG.md         ← running log of deferred work
├── ARCHITECTURE.md        ← ADRs
├── DEVELOPMENT.md         ← setup guide (includes webkit shim)
├── README.md              ← bilingual EN/ES, standing project requirement
├── LICENCE                ← EUPL-1.2
├── CHANGELOG.md
├── Makefile               ← make setup / dev / build / audit / icons
├── package.json           ← Tauri v1 CLI only
├── src/
│   ├── index.html         ← three-pane shell: navigator | canvas | code-view
│   ├── styles/app.css     ← dark chrome, PointSav gold
│   └── js/
│       ├── editor.js      ← document state, file I/O, menu wiring
│       ├── slides.js      ← slide navigator (left pane)
│       ├── canvas.js      ← active slide canvas (centre)
│       ├── codeview.js    ← HTML source pane (right, toggleable)
│       ├── slideshow.js   ← F5 fullscreen runtime — also embedded in saved file
│       ├── export.js      ← HTML assembly, font embedding, SHA-256 seal
│       ├── print.js       ← Paged.js landscape handout pipeline
│       ├── fonts.js       ← font panel, @font-face injection
│       ├── schema.js      ← document structure + newDocument()
│       ├── font-data.js   ← placeholder, replaced by embed-fonts.sh
│       └── vendor/paged.polyfill.js  ← placeholder, replaced by download-deps.sh
├── src-tauri/
│   ├── Cargo.toml         ← Tauri v1.7 + [workspace] opt-out
│   ├── tauri.conf.json    ← window, CSP, allowlist
│   ├── src/main.rs        ← 4 IPC commands
│   └── icons/             ← icon-source.png is master; derived formats gitignored
├── scripts/
│   ├── download-deps.sh   ← fetches Paged.js + 8 WOFF2 font families
│   └── embed-fonts.sh     ← base64-encodes fonts into src/js/font-data.js
├── fonts/                 ← downloaded, gitignored
└── docs/
    ├── print-pipeline.md
    ├── slideshow-runtime.md
    ├── split-code-view.md
    └── fonts.md
```

---

## The UX — PowerPoint familiarity, not parity

A PowerPoint user sits down and works within 30 seconds. No training.

| Expected from PowerPoint | Delivered here |
|---|---|
| Slide sorter on left with thumbnails | Yes |
| Active slide canvas in centre | Yes |
| Top menubar: File / Home / Insert / Design / Slide Show / View | Flat menubar, same names, no ribbon tabs |
| F5 starts slideshow | Yes |
| Ctrl+M inserts new slide | Yes |
| Status bar: slide number, zoom | Yes |
| Right-click slide → duplicate/delete | Yes |
| Drag-to-reorder slides | Yes |
| Format pane on right | Optional, toggleable — replaced by Code View when active |

**Removed on purpose:** Microsoft account bar, cloud indicators, co-authoring chrome, AI assistants, templates gallery at startup, "recent files" cloud sync.

---

## The split-screen code view — this app's unique feature

Memo deliberately dropped the markdown/code pane. Presentation brings a code pane back. This is an intentional UX divergence, not drift.

- **Toggle:** View menu → Split Code View, or `Ctrl+/`
- **Layout:** Left 50% canvas, right 50% syntax-highlighted HTML source
- **Scope:** The code pane shows the HTML of the **active slide only**, not the whole document
- **Sync:** Edits in either pane commit to the underlying document model on blur (not on every keystroke — avoids cursor thrash)
- **Why this exists:**
  1. Slides are structurally discrete (one `<section>` per slide) — raw HTML is tractable
  2. Power users want direct CSS/HTML control no ribbon can expose
  3. Reinforces commercial position — the user seeing their own raw HTML proves they own it

---

## The `.html` file format

Saved file structure:

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Presentation Title</title>
  <meta name="workplace-presentation-document"
        content='{"author":"...","created":"...","sha256":"...","version":1}'>
  <style>/* layout CSS + base64-embedded @font-face rules */</style>
  <script>/* minimal slideshow runtime — ~2KB — arrows, F for fullscreen, Esc */</script>
</head>
<body>
  <section class="slide" data-slide="1" data-layout="title">...</section>
  <section class="slide" data-slide="2" data-layout="content">...</section>
</body>
</html>
```

**Editor state** (current slide index, split-pane open/closed, zoom level) lives in app data, **not in the file**. The file is for the user; editor state is for the editor.

**The SHA-256 seal** in the meta tag covers everything between `<body>` and `</body>`. Tampering is detectable by re-running the hash. Same discipline as proforma's audit hash.

---

## Inheritance from memo and proforma — copy unchanged

These files are copied directly with minimal adjustment (product name, icon path, window title):

- `src-tauri/src/main.rs` — all four IPC commands
- `src-tauri/tauri.conf.json` — CSP, allowlist, bundle config (adjust `identifier` and `productName`)
- `src-tauri/Cargo.toml` — dependencies list + `[workspace]` opt-out
- `package.json` — Tauri CLI dev dependency
- `Makefile` — setup/dev/build/audit targets
- `.gitignore` — same exclusions
- `scripts/download-deps.sh` — Paged.js + fonts download
- `scripts/embed-fonts.sh` — base64 encoder
- PointSav gold chrome tokens in `src/styles/app.css`
- `docs/licence-header.txt` — EUPL-1.2 header block

---

## Commit convention

Conventional Commits. One concern per commit. Examples:

- `feat(canvas): insert text box on click`
- `feat(slides): drag-to-reorder in navigator`
- `feat(codeview): sync canvas → HTML on blur`
- `fix(export): escape HTML entities in slide titles`
- `chore(config): opt out of monorepo workspace`
- `docs(next): update after phase 2 completion`

Bug fixes reference the file. Architecture decisions reference the ADR.

---

## What not to do

- Do not add a framework. Vanilla JS.
- Do not add a build step for the frontend. Files in `src/` are served as-is.
- Do not add network calls. CSP forbids it.
- Do not store document content in app data. The file is the product.
- Do not add cloud sync, account login, or telemetry. Ever.
- Do not migrate to Tauri v2 until coordinated across all three sibling apps.
- Do not pull new dependencies without adding them to ARCHITECTURE.md with a rationale.
- Do not change the four IPC commands. If new functionality needs Rust, ask first.
