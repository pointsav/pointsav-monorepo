---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: components/home-grid/
target_filename: recipe.html
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-COMPONENT
authored: 2026-04-29T00:30:00Z
authored_by: task-project-knowledge session d4c01713119a98fc (Opus parent + Sonnet sub-agent for implementation; this draft authored by parent post-commit)
authored_with: claude-opus-4-7
component_metadata:
  component_name: home-grid
  component_kind: navigation
  carbon_baseline: ProductCard / Tile
  accessibility_targets:
    - wcag-2-2-aa
    - focus-visible
    - keyboard-navigable
    - reduced-motion-respect
  brand_voice_alignment:
    - confident
    - direct
    - professional
  preview_html: |
    <section class="ps-home-grid" aria-label="Browse by category">
      <article class="ps-home-grid__card">
        <h3 class="ps-home-grid__title"><a href="/architecture">Architecture</a></h3>
        <p class="ps-home-grid__count">12 articles</p>
        <ul class="ps-home-grid__list">
          <li><a href="/architecture/three-layer-stack">Three-layer stack</a></li>
        </ul>
        <a class="ps-home-grid__more" href="/architecture">More →</a>
      </article>
    </section>
references:
  - app-mediakit-knowledge/src/server.rs (cf136e1) — home_chrome + 9-cat grid
  - app-mediakit-knowledge/static/style.css (cf136e1) — .wiki-home-* CSS conventions
  - content-wiki-documentation/.claude/rules/naming-convention.md §10 Q5-A — operator-ratified 9-category set
  - conventions/cluster-design-draft-pipeline.md — DESIGN-* pipeline contract
  - external:carbondesignsystem.com/components/tile — Carbon Tile baseline
  - external:carbondesignsystem.com/components/grid — Carbon Grid system
notes_for_designer: |
  This draft documents the home-page category grid component as it
  shipped in app-mediakit-knowledge commit cf136e1 (cluster/project-knowledge,
  2026-04-28). The substrate version should harmonize the .wiki-home-*
  class names to a substrate prefix (suggest .ps-home-grid* per Carbon
  ProductCard conventions).

  What's loose, designer to tighten:
  - Card hover effect intensity (current implementation: subtle border-
    colour shift; substrate should normalize against Carbon Tile hover
    pattern)
  - Featured-panel left-border accent thickness (current: 4px solid
    var(--link); Carbon would suggest 2px or a fill instead)
  - Recent-additions date column width (current: monospace fixed; Carbon
    DataTable would use proper column layout)
  - Empty-state copy "0 articles — in preparation" — wording is
    placeholder-grade; substrate should ratify the canonical empty-state
    message across the platform

  What to preserve verbatim:
  - The 9-category set order (architecture / services / systems /
    applications / governance / infrastructure / company / reference /
    help) — this is operator-ratified naming-convention.md §10 Q5-A and
    must not be alphabetized or re-ordered
  - The "0 articles — in preparation" placeholder behavior (do not
    suppress empty categories; render all 9 always) — this is a
    deliberate iteration-1 spec choice per Master's 22:40Z engine-spec
  - The 3-col → 2-col → 1-col responsive breakpoints (960px, 600px) —
    matches existing wiki chrome breakpoints

  Divergences from Carbon ProductCard + why:
  - Carbon ProductCard implies commerce / catalog framing; this is a
    documentation index. Renamed conceptually to match docs context.
  - Carbon Tile would be a closer match but lacks the "list of children"
    sub-list pattern; this component shows top-3 children inline plus a
    "More →" link, which extends Tile.
  - Brand-voice: Carbon is neutral/IBM-corporate; PointSav voice is
    confident-direct-professional. The "Browse by category" heading
    rather than Carbon's typical heading-less tile grids reflects this.
research_done_count: 5
research_suggested_count: 4
research_inline: true
open_questions_count: 2
research_provenance: mixed
---

# COMPONENT-home-grid — by-category browse component for the documentation wiki home page

