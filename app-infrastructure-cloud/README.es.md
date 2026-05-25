# app-infrastructure-cloud

<div align="center">

[ 🇬🇧 Read this document in English ](./README.md)

</div>

**Proveedor:** PointSav Digital Systems
**Tipo de paquete:** Superficie de aplicación — postura de despliegue en la nube

---

`app-infrastructure-cloud` es el paquete de superficie de aplicación previsto para el nodo de retransmisión en GCP (el hub en la topología de concentrador y radios de la PPN, en `10.50.0.1`). Proporciona la capa orientada al operador sobre la imagen de OS para el despliegue en la nube, exponiendo la configuración de retransmisión, el estado de los pares y los controles administrativos para el nodo que ancla la accesibilidad WireGuard en toda la malla.

Este paquete es el equivalente en la nube de `app-infrastructure-onprem` y `app-infrastructure-leased`.

*Directorio reservado — implementación pendiente. Condicionado a WireGuard Parte A y al aprovisionamiento de IP estática en GCP.*
