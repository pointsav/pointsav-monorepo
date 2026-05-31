---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: master
target_repo: pointsav-design-system
target_path: components/map-stats-panel/
target_filename: recipe.html
audience: vendor-public
bcsc_class: current-fact
language_protocol: DESIGN-COMPONENT
authored: 2026-04-30T04:00:00Z
authored_by: master-claude (workspace v0.1.94 session)
authored_with: opus-4-7
component_metadata:
  component_name: map-stats-panel
  component_kind: data-display
  carbon_baseline: tile (with-stat variant)
  accessibility_targets:
    - wcag-2-2-aa
    - live-region-polite-on-update
    - landmark-region-role
  brand_voice_alignment:
    - confident
    - direct
    - data-forward
  preview_html: |
    <aside class="ps-map-stats" role="region" aria-label="Map statistics" aria-live="polite">
      <dl class="ps-map-stats__grid">
        <div class="ps-map-stats__cell">
          <dt class="ps-map-stats__label">Corridors</dt>
          <dd class="ps-map-stats__value">12</dd>
        </div>
        <div class="ps-map-stats__cell">
          <dt class="ps-map-stats__label">Anchors</dt>
          <dd class="ps-map-stats__value">41</dd>
        </div>
        <div class="ps-map-stats__cell">
          <dt class="ps-map-stats__label">Countries</dt>
          <dd class="ps-map-stats__value">4</dd>
        </div>
        <div class="ps-map-stats__cell">
          <dt class="ps-map-stats__label">Avg cluster grade</dt>
          <dd class="ps-map-stats__value">&lt;1km</dd>
        </div>
      </dl>
    </aside>
research_done_count: 1
research_suggested_count: 3
open_questions_count: 1
research_provenance: sub-agent
research_inline: true
references:
  - https://gis.woodfinegroup.com  # live reference implementation
  - conventions/cluster-design-draft-pipeline.md
notes_for_designer: |
  Reference implementation: top-right floating card on the GIS surface.
  Reactive to filter state — when ALL is selected shows global counts;
  when a country chip is selected shows that country's counts; when
  family checkboxes are toggled shows filtered counts.

  Carbon's tile-with-stat is the closest baseline but it's static; this
  component is reactive. Use Carbon tile tokens but layer in
  aria-live="polite" + visual transition (number-flip or fade) on update.
---

# Component — Map Stats Panel

A floating data-display panel that surfaces aggregate statistics about the
current filtered map view. Always visible (not collapsed); updates
reactively when filters change. Positioned top-right by default to avoid
collision with map zoom controls (typically top-left or bottom-right).

## When to use

When a customer-public map surface holds aggregate-able data (counts,
averages, grades) and the user benefits from constant visibility of those
aggregates as filters change. Specifically:

- Co-location maps (current GIS use)
- Customer-portfolio dashboards with map view
- Future: federated cluster comparison views

Do not use for transient feedback (toast/snackbar) or for data that should
be revealed on-demand (drawer / collapsible panel).

## Anatomy

```
+---------------------------+
| 12      |   41            |
| Corr.   |   Anchors       |
+---------+-----------------+
| 4       |   <1km          |
| Countries|  Avg grade     |
+---------+-----------------+
```

A 2-column grid of stat cells; each cell contains:
- Large value (heading-level typography)
- Compact label below (caption-level typography)

Default 4 cells; component supports 2-6 cells with a responsive grid
(2-col / 3-col).

## Behaviour

- **Static positioning** — top-right of the map container; floats above
  the map; respects map padding token
- **Reactive updates** — when parent filters change (country chip / family
  checkbox), values update; the panel announces updates via
  `aria-live="polite"`
- **No interactivity** — values are read-only; clicking a stat does not
  filter or navigate

## Accessibility (WCAG 2.2 AA)

- `role="region"` + `aria-label="<surface-name> statistics"`
- `aria-live="polite"` on the panel; updates announced to screen readers
  without interrupting current speech
- Each stat cell uses `<dt>` + `<dd>` for label / value pairing
- Number value has aria-label spelling out unit when not implicit
  ("12 corridors", not just "12") — implementation: `<dd aria-label="12 corridors">12</dd>`
- Contrast 4.5:1 on label, 7:1 on value (AA + AAA respectively)

## Tokens (DTCG)

```yaml
ps.map-stats.bg: surface-elevated
ps.map-stats.padding: spacing-12
ps.map-stats.border-radius: 8px
ps.map-stats.shadow: shadow-floating
ps.map-stats.gap: spacing-08
ps.map-stats.value.font: heading-04
ps.map-stats.value.color: text-primary
ps.map-stats.label.font: caption-01
ps.map-stats.label.color: text-secondary
ps.map-stats.position.top: 16px
ps.map-stats.position.right: 16px
ps.map-stats.transition.duration: 200ms     # number-update fade
```

## Variants

- **Default** — 4 cells, 2x2 grid (current GIS use)
- **Compact** — 2 cells horizontal (filter-driven dashboards with single-axis stat)
- **Wide** — 6 cells, 3x2 grid (federated cluster comparison)
- **With sparkline** — each cell carries a small inline sparkline below
  the value (cluster-grade distribution; future Phase-2 use)

## AI consumption hint

```
"This component is a map statistics panel that shows aggregate data
about the current filtered view.
Use when: a customer-public map surface has aggregate-able data and
constant visibility of those aggregates is valuable as filters change.
Don't use when: stats are transient (toast) or on-demand (drawer)."
```

## Research trail

### Done

- [https://gis.woodfinegroup.com] reference implementation observed; 4-cell stats card top-right; reactive to filter state at v0.1.94.

### Suggested

- Validate `aria-live="polite"` cadence with NVDA + JAWS — does each filter-change announcement interrupt or queue politely?
- Measure stat-cell update animation impact on Lighthouse / Core Web Vitals; if number-flip animation degrades CLS, fallback to instant-replace.
- Test variant selection at panel-content overflow (e.g., 6 cells on 320px viewport) — responsive collapse to vertical stack vs scrollable horizontal.

### Open questions

- Should the panel auto-collapse on small viewports (mobile <640px) and expand on tap, or always stay visible? Decision pending mobile usage telemetry.

## Provenance

Reference implementation deployed via workspace v0.1.94 (2026-04-30) at
`gateway-orchestration-gis-1`. Pattern is one of the v0.1.94 UI/UX
upgrade primitives that the operator directed move into project-design.
