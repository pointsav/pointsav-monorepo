# 🏗️ SERVICE-PARSER
**Entity:** PointSav Digital Systems™ (The Vendor)
**Taxonomy:** Tier-5 Core Component
**Status:** Architectural Scaffold (Active Engineering Cycle)

---

## I. ARCHITECTURAL MANDATE
This component acts as the Active Intelligence Router. It refines toxic third-party network payloads into clean, machine-readable Entity Bundles.

## II. EXECUTION MECHANICS
* **Inputs:** Raw `.json` payloads located in `/assets/tmp-maildir/`.
* **Outputs:** Timestamped Entity Bundles (Structured Plaintext + Native Binaries) routed to `/assets/personnel-maildir/` or ephemeral AI queues.
* **Dependencies:** `service-email` extraction completion.
