---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.1"
title: "Pooled Compute from Heterogeneous Hardware: A Private Platform Network Deployment Case Study"
target_journal: "IEEE Transactions on Cloud Computing"
target_publisher: "IEEE Computer Society"
impact_factor: "6.49"
alternate_venue: "Future Generation Computer Systems (Elsevier, IF 6.23); Journal of Systems and Software (Elsevier, IF 3.76)"
authors:
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: jmwoodfine@gmail.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Software
      - Formal Analysis
      - Writing – Review & Editing
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Project Administration
      - Writing – Review & Editing
created: 2026-06-11
archive: project-infrastructure
research_trail:
  session: totebox@project-infrastructure 2026-06-11
  sources: [BRIEF-totebox-transformation.md §13-§15, live deployment records 2026-06-11]
  claim_verification: pending peer review
  bcsc_reviewed: false
  bilingual_pair: none
---

# Pooled Compute from Heterogeneous Hardware: A Private Platform Network Deployment Case Study

## Abstract

We describe the architecture and live deployment of a three-node Private Platform Network (PPN) assembled from heterogeneous hardware — a cloud virtual machine, two consumer-grade laptops running Linux — into a unified compute pool. The system employs WireGuard for encrypted inter-node transport, a lightweight Rust fleet controller for advisory VM placement, and per-node agents (service-vm-host) for QEMU/KVM virtual machine lifecycle management. Aggregate available memory across the three-node pool reached 8.6 GB, with two of three nodes providing KVM-accelerated virtualization. Advisory placement correctly selected the node with the most available memory and hardware virtualization capability in end-to-end testing. VM creation — copy-on-write qcow2 disk, cloud-init first-boot, QEMU launch — completed across a node boundary delegated over HTTP within the encrypted mesh. The deployment required three short operator commands to onboard each bare-metal node; all subsequent configuration was automated over SSH. We analyze the cost model, the current isolation boundary (network-layer encryption without host-level enforcement), and the planned seL4 microkernel isolation layer that would extend the security boundary to the hardware level. The primary contribution is a demonstration that pooled compute accessible to small businesses can be assembled from end-of-life consumer hardware at marginal cost, with the encrypted mesh providing meaningful network isolation from cloud and hosting providers today.

## 1. Introduction

Cloud compute pricing models favor organizations with predictable, large-scale workloads. Small businesses and early-stage ventures face a structural disadvantage: their workloads are variable, their budgets constrained, and they lack the negotiating leverage to obtain reserved-capacity pricing. Meanwhile, consumer hardware — laptops, desktop machines, workstations — is retired at intervals far shorter than its useful compute life. A three-year-old laptop discarded because it cannot run a current desktop OS efficiently can still host multiple virtual machines under a thin hypervisor layer.

We address the question: can pooled compute meaningful for small-business workloads be assembled from end-of-life consumer hardware, with encrypted transport isolating workloads from cloud and hosting providers, and with operational complexity low enough that a small IT team or a technically-literate employee can operate it?

Our deployment demonstrates that the answer is yes with meaningful qualification. The pool operates and placement functions correctly. The isolation guarantee is partial: network-layer encryption is in service; host-level enforcement through a formally verified kernel is planned and not yet running on bare metal. This paper is explicit about that boundary.

## 2. System Architecture

### 2.1 Node types and topology

The reference topology consists of three node roles:

**Genesis relay.** A cloud VM (Google Cloud f1-micro, 1 vCPU, 600 MB RAM, 3 GB available) with a static public IP address. The relay serves two functions: as the WireGuard hub accepting inbound connections from behind-NAT nodes, and as the fleet controller host. It has no hardware virtualization (no `/dev/kvm` on the n1-standard class used). It contributes little to the compute pool but is essential as the topological anchor.

**Spoke nodes.** Consumer-grade laptops running Linux (Ubuntu, Linux Mint). In the June 2026 deployment: a MacBook Pro 13" (2019, Intel Core i5, 8 GB RAM, ~3.0 GB available after OS) and a MacBook Air 13" (2017, Intel Core i5, 8 GB RAM, ~2.5 GB available). Both have `/dev/kvm` via Intel VT-x. Spoke nodes are behind residential NAT; they initiate WireGuard connections to the genesis relay.

