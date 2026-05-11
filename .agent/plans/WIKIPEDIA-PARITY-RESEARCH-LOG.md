# WIKIPEDIA-PARITY-RESEARCH-LOG.md — Muscle Memory Audit

> Detailed log of MediaWiki Vector 2022 characteristics to be ported to `app-mediakit-knowledge`.

---

## 2026-05-11 — Initial Audit (MediaWiki Vector 2022)

### DOM Naming Convention (Standard MW)
| Component | MediaWiki ID/Class | Current app-mediakit-knowledge |
|---|---|---|
| Page Body Wrapper | `.mw-body` | `.wiki-main` |
| Content Wrapper | `#mw-content-text` | `.wiki-article` |
| Site Header | `.mw-header` | `.site-header` |
| Navigation Menu | `.vector-main-menu` | `.wiki-nav-portlet` |
| TOC Wrapper | `.vector-toc` | `.wiki-toc` |
| Sidebar (Left) | `#mw-panel` | `.wiki-left-rail` |
| Page Actions (Top) | `#p-views` | `.wiki-action-tabs` |

### Visual Tokens (Colors & Typography)
- **Wikipedia Blue:** `#36c` (Links), `#3366cc` (Tabs).
- **Wikipedia Red:** `#ba0000` (Redlinks).
- **Background Gray:** `#f8f9fa` (Chrome/TOC).
- **Border Gray:** `#a2a9b1`.
- **Typography:** 
  - Article Body: `serif` (Georgia, "Times New Roman").
  - Headers/Chrome: `sans-serif` (-apple-system, BlinkMacSystemFont, "Segoe UI").
  - Content Font Size: `0.875rem` (14px) baseline.

### Interaction Behaviors
1. **Keyboard Shortcuts:**
   - `/`: Focus search input.
   - `Alt+Shift+F`: Focus search input.
   - `Alt+Shift+E`: Edit page.
   - `Alt+Shift+H`: View history.
2. **TOC "Pinning":**
   - In Vector 2022, the TOC can be "pinned" to the left rail.
   - When pinned, the main content container shifts to the right to avoid overlapping.
   - When unpinned, it collapses into a menu.
3. **Sticky Header Behavior:**
   - Appears after scrolling past the main title.
   - Contains the article title, search, and edit tools.
   - Implementation in `wiki.js` (Sprint H) is a good start but needs visual refinement.

### Rendering Components
- **Infoboxes:** Must be `table.infobox` with specific styles for headers (`th.infobox-above`), images (`td.infobox-image`), and data rows (`th.infobox-label` / `td.infobox-data`).
- **Navboxes:** Must be `div.navbox` with `table.nowraplinks` and collapsible logic (`initNavboxes` in `wiki.js`).
- **Hatnotes:** Must be `div.hatnote` with 1.6em indentation and italics.

---

## Next Research Steps
- [ ] Audit `src/render.rs` Infobox parser.
- [ ] Compare search result page with Wikipedia's Special:Search.
- [ ] Analyze "Talk" page threading (MediaWiki's new Discussion Tools vs our Sprint C4 implementation).
