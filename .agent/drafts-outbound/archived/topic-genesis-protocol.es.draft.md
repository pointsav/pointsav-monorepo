---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: genesis-protocol.es.md
audience: operadores técnicos e ingenieros que evalúan o despliegan flotas PointSav
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-20
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/infrastructure-os.md
  - architecture/machine-based-auth.md
  - architecture/diode-standard.md
  - infrastructure/sovereign-mesh.md
notes_for_editor: >
  Traducción paralela del borrador en inglés topic-genesis-protocol.draft.md.
  Mismo contenido y estructura. Los nombres de producto (Genesis Protocol,
  Diode Standard, os-infrastructure, os-network-admin, service-pointsav-link,
  service-pairing, Noise Protocol, WireGuard, WebSocket) no se traducen.
  Frontmatter del artículo al hacer commit: title "El Genesis Protocol",
  category "architecture", status "active", quality "review",
  cites [infrastructure-os, machine-based-auth, sovereign-mesh].
research_done_count: 4
research_suggested_count: 2
open_questions_count: 1
research_provenance: >
  Derivado del borrador en inglés topic-genesis-protocol.draft.md, a su vez
  basado en cuatro documentos fuente leídos directamente del árbol de trabajo.
  No se realizó investigación adicional para la traducción.
research_inline: true
---

# El Genesis Protocol

El Genesis Protocol es la secuencia de arranque inicial de flota que utiliza cada nodo `os-infrastructure` en su primer arranque. Permite que un nodo se vuelva operativo en hardware aislado — sin configuración previa, sin conexión a ningún plano de control y sin conocimiento de la flota a la que eventualmente se unirá — y que permanezca en un estado seguro y reclamable hasta que un administrador esté listo para incorporarlo a la gestión. El protocolo invierte el supuesto convencional de que el plano de control debe existir antes de que se pueda añadir cómputo a él.

## El problema que resuelve

La gestión convencional de flotas requiere una dependencia de secuenciación: el plano de control debe estar configurado, la red debe estar enrutada y el nodo debe estar inscrito antes de que sea útil. Para un operador que envía hardware a una ubicación remota, esto crea un problema de coordinación — el hardware llega al sitio remoto antes de que la capa de gestión de flotas esté lista, o viceversa.

El Genesis Protocol elimina la dependencia de secuenciación. Un nodo puede enviarse a cualquier ubicación, arrancar en cualquier entorno de red y alcanzar un estado seguro y autónomo sin ningún aprovisionamiento previo. El administrador lo reclama cuando esté listo.

## Los cinco pasos

### 1 — Arranque ciego

En el primer arranque, el kernel seL4 genera un par de claves fiduciarias de Nivel 1 a partir de entropía de hardware. A continuación, el nodo entra en modo de arranque ciego: deliberadamente ignora DHCP y DNS, rechazando adquirir una dirección de red mediante mecanismos convencionales. Esto impide que el nodo sea alcanzado por, o se comunique con, cualquier infraestructura que no haya verificado previamente.

### 2 — Exploración

El nodo explora la red local en busca de una baliza `os-network-admin` en el puerto de malla PPN. La exploración utiliza la clave pública fiduciaria del nodo como identidad que presenta — de modo que solo una instancia legítima de `os-network-admin` que posea el material de clave administrativa correspondiente puede responder con autoridad.

### 3 — Bifurcación génesis

Si la exploración no encuentra ninguna baliza `os-network-admin` dentro de la ventana de exploración, el nodo forma una Red Privada de Uno. Sella todos los puertos externos excepto uno, un único punto de acceso reforzado. No intenta contactar ningún servicio externo. No falla. Espera.

Un nodo que ha realizado la bifurcación génesis está completamente operativo: tiene su par de claves, tiene un perímetro de red sellado y está esperando una reclamación. No es un nodo defectuoso — es un nodo preparado para la flota que aún no ha sido reclamado.

### 4 — Patrón de espera

El único punto de acceso abierto es una interfaz WebSocket reforzada. Solo acepta una clase de mensaje: una solicitud de reclamación administrativa que presente un par de claves fiduciarias válido. Cualquier otro intento de conexión se descarta silenciosamente. El nodo no emite ninguna información identificativa a la red; para un observador externo, el punto de acceso es opaco.

### 5 — Reclamación

Cuando un administrador arranca `os-network-admin` y presenta la clave fiduciaria administrativa, el punto de acceso en patrón de espera verifica la clave contra el par de claves almacenado localmente en el nodo. Si el par verifica:

