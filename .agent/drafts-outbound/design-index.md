---
schema: foundry-draft-v1
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/pointsav-design-system
target_path: (multiple — see body)
audience: design-system
bcsc_class: vendor-internal
language_protocol: DESIGN-RESEARCH
authored: 2026-04-28T22:35:00Z
authored_by: task-cluster-project-bim
authored_with: claude-opus-4-7
research_done_count: 4
research_suggested_count: 0
open_questions_count: 1
research_provenance:
  - workspace-tier sub-agent A — `~/Foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md`
  - cluster sub-agent BB.2 — `~/Foundry/clones/project-bim/.claude/sub-agent-results/BB.2-xeokit-vs-thatopen-2026-04-28.md`
  - cluster sub-agent BB.4 — `~/Foundry/clones/project-bim/.claude/sub-agent-results/BB.4-bonsai-interface-deepdive-2026-04-28.md`
  - strategic source — `~/Foundry/BIM_Buildable Architecture.md`
research_inline: true
notes_for_editor: |
  This is a DESIGN-INDEX rather than per-component DESIGN-COMPONENT
  drafts because cluster/project-bim already landed the BIM extension
  directly in its own clone of pointsav-design-system on
  cluster/project-bim branch (per cluster manifest's "BIM-semantic
  sub-substrate owner" framing). project-design Task reviews the
  landed artefacts on cluster/project-bim, accepts as-is or proposes
  refinements via cross-cluster outbox response. When cluster/project-bim
  promotes to canonical via Stage 6, the canonical pointsav-design-system
  receives the BIM extension; project-design's own cluster branch can
  rebase or merge cleanly because all paths are namespaced under
  `tokens/bim/`, `components/bim-*/`, `research/bim-*.md`.

  Per-component DESIGN-COMPONENT drafts queue for v0.0.2 onwards as
  the surface deepens.
---

# DESIGN-INDEX — BIM extension to the design-system substrate

## Summary

Cluster `cluster/project-bim` v0.0.1 landed the Building Design System
extension directly in its sub-clone of `pointsav-design-system` on
the `cluster/project-bim` branch. This index handoff to project-design
Task names what landed and where, asking for review.

## What landed

### Tokens

| File | Purpose |
|---|---|
| `tokens/bim/spatial.dtcg.json` | SPATIAL primitive (IfcSite / IfcBuilding / IfcBuildingStorey / IfcSpace + IfcFacility for IFC 4.3 infrastructure) |
| `tokens/bim/elements.dtcg.json` | ELEMENTS primitive (IfcWall, IfcSlab, IfcColumn, IfcBeam, IfcDoor, IfcWindow, IfcRoof, IfcStair, IfcRailing, IfcCovering) |
| `tokens/bim/systems.dtcg.json` | SYSTEMS primitive (IfcDistributionElement family — pipes, ducts, cables, terminals) |
| `tokens/bim/materials.dtcg.json` | MATERIALS primitive (IfcMaterial + LayerSet + ConstituentSet + Pset_Material* templates) |
| `tokens/bim/assemblies.dtcg.json` | ASSEMBLIES primitive (IfcElementAssembly + IfcGeographicElement + IfcFurnishingElement) |
| `tokens/bim/performance.dtcg.json` | PERFORMANCE primitive (Pset_*Common templates + Qto_*BaseQuantities) |
| `tokens/bim/identity-codes.dtcg.json` | IDENTITY+CODES primitive (IfcRoot.GlobalId + IfcClassificationReference + IfcConstraint + Uniclass / OmniClass / MasterFormat metadata) |
| `tokens/bim/relationships.dtcg.json` | RELATIONSHIPS primitive (IfcRelContainedInSpatialStructure, IfcRelAggregates, IfcRelFills/VoidsElement, IfcRelAssociates*, IfcRelDefinesByProperties, IfcRelConnectsPathElements) |
| `tokens/uniclass-2015.dtcg.json` | Classification floor (NBS UK Uniclass 2015; eleven primary tables; April 2026 update) |

### Components

| Folder | Purpose |
|---|---|
| `components/bim-spatial-tree/` | recipe.html + recipe.css + aria.md. Universal AEC SpatialTree. Storey-level default expansion (BB.4). Purpose-built widget, not Outliner-as-tree. |
| `components/bim-properties-panel/` | recipe.html + recipe.css + aria.md. Universal AEC PropertiesPanel. Mode-prop pattern; Pset / Qto grouping per Bonsai convention. |
| `components/bim-viewport-3d/` | recipe.html + recipe.css + aria.md. Universal AEC Viewport3D. xeokit embed surface (workplace AGPL-3.0; console may degrade to non-3D for EUPL-1.2). |

### Research

| File | Purpose |
|---|---|
| `research/bim-design-philosophy.md` | The architectural stance — why the constraints are the differentiators |
| `research/bim-token-taxonomy.md` | Why eight primitives anchored to IFC 4.3; 18 components; Uniclass 2015 floor; component-recipe pattern |
| `research/bim-aec-muscle-memory.md` | What every AEC tool does the same; what to mirror from Bonsai; what to deliberately NOT inherit |

## Architecture decisions encoded

### Viewer (BB.2)

- **xeokit-sdk over @thatopen.** Deciding factor: double-precision
  rendering for georeferenced IFC. xeokit RTC is first-class WebGL
  feature; @thatopen via Three.js cannot match.
- **License correction (material — flag for Master ratification):**
  combined xeokit + EUPL-1.2 work distributes as **AGPL-3.0**, not
  EUPL-1.2. project-bim's `app-workplace-bim` is therefore AGPL-3.0;
  `app-orchestration-bim` (no xeokit coupling) and the services
  remain EUPL-1.2. This is a workspace-tier licensing fact worth
  surfacing to BCSC posture.

### Tauri 2.10 IPC (BB.3)

- Never pipe IFC bytes over IPC. Use `convertFileSrc()` + Tauri
  `asset:` protocol. `Channel<T>` streaming for large query results.
- IfcConvert sidecar binary downloaded on first run, not bundled.
  SHA-256 verify before invoke. Mobile builds skip the sidecar
  (viewer-only).

### IfcOpenShell sidecar (BB.1)

- LGPL-3.0 compliance via dynamic CLI invocation only.
- `tokio::process::Command::wait_with_output()` to avoid stderr
  deadlock.
- `ifctester` exits with code 0 regardless of validation outcome —
  parse JSON output. Fall back to Console reporter on issue #4526.

### AEC interface (BB.4)

- SpatialTree default-expand to storey level. No auto-expand to
  spaces.
- Build a purpose-built SpatialTree widget. Do not reuse a
  general-purpose scene-graph viewer.

## Coordination ask

project-design Task: please review the landed artefacts on
`cluster/project-bim` branch of `pointsav-design-system`. Either:

1. Accept as-is — no changes required, wait for Stage-6 promotion to
   land the extension on canonical.
2. Propose refinements — respond via cross-cluster outbox naming the
   files + the changes. project-bim Task lands the refinements before
   promotion.

The 18 components named in the manifest's `planned_design_drafts`
list are not all present yet. Three universal components landed at
v0.0.1 (`bim-spatial-tree`, `bim-properties-panel`,
`bim-viewport-3d`). The remaining 15 (7 universal + 4 console-unique
+ 4 workplace-unique) plus the leapfrog component
`bim-code-rs1` (Doctrine claim #41) queue for v0.0.2.

## Per-component DESIGN drafts (v0.0.2 onwards)

The cluster manifest's `planned_design_drafts` list anchors the v0.0.2
queue:

- `DESIGN-RESEARCH-bim-token-taxonomy.draft.md` — already covered by
  `pointsav-design-system/research/bim-token-taxonomy.md`; no DESIGN
  draft needed
- `DESIGN-COMPONENT-bim-spatial-tree.draft.md` — landed at v0.0.1
- `DESIGN-COMPONENT-bim-properties-panel.draft.md` — landed at v0.0.1
- `DESIGN-COMPONENT-bim-viewport-3d.draft.md` — landed at v0.0.1
- `DESIGN-COMPONENT-bim-view-navigator.draft.md` — v0.0.2
- `DESIGN-COMPONENT-bim-guid-search.draft.md` — v0.0.2 (console-unique)
- `DESIGN-COMPONENT-bim-audit-log.draft.md` — v0.0.2 (console-unique)
- `DESIGN-COMPONENT-bim-code-rs1.draft.md` — v0.0.2 (the leapfrog
  invention; Doctrine claim #41 candidate)

## Research trail

### Done

- Token taxonomy validated against IFC 4.3 entity hierarchy (sub-agent A)
- xeokit vs @thatopen decision validated against double-precision rendering requirement (sub-agent BB.2)
- Tauri 2.10 IPC + sidecar architecture validated against BIM-scale model loading (sub-agent BB.3)
- Bonsai interface conventions cross-walked vs Revit / ArchiCAD / BricsCAD (sub-agent A + BB.4)

### Open questions

- Whether project-design wants the BIM extension co-resident under
  `tokens/bim/`, `components/bim-*/`, `research/bim-*.md` (current
  default chosen by project-bim Task) or isolated under a top-level
  `bim/` subdirectory of `pointsav-design-system`. project-design
  Task chooses; project-bim Task aligns.
