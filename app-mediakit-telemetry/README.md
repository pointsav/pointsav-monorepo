# app-mediakit-telemetry
### *Cookieless Intelligence Loop & Edge Ingestion*

**Status: Active Engineering** | **Taxonomy: Core Component**

## ⚙️ Execution Mechanics
This component is a lightweight Rust asynchronous HTTP server (`tokio` / `hyper`). It serves as the secure bridge between the public Edge Delivery Network and the internal PointSav data mesh, strictly enforcing DS-ADR-06 (Zero-State Telemetry).

### 1. The Ingestion Loop
Exposes a single `/telemetry-endpoint` API route to receive `navigator.sendBeacon()` POST requests from the frontend UI. It accepts strictly three JSON keys: `uri`, `timestamp`, and `user_agent`. It rejects all other traffic per the Diode Standard.

### 2. Live Anonymization (IP Masking)
Executes cryptographic IP masking in ephemeral memory. The system truncates the final octet of the incoming IPv4 or IPv6 address (e.g., transforming `203.0.113.45` into `203.0.113.0`), permanently destroying Personally Identifiable Information (PII) before storage.

### 3. Stateless Passthrough (SYS-ADR-12)
Adhering to the stateless presentation doctrine, this component retains zero memory. It formats the anonymized intelligence and appends it directly to a flat-file ledger (`telemetry_ledger.csv`) within an adjacent `os-totebox` vault.

## 📥 Inputs & Outputs
* **Input:** Asynchronous POST payloads via public network edge.
* **Output:** Comma-Separated Values (.csv) strings routed to the Totebox Archive.

---
*© 2026 PointSav Digital Systems™*
