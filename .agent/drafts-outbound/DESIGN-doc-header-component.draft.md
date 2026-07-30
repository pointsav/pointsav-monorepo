---
schema: foundry-draft-v1
artifact_type: DESIGN-COMPONENT
language_protocol: DESIGN-COMPONENT
status: staged
created: 2026-06-01
created-by: totebox@project-knowledge
destination: project-design → pointsav-design-system
component_metadata:
  component_name: doc-header
  carbon_baseline: "none — Carbon's Page Header is a banner-style component designed for app shells (with breadcrumb, title, and action rows in a coloured band). This component is inline within the article column, sits above the article body with no background fill, and renders a breadcrumb → h1 → lede → meta-row sequence without the Carbon banner treatment. Departure from Carbon is intentional."
  accessibility_targets:
    - "Breadcrumb rendered as <nav aria-label='Breadcrumb'> with <ol> for correct screen-reader announcement"
    - "Article title is an <h1> — only one per page"
    - "Last-edited date uses <time datetime='YYYY-MM-DD'> for machine-readable semantics"
    - "Edit/View-source row hidden from unauthenticated users via html[data-auth='anon'] .doc-edit-row { display: none } — not present in DOM for anon, preventing phantom tab stops"
research_done_count: 2
research_suggested_count: 0
open_questions_count: 1
research_provenance: "Stripe docs article header, Vercel docs article header surveyed live 2026-06-01. Both follow breadcrumb → h1 → meta pattern. Carbon Page Header reviewed and rejected (banner-style, not inline). Pattern derived from implementation in app-mediakit-knowledge commit 914cd836."
research_inline: true
paired_with: "not-required — developer-facing design system recipe, not public editorial content"
source_commit: "914cd836 (pointsav-monorepo, app-mediakit-knowledge)"
---

# DESIGN-COMPONENT: doc-header

Inline article header for product-documentation surfaces. Renders a
breadcrumb navigation, article `<h1>` title, optional lede (first paragraph
promoted as a standfirst), last-edited date with history link, and an
auth-gated edit/view-source footer row. Sits at the top of the article
column with no background fill — part of the text flow, not a banner.

Includes: `doc-header`, `doc-header__titlewrap`, `doc-header__meta`,
`doc-header__edited`, `doc-edit-row`, `doc-edit-link`.

---

## HTML recipe

```html
<!-- Breadcrumb ---------------------------------------------------------- -->
<nav class="crumb" aria-label="Breadcrumb">
  <ol>
    <li><a href="/">Home</a></li>
    <li><a href="/wiki/infrastructure/">Infrastructure</a></li>
    <li aria-current="page">WORM Ledger Design</li>
  </ol>
</nav>

<!-- Article header ------------------------------------------------------- -->
<header class="doc-header">
  <div class="doc-header__titlewrap">
    <h1 class="article__title">WORM Ledger Design</h1>
    <!-- Optional: language switcher sits here as a sibling -->
  </div>

  <!-- Optional lede: render the article's first paragraph as a standfirst
       when frontmatter declares lede: true or content_type: guide/research -->

  <div class="doc-header__meta">
    <span class="doc-header__edited">
      Updated
      <a href="/wiki/infrastructure/worm-ledger-design/history">
        <time datetime="2026-05-29">May 29, 2026</time>
      </a>
    </span>
    <!-- Additional meta slots (author, reading time) go here if added -->
  </div>
</header>

<!-- Article body renders here ------------------------------------------ -->
<div class="article__body mw-parser-output">
  ...
</div>

<!-- Edit row (auth-gated — hidden for anon via CSS) --------------------- -->
<div class="doc-edit-row">
  <a href="/wiki/infrastructure/worm-ledger-design/edit"
     class="doc-edit-link">Edit this page</a>
  ·
  <a href="/wiki/infrastructure/worm-ledger-design/source"
     class="doc-edit-link">View source</a>
</div>
```

