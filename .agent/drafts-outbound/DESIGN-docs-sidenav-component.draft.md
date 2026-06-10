---
schema: foundry-draft-v1
artifact_type: DESIGN-COMPONENT
language_protocol: DESIGN-COMPONENT
status: staged
created: 2026-06-01
created-by: totebox@project-knowledge
destination: project-design → pointsav-design-system
component_metadata:
  component_name: docs-sidenav
  carbon_baseline: "none — closest is IBM Carbon Side Nav, but that pattern carries icon slots, multi-level nesting, and enterprise-scale rail width (256px fixed). This component is deliberately simpler: no icons, single-level categories with article lists inside <details>/<summary>, 256px width via --sidenav-w token. Departure from Carbon is intentional."
  accessibility_targets:
    - "nav landmark with aria-label='Documentation navigation'"
    - "<details>/<summary> sections are keyboard-navigable; chevron rotates on open state via CSS only (no JS)"
    - "Active link indicated by border-left and font-weight, not colour alone"
    - "Hidden below 1024px via display:none — no content is sidenav-exclusive (all content reachable via breadcrumb and article)"
research_done_count: 2
research_suggested_count: 1
open_questions_count: 2
research_provenance: "Stripe docs nav, Vercel docs nav, Cloudflare docs nav — all surveyed live 2026-06-01. Carbon Side Nav reviewed and explicitly rejected (too heavy). Pattern derived from implementation in app-mediakit-knowledge commit 914cd836."
research_inline: true
paired_with: "not-required — developer-facing design system recipe, not public editorial content"
source_commit: "914cd836 (pointsav-monorepo, app-mediakit-knowledge)"
---

# DESIGN-COMPONENT: docs-sidenav

Persistent left navigation column for product-documentation surfaces. Renders a
two-level hierarchy: uppercase category headings as `<details>` sections, article
links as `<a>` elements inside. Active page highlighted with border-left accent.
Sticky-scrolls independent of article content. Collapses to hidden below 1024px.

---

## HTML recipe

```html
<nav class="docs-sidenav" aria-label="Documentation navigation">
  <div class="docs-sidenav__inner">

    <!-- One <details> per category -->
    <details class="docs-sidenav__cat" open>
      <summary>Infrastructure</summary>
      <ul class="docs-sidenav__list">
        <li>
          <a href="/wiki/infrastructure/worm-ledger-design"
             class="docs-sidenav__link is-active"
             aria-current="page">
            WORM Ledger Design
          </a>
        </li>
        <li>
          <a href="/wiki/infrastructure/other-article"
             class="docs-sidenav__link">
            Other Article
          </a>
        </li>
      </ul>
    </details>

    <details class="docs-sidenav__cat">
      <summary>Applications</summary>
      <ul class="docs-sidenav__list">
        <li>
          <a href="/wiki/applications/app-privategit-workbench"
             class="docs-sidenav__link">
            PrivateGit Workbench
          </a>
        </li>
      </ul>
    </details>

  </div>
</nav>
```

**Notes:**
- The category whose active article belongs to should receive `open` on
  `<details>` — the server adds this attribute server-side based on the
  current slug's category.
- `aria-current="page"` goes on the active article link only.
- Category label text is the humanized category name (title-case, acronym-aware).
- The `<nav>` sits as the first column child in a CSS grid shell
  (`grid-template-columns: var(--sidenav-w) minmax(0, 1fr)`).

---

## CSS

