# OS Boot Specifications | Especificaciones de Arranque

## 1. Fleet-Infrastructure (Laptop B)
* **Hardware Profile**: MacBook Pro (~2019-2021)
* **Target OS**: `os-infrastructure` (Stateless Hypervisor)
* **Substrate Implementation**: `system-substrate-broadcom`
* **Connectivity**: WiFi-Only (Air-Gapped Bootstrap)
* **Identity**: Machine-Based Authorization (MBA) via hardware key

## 2. Route-Network-Admin (iMac 12.1 VM)
* **Hardware Profile**: Virtual Machine (Bridged Interface)
* **Target OS**: `os-network-admin` (Authority Node)
* **Substrate Implementation**: `system-substrate` (Standard)
* **Connectivity**: Virtual Bridge to Host WiFi

## 3. Burn/Flash Protocol
1. Compile the target OS using the `no_std` toolchain.
2. Link the specialized Tier 6 Substrate.
3. Flash to the physical Totebox Archive (USB).
