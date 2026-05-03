# BIM Product Family — Rules and Context

This file is loaded by Claude Code when working on any `app-*-bim` or `service-bim` crate.
Read it alongside the root `CLAUDE.md`. Root rules take precedence on conflict.

---

## Product family map

Four components. Three distinct roles. One data contract.

| Crate | Layer | Host OS | Primary job |
|---|---|---|---|
| `service-bim` | Archive daemon | `os-totebox` | Maintain flat-file BIM archive. No UI. No network. |
| `app-console-bim` | Coordination terminal | `os-console` | Query, coordinate, link work orders, F12 commit. Navisworks muscle memory. |
| `app-workplace-bim` | Editor | `os-workplace` | Author and edit BIM geometry and data. AutoCAD / Revit muscle memory. |
| `app-orchestration-bim` | Aggregation hub | `os-orchestration` | Multi-archive queries, heavy compute, glTF streaming. Proprietary. |

**The distinction that matters most:** `app-console-bim` retrieves and coordinates. `app-workplace-bim` creates and edits. These are different products for different workflows. Never conflate them in naming, scope, or UX decisions.

---

## The canonical data contract

Every component reads from and writes to the same flat-file structure inside `cluster-totebox-property`. The layout is the shared contract — violating it breaks every other component.

```
cluster-totebox-property/
└── service-bim/
    ├── canonical/
    │   ├── model.ifc              ← IFC-SPF, ISO 16739-1:2024. THE system of record.
    │   ├── model.ifc.sha256       ← Cryptographic seal
    │   └── model.ifc.meta.yaml   ← Version, source, ingestion timestamp
    ├── elements/
    │   └── {IFC-GUID}/
    │       ├── element.yaml       ← Type, name, classification, bSDD URI
    │       ├── work-orders.yaml   ← Maintenance history
    │       ├── sensors.yaml       ← IoT sensor assignments (Brick Schema URIs)
    │       └── documents/         ← Warranties, spec sheets, photos
    ├── drawings/
    │   ├── floor-plans/           ← SVG, one per level, IFC GUIDs as element IDs
    │   └── cache/
    │       └── model.glb          ← glTF binary. Regenerated on demand. NOT canonical.
    ├── issues/
    │   └── issue-{N}/
    │       ├── markup.bcf         ← BCF 3.0 XML
    │       └── viewpoint-{N}.png  ← Camera snapshot
    ├── requirements/
    │   └── handover.ids           ← IDS 1.0 validation spec
    └── ingestion/
        ├── queue/                 ← Incoming files. service-bim watches this.
        └── log/                   ← Conversion reports, error records
```

**Critical rules for all components:**
- `canonical/model.ifc` is read-only for everything except `service-bim`
- `app-workplace-bim` writes to `ingestion/queue/` only — never to `canonical/`
- `service-bim` is the only process that writes to `canonical/`
- `cache/model.glb` is a derived output. It is regenerated. It is never the source of truth.

---

## File formats — the decisions that are made

These are not up for reconsideration per session. They are documented decisions.

| Format | Role | Canonical? | Why |
|---|---|---|---|
| IFC-SPF (`.ifc`) | Building geometry and semantics | **Yes** | ISO 16739-1:2024. Plain ASCII. 50-year readable. Universal tool support. |
| YAML sidecars | PointSav operational metadata | **Yes** | Keyed by IFC GUID. Human-readable. Git-diffable. |
| BCF 3.0 (`.bcf` decomposed) | Issue markup | **Yes** | buildingSMART standard. XML. Git-native per-topic directories. |
| IDS 1.0 (`.ids`) | Handover validation spec | **Yes** | buildingSMART standard June 2024. XML. Machine-validated at ingestion. |
| SVG floor plans | 2D drawings | Derived | Generated from IFC by IfcOpenShell. IFC GUIDs as SVG element IDs. |
| glTF 2.0 (`.glb`) | 3D viewer cache | **No** | ISO 12113:2022. Geometry only — no BIM semantics survive export. Regenerated on demand. |
| RVT (Revit) | Ingestion source only | **No** | Closed binary. Autodesk-proprietary. Never stored in archive. |
| DWG | Ingestion source only | **No** | ODA-compatible. Never stored. DXF is the open coordination export. |

Do not introduce new file formats without a documented ADR. The format list is closed.

---

## IFC operations — the routing rules

SYS-ADR-07 applies without exception. The question to ask for every IFC operation:

- **Is this operation on structured geometric or property data?** → IfcOpenShell subprocess only. Never through `service-slm` or any AI layer.
- **Is this operation on unstructured human text** (operator notes, document descriptions, issue comments)? → `service-slm` may assist. It may suggest. It does not commit.