A 9-card responsive grid component that surfaces all top-level categories of the PointSav documentation wiki on the home page. Sits below an optional Featured Article panel and above a Recent Additions feed. Always renders all 9 ratified categories regardless of article count, including empty categories with a "0 articles — in preparation" placeholder so the platform's intended scope is visible at launch.

This is the component the operator first sees when documentation.pointsav.com refines its home page from the iteration-1 placeholder index to the iteration-1 Wikipedia-Main-Page-shaped chrome.

## When to use

- The single home page of `app-mediakit-knowledge` — the wiki engine — at the URL `/`.
- Anywhere a top-level category browse pattern is needed for content with a closed taxonomy (the 9 ratified categories).
- Substrate-level: any documentation deployment that wants a Wikipedia-Main-Page-shaped category surface above a content index.

## When not to use

- Per-category landing pages (`/architecture`, `/services`) — those use a different `_index.md` MOC pattern per content-contract.md §7.
- Search results page — uses a flat ordered list, not a card grid.
- Article TOC — uses the left-rail collapsible TOC component.
- For dynamic / open taxonomies — this component assumes 9 fixed categories. Open-taxonomy variants belong in a separate substrate component.

## Recipe

### HTML

```html
<section class="ps-home-grid" aria-label="Browse by category">
  <h2 class="ps-home-grid__heading">Browse by category</h2>
  <div class="ps-home-grid__row">
    <article class="ps-home-grid__card">
      <h3 class="ps-home-grid__title">
        <a href="/architecture">Architecture</a>
      </h3>
      <p class="ps-home-grid__count">12 articles</p>
      <ul class="ps-home-grid__list">
        <li><a href="/architecture/three-layer-stack">Three-layer stack</a></li>
        <li><a href="/architecture/compounding-substrate">Compounding substrate</a></li>
        <li><a href="/architecture/citation-substrate">Citation substrate</a></li>
      </ul>
      <a class="ps-home-grid__more" href="/architecture">More →</a>
    </article>
    <!-- ... 8 more cards in operator-ratified order ... -->
    <article class="ps-home-grid__card ps-home-grid__card--empty">
      <h3 class="ps-home-grid__title">
        <a href="/help">Help</a>
      </h3>
      <p class="ps-home-grid__empty">0 articles — in preparation</p>
    </article>
  </div>
</section>
```

### CSS

```css
.ps-home-grid__row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
  margin-bottom: 2rem;
}

@media (max-width: 960px) {
  .ps-home-grid__row { grid-template-columns: repeat(2, 1fr); }
}

@media (max-width: 600px) {
  .ps-home-grid__row { grid-template-columns: 1fr; }
}

.ps-home-grid__card {
  border: 1px solid var(--border);
  border-radius: 3px;
  padding: 1rem;
  background: var(--bg);
  transition: border-color 0.15s ease;
}

.ps-home-grid__card:hover {
  border-color: var(--link);
}

@media (prefers-reduced-motion: reduce) {
  .ps-home-grid__card { transition: none; }
}

.ps-home-grid__title {
  font-size: 1.0625rem;
  margin: 0 0 0.25rem;
  font-weight: 600;
}

.ps-home-grid__title a:focus-visible {
  outline: 2px solid var(--link);
  outline-offset: 2px;
}

.ps-home-grid__count,
.ps-home-grid__empty {
  font-size: 0.8125rem;
  color: var(--fg-muted);
  margin: 0 0 0.5rem;
}

.ps-home-grid__list {
  list-style: none;
  padding: 0;
  margin: 0 0 0.5rem;
  font-size: 0.875rem;
}

.ps-home-grid__list li {
  padding: 0.125rem 0;
}

.ps-home-grid__more {
  font-size: 0.8125rem;
  color: var(--link);
}
```

### ARIA + accessibility

