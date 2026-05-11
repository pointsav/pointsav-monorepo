---
schema: foundry-draft-v1
draft_id: design-research-climate-zone-constraints
language_protocol: DESIGN-RESEARCH
state: ready-for-sweep
target_path: vendor/pointsav-design-system/research/bim-climate-zone-constraints.md
created: 2026-05-05T00:00:00Z
revised: 2026-05-06T17:45:00Z
author: task@project-bim
cites: [ifc-4-3, ids-1-0, bsdd-v1, ashrae-90-1]
research_done_count: 1
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  Originated: .agent/artifacts/BIM-ECOREGION-CONSTRAINT.md (2026-05-05)
  Revised: "EcoRegion" → "Climate Zone" throughout (glossary canonical correction).
  IfcSpatialZone ObjectType updated: ECOREGION → CLIMATE_ZONE.
  bSDD URI updated: removed "EcoRegion"-named URIs; use ASHRAE zone identifiers.
research_inline: true
---

# BIM Climate Zone Constraints — IFC Mapping Research

## IFC 4.3 Entity Mapping

Climate zone data is associated with a project using `IfcSpatialZone`:

| IFC field | Value |
|---|---|
| Entity | `IfcSpatialZone` |
| PredefinedType | `USERDEFINED` |
| ObjectType | `CLIMATE_ZONE` |
| Name | Zone identifier, e.g., `ASHRAE-5C` |

The zone entity is related to constrained elements via `IfcRelAssociatesConstraint`, which links an `IfcSpatialZone` to `IfcMetric` records encoding the performance requirements.

## IFC Constraint Chain

```
IfcSpatialZone (CLIMATE_ZONE, Name="ASHRAE-5C")
  └── IfcRelContainedInSpatialStructure
        └── IfcBuilding (project building)
  └── IfcRelAssociatesConstraint
        └── IfcObjective (name="ThermalPerformanceRequirement")
              └── IfcMetric (name="MaxUValueExteriorWall", value=0.104, unit="Btu/h·ft²·°F")
```

## Semantic Data Schema (YAML Sidecar Template)

```yaml
climate_zone:
  id: ashrae-5c
  ifc_object_type: CLIMATE_ZONE
  label: "ASHRAE Climate Zone 5C — Marine"
  description: "Marine climate, moist, heating-dominated. Pacific Northwest, coastal BC."
  bsdd_classification: "https://identifier.buildingsmart.org/uri/buildingsmart/ifc/4.3/class/IfcSpatialZone"
  source_standard: "ASHRAE 90.1-2022"
  constraints:
    - ifc_class: IfcWall
      parameter: max_u_value
      value: 0.104
      unit: "Btu/h·ft²·°F"
      source: "ASHRAE 90.1-2022 Table 5.5-5"
    - ifc_class: IfcSlab
      parameter: max_u_value
      value: 0.029
      unit: "Btu/h·ft²·°F"
      source: "ASHRAE 90.1-2022 Table 5.5-5"
    - ifc_class: IfcWindow
      parameter: max_u_factor
      value: 0.32
      unit: "Btu/h·ft²·°F"
      source: "ASHRAE 90.1-2022 Table 5.5-5"
```

## IDS 1.0 Encoding

Each climate zone constraint row is encodable as an IDS 1.0 specification:

```xml
<ids:specification name="ExteriorWall-MaxUValue-ASHRAE5C"
                   ifcVersion="IFC4X3"
                   minOccurs="1">
  <ids:applicability>
    <ids:entity>
      <ids:name><ids:simpleValue>IFCWALL</ids:simpleValue></ids:name>
    </ids:entity>
  </ids:applicability>
  <ids:requirements>
    <ids:property dataType="IfcThermalTransmittanceMeasure">
      <ids:propertySet><ids:simpleValue>Pset_WallCommon</ids:simpleValue></ids:propertySet>
      <ids:baseName><ids:simpleValue>ThermalTransmittance</ids:simpleValue></ids:baseName>
      <ids:value>
        <xs:restriction base="xs:double">
          <xs:maxInclusive value="0.104"/>
        </xs:restriction>
      </ids:value>
    </ids:property>
  </ids:requirements>
</ids:specification>
```

## Open Questions

1. The IDS `xs:maxInclusive` restriction is the correct encoding for an upper-bound
   performance requirement. The IDS 1.0 specification (buildingSMART, 2023) confirms this
   is valid. However, the unit conversion between SI (W/m²K) and imperial (Btu/h·ft²·°F)
   must be handled at IDS authoring time — the IDS file carries one unit system.
   A tooling decision for which unit system PointSav uses in generated IDS files is
   needed before v0.0.3 overlay publication.
