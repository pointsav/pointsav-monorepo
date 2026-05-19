---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: components/research-trail-footer/
target_filename: recipe.html
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-COMPONENT
authored: 2026-04-30T01:05:00Z
authored_by: task-project-knowledge (Opus parent synthesis from 4× Sonnet sub-agent reports)
authored_with: claude-opus-4-7
component_metadata:
  component_name: research-trail-footer
  component_kind: data-display
  carbon_baseline: Accordion (Carbon collapsible-content pattern)
  accessibility_targets:
    - wcag-2-2-aa
    - aria-expanded-on-summary
    - keyboard-navigable
    - heading-hierarchy-respected
  brand_voice_alignment:
    - confident
    - direct
    - professional
  preview_html: |
    <details class="ps-research-trail">
      <summary class="ps-research-trail__summary" aria-expanded="false">
        Research trail — 4 done · 3 suggested · 1 open question
      </summary>
      <section class="ps-research-trail__body">
        <h3 class="ps-research-trail__heading ps-research-trail__heading--done">
          Research done
        </h3>
        <ul class="ps-research-trail__list">
          <li>seL4 formal verification (Klein 2009) — primary academic source</li>
          <li>capability-based-security Wikipedia article — secondary survey</li>
        </ul>
        <h3 class="ps-research-trail__heading ps-research-trail__heading--suggested">
          Suggested research
        </h3>
        <ul class="ps-research-trail__list">
          <li>seL4 capability-revocation 2024–2026 papers — verify current state</li>
        </ul>
        <h3 class="ps-research-trail__heading ps-research-trail__heading--open">
          Open questions
        </h3>
        <ul class="ps-research-trail__list">
          <li>Does the seL4 microkernel revoke capabilities transitively when a parent is destroyed?</li>
        </ul>
      </section>
    </details>
