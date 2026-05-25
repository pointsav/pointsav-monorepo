---
schema: foundry-draft-v1
draft_id: topic-bim-token-what-it-is
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-documentation/topic-bim-token-what-it-is.md
created: 2026-05-06T17:30:00Z
author: task@project-bim
cites: [ifc-4-3, uniclass-2015, bsdd-v1, ids-1-0, dtcg-w3c]
doctrine_claims: [40, 41]
research_done_count: 4
research_suggested_count: 2
open_questions_count: 1
research_provenance: |
  Primary: /srv/foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md (414 lines)
  Terminology: /srv/foundry/clones/project-intelligence/service-content/ontology/glossary/glossary_projects.csv (339 entries)
  Plan Part 1 canonical definition: /home/mathew/.claude/plans/1-we-need-to-frolicking-taco.md
  Existing token files: /srv/foundry/clones/project-bim/pointsav-design-system/tokens/bim/ (9 DTCG files)
research_inline: false
---

# What Is a BIM Token

## Lede

Building Information Modelling produces detailed digital representations of structures. It does not, in its standard form, prevent violations. A model can be geometrically complete, materially specified, and classified by IFC entity type while still containing elements that fail code compliance, violate climate performance floors, or conflict with jurisdictional regulation — discoveries made only when a post-design checker runs. A BIM Token addresses this upstream. It encodes a built-environment element decision as a composable, aliasable specification unit that pre-constrains the design space rather than auditing a completed model.

---

## Definition

A BIM Token is a composable built-environment specification unit — the structural counterpart of a Design System Token. Where a Design System Token encodes a design decision (a colour, a spacing unit, a component recipe) as a reusable, aliasable value that all conforming surfaces must honour, a BIM Token encodes a built-environment element decision across three simultaneous axes:

1. **What the element IS** — its IFC entity class, Uniclass 2015 classification, bSDD identity URI, and applicable property set templates.
2. **What it MUST satisfy** — the regulatory requirements imposed by its jurisdiction, expressed as jurisdictional overlays (IDS 1.0 constraint files and IFC geometric exclusion fragments).
3. **How it MUST perform** — the energy, thermal, structural, and acoustic requirements imposed by its climate zone, expressed as tabular performance parameters keyed to ASHRAE and equivalent national standards.

Placing a BIM Token in a model simultaneously places the element, its regulatory envelope, and its climate performance floor. Violations are geometrically impossible by construction rather than discovered in post-design review.

---

## What a BIM Token Is Not

Precision requires distinguishing the BIM Token from four structures it superficially resembles.

**Not an IFC entity type.** IFC 4.3 (ISO 16739-1:2024) defines `IfcWall`, `IfcSlab`, `IfcBeam`, and approximately 900 other schema classes. An IFC entity type is a schema class — it defines what data shape a wall record must conform to. It carries no constraint values, no jurisdiction-specific requirements, and no performance parameters. A BIM Token uses the IFC entity class as its identity anchor but adds the three constraint layers the schema class cannot hold.

**Not a Revit Family.** A Revit Family is geometry parameterised for one authoring tool, stored in a proprietary binary format, and vendor-locked. It carries no normative regulatory data, no jurisdiction mapping, and no climate zone performance specification. It cannot be read by non-Revit tooling without export. A BIM Token is plain JSON (W3C DTCG format), tool-neutral, and machine-readable by any conformant consumer.

**Not a COBie spreadsheet.** COBie (Construction Operations Building Information Exchange) is after-the-fact data capture — facility management data extracted from a model after design is complete. It documents what was built. A BIM Token constrains what may be placed.

**Not an IFC Property Set.** An IFC Property Set (`Pset_WallCommon`, etc.) is a template for values without enforcement logic, constraint hierarchy, or jurisdiction mapping. It records properties; it does not enforce them. A BIM Token includes applicable property set definitions but adds the regulatory and climate zone enforcement layers that property sets cannot express.

**Not a BIM model file.** An IFC model is a geometry instance for a specific project. A BIM Token is a reusable specification — a template that generates conformant geometry instances when instantiated. The relationship is analogous to Design System component recipe to rendered DOM node.

---

## The Pre-Constraining Thesis

Two decades of BIM tooling have been built on a validation-first assumption: design freely, then check. Solibri, Archistar, and similar platforms run rules against completed models and report violations. Singapore's CORENET X — the most advanced government BIM submission system in production — operates on the same principle. Submit a model; receive a compliance report.

