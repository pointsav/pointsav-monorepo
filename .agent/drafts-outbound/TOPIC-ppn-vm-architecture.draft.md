---
artifact: topic
schema: foundry-draft-v1
draft-id: project-infrastructure-topic-ppn-vm-architecture
title: "PPN VM Resource Pool Architecture"
language_protocol: PROSE-RESEARCH
status: draft
owner: project-infrastructure
created: 2026-06-14
research_trail:
  sources: ["BRIEF-totebox-transformation.md §13–§18", "service-vm-fleet/src/main.rs", "service-vm-host/src/vm_spawn.rs", "service-vm-tenant/src/main.rs", "app-orchestration-slm/CLAUDE.md"]
  method: code and brief review
  confidence: high
  gaps: []
  forbidden_terms_cleared: true
bcsc_class: internal
route_to: project-editorial
---

## §1 Overview

The PointSav Private Network (PPN) VM resource pool is a three-service stack that provisions, places, and accounts for virtual machines across a heterogeneous WireGuard mesh. The current pool spans one GCP instance and two physical laptops, forming a small but fully operational distributed compute substrate.

Three services partition the responsibility surface cleanly. `service-vm-fleet` (:9203) maintains a global view of node capacity and handles advisory placement decisions. `service-vm-host` (:9220) runs per-node as the spawn authority, communicating with QEMU/KVM and holding the local disk state for each VM. `service-vm-tenant` (:9221) sits at the customer boundary, enforcing Bearer authentication, tenant namespace isolation, quota limits, and an immutable audit trail. A fourth process, `app-orchestration-slm` (:9180), sits above this stack as the commercial Yo-Yo broker for metered inference workloads.

All four processes communicate across the PPN WireGuard underlay (10.8.0.0/24). No service exposes a public interface; all customer-facing traffic enters through the tenant proxy.

## §2 service-vm-fleet (:9203)

`service-vm-fleet` is the fleet controller. It receives periodic heartbeats from every `service-vm-host` instance and maintains an in-memory registry of node state: available RAM, KVM availability flag, current VM count, and reservation status.

Placement uses a two-pass selection algorithm. Pass 1 considers only non-reserved nodes — those where `VM_RESERVED` is not set — satisfying the requested RAM ceiling. If Pass 1 yields no candidate, Pass 2 widens the search to include reserved nodes. This separation allows operators to designate nodes for latency-sensitive workloads while still permitting overflow placement under load.

The controller exposes a `GET /v1/fleet` endpoint for status inspection and a `GET /v1/vms` endpoint that accepts a `tenant_id` query parameter, returning the VMs belonging to that tenant across all nodes. Heartbeat ingestion is unauthenticated at the fleet boundary; the PPN underlay and service-vm-host node identity provide the implicit trust boundary.

Fourteen tests pass against the controller logic as of the 2026-06-12 activation.

## §3 service-vm-host (:9220)

`service-vm-host` is the per-node spawn authority. Each node in the pool runs one instance, which listens on :9220 and accepts spawn and destroy requests forwarded by the fleet controller after placement.

On receiving a spawn request, the agent assembles a cloud-init ISO from the request parameters, invokes QEMU with the appropriate `-machine`, `-cpu`, and `-drive` arguments, and writes a `.meta.json` sidecar alongside the disk image. The sidecar stores the VM identifier, QEMU monitor socket path, and heartbeat sequence number, enabling QMP-based heartbeat recovery after a process restart or node reboot without querying the fleet controller.

The `VM_RESERVED` environment variable, read at startup, signals to the fleet controller that this node should be excluded from Pass 1 placement. The `NODE_ID` variable provides the stable identity used in heartbeat messages and fleet registry entries. `FLEET_URL` points to the fleet controller's base URL, used for heartbeat delivery.

KVM availability is detected at startup by probing `/dev/kvm`. On nodes where KVM is absent — including GCP instances without nested virtualisation enabled — the agent reports `kvm: false` in its heartbeat and QEMU falls back to software emulation. GCP currently acts as the relay node rather than a KVM host; the two laptops provide hardware-accelerated virtualisation.

