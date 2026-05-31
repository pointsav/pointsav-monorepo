---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: ppn-architecture-overview.es.md
audience: responsables técnicos, operadores e ingenieros que evalúan o despliegan PointSav
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/infrastructure-os.md
  - systems/os-network-admin.md
  - architecture/sovereign-mesh.md
  - architecture/genesis-protocol.md
  - architecture/ppn-command-protocol.md
  - architecture/ppn-hypervisor-resource-pool.md
  - systems/totebox-archive.md
  - systems/os-orchestration.md
notes_for_editor: >
  Par en español de topic-ppn-architecture-overview.draft.md. Traducción directa del inglés.
  Verificar terminología técnica en español contra el glosario de content-wiki-documentation.
  Frontmatter del artículo a agregar al confirmar: title "Descripción General de la Arquitectura PPN",
  category "architecture", status "active", quality "review",
  cites [infrastructure-os, os-network-admin, sovereign-mesh, genesis-protocol,
         ppn-command-protocol, ppn-hypervisor-resource-pool, totebox-archive, os-orchestration].
research_done_count: 8
research_suggested_count: 0
open_questions_count: 0
---

# Descripción General de la Arquitectura PPN

La **Red Privada PointSav (PPN)** es el plano de infraestructura física del stack de
PointSav. Es la capa responsable de: incorporar nodos físicos a una malla autenticada
criptográficamente, gestionar los recursos de cómputo que esos nodos proporcionan, y
alojar las máquinas virtuales que ejecutan Totebox Archives y pasarelas de orquestación.
La PPN no es una capa de acceso a datos. No almacena datos. No toma decisiones de
autenticación sobre quién puede leer un archivo. Gestiona la infraestructura física para
que la capa de datos pueda ejecutarse sobre ella.

## Las cuatro capas

La PPN y los sistemas construidos sobre ella se organizan en cuatro capas. Cada capa es
ciega al estado interno de las capas por debajo y por encima de ella.

### Capa del operador

La capa del operador es donde un administrador humano interactúa con la flota.

**`os-network-admin`** es la capa Foundation OS — el plano de control para la malla PPN.
Se ejecuta en la máquina del operador (bare metal o contenedor LXC), gestiona la
distribución del mapa de pares, y aplica las reglas del Diodo que restringen el flujo de
comandos. No posee autoridad criptográfica: no puede leer datos de archivos, no puede
aprobar el acceso a datos, y no puede emitir credenciales de identidad. Su rol es conocer
qué nodos físicos están en la malla y hacer cumplir esa membresía — nada más.

**`app-network-admin`** es la interfaz del Terminal F8 que se ejecuta sobre
`os-network-admin`. Acepta la intención del operador en lenguaje natural en el puerto HTTP
8085, la enruta a través de `service-slm` para producir un comando binario de 16 bytes
autorizado, y transmite ese comando por UDP en el puerto 8090 a la malla.

Ver: [[os-network-admin]], [[ppn-command-protocol]]

### Capa PPN

La capa PPN es el sustrato de transporte físico y de ceremonia.

La **[[sovereign-mesh|malla soberana]]** es una superposición criptográfica WireGuard en
una interfaz `ppn0` dedicada. Cada nodo de la flota posee un par de claves a largo plazo;
cada paquete se encripta antes de salir del nodo. Los comandos viajan como paquetes
binarios de 16 bytes transmitidos simultáneamente a todos los pares de la malla; solo el
nodo al que se dirige actúa.

**`service-ppn-pairing`** es el backend de ceremonia que gestiona las solicitudes de unión
de nodos. Cuando un nuevo nodo físico quiere unirse a la malla, genera un código corto
en base32 de Crockford (~40 bits de entropía). El operador ingresa este código; un
intercambio CPace PAKE establece una clave de sesión compartida; una comparación de
Cadena Corta Autenticada (SAS) cierra la brecha de intermediario. El nodo aprobado se
escribe en el registro de solo adición `nodes.jsonl`.

El **[[genesis-protocol|Protocolo Génesis]]** gobierna el primer arranque: un nodo genera
su par de claves a partir de entropía de hardware, entra en un patrón de espera sellado,
y aguarda una reclamación administrativa — sin ninguna preconfiguración ni dependencia del
plano de control.

### Capa de hipervisor

La capa de hipervisor es el sustrato de cómputo.

**`os-infrastructure`** es el hipervisor Tipo I que aloja las máquinas virtuales que
ejecutan Totebox Archives y pasarelas de orquestación. Gestiona un **pool de recursos
por nodo**: memoria mediante `virtio_balloon` (la inflación recupera la RAM del invitado
en el pool del nodo; la deflación la devuelve) y CPU mediante `cpu.weight` de cgroups v2
por proceso QEMU.

