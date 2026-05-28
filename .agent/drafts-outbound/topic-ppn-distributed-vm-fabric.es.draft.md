---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: ppn-distributed-vm-fabric.es.md
audience: arquitectos e ingenieros que evalúan infraestructura de cómputo distribuido soberana
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - architecture/ppn-architecture-overview.md
  - architecture/ppn-hypervisor-resource-pool.md
  - architecture/sovereign-mesh.md
  - architecture/genesis-protocol.md
  - systems/infrastructure-os.md
  - systems/os-orchestration.md
notes_for_editor: >
  Par en español de topic-ppn-distributed-vm-fabric.draft.md. Traducción directa del inglés.
  Verificar terminología técnica en español contra el glosario de content-wiki-documentation.
  Todos los componentes distribuidos son PLANIFICADOS/PREVISTOS — aún no construidos.
  Postura BCSC estricta: solo la capa por nodo usa tiempo presente.
  Frontmatter del artículo a agregar al confirmar: title "Tejido VM Distribuido PPN",
  category "architecture", status "active", quality "review",
  cites [ppn-architecture-overview, ppn-hypervisor-resource-pool, sovereign-mesh,
         genesis-protocol, infrastructure-os, os-orchestration].
research_done_count: 7
research_suggested_count: 0
open_questions_count: 0
---

# Tejido VM Distribuido PPN

El **Tejido VM Distribuido PPN** es la extensión planificada de la capa de hipervisor PPN
por nodo hacia un pool de recursos multinodo. Donde la capa de hipervisor actual gestiona
CPU y RAM dentro de un único nodo físico, el tejido distribuido tiene como objetivo
permitir que las VMs tomen prestado cómputo de otros nodos de la malla y colocar y migrar
VMs a través de la flota sin intervención del operador en cada movimiento.

Este tema describe la arquitectura planificada. La capa por nodo — virtio_balloon, pesos
de cgroups v2 y la prueba vm-prove.sh — está implementada y verificada a partir del
2026-05-28. Los cuatro componentes distribuidos que se describen a continuación son hitos
planificados; ninguno está construido todavía.

## Estado actual: solo por nodo

La capa de hipervisor implementada asigna CPU y RAM dentro de un único nodo físico. El
pool está acotado por el hardware de ese nodo. Expandir la memoria de una VM significa
tomar del pool libre del mismo nodo. Colocar una VM en un nodo diferente requiere
detenerla, transferir la imagen de disco y reiniciarla — una operación manual que la
capa de Orquestación de Totebox tiene como objetivo automatizar.

El mecanismo `virtio_balloon` probado en `infrastructure/virt/vm-prove.sh` opera
completamente dentro de un único nodo. No se requiere comunicación de red. No se
necesita reiniciar el anfitrión ni el invitado para las operaciones de balloon —
la inflación, deflación y los cambios de peso de cgroups v2 son todos ajustes
dinámicos a un sistema en ejecución.

## Componente 1 — Préstamo virtio-mem sobre WireGuard (planificado)

`virtio-mem` (kernel Linux desde la versión 5.8; QEMU desde 5.1) admite la conexión y
desconexión en caliente de bloques de memoria individuales en un invitado en ejecución.
A diferencia de `virtio_balloon` — que infla un único dispositivo para recuperar páginas
— `virtio-mem` expone un conjunto de bloques granulares que el kernel del invitado puede
aceptar o devolver de uno en uno. Una VM puede crecer más allá de su asignación inicial
sin reiniciarse.

La extensión entre nodos prevista:

1. Un nodo con RAM sobrante anuncia los bloques `virtio-mem` disponibles a través de la
   malla WireGuard
2. Una VM en un nodo diferente que necesita más RAM recibe esos bloques como un
   dispositivo conectado en caliente
3. Se prevé que el modelo de capacidades de seL4 en el nodo prestamista garantice que
   no se retenga capacidad de lectura sobre los bloques prestados — las páginas físicas
   se mapean exclusivamente en el espacio de direcciones de la VM prestataria
4. Cuando finaliza el período de préstamo, los bloques se desconectan en caliente y
   se devuelven al pool del nodo prestamista

Este enfoque difiere de CXL 3.0 (Compute Express Link), que proporciona intercambio de
memoria coherente por hardware a través de la tela PCIe. CXL requiere proximidad física —
no funciona sobre una WAN o enlace de internet. El enfoque de préstamo basado en
WireGuard tiene como objetivo funcionar sobre cualquier red, incluida la malla cifrada
existente, a costa de una latencia mayor que la de PCIe.

## Componente 2 — Ledger de capacidades distribuido (planificado; moonshot-protocol, moonshot-database)

