---
artifact: topic
schema: foundry-draft-v1
title: "PPN Small-Business Compute"
slug: topic-ppn-small-business-compute
status: draft
language: en
bilingual_pair_required: true
bcsc_class: internal
forbidden_terms_cleared: false
route_to: project-editorial
created: 2026-06-14
updated: 2026-06-14
research_trail:
  sources_cited: false
  claims_verified: false
  sme_review: pending
  external_review: not-required
  last_checked: 2026-06-14
---

# PPN Small-Business Compute

The Private Pointsav Network (PPN) compute layer provides pooled virtual machine
capacity to small-business customers. Capacity runs on GCP e2 instances under the
QEMU TCG hypervisor and is managed through three cooperating services: a fleet
controller, a per-node heartbeat agent, and a customer-facing tenant proxy.

## Architecture

### VM Image

Each VM instance boots from a NetBSD 10.1 QCOW2 disk image built by
`os-totebox/scripts/build-image.sh`. The image carries a curated set of services
in its overlay:

- `system-ledger-server` — append-only capability ledger (Unix socket at `/run/system-ledger/ledger.sock`)
- `slm-doorman-server` — small-language-model inference gateway
- `sshd` — operator access; `UseDNS no` to avoid banner timeout under TCG

Veriexec runs in strict mode (`kern.veriexec.strict=1`). A manifest generated at
image-build time covers every executable in the overlay; unsigned binaries are
denied execution at runtime.

### Fleet Controller (`service-vm-fleet`)

`service-vm-fleet` runs on the host at port 9203. It exposes three endpoints:

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/v1/vms` | POST | Register a new VM and receive advisory placement |
| `/v1/vms` | GET | List VMs; optional `?tenant_id=` filter |
| `/healthz` | GET | Health probe |

The controller ingests periodic heartbeats from `service-vm-host` agents running
on each node. Placement advice is based on reported available memory; the
controller holds no scheduling authority over the hypervisor.

### Per-Node Agent (`service-vm-host`)

`service-vm-host` runs on every physical host. It reads `/proc/meminfo` at each
heartbeat interval and reports current memory availability to the fleet controller.
A QEMU monitor stub is present for future live-migration support.

### Tenant Proxy (`service-vm-tenant`)

`service-vm-tenant` runs at port 9221 and is the only endpoint exposed to customers.
Requests carry a `Bearer` token in the `Authorization` header. The proxy:

1. Validates the token against the in-memory `TenantStore`.
2. Checks quota (maximum VMs per tenant) before forwarding create requests to the fleet controller.
3. Writes a WORM audit entry to `system-ledger-server` for each create attempt, approval, denial, and destroy.

If the ledger socket is unavailable, the proxy logs a warning and proceeds — the
ledger is advisory in the current phase.

### Wire Types (`system-vm-fleet-types`)

`system-vm-fleet-types` is a `no_std`-compatible library crate that defines the
shared wire types used by all three services:

- `VmRecord` — represents a registered VM instance; carries `tenant_id: Option<String>`
- `CreateVmRequest` — customer create payload; carries `tenant_id: Option<String>`
- `HeartbeatPayload` — per-node availability report

## Quota and Isolation

Each tenant is isolated by a numeric VM cap stored in `TenantRecord`. The proxy
enforces the cap before forwarding to the fleet controller; the fleet controller
does not validate tenant identity. A TOCTOU-safe create path holds a per-tenant
`Mutex` for the duration of the create–response cycle, preventing quota overshoot
under concurrent requests from the same tenant.

## Deployment

The PPN compute layer intended deployment target is `os-console` (the operator
surface) paired with one or more QEMU hosts. The fleet controller and tenant proxy
are workspace members in `pointsav-monorepo`. The NetBSD image is built from
source on each release via `build-image.sh`.

## Relationship to Other Topics

- [OS Console Architecture](topic-os-console-architecture) — operator TUI for
  managing fleet state
- [Software Distribution Substrate](topic-software-distribution-substrate) — the
  licensing layer that gates customer access to PPN capacity
- [Private Git Paid Customer Endpoint](topic-private-git-paid-customer-endpoint) —
  binary delivery for licensed customers
