---
schema: foundry-draft-v1
draft_id: topic-bim-token-three-layers
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-documentation/topic-bim-token-three-layers.md
created: 2026-05-06T17:30:00Z
author: task@project-bim
cites: [ifc-4-3, uniclass-2015, bsdd-v1, ids-1-0, dtcg-w3c, ashrae-90-1, nbc-2020]
doctrine_claims: [40, 41]
research_done_count: 4
research_suggested_count: 1
open_questions_count: 2
research_provenance: |
  Plan Part 1 (three-layer architecture): /home/mathew/.claude/plans/1-we-need-to-frolicking-taco.md
  Sub-agent A: /srv/foundry/.claire/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md
  Sub-agent B: /srv/foundry/.claire/sub-agent-results/B-bim-city-code-as-geometry-2026-04-28.md
  Token files: /srv/foundry/clones/project-bim/pointsav-design-system/tokens/bim/climate-zones.dtcg.json (formerly ecoregions)
  Terminology: /srv/foundry/clones/project-intelligence/service-content/ontology/glossary/glossary_projects.csv
research_inline: false
---

# The Three Layers of a BIM Token

## Lede

A BIM Token has three layers: Specification, Regulation, and Climate Zone. All three are embedded data in the token. None of the three layers is a runtime user-selectable option — a designer does not "switch" between climate zones any more than they switch building codes. The three layers are displayed simultaneously as static reference tables, each showing all registered overlays for the token's element type. This structure reflects a physical reality: a built element has a fixed type (Specification), exists in a fixed jurisdiction (Regulation), and performs in a fixed climate (Climate Zone). All three are facts about the element's physical context, not user preferences.

---

## Why Three Layers

Software design system tokens typically have two concerns: what a value IS (its semantic role — primary colour, heading size) and what value it resolves to (its computed output — `#164679`, `24px`). BIM tokens address a fundamentally different problem space that requires three concerns.

A built-environment element specification must simultaneously answer:
- **What is this element?** — its type in a neutral, tool-independent schema (IFC), its classification in a neutral reference system (Uniclass), and its semantic identity in a jurisdiction-spanning dictionary (bSDD). This is stable across all deployments.
- **What does the jurisdiction require of it?** — the specific regulatory requirements imposed by the law of the place where the building is located. These vary by jurisdiction, change when regulations are updated, and may include geometric constraints (setbacks, clearances, fire compartment boundaries) that cannot be expressed as numeric values.
- **What does the climate require of it?** — the thermal, moisture, and structural performance requirements imposed by the physical climate of the site. These vary by climate zone, are expressed in energy codes (ASHRAE 90.1, NBC Part 11, EN ISO 52000), and change as climate zone mapping is updated.

The three-layer structure gives each concern a clean home and a clear composition rule.

---

## Layer 1 — Specification

The Specification layer is the token's permanent identity. It does not vary by jurisdiction or climate. Every deployed instance of the token reads the same Specification layer regardless of where the project is located.

**Fields:**

| Field | Content | Format |
|---|---|---|
| `ifc_class` | IFC 4.3 entity class | `IfcWall`, `IfcSlab`, etc. |
| `ifc_predefined_type` | IFC predefined type where applicable | `SOLIDWALL`, `FLOORPLATE`, etc. |
| `uniclass_ref` | Uniclass 2015 code | `Ss_20_05_30_75` |
| `uniclass_title` | Uniclass 2015 title text | "Masonry walls" |
| `bsdd_uri` | bSDD concept URI | `https://identifier.buildingsmart.org/...` |
| `description` | Plain-language element description | Free text, max 280 chars |
| `applicable_psets` | Applicable IFC Property Sets | Array of Pset names |
| `dtcg_type` | DTCG token type extension | `bim-element`, `bim-material`, etc. |

The Specification layer is authored once per token and promoted through the standard design-system pipeline (vendor → customer → deployment). Changes to the Specification layer require a version bump and changelog entry.

**IFC entity hierarchy breadcrumb.** Every Specification record includes the full IFC inheritance path from `IfcRoot` to the specific class. For `IfcWall`:

```
IfcRoot → IfcObjectDefinition → IfcObject → IfcProduct → IfcElement → IfcBuiltElement → IfcWall
```

This breadcrumb serves two purposes: it enables authoring tools to present the element's position in the IFC hierarchy without reference to a separate schema document, and it enables inheritance-based rule application (rules specified at `IfcBuiltElement` apply to all elements in that subtree).

---

## Layer 2 — Regulation

The Regulation layer holds jurisdiction-specific requirements. It is a table of registered overlays — one row per jurisdiction per constraint — not a single value. The table shows all registered overlays simultaneously.

**Why a table, not a dropdown.** A regulatory requirement is a fact about the jurisdiction where a building is located, not a choice a designer makes. The token does not ask "which jurisdiction are you in?" and display a single jurisdiction's requirements. It displays all registered jurisdictions' requirements as reference data, the same way a technical standards datasheet shows multiple national standards rows side by side. If the project location is known, the authoring tool may highlight the applicable row — but the table always shows the full picture.

