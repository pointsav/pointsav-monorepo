---
schema: foundry-draft-v1
state: committed
committed_ref: 61fc430
language_protocol: DESIGN-COMPONENT
originating_cluster: project-design
target_repo: vendor/pointsav-design-system
target_path: dtcg-vault/components/wiki-search-results/
target_filename: recipe.json
audience: design-system
bcsc_class: vendor-internal
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z.
  Wikipedia search results page anatomy: title match, excerpt, breadcrumb path.
  ps-empty-state-card reused for zero-results state.
research_inline: true
---

# DESIGN-COMPONENT — wiki-search-results

Search results page listing article matches with title, excerpt, and breadcrumb path. Zero-results state reuses `ps-empty-state-card`.

## Structure

```html
<main class="ps-wiki-search-results" aria-label="Search results">
  <h1 class="ps-wiki-search-results__heading">
    Results for <mark class="ps-wiki-search-results__query">{{query}}</mark>
  </h1>
  <p class="ps-wiki-search-results__count" aria-live="polite">
    {{count}} results
  </p>

  <ol class="ps-wiki-search-results__list">
    <li class="ps-wiki-search-result">
      <a class="ps-wiki-search-result__title" href="{{article-url}}">{{title}}</a>
      <p class="ps-wiki-search-result__excerpt">
        …{{excerpt-with-<mark>highlights</mark>}}…
      </p>
      <span class="ps-wiki-search-result__path">{{breadcrumb-path}}</span>
    </li>
  </ol>
</main>
```

## Zero-results state

```html
<div class="ps-empty-state">
  <h2 class="ps-empty-state__title">No results for "{{query}}"</h2>
  <p class="ps-empty-state__body">
    Try a broader search term or browse by category.
  </p>
  <div class="ps-empty-state__links">
    <a href="/categories">Browse categories</a>
    <a href="/">Return to home</a>
  </div>
</div>
```

## Tokens needed

- `--ps-wiki-font-body` — IBM Plex Sans
- `--ps-wiki-link` — result title link
- `--ps-ink-primary` — result excerpt text
- `--ps-ink-secondary` — breadcrumb path
- `--ps-color-caution-30` — `<mark>` highlight background (light mode)
- `--ps-surface-elevated` — result card hover surface
- `--ps-border-subtle` — divider between results
- `--ps-space-5`, `--ps-space-6` — result item padding

## ARIA

- `aria-live="polite"` on the result count so screen readers announce the count after search completes without forcing focus.
- `<ol>` for the result list — ordered implies ranked relevance.
- `<mark>` elements inside excerpts highlight query matches; they are announced as "highlighted" by most screen readers. No additional ARIA needed.

## Open questions

- oq-1: Should result titles use `<h2>` elements (making them headings in the page outline) or `<a>` links without heading wrapping? Wikipedia uses plain `<a>`. Heading wrapping is more accessible for long result lists but changes the outline. **Decision deferred to project-editorial editorial pass.**

## Research trail

### Done
- Wikipedia search results anatomy cross-referenced (title, excerpt, path).
- `aria-live="polite"` for dynamic result count verified.
- `ps-empty-state-card` reuse confirmed (committed this session).

### Suggested
- Verify `<mark>` contrast ratio for `--ps-color-caution-30` as highlight background. At 106.25% base, text within `<mark>` must meet 4.5:1.
