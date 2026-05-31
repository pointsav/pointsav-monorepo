---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: ppn-architecture-overview.md
audience: technical decision-makers, operators, and engineers evaluating or deploying PointSav
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/infrastructure-os.md
  - systems/os-network-admin.md
  - architecture/sovereign-mesh.md
  - architecture/genesis-protocol.md
  - architecture/ppn-command-protocol.md
  - architecture/ppn-hypervisor-resource-pool.md
  - systems/totebox-archive.md
  - systems/os-orchestration.md
notes_for_editor: >
  New topic — no existing file at this path. High-level overview of the PPN as a whole:
  four layers, three key properties, what it is and is not. Deliberately concise (~120 lines).
  This topic links to the 8 detailed TOPICs rather than duplicating their content. It is the
  entry point for readers evaluating PPN. Companion Spanish draft staged alongside.
  Article frontmatter to add on commit: title "PPN Architecture Overview",
  category "architecture", status "active", quality "review",
  cites [infrastructure-os, os-network-admin, sovereign-mesh, genesis-protocol,
         ppn-command-protocol, ppn-hypervisor-resource-pool, totebox-archive, os-orchestration].
research_done_count: 8
research_suggested_count: 0
open_questions_count: 0
---

# PPN Architecture Overview

The **PointSav Private Network (PPN)** is the physical infrastructure plane of the PointSav
stack. It is the layer responsible for: enrolling physical nodes into a cryptographically
authenticated mesh, managing the compute resources those nodes provide, and hosting the
virtual machines that run Totebox Archives and orchestration gateways. The PPN is not a
data access layer. It does not hold data. It does not make authentication decisions about
who may read an archive. It manages physical infrastructure so that the data layer can run
on top of it.

## The four layers

The PPN and the systems built on top of it are organised into four layers. Each layer is
blind to the internal state of the layers below and above it.

### Operator layer

The operator layer is where a human administrator interacts with the fleet.

**`os-network-admin`** is the Foundation OS layer — the control plane for the PPN mesh. It
runs on the operator's machine (bare metal or LXC container), manages peer-map distribution,
and enforces the Diode rules that restrict command flow. It holds zero cryptographic
authority: it cannot read archive data, it cannot approve data access, and it cannot issue
identity credentials. Its role is to know which physical nodes are on the mesh and to
enforce that membership — nothing more.

**`app-network-admin`** is the F8 Terminal interface that runs on top of `os-network-admin`.
It accepts plain-language operator intent at HTTP port 8085, routes it through `service-slm`
to produce an authorised 16-byte binary command, and broadcasts that command over UDP port
8090 to the mesh.

See: [[os-network-admin]], [[ppn-command-protocol]]

### PPN layer

The PPN layer is the physical transport and ceremony substrate.

The **[[sovereign-mesh]]** is a WireGuard cryptographic overlay on a dedicated `ppn0`
interface. Every node in the fleet holds a long-term keypair; every packet is encrypted
before leaving the node. Commands travel as 16-byte binary packets broadcast simultaneously
to all mesh peers; only the addressed node acts.

**`service-ppn-pairing`** is the ceremony backend that manages node-join requests. When a
new physical node wants to join the mesh, it generates a Crockford base32 short code (~40
bits of entropy). The operator enters this code; a CPace PAKE exchange establishes a shared
session key; a Short Authenticated String comparison closes the man-in-the-middle gap. The
approved node is written to the `nodes.jsonl` append-only registry.

The **[[genesis-protocol]]** governs first boot: a node generates its keypair from hardware
entropy, enters a sealed holding pattern, and awaits an administrative claim — without any
pre-provisioning or control-plane dependency.

### Hypervisor layer

The hypervisor layer is the compute substrate.

**`os-infrastructure`** is the Type I hypervisor that hosts the virtual machines running
Totebox Archives and orchestration gateways. It manages a **per-node resource pool**:
memory via `virtio_balloon` (inflation reclaims guest RAM into the node pool; deflation
returns it) and CPU via cgroups v2 `cpu.weight` per QEMU process.

