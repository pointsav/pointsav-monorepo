---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: service-pointsav-link.es.md
audience: operadores técnicos e ingenieros que evalúan o despliegan flotas PointSav
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-20
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - architecture/diode-standard.md
  - architecture/machine-based-auth.md
  - systems/os-network-admin.md
  - systems/infrastructure-os.md
notes_for_editor: >
  Traducción paralela del borrador en inglés topic-service-pointsav-link.draft.md.
  Mismo contenido y estructura. Los nombres de producto (service-pointsav-link,
  pointsav-protocol, Diode Standard, os-network-admin, os-infrastructure, os-totebox,
  os-mediakit, os-privategit, os-console, WireGuard, Noise Protocol, PPN Command Protocol)
  no se traducen.
  Frontmatter del artículo al hacer commit: title "service-pointsav-link",
  category "architecture", status "active", quality "review",
  cites [diode-standard, machine-based-auth, os-network-admin, infrastructure-os].
research_done_count: 4
research_suggested_count: 2
open_questions_count: 0
research_provenance: >
  Derivado del borrador en inglés topic-service-pointsav-link.draft.md, a su vez
  basado en cuatro documentos fuente leídos directamente del árbol de trabajo.
  No se realizó investigación adicional para la traducción.
research_inline: true
---

# service-pointsav-link

`service-pointsav-link` es el adaptador de conexión en caliente que conecta un nodo Sujeto `os-*` a una flota PointSav. Es el único código responsable de traducir los comandos de autoridad — emitidos por `os-network-admin` y entregados a través del [[ppn-command-protocol|PPN Command Protocol]] — en operaciones ejecutables por el Sujeto. El adaptador se distribuye como el paquete `pointsav-protocol`. Su propiedad más importante es su estado predeterminado: no está instalado. Un Sujeto sin `service-pointsav-link` no tiene ningún concepto de llamar a casa, recibir comandos o participar en la gestión de flota.

## Las cuatro propiedades

| Propiedad | Comportamiento |
|---|---|
| Estado predeterminado | No instalado; el Sujeto no tiene ningún concepto de llamar a casa |
| Estado activado | Conectado en caliente por el operador con un solo comando; pone al Sujeto bajo gestión de flota |
| Modo de fallo | Si el adaptador falla, el enlace se corta limpiamente; el Sujeto continúa ejecutándose de forma autónoma; la superficie de gestión de flota queda oscura |
| Ruta de código | La política del Diode Standard reside dentro del adaptador, no en el kernel del sistema operativo — la política puede actualizarse sin tocar el resto del sistema |

## Estado predeterminado: sin capacidad de llamar a casa

Un nodo `os-infrastructure` recién arrancado no tiene instalado `service-pointsav-link`. Esto no es una elección de configuración — es un invariante arquitectónico. El sistema operativo Sujeto no contiene ningún código que le permita iniciar contacto con ninguna autoridad. No hay cliente `ssh`, ni tabla de enrutamiento entre pares, ni iniciador RPC. El Sujeto es estructuralmente incapaz de llamar a casa.

Esta propiedad se cumple en toda la familia `os-*`. Cada Sujeto — `os-totebox`, `os-mediakit`, `os-privategit`, `os-infrastructure` y `os-network-admin` cuando actúa como Sujeto — comienza en un estado en el que la gestión de flota está ausente. Poner un nodo bajo gestión es siempre una acción explícita del operador, nunca automática.

## Activación: conectar el adaptador en caliente

Un operador activa `service-pointsav-link` en un Sujeto con un solo comando emitido desde `os-network-admin`. La secuencia de activación instala el paquete `pointsav-protocol`, registra el par de claves fiduciarias del nodo en el registro de emparejamiento de la flota como una entrada de Sujeto, y abre el único canal de comandos entrantes del adaptador.

Tras la activación, el Sujeto se vuelve direccionable por la difusión de comandos de la flota. El canal entrante del adaptador escucha en la malla PPN y acepta solo los paquetes cuyo código de operación está dentro del conjunto permitido definido por la política del [[diode-standard|Diode Standard]] de la flota. Todo el demás tráfico se descarta en el límite del adaptador.

## Modo de fallo: separación limpia

Si `service-pointsav-link` falla o se desinstala deliberadamente, el Sujeto no colapsa. Continúa ejecutando sus cargas de trabajo — sirviendo contenido, ejecutando cómputos, manteniendo su estado local — como si la gestión de flota nunca hubiera estado presente. La superficie de gestión de flota queda oscura: `os-network-admin` pierde visibilidad del Sujeto y no puede emitirle comandos. Pero los servicios propios del Sujeto no se ven afectados.

