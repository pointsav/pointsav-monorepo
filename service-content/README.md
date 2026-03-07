# Service-Content: Linguistic Synthesis Engine

This silo contains the PointSav OS synthesis engine. It is a stateless Rust application that leverages the Gemini API to transform legacy documents into machine-readable institutional memory.

## 🏛️ Ingestion Gateway (The Console)

The engine is driven by a Desktop Console interface designed for Mathew and Jennifer.

### 1. Installation
To set up the gateway on a new workstation (macOS 13+ or Linux Mint):
1. Clone this repository.
2. Navigate to `service-content/tools/`.
3. Run the installer:
   ```bash
   bash install_console_input.sh
   ```

### 2. Usage Workflow
1.  **Airlock**: Drop legacy files into `~/Desktop/service-content/input`.
2.  **Trigger**: Run `pointsav-input` in your terminal.
3.  **Target**: Select the destination Silo (The Data Mesh) for persistence.
4.  **Result**: The artifact appears in `~/Desktop/service-content/output`.

## ⚙️ Architecture
- **Engine**: Rust-based synthesis (Stateless).
- **Protocols**: `pointsav-design-system/tokens/linguistic/`.
- **Data Mesh**: Results persisted to service-silo `assets/` on node-gcp-free.

---
**Status**: v1.5 Deployed (Airlock-to-Mesh Active)
