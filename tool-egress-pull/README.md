# 🧰 TOOL-EGRESS-PULL: THE ASYMMETRIC DIODE
**Vendor:** PointSav Digital Systems™
**Standard:** Leapfrog 2030 (Asymmetric Storage Protocol)
**Tier:** 3 (Operational Tool)

---

## I. ARCHITECTURAL MANDATE
This component operates strictly on the Tier-1 or Tier-3 local terminals (e.g., the iMac Command Authority). It is a manual operational script executed by a human-in-the-loop when a secure cold-storage drive (e.g., a 3TB USB) is physically mounted.

## II. EXECUTION MECHANICS
* **Action:** Executes a secure authenticated pull of the chunked binaries staged by `service-egress` on the cloud node.
* **Reconstruction:** Decompresses the chunks and perfectly reconstructs the 1-Dimensional physical file tree (`service-email/`, `service-content/`) directly onto the physical USB drive.
* **Verification:** Computes SHA-256 checksums to verify absolute fidelity against the cloud, then transmits a "WIPE" authorization back to the cloud node to complete the Asymmetric Storage cycle.
