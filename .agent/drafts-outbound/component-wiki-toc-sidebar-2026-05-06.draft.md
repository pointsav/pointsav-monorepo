---
schema: foundry-draft-v1
state: committed
committed_ref: 61fc430
language_protocol: DESIGN-COMPONENT
originating_cluster: project-design
target_repo: vendor/pointsav-design-system
target_path: dtcg-vault/components/wiki-toc-sidebar/
target_filename: recipe.json
audience: design-system
bcsc_class: vendor-internal
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 1
open_questions_count: 0
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z.
  Wikipedia Table of Contents: sticky sidebar at wide breakpoint, collapsed at compact.
  scroll-driven active-section highlight via IntersectionObserver.
research_inline: true
---

# DESIGN-COMPONENT — wiki-toc-sidebar

Sticky table of contents sidebar, rendering article heading anchors. Active section highlighted as the user scrolls. At compact breakpoints, collapses into the `ps-wiki-drawer` (separate component).

## Structure

```html
<nav class="ps-wiki-toc" aria-label="Table of contents">
  <h2 class="ps-wiki-toc__heading">Contents</h2>
  <ol class="ps-wiki-toc__list">
    <li class="ps-wiki-toc__item">
      <a class="ps-wiki-toc__link ps-wiki-toc__link--active"
         href="#{{section-id}}" aria-current="location">{{section-title}}</a>
      <ol class="ps-wiki-toc__sublist">
        <li class="ps-wiki-toc__item">
          <a class="ps-wiki-toc__link" href="#{{subsection-id}}">{{subsection}}</a>
        </li>
      </ol>
    </li>
  </ol>
</nav>
```

## Tokens needed

- `--ps-surface-elevated` — sidebar card background
- `--ps-border-subtle` — sidebar left border (2px accent on active)
- `--ps-interactive-primary` — active section left border accent
- `--ps-wiki-link` — link color
- `--ps-ink-secondary` — non-active link
- `--ps-wiki-font-body` — IBM Plex Sans
- `--ps-space-4`, `--ps-space-5` — item padding
- `--ps-viewport-wide` — media query for sticky display

## Positioning

```css
@media (min-width: 1152px) {
  .ps-wiki-toc {
    position: sticky;
    top: var(--ps-space-7);
    max-height: calc(100vh - var(--ps-space-7));
    overflow-y: auto;
  }
}
@media (max-width: 799px) {
  .ps-wiki-toc { display: none; } /* replaced by ps-wiki-drawer */
}
```

## Active section tracking

JS-driven via `IntersectionObserver` on all `h2`, `h3`, `h4` in the article body. When a heading enters the viewport, the corresponding TOC link receives `ps-wiki-toc__link--active` and `aria-current="location"`.

## ARIA

- `<nav aria-label="Table of contents">` — distinct landmark from the site navigation.
- Active link carries `aria-current="location"` (not `page` — TOC is in-page navigation).
- Nested `<ol>` for `h3` entries under an `h2` group.

## Research trail

### Done
- Wikipedia TOC anatomy cross-referenced (numbered levels, sticky sidebar, active highlight).
- `IntersectionObserver` active-section pattern documented (no requestAnimationFrame polling needed).
- Compact breakpoint collapse to `ps-wiki-drawer` planned (cross-component dependency noted).
- `aria-current="location"` verified as correct for in-page anchor navigation (WCAG 1.3.1, ARIA 1.2).

### Suggested
- Verify max nesting depth — Wikipedia TOC supports 4 levels; 2 levels (h2/h3) is sufficient for most PointSav wiki articles.
