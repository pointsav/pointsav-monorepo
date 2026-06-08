---
schema: foundry-draft-v1
artifact: JOURNAL-NOTES
journal: j3
journal_title: "Open-Source Building-Systems Data Layers for Urban-Scale Site Analysis"
section: "§4 Architecture + §5 Implementation — bim-workspace-v1.0 schema (2026-06-02)"
state: draft-pending-editorial-review
originating_cluster: project-workplace
created: 2026-06-02
to: project-editorial
language_protocol: PROSE-TOPIC
bcsc_class: current-fact
research_trail:
  source_files:
    - app-workplace-http-prototype/src/workbench.rs
    - app-workplace-http-prototype/src/assets/bim.html
  commit: 3540c89f
  notes: >
    Stage 8 BIM schema implementation in the HTTP prototype. Verified via
    /bim route + /api/bim/files + /api/bim/create endpoints at http://10.8.0.9:9200.
    Create/list/read-back round-trip confirmed. bim/ directory auto-created at startup.
---

# JOURNAL-NOTES — J3 §4 Architecture + §5 Implementation
## bim-workspace-v1.0 Schema Decisions — 2026-06-02

**Routing:** project-editorial → J3 §4 Architecture + §5 Implementation

---

## Schema format decision — W3C DTCG JSON

The `bim-workspace-v1.0` schema uses W3C Design Token Community Group (DTCG) JSON
conventions. Each token carries three fields: `$value` (the token value), `$type` (the
value type descriptor, e.g. `color` or `boolean`), and `$description` (human-readable
intent). File extension is `.bim.json`; files are stored in a per-workspace `bim/`
directory that the prototype creates automatically at startup.

The DTCG format was chosen over proprietary JSON because it is an active W3C community
specification with tooling support in open-source design-system pipelines. A `.bim.json`
file produced by this schema can be consumed by any DTCG-aware token processor without
modification. This makes BIM visual configuration portable across rendering contexts:
a web-based IFC viewer (web-ifc, MPL-2.0), a PDF export pipeline, and a 2D floor plan
generator can all consume the same token file to apply consistent element styling.

No prior open-source tooling uses DTCG conventions for IFC element styling. Existing
BIM colour configuration systems are either proprietary (Revit view templates, Navisworks
Appearance Profiler) or viewer-specific (xeokit colour-coding, which is not
interoperable). The DTCG approach is an original contribution meriting citation in J3 §5.

## Schema structure

A `.bim.json` file contains two top-level keys:

The `project` block carries `title` (a human-readable project name) and `ifc-file` (a
path reference to the associated IFC-SPF source file). This block links the style token
file to the canonical IFC data it colours.

The `element-styles` block contains one child object per IFC type, keyed by the IFC
entity name (e.g. `IfcWall`). Each child object contains a `color` token (DTCG `$type:
color`, `$value` as a CSS hex string) and a `visible` token (`$type: boolean`). The
`visible` token allows an element type to be hidden from view without removing it from
the IFC file — a pattern used in Navisworks clash detection workflows to isolate
structural from architectural elements.

## IFC element type selection

Eight default IFC types are included in the initial schema: `IfcWall`, `IfcDoor`,
`IfcWindow`, `IfcSlab`, `IfcColumn`, `IfcBeam`, `IfcStair`, `IfcRoof`.

The selection criterion was coverage of visible model geometry in typical commercial
building IFC files. These eight types represent the primary structural and envelope
elements in an IFC model; together they account for the majority of polygon count
and all visible exterior surfaces in standard commercial construction. The selection
matches the default element categories exposed in Navisworks and Revit's visibility
graphics dialogs, which are the reference tools for the target user population.

Mechanical, electrical, and plumbing (MEP) elements (`IfcDuctSegment`, `IfcPipeSegment`,
etc.) are excluded from the initial set. MEP styling is a second-tier concern for the
coordination workflows the prototype targets; it is deferred to a subsequent schema
version to keep the initial set learnable.

## API surface

The `/bim` route serves `bim.html`, a two-panel editor: the left panel presents the
element styles as an editable table; the right panel shows the live DTCG JSON preview
updating in real time as values change.

Two backend endpoints manage workspace file operations:

`GET /api/bim/files` lists all `.bim.json` files in the workspace `bim/` directory,
returning a JSON array of filenames.

`POST /api/bim/create` creates a new `.bim.json` file with the eight-type default
schema populated. The endpoint accepts a filename parameter and rejects names that
would overwrite an existing file.

A create/list/read-back round trip was verified on 2026-06-01 at the prototype endpoint.

## Route to J3

These notes supplement J3 §5 Implementation. The DTCG-based BIM style schema is the
primary implementation contribution from the workplace surface to J3. The paper's
argument in §4 (open-source data layers require interoperable formats) is concretely
instantiated here: DTCG provides a W3C-backed interchange format for BIM visual
configuration that no prior open-source tool has adopted.

Cite the prototype endpoint (`/bim` at the workbench URL, commit `3540c89f`) alongside
the schema specification in §5.3. The schema file format should be formally specified
in a J3 appendix or supplementary material for reproducibility.
