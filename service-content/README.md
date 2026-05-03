# ⚙️ SERVICE-CONTENT: KNOWLEDGE SYNTHESIS

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Vendor:** PointSav Digital Systems™
**Standard:** SYS-ADR-05 (Stateless Presentation vs. Stateful Storage)
**Tier:** 5 (Service Logic)

---

## I. ARCHITECTURAL MANDATE
This service acts as the automated drafting core and linguistic compiler for the organization. It ingests raw data from the `service-slm` knowledge graph and processes it through distinct ontological matrices to produce unified, Institutional-Grade documentation.

## II. THE CONTENT COMPILER
The **Content Compiler** (`content-compiler.rs`) is a deterministic Rust engine that manages the Chart of Accounts (COA) and Archetype mapping. 
* It reads the extracted Markdown artifacts from the `knowledge-graph`.
* It queries the local `.csv` ontologies (`chart_of_accounts.csv`, `archetypes.csv`).
* It assigns the data a specific Entity ID and Domain (e.g., `ARTHUR_PENDELTON` | `PROJECTS`).
* It writes the final, verified document to the immutable `verified-ledger`.