references:
  - clones/project-knowledge/.claude/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md (§3 weakness #8 + §6.2 first-class primitive)
  - conventions/draft-research-trail-discipline.md (Doctrine claim #39 — mandatory v0.1.58+)
  - external:carbondesignsystem.com/components/accordion/usage
  - external:w3.org/WAI/ARIA/apg/patterns/disclosure
  - external:schema.org/Question
  - external:schema.org/SearchAction
notes_for_designer: |
  This component renders the Doctrine claim #39 research trail at article scale.
  Currently `draft-research-trail-discipline.md` mandates the trail at draft scale
  (every PROSE-* and DESIGN-* draft entering the editorial pipelines carries the 5
  frontmatter fields + body Research-trail section). This component extends the
  same pattern to the published article surface.

  Article-level scope:
  - The article's published frontmatter declares `research_trail: true` (or false)
  - The body contains an authored "## Research trail" section near the end (after
    See also, before References — different from the draft pipeline's footer
    placement which is after Provenance)
  - The substrate renders the section as a collapsible disclosure widget using
    the recipe below
  - At render time, the engine emits JSON-LD `Question` and `SearchAction` nodes
    per item

  What's loose, designer to tighten:
  - Collapsed-default vs expanded-default — current §7(f) open question. If chrome
    posture, collapsed-default; if body-content posture, expanded-default.
  - Subheading colour signal — done/suggested/open-question. Currently uses
    `color.status.*`. Substrate may prefer a single muted neutral with an icon
    differentiator (check / arrow / question-mark) for less visual weight.
  - Item-list density — currently `<ul>` matching surrounding article body
    register. Substrate may prefer denser list (fewer line-heights between items)
    to signal subordination to body content.

  What to preserve verbatim:
  - The three-subsection structure (Done / Suggested / Open questions) per
    `draft-research-trail-discipline.md` body convention
  - The summary line text format: "Research trail — N done · N suggested · N
    open question(s)" — counts come directly from the frontmatter fields the
    discipline mandates
  - The `<details>/<summary>` semantic — NOT a div with a click handler. Native
    keyboard-navigation, native screen-reader-announcement
  - The aria-expanded attribute mirroring the `<details>` open state (browsers
    handle this natively for `<details>`; the attribute is informational here)

  Divergences from Carbon Accordion + why:
  - Carbon Accordion is built for FAQ-style content stacks (multiple parallel
    items). The research trail is one specific item with three sub-categories.
    Single `<details>` is structurally accurate.
  - Carbon Accordion has expand-all / collapse-all controls. The trail does not
    need them — single item.
research_done_count: 5
research_suggested_count: 3
open_questions_count: 2
research_provenance: synthesized-from-research-draft
research_inline: true
---

# COMPONENT-research-trail-footer — collapsible epistemic-frontier disclosure for article foot

A bottom-of-article collapsible block that surfaces an article's research trail in three subsections: research done, suggested research, open questions. The pattern is mandated at draft scale by `conventions/draft-research-trail-discipline.md` (Doctrine claim #39) for every draft entering the project-language or project-design pipelines. This component extends the same pattern to the article-shell reading surface — making the editorial pipeline's epistemic discipline visible to readers, researchers, regulators, and AI consumers.

This is one of the three first-class leapfrog primitives proposed in `research-wikipedia-leapfrog-2030.draft.md` §6.2. The combination of citation-authority ribbon + research-trail footer is what makes the article's epistemological position legible without reading all the footnotes. Wikipedia structurally does not ship this; the Talk: namespace records discussion but provides no reader-facing epistemic-frontier signal.

## When to use

- Any article whose frontmatter declares `research_trail: true` and whose body contains an authored "## Research trail" section
- The substrate-canonical placement is bottom-of-article, after See also and before References (different from the draft-pipeline placement which is after Provenance)

## When not to use

- Articles whose frontmatter declares `research_trail: false` — render no widget
- Articles in `status: draft` — the trail is for published material; in-draft trails are pipeline-internal
- Stub articles where the trail has no items (all three counts zero) — render nothing

## Recipe

### HTML

```html
<details class="ps-research-trail">
  <summary class="ps-research-trail__summary">
    Research trail — 4 done · 3 suggested · 1 open question
  </summary>
  <section class="ps-research-trail__body" aria-label="Research trail detail">
    <h3 class="ps-research-trail__heading ps-research-trail__heading--done"
        id="research-trail-done">
      Research done
    </h3>
    <ul class="ps-research-trail__list">
      <li>Klein 2009 — seL4 formal verification (academic source)</li>
      <li>Wikipedia capability-based-security article (secondary survey)</li>
      <li>conventions/system-substrate-doctrine.md (workspace-direct)</li>
      <li>seL4 ARM port documentation (direct primary source)</li>
    </ul>

    <h3 class="ps-research-trail__heading ps-research-trail__heading--suggested"
        id="research-trail-suggested">
      Suggested research
    </h3>
    <ul class="ps-research-trail__list">
      <li>seL4 capability-revocation 2024–2026 papers — verify transitive revocation behaviour</li>
      <li>Cross-reference with NetBSD compatibility layer's capability mapping</li>
      <li>BCSC NI 51-102 §4 disclosure-class for substrate-running-on-customer-metal</li>
    </ul>

    <h3 class="ps-research-trail__heading ps-research-trail__heading--open"
        id="research-trail-open">
      Open questions
    </h3>
    <ul class="ps-research-trail__list">
      <li>Does the seL4 microkernel revoke capabilities transitively when a parent capability is destroyed, or only on explicit child enumeration?</li>
    </ul>
  </section>
</details>
```

### CSS

```css
.ps-research-trail {
  margin-top: var(--space-4);
  padding: var(--space-2);
  background: var(--article-research-trail-trail-bg);
  border-left: 3px solid var(--article-research-trail-trail-border);
}

.ps-research-trail__summary {
  font-family: var(--font-family-sans);
  font-size: var(--font-size-3);
  font-weight: var(--font-weight-semibold);
  color: var(--text-secondary);
  cursor: pointer;
  list-style: revert;  /* preserve native disclosure triangle */
  user-select: none;
}

.ps-research-trail__summary:focus-visible {
  outline: 2px solid var(--interactive-focus-ring);
  outline-offset: 2px;
}

.ps-research-trail__body {
  margin-top: var(--space-2);
}

.ps-research-trail__heading {
  font-family: var(--font-family-sans);
  font-size: var(--font-size-4);
  font-weight: var(--font-weight-semibold);
  margin: var(--space-2) 0 var(--space-1);
  padding-left: var(--space-1);
  border-left: 3px solid transparent;
}

.ps-research-trail__heading--done {
  border-left-color: var(--article-research-trail-done-color);
}

.ps-research-trail__heading--suggested {
  border-left-color: var(--article-research-trail-suggested-color);
}

.ps-research-trail__heading--open {
  border-left-color: var(--article-research-trail-open-question-color);
}

.ps-research-trail__list {
  margin: 0;
  padding-left: var(--space-2);
  font-size: var(--font-size-3);
  line-height: var(--line-height-body);
}

.ps-research-trail__list li {
  padding: var(--space-025) 0;
  color: var(--text-primary);
}

@media (prefers-reduced-motion: reduce) {
  .ps-research-trail[open] .ps-research-trail__body {
    /* no transition; instant reveal */
  }
}
```

### ARIA + accessibility

| Concern | Treatment |
|---|---|
| Disclosure pattern | Native `<details>/<summary>` element pair — no JS needed; browsers ship keyboard, screen-reader, and aria-expanded handling natively |
| Heading hierarchy | Subsections use `<h3>` because the article body's outermost "## Research trail" markdown heading renders as `<h2>`; engine inserts the `<details>` block immediately under that `<h2>` |
| Summary line semantics | Counts in the summary line ("4 done · 3 suggested · 1 open question") are derivable at render time from the article frontmatter — render-time substitution, not author-maintained |
| Focus visibility | `:focus-visible` ring on the summary line satisfies keyboard-navigation discoverability |
| Subsection colour signal | Coloured left-border on each subsection heading is supplementary to the heading text — colour is not the sole differentiator |
| Reduced-motion | `prefers-reduced-motion: reduce` media query disables any expand/collapse transition |

## Brand-voice alignment

- **Confident** — the trail names what the editor knows they don't know. Open questions are first-class, not buried.
- **Direct** — three subsections, fixed names. No "Notes" / "TODO" / "Considerations" — the language is operational.
- **Professional** — collapsed by default; respects the reader who came for the article body, not the editorial frontier. A reader who opens it gets the editorial discipline made visible.

## AI-consumption hint

The substrate emits the research trail as JSON-LD `potentialAction` nodes on the article's `TechArticle` schema:

```json
{
  "@context": "https://schema.org",
  "@type": "TechArticle",
  "potentialAction": [
    {
      "@type": "SearchAction",
      "name": "seL4 capability-revocation 2024–2026 papers — verify transitive revocation behaviour",
      "category": "research-suggested"
    },
    {
      "@type": "Question",
      "name": "Does the seL4 microkernel revoke capabilities transitively when a parent capability is destroyed?",
      "category": "research-open"
    }
  ]
}
```

LLM consumers identify the article's epistemic frontier — what is known, what should be researched next, what remains unanswered — without reading article prose. This addresses the structural weakness in §3 weakness #10 of the research draft (Wikipedia's reading-surface HTML provides no per-section semantic hints; AI consumers fall back to undifferentiated full-article scraping).

## Token references

Per `token-knowledge-wiki-baseline.draft.md` §4 `article.research-trail.*` group:

- `--article-research-trail-done-color` → `{color.status.success.base}`
- `--article-research-trail-suggested-color` → `{color.brand.blue.60}`
- `--article-research-trail-open-question-color` → `{color.status.warn.base}`
- `--article-research-trail-trail-bg` → `{surface.layer-accent}` (chrome posture) OR `{surface.layer}` (body-content posture; pending §7(f) ratification)
- `--article-research-trail-trail-border` → `{border.subtle}`

## Research trail (this component's own)

### Done

- [research-wikipedia-leapfrog-2030.draft.md §3 weakness #8] — no citation trail back to source fact, addressed structurally by exposing the trail
- [research-wikipedia-leapfrog-2030.draft.md §6.2] — first-class leapfrog primitive
- [conventions/draft-research-trail-discipline.md] — workspace-canonical research-trail discipline (Doctrine claim #39) — this component extends the draft pattern to article surface
- [external:schema.org/Question] — formal `Question` type for open-question emission
- [external:schema.org/SearchAction] — formal `SearchAction` type for suggested-research emission

### Suggested

- [external:carbondesignsystem.com/components/accordion/usage] — verify Carbon Accordion vs native `<details>` choice (priority: medium)
- [pointsav-design-system/research/] — check for existing research on disclosure-pattern conventions (priority: medium)
- [external:w3.org/WAI/ARIA/apg/patterns/disclosure] — disclosure pattern accessibility checklist (priority: high)

### Open questions

- §7(f) of the research draft — visual weight as chrome vs body content. Substrate ratifies; affects `--article-research-trail-trail-bg` token resolution.
- Render-time emission of trail counts in summary line — engine-side scope; should the engine read the frontmatter `research_done_count` / `research_suggested_count` / `open_questions_count` fields directly OR count items in the body subsections at render time? (Engine implementation detail; surface to project-knowledge cluster for next iteration.)

## Provenance

Synthesizes Doctrine claim #39 (draft research trail discipline) into an article-scale rendering recipe. Direct line of descent: cluster Tasks stage drafts with frontmatter trail fields → project-language gateway preserves the trail in refined output → content-wiki-documentation Root commits the article → engine renders the article with this component → reader sees the editorial pipeline's epistemic position.

This component is the visible-operational manifestation of the editorial-substrate Doctrine. Wikipedia has no equivalent — the Talk namespace is editorial discussion record, not editorial epistemic-frontier signal.
