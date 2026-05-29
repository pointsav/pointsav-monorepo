# service-vm-fleet

Controlador de flota del grupo de recursos VM de la PPN. Se ejecuta en el nodo GCP, escucha en `:9203`.

Recibe latidos de `service-vm-host` en cada nodo de infraestructura, rastrea los
recursos disponibles y proporciona ubicación orientativa de VM.

## Puntos de acceso

| Método | Ruta | Descripción |
|---|---|---|
| `POST` | `/v1/nodes/heartbeat` | Actualizar registro de nodo desde latido |
| `GET` | `/v1/fleet` | FleetStatus — todos los nodos registrados |
| `GET` | `/v1/nodes/:node_id` | Registro de nodo individual |
| `POST` | `/v1/vms` | Crear VM — ubicación orientativa y despacho |
| `DELETE` | `/v1/vms/:vm_id` | Destruir VM |

## Reglas de ubicación

1. Filtrar: `ram_available_mb >= request.ram_mb + 512` (512 MB de margen de seguridad)
2. Ordenar: `ram_available_mb DESC`
3. El primer candidato gana

`auto_rebalance` es permanentemente `false`. La migración en vivo de VM está excluida.
Las instancias VM-Totebox deben especificar `preferred_node` — los datos WORM no pueden migrarse.

## Unidad systemd

`infrastructure/systemd/orchestration/local-vm-fleet.service`
