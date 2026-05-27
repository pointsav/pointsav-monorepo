# app-console-system

Cartucho del sistema F11 para os-console — panel de operador para aprobaciones de solicitudes de conexión pendientes.

## Descripción general

Proporciona el `SystemCartridge` que implementa el trait `Cartridge`. Consulta
`GET /v1/pair/pending` cada cinco segundos y presenta las solicitudes de conexión
pendientes en una lista. Acciones del operador:

| Tecla | Acción |
|-------|--------|
| ↑ / k | Mover selección hacia arriba |
| ↓ / j | Mover selección hacia abajo |
| Enter | Aprobar solicitud seleccionada |
| D | Rechazar solicitud seleccionada |
| R | Actualizar manualmente |

Un contador de insignia se muestra en la barra de estado del chasis cuando hay solicitudes pendientes.

## Dependencias

- `app-console-keys` — trait Cartridge y FKey
- `reqwest` (blocking) — llamadas HTTP al servidor de emparejamiento
