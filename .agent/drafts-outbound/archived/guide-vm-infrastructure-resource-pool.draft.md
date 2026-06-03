---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: woodfine-fleet-deployment
target_path: fleet-infrastructure-cloud-1/
target_filename: guide-vm-infrastructure-resource-pool.md
target_cluster: fleet-infrastructure-cloud-1
audience: operators deploying the PPN VM resource pool across the three-node WireGuard mesh
bcsc_class: no-disclosure-implication
language_protocol: GUIDE
authored: 2026-05-29
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
notes_for_editor: >
  New guide — no existing file at this path. English only (GUIDEs are EN-only).
  This guide assumes the WireGuard mesh (Part A-lite, 10.8.0.0/24) is already live.
  service-vm-fleet and service-vm-host are new crates scaffolded in session 12.
  Binaries will not be available until Stage 6 promotes the session 12 commits.
  Steps 2-4 are "when binaries are available" instructions — not executable today.
  Step 1 (GCP nested KVM) is an operator action, executable now.
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
---

# Guide: VM Infrastructure Resource Pool

This guide sets up the VM resource pool across the three-node WireGuard mesh:
Laptop A, Laptop B, and the GCP node. When complete, `app-network-admin` F9 can
create and destroy virtual machines on any node in the mesh with a single operator
confirmation.

**Prerequisites:**
- WireGuard mesh live: Laptop A (10.8.0.6), Laptop B (10.8.0.1), GCP (10.8.0.9)
- SSH access from GCP to all nodes over WireGuard
- `service-vm-fleet` and `service-vm-host` binaries built and installed
  (Stage 6 promotion of session 12 commits required)

---

## Step 1 — Enable GCP Nested Virtualisation (operator action)

The GCP workspace VM currently runs QEMU in TCG (software emulation) mode because
nested virtualisation is not enabled. Without KVM, VM boot times are approximately
ten times slower than on hardware with VT-x.

In the GCP console:

1. Navigate to **Compute Engine → VM instances → foundry-workspace**.
2. Click **Edit**.
3. Under **CPU platform and GPU**, enable **Virtualized nested hardware performance counters**.
4. Click **Save**, then **Restart**.

Verify after restart:
```bash
ls /dev/kvm
# Expected: /dev/kvm (character device)
```

**Laptop A:** VT-x is present on the Sandy Bridge i5-2400S. Confirm KVM is available:
```bash
# On Laptop A
ls /dev/kvm
```
If absent, load the module: `sudo modprobe kvm kvm_intel`

---

## Step 2 — Deploy service-vm-fleet on GCP

`service-vm-fleet` is the fleet controller. It receives heartbeats from all nodes,
tracks available resources, and handles VM placement requests. It runs only on GCP.

Install the binary:
```bash
# On GCP VM (10.8.0.9)
sudo cp /srv/foundry/clones/project-infrastructure/target/release/service-vm-fleet \
     /usr/local/bin/service-vm-fleet
sudo chmod 755 /usr/local/bin/service-vm-fleet
```

Enable the systemd unit:
```bash
sudo cp /srv/foundry/clones/project-infrastructure/infrastructure/systemd/orchestration/local-vm-fleet.service \
     /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now local-vm-fleet
```

Required environment variables (set in the systemd unit or `/etc/default/vm-fleet`):
```
VM_NODE_ID=gcp-cloud-1
VM_WG_IP=10.8.0.9
VM_HEARTBEAT_INTERVAL_S=10
```

Verify:
```bash
systemctl status local-vm-fleet
curl http://localhost:9203/v1/fleet
# Expected: {"nodes":[],"last_updated":"..."}
```

---

## Step 3 — Deploy service-vm-host on Each Node

`service-vm-host` runs on every infrastructure node — GCP, Laptop A, and Laptop B.
It polls local resources and reports to the fleet controller.

**On GCP (10.8.0.9):**
```bash
sudo cp /srv/foundry/clones/project-infrastructure/target/release/service-vm-host \
     /usr/local/bin/service-vm-host
sudo cp infrastructure/systemd/ppn/local-vm-host.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now local-vm-host
```

