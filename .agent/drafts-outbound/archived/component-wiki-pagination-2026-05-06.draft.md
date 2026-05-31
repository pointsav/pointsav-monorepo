---
schema: foundry-draft-v1
state: committed
committed_ref: 61fc430
language_protocol: DESIGN-COMPONENT
originating_cluster: project-design
target_repo: vendor/pointsav-design-system
target_path: dtcg-vault/components/wiki-pagination/
target_filename: recipe.json
audience: design-system
bcsc_class: vendor-internal
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z — specifies prev/next article
  within category (not numbered page pagination).
  ARIA Authoring Practices navigation landmark pattern.
  Wikipedia category traversal pattern (bottom of article).
  MDN rel=prev/rel=next for SEO link relations.
research_inline: true
---

# DESIGN-COMPONENT — wiki-pagination

Prev/Next article navigation within a wiki category. Allows readers to traverse a category's article sequence linearly (← previous article / next article →). Link-based — each article URL is a real permalink; no JS required. JS enhancement may prefetch on hover.

This is **not** numbered-page pagination. For search-results page lists, use a separate `ps-wiki-page-list` component (not yet specced).

## Structure

```html
<nav class="ps-wiki-pagination" aria-label="Articles in {{category-name}}">

  <!-- Prev link — omit entirely when no previous article exists -->
  <a class="ps-wiki-pagination__prev"
     href="{{prev-article-url}}"
     rel="prev"
     aria-label="Previous article: {{prev-article-title}}">
    <span class="ps-wiki-pagination__arrow" aria-hidden="true">←</span>
    <span class="ps-wiki-pagination__direction">Previous</span>
    <span class="ps-wiki-pagination__article-title">{{prev-article-title}}</span>
  </a>

  <!-- When no previous article exists: decorative spacer to hold grid layout -->
  <!-- <span class="ps-wiki-pagination__edge" aria-hidden="true"></span> -->

  <!-- Category context — centre cell -->
  <span class="ps-wiki-pagination__context">
    <a href="{{category-url}}" class="ps-wiki-pagination__category-link">
      {{category-name}}
    </a>
  </span>

  <!-- Next link — omit entirely when no next article exists -->
  <a class="ps-wiki-pagination__next"
     href="{{next-article-url}}"
     rel="next"
     aria-label="Next article: {{next-article-title}}">
    <span class="ps-wiki-pagination__article-title">{{next-article-title}}</span>
    <span class="ps-wiki-pagination__direction">Next</span>
    <span class="ps-wiki-pagination__arrow" aria-hidden="true">→</span>
  </a>

</nav>
```

### Edge cases

- **First article in category** — omit `__prev` `<a>`; render `<span class="ps-wiki-pagination__edge" aria-hidden="true">` as spacer so the three-column grid does not collapse.
- **Last article in category** — omit `__next` `<a>`; same spacer treatment.
- **Only article in category** — both edges empty; render `__context` only; consider hiding the component entirely.

## Layout

Three-column CSS grid (`grid-template-columns: 1fr auto 1fr`):

- Left cell: `__prev` — right-aligned text
- Centre cell: `__context` — centred category link
- Right cell: `__next` — left-aligned text

On compact (≤799px): stack vertically — prev on top, context in middle, next on bottom; all three full-width, centred.

## Keyboard

- Tab order: `__prev` link → `__category-link` → `__next` link.
- No custom keyboard handler required — native `<a>` elements.
- Optional JS enhancement: `keydown` on `document`, Left arrow → prev, Right arrow → next. Guard: skip when focus is inside any `<input>`, `<textarea>`, or `[contenteditable]`. Respect `prefers-reduced-motion`.

## Tokens needed

| Token | Use |
|-------|-----|
| `--ps-wiki-link` | prev/next link color |
| `--ps-wiki-link-hover` | hover state |
| `--ps-ink-secondary` | article title snippet (muted) |
| `--ps-border-subtle` | top border separating pagination from article body |
| `--ps-space-6` | component padding top/bottom |
| `--ps-space-5` | gap between arrow + direction + title spans |
| `--ps-wiki-font-body` | font-family |

## ARIA

- `<nav aria-label="Articles in {{category-name}}">` — distinct `navigation` landmark; label includes category name so multiple navs on the page (site nav, TOC, this) are distinguishable. Satisfies WCAG 2.4.1 / Technique ARIA11.
- `aria-label="Previous article: {{prev-article-title}}"` on the prev `<a>` — full context announced; screen reader does not read child spans individually.
- `aria-label="Next article: {{next-article-title}}"` on the next `<a>` — same rationale.
- Arrow glyphs `← →` are `aria-hidden="true"` — direction already conveyed by `aria-label`.
- `rel="prev"` / `rel="next"` on the `<a>` elements; wiki engine should also emit these in `<head>` `<link>` elements for SEO (engine responsibility, not this component).
- Spacer `<span>` elements: `aria-hidden="true"`.

## Research trail

### Done

- **Brief confirmed**: component is prev/next article within category, not numbered page-list pagination. Full rewrite from initial stub which incorrectly implemented numbered pagination.
- **ARIA navigation landmark**: `<nav aria-label>` with descriptive label verified per APG navigation pattern.
- **`aria-label` on links**: full-context label suppresses redundant child span readout — verified against screen reader behaviour with complex link children.
- **SEO link relations**: `rel="prev"` / `rel="next"` on `<a>` elements documented. MDN and Google Search Central confirm value for series navigation.

### Suggested

- Confirm with wiki engine (Maud/Tantivy) how article order within a category is determined — see open question below.

### Open questions

1. **Category ordering** — what determines article sequence within a category (alphabetical by slug? `weight` frontmatter field? insertion date?)? Required before engine-side prev/next resolver can be implemented. Project-editorial or project-knowledge to clarify.
