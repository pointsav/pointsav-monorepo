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
| 🔴 Research | Collaborative State & Version Lineage | Sovereign Data Foundation (planned) |

## 📖 Architectural Audit Placeholder

This directory is a structural placeholder in the **MOONSHOTS / PROYECTOS ESPECIALES** ledger.
It records the architectural intent to replace third-party collaborative-state libraries with a
native, verified, Rust-first replicated data type. The blueprint exists; implementation is planned.

## I. WHAT THIS MOONSHOT REPLACES

`moonshot-crdt` targets the third-party Conflict-free Replicated Data Type libraries a
collaborative workbench would otherwise depend on: **Loro**, **Yjs**, and **Automerge**. Of
these, only Loro is Rust-native; the others are JavaScript-bound and cannot serve the native
`os-workplace` surface without re-implementation.

## II. WHAT IS BEING BUILT

A native **replicated data type written in Rust** for document collaboration and history:

- Deterministic, offline-first merge of concurrent edits with no central coordinator required.
- First-class **undo/redo history and version lineage** as a product capability, not an add-on.
- A change log that is intended to be **anchorable to the customer-rooted Merkle ledger**
  (Doctrine claims #33/#34), so document history is verifiable.
- Operates on the canonical model from [`moonshot-docengine`](../moonshot-docengine).

The replacement horizon is **inside-out**: begin on a vetted, forkable Rust foundation and
migrate to this engine as it matures, retiring the external dependency.

---
*© 2026 PointSav Digital Systems.*
*Public Architectural Blueprint. Governed by the Sovereign Data Protocol.*
