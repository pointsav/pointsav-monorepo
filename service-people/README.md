# ⚙️ SERVICE-PEOPLE: SOVEREIGN PERSONNEL LEDGER
**Vendor:** PointSav Digital Systems™
**Protocol:** DS-ADR-02 (Files over Databases)
**Tier:** 1 (Core Engineering Monorepo)

---

## I. ARCHITECTURAL OVERVIEW
This engine is a deterministic JSON state machine. It manages the centralized personnel ledger. It replaces fragile database clusters with verifiable flat files. It provides CLI query and update operations to local execution adapters.

## II. DATA STRUCTURE (JSON)
The engine reads and writes strictly to `ledger_personnel.json`. It stores nested, multi-dimensional communication history for every contact ID. This ensures schema stability for future Sovereign AI Routing ingestion.

## III. LEGAL & LICENSING
Refer to the `LICENSE` file in this directory. This software is currently under a strict **Incubation Phase**. All rights are reserved by Woodfine Capital Projects Inc.