Este modo de fallo es por diseño. Un Sujeto que dependiera del adaptador para su operación normal estaría permanentemente acoplado al plano de control — cualquier interrupción del plano de control se propagaría a las cargas de trabajo del Sujeto. Al desacoplar la operación del Sujeto de la presencia del adaptador, `service-pointsav-link` garantiza que un fallo del plano de control sea una pérdida de observabilidad, no una interrupción del servicio.

## La política en el adaptador, no en el kernel

La política del [[diode-standard|Diode Standard]] — qué flujos de comandos están permitidos, qué operaciones se permiten desde qué autoridades, qué telemetría se emite — reside dentro de `service-pointsav-link`, no en el kernel del sistema operativo Sujeto. Esta separación tiene una consecuencia práctica: actualizar la política de flota requiere actualizar el adaptador, no reconstruir ni reiniciar el sistema operativo.

Una versión del sistema operativo Sujeto y una actualización de política del Diode Standard son despliegues independientes. Un operador puede endurecer o relajar la política de flota enviando una nueva versión del paquete `pointsav-protocol` a los Sujetos activos sin tocar ningún código del kernel.

## El estándar universal

`service-pointsav-link` no es una característica de un sistema operativo específico. El mismo paquete `pointsav-protocol`, con diferentes vinculaciones de política, es el adaptador entre cualquier par de instancias del sistema operativo PointSav que necesiten comunicarse. Un Sujeto `os-totebox` conectado a una Autoridad `os-console` utiliza la misma ruta de código del adaptador que un Sujeto `os-infrastructure` conectado a `os-network-admin`.

Este estándar uniforme es lo que hace auditable una flota compleja. Cada conexión tiene el mismo aspecto en la capa del protocolo. Un auditor que examine cualquier par de nodos conectados ve la misma forma del adaptador — el mismo comportamiento predeterminado desactivado, la misma secuencia de activación, la misma garantía de separación limpia.

## Véase también

- [[diode-standard]] — la jerarquía de autoridad y las reglas de tráfico que aplica el adaptador
- [[os-network-admin]] — la Autoridad que activa y ordena a los Sujetos a través del adaptador
- [[infrastructure-os]] — un Sujeto os-infrastructure que usa este adaptador para unirse a una flota
- [[ppn-command-protocol]] — el formato de cable binario de 16 bytes que recibe el adaptador
- [[machine-based-auth]] — los pares de claves fiduciarias que autentican el canal de comandos del adaptador

---

## Rastro de investigación

### Realizado

1. Lectura de `architecture/diode-standard.md` — §"El adaptador" (tabla de 4 propiedades: estado predeterminado no instalado, activado con un solo comando, modo de fallo separación limpia, ruta de código en el adaptador no en el kernel); §"El Estándar Universal" (mismo paquete distintas vinculaciones de política; el estándar uniforme hace la flota auditable); §"Por qué importa" (prevención del movimiento lateral)
2. Lectura de `architecture/machine-based-auth.md` — tipos de emparejamiento (ADMIN/INPUT/USER/INTERFACE); claves Noise Protocol vinculadas al hardware; revocación mediante eliminación de entrada de emparejamiento; service-pointsav-link participa en el registro de emparejamiento en la activación
3. Lectura de `systems/os-network-admin.md` — Autoridad que activa Sujetos a través del adaptador; gestión del registro de emparejamiento; propagación de reglas de cumplimiento del Diode Standard; reclamaciones de flota a través del Genesis Protocol
4. Lectura de `systems/infrastructure-os.md` — os-infrastructure es un Sujeto; el Genesis Protocol crea el emparejamiento ADMIN que precede a la activación; la malla WireGuard sobre la que se ejecuta el canal entrante del adaptador

### Sugerido (no realizado en esta sesión)

1. Leer el código fuente de `service-pointsav-link/` si está presente en el monorepo — confirmaría la API de activación exacta, el conjunto de códigos de operación permitidos para los Sujetos os-infrastructure, y el formato actual de vinculación de política
2. Leer `architecture/deployment-patterns.md` si existe — contendría ejemplos de secuencias de activación en las seis configuraciones de flota canónicas

### Preguntas abiertas

*(ninguna)*
