---
schema: foundry-draft-v1
state: draft-pending-editorial-pass
originating_cluster: project-bim
target_repo: vendor/content-wiki-documentation
target_path: topic-bim-tokens-substrate.md
audience: vendor-public
bcsc_class: vendor-public
language_protocol: PROSE-TOPIC
authored: 2026-05-17T00:00:00Z
authored_by: totebox@project-bim
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 2
open_questions_count: 1
research_provenance:
  - workspace-tier sub-agent A — `~/Foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md`
  - woodfine-design-bim DTCG token files — `woodfine-design-bim/tokens/bim/`
  - plans/tool-buildingwidth-architecture.md — zone derivation provenance from manufacturer SKUs
research_inline: true
references:
  - cites: [iso-16739-1-2024, ifc-43, uniclass-2015, bsdd-uri, w3c-dtcg, ids-1-0]
notes_for_editor: |
  Technical reference article. Audience is BIM operators and architects
  who will consume the Building Design System tokens to author IFC files.
  Tone: precise, operational. No "Do Not Use" vocabulary.
  BCSC vendor-public class — describes the substrate factually;
  no revenue projections or deployment commitments without qualifier.
  The eight token categories should link to the live DTCG files in
  woodfine-design-bim at publication time.
---

# The BIM tokens substrate

The Building Design System token library anchors every value to a node
in the IFC 4.3 entity hierarchy, classified by Uniclass 2015, and
published as a dereferenceable bSDD URI. This document describes how
the three-layer reference system works, what the eight BIM primitive
token categories encode, and how a BIM operator reads and applies them
when authoring an IFC file.

## Three layers of reference

### Layer 1 — IFC 4.3 entity hierarchy

IFC 4.3 was published as ISO 16739-1:2024 in April 2024. The standard
defines a hierarchy of named entity classes — `IfcSite`, `IfcBuilding`,
`IfcBuildingStorey`, `IfcSpace`, `IfcWall`, `IfcSlab`, `IfcDoor`, and
hundreds more — each with typed Property Sets (Psets) specifying what
attributes a compliant implementation must support.

Every Building Design System token maps to a specific IFC entity class
or Pset. The spatial token `bim.spatial.storey` anchors to
`IfcBuildingStorey`; the performance token `bim.performance.door-fire-exit`
anchors to `Pset_DoorCommon.IsFireExit`. When the token value changes,
a validator can trace the change back to the IFC Pset that the token
represents.

### Layer 2 — Uniclass 2015 classification floor

Uniclass 2015 is the unified classification system for the UK
construction industry, maintained by the NBS. It covers entities
(Ss), activities (Ac), products (Pr), and systems (Ss). Building
Design System tokens carry a Uniclass 2015 reference in their
`$extensions.uniclass` field, establishing a classification floor
that maps to the national procurement and specification context.

The classification is non-normative for jurisdictions outside the UK
but serves as the consistent structural vocabulary for the token
catalogue. Where a North American equivalent exists — OmniClass,
UniFormat II, or MasterFormat — the token carries a cross-map
annotation in `$extensions.omniclass`.

### Layer 3 — bSDD URIs

buildingSMART's Data Dictionary (bSDD) publishes machine-readable
JSON-LD definitions for building properties and classifications, each
with a stable dereferenceable URI such as
`https://identifier.buildingsmart.org/uri/buildingsmart/ifc-4.3/prop/IsFireExit`.
Building Design System tokens carry these URIs in their `$description`
or `$extensions.bsdd_uri` fields, linking the token to an authoritative
definition that a validator or authoring tool can resolve at runtime.

## Eight token primitive categories

The Building Design System groups BIM tokens into eight categories,
each mapped to a distinct layer of the IFC entity model.

### 1. Spatial (`bim.spatial.*`)

Site, Building, Storey, and Space — the `IfcSpatialStructureElement`
hierarchy. Tokens in this category encode floor-to-floor heights,
minimum net area per use type, and occupancy limits. These are the
tokens that drive building massing at the earliest design stage.

Source file: `spatial.dtcg.json`

### 2. Elements (`bim.elements.*`)

Walls, slabs, columns, beams, curtain walls, doors, and windows —
`IfcProduct` subclasses. Tokens encode structural fire ratings,
acoustic performance classes, maximum glazing ratios, and the
door-width minima required by ADA 2010 and CSA-B651.

Source file: `elements.dtcg.json`

### 3. Materials (`bim.materials.*`)

Thermal conductivity, density, acoustic absorption coefficients, and
compressive strength — `Pset_MaterialCommon` and material-specific
Psets. Material tokens encode the properties of the substrate, not
the assembly. Assembly-level properties (U-values for a wall
assembly, for instance) are encoded in the Assemblies category.

Source file: `materials.dtcg.json`

### 4. Assemblies (`bim.assemblies.*`)

Composite element constructions — wall assemblies, floor-ceiling
assemblies, roof assemblies — where the performance property belongs
to the layered system, not any single material. Thermal transmittance
(U-value) and sound reduction index (Rw) live here, anchored to
ISO 10077 and ISO 717-1 respectively.

