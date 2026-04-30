# service-bim

Background daemon running inside `os-totebox` PropertyArchive. No UI. No network. One job: maintain the flat-file BIM archive for one property.

Read `.claude/rules/bim-product-family.md` before touching anything here.

---

## What this crate does

Watches `ingestion/queue/` for incoming files. When a file arrives:

1. Converts to IFC-SPF if not already IFC (via IfcOpenShell subprocess)
2. Validates against `requirements/handover.ids` (via `ifctester`)
3. On pass: writes to `canonical/model.ifc`, computes SHA-256, updates `objects/` store
4. Generates SVG floor plans (IfcOpenShell SVG slicer)
5. Regenerates `cache/model.glb` (IfcConvert → glTF)
6. On fail: writes report to `ingestion/log/`. Never commits.

It never writes directly to `canonical/` except as step 3 above. No other component writes to `canonical/`.

---

## Hard rules for this crate

- IFC geometry and property data never routes through `service-slm` or any AI layer — SYS-ADR-07
- `service-bim` is the only process with write access to `canonical/` — surface any code that violates this
- All file operations are append-only or write-once — no modification of committed records
- IfcOpenShell is invoked as a Python subprocess, not linked into the binary (LGPL compliance)
- Zero outbound network calls — if you see a network call, it is a bug

---

## Key directories

| Path | What it is |
|---|---|
| `src/` | Rust daemon source |
| `src/watcher.rs` | Filesystem watch on `ingestion/queue/` |
| `src/ingest.rs` | Conversion and validation pipeline |
| `src/archive.rs` | Canonical write, SHA-256, objects store |
| `src/derive.rs` | SVG and glTF generation (delegates to IfcOpenShell) |
| `scripts/` | IfcOpenShell Python helpers invoked as subprocesses |

---

## IPC surface

None. `service-bim` has no IPC. It is not a server. Other components interact with it through the filesystem only — writing to `ingestion/queue/`, reading from `drawings/`, reading `canonical/model.ifc`.

---

## Research context

`RESEARCH.md` in this directory — read it for the full rationale on IFC format decisions, IDS validation, IoT sensor integration, and the competitive context.