**WireGuard mesh.** A hub-and-spoke overlay network on the 10.8.0.0/24 subnet. All inter-node communication — heartbeats, VM spawn delegation, inter-VM traffic — passes through this encrypted layer. PersistentKeepalive keepalives maintain hole-punched paths through NAT.

### 2.2 Fleet controller (service-vm-fleet, :9203)

A Rust async HTTP service (Axum, Tokio) maintaining an in-memory node registry. Nodes self-register by POSTing heartbeats every 10 seconds. The registry evicts nodes missing two consecutive heartbeat windows (~22 seconds). Advisory placement examines available RAM and KVM capability to select a target node, then delegates the spawn request via HTTP to the target node's service-vm-host.

The placement algorithm is intentionally simple: select the node with the most available RAM, with KVM nodes preferred when memory is comparable. `auto_rebalance` is disabled; the controller does not migrate running VMs.

### 2.3 Per-node agent (service-vm-host, :9220)

A Rust async HTTP service on each node accepting spawn and destroy requests from the fleet controller. Spawn:
1. Creates a qcow2 copy-on-write disk backed by a cached Ubuntu 24.04 cloud image (nearly instantaneous; base image downloaded once at setup time).
2. Assembles a cloud-init seed ISO with user-data (SSH authorized key, hostname).
3. Launches a QEMU/KVM process with the disk and seed ISO, user-mode networking (SLIRP), a MAC address deterministically derived from the VM ID, and the specified RAM allocation.

The agent reports a `VmRecord` back to the fleet controller, which records it in the node's VM list for subsequent destroy delegation.

### 2.4 WireGuard isolation model

All inter-node traffic is encrypted with WireGuard (ChaCha20-Poly1305, Curve25519 key exchange). A cloud provider hosting the genesis relay sees only encrypted UDP datagrams on port 51820; the contents of heartbeats, spawn requests, and inter-VM traffic are not visible to the provider.

Peer admission is currently manual: adding a node to the mesh requires editing the hub's peer table. This is Phase S3 of the planned os-network-admin admission control automation, currently a simulation stub.

### 2.5 Planned host isolation (seL4)

WireGuard encryption does not protect against an adversary who controls the physical or virtual machine. A cloud provider's hypervisor can in principle access guest memory; a person with physical access to a laptop can do the same. The intended extension is the seL4 microkernel as the os-infrastructure boot layer, partitioning workloads from the hypervisor host with formal proof of noninterference. This is planned and not in service on bare metal as of the June 2026 deployment. The deployment reported here provides network-layer isolation; host-layer isolation is a research and engineering milestone ahead of it.

## 3. Deployment Case Study — June 2026

### 3.1 Configuration

| Node | Hardware | OS | WireGuard IP | KVM | RAM available |
|---|---|---|---|---|---|
| gcp-cloud-1 | GCP n1 VM | Ubuntu 22.04 | 10.8.0.9 | No | 3,070 MB |
| laptop-b-1 | MacBook Pro 2019 | Linux Mint | 10.8.0.1 | Yes | 2,484 MB |
| laptop-a-1 | MacBook Air 2017 | Ubuntu 24.04 | 10.8.0.6 | Yes | 3,047 MB |

Total pool: 8,601 MB available RAM. KVM-capable nodes: 2/3.

### 3.2 Onboarding procedure

Each spoke node required three operator-initiated commands before remote setup could proceed:
1. Install OpenSSH server.
2. Append the fleet automation SSH public key to `~/.ssh/authorized_keys`.
3. Grant passwordless sudo to the node's user account.

All subsequent configuration — package installation, base image download, WireGuard interface creation, systemd unit installation, heartbeat registration — was performed autonomously over SSH.

A complication encountered on laptop-a-1: its WireGuard interface (`wg0`) was already allocated to a separate encrypted network (10.50.0.0/24, used for LXC container networking). Rather than modify the existing interface, the PPN mesh was assigned to a second interface (`wg1`) at 10.8.0.6. Both WireGuard interfaces coexist without conflict.

Total operator interaction time to add laptop-a-1 (the more complex case, with no prior SSH access and an existing WireGuard interface to navigate): approximately 10 minutes of terminal interaction.

### 3.3 Advisory placement validation

