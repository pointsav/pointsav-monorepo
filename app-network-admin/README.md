# app-network-admin

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Vendor:** PointSav Digital Systems
**Crate type:** App surface — network admin node

---

`app-network-admin` is the F8 Terminal Gateway binary for the `os-network-admin` node. It provides two surfaces:

- **HTTP command surface** (`POST /translate`, `POST /authorize`, `POST /upload`) on port 8085 — accepts plain-language operator intent, routes it through `service-slm` to produce an authorised command, and dispatches it to the mesh
- **UDP mesh broadcast** on port 8090 — sends signed JSON payloads to the three PPN peer addresses (`10.50.0.1`, `10.50.0.2`, `10.50.0.3`)

This crate was extracted from `system-network-interface`, which carries only the bare-metal NIC substrate lib used by `os-infrastructure`. The F8 Gateway requires a standard async runtime (tokio + warp) and cannot coexist in the same crate as a `no_std` bare-metal library.

## Known limitations

- `handle_translation` shells out to `/opt/pointsav/f8-gateway/system-slm` — a hardcoded binary path. The intended architecture routes through the `service-slm` Doorman HTTP API instead. This is tracked as a pending alignment item.
- Mesh commands are sent as JSON strings over UDP. The intended protocol is a 16-byte binary packet format. See `os-network-admin` TOPIC for the target specification.
- Target node resolution (`NODE-CLOUD-RELAY`, `NODE-LAPTOP-A`, `NODE-IMAC-12`) uses hardcoded IP addresses. These will be driven by the pairing registry once the Genesis Protocol implementation lands.
