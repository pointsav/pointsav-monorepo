# os-infrastructure
### *Edge & Cloud Relays*

**Status: Verified (Production Iteration 1)** | **Taxonomy: Tier-4-OS**

This crate is the delivery vehicle for the PointSav Private Network (PPN) edge nodes. It is responsible for packaging the compiled Capability-Based Manager (`system-security`) and the seL4 microkernel into a deployable, bootable ISO image.

## 🏛️ Deployment Architecture
`os-infrastructure` serves as the lightweight execution environment. It is hardware-agnostic at the payload level but strictly specifies the bootloader sequence to ensure verifiable execution across diverse deployment targets (e.g., On-Premise Metal, Virtualization Bridges, and Cloud Providers).

### Target Execution Environments
* **Bare Metal (Native):** Direct execution on silicon possessing compliant IOMMU (VT-d) capabilities.
* **The Virtualization Bridge (Hosted):** Execution within a `vendor-virtio` bridge operating atop `system-substrate-freebsd`. This pattern is utilized for legacy silicon (e.g., Intel Penryn P8600) lacking hardware passthrough, or cloud instances (e.g., Google Cloud Platform) where hypervisor abstraction is mandated.

## ⚙️ Build Orchestration
Standard kernel loading mechanisms do not reliably support 64-bit seL4 binaries. Therefore, this component orchestrates a strict dual-payload boot sequence.

### The Boot Process
1. **ELF Synthesis:** Ingests the `final_image.elf` synthesized by the Tier-6 `system-security` component.
2. **Bootloader Wrapping:** Utilizes GRUB with Multiboot2 compliance.
3. **Primary Payload:** The verified seL4 microkernel is loaded into memory first.
4. **Secondary Payload:** The `system-security` user-space initializer is loaded as a Multiboot module.
5. **ISO Forging:** The components are packaged into a `.iso` artifact.

## 🛡️ Fleet Integration
The resulting ISO is "Silicon-Pinned" through configuration profiles, meaning the memory boundaries and expected VirtIO interfaces are strictly defined prior to compilation. The artifact is then deployed to nodes designated as `fleet-infrastructure-*` within the Woodfine Fleet Manifest.
