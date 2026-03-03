# system-substrate-freebsd
### *Sovereign Driver Domain (Host OS)*

**Status: Operational (Showcase MVP)** | **Taxonomy: Tier-6-System**

This component serves as the BSD-licensed foundation providing battle-tested hardware support. It operates as Layer 0 on bare-metal nodes lacking VT-d (IOMMU) capabilities, specifically targeted at legacy silicon such as the Intel Penryn P8600.

## 🏛️ Architectural Mandate
To ensure 100% legal and architectural sovereignty over the driver substrate, PointSav utilizes a minimal FreeBSD implementation to bypass multi-month driver development for legacy components.

* **Broadcom Support:** Natively interfaces with `14e4:432b` and `14e4:16b4` network interface cards.
* **ACPI States:** Enforces mandatory lid-switch overrides for continuous headless operation.
* **Hypervisor Foundation:** Provides the bare-metal execution environment required to host the `vendor-virtio` (`bhyve`) shim layer.
