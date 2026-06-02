---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: systems/
target_filename: os-infrastructure-ppn-node.md
audience: operators and system integrators deploying PointSav Private Network nodes
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-29
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/vm-architecture.md
  - architecture/ppn-architecture-overview.md
  - architecture/genesis-protocol.md
  - architecture/ppn-hypervisor-resource-pool.md
notes_for_editor: >
  New topic — no existing file at this path. Companion Spanish draft staged alongside
  as topic-os-infrastructure-ppn-node.es.draft.md.
  os-infrastructure is the deepest technical os-* type; this article covers it exclusively.
  Distinct from topic-vm-architecture (which surveys all five VM types) — this one goes
  deep on os-infrastructure alone.
  Phase 1 (Ubuntu 24.04) is present tense — it is operational.
  Phase 2 (NetBSD/NVMM) and Phase 3 (seL4) use planned/intended language (BCSC posture).
  The 7-PD seL4 architecture is planned/intended — do not present as shipped.
  Resource targets (8 MB disk / 12 MB RAM idle) are Phase 3 intended targets, not current.
  Genesis Protocol CPace PAKE and short-code ceremony are present-tense architecture
  (the ceremony protocol is implemented; the seL4 pd-genesis PD is Phase 3 planned).
research_done_count: 6
research_suggested_count: 0
open_questions_count: 1
open_questions:
  - "AArch64 hardware acquisition decision pending — no timeline to include in article"
---

# os-infrastructure — PPN Node Operating System

`os-infrastructure` is the operating system layer for PointSav Private Network nodes.
It is not a general-purpose OS. Its sole purpose is to set up, operate, and maintain a
node in a PointSav Private Network: managing WireGuard tunnels, hosting guest virtual
machines for other platform services, and exposing the operator control plane.

---

## What os-infrastructure Is

An `os-infrastructure` node is the physical or virtual host that anchors a PointSav
Private Network. Every node in the mesh runs `os-infrastructure` as its host OS.
The three nodes in a typical deployment — a cloud instance and two on-premises machines —
each run an independent instance of `os-infrastructure`. They communicate exclusively
over WireGuard tunnels; there is no shared network fabric, no cluster control plane, and
no VLAN dependency.

`os-infrastructure` manages:

- **WireGuard tunnels** to all mesh peers
- **Guest virtual machines** for the platform services that process data (VM-MediaKit,
  VM-Totebox, VM-Orchestration, VM-PrivateGit)
- **The Genesis Protocol ceremony** for adding new nodes to the mesh
- **The VM resource pool** — tracking available CPU and RAM across all nodes and
  dispatching VM creation requests to the node with sufficient headroom

`os-infrastructure` does not store user data. Archives, media files, and database records
live inside guest virtual machines managed by os-totebox, os-mediakit, or os-privategit.

---

## Phase 1 — Ubuntu 24.04

The current operational deployment runs Ubuntu 24.04 as the host OS. QEMU provides the
hypervisor; on hardware with Intel VT-x or AMD-V extensions, QEMU runs KVM-accelerated
guests. On GCP virtual instances where nested virtualisation is not enabled, QEMU falls
back to TCG software emulation.

WireGuard is in-kernel on Ubuntu 24.04 (kernel 5.6+). The three-node mesh uses a
10.8.0.0/24 address range with Laptop B as the routing hub.

Systemd manages all platform services. Each guest VM runs as a systemd unit wrapping
a QEMU process with a UNIX monitor socket for control operations.

The VM resource pool in Phase 1 consists of two services. `service-vm-host` runs on
each node and sends a heartbeat to the fleet controller every ten seconds, reporting
available RAM and CPU load. `service-vm-fleet` runs on the GCP node and receives those
heartbeats; when an operator requests a new VM through the `app-network-admin` interface,
the fleet controller selects the node with the most available RAM above a safety margin
and dispatches the creation request.

---

## Phase 2 — NetBSD 11.0 + NVMM (planned)

Phase 2 is intended to replace the Ubuntu 24.04 host with NetBSD 11.0, a BSD-licensed
operating system with a stronger security posture for production PPN node operation.

