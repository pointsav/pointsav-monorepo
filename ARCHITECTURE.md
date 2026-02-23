# FACTORY ARCHITECTURE: The PointSav Monorepo

> **Mission:** To replace Legacy Debt (Tier 0) with Sovereign Rust Primitives (Tier 3-6).

## 1. System Matrix (The 3-Layer Stack)

This architecture decouples "Logic" from "Metal," allowing sovereign portability.

### Layer A: Infrastructure (The Rails)
* **`os-infrastructure`**: A stateless hypervisor node. It holds no data, only compute.
* **`os-network-admin`**: The routing authority that manages the Private Network.

### Layer B: Platform (The Payload)
* **`os-totebox`**: The isolated vault for data storage.
* **`os-interface`**: The aggregation engine that connects multiple vaults.

### Layer C: Delivery (The Terminal)
* **`os-console`**: The user-facing TUI terminal.
* **`os-workplace`**: The bare-metal desktop environment.

## 2. Detailed Inventory
For a complete list of all folders and tiers, please refer to [`GLOSSARY.md`](./GLOSSARY.md).
