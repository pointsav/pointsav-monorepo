---
schema: foundry-draft-v1
draft_id: guide-regulation-overlay-publishing
language_protocol: PROSE-GUIDE
state: ready-for-sweep
target_path: customer/woodfine-fleet-deployment/gateway-orchestration-bim/guide-regulation-overlay-publishing.md
created: 2026-05-06T19:10:00Z
author: task@project-bim
cites: [ids-1-0, ifc-4-3, ifctester, bsdd-v1]
research_done_count: 2
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  Plan Part 3 (regulation overlay publishing): /home/mathew/.claude/plans/1-we-need-to-frolicking-taco.md
  IDS 1.0 specification: github.com/buildingSMART/IDS (research from sub-agent B)
  IFC fragment mechanism: B-bim-city-code-as-geometry-2026-04-28.md
research_inline: false
---

# Guide: Publishing a Regulatory Overlay

A Regulatory Overlay adds jurisdiction-specific requirements to one or more BIM Token types. Each overlay is an independently versioned bundle of three files. This guide covers authoring, validating, and promoting an overlay to a live token vault.

All paths below are relative to the `woodfine-design-bim` repository root.

---

## Overview

A complete Regulatory Overlay bundle contains:

| File | Required | Purpose |
|---|---|---|
| `<overlay-id>.ids` | Yes | IDS 1.0 constraint file — numeric + property requirements |
| `<overlay-id>-exclusion.ifc` | Conditional | IFC geometric exclusion fragment — spatial/topology requirements |
| `<overlay-id>.yaml` | Yes | Overlay metadata (jurisdiction, standard, element types covered) |

The metadata YAML is the entry point. It declares which IFC element types the overlay covers and references the IDS and IFC fragment files.

---

## Step 1 — Name the Overlay

Overlay IDs follow this convention:
```
<jurisdiction-code>-<standard-slug>-<element-slug>
```

Examples:
- `ca-bc-bcbc2024-exterior-wall` — BC Building Code 2024, exterior walls
- `us-va-ashrae9012022-glazing` — ASHRAE 90.1-2022, windows and glazing
- `de-enev2020-roof-slab` — German EnEV 2020, roof slabs

The jurisdiction code is ISO 3166-2 (e.g., `CA-BC`, `US-VA`, `DE`, `SG`).

---

## Step 2 — Author the IDS File

Create `regulation/<jurisdiction>/<overlay-id>.ids` using the IDS 1.0 XML schema:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<ids:ids xmlns:ids="http://standards.buildingsmart.org/IDS"
         xmlns:xs="http://www.w3.org/2001/XMLSchema"
         xsi:schemaLocation="http://standards.buildingsmart.org/IDS ids.xsd"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">

  <ids:info>
    <ids:title>BC Building Code 2024 — Exterior Wall Thermal Requirements</ids:title>
    <ids:copyright>Province of British Columbia</ids:copyright>
    <ids:version>1.0.0</ids:version>
    <ids:description>
      Thermal transmittance requirements for above-grade exterior walls
      per BCBC 2024 Part 11 Table 11.2.2.1.
    </ids:description>
    <ids:author>task@project-bim</ids:author>
    <ids:date>2026-05-06</ids:date>
    <ids:purpose>Regulation overlay for woodfine-design-bim vault</ids:purpose>
  </ids:info>

  <ids:specifications>
    <ids:specification name="ExteriorWall-ThermalTransmittance-BCBC2024"
                       ifcVersion="IFC4X3"
                       description="Max thermal transmittance for above-grade exterior walls"
                       minOccurs="1">
      <ids:applicability>
        <ids:entity>
          <ids:name><ids:simpleValue>IFCWALL</ids:simpleValue></ids:name>
        </ids:entity>
        <ids:property dataType="IfcLabel">
          <ids:propertySet><ids:simpleValue>Pset_WallCommon</ids:simpleValue></ids:propertySet>
          <ids:baseName><ids:simpleValue>IsExternal</ids:simpleValue></ids:baseName>
          <ids:value><ids:simpleValue>TRUE</ids:simpleValue></ids:value>
        </ids:property>
      </ids:applicability>
      <ids:requirements>
        <ids:property dataType="IfcThermalTransmittanceMeasure">
          <ids:propertySet><ids:simpleValue>Pset_WallCommon</ids:simpleValue></ids:propertySet>
          <ids:baseName><ids:simpleValue>ThermalTransmittance</ids:simpleValue></ids:baseName>
          <ids:value>
            <xs:restriction base="xs:double">
              <xs:maxInclusive value="0.210"/>
            </xs:restriction>
          </ids:value>
        </ids:property>
      </ids:requirements>
    </ids:specification>
  </ids:specifications>
