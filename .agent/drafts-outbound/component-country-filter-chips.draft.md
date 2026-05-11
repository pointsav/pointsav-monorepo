---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: master
target_repo: pointsav-design-system
target_path: components/country-filter-chips/
target_filename: recipe.html
audience: vendor-public
bcsc_class: current-fact
language_protocol: DESIGN-COMPONENT
authored: 2026-04-30T03:50:00Z
authored_by: master-claude (workspace v0.1.94 session)
authored_with: opus-4-7
component_metadata:
  component_name: country-filter-chips
  component_kind: navigation
  carbon_baseline: tag (selected variant) + content-switcher
  accessibility_targets:
    - wcag-2-2-aa
    - focus-visible
    - keyboard-arrow-navigation
    - role-radiogroup-semantic
  brand_voice_alignment:
    - confident
    - direct
    - geographic
  preview_html: |
    <div class="ps-country-chips" role="radiogroup" aria-label="Filter by country">
      <button type="button" class="ps-chip ps-chip--active" aria-checked="true" role="radio">ALL</button>
      <button type="button" class="ps-chip" aria-checked="false" role="radio">🇺🇸 US</button>
      <button type="button" class="ps-chip" aria-checked="false" role="radio">🇨🇦 CA</button>
      <button type="button" class="ps-chip" aria-checked="false" role="radio">🇲🇽 MX</button>
      <button type="button" class="ps-chip" aria-checked="false" role="radio">🇪🇸 ES</button>
    </div>
research_done_count: 2
research_suggested_count: 3
open_questions_count: 1
research_provenance: sub-agent
research_inline: true
references:
  - https://gis.woodfinegroup.com  # live reference implementation
  - conventions/cluster-design-draft-pipeline.md
notes_for_designer: |
  Reference implementation lives at deployments/gateway-orchestration-gis-1/www/index.html.
  Five chips: ALL | US | CA | MX | ES. ALL is exclusive (de-selects others when
  chosen); single-country chips fly the map to that country's bounds and filter
  the data. Carbon's content-switcher is the closest baseline but it's designed
  for view-state switching not data filtering — this component sits between
  Tag (selected variant) and content-switcher semantically.

  Future-proof for additional countries: layout should grow horizontally with
  optional overflow scroll past N=8. project-design owns the threshold.
---

# Component — Country Filter Chips

A horizontal row of country chips that filter map data and fly the viewport
to the selected country's bounds. Default state is `ALL` (no filter; world
view). Tapping any country chip filters to that country's anchors and
flyTo's the map to a country-extent zoom level.

## When to use

When a customer-public map surface holds data spanning multiple jurisdictions
and the user benefits from "show me just my country" cuts. Specifically:

- Multi-country location intelligence surfaces (current GIS use)
- Future: customer-portfolio dashboards where the customer operates in N countries
- Future: bilingual / multi-jurisdictional disclosure surfaces (BCSC + EU)

Do not use for layer toggles (those belong in a different component) or for
modes that change the entire view-state (Carbon's content-switcher).

## Anatomy

```
[ALL]  [🇺🇸 US]  [🇨🇦 CA]  [🇲🇽 MX]  [🇪🇸 ES]
  ^selected
```

Each chip carries:
- Optional ISO flag emoji (or named SVG icon for design-system polish)
- ISO 3166-1 alpha-2 country code (or "ALL" for the unfilter state)
- Selected / unselected visual states
- ARIA radiogroup semantics (mutually exclusive selection)

## Behaviour

- **Selection** — tapping a chip selects it and de-selects all others. The
  parent map filters data to that country's `iso_country_code` and animates
  to that country's bounds (flyTo, 700ms cubic-bezier).
- **ALL** — selecting `ALL` unfilters and animates to a world-extent default
  (e.g., zoom 2.6 centred on the data centroid).
- **Keyboard** — Tab focuses the chip group; arrow keys move between chips;
  Space/Enter activates.
- **Touch** — chips are minimum 44px tall (WCAG 2.2 target size AAA).

## Accessibility (WCAG 2.2 AA)

- `role="radiogroup"` on the container with `aria-label` describing the filter
- `role="radio"` + `aria-checked` on each chip (not `aria-pressed`; this is
  exclusive selection, not toggle)
- Flag emoji has accessible-text fallback (the ISO code is always rendered)
- Focus-visible on each chip; high-contrast mode supported
- Selected state is signalled by background AND border AND aria-checked, not
  colour alone

## Tokens (DTCG)

```yaml
ps.chip.height: 36px
ps.chip.padding: 0 12px
ps.chip.border-radius: 18px      # full pill
ps.chip.bg.default: surface
ps.chip.bg.selected: brand-primary
ps.chip.fg.default: text-primary
ps.chip.fg.selected: text-on-brand
ps.chip.border.default: border-subtle
ps.chip.border.selected: brand-primary
ps.chip.font: body-compact-01
ps.chip.transition.duration: 150ms
ps.chip.gap: spacing-08
```

## Variants

- **Default** — exclusive selection (radiogroup); current GIS use
- **Multi-select** — multiple chips can be active simultaneously; for
  cross-country composition (e.g., "show me US + CA only"); changes
  `role="radiogroup"` → `role="group"` and `aria-checked` → `aria-pressed`

## AI consumption hint

```
"This component is a horizontal radiogroup of country/region chips that
filters a map view and flyTo's the selected region.
Use when: a customer-public map surface holds multi-jurisdiction data
and users benefit from a quick country cut.
Don't use when: the chips would represent layer toggles (use checkbox)
or view modes (use content-switcher)."
```

## Research trail

### Done

- [https://gis.woodfinegroup.com] reference implementation operational with 5 chips (ALL/US/CA/MX/ES); flyTo-to-country-bounds confirmed on every chip click.
- Sonnet sub-agent retail-mapping-UI-research dispatched 2026-04-30 returned: Linear / Stripe / Vercel + premium dashboard convergent on pill-shaped exclusive-selection chips for primary segmentation; 36-44px height; pill border-radius (18-22px).

### Suggested

- Validate flag-emoji rendering across OS / browser combinations; some Linux distributions render flag emoji as text codes. Fallback strategy: pair emoji with text (already done; emoji is supplementary).
- Measure usability of country-selection vs continent-selection at N≥10 countries — is geographic grouping (Americas / Europe / APAC) better than flat country list?
- Audit colour contrast on selected vs unselected at WCAG AAA, not just AA.

### Open questions

- Multi-select variant: when activated, does `ALL` become disabled, or does it act as a "clear all" affordance? Decision pending first multi-select use-case.

## Provenance

Reference implementation deployed via workspace v0.1.94 (2026-04-30) at
`gateway-orchestration-gis-1`. Pattern is one of the v0.1.94 UI/UX
upgrade primitives that the operator directed move into project-design.
