---
schema: foundry-draft-v1
artifact_type: TOPIC
title: "OS Mediakit"
slug: os-mediakit
language: es
category: systems
status: active
quality: review
target_path: content-wiki-documentation/systems/os-mediakit.es.md
bilingual_pair: topic-os-mediakit.draft.md
cites:
  - infrastructure-os
  - os-network-admin
  - totebox-archive
  - ppn-hypervisor-resource-pool
  - ppn-architecture-overview
  - genesis-protocol
bcsc_reviewed: false
operator_approved: false
notes_for_editor: >
  Los nombres de productos (os-mediakit, vm-mediakit, PointSav Private Network, seL4,
  Microkit, WireGuard, QEMU, Debian, service-fs, system-core, system-ledger,
  virtio-balloon, moonshot-toolkit, CPIO, AArch64, x86_64, QCOW2) no se traducen.
  Postura BCSC: fase Debian 12 en tiempo presente; fase seL4 Microkit en lenguaje
  planificado/previsto.
---

# OS Mediakit

**os-mediakit** es la imagen del sistema operativo invitado para el nivel de VM
`vm-mediakit` en la capa del hipervisor de la Red Privada PointSav. Aísla la superficie
de servicio MediaKit — wikis de conocimiento, sitios de marketing, corrector de pruebas
y orquestación BIM — del almacén de fuentes y los niveles de orquestación.

---

## Posición en la arquitectura

Las cuatro capas del stack Totebox ubican os-mediakit en la **capa del hipervisor**:

```
Operador
  ↓
PPN (malla WireGuard, plano de control os-network-admin)
  ↓
Capa del hipervisor  ←— el SO invitado os-mediakit se ejecuta aquí
  ↓
Orquestación Totebox (app-mediakit-*, service-fs, system-core)
```

os-mediakit es uno de los tres invitados en el esquema de tres VMs:

| VM | SO invitado | Nivel |
|---|---|---|
| vm-workspace | SO anfitrión (Linux) | os-privategit (anfitrión permanente) |
| vm-intelligence | os-intelligence (previsto) | os-totebox + inferencia |
| vm-mediakit | **os-mediakit** | os-mediakit |

---

## Fase 1: Debian 12 provisional (presente)

El primer despliegue de vm-mediakit utiliza una imagen **Debian 12 genericcloud x86_64 QCOW2**
como SO invitado. Esta es la implementación provisional de producción mientras se desarrolla
la imagen seL4 Microkit.

Lo que está en funcionamiento actualmente:
- Debian 12 arrancado mediante `provision-vm-mediakit.sh` bajo QEMU/TCG
- 6 GiB de RAM, disco QCOW2 de 20 GB
- Red NAT de modo usuario: reenvíos de puerto anfitrión `1xxxx → :xxxx` por cada servicio
- Dispositivo `virtio-balloon`: ajuste dinámico de RAM sin reinicio del invitado [infrastructure-os]
- Primer arranque cloud-init: nombre de host `vm-mediakit`, usuario `foundry`, systemd nativo

Servicios en ejecución dentro del invitado Debian 12:

| Servicio | Puerto | Propósito |
|---|---|---|
| service-fs | 9100 | Registro WORM — columna vertebral de ingestión de datos |
| system-core | — | Substrato del Registro de Capacidades (biblioteca) |
| system-ledger | — | Máquina de estado del registro, revocación |
| local-proofreader | 9092 | Servicio de corrección de pruebas |
| local-knowledge-documentation | 9090 | Wiki de documentación |
| local-knowledge-corporate | 9095 | Wiki corporativa |
| local-knowledge-projects | 9093 | Wiki de proyectos |
| local-marketing-pointsav | 9101 | Sitio de marketing PointSav |
| local-marketing | 9102 | Sitio de marketing Woodfine |
| local-bim-orchestration | 9096 | Puerta de enlace BIM |

---

## Fase 3: imagen seL4 Microkit (prevista)

La forma prevista a largo plazo de os-mediakit es una **imagen seL4 Microkit 2.2 AArch64**
ensamblada por `moonshot-toolkit`. Cada servicio se ejecuta como un Dominio de Protección
(PD) seL4 aislado dentro del micronúcleo con verificación formal.

