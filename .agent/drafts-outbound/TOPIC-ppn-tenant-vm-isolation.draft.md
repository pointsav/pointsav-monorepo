---
schema: foundry-draft-v1
artifact: TOPIC
draft-id: project-infrastructure-topic-tenant-isolation
title: PPN Tenant VM Isolation
status: staged
owner: project-infrastructure
created: 2026-06-14
updated: 2026-06-14
route: project-editorial
destination: media-knowledge-documentation
language_protocol: EN+ES
bcsc_class: internal-draft
gaps:
  - "ES sibling required before publication — to be written by project-editorial"
  - "Network-layer isolation section needs update after Phase S3 ships"
research_trail:
  methodology: "Architecture derived from BRIEF-ppn-infrastructure-reference.md §16 (service-vm-tenant Fable review) and §17 (OpenStack/CloudStack comparison)"
  sources_consulted:
    - "BRIEF-ppn-infrastructure-reference.md §16"
    - "BRIEF-ppn-infrastructure-reference.md §17"
    - "service-vm-tenant/src/ (implementation as of dbf6a528)"
    - "system-vm-fleet-types/src/lib.rs (HostPortMapping, dbf6a528)"
  claims_verified: true
  forbidden_terms_cleared: false
  notes: "Needs Bloomberg vocabulary check; isolation guarantees are factual descriptions of current implementation, not forward-looking claims"
---

# PPN Tenant VM Isolation

The PointSav Private Network (PPN) resource pool allows multiple tenants to run virtual machines
on a shared set of physical and cloud nodes. This topic describes the isolation model: what
separation is guaranteed, what is not, and the planned path to stronger network-level isolation.

---

## The stack

Every virtual machine request passes through three layers before a QEMU process starts on a
physical node:

```
Tenant request (authenticated)
  → service-vm-tenant  :9221   — identity check, namespace, quota enforcement
  → service-vm-fleet   :9203   — placement, delegation (internal; no direct tenant access)
  → service-vm-host    :9220   — QEMU spawn on the selected node (internal; no direct tenant access)
```

`service-vm-tenant` is the sole entry point for external callers. The fleet controller and
host agent are firewalled to accept connections only from the tenant proxy or from other
internal mesh participants. A caller who has the fleet controller's address but not a valid
tenant credential cannot reach the fleet controller directly — the auth layer cannot be
bypassed.

---

## What tenant isolation provides

### Namespace isolation

An authenticated tenant can create, list, and destroy only their own virtual machines. The
tenant identity is injected by `service-vm-tenant` into every request forwarded to the fleet
controller; a tenant cannot supply a different identity in a request body. Listing VMs returns
only records owned by the authenticated tenant. Destroy requests are validated against ownership
before being forwarded.

This isolation survives fleet controller restarts: the `tenant_id` field is stored in the VM
record, echoed back in every node heartbeat, and restored to the in-memory registry on startup.
Ownership is not held only in memory on the tenant proxy.

### Process isolation

Each virtual machine is a separate QEMU process on the physical node. Guest operating systems
run in hardware-isolated address spaces (on nodes with KVM acceleration) or software-isolated
address spaces (on TCG-only nodes such as the GCP genesis relay). One tenant's virtual machine
cannot read the memory or disk of another tenant's virtual machine through normal software paths.

### Per-VM network containment

Virtual machines use SLIRP user-mode networking: each guest receives a private NAT address
stack (`10.0.2.0/24`) with no inbound path from the host network or from other VMs. A guest
process that opens a server socket is not reachable unless a host-forwarding rule (`hostfwd`)
was configured at spawn time. The VM spawn response includes the list of forwarded host ports
so callers know how to reach their VM.

### Bearer token security

Tenant credentials are opaque bearer tokens — random strings that map to a tenant identity
in the `TOKEN_MAP` environment variable on the tenant proxy. Knowing a tenant's identity string
is not sufficient to authenticate; the caller must present the associated token. Tokens are
not logged in any audit record.

### Audit trail

Every tenant lifecycle operation — create VM, destroy VM — is recorded in two places: a local
JSONL file on the tenant proxy, and the `service-fs` WORM ledger (write-once, append-only
storage). The WORM record includes the tenant identity, operation type, VM ID, timestamp, and
outcome. WORM entries cannot be overwritten or deleted.

---

## What tenant isolation does not provide

### No per-tenant network subnet

All virtual machines on a given physical node egress through the same host network interface.
A network observer watching the node's outbound traffic cannot distinguish traffic from tenant
A's VM from traffic from tenant B's VM at the transport layer. Tenants are isolated at the
application layer (separate SLIRP stacks) but not at the network layer.

### No isolation from the node operator

Anyone with root access to the physical machine that hosts a VM can read the guest's disk
image and memory. This is a property of the current QEMU-based virtualization model, not a
limitation of the PPN control plane.

The intended response to both limitations is the seL4 isolation layer, described below.

---

## Quota enforcement

Each tenant has a maximum VM count configured in the `TOKEN_MAP` environment variable
(alongside the token-to-identity mapping). The tenant proxy enforces quotas using a mutex
to serialize concurrent create requests: two simultaneous requests from the same tenant both
pass the quota check only if sufficient quota exists for both.

---

## Path to network-level isolation

Network-level isolation — per-tenant WireGuard subnets so tenant VMs are cryptographically
unreachable from other tenants, not merely API-separated — requires two future milestones:

**Phase S3 of os-network-admin (planned):** The PPN network control plane gains the ability
to manage WireGuard peer tables programmatically. When a new node joins the mesh, the control
plane updates all nodes' routing configurations automatically. This is the foundation for
assigning each tenant a distinct subnet with its own WireGuard keys.

**seL4 Mode B (planned/intended):** The network control plane itself runs as a first-class
VM inside the PPN, isolated by a formally-verified microkernel. In Mode B, even a node
operator with root access to the physical machine cannot inspect the network control plane's
configuration or extract tenant keys.

Until these milestones are delivered, the isolation boundary is accurately described as
API-level isolation: tenants cannot reach each other's resources through the control plane,
but they share physical network paths at the transport layer, and node operators retain
root-level access to the physical machines they administer.

---

## Related topics

- [PPN VM Resource Pool Architecture] — system overview, three-node stack, WireGuard mesh
- [PPN Node Setup] — how to join a physical machine to the resource pool
- [PPN Fleet Operations] — operator guide for managing nodes, spawning VMs, monitoring quotas
