---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: systems/
target_filename: totebox-archive.es.md
audience: operadores, ingenieros y evaluadores que comprenden el concepto central de bóveda de datos
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-28
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/infrastructure-os.md
  - systems/os-orchestration.md
  - systems/os-network-admin.md
  - architecture/machine-based-auth.md
  - architecture/ppn-hypervisor-resource-pool.md
notes_for_editor: >
  Par en español de topic-totebox-archive.draft.md. Traducción directa del inglés.
  Verificar terminología técnica en español contra el glosario de content-wiki-documentation.
  Frontmatter del artículo a agregar al confirmar: title "Totebox Archive",
  category "systems", status "active", quality "review",
  cites [infrastructure-os, os-orchestration, os-network-admin, machine-based-auth,
         ppn-hypervisor-resource-pool].
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
---

# Totebox Archive

Un Totebox Archive es una bóveda de datos soberana asignada a una única entidad — un
edificio, una empresa, una persona, o cualquier otra unidad que el operador defina. Es
la unidad fundamental de almacenamiento e identidad del stack de PointSav. El archivo se
empaqueta como una imagen de disco de arranque y se ejecuta como una máquina virtual en la
capa de hipervisor de la PPN. Se lanza bajo Apache 2.0 y está disponible de forma gratuita:
un operador, un archivo, es un despliegue gratuito.

## La imagen de disco es el archivo

La propiedad más importante de un Totebox Archive es que la imagen de disco de arranque y
el archivo son la misma cosa. No existe una base de datos externa, ningún bucket de
almacenamiento en la nube, ni ningún registro central que almacene los datos del archivo.
La imagen de disco contiene todo: los datos, el par de claves de identidad y el sistema
operativo que sirve las consultas.

Esto significa que un archivo puede:

- **Detenerse** — la VM se apaga; no se pierden datos; nada expira
- **Copiarse** — la imagen de disco es un archivo que puede duplicarse como cualquier otro
- **Reubicarse** — la copia se inicia en un nodo PPN diferente; el hipervisor del nodo
  destino asigna recursos de su propio pool para la nueva VM
- **Reanudarse** — el archivo continúa exactamente desde el estado en que estaba cuando se detuvo

El hipervisor del nodo receptor no sabe nada sobre los datos dentro de la VM. Asigna CPU
y RAM de su pool por nodo para alojar la VM. La identidad, las claves y los datos del
archivo viajan con la imagen de disco sin cambios. Esta es la propiedad de **transferencia libre**.

## Modelo de almacenamiento: archivos planos WORM

Un Totebox Archive almacena datos como archivos planos inmutables:

| Formato | Contenido |
|---|---|
| JSONL | Registros estructurados (entradas de ledger, registros de eventos, metadatos de entidad) |
| GeoParquet | Datos geoespaciales (límites de sitios, registros de ubicación) |
| Markdown | Documentos legibles por humanos (memorandos, notas, informes) |

Cada escritura es un anexo; ningún registro se modifica ni se elimina. Este es un ledger
**WORM (Write Once, Read Many)**. El archivo acumula un historial completo e inalterable
de cada entrada registrada. No existe operación `DELETE` ni `UPDATE` — solo anexar. Una
entrada que debe ser reemplazada se sigue de una entrada de corrección; la entrada
original permanece visible permanentemente en el registro.

Este diseño es intencional: el archivo es un registro de calidad legal de lo que una
entidad sabía y cuándo lo supo. La garantía de inmutabilidad es estructural, no una
opción de configuración.

## Modelo de acceso: solo Diodo + PSP

Un Totebox Archive no expone una API de propósito general. Solo responde a consultas
entregadas a través del **Estándar Diodo** — el flujo de comandos unidireccional que
gobierna el stack de PointSav:

```
os-console  ──┐
               ├──▶  os-orchestration  ──▶  os-totebox (dentro de la VM del archivo)
               │         (PSP)
               └──▶  os-totebox (directo, archivo único)
```

Las consultas llegan como **objetos de capacidad firmados** entregados a través del
**Protocolo PointSav (PSP)**, un protocolo binario basado en capacidades que se tuneliza
sobre TLS. Un objeto de capacidad otorga permiso para leer una fila específica o un
conjunto de filas — nada más. El archivo verifica la firma del objeto contra su par de
claves MBA, ejecuta la consulta y devuelve solo las filas de resultados coincidentes.
Nunca devuelve registros brutos en bloque. Nunca acepta comandos desde una dirección
fuera del flujo del Diodo.

