---
schema: foundry-draft-v1
version: "1.0"
draft_id: design-component-bim-viewport-3d-2026-05-06
language_protocol: DESIGN-COMPONENT
state: ready-for-sweep
created: 2026-05-06T22:00:00Z
research_done_count: 4
research_suggested_count: 0
open_questions_count: 1
research_provenance: "pointsav-design-system/components/bim-viewport-3d/{recipe.html,recipe.css,aria.md}; sub-agent A+B research 2026-04-28 decisions BB.2 (xeokit) + BB.3 (Tauri 2.10 asset protocol); bim-design-philosophy.md"
research_inline: false
route_to: project-design
target_path: pointsav-design-system/components/bim-viewport-3d/
open_question_1: "Console fallback: should the non-AGPL console surface render a static SVG plan thumbnail, a 2D CAD-style floor plan from IFC, or an empty placeholder? Decision affects whether the EUPL-1.2 license boundary holds for app-console-bim."
---

# bim-viewport-3d — component recipe

## Identity

| Field | Value |
|---|---|
| Component name | `bim-viewport-3d` |
| IFC anchor | `IfcGeometricRepresentationItem` |
| Uniclass 2015 | FI_60_30 |
| Surface scope | Universal AEC — workplace + console |
| Mode prop | `data-mode="workplace"` \| `data-mode="console"` |
| Container element | `<section class="bim-viewport-3d">` |
| ARIA role | `<section>` with `aria-label="3D viewport"` |
| License note | Workplace 3D integration is AGPL-3.0 (xeokit coupling) |

## Purpose

Hosts the 3D model canvas. The recipe defines the chrome — navcube,
toolbar, status bar, and container shell — without prescribing the 3D
engine. At v0.0.1 the recipe shows a placeholder surface. The runtime
integration in `app-workplace-bim` mounts `xeokit-bim-viewer` into
`.bim-viewport-3d__canvas`.

The console surface may use a non-3D fallback (static SVG plan
thumbnail or 2D floor plan) to keep the console app EUPL-1.2 and
avoid pulling the AGPL-3.0 xeokit dependency into the read-only path.
This decision is open (see frontmatter `open_question_1`).

## Visual anatomy

```
.bim-viewport-3d (section, aria-label="3D viewport")
  .bim-viewport-3d__chrome
    .bim-viewport-3d__navcube (aria-hidden="true")
      .bim-viewport-3d__navcube-face "N/S/E/W/T/B"
    .bim-viewport-3d__toolbar (role="toolbar", aria-label="Viewport controls")
      .bim-viewport-3d__tool  (Home, Fit, Ortho/Persp, Section, Annotate)
    .bim-viewport-3d__status
      .bim-viewport-3d__element-label "IfcWall — WAL-EXT-001"
      .bim-viewport-3d__coord "X: 12.4  Y: 0.0  Z: 3.2"
  .bim-viewport-3d__canvas (aria-label="3D model view")
  .bim-viewport-3d__overlay
    .bim-viewport-3d__placeholder (visible when no IFC loaded)
```

## Mode-prop behaviour

| Behaviour | `workplace` | `console` |
|---|---|---|
| 3D engine | xeokit-bim-viewer (AGPL-3.0) | SVG plan thumbnail or placeholder |
| Element selection | Click → emit GUID → sync SpatialTree + PropertiesPanel | Click → emit GUID → read-only highlight |
| Section planes | Editable — create/move/delete | Show existing; no edit |
| BCF viewpoint capture | Camera → BCF viewpoint JSON | Disabled |
| Orbit/pan/zoom | Touch + mouse | Touch + mouse (read-only) |
| IFC model swap | Permitted | Disabled |

## Touch / mobile controls (BB.3 Tauri)

Per BB.3 research (2026-04-28):
- Never pipe IFC geometry over Tauri IPC — use `convertFileSrc` + asset protocol
- Touch: one finger = orbit; two fingers = pan; pinch = zoom
- Channel<T> for live progress updates during model load
- Sidecar SHA-256 verify before any IFC parser invocation

## ARIA contract

- Container: `role="region"` or `<section>` with `aria-label="3D viewport"`
- Canvas: `<div role="img" aria-label="3D model view">` (canvas is not natively
  accessible; the ARIA label provides the landmark)
- Toolbar: `role="toolbar"`, individual tools are `<button>` with `aria-label`
- Navcube: `aria-hidden="true"` — decorative orientation aid, not operable

## CSS token dependencies

- `--bim-bg-surface` — chrome background
- `--bim-border` — chrome dividers
- `--bim-accent` — selected element highlight colour
- `--bim-font-mono` — coordinate readout, element GUID
- `--bim-radius-md` — navcube corner rounding

## Not part of this component

- xeokit runtime initialisation: `app-workplace-bim/src/main.rs`
- IFC file loading sequence: per BB.3, loaded via Tauri asset protocol,
  not streamed through IPC
- Section plane state management: bim-section-plane component
- BCF issue creation: bim-annotation-layer component