NetBSD 11.0 ships NVMM (NetBSD Virtual Machine Monitor), a bare-metal hypervisor
mainline since NetBSD 9.0 that uses Intel VT-x EPT for hardware isolation. QEMU runs
guests using the `-accel nvmm` flag. A single Phase 2 node is intended to host 128
virtual machines across 256 vCPU capacity.

NetBSD 11.0 also ships in-kernel `wg(4)` WireGuard, eliminating the userspace WireGuard
dependency of Phase 1. `securelevel=2` locks the running kernel against modification.
VeriExec validates load-time binary integrity against a signed manifest.

Planned resource target: 120 MB disk, 48 MB RAM idle.

---

## Phase 3 — seL4 + Microkit 2.x (intended)

Phase 3 is intended to replace the NetBSD host with a formally verified microkernel
built on seL4 v15 and Microkit 2.x on AArch64 hardware.

The seL4 kernel itself is 162 KiB of machine-checked binary. Its formal proof establishes
intransitive non-interference: a compromised guest cannot read or write the state of any
other protection domain without an explicit capability grant. This is a stronger isolation
claim than any hypervisor with an unverified TCB.

The Phase 3 `os-infrastructure` is intended to run as seven seL4 protection domains:

| Protection domain | Role |
|---|---|
| `pd-genesis` | CPace PAKE handshake; generates Crockford base32 short-code for operator verification; reaped after the pairing ceremony completes (cap revocation) |
| `pd-ledger` | Ed25519 WORM capability ledger; append-only; signs all capability grants |
| `pd-wireguard` | BoringTun `no_std` WireGuard implementation; runs inside seL4 with no libc dependency |
| `pd-net-driver` | NIC MMIO and IRQ handling; provides the network capability to `pd-wireguard` |
| `pd-vmm` | Guest VM monitor using `libsel4vm`; manages VMs for other os-* types |
| `pd-fleet` | Heartbeat client to the resource pool fleet controller |
| `pd-network-admin` | F8 TUI surface; receives UDP signed broadcasts; F12-gated configuration commits |

`pd-genesis` is reaped after the node-join ceremony is complete. The capability it held
during the ceremony is revoked and cannot be reconstructed — there is no back-door to the
pairing flow after it closes.

Phase 3 requires AArch64 hardware. Microkit 2.x includes an `x86_64_generic_vtx` target,
but x86-64 Microkit restricts each guest to one vCPU and requires Intel VT-x. For
production deployments with multi-vCPU guests, AArch64 is the intended platform.

Intended resource target: 8 MB disk, 12 MB RAM idle.

---

## Genesis Protocol

The Genesis Protocol is the node-join ceremony that adds a new node to the mesh.

An operator initiating a new node starts `service-ppn-pairing` on the node being added.
The service performs a CPace PAKE handshake and presents a Crockford base32 short-code
on the console — typically six to ten characters. The operator reads this code and enters
it in the `app-network-admin` F11 approval panel on the administering machine.

Once the codes match, the pairing establishes mutual WireGuard peer records on both
nodes, adds an entry to `nodes.jsonl` in the capability ledger, and terminates `service-ppn-pairing`.
The ceremony window is 600 seconds; if the operator does not complete approval within
that window, the code expires and the ceremony must restart from the beginning.

No keys are transmitted over the network. The short-code comparison is the sole
authentication mechanism — the operator is the root of trust, not a certificate authority.

---

## Resource Targets

| Phase | Disk | RAM idle | RAM loaded |
|---|---|---|---|
| Phase 1 (Ubuntu 24.04, now) | ~1.5 GB (OS + services) | ~400 MB | ~800 MB |
| Phase 2 (NetBSD/NVMM, planned) | 120 MB | 48 MB | 200 MB |
| Phase 3 (seL4+Microkit, intended) | **8 MB** | **12 MB** | 48 MB |

Phase 3 targets are intended to make any operator's spare hardware outperform cloud
provider minimum VM tiers: AWS Lambda minimum is 128 MB RAM; the Phase 3 target is
12 MB at idle — more than ten times lighter.
