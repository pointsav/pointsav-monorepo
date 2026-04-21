# CHANGELOG

All notable changes to Workplace✦Presentation are documented in this file.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning: [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Repository scaffold: CLAUDE.md, NEXT.md, ROADMAP.md, CLEANUP_LOG.md, ARCHITECTURE.md, DEVELOPMENT.md, README.md (bilingual EN/ES), LICENCE (EUPL-1.2), CHANGELOG.md.
- Build config: Makefile, package.json, .gitignore.
- Tauri v1.7 configuration: src-tauri/Cargo.toml (with [workspace] opt-out), src-tauri/tauri.conf.json, src-tauri/src/main.rs (four IPC commands copied from memo pattern).
- **Phase 1:** shell forked from app-workplace-memo. Four IPC commands, `download-deps.sh`, `embed-fonts.sh`, and licence header copied unchanged. App-identity fields updated in Cargo.toml, tauri.conf.json, and package.json. PointSav gold chrome tokens carried over from memo's `src/styles/app.css`. Three-pane HTML shell (navigator | canvas | code-view placeholders), flat menubar, status bar. Icons generated from placeholder gold `icon-source.png` via `npx tauri icon`. `make dev` launches a window titled "Workplace Presentation" with three visibly distinct panes and no console errors.
- **Phase 2:** blank slide canvas — `src/js/schema.js` (document model: `newDocument()`, `newSlide()`, `newElement()`; three registered layouts with Blank as default), `src/js/canvas.js` (active slide renderer at 1100×850 logical units, CSS-transform letterbox scaling, click-to-insert text box with viewport→logical coord translation, contenteditable blur commits content), `src/js/editor.js` (document state, dirty flag, keyboard wiring — Enter-inside-textbox commits/blurs, Enter-outside adds a new slide, Left/Right arrows navigate slides, Escape blurs, Ctrl+S logs a placeholder for Phase 5). Default typography Source Sans 3, 24pt body. US Letter landscape per ADR-PR-09. Status bar counter updates with navigation. Scripts wired into `src/index.html`; canvas styling added to `src/styles/app.css`.
- **Phase 3:** slide navigator — `src/js/slides.js` renders live thumbnails from the document model (scaled 0.145× via CSS transform), highlights the active slide with a PointSav-gold border, and supports click-to-jump, native HTML5 drag-and-drop reorder, and a right-click context menu (Duplicate / Delete / New Slide After). `schema.js` gains `cloneSlide()`. `editor.js` wires navigator render into `renderAll()` and adds keyboard shortcuts: Ctrl+M (new slide after active), Ctrl+D (duplicate active slide). Navigator chrome added to `app.css`.

### Pending
- Phase 4: split-screen code view (see NEXT.md).
- Phases 5–7: per ROADMAP.md.

---

## [0.1.0] — Not yet released

Initial internal release target: after Phase 7 (print pipeline) completes.
