# system-network-interface

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Vendor:** PointSav Digital Systems
**Crate type:** System substrate — bare-metal network interface lib

---

`system-network-interface` is the bare-metal NIC adapter substrate used by `os-infrastructure`. It provides the hardware abstraction layer between the Multiboot2 OS image and the network interface hardware, with no standard library dependency.

This crate is a library only (`lib.rs`). The F8 Terminal Gateway binary — the async HTTP/UDP command surface that previously lived in this crate's `main.rs` — was extracted to `app-network-admin` to resolve an incompatibility between the `no_std` library and the tokio/warp runtime requirements of the gateway.
