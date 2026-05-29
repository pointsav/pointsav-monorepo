# system-vm-fleet-types

Shared wire types for the PPN VM resource pooling layer.

Used by `service-vm-fleet` (fleet controller, GCP-resident) and `service-vm-host`
(per-node heartbeat agent). `no_std`-compatible design; no tokio dependency.

## Types

| Type | Purpose |
|---|---|
| `NodeHeartbeat` | Sent by service-vm-host to service-vm-fleet every 10s |
| `VmRecord` | Single VM as reported by the host agent |
| `VmState` | Running / Stopped / Provisioning / Error |
| `PlacementAdvice` | Advisory placement result from service-vm-fleet |
| `FleetStatus` | Snapshot of entire fleet |
| `NodeRecord` | Summary of a single node in the fleet |
| `CreateVmRequest` | Request to create a new VM; preferred_node required for VmTotebox |

## Architecture invariant

`auto_rebalance` is permanently `false`. Live VM migration over WireGuard is excluded —
bandwidth constraints (~20 Mbps × 6 GB VM ≈ 40 min) make it impractical.
VMs are placed once and remain on their assigned node.
