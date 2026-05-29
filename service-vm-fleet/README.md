# service-vm-fleet

PPN VM resource pool fleet controller. Runs on the GCP node, listens on `:9203`.

Receives heartbeats from `service-vm-host` running on each infrastructure node,
tracks available resources, and provides advisory VM placement.

## Endpoints

| Method | Path | Description |
|---|---|---|
| `POST` | `/v1/nodes/heartbeat` | Update node record from heartbeat |
| `GET` | `/v1/fleet` | FleetStatus — all registered nodes |
| `GET` | `/v1/nodes/:node_id` | Single node record |
| `POST` | `/v1/vms` | Create VM — advisory placement + dispatch |
| `DELETE` | `/v1/vms/:vm_id` | Destroy VM |

## Placement rules

1. Filter: `ram_available_mb >= request.ram_mb + 512` (512 MB safety margin)
2. Sort: `ram_available_mb DESC`
3. First candidate wins

`auto_rebalance` is permanently `false`. Live VM migration is excluded.
VM-Totebox instances must specify `preferred_node` — WORM data cannot migrate.

## Environment variables

| Variable | Default | Description |
|---|---|---|
| `VM_NODE_ID` | — | Node ID for this host (required) |
| `VM_WG_IP` | — | WireGuard IP for this host (required) |
| `RUST_LOG` | `service_vm_fleet=info` | Log filter |

## Systemd unit

`infrastructure/systemd/orchestration/local-vm-fleet.service`
