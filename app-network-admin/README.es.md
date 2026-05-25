# app-network-admin

<div align="center">

[ 🇬🇧 Read this document in English ](./README.md)

</div>

**Proveedor:** PointSav Digital Systems
**Tipo de paquete:** Superficie de aplicación — nodo de administración de red

---

`app-network-admin` es el binario F8 Terminal Gateway para el nodo `os-network-admin`. Proporciona dos superficies:

- **Superficie de comandos HTTP** (`POST /translate`, `POST /authorize`, `POST /upload`) en el puerto 8085 — acepta la intención del operador en lenguaje natural, la enruta a través de `service-slm` para producir un comando autorizado y lo envía a la malla
- **Difusión UDP en la malla** en el puerto 8090 — envía cargas útiles JSON firmadas a las tres direcciones de pares de la PPN (`10.50.0.1`, `10.50.0.2`, `10.50.0.3`)

Este paquete se extrajo de `system-network-interface`, que contiene únicamente la biblioteca de sustrato NIC bare-metal utilizada por `os-infrastructure`. El F8 Gateway requiere un entorno de ejecución asíncrono estándar (tokio + warp) y no puede coexistir en el mismo paquete que una biblioteca bare-metal `no_std`.

## Limitaciones conocidas

- `handle_translation` invoca `/opt/pointsav/f8-gateway/system-slm` — una ruta de binario codificada de forma fija. La arquitectura prevista enruta a través de la API HTTP del Portero `service-slm`. Esto se registra como un elemento de alineación pendiente.
- Los comandos de la malla se envían como cadenas JSON por UDP. El protocolo previsto es un formato de paquete binario de 16 bytes. Consulte el artículo de `os-network-admin` para la especificación objetivo.
- La resolución del nodo destino (`NODE-CLOUD-RELAY`, `NODE-LAPTOP-A`, `NODE-IMAC-12`) utiliza direcciones IP codificadas de forma fija. Estas serán suministradas por el registro de emparejamiento una vez que se implemente el Genesis Protocol.
