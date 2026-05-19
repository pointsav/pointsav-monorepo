---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: components/freshness-ribbon/
target_filename: recipe.html
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-COMPONENT
authored: 2026-04-30T01:10:00Z
authored_by: task-project-knowledge (Opus parent synthesis from 4× Sonnet sub-agent reports)
authored_with: claude-opus-4-7
component_metadata:
  component_name: freshness-ribbon
  component_kind: data-display
  carbon_baseline: Tag (Carbon Tag pattern in muted register)
  accessibility_targets:
    - wcag-2-2-aa
    - colour-not-sole-differentiator
    - aria-label-on-badge
    - reader-preference-toggle
  brand_voice_alignment:
    - confident
    - direct
    - professional
  preview_html: |
    <article class="ps-article">
      <h2 class="ps-article__section-heading">
        Background
        <a class="ps-article__section-edit" href="?action=edit&section=2">[edit]</a>
        <span class="ps-freshness-ribbon ps-freshness-ribbon--fresh"
              aria-label="Last reviewed 2026-03-15 — fresh">
          2026-03-15
        </span>
      </h2>
      <p>...</p>

      <h2 class="ps-article__section-heading">
        Current implementations
        <a class="ps-article__section-edit" href="?action=edit&section=3">[edit]</a>
        <span class="ps-freshness-ribbon ps-freshness-ribbon--stale"
              aria-label="Last reviewed 2024-09-12 — stale, over a year ago">
          2024-09-12
        </span>
      </h2>
      <p>...</p>
    </article>
references:
  - clones/project-knowledge/.claude/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md (§3 weakness #6 + §6.3 first-class primitive)
  - external:meta.wikimedia.org/wiki/Wikimedia_Foundation_Annual_Plan/2025-2026/Product_%26_Technology_OKRs (65% expensive requests are AI scrapers)
  - external:carbondesignsystem.com/components/tag/usage
  - external:schema.org/dateModified
  - conventions/citation-substrate.md
notes_for_designer: |
  Per-section last-content-review badge — addresses the Wikipedia weakness where
  an article-level last-edited timestamp gives no signal that one section was
  reviewed yesterday while 19 others have been unchanged since 2019.

  Two data sources:
  - Default: git-blame at section level — most recent commit that touched content
    lines within the section's heading-to-next-heading range
  - Override: `content_reviewed_on` per-section frontmatter field (when an editor
    manually ratifies the section as reviewed without changing content)

  The distinction matters: a cosmetic formatting change should NOT update the
  freshness signal. Engine implementation detail: render-time git-blame must
  exclude commits whose changeset within the section is whitespace-only or
  non-substantive (matching `--ignore-all-space --ignore-blank-lines`).

  What's loose, designer to tighten:
  - Display format: full ISO date "2026-03-15" vs relative "6 weeks ago" vs
    seasonal "Spring 2026". ISO is most precise; relative is most reader-
    friendly; seasonal is least precise. Substrate ratifies one canonical form.
  - Three-stop colour scale: fresh (green) / stale (amber) / archived (gray).
    Thresholds (90 / 365 days) are token defaults; per-deployment override
    possible via `article.freshness-ribbon.threshold-fresh-days` token override.
  - Position: right-end of section heading vs. inline-after-edit-pencil vs.
    above-section. Right-end is most visually balanced; inline-after-edit-pencil
    is denser; above-section gets more attention but breaks heading flow.
  - Reader preference toggle: substrate ratifies whether the ribbon is on by
    default OR off by default with reader-toggle. Currently implies on-by-default;
    flipping to off-by-default makes Wikipedia-muscle-memory baseline cleaner.

  What to preserve verbatim:
  - The three-stop semantic (fresh / stale / archived) — these are the canonical
    states, not an open enum. Threshold values are tunable per deployment.
  - The git-blame default with frontmatter override pattern — substrate cannot
    infer the override-vs-default distinction from a date string alone, so the
    engine's render output must encode which mechanism produced the date (via a
    `data-source` attribute on the ribbon for the substrate's reference)
  - The aria-label phrasing pattern: "Last reviewed YYYY-MM-DD — {fresh|stale|
    archived}". Screen-reader users get the absolute date AND the semantic class.
  - The `dateModified` JSON-LD emission per section — load-bearing for AI
    consumers per §3 weakness #10 of the research draft

  Divergences from Carbon Tag + why:
  - Carbon Tag is interactive (filter chip). Freshness ribbon is informational
    only — the date itself is not clickable
  - Carbon Tag has 8 colour classes; the 3-stop semantic is a strict subset
research_done_count: 5
research_suggested_count: 3
open_questions_count: 2
research_provenance: synthesized-from-research-draft
research_inline: true
---

# COMPONENT-freshness-ribbon — per-section last-content-review badge for article-shell

A small badge on each section heading (right-end of the heading row, after the `[edit]` pencil) showing the date of the last substantive content change to that section. Three-stop colour scale signals fresh / stale / archived per configurable date thresholds. Surfaces a signal Wikipedia structurally does not — a section-level review date that distinguishes "Background unchanged since 2019" from "Current implementations updated yesterday" — without modifying the article body register.

