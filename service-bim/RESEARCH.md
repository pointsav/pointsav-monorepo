# service-BIM — Research Context

**Scope:** This document covers the research context specific to `service-bim` — the background daemon running inside `os-totebox` PropertyArchive.
**Full research:** `content-wiki-documentation/research/RESEARCH-BIM-MARKET.md`
**Audience:** Developers joining this component

---

## What This Service Is

`service-bim` is a background daemon — no UI, zero user interaction. It has one job: maintain the canonical flat-file BIM archive for one PropertyArchive instance. It watches for incoming files, validates them, converts them to the canonical format stack, and writes the results to the directory structure it owns.

It follows the same daemon pattern as `service-email` and `service-extraction`: deterministic processing, no AI, no network dependency, append-only output to `service-fs`.

---

## The File Format Decisions — Why IFC-SPF

The canonical archive format is **IFC-SPF** (`.ifc`) — ISO 16739-1:2024. This is plain ASCII text. Every government BIM mandate worldwide (UK, EU, Norway, Denmark, Finland, Netherlands, Singapore, Italy, Germany, Spain) specifies IFC as the required deliverable format. No country mandating BIM has mandated RVT, DWG, or any proprietary format.

IFC-SPF is the right canonical format because:
- It is an ISO standard with a published specification
- It is readable without any proprietary SDK by any tool that implements ISO 10303-21
- It passes the 50-year readability test: a file written today is readable in 2076 because the specification is public
- Every major BIM authoring tool (Revit, ArchiCAD, Allplan, FreeCAD, Bonsai) exports it
- IfcOpenShell 0.8.5 (LGPL-3.0) provides production-grade Rust-invocable conversion via `IfcConvert`

**What gets generated from the IFC-SPF:**
- SVG floor plans — one per building level — via IfcOpenShell's SVG slicer. IFC GUIDs are embedded as SVG element IDs.
- glTF binary (`.glb`) — the 3D viewer cache. Regenerated on demand. Not archival. Not canonical.
- Per-element YAML sidecars — PointSav operational layer. Keyed by IFC GUID.

**What `service-bim` never does:**
- Touches a network
- Writes to any location outside its own directory tree
- Invokes AI on geometric or structured data (SYS-ADR-07)
- Modifies a committed record without F12 authorization (SYS-ADR-10)

---

## The RVT Ingestion Problem

RVT is a closed binary format. For Phase 1 (Woodfine showcase), architects export IFC at handover — this is what ISO 19650 requires on every government-mandated project. For Phase 2 (broader customer base), the Open Design Alliance BimRv library provides legal RVT read access.

The ingestion pipeline: incoming file → `ingestion/queue/` → `service-bim` detects via filesystem watch → invokes appropriate converter as subprocess → validates result against `requirements/handover.ids` via `ifctester` → writes to `canonical/` with SHA-256 seal.

---

## IDS Validation

IDS 1.0 (Information Delivery Specification) became the official buildingSMART standard on 1 June 2024. An `.ids` file is XML that machine-validates whether an IFC model contains the required properties. `service-bim` runs `ifctester` against every incoming IFC before it is accepted into the archive.

The `requirements/handover.ids` file defines what PointSav requires of any IFC at handover: space boundaries, occupant-relevant property sets, element GUIDs, and classification against bSDD.

---

## IoT Integration

Sensor readings arrive via local MQTT broker. `service-bim` writes timestamped readings to element YAML sidecars and JSONL time-series files. The Brick Schema (brickschema.org v1.3) provides the controlled vocabulary for sensor types and their relationships to building equipment. SAREF4BLDG aligns Brick semantics with IFC element types.

No cloud. No vendor API. Sensor data stays on the PropertyArchive node.

---

## Key References

- Full market research: `content-wiki-documentation/research/RESEARCH-BIM-MARKET.md`
- IFC specification: ISO 16739-1:2024 — [buildingsmart.org](https://www.buildingsmart.org)
- IfcOpenShell 0.8.5: [ifcopenshell.org](https://ifcopenshell.org)
- IDS 1.0: [technical.buildingsmart.org](https://technical.buildingsmart.org/projects/information-delivery-specification-ids/)
- Brick Schema: [brickschema.org](https://brickschema.org)
- ADRs governing this service: SYS-ADR-07, SYS-ADR-10, SYS-ADR-19