**Overlay structure:**

| Column | Content |
|---|---|
| Jurisdiction | ISO 3166-1/2 jurisdiction code (e.g., `CA-BC`, `US-VA`, `DE`, `SG`) |
| Standard | Identifying reference for the regulatory document |
| Constraint type | Numeric / Geometric / Classification / Approval |
| Parameter | The specific property being constrained |
| Required value | The threshold or required value |
| Unit | SI unit or categorical value set |
| IDS file | Path to the IDS 1.0 constraint file encoding this requirement |
| IFC fragment | Path to IFC geometric exclusion fragment, if applicable |
| Source URI | URI to the authoritative regulatory document |
| Effective date | ISO 8601 date from which this overlay is in effect |

**Example rows for `IfcWall` (exterior wall, residential):**

| Jurisdiction | Standard | Parameter | Required value | Unit |
|---|---|---|---|---|
| CA-BC | NBC 2020 Part 11 | Thermal resistance (opaque wall) | ≥ RSI 3.85 | m²K/W |
| DE | EnEV 2020 | Wärmedurchgangskoeffizient (U-value) | ≤ 0.28 | W/m²K |
| SG | SGBC BCA Green Mark | Thermal transmittance (OTTV) | ≤ 45 | W/m² |
| US-VA (federal) | ASHRAE 90.1-2022 | Assembly U-factor (climate zone 4A) | ≤ 0.124 | Btu/h·ft²·°F |

**Empty state.** At v0.0.1, most tokens have no registered overlays — the overlay structure is defined but unpopulated. The v0.0.3 milestone is planned to deliver the first overlay set: BC RS-1 residential zoning requirements for exterior walls, slabs, and windows. The empty state is displayed explicitly: "No regulatory overlays registered. BC RS-1 in development (v0.0.3)."

**Geometric exclusion fragments.** Where a regulatory requirement has geometric expression — fire compartment boundaries, accessibility clearances, setback envelopes — the overlay row includes an IFC fragment: a solid geometry encoded in IFC that defines the spatial constraint. Geometric exclusion takes unconditional precedence over numeric constraints in the composition rule. See "Composition Rule" below.

---

## Layer 3 — Climate Zone

The Climate Zone layer holds climate-based performance requirements. Like the Regulation layer, it is a table of registered climate zone rows — all zones shown simultaneously, not selected by dropdown.

**Why Climate Zone is distinct from Regulation.** Climate zone classifications are physical geography — they are determined by latitude, altitude, precipitation, and temperature range. Building energy codes frequently reference climate zones as performance multipliers (a wall in Climate Zone 6 must achieve a higher thermal resistance than the same wall in Climate Zone 2A), but the climate zone itself is not a building code. It is a geographic classification that energy codes reference.

The glossary_projects.csv canonical terminology distinguishes:
- **Eco-regions** (glossary entry): "Large areas of land or water that contain a geographically distinct group of natural communities and species" — a biological geography classification.
- **Climate Zones** (glossary entry): "Regions of the Earth broadly classified by similar climatic characteristics" — the engineering classification used in energy codes, ASHRAE, passive house, and NZEB standards.

BIM Token performance specifications use **Climate Zones** exclusively. The eco-region concept (WWF biome classification) is not used in built-environment regulatory contexts and does not appear in BIM Token layer definitions.

**Climate zone classification systems used:**

| System | Jurisdiction coverage | Reference standard |
|---|---|---|
| ASHRAE 90.1 climate zones (1A–8) | US, international reference | ASHRAE Standard 90.1-2022 |
| National Building Code climate zones | Canada | NBC 2020 Part 11 |
| EN ISO 52000 energy performance zones | EU member states | EN ISO 52000-1:2017 |
| Köppen-Geiger simplified (temperate/continental/etc.) | Global cross-reference | Kottek et al. 2006 (updated 2021) |

**Performance parameter table structure:**

| Column | Content |
|---|---|
| Zone ID | Climate zone identifier (e.g., `ASHRAE-5C`, `NBC-Zone-6`) |
| Parameter | Performance property (e.g., "Max U-value exterior wall") |
| Required value | Threshold value |
| Unit | SI or imperial unit |
| Source standard | Standard or code edition |
| Notes | Qualification or condition |

**Example rows for `IfcWall` (exterior wall):**

| Zone ID | Parameter | Required value | Unit | Source |
|---|---|---|---|---|
| ASHRAE-4A | Assembly U-factor (steel-framed wall) | ≤ 0.064 | Btu/h·ft²·°F | ASHRAE 90.1-2022 Table 5.5-4 |
| ASHRAE-5C | Assembly U-factor (mass wall) | ≤ 0.104 | Btu/h·ft²·°F | ASHRAE 90.1-2022 Table 5.5-5 |
| NBC-Zone-6 | Effective thermal resistance (above-grade wall) | ≥ RSI 4.91 | m²K/W | NBC 2020 Table |
| EN-Dfc | U-value requirement (continental subarctic) | ≤ 0.15 | W/m²K | EN ISO 52000 |