Se trata de un hito planificado. La ruta seL4 requiere un anfitrión AArch64 (Microkit 2.2.0
admite AArch64 y RISC-V 64; no hay objetivo x86_64 para Microkit).

### Distribución planificada de componentes

Cada servicio principal se convierte en un PD seL4 con un conjunto mínimo de capacidades:

| PD | Binario | Capacidad seL4 |
|---|---|---|
| `mediakit-root` | servidor raíz os-mediakit | Arranque, distribución de capacidades |
| `service-fs-pd` | service-fs Envelope B | IPC al ledger-pd; solo punto final de sistema de archivos |
| `system-ledger-pd` | system-ledger (función nativa) | seL4_Call al oráculo de capacidades |
| `proofreader-pd` | service-proofreader | Punto final HTTP; sin capacidad de sistema de archivos |
| `knowledge-pd` | app-mediakit-knowledge | Punto final HTTP; capacidad de SF de solo lectura |
| `marketing-pd` | app-mediakit-marketing | Punto final HTTP; sin capacidad de SF |

El invariante de aislamiento: ningún PD tiene capacidad de lectura sobre la memoria de
otro PD. Impuesto por el modelo de capacidades seL4 — no por permisos a nivel de SO.
[ppn-architecture-overview]

### El shim `system-substrate-sel4`

`system-core` y `system-ledger` están escritos para entornos `std` (forma de demonio Linux).
Ejecutarlos como PDs seL4 requiere `system-substrate-sel4` — una caja shim con indicadores
de características `["native"]` (seL4_Call/seL4_Send vía rust-sel4) y `["compat"]`
(envoltorio std para Linux). El shim es una caja planificada. El documento
ARCHITECTURE.md §Envelope B de service-fs documenta el mismo patrón.

---

## Qué cambia respecto a la Fase 1 y qué permanece igual

| Propiedad | Debian 12 (Fase 1) | seL4 Microkit (Fase 3, previsto) |
|---|---|---|
| SO invitado | Linux 6.x | Micronúcleo seL4 + PDs Microkit |
| Anfitrión | QEMU/TCG (x86_64) | QEMU/KVM o bare metal AArch64 |
| Binarios de servicio | Los mismos (compilación cruzada) | Los mismos (recompilados para AArch64 no_std) |
| Protocolos de comunicación | CBOR sobre HTTP | CBOR sobre QUIC (mismo esquema de datos) |
| Números de puerto | Los mismos (9090, 9092, ...) | Los mismos (superposición WireGuard) |
| virtio-balloon | Presente | Presente (capa del hipervisor sin cambios) |
| Aislamiento formal | Modelo de seguridad del núcleo Linux | Prueba de no-interferencia intransitiva seL4 |
| Custodia de claves | Permisos de archivos del SO | Objeto de capacidad seL4 — sin SO |

---

## Relación con os-infrastructure y el Genesis Protocol

`os-infrastructure` es la capa de arranque del hipervisor — ejecuta el Genesis Protocol
en el anfitrión físico para establecer la identidad WireGuard del nodo PPN. os-mediakit
es un *invitado* que se ejecuta sobre os-infrastructure. Son capas y binarios diferentes.

La secuencia de primer arranque del Genesis Protocol [genesis-protocol] se aplica al
**nodo anfitrión** (os-infrastructure), no al invitado (os-mediakit). Un nuevo invitado
vm-mediakit se une a la malla mediante la ceremonia de emparejamiento MBA después de que
el nodo anfitrión ya es miembro del PPN.

---

## Véase también

- `BRIEF-totebox-transformation.md §2/§6/§9` — esquema de tres VMs, secuenciación Parte C, decisión seL4
- `BRIEF-PPN-DEV-BOOTSTRAP.md §12` — logro Phase 1C.d de moonshot-toolkit y análisis de brechas
- [ppn-hypervisor-resource-pool] — cómo virtio-balloon gestiona la RAM para vm-mediakit
- [totebox-archive] — qué hace el nivel Totebox Archive sobre el SO invitado
- [os-network-admin] — el plano de control PPN; vm-mediakit se une a la malla a través de él
