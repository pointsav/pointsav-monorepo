---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: master
target_repo: pointsav-design-system
target_path: components/map-side-drawer/
target_filename: recipe.html
audience: vendor-public
bcsc_class: current-fact
language_protocol: DESIGN-COMPONENT
authored: 2026-04-30T03:45:00Z
authored_by: master-claude (workspace v0.1.94 session)
authored_with: opus-4-7
component_metadata:
  component_name: map-side-drawer
  component_kind: overlay
  carbon_baseline: side-panel
  accessibility_targets:
    - wcag-2-2-aa
    - focus-visible
    - reduced-motion-respect
    - keyboard-dismiss-esc
    - focus-trap-while-open
  brand_voice_alignment:
    - confident
    - direct
    - data-forward
  preview_html: |
    <aside class="ps-map-drawer" aria-label="Anchor details" role="complementary">
      <header class="ps-map-drawer__header">
        <span class="ps-brand-badge ps-brand-badge--hardware">HD</span>
        <h2 class="ps-map-drawer__title">Home Depot — Bellingham Bakerview</h2>
      </header>
      <dl class="ps-map-drawer__facts">
        <dt>Address</dt><dd>4255 Meridian St, Bellingham WA 98226</dd>
        <dt>NAICS</dt><dd>444110</dd>
        <dt>Open</dt><dd>2002</dd>
      </dl>
    </aside>
research_done_count: 3
research_suggested_count: 4
open_questions_count: 2
research_provenance: sub-agent
research_inline: true
references:
  - https://gis.woodfinegroup.com  # live reference implementation
  - vendor/pointsav-design-system/components/  # carbon baseline alignment
  - conventions/cluster-design-draft-pipeline.md