Ninguna VM puede emitir comandos de regreso al plano de control. Ningún archivo puede
instruir a la malla PPN para agregar o eliminar un nodo. El flujo es estructuralmente
unidireccional.

## Par de claves de Autorización Basada en Máquina

Cada Totebox Archive posee un par de claves Ed25519. Este par de claves es la identidad
del archivo en la malla PointSav. Está registrado en `pairings.yaml`, el archivo que
gobierna qué operadores y orquestadores están autorizados para consultar el archivo.

Los tipos de emparejamiento son:

| Tipo | Nivel de acceso |
|---|---|
| ADMIN | Autoridad administrativa completa sobre este archivo |
| INPUT | Lectura-escritura; emparejamiento predeterminado para entrada de datos activa |
| USER | Solo lectura |
| INTERFACE | Solo metadatos; sin contenido de registros |

Un operador sin entrada en `pairings.yaml` no puede consultar el archivo,
independientemente de si está en la misma malla PPN. La accesibilidad de red y el acceso
a los datos son planos separados sin autoridad compartida.

## Convención de nomenclatura de clústeres

Los archivos se nombran con el prefijo `cluster-totebox-` seguido del dominio de datos
y un número de instancia:

| Nombre de instancia | Dominio |
|---|---|
| `cluster-totebox-corporate-1` | Registros de entidad corporativa |
| `cluster-totebox-personnel-1` | Registros de personal |
| `cluster-totebox-property-1` | Registros de propiedad inmobiliaria |

El prefijo `cluster-` indica una instancia de VM gestionada por la capa de hipervisor de
la PPN. El sufijo numérico permite que múltiples archivos del mismo dominio coexistan en
la misma flota.

## Relación con la familia de sistemas operativos

| Componente | Rol |
|---|---|
| `os-totebox` | El sistema operativo que se ejecuta dentro de la VM del archivo; gestiona el ledger WORM y la superficie de consulta PSP |
| `os-console` | Un terminal de operador nativo de teclado que se conecta a un archivo a la vez; nivel gratuito |
| `os-orchestration` | Un agregador sin estado para múltiples archivos que distribuye consultas a través de PSP; nivel de pago |
| `os-infrastructure` | El hipervisor Tipo I que aloja las VMs de archivo y gestiona el pool de recursos por nodo |

`os-totebox` y `os-console` son Apache 2.0 — gratuitos. `os-orchestration` introduce el
límite comercial: consultar un único archivo directamente a través de `os-console` es
gratuito; enrutar consultas a través de múltiples archivos mediante `os-orchestration`
es el nivel de pago.

## Lo que un Totebox Archive no es

- **No es una base de datos.** No hay planificador de consultas, ni índice, ni registro
  de transacciones en el sentido de una base de datos relacional. El archivo es un ledger
  de solo anexar que devuelve filas que coinciden con un objeto de capacidad firmado.
- **No es almacenamiento en la nube.** No hay bucket S3, ni API de almacenamiento de
  objetos, ni URL prefirmada. Los datos salen del archivo solo a través del camino
  Diodo + PSP.
- **No es un recurso compartido de archivos.** No hay recurso SMB, ni montaje NFS, ni
  punto final SFTP. Los operadores acceden a los datos a través de `os-console` u
  `os-orchestration`, no a través de protocolos de sistema de archivos.
- **No es una VM en el sentido general.** La VM es el medio de empaquetado. El archivo
  son los datos, la identidad y el historial que contiene. Cuando alguien se refiere al
  "Totebox Archive", se refiere a la imagen de disco y su contenido — no a la instancia
  de hipervisor que casualmente lo ejecuta en este momento.

## Temas relacionados

- **Infrastructure OS** — el hipervisor Tipo I que aloja las VMs de archivo y gestiona el pool de recursos
- **Pool de Recursos del Hipervisor PPN** — cómo el hipervisor otorga y recupera CPU/RAM de cada VM
- **OS Orchestration** — el agregador PSP sin estado para consultas de múltiples archivos
- **Autorización Basada en Máquina** — el modelo de par de claves Ed25519 que gobierna el acceso al archivo
- **Estándar Diodo** — el flujo de comandos unidireccional que atraviesa cada consulta al archivo
