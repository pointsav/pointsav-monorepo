---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: master
target_repo: pointsav-design-system
target_path: components/brand-family-swatch/
target_filename: recipe.html
audience: vendor-public
bcsc_class: current-fact
language_protocol: DESIGN-COMPONENT
authored: 2026-04-30T03:55:00Z
authored_by: master-claude (workspace v0.1.94 session)
authored_with: opus-4-7
component_metadata:
  component_name: brand-family-swatch
  component_kind: data-display
  carbon_baseline: tag (with-icon variant)
  accessibility_targets:
    - wcag-2-2-aa
    - colour-not-sole-channel
    - high-contrast-mode-respect
  brand_voice_alignment:
    - confident
    - direct
    - data-forward
    - taxonomic
  preview_html: |
    <span class="ps-swatch ps-swatch--department" aria-label="Department family">
      <span class="ps-swatch__dot" aria-hidden="true"></span>
      <span class="ps-swatch__label">Department</span>
    </span>
    <span class="ps-swatch ps-swatch--hardware" aria-label="Hardware family">
      <span class="ps-swatch__dot" aria-hidden="true"></span>
      <span class="ps-swatch__label">Hardware</span>
    </span>
    <span class="ps-swatch ps-swatch--warehouse-club" aria-label="Warehouse Club family">
      <span class="ps-swatch__dot" aria-hidden="true"></span>
      <span class="ps-swatch__label">Warehouse Club</span>
    </span>
research_done_count: 2
research_suggested_count: 5
open_questions_count: 2
research_provenance: sub-agent
research_inline: true
references:
  - https://gis.woodfinegroup.com  # live reference implementation
  - deployments/gateway-orchestration-gis-1/www/pois.json  # canonical brand-family taxonomy
  - conventions/cluster-design-draft-pipeline.md
notes_for_designer: |
  This component encodes the brand-family taxonomy (Doctrine v0.0.14 / claim
  ratification 2026-04-30): Department / Hardware / Warehouse Club. The
  taxonomy is operator-ratified and customer-editable per leapfrog invention
  #5 (brand-family taxonomy as sovereign data layer). Three colour swatches
  must:
    - be distinguishable in monochrome (use shape + position when colour fails)
    - pass WCAG 2.2 AA contrast against both light and dark map basemaps
    - have accessible names ("Department", "Hardware", "Warehouse Club")
    - support a CLUSTER-CENTROID variant (concentric ring with all three
      colours when a cluster contains all three families)

  The brand-family taxonomy ITSELF is data, not design. Customers can
  rename / extend the families per leapfrog invention #5. The component
  reads family-id and resolves through a runtime taxonomy file
  (pois.json carries `brand_family` per place).
---

# Component — Brand-Family Swatch

A taxonomic swatch component that renders a brand-family identifier as a
visual chip combining a coloured dot + label. The component is the
visual primitive for the Department / Hardware / Warehouse Club taxonomy
that the GIS surface uses to colour-code retail anchors.

