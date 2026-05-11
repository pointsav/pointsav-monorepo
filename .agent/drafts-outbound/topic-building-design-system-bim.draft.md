---
schema: foundry-draft-v1
draft_id: topic-building-design-system-bim
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-documentation/topic-building-design-system-bim.md
created: 2026-05-06T17:30:00Z
author: task@project-bim
cites: [ifc-4-3, uniclass-2015, dtcg-w3c, bsdd-v1, w3c-aria-1-2]
doctrine_claims: [40]
research_done_count: 3
research_suggested_count: 2
open_questions_count: 1
research_provenance: |
  Primary: /srv/foundry/.claire/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md (414 lines)
  Token files: /srv/foundry/clones/project-bim/pointsav-design-system/tokens/bim/ (9 DTCG files)
  Component recipes: /srv/foundry/clones/project-bim/pointsav-design-system/components/bim-*/
  Terminology: /srv/foundry/clones/project-intelligence/service-content/ontology/glossary/glossary_projects.csv
research_inline: false
---

# The Building Design System

## Lede

Software design systems — IBM Carbon, Google Material, Apple Human Interface Guidelines — solve a coordination problem at scale. When dozens of teams author UI surfaces independently, consistency breaks down unless the design decisions are encoded in a shared layer of tokens, component recipes, and interaction patterns that all surfaces must consume. The tokens layer is the foundation: every colour, spacing unit, and typographic scale is a named, aliasable value. Surfaces reference values; they do not hard-code them.

No equivalent system exists for the built environment. Building Information Modelling production is coordinated through shared standards (IFC, Uniclass, bSDD) and shared authoring tools (Revit, Archicad), but there is no common token layer — no canonical, machine-readable library of built-environment element specifications that authoring surfaces consume by reference. Research commissioned in April 2026 surveyed the available prior art and found the space empty.

The Building Design System for the built environment is intended to fill that space. It is a platform of composable built-environment specification units — BIM Tokens — anchored to open standards, published as plain JSON, and consumable by any conformant tooling.

---

## Why the Space Is Empty

The absence of a built-environment design system is not accidental. Three structural factors kept the space empty.

**Proprietary authoring tool dominance.** The dominant BIM authoring tools have historically stored element specifications in proprietary formats (Revit Families, ArchiCAD GDL). Families and GDL objects carry geometry, parametric behaviour, and some metadata — but they are format-locked, not interoperable across tools, and not designed to carry normative regulatory data. They serve as the element library for a single tool, not as a shared cross-tool specification layer.

**IFC as exchange format, not specification format.** IFC 4.3 (ISO 16739-1:2024) is a neutral exchange format — it expresses what a model contains, not what a token specification requires. The IFC Property Set mechanism (`Pset_WallCommon`, `Pset_SlabCommon`) provides templates for property values but carries no enforcement logic, constraint hierarchy, or jurisdiction mapping. IFC is an excellent exchange substrate but was not designed to be a design system.

**Standards fragmentation.** The built-environment standards stack — IFC, Uniclass, Omniclass, MasterFormat, CoClass, NBS, bSDD — evolved in parallel across jurisdictions. Each solves part of the identification and classification problem. None provides a composable specification layer analogous to DTCG.

The Building Design System uses each of these standards in its proper role: IFC as the entity type hierarchy, Uniclass 2015 as the classification floor, bSDD as semantic identity, and IDS 1.0 as constraint expression. DTCG provides the container format and aliasing mechanism.

---

## The Eight Token Primitive Categories

The token layer of the Building Design System is organised into eight primitive categories, each corresponding to a cluster of IFC 4.3 entity types. This structure mirrors the organisation of software design system token primitives (colour, spacing, typography, elevation, motion, shape, content, component) but is anchored to the IFC entity hierarchy rather than to visual design properties.

