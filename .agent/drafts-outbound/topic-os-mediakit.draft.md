---
schema: foundry-draft-v1
artifact_type: TOPIC
title: "OS Mediakit"
slug: os-mediakit
language: en
category: systems
status: active
quality: review
target_path: content-wiki-documentation/systems/os-mediakit.md
bilingual_pair: topic-os-mediakit.es.draft.md
cites:
  - infrastructure-os
  - os-network-admin
  - totebox-archive
  - ppn-hypervisor-resource-pool
  - ppn-architecture-overview
  - genesis-protocol
bcsc_reviewed: false
operator_approved: false
research_sources: >
  BRIEF-totebox-transformation.md §2/§9; BRIEF-PPN-ARCHITECTURE.md §1/§9.2;
  BRIEF-PPN-DEV-BOOTSTRAP.md §12; project-system moonshot-toolkit v0.3.0 Phase 1C.d;
  seL4 Foundation Microkit 2.2.0 documentation; internet research 2026-05-29
  (Microkit target set, x86_64 seL4 status, Foundation guidance for small teams)
research_inline: true
notes_for_editor: >
  BCSC posture is critical here. The Ubuntu 24.04 vm-mediakit phase is present tense
  (proven, running). The seL4 Microkit Phase 3 is planned/intended language throughout.
  No wikilinks to os-infrastructure (its EAPOL-era content is superseded — BRIEF §9.2).
  Comparison table should make clear what changes (OS image) vs what stays (service binaries,
  wire protocols, port numbers). Ubuntu 24.04 is required (not Debian 12) because all
  host-compiled Rust binaries link against glibc 2.39; Debian 12 only provides glibc 2.36.
---

# OS Mediakit

**os-mediakit** is the guest operating system image for the `vm-mediakit` VM tier in
the PointSav Private Network hypervisor layer. It isolates the MediaKit service surface
— knowledge wikis, marketing sites, proofreader, and BIM orchestration — from the source
vault and orchestration tiers.

---

## Stack position

The four-layer Totebox stack places os-mediakit in the **Hypervisor layer**:

```
Operator
  ↓
PPN (WireGuard mesh, os-network-admin control plane)
  ↓
Hypervisor layer  ←— os-mediakit guest OS runs here
  ↓
Totebox Orchestration (app-mediakit-*, service-fs, system-core)
```

os-mediakit is one guest among three in the three-VM layout:

| VM | Guest OS | Tier |
|---|---|---|
| vm-workspace | host OS (Linux) | os-privategit (permanent host) |
| vm-intelligence | os-intelligence (planned) | os-totebox + inference |
| vm-mediakit | **os-mediakit** | os-mediakit |

The host — foundry-workspace GCP VM — runs QEMU to manage all guests. The hypervisor
itself is `os-infrastructure` (the Genesis Protocol boot layer).

---

## Phase 1: Ubuntu 24.04 interim (present)

The first deployment of vm-mediakit uses an **Ubuntu 24.04 server cloud x86_64 QCOW2** as
the guest OS. This is the production interim while the seL4 Microkit image is developed.

Ubuntu 24.04 is required — not Debian 12 — because all service binaries compiled on the
GCP host (Ubuntu 24.04, glibc 2.39) link against `GLIBC_2.39` symbols. Debian 12 provides
only glibc 2.36 and would fail to execute the binaries at load time.

What is running today:
- Ubuntu 24.04 booted via `provision-vm-mediakit.sh` under QEMU/TCG (GCP workspace has no
  hardware KVM; TCG is adequate for Phase 1 testing)
- 6 GiB RAM (`-m 6144`), 20 GB QCOW2 disk
- User-mode NAT networking: host port-forwards `1xxxx → :xxxx` for each service
- `virtio-balloon` device: dynamic RAM adjustment without guest reboot [infrastructure-os]
- cloud-init first boot: hostname `vm-mediakit`, user `foundry`, systemd-native
- nginx/1.24.0 and build-essential installed post-boot

Services running inside the Ubuntu 24.04 guest (Phase 1 state, 2026-05-29):

| Service | Port | Purpose | Phase 1 status |
|---|---|---|---|
| local-proofreader | 9092 | Proofreader service | ✓ active |
| local-knowledge-documentation | 9090 | Documentation wiki | ✓ active |
| local-knowledge-corporate | 9095 | Corporate wiki | ✓ active |
| local-knowledge-projects | 9093 | Projects wiki | ✓ active |
| local-marketing-pointsav | 9101 | PointSav marketing site | ✓ active |
| local-marketing | 9102 | Woodfine marketing site | ✓ active |
| service-fs | 9100 | WORM ledger — data ingest backbone | pending (project-data build) |
| local-bim-orchestration | 9096 | BIM gateway | pending (depends on service-fs) |
| system-core | — | Capability Ledger substrate | pending (project-system install) |
| system-ledger | — | Ledger state-machine | pending (project-system install) |

