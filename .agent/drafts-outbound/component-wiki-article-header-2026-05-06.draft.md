---
schema: foundry-draft-v1
state: committed
committed_ref: 61fc430
language_protocol: DESIGN-COMPONENT
originating_cluster: project-design
target_repo: vendor/pointsav-design-system
target_path: dtcg-vault/components/wiki-article-header/
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
  project-editorial brief 2026-05-06T18:30Z (24-agent research pass).
  Wikipedia article header pattern analysis (title + metadata bar + quality badge).
  PointSav wiki typography spec: IBM Plex Sans, 2.25rem h1.
  Wikipedia article quality grade system (FA/GA/A/B/C/Stub).
research_inline: true
---

# DESIGN-COMPONENT — wiki-article-header

Top-of-article surface displaying the slug breadcrumb, article title (H1 from frontmatter), byline (last edited + editor), and quality badge. Maps Wikipedia's article-header muscle-memory while using PointSav's IBM Plex type system.

## Structure

```html
<header class="ps-wiki-article-header">

  <!-- Slug breadcrumb — category / subcategory path -->
  <nav class="ps-wiki-article-header__breadcrumb" aria-label="Article location">
    <ol class="ps-wiki-article-header__crumb-list">
      <li><a href="{{category-url}}">{{category-name}}</a></li>
      <!-- additional segments when nested -->
    </ol>
  </nav>

  <!-- Title row: H1 + quality badge aligned on same baseline -->
  <div class="ps-wiki-article-header__title-row">
    <h1 class="ps-wiki-article-header__title">{{title}}</h1>
    <!-- Quality badge — present when article has a grade; omit when ungraded -->
    <span class="ps-wiki-badge ps-wiki-badge--{{quality-grade}}"
          aria-label="Article quality: {{quality-grade-label}}">
      {{quality-grade-label}}
    </span>
  </div>

  <!-- Byline / metadata bar -->
  <div class="ps-wiki-article-header__meta" role="doc-subtitle">
    <span class="ps-wiki-article-header__date">
      Last edited <time datetime="{{iso-date}}">{{display-date}}</time>
    </span>
    <span class="ps-wiki-article-header__divider" aria-hidden="true">·</span>
    <span class="ps-wiki-article-header__editor">{{editor-display-name}}</span>
    <span class="ps-wiki-article-header__divider" aria-hidden="true">·</span>
    <a class="ps-wiki-article-header__history" href="{{history-url}}">View history</a>
  </div>

</header>
```

### Quality badge values

Maps to `ps-wiki-badge` component (see `component-wiki-badge-tag-2026-05-06.draft.md`):

| `quality-grade` modifier | `quality-grade-label` | Visual |
|---|---|---|
| `featured` | Featured article | Gold star |
| `good` | Good article | Green circle |
| `a-class` | A-class | Blue A |
| `b-class` | B-class | Teal B |
| `c-class` | C-class | Yellow C |
| `stub` | Stub | Grey stub icon |

Omit badge element entirely when no grade is assigned.

## Tokens needed

- `--ps-wiki-font-body` — IBM Plex Sans family
- `--ps-wiki-text-h1` — 2.25rem / 36px (per editorial research pass 2026-05-06)
- `--ps-ink-primary` — title text
- `--ps-ink-secondary` — metadata bar + breadcrumb
- `--ps-wiki-link` — breadcrumb links
- `--ps-border-subtle` — optional bottom border separating header from body
- `--ps-space-3` — gap between H1 and quality badge (baseline-aligned)
- `--ps-space-5` — gap between metadata bar items
- `--ps-space-7` — header top/bottom padding

## ARIA

- `<header>` carries `role="banner"` implicitly when a direct child of `<body>`. Inside an article layout it is a generic landmark — no role needed.
- Breadcrumb `<nav aria-label="Article location">` with `<ol>` — standard breadcrumb pattern per APG. Last item (current article) is not a link; add `aria-current="page"` if rendered as `<li>` text.
- Quality badge `<span aria-label="Article quality: {{grade-label}}">` — the visible abbreviated text (e.g., "GA") may be cryptic; `aria-label` supplies the full grade name for screen readers.
- `<time datetime>` supplies machine-readable date; visible text may be relative ("3 days ago") or absolute.
- If a lead image is present: `<figure>` + `<figcaption>` with descriptive alt text.

## Variants

- **Standard** — title + meta bar only
- **With lead image** — title + meta bar + right-floated `<figure>`
- **With infobox** — title + meta bar + right-floated `<aside class="ps-wiki-infobox">`

## Variants

- **Standard** — breadcrumb + title row (H1 + badge) + byline
- **With lead image** — standard + right-floated `<figure>`
- **With infobox** — standard + right-floated `<aside class="ps-wiki-infobox">`
- **Ungraded** — badge omitted; title row is H1 only

## Research trail

### Done

- **Brief confirmed**: slug breadcrumb + title + byline + quality badge slot. Added breadcrumb and badge to initial stub which had only title + byline.
- **H1 size corrected**: brief specifies 2.25rem (editorial research pass 2026-05-06); initial stub incorrectly used 3.052rem (Major Third calculated value).
- **Wikipedia article header anatomy** cross-referenced: title, quality grade badge (top-right of title row on en.wikipedia.org), meta bar, lead image placement.
- **`<time datetime>`** machine-readable pattern verified per WCAG 1.3.1.

### Suggested

- Cross-reference with `ps-wiki-article-footer` to ensure consistent meta-bar token usage.
