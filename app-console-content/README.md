# 🏗️ APP-CONSOLE-CONTENT (Verification Surveyor)

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems™ (The Vendor)
**Taxonomy:** Tier-3 Delivery Layer

## I. ARCHITECTURAL MANDATE
This CLI application enforces the Human-in-the-Loop verification sequence for the Sovereign Data Archive. It retrieves unverified identity fragments from `service-people/discovery-queue` and prompts the operator to verify them using air-gapped, external browsing logic.

## II. THE COGNITIVE THROTTLE
To guarantee maximum data fidelity and eliminate alert fatigue, this binary enforces a mathematically strict limit of **10 verifications per 24-hour cycle**. Once the threshold is met, the kernel physically locks the application until midnight.
