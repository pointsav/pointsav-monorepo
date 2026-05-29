---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: systems/
target_filename: vm-architecture.es.md
audience: operadores, integradores de sistemas y miembros de la comunidad que implementan componentes de la plataforma PointSav
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-29
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/infrastructure-os.md
  - systems/os-network-admin.md
  - systems/totebox-archive.md
  - architecture/ppn-architecture-overview.md
  - architecture/ppn-distributed-vm-fabric.md
  - architecture/genesis-protocol.md
notes_for_editor: >
  Par bilingüe de topic-vm-architecture.draft.md.
  La Fase 1 (Ubuntu 24.04 QEMU) usa tiempo presente — está operativa.
  Las fases 2 y 3 (unikernel/BSD) usan lenguaje condicional (planificado/previsto).
  VM-Infrastructure: usar "flota de hosts con confianza en malla" y no "clúster".
  Frontmatter del artículo al confirmar: title "Arquitectura VM-* y Familia de SO",
  category "systems", status "active", quality "review".
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
---

# Arquitectura VM-* y Familia de Sistemas Operativos

La plataforma PointSav organiza sus implementaciones en tiempo de ejecución en cinco tipos de máquinas virtuales (VM): VM-Totebox, VM-MediaKit, VM-Orchestration, VM-PrivateGit e VM-Infrastructure. Cada tipo de VM corresponde exactamente a un binario fuente `os-*`. El nombre en tiempo de ejecución y el nombre fuente son dos identidades para lo mismo: lo que se compila como `os-totebox` se ejecuta como VM-Totebox.

Esta correspondencia no es casual. La plataforma está diseñada para que un entorno de desarrollo y un sistema de producción de un cliente se implementen de la misma manera. Los cinco tipos de VM reflejan directamente las cinco formas en que clientes y miembros de la comunidad interactúan con la plataforma.

## Tipos de VM y Sus Propósitos

### VM-Totebox

**Binario fuente:** `os-totebox`  
**Propósito:** Bóveda de datos soberana por entidad. La unidad de cómputo principal de la plataforma.

Una instancia de VM-Totebox contiene todos los datos estructurados que pertenecen a una entidad: registros corporativos, personal, bienes inmuebles, correo electrónico, documentos y los libros de contabilidad derivados de ellos. Los datos ingresan a través de los servicios de ingestión del Anillo 1 y son procesados por los servicios de conocimiento del Anillo 2. La bóveda tiene disciplina WORM: los datos pueden adjuntarse y reemplazarse, pero no eliminarse de manera silenciosa.

Cada VM-Totebox es implementable de forma independiente: en un servidor físico, un servidor en alquiler o una máquina virtual en la nube. La imagen de disco constituye el archivo. Migrar un Totebox equivale a mover esa imagen.

Servicios: `service-fs` (almacenamiento de bloques WORM), `service-people`, `service-email`, `service-extraction`, `service-content` e inteligencia opcional del Anillo 3 mediante `service-slm`.

### VM-MediaKit

**Binario fuente:** `os-mediakit`  
**Propósito:** Dispositivo web de cara al público. Funciona de forma independiente de un Totebox.

VM-MediaKit aloja los sitios web y portales de conocimiento que un emisor de información regulada o una pequeña empresa presenta al público. Ejecuta wikis de conocimiento basadas en MediaWiki, sitios de marketing estáticos y una sala de noticias derivada de FreshRSS. Cada aplicación alojada es un servicio sin estado: lee de un directorio de contenido pero no mantiene estado de libro mayor por entidad.

El servicio de corrección ortográfica (proofreader) se ubica junto con VM-MediaKit porque sus clientes residen en MediaKit. Moverlo a VM-Totebox enrutaría cada solicitud editorial a través de la frontera PPN.

Servicios: `app-mediakit-knowledge` (wikis de documentación, corporativa y de proyectos), `app-mediakit-marketing`, `service-proofreader`.

### VM-Orchestration

**Binario fuente:** `os-orchestration`  
**Propósito:** Agregador multi-archivo sin estado. Nivel comercial de pago.

VM-Orchestration consulta múltiples instancias de VM-Totebox y presenta vistas a nivel de flota o portafolio. No retiene datos propios: agrega mediante el Protocolo PointSav (PSP), un protocolo de consulta basado en capacidades. Una instancia de VM-Orchestration sirve el terminal de coordinación BIM, el mapa de flota GIS y el corredor de SLM.

Servicios: `app-orchestration-bim`, `app-orchestration-gis`, `app-orchestration-slm` (:9180).

### VM-PrivateGit

**Binario fuente:** `os-privategit`  
**Propósito:** Control de fuentes soberano y alojamiento del sistema de diseño.

VM-PrivateGit ejecuta Gitea como espejo bidireccional de los repositorios canónicos en GitHub y, opcionalmente, un servidor de previsualización del sistema de diseño. Proporciona independencia de control de fuentes respecto a proveedores externos para la propiedad intelectual y los activos de marca. El espacio de trabajo Foundry (`vault-privategit-source-1`) es la primera implementación de este tipo.

