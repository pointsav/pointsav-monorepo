---
artifact: topic-draft
foundry-draft-v1: true
language_protocol: PROSE
route_to: project-editorial
status: draft
created: 2026-06-11
archive: project-infrastructure
research_trail:
  session: totebox@project-infrastructure 2026-06-11
  sources: [BRIEF-totebox-transformation.md §13-§15, live test results 2026-06-11]
  claim_verification: pending editorial review
  bcsc_reviewed: false
  bilingual_pair: TOPIC-ppn-small-business-compute.es.md
---

# The Private Platform Network: Pooled Compute from Hardware You Already Own

## What a PPN is

A Pointsav Private Platform Network (PPN) is a private, encrypted compute network assembled from machines a business owns or leases. Each machine — an old laptop in a back office, a rented server in a data centre, a virtual machine at a cloud provider — runs the same operating layer, os-infrastructure, and joins the same encrypted mesh. Once joined, the machines stop being individual computers and become nodes in a single pool of compute capacity.

The design goal is straightforward to state: workloads running inside the network should not be accessible to anyone outside it — not the cloud provider hosting one of the nodes, not the data centre leasing another, not a person with physical access to the hardware. Parts of that goal are operating today; parts are planned. This document is explicit about which is which.

## Three node types, one network

A PPN node can be any of three things:

1. **Bare metal** — a physical machine the business owns. In the June 2026 tests, these were consumer laptops several years old, running ordinary Linux.
2. **Leased server** — a dedicated or virtual private server rented from a hosting provider.
3. **Cloud VM** — an instance at a public cloud provider, such as Google Cloud.

Once os-infrastructure is installed, the three are operationally identical. The network does not distinguish a laptop from a cloud instance; each node reports the same information — available memory, virtualization capability, a periodic heartbeat — and each can host workloads. The differences that remain are physical facts: a cloud VM is reachable from the public internet and is useful as a relay point; a laptop behind a home router is not, and contributes capacity instead.

## What "pooled resources" means

A business using a PPN does not assign work to specific machines. It asks the network for a virtual machine, and a fleet controller decides where that VM runs. The controller's placement logic is advisory: it examines the current state of every node — reported memory, whether hardware virtualization (KVM) is available — and selects the node best able to take the work. The request is then delegated to that node, which creates the VM locally and reports back.

The caller never needs to know which physical machine was chosen. In the June 2026 test, a request submitted to the fleet controller running on a cloud node was placed on a laptop in a different building, because the laptop had the most free memory and hardware virtualization the cloud node lacked. The requester saw only a record of a new VM entering service.

This is what makes aged hardware useful. A laptop too slow for daily desktop work still has memory and a capable processor. Pooled behind a single interface, several such machines amount to a small private cloud.

## The isolation model — current and intended

The isolation model has two layers, and they are at different stages of maturity.

**Network isolation — operating today.** All traffic between nodes travels through WireGuard, an audited, kernel-level encrypted tunnel. A cloud provider hosting a PPN node sees only encrypted UDP packets. It cannot read the contents of inter-node traffic, observe which workloads exist, or inject itself into the mesh.

**Host isolation — planned.** Encryption protects data in transit, but the operator of the physical machine can in principle inspect what runs on it: a cloud provider controls its hypervisor, and a person with physical access controls a laptop. The intended answer is the seL4 microkernel, a formally verified kernel designed to enforce strict partitions between workloads and the host environment. The target state is that os-infrastructure boots seL4 as its isolation layer, so that the owner of the hardware — including a cloud or hosting provider — cannot inspect the memory of guest workloads. This capability is planned and is not running on bare metal today. Until it ships, host-level isolation rests on conventional Linux and QEMU/KVM boundaries, and a party who controls the physical machine should be assumed able to access workloads on that machine.

## Who controls admission: the role of os-network-admin

An encrypted mesh is only as trustworthy as its membership. os-network-admin is the PPN's routing authority: it decides which machines may join the mesh, approves or denies join requests, and is intended to manage the WireGuard peer configuration across all nodes automatically. Peer admission control matters because the threat to a private network is rarely cryptographic — it is an unauthorized machine being accepted as a peer. Centralizing admission in one auditable component keeps the membership list deliberate.

A design question is open: where should this authority live?

- **External mode.** os-network-admin runs on a node outside the PPN, typically a cloud machine, and manages the peer table from outside. This is simpler to bootstrap, at the cost of depending on an authority the PPN itself does not contain.
- **Internal mode.** os-network-admin runs as the first VM on the PPN, inside an isolation partition on the founding node — under the planned seL4 layer, a partition the hardware owner is intended to be unable to access. The network would then govern its own membership with no external dependency. This is harder to bootstrap, because the founding protocol must designate the first node as the initial authority.

Both modes are considered valid for different deployment profiles, and the platform is being designed to permit either.

## The economics for a small business

The cost structure of a PPN differs from renting cloud capacity. Most of the network is hardware the business already owns: retired laptops and desktops contribute real compute at near-zero marginal cost beyond electricity. The one recurring cost in the reference configuration is a small cloud VM acting as a publicly reachable relay — on the order of US$15–20 per month at current pricing. Everything else — the operating layer, the mesh, the fleet controller — runs on owned equipment.

For a small business, the practical proposition is this: three or four machines that would otherwise be recycled can be assembled, with a simple install on each, into a private network that provisions virtual machines on demand. The intent is that this network is one the business alone controls.

## What the June 2026 test demonstrated

In June 2026, a three-node PPN was assembled and exercised live:

- A cloud VM at Google Cloud, a MacBook Pro running Linux, and a MacBook Air running Linux formed a working WireGuard mesh, with the laptops contributing hardware virtualization the cloud node lacked.
- The fleet controller performed advisory placement across the mesh: presented with a VM request, it selected the laptop with the most available memory and hardware virtualization, and delegated the spawn to it across the node boundary. The laptop accepted and created the VM.
- VM creation itself was verified end to end: copy-on-write disks backed by a standard Ubuntu cloud image, automated first-boot configuration, and QEMU launch, with two VMs confirmed running concurrently on the cloud node.
- Converting a consumer laptop into a PPN node required three short manual commands; the remainder of the setup was automated over SSH.

What the test did not demonstrate is equally important to state: host-level isolation through seL4 remains planned, and automated peer admission through os-network-admin is not yet in service. The June test established the pooled-compute substrate; the sovereignty layer is the target the project is building toward.