El modelo de capacidades de seL4 opera dentro de un único espacio de direcciones: las
capacidades son identificadores no falsificables que median todo el acceso a objetos,
pero no atraviesan los límites de las máquinas de forma nativa. La extensión distribuida
requiere un protocolo de capacidades que funcione entre nodos.

El diseño previsto:

- Los **tokens de capacidad** son concesiones firmadas con HMAC emitidas por un nodo
  prestamista, vinculadas a la identidad de la ceremonia de emparejamiento del nodo
  (establecida mediante CPace PAKE en el momento de la incorporación). Se prevé que cada
  token codifique: `{nodo_concesionario, tipo_recurso, id_recurso, expira,
  número_de_secuencia}`.
- Se prevé que la **revocación** sea un registro de revocación firmado añadido a un
  Merkle DAG. Cada nodo mantendría una copia local del DAG. Los nodos distribuirían
  registros delta a través de la malla WireGuard; se prevé que una revocación llegue a
  todos los pares en tiempo subsegundar en una malla LAN.
- No se requiere autoridad de revocación central. La ceremonia de emparejamiento
  establece las raíces de confianza; se prevé que las concesiones y revocaciones
  posteriores fluyan de par a par.

Los directorios de proyecto `moonshot-protocol` y `moonshot-database` están reservados
para este trabajo. El formato de cable binario de 16 bytes existente (para los comandos
de malla de `app-network-admin`) es el prototipo para la codificación compacta de tokens.

## Componente 3 — Planificador de VMs entre nodos (planificado; os-orchestration)

`gateway-orchestration-command-1` es el hogar previsto para la lógica de colocación
entre nodos. Hoy es sin estado — agrega consultas de datos PSP a través de Totebox
Archives y devuelve filas de resultados. La extensión planificada tiene como objetivo
agregar una capa de planificación de recursos:

- **Colocación**: cuando es necesario lanzar una nueva VM, el planificador leería los
  anuncios de disponibilidad de recursos de `os-network-admin` de cada nodo, verificaría
  el estado de confianza del nodo en el ledger de capacidades y colocaría la VM en el
  nodo de mejor ajuste.
- **Migración**: la migración en vivo de QEMU transfiere el estado de una VM en ejecución
  (memoria, registros de CPU, estado de dispositivos) a través de una conexión TCP
  tunelizada mediante WireGuard. La VM permanece disponible durante la transferencia;
  la pausa de transición es típicamente de menos de un segundo en una LAN.
- **Restricción de soberanía**: se prevé que el operador pueda anclar cualquier VM a un
  nodo de confianza específico. Una VM anclada a Laptop A no sería migrada a Laptop B ni
  al nodo GCP independientemente del desequilibrio de carga. La soberanía del operador
  sobre la colocación no sería anulada por la optimización automatizada.

No se necesita ningún transporte nuevo. La migración en vivo de QEMU sobre WireGuard usa
la malla cifrada existente. Se prevé que el planificador permanezca sin estado — las
decisiones de colocación se derivarían del ledger de capacidades y los anuncios de nodos,
sin estado de planificador persistente.

## Componente 4 — Cadena de atestación soberana (planificado)

Intel TDX (Xeon de 5ª generación, Azure GA nov 2025) y AMD SEV-SNP (EPYC 9000 Turin)
proporcionan aislamiento de VM aplicado por hardware donde el hipervisor no puede leer
la memoria del invitado. Ambos requieren la infraestructura de atestación del fabricante
de CPU para verificar las reclamaciones de aislamiento — una cadena de certificados que
pasa por Intel Trust Authority o el servicio de gestión de claves de AMD. El operador
confía en el proveedor de silicio, no solo en su propio hardware.

El diseño de atestación previsto de PPN es diferente:

- **Raíz de atestación**: la propia ceremonia de emparejamiento. Cuando un nodo se une
  a la malla mediante CPace PAKE y la comparación del código corto SAS, el operador
  presencia físicamente el intercambio. Se prevé que la clave de identidad del nodo sirva
  como ancla de atestación — sin proveedor de TPM, sin proveedor de silicio, sin
  proveedor de nube en la cadena de confianza.
- **Atestación de imagen del invitado**: se prevé que `dm-verity` (device mapper de
  Linux, estándar desde el kernel 3.4) ancle el sistema de archivos raíz del SO invitado
  a un hash confirmado en el momento del aprovisionamiento de la VM. El hash sería
  firmado por la clave de la ceremonia de emparejamiento del nodo de aprovisionamiento.
  Un invitado modificado desde el aprovisionamiento fallaría la verificación `dm-verity`
  y no arrancaría.