**1. Spatial** — `IfcSpatialElement` hierarchy: site, building, building storey (level), space, zone. These tokens define the organisational units that contain elements. A spatial token encodes the IFC spatial entity type, the Uniclass space reference, and applicable occupancy and area requirements.

**2. Elements** — `IfcBuiltElement` hierarchy: walls, slabs, beams, columns, doors, windows, stairs, ramps, and related structural and envelope components. This is the most populous category. Element tokens carry the full three-layer specification (Specification / Regulation / Climate Zone) because built elements are the primary objects of regulatory constraint.

**3. Systems** — `IfcDistributionElement` hierarchy: mechanical, electrical, plumbing, fire protection, and HVAC distribution elements. System tokens connect to climate zone performance requirements (heating/cooling loads, ventilation rates) more directly than structural elements.

**4. Materials** — `IfcMaterial` and `IfcMaterialLayer` specifications. Material tokens encode thermal conductivity, structural grade, fire classification, embodied carbon (where data is available), and applicable bSDD material URIs. A material token is referenced (aliased) by element tokens that specify its composition.

**5. Assemblies** — `IfcElementAssembly` compositions. Assembly tokens are composed references: a curtain wall assembly aliases its glazing unit token, mullion profile token, and thermal break material token. The assembly's performance parameters are derived from the aliased component tokens and the applicable composition rule (most restrictive wins).

**6. Performance** — `IfcPropertySet` templates for performance specifications not covered by entity-specific property sets. This category holds cross-cutting performance definitions: thermal bridges, air permeability, acoustic isolation ratings. Performance tokens are referenced by element tokens that must satisfy non-standard performance requirements.

**7. Identity + Codes** — `IfcClassificationReference` and `IfcExternalReferenceRelationship` specifications. This category holds the mapping tables that relate PointSav BIM Token identifiers to external classification systems: Uniclass 2015, Omniclass Table 23, NBS Section references, Masterformat 2018. It enables cross-system lookup without duplicating classification data.

**8. Climate Zones** — Performance parameter tables keyed to climate zone identifiers (ASHRAE 90.1 zones 1A through 8, Canadian NBC climate zones, and European EN ISO 52000 zones). Each row is one climate zone × one element type × one performance parameter. Element tokens reference climate zone rows by zone identifier. See `topic-bim-token-three-layers.md` for the full Climate Zone layer specification.

---

## The AEC Interface Component Layer

Above the token primitive layer, the Building Design System defines a set of universal AEC interface components — UI patterns common to any BIM-capable authoring surface regardless of the specific programme type.

Sub-agent research identified ten universal components that appear in every BIM authoring context:

1. **BIM Spatial Tree** — Hierarchical display of `IfcSite → IfcBuilding → IfcBuildingStorey → IfcSpace`. The canonical navigation tree for spatial containment. Default expansion to storey level; click to expand to spaces. Built around the IFC containment hierarchy, not a generic tree widget.

2. **BIM Properties Panel** — Structured display of element properties: IFC type, applicable property sets, current property values, Uniclass reference, bSDD URI link, and regulatory overlay summary. Two-column layout: property name / value.

3. **BIM Viewport 3D** — Double-precision WebGL renderer for IFC geometry. Uses xeokit-sdk for double-precision rendering (required for large-site coordinate accuracy). Renders IFC model sections with element-type colouring by token category.

4. **BIM View Navigator** — Orthographic view controls: plan, section, elevation. Section plane control. Viewport synchronisation for multi-view layouts.

5. **BIM GUID Search** — IFC GlobalId lookup surface. Accepts IfcGloballyUniqueId format and returns entity type, property values, spatial location, and applicable token specification.

6. **BIM Audit Log** — Chronological record of model changes against token specification conformance. Displays: timestamp, changed element, previous state, new state, conformance status.

7. **BIM Regulation Navigator** — Jurisdiction and standard browser. Displays registered regulatory overlays with filter by jurisdiction, element type, and effective date. Entry point to the Regulation layer of the token detail pages.

