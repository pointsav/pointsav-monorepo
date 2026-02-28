# PointSav Monorepo
### *Engineering & Source Code for Sovereign Digital Systems*

---

## 🏛️ Development Matrix (Six-Tier Taxonomy)
This repository contains the proprietary source code, Rust crates, and seL4 foundation modules engineered by **PointSav Digital Systems AG**. It is strictly a software development and compilation environment.

*(Note: Master design tokens and visual resources are maintained in the separate `resource-design-system` repository).*

### **Tier 6: System (Foundation)**
* `system-substrate`: Core hardware abstraction layer (HAL) source.
* `system-network-interface`: The Universal Vertical Adapter for decoupled network routing.
* `system-substrate-broadcom`: Driver logic for Broadcom hardware.
* `system-substrate-wifi`: Modular wireless networking codebase.
* `system-security`: The Capability-Based Manager (CBM) in no_std Rust.
* `system-core`: Core kernel-space primitives and the Rust no_std hardware manager.
* `system-interface`: Internal system-call abstractions.

### **Tier 5: Service (Logic)**
* `service-content`: Asset & Knowledge synthesis source code.
* `service-people`: Personnel signal distillation logic.
* `service-email`: Sovereign Email Bridge (Maildir) service codebase.

### **Tier 4: OS (Engine)**
* `os-infrastructure`: Source code for the lightweight hypervisor module.
* `os-totebox`: The capability-aware micro-kernel source.
* `os-console`: Type II hosted window manager and terminal codebase.
* `os-network-admin`: Source code for the network routing engine.
* `os-interface`: Common UI framework for PointSav operating systems.
* `os-privategit`: Sovereign source control engine (Foundry core).
* `os-mediakit`: Specialized institutional media handling source.
* `os-workplace`: Type I bare-metal OS logic.

### **Tier 3: Moonshot (Sovereign R&D)**
* `moonshot-kernel`: Project Vector (Formal Rust Verification) development.
* `moonshot-gpu`: Project X-Ray (Native Sovereign Drivers) development.
* `moonshot-database`: PSDB (Capability-aware deterministic storage).
* `moonshot-index`: Deterministic indexing engine codebase.
* `moonshot-protocol`: Verified message standard development.
* `moonshot-hypervisor`: Proprietary Rust VMM development.
* `moonshot-network`: Proprietary Rust bare-metal private network fabric.
* `moonshot-toolkit`: Rust-only toolchain development.

### **Tier 1: App (Delivery Interfaces)**
* `app-privategit-source-control`: Web interface source for the sovereign Foundry.
* `app-privategit-design-system`: Delivery interface for design system distribution.
* `app-mediakit-knowledge`: Front-end source for the institutional knowledge hub.
* `app-mediakit-distributions`: Interface for secure institutional media distribution.
* `app-mediakit-marketing`: Interface for public-facing market logic.
* `app-totebox-corporate`: Specialized vault interface for Woodfine corporate management.
* `app-totebox-real-property`: Specialized vault interface for Woodfine real property assets.

### **Tier 0: Vendor (Foreign Dependencies)**
* `vendor-sel4-kernel`: The legacy C-language kernel (Targeted by `moonshot-kernel`).
* `vendor-gpu-drivers`: Closed-source hardware firmware (Targeted by `moonshot-gpu`).
* `vendor-virtio`: 3rd-party VM network scaffolding (Targeted by `moonshot-network`).
* `vendor-azure-auth`: Microsoft authentication dependencies.
* `vendor-microsoft-graph`: Microsoft 365 Exchange API endpoints.

---
*© 2026 PointSav Digital Systems AG. All code strictly governed by the Sovereign Data Foundation.*