**Mobile display.** On small viewports, the Climate Zone table is the engineer's reference during site visits. The table must scroll horizontally. The Zone ID column is pinned. On mobile, the Specification tab is collapsed and the Climate Zone tab is the default open accordion.

---

## Composition Rule

When Regulation and Climate Zone both specify a numeric constraint on the same performance parameter, the effective requirement is the more restrictive value.

```
effective_value = max(regulation_requirement, climate_zone_requirement)
```

This is a lower-bound composition: both layers express performance minima (higher values are always acceptable; lower values are not). The maximum of the two minima is the binding requirement.

**Priority stack for Regulation overlays:**

1. Municipal (local) — highest authority for land use, zoning, setbacks.
2. Provincial/State — structural, fire, energy performance.
3. Federal — ITAR-restricted facilities, federal standards (ASHRAE via GSA).
4. Accessibility — ADA, EN 301 549, equivalent national accessibility standards.

Where two Regulation overlays at different priority levels conflict on the same parameter, the higher-priority (more local) overlay takes precedence for land-use constraints; the more restrictive value applies for performance parameters.

**Geometric exclusion unconditional precedence.** An IFC geometric exclusion fragment in any overlay cannot be overridden by any numeric constraint. If a fire compartment boundary requires a wall to span a specific spatial volume, that geometric requirement applies regardless of whether a numeric thermal performance constraint is more or less restrictive.

**Fail-open.** When a layer has no registered data for a given element type in the project's jurisdiction and climate zone, the token remains valid. The Specification layer is always present; the Regulation and Climate Zone layers are optional additions. A token with Specification data only serves the element identity and classification function without regulatory constraint — a valid state for early design phases before jurisdiction is established.

---

## The Four-Zone CMS Authoring Model

When a new BIM Token is authored — by a standards body, a jurisdiction, or a design system maintainer — the authoring interface follows a four-zone model that maps directly to the three token layers plus a publishing workflow zone.

**Zone 1 — Specification.** IFC class selector (dropdown against IFC 4.3 entity tree), Uniclass 2015 lookup (type-ahead search), bSDD URI field, plain-language description (max 280 chars), applicable property set selector (multi-select from Pset registry). This zone is completed once per new element type.

**Zone 2 — Regulation.** Add-overlay form: jurisdiction selector (ISO 3166-1/2), standard name field, constraint type selector (numeric/geometric/classification/approval), parameter name, required value, unit, IDS file upload, IFC fragment upload (optional), source URI, effective date. Multiple overlays per token. Each overlay is versioned independently.

**Zone 3 — Climate Zone.** Add-zone-row form: zone system selector (ASHRAE / NBC / EN ISO 52000), zone identifier, parameter name, required value, unit, source standard reference. Multiple zone rows per token.

**Zone 4 — Publishing.** Validation summary (IDS file syntax check, IFC fragment geometry check if applicable), preview of token JSON output, approval workflow (for tokens requiring master co-sign per DOCTRINE.md), commit message field, publish action (git commit to token vault).

The CMS authoring model is the intended interface for `app-console-bim` (planned, v0.1.x). At v0.0.1, tokens are authored directly as DTCG JSON files and committed via git.

---

## How This Differs from IFC Property Sets

The distinction from IFC Property Sets is worth restating precisely.

An IFC Property Set (`Pset_WallCommon`) is a list of property definitions: property name, data type, unit. It declares what properties an element of a given type may carry. It does not declare what values those properties must have. `Pset_WallCommon` includes `ThermalTransmittance` of type `IfcThermalTransmittanceMeasure`. It does not say that `ThermalTransmittance` must be ≤ 0.28 W/m²K in Germany. That constraint lives in the Regulation layer of the BIM Token.

The BIM Token Regulation layer consumes the IFC Property Set structure (it references `Pset_WallCommon.ThermalTransmittance` in its constraint rows) but adds the values that the Property Set specification omits: the required value, the jurisdiction for which it applies, and the IDS 1.0 file that formally encodes the constraint in a machine-executable form.

This is the same relationship as between a CSS property specification (declaring that `color` accepts a colour value) and a design token (`--color-primary: #164679`). The property set defines the shape; the token populates the values and adds the constraint hierarchy.

---

## Open Questions

1. **Climate zone boundary data.** The ASHRAE climate zone map is available in GIS format but requires a licence for commercial redistribution. A jurisdiction's climate zone classification for a project site must currently be looked up manually. A planned `app-privategit-bim` feature would accept project coordinates and return the applicable climate zone IDs. The data sourcing question is unresolved.

2. **Regulatory overlay deprecation.** Building regulations are updated on irregular cycles. An overlay published for a regulation version may become out of date without notification. A policy for overlay versioning and deprecation notice is not yet specified.

---

*Draft prepared for project-editorial sweep. Research provenance: plan Part 1 canonical definition; sub-agent A + B research; climate-zones.dtcg.json token file; glossary_projects.csv. Doctrine claims: #40, #41. BCSC posture: all forward-looking items use planned/intended language.*
