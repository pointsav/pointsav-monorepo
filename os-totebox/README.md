# os-totebox
### Sovereign Data Archive & Federated Cluster

**Status: Active Engineering** | **Taxonomy: Tier-3 (Platform Layer)**

This component is the fundamental unit of the PointSav architecture. It is an isolated, highly secure container designed to hold specific organizational data. Because of its "Cloud-Blind" design, an `os-totebox` can be safely deployed on private servers or public cloud infrastructure (AWS/GCP) without compromising data sovereignty.

## Architectural Mandate & Partitioning

### Microkernel Partitions (Isolated Protection Domains)
An `os-totebox` operates as a Federated Cluster. Under the seL4 microkernel, each service (e.g., `service-study`, `service-people`, `service-content`) is executed within its own strictly isolated cryptographic partition. This prevents cross-domain memory access, ensuring that a vulnerability in one active engine cannot compromise the passive data vaults.

### The Payload-Agnostic Flat-File Ledger (SYS-ADR-12)
Passive vaults (e.g., `service-study`) reject opaque SQL databases. Original assets are stored as inert binaries in `/assets/`, while deterministic metadata and cryptographic checksums are stored as `.yaml` pointers in `/ledger/`. Execution permissions are stripped at the kernel level.

### The Relational Exception (SYS-ADR-13)
Vaults managing highly mutable PII (e.g., `service-people`) operate inside a dedicated partition housing a Zero-Trust relational database, physically isolated from the flat-file ledgers.

---
*© 2026 PointSav Digital Systems™*
