# service-vm-host

PPN VM resource pool per-node heartbeat agent. Runs on every infrastructure node.

Polls local CPU and RAM every 10 seconds and sends a `NodeHeartbeat` to
`service-vm-fleet` at `VM_FLEET_ENDPOINT`. Accepts `CreateVm` dispatch from the
fleet controller (Phase 2 — not yet implemented).

## Environment variables

| Variable | Default | Description |
|---|---|---|
| `VM_FLEET_ENDPOINT` | — | Fleet controller URL, e.g. `http://10.8.0.9:9203` (required) |
| `VM_NODE_ID` | — | Unique node identifier, e.g. `gcp-cloud-1` (required) |
| `VM_WG_IP` | — | WireGuard IP of this node, e.g. `10.8.0.9` (required) |
| `VM_HEARTBEAT_INTERVAL_S` | `10` | Heartbeat interval in seconds |
| `RUST_LOG` | `service_vm_host=info` | Log filter |

## Systemd unit

`infrastructure/systemd/ppn/local-vm-host.service`

## Phase 1 limitations

`qemu_monitor.rs` returns an empty VM list. Phase 2 will scan UNIX monitor sockets
under `/run/vm-*/monitor.sock` to report running VM state without polling the fleet
controller.
