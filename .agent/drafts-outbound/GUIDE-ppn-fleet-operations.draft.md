---
artifact: guide
schema: foundry-draft-v1
draft-id: project-infrastructure-guide-ppn-fleet-operations
title: "PPN Fleet Operations"
language_protocol: PROSE-RESEARCH
status: draft
owner: project-infrastructure
created: 2026-06-14
research_trail:
  sources: ["BRIEF-totebox-transformation.md §13", "service-vm-fleet/src/main.rs", "service-vm-host/src/main.rs", "service-vm-tenant/src/main.rs"]
  method: code review
  confidence: high
  gaps: []
  forbidden_terms_cleared: true
bcsc_class: internal
route_to: project-editorial
---

## §1 Prerequisites

Before performing any fleet operation, confirm the following conditions:

- WireGuard mesh is active on 10.8.0.0/24. Verify with `wg show` on each node; all expected peers should appear with a recent last-handshake timestamp.
- `local-vm-fleet.service` is running on the GCP controller node. Check with `systemctl is-active local-vm-fleet`.
- `local-vm-host.service` is running on each node that participates in the pool. Check on each node with `systemctl is-active local-vm-host`.
- `local-vm-tenant.service` is running on the node that serves customer traffic. Check with `systemctl is-active local-vm-tenant`.
- `local-orchestration-slm.service` is running if metered inference is in use. Check with `systemctl is-active local-orchestration-slm`.

If any required service is inactive, check `journalctl -u <unit> -n 50` for fault messages before proceeding.

## §2 Check Fleet Status

Retrieve the current state of all registered nodes:

```
curl http://127.0.0.1:9203/v1/fleet
```

The response is a JSON array. Each element represents one node and includes:

- `node_id` — the stable identifier configured via `NODE_ID` on that host
- `ram_mb_total` and `ram_mb_used` — total and allocated RAM in megabytes
- `kvm` — boolean; `true` if `/dev/kvm` was present at service startup
- `vm_count` — number of VMs currently tracked on that node
- `reserved` — boolean; `true` if `VM_RESERVED=true` is set on that host
- `last_heartbeat` — ISO 8601 timestamp of the most recent heartbeat received

A node that has not sent a heartbeat within the expected interval will still appear in the list but its `last_heartbeat` will be stale. The fleet controller does not automatically evict nodes; stale entries indicate a host that has stopped sending heartbeats and should be investigated.

## §3 Add a Node

To add a new node to the pool:

1. Install `service-vm-host` on the node. Copy the compiled binary to `/usr/local/bin/vm-host`.

2. Create `/etc/default/vm-host` with the required environment variables:

   ```
   NODE_ID=<unique-identifier-for-this-node>
   FLEET_URL=http://<fleet-controller-wireguard-ip>:9203
   VM_RESERVED=false
   ```

   Use the node's WireGuard address (10.8.0.x) for the `FLEET_URL` host.

3. Install and start the systemd unit:

   ```
   systemctl enable local-vm-host
   systemctl start local-vm-host
   ```

4. Confirm the node appears in the fleet within 30 seconds:

   ```
   curl http://127.0.0.1:9203/v1/fleet
   ```

   Look for a new entry with the `NODE_ID` you configured. If the entry does not appear, check `journalctl -u local-vm-host -n 30` on the new node and confirm the WireGuard route to the fleet controller is active.

## §4 Reserve a Node

Reserving a node excludes it from Pass 1 placement. The fleet controller will only assign VMs to a reserved node when no non-reserved node can satisfy the request (Pass 2 fallback).

On the target node, edit `/etc/default/vm-host`:

```
VM_RESERVED=true
```

Restart the service:

```
systemctl restart local-vm-host
```

Verify the change is reflected in the fleet within one heartbeat interval:

```
curl http://127.0.0.1:9203/v1/fleet
```

Confirm the node's `reserved` field is now `true`. Existing VMs on the node are not affected; reservation only influences future placement decisions.

To un-reserve a node, set `VM_RESERVED=false` and restart. The node returns to Pass 1 eligibility immediately on the next heartbeat.

## §5 Spawn a Test VM (Operator Path)

The fleet controller's spawn endpoint requires no authentication. This path is for operator testing and internal tooling only; customer VMs must go through the tenant proxy (§6).

```
curl -s -X POST http://127.0.0.1:9203/v1/vms \
  -H 'Content-Type: application/json' \
  -d '{
    "ram_mb": 1024,
    "vcpu_count": 1,
    "vm_type": "linux",
    "prefer_kvm": true
  }'
```

A successful response includes:

- `vm_id` — the assigned VM identifier (retain this for subsequent operations)
- `node_id` — the node where the VM was placed
- `kvm` — whether KVM acceleration is active for this VM

If `prefer_kvm` is `true` but no KVM-capable node has sufficient RAM, the controller places the VM on a non-KVM node. The response `kvm` field reflects the actual result, not the preference.

If no node has sufficient RAM, the controller returns a 503. Check `/v1/fleet` to review available capacity.

## §6 Spawn as Tenant (Customer Path)

Customer spawn requests pass through `service-vm-tenant` (:9221), which enforces Bearer authentication and quota limits.

```
curl -s -X POST http://127.0.0.1:9221/v1/vms \
  -H 'Authorization: Bearer <tenant-token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "ram_mb": 1024,
    "vcpu_count": 1,
    "vm_type": "linux",
    "prefer_kvm": true
  }'
```

Check remaining quota before spawning:

```
curl -s http://127.0.0.1:9221/v1/quota \
  -H 'Authorization: Bearer <tenant-token>'
```

The quota response shows `ram_mb_used` and `ram_mb_limit` for the tenant. A spawn request that would push `ram_mb_used` above `ram_mb_limit` receives a 429 response. To increase a tenant's quota, update the tenant record through the provisioning tooling; quota changes are not available through the tenant API.

All spawn and destroy operations through the tenant proxy are appended to the WORM audit log. Entries are permanent and cannot be modified or deleted through any API path.

## §7 Destroy a VM

To destroy a VM through the tenant proxy:

```
curl -s -X DELETE http://127.0.0.1:9221/v1/vms/<vm_id> \
  -H 'Authorization: Bearer <tenant-token>'
```

A successful response returns 204. Verify the VM count has decreased:

```
curl http://127.0.0.1:9203/v1/fleet
```

The node that held the VM should show a lower `vm_count`. The fleet controller also removes the VM from its `GET /v1/vms` listing.

If the VM does not appear in the fleet but the destroy request returns an error, confirm the `vm_id` matches the tenant's own VMs — cross-tenant deletes are rejected with 403 regardless of whether the VM exists.

## §8 Check Orchestration-SLM

Retrieve the readiness and circuit state for the inference broker:

```
curl http://127.0.0.1:9180/readyz
```

The response includes:

- Circuit breaker states for each configured inference backend (open / half-open / closed)
- Flow gate state (open / closed) — when closed, new inference requests are rejected at the gate
- License status — any fault here closes the flow gate

Retrieve the metering audit rollup by tenant:

```
curl http://127.0.0.1:9180/v1/audit/rollup
```

The rollup response aggregates token counts and request counts per tenant identifier. Use this for billing reconciliation. Individual audit entries are append-only and are not exposed through this endpoint; the rollup reflects all recorded activity since service start.

If the circuit breaker is in an open state and the underlying cause has been resolved, the breaker will transition to half-open automatically after the configured timeout. Do not restart the service to reset the breaker; a restart clears in-flight metering state.
