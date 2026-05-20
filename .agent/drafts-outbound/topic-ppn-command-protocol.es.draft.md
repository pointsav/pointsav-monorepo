---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: ppn-command-protocol.es.md
audience: operadores técnicos e ingenieros que evalúan o despliegan flotas PointSav
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-20
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - systems/os-network-admin.md
  - architecture/diode-standard.md
  - systems/infrastructure-os.md
  - architecture/machine-based-auth.md
notes_for_editor: >
  Traducción paralela del borrador en inglés topic-ppn-command-protocol.draft.md.
  Mismo contenido y estructura. Los nombres de producto (PPN Command Protocol,
  Diode Standard, os-network-admin, os-infrastructure, service-slm, service-udp,
  WireGuard, Noise Protocol, F8 Terminal, UDP) no se traducen.
  Frontmatter del artículo al hacer commit: title "El PPN Command Protocol",
  category "architecture", status "active", quality "review",
  cites [os-network-admin, diode-standard, infrastructure-os].
research_done_count: 4
research_suggested_count: 2
open_questions_count: 0
research_provenance: >
  Derivado del borrador en inglés topic-ppn-command-protocol.draft.md, a su vez
  basado en cuatro documentos fuente leídos directamente del árbol de trabajo.
  No se realizó investigación adicional para la traducción.
research_inline: true
---

# El PPN Command Protocol

El PPN Command Protocol es el formato de cable utilizado por cada plano de control `os-network-admin` para emitir comandos hacia los nodos `os-infrastructure` a través de la PointSav Private Network. Transmite comandos de flota como paquetes binarios de 16 bytes difundidos por UDP en el puerto 8090 sobre la malla WireGuard — sin intermediario central, sin cola y sin ningún servicio de terceros en el camino. Cada nodo de la flota recibe cada paquete simultáneamente; solo el nodo destinatario actúa.

## Restricciones de diseño

El protocolo fue moldeado por tres requisitos que excluyen los enfoques convencionales:

**Sin intermediario.** Un intermediario de mensajes es un punto único de fallo y un problema de frontera de confianza — debe autenticarse, mantenerse y ser de confianza. El PPN Command Protocol elimina completamente al intermediario. El plano de control difunde; la malla entrega; el nodo decide.

**Sin texto en claro.** El protocolo se ejecuta exclusivamente sobre la malla WireGuard. El handshake IK del Noise Protocol de WireGuard autentica a cada par antes de que se entregue cualquier paquete. Un paquete de comandos nunca viaja por un enlace sin cifrar.

**Sin verbosidad.** Los comandos tienen 16 bytes. No hay negociación de sesión, ni handshake de reconocimiento, ni sobrecarga de enmarcado. En el nodo receptor, una lectura de 16 bytes coincide con una operación conocida o no lo hace.

## El formato del paquete

Cada comando tiene exactamente 16 bytes. Los primeros dos bytes constituyen el código de operación: un byte identificando la clase de operación, un byte identificando el nodo destino. Los 14 bytes restantes están disponibles para llevar carga útil específica de la operación — dirección del nodo, política de aislamiento u otros parámetros según la clase de operación.

```
 0               1               2               3  ...  15
 ┌───────────────┬───────────────┬───────────────────────────┐
 │  clase op (1) │  destino (1)  │  carga útil (14 bytes)    │
 └───────────────┴───────────────┴───────────────────────────┘
```

El código de operación es producido por `service-slm` ejecutándose localmente en el nodo `os-network-admin` — la frase en lenguaje natural que escribe el administrador nunca llega al cable.

## La secuencia de despacho

1. El administrador escribe su intención en lenguaje natural en el terminal F8 — por ejemplo, instruyendo a la flota que aísle un nodo de borde específico.
2. `service-slm`, ejecutándose localmente en el nodo `os-network-admin`, analiza la frase y produce el código de operación de dos bytes que identifica la clase de operación y el destino.
3. `service-udp` construye el paquete completo de 16 bytes y lo difunde a través de la malla WireGuard en el puerto UDP 8090.
4. Cada nodo de la flota recibe el paquete simultáneamente. Solo el nodo cuya dirección coincide con el byte destino actúa; todos los demás descartan.

