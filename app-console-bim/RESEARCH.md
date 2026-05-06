# app-orchestration-BIM & app-console-BIM — Research Context

**Scope:** Research context for the OrchestrationOS aggregation layer and the ConsoleOS coordination terminal.
**Full research:** `content-wiki-documentation/research/RESEARCH-BIM-MARKET.md`
**Audience:** Developers joining these components

---

## The Orchestration Layer — Why It Exists

`app-orchestration-bim` is stateless. It holds no data. It connects multiple `cluster-totebox-property` instances and provides compute that a base ToteboxOS node cannot. This is the same architectural principle as `app-orchestration-command` (CommandCentre): the orchestration layer aggregates; the archive layers store.

A property manager with one building needs only `app-console-bim` connecting directly to `cluster-totebox-property`. The orchestration layer becomes necessary when:

- Multiple buildings must be queried simultaneously ("across all properties, which HVAC units have exceeded maintenance intervals?")
- Heavy compute is required — IFC clash detection, portfolio-level CityJSONSeq aggregation, BIM model conversion at scale
- Multiple `app-console-bim` clients need to work on the same set of archives simultaneously
- BIM data from `cluster-totebox-property` needs to join with corporate data from `cluster-totebox-corporate` (for lease-to-element linkage)

The commercial boundary follows the existing platform pattern: `service-bim` (archive daemon) is Apache 2.0. `app-orchestration-bim` is proprietary — paid.

---

## The Console Layer — What It Does Not Do

`app-console-bim` is a **routing and coordination terminal**, not an editor. This is the same distinction that governs every `app-console-*` component:

- `app-console-bim` → queries elements, links work orders, creates BCF issues, authorizes F12 commits. **Muscle memory: Navisworks.**
- `app-workplace-bim` → authors and edits BIM geometry and data. **Muscle memory: Revit / AutoCAD.**

Operators who need to view a floor plan, query which HVAC element serves a specific space, attach a maintenance work order to a building element, or review BCF issues from an architect — that is `app-console-bim`. Operators who need to edit geometry or author new BIM content — that is `app-workplace-bim`.

---

## The Competitive Context for Coordination Tools

The market segment `app-console-bim` occupies is currently served by tools that fail in two opposite directions:

**Too complex:** Navisworks, Solibri (Solibri Anywhere deprecated April 2026), BIMcollab Zoom — these require BIM expertise, cloud subscriptions, and carry no operational linkage (work orders, leases, sensor data). They are coordination tools for design-phase clash detection, not operations-phase FM coordination.

**Too lightweight:** Simple IFC viewers (BIMvision, Open IFC Viewer, BIMx) are read-only. No issue creation. No work order linkage. No F12 commit. No ledger integration.

`app-console-bim` occupies the gap: IFC-aware, operationally linked (work orders, leases, IoT sensor overlays), F12-authorized commits to the immutable ledger, offline-capable, no per-seat cloud subscription.

---

## The Federated Model Architecture

For multi-building views, `app-orchestration-bim` assembles a federated scene from multiple `cluster-totebox-property` glTF caches. The reference implementation for federated IFC viewing in the open-source ecosystem is **xeokit-sdk** (AGPL-3.0), which:

- Renders multiple IFC models in a single WebGL scene with double-precision coordinates
- Compresses IFC to XKT format (approximately 25× size reduction vs raw IFC)
- Supports section planes, measurement, picking, and tree-view navigation
- Is used in production by Autodesk Construction Cloud's viewer layer

EUPL-1.2 is compatible with AGPL-3.0 under the EUPL appendix (confirmed by the European Commission's Joinup Licensing Assistant), so xeokit can be embedded in a Tauri webview in a PointSav product.

The alternative is `@thatopen/components` (MIT licence, ThatOpen Company) — lighter dependency burden, EUPL-compatible without the AGPL consideration, but less performance-optimised for large federated scenes than xeokit.

---

## CityJSONSeq for Portfolio Queries

For portfolio-level spatial queries across multiple properties, `app-orchestration-bim` uses **CityJSONSeq** (OGC Community Standard) — one JSON object per building, one building per line. TU Delft's 3DBAG dataset serves the entire Netherlands building stock (~10 million buildings) in this format. For a Woodfine portfolio of 10–100 properties, CityJSONSeq provides:

- Portfolio map rendering (building footprints, heights, attributes)
- Cross-property spatial queries (all properties within 2km of a specific address)
- Portfolio-level energy and compliance aggregations
- GIS integration (the format is OGC-compatible with standard GIS toolchains)

---

## Key References

- Full market research: `content-wiki-documentation/research/RESEARCH-BIM-MARKET.md`
- xeokit-sdk: [github.com/xeokit/xeokit-sdk](https://github.com/xeokit/xeokit-sdk)
- @thatopen/components: [github.com/ThatOpen](https://github.com/ThatOpen)
- CityJSONSeq / 3DBAG: [3dbag.nl](https://3dbag.nl)
- Speckle federated architecture: [github.com/specklesystems](https://github.com/specklesystems)
- ADRs: SYS-ADR-07, SYS-ADR-10, SYS-ADR-19