The systemd host unit `infrastructure/local-vm-mediakit/vm-mediakit.service` manages the
QEMU process and handles graceful shutdown via the QEMU monitor socket.

---

## Phase 3: seL4 Microkit image (planned)

The intended long-term form of os-mediakit is a **seL4 Microkit 2.2 AArch64 image**
assembled by `moonshot-toolkit`. Each service runs as an isolated seL4 Protection Domain
(PD) within the formally-verified microkernel.

This is a planned milestone. The seL4 path requires an AArch64 host (Microkit 2.2.0
supports AArch64 and RISC-V 64; there is no x86_64 Microkit target).

### Planned component layout

Each major service becomes a seL4 PD with minimal capability set:

| PD | Binary | seL4 capability |
|---|---|---|
| `mediakit-root` | os-mediakit rootserver | Bootstrap, capability distribution |
| `service-fs-pd` | service-fs Envelope B | IPC to ledger-pd; file-system endpoint only |
| `system-ledger-pd` | system-ledger (native feature) | seL4_Call to capability oracle |
| `proofreader-pd` | service-proofreader | HTTP endpoint; no FS capability |
| `knowledge-pd` | app-mediakit-knowledge | HTTP endpoint; read-only FS cap |
| `marketing-pd` | app-mediakit-marketing | HTTP endpoint; no FS capability |

The isolation invariant: no PD has read capability over another PD's memory. Enforced by
the seL4 capability model — not by OS-level permissions. [ppn-architecture-overview]

### The `system-substrate-sel4` shim

`system-core` and `system-ledger` are written for `std` environments (Linux daemon form).
Running them as seL4 PDs requires `system-substrate-sel4` — a shim crate with feature flags
`["native"]` (seL4_Call/seL4_Send via rust-sel4) and `["compat"]` (std wrapper for Linux).
The shim is a planned crate. service-fs ARCHITECTURE.md §Envelope B documents the same
pattern for service-fs specifically.

### Assembly

`moonshot-toolkit build os-mediakit/system-spec.toml` is the intended build command.
`system-spec.toml` declares the PDs, memory regions, and channels in a Microkit-shaped
TOML format. The output `build/system-image.bin` is bootable on any seL4-supported
AArch64 platform (qemu-arm-virt, Raspberry Pi 4, AWS Graviton).

---

## What changes vs Phase 1, what stays the same

| Property | Ubuntu 24.04 (Phase 1) | seL4 Microkit (Phase 3, planned) |
|---|---|---|
| Guest OS | Ubuntu 24.04 Linux 6.x (glibc 2.39) | seL4 microkernel + Microkit PDs |
| Host | QEMU/TCG (x86_64) | QEMU/KVM or bare metal AArch64 |
| Service binaries | Same (cross-compiled) | Same (recompiled for AArch64 no_std) |
| Wire protocols | CBOR-over-HTTP | CBOR-over-QUIC (same data schema) |
| Port numbers | Same (9090, 9092, ...) | Same (WireGuard overlay) |
| virtio-balloon | Present | Present (hypervisor layer unchanged) |
| Formal isolation | Linux kernel security model | seL4 intransitive non-interference proof |
| Key custody | OS file permissions | seL4 capability object — no OS |

---

## Relationship to os-infrastructure and Genesis Protocol

`os-infrastructure` is the hypervisor boot layer — it runs Genesis Protocol on the physical
host to establish the PPN node's WireGuard identity and claim ceremony. os-mediakit is a
*guest* that runs above os-infrastructure. They are different layers and different binaries.

The Genesis Protocol first-boot sequence [genesis-protocol] applies to the **host node**
(os-infrastructure), not to the guest (os-mediakit). A new vm-mediakit guest joins the mesh
via the MBA pairing ceremony after the host node is already a PPN member.

---

## See also

- `BRIEF-totebox-transformation.md §2/§6/§9` — three-VM layout, Part C sequencing, seL4 decision
- `BRIEF-PPN-DEV-BOOTSTRAP.md §12` — moonshot-toolkit Phase 1C.d achievement and gap analysis
- [ppn-hypervisor-resource-pool] — how virtio-balloon manages RAM for vm-mediakit
- [totebox-archive] — what the Totebox Archive tier does above the guest OS
- [os-network-admin] — the PPN control plane; vm-mediakit joins the mesh through it