## §4 service-vm-tenant (:9221)

`service-vm-tenant` is the customer-facing proxy layer. It accepts spawn, destroy, and status requests from authenticated callers and enforces the tenant contract before forwarding to the fleet controller.

Authentication uses Bearer tokens issued at tenant provisioning time. Each token carries a tenant identifier that the proxy uses to namespace all VM records. A tenant may not query, modify, or destroy VMs belonging to another tenant; the proxy enforces this at every endpoint before any fleet interaction.

Quota enforcement operates at the RAM level. Each tenant is assigned a RAM ceiling at provisioning time. The proxy checks the tenant's current allocated RAM against the ceiling before forwarding a spawn request; requests that would exceed the quota receive a 429 response with a quota-exceeded body.

All write operations — spawn and destroy — are appended to a WORM audit log. The log entry records the tenant identifier, VM identifier, operation type, timestamp, and the source IP of the request. The log is append-only at the application layer; no destroy or update path touches existing entries. This design satisfies the audit retention requirement stated in DOCTRINE §IV without a separate logging service.

A TOCTOU race on concurrent creates from the same tenant is resolved by a per-tenant Mutex held for the duration of the quota check and fleet-forward sequence. This prevents two simultaneous spawn requests from both passing the quota check against the same pre-spawn RAM total.

## §5 app-orchestration-slm (:9180)

`app-orchestration-slm` is the commercial Yo-Yo broker for metered inference traffic, positioned above the VM stack as the Tier B revenue surface (DOCTRINE #23). It does not spawn or manage VMs directly; it brokers inference requests to the local SLM tier and accounts for per-tenant consumption.

The broker implements a circuit breaker with configurable open/half-open/closed thresholds. A flow gate controls admission: when the circuit is open or the gate is closed, incoming inference requests receive a service-unavailable response rather than queuing indefinitely. This protects the local SLM from load spikes that would otherwise degrade response latency for all tenants.

Per-tenant metering records token counts for each completed inference. An audit rollup endpoint at `GET /v1/audit/rollup` aggregates metering records by tenant for billing reconciliation. License status is checked at startup and periodically thereafter; a license fault closes the flow gate without dropping in-flight requests.

## §6 WireGuard Mesh

The PPN underlay uses a WireGuard mesh configured on the 10.8.0.0/24 subnet. All three service processes bind to their WireGuard interface addresses, not to public interfaces. Peer configuration is static; each node holds the public keys of its peers and the allowed-IP ranges that route through each tunnel.

GCP functions as the relay node in the current topology. Traffic between the two laptop nodes passes through GCP when the laptops are on separate networks without a direct route, though WireGuard will use direct peer paths when available. This relay role does not require GCP to participate in VM placement — it operates at the network layer only.

Node heterogeneity is a deliberate property of the pool. Each node type contributes a different capability profile: GCP provides reliable uptime and elastic storage; the laptops provide KVM-accelerated virtualisation. Placement decisions reflect these differences through the KVM flag and the two-pass reserved/non-reserved selection.

## §7 Planned Extensions

Three extensions to the compute substrate are under development and are intended for future deployment; none is operational today.

**seL4 microkernel isolation** is planned for the VM spawn path on AArch64 hardware, using the Microkit 2.2.0 component model. The intended design would replace the host kernel as the trust boundary for VM isolation, providing formal verification of the scheduler and IPC paths between VMs. Availability on AArch64-first hardware is intended; x86_64 support would follow.

**Firecracker microVM** is under evaluation as an alternative to QEMU for workloads where sub-second boot time and reduced memory overhead are priorities. The intended integration point is `service-vm-host`, which would select the hypervisor backend based on the `vm_type` field in the spawn request.

**Genesis Protocol bare-metal ISO** is intended as the provisioning substrate for new PPN nodes. The planned design would produce a signed, reproducible ISO that installs the full node stack — WireGuard, service-vm-host, and supporting services — from a single boot, without requiring an existing OS install or manual configuration. Bare-metal support is planned and intended; delivery timeline depends on the seL4 and Firecracker integration milestones.