Source file: `assemblies.dtcg.json`

### 5. Systems (`bim.systems.*`)

HVAC zones, electrical circuits, and plumbing networks —
`IfcSystem` and `IfcDistributionSystem`. The central token in this
category is the climate-zone-autonomy principle: one Tile equals one
HVAC zone. A tenant who leases a Tile controls a thermostat. The
token `bim.systems.tile-climate-autonomy` encodes this invariant so
any downstream validator can confirm a proposed floor-plate
composition satisfies it.

Source file: `systems.dtcg.json`

### 6. Performance (`bim.performance.*`)

Energy, daylight, accessibility, and fire-safety thresholds —
regulatory minima drawn from EN 12464-1 (workplace daylight), EN
17037 (daylight in buildings), ADA 2010, IBC 2021, and ASHRAE
90.1. Performance tokens are the regulatory floor that the spatial
and element tokens must satisfy.

The most architecturally consequential performance token is
`bim.performance.max-workstation-to-window`, set at 6.0 metres from
EN 12464-1:2021. This single value drives the Zone 1 Habitat depth
in every professional office floor plate.

Source file: `performance.dtcg.json`

### 7. Climate zones (`bim.climate-zones.*`)

ASHRAE 90.1 climate zones 1 through 8, encoding the heating-degree-
day and cooling-degree-day thresholds that govern mechanical-system
sizing across North American jurisdictions. These tokens are consumed
by service-codes when a building site is assigned to an ASHRAE zone
and the energy model is validated.

Source file: `climate-zones.dtcg.json`

### 8. Identity codes (`bim.identity-codes.*`)

Use-type codes (FFE series: M1, M2, B1, L1, A1, C1), tile codes
(Tile A through Tile H), and key-plan index ranges. These are the
opaque identifiers that planning documents use when describing floor-
plate compositions. The token catalogue is the lexicon that maps
each code to its geometric and programme meaning.

Source file: `identity-codes.dtcg.json`

## How tokens are delivered

The token library is distributed as a set of DTCG-format JSON files
in the `woodfine-design-bim` repository. DTCG (W3C Design Token
Community Group format) is a JSON structure where each token carries
a `$value`, `$type`, and `$description`, with optional `$extensions`
for standards references and provenance.

A BIM authoring tool or validator reads the DTCG files at startup and
constructs an in-memory token index. Any dimension, threshold, or
classification referenced in a design session resolves to a token
value rather than a hard-coded number. When the token changes —
because a code amendment raises the minimum corridor width, or a
new Uniclass edition replaces a classification code — the consuming
tool receives the update via a library version bump, not a manual
re-entry of values across hundreds of model elements.

## Relationship to the IFC file

The DTCG token library does not modify IFC files directly. It is an
authoring-time reference — the source of truth for what values to
write into an IFC Pset when a model element is created or modified.
A BIM operator who follows the Building Design System writes the IFC
file with the token values; IDS 1.0 contracts then validate that
the IFC file conforms to the published thresholds.

The token-to-IFC pipeline is therefore:

```
Token value (bim.performance.max-workstation-to-window = 6.0 m)
        ↓ applied at authoring time
IFC Pset_SpaceOccupancy attribute in IfcSpace
        ↓ validated at delivery time
IDS 1.0 rule: IfcSpace.MaxWorkstationToWindow ≤ 6000 mm
```

This separation — author with tokens, validate with IDS — is the
substrate that makes City Code as Composable Geometry possible at the
next layer.

## Provenance

Authored by totebox@project-bim, 2026-05-17. Sources: IFC 4.3 /
ISO 16739-1:2024; EN 12464-1:2021; ADA 2010 §305; IBC 2021
§1020/§1029; DTCG token files in woodfine-design-bim; workspace
sub-agent A research (2026-04-28).

Refinement target: `vendor/content-wiki-documentation/topic-bim-tokens-substrate.md`
plus Spanish adaptation at `topic-bim-tokens-substrate.es.md`.

## Research trail

### Done

- Mapped all eight token categories to IFC 4.3 entity hierarchy and
  confirmed Pset anchors against IFC 4.3 schema. Workspace sub-agent A.
- Validated DTCG file contents against manufacturer and regulatory
  sources for professional-office zone depths. plans/tool-buildingwidth-
  architecture.md §Research provenance.
- Confirmed bSDD URI format and dereferenceability via
  identifier.buildingsmart.org. Sub-agent A.

### Suggested

- Verify current bSDD URI format stability — buildingSMART revises URI
  patterns; confirm identifiers resolve at publication time.
- Cross-map Uniclass 2015 codes to OmniClass for North American appendix.

### Open questions

- Whether DTCG W3C CG format will be superseded by a formal W3C
  Recommendation before the token library ships a v1.0 release. The
  v0.0.x token files use the community-group schema; a format migration
  at v1.0 may be needed.
