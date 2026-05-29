# system-vm-fleet-types

Tipos de cable compartidos para la capa de agrupación de recursos VM de la PPN.

Utilizado por `service-vm-fleet` (controlador de flota, residente en GCP) y
`service-vm-host` (agente de latido por nodo). Diseño compatible con `no_std`;
sin dependencia de tokio.

## Tipos

| Tipo | Propósito |
|---|---|
| `NodeHeartbeat` | Enviado por service-vm-host a service-vm-fleet cada 10 segundos |
| `VmRecord` | Máquina virtual individual según informa el agente host |
| `VmState` | Running / Stopped / Provisioning / Error |
| `PlacementAdvice` | Resultado de ubicación orientativa de service-vm-fleet |
| `FleetStatus` | Instantánea de toda la flota |
| `NodeRecord` | Resumen de un nodo individual en la flota |
| `CreateVmRequest` | Solicitud de creación de VM; preferred_node obligatorio para VmTotebox |

## Invariante arquitectónico

`auto_rebalance` es permanentemente `false`. La migración en vivo de VM a través de
WireGuard está excluida — las restricciones de ancho de banda (~20 Mbps × 6 GB VM ≈
40 min) la hacen impráctica. Las VM se colocan una vez y permanecen en el nodo asignado.
