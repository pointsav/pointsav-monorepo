---
schema: foundry-draft-v1
state: master-cosigned
originating_cluster: project-bim
target_repo: customer/woodfine-media-assets
target_path: token-global-color.yaml
audience: brand-canonical
bcsc_class: vendor-public
language_protocol: DESIGN-TOKEN-CHANGE
master_cosign: master@claude-code 2026-05-06T19:10Z
authored: 2026-04-29T00:35:00Z
authored_by: task-cluster-project-bim
authored_with: claude-opus-4-7
research_done_count: 3
research_suggested_count: 1
open_questions_count: 1
research_provenance:
  - cluster sub-agent BB.13 — `~/Foundry/clones/project-bim/.claude/sub-agent-results/BB.13-design-system-showcase-survey-2026-04-28.md`
  - cluster sub-agent BB.14 — `~/Foundry/clones/project-bim/.claude/sub-agent-results/BB.14-design-system-website-pick-2026-04-28.md`
  - workspace-tier sub-agent C — `~/Foundry/.claude/sub-agent-results/C-bim-regulatory-acceptance-2026-04-28.md`
research_inline: true
notes_for_editor: |
  Cross-repo handoff to woodfine-media-assets (admin-only repo;
  mcorp-administrator commits; not project-bim Task scope). Proposes
  small-step additions to Woodfine's canonical palette to cover AEC
  semantic colors that the BIM showcase + future Woodfine BIM
  surfaces require. Each addition is documented with the AEC
  vernacular role it fills.
---

# Woodfine palette additions — small-step forward

