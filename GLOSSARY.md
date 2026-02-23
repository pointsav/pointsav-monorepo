# INSTITUTIONAL GLOSSARY

## 1. NOMENCLATURE LOCK (Immutable Terms)
* **The Factory:** PointSav Digital Systems AG (The Vendor/Source Code).
* **The Fleet:** Woodfine Management Corp (The Customer/Live Operations).
* **Totebox Orchestration:** The complete environment for independent data storage.
* **Private Network:** The physical/virtual routing layer.
* **Machine-Based Authorization (MBA):** Permission via hardware pairing, not passwords.

## 2. THE POINTSAV MONOREPO MAP (Six-Tier Taxonomy)

### **Tier 0: Vendor (The Debt)**
*External dependencies and legacy code scheduled for replacement.*
* `vendor-azure-auth/` — Legacy Identity SDKs (Azure Active Directory).
* `vendor-microkit-tool/` — Legacy Build Scripts (Python/CMake).
* `vendor-microsoft-graph/` — Legacy Transport Bindings (Exchange API).
* `vendor-sel4-kernel/` — Legacy Microkernel Source (C-Language).
* `vendor-sel4-vmm/` — Legacy Virtual Machine Monitor (C-Language).
* `vendor-sled-db/` — Legacy Embedded Database (Sled).
* `vendor-tantivy-search/` — Legacy Search Library (Tantivy/Regex).
* `vendor-uefi-gop/` — Legacy Firmware Protocols (Basic Video).

### **Tier 1: App (Personalities)**
*User-facing business logic that executes within an Engine.*
* `app-design-system/` — Central Logic for Design Tokens and Visual Assets.
* `app-distribution/` — Immutable News Aggregation and Syndication.
* `app-knowledge/` — Corporate Wiki and Project Documentation.
* `app-marketing/` — Public Landing Page and Marketing Logic.
* `app-source-control/` — Sovereign Git Hosting and Versioning Logic.
* `app-totebox-corporate/` — Corporate Record Keeping and Data Management.
* `app-totebox-personnel/` — Employee Record Keeping and Identity Data.
* `app-totebox-real-property/` — Real Estate Data and Cyberphysical Connectivity.

### **Tier 2: Asset (Resources)**
*Static binaries and visual definitions.*
* `asset-fonts/` — Corporate Typography and Font Binaries.
* `asset-tokens/` — JSON-based Design Values (Colors, Spacing).

### **Tier 3: Moonshot (R&D)**
*Sovereign Rust replacements for Tier 0 dependencies.*
* `moonshot-database/` — PSDB: Capability-Aware Key-Value Store.
* `moonshot-gpu/` — Project X-Ray: Native Metal Graphics Drivers.
* `moonshot-kernel/` — Project Vector: Sovereign Rust Microkernel.
* `moonshot-network/` — Private Mesh Routing Protocol.
* `moonshot-toolkit/` — Sovereign Rust Build System.

### **Tier 4: OS (Engines)**
*Bootable runtimes and hardware managers.*
* `os-console/` — Graphical Interface Engine (TUI/WGPU).
* `os-infrastructure/` — Headless Node Provisioning Engine.
* `os-interface/` — Archive Aggregation Gateway.
* `os-mediakit/` — Compliance and Investor Relations Appliance.
* `os-network-admin/` — Routing Policy and Authority Engine.
* `os-privategit/` — Sovereign Code Repository Engine.
* `os-totebox/` — Data Vault and Segregation Root.
* `os-workplace/` — Desktop Environment and Metal Runtime.

### **Tier 5: Service (Logic)**
*Background daemons for data processing.*
* `service-content/` — Knowledge Synthesis and Memo Generation.
* `service-email/` — Microsoft Exchange Bridge (Maildir).
* `service-people/` — Identity Mining and Recruitment Logic.

### **Tier 6: System (Foundation)**
*Immutable primitives and hardware abstraction.*
* `system-core/` — Shared Types, Error Definitions, and Root Traits.
* `system-interface/` — Low-Level Rasterizer and Layout Primitives.
* `system-security/` — Capability Management and Access Control.
* `system-substrate/` — Hardware Abstraction Layer (HAL) and Drivers.
