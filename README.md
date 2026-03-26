<div align="center">

# PointSav Digital Systems
### *Engineering & Source Code for Independent Digital Infrastructure | Ingeniería y Código Fuente para Infraestructura Digital Independiente*
**Vancouver | New York | Berlin**

[ **Documentation Wiki** ](https://github.com/pointsav/content-wiki-documentation) | [ **Design System** ](https://github.com/pointsav/pointsav-design-system) | [ **Reference Deployment (Woodfine Fleet)** ](https://github.com/woodfine/woodfine-fleet-deployment)

</div>

---

## 1. EXECUTIVE OVERVIEW | VISIÓN GENERAL EJECUTIVA

**[ EN ]** PointSav Digital Systems engineers Trustworthy Systems for the Digital First enterprise. We construct Capability-Based Managers in `no_std` Rust, establishing decentralized operating environments that guarantee absolute custodial control. This architecture mathematically bypasses legacy Software-as-a-Service (SaaS) vulnerabilities, ensuring that institutional data remains permanently secured on physical hardware owned by the deploying enterprise. Our commercial model physically separates generic open-source foundations (Apache 2.0) from proprietary infrastructure routing and archive aggregation.

**[ ES ]** PointSav Digital Systems diseña Sistemas Confiables para la empresa de prioridad digital. Construimos Gestores Basados en Capacidades en Rust `no_std`, estableciendo entornos operativos descentralizados que garantizan un control de custodia absoluto. Esta arquitectura evita matemáticamente las vulnerabilidades del Software como Servicio (SaaS) tradicional, asegurando que los datos institucionales permanezcan bajo el control de la empresa. Nuestro modelo comercial separa físicamente las bases de código abierto (Apache 2.0) del enrutamiento de infraestructura propietario.

---

## 2. THE 3-LAYER STACK | LA PILA DE 3 NIVELES
This architecture decouples computational logic from physical hardware, establishing a verifiable, cloud-agnostic operating environment.

### 2.1 Infrastructure Layer (The Private Network)
* **`os-infrastructure`**: A lightweight hypervisor enabling stateless node provisioning across physical or virtual boundaries.
* **`os-network-admin`**: The routing authority orchestrating the Private Network and enforcing physical access policies.

### 2.2 Platform Layer (Portable Unikernels)
* **`os-totebox`**: The core, hardware-agnostic microkernel foundation for absolute data segregation.
* **`os-integration`**: The gateway component executing the aggregation of multiple isolated Totebox Archives.
* **`os-mediakit`**: A high-performance "Just-Enough-OS" (JeOS) optimized for compliance reporting and investor relations.
* **`os-privategit`**: The independent version-control and local build server.

### 2.3 Delivery Layer (User Terminals)
* **`os-workplace`**: A bare-metal Desktop Environment installed on trusted hardware, executing the interface layer.
* **`os-console`**: The graphical terminal and Command Ledger, hosting UI plugins (`app-console-*`) to interact with the underlying Totebox Archives.

---

## 3. ACTIVE SERVICES (TOTEBOX ORCHESTRATION)
Specialized business logic engines compiled to execute within the isolated bounds of a Totebox Archive.

* **`service-content`**: The deterministic Linguistic Compiler. Synthesizes institutional knowledge, applies continuous disclosure protocols, and generates structured reporting.
* **`service-people`**: The Personnel Ledger. Distills raw digital signals into a deterministic identity matrix, bypassing standard database schemas.
* **`service-email`**: The Ingestion Gateway. Penetrates legacy infrastructure to securely extract and archive corporate communications using the local Maildir format.
* **`service-slm`**: The Localized Small Language Model. A dedicated semantic parser isolating the microkernel from external commercial intelligence models.

---

## 4. ENGINEERING TO DATE (THE MONOREPO CORE)
The active foundation of the Trustworthy System, defining the mathematical limits of the physical hardware.

* **`system-substrate`**: The operational microkernel (seL4) and physical hardware bridges (e.g., Broadcom, FreeBSD).
* **`system-core` & `system-foundation`**: Proprietary Rust `no_std` libraries managing hardware resource allocation and execution parameters.
* **`system-security`**: The Capability Monitor enforcing strict cryptographic pairing and authorization across the network.
* **`system-interface`**: The pure software rendering engine and UI layout rasterizer.

---

## 5. VENDOR QUARANTINES & NATIVE REPLACEMENTS
PointSav systematically tracks and isolates legacy dependencies (Foreign Technology). Each Quarantined Component represents technical debt slated for replacement by an active, native Rust Moonshot initiative.

| Quarantined Component (Legacy) | Functional Role | Native Replacement (Moonshot) |
| :--- | :--- | :--- |
| `vendor-sel4-kernel` | C-Language Microkernel | **`moonshot-kernel`** (Project Vector: `no_std` Rust) |
| `vendor-virtio` | Virtualization Bridge | **`moonshot-hypervisor`** (Rust VMM) |
| `vendor-gpu-drivers` | UEFI Firmware | **`moonshot-gpu`** (Project X-Ray: Native Drivers) |
| `vendor-linux-systemd` | Process Supervision | **`moonshot-toolkit`** (Rust-Only Toolchain) |
| `vendor-azure-auth` | Commercial Identity API | Quarantined outside core microkernel. |
| `vendor-microsoft-graph` | External Mail API | Quarantined to `service-email` bridge. |

---
*© 2026 PointSav Digital Systems. Public Architectural Blueprint.*
