---
artifact: foundry-draft-v1
type: TOPIC
slug: topic-ppn-three-path-architecture
title: "PPN Three-Path seL4 Architecture — Option B, C, and A"
status: draft
created: 2026-06-29
author: totebox@project-infrastructure
route_to: project-editorial
language_protocol: PROSE-*
research_source: seL4-architecture-research-2026-06-29
research_claim: "Three sequential seL4 architecture options, each formally defined with gate conditions"
research_method: seL4 Foundation Microkit 2.2.0 docs + CAmkES VMM docs + libvmm release notes
research_verification: cross-checked against seL4.systems + GitHub release notes + UK NCSC announcement
language: en
---

# PPN Three-Path seL4 Architecture

The PointSav Private Network node stack is designed around the seL4 microkernel. Three
architecture options exist. They are sequential — Option B ships first, then Option C
gates on Option B, then Option A gates on Option C. All three are documented at the start
so that decisions made in Option B do not foreclose Options C and A.

---

## Option B — Current Path (seL4 + CAmkES VMM + Linux guest)

**Status:** Active development. `os-infrastructure` v0.0.1 in progress.

**What it is:**
seL4 runs as a Type 1 hypervisor at EL2 (AArch64) or with VT-x (x86-64). The CAmkES
VMM runs as a seL4 protection domain. The CAmkES VMM hosts one Linux (Debian 12) guest
VM. All PPN services (WireGuard, fleet management, inference routing) run inside the
Linux guest.

```
Hardware
└── seL4 microkernel (EL2 / VT-x)
    └── CAmkES VMM [seL4 PD]
        └── Linux (Debian 12) guest
            ├── WireGuard mesh interface
            ├── service-vm-fleet
            ├── service-vm-host
            └── os-network-admin
```

**What seL4 provides:** The kernel is formally verified. No guest VM can access the
hypervisor layer, another guest's memory, or any seL4 kernel object without an explicit
seL4 capability grant. On AArch64 EL2, the integrity proof (April 2025, UK NCSC funded)
extends this to a machine-checked guarantee.

**What seL4 does NOT provide in Option B:** The CAmkES VMM and Linux guest are not
formally verified. A compromised Linux guest remains confined by seL4's capability
topology, but the services inside the guest are trusted relative to each other by Linux's
normal process model.

**Formal security coverage:**
- AArch64 EL2: seL4 integrity proof (April 2025). Valid formal security claim at the
  hypervisor boundary.
- x86-64: seL4 functional correctness only. No formal integrity proof. Runtime and
  development target only — no "topology determines security" claim.

**Three-artifact distribution per software.pointsav.com:**
- `.iso` — bare metal (GRUB2 multiboot, write to USB and boot)
- `.qcow2` — cloud VM import (GCP raw import, DigitalOcean, etc.)
- Not applicable for daemon distribution — see `os-network-admin` for daemon mode.

---

## Option C — Moonshot: seL4 PDs Own WireGuard (moonshot-sel4-vmm)

**Status:** Planned/intended. Gate: Option B ships + ≥6 months stable.

**What it is:**
Hybrid architecture. seL4 protection domains own WireGuard and the PPN network control
plane. A Linux guest VM hosts non-critical workloads. The security boundary is the
WireGuard PD — the Linux VM cannot modify peer tables without going through seL4 IPC.

```
seL4 microkernel (EL2 / VT-x)
├── PD: wireguard-control  [WireGuard peer table, CPace gating]
├── PD: ppn-gate           [seL4 IPC channel enforcement]
└── CAmkES VMM
    └── Linux guest (Doorman, fleet, telemetry, media)
```

**Why:** The WireGuard mesh interface is the highest-security component. By moving it
to a seL4 PD, the Linux VM can be fully compromised without the attacker gaining access
to peer tables or the ability to add/remove nodes. Peer addition still requires going
through seL4 IPC to the wireguard-control PD, which validates the request.

**What changes from Option B:**
- `os-network-admin`'s WireGuard management ported from Linux to a seL4 PD.
- Linux guest has no `wg0` interface.
- Peer addition requests flow: Linux VM → seL4 IPC → wireguard-control PD → kernel state.

---

## Option A — Moonshot: Pure seL4 PDs, No VMs (moonshot-hypervisor)

**Status:** Planned/intended. Gate: Option C ships + ≥6 months stable.

**What it is:**
No virtual machines. No VMM. Every PPN service is a seL4 protection domain. WireGuard
is ported to a seL4 PD (no Linux socket layer, no net_device API). Fleet management and
service routing are PDs. Smallest possible trusted computing base (TCB).

```
seL4 microkernel (EL2 — AArch64 only for formal claims)
├── PD: wireguard     [WireGuard crypto + peer table, seL4 IPC only]
├── PD: fleet-tracker [service-vm-fleet, no tokio, no std net]
├── PD: doorman       [inference routing, seL4 IPC-based]
└── PD: ppn-control   [os-network-admin, CPace ceremony]
```

**Why:** Eliminates the Linux TCB entirely. Any seL4-to-Linux boundary is a trust boundary;
removing Linux removes the largest unverified component from the stack.

**Verification target:** AArch64 EL2 with confidentiality proof (in progress as of
2026-06-29). Once the confidentiality proof is published, Option A on AArch64 achieves
all CIA properties at the kernel layer — the deepest security posture of any PPN option.

**Gate:** Option C ships and proves the seL4 PD model for network control planes.

---

## Microkit 2.2.0 — Architecture Targets

Microkit 2.2.0 (the seL4 Foundation's recommended framework for new seL4 projects)
supports these hardware targets as of 2026-06-29:

| Target | Architecture | VT-x required? | Verification |
|---|---|---|---|
| `aarch64` | AArch64 | No | Integrity proof (Apr 2025) |
| `x86_64_generic` | x86-64 | No (TCG/software) | Functional correctness |
| `x86_64_generic_vtx` | x86-64 | Yes (VT-x/AMD-V) | Functional correctness |

x86-64 support was added in Microkit 2.1.0 (November 26, 2025). Prior assumption that
Microkit was AArch64/RISC-V only has been superseded.

---

## Summary table

| Option | seL4 as | Linux | TCB | Verification | Status |
|---|---|---|---|---|---|
| B (current) | Hypervisor | Guest VM | seL4 + VMM + Linux | AArch64 integrity | Active |
| C (moonshot) | Hypervisor + WG PD | Guest VM (limited) | seL4 + VMM + WG PD + Linux | AArch64 integrity | Planned/intended |
| A (moonshot) | OS (all components) | None | seL4 only | AArch64 all CIA (in progress) | Planned/intended |
