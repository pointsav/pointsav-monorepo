---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: components/citation-authority-ribbon/
target_filename: recipe.html
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-COMPONENT
authored: 2026-04-30T01:00:00Z
authored_by: task-project-knowledge (Opus parent synthesis from 4× Sonnet sub-agent reports)
authored_with: claude-opus-4-7
component_metadata:
  component_name: citation-authority-ribbon
  component_kind: data-display
  carbon_baseline: Tag / Notification (no direct match — extends the Tag pattern)
  accessibility_targets:
    - wcag-2-2-aa
    - colour-not-sole-differentiator
    - aria-label-on-badge
    - keyboard-navigable
  brand_voice_alignment:
    - confident
    - direct
    - professional
  preview_html: |
    <ol class="ps-references">
      <li class="ps-references__entry" data-source-authority="academic">
        <span class="ps-citation-badge ps-citation-badge--academic" aria-label="Academic source">A</span>
        <span class="ps-references__text">Klein, G. et al. seL4: Formal Verification of an OS Kernel. ACM SOSP 2009. <a href="https://sel4.systems/">sel4.systems</a></span>
      </li>
      <li class="ps-references__entry" data-source-authority="regulator">
        <span class="ps-citation-badge ps-citation-badge--regulator" aria-label="Regulator source">R</span>
        <span class="ps-references__text">National Instrument 51-102 — Continuous Disclosure Obligations. BCSC. <a href="https://www.bcsc.bc.ca/">bcsc.bc.ca</a></span>
      </li>
      <li class="ps-references__entry" data-source-authority="industry">
        <span class="ps-citation-badge ps-citation-badge--industry" aria-label="Industry source">I</span>
        <span class="ps-references__text">Anthropic engineering blog. Contextual retrieval. 2024. <a href="https://www.anthropic.com/engineering/contextual-retrieval">anthropic.com</a></span>
      </li>
    </ol>
