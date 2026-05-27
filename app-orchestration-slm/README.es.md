# app-orchestration-slm

Chasis intermediario de Yo-Yo — conecta múltiples Archivos Totebox a una flota compartida de GPU Yo-Yo.

**Licencia:** Propietaria. Consulte `CLAUDE.md` para el límite del nivel comercial.

## Puertos

| Puerto | Función |
|---|---|
| `:9180` | HTTP del chasis — conexiones del Portero Totebox |

## Inicio rápido

```bash
# Sin Yo-Yo configurado (solo registro de flota)
ORCHESTRATION_BIND_ADDR=127.0.0.1:9180 \
    cargo run -p orchestration-slm-server
```

## Conexión Totebox

Cada Portero Totebox apunta su configuración de Yo-Yo al chasis:

```bash
SLM_YOYO_DEFAULT_ENDPOINT=http://<chasis>:9180/v1/yoyo/proxy
SLM_YOYO_TRAINER_ENDPOINT=http://<chasis>:9180/v1/yoyo/trainer
SLM_YOYO_GRAPH_ENDPOINT=http://<chasis>:9180/v1/yoyo/graph
```

## Arquitectura

Consulte `CLAUDE.md` para las notas completas de arquitectura.