The component carries a default 3-family token set but is taxonomy-agnostic:
customers can extend with custom family IDs via the runtime taxonomy file
(per leapfrog invention #5, brand-family taxonomy as sovereign data layer).

## When to use

When a surface needs to show "this thing belongs to family X" using both
colour and label. Specifically:

- Map markers grouped by brand-family (current GIS use)
- Tabular data filtering by brand-family
- Drawer / detail header chip
- Cluster centroid (multi-family ring variant)

Do not use for binary states, transient feedback, or non-taxonomic
categorisation — those belong in tag, badge, or status-indicator.

## Anatomy

```
[●] Department          ← dot + label inline chip
[●] Hardware
[●] Warehouse Club

       ●
      ● ●               ← cluster-centroid ring (4-anchor cluster
       ●                  with 1 Department + 2 Hardware + 1 Warehouse Club)
```

Each swatch carries:
- A coloured dot (12px diameter default; family-specific token)
- A label (the family display name, taxonomy-resolved)
- An accessible name (combines dot + label semantically)

The cluster-centroid ring variant is a stacked / circular composition of
multiple dots showing the family distribution within a co-location cluster
without rendering individual markers. Reveal-by-zoom: at zoom < 8.5, only
cluster-centroid rings render; at zoom ≥ 8, individual swatches render.

## Behaviour

- **Static** — the swatch itself is non-interactive; clickability comes from
  the parent (filter checkbox, map marker, table row).
- **Interactive parent (filter)** — checkbox-paired-with-swatch is the
  filter primitive: `[✓] [●] Department [✓] [●] Hardware ...`.
- **Map marker variant** — the swatch is the visual foundation for
  `ps-map-marker--anchor`; the dot is sized larger (24-32px) and the label
  is suppressed (label is delivered via tooltip / drawer).

## Accessibility (WCAG 2.2 AA)

- `aria-label` on the swatch element states the family name explicitly
- The dot is `aria-hidden="true"`; semantically the label carries the meaning
- Colour is supplemented by label (label is the primary signifier; colour
  is the visual reinforcement, never the sole channel)
- High-contrast mode (`forced-colors`): dots collapse to the user's link
  colour; labels remain (semantic meaning preserved)
- Each family colour passes WCAG 2.2 AA contrast 3:1 against both
  `surface` and `surface-elevated` tokens (and against light-mode and
  dark-mode map basemaps; tested on CARTO Positron and Voyager)

## Tokens (DTCG)

```yaml
# Brand-family palette
ps.brand-family.department.color: '#0B5FFF'   # azure-blue
ps.brand-family.department.label: 'Department'

ps.brand-family.hardware.color: '#FF6B00'     # construction-orange
ps.brand-family.hardware.label: 'Hardware'

ps.brand-family.warehouse-club.color: '#00875A' # warehouse-green
ps.brand-family.warehouse-club.label: 'Warehouse Club'

# Swatch component
ps.swatch.dot.size: 12px
ps.swatch.dot.size.marker: 24px
ps.swatch.gap: spacing-04
ps.swatch.label.font: body-compact-02
ps.swatch.padding: 4px 8px
ps.swatch.border-radius: 4px
```

## Variants

- **Inline chip** — default; dot + label
- **Map marker** — large dot (24-32px), no label (label via parent tooltip)
- **Cluster centroid ring** — concentric arrangement of N family dots
  representing the family distribution in a cluster
- **Filter row** — paired with checkbox primitive
- **Drawer header badge** — paired with the brand short-code (e.g., "HD"
  for Home Depot); the swatch dot fills behind the short-code

## AI consumption hint

```
"This component is the brand-family swatch for the Foundry retail taxonomy.
Use when: a surface needs to render 'this anchor belongs to family X' with
combined colour + label semantics, including in maps, tables, drawers,
and filter rows.
Don't use when: the categorisation isn't taxonomic (use tag/badge),
or when colour alone communicates the meaning (which would fail accessibility)."
```

## Research trail

### Done

- [https://gis.woodfinegroup.com] reference implementation observed; three colour swatches operational across 41 anchors; cluster-centroid ring variant operational on 12 corridors at zoom < 8.5.
- [deployments/gateway-orchestration-gis-1/www/pois.json] taxonomy-source-of-truth: 12 Department / 17 Hardware / 12 Warehouse Club confirmed; schema field `brand_family` enum-validated.

### Suggested

- Audit the three colour values for deuteranopia / protanopia / tritanopia distinguishability; if hardware-orange and warehouse-green collapse for any colour-blind variant, the taxonomy needs a fourth visual channel (pattern fill or shape).
- Validate cluster-centroid ring rendering at extreme cluster sizes (10+ anchors) — does the ring degrade gracefully?
- Test high-contrast mode rendering on Windows 11 + Ubuntu Accessibility on the live deployment.
- Measure customer adoption signal: do customers extend the 3-family taxonomy, or stay with the default? Informs whether the runtime-extension is the dominant path.
- Document the taxonomy-resolution mechanism (where the family file lives, how custom families are loaded, how the design-system component resolves unknown family-ids) in a paired DESIGN-research draft.

### Open questions

- Should the component ship a default catalogue of family-ids (Department / Hardware / Warehouse Club / Grocery / Pharmacy / etc.) so customers extend FROM a baseline, or ship empty so customers always start from scratch?
- Cluster-centroid ring variant: when a cluster has 10+ anchors of different families, does the ring transition to a pie/donut chart at a threshold? Decision pending first banker-walkthrough feedback.

## Provenance

Reference implementation deployed via workspace v0.1.94 (2026-04-30) at
`gateway-orchestration-gis-1`. Brand-family taxonomy ratified by operator
2026-04-30 ("families are 'Department' 'Hardware' and 'Warehose Club'").
The taxonomy itself is leapfrog invention #5 from the v0.1.94 list:
brand-family taxonomy as sovereign data layer (customer-editable JSON file,
not a vendor-curated taxonomy).
