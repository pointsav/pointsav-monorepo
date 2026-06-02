---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: systems/
target_filename: os-infrastructure-ppn-node.es.md
audience: operadores e integradores de sistemas que implementan nodos de la Red Privada PointSav
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-29
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/vm-architecture.es.md
  - architecture/ppn-architecture-overview.es.md
  - architecture/genesis-protocol.es.md
notes_for_editor: >
  Par bilingüe de topic-os-infrastructure-ppn-node.draft.md.
  La Fase 1 (Ubuntu 24.04) usa tiempo presente — está operativa.
  La Fase 2 (NetBSD/NVMM) y la Fase 3 (seL4) usan lenguaje condicional (planificado/previsto).
  La arquitectura de 7 PDs de seL4 es planificada/prevista — no presentar como enviada.
  Los objetivos de recursos (8 MB disco / 12 MB RAM inactiva) son objetivos de la Fase 3,
  no valores actuales.
research_done_count: 6
research_suggested_count: 0
open_questions_count: 1
---

# os-infrastructure — Sistema Operativo de Nodo PPN

`os-infrastructure` es la capa del sistema operativo para los nodos de la Red Privada
PointSav. No es un sistema operativo de propósito general. Su único propósito es
configurar, operar y mantener un nodo en una Red Privada PointSav: gestionar túneles
WireGuard, alojar máquinas virtuales para otros servicios de la plataforma y exponer
el plano de control del operador.

---

## Qué es os-infrastructure

Un nodo `os-infrastructure` es el host físico o virtual que ancla una Red Privada
PointSav. Cada nodo de la malla ejecuta `os-infrastructure` como su sistema operativo
host. Los tres nodos de un despliegue típico — una instancia en la nube y dos máquinas
en las instalaciones — ejecutan cada uno una instancia independiente de
`os-infrastructure`. Se comunican exclusivamente a través de túneles WireGuard; no
existe ningún tejido de red compartido, ningún plano de control de clúster ni
dependencia de VLAN.

`os-infrastructure` gestiona:

- **Túneles WireGuard** hacia todos los pares de la malla
- **Máquinas virtuales para los servicios de la plataforma** que procesan datos
  (VM-MediaKit, VM-Totebox, VM-Orchestration, VM-PrivateGit)
- **La ceremonia del Protocolo Génesis** para añadir nuevos nodos a la malla
- **El grupo de recursos de VM** — seguimiento de CPU y RAM disponibles en todos los
  nodos y despacho de solicitudes de creación de VM al nodo con capacidad suficiente

`os-infrastructure` no almacena datos de usuario. Los archivos, archivos multimedia y
registros de bases de datos residen dentro de las máquinas virtuales gestionadas por
os-totebox, os-mediakit u os-privategit.

---

## Fase 1 — Ubuntu 24.04

El despliegue operativo actual utiliza Ubuntu 24.04 como sistema operativo host. QEMU
proporciona el hipervisor; en hardware con extensiones Intel VT-x o AMD-V, QEMU ejecuta
invitados con aceleración KVM. En instancias virtuales de GCP donde no está habilitada
la virtualización anidada, QEMU utiliza la emulación por software TCG como alternativa.

WireGuard está integrado en el núcleo de Ubuntu 24.04 (kernel 5.6+). La malla de tres
nodos utiliza el rango de direcciones 10.8.0.0/24 con el Laptop B como concentrador de
enrutamiento.

Systemd gestiona todos los servicios de la plataforma. Cada VM invitada se ejecuta como
una unidad systemd que envuelve un proceso QEMU con un socket de monitor UNIX para las
operaciones de control.

El grupo de recursos de VM en la Fase 1 consta de dos servicios. `service-vm-host` se
ejecuta en cada nodo y envía un latido al controlador de la flota cada diez segundos,
informando sobre la RAM disponible y la carga de CPU. `service-vm-fleet` se ejecuta en
el nodo GCP y recibe esos latidos; cuando un operador solicita una nueva VM a través de
la interfaz `app-network-admin`, el controlador de la flota selecciona el nodo con más
RAM disponible por encima de un margen de seguridad y envía la solicitud de creación.

---

## Fase 2 — NetBSD 11.0 + NVMM (planificado)

Se prevé que la Fase 2 reemplace el host Ubuntu 24.04 con NetBSD 11.0, un sistema
operativo con licencia BSD con una postura de seguridad más sólida para la operación
de nodos PPN en producción.

NetBSD 11.0 incluye NVMM (NetBSD Virtual Machine Monitor), un hipervisor bare-metal
disponible en la distribución principal desde NetBSD 9.0 que utiliza Intel VT-x EPT
para el aislamiento por hardware. QEMU ejecuta invitados con el indicador `-accel nvmm`.
Se prevé que un único nodo de Fase 2 aloje 128 máquinas virtuales con una capacidad de
256 vCPU.

