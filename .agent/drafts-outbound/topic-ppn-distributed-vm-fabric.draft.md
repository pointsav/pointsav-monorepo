---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: ppn-distributed-vm-fabric.md
audience: architects and engineers evaluating sovereign distributed compute infrastructure
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - architecture/ppn-architecture-overview.md
  - architecture/ppn-hypervisor-resource-pool.md
  - architecture/sovereign-mesh.md
  - architecture/genesis-protocol.md
  - systems/infrastructure-os.md
  - systems/os-orchestration.md
notes_for_editor: >
  New topic — no existing file at this path. Companion Spanish draft staged alongside
  as topic-ppn-distributed-vm-fabric.es.draft.md.
  All four distributed components are PLANNED/INTENDED — not yet built. Strict BCSC
  posture: only the per-node layer (virtio_balloon, cgroups v2, proven 2026-05-28) uses
  present tense. Everything else uses planned/intended/target language.
  The comparison table in §6 describes what others have today (factual, present tense is
  fine) and what PPN plans to have (intended language). No BCSC qualification needed for
  the competitor-description column.
  Article frontmatter to add on commit: title "PPN Distributed VM Fabric",
  category "architecture", status "active", quality "review",
  cites [ppn-architecture-overview, ppn-hypervisor-resource-pool, sovereign-mesh,
         genesis-protocol, infrastructure-os, os-orchestration].
research_done_count: 7
research_suggested_count: 0
open_questions_count: 0
---

# PPN Distributed VM Fabric

The **PPN Distributed VM Fabric** is the planned extension of the per-node PPN hypervisor
layer to a multi-node resource pool. Where the current hypervisor layer manages CPU and
RAM within a single physical node, the distributed fabric is intended to allow VMs to
borrow compute from other nodes in the mesh and to place and migrate VMs across the fleet
without per-move operator involvement.

This topic describes the planned architecture. The per-node layer — virtio_balloon,
cgroups v2 weights, and the vm-prove.sh proof — is implemented and proven as of
2026-05-28. The four distributed components described below are planned milestones; none
is built yet.

## Current state: per-node only

The implemented hypervisor layer allocates CPU and RAM within one physical node. The pool
is bounded by that node's hardware. Expanding a VM's memory means taking from the same
node's free pool. Placing a VM on a different node requires stopping it, transferring the
disk image, and restarting — a manual operation the Totebox Orchestration layer is
intended to automate.

The `virtio_balloon` mechanism proven in `infrastructure/virt/vm-prove.sh` operates
entirely within a single node. No network communication is involved. No reboot of host or
guest is required for balloon operations — inflation, deflation, and cgroups v2 weight
changes are all dynamic adjustments to a running system.

## Component 1 — Virtio-mem lending over WireGuard (planned)

`virtio-mem` (upstream Linux kernel since 5.8; QEMU since 5.1) supports hot-plug and
hot-unplug of individual memory blocks in a running guest. Unlike `virtio_balloon` —
which inflates a single device to reclaim pages — `virtio-mem` exposes a set of granular
blocks that the guest kernel can accept or return one at a time. A VM can grow beyond its
initial allocation without a restart.

The intended cross-node extension:

1. A node with surplus RAM advertises available `virtio-mem` blocks over the WireGuard
   mesh
2. A VM on a different node that needs more RAM receives those blocks as a hot-plugged
   device
3. The seL4 capability model on the lending node is intended to ensure no read capability
   is retained over the lent blocks — the physical pages are mapped exclusively into the
   borrowing VM's address space
4. When the lending period ends, the blocks are hot-unplugged and returned to the lending
   node's pool

This approach differs from CXL 3.0 (Compute Express Link), which provides
hardware-coherent memory sharing over PCIe fabric. CXL requires physical proximity — it
does not work over a WAN or internet link. The WireGuard-based lending approach is
intended to work over any network, including the existing encrypted mesh, at the cost of
higher latency than PCIe.

## Component 2 — Distributed capability ledger (planned; moonshot-protocol, moonshot-database)

The seL4 capability model operates within a single address space: capabilities are
unforgeable handles that mediate all object access, but they do not cross machine
boundaries natively. The distributed extension requires a capability protocol that works
across nodes.

The intended design:

- **Capability tokens** are HMAC-signed grants issued by a lending node, keyed to the
  node's pairing-ceremony identity (established via CPace PAKE at join time). Each token
  is intended to encode: `{grantee_node, resource_type, resource_id, expires,
  sequence_number}`.
- **Revocation** is intended to be a signed revocation record appended to a Merkle DAG.
  Every node would maintain a local copy of the DAG. Nodes would gossip delta records over
  the WireGuard mesh; a revocation is intended to reach all peers in sub-second time on a
  LAN mesh.
- No central revocation authority is required. The pairing ceremony establishes trust
  roots; subsequent grants and revocations are intended to flow peer-to-peer.

The `moonshot-protocol` and `moonshot-database` project directories are reserved for this
work. The existing 16-byte binary wire format (for `app-network-admin` mesh commands) is
the prototype for the compact token encoding.

## Component 3 — Cross-node VM scheduler (planned; os-orchestration)

`gateway-orchestration-command-1` is the intended home for cross-node placement logic. It
is stateless today — it aggregates PSP data queries across Totebox Archives and returns
result rows. The planned extension is intended to add a resource scheduling layer:

- **Placement**: when a new VM needs to be launched, the scheduler would read
  resource-availability advertisements from each node's `os-network-admin`, check the
  capability ledger for node trust status, and place the VM on the best-fit node.
