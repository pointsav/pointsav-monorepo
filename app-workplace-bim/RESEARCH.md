# app-workplace-BIM — Research Context

**Document:** `RESEARCH.md`
**Repository:** `pointsav-monorepo/app-workplace-bim/`
**Version:** 1.0 — April 2026
**Full market research:** `content-wiki-documentation/research/RESEARCH-BIM-MARKET.md`
**Audience:** Developers joining this component · Architects and engineers evaluating the tool

---

## What This Application Is

`app-workplace-bim` is the **editor** in the PointSav BIM product family. It runs on `os-workplace` as a Tauri desktop application. It creates and edits BIM files. This is what distinguishes it from every other component in the stack:

- `service-bim` — maintains the archive (no UI, no authoring)
- `app-console-bim` — retrieves, coordinates, links work orders (Navisworks muscle memory)
- `app-orchestration-bim` — aggregates and delivers across archives (no direct operator UI)
- **`app-workplace-bim`** — **authors and edits BIM geometry and data** (Revit / AutoCAD muscle memory)

The output of `app-workplace-bim` is an IFC-SPF file saved directly into `service-bim/ingestion/queue/` on the target `cluster-totebox-property`. The `service-bim` daemon picks it up, validates it against `requirements/handover.ids`, and commits it to the canonical archive. The editor never writes directly to the canonical store — it writes to the queue. The daemon maintains the ledger.

This follows SYS-ADR-10: F12 is the human authorization point. In `app-workplace-bim`, the F12 equivalent is the explicit commit action that moves a file from the working draft into the ingestion queue for archive processing. The AI may assist with geometry suggestions or property extraction. It cannot execute the commit.

---

## The Three Muscle Memory Targets — Phased

The target users are the same professionals who currently use Autodesk products. The onboarding principle is identical to the rest of the Workplace family: a professional from the target tool should recognize the workspace and begin working within 30 seconds. The phasing is determined by implementation complexity, not by commercial priority.

### Phase 1 — AutoCAD Muscle Memory (2D Drafting)

AutoCAD is command-line-driven. Its defining characteristic, documented by professional training firms consistently, is that expert users keep their left hand on the keyboard at all times, right hand on the mouse, and never touch the ribbon for high-frequency operations. Speed comes from command aliases: single or double-letter inputs that fire instantly. The core 20 commands account for roughly 80% of drafting activity.

**The essential AutoCAD command set that Phase 1 must replicate:**

| Alias | Command | Operation |
|---|---|---|
| `L` | LINE | Draw line segments |
| `PL` | PLINE | Draw polyline (single object) |
| `C` | CIRCLE | Draw circle |
| `REC` | RECTANG | Draw rectangle as closed polyline |
| `A` | ARC | Draw arc |
| `H` | HATCH | Fill closed area |
| `M` | MOVE | Move selected objects |
| `CO` | COPY | Copy objects |
| `TR` | TRIM | Trim to cutting edge |
| `EX` | EXTEND | Extend to boundary |
| `O` | OFFSET | Offset parallel to existing |
| `MI` | MIRROR | Mirror across axis |
| `RO` | ROTATE | Rotate around point |
| `SC` | SCALE | Scale objects |
| `F` | FILLET | Round corner between two objects |
| `LA` | LAYER | Open layer manager |
| `Z` | ZOOM | Zoom controls |
| `E` | ERASE | Delete selected objects |
| `B` | BLOCK | Define block |
| `I` | INSERT | Insert block or drawing |

**F-key toggles** (the other half of AutoCAD muscle memory):

| Key | Function |
|---|---|
| F3 | OSNAP on/off |
| F8 | ORTHO on/off |
| F10 | Polar tracking on/off |
| F12 | Dynamic input on/off |

