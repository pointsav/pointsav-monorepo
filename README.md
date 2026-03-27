<div align="center">

<img src="https://raw.githubusercontent.com/pointsav/pointsav-media-assets/main/ASSET-SIGNET-MASTER.svg" width="80" alt="PointSav Signet">

# PointSav Monorepo | Código Fuente Soberano
### *Engineering & Source Code for Sovereign Digital Systems*

[![Architecture](https://img.shields.io/badge/Architecture-seL4_Unikernel-111827?style=flat-square)](#)
[![Language](https://img.shields.io/badge/Language-Rust_no__std-111827?style=flat-square)](#)
[![Compliance](https://img.shields.io/badge/Compliance-SOC_3_%7C_DARP-111827?style=flat-square)](#)
[![Telemetry](https://img.shields.io/badge/Telemetry-Zero_Cookie-111827?style=flat-square)](#)

[ **Documentation Wiki** ](https://github.com/pointsav/content-wiki-documentation) | [ **Design System** ](https://github.com/pointsav/pointsav-design-system) | [ **Live Fleet Manifest** ](https://github.com/woodfine/woodfine-fleet-deployment)

</div>

---

> [!NOTE]
> **OPERATIONAL POSTURE [MARCH 2026]**
> **Phase:** Production Iteration 1 | **Focus:** 3-Track Architecture & SOC 3 / DARP Compliance | **Estado:** Desarrollo activo de servicios Totebox.

## 🚀 The Digital First Operating System
**[ EN ]** PointSav engineers Trustworthy Systems. We build decentralized, mathematically verified operating environments that guarantee structural sovereignty natively, completely bypassing legacy Software-as-a-Service (SaaS) databases.

**[ ES ]** PointSav diseña Sistemas Confiables. Construimos entornos operativos descentralizados y verificados matemáticamente que garantizan la soberanía estructural de forma nativa.

<details>
<summary><b>🛡️ VIEW SECURITY BOUNDARY DECLARATION</b></summary>
<br>
This repository contains the <code>no_std</code> Rust engineering source code and Capability-Based Manager definitions. <b>It contains zero active cryptographic keys, network payloads, or client data.</b>
<br><br>
</details>

---

## 📐 The 3-Track Capability Matrix
Our architecture strictly decouples computational logic from physical hardware, establishing a verifiable, cloud-agnostic operating environment.

### ⚙️ Track 1: Infrastructure (Private Network)
| Component Directory | Hardware Target | Status |
| :--- | :--- | :--- |
| [`os-infrastructure`](./os-infrastructure) | Intel P8600 (Edge Node) | 🟢 `Verified (Multiboot2)` |
| [`os-network-admin`](./os-network-admin) | Intel i5-2400S (Command) | 🟡 `Active Engineering` |

### 🧠 Track 2: Totebox Orchestration (Active Services)
| Component Directory | Payload Engine | Status |
| :--- | :--- | :--- |
| [`service-content`](./service-content) | Linguistic compiler & institutional synthesis. | 🟡 `Active Engineering` |
| [`service-people`](./service-people) | Personnel signal distillation & scoring. | 🟡 `Active Engineering` |
| [`service-email`](./service-email) | Sovereign Exchange bridge (Maildir local vault). | 🟡 `Active Engineering` |

### 🖥️ Track 3: Workplace (Sovereign Desktop)
| Component Directory | Deterministic File Output | Status |
| :--- | :--- | :--- |
| [`app-workplace-editor`](./app-workplace-editor) | Markdown, YAML, CSV | 🟡 `Active Engineering` |
| [`app-workplace-comm`](./app-workplace-comm) | Maildir local storage | 🟡 `Active Engineering` |

---
*© 2026 PointSav Digital Systems™*
