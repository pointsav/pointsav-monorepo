# app-mediakit-telemetry
### *Cookieless Intelligence Loop & Edge Ingestion*

**Status: Provisioning** | **Taxonomy: Core Component**

## ⚙️ Execution Mechanics
This component operates on the `os-mediakit` substrate. It serves as the secure bridge between the public Edge Delivery Network (GitHub Pages) and the internal PointSav data mesh, strictly enforcing DS-ADR-06 (Zero-State Telemetry).

### 1. The Ingestion Loop
Exposes a single API endpoint to receive `navigator.sendBeacon()` POST requests from the frontend UI. It accepts strictly three JSON keys: `uri`, `timestamp`, and `user_agent`.

### 2. Live Anonymization (IP Masking)
Executes cryptographic IP masking in ephemeral memory. The system truncates the final octet of the incoming IP address (e.g., transforming `203.0.113.45` into `203.0.113.0`), permanently destroying Personally Identifiable Information (PII) before storage.

### 3. Stateless Passthrough
Adhering to SYS-ADR-05, this component retains zero memory. It formats the anonymized intelligence and passes it securely to an `os-totebox` for permanent storage as a `.csv` flat-file ledger.

## 📥 Inputs
* Asynchronous POST payloads via public network edge.

## 📤 Outputs
* Comma-Separated Values (.csv) strings routed to the Totebox Archive.

## 🔗 Dependencies
* `system-core`
* `os-mediakit`
* `DS-ADR-06` (Privacy Posture)

---
*© 2026 PointSav Digital Systems™*
