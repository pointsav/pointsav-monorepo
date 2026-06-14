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
| 🔴 Research | Document Model & Bidirectional Mapping | Sovereign Data Foundation (planned) |

## 📖 Architectural Audit Placeholder

This directory is a structural placeholder in the **MOONSHOTS / PROYECTOS ESPECIALES** ledger.
It records the architectural intent to replace third-party document-model libraries with a
native, verified, Rust-first engine. The blueprint exists; implementation is planned.

## I. WHAT THIS MOONSHOT REPLACES

`moonshot-docengine` targets the third-party rich-document model dependencies the workbench
would otherwise carry: **ProseMirror**, **Lexical**, and **TipTap** (JavaScript document
models), together with the hand-rolled JavaScript markdown parser and renderer presently
embedded in `app-privategit-workbench`. These are foreign, JavaScript-bound, and cannot reach
the native `os-workplace` surface without a second implementation.

## II. WHAT IS BEING BUILT

A single canonical **document engine written in Rust and compiled to WebAssembly**:

- A canonical document model (tree of typed nodes + marks) that is the single source of truth.
- **AST-accurate bidirectional mapping** between a rendered "what-you-see-is-what-you-get"
  view and the serialized source form — every rendered node carries its exact source range,
  so a highlight in the viewer maps to a precise source span and back. This replaces the
  current lossy text-match synchronisation.
- Deterministic round-tripping (render → source → render) with no drift.
- One engine shared by two front ends: the browser prototype today, the native
  `os-workplace` surface tomorrow — the engine is intended to remain unchanged across both.

Pairs with [`moonshot-parser`](../moonshot-parser) (incremental parse),
[`moonshot-crdt`](../moonshot-crdt) (collaborative state), and
[`moonshot-editor`](../moonshot-editor) (widget surface).

---
*© 2026 PointSav Digital Systems.*
*Public Architectural Blueprint. Governed by the Sovereign Data Protocol.*
