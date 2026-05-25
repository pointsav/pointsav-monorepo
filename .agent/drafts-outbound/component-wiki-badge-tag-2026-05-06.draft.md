---
schema: foundry-draft-v1
state: committed
committed_ref: 61fc430
language_protocol: DESIGN-COMPONENT
originating_cluster: project-design
target_repo: vendor/pointsav-design-system
target_path: dtcg-vault/components/wiki-badge-tag/
target_filename: recipe.json
audience: design-system
bcsc_class: vendor-internal
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z.
  Wikipedia category tag + article status badge (stub, good article, featured).
  Existing ps-chip-row component (committed this session) is the generic form;
  this wiki-specific variant adds status semantics.
research_inline: true
---

# DESIGN-COMPONENT — wiki-badge-tag

Inline badge and tag components for wiki articles. Badges communicate article status (stub, draft, verified); tags communicate topic categorisation. Built on the same primitive as `ps-chip-row` but with wiki-specific semantic variants.

## Structure — badge (article status)

```html
<span class="ps-wiki-badge ps-wiki-badge--{{variant}}" role="note">
  <span class="ps-wiki-badge__label">{{status}}</span>
</span>
```

Variants: `stub` (caution/amber), `draft` (neutral), `verified` (positive/green), `featured` (primary/blue).

## Structure — tag (category link)

```html
<a class="ps-wiki-tag" href="{{category-url}}">{{category}}</a>
```

Tags are always links in a wiki context.

## Structure — tag row (multiple tags)

```html
<div class="ps-wiki-tag-row" role="list" aria-label="Categories">
  <a class="ps-wiki-tag" role="listitem" href="{{url}}">{{category}}</a>
</div>
```

## Tokens needed

- `--ps-corner-1` — 2px radius (tighter than general chips)
- `--ps-wiki-font-body` — IBM Plex Sans, smaller weight for tags
- `--ps-support-caution` / `--ps-support-caution-bg` — stub badge
- `--ps-ink-secondary` / `--ps-color-neutral-20` — draft badge
- `--ps-support-positive` / `--ps-support-positive-bg` — verified badge
- `--ps-interactive-primary` / `--ps-color-primary-10` — featured badge
- `--ps-wiki-link` — tag link color

## ARIA

- Badge: `role="note"` signals supplementary information; screen readers announce its content as a note.
- Tag row: `role="list"` with `aria-label="Categories"` for grouping; each tag is `role="listitem"`.
- Featured / stub badges also benefit from a visually-hidden SR text: `<span class="sr-only">Article status:</span>` before the label.

## Research trail

### Done
- Wikipedia article status badge pattern cross-referenced (stub, good, featured).
- `role="note"` vs `role="status"` for badges: `note` is correct (not a live region; no auto-announcement needed).
- Tag row `role="list"` pattern documented per WCAG 1.3.1 info and relationships.