This is one of the three first-class leapfrog primitives proposed in `research-wikipedia-leapfrog-2030.draft.md` §6.3. The component addresses the freshness illusion of Wikipedia's article-level "last edited" timestamp and produces structured `dateModified` per-section JSON-LD output that addresses the AI-scraper-65%-of-expensive-requests problem the Wikimedia Foundation explicitly named in its 2025-2026 OKRs.

## When to use

- Every section heading on an article whose frontmatter declares `freshness_ribbon: true` (default per substrate ratification)
- Substrate-level: any documentation deployment where per-section content-review currency is a meaningful editorial signal

## When not to use

- Stub articles or pre-build status — no review history exists yet
- Article-level summary headings (article title `<h1>`) — the ribbon attaches to section `<h2>`/`<h3>`, not the article title
- When a reader has toggled freshness display off in their preferences (substrate may default the ribbon to on or off — open question)

## Recipe

### HTML

```html
<h2 class="ps-article__section-heading">
  Background
  <a class="ps-article__section-edit" href="?action=edit&section=2"
     aria-label="Edit section: Background">[edit]</a>
  <span class="ps-freshness-ribbon ps-freshness-ribbon--fresh"
        data-source="git-blame"
        data-iso="2026-03-15"
        aria-label="Last reviewed 2026-03-15 — fresh">
    2026-03-15
  </span>
</h2>

<!-- stale section -->
<h2 class="ps-article__section-heading">
  Current implementations
  <a class="ps-article__section-edit" href="?action=edit&section=3">[edit]</a>
  <span class="ps-freshness-ribbon ps-freshness-ribbon--stale"
        data-source="git-blame"
        data-iso="2024-09-12"
        aria-label="Last reviewed 2024-09-12 — stale, over a year ago">
    2024-09-12
  </span>
</h2>

<!-- archived section, manually ratified via content_reviewed_on frontmatter -->
<h2 class="ps-article__section-heading">
  Historical context
  <a class="ps-article__section-edit" href="?action=edit&section=4">[edit]</a>
  <span class="ps-freshness-ribbon ps-freshness-ribbon--archived"
        data-source="frontmatter-review"
        data-iso="2022-11-04"
        aria-label="Last reviewed 2022-11-04 — archived, content under historical review only">
    2022-11-04
  </span>
</h2>
```

### CSS

```css
.ps-article__section-heading {
  display: flex;
  align-items: baseline;
  gap: var(--space-1);
  flex-wrap: wrap;
}

.ps-freshness-ribbon {
  display: inline-block;
  margin-left: auto;  /* push to right end of heading row */
  font-family: var(--font-family-mono);
  font-size: var(--font-size-2);
  font-weight: var(--font-weight-regular);
  padding: var(--space-025) var(--space-1);
  border-radius: var(--radius-xs);
  border: 1px solid transparent;
  white-space: nowrap;
  cursor: default;
}

.ps-freshness-ribbon--fresh {
  background: var(--surface-layer-accent);
  color: var(--article-freshness-ribbon-color-fresh);
  border-color: var(--article-freshness-ribbon-color-fresh);
}

.ps-freshness-ribbon--stale {
  background: var(--surface-layer-accent);
  color: var(--article-freshness-ribbon-color-stale);
  border-color: var(--article-freshness-ribbon-color-stale);
}

.ps-freshness-ribbon--archived {
  background: var(--surface-layer-accent);
  color: var(--article-freshness-ribbon-color-archived);
  border-color: var(--article-freshness-ribbon-color-archived);
}

/* When a reader has toggled freshness off, hide all ribbons */
:root[data-freshness-display="off"] .ps-freshness-ribbon {
  display: none;
}
```

### ARIA + accessibility

| Concern | Treatment |
|---|---|
| Colour-not-sole-differentiator | Each ribbon shows the explicit ISO date. Screen readers announce the date AND the semantic class via `aria-label` |
| Heading-hierarchy preservation | Ribbon is `<span>`, not focusable, embedded inside the `<h2>`/`<h3>` — heading semantic is unbroken |
| Tab-order | Ribbon is not focusable; tab-order on the section heading remains: `[edit]` link only |
| Reader preference toggle | `:root[data-freshness-display="off"]` selector hides all ribbons site-wide; persisted in `localStorage` per the existing density-toggle pattern |
| Hover/focus | No interactive affordance — date display only. Substrate may add a tooltip showing the source (`git-blame` vs `frontmatter-review`) on hover for editorial-curious readers |
| Reduced-motion | No animation; static ribbon |

## Brand-voice alignment

- **Confident** — fixed three-stop semantic (fresh / stale / archived); thresholds are explicit (90 days / 365 days defaults). No "fresh-ish" hedge.
- **Direct** — date is the date. Format is monospace ISO (2026-03-15), no creative decoration.
- **Professional** — visually subordinate to the section heading text; never competes with the heading for visual weight.

## AI-consumption hint

The substrate emits per-section `dateModified` properties on `WebPageElement` JSON-LD nodes. Each section is its own `WebPageElement`:

```json
{
  "@context": "https://schema.org",
  "@type": "TechArticle",
  "hasPart": [
    {
      "@type": "WebPageElement",
      "name": "Background",
      "dateModified": "2026-03-15",
      "additionalType": "FreshSection"
    },
    {
      "@type": "WebPageElement",
      "name": "Current implementations",
      "dateModified": "2024-09-12",
      "additionalType": "StaleSection"
    },
    {
      "@type": "WebPageElement",
      "name": "Historical context",
      "dateModified": "2022-11-04",
      "additionalType": "ArchivedSection"
    }
  ]
}
```

The `additionalType` (`FreshSection` / `StaleSection` / `ArchivedSection`) is a PointSav-namespace classification layered onto Schema.org's `WebPageElement`, so an AI consumer can request "only sections marked Fresh" without computing the threshold itself. Threshold parameters are documented in the wiki's `llms.txt` so external consumers know the substrate's `<= 90 days = fresh, > 90 days <= 365 days = stale, > 365 days = archived` semantics.

## Token references

Per `token-knowledge-wiki-baseline.draft.md` §4 `article.freshness-ribbon.*` group:

- `--article-freshness-ribbon-color-fresh` → `{color.status.success.base}`
- `--article-freshness-ribbon-color-stale` → `{color.status.warn.base}`
- `--article-freshness-ribbon-color-archived` → `{color.neutral.50}`
- `--article-freshness-ribbon-threshold-fresh-days` → `90` (number token)
- `--article-freshness-ribbon-threshold-stale-days` → `365` (number token)

The thresholds are number tokens — DTCG 2025.10 supports number-typed tokens, allowing per-deployment override (a regulator-track tenant might set fresh threshold to 30 days; a historical-archive tenant might set stale threshold to 5 years).

## Engine integration note (cluster-side)

The substrate ships the recipe; the engine implements the render-time machinery. The engine work is project-knowledge cluster scope (the `app-mediakit-knowledge` crate). Required engine extensions:

1. Section-boundary detection — pass over the parsed Markdown AST to identify section ranges by heading boundaries
2. Git-blame at section level — `git log --follow --no-merges -- <file>` filtered to the section's line range, with `--ignore-all-space --ignore-blank-lines` excluding non-substantive changes
3. Frontmatter `content_reviewed_on` parsing — per-section override map keyed by section slug
4. Threshold computation at render time — `(today - dateModified).days` against the threshold tokens
5. JSON-LD emission — per-section `WebPageElement` with `dateModified` and `additionalType`
6. ARIA-label generation — render-time substitution from the date and threshold class

These are tracked as engine work for project-knowledge cluster's next iteration; this DESIGN-COMPONENT recipe is the substrate-canonical visual contract the engine work will satisfy.

## Research trail

### Done

- [research-wikipedia-leapfrog-2030.draft.md §3 weakness #6] — no per-section last-edited or authorship granularity
- [research-wikipedia-leapfrog-2030.draft.md §6.3] — first-class leapfrog primitive
- [external:meta.wikimedia.org/wiki/Wikimedia_Foundation_Annual_Plan/2025-2026/Product_%26_Technology_OKRs] — Wikimedia's own acknowledgment of the AI-scraper structural problem this component addresses
- [external:schema.org/dateModified] — formal property type for per-section date emission
- [external:schema.org/WebPageElement] — formal type for section subdivision

### Suggested

- [external:carbondesignsystem.com/components/tag/usage] — verify Carbon Tag muted-style as the visual baseline (priority: medium)
- [pointsav-design-system/research/] — check for existing date-display conventions (priority: medium)
- [external:webaim.org/articles/contrast/] — verify ribbon colour contrast meets WCAG 2.2 AA across all three states on `surface.layer-accent` background (priority: high)

### Open questions

- Reader-preference default: ribbons on by default OR off by default with toggle? On-by-default makes the freshness signal universal but adds visual weight to the article shell that breaks the Wikipedia muscle-memory baseline. Off-by-default keeps the muscle-memory clean but means the signal is hidden until activated. (Substrate ratifies; affects the component's visibility footprint.)
- Display format: ISO `2026-03-15` vs relative `6 weeks ago` vs seasonal `Spring 2026`. ISO is most precise and easiest to render; relative requires render-time computation; seasonal is editorial preference. (Substrate ratifies.)

## Provenance

Synthesizes Wikimedia's own structural acknowledgment of the AI-scraper problem (2025-2026 OKRs) with the freshness-illusion weakness in Wikipedia's article-level last-edited surface. The substrate's `dateModified` per-section JSON-LD emission directly addresses the structure mismatch the Wikimedia Foundation named — making AI consumers' work cheaper and more accurate, which is downstream good both for the substrate and for the broader knowledge-commons.

This is the component that a Communication Arts or Webby Reference jury can demonstrably point to when asked "what did this wiki ship that Wikipedia didn't?". The structural comparison is unambiguous: Wikipedia ships article-level `dateModified` only; this substrate ships section-level `dateModified` with a three-stop semantic and a reader-toggleable visual surface.