1. El nodo se vincula a la flota, aceptando la instancia de `os-network-admin` como su autoridad
2. El nodo recibe su configuración WireGuard de malla y se une a la [[sovereign-mesh|malla soberana]]
3. El par de claves fiduciarias del nodo se registra en el registro de emparejamiento de `os-network-admin` como emparejamiento ADMIN
4. Los puertos externos sellados se abren de acuerdo con la política del Diode Standard de la flota

Si la clave no verifica, la reclamación se rechaza silenciosamente. El nodo permanece en su patrón de espera y no emite ninguna respuesta de error.

## Ensamblaje de flota diferido

El Genesis Protocol está diseñado para el caso en que el hardware se envía antes de que el administrador esté listo para gestionarlo. Un operador puede enviar cincuenta servidores a cincuenta ubicaciones remotas. Cada uno llega, arranca y forma una PPN de un solo nodo. Cuando el administrador esté listo — días, semanas o meses después — `os-network-admin` reclama los cincuenta en secuencia. Cada uno verifica la reclamación y se une a la flota. El operador nunca necesita tocar físicamente los nodos después del envío inicial.

Este patrón funciona porque el patrón de espera es indefinidamente estable. Un nodo en patrón de espera consume energía mínima, mantiene su perímetro sellado y no presenta ninguna superficie de ataque más allá del punto de acceso de reclamación reforzado.

## Relación con la autorización basada en máquina

El Genesis Protocol es la fase de aprovisionamiento de la [[machine-based-auth|autorización basada en máquina]]. El par de claves fiduciarias generado en el primer arranque se convierte en la entrada de emparejamiento ADMIN de ese nodo en el registro de emparejamiento de la flota. Tras la reclamación:

- El material de clave del nodo es la única base para toda la autenticación posterior — nunca se transmite ninguna contraseña
- El emparejamiento puede revocarse eliminando la entrada ADMIN del registro; el nodo se vuelve ingobernable por la flota (aunque continúa ejecutando sus cargas de trabajo de forma autónoma)
- La autoridad de clave fiduciaria administrativa reside físicamente con el administrador — nunca se delega a un servicio en la nube

## Véase también

- [[infrastructure-os]] — el sustrato de cómputo que ejecuta el Genesis Protocol en el primer arranque
- [[os-network-admin]] — el plano de control que ejecuta la secuencia de reclamación
- [[sovereign-mesh]] — la superposición WireGuard a la que se une el nodo tras una reclamación exitosa
- [[machine-based-auth]] — el sistema de pares de claves fiduciarias en el que se basa el protocolo
- [[diode-standard]] — la jerarquía de autoridad que rige la flota después de la reclamación

---

## Rastro de investigación

### Realizado

1. Lectura de `systems/infrastructure-os.md` — secuencia de cinco pasos del Genesis Protocol, fundamento del arranque ciego, patrón de ensamblaje de flota diferido, formación de PPN
2. Lectura de `architecture/machine-based-auth.md` — tipo de emparejamiento ADMIN, Noise Protocol y claves estilo WireGuard, vinculación de hardware de claves privadas, mecanismo de revocación
3. Lectura de `architecture/diode-standard.md` — contexto de jerarquía de autoridad; os-infrastructure es un Sujeto, os-network-admin es la Autoridad que lo rige
4. Lectura del borrador `infrastructure/sovereign-mesh.md` — secuencia de unión a la malla post-reclamación, registro del par WireGuard, asignación de IP desde el registro de emparejamiento

### Sugerido (no realizado en esta sesión)

1. Leer `fleet-infrastructure-onprem/guide-genesis-protocol.md` si existe — el manual operativo puede contener detalles adicionales sobre la secuencia de reclamación y la ceremonia de claves
2. Leer `os-infrastructure/src/main.rs` — código del prototipo actual; comprender la brecha entre la implementación EAPOL actual y el objetivo del Genesis Protocol descrito aquí

### Preguntas abiertas

1. **EAPOL vs Genesis Protocol** — el `os-infrastructure/src/main.rs` actual implementa modo monitor EAPOL, no el Genesis Protocol descrito en este artículo. Este artículo describe la arquitectura prevista. La alineación del código es una decisión pendiente del operador. No se necesita corrección en este artículo — la arquitectura prevista es lo que el artículo debe describir.
