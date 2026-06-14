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
| 🔴 Research | IFC Parser & BIM Render Engine | Sovereign Data Foundation (planned) |

## 📖 Architectural Audit Placeholder

This directory is a structural placeholder in the **MOONSHOTS / PROYECTOS ESPECIALES** ledger.
It records the architectural intent to replace third-party BIM rendering infrastructure with a
native, verified, Rust-first engine. The blueprint exists; implementation is planned.

## I. WHAT THIS MOONSHOT REPLACES

`moonshot-bim-engine` targets the third-party Building Information Modelling stack the workbench
would otherwise depend on: **web-ifc** (MPL-licensed IFC parser) and **xeokit** (commercially
licensed BIM viewer). This dependency is the licensing gate recorded against the `app-workplace-bim`
surface in the architecture ledger — owning it removes the gate.

## II. WHAT IS BEING BUILT

A native **IFC parser and BIM geometry/render engine written in Rust** (WebAssembly-targetable):

- An **IFC (ISO 16739-1:2024) parser** for the canonical, open building model format — the
  fiduciary record is plain, standard, and never a proprietary binary.
- A geometry and render pipeline for BIM models that runs without a third-party viewer.
- The highest-complexity item in this initiative; staged behind the document-centric engines.

> [!NOTE]
> This is the most ambitious moonshot in the workbench set and carries the most schedule risk.
> A vetted interim path may be used for early demonstrations while this engine matures; the
> objective remains full ownership and the retirement of the external BIM dependency.

---
*© 2026 PointSav Digital Systems.*
*Public Architectural Blueprint. Governed by the Sovereign Data Protocol.*