NetBSD 11.0 también incluye WireGuard `wg(4)` integrado en el núcleo, eliminando la
dependencia de WireGuard en espacio de usuario de la Fase 1. `securelevel=2` bloquea
el núcleo en ejecución frente a modificaciones. VeriExec valida la integridad de los
binarios en tiempo de carga mediante un manifiesto firmado.

Objetivo de recursos planificado: 120 MB de disco, 48 MB de RAM inactiva.

---

## Fase 3 — seL4 + Microkit 2.x (previsto)

Se prevé que la Fase 3 reemplace el host NetBSD con un micronúcleo formalmente
verificado basado en seL4 v15 y Microkit 2.x en hardware AArch64.

El núcleo seL4 en sí tiene 162 KiB de binario verificado mecánicamente. Su prueba
formal establece la no interferencia intransitiva: un invitado comprometido no puede
leer ni escribir el estado de ningún otro dominio de protección sin una concesión de
capacidad explícita. Esta es una afirmación de aislamiento más sólida que la de
cualquier hipervisor con una base de código de confianza no verificada.

Se prevé que el `os-infrastructure` de la Fase 3 se ejecute como siete dominios de
protección seL4:

| Dominio de protección | Función |
|---|---|
| `pd-genesis` | Protocolo CPace PAKE; genera código corto en base32 Crockford para verificación del operador; eliminado tras completar la ceremonia de emparejamiento (revocación de capacidad) |
| `pd-ledger` | Libro mayor WORM de capacidades Ed25519; solo de adición; firma todas las concesiones de capacidad |
| `pd-wireguard` | Implementación WireGuard BoringTun `no_std`; se ejecuta dentro de seL4 sin dependencia de libc |
| `pd-net-driver` | Gestión de MMIO e IRQ de NIC; proporciona la capacidad de red a `pd-wireguard` |
| `pd-vmm` | Monitor de VM invitada mediante `libsel4vm`; gestiona VMs para otros tipos os-* |
| `pd-fleet` | Cliente de latido al controlador de la flota del grupo de recursos |
| `pd-network-admin` | Superficie TUI F8; recibe transmisiones UDP firmadas; confirmaciones de configuración con F12 |

`pd-genesis` es eliminado una vez que la ceremonia de unión del nodo está completa. La
capacidad que poseía durante la ceremonia es revocada y no puede reconstruirse — no
existe puerta trasera al flujo de emparejamiento una vez que se cierra.

La Fase 3 requiere hardware AArch64. Microkit 2.x incluye un objetivo `x86_64_generic_vtx`,
pero Microkit x86-64 restringe cada invitado a un vCPU y requiere Intel VT-x. Para
despliegues en producción con invitados de múltiples vCPU, AArch64 es la plataforma prevista.

Objetivo de recursos previsto: 8 MB de disco, 12 MB de RAM inactiva.

---

## Protocolo Génesis

El Protocolo Génesis es la ceremonia de unión de nodos que añade un nuevo nodo a la malla.

Un operador que inicia un nuevo nodo arranca `service-ppn-pairing` en el nodo que se va
a añadir. El servicio realiza un protocolo CPace PAKE y presenta un código corto en
base32 Crockford en la consola — normalmente de seis a diez caracteres. El operador lee
este código y lo introduce en el panel de aprobación F11 de `app-network-admin` en la
máquina administradora.

Una vez que los códigos coinciden, el emparejamiento establece registros de pares
WireGuard mutuos en ambos nodos, añade una entrada a `nodes.jsonl` en el libro mayor de
capacidades y termina `service-ppn-pairing`. La ventana de la ceremonia es de 600
segundos; si el operador no completa la aprobación dentro de ese período, el código
caduca y la ceremonia debe reiniciarse desde el principio.

No se transmiten claves a través de la red. La comparación del código corto es el único
mecanismo de autenticación — el operador es la raíz de confianza, no una autoridad
de certificación.

---

## Objetivos de Recursos

| Fase | Disco | RAM inactiva | RAM en carga |
|---|---|---|---|
| Fase 1 (Ubuntu 24.04, actual) | ~1,5 GB (SO + servicios) | ~400 MB | ~800 MB |
| Fase 2 (NetBSD/NVMM, planificado) | 120 MB | 48 MB | 200 MB |
| Fase 3 (seL4+Microkit, previsto) | **8 MB** | **12 MB** | 48 MB |

Se prevé que los objetivos de la Fase 3 hagan que el hardware de reserva de cualquier
operador supere el nivel mínimo de VM de los proveedores de nube: el mínimo de AWS
Lambda es de 128 MB de RAM; el objetivo de la Fase 3 es de 12 MB en reposo, más de
diez veces más ligero.