8. **BIM Token Catalog Card** — The grid card for a token category on the catalog index page. Displays: category name, IFC entity anchor, token count, one-line description, link to detail page.

9. **BIM Token Detail Tab** — The tabbed detail view for a single token category: Specification tab, Regulation tab, Climate Zone tab. Each tab is a structured reference table. See `topic-bim-token-three-layers.md` for tab specifications.

10. **BIM Code Overlay Card** — Display unit for a single registered regulatory overlay: jurisdiction name, standard identifier, constraint summary, IDS file link, effective date, issuing authority.

Three of these components (Spatial Tree, Properties Panel, Viewport 3D) are implemented at v0.0.1. The remaining seven are intended for v0.0.2 and v0.0.3.

---

## Surface-Specific Extensions

Beyond universal components, each built-environment programme type has a distinct set of interface requirements. Three programme types are in scope at v0.0.1:

**app-orchestration-bim** — The BIM Token Catalog web surface. Displays the full catalog, regulatory overlays, and standards reference. Read-only; serves designers, regulators, and machine consumers. Implements the Token Catalog Card and Token Detail Tab components.

**app-workplace-bim** — The BIM authoring workbench. Tauri 2.x desktop application embedding xeokit for 3D rendering and IfcOpenShell (via sidecar) for IFC parsing. Implements Spatial Tree, Properties Panel, Viewport 3D, and View Navigator. Licensed AGPL-3.0 due to xeokit component licensing.

**app-console-bim** — The administrative console for BIM token vault management. Web-only Axum application. Read surface for token browsing; write surface (intended) for token authoring via the four-zone CMS model.

---

## Uniclass 2015 as Classification Floor

All BIM Tokens use Uniclass 2015 as the classification floor. Uniclass 2015 is the UK-maintained open classification standard that provides a structured reference for elements, systems, products, and activities in the built environment. It is maintained by NBS (now part of Byggfakta Group) and published under open licence terms.

Uniclass 2015 was selected over Omniclass (North American) and CoClass (Swedish) because:
1. It is the most consistently maintained and revised of the open classification systems.
2. It maps well to IFC 4.3 entity types at a level of detail appropriate for token specification.
3. It is jurisdiction-neutral despite its UK origin — it is used in international projects alongside other national classification standards.

The token vault includes the full Uniclass 2015 table as a separate DTCG file (`identity-codes.dtcg.json`), enabling tokens to reference classification codes by alias rather than duplicating the classification text.

---

## The Sovereign Vault Model

The Building Design System is not a hosted service. It is a set of JSON files in a git repository — the BIM token vault. Organisations operating the platform clone the vault repository, extend it with their own regulatory overlays and climate zone data, and configure their deployed `app-orchestration-bim` instance to read from their local copy.

Woodfine Management Corp. operates `woodfine-design-bim` as its sovereign BIM token vault — a GitHub repository that will hold the organisation's specific token extensions, regulatory overlays for British Columbia, and climate zone data for the temperate-coastal region. This repository is a fork-in-structure (not a git fork) of the `pointsav-design-system` token architecture.

The vendor-tier `pointsav-design-system` maintains the universal token primitives, component recipes, and research files. Customer-tier vaults extend the vendor layer with jurisdiction-specific and programme-specific additions. Deployment instances read from the customer vault. No data flows upward: vendor → customer → deployment only.

---

## Open Questions

1. **Design token aliasing across tiers.** The DTCG aliasing mechanism works within a single token file. Cross-tier aliasing (a customer token referencing a vendor token by path) requires a resolver convention not yet specified. This is a v0.0.3 tooling decision.

---

*Draft prepared for project-editorial sweep. Research provenance: sub-agent A (prior art, 2026-04-28); DTCG token files in pointsav-design-system; glossary_projects.csv. Doctrine claim: #40 (Flat-File BIM Substrate). BCSC posture: future implementation items use intended/planned language.*