Per cluster v0.0.2 visual upgrade work (operator framing 2026-04-28
"OK to iterate forward these tokens as they do need upgrade, but we
should just take small steps forward"). The current Woodfine canonical
palette in `token-global-color.yaml` covers the institutional brand
spine (canvas, slate, drafting-blue, grey-mid, grey-light, white,
black-pure). It does not yet cover semantic colors needed for AEC
domain surfaces — regulation alerts, MEP indicators, IDS-validation
outcomes, clash markers.

The BIM showcase (`bim.woodfinegroup.com`) is the first Woodfine
surface to need these. Future surfaces (`app-console-bim`,
operational dashboards, future BIM authoring tools) will reuse them.

## Proposed additions

Add to `customer/woodfine-media-assets/token-global-color.yaml`:

```yaml
# AEC semantic palette additions — small-step forward (cluster/project-bim
# v0.0.2; per BB.13 AEC vernacular research + BB.14 design-system-website
# pick). These cover regulation, MEP, validation-outcome, and clash
# semantics that the institutional brand spine does not address.

  woodfine-amber: "#B54708"       # Regulation-warning / IDS validation alert
  woodfine-amber-bg: "#FFF8ED"    # Soft amber tint surface

  woodfine-cyan: "#0E7490"        # MEP / systems-element indicator
                                  # (the AEC sector's conventional teal for
                                  #  ductwork / piping / electrical systems)
  woodfine-cyan-bg: "#ECFEFF"     # Soft cyan tint surface

  woodfine-error: "#B42318"       # Clash / IDS validation failure / regulation violation
  woodfine-error-bg: "#FEF3F2"    # Soft error tint surface

  woodfine-green-bg: "#ECFDF3"    # Soft green tint surface (companion to existing
                                  # accent-secure / IDS validation pass)
```

## Promote `accent-secure` to global

The existing `theme-woodfine-light.css` carries `--accent-secure: #54924E`
("Status: Verified/Safe") but that color is NOT in
`token-global-color.yaml` as a primitive — it lives only in the theme
mapping layer. Promote it to a top-level token:

```yaml
  woodfine-green: "#54924E"       # Verified/safe state — IDS pass, compliance ok
  woodfine-green-bg: "#ECFDF3"    # Soft green tint surface (above)
```

Then update `theme-woodfine-light.css` to reference the global token:

```diff
- --accent-secure: #54924E;
+ --accent-secure: #54924E;       /* maps to woodfine-green; see token-global-color.yaml */
```

(Or replace with a `var(--woodfine-green)` reference once a CSS layer
emits the global tokens directly.)

## Why each color

### `woodfine-amber: #B54708`

Regulation amber. Used by every BIM coordination tool (Solibri,
Navisworks, BIMcollab) for the warning-level state — "validation
outcome requires attention but is not failing." In the BIM showcase
v0.0.2, this color anchors the **Code overlays** chip (per Doctrine
claim #41 candidate — City Code as Composable Geometry) when overlays
are registered against an IFC anchor. It also anchors the clash-warning
chip when an IDS validation produces non-blocking findings.

The hex `#B54708` is in the same regulation-amber family used by
Adobe Spectrum's warning palette (`#DA7B11`) and Polaris's warning
(`#B98900`), at a slightly desaturated tone that pairs cleanly with
Woodfine's drafting-blueprint navy `#164679`.

### `woodfine-cyan: #0E7490`

MEP / systems indicator. Conventional teal-cyan used in 3D BIM model
views to signal mechanical, electrical, and plumbing systems. In the
BIM showcase v0.0.2, this anchors the Uniclass-classification chip
(`Uniclass SL`, `Uniclass EF_25_10`) so a glance at a token page
immediately distinguishes the IFC class chip (navy) from the
classification chip (cyan).

The cyan is also the conventional fill color for `IfcDistributionElement`
family elements — pipes, ducts, cables — in BIM viewport rendering
(matches Bonsai, BricsCAD, ArchiCAD MEP system display defaults).

### `woodfine-error: #B42318`

Clash / IDS validation failure / regulation violation. The "compliance
fails" state. Required for any deployment surface that surfaces IDS
validation results (the `service-codes` `IdsValidationResult.passed:
false` case will render with this color). Critical for the City Code
as Composable Geometry workflow when violations are detected.

### `woodfine-green` (promotion of `accent-secure`)

The existing `accent-secure: #54924E` is the "verified/safe" state in
the theme CSS but isn't a primitive token. Promoting it to
`woodfine-green` makes it consumable by any future Woodfine surface
without going through theme-mapping indirection. Companion `bg` for
soft-fill surfaces.

## v0.0.2 BIM showcase usage

The cluster ships a CSS file at
`pointsav-monorepo/app-orchestration-bim/src/style.css` that mirrors
these tokens locally as `--bim-amber`, `--bim-cyan`, `--bim-success`,
`--bim-error` (with their `-bg` variants) — matching Woodfine canonical
hexes. Once these additions land in `token-global-color.yaml`, the BIM
showcase can either continue mirroring (offline-first; no network
fetch) or consume directly via a build step.

## Cross-repo handoff procedure

1. Master Claude reads this draft.
2. Master coordinates with the holder of mcorp-administrator commit
   authority (the System Administrator, per `~/Foundry/CLAUDE.md` §1).
3. Admin-tier commit (per `~/Foundry/CLAUDE.md` §8 admin-tier procedure)
   adds the 7 new tokens to `token-global-color.yaml` and updates
   `theme-woodfine-light.css` if the green-promote path is taken.
4. Single commit on the `customer/woodfine-media-assets` repo;
   commit message references this draft.
5. After landing, Master signals project-bim cluster Task to update
   the BIM showcase's `style.css` comment block to remove the
   "proposed additions" framing (the tokens become canonical).

## Open question

Should we add a fifth semantic color — **MEP-electrical-yellow**
(approximately `#CA8A04`) for the electrical-system-element indicator?
Conventional 3D BIM views often render electrical elements in a
distinct yellow that's not the same as regulation-amber. AEC visual
literacy would benefit from the distinction. Defer to a later
addition if the v0.0.2 showcase doesn't surface electrical-specific
content (it doesn't yet — service-buildings.elements.dtcg.json
covers IfcCableSegment but no per-element renderings exist yet).

## Research trail

### Done
- BB.13 (workspace 2026-04-28): catalogued AEC color conventions across Revit, ArchiCAD, BricsCAD, Bonsai, xeokit. The `#B54708` / `#0E7490` / `#B42318` family is conservative-vs-saturated within each role. Validated against Spectrum, Polaris, Atlassian semantic-color systems.
- BB.14 (workspace 2026-04-28): recommended the same hexes when adapting the design-system-website pick (Adobe Spectrum) for AEC audience. Cross-walked against woodfine-media-assets/token-global-color.yaml.
- Sub-agent C (workspace 2026-04-28): government regulatory acceptance research confirms the BIM showcase needs IDS-validation-outcome surfacing for US/EU government project workflows.

### Suggested
- Add MEP-electrical-yellow (~`#CA8A04`) at v0.0.3 if/when the showcase or any other Woodfine surface renders electrical-system content distinctly.

### Open questions
- Should the new tokens carry `(BIM-derived)` annotation in `token-global-color.yaml` to mark provenance? Or treat them as full Woodfine palette? Operator preference.