</ids:ids>
```

Key points:
- `ifcVersion="IFC4X3"` — always use IFC4X3 (IFC 4.3) for new overlays.
- Use `<ids:applicability>` to narrow the constraint to the correct element subtype (e.g., exterior walls only, not all walls).
- `<xs:maxInclusive>` for upper-bound requirements; `<xs:minInclusive>` for lower bounds.
- All property values must match the data type declared in `dataType`.

---

## Step 3 — Author the IFC Fragment (if required)

An IFC geometric exclusion fragment is required when the regulatory constraint has spatial or topological expression — fire compartment boundaries, setbacks, accessibility clearances.

The fragment is a minimal valid IFC file containing:
1. An `IfcSpace` or `IfcZone` entity defining the constrained volume
2. A `IfcRelAssociatesConstraint` linking the zone to an `IfcObjective`
3. An `IfcShapeRepresentation` encoding the geometry of the exclusion volume

Creating IFC fragments requires an IFC-authoring-capable tool (FreeCAD + BIM workbench, Archicad, or equivalent). Author the fragment geometry, export as IFC 4.3, and save as `regulation/<jurisdiction>/<overlay-id>-exclusion.ifc`.

Geometric exclusion fragments are not required for purely numeric constraints. When in doubt, omit and note the absence in the metadata YAML.

---

## Step 4 — Author the Metadata YAML

Create `regulation/<jurisdiction>/<overlay-id>.yaml`:

```yaml
id: ca-bc-bcbc2024-exterior-wall
jurisdiction: CA-BC
jurisdiction_name: "British Columbia, Canada"
standard: "BC Building Code 2024"
standard_uri: "https://www.bccodes.ca/building-code.html"
authority: "Province of British Columbia"
effective_date: "2024-03-01"
ifc_classes_covered:
  - IfcWall
  - IfcSlab  # include roof slabs
constraint_summary: "Max thermal transmittance for above-grade exterior walls and exposed slabs"
ids_path: "ca-bc-bcbc2024-exterior-wall.ids"
ifc_fragment_path: null  # no geometric exclusion for thermal constraint
version: "1.0.0"
bsdd_authority_uri: "https://identifier.buildingsmart.org/uri/buildingsmart/ifc/4.3"
```

---

## Step 5 — Validate

Run `ifctester` to validate the IDS file syntax before committing:

```bash
# Syntax-only validation (no model required)
ifctester --ids regulation/CA-BC/ca-bc-bcbc2024-exterior-wall.ids \
          --report json
```

A valid IDS file produces output with `"status": "pass"` and no schema errors. If the output contains schema errors, correct the IDS file before proceeding.

For validation against a live IFC model:
```bash
ifctester --ids regulation/CA-BC/ca-bc-bcbc2024-exterior-wall.ids \
          --ifc <path-to-model.ifc> \
          --report json \
          --output regulation/CA-BC/validation-report.json
```

---

## Step 6 — Register in the Token Vault

Add the overlay reference to the relevant DTCG token file. Open `tokens/bim/elements.dtcg.json` and add the overlay to the element token:

```json
"elements.IfcWall": {
  ...existing fields...,
  "regulation_overlays": [
    {
      "id": "ca-bc-bcbc2024-exterior-wall",
      "jurisdiction": "CA-BC",
      "standard": "BC Building Code 2024",
      "ids_path": "regulation/CA-BC/ca-bc-bcbc2024-exterior-wall.ids",
      "effective_date": "2024-03-01"
    }
  ]
}
```

---

## Step 7 — Commit and Promote

```bash
git add regulation/CA-BC/ tokens/bim/elements.dtcg.json
git commit -m "feat(regulation): add CA-BC BCBC 2024 exterior wall thermal overlay"
```

After commit to `woodfine-design-bim`, restart `app-orchestration-bim` on the deployment host to load the new overlay. Verify the overlay appears in the Regulation tab for the `elements.IfcWall` token page at `bim.woodfinegroup.com/tokens/elements.dtcg`.

---

## Open Questions

1. The unit system for `xs:maxInclusive` values in IDS files — SI (W/m²K) or imperial (Btu/h·ft²·°F) — is not yet standardised in the token vault tooling. The first overlay set should establish the convention. SI is recommended for consistency with international standards; imperial conversion documentation must be added alongside.
