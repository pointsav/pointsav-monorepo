---
schema: foundry-draft-v1
state: draft-pending-editorial-pass
originating_cluster: project-bim
target_repo: vendor/content-wiki-documentation
target_path: topic-aec-interface-conventions.md
audience: vendor-public
bcsc_class: vendor-public
language_protocol: PROSE-TOPIC
authored: 2026-05-17T00:00:00Z
authored_by: totebox@project-bim
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 2
open_questions_count: 0
research_provenance:
  - workspace-tier sub-agent A — `~/Foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md`
  - strategic source — `~/Foundry/BIM_Buildable Architecture.md`
  - manifest.md §planned_design_drafts — 10 universal interface components list
research_inline: true
references:
  - cites: [iso-16739-1-2024, ifc-43, revit-api-docs, ifc-openShell-docs]
notes_for_editor: |
  Audience is BIM operators who know at least one authoring tool —
  Revit, ArchiCAD, Bonsai, or BricsCAD — and are evaluating a new
  platform. The goal of this article is to establish that the Building
  Design System interface vocabulary is already familiar to them.
  Structural positioning only — name the tools factually when the
  interface vocabulary is directly comparable; no evaluative comparison.
  BCSC vendor-public class.
---

# AEC interface conventions

Every major BIM authoring platform — Revit, ArchiCAD, Bonsai, BricsCAD —
ships with four interface conventions that an architect or engineer learns
once and carries across products: a hierarchy tree for the spatial
structure, a properties panel for element attributes, a 3D viewport, and
a saved-view navigator. These conventions exist because the underlying
data model (the IFC entity hierarchy) is the same regardless of which
tool authors it. The Building Design System's universal interface
components are built on this shared vocabulary.

## The four universal conventions

### Spatial tree

Every BIM authoring tool displays the spatial structure of a building
as a hierarchical tree: Site contains Building, Building contains
Storey, Storey contains Space, Space contains Elements. This
corresponds directly to the `IfcSpatialStructureElement` hierarchy in
IFC 4.3. A Revit user recognises the Project Browser; an ArchiCAD user
recognises the Navigator; a Bonsai user recognises the Outliner. The
labels differ; the structure does not.

The Building Design System `SpatialTree` component renders this
hierarchy with consistent expand/collapse behaviour, selection
propagation to the Viewport3D, and IFC GUID display on hover. An
operator who has used any of the major authoring tools can navigate
a `SpatialTree` without reading documentation.

### Properties panel

When an element is selected, a properties panel shows the element's
IFC class name, its globally unique identifier (IFC GUID), and all
attached Property Set values. In Revit this is the Properties palette;
in ArchiCAD it is the Element Settings dialog; in Bonsai and the
web-ifc viewer it is typically called the Properties panel.

The Building Design System `PropertiesPanel` component renders the
same data with a mode-prop variant: the `view` mode shows all Pset
values flat; the `edit` mode shows only the values the current role
is authorised to modify. A BIM operator finds their familiar Pset
vocabulary — `Pset_WallCommon.FireRating`, `Pset_SpaceOccupancy.
OccupancyNumber`, `Pset_DoorCommon.IsFireExit` — in the same
position as in their authoring tool, without an interface-mapping
learning period.

### 3D viewport

The principal interface surface of every BIM tool is a perspective
or orthographic 3D viewport. Camera controls (orbit, pan, zoom) use
industry-standard mouse bindings: middle-button orbit, scroll zoom,
shift-scroll pan. Section cuts are applied as clipping planes.
Isolated element visibility is toggled by element class or storey.

The Building Design System `Viewport3D` component embeds the
xeokit-sdk or @thatopen/web-ifc viewer — both open-source, MIT or
AGPL licensed — with these standard camera controls. An IFC file
loaded into the viewport renders correctly because both viewers
implement the IFC 4.3 geometry pipeline natively, without round-
tripping through a proprietary format.

### View navigator

Named saved views — floor-plan views, section cuts, elevation views,
3D perspectives framed on a specific storey — are how BIM operators
communicate intent without sending full model files. In Revit, saved
views are Project Browser items. In ArchiCAD they are Viewpoints. In
Bonsai they are Camera presets.

