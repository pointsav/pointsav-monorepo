---
artifact: guide-draft
foundry-draft-v1: true
language_protocol: PROSE
route_to: project-editorial
status: draft
created: 2026-06-11
archive: project-infrastructure
research_trail:
  session: totebox@project-infrastructure 2026-06-11
  sources: [infrastructure/virt/setup-vm-host-laptop.sh, live deployment 2026-06-11]
  claim_verification: pending editorial review
  bcsc_reviewed: false
  bilingual_pair: none
---

# GUIDE: Joining a Node to an Existing PPN

Operational runbook for adding a Linux machine — bare metal, leased server, or cloud VM — to a running Pointsav Private Platform Network. Audience: an operator comfortable with a shell, SSH, and systemd, starting from a fresh Linux install.

## 1. Prerequisites

Before starting, confirm:

- **OS:** Ubuntu 22.04 or 24.04 (server or desktop). Derivatives such as Linux Mint work; the June 2026 deployment included one Mint node.
- **Internet access** from the candidate node — required for package installation and the base image download (~630 MB).
- **A reachable genesis node** — an existing PPN node whose WireGuard endpoint the new node can reach over UDP. In the reference topology this is the cloud relay or the hub node.
- **Memory:** the node should have at least 2 GB available after OS overhead. Nodes in the June 2026 mesh contributed 2.5–3.1 GB.
- **KVM (optional but preferred):** check with `ls /dev/kvm`. Bare-metal machines normally have it; many cloud VMs do not. A node without KVM can still join — the fleet controller weights placement toward KVM-capable nodes.

## 2. What the setup does

The automated setup (run over SSH from the fleet's control machine) performs the following on the new node. You do not run these by hand; this is what to expect and verify.

1. **Installs the QEMU/KVM stack:** `qemu-system-x86`, `qemu-utils`, `genisoimage` via apt.
2. **Downloads the base image:** the Ubuntu 24.04 cloud image (~630 MB), cached locally. Every VM spawned on this node uses a copy-on-write qcow2 disk backed by this image, so the download happens once.
3. **Installs the service-vm-host binary** — the per-node agent that accepts spawn requests and reports heartbeats.
4. **Configures WireGuard:** writes the interface config, adds the node as a peer of the mesh, brings the interface up.
5. **Writes and starts a systemd unit** for service-vm-host, with environment variables identifying the node (node ID, advertised mesh address, fleet controller URL).

## 3. Deployment profiles — what differs

| | Bare metal (old laptop/desktop) | Leased server / VPS | Cloud VM (e.g. GCP) |
|---|---|---|---|
| KVM | Yes (verify `/dev/kvm`) | Usually yes on dedicated; varies on VPS | Often absent; QEMU falls back to software emulation (slow) |
| WireGuard address | Assigned from mesh range (e.g. 10.8.0.6) | Assigned from mesh range | Assigned from mesh range (e.g. 10.8.0.9) |
| Public reachability | Behind NAT — node dials out to the hub | Usually has a public IP; can serve as hub | Public IP; typical relay/hub candidate |
| Power management | **Disable suspend/hibernate** — a sleeping laptop drops out of the mesh | n/a | n/a |
| Interface conflicts | If `wg0` is already in use for another network, create the PPN interface as `wg1` (done for Laptop A on 2026-06-11) | Same check applies | Same check applies |

## 4. Manual steps that still require operator presence

Automation covers everything except initial trust. On a fresh machine, an operator must run three short commands locally before remote setup can proceed:

1. **Install the SSH server:**
   ```
   sudo apt-get install -y openssh-server
   ```
2. **Authorize the fleet SSH key** — append the fleet control machine's public key to `~/.ssh/authorized_keys`.
3. **Grant passwordless sudo** for the setup user:
   ```
   echo "<user> ALL=(ALL) NOPASSWD:ALL" | sudo tee /etc/sudoers.d/ppn-setup
   ```

After these three commands, all remaining setup is performed remotely over SSH. Additionally, **peer admission is currently a manual operator action** — see Known limitations (§7): the genesis/hub node's WireGuard peer table must be edited by hand to admit the new node.

## 5. What the node contributes once joined

- **Heartbeat:** service-vm-host reports to the fleet controller every 10 seconds — available memory (read from `/proc/meminfo`), KVM availability, and node liveness.
- **Spawn server on :9220:** the node accepts delegated `POST` spawn requests from the fleet controller and launches QEMU VMs locally (copy-on-write disk, cloud-init seed ISO, user-mode networking).
- **Cached base image:** the Ubuntu 24.04 cloud image held locally, so spawns do not re-download.

The fleet controller (`service-vm-fleet`, port :9203) factors the node into advisory placement immediately once heartbeats arrive: requests are routed to the node with the most available RAM, with KVM-capable nodes preferred.

## 6. Verification

Run these from any machine on the mesh (or via SSH to a mesh node).

**a. Confirm the node appears in the fleet:**
```
curl http://<fleet-controller-mesh-ip>:9203/v1/fleet
```
The new node should be listed with a recent heartbeat timestamp, its reported memory, and KVM flag. If it is absent, check that the WireGuard interface is up (`sudo wg show`) and that the service-vm-host unit is active (`systemctl status service-vm-host`).

**b. Test a spawn end to end:**
```
curl -X POST http://<fleet-controller-mesh-ip>:9203/v1/vms \
  -H 'Content-Type: application/json' \
  -d '{"name": "verify-node-join", "memory_mb": 512}'
```
Expect a `VmRecord` response in `Provisioning` state. Note which node the controller selected — if the new node has the most free RAM and KVM, placement should land on it (as it did for laptop-a-1 in the 2026-06-11 live test). Confirm on the selected node that a QEMU process is running:
```
pgrep -a qemu-system-x86_64
```

## 7. Known limitations (as of 2026-06-11)

- **Peer admission is not yet automated.** os-network-admin's WireGuard automation (`wg set` / `wg addconf` across nodes) is Phase S3 and currently a simulation stub. Admitting a node means hand-editing the hub's peer table. Plan operator time for this step.
- **seL4 isolation is planned, not current.** Host-level isolation today is conventional Linux + QEMU/KVM. A party with root on the physical host can inspect guest workloads. Do not represent the node as host-isolated.
- **WireGuard changes require an explicit save.** Runtime `wg set` changes are lost on interface restart unless persisted (`wg-quick save <iface>` or editing `/etc/wireguard/<iface>.conf`). Verify the config file reflects the live peer table before closing out.
- **Placement is advisory, not enforced.** The controller selects a node and delegates; there is no rebalancing (`auto_rebalance: false`) and no enforcement if a node's reported state is stale.
- **No KVM means slow VMs.** Nodes without `/dev/kvm` (most cloud VMs in the current fleet) run QEMU in software emulation. They remain useful as controllers/relays; treat their spawn capacity as limited.
