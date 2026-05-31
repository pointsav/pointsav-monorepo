---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: ppn-hypervisor-resource-pool.md
audience: technical operators and engineers understanding PPN resource management
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/infrastructure-os.md
  - systems/os-network-admin.md
  - systems/totebox-archive.md
  - systems/os-orchestration.md
notes_for_editor: >
  New topic — no existing file at this path. Explains the per-node resource pool model
  for the PPN hypervisor layer. Companion Spanish draft staged alongside.
  Key distinction to preserve: PPN pools CPU/RAM per physical node (hypervisor concern);
  os-orchestration pools data access across Totebox Archives (data-layer concern).
  These are orthogonal. balloon controller implementation is future milestone —
  use planned/intended language (BCSC posture).
  Article frontmatter to add on commit: title "PPN Hypervisor Resource Pool",
  category "architecture", status "active", quality "review",
  cites [infrastructure-os, os-network-admin, totebox-archive, os-orchestration].
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
---

# PPN Hypervisor Resource Pool

The PointSav Private Network (PPN) hypervisor layer manages a per-node pool of CPU
and RAM, dynamically allocating those resources across the virtual machines it runs.
This is the mechanism by which the PPN gives more or less compute capacity to each
Totebox Archive VM in response to workload demand.

## One pool per physical node

Each physical PPN node — a GCP instance, an on-premises server, a leased machine —
controls a pool bounded by its own hardware. The pool is not shared across nodes. A
node with 31 GB of RAM manages 31 GB; it does not borrow from a neighbouring node.

Cross-node workload placement is a separate concern: the Totebox Orchestration Layer
(`gateway-orchestration-command-1`) decides which physical node a cluster-totebox
instance runs on, based on MBA pairing and available capacity signals. Once that
decision is made, the receiving node's hypervisor manages the local resource pool for
that VM. The PPN pool and the Totebox scheduler are orthogonal.

## Memory pool: virtio_balloon

The primary memory reclaim mechanism is the `virtio_balloon` paravirtual device. Every
VM provisioned by `os-infrastructure` is started with a balloon driver, which runs as
a standard kernel module inside the guest operating system.

**How inflation works (reclaiming memory):**

1. The hypervisor (balloon controller) signals the balloon driver to inflate by N pages.
2. The driver allocates those pages inside the guest, removing them from the guest's
   usable address space.
3. The hypervisor recovers those physical pages for the node-level pool.
4. The pool grows by N pages; the guest's available RAM shrinks by N pages.

**How deflation works (giving memory back):**

1. The hypervisor signals the balloon driver to deflate.
2. The driver releases balloon pages back into the guest's free list.
3. The guest's available RAM grows; the pool shrinks.

**The pool at any instant:**

```
pool_available = physical_ram − Σ(balloon_minimum across all VMs)
```

Each VM has a minimum balloon reservation below which the controller will not inflate.
This prevents a VM from being starved of memory when the node is under pressure.

## CPU pool: vCPU scheduling weights

CPU pool management uses the Linux cgroups v2 `cpu.weight` interface. Each QEMU process
(one per VM) is placed in a cgroup with a weight drawn from the capability ledger. Under
CPU contention, the scheduler distributes vCPU time proportionally to those weights.
When the node is not under contention, all VMs run at full speed regardless of weight.

A cluster-totebox VM running an active inference workload (via `service-slm`) can be
assigned a higher weight than an idle archive VM. The ledger entry is the authoritative
weight; `os-infrastructure` applies it at VM launch and can adjust it live.

## Relationship to os-orchestration

`os-orchestration` is a data-layer aggregator. It aggregates **data access** across
Totebox Archives using the PointSav Protocol (PSP) — capability-based queries that
return only result rows, never raw records. It is stateless and holds no keys to archives.

`os-orchestration` does not allocate CPU. It does not adjust memory. It does not
communicate with the hypervisor balloon controller. The two layers are designed to be
blind to each other:

- The hypervisor knows a VM is consuming N pages and Y vCPU percent. It does not know
  whether the VM is running `os-totebox`, `os-orchestration`, or anything else.
- The Totebox Archive inside the VM knows nothing about balloon inflation, cgroup weights,
  or which physical node it is on.

This is the **isolation invariant** from BRIEF-PPN-ARCHITECTURE.md §1.1 Contribution #2:
the hypervisor has zero read capability over VM-internal state.

## Freely transferable archives

Because the hypervisor manages only VM lifecycle and resource allocation — not the data
inside the VMs — a Totebox Archive can be stopped, the disk image copied to another node,
and restarted there without any change to its data or its identity. The destination node's
hypervisor will allocate resources from its own pool for the relocated VM.

This is the **freely transferable** property of Totebox Archives: the bootable disk image
is the archive; the resource pool is the node's infrastructure. Moving the image moves the
archive. The new node's pool absorbs the workload.

## Implementation status

The `virtio_balloon` device flag is available in QEMU 7.x and in the NetBSD/bhyve
`virtio_balloon` module. Adding `-device virtio-balloon` to the VM launch command installs
the balloon driver in the guest.

The balloon **controller** — the component inside `os-infrastructure` that decides when to
inflate or deflate each VM's balloon in response to demand signals — is a planned milestone.
Until the controller is implemented, operators can exercise the mechanism manually via the
QEMU monitor:

```
(qemu) info balloon      # show current guest-visible RAM
(qemu) balloon 128       # request guest to give back memory down to 128 MB
(qemu) info balloon      # confirm reclaim
```

The `infrastructure/virt/vm-prove.sh` script includes `-device virtio-balloon` so that
the balloon driver is present in the test VM from the first boot.

## Related topics

- **Infrastructure OS** — the Type I hypervisor that implements the balloon controller
- **Totebox Archive** — the sovereign data vault running inside each VM
- **OS Orchestration** — the stateless data aggregator (separate from the resource pool)
- **Sovereign Mesh** — the WireGuard transport layer connecting PPN nodes
