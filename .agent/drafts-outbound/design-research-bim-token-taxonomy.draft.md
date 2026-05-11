---
schema: foundry-draft-v1
version: "1.0"
draft_id: design-research-bim-token-taxonomy-2026-05-06
language_protocol: DESIGN-RESEARCH
state: ready-for-sweep
created: 2026-05-06T22:00:00Z
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: "bim-token-taxonomy.md in pointsav-design-system/research/; sub-agent A report 2026-04-28; IFC 4.3 entity hierarchy (ISO 16739-1:2024); Uniclass 2015 NBS tables"
research_inline: false
route_to: project-design
target_path: pointsav-design-system/research/bim-token-taxonomy.md
---

# BIM token taxonomy — eight primitives and eighteen components

This research file documents the design rationale for the BIM Token
Catalog's eight primitive categories and eighteen interface component
recipes. It is an AI-readable substrate for BIM authoring assistance
and a human-readable design reference for engineers and system
architects.

## Why eight primitives

The eight categories were selected by mapping the full IFC 4.3 entity
hierarchy (ISO 16739-1:2024) against the built-environment design
decisions an architect or engineer must encode during a project
lifecycle. The taxonomy is not a personal invention — it reflects
how buildingSMART's IFC schema itself organises the built environment.

| Category | IFC anchor | Uniclass 2015 | Scope |
|---|---|---|---|
| SPATIAL | `IfcSpatialElement` | SL | Site / Building / Level (IfcBuildingStorey) / Spatial unit (IfcSpace) |
| ELEMENTS | `IfcBuiltElement` | EF_25 | Walls, slabs, columns, beams, doors, windows, roofs, stairs, railings, coverings |
| SYSTEMS | `IfcDistributionElement` | SS | HVAC, plumbing, electrical distribution, fire protection |
| MATERIALS | `IfcMaterial` + bSDD | Pr | Single materials, layer sets, constituent sets, Pset_Material* |
| ASSEMBLIES | `IfcElementAssembly` | EF | Curtain walls, stair assemblies, roof systems, furniture groupings |
| PERFORMANCE | `IfcPropertySet` + `IfcQuantitySet` | — | Pset_*Common templates, Qto_*BaseQuantities |
| IDENTITY + CODES | `IfcClassificationReference` + `IfcConstraint` | Co | Uniclass / OmniClass / MasterFormat + jurisdictional constraint references |
| RELATIONSHIPS | `IfcRel*` | — | Containment, aggregation, openings, classification association, material association |

## Refinements from operator's original framing

The operator's original category set (Materials / Elements / Spaces /
Systems / Connections / Codes / Performance / Identity) was adjusted
at v0.0.1 to match IFC canonical naming:

- **Spaces → SPATIAL.** IfcSpace is one leaf in the IfcSpatialElement
  hierarchy. The category covers Site, Building, Storey, and Space —
  naming it Spatial avoids the false narrowing to room-scale only.
- **Connections → RELATIONSHIPS.** IFC's own vocabulary is `IfcRel*`.
  Using the IFC noun prevents vocabulary drift between this design
  system and IFC tool integrations.
- **Identity + Codes merged.** Both anchor on `IfcClassificationReference`.
  An Identity reference points into a published taxonomy (Uniclass,
  bSDD); a Code reference points into a jurisdictional bylaw taxonomy.
  Same IFC shape, different namespace — merging them produces a
  cleaner taxonomy without loss of semantic distinction.

## Classification floor — Uniclass 2015

Uniclass 2015 (NBS, UK) is the classification floor for all token
categories. Published by NBS, free to use, openBIM-recognised. The
eleven primary Uniclass tables map cleanly to the eight BIM token
primitives. Uniclass plays the role in the BIM Token Catalog that
Carbon-baseline plays in `pointsav-design-system`'s META-substrate:
the floor every element carries by default, with deployment-specific
classifications layering on top.

## Eighteen interface components

### Universal AEC (10) — present in all surfaces under mode-prop

| Component | IFC anchor | Surface scope |
|---|---|---|
| bim-spatial-tree | IfcSpatialElement | workplace + console |
| bim-properties-panel | IfcPropertySet | workplace + console |
| bim-viewport-3d | IfcGeometricRepresentationItem | workplace + console |
| bim-view-navigator | IfcViewDefinition | workplace + console |
| bim-toolbar | — | workplace + console |
| bim-status-bar | — | workplace + console |
| bim-selection-filter | IfcBuiltElement | workplace + console |
| bim-type-browser | IfcTypeObject | workplace + console |
| bim-section-plane | IfcSectionedSurface | workplace + console |
| bim-annotation-layer | IfcAnnotation | workplace + console |

### Console-unique (4) — read-only operations surface

| Component | IFC anchor | Purpose |
|---|---|---|
| bim-guid-search | IfcRoot.GlobalId | Look up any element by GUID |
| bim-audit-log | IfcOwnerHistory | Show element change history |
| bim-dashboard | — | Portfolio KPI surface |
| bim-export-panel | — | COBie / IDS / BCF export |

### Workplace-unique (4) — authoring affordances

| Component | IFC anchor | Purpose |
|---|---|---|
| bim-materials-browser | IfcMaterial | Search + assign materials |
| bim-type-editor | IfcTypeObject | Edit type-level properties |
| bim-clash-detector | IfcRelInterferesElements | Surface geometric clashes |
| bim-version-history | IfcOwnerHistory | Per-element version diff |

## Mode-prop pattern

Every universal AEC component accepts a `data-mode` attribute with two
values: `workplace` (authoring — editable, drag-capable, BCF capture
enabled) and `console` (read-only, selection only, no edit
affordances). Mode is set at the host application level; the component
itself never derives mode from state. This ensures the component
recipe is testable in isolation at either mode.

## DTCG token naming convention

Token keys follow `{category}.{ifc-class-kebab}.{property}`:
- `spatial.ifc-space.net-floor-area`
- `elements.ifc-wall.thermal-transmittance`
- `materials.ifc-material.mass-density`

This makes the DTCG bundle machine-parseable for any tool that
understands the IFC class vocabulary.

## Cross-references

- Architecture: `research/bim-design-philosophy.md`
- AEC muscle memory: `research/bim-aec-muscle-memory.md`
- Standards floor: IFC 4.3 (ISO 16739-1:2024) · Uniclass 2015 · IDS 1.0 · bSDD
