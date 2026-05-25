---
schema: foundry-sub-agent-result-v1
brief_id: BB.4
authored: 2026-04-28
authored_by: research sub-agent (Sonnet 4.6)
cluster: project-bim
target_consumer: app-workplace-bim Task Claude
---

# BB.4 — Bonsai Interface Deep-Dive

## 1. Bonsai Overview

**Naming and history.** The tool was called the BlenderBIM Add-on from its inception through mid-2024. Version 0.8.0, released September 2024, introduced the rename to Bonsai — announced by Dion Moult as a deliberate rebranding to separate the project's identity from the Blender name and to signal its maturity as a standalone OpenBIM platform. The rename coincided with the move to Blender's new official Extensions platform (replacing the older manual add-on install path) and support for Blender 4.2 LTS.

**Current version.** As of April 2026, the stable release is **Bonsai 0.8.5**. Alpha builds for 0.8.6 are releasing daily (CI artifacts named `bonsai-0.8.6-alpha<date><time>`).

**Blender version requirements.** Bonsai 0.8.5 targets Blender 4.2 and later. The project officially supports the Python version shipped with the three most recent major Blender releases. Blender 5.0 and its Python 3.13 requirement are tracked in an open compatibility issue (#7623) as of early 2026.

**IfcOpenShell version.** Bonsai ships with and depends on IfcOpenShell 0.8.5, the same version series. The two are released together; Bonsai is architecturally a thick GUI layer on top of IfcOpenShell's Python bindings.

**Install footprint.** Download sizes for Bonsai 0.8.5 as a Blender extension:
- Windows x64: ~118 MB
- macOS Intel: ~133 MB
- macOS Apple Silicon: ~130 MB
- Linux x64: ~145 MB

Alpha builds from the GitHub releases page confirm sizes in the 115–143 MB range across Python 3.11, 3.12, and 3.13 variants. The install bundles IfcOpenShell, all Python dependencies, and IFC schema definition files. No separate database or server process is required.

**Platform support.** Linux x64, macOS (Intel and Apple Silicon), Windows x64. No mobile client. No browser-based version (a separate `ifctester` web tool exists for IDS validation only, not authoring). No iOS or Android distribution. The tool requires a desktop-class machine with GPU support for Blender's viewport rendering — minimum hardware is whatever Blender 4.2 requires.

**License.** GPL-3.0. IfcOpenShell itself is LGPL-3.0.

**Governance and development.** Volunteer-driven, coordinated by Dion Moult and the OSArch community. Primary repository: `github.com/IfcOpenShell/IfcOpenShell`. Issues, feature requests, and release tracking are all public.

---

## 2. Interface Anatomy

### Blender host panels Bonsai repurposes

Bonsai is not a standalone application — it is a Blender extension. The host application's UI is repurposed as follows:

**Outliner (top-right in default layout).** In a standard Blender session the Outliner shows scene objects, collections, and data-blocks. When a Bonsai/IFC project is loaded, the Outliner displays IFC-mapped collections. The default spatial hierarchy (`IfcProject > IfcSite > IfcBuilding > IfcBuildingStorey`) appears as a collection hierarchy. `IfcWallType`, `IfcSlabType`, etc. appear in a `Types` collection. The Outliner is Bonsai's closest equivalent to a traditional AEC Spatial Tree, but it is Blender's Outliner, not a custom tree widget.

**Properties Editor (right sidebar, icon tabs).** This is the primary data panel in Bonsai. Blender's Properties Editor is normally organized by data domain (Object, Modifier, Physics, etc.). Bonsai replaces or augments several of those tabs with IFC-aware counterparts and adds new tabs entirely. The full tab set exposed when an IFC project is loaded:

| Tab icon | Tab name | Primary content |
|---|---|---|
| Scene | Project Overview | Project Info, spatial hierarchy, geometry contexts, stakeholders, grouping |
| Object | Object Information | IFC class, attributes, GlobalId, Psets, Qsets, spatial container, classification |
| (custom) | Geometry and Materials | Material assignments, parametric geometry controls, representation contexts |
| (custom) | Drawings and Documents | Drawing sheets, view definitions, annotations |
| (custom) | Services and Systems | MEP systems, circuits, zones |
| (custom) | Structural Analysis | Analytical model, loads, reactions |
| (custom) | Costing and Scheduling | Cost schedules, quantity takeoff, work schedules |
| (custom) | Facility Management | BrickSchema integration, FM data (WIP) |
| (custom) | Quality and Coordination | IDS, clash detection, BCF (WIP) |

Navigation between tabs uses a horizontal icon row at the top of the Properties Editor. `Ctrl+Tab` toggles between the two most recently active tabs — this is a Bonsai-specific hotkey that overrides Blender's default.

**3D Viewport (centre).** Standard Blender 3D viewport. Bonsai adds a left-side toolbar (T-panel) with IFC-specific tools (Create Wall, Create Slab, Create Door, Create Window, Create Column, Create Beam, Create Duct, Spatial Tool, Structural Tool, Annotation Tool, Covering Tool, Explore Tool). The N-panel (right sidebar, toggled with N key) is also available and can carry per-tool option panels, but Bonsai's main data panels are in the Properties Editor, not the N-panel.

**Header/Top Bar.** Bonsai modifies the standard File menu (`File > New IFC Project`, `File > Open IFC Project`, `File > Save IFC Project`). These are separate from Blender's own `.blend` file operations. The status bar at the bottom of the Blender window shows counts of selected objects and serves the StatusBar role.

### Mapping to the 10 universal AEC interface components

| Universal component | Bonsai equivalent | Location |
|---|---|---|
| SpatialTree | Outliner (collection hierarchy) + Properties > Project Overview > Spatial Decomposition panel | Outliner top-right + Properties Editor Scene tab |
| PropertiesPanel | Properties > Object Information tab (attributes, Psets, Qsets, classification, spatial container) | Properties Editor |
| Viewport3D | Blender 3D Viewport (standard) | Centre |
| ViewNavigator | Navigate Gizmo (top-right corner of viewport) + mouse (MMB orbit, scroll zoom, Shift+MMB pan) + Explore Tool for first-person WASD walk | Viewport overlay |
| Toolbar | Left T-panel inside 3D Viewport; IFC tools listed above | Left side of 3D Viewport |
| StatusBar | Blender status bar (bottom) — shows selected object count and basic info | Bottom |
| SelectionFilter | Properties > Scene > IFC Categorisation panel — filter objects by class, type; also Blender's standard box/circle select plus Bonsai-aware selection operators | Properties Editor + viewport |
| TypeBrowser | No dedicated panel — types live in the Outliner's Types collection; a third-party IFC Product Library add-on provides a sidebar browser (community-developed, not bundled with Bonsai) | Outliner / add-on sidebar |
| SectionPlane | Section plane tool in 3D Viewport (specific implementation documented as WIP); drawing views use IfcAnnotation section views | Properties > Drawings tab |
| AnnotationLayer | Properties > Drawings and Documents tab — annotations, text, symbols, dimensions | Properties Editor |

The **Outliner doubles as the SpatialTree**. There is no separate dedicated spatial tree widget with expand/collapse controls customized for AEC — architects learn to read the Outliner as the hierarchy display. This is a significant ergonomic compromise versus dedicated AEC tools like Revit's Project Browser or ArchiCAD's Navigator.

---

## 3. IFC Authoring Workflows

### (a) Create a new project from IFC template

**UI path:** `File > New IFC Project > New Metric (m) Project` (or mm, ft, Demo, or Project Wizard).

**Project Wizard path (more control):** `File > New IFC Project > New Project Wizard` or `Properties > Project Overview > Project Info`. Configures:
- IFC schema version: IFC2X3, IFC4, IFC4X3
- Unit system and specific unit choices
- Template: Blank or IFC4 Demo

**What gets automatically created:**
- IfcProject entity
- IfcSite
- IfcBuilding
- IfcBuildingStorey (one, labelled "My Storey")
- Default geometric representation contexts (model 3D + plan 2D)
- Owner history record

The default cube, camera, and light that Blender inserts into every new session are automatically removed by Bonsai during project setup. Custom pre-existing objects are preserved.

**Saving distinction:** `Ctrl+S` maps to `Save IFC Project` (writes the `.ifc` file). The `.blend` file must be saved separately via standard Blender save. The `.ifc` file is the authoritative BIM record; the `.blend` carries Blender-specific display settings.

**What is persisted to IFC:** All entity data — spatial hierarchy, element instances, type definitions, property sets, classification assignments, owner history, geometric representations — is written to the SPF (STEP Physical File) serialization. Bonsai's native-IFC approach means edits are parsimonious: only changed entities are rewritten, making the file diffable.

### (b) Place a wall/door/window with proper Pset_* attached

**Wall creation:**
1. Activate the Create Wall tool from the T-panel (left toolbar).
2. Click to set start point; click again to set end point. Wall is created in the active storey's spatial container.
3. Height and thickness are set in the tool options panel (N-panel or top bar).
4. Wall receives an IfcWall entity. Psets are not automatically attached at creation; they must be assigned separately via Properties > Object Information > Psets.

**Door creation:**
1. Select the host wall first (this establishes the void relationship automatically).
2. Position the 3D cursor at the desired location on the wall.
3. Activate Create Door from T-panel.
4. In the tool header: enter a type name (e.g., `DOOR001`), click `+ Add IfcDoorType`.
5. Press `Shift+A` or click Add to instantiate the door.
6. Width/height adjustable via top bar parameters.
7. `Shift+G` regenerates the wall geometry to cut the opening.
8. `Shift+F` flips door swing direction.
9. If wall was not pre-selected: `Shift+O` manually applies the void relationship.

**Window creation:** Same sequence as door, substituting `Create Window` and `+ Add IfcWindowType`. Keyboard shortcuts identical (`Shift+A`, `Shift+G`, `Shift+O`).

**Attaching Psets:** After placing an element, select it, go to `Properties > Object Information`. The Psets section lists existing property sets. To add a Pset:
- Standard IFC Psets (names prefixed `Pset_`) are sourced from CSV template files bundled in Bonsai's data directory.
- Custom Psets are defined as CSV files placed in `bim/data/pset/<PsetName>/` — the folder name becomes the Pset name. The CSV contains property-name/value pairs.
- Once a Pset template is present, it appears in a dropdown for assignment to selected elements.
- CSV-sourced property values propagate to all assigned elements automatically on IFC export; no per-element re-assignment required for template values.

**What is persisted:** The void relationship (IfcRelVoidsElement) between door/window and host wall, the IfcDoor/IfcWindow entity with its IfcDoorType/IfcWindowType, all Pset assignments (IfcRelDefinesByProperties), and parametric geometry are written to the IFC file on save.

**Known limitation:** Moving a window using standard Blender transform tools (G key) does not reliably update the IFC model position. Positional changes should be made via Bonsai's own placement controls rather than raw Blender transforms.

### (c) Generate IFC space classifications and zones

**Spatial Tool** (T-panel) creates IfcSpace elements:
1. Switch to floor plan view (Numpad 7 for top view, Numpad 5 for orthographic).
2. Activate the Spatial Tool.
3. Click to define the space boundary polygon.
4. Space is assigned to the active storey automatically.
5. In Properties > Object Information: assign IFC class `IfcSpace`, set Name, LongName, and space type (e.g., INTERNAL, EXTERNAL).

**IfcZone assignment:** Zones aggregate spaces. Created via Properties > Scene > IFC Categorisation or via scripting. There is no dedicated zone-creation panel with a clear graphical workflow in the documented UI; zone management is primarily a data-relationship operation done through the Properties panels or Python scripting.

**Classification systems (e.g., Uniclass 2015, OmniClass):** Assigned via Properties > Scene > IFC Classifications panel. The classification source data must be available as a configuration file in the project setup. From Properties > Object Information > Object Metadata, the classification reference can be reassigned per element.

### (d) Export back to IFC-SPF

`File > Save IFC Project` (`Ctrl+S`) — writes the current in-memory IFC data to the `.ifc` file in SPF format. There is no separate export step; Bonsai operates directly on the IFC data structure (native-IFC pattern). The file is human-readable STEP Physical File format.

For a distinct output file: `File > Save IFC Project As...` opens a path selector.

The native-IFC approach means the serialized output is minimal-delta relative to any previously saved version — only modified entities are rewritten. This makes the output suitable for line-by-line diff and asynchronous multi-author workflows without a central lock.

---

## 4. Pset / Quantity / Classification Editor

### Property sets (Psets)

Psets in Bonsai are CSV-file-driven. The data directory structure:

```
bonsai/bim/data/pset/
  Pset_WallCommon/
    Pset_WallCommon.csv
  Pset_DoorCommon/
    Pset_DoorCommon.csv
  <CustomPsetName>/
    <CustomPsetName>.csv
  ...
```

Each CSV has rows of `PropertyName, DataType, DefaultValue` (exact format varies; community documentation describes `property_name, property_value` pairs at minimum).

**Custom Psets:** Folders NOT prefixed `Pset_` are user-defined. Create a folder with your desired Pset name, place a CSV inside it, and Bonsai will surface that Pset in the assignment dropdown. The Pset name must not start with `Pset_`, `Qto_`, or `ePset_` — those prefixes are reserved for official buildingSMART Psets.

**Assignment workflow:**
1. Select elements.
2. Properties > Object Information > Psets section.
3. Click the `+` button next to the Pset name to assign.
4. Property values are editable inline in the Properties panel after assignment.

**Quantity sets (Qsets):** Similar mechanism to Psets. `Qto_` prefix is the official quantity set convention. Quantities can be calculated via the `Q` shortcut in some tools (e.g., `Q` when a window is selected to calculate its quantities). The Costing and Scheduling tab in the Properties Editor handles quantity takeoff workflows.

### Classification

**IfcClassification assignment** (Uniclass, OmniClass, etc.):
- Scene-level: Properties > Scene > IFC Classifications panel — links a classification library to the project.
- Element-level: Properties > Object Information > Object Metadata — dropdown to assign a specific classification reference (table code + description) to the selected element.

Classification source data must be present as a file in the project's configuration directory. Bonsai does not bundle Uniclass or OmniClass data directly due to licensing; the user provides the reference files.

**IfcClassification vs. IfcType:** Bonsai exposes both concepts. Types (IfcWallType, IfcDoorType, etc.) are defined once and instanced many times. Types carry shared Psets. Classifications are independent reference codes (Uniclass 2015 Ss table, for example) assigned per element or per type. These are editable through different sections of the Object Information tab.

---

## 5. IDS Support — IfcTester Integration

**IDS (Information Delivery Specification)** is the buildingSMART standard for formally specifying what IFC data must be present and valid in a model.

**Bonsai / ifctester integration:** Bonsai includes IDS validation capability via the `ifctester` library (part of the IfcOpenShell ecosystem). The validation is accessible from the Quality and Coordination tab in the Properties Editor, though as of 0.8.5 the official documentation for this panel is marked as a work in progress.

**Workflow (from available community documentation and search results):**
1. Load or author an IFC model.
2. Navigate to Properties > Quality and Coordination.
3. Provide the path to an `.ids` file.
4. Run validation. IfcTester checks each applicable requirement in the IDS against every relevant element in the model.
5. Results appear in the panel as a pass/fail report — elements failing requirements are listed with the specific failing facet (attribute, property, classification, etc.).

**Result storage:** Results are surfaced in the Blender panel UI. They can be communicated to collaborators by exporting BCF topics from the failing elements (the workflow bridges IDS result → BCF topic creation). The `.ids` file itself is an external XML document; Bonsai does not modify it.

**Web alternative:** A separate browser-based `ifctester` tool exists at `ifctester.com` (part of the IfcOpenShell project). It allows creating and validating IDS files without Blender. For `app-workplace-bim`, this web tool is a relevant reference for how IDS validation results should be presented — the web tool shows a structured pass/fail tree with specification names, requirement types, and failing entity GlobalIds.

---

## 6. BCF Authoring

**BCF (BIM Collaboration Format)** is the buildingSMART standard for issue tracking that is model-linked — topics include viewpoints that capture camera position and a set of highlighted/hidden components.

**Bonsai BCF capability:** BCF authoring is present in Bonsai and accessible via the Quality and Coordination tab (Properties Editor). The feature is marked work-in-progress in official documentation, but community usage is documented.

**Workflow (from community sources):**
1. Properties > Quality and Coordination > BCF section.
2. Create a new topic: set title, type (Coordination, Issue, Request, Solution), status (Open, Closed, etc.), priority, assignee.
3. Capture a viewpoint: position the 3D Viewport as desired; click "Add Viewpoint". The viewpoint stores camera matrix plus selected/hidden component states.
4. Add comments to the topic thread.
5. Multiple viewpoints per topic are supported.
6. Export: BCF zip export writes a `.bcfzip` file conforming to BCF 2.1 or 3.0.
7. Import: Load a `.bcfzip` from a collaborator; topics and viewpoints appear in the panel. Clicking a viewpoint restores the camera and component visibility state.

**BCF 3.0 support:** Confirmed available in IfcOpenShell 0.8.5 (the BCF Python library is a first-class component of IfcOpenShell alongside Bonsai).

**Limitation noted in community discussion:** The BCF panel in Bonsai has many options, and the discoverability of the workflow is poor. New users report confusion about where to start. The documentation gap is acknowledged.

---

## 7. Spatial Element Creation

### The IFC spatial hierarchy

Bonsai strictly enforces the IFC spatial structure:

```
IfcProject
  └── IfcSite
        └── IfcBuilding
              └── IfcBuildingStorey
                    └── IfcSpace (optional)
                    └── IfcWall, IfcDoor, etc. (building elements)
```

All IFC objects must belong inside this tree. New objects created without explicit spatial container assignment are automatically placed in the active storey.

### Where the hierarchy lives in the UI

**Properties > Project Overview > Spatial Decomposition panel:** The primary tree display. Shows the IfcProject → IfcSite → IfcBuilding → IfcBuildingStorey chain with expand/collapse. From this panel, storeys can be added, renamed, and reordered. Clicking a storey makes it the active container for new element creation.

**Outliner:** The same hierarchy is reflected in Blender's Outliner as nested collections. The Outliner is the secondary view — changes in the Properties panel propagate to the Outliner, and vice versa.

**Properties > Object Information > Spatial Container panel:** For a selected element, shows which storey (or space) it is contained in. The container can be changed here — moving an element to a different storey.

### Creating spatial elements

**Site / Building / Storey:** Created via the Spatial Decomposition panel's add buttons (`+` next to the appropriate level). The panel supports adding multiple buildings on one site, multiple storeys per building.

**Space:** The Spatial Tool (T-panel in 3D Viewport) creates IfcSpace entities. Click to draw the boundary polygon in the active floor plan view.

### Blender collections as IFC spatial containers

Bonsai maps IFC spatial containers to Blender collections. An IfcBuildingStorey named "Ground Floor" becomes a Blender collection named "Ground Floor". Building elements contained in that storey appear nested within that collection in the Outliner. This is the bridge between Blender's collection-based scene graph and IFC's IfcRelContainedInSpatialStructure relationship. The mapping is transparent in normal use but becomes visible when using advanced Blender features or Python scripting.

---

## 8. Sectioning and Views

### Section planes

Section planes in Bonsai clip the 3D Viewport to show a cut through the model. They are stored as IfcAnnotation entities (with representation context set to the plan or section subcontext) in the IFC file, making them part of the model data, not just a display state.

**Creating a section plane:** Activate the relevant drawing view tool from Properties > Drawings and Documents. Define the cut plane by specifying elevation/orientation parameters. The viewport clips to show the section.

### Drawing generation

Bonsai can generate 2D drawing output (floor plans, sections, elevations) from the 3D model:

- **Floor plans:** Generated by defining a plan view at a specified elevation cut. The output captures wall profiles, door/window openings, room annotations.
- **Sections and elevations:** Similar process — define a cutting plane or elevation target, generate the view.
- **Annotation layer:** Text labels, dimension strings, room tags, door/window tags, grid lines, material hatches are added via the Annotation Tool and managed in the Drawings and Documents tab.
- **Sheet composition:** Multiple views are placed on drawing sheets. Output format is SVG-based (Bonsai uses SVG as the intermediate format for 2D drawings).

**What is persisted:** Section views, their camera/cut definitions, annotations, and sheet layouts are all stored as IFC entities (IfcShapeRepresentation with plan context, IfcAnnotation, IfcDocumentInformation for sheets). The IFC file carries the drawing metadata, not just the 3D geometry.

**Limitation:** The drawing generation workflow documentation is explicitly marked incomplete as of 0.8.5. The feature works but the learning curve is high and the documentation does not cover it comprehensively.

### Saved views

Camera positions and viewport states can be saved within Blender using standard Blender camera objects. Bonsai adds IFC context by associating camera views with IfcShapeRepresentation plan contexts. There is no dedicated "saved views" panel analogous to Revit's View Browser; views are managed through the Drawings tab.

---

## 9. Library / Type Browser

### How types work in Bonsai

IFC types (IfcWallType, IfcDoorType, IfcWindowType, etc.) are first-class entities in Bonsai. A type carries shared geometry (the type's default shape) and shared Psets. All instances of that type inherit both. Types appear in the Outliner under a `Types` collection, organized by IFC class.

**Creating a type:** During element creation (e.g., adding a door), the tool header shows a type name input and `+ Add IfcXxxType` button. Click to create the type before instantiating the element.

**Assigning an existing type to an element:** Properties > Object Information — the type assignment dropdown lists all available types of the matching class. Changing the type reassigns the element's shared geometry and shared Psets.

**Editing a type's shared properties:** Select the type object in the Outliner (from the Types collection), then Properties > Object Information shows the type's attributes and Psets editable in the same panel used for element instances.

### IFC library files

Bonsai supports linking external IFC library files — separate `.ifc` files that contain type definitions for reuse across projects. A library file is a standard IFC file; types are inserted from it into the active project via copy operations.

**Community product library add-on:** A third-party add-on (not bundled with Bonsai, developed separately by a community contributor) provides a sidebar panel to browse, search, and insert manufacturer IFC products. It includes an import wizard for converting OBJ/STL/glTF files into catalogued IFC library products. The add-on works with manufacturer IFC downloads (e.g., from Armitage Shanks, Hansgrohe catalogues). As of 2024–2025 this was in feedback/development stage; it is not part of the core Bonsai distribution.

**Gap:** There is no bundled, ready-to-use type browser UI in Bonsai comparable to Revit's Type Selector or ArchiCAD's GDL Object Library. Type management is functional but requires understanding the Outliner's Types collection structure — not discoverable for new users.

---

## 10. What Bonsai Does Not Do Well

The following are gaps that `app-workplace-bim` should explicitly address in its scope and design decisions.

### Ergonomics for non-architects

Bonsai's UI vocabulary assumes Blender familiarity. New users face two learning curves simultaneously: Blender's general 3D application paradigm (middle-mouse navigation, keyboard shortcut density, non-modal tool activation, Properties Editor tab structure) and IFC data concepts on top of that. The Blender host is an asset for power users and a barrier for everyone else.

The tool has been described by the project itself as "not ready for regular production work if you're not willing to expend a lot of pain and effort" — this was accurate for earlier versions and the complexity has only grown as features were added. For a property manager, building operator, or FM technician who does not use 3D software daily, the interface is inaccessible without training.

### FM workflows — work orders, leases, sensor integration

Bonsai's Facility Management tab exists in the Properties Editor but is marked as a work in progress in the 0.8.5 documentation. The tab references BrickSchema integration (a smart-building semantic ontology for sensor/system description) but provides no documented workflow for:
- Creating or tracking work orders
- Lease record management
- Room/space availability or booking
- Integration with CMMS or CAFM systems
- IoT sensor data visualization against IFC spaces
- Maintenance schedule management

The FM tab is structurally present as a placeholder for future development. It is not an operational FM tool.

### Offline-archive-first model

Bonsai operates on local files — this is a strength for the open-source community but means there is no built-in concept of a versioned model repository, change history beyond the single-file delta, or access-controlled model archive. Multi-user workflows require manual file exchange or external version control (e.g., a git-based workflow using Bonsai's native-IFC parsimonious serialization). There is no hosted service, no model server integration, and no offline-sync model aligned to a property archive of record.

### Mobile field use

No mobile client. Bonsai requires Blender on a desktop or laptop. There is no companion iOS/Android app, no lightweight mobile viewer that syncs with a Bonsai-authored model, and no tablet-optimized interface for field inspection tasks. A field technician cannot use Bonsai to confirm a space classification, scan a QR code linked to an IFC element, or submit a defect as a BCF topic from a phone.

### Small property manager persona

Bonsai's conceptual model is the IFC data schema exposed directly — "no translation layer." This is a correct architectural decision for interoperability and openness, but it means the user must understand IfcBuildingStorey, IfcRelContainedInSpatialStructure, Pset templates, and schema versions to use the tool effectively. A property manager who owns a 12-unit apartment building and wants to track maintenance, store lease data, and view a floor plan has no path to productivity in Bonsai without a substantial investment in learning IFC semantics and Blender navigation.

### IFC schema exposure as a UI anti-pattern (for some personas)

The direct-IFC philosophy is Bonsai's greatest technical strength and its greatest UI limitation for non-AEC users. Every panel label, dropdown option, and property name is the IFC term: `IfcWall`, `IfcRelDefinesByProperties`, `Pset_WallCommon`, `ObjectType`. For an architect who knows IFC, this is clarity. For a property manager or building operator who does not, it is a foreign vocabulary with no on-ramp.

---

## 11. Lessons for `app-workplace-bim` v0.0.1

The following recommendations are drawn directly from Bonsai's demonstrated conventions and their implications for the workplace surface `app-workplace-bim` is building.

**L1 — SpatialTree: expand to storey level by default, not to space level.**

Bonsai's Spatial Decomposition panel and Outliner both show the hierarchy. The natural initial expansion reveals IfcProject > IfcSite > IfcBuilding > IfcBuildingStorey. Expanding further to IfcSpace and then to individual elements overwhelms the tree immediately in any non-trivial model. AEC practice universally treats the storey as the primary navigation unit.

Recommendation: The `SpatialTree` component in `app-workplace-bim` should default to expanded-to-storey-level on initial load. Do not auto-expand to spaces or elements. User action (click, keyboard) expands further. This mirrors Bonsai convention without Bonsai's Outliner awkwardness.

**L2 — PropertiesPanel: show Psets in collapsible groups, not a flat list.**

Bonsai organizes Psets by set name in the Object Information panel — `Pset_WallCommon`, `Pset_ConcreteElementGeneral`, etc. are shown as expandable groups, each listing their properties inside. This is correct. A flat list of all properties from all Psets mixed together is unnavigable.

Recommendation: The `PropertiesPanel` component groups properties under their parent Pset/Qset name as collapsible sections. The section header shows the Pset name and a count of properties inside. Default state: all collapsed except the most common Pset for the element class (e.g., `Pset_SpaceCommon` expanded when a space is selected).

**L3 — Type assignment: expose the type name prominently in the properties header.**

In Bonsai, type assignment is in the Object Information panel but not surfaced prominently — users must scroll to find it. Type identity (e.g., "DOOR001", "WALL-EXTERIOR-200") is the most important classification for a building element in practice.

Recommendation: The `PropertiesPanel` in `app-workplace-bim` should show the assigned type name in the panel header or as the first editable field, above the Psets list. "Type: WALL-EXT-200" should be immediately visible on selection.

**L4 — Toolbar: use intent-based labels, not IFC class names.**

Bonsai's T-panel uses tool names like "Create Wall", "Create Slab", "Create Door" — readable and intent-based. This is correct. Avoid exposing "Create IfcWall" or "Assign IfcRelContainedInSpatialStructure" in any label the user sees.

Recommendation: All toolbar labels and panel labels in `app-workplace-bim` use plain English intent labels. IFC entity names are confined to the Properties panel's attribute display (where AEC professionals expect them) and to developer-facing debug views.

**L5 — Navigation: adopt the same viewport defaults as Bonsai/Blender.**

Middle-mouse orbit, scroll zoom, Shift+MMB pan is the de facto AEC navigation standard in 2026 — established by Blender, adopted by Bonsai, expected by users who have touched any modern 3D tool. Number pad shortcuts (Numpad 1/3/7 for front/right/top views) are muscle memory for this user base.

Recommendation: `app-workplace-bim`'s Viewport3D component should implement this same navigation model. Do not invent custom navigation.

**L6 — IFC save vs. local save: make the distinction explicit in the UI.**

Bonsai's split between `Save IFC Project` (Ctrl+S → writes `.ifc`) and `Save Blender file` (separate → writes `.blend`) is a persistent source of confusion. Users lose work by only saving the `.blend`. The distinction exists because of the host application; `app-workplace-bim` does not have this problem (there is no separate Blender state), but the lesson is to make the save target unambiguous.

Recommendation: The save action in `app-workplace-bim` should show the filename being written and confirm "IFC model saved" explicitly. Never show just a generic save icon without confirming what was saved.

**L7 — Spatial element creation: require active storey before placing elements.**

Bonsai automatically places new elements in the active storey. But when the active storey is not clearly communicated to the user, elements end up in unexpected containers. Bonsai surfaces the active storey only subtly.

Recommendation: `app-workplace-bim` should display the active storey in a persistent status indicator (e.g., breadcrumb "Ground Floor > Space 101") and prevent element creation if no storey is active (show an explicit prompt to select or create a storey first).

**L8 — Door/window void relationship: handle it transparently.**

Bonsai's `Shift+O` manual void-application shortcut exists because the automatic void creation requires the wall to be pre-selected. Users who miss the pre-selection step end up with doors that are not cut into their walls. This is a known workflow friction point.

Recommendation: `app-workplace-bim` should handle void relationships automatically at placement time, inferring the host wall from geometric intersection without requiring an explicit pre-selection or a manual repair shortcut. Surface a warning (not a crash) if no intersecting host wall is found.

**L9 — Pset CSV templating: support it but add a first-class UI editor.**

Bonsai's CSV-based Pset template system is flexible but requires file system access and CSV editing in an external tool. For the target persona (FM, property manager), editing a CSV file is not on the table.

Recommendation: `app-workplace-bim` should provide an in-app Pset template editor: list available Psets for an element class, allow adding/removing properties, set data types and default values, and save templates within the app. The CSV substrate can remain as the persistence format, but it should never be exposed to the user.

**L10 — BCF: make topic creation a first-class surface operation.**

Bonsai's BCF panel requires navigation through Quality and Coordination tab → BCF section → multiple clicks to create a topic and viewpoint. For coordination-heavy workflows, this needs to be a persistent panel or a toolbar shortcut.

Recommendation: BCF topic creation in `app-workplace-bim` should be accessible with a single keyboard shortcut or a floating action button. The most common action (flag this element with a comment) should be one key/click from any context.

---

## 12. Lessons for `app-console-bim`

`app-console-bim` is a read-only console surface. Bonsai is exclusively an authoring tool, but its design decisions reveal which panels reduce naturally to read-only inspection.

**C1 — PropertiesPanel collapses to an inspector.**

Remove all edit affordances from the PropertiesPanel in the console surface. The inspector shows the same grouped Pset/Qset structure as the authoring surface, but every field is read-only. No `+` buttons to add Psets, no inline editing. Selection triggers display only.

**C2 — SpatialTree is the primary navigation UI.**

Without authoring tools, the SpatialTree is the main interactive surface. The console user navigates the tree to find spaces, selects elements to inspect properties. The tree should be the dominant left-panel element.

**C3 — No Pset template management.**

The Pset template editor (L9 above) and the classification assignment dropdowns are authoring-only. The console surface has no path to creating or editing templates, and no dropdowns for reassigning types or classifications.

**C4 — No BCF posting — only BCF viewing.**

The console can display existing BCF topics and viewpoints (jump to viewpoint, read comments, see issue status). It cannot create new topics, add comments, or capture new viewpoints. BCF remains read-only on the console.

**C5 — Drawing views are the console's primary output.**

Bonsai's drawing generation is a complex authoring workflow (L8 noted it as poorly documented). On the console surface, the output of that workflow — rendered floor plans, sections, elevations — is the primary display. The console does not generate drawings; it consumes them. The console should render the IFC-embedded drawing views as static 2D panels, clickable for element inspection.

**C6 — IDS validation results are read-only reports.**

The console can display a pre-computed IDS validation result (imported as a JSON or BCF report), showing pass/fail per specification. The console does not run validation; it displays results previously generated by the authoring surface.

**C7 — Type and classification references are hyperlinks, not editable dropdowns.**

Where the authoring surface has a "Type: DOOR001" dropdown allowing reassignment, the console surface shows "Type: DOOR001" as a non-interactive label or a link to a type detail view. The same applies to classification codes — clickable to expand the classification reference, not editable.

---

## 13. Open Questions and Risks

**Q1 — Bonsai 0.8.6 / Blender 5.0 compatibility.** An open GitHub issue (#7623) tracks Python 3.13 support required by Blender 5.0. The alpha builds as of April 2026 target Python 3.11, 3.12, and 3.13, so this appears to be actively addressed, but the stable 0.8.5 release may not officially support Blender 5.0. If `app-workplace-bim` intends to call IfcOpenShell via subprocess, it should test against 0.8.6 alpha or confirm the Python version being bundled.

**Q2 — BCF 3.0 specifics.** The research confirms BCF 3.0 support in IfcOpenShell but the panel-level Bonsai documentation for BCF is marked work-in-progress. The exact BCF 3.0 schema extensions (e.g., `documentReferences`, `labels`, `stage`) and whether Bonsai's panel exposes them is unconfirmed. The Python `bcf` library in IfcOpenShell 0.8.5 likely handles 3.0, but verification against the Bonsai panel UI requires hands-on testing.

**Q3 — IDS panel exact UI layout.** The Quality and Coordination tab documentation is incomplete. The precise sequence for loading an `.ids` file, running validation, and navigating the result tree in Bonsai 0.8.5 is described in community sources but not in official docs. This should be verified against an actual Bonsai install before specifying the `app-workplace-bim` IDS result panel layout.

**Q4 — TypeBrowser: community add-on status.** The IFC Product Library add-on described in the OSArch community discussion is community-developed and not bundled. Its current install state (whether it ships for 0.8.5, what its API looks like) is unconfirmed. If `app-workplace-bim` wants a type browser backed by a manufacturer library, this will be custom work, not a Bonsai convention to mirror.

**Q5 — IfcZone creation workflow.** The workflow for creating IfcZone entities in Bonsai (zone ≠ space; zones aggregate spaces and may cross storeys) is not clearly documented in official sources. Zone management is likely scripting-only in the current release, which would represent a gap for space planning workflows in `app-workplace-bim`.

**Q6 — Linux install path for custom Pset CSVs.** The documented Windows path for custom Pset templates is `AppData\Roaming\Blender Foundation\Blender\4.2\extensions\...bonsai\bim\data\pset`. The Linux equivalent path is not confirmed in research sources. Before implementing a Pset template import feature in `app-workplace-bim` that reads from IfcOpenShell's Pset data directory, confirm the path resolution logic in IfcOpenShell's Python source.

**Q7 — Bonsai's section plane IFC representation.** The claim that section planes are stored as IfcAnnotation entities with plan context is inferred from how Bonsai persists drawing views, but the exact IfcRepresentationContext and IfcAnnotation subtype used is not confirmed from primary documentation. This matters for `app-workplace-bim` if it needs to parse and display Bonsai-authored section views from an IFC file.

**Risk R1 — Documentation lag.** A substantial fraction of Bonsai 0.8.5's official documentation is marked "Work in Progress" — the FM, Coordination, Drawings, and Object Information reference sections are all incomplete. Feature capability often runs ahead of documentation by several release cycles. The research above relies on a combination of official docs, community discussions, and inferred behavior. Hands-on testing of specific workflows (BCF creation, IDS validation, section generation) is required before treating the above as specifications.

**Risk R2 — Blender host as cognitive load.** Any decision to "mirror Bonsai conventions" must distinguish between conventions that originate in Bonsai's IFC expertise (good to mirror) and conventions that are artifacts of the Blender host (should be replaced with purpose-built alternatives). The SpatialTree-in-the-Outliner is an example of the latter — a Bonsai convention `app-workplace-bim` should deliberately replace, not mirror.

---

## 14. Sources

- Bonsai 0.8.5 documentation home: https://docs.bonsaibim.org/
- Bonsai official site: https://bonsaibim.org/
- IfcOpenShell / Bonsai documentation: https://docs.ifcopenshell.org/bonsai.html
- Blender Extensions listing: https://extensions.blender.org/add-ons/bonsai/
- GitHub releases (IfcOpenShell/IfcOpenShell): https://github.com/IfcOpenShell/IfcOpenShell/releases
- Starting a new IFC project: https://docs.bonsaibim.org/guides/authoring/starting_new_project.html
- Door workflow: https://docs.bonsaibim.org/guides/authoring/basic_modeling/door.html
- Window workflow: https://docs.bonsaibim.org/guides/authoring/basic_modeling/window.html
- Hotkeys reference: https://docs.bonsaibim.org/reference/general/hotkeys.html
- Scene Properties reference: https://docs.bonsaibim.org/reference/general/scene.html
- Properties panel reference: https://docs.bonsaibim.org/reference/general/properties.html
- Project Info reference: https://docs.bonsaibim.org/reference/project_overview/project_info.html
- Toolbar reference: https://docs.bonsaibim.org/reference/toolbar/index.html
- Exploring an IFC model (quickstart): https://docs.bonsaibim.org/quickstart/explore_model.html
- Creating an IFC model (quickstart): https://docs.bonsaibim.org/quickstart/create_model.html
- Introduction to BIM: https://docs.bonsaibim.org/quickstart/introduction_to_bim.html
- Drawings guide: https://docs.bonsaibim.org/guides/drawings/index.html
- Understanding IFC: https://docs.bonsaibim.org/guides/authoring/understanding_ifc/index.html
- BCF in IfcOpenShell: https://docs.ifcopenshell.org/bcf.html
- BlenderNation v0.8.0 announcement: https://www.blendernation.com/2024/09/01/bonsai-previously-blenderbim-add-on-v0-8-0-adds-blender-4-2-and-much-more/
- Engineering Skills — structural workflows article: https://www.engineeringskills.com/posts/bonsai-bim-the-essential-ifc-tool-for-structural-engineering-workflows
- OSArch community — BCF discussion: https://community.osarch.org/discussion/2806/bcf-in-bonsai-lots-of-options-but-how-does-it-work
- OSArch community — IFC product library addon: https://community.osarch.org/discussion/3417/ifc-product-library-addon-for-blender-bonsai-seeking-feedback
- OSArch community — type assignment: https://community.osarch.org/discussion/2672/how-to-asign-ifctype-names-to-model-elements-in-bonsai
- DTU Course 41934 — Bonsai fka BlenderBIM: https://timmcginley.github.io/41934/Concepts/Bonsai/index.html
- GitHub issue — Blender 5.0/Python 3.13 support: https://github.com/IfcOpenShell/IfcOpenShell/issues/7623
- Bonsai migration guide issue: https://github.com/IfcOpenShell/IfcOpenShell/issues/5422
- Bonsai MCP server reference: https://playbooks.com/mcp/jotaderodriguez-bonsai-blender-ifc
- ITcon 2025 — BIM-FM integration through OpenBIM: https://www.itcon.org/papers/2025_12-ITcon-Otranto.pdf
- OSArch standard BIM workflow discussion: https://community.osarch.org/discussion/3415/a-standard-bim-workflow-in-bonsai-arch-structure-mep
- Bonsai on OSArch wiki (BlenderBIM features guide): https://wiki.osarch.org/index.php?title=BlenderBIM_Add-on/BonsaiBIM_Features_Guide
