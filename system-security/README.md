# system-security
### *Capability-Based Manager (CBM)*

**Status: Operational (Production Iteration 1)** | **Taxonomy: Tier-6-System**

This component replaces standard operating system abstraction layers with a verifiable Rust implementation. Operating directly on the seL4 microkernel, it manages hardware resources securely and enforces strict, one-way command flow before guest operating systems are permitted to boot.

## 🏛️ Architecture & Fault Tolerance
The Capability-Based Manager utilizes a dual-domain architecture designed for autonomous self-healing and continuous execution.

### 1. Protection Domains (PD)
* **System Security (Muscle):** The primary initialization and hardware management domain. It is responsible for orchestrating the boot sequence and continuously asserting its operational state via a shared memory heartbeat.
* **System Watchdog:** A secondary, isolated domain operating at a lower scheduler priority. It is strictly tasked with monitoring the primary domain's operational state and enforcing recovery protocols upon failure detection.

### 2. The Telemetry Plane (Shared Memory)
Communication between the isolated domains is facilitated through a kernel-enforced shared memory segment (`telemetry_shared`) mapped to virtual address `0x4000000`.
* **Byte 0 (Heartbeat):** Continuously written by the primary domain. A flatline (0x00) triggers immediate recovery.
* **Byte 1 (Crash Counter):** A persistent execution counter that increments upon each software reset, allowing the system to track stability metrics across failure events.

### 3. Capability Routing & Inter-Process Communication (IPC)
The system leverages seL4 Notification Channels to execute software resets securely. To maintain strict Application Binary Interface (ABI) compliance with the Microkit SDK, capability routing is handled via a native C-Wrapper (`notify.c`). This bridge ensures the Rust logic interfaces correctly with the kernel's dynamically assigned capability IDs without requiring insecure inline assembly.

## ⚙️ Build Constraints
* **Language:** Rust (`no_std`).
* **Dependencies:** Bare-metal execution only. Zero standard library dependencies.
* **Toolchain:** Compilation requires the `microkit` toolchain for final ELF synthesis and metadata patching.