The ingestion pipeline routing:

```
Incoming file arrives in ingestion/queue/
  ↓
service-bim detects via filesystem watch
  ↓
If RVT: invoke IfcOpenShell IfcConvert subprocess → .ifc
If DWG/DXF: invoke IfcOpenShell subprocess → .ifc
If .ifc: validate directly
  ↓
Run ifctester against requirements/handover.ids
  ↓
If pass: write to canonical/, compute SHA-256, update objects/ store
If fail: write to ingestion/log/ with failure report. Do not commit.
  ↓
Generate SVG floor plans (IfcOpenShell SVG slicer)
Regenerate cache/model.glb (IfcConvert → glTF)
```

---

## The F12 rule for BIM

SYS-ADR-10 applies to BIM archive commits. No file enters `canonical/` without a human operator explicitly committing it.

In `app-workplace-bim`: the commit action that moves a working draft into `ingestion/queue/` is the F12 equivalent. It is an explicit operator action. It is not automatic. It is not triggered by the AI.

In `app-console-bim`: linking an IFC element to a work order, a lease record, or a sensor assignment requires explicit operator selection of the IFC GUID and the target record. The AI may suggest a link. It does not execute it.

---

## Licence constraints — check before adding a dependency

The BIM product family has licence constraints that differ by component.

| Component | Licence | Constraint |
|---|---|---|
| `service-bim` | Apache 2.0 | No GPL dependencies. LGPL via subprocess only. |
| `app-console-bim` | Apache 2.0 | No GPL dependencies. LGPL via subprocess only. |
| `app-workplace-bim` | EUPL-1.2 | LGPL via subprocess clean. MPL-2.0 (web-ifc) clean. AGPL-3.0 (xeokit) compatible per EC Joinup. GPL via subprocess only — do not link. |
| `app-orchestration-bim` | Proprietary | No open-source licence constraints on the commercial layer. |

**IfcOpenShell (LGPL-3.0):** invoked via Python subprocess in all components. This is clean LGPL compliance — dynamic invocation, not static linking. Do not link IfcOpenShell into any Rust binary directly.

**Bonsai (GPL-3.0):** reference implementation only. Do not use as a library. Do not import. Study the architecture; do not copy the code.

**web-ifc 0.77 (MPL-2.0):** used as a webview JS module in `app-workplace-bim` and `app-console-bim`. MPL source-file copyleft only — clean for EUPL-1.2 host applications.

---

## Muscle memory targets — phase by phase

**`app-workplace-bim` (the editor):**
- Phase 1: AutoCAD — command-line driven, left hand on keyboard. Core alias set: `L`, `PL`, `C`, `M`, `CO`, `TR`, `O`, `F`, `LA`. F3/F8/F10 toggles. Layer panel maps to IFC element categories, not arbitrary layer names.
- Phase 2: Navisworks — 3D orbit/pan/zoom/select. Properties panel on selection. BCF issue creation linked to IFC element GUIDs. Section planes. Measurement.
- Phase 3: Revit — IFC property editing, element reclassification, schedule views, simple element creation. Full parametric constraint solving is post-Phase 3.

**`app-console-bim` (the coordination terminal):**
- Single phase: Navisworks coordination and FM operations. No authoring. No geometry creation. Retrieval, linking, issue management, F12 commit.

Do not implement Phase 2 features in Phase 1. Do not implement Phase 3 features in Phase 2. The phase boundaries are scope boundaries, not preferences.

---

## Open questions — do not resolve without confirmation

These items are not yet settled. Do not make assumptions. Surface them when relevant.

- `moonshot-bim-parser` — a native Rust IFC parser is the eventual direction. `ifc_rs` (0.1.0-alpha) and `ifc-lite-core` are candidates. Neither is production-ready as of April 2026. Do not replace IfcOpenShell subprocess with either until explicitly commissioned.
- ODA BimRv integration (Phase 2 RVT ingestion) — not yet licensed. Phase 1 uses IFC export from authoring tools. Do not implement ODA integration until the licence is in place.
- Verification Surveyor daily throttle — applies to `service-people`, not `service-bim`. Noted here for completeness. Do not cite a specific number.

---

## Research references

For deep context on market, standards, and architectural decisions:

- Full market research: `content-wiki-documentation/research/RESEARCH-BIM-MARKET.md`
- Component research: `RESEARCH.md` in each BIM app directory
- IFC specification: ISO 16739-1:2024
- buildingSMART standards: IDS 1.0, BCF 3.0, bSDD
- IfcOpenShell 0.8.5 docs: https://docs.ifcopenshell.org
