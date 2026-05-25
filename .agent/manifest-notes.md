# project-bim cluster manifest — strategic context notes

Sibling file to `manifest.md`. Holds historical strategic prose
extracted from manifest.md to keep that file under the 30 KB
cluster-manifest cap per `conventions/agent-file-size-discipline.md`.

This file is **not parsed by tooling** — it's narrative reference
context for Task / Master sessions opening this cluster.

---

## Cluster mission

Per `BIM_Buildable Architecture.md` (96-line workspace-root strategic
source, April 2026): ship a leapfrog-2030 flat-file open-BIM
substrate that the AEC hyperscalers (Autodesk, Bentley, Trimble)
cannot follow by design.

**Five hyperscaler assumptions are simultaneously their revenue
mechanism and their structural vulnerability:**

1. Authoritative database lives in vendor cloud
2. Subscription check for own data
3. Lossy export/import (proprietary interchange)
4. AI in vendor tenancy
5. Per-seat economics (+ version lock as bonus moat)

Foundry's existing architectural commitments — flat-file storage,
open standards, Rust + Tauri, offline-first, EUPL-licensed,
seL4-hardened — are not stylistic preferences; they are the precise
constraints that make the five hyperscaler weaknesses into
customer-visible differentiators.

## Building Design System pattern

The cluster ships a pattern parallel to project-design (Doctrine
claim #38 Design System Substrate): a "Building Design System" that
extends the existing DTCG vault with BIM-semantic tokens (8 primitive
categories anchored to IFC 4.3 entity hierarchy) and AEC interface
conventions (10 universal components: SpatialTree, PropertiesPanel,
Viewport3D, ViewNavigator, Toolbar, StatusBar, SelectionFilter,
TypeBrowser, SectionPlane, AnnotationLayer).

## Leapfrog invention — claim #41

"City Code as Composable Geometry" — cities as design-system
publishers; building codes published as composable BIM design tokens
(bSDD + IDS + IFC geometric exclusion-zone fragments); designer
assembles inside pre-constrained envelopes rather than submitting
designs for retrospective compliance checking. No prior art addresses
this compositional-first framing (Sonnet sub-agent B research, full
report at `.claude/sub-agent-results/B-bim-city-code-as-geometry-2026-04-28.md`).

---

## Vendor focus blocks (tetrad → vendor leg)

Extracted 2026-05-12 from `tetrad.vendor[0].focus` in manifest.md.

### 6 new projects — per-project detail

**service-materials** (NEW; Ring 2 material substrate) — flat files
unless real DB needed; IfcMaterial + bSDD URI references for material
classes; Pset_Material* property sets; Uniclass Pm classification.

**service-buildings** (NEW; Ring 2 building element substrate) — IFC
GUID-keyed element store; per-element YAML sidecars keyed on IFC
GUIDs; serves IfcBuiltElement family across walls/slabs/columns/beams/
doors/windows/roofs/stairs; spatial hierarchy via IfcSpatialElement;
Speckle-inspired hash-addressed object store at objects/<hash>.json;
UI mode-prop pattern.

**service-codes** (NEW; Ring 2 regulatory substrate) — the
City-Code-as-Composable-Geometry invention; manages bSDD URI
references to per-jurisdiction zoning dictionaries; authors/consumes
IDS 1.0 constraint files via ifctester; federates IFC geometric
exclusion-zone fragments (woodfine-rs1-constraints.ifc); composes
per-jurisdiction overlays (municipal + provincial + federal +
accessibility).

**app-orchestration-bim** (NEW; Yew/Leptos + Axum frontend) —
"browser interface for all parties involved" — architects, engineers,
construction managers, property managers; the Building Design System
showcase parallel to design.pointsav.com pattern; aggregates Totebox
property archives; renders IFC + bSDD + IDS + Building Design System
tokens for AEC consumption.

**app-workplace-bim** (NEW; Tauri 2.10 + Rust full-feature BIM
editor) — Pattern 1 architecture per BIM_Buildable Architecture.md —
thin Rust shell + xeokit/@thatopen webview + IfcOpenShell sidecar via
subprocess; "muscle memory" universal AEC interface — SpatialTree,
PropertiesPanel, Viewport3D, etc.; IFC-SPF authoritative;
Speckle-inspired flat-file object store from day one.

**app-console-bim** (NEW; READ surface) — property managers, FM
operators, AEC collaborators querying without editing; BimGuidSearch +
BimAuditLog + BimDashboard + BimExportPanel unique components;
mode-prop variants of shared components.

Pre-existing speculative registry rows for app-{console,orchestration,
workplace}-bim + service-bim are replaced by this scope. service-bim
is RETIRED — split into service-materials + service-buildings +
service-codes per architectural decomposition above. Codifies Doctrine
claim #37 (Tetrad) + proposes claim #40 (Flat-File BIM Substrate) +
claim #41 (City Code as Composable Geometry).

---

## Clone focus blocks (clones section)

Extracted 2026-05-12 from the verbose `focus:` values in the `clones:`
section of manifest.md.

### pointsav-design-system clone focus

The Building Design System EXTENSION OF the existing DTCG vault. 8
BIM token primitive categories (SPATIAL / ELEMENTS / SYSTEMS /
MATERIALS / ASSEMBLIES / PERFORMANCE / IDENTITY+CODES / RELATIONSHIPS)
anchored to IFC 4.3 entity hierarchy. 10 universal interface
components + 4 console-unique + 4 workplace-unique. Coordinates with
project-design (the META-substrate owner) — see cross-cluster handoff
entry in manifest.md. Uniclass 2015 imported as classification floor
(analogous to project-design's Carbon-baseline-floor pattern).

### woodfine-fleet-deployment clone focus

Two catalog subfolders: cluster-totebox-property/ (existing; Task
extends with GUIDE-bim-archive-operations.md + GUIDE-vault-export.md +
GUIDE-bim-code-encoding.md) + gateway-orchestration-bim/ (NEW; Task
creates catalog folder + GUIDE-deploy-bim-substrate.md +
GUIDE-bim-orchestration-operations.md). Customer-tier operational
mirror for both deployment instances.

---

*Sections above extracted 2026-05-12 by Totebox Session (project-bim)
as part of second manifest trim pass — manifest was 32.9 KB / 593
lines; target under 30 KB. Full operational manifest at `manifest.md`.*
