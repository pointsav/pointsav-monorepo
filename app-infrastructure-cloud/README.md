# app-infrastructure-cloud

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Vendor:** PointSav Digital Systems
**Crate type:** App surface — cloud relay deployment posture

---

`app-infrastructure-cloud` is the intended application-surface cartridge for the GCP cloud relay node (the hub in the PPN hub-and-spoke topology, at `10.50.0.1`). It provides the operator-facing layer above the cloud deployment OS image, exposing relay configuration, peer status, and administrative controls for the node that anchors WireGuard reachability across the mesh.

This crate is the cloud counterpart to `app-infrastructure-onprem` and `app-infrastructure-leased`.

*Reserved-folder — implementation pending. Gated on WireGuard Part A and GCP static IP provisioning.*