The pool is bounded to the physical node. Cross-node workload placement is the Totebox
Orchestration layer's responsibility; once a VM is placed on a node, the hypervisor manages
its local resource allocation.

See: [[ppn-hypervisor-resource-pool]]

### Totebox Orchestration layer

The Totebox Orchestration layer is the data plane. It runs inside the VMs managed by the
hypervisor and is entirely separate from the PPN.

**Totebox Archives** (`cluster-totebox-*`) are sovereign data vaults — immutable WORM
ledgers packaged as freely transferable bootable disk images. Each archive holds an
Ed25519 keypair registered in `pairings.yaml`. It exposes data only via signed capability
objects delivered over the PointSav Protocol (PSP).

**`os-console`** is the keyboard-native operator terminal. It connects to one archive at
a time (or many via `os-orchestration`) and is the free tier.

**`os-orchestration`** is a stateless multi-archive aggregator. It fans PSP queries across
many archives simultaneously, returns only result rows (never raw records), and holds no
keys of its own. It is the paid tier: multi-archive aggregation is the commercial boundary.

See: [[totebox-archive]], [[os-orchestration]]

## Three key properties

### Isolation invariant

The hypervisor has zero read capability over VM-internal state. It knows a VM is consuming
N pages of RAM and Y percent of vCPU time. It does not know whether the VM is running
`os-totebox`, `os-orchestration`, or anything else. The Totebox Archive inside the VM knows
nothing about balloon inflation, cgroup weights, or which physical node it is on.

This means a compromise of the routing layer — the PPN — yields no access to archive
contents. The two planes are structurally blind to each other.

### Freely transferable archives

Because the hypervisor manages only VM lifecycle and resource allocation — not the data
inside VMs — a Totebox Archive can be stopped, the disk image copied to any other PPN node,
and restarted there without any change to its data or its identity. The destination node's
hypervisor pool absorbs the workload. The archive's history, keys, and state are unchanged.

The disk image is the archive. The resource pool is node infrastructure. Moving the image
moves the archive.

### Zero crypto authority at the network plane

`os-network-admin` and the PPN mesh carry no cryptographic authority over archive data.
Routing a node onto the mesh does not grant any data access. Removing a node from the mesh
does not revoke any data access. Data access is governed entirely by `pairings.yaml` and
the MBA keypair system — a separate plane with no connection to the PPN.

This separation is intentional: the network control plane and the data access plane are
designed so that a failure or compromise of one does not cascade to the other.

## What PPN is not

- **Not a data access layer.** PPN manages physical nodes. Archive data access goes through
  `os-console` or `os-orchestration` via MBA + PSP — not through WireGuard.
- **Not a compute scheduler.** `os-network-admin` does not schedule workloads across nodes.
  Cross-node placement is `gateway-orchestration-command-1`'s job (Totebox Orchestration
  layer). The hypervisor manages the local resource pool after placement.
- **Not an identity authority.** The PPN mesh knows which physical nodes are enrolled.
  It does not know which operators are authorised to read which archives. That is `pairings.yaml`.

## Related topics

- [[sovereign-mesh]] — WireGuard overlay, 16-byte binary command protocol, hub-spoke topology
- [[genesis-protocol]] — autonomous first-boot bootstrap sequence, deferred fleet assembly
- [[ppn-command-protocol]] — the 16-byte binary wire format broadcast over UDP port 8090
- [[service-pointsav-link]] — hot-pluggable adapter connecting os-* nodes to the fleet
- [[os-network-admin]] — Foundation OS layer, zero crypto authority, node-join ceremony
- [[ppn-hypervisor-resource-pool]] — per-node virtio_balloon + vCPU scheduling
- [[totebox-archive]] — sovereign WORM data vault, freely transferable disk image
- [[os-orchestration]] — stateless PSP aggregator, multi-archive queries, paid tier
