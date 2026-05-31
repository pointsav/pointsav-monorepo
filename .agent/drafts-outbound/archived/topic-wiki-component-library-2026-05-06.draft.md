---
schema: foundry-draft-v1
state: ready-for-sweep
language_protocol: TOPIC
originating_cluster: project-design
target_repo: vendor/content-wiki-documentation
target_path: content/
target_filename: topic-wiki-component-library.md
audience: editorial
bcsc_class: vendor-public
bilingual: true
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z (24-agent research pass).
  ARIA Authoring Practices Guide (APG) — landmark, dialog, navigation patterns.
  Wikipedia article layout analysis (title, TOC, header, footer patterns).
  MDN HTML: native <dialog>, inert attribute.
research_inline: true
---

# Wiki Component Library

The PointSav wiki component library defines nine reusable interface units that together render a complete wiki article page. Each component targets Wikipedia's established layout muscle-memory while applying modern accessibility standards and the PointSav token system.

---

## Architecture overview

A wiki article page is composed from a stack of components:

```
┌─────────────────────────────────────────────────────┐
│  wiki-drawer-mobile-nav  (compact only — hamburger) │
├──────────────────────────────────────────────────────┤
│  wiki-article-header  (breadcrumb + H1 + badge + meta)│
├──────────────────┬──────────────────────────────────┤
│                  │  wiki-toc-sidebar  (right rail)  │
│  Article body    │  (desktop only)                  │
│  (prose)         ├──────────────────────────────────┤
│                  │  wiki-badge-tag (quality/category)│
├──────────────────┴──────────────────────────────────┤
│  wiki-article-footer  (categories + refs + edit)     │
├─────────────────────────────────────────────────────┤
│  wiki-pagination  (prev/next article in category)    │
└─────────────────────────────────────────────────────┘

Overlays (triggered by user action):
  wiki-modal-dialog     — image lightbox, search overlay
  wiki-dark-mode-toggle — persistent theme switch (site header)

Search:
  wiki-search-results   — results list (own page or in modal)
```

---

## Components

### wiki-article-header

Top-of-article surface. Renders four elements in order: (1) slug breadcrumb showing the article's category path, (2) the article H1 title from frontmatter, (3) an optional quality badge, and (4) a byline showing last-edited date, editor name, and a history link.

Quality grades: Featured Article (gold), Good Article (green), A-class (blue), B-class (light blue), C-class (grey), Stub (light grey). Ungraded articles omit the badge slot.

**Token:** `--ps-wiki-text-h1` = 2.25rem (36px at 17px base).

---

### wiki-article-footer

Bottom-of-article surface. Three sections: (1) category tag links (each category the article belongs to), (2) numbered references/citations list using `<ol id="ref-N">` for in-article back-links, and (3) an edit-on-GitHub link for contributors.

---

### wiki-toc-sidebar

Sticky right-rail sidebar listing article headings (H2 and H3). On desktop (≥800px) it is `position: sticky; top: 1rem`. On compact (≤799px) it collapses to an inline `<details>`/`<summary>` toggle above the article body.

Active section is highlighted by JavaScript using `IntersectionObserver`. The observer watches all `[id]` heading anchors; when a heading enters the viewport, its TOC entry receives `aria-current="true"` and the active visual style.

---

### wiki-search-results

Ordered list of search hits returned by the Tantivy search engine. Each hit displays: article title (as a link) and a plain-text excerpt (~180 characters, truncated at word boundary with ellipsis). The component wraps in `aria-live="polite"` so screen readers announce result count updates.

**API shape:** `POST /mcp`, JSON-RPC 2.0, method `search`. Response: `{query, count, hits: [{slug, title, snippet}]}`. No HTML in snippets — plain text only. A zero-results state is toggled with the `[hidden]` attribute.

---

### wiki-modal-dialog

Overlay using the native `<dialog>` element with `showModal()`. Native `<dialog>` provides a built-in focus trap — all content outside the dialog is unreachable by keyboard until the dialog closes. Used for image lightboxes, the search overlay, and confirmation prompts. Dismisses on Escape (native browser behaviour) and on backdrop click.

---

### wiki-dark-mode-toggle

A toggle button that sets `data-theme="dark"` on `<html>` and persists the user choice in `localStorage` under key `ps-theme`. On each page load, an inline script reads this value before the stylesheet renders, preventing a flash of the wrong theme. Falls back to `prefers-color-scheme: dark` if no explicit choice has been stored.

---

### wiki-badge-tag

Dual-purpose chip: (1) article quality grade badge (non-interactive `<span>` with `aria-label` providing the full grade name), and (2) category tag links (`<a>` chips). Used in the article header (quality badge) and article footer (category tags).

---

### wiki-pagination

Prev/Next article navigation within a category. The component is a three-column grid: previous article link (left), current category name link (centre), next article link (right). Each directional link carries `rel="prev"` / `rel="next"` for SEO, and an `aria-label` providing the full adjacent article title for screen readers.

This is not numbered-page pagination. Numeric pagination (for search results, category listings) is a separate planned component (`wiki-page-list`, not yet specced).

---

### wiki-drawer-mobile-nav

Slide-in left-panel navigation for compact (≤799px) viewports. A hamburger trigger button opens the drawer. When open, the `inert` attribute is applied to the page's main content region and header, locking keyboard focus inside the drawer until it is dismissed. Closes on Escape or backdrop click.

Browser support: `inert` is natively supported in Chrome 102+, Firefox 112+, Safari 15.5+. A conditional WICG polyfill (~3KB gzipped) covers older browsers.

---

## Token dependency

All nine components draw exclusively from the PointSav token system defined in `dist/tokens.css`. No component introduces raw colour or dimension values. New wiki pages and templates add components without adding new CSS variables.

---

## Research trail

### Done
- Component list and roles confirmed by project-editorial brief (24-agent research pass 2026-05-06).
- ARIA patterns verified per APG: nav landmark, dialog, aria-live, aria-current, aria-pressed.
- Tantivy search API shape confirmed from project-knowledge source code (2026-05-06).
- inert browser support verified: ~94% native, WICG polyfill ~3KB (training data, Aug 2025).

### Suggested
- Document page-level HTML structure (which element wraps what) in a companion GUIDE once the Zola template layout is finalised.

### Open questions
1. **wiki-page-list component** — numbered pagination for search results and category listings is not yet specced. Needed before launching a category index page.