La capa de traducción es invisible en el límite del protocolo — la malla solo ve el comando binario, no la frase en lenguaje natural. No hay registro de auditoría del texto original en la capa del protocolo; ese registro reside en el log del terminal F8 en `os-network-admin`, no en la malla.

## Por qué difusión simultánea

El modelo de difusión es deliberado. Un modelo de unidifusión requeriría que el plano de control mantuviera una tabla de enrutamiento con direcciones individuales para cada nodo, y requeriría sesiones TCP por nodo o reconocimientos. Ambos introducen estado que puede desincronizarse.

La difusión sobre una malla WireGuard elimina ambos problemas. Cada par recibe cada paquete. El nodo destinatario actúa; los demás descartan en la primera comparación de bytes. El plano de control no tiene estado de enrutamiento que mantener más allá de la lista de pares de la malla, que gestiona la función de política de enrutamiento de malla de `os-network-admin`.

La restricción de seguridad está satisfecha por la propia malla: los no miembros no pueden recibir paquetes de la malla porque no poseen un handshake WireGuard válido con el hub.

## Relación con el Diode Standard

El PPN Command Protocol es la implementación en cable de la categoría de control descendente del [[diode-standard|Diode Standard]]. Fluye desde la autoridad (`os-network-admin`) hacia el sujeto (`os-infrastructure`) y nunca al revés. No existe un camino de comandos ascendente en el protocolo: el formato del paquete no contiene ninguna dirección de respuesta, ningún campo de reconocimiento y ningún mecanismo para que un nodo Sujeto inicie un paquete hacia la Autoridad.

La telemetría ascendente — logs, latidos, estado — viaja por un canal separado y estrictamente saneado. El canal de comandos y el canal de telemetría están desacoplados intencionalmente para que un fallo en uno no afecte al otro.

## Véase también

- [[os-network-admin]] — el plano de control que produce y difunde los paquetes de comandos
- [[infrastructure-os]] — los nodos de sustrato de cómputo que reciben y ejecutan los comandos
- [[diode-standard]] — la jerarquía de autoridad y las reglas de tráfico que implementa el protocolo
- [[sovereign-mesh]] — la superposición WireGuard sobre la que se ejecuta el protocolo
- [[service-slm]] — el enrutador semántico local que traduce la intención al código de operación de dos bytes
- [[machine-based-auth]] — los pares de claves fiduciarias que aseguran los pares de la malla

---

## Rastro de investigación

### Realizado

1. Lectura de `systems/os-network-admin.md` — secuencia de despacho de 4 pasos, "comando binario de dos bytes que identifica la operación y el nodo destino", "paquete de 16 bytes", "puerto 8090", "cada nodo de la flota recibe el paquete simultáneamente; solo el nodo destinatario actúa", "la capa de traducción es invisible en el límite del protocolo", "sin intermediario de mensajes central"
2. Lectura de `architecture/diode-standard.md` — tabla de categorías de tráfico (control descendente Autoridad→Sujeto permitido; control ascendente bloqueado estructuralmente); tabla del adaptador (estado predeterminado no instalado; activado con un solo comando; modo de fallo: separación limpia)
3. Lectura de `systems/infrastructure-os.md` — lado receptor: el nodo destinatario actúa, los demás descartan; contexto del par WireGuard; secuencia de unión a la malla del Genesis Protocol que coloca el nodo en la malla
4. Lectura de `architecture/machine-based-auth.md` — handshake IK del Noise Protocol, claves estilo WireGuard; claves privadas vinculadas al hardware; esta es la capa de seguridad que subyace a la malla sobre la que se ejecuta el protocolo

### Sugerido (no realizado en esta sesión)

1. Leer `service-udp/src/` — la implementación en Rust del emisor de difusión; confirmaría la construcción de la carga útil, la aplicación del tamaño del paquete y si los 14 bytes restantes se usan actualmente o se ponen a cero
2. Leer la documentación del modelo de enrutamiento de `service-slm/` — confirmaría cómo `service-slm` mapea una frase en lenguaje natural al código de operación de 2 bytes y cómo es la tabla de opcodes actual

### Preguntas abiertas

*(ninguna)*