```css
/* Token ------------------------------------------------------------------ */
:root {
  --sidenav-w: 256px;   /* docs left navigation column width */
}

/* Shell grid (parent) ---------------------------------------------------- */
.shell {
  display: grid;
  grid-template-columns: var(--sidenav-w) minmax(0, 1fr);
  align-items: start;
}

/* Sidenav container ------------------------------------------------------ */
.docs-sidenav {
  position: sticky;
  top: var(--header-h);
  align-self: start;
  height: calc(100vh - var(--header-h));
  overflow-y: auto;
  overscroll-behavior: contain;
  padding: 24px 22px 48px 0;
  border-right: 1px solid var(--rule);
  scrollbar-width: thin;
}

.docs-sidenav__inner { display: flex; flex-direction: column; gap: 2px; }
.docs-sidenav__cat   { margin-bottom: 4px; }

/* Category heading (the <summary>) --------------------------------------- */
.docs-sidenav__cat > summary {
  list-style: none;
  cursor: pointer;
  font-family: var(--font-display, var(--font-body));
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.09em;
  text-transform: uppercase;
  color: var(--fg-3);
  padding: 8px 10px 6px;
  border-radius: 7px;
  user-select: none;
  display: flex;
  align-items: center;
  gap: 6px;
}
.docs-sidenav__cat > summary::-webkit-details-marker { display: none; }
.docs-sidenav__cat > summary::before {
  content: "›";
  display: inline-block;
  font-size: 13px;
  color: var(--fg-4);
  transition: transform 0.15s ease;
}
.docs-sidenav__cat[open] > summary::before { transform: rotate(90deg); }
.docs-sidenav__cat > summary:hover { color: var(--fg-1); }

/* Article link list ------------------------------------------------------ */
.docs-sidenav__list { list-style: none; margin: 2px 0 8px; padding: 0; }
.docs-sidenav__list li { margin: 0; }

.docs-sidenav__link {
  display: block;
  padding: 6px 10px 6px 24px;
  border-radius: 7px;
  font-size: 13.5px;
  line-height: 1.35;
  color: var(--fg-2);
  text-decoration: none;
  border-left: 2px solid transparent;
  transition: background 0.12s ease, color 0.12s ease;
}
.docs-sidenav__link:hover  { background: var(--bg-hover); color: var(--fg-1); }
.docs-sidenav__link.is-active {
  color: var(--link, var(--navy));
  font-weight: 600;
  background: var(--bg-subtle);
  border-left-color: var(--link, var(--navy));
}

/* Responsive collapse ---------------------------------------------------- */
@media (max-width: 1023px) {
  .shell          { grid-template-columns: minmax(0, 1fr); }
  .docs-sidenav   { display: none; }
}
```

---

## ARIA checklist

| Requirement | Implementation |
|---|---|
| Landmark | `<nav aria-label="Documentation navigation">` |
| Active page | `aria-current="page"` on active `<a>` |
| Keyboard: open/close category | Native `<details>`/`<summary>` — Enter/Space work without JS |
| Active state not colour-only | `border-left` accent + `font-weight: 600` alongside colour change |
| Hidden from screen readers below breakpoint | `display: none` at ≤1023px removes from accessibility tree; article still reachable via breadcrumb |
| Scroll overflow | `overflow-y: auto` on nav itself; page scroll independent |

---

## Design decisions

**`<details>`/`<summary>` over JS accordion.** Category expand/collapse is
a progressive-enhancement concern. The native elements work without JS,
are keyboard-navigable by default, and have correct ARIA semantics
(`aria-expanded` is implicit). The chevron animation is CSS-only via the
`[open]` attribute selector.

**No icons.** Stripe, Vercel, and Cloudflare docs all use icon slots in
their left navs for brand identity. This component omits them: the content
corpus is a documentation wiki, not a branded product surface, and icons
would require per-article icon assignments that don't exist in the current
content schema.

**`--sidenav-w: 256px` as a token.** Width extracted to a custom property
so the shell grid and any future sidenav-aware components (e.g. a mobile
drawer) share the same measurement without hardcoding.

**Active category auto-expanded server-side.** The server sets `open` on
the `<details>` whose category matches the current article's category slug.
This avoids the flash-of-collapsed-nav that JS-based lazy expansion causes
on initial page load.

---

## Open questions for project-design

1. **Mobile drawer vs. hidden.** Currently the sidenav is `display: none`
   below 1024px. Should a future sprint add a hamburger-triggered drawer at
   the mobile breakpoint, or is the breadcrumb sufficient navigation for
   mobile readers?
2. **Sub-navigation depth.** The current implementation supports one level
   of hierarchy (category → articles). If content warrants section-level
   grouping within a category (e.g. "Applications → Console apps → ..."),
   what is the maximum depth before the pattern should change?