Following deployment, a VM creation request was submitted to the fleet controller at the GCP genesis relay node. The controller evicted stale registry entries, selected laptop-a-1 as the target (most available RAM at 3,047 MB, KVM capability), and delegated the spawn request to `http://10.8.0.6:9220/v1/spawn` over the WireGuard mesh. laptop-a-1's service-vm-host created a qcow2 disk, assembled a cloud-init seed ISO, and launched a QEMU/KVM process. The controller received and recorded the resulting `VmRecord`. Cross-node VM spawn delegation over an encrypted mesh boundary was confirmed end to end.

Simultaneously, two VMs were confirmed running concurrently on gcp-cloud-1 under QEMU software emulation (no KVM), demonstrating that nodes without hardware virtualization remain viable for non-latency-critical workloads.

### 3.4 Cost model

| Component | Cost | Notes |
|---|---|---|
| gcp-cloud-1 genesis relay | ~US$15–20/month | Small cloud VM, any provider |
| laptop-b-1 (MacBook Pro 2019) | $0 marginal | Hardware owned; amortized at purchase |
| laptop-a-1 (MacBook Air 2017) | $0 marginal | Hardware owned; amortized at purchase |
| WireGuard, Rust fleet software | $0 | Open-source |
| Electricity (estimate, 2 laptops) | ~$8–12/month | At $0.12/kWh, 15W average each |

Approximate total monthly cost: US$23–32 for a three-node PPN providing ~8.6 GB pooled RAM and KVM-accelerated virtualization on two nodes. A comparable cloud-hosted capacity (3 × 2 vCPU / 4 GB RAM VMs) would cost approximately US$100–150/month at on-demand pricing on a major provider.

## 4. Discussion

### 4.1 What this demonstrates

The deployment shows that the coordination layer — encrypted mesh, advisory placement, delegated spawn — functions correctly across heterogeneous hardware with meaningfully different network characteristics. The fleet controller's design (eviction-based liveness, advisory rather than prescriptive placement) is appropriate for a pool where node availability is not guaranteed: a laptop that hibernates or loses connectivity is evicted without affecting the placement decisions for remaining nodes.

The three-command onboarding procedure is a working proof of concept, not a product. The target operator experience is a bootable ISO (write to USB, boot, answer three questions) that replaces the three manual commands and the subsequent SSH-based setup entirely. os-infrastructure is the software layer toward that target.

### 4.2 Limitations and future work

**Host isolation.** The most significant security boundary not yet closed is host-level isolation. Network encryption is necessary but insufficient: an operator of the hardware can access guest workloads. The seL4 microkernel isolation layer, when delivered on bare metal, would address this by enforcing formal noninterference between the guest partitions and the hypervisor host.

**Peer admission automation.** os-network-admin's role in automating WireGuard peer table updates across the mesh (Phase S3 in the simulation discipline) would eliminate the manual `wg set` step and allow the admission ceremony to be driven by the CPace pairing protocol without operator shell access.

**Power management on bare-metal nodes.** Laptops used as PPN nodes must have suspend/hibernate disabled to remain in the mesh. This is a deployment configuration step not addressed by the current setup automation.

**WireGuard persistence.** Runtime `wg set` changes are not persisted automatically. The setup automation writes a wg-quick configuration file and enables the corresponding systemd unit, which handles persistence on nodes where setup completed fully. Nodes where the interface was configured at runtime without the wg-quick unit need manual persistence before reboots.

## 5. Conclusion

A three-node PPN providing pooled virtual machine compute was assembled from one cloud VM and two end-of-life consumer laptops in June 2026. The fleet controller performed advisory placement correctly, selecting the highest-memory KVM-capable node and delegating VM spawn across a WireGuard-encrypted mesh boundary. Each bare-metal node required three operator commands to onboard; everything else was automated. The monthly cost of the three-node pool is approximately US$23–32, compared with US$100–150 for equivalent capacity rented at cloud on-demand rates.

The network isolation layer (WireGuard encryption) is in service. The host isolation layer (seL4 microkernel partitioning) is the primary outstanding engineering milestone before the system can make a strong guarantee of workload isolation from hardware owners and cloud providers. The deployment establishes the pooled-compute substrate on which that isolation layer will be mounted.
