<div align="center">

# moonshot-registry

[ Read in English ](./README.md)

</div>

---

> [!NOTE]
> **ESTADO DE TRADUCCIÓN:** Este componente contiene lógica de ingeniería activa o activos operativos. La documentación estructural completa se mantiene en la matriz primaria en inglés. Consulte el documento principal para obtener parámetros de ejecución detallados.

---

**Entidad:** PointSav Digital Systems (El Proveedor)
**Taxonomía:** Iniciativa Moonshot — familia `moonshot-*`
**Versión:** 0.1.0
**Estado:** Carpeta reservada — fase de investigación
**Prioridad:** MEDIA

---

## Qué reemplaza

Este crate es la implementación interna planificada de un servidor y cliente
del registro OCI Distribution Spec v1.1.0, reemplazando el crate `oci-client`
prestado en la Fase 2 de `app-privategit-bim`.

El modelo de distribución de Objetos BIM (framework PBS-1, Fase 2) utiliza
el OCI Distribution Spec para direccionar objetos BIM por hash de contenido:

```
woodfine/key-plans/private-office:small@sha256:abc123
```

## Horizonte temporal

**Horizonte medio (Fase 2 de app-privategit-bim, 2027).** La Fase 1 del
rewrite toma prestado `oci-client` para las extracciones del lado del cliente.

---

*© 2026 PointSav Digital Systems™.*
