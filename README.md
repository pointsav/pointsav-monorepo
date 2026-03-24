<div align="center">

# PointSav Monorepo
### *Engineering & Source Code for Independent Digital Infrastructure*

[ **Documentation Wiki** ](https://github.com/pointsav/content-wiki-documentation) | [ **Design System** ](https://github.com/pointsav/pointsav-design-system) | [ **Main Profile** ](https://github.com/pointsav)

*Operational Deployment:* [ **Woodfine Management Corp.** ](https://github.com/woodfine)

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

---

> [!NOTE]
> **OPERATIONAL POSTURE [MARCH 2026]**
> **Phase:** Production Iteration 2 | **Focus:** Sovereign Data Pipeline & Cryptographic Build Ledgers | **State:** Active Totebox Service Deployment.

### 🚀 The Digital First Operating System
PointSav provides the foundational engineering to secure the modern enterprise and democratize data ownership. We build decentralized Unikernel operating environments that bypass the vulnerabilities of legacy web applications and commercial hyperscaler infrastructure. 

By eradicating the generic, multi-tenant Linux OS entirely, our architecture isolates critical workloads—such as cryptographic file systems and cognitive AI routing—into dedicated, memory-safe Unikernels. We guarantee that operational ledgers remain mathematically sealed from external extraction or systemic cyber threats.

> [!WARNING]
> **SECURITY BOUNDARY DECLARATION**
> This repository contains the `no_std` Rust engineering source code and Capability-Based Manager definitions. **It contains zero active cryptographic keys, network payloads, or client financial data.**

### ⚙️ Track 1: Infrastructure (The Cryptographic Build Ledger)
Focus: Bare-metal execution, virtualization bridges, and mathematical locks.
| Component Directory | Engineering Target | Status |
| :--- | :--- | :--- |
| [`os-infrastructure`](./os-infrastructure) | Edge Delivery Nodes | 🟢 `Verified Architecture` |
| [`os-network-admin`](./os-network-admin) | Command & Routing Gateways | 🟡 `Active Engineering` |
| [`system-security`](./system-security) | seL4 Capability-Based Manager (Rust) | 🟢 `Verified` |

### 🧠 Track 2: Totebox Orchestration (Active Service Sandboxes)
Focus: Data processing, identity isolation, and deterministic intelligence routing.
| Component Directory | Payload Engine & Risk Mitigation | Status |
| :--- | :--- | :--- |
| [`service-email`](./service-email) | Ingestion Gateway (MSFT Graph Harvester & MIME Splinter). Bypasses API scraping. | 🟢 `Verified` |
| [`service-people`](./service-people) | Personnel Signal Distillation (Sovereign ACS Engine). | 🟢 `Verified` |
| [`service-slm`](./service-slm) | Linguistic Air-Lock (Cognitive Forge - SmolLM2 135M). Headless noise reduction. | 🟢 `Verified` |
| [`service-content`](./service-content) | Knowledge Synthesis (Content Compiler, Verified Ledgers & Self-Healing Wikis). | 🟢 `Verified` |
| [`service-search`](./service-search) | Sovereign Search (Leapfrog 2050 Flat-File Inverted Index). Replaces Elasticsearch. | 🟡 `Active Engineering` |

### 🖥️ Track 3: Workplace (The Sovereign Desktop)
Focus: Operator environments engineered to produce zero-dependency outputs.
| Component Directory | Deterministic File Output & Interfaces | Status |
| :--- | :--- | :--- |
| [`os-console`](./os-console) | Operator Delivery Vehicle (Local HTTP Server). | 🟡 `Active Engineering` |
| [`app-console-*`](./) | Admin & Surveyor Interfaces (Content, Email, People, Input). | 🟡 `Active Engineering` |
| [`app-workplace-editor`](./app-workplace-editor) | Machine-readable standards: Markdown, YAML, CSV. | 🟡 `Active Engineering` |
| [`app-workplace-comm`](./app-workplace-comm) | Localized, asynchronous Maildir storage. | 🟡 `Active Engineering` |

---
*© 2026 PointSav Digital Systems™*
