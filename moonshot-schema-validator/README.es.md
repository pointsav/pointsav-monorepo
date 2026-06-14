<div align="center">

# moonshot-schema-validator

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
**Prioridad:** ALTA

---

## Qué reemplaza

Este crate es el reemplazo interno planificado para dos dependencias prestadas
en `app-privategit-bim`:

1. **`jsonschema` Rust crate v0.20+** (MIT) — validación JSON Schema del lado
   del servidor en el controlador POST /edit/:slug
2. **`ajv` v8.x** (MIT, JS vendorizado, ~120KB) — validación JSON en tiempo
   real del lado del cliente en el panel del navegador CodeMirror

El objetivo es una única implementación en Rust compilada una vez y desplegada
dos veces: como binario nativo en el servidor y como módulo WASM en el navegador.

## Horizonte temporal

**Horizonte medio (2027–2028).** Prerequisito: `app-privategit-bim` v1 debe
estar en producción antes de invertir en el reemplazo propio.

---

*© 2026 PointSav Digital Systems™.*
