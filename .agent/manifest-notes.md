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

*Extracted 2026-05-09 by Master Command Session as part of A5
manifest trim — cluster manifest was 34 KB / 623 lines, over the
30 KB cap. This file holds the strategic prose; the active
operational manifest at `manifest.md` retains the YAML schema fields
+ tetrad declarations + scope sections.*
