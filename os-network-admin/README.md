# 💿 OS Network Admin (Node 3: Brain)

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

### *Mesh Orchestration & Routing Authority*

**Current Silicon Target:** iMac 12,1 (Mid-2011)
* **CPU:** Intel Sandy Bridge i5-2400S (Entry: 0x1002a3)
* **NIC:** Broadcom 14e4:16b4 

## 📜 Architectural Mandate
This crate generates the bootable ISO for the infrastructure routing gateway. It is strictly responsible for establishing the PointSav Private Network (PPN) over the PSST tunnels. 

**Zero Cryptographic Authority:** `os-network-admin` handles packet routing and tunnel integrity only. It does *not* hold F-Keys, Machine-Based Authorization (MBA) credentials, or payload capabilities. It acts as a blind, secure transport layer for the `os-console` delivery vehicles.