**On Laptop A (10.8.0.6) and Laptop B (10.8.0.1):** copy binary via SCP, then enable:
```bash
# From GCP — copy to Laptop A
scp /usr/local/bin/service-vm-host mathew@10.8.0.6:/tmp/
scp infrastructure/systemd/ppn/local-vm-host.service mathew@10.8.0.6:/tmp/

# On Laptop A
sudo mv /tmp/service-vm-host /usr/local/bin/
sudo mv /tmp/local-vm-host.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now local-vm-host
```

Required environment variables (set in the systemd unit):
```
VM_FLEET_ENDPOINT=http://10.8.0.9:9203
VM_NODE_ID=<node-id>     # laptop-a-1 or laptop-b-1
VM_WG_IP=<wg-ip>         # 10.8.0.6 or 10.8.0.1
VM_HEARTBEAT_INTERVAL_S=10
```

---

## Step 4 — Verify All Three Nodes Appear

Wait 15 seconds for the first heartbeat cycle, then:
```bash
curl http://10.8.0.9:9203/v1/fleet | python3 -m json.tool
```

Expected output (all three nodes, each within `last_heartbeat < 15s`):
```json
{
  "nodes": [
    {"node_id": "gcp-cloud-1", "hostname": "foundry-workspace", "wg_ip": "10.8.0.9", "ram_available_mb": ..., "vm_count": 0, "last_heartbeat": "..."},
    {"node_id": "laptop-a-1",  "hostname": "...",                "wg_ip": "10.8.0.6", "ram_available_mb": ..., "vm_count": 0, "last_heartbeat": "..."},
    {"node_id": "laptop-b-1",  "hostname": "...",                "wg_ip": "10.8.0.1", "ram_available_mb": ..., "vm_count": 0, "last_heartbeat": "..."}
  ],
  "last_updated": "..."
}
```

---

## Step 5 — Create a Virtual Machine

Creating a VM is an operator action and requires F12 confirmation in the
`app-network-admin` F9 panel (SYS-ADR-10). To test the API directly:

```bash
# Request a VM-MediaKit instance (2 GiB RAM, 2 vCPUs)
# The fleet controller selects the node with most available RAM
curl -s -X POST http://10.8.0.9:9203/v1/vms \
  -H 'Content-Type: application/json' \
  -d '{"vm_type":"VmMediaKit","ram_mb":2048,"vcpu_count":2}' \
  | python3 -m json.tool
```

Expected: a `VmRecord` with `state: "Provisioning"` and the assigned node ID.

**VM-Totebox instances must always specify `preferred_node`** because WORM archive data
cannot be migrated over WireGuard:
```bash
curl -s -X POST http://10.8.0.9:9203/v1/vms \
  -H 'Content-Type: application/json' \
  -d '{"vm_type":"VmTotebox","ram_mb":4096,"vcpu_count":2,"preferred_node":"laptop-a-1"}' \
  | python3 -m json.tool
```

---

## Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| Node not appearing in `/v1/fleet` after 30s | Heartbeat not reaching GCP | Check firewall allows TCP 9203 inbound on GCP; check `service-vm-host` logs (`journalctl -u local-vm-host`) |
| Placement fails: "insufficient RAM on all nodes" | All nodes below request + 512 MB threshold | Reduce `ram_mb` in request, or free memory on a node |
| GCP VMs boot in >300s | TCG emulation active | Enable nested KVM (Step 1) |
| `service-vm-fleet` exits immediately | Missing env var `VM_NODE_ID` | Check `/etc/systemd/system/local-vm-fleet.service` has all three env vars set |
| Laptop A or B shows `vm_count` but `ram_available_mb: 0` | `/proc/meminfo` read failed | Verify `service-vm-host` runs as a user with read access to `/proc`; root is safest for Phase 1 |

---

## Notes

**Live migration is not available.** VMs are placed once and remain on their assigned
node. This is by design — WireGuard at typical internet speeds would take 30–40 minutes
to transfer a 6 GB VM image, making live migration impractical. Plan VM placement at
creation time.

**Node eviction is automatic.** If a node's heartbeat stops for more than 30 seconds,
the fleet controller removes it from the active pool. VMs on that node are marked with
state `Error` in the fleet view. Restore connectivity to the node and restart
`service-vm-host` to re-register.

**Phase 2 upgrade path.** When Phase 2 (NetBSD/NVMM) hosts replace the Ubuntu 24.04
hosts, the `service-vm-host` binary continues to work unchanged — it reads `/proc/meminfo`
which is available on NetBSD via `linprocfs`. No code changes are required for Phase 2.
