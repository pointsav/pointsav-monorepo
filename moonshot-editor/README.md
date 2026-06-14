<div align="center">

# Sovereign Replacement Initiative | Iniciativa de Reemplazo Soberano

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

### *An active engineering initiative to replace foreign third-party architecture.*

</div>

<br/>

> [!WARNING]
> **SOVEREIGN FRAMEWORK DECLARATION**
> This repository is a reference implementation of the Sovereign Data Protocol. It enforces absolute data isolation. It contains zero active proprietary network payloads.

| Architecture Tier | Component Role | Governance Anchor |
| :--- | :--- | :--- |
| 🔴 Research | Editor / Viewer / File-Tree Widget Surface | Sovereign Data Foundation (planned) |

## 📖 Architectural Audit Placeholder

This directory is a structural placeholder in the **MOONSHOTS / PROYECTOS ESPECIALES** ledger.
It records the architectural intent to replace third-party editor and tree-virtualization
widgets with a native, verified, Rust-first widget surface. The blueprint exists; implementation
is planned.

## I. WHAT THIS MOONSHOT REPLACES

`moonshot-editor` targets the third-party interactive widgets a workbench would otherwise carry:
the **CodeMirror 6** / **Monaco** code editors and the **react-arborist** / **react-window**
tree-virtualization libraries. It also retires the current `app-privategit-workbench` editor —
a transparent `<textarea>` layered over a highlight `<pre>` — and its non-virtualized file tree,
which renders one DOM node per file and is the source of the navigation delay on large trees.

## II. WHAT IS BEING BUILT

A native **editor / viewer / file-tree widget surface** (Rust → WebAssembly):

- A text editor core: cursor and selection model, gutter, find/replace, multi-cursor — driven
  by [`moonshot-docengine`](../moonshot-docengine) and highlighted by
  [`moonshot-parser`](../moonshot-parser).
- A **virtualized file tree** that renders only visible rows and loads directory children on
  demand, eliminating the O(n)-DOM and synchronous-per-folder-fetch behaviour that causes the
  current lag.
- A selection model that yields a **stable section handle**, the unit the AI section-editing
  bridge operates on.

The same widget surface is intended to serve both the browser prototype and the native
`os-workplace` surface without a second implementation.

---
*© 2026 PointSav Digital Systems.*
*Public Architectural Blueprint. Governed by the Sovereign Data Protocol.*