El pool está acotado al nodo físico. La colocación de cargas de trabajo entre nodos es
responsabilidad de la capa de Orquestación de Totebox; una vez que una VM se coloca en
un nodo, el hipervisor gestiona su asignación de recursos local.

Ver: [[ppn-hypervisor-resource-pool]]

### Capa de Orquestación de Totebox

La capa de Orquestación de Totebox es el plano de datos. Se ejecuta dentro de las VMs
gestionadas por el hipervisor y está completamente separada de la PPN.

Los **Totebox Archives** (`cluster-totebox-*`) son bóvedas de datos soberanas — ledgers
WORM inmutables empaquetados como imágenes de disco de arranque libremente transferibles.
Cada archivo posee un par de claves Ed25519 registrado en `pairings.yaml`. Expone datos
solo mediante objetos de capacidad firmados entregados a través del Protocolo PointSav (PSP).

**`os-console`** es el terminal de operador nativo de teclado. Se conecta a un archivo a
la vez (o a varios mediante `os-orchestration`) y es el nivel gratuito.

**`os-orchestration`** es un agregador sin estado para múltiples archivos. Distribuye
consultas PSP a través de muchos archivos simultáneamente, devuelve solo filas de
resultados (nunca registros brutos), y no posee claves propias. Es el nivel de pago: la
agregación de múltiples archivos es el límite comercial.

Ver: [[totebox-archive]], [[os-orchestration]]

## Tres propiedades clave

### Invariante de aislamiento

El hipervisor no tiene capacidad de lectura sobre el estado interno de la VM. Sabe que
una VM está consumiendo N páginas de RAM y Y por ciento de vCPU. No sabe si la VM está
ejecutando `os-totebox`, `os-orchestration` o cualquier otra cosa. El Totebox Archive
dentro de la VM no sabe nada sobre la inflación del balloon, los pesos de cgroup, ni en
qué nodo físico se encuentra.

Esto significa que una vulnerabilidad en la capa de enrutamiento — la PPN — no proporciona
acceso al contenido de los archivos. Los dos planos son estructuralmente ciegos entre sí.

### Archivos libremente transferibles

Debido a que el hipervisor gestiona solo el ciclo de vida de la VM y la asignación de
recursos — no los datos dentro de las VMs — un Totebox Archive puede detenerse, copiarse
la imagen de disco a cualquier otro nodo PPN, y reiniciarse allí sin ningún cambio en sus
datos ni en su identidad. El pool del nodo destino absorbe la carga de trabajo. El
historial, las claves y el estado del archivo permanecen sin cambios.

La imagen de disco es el archivo. El pool de recursos es la infraestructura del nodo.
Mover la imagen mueve el archivo.

### Autoridad criptográfica nula en el plano de red

`os-network-admin` y la malla PPN no llevan autoridad criptográfica sobre los datos de
los archivos. Enrutar un nodo a la malla no concede ningún acceso a datos. Eliminar un
nodo de la malla no revoca ningún acceso a datos. El acceso a datos está gobernado
completamente por `pairings.yaml` y el sistema de par de claves MBA — un plano separado
sin conexión con la PPN.

Esta separación es intencional: el plano de control de red y el plano de acceso a datos
están diseñados de modo que un fallo o vulnerabilidad en uno no se propague al otro.

## Lo que la PPN no es

- **No es una capa de acceso a datos.** La PPN gestiona nodos físicos. El acceso a los
  datos de los archivos se realiza a través de `os-console` u `os-orchestration` mediante
  MBA + PSP — no a través de WireGuard.
- **No es un planificador de cómputo.** `os-network-admin` no planifica cargas de trabajo
  entre nodos. La colocación entre nodos es responsabilidad de `gateway-orchestration-command-1`
  (capa de Orquestación de Totebox). El hipervisor gestiona el pool de recursos local
  después de la colocación.
- **No es una autoridad de identidad.** La malla PPN sabe qué nodos físicos están
  inscritos. No sabe qué operadores están autorizados para leer qué archivos. Eso
  corresponde a `pairings.yaml`.

## Temas relacionados

- [[sovereign-mesh]] — superposición WireGuard, protocolo de comandos binarios de 16 bytes, topología hub-spoke
- [[genesis-protocol]] — secuencia de arranque autónomo en primer arranque, ensamblaje de flota diferido
- [[ppn-command-protocol]] — el formato de cable binario de 16 bytes transmitido por UDP en el puerto 8090
- [[service-pointsav-link]] — adaptador conectable en caliente que conecta nodos os-* a la flota
- [[os-network-admin]] — capa Foundation OS, autoridad criptográfica nula, ceremonia de unión de nodos
- [[ppn-hypervisor-resource-pool]] — virtio_balloon por nodo + planificación de vCPU
- [[totebox-archive]] — bóveda de datos WORM soberana, imagen de disco libremente transferible
- [[os-orchestration]] — agregador PSP sin estado, consultas de múltiples archivos, nivel de pago
