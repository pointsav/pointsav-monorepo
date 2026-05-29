---
artifact: brief
name: BRIEF-OS-FAMILY
status: active
created: 2026-05-29
engine: claude-code
session: totebox@project-infrastructure
synthesises:
  - BRIEF-VM-ARCHITECTURE.md
  - BRIEF-PPN-ARCHITECTURE.md
  - BRIEF-LEAPFROG-2030.md
  - session-context.md (sessions 8–12)
description: >
  Single consolidated reference for all five os-* types. Any agent starting work on
  os-*, system-*, or service-* crates should read this first. One section per os-*:
  purpose, hosted services, placement principle, Phase 1 stack, Phase 2 target, Phase 3
  target, resource targets, and blocking decisions.
---

# os-* Family Reference

**Read this before starting work on any os-*, system-*, or service-* crate.**

Each os-* type maps 1:1 to a VM-* deployment type. The placement principle for every
service: *the service belongs in the VM whose os-* namespace owns its data lifecycle.*

Cross-references:
- VM-to-os mapping: `BRIEF-VM-ARCHITECTURE.md`
- Resource targets detail: `BRIEF-LEAPFROG-2030.md`
- PPN fabric: `BRIEF-PPN-ARCHITECTURE.md`
- Bootstrap sequence: `BRIEF-PPN-DEV-BOOTSTRAP.md`

---

## os-infrastructure

**Purpose:** The PPN node OS. Network fabric and hypervisor substrate. NOT a
general-purpose OS — its sole purpose is to set up and act as a node in a PointSav
Private Network. Manages the WireGuard mesh, hosts other VM-* instances, and exposes
the F8/F9 operator surface via app-network-admin.

**Hosted services:**
- `service-ppn-pairing` (:9202) — Genesis Protocol node-join ceremony
- `service-vm-fleet` (:9203, GCP-resident) — fleet controller for resource pooling
- `service-vm-host` (per-node, outbound-only) — heartbeat agent
- `app-network-admin` (F8 terminal; F9 ratatui panel)

**VM-* counterpart:** VM-Infrastructure

**Data lifecycle:** Network configuration (WireGuard peers, routes, ceremony records).
No Totebox archive data. No user content.

**Phase 1 (now — Ubuntu 24.04):**
- QEMU/TCG (GCP, nested KVM not enabled) or QEMU/KVM (Laptop A/B)
- In-kernel WireGuard
- Systemd service management
- service-ppn-pairing for Genesis Protocol ceremony
- service-vm-host heartbeat to service-vm-fleet

**Phase 2 (planned — NetBSD 11.0 + NVMM):**
- NVMM hypervisor (`qemu -accel nvmm`); VT-x EPT isolation
- In-kernel `wg(4)` WireGuard (NetBSD 11.0)
- `securelevel=2` read-only kernel; VeriExec load-time integrity
- MICROVM kernel config (NetBSD 11.0 RC4+): minimal drivers, fast boot
- Capacity: 128 VMs × 256 vCPU × 128 GB RAM per host
- Target: 120 MB disk / 48 MB RAM idle

**Phase 3 (intended — seL4 + Microkit 2.2.0, AArch64):**
- 7 protection domains (PDs):
  - `pd-genesis` — CPace PAKE; generates Crockford base32 short-code; reaped after pairing (cap revocation)
  - `pd-ledger` — Ed25519 WORM capability ledger; append-only
  - `pd-wireguard` — BoringTun `no_std` WireGuard implementation
  - `pd-net-driver` — NIC MMIO+IRQ; virtio or native NIC
  - `pd-vmm` — libsel4vm for hosting VM-* guests
  - `pd-fleet` — heartbeat client to service-vm-fleet
  - `pd-network-admin` — F8 TUI; receives UDP signed broadcasts; F12-gated config commits
- Machine-checked intransitive non-interference (Murray et al. 2013)
- **AArch64 hardware required** — operator decision pending
- Target: 8 MB disk / 12 MB RAM idle / 48 MB RAM loaded

**Blocking decisions:**
- GCP nested KVM enablement (operator: GCP console) — unblocks KVM on GCP
- Laptop A `ls /dev/kvm` confirmation (operator) — confirm KVM available
- AArch64 hardware acquisition — gates Phase 3

---

## os-totebox

**Purpose:** Sovereign WORM data vault. Archives are flat-file JSONL/GeoParquet/Markdown
structures. The disk image IS the archive — freely transferable, no vendor lock-in.
All writes are append-only. Diode + PSP are the only access protocols.

**Hosted services:**
- `service-fs` (:9100) — filesystem vault daemon; WORM append-only
- `service-people` (:9200) — identity and personnel records
- `service-email` (:9204) — email archive
- `service-input` — structured input ingestion

**VM-* counterpart:** VM-Totebox

**Data lifecycle:** Owner of all Totebox archive data. WORM constraint means VMs cannot
be live-migrated — `preferred_node` must be operator-specified in service-vm-fleet.

**Phase 1 (now — Ubuntu 24.04):**
- service-fs ACTIVE (127.0.0.1:9100, 30 tests passing)
- service-people ACTIVE (Master binary deploy pending)
- service-email ACTIVE (127.0.0.1:9204)
- VM-Totebox Phase 1 BLOCKED: project-data 33 commits pending Stage 6 promotion

**Phase 2 (planned — NetBSD 11.0 + FreeBSD-style jails):**
- One jail per service; VeriExec per binary; in-kernel `wg(4)`
- Target: 80 MB disk / 64 MB RAM idle

**Phase 3 (intended — Nanos/OPS or seL4+Microkit):**
- One unikernel image per service; WORM archive on host filesystem
- Target: 16 MB disk / 24 MB RAM idle / 96 MB RAM loaded

