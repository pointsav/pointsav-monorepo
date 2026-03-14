# ⚙️ SERVICE-SLM
**Entity:** PointSav Digital Systems™ (The Vendor)
**Taxonomy:** Tier-5 Core Component
**Status:** Active Engineering

---

## I. ARCHITECTURAL MANDATE
This component acts as the Sovereign Dispatcher. It is a highly controlled, headless bridging script that passes localized data to the `vendor-slm-engine` for linguistic processing. 

## II. THE ZERO-OMNIPRESENCE MANDATE
To protect the integrity of the Totebox Archive, this service is mathematically forbidden from running as a persistent background daemon. 

* **Execution:** It is a point-in-time filter. It is invoked, processes a single text file via standard input (STDIN), writes to standard output (STDOUT), and terminates immediately.
* **No Stateful Access:** It cannot directly modify `ledger_personnel.json` or corporate databases. It writes exclusively to isolated `/outbox/` or `/drafts/` directories for human review.
