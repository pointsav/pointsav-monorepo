# system-network-interface

<div align="center">

[ 🇬🇧 Read this document in English ](./README.md)

</div>

**Proveedor:** PointSav Digital Systems
**Tipo de paquete:** Sustrato de sistema — biblioteca de interfaz de red bare-metal

---

`system-network-interface` es el sustrato adaptador NIC bare-metal utilizado por `os-infrastructure`. Proporciona la capa de abstracción de hardware entre la imagen de OS Multiboot2 y el hardware de la interfaz de red, sin dependencia de la biblioteca estándar.

Este paquete es únicamente una biblioteca (`lib.rs`). El binario F8 Terminal Gateway — la superficie de comandos HTTP/UDP asíncrona que anteriormente residía en el `main.rs` de este paquete — fue extraído a `app-network-admin` para resolver una incompatibilidad entre la biblioteca `no_std` y los requisitos de tokio/warp del gateway.
