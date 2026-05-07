# ⚙️ SERVICE-MARKET

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Vendor:** PointSav Digital Systems™
**Standard:** Leapfrog 2030 (Reverse-Flow Substrate — Doctrine claim #52)
**Tier:** 5 (Service Logic)

---

## I. ARCHITECTURAL MANDATE

`service-market` is the Ring 2 data marketplace layer. It manages outbound
data connectors (Snowflake, AWS Data Exchange, LiveRamp) and exposes an
inbound Delta Sharing API for structured data distribution. Runs on
`os-totebox`.

*Reserved-folder — implementation pending. See `conventions/reverse-flow-substrate.md`.*
