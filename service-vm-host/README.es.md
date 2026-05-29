# service-vm-host

Agente de latido por nodo del grupo de recursos VM de la PPN. Se ejecuta en cada nodo de infraestructura.

Sondea CPU y RAM local cada 10 segundos y envía un `NodeHeartbeat` a
`service-vm-fleet` en `VM_FLEET_ENDPOINT`.

## Variables de entorno

| Variable | Predeterminado | Descripción |
|---|---|---|
| `VM_FLEET_ENDPOINT` | — | URL del controlador de flota, p. ej. `http://10.8.0.9:9203` (obligatorio) |
| `VM_NODE_ID` | — | Identificador único del nodo, p. ej. `gcp-cloud-1` (obligatorio) |
| `VM_WG_IP` | — | IP WireGuard de este nodo, p. ej. `10.8.0.9` (obligatorio) |
| `VM_HEARTBEAT_INTERVAL_S` | `10` | Intervalo de latido en segundos |

## Unidad systemd

`infrastructure/systemd/ppn/local-vm-host.service`
