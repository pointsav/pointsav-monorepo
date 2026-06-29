---
artifact: foundry-draft-v1
type: GUIDE
slug: guide-installing-os-network-admin-daemon
title: "Installing os-network-admin (Linux daemon)"
status: draft
created: 2026-06-29
author: totebox@project-infrastructure
route_to: project-editorial
language_protocol: PROSE-*
research_source: distribution-model-research-2026-06-29
research_claim: "Daemon mode for os-network-admin on existing Linux systems"
research_method: WireGuard Linux install docs + AppImage packaging pattern + Phase S3 implementation
research_verification: Phase S3 (commit 13ef4654) tested on fleet; WireGuard Mint apt install verified
language: en
audience: SMB operator; Linux Mint / Ubuntu / Debian experience assumed
---

# Installing os-network-admin (Linux daemon)

os-network-admin manages the WireGuard mesh for a PPN fleet. In daemon mode, it runs on
an existing Linux system without re-imaging — install alongside your current OS.

**Price:** $1 USDC at software.pointsav.com.

**Supported systems:** Linux x86-64. Linux Mint 21.x, Ubuntu 22.04+, Debian 12+.
Tested target: iMac (2010–2012, Intel x86-64) running Linux Mint.

---

## Prerequisites

### WireGuard

os-network-admin uses WireGuard for all mesh communication. Install it first:

```bash
# Debian / Ubuntu / Linux Mint
sudo apt update && sudo apt install wireguard wireguard-tools

# Verify
which wg
```

### Elevated capability

os-network-admin needs `CAP_NET_ADMIN` to manage WireGuard peer tables. The simplest
approach is running as root or with `sudo`. For production, see the capability hardening
section below.

---

## Installation

1. **Purchase and download** from software.pointsav.com ($1 USDC). You receive a download
   token. Download `os-network-admin-<ver>-x86_64.AppImage`.

2. **Make executable and run:**
   ```bash
   chmod +x os-network-admin-<ver>-x86_64.AppImage
   sudo ./os-network-admin-<ver>-x86_64.AppImage
   ```

3. **Initial setup.** On first run, the TUI (terminal UI) prompts for:
   - **Node name:** short identifier for this machine (e.g., `imac-1`)
   - **Genesis endpoint:** WireGuard endpoint of the genesis relay node
   - **WireGuard interface:** usually `wg0`

   The daemon generates a WireGuard keypair, sends a join request to the genesis node,
   and waits for an operator to approve the pairing (via `os-network-admin` on the genesis
   node, or via the os-infrastructure control plane).

4. **After approval,** os-network-admin writes the WireGuard configuration to `/etc/wireguard/wg0.conf`
   and brings up the interface:
   ```bash
   # Automatic (done by daemon) but verifiable:
   sudo wg show wg0
   ```

5. **Verify fleet registration.** On any fleet management node:
   ```bash
   curl http://<fleet-node-ip>:9203/nodes
   ```
   The new node should appear within 30 seconds.

---

## Running as a systemd service

For production use, run os-network-admin as a systemd unit:

```bash
# Copy the binary to a stable location
sudo cp os-network-admin-<ver>-x86_64.AppImage /usr/local/bin/os-network-admin
sudo chmod +x /usr/local/bin/os-network-admin

# Create the unit file
sudo tee /etc/systemd/system/os-network-admin.service << 'EOF'
[Unit]
Description=os-network-admin PPN mesh control plane
After=network.target

[Service]
ExecStart=/usr/local/bin/os-network-admin
Restart=on-failure
RestartSec=10
AmbientCapabilities=CAP_NET_ADMIN
CapabilityBoundingSet=CAP_NET_ADMIN

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable --now os-network-admin
```

---

## iMac 2010–2012 specific notes

- **VT-x support:** Sandy Bridge iMac (late 2011+, Core i5/i7) has VT-x. Earlier iMac
  (mid-2010, Core 2 Duo Westmere) does not. Daemon mode does not require VT-x.
- **RAM:** 4 GB minimum. The daemon itself uses < 50 MB; WireGuard kernel module < 5 MB.
- **Linux Mint:** WireGuard is in the standard Mint 21.x repository. `apt install wireguard` works.
- **Boot order:** Install Linux Mint first via its standard installer. Then install os-network-admin.
  No re-imaging required.

---

## Verifying the connection

```bash
# WireGuard peer status
sudo wg show wg0

# Should show the genesis relay + any approved peers
# Example:
# peer: <genesis-public-key>
#   endpoint: 34.x.x.x:51820
#   latest handshake: X seconds ago
#   transfer: X MiB received, X MiB sent

# Fleet registration
curl http://<fleet-wg-ip>:9203/nodes | python3 -m json.tool
```

---

## Troubleshooting

| Problem | Check |
|---|---|
| "wg: command not found" | `sudo apt install wireguard-tools` |
| Pairing request not appearing | Confirm genesis node is reachable: `ping <genesis-public-ip>` |
| "CAP_NET_ADMIN: Operation not permitted" | Run with sudo or add `AmbientCapabilities=CAP_NET_ADMIN` in systemd unit |
| iMac WiFi not working as WireGuard endpoint | Use Ethernet for genesis node connectivity; WiFi works for traffic after handshake |