The pre-constraining approach inverts this. If the only elements available to a designer are BIM Tokens, and each token already encodes the regulatory and performance constraints applicable to its type in a given jurisdiction and climate zone, then the compliance report is structurally unnecessary. The model cannot be non-compliant because non-compliant configurations cannot be placed.

This is a compositional claim, not a validation claim. The distinction matters because validators generate reports that require human remediation. A composition system that enforces constraints at placement time has no violations to report.

---

## Three Layers

A BIM Token has three layers. All three are data embedded in the token. Neither Regulation nor Climate Zone is a runtime user-selectable option — they are reference data displayed as static lookup tables, exactly as a technical standard datasheet shows multiple jurisdiction rows simultaneously.

**Layer 1 — Specification.** The IFC entity class (e.g., `IfcWall`), Uniclass 2015 classification reference (e.g., `Ss_20_05_30_75`), bSDD concept URI, plain-language description, and applicable property set templates. This layer is the token's permanent identity.

**Layer 2 — Regulation.** A table of registered jurisdictional overlays. Each row is one overlay: jurisdiction, standard, constraint type, required value, source authority, and IDS 1.0 file path. Where an overlay includes an IFC geometric exclusion fragment (a solid geometry encoded as IFC that the element must not occupy), that fragment takes unconditional precedence over any numeric constraint.

**Layer 3 — Climate Zone.** A table of registered climate zone performance requirements. Each row is one registered zone: zone identifier (aligned to ASHRAE 90.1 or national equivalent), performance parameter, required value, unit, and source standard. Where a climate zone row conflicts with a Regulation row for the same parameter, the more restrictive value applies.

**Composition rule:** `effective_value = max(regulation_requirement, climate_zone_requirement)` where both express lower bounds. Geometric exclusion fragments override numeric constraints unconditionally. When a layer has no registered data for a given element type, the token remains valid and serves specification-only.

---

## Implementation Form

BIM Tokens are stored as W3C Design Token Community Group (DTCG) format JSON, extended with BIM-specific token types. The `$type` field is extended beyond the DTCG core set to include `bim-element`, `bim-material`, `bim-assembly`, and related AEC-specific types. The standard DTCG aliasing mechanism (`{token.reference}`) is preserved, enabling BIM tokens to reference each other compositionally — a curtain wall assembly token can alias its glazing unit token, its mullion profile token, and its thermal break token.

The machine-readable format enables:
- Tooling integration: any BIM authoring tool with a DTCG parser can consume BIM Tokens without proprietary plugin development.
- Regulatory versioning: jurisdictional overlays are versioned separately from the specification layer, allowing a jurisdiction to update its constraint rows without breaking the token's identity.
- Offline operation: a complete vault of BIM Tokens is a directory of JSON files, cloneable via git and queryable without network access — a prerequisite for ITAR-restricted, GDPR-sovereign, and construction-site use cases.

---

## Relationship to the Design System

The BIM Token system parallels the structure of a software design system. Where IBM Carbon or a similar system provides a token primitive layer (colours, spacing, typography), a component recipe layer (button, card, navigation), and a surface-specific extension layer (mobile, web, print), the BIM Token platform provides a token primitive layer (the 8 DTCG token categories anchored to IFC 4.3), a universal AEC component layer (spatial tree, properties panel, viewport renderer), and surface-specific extensions per built-environment programme type.

The analogy is structural, not metaphorical. Both systems address the same problem: enforcing consistency across independent authoring surfaces by encoding decisions as reusable, aliasable, versionable units with machine-readable constraint specifications. The BIM platform extends the model into a physical constraint domain that software design systems do not address.

---

## Open Questions

1. **bSDD version stability.** The buildingSMART Data Dictionary API is versioned separately from IFC 4.3. BIM Token frontmatter includes bSDD URI references. A policy for handling bSDD URI deprecation or schema migration is not yet formalised. This is a v0.0.3 documentation task.

---

*Draft prepared for project-editorial sweep. Research provenance: sub-agent A (prior art, 2026-04-28); glossary_projects.csv (canonical terminology). Doctrine claims: #40 (Flat-File BIM Substrate), #41 (City Code as Composable Geometry). BCSC posture: forward-looking items use "intended/planned" language.*
