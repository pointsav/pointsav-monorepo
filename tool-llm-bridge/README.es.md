# tool-llm-bridge

Tier X — un puente aislado de investigación y asistencia de código con un LLM
externo. Se ejecuta como su propio proceso, separado del bucle soberano
principal (service-content, service-slm), de modo que aunque el proceso del
bucle principal fuera comprometido, no podría leer la clave real de la API
externa.

Consulte `BRIEF-os-totebox-platform.md` §7 y §14 #12 para la justificación
completa del diseño y su relación con SYS-ADR-07.

## Para qué sirve

Asistencia de investigación y escritura de código para el desarrollo propio de
este archivo. **Nunca** se usa para extracción de entidades, escrituras al
DataGraph, o generación de señales de entrenamiento — este crate no tiene
ninguna dependencia de cliente DataGraph, por construcción.

## Diseño

- **Aislamiento de credenciales tipo sidecar.** Solo este proceso posee la
  clave real de la API externa (`LLM_BRIDGE_PROVIDER_API_KEY`). Los llamadores
  se autentican con un token local independiente y no relacionado
  (`LLM_BRIDGE_ACCESS_TOKEN`) y nunca ven la clave real — se inyecta en la
  solicitud saliente aquí y nunca se repite en ninguna respuesta.
- **Lista blanca por etiqueta**, verificada antes de cualquier intento de red
  — refleja el diseño existente de Tier C en `service-slm`
  (`slm-doorman/src/tier/external.rs`). Vacía por defecto: ninguna etiqueta
  está permitida hasta que se configure explícitamente.
- **Puerta de etiquetado.** Cada solicitud debe llevar un `tag` explícito:
  `local-only` o `cloud-allowed`. Solo las solicitudes `cloud-allowed` pueden
  llegar al proveedor externo — esto es una verificación estructural, no una
  convención del llamador.

## Configuración (variables de entorno)

| Variable | Requerida | Por defecto | Propósito |
|---|---|---|---|
| `LLM_BRIDGE_BIND_ADDR` | no | `127.0.0.1:9210` | Dirección de enlace HTTP |
| `LLM_BRIDGE_ACCESS_TOKEN` | **sí** | — | Token local que deben presentar los llamadores |
| `LLM_BRIDGE_PROVIDER_ENDPOINT` | no | vacío (rechaza todo) | URL del proveedor externo |
| `LLM_BRIDGE_PROVIDER_API_KEY` | no | vacío (rechaza todo) | Credencial real del proveedor |
| `LLM_BRIDGE_ALLOWED_LABELS` | no | vacío (rechaza todo) | Lista blanca de etiquetas, separadas por comas |

## API

`POST /v1/bridge/complete`

```json
{
  "tag": "cloud-allowed",
  "label": "research",
  "payload": { "...": "reenviado tal cual al proveedor externo" }
}
```

`GET /healthz` — verificación de disponibilidad.

## Estado

Fase 1 del plan de ejecución de `BRIEF-os-totebox-platform.md` §16. MVP:
aislamiento de proceso y credenciales, lista blanca, puerta de etiquetado.
Aún no desplegado en ningún lugar; aún no está en
`.agent/binary-targets.yaml` (declarar antes de distribuir — ver
`conventions/soft-distribution-pipeline.md` §3, `soft_enabled: false` es
correcto hasta que esto esté listo para producción).