**What Phase 1 delivers:** a 2D drafting environment where the command line is the primary input, the canvas renders IFC geometry in 2D plan view as SVG (from the archive's `drawings/floor-plans/`), and all drafting operations write back to IFC elements rather than to DWG entities. The layer system maps to IFC building element categories, not to arbitrary layer names.

This last point is architecturally significant. In AutoCAD, layers are a display-management convention — they carry no semantic meaning. In `app-workplace-bim`, what appears to be a layer is an IFC element category: `IfcWall`, `IfcDoor`, `IfcSpace`, `IfcFurnishingElement`. The visual convention is identical; the data model is semantically rich.

### Phase 2 — Navisworks Muscle Memory (3D Coordination)

Navisworks is a 3D federated model viewer and coordination tool. Its defining operations for the property manager and contractor workflow are: navigating a 3D model, selecting elements to inspect their properties, creating section planes, measuring distances, and attaching issues (markup) to specific elements or viewpoints.

**The core Navisworks operations Phase 2 must replicate in 3D:**

| Operation | Navisworks convention | app-workplace-bim implementation |
|---|---|---|
| Orbit | Middle-mouse drag | Middle-mouse drag |
| Pan | Shift + middle-mouse | Shift + middle-mouse |
| Zoom | Scroll wheel | Scroll wheel |
| Select element | Left click | Left click → IFC GUID retrieved |
| Properties panel | Right-click → Properties | Automatic on selection |
| Section plane | Sectioning toolbar | Section plane tool |
| Measure distance | Review → Measure | Measure tool |
| Create issue | Review → Redline | BCF 3.0 issue → `service-bim/issues/` |
| Find item | Find Items panel | GUID or property search |

The 3D rendering engine for Phase 2 is `@thatopen/components` (MIT licence, ThatOpen Company) or `xeokit-sdk` (AGPL-3.0, EUPL-compatible) embedded in the Tauri webview. See licence note below.

### Phase 3 — Revit Muscle Memory (Parametric Authoring)

Revit is a parametric BIM authoring tool. Its defining capability — and the one most demanding to replicate — is that elements are parametric objects with intelligent relationships: a wall knows it has a door opening, a floor knows its boundary, a roof knows the walls it sits on. Changing a wall height propagates correctly to all dependent elements.

**Important scope note for Phase 3:** `app-workplace-bim` does not need to replicate Revit's full parametric engine to serve the target market. The property manager use case — the Woodfine showcase — does not require creating new buildings from scratch. It requires editing existing IFC models: updating element properties, adding elements, reclassifying objects, linking documents. Full parametric constraint solving is a Phase 3+ ambition, not a Phase 1–2 requirement.

**Phase 3 Revit operations in priority order:**

| Priority | Operation | Why it matters |
|---|---|---|
| 1 | Edit IFC element properties | Update material specs, classification, quantities after construction changes |
| 2 | Reclassify elements | Fix misclassified `IfcBuildingElementProxy` to correct type |
| 3 | Add new elements | Model as-built changes not reflected in original IFC |
| 4 | Delete or void elements | Remove demolished elements |
| 5 | Schedule view | IFC property sets → tabular view (CSV / spreadsheet export) |
| 6 | Tag and annotate | Dimension strings, text notes linked to elements |
| 7 | Simple walls, slabs, doors | Create new building elements from scratch |
| 8 | Parametric families | Custom element types with constraint relationships |

Items 1–6 are buildable with IfcOpenShell's Python API via a Rust subprocess today. Items 7–8 require a geometry kernel — see the Rust ecosystem note below.

---

## The Open-Source Authoring Toolchain

### Bonsai (formerly BlenderBIM) — The Reference Implementation

Bonsai is the most important open-source reference for `app-workplace-bim`. It is the only IFC-native authoring tool in production use — meaning the model IS an IFC file at all times, not a proprietary format with IFC export bolted on.

**Current state (April 2026):**
- Version 0.8.5 — released alongside IfcOpenShell 0.8.5, April 2026
- Runs inside Blender 4.2 LTS and Blender 5.1
- Licence: **GPL-3.0-or-later** (distinct from IfcOpenShell's LGPL-3.0)
- Maintained by Dion Moult and open-source contributors
- Capabilities: full IFC authoring including walls, slabs, doors, windows, MEP elements; 2D drawing generation; IFC property editing; BCF issue management; IDS validation; scheduling; classification management

**What Bonsai proves for PointSav:** IFC-native authoring in a Tauri/desktop application is architecturally feasible. Bonsai demonstrates the complete feature set. The PointSav implementation targets a different UX (AutoCAD/Revit muscle memory in a lean Tauri shell) and a different integration point (writes to `service-bim/ingestion/queue/` rather than to arbitrary disk locations), but the underlying IFC authoring operations are identical and IfcOpenShell provides the same library foundation.

**Licence note:** Bonsai is GPL-3.0. IfcOpenShell itself (the C++ library and Python API) is LGPL-3.0. `app-workplace-bim` uses **IfcOpenShell under LGPL-3.0**, not Bonsai directly, so the GPL does not propagate. The LGPL is satisfied by dynamic linking or subprocess invocation — both are clean patterns for a Rust/Tauri host.

**The IFC-native distinction — why it matters for developers:**

A professional training course published in April 2026 describes the Bonsai workflow: "Ryan treats the .ifc file like source code — tracked in Git, with a full history of every change." This is precisely the PointSav model. The IFC file in `service-bim/canonical/` is the source of truth. `app-workplace-bim` reads it, modifies it, and returns it to the ingestion queue. There is no intermediate proprietary format. There is no "save as IFC" step. The file IS IFC at every moment.

### IfcOpenShell 0.8.5 — The Engine

IfcOpenShell (LGPL-3.0, version 0.8.5, April 2026) is the production IFC library that `app-workplace-bim` invokes for all geometry and data operations. Its capabilities relevant to this application:

| Capability | IfcOpenShell API | Phase |
|---|---|---|
| Read/write IFC-SPF | `ifcopenshell.open()`, `ifcopenshell.write()` | 1 |
| Read/write ifcJSON | `ifcopenshell.geom.*` | 1 |
| Generate SVG floor plans | `ifcopenshell.geom.*` + SVG slicer | 1 |
| Tessellate geometry for 3D | `ifcopenshell.geom.create_shape()` | 2 |
| Convert to glTF | `IfcConvert` CLI | 2 |
| Edit element properties | `element.OverriddenAttributes` | 1 |
| Reclassify elements | `api.root.reassign_class()` | 3 |
| Create new elements | `api.root.create_entity()` | 3 |
| Validate against IDS | `ifctester.ids.Ids()` | 1 |
| Compute differences between versions | `ifcdiff.IfcDiff()` | 2 |

The Rust host invokes IfcOpenShell operations via Python subprocess — a clean LGPL-compliant pattern. The Python process receives an IFC file path, executes operations, and returns results as JSON or as a modified IFC file. No shared memory, no dynamic linking complexity.

### web-ifc 0.77 — The Browser-Side Parser

`web-ifc` (MPL-2.0, ThatOpen Company, version 0.77, March 2026) is the WASM-compiled IFC parser used in the Tauri webview for real-time 3D rendering. It parses IFC-SPF in the browser environment, converts geometry to Three.js-compatible mesh data, and feeds the `@thatopen/components` rendering pipeline.

The Rust backend owns the authoritative IFC file. The webview uses web-ifc to display it. When the operator makes a change in the 3D view, the webview sends the change as a structured command (element GUID + property key + new value, or geometry delta) to the Rust IPC layer, which invokes IfcOpenShell to apply the change to the canonical file. The webview never modifies the IFC directly — it is a display surface.

### FreeCAD BIM Workbench — Alternative Reference

FreeCAD 1.0 (November 2024, LGPL-2.1) shipped a unified BIM workbench that uses IFC as the live data model rather than an import format. Like Bonsai, it demonstrates IFC-native authoring on a production codebase. FreeCAD's Python API is accessible from external processes, making it an alternative subprocess target for geometry operations that IfcOpenShell does not cover natively. FreeCAD uses OpenCascade (LGPL-2.1) as its geometry kernel — the most mature open-source b-rep kernel available.

---

## The Rust Ecosystem for Geometry

Phase 1 (2D drafting) and element property editing are fully achievable with IfcOpenShell and the current Rust ecosystem. Phase 3 (parametric authoring, constraint solving, new element creation with correct b-rep geometry) requires either a native Rust geometry kernel or continued delegation to IfcOpenShell/FreeCAD via subprocess.

**Current Rust geometry landscape (April 2026):**

| Crate | Purpose | Production-ready? |
|---|---|---|
| `glam` 0.32 | 3D math (vectors, matrices, transforms) | Yes |
| `nalgebra` 0.33 | Linear algebra | Yes |
| `parry3d` | Spatial queries, collision, bounding volumes | Yes |
| `kurbo` | 2D curves and paths (Bézier, lines) | Yes |
| `lyon` | 2D tessellation for GPU rendering | Yes |
| `resvg` + `tiny-skia` | SVG rendering | Yes |
| `gltf` v1.4 | glTF 2.0 read/write | Yes |
| `wgpu` | Cross-platform WebGPU (native GPU) | Yes |
| `Truck` | Pure Rust b-rep CAD kernel | Pre-production |
| `Fornjot` | Next-gen Rust CAD kernel | Pre-production |
| `opencascade-rs` | Rust bindings to OpenCascade | Experimental |
| `ifc_rs` 0.1.0-alpha | IFC-SPF parser in Rust | Alpha — read-only subset |
| `ifc-lite-core` | IFC4/IFC4X3/IFC5 Rust parser | Alpha — unvalidated claims |

**The practical architecture decision for Phase 1–2:** all geometry operations delegate to IfcOpenShell via Python subprocess. The Rust layer owns file I/O, IPC, the ingestion queue handoff, SHA-256 sealing, and the Tauri shell. Geometry belongs to IfcOpenShell until a Rust-native IFC kernel reaches production grade — expected no earlier than 2027 based on current `ifc_rs` and `ifc-lite-core` trajectories.

---

## Licence Architecture

The `app-workplace-bim` licence stack:

| Component | Licence | Integration method | Clean? |
|---|---|---|---|
| Tauri 2.x | MIT / Apache 2.0 | Direct dependency | Yes |
| IfcOpenShell | LGPL-3.0 | Python subprocess (dynamic, not linked) | Yes |
| Bonsai | GPL-3.0 | Not used directly — reference only | N/A |
| web-ifc 0.77 | MPL-2.0 | Webview JS module | Yes (MPL source-file copyleft only) |
| @thatopen/components | MIT | Webview JS module | Yes |
| xeokit-sdk | AGPL-3.0 | Webview JS module | Yes (EUPL-1.2 + AGPL-3.0 compatible per EC Joinup) |
| FreeCAD (optional) | LGPL-2.1 | Python subprocess | Yes |
| OpenCascade (via FreeCAD) | LGPL-2.1 | Transitive via FreeCAD subprocess | Yes |
| wgpu (if native renderer) | MIT / Apache 2.0 | Rust direct dependency | Yes |
| gltf crate | MIT / Apache 2.0 | Rust direct dependency | Yes |
| **app-workplace-bim** | **EUPL-1.2** | — | — |

The LGPL compliance for IfcOpenShell and FreeCAD is satisfied by subprocess invocation: the Rust host launches a Python process, passes file paths as arguments, and receives results via stdout/JSON. There is no static linking and no dynamic linking into the Tauri binary. This is the same pattern IfcConvert uses in production.

---

## The IFC Authoring Data Flow

```
Operator action in app-workplace-bim
        │
        ▼
Tauri IPC (5 commands maximum per ADR pattern)
  open_ifc_file          → returns IFC path to webview
  save_ifc_draft         → writes working draft to temp location
  commit_to_queue        → F12 equivalent, moves draft to ingestion/queue/
  invoke_ifcopenshell    → subprocess for geometry operations
  get_archive_dir        → returns cluster-totebox-property path
        │
        ▼
IfcOpenShell Python subprocess
  Reads canonical IFC from service-bim/canonical/model.ifc
  Applies operation (edit property / add element / reclassify)
  Writes modified IFC to working draft location
  Returns operation result as JSON
        │
        ▼
Webview re-renders via web-ifc
  Parses updated IFC
  Refreshes 3D scene / 2D floor plan
  Operator reviews changes
        │
        ▼
F12 commit action
  Tauri moves working draft → service-bim/ingestion/queue/
  service-bim daemon picks up, validates, seals, archives
  Immutable ledger entry written via service-fs
```

No AI touches this data flow at any point (SYS-ADR-07). The `service-slm` gateway is available for unstructured text in operator notes or document descriptions — it is never invoked on IFC geometry or structured property data.

---

## The Competitive Context for Authoring Tools

The AEC authoring market is dominated by Revit (Autodesk, ~$3,000/seat/year) and ArchiCAD (Graphisoft/Nemetschek, subscription-only from 2026 at comparable pricing). Both are moving to subscription-only models. Both generate files that are not readable without the originating software at the same version or newer. Revit 2005 files require Autodesk conversion services to open in Revit 2026.

The open-source alternative — Bonsai — is IFC-native and free. Its limitation for the PointSav target market is UX: Blender's interface, while powerful, does not present the command-line-centric, keyboard-driven workflow that AutoCAD professionals expect. `app-workplace-bim` addresses this gap: IFC-native authoring with AutoCAD/Revit muscle memory, in a lean Tauri shell that integrates directly with the PointSav archive stack.

No shipping product in April 2026 combines:
- IFC-native authoring (not IFC-export-from-proprietary)
- AutoCAD command-line keyboard muscle memory
- Direct integration with a persistent, flat-file, offline-capable property archive
- F12 authorized commit to an immutable ledger
- EUPL-1.2 licence

That combination is what `app-workplace-bim` delivers.

---

## ADR Pre-Commitments

The following ADRs from the platform apply to this application without exception. Any new ADR specific to `app-workplace-bim` must be numbered `BIM-WP-ADR-*`.

| ADR | Rule | Implication for this app |
|---|---|---|
| SYS-ADR-07 | Structured data never routes through AI | IFC geometry, property sets, and quantities are always processed by IfcOpenShell, never by `service-slm` |
| SYS-ADR-10 | F12 is the mandatory human checkpoint | Operator must explicitly commit changes to the ingestion queue; no automated archive writes |
| SYS-ADR-19 | Automated AI publishing to verified ledgers is prohibited | `service-slm` may suggest; it never commits |

---

## Key References

**Open-source toolchain:**
- IfcOpenShell 0.8.5 documentation: [docs.ifcopenshell.org](https://docs.ifcopenshell.org)
- IfcOpenShell GitHub: [github.com/IfcOpenShell/IfcOpenShell](https://github.com/IfcOpenShell/IfcOpenShell)
- Bonsai (BlenderBIM): [bonsaibim.org](https://bonsaibim.org) — GPL-3.0
- Bonsai Blender Extensions: [extensions.blender.org/add-ons/bonsai](https://extensions.blender.org/add-ons/bonsai)
- web-ifc: [thatopen.github.io/engine_web-ifc](https://thatopen.github.io/engine_web-ifc/docs/)
- @thatopen/components: [github.com/ThatOpen](https://github.com/ThatOpen)
- xeokit-sdk: [github.com/xeokit/xeokit-sdk](https://github.com/xeokit/xeokit-sdk)
- FreeCAD BIM Workbench: [freecad.org](https://www.freecad.org)

**Standards:**
- IFC-SPF: ISO 16739-1:2024
- IDS 1.0: [technical.buildingsmart.org](https://technical.buildingsmart.org/projects/information-delivery-specification-ids/)
- BCF 3.0: [technical.buildingsmart.org](https://technical.buildingsmart.org/projects/bim-collaboration-format-bcf/)

**AutoCAD muscle memory:**
- Autodesk. *AutoCAD Keyboard Shortcuts Guide.* [autodesk.com/shortcuts/autocad](https://www.autodesk.com/shortcuts/autocad)
- Interscale Education. *AutoCAD Keyboard Shortcuts For Faster Drafting.* February 2026. [interscaleedu.com](https://interscaleedu.com/en/blog/autocad-keyboard-shortcuts/)
- CAD Authority. *Essential AutoCAD Shortcuts and Commands List.* April 2026. [cadauthority.com](https://cadauthority.com/guides/autocad-shortcuts-commands-list/)

**IFC authoring case studies:**
- BlenderNation. *Learn BonsaiBIM with a Full Project.* April 2026. [blendernation.com](https://www.blendernation.com/2026/04/03/learn-bonsaibim-with-a-full-project-a-course-by-architect-ryan-schultz/)
- EngineeringSkills.com. *Bonsai BIM — The Essential IFC Tool for Structural Engineering Workflows.* December 2025. [engineeringskills.com](https://www.engineeringskills.com/posts/bonsai-bim-the-essential-ifc-tool-for-structural-engineering-workflows)

**Full market research:** `content-wiki-documentation/research/RESEARCH-BIM-MARKET.md`
