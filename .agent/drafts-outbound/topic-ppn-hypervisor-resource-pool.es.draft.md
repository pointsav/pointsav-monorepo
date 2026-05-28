---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: ppn-hypervisor-resource-pool.es.md
audience: operadores técnicos e ingenieros que entienden la gestión de recursos PPN
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/infrastructure-os.md
  - systems/os-network-admin.md
  - systems/totebox-archive.md
  - systems/os-orchestration.md
notes_for_editor: >
  Par en español de topic-ppn-hypervisor-resource-pool.draft.md. Traducción directa del inglés.
  Verificar terminología técnica en español contra el glosario de content-wiki-documentation.
  Distinción clave a preservar: los pools de PPN gestionan CPU/RAM por nodo físico (capa
  hipervisor); os-orchestration agrupa el acceso a datos a través de Totebox Archives (capa
  de datos). Estas capas son ortogonales. La implementación del controlador de balloon es un
  hito futuro — usar lenguaje planificado/previsto (postura BCSC).
  Frontmatter del artículo a agregar al confirmar: title "Pool de Recursos del Hipervisor PPN",
  category "architecture", status "active", quality "review",
  cites [infrastructure-os, os-network-admin, totebox-archive, os-orchestration].
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
---

# Pool de Recursos del Hipervisor PPN

La capa de hipervisor de la Red Privada PointSav (PPN) gestiona un pool de CPU y RAM por
nodo, asignando dinámicamente esos recursos a través de las máquinas virtuales que ejecuta.
Este es el mecanismo por el cual la PPN otorga mayor o menor capacidad de cómputo a cada VM
de Totebox Archive en respuesta a la demanda de carga de trabajo.

## Un pool por nodo físico

Cada nodo físico de PPN — una instancia GCP, un servidor en las instalaciones, una máquina
arrendada — controla un pool limitado por su propio hardware. El pool no se comparte entre
nodos. Un nodo con 31 GB de RAM gestiona 31 GB; no toma prestado de un nodo vecino.

La colocación de carga de trabajo entre nodos es una preocupación separada: la Capa de
Orquestación de Totebox (`gateway-orchestration-command-1`) decide en qué nodo físico se
ejecuta una instancia cluster-totebox, basándose en el emparejamiento MBA y las señales de
capacidad disponible. Una vez tomada esa decisión, el hipervisor del nodo receptor gestiona
el pool de recursos local para esa VM. El pool de PPN y el planificador de Totebox son
ortogonales.

## Pool de memoria: virtio_balloon

El mecanismo principal de recuperación de memoria es el dispositivo paravirtual
`virtio_balloon`. Cada VM aprovisionada por `os-infrastructure` se inicia con un
controlador de balloon, que se ejecuta como un módulo estándar del kernel dentro del
sistema operativo invitado.

**Cómo funciona la inflación (recuperar memoria):**

1. El hipervisor (controlador de balloon) señala al controlador de balloon que se inflate
   en N páginas.
2. El controlador asigna esas páginas dentro del invitado, eliminándolas del espacio de
   direcciones utilizables del invitado.
3. El hipervisor recupera esas páginas físicas para el pool a nivel de nodo.
4. El pool crece en N páginas; la RAM disponible del invitado disminuye en N páginas.

**Cómo funciona la deflación (devolver memoria):**

1. El hipervisor señala al controlador de balloon que se deflate.
2. El controlador libera las páginas del balloon de vuelta a la lista libre del invitado.
3. La RAM disponible del invitado crece; el pool disminuye.

**El pool en cualquier instante:**

```
pool_disponible = ram_física − Σ(balloon_mínimo en todas las VMs)
```

Cada VM tiene una reserva mínima de balloon por debajo de la cual el controlador no
inflará. Esto evita que una VM sea privada de memoria cuando el nodo está bajo presión.

## Pool de CPU: pesos de planificación de vCPU