notes_for_designer: |
  Reference implementation is live at https://gis.woodfinegroup.com — click
  any individual anchor at zoom >= 8 to see the drawer in action. The
  hand-authored HTML lives at deployments/gateway-orchestration-gis-1/www/index.html.
  This component is the cleanest extraction target for the DTCG vault because
  the drawer pattern recurs across every customer-public surface (orchestration,
  bookkeeping, presentation). project-design should align tokens to Carbon's
  side-panel baseline but tighten:
    - 340px fixed width (not Carbon's 320px responsive)
    - 250ms cubic-bezier(0.16, 1, 0.3, 1) ease (not Carbon's default)
    - Inset shadow on left edge (depth cue without full overlay scrim)
    - Brand badge at top-left in title block (not Carbon's leading icon slot)
---

# Component — Map Side Drawer

The side drawer info card pattern from `gis.woodfinegroup.com`. Replaces
the legacy popup-on-marker pattern with a persistent right-side panel that
slides in when an anchor is selected and stays visible while the map remains
interactive.

## When to use

When a map view exposes per-feature detail that a user wants to read while
keeping the map context. Specifically:

- Retail co-location maps (current GIS surface)
- Building-envelope detail with mobility composition (project-bim crossover)
- Future: cluster comparison views (federated cluster comparison invention #9)

Do not use for transient confirmation messages — those belong in toast/snackbar.

## Anatomy

```
+--------------------------+
| [BADGE] Title            |  ← header block
| Subtitle                 |
+--------------------------+
| Address line             |
|                          |
| Facts grid (dl)          |
|   NAICS    | 444110      |
|   Open     | 2002        |
|   Placekey | abc-def-...  |
+--------------------------+
| Cluster context block    |  ← optional; only when feature has parent
| (Within Cluster X / 4    |
|  anchors / 730m max...)  |
+--------------------------+
```

## Behaviour

- **Open** — triggered by feature click on the map. Slides in from the right
  edge over 250ms with cubic-bezier(0.16, 1, 0.3, 1) ease. `prefers-reduced-motion`
  collapses to a 0ms transition with opacity fade only.
- **Close** — close affordance in top-right (icon-button); ESC key when focus
  is inside the drawer; clicking another feature replaces the drawer content
  rather than closing/re-opening.
- **Focus management** — on open, focus moves to the drawer's first interactive
  element. ESC dismisses and returns focus to the map canvas. While open, focus
  is trapped inside the drawer (Tab cycles within).
- **Map interaction** — the map remains pannable and zoomable behind the drawer
  (drawer is `aria-modal="false"`). The drawer is a complementary landmark, not
  a modal.

## Accessibility (WCAG 2.2 AA)

- `role="complementary"` + `aria-label="<feature-type> details"`
- Heading inside the drawer is the page's secondary heading (h2 if map is h1)
- All interactive controls reachable by keyboard
- Close affordance is a real `<button>` with `aria-label="Close details"`
- Brand badge has accessible name (visible text or `aria-label`); colour is
  not the only family signifier
- Focus indicator visible on every interactive element (focus-visible)

## Tokens (DTCG)

```yaml
ps.map-drawer.width: 340px
ps.map-drawer.bg: surface-elevated
ps.map-drawer.border-color: border-subtle
ps.map-drawer.shadow: shadow-side-panel
ps.map-drawer.transition.duration: 250ms
ps.map-drawer.transition.easing: cubic-bezier(0.16, 1, 0.3, 1)
ps.map-drawer.padding: spacing-16
ps.map-drawer.title.font: heading-03
ps.map-drawer.fact.label.color: text-secondary
ps.map-drawer.fact.value.color: text-primary
```

## Variants

- **Default** — single-feature detail (current GIS use)
- **Comparison** — split drawer with two features side-by-side; for federated
  cluster comparison (Doctrine invention #9 from v0.1.94 leapfrog list)
- **Forward-looking** — banner at top of drawer when content is forward-looking
  per BCSC posture; uses `ps-disclosure-banner` component

## AI consumption hint

```
"This component is a side drawer overlay for map feature detail.
Use when: user clicks a feature on a customer-public map surface
and wants to read about it without losing map context.
Don't use when: the detail is transient (toast/snackbar)
or requires user action that blocks map interaction (modal)."
```

## Research trail

### Done

- [https://gis.woodfinegroup.com] reference implementation observed in production with 41 anchors across 4 countries; drawer slide animation confirmed at 250ms; click-through to drawer measured; ESC dismiss validated.
- [vendor/pointsav-design-system/components/] Carbon side-panel baseline reviewed — Carbon's pattern is responsive to viewport; this component tightens to 340px fixed because the use is map-overlay, not responsive content.
- Sonnet sub-agent retail-mapping-UI-research dispatched 2026-04-30 returned: Linear / Vercel / Stripe convergent on 320-360px right-drawer; 200-280ms transitions; Inter typography. This pattern aligns with the convergent enterprise dashboard standard.

### Suggested

- Validate focus-trap behaviour on real screen-reader pairs (NVDA + Firefox; JAWS + Edge; VoiceOver + Safari) before public design-system release.
- Measure `prefers-reduced-motion` adoption rate via service-content telemetry once design-system is shipping; informs whether the reduced-motion fallback is the dominant path.
- Test drawer behaviour on touchscreens (iPad Pro, Surface Pro) — does the swipe-to-dismiss gesture conflict with map pan?
- Audit colour contrast for brand-family badges against `surface-elevated` background at WCAG 2.2 AA + AAA.

### Open questions

- Should the drawer expand to full-width on viewports below 640px, or remain a 340px overlay with map underneath? Carbon's responsive pattern would say expand; the GIS UX rationale says preserve map context.
- Cluster comparison variant — does it occupy two drawer columns (640px total), or split-screen with map between? Decision pending project-bim cluster-comparison feature work.

## Provenance

Reference implementation deployed via workspace v0.1.94 (2026-04-30) at
`gateway-orchestration-gis-1`. Operator chat-surface direction:
*"need a UI/UX upgrade ... when you click on it youcan see info about it
... lets get some retail higher-end mappign UI/UX patterns/themes and
apply them ... we'll move these compoents in to project-design as well"*.