The Building Design System `ViewNavigator` component renders saved
views as labelled tabs: `L1 Floor Plan`, `Section A-A`, `North
Elevation`, `Mechanical Room 3D`. Selecting a tab loads the camera
state and, optionally, the IFC storey-filter for that view. A
collaborator on a different machine opens the same view without any
out-of-band coordination.

## Ten universal interface components

The Building Design System defines ten interface components that appear
on every surface — whether the app-workplace-bim field client, the
app-console-bim facility management console, or any future surface
that consumes the Building Design System.

| Component | Role |
|---|---|
| `SpatialTree` | Spatial hierarchy navigation (Site → Building → Storey → Space) |
| `PropertiesPanel` | IFC Pset viewer and editor (mode-prop variant) |
| `Viewport3D` | 3D model viewport (xeokit / @thatopen embed) |
| `ViewNavigator` | Named saved views as labelled tabs |
| `IssueTracker` | BCF 3.0 topic list with status and assignee filters |
| `ElementSearch` | IFC GUID or Pset-value search across the loaded model |
| `ClashReview` | Clash detection result list with viewport highlight |
| `HistoryTimeline` | Git commit history rendered as model-state timeline |
| `ExportPanel` | COBie export, IDS validation run, BCF ZIP download |
| `StatusBar` | Model load progress, validation counts, last-sync timestamp |

The four surface-unique components — `GuidSearch` and `AuditLog` for
the facility management console, and two app-workplace-bim components —
extend the universal set without replacing it. An operator who learns
the ten universal components has a working mental model of every
PointSav BIM surface before opening it.

## Why shared vocabulary matters

BIM project teams frequently work across multiple authoring tools in
a single project. The structural engineer's Tekla model, the architect's
Revit model, and the MEP engineer's MagiCAD model all export IFC-SPF.
Coordination happens in a common viewer — historically Solibri, Navisworks,
or the web-ifc viewer — where no one is in their native authoring
environment.

A shared interface vocabulary means that the coordination viewer does
not introduce a new learning surface on top of the authoring tools. An
architect opening a building model in the Building Design System viewer
finds the same tree, the same properties panel, and the same viewport
controls they use in ArchiCAD. The tool is invisible; the model is
visible. That is the goal of AEC interface conventions.

## Relationship to Carbon and the design system substrate

The Building Design System is a BIM-semantic extension of the Carbon
Design System baseline. Carbon provides the foundational UI primitives —
buttons, inputs, data tables, colour tokens, typography scale. The
Building Design System adds the AEC-semantic layer on top: the ten
universal interface components and the eight BIM token primitive
categories.

A designer who contributes to the Carbon-based design.pointsav.com
surface uses the same token and component authoring workflow to
contribute a new BIM component to bim.woodfinegroup.com. The substrate
is the same; the semantic domain is different. This is the claim #38
extension pattern: the Carbon-baseline-floor reaches into the AEC domain
by adding domain-specific components without replacing the foundational
layer.

## Provenance

Authored by totebox@project-bim, 2026-05-17. Sources: IFC 4.3 /
ISO 16739-1:2024; workspace sub-agent A research (2026-04-28);
manifest.md §planned_design_drafts; strategic source `BIM_Buildable
Architecture.md`.

Refinement target: `vendor/content-wiki-documentation/topic-aec-interface-conventions.md`
plus Spanish adaptation at `topic-aec-interface-conventions.es.md`.

## Research trail

### Done

- Mapped ten universal interface components against Revit, ArchiCAD,
  Bonsai, and BricsCAD equivalent panels/dialogs. Sub-agent A.
- Confirmed xeokit-sdk and @thatopen/web-ifc as the two viable open-
  source IFC 4.3 web viewers with appropriate licensing. Sub-agent A.
- Validated IFC GUID stability across model revisions (per IFC schema
  spec, GUIDs are immutable once assigned). Standard documentation.

### Suggested

- Verify xeokit-sdk current licence (historically MIT; confirm no
  relicensing event post-2025).
- Confirm @thatopen/web-ifc IFC 4.3 entity support level at publication
  time — the library is actively developed and entity coverage may
  have expanded since research date.
