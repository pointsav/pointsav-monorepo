# ⚙️ SERVICE-EMAIL: SOVEREIGN INGESTION GATEWAY

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Vendor:** PointSav Digital Systems™
**Standard:** SYS-ADR-07 (Bifurcated Ingestion)
**Tier:** 5 (Service Logic)

---

## I. ARCHITECTURAL MANDATE
This component operates as the primary ingestion diode for the Sovereign Totebox Architecture. It penetrates legacy email infrastructures, extracts inbound assets without retaining state, and mathematically splinters the payload for secure downstream processing.

## II. THE DUAL-ENGINE PIPELINE
This service is divided into two distinct Rust binaries:

1. **`master-harvester-rs` (The Pull Diode):** An asynchronous `tokio` engine that queries Microsoft Graph API (OData JSON). It utilizes a 2-minute `cron` pulse and micro-batching (Max 5 assets) to securely extract payloads from targeted folders (`totebox-ingress`, `OpenStack`) directly into the local `/new` Spool, bypassing public cloud storage entirely.
2. **`mime-splinter` (The Forensic Diode):** A deterministic routing engine triggered by the `spool-daemon`. It shatters raw `.eml` payloads into their constituent parts, routing Identity Mass (`.csv`) to `service-people`, linguistic bodies (`.txt`/`.html`) to `service-slm`, and inert media (`.png`/`.pdf`) to `assets/inert-media`.