Servicios: `app-privategit-source-control`, `app-privategit-design-system`.

### VM-Infrastructure

**Binario fuente:** `os-infrastructure`  
**Propósito:** La flota de hosts en sí: tejido WireGuard PPN, hipervisor y ubicación de VM.

VM-Infrastructure no es una VM en el sentido convencional. Es el binario `os-infrastructure` ejecutándose en hardware físico, proporcionando la capa de hipervisor que aloja todos los demás tipos de VM. Tres nodos forman la flota de producción mínima:

- **Laptop A (nodo génesis):** Primer nodo. Ejecuta `provision-vm-infrastructure-onprem.sh --genesis`, que configura WireGuard de forma autónoma y abre el servidor de ceremonia de emparejamiento. Aloja VM-Totebox-1.
- **Laptop B (retransmisor):** Segundo nodo. Se une a la malla mediante `--join <código-corto>` (PAKE CPace + confirmación SAS). Aloja el concentrador WireGuard.
- **Nodo GCP (nube):** Tercer nodo. Se une mediante `provision-vm-infrastructure-cloud.sh --join <código-corto>`. Aloja VM-MediaKit, VM-Orchestration y VM-PrivateGit.

VM-Infrastructure es una flota de hosts con confianza en malla, no un planificador de recursos en clúster. Cada nodo se aprovisiona de forma independiente. Las decisiones de ubicación — qué VM se ejecuta en qué nodo — son política del operador. La malla WireGuard proporciona la vinculación nombre-a-endpoint.

## Principio de Ubicación

Un servicio pertenece a la VM cuyo espacio de nombres `os-*` posee el ciclo de vida de sus datos y la frontera de confianza, no a la VM donde se ejecutó su binario por primera vez.

Derivaciones de esta regla:
- `service-fs` (libro mayor WORM) pertenece a VM-Totebox. Es el sustrato de almacenamiento del Totebox.
- `app-orchestration-bim` pertenece a VM-Orchestration. Su nombre declara su clase.
- `service-proofreader` pertenece a VM-MediaKit. Sus clientes residen en MediaKit.
- WireGuard y la ceremonia de emparejamiento pertenecen a VM-Infrastructure. Son preocupaciones del tejido.

## Rutas de Implementación para Clientes

Los tipos de VM se corresponden directamente con las formas en que clientes y miembros de la comunidad se relacionan con la plataforma.

**Usuarios de la Red Privada PointSav** implementan VM-Infrastructure en su propio hardware: como mínimo un nodo físico (Laptop A) y el retransmisor en la nube GCP. El Protocolo Génesis inicia la malla desde un único nodo. No se requiere ninguna autoridad de certificación externa.

**Usuarios de Totebox Orchestration** implementan VM-Totebox (bóveda de datos) y VM-Orchestration (vista de flota). Una sola instancia de VM-Totebox es suficiente para una pequeña empresa. Las organizaciones más grandes añaden VM-Orchestration para agregar a través de múltiples archivos.

**Usuarios de sistemas independientes** implementan VM-MediaKit (sitios web y portales de conocimiento) o VM-PrivateGit (control de fuentes) sin dependencia de un Totebox. Estos son dispositivos independientes: no requieren una malla WireGuard para funcionar, aunque opcionalmente pueden unirse a una.

## Hoja de Ruta de Unikernels

La Fase 1 para todos los tipos de VM utiliza Ubuntu 24.04 bajo QEMU (acelerado por KVM donde esté disponible, TCG como alternativa). Esta es la línea de base operativa actual.

La Fase 2 prevé alojar cada VM de forma más ligera: jails de FreeBSD para el aislamiento por carga de trabajo de MediaKit, Alpine Linux con binarios estáticos enlazados a musl para Totebox, y sandboxing gVisor para los agregadores de Orchestration.

La Fase 3 es el objetivo previsto de unikernel. Se prevé que VM-Totebox y VM-MediaKit se ejecuten como dominios de protección seL4 Microkit en hardware AArch64 (condicionado a la adquisición de hardware). Se prevé que los procesos del agregador de VM-Orchestration apunten a NanoVMs/OPS; las cargas de trabajo de inferencia SLM y GPU se prevé que permanezcan en un host Linux completo. Se prevé que la propia flota de hosts (VM-Infrastructure) ejecute el binario `os-infrastructure` en NetBSD+bhyve (capa de compatibilidad x86-64) o seL4+Microkit (capa nativa AArch64).

La restricción de Microkit 2.2.0 — solo AArch64 a partir de 2026 — implica que la Fase 3 en hardware x86-64 utiliza la ruta de compatibilidad NetBSD/bhyve, no la ruta seL4. Ambas rutas comparten el mismo libro mayor de capacidades (`system-core`).
