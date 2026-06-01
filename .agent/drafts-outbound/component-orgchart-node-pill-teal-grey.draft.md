---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-orgcharts
target_repo: pointsav-design-system
target_path: components/
target_filename: nodes.css
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-COMPONENT
authored: 2026-06-01T00:00:00Z
authored_by: totebox@project-orgcharts session-2026-06-01
authored_with: sonnet-4-6
component_metadata:
  component_name: org-chart-node-pill-variants
  component_kind: data-display
  carbon_baseline: tag (Carbon Tag component — pill shape semantics)
  accessibility_targets: [wcag-2-2-aa, reduced-motion-respect]
  brand_voice_alignment: [confident, direct, institutional]
  preview_html: |
    <!-- Teal pill — fund / flow-through entity -->
    <div class="org-token-pill org-token-pill--teal">
      <div class="zone-top"><span class="t-title">Fund Name Ltd.</span></div>
      <div class="zone-mid"><span class="t-code">FND-001</span></div>
    </div>
    <!-- Grey pill — inactive / placeholder entity -->
    <div class="org-token-pill org-token-pill--grey">
      <div class="zone-top"><span class="t-title">TBD Entity</span></div>
    </div>
research_done_count: 3
research_suggested_count: 2
open_questions_count: 1
research_provenance: sub-agent
research_inline: true
references:
  - archive-2026-06-01/pointsav-design-system/components/nodes.css
  - inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-27_Chart_Bencal_Organization_JW2.html
  - inputs/current-org-chart-html/INVESTOR_RELATIONS_2026-05-25_Chart_Bencal_SPV2_JW2.html
notes_for_designer: |
  The existing `.org-token-pill` in nodes.css is a single amber-role pill
  (250×110px, border-radius 110px, dashed border). The Bencal Organization_JW2
  and SPV2_JW2 charts introduce teal-pill and grey-pill variants using the
  same shape but different token colors and border styles.

  This draft proposes two modifier classes that extend the base pill.
  Base pill dimensions + interior zone-top/mid/bottom structure are unchanged.

  Dependency: teal variant references `--wf-teal` / `--wf-teal-tint` which
  are not yet in theme-woodfine.css. See companion token draft
  token-woodfine-theme-teal-red-additions.draft.md — process that draft
  first, or bundle the token additions here.
---

# DESIGN-COMPONENT — Org chart node pill variants: teal and grey

## Context

`nodes.css` has one pill shape: `.org-token-pill` (amber role, dashed border).
The Bencal Organization_JW2 and SPV2_JW2 charts introduced two additional
pill uses:

- **Teal pill** (Organization_JW2, SPV2_JW2): fund / flow-through entity
  shown as a rounded container — same shape as the amber pill but teal
  border/background, and dotted (not dashed) border style.
- **Grey pill** (Organization_JW2): placeholder / TBD entity — grey border,
  neutral background, dotted border. Signals "structure not yet finalized."

Both use the same 250×110px dimensions and border-radius: 110px as the
existing `.org-token-pill`.

## Proposed recipe — addition to `nodes.css`

Add after the existing `.org-token-pill` and `.org-token-pill--sm` lines:

```css
/* Teal pill — fund / flow-through entity role */
.org-token-pill--teal {
    background: var(--wf-teal-tint);
    border-color: var(--wf-teal);
    border-style: dotted;
}

/* Grey pill — placeholder / TBD entity */
.org-token-pill--grey {
    background: var(--wf-grey-tint);
    border-color: var(--wf-grey);
    border-style: dotted;
}
```

Usage: add modifier class alongside `.org-token-pill`:
```html
<div class="org-token-pill org-token-pill--teal"> ... </div>
<div class="org-token-pill org-token-pill--grey"> ... </div>
```

The base `.org-token-pill` retains its amber/dashed role. Modifier classes
override `background`, `border-color`, and `border-style` only — all other
dimensions, padding, and interior zone structure inherited from the base.

## Print rule

Add to the `@media print` block in nodes.css (after the existing pill
print overrides):

```css
.org-token-pill--teal,
.org-token-pill--grey { background: #FFF !important; }
```

## ARIA notes

Pill nodes carry the same implicit role as standard org-chart nodes —
they are containers for entity information, not interactive controls.
No additional ARIA attributes required beyond what standard `.org-token`
nodes carry. Dotted border is a purely visual distinction; a screen-reader
context label (e.g., `aria-label="Flow-through entity: Fund Name Ltd."`)
should be added by the chart author at the instance level.

## When to use

| Pill type | Role | When |
|---|---|---|
| Base (amber, dashed) | Holding structure / SPV | Direct-hold or special purpose vehicle |
| `--teal` (dotted) | Fund / flow-through entity | Pass-through investment vehicle, fund LP |
| `--grey` (dotted) | Placeholder / TBD | Structure not yet finalized; awaiting decision |

---

## Research trail

### Done

1. Confirmed `nodes.css` has `.org-token-pill` (amber, dashed) only —
   no teal or grey pill modifier exists.
2. Confirmed both Bencal charts use the pill shape with inline `.token-teal-pill`
   and `.token-grey-pill` classes (not the design-system convention `.org-token-pill--*`).
3. Confirmed `--wf-grey: #6B7280` and `--wf-grey-tint: #e9ecef` already exist
   in `theme-woodfine.css` — grey pill can be rendered immediately.
   Teal pill depends on the companion token-change draft.

### Suggested

1. project-design to verify dotted border (not dashed) is the right
   distinction for teal/grey pills vs the base amber pill (dashed).
   The Bencal charts consistently use dotted for these two variants — confirm
   this is intentional semantic differentiation.
2. project-design to check whether a `--sm` modifier is needed for teal/grey
   pills (`.org-token-pill--teal.org-token-pill--sm`), parallel to the
   existing `.org-token-pill--sm`. Not observed in the Bencal charts but
   may be needed for dense layouts.

### Open questions

1. **Token dependency**: the teal pill references `--wf-teal` / `--wf-teal-tint`
   which don't exist in `theme-woodfine.css` yet. Should this component
   draft wait for the token-change to be ratified, or should project-design
   land it with a fallback hardcoded `#005D5D` / `#9EF0F0` pending the token?
   Fallback approach would allow the component to ship sooner but introduces
   temporary drift.
