# app-workplace-bim

WorkplaceOS BIM editor. AutoCAD / Revit muscle memory. This is the tool that creates and edits BIM files — the editor counterpart to `app-console-bim`'s coordination terminal.

Read `.claude/rules/bim-product-family.md` before touching anything here.

---

## What this is — and what it is not

**Is:** a BIM authoring editor. Creates and edits IFC geometry and data. Writes working drafts to `service-bim/ingestion/queue/`. Operator reviews, commits via F12 equivalent, `service-bim` daemon archives.

**Is not:** a coordination terminal (`app-console-bim`), a viewer, or a CDE. It does not write to `canonical/` directly. It does not manage work orders or sensor linkages — those belong to `app-console-bim`.

---

## Muscle memory targets — phased

### Phase 1 — AutoCAD (2D drafting)

Command-line driven. Left hand on keyboard, right hand on mouse. Never the ribbon for high-frequency operations. The core alias set that must work from day one:

```
L      LINE          PL     PLINE         C      CIRCLE
REC    RECTANG       A      ARC           H      HATCH
M      MOVE          CO     COPY          TR     TRIM
EX     EXTEND        O      OFFSET        MI     MIRROR
RO     ROTATE        SC     SCALE         F      FILLET
LA     LAYER         Z      ZOOM          E      ERASE
```

F-key toggles: F3 (OSNAP), F8 (ORTHO), F10 (Polar), F12 (Dynamic input).

**The layer-to-IFC mapping rule:** what looks like an AutoCAD layer is an IFC element category (`IfcWall`, `IfcDoor`, `IfcSpace`, etc.). Visual convention is identical; the data model is semantically precise. Never allow freeform layer names in new code.

### Phase 2 — Navisworks (3D coordination view)

Orbit/pan/zoom/select. Properties panel auto-opens on element selection. Section planes. Measurement. BCF issue creation from 3D selection. 3D rendering via web-ifc + @thatopen/components in the webview.

### Phase 3 — Revit (property authoring)

IFC property editing, element reclassification (`IfcBuildingElementProxy` → correct type), schedule views (IFC property sets → CSV), simple element creation. **Full parametric constraint solving is post-Phase 3 — do not build it in Phase 1 or 2.**

---

## The IFC authoring data flow

```
Operator action in webview
    ↓
Tauri IPC (five commands max — see below)
    ↓
IfcOpenShell Python subprocess
  Reads from service-bim/canonical/model.ifc
  Applies operation
  Writes modified IFC to working draft (temp location)
    ↓
Webview re-renders via web-ifc
  Display only — webview never modifies the IFC directly
    ↓
Operator reviews → F12 commit action
  Tauri moves draft → service-bim/ingestion/queue/
  service-bim daemon validates, seals, archives
```

**Critical:** the webview is a display surface. It renders. It does not author. All IFC modifications go through the IfcOpenShell subprocess via Rust IPC. If you see a code path where the webview writes IFC directly, it is a bug.

---

## IPC surface — five commands maximum

| Command | Direction | Purpose |
|---|---|---|
| `open_ifc_file` | JS → Rust | Open file picker, return IFC path |
| `save_ifc_draft` | JS → Rust | Write working draft to temp location |
| `commit_to_queue` | JS → Rust | F12 equivalent — move draft to ingestion/queue/ |
| `invoke_ifcopenshell` | JS → Rust | Subprocess for geometry and property operations |
| `get_archive_dir` | JS → Rust | Return cluster-totebox-property path |

Adding a sixth command requires a new ADR entry (`BIM-WP-ADR-*`).

---

## Dependency rules

- **IfcOpenShell (LGPL-3.0):** Python subprocess only. Do not link into the Rust binary.
- **web-ifc 0.77 (MPL-2.0):** webview JS module. Clean for EUPL-1.2 host.
- **@thatopen/components (MIT):** webview JS module. Clean.
- **xeokit-sdk (AGPL-3.0):** webview JS module. EUPL-1.2 + AGPL-3.0 compatible per EC Joinup Licensing Assistant. Confirm before adding.
- **Bonsai (GPL-3.0):** reference only. Do not import. Study; do not copy.
- **ifc_rs / ifc-lite-core:** alpha stage as of April 2026. Do not use as production IFC parser. Track for future migration.

---

## ADR pre-commitments

| ADR | Rule |
|---|---|
| SYS-ADR-07 | IFC geometry and property data never routes through AI |
| SYS-ADR-10 | Operator must explicitly commit to ingestion queue — no automated archive writes |
| SYS-ADR-19 | AI may suggest; it never commits to the verified ledger |

New ADRs specific to this app: `BIM-WP-ADR-*`. Document in `ARCHITECTURE.md`.

---

## Research context

`RESEARCH.md` in this directory — covers AutoCAD command muscle memory in depth, the Bonsai reference implementation analysis, the Rust geometry ecosystem, licence architecture, and the competitive gap this tool fills.