La gestión del pool de CPU utiliza la interfaz `cpu.weight` de cgroups v2 de Linux. Cada
proceso QEMU (uno por VM) se coloca en un cgroup con un peso tomado del ledger de
capacidades. Bajo contención de CPU, el planificador distribuye el tiempo de vCPU
proporcionalmente a esos pesos. Cuando el nodo no está bajo contención, todas las VMs
se ejecutan a máxima velocidad independientemente del peso.

Una VM cluster-totebox que ejecuta una carga de trabajo de inferencia activa (a través de
`service-slm`) puede recibir un peso más alto que una VM de archivo inactiva. La entrada
del ledger es el peso autoritativo; `os-infrastructure` lo aplica en el lanzamiento de la
VM y puede ajustarlo en vivo.

## Relación con os-orchestration

`os-orchestration` es un agregador de capa de datos. Agrega **acceso a datos** a través
de Totebox Archives utilizando el Protocolo PointSav (PSP) — consultas basadas en
capacidades que devuelven solo filas de resultados, nunca registros sin procesar. Es
sin estado y no posee claves de archivos.

`os-orchestration` no asigna CPU. No ajusta la memoria. No se comunica con el controlador
de balloon del hipervisor. Las dos capas están diseñadas para ser ciegas entre sí:

- El hipervisor sabe que una VM está consumiendo N páginas y Y por ciento de vCPU. No sabe
  si la VM está ejecutando `os-totebox`, `os-orchestration` o cualquier otra cosa.
- El Totebox Archive dentro de la VM no sabe nada sobre la inflación del balloon, los pesos
  de cgroup, ni en qué nodo físico se encuentra.

Este es el **invariante de aislamiento** de BRIEF-PPN-ARCHITECTURE.md §1.1 Contribución #2:
el hipervisor no tiene capacidad de lectura sobre el estado interno de la VM.

## Archivos libremente transferibles

Debido a que el hipervisor gestiona solo el ciclo de vida de la VM y la asignación de
recursos — no los datos dentro de las VMs — un Totebox Archive puede detenerse, copiar
la imagen de disco a otro nodo y reiniciarse allí sin ningún cambio en sus datos ni en
su identidad. El hipervisor del nodo de destino asignará recursos de su propio pool para
la VM reubicada.

Esta es la propiedad de **transferencia libre** de los Totebox Archives: la imagen de disco
de arranque es el archivo; el pool de recursos es la infraestructura del nodo. Mover la
imagen mueve el archivo. El pool del nuevo nodo absorbe la carga de trabajo.

## Estado de implementación

El indicador de dispositivo `virtio_balloon` está disponible en QEMU 7.x y en el módulo
`virtio_balloon` de NetBSD/bhyve. Agregar `-device virtio-balloon` al comando de lanzamiento
de la VM instala el controlador de balloon en el invitado.

El **controlador** de balloon — el componente dentro de `os-infrastructure` que decide
cuándo inflar o deflatar el balloon de cada VM en respuesta a las señales de demanda —
es un hito planificado. Hasta que el controlador esté implementado, los operadores pueden
ejercer el mecanismo manualmente a través del monitor QEMU:

```
(qemu) info balloon      # mostrar la RAM visible por el invitado actualmente
(qemu) balloon 128       # solicitar al invitado que devuelva memoria hasta 128 MB
(qemu) info balloon      # confirmar la recuperación
```

El script `infrastructure/virt/vm-prove.sh` incluye `-device virtio-balloon` para que el
controlador de balloon esté presente en la VM de prueba desde el primer arranque.

## Temas relacionados

- **Infrastructure OS** — el hipervisor Tipo I que implementa el controlador de balloon
- **Totebox Archive** — la bóveda soberana de datos que se ejecuta dentro de cada VM
- **OS Orchestration** — el agregador de datos sin estado (separado del pool de recursos)
- **Malla Soberana** — la capa de transporte WireGuard que conecta los nodos PPN
