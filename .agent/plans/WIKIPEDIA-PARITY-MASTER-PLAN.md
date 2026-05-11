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

## 2. To-Do List (The Road to +98%)

### [ ] Infrastructure
- [ ] Verify `templates/` can be safely deleted (confirmed: Maud macros in `src/server.rs` are the source of truth).
- [ ] Implement CSS Variable injection for global theme management.

### [ ] Content Rendering (`src/render.rs`)
- [ ] Update Infobox/Navbox renderers to emit standard MediaWiki-style tables.
- [ ] Ensure `inject_edit_pencils` matches Wikipedia's `[edit]` link placement and styling.

### [ ] UI Chrome (`src/server.rs`)
- [ ] Refactor `wiki_chrome` to match Vector 2022's header/sidebar/content wrapper structure.
- [ ] Implement "Read / Edit / View history" as a standard `.vector-menu-tabs` container.
- [ ] Update the language switcher to match the "Global" button layout in Vector 2022.

### [ ] Client-side Interactivity (`static/wiki.js`)
- [ ] Add Keyboard shortcut listener.
- [ ] Implement "TOC Pinning" (shifting content when TOC is fixed).
- [ ] Smooth scrolling with proper offset for the sticky header.

---

## 3. Success Metrics
- **Side-by-side Visual Test:** 0px deviation in layout alignment between Wikipedia and `app-mediakit-knowledge`.
- **Shortcut Test:** All standard Wikipedia shortcuts work.
- **Performance Test:** Page load < 100ms (Leapfrog 2030 target).