- **Informe de atestación**: se prevé que un invitado pueda generar una declaración
  firmada — firmada con la clave de identidad del nodo de aprovisionamiento — que afirme
  que está ejecutando una imagen sin modificar. Los auditores externos podrían verificar
  esta declaración sin contactar a Intel, AMD ni a ningún proveedor de nube.

Esta cadena es más corta y está más controlada por el operador que TDX/SEV-SNP. El
compromiso es intencional: el modelo de soberanía sitúa al operador, no al proveedor de
silicio, en la raíz de confianza.

## Comparación con las capacidades de los principales proveedores de nube

La tabla a continuación describe lo que los principales proveedores de nube tienen
desplegado hoy y lo que el tejido VM distribuido de PPN tiene como objetivo proporcionar.
Las capacidades de los competidores en la primera columna son afirmaciones factuales en
tiempo presente. Los elementos de PPN usan lenguaje planificado/previsto.

| Capacidad | AWS / Azure / GCP (hoy) | Tejido VM distribuido PPN (planificado) |
|---|---|---|
| Aislamiento de memoria por VM | TDX (Azure GA nov 2025), SEV-SNP (AMD EPYC 9005); el hipervisor no puede leer la RAM del invitado por hardware | Prueba formal seL4: invariante verificado mecánicamente en Isabelle/HOL de que el hipervisor no tiene capacidad de lectura sobre el estado de la VM |
| Intercambio de memoria entre nodos | CXL 3.0 (tela PCIe, interno al centro de datos; no expuesto a los inquilinos como API programable) | Préstamo virtio-mem sobre WireGuard (previsto para funcionar sobre cualquier red, incluido internet) |
| Revocación de capacidades | Políticas IAM; propagación centralizada; típicamente 10–60 segundos | Chisme Merkle DAG de par a par; revocación subsegundar prevista; sin autoridad central |
| Raíz de atestación | CA del proveedor de silicio (Intel Trust Authority / AMD Key Management Service) | Ceremonia de emparejamiento presenciada por el operador; operador previsto como raíz de confianza |
| Prueba formal de aislamiento | Ninguna en hipervisores en producción en ningún proveedor de nube importante | Prueba de corrección funcional y seguridad de flujo de información seL4 en Isabelle/HOL (en vendor-sel4-kernel hoy) |
| Tiempo de despliegue para PYMEs | Horas: consola en la nube, IAM, configuración de red VPC | Previsto: menos de cinco minutos, dos preguntas, ceremonia de emparejamiento con código corto |
| Soberanía del operador | Ninguna: el proveedor de nube controla el hardware físico y el hipervisor | Completa: el operador es dueño del sustrato; el hipervisor no puede leer las VMs; el proveedor de nube no está en la cadena de confianza |

## Secuencia de construcción y directorios reservados

La capa por nodo es la base. Se prevé que el tejido distribuido se construya sobre ella
en orden:

| Paso | Estado | Directorio reservado |
|---|---|---|
| virtio_balloon por nodo + cgroups v2 | **Completo** — probado 2026-05-28 | `os-infrastructure/` |
| Despliegue de ceremonia (`service-ppn-pairing` en GCP VM) | Pendiente — NEXT.md §7 Paso 1 | `service-ppn-pairing/` |
| Primera incorporación real de nodo (`os-network-admin` en Laptop A) | Pendiente — NEXT.md §7 Paso 3 | `os-network-admin/` |
| Reescritura de `os-infrastructure` según el Protocolo Génesis | Planificado | `os-infrastructure/`, `moonshot-hypervisor/` |
| Controlador automático de balloon | Planificado | `os-infrastructure/` |
| Demonio de préstamo virtio-mem | Planificado | `moonshot-network/` |
| Ledger de capacidades distribuido | Planificado | `moonshot-protocol/`, `moonshot-database/` |
| Planificador de VMs entre nodos | Planificado | `os-orchestration/` |
| Cadena de atestación soberana (dm-verity + clave de ceremonia) | Planificado | `os-infrastructure/`, `moonshot-kernel/` |

## Temas relacionados

- [[ppn-architecture-overview]] — descripción general de PPN en cuatro capas; el tejido distribuido es la extensión planificada de la capa de hipervisor
- [[ppn-hypervisor-resource-pool]] — el pool por nodo implementado: virtio_balloon, cgroups v2, hito del controlador de balloon
- [[sovereign-mesh]] — la capa de transporte WireGuard sobre la que se ejecuta el tejido distribuido
- [[genesis-protocol]] — la ceremonia de primer arranque; la raíz de atestación prevista para el tejido distribuido
- [[infrastructure-os]] — el hipervisor Tipo I; hogar del controlador de balloon y el demonio de préstamo virtio-mem previsto
- [[os-orchestration]] — el hogar previsto para el planificador de VMs entre nodos
