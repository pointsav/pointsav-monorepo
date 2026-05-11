---
schema: foundry-draft-v1
state: committed
committed_ref: 61fc430
language_protocol: DESIGN-COMPONENT
originating_cluster: project-design
target_repo: vendor/pointsav-design-system
target_path: dtcg-vault/components/wiki-article-footer/
target_filename: recipe.json
audience: design-system
bcsc_class: vendor-internal
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 1
open_questions_count: 0
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z.
  Wikipedia article footer pattern: categories, references, edit-this-page.
  Existing ps-machine-footer pattern referenced for structural alignment.
research_inline: true
---

# DESIGN-COMPONENT — wiki-article-footer

Bottom-of-article surface with: categories, references/citations block, and editorial controls (Edit on GitHub, CC licence notice). Mirrors Wikipedia's article footer muscle-memory.

## Structure

```html
<footer class="ps-wiki-article-footer">
  <section class="ps-wiki-article-footer__categories" aria-label="Categories">
    <h2 class="ps-wiki-article-footer__section-heading">Categories</h2>
    <ul class="ps-wiki-article-footer__category-list">
      <li><a href="{{category-url}}">{{category}}</a></li>
    </ul>
  </section>

  <section class="ps-wiki-article-footer__references" aria-label="References">
    <h2 class="ps-wiki-article-footer__section-heading">References</h2>
    <ol class="ps-wiki-article-footer__ref-list">
      <li id="ref-{{n}}">{{citation}}</li>
    </ol>
  </section>

  <div class="ps-wiki-article-footer__controls">
    <a class="ps-edit-link" href="{{github-source-url}}" target="_blank" rel="noopener noreferrer">
      Edit this page
    </a>
    <span class="ps-wiki-article-footer__licence">
      Content available under <a href="https://creativecommons.org/licenses/by/4.0/">CC BY 4.0</a>.
    </span>
  </div>
</footer>
```

## Tokens needed

- `--ps-border-subtle` — top separator
- `--ps-ink-secondary` — section headings
- `--ps-ink-primary` — reference text
- `--ps-wiki-link` — category + reference links
- `--ps-wiki-font-body` — IBM Plex Sans
- `--ps-space-6`, `--ps-space-7` — section spacing

## ARIA

- Two `<section aria-label>` landmarks inside the footer distinguish categories from references.
- Reference list uses `<ol>` (ordered, numbered). Back-links from footnote anchors in body text use `id="ref-N"` pattern (Wikipedia convention).
- `<footer>` inside a `<main>` is a generic element, not a `contentinfo` landmark — no conflict with `<ps-machine-footer>` at page level.

## Research trail

### Done
- Wikipedia article footer anatomy cross-referenced (categories, references, licence, edit-link).
- Reference anchor pattern (`#ref-N` / `#cite-N`) verified for bidirectional linking.
- Reuses `ps-edit-link` component (committed this session, stub).

### Suggested
- Coordinate with `ps-wiki-article-header` to share `--ps-wiki-font-body` token usage.