**Notes:**
- The `crumb` nav is a sibling of `doc-header`, not nested inside it.
  Breadcrumb renders above the header in document order.
- The `<h1>` uses the existing `article__title` class for typography
  inheritance — `doc-header` provides spacing and layout only.
- Auth gating: set `data-auth="anon"` on `<html>` for unauthenticated
  sessions; the CSS rule `html[data-auth="anon"] .doc-edit-row { display: none }`
  hides the edit row without a server-side branch in the template.
- Language switcher (if present) sits inside `doc-header__titlewrap` as a
  flex sibling to the `<h1>`, pushed right via `margin-left: auto`.

---

## CSS

```css
/* Article header --------------------------------------------------------- */
.doc-header { margin-bottom: 28px; }

.doc-header__titlewrap {
  display: flex;
  align-items: baseline;
  flex-wrap: wrap;
  gap: 12px;
}

.doc-header .article__title { margin-bottom: 8px; }

.doc-header__meta {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 16px;
  margin-top: 10px;
  font-size: 13px;
  color: var(--fg-3);
}

.doc-header__edited a { color: var(--fg-3); font-weight: 600; }
.doc-header__edited a:hover { color: var(--link, var(--navy)); }

/* Edit / View-source row (auth-gated) ------------------------------------ */
.doc-edit-row {
  margin: 48px 0 8px;
  padding-top: 20px;
  border-top: 1px solid var(--rule);
  font-size: 13px;
  color: var(--fg-3);
}

.doc-edit-link { color: var(--fg-3); font-weight: 600; text-decoration: none; }
.doc-edit-link:hover { color: var(--link, var(--navy)); text-decoration: underline; }

/* Hide edit row for unauthenticated sessions ----------------------------- */
html[data-auth="anon"] .doc-edit-row { display: none; }
```

---

## ARIA checklist

| Requirement | Implementation |
|---|---|
| Breadcrumb landmark | `<nav aria-label="Breadcrumb">` wrapping an `<ol>` |
| Current page in breadcrumb | `aria-current="page"` on final `<li>` |
| Unique page title | `<h1 class="article__title">` — one per page |
| Machine-readable date | `<time datetime="YYYY-MM-DD">` for the last-edited value |
| Edit row not a phantom tab stop for anon | `display: none` removes from accessibility tree entirely |

---

## Design decisions

**Inline header, not a banner.** Product-docs surfaces (Stripe, Vercel,
Cloudflare) treat the article title as part of the reading flow — the
header has no background, no horizontal rule below the title, no coloured
band. The Wikipedia/Carbon approach (bold-background page header) breaks
the sense that you are reading a document rather than navigating an app.

**Auth-gated edit row via CSS attribute selector.** The server sets
`data-auth="anon"` on the `<html>` element for unauthenticated requests.
This lets a single template emit the edit row unconditionally; the CSS
hides it for anon readers. No server-side branch, no duplicated template
block. The tradeoff: the edit links are briefly present in the DOM for
anon users if CSS is slow to parse — acceptable because the links lead to
an authentication wall, not a destructive action.

**`<time datetime>` for last-edited.** Machine-readable date in the `datetime`
attribute lets browsers, search engines, and screen readers report the
correct date regardless of the display format chosen ("May 29" vs
"2026-05-29" vs locale-specific).

**Lede promotion deferred to content-type phase.** The header is
designed with a slot between `h1` and `doc-header__meta` for a standfirst
paragraph. The current implementation promotes the first paragraph as lede
only when `content_type: guide` or `content_type: research` is set in
frontmatter. Future: add a `lede:` frontmatter key to enable on any article.

---

## Open question for project-design

1. **Author attribution slot.** The `doc-header__meta` row currently shows
   only the last-edited date. Should an author/contributor field be added
   (e.g. "By Jennifer Woodfine · Updated May 29")? Requires a decision on
   whether to surface git blame attribution in the rendered header or keep
   the header impersonal (documentation-style, no byline).