| Concern | Treatment |
|---|---|
| Section semantics | `<section aria-label="Browse by category">` carries the visual heading via `aria-label` so screen readers announce the region |
| Card role | Plain `<article>` — no `role` override needed; native semantics suffice |
| Heading hierarchy | `h2` for section heading; `h3` for card title; never skip heading levels |
| Focus visibility | `:focus-visible` ring on every link (title + child + "More") |
| Keyboard navigation | Tab order follows DOM order which follows the ratified 9-category render order; no `tabindex` overrides needed |
| Empty-state announcement | "0 articles — in preparation" reads naturally to screen readers via standard text content; the `--empty` modifier is visual only |
| Reduced motion | `prefers-reduced-motion: reduce` disables the hover transition |
| Colour contrast | Card text uses `var(--fg-muted)` against `var(--bg)`; substrate must verify the muted-foreground meets WCAG 2.2 AA contrast ratio of 4.5:1 |

## Brand-voice alignment

- **Confident**: ratified-set ordering signals decisiveness — these are *the* categories, not *some* categories. No "tags" sprawl, no folksonomy.
- **Direct**: card content shows the count and the top-3 children verbatim; no marketing copy, no editorial flourish.
- **Professional**: empty categories carry a factual placeholder ("0 articles — in preparation"), not an apology or a "stay tuned!" friendly framing.

## AI-consumption hint

The 9-card grid is structurally machine-readable: each card carries a stable category slug (`/<category>`), a numeric count, and 3 child slugs. RAG agents ingesting the home page can extract category-to-article mappings directly from the DOM without parsing prose. The 9-category set is closed and operator-ratified; agents can hard-code the set if needed (per `naming-convention.md` §10 Q5-A: architecture / services / systems / applications / governance / infrastructure / company / reference / help).

## Research trail

### Done — what informed this draft

- [app-mediakit-knowledge/src/server.rs:cf136e1] — the iteration-1 implementation shipped this session is the canonical reference; the draft documents what shipped, not a future-looking proposal
- [app-mediakit-knowledge/static/style.css:cf136e1] — the .wiki-home-* CSS class conventions; substrate should harmonize prefix (.wiki-* is engine-internal; substrate uses .ps-*)
- [content-wiki-documentation/.claude/rules/naming-convention.md §10 Q5-A] — operator-ratified 9-category set (architecture / services / systems / applications / governance / infrastructure / company / reference / help) — this is the ordering preserved verbatim
- [conventions/cluster-design-draft-pipeline.md] — DESIGN-* schema and frontmatter discipline
- [tacit] — Carbon Design System ProductCard / Tile / Grid conventions are the closest published-substrate analogues; the divergences are explicit in `notes_for_designer:`

### Suggested — what project-design should consult

- [external:carbondesignsystem.com/components/tile] — verify the substrate refinement aligns with Carbon Tile's hover + focus pattern (priority: high)
- [external:carbondesignsystem.com/components/grid] — confirm 3-col → 2-col → 1-col breakpoints (960px, 600px) match Carbon Grid breakpoints exactly OR document the deviation (priority: high)
- [external:w3.org/WAI/ARIA/apg/patterns/landmarks] — verify `<section aria-label>` pattern for landmark-region semantics (priority: medium)
- [pointsav-design-system/research/] — check for existing research on muted-foreground contrast ratios; the card meta-text uses `var(--fg-muted)` and substrate must verify WCAG AA 4.5:1 (priority: medium)

### Open questions — for project-design or operator

- Class-name prefix: keep `.wiki-*` (engine-internal scope) OR rename to `.ps-*` (substrate-shared scope)? Substrate refinement should pick one; the engine will accept whichever the substrate ratifies and updates `static/style.css` accordingly. Potential sources: existing pointsav-design-system convention; DTCG bundle naming.
- Empty-state copy ratification: "0 articles — in preparation" is iteration-1 placeholder. Should substrate ratify this as the canonical empty-state message platform-wide, OR define a different message for the design-system, OR allow per-deployment customization? Potential sources: ask Master + project-language for canonical wording.

## Provenance

This component shipped operationally in `app-mediakit-knowledge` commit `cf136e1` on `cluster/project-knowledge` 2026-04-28. The substrate refinement is downstream of operational state, not the reverse — the engine is the live ground truth and the substrate substrate-refines what already works. This is the customer-first ordering principle in action (per `conventions/customer-first-ordering.md`): build the package the customer installs first, then refine the substrate to match.