**Blocking decisions:**
- Stage 6 promotion of project-data's 33 commits (Command Session action)
- service-people Master binary deploy

---

## os-mediakit

**Purpose:** Knowledge and content publishing layer. Hosts MediaWiki-based knowledge
bases (documentation, corporate, projects), marketing services, proofreader, and
BIM orchestration.

**Hosted services:**
- `local-proofreader` (:9092)
- `local-knowledge-documentation` (:9090)
- `local-knowledge-corporate` (:9095)
- `local-knowledge-projects` (:9093)
- `local-marketing-pointsav` (:9101)
- `local-marketing` (:9102)
- `bim-orchestration` (:9096) — BLOCKED on service-fs in VM-Totebox

**VM-* counterpart:** VM-MediaKit

**Data lifecycle:** Media content, wiki articles, knowledge base entries. Does NOT own
Totebox archive data.

**Phase 1 (now — Ubuntu 24.04, COMPLETE 6/6):**
- All 6 unblocked services migrated and smoke-tested ✓
- bim-orchestration blocked on VM-Totebox service-fs (blocked separately)
- Port forwards: 10022/19090/19092/19093/19095/19096/19100/19101/19102
- glibc 2.39 (Ubuntu 24.04) required — Debian 12 (2.36) would segfault

**Phase 2 (planned — FreeBSD jails):**
- One jail per PHP/Rust workload; MediaWiki stays on PHP 8.x
- Target: 600 MB disk / 180 MB RAM idle (constrained by MediaWiki/PHP)

**Phase 3 (intended — Unikraft+Rust, contingent on app decision):**
- Requires replacing MediaWiki/PHP with Rust static renderer
- If MediaWiki retained: P3 target unachievable (PHP ~30 MB alone)
- Target (if app decision made): 24 MB disk / 32 MB RAM idle

**Blocking decisions:**
- VM-Totebox service-fs (for bim-orchestration Phase 1 completion)
- Application decision: retire MediaWiki/PHP for Rust renderer (Phase 3 gate)

---

## os-orchestration

**Purpose:** Stateless aggregation layer. Paid tier. Multi-Totebox data aggregation,
SLM/AI inference, GIS processing, BIM aggregation. Holds no archive keys — all data
flows through PSP capability grants from os-totebox.

**Hosted services:**
- `app-orchestration-slm` (:9180) — commercial Yo-Yo SLM broker
- `app-orchestration-bim` — multi-archive BIM queries
- `app-orchestration-gis` — continental-scale GIS processing
- `app-orchestration-exchange` — gateway for ad campaigns

**VM-* counterpart:** VM-Orchestration

**Data lifecycle:** Stateless aggregation — no persistent data ownership. Computes over
capability-granted views of Totebox archives. The commercial boundary: Orchestration
charges for multi-Totebox aggregation; PPN resource pooling (service-vm-fleet) is free.

**Phase 1 (not yet started):**
- VM-Orchestration provision script (`provision-vm-orchestration.sh`) stub ready
- BLOCKED on VM-Totebox service-fs for bim-orchestration

**Phase 2 (planned — NetBSD 11.0 + NVMM + gVisor):**
- gVisor sandboxing for untrusted aggregator workloads
- NVMM for sub-VMs; in-kernel `wg(4)`
- Target: 96 MB disk / 96 MB RAM idle

**Phase 3 (intended — Unikraft+Rust):**
- NanoVMs for SLM/GPU inference (fat Linux retained for GPU — GPU driver incompatible with unikernel)
- Target: 12 MB disk / 18 MB RAM idle (non-GPU workloads only)

**Blocking decisions:**
- VM-Totebox service-fs (blocks bim-orchestration VM-Orchestration Phase 1)

---

## os-privategit

**Purpose:** Private Git hosting for source control, design system, and customer guide catalogs.
Hosts Gitea and Storybook; SSH forwarding; serves pointsav-monorepo cluster branches.

**Hosted services:**
- Gitea — private Git server
- `app-privategit-design-system` (Storybook)
- `app-privategit-source-control`

**VM-* counterpart:** VM-PrivateGit

**Data lifecycle:** Git repository objects (content-addressed, immutable blocks).
Architecturally append-only — suitable for WORM-adjacent storage.

**Phase 1 (not yet started):**
- `provision-vm-privategit.sh` stub ready
- No specific blockers — can proceed when VM-Infrastructure Phase 1 is stable

**Phase 2 (planned — FreeBSD + jail):**
- One jail around Gitea; VeriExec per binary
- Target: 180 MB disk / 96 MB RAM idle

**Phase 3 (intended — Unikraft+Rust, contingent on app decision):**
- Requires gitoxide-based Rust server to replace Gitea/Go
- If Gitea retained: P3 target unachievable (Go runtime ~20 MB)
- Target (if app decision made): 20 MB disk / 24 MB RAM idle

**Blocking decisions:**
- Application decision: retire Gitea/Go for gitoxide-based server (Phase 3 gate)

---

## Cross-Cutting Engineering Discipline

### Rust binary size (all new crates)
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### Tokio runtime (all new service-* daemons)
```rust
#[tokio::main(flavor = "current_thread")]
async fn main() { ... }
```
Exception: service-fs uses multi-thread for concurrent append serialization.

### Current-thread discipline rationale
Multi-threaded Tokio: ~30–40 MB RSS floor per process (thread per vCPU + work-stealing).
current_thread: ~4–8 MB RSS floor. For Phase 3 targets of 12–24 MB idle, every daemon
must be current_thread or the targets are unreachable.

---

*Created: 2026-05-29 | project-infrastructure session 12 | claude-code (Sonnet 4.6)*
*This is the authoritative per-os-* reference. Update when phase status changes.*
