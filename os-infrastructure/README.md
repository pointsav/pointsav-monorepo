# 💿 OS Infrastructure (Node 1: Muscle)
This crate generates the bootable ISO for the bare-metal execution nodes.

**Current Silicon Target:** MacBookPro7,1 (Mid-2010)
* **CPU:** Intel Penryn P8600
* **NIC:** Broadcom 14e4:432b (Para-Virtualized Bridge)

## 🛠️ Build Orchestration
The `/build_iso` directory contains the specialized scripts required to:
1. Compile the Rust `no_std` source into a 64-bit ELF.
2. Wrap the binary in a GRUB Multiboot2 header.
3. Forge the final `.iso` file for physical deployment.

*Note: This ISO is Silicon-Pinned. It is delivered to the Woodfine Fleet for deployment on identical hardware.*
