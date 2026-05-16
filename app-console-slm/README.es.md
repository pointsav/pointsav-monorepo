<div align="center">

# app-console-slm | Consola de Infraestructura Soberana

[ 🇬🇧 Read this document in English ](./README.md)

**Entidad:** PointSav Digital Systems™ (El Proveedor)
**Taxonomía:** Consola de Aplicación — Infraestructura de IA

</div>

<br/>

## I. PROPÓSITO

`app-console-slm` es la consola de operación para la infraestructura de IA de Foundry. Consolida la visibilidad y el control de `service-slm` (Doorman), la VM de inferencia Yo-Yo, el corpus de entrenamiento y el almacén de entidades DataGraph en un único binario. Reemplaza las verificaciones manuales con curl, `start-yoyo.sh`, el acceso SSH directo a la VM Yo-Yo y el script `slm-chat.sh`.

## II. SUPERFICIE DEL BINARIO

### Sprint 4a — Planificado

**`console-slm status`** — Verificación de estado en un solo comando.

**`console-slm admin`** — TUI de operación (ratatui) con paneles para: estado de tiers Doorman, controles de la VM Yo-Yo, registro en tiempo real de ejecución nocturna, estadísticas del corpus, conteo de entidades DataGraph y resumen del registro de aprendizaje.

### Sprint 4b — Diferido

**`console-slm code`** y **`console-slm chat`** están diferidos hasta que el tamaño del equipo justifique la inversión y el modelo soberano local sea competitivo con el nivel de modelo externo actual.

## III. ESTADO ACTUAL

Sprint 4a: planificado. El crate es un marcador estructural. La implementación comienza después de que Sprint 3 (servidor MCP) esté en producción.

---
*© 2026 PointSav Digital Systems™.*