- **Migration**: QEMU live migration transfers a running VM's state (memory, CPU
  registers, device state) over a TCP connection tunnelled through WireGuard. The VM
  remains available during the transfer; the cutover pause is typically under one second
  on a LAN.
- **Sovereignty constraint**: the operator is intended to be able to pin any VM to a
  specific trusted node. A VM pinned to Laptop A would not be migrated to Laptop B or the
  GCP node regardless of load imbalance. Operator sovereignty over placement is not
  overridden by automated optimisation.

No new transport is needed. QEMU live migration over WireGuard uses the existing encrypted
mesh. The scheduler is intended to remain stateless — placement decisions derived from
the capability ledger and node advertisements, no persistent scheduler state.

## Component 4 — Sovereign attestation chain (planned)

Intel TDX (5th-Gen Xeon, Azure GA 2025) and AMD SEV-SNP (EPYC 9000 Turin) provide
hardware-enforced VM isolation where the hypervisor cannot read guest memory. Both
require the CPU manufacturer's attestation infrastructure to verify isolation claims — a
certificate chain that passes through Intel Trust Authority or AMD Key Management Service.
The operator trusts the silicon vendor, not only their own hardware.

The PPN's intended attestation design is different:

- **Attestation root**: the pairing ceremony itself. When a node joins the mesh via CPace
  PAKE and SAS short-code comparison, the operator physically witnesses the exchange. The
  node's identity key is intended to serve as the attestation anchor — no TPM vendor, no
  silicon vendor, no cloud vendor in the trust chain.
- **Guest image attestation**: `dm-verity` (Linux device mapper, standard since kernel
  3.4) is intended to anchor the guest OS root filesystem to a hash committed at VM
  provision time. The hash would be signed by the provisioning node's pairing-ceremony
  key. A guest modified since provisioning would fail the `dm-verity` check and not boot.
- **Attestation report**: a guest is intended to generate a signed statement — signed with
  the provisioning node's identity key — asserting that it is running an unmodified image.
  External auditors could verify this statement without contacting Intel, AMD, or any
  cloud provider.

This chain is shorter and more operator-controlled than TDX/SEV-SNP. The trade-off is
intentional: the sovereignty model places the operator, not the silicon vendor, at the
root of trust.

## Comparison with major cloud provider capabilities

The table below describes what major cloud providers have deployed today and what the PPN
distributed fabric is intended to provide. Competitor capabilities in the first column are
present-tense factual statements. PPN items use planned/intended language.

| Capability | AWS / Azure / GCP (today) | PPN distributed fabric (planned) |
|---|---|---|
| Per-VM memory isolation | TDX (Azure GA Nov 2025), SEV-SNP (AMD EPYC 9005); hypervisor cannot read guest RAM in hardware | seL4 formal proof: machine-checked Isabelle/HOL invariant that hypervisor has zero read capability over VM state |
| Cross-node memory sharing | CXL 3.0 (PCIe fabric, data-centre internal; not exposed to tenants as a programmable API) | virtio-mem lending over WireGuard (intended to work over any network, including internet) |
| Capability revocation | IAM policies; centrally propagated; typically 10–60 seconds | Merkle DAG peer-to-peer gossip; intended sub-second revocation; no central authority |
| Attestation root | Silicon vendor CA (Intel Trust Authority / AMD Key Management Service) | Operator-witnessed pairing ceremony; operator intended as root of trust |
| Formal isolation proof | None in production hypervisors at any major cloud provider | seL4 Isabelle/HOL functional correctness and information-flow security proof (in vendor-sel4-kernel today) |
| SMB deployment time | Hours: cloud console, IAM, VPC networking configuration | Intended: under five minutes, two questions, short-code pairing ceremony |
| Operator sovereignty | None: cloud provider controls physical hardware and hypervisor | Full: operator owns the substrate; hypervisor cannot read VM state; cloud provider not in the trust chain |

## Build sequence and reserved directories

The per-node layer is the foundation. The distributed fabric is intended to build on top
of it in order:

| Step | State | Reserved directory |
|---|---|---|
| Per-node virtio_balloon + cgroups v2 | **Complete** — proven 2026-05-28 | `os-infrastructure/` |
| Ceremony deployment (`service-ppn-pairing` on GCP VM) | Pending — NEXT.md §7 Step 1 | `service-ppn-pairing/` |
| First real node join (`os-network-admin` on Laptop A) | Pending — NEXT.md §7 Step 3 | `os-network-admin/` |
| Genesis Protocol `os-infrastructure` rewrite | Planned | `os-infrastructure/`, `moonshot-hypervisor/` |
| Automated balloon controller | Planned | `os-infrastructure/` |
| virtio-mem lending daemon | Planned | `moonshot-network/` |
| Distributed capability ledger | Planned | `moonshot-protocol/`, `moonshot-database/` |
| Cross-node VM scheduler | Planned | `os-orchestration/` |
| Sovereign attestation chain (dm-verity + ceremony key) | Planned | `os-infrastructure/`, `moonshot-kernel/` |

## Related topics

- [[ppn-architecture-overview]] — four-layer PPN overview; the distributed fabric is the planned extension of the hypervisor layer
- [[ppn-hypervisor-resource-pool]] — the implemented per-node pool: virtio_balloon, cgroups v2, balloon controller milestone
- [[sovereign-mesh]] — the WireGuard transport the distributed fabric runs over
- [[genesis-protocol]] — the first-boot ceremony; the intended attestation root for the distributed fabric
- [[infrastructure-os]] — the Type I hypervisor; home of the balloon controller and intended virtio-mem lending daemon
- [[os-orchestration]] — the intended home for the cross-node VM scheduler
