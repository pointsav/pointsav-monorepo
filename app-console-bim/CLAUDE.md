# app-console-bim

ConsoleOS coordination terminal for BIM operations. Navisworks muscle memory. This is the operator's tool for querying, coordinating, and authorizing — not for authoring geometry.

Read `.claude/rules/bim-product-family.md` before touching anything here.

---

## What this is — and what it is not

**Is:** a routing terminal. Reads from `service-bim` flat files. Displays floor plans and 3D models. Lets the operator select IFC elements, link work orders, create BCF issues, and execute F12 commits to the immutable ledger.

**Is not:** an editor. It does not create or modify IFC geometry. It does not write to `canonical/`. An operator who needs to edit geometry uses `app-workplace-bim`.

This distinction is architectural, not cosmetic. Keep it clear in every UI decision.

---

## Muscle memory target — Navisworks

The operator should recognize these interactions immediately:

| Action | Navisworks convention | Implementation |
|---|---|---|
| Orbit 3D | Middle-mouse drag | Middle-mouse drag |
| Select element | Left click | Left click → IFC GUID |
| Properties panel | Appears on selection | Auto on selection |
| Section plane | Sectioning toolbar | Section plane tool |
| Create issue | Review → Redline | BCF 3.0 → `service-bim/issues/` |
| Navigate hierarchy | Selection tree | IFC spatial tree: Site → Building → Storey → Space → Element |

Do not invent new interaction patterns for operations that Navisworks has already established. Match first, extend later.

---

## The F12 pattern here

F12 in this application is the commit gate for operational linkages:
- Linking an IFC element GUID to a work order record
- Linking an IFC element GUID to a lease register entry
- Linking a sensor ID to an IFC element GUID
- Marking a BCF issue resolved

The AI (`service-slm`) may suggest which element a work order relates to. It cannot execute the link. The operator selects, confirms, F12 commits. SYS-ADR-10 — no exceptions.

---

## Connection to orchestration

Single-property mode: connects directly to `cluster-totebox-property`. No orchestration layer needed.

Multi-property mode: connects to `app-orchestration-bim`, which aggregates across multiple `cluster-totebox-property` instances. The console UI is identical in both modes — the orchestration layer is transparent to the operator.

---

## IPC surface

Four commands maximum, following the platform pattern:

| Command | Direction | Purpose |
|---|---|---|
| `open_archive` | JS → Rust | Connect to a PropertyArchive path |
| `query_element` | JS → Rust | Retrieve element YAML by IFC GUID |
| `write_link` | JS → Rust | F12 commit — write operational link to sidecar |
| `create_issue` | JS → Rust | Write BCF issue to `service-bim/issues/` |

Adding a fifth command requires a new ADR entry (`BIM-CON-ADR-*`).

---

## Research context

`RESEARCH.md` in this directory — covers the competitive gap this tool fills, federated model architecture using xeokit or @thatopen/components, and the CityJSONSeq portfolio query pattern.
