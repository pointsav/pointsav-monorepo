# WIKIPEDIA-PARITY-MASTER-PLAN.md — Achieving +98% Muscle Memory

> **Objective:** Bridge the gap between the Rust-based `app-mediakit-knowledge` engine and the MediaWiki (Wikipedia) Vector 2022 skin experience.
> **Constraint:** Maintain "flat software" (Markdown-backed) and "Leapfrog 2030" (Rust, performance, no legacy baggage) standards.
> **Target:** 100% visual and behavioral match for the end-user.

---

## 1. The Strategy: "Skin-Deep Parity, Rust-Deep Performance"

We do not use MediaWiki code. We use MediaWiki's **Interface Contract**. By emitting the exact same DOM (IDs and Classes) and CSS variables as Wikipedia, we achieve parity without the PHP/SQL/jQuery overhead.

### Phase 1: DOM Standardisation (The Naming Pass)
Update `src/server.rs` (Maud macros) and `src/render.rs` to move from custom classes to standard MediaWiki/Vector 2022 classes.
- `.wiki-body` -> `.mw-body`
- `.wiki-article` -> `.mw-content-ltr`
- `.wiki-toc` -> `.vector-toc`
- Standardize IDs: `#site-header`, `#mw-content-text`, etc.

### Phase 2: Design Token Port (The Visual Pass)
Audit `static/style.css` against Wikipedia's **Codex** design system.
- Port Wikipedia Blue (`#36c`), Red (`#ba0000`), and Purple (`#6b4ba1`).
- Match typography: Serif for body, Sans for chrome.
- Mirror the 960px limited-width content discipline.
- Port Infobox/Navbox specific CSS structures.

### Phase 3: Interaction Parity (The Muscle Memory Pass)
Expand `static/wiki.js` to include "hidden" Wikipedia behaviors.
- **Keyboard Shortcuts:** `/` for search focus, `e` for edit (when authorized).
- **TOC Pinning:** Implement the Vector 2022 "pinned" state vs "collapsed" state.
- **AJAX Previews:** Refine the existing hover-card logic to match Wikipedia's timing and layout.

### Phase 4: Leapfrog 2030 Integration
Maintain and enhance the non-Wikipedia features that provide the "Leapfrog" advantage.
- **Research Trails:** Collapsible data-pedigree footer.
- **Citation Ribbons:** Visual verification indicators.
- **Doorman AI:** Integrated edit assistance.
- **Git Backing:** Transparent version control without a DB.

---

## 2. Status — ALL THREE PHASES SHIPPED (2026-05-12)

### [x] Phase 1 — DOM Standardisation — COMPLETE (`3b557cf`, Peter, 2026-05-12)
7 structural class/ID renames to MediaWiki/Vector 2022 names across `server.rs`,
`style.css`, `wiki.js`. CSS custom properties seeded (`--mw-*` aliases). 60/60 tests.

### [x] Phase 2A — Design Token Port — COMPLETE (`68c643c`, Jennifer, 2026-05-12)
Article typography regression fixed (`.page-body` selector). 9 hardcoded hex colors ported
to CSS variables. 4 body-level hardcoded colors ported. 60/60 tests.

### [x] Phase 3 — Interaction Parity — COMPLETE (`3cee49d`, Jennifer, 2026-05-12)
Keyboard shortcuts (`?` overlay, `Esc` close), AccessKey attributes, TOC pin button with
localStorage, AJAX page navigation (fetch + DOM swap + loading bar). 60/60 tests.

**Stage 6 pending** — Sprints G–K (`fa47611`, `11ea232`, `416437d`) + Phases 1/2A/3 above
need `bin/promote.sh` from Command Session. All commits signalled in outbox.

---

## 3. Success Metrics — achieved
- DOM matches Vector 2022 class/ID contract ✓
- CSS variables match Codex token names ✓  
- Keyboard shortcuts (`?` help, accesskeys) ✓
- TOC pinning with localStorage persistence ✓
- AJAX navigation with loading bar ✓
- Page load well under 100ms (Rust binary) ✓

## 4. Next work after Stage 6 deploy
Phase 4 implementation continues at Steps 4.6 (MCP server via rmcp) and 4.7
(git smart-HTTP remote). Steps 4.1–4.5 are all shipped. See
`app-mediakit-knowledge/docs/PHASE-4-PLAN.md` for the full step table.
