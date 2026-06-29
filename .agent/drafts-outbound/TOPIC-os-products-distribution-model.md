---
artifact: foundry-draft-v1
type: TOPIC
slug: topic-os-products-distribution-model
title: "os-infrastructure and os-network-admin — Distribution Model"
status: draft
created: 2026-06-29
author: totebox@project-infrastructure
route_to: project-editorial
language_protocol: PROSE-*
research_source: distribution-model-research-2026-06-29
research_claim: "Three-artifact distribution model per product; two-click install for SMBs"
research_method: Talos Linux image factory pattern + software.pointsav.com site visit + seL4 packaging research
research_verification: cross-checked live software.pointsav.com modal flow; Talos Linux docs verified
language: en
---

# os-infrastructure and os-network-admin — Distribution Model

Two products are planned/intended for software.pointsav.com:

| Product | Price | What it provides |
|---|---|---|
| `os-infrastructure` | $19 USDC | Sovereign OS for PPN infrastructure nodes |
| `os-network-admin` | $1 USDC | Mesh control plane; also available as Linux daemon |

---

## Three artifacts per product

Each product ships three artifacts per version, signed with the same Ed25519 key:

### `.iso` — Bare metal

ISO 9660 image with El Torito + GRUB2 multiboot loader. Write to USB with standard tools
(`dd`, Balena Etcher). Boot on any x86-64 machine with VT-x (or AArch64 when AArch64
target is published).

Target hardware: old laptops and desktops, leased servers, decommissioned enterprise kit.
Two-click install: download `.iso` → write to USB → boot → answer 3 questions.

### `.qcow2` — Cloud VM import

QCOW2 image (QEMU native format). Import to GCP as a custom image via `gcloud compute images import`,
or to DigitalOcean, Hetzner, Linode as a custom image. Boot the same seL4 kernel image
used for bare metal — a single `platform=` boot argument selects behavior at runtime
(inspired by the Talos Linux image factory pattern).

### Daemon AppImage / `.deb` — Linux daemon (os-network-admin only)

For `os-network-admin`, a standalone Linux binary packaged as an AppImage. No seL4 boot
required. Runs on any Linux x86-64 system with WireGuard installed.

Target: existing Linux installs (Linux Mint, Ubuntu, Debian). Joins the PPN mesh without
re-imaging the system. Same $1 USDC license token unlocks the AppImage download.

---

## License model

Payment is on Polygon PoS USDC. The software.pointsav.com modal flow:

1. User selects product and clicks "Get"
2. Modal: connect wallet → confirm USDC transaction
3. tx hash → server validates → generates Ed25519-signed license token + download token
4. Download token unlocks all three artifact formats for the purchased version
5. No account required. No email required. USDC on-chain is the receipt.

Source code is available free on GitHub. The $19 / $1 payment is for:
- The pre-built, tested binary distribution
- Ed25519 signature from the PointSav signing key (verifiable independently)
- The license token for commercial use

---

## Two-click install principle (SMB audience)

Target market: small and medium businesses with limited IT staff and no dedicated
DevOps team. Installation complexity must be ≤3 operator decisions.

**os-infrastructure bare metal:**
1. Download `.iso` (one click from software.pointsav.com)
2. Write to USB (Balena Etcher: drag → select USB → flash)
3. Boot machine → answer: node name, genesis endpoint, pairing code
4. Node appears in fleet. Done.

**os-network-admin daemon:**
1. Download `.AppImage` (one click)
2. `chmod +x os-network-admin-*.AppImage && ./os-network-admin-*.AppImage`
3. Configure WireGuard endpoint (guided by TUI)
4. Node joins mesh. Done.

Any step that adds complexity beyond this model is product scope reduction work, not a feature.

---

## Gate for listing

Before software.pointsav.com is updated to list either product, the three-node mesh test
must pass:

1. Laptop A: os-infrastructure ISO boot → seL4 + Linux guest + WireGuard peer registers
2. foundry-workspace: os-infrastructure QCOW2 under QEMU/TCG → peer registers
3. iMac Linux Mint: os-network-admin daemon → peer registers

All three nodes visible in `service-vm-fleet` → upload and list.

---

## Comparison — Talos Linux pattern

Talos Linux uses a single kernel image across bare metal, cloud, and container targets.
A runtime boot argument (`platform=metal`, `platform=gcp`, `platform=aws`) selects
behavior. This is the distribution pattern adopted for os-infrastructure: same seL4
kernel image, `platform=` selects the Linux guest configuration (bridge networking for
bare metal vs. virtio for cloud VM import).