references:
  - clones/project-knowledge/.claude/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md (§3 weakness analysis #1; §6.1 first-class primitive)
  - external:en.wikipedia.org/wiki/Wikipedia:Citing_sources
  - external:en.wikipedia.org/wiki/COinS
  - external:carbondesignsystem.com/components/tag/usage
  - external:w3.org/WAI/ARIA/apg/patterns/landmarks
  - conventions/citation-substrate.md
  - citations.yaml (workspace registry)
notes_for_designer: |
  This component is the visual differentiation layer on top of the existing flat
  references list — it does NOT replace the references list. Each `<li>` in the
  references `<ol>` gains a `data-source-authority` attribute and a leading badge.

  What's loose, designer to tighten:
  - Badge shape: currently single-letter capitals in a coloured square. Carbon Tag
    pattern would suggest the full word ("Academic", "Regulator") in a tag pill.
    Single-letter is denser and matches Wikipedia's reference-list density floor;
    full-word is more legible on first encounter. Substrate ratifies one.
  - Badge colour mapping: 6 source classes proposed (academic / regulator / industry
    / direct-source / news / web-informal). The colour palette should pull from
    `color.brand.*` and `color.status.*` per the proposed token bundle, not invent
    new hues. Specific token references in the CSS section below.
  - Badge position: leading the entry text vs. left-margin gutter outside the entry.
    Leading is denser; left-gutter creates a clear scan column. Substrate ratifies.
  - Aria-label phrasing: "Academic source" vs. "Source: Academic" vs. just
    "Academic". Screen-reader-test before ratification.

  What to preserve verbatim:
  - The 6-class source-authority taxonomy: academic / regulator / industry /
    direct-source / news / web-informal. This taxonomy maps to BCSC source-quality
    posture and to the citation-template categories the existing wiki engine
    already discriminates by URL pattern.
  - The `data-source-authority` HTML attribute as the canonical machine-readable
    surface. Even if the visual badge is restyled, the attribute is the contract
    JSON-LD emission depends on.
  - The colour-is-not-sole-differentiator accessibility rule. Every badge carries
    a text affordance (single letter or full word) AND an aria-label. Screen
    readers and colour-blind readers both receive the source-class signal.

  Divergences from Carbon Tag + why:
  - Carbon Tag implies user-actionable filter chips (clickable to filter the page).
    Citation-authority badges are NOT clickable filters; they are decorative
    differentiators. Reusing the Tag pattern visually but without the interactive
    affordance.
  - Carbon Tag has 8 colour classes (red, magenta, purple, blue, cyan, teal,
    green, gray, cool-gray, warm-gray, high-contrast, outline). The 6-class
    source-authority maps subset: academic→blue, regulator→green, industry→
    warm-gray, direct-source→teal, news→cool-gray, web-informal→outline.
research_done_count: 4
research_suggested_count: 3
open_questions_count: 2
research_provenance: synthesized-from-research-draft
research_inline: true
---

# COMPONENT-citation-authority-ribbon — visual source-type differentiation for references sections

A small leading badge on each entry in an article's References section, indicating the source category — academic / regulator / industry / direct-source / news / web-informal. Addresses the structural weakness in Wikipedia's flat numeric references list (a peer-reviewed Nature paper and a personal blog post occupy identical visual registers) without altering the body register or the navigation primitive contract.

This is one of the three first-class leapfrog primitives proposed in `research-wikipedia-leapfrog-2030.draft.md` §6.1. The combination of citation-authority ribbon + research-trail footer (the next component draft) is what makes the article's epistemological position legible without reading all the footnotes — directly serving financial-community readers, analysts, regulators, and AI consumers.

## When to use

- Article references section (`<ol class="ps-references">` or equivalent) on any wiki article that carries inline citations.
- Anywhere a list of citations is rendered with verifiable source-authority metadata in the underlying data.

## When not to use

- Inline body citations (the `[1]` superscripts) — these remain unchanged. The badge attaches only to the resolved-reference entry in the References section.
- Plain-text bibliographies in non-wiki contexts — the badge taxonomy is wiki-specific.
- Citation-template-only systems with no source-authority signal in the data — without the underlying classification, the badge has no anchor to derive from.

## Recipe

### HTML

```html
<ol class="ps-references">
  <li class="ps-references__entry" id="cite-1" data-source-authority="academic">
    <span class="ps-citation-badge ps-citation-badge--academic" aria-label="Academic source">A</span>
    <span class="ps-references__text">
      <!-- citation prose: author, title, publisher, date, URL, access-date, DOI/ISBN -->
      Klein, G. et al. seL4: Formal Verification of an OS Kernel. ACM SOSP 2009.
      <a href="https://sel4.systems/">sel4.systems</a>
    </span>
    <a class="ps-references__backref" href="#cite-ref-1" aria-label="Back to citation in body">↑</a>
  </li>
  <!-- additional entries follow same shape with their own data-source-authority value -->
</ol>
```

### CSS

```css
.ps-references {
  /* keep existing list styling — this component is additive */
  padding-left: var(--space-2);
}

.ps-references__entry {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: var(--space-1);
  align-items: baseline;
  padding: var(--space-05) 0;
  font-size: var(--font-size-3); /* Wikipedia ~85% pattern preserved */
  line-height: var(--line-height-body);
}

.ps-citation-badge {
  display: inline-block;
  width: 1.25em;
  height: 1.25em;
  font-size: var(--font-size-2);
  font-weight: var(--font-weight-semibold);
  font-family: var(--font-family-sans);
  text-align: center;
  line-height: 1.25em;
  border-radius: var(--radius-xs);
  border: 1px solid transparent;
  cursor: default;
  user-select: none;
}

.ps-citation-badge--academic {
  background: var(--article-references-citation-badge-academic-bg);
  color: var(--text-on-color);
}
.ps-citation-badge--regulator {
  background: var(--article-references-citation-badge-regulator-bg);
  color: var(--text-on-color);
}
.ps-citation-badge--industry {
  background: var(--article-references-citation-badge-industry-bg);
  color: var(--text-primary);
}
.ps-citation-badge--direct-source {
  background: var(--article-references-citation-badge-direct-source-bg);
  color: var(--text-on-color);
}
.ps-citation-badge--news {
  background: var(--surface-layer-accent);
  color: var(--text-secondary);
}
.ps-citation-badge--web-informal {
  background: transparent;
  color: var(--text-tertiary);
  border-color: var(--border-subtle);
}

.ps-references__text {
  /* preserve Wikipedia citation prose treatment — no change */
}

.ps-references__backref {
  color: var(--article-references-backlink-bracket-color);
  text-decoration: none;
  font-size: var(--font-size-3);
}

.ps-references__backref:focus-visible {
  outline: 2px solid var(--interactive-focus-ring);
  outline-offset: 2px;
}
```

### ARIA + accessibility

| Concern | Treatment |
|---|---|
| Colour-blind readers | Each badge carries the single-letter text glyph (A / R / I / D / N / W) — colour is never sole differentiator |
| Screen-reader announcement | `aria-label` on each badge announces the source class explicitly ("Academic source", "Regulator source", etc.) |
| Tab-order disruption | Badge is `<span>`, not focusable. Tab-order on the references list is unchanged: only the citation hyperlinks and the backref arrow are focusable |
| Decorative-vs-content boundary | Badge is informative, not decorative — it carries `aria-label`, not `aria-hidden` |
| Backref affordance | `<a class="ps-references__backref">` returns the reader to the cited body location; matches Wikipedia's `^` arrow pattern |

## Brand-voice alignment

- **Confident** — six fixed source classes; no "Other" / "Misc" / "Unknown" escape hatch. Every cited source has a definite authority class.
- **Direct** — single-letter glyph + colour. No marketing flourish, no decorative iconography.
- **Professional** — references list density preserved; badges sit in a 1.25em gutter that does not break the line-rhythm of the surrounding citation prose.

## AI-consumption hint

The `data-source-authority` attribute is the canonical machine-readable surface. RAG and LLM consumers parsing the rendered HTML can derive citation source-class without parsing prose. JSON-LD emission attaches the same class to each `citation` entry as a `@type` refinement on the `CreativeWork` node:

```json
{
  "@context": "https://schema.org",
  "@type": "TechArticle",
  "citation": [
    {
      "@type": ["ScholarlyArticle", "AcademicSource"],
      "name": "seL4: Formal Verification of an OS Kernel",
      "url": "https://sel4.systems/",
      "datePublished": "2009-10"
    }
  ]
}
```

The `AcademicSource` (and parallel `RegulatorSource`, `IndustrySource`, `DirectSource`, `NewsSource`, `WebInformalSource`) are PointSav-namespace types layered onto Schema.org's existing `CreativeWork` hierarchy — formally announced in the wiki's `llms.txt` so external AI consumers know how to read them.

## Token references

Per `token-knowledge-wiki-baseline.draft.md` §4 `article.references.*` group:

- `--article-references-citation-badge-academic-bg` → `{color.status.info.bg}` → `{color.brand.blue.50}`
- `--article-references-citation-badge-regulator-bg` → `{color.status.success.base}` → workspace-defined regulator-green
- `--article-references-citation-badge-industry-bg` → `{surface.layer-accent}` → `{color.neutral.20}`
- `--article-references-citation-badge-direct-source-bg` → `{color.brand.teal.50}`
- `--article-references-backlink-bracket-color` → `{text.secondary}`

The 6-class palette draws exclusively from `color.brand.*` and `color.status.*` — no new primitive tokens introduced for this component.

## Research trail

### Done

- [research-wikipedia-leapfrog-2030.draft.md §3 weakness #1] — flat numeric list with no source-authority semantics is the structural problem this component addresses
- [research-wikipedia-leapfrog-2030.draft.md §6.1] — first-class leapfrog primitive prioritization
- [external:en.wikipedia.org/wiki/Wikipedia:Citing_sources] — Wikipedia's citation-template type catalogue (cite journal, cite news, cite web, cite book, cite report) maps cleanly to the 6-class authority taxonomy
- [external:en.wikipedia.org/wiki/COinS] — COinS metadata is already embedded in Wikipedia footnote rendering; this component surfaces the same data class to readers

### Suggested — what project-design should consult before refinement

- [external:carbondesignsystem.com/components/tag/usage] — verify Carbon Tag pattern as the visual baseline; the substrate may prefer Carbon Notification's left-border treatment for a denser look (priority: high)
- [external:webaim.org/articles/contrast/] — verify badge text-on-color contrast ratios meet WCAG 2.2 AA (4.5:1) across all 6 colour classes (priority: high)
- [pointsav-design-system/research/] — check for existing research on citation-display patterns (priority: medium)

### Open questions

- Single-letter glyph vs. full-word tag — substrate ratifies. Trade-off: single-letter preserves Wikipedia density floor; full-word is more legible on first encounter and more accessible to readers with low literacy.
- Badge position — leading the entry text vs. outside-left-gutter. Leading is denser; left-gutter creates a clear scan column for readers skimming for source authority.

## Provenance

This component recipe synthesizes the source-authority weakness analysis from `research-wikipedia-leapfrog-2030.draft.md` §3 weakness #1 into a concrete substrate-level recipe. The 6-class taxonomy is derived from Wikipedia's citation-template categories (cite journal → academic; cite news → news; cite web → web-informal or industry depending on URL pattern; cite report → regulator or industry depending on issuing organization), giving every existing Wikipedia-shaped citation a derivable authority class without manual editorial intervention.

Aligns with `conventions/citation-substrate.md` (Doctrine claim #25): citations are part of the substrate, machine-readable, audit-traceable. The `data-source-authority` attribute and JSON-LD `@type` refinement are the substrate-traceable surfaces.
