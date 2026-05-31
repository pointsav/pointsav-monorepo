---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-capability-ledger-substrate.es.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: TRANSLATE-ES
authored: 2026-05-20T00:00:00Z
authored_by: task-project-system (session 37ac0f6b)
authored_with: claude-sonnet-4-6
references:
  - topic-capability-ledger-substrate.md (canónico en inglés)
notes_for_editor: |
  Panorama estratégico per DOCTRINE.md §XII: adaptación, no traducción 1:1.
  El objetivo es que un lector en español pueda entender el alcance del TOPIC
  en inglés y decidir si leerlo completo.
---

# El Sustrato del Libro de Capacidades

El Sustrato del Libro de Capacidades es el mecanismo por el cual cada decisión
de control de acceso en un despliegue de Foundry se convierte en un evento
criptográficamente auditable, anclado en un registro que controla el cliente.
Extiende el modelo de capacidades nativo del microkernel seL4 — verificado
formalmente — con una capa de transparencia que hace que el registro de
auditoría sea portátil, de raíz del cliente y verificable por terceros sin
ninguna relación de confianza con el operador. Esta es la afirmación
doctrinal #33.


## Resumen

Un microkernel decide si un proceso tiene permiso para acceder a un recurso
de hardware. seL4 usa *capacidades* — tokens infalsificables que codifican
exactamente qué recurso posee un proceso y qué operaciones puede realizar. La
implementación C de seL4 está verificada formalmente contra una especificación
matemática: el kernel no puede ser engañado para honrar una capacidad que
debería rechazar.

El Sustrato del Libro de Capacidades añade una propiedad nueva: cada decisión
de invocación de capacidad puede vincularse, mediante una prueba de inclusión
de Merkle, a un checkpoint en un registro de transparencia firmado. El cliente
posee las claves de firma para ese registro. El cliente puede auditar el
historial completo. Terceros pueden verificar entradas individuales contra
checkpoints publicados sin acceder al registro completo.

El resultado es un sustrato de seguridad con dos capas verificables
independientemente: la capa del kernel (prueba formal de seL4) y la capa del
libro (registro de auditoría Merkle, que no puede ser reescrito sin las claves
apex del cliente). La combinación es lo que la afirmación doctrinal #33 nombra
como el salto adelante.


## Secciones del TOPIC en inglés

### §1 — Qué es el Sustrato del Libro de Capacidades

Explica el modelo de capacidades de seL4 (tokens infalsificables, sin
autoridad ambiental, sin usuario root), la prueba formal del kernel, y qué
añade el Sustrato del Libro: un anclaje de cada capacidad a un checkpoint
firmado en un registro de transparencia de raíz del cliente. El kernel comprueba
dos capas independientes: la capa formal (¿es esta capacidad estructuralmente
válida?) y la capa del libro (¿está el estado de esta capacidad en el registro
auditado?).

### §2 — El tipo `Capability` — campos, semántica, enlace al kernel

Documenta la estructura `Capability` en `system-core`: `cap_type`
(Endpoint, Memory, Irq, Notification, CNode — vocabulario nativo de seL4),
`rights` (Read, Write, Invoke, Grant, Revoke), `expiry_t` (opcional: segundos
Unix; None = sin caducidad incorporada), `witness_pubkey` (clave SSH para
extensión de caducidad), y `ledger_anchor` (identificador del checkpoint en el
registro Merkle del cliente). Explica cómo `Capability::hash()` produce un
SHA-256 determinista que sirve como clave en el árbol Merkle y en el conjunto
de revocación.

### §3 — Capacidades con límite de tiempo (Mecanismo A)

Explica el flujo de decisión del kernel para una capacidad con `expiry_t`:
si `now < expiry_t`, se honra la invocación; si ha caducado y no hay testigo,
se rechaza; si hay un `WitnessRecord` con firma válida y prueba de inclusión
Merkle que lo acredita en el registro actual, se extiende la caducidad y se
honra la invocación. El requisito de prueba de inclusión significa que una
extensión de vida de capacidad no puede ser honrada hasta que haya sido
comprometida en el registro de transparencia del cliente.

### §4 — La ceremonia de traspaso de apex (N+3+)

Documenta el protocolo formal de rotación de claves apex a través de cuatro
alturas de checkpoint: N (última operación con P-antiguo), N+1 (entrada de
revocación de P-antiguo), N+2 (checkpoint de traspaso co-firmado por ambos
apexes), N+3+ (sólo P-nuevo aceptado; P-antiguo rechazado con `StaleApex`).
Explica las tres propiedades del protocolo: atomicidad (el traspaso es un
evento único en el registro), auditabilidad (cualquier tercero puede
identificar el checkpoint exacto donde cambió la autoridad), y finalidad (la
rotación es permanente sin una nueva ceremonia).

### §5 — La máquina de estados `LedgerConsumer`

Describe el rasgo `LedgerConsumer` en `system-ledger` y el flujo de consulta:
validez del apex → invariante post-traspaso → comprobación de revocación →
comprobación de caducidad → extensión por testigo. Cubre los tres tipos de
veredicto (`Allow`, `Refuse(reason)`, `ExtendThenAllow`) y los cuatro métodos:
`consult_capability` (lectura), `apply_revocation`, `apply_apex_handover`,
`apply_witness_record` (escrituras).

### §6 — Disciplina de caché — por qué la diferencia de 358.000× es crítica

Explica el problema: la verificación de firma ed25519 tarda ~4 ms en el
hardware de referencia; no se puede verificar en cada invocación de capacidad
sin un coste insostenible. La solución: el `CheckpointCache` guarda los últimos
N checkpoints verificados; una búsqueda en caché cuesta 11,2 ns frente a
4 ms — una diferencia de 358.000×. En funcionamiento estable, la tasa de
aciertos de caché se acerca al 100%. Explica que la caché y las pruebas de
inclusión son complementarias, no alternativas: la caché hace rápida la ruta
de lectura; las pruebas hacen fiable la ruta de escritura.

### §7 — Revocación e invariantes post-traspaso

Explica la diferencia entre revocación (una capacidad específica es
permanentemente inválida — comprobación O(1) en un `HashSet<Hash256>`) e
invariante post-traspaso (una clave apex entera es inválida para un período
entero de tiempo, gobernado por la altura del checkpoint). Ambas son
aplicadas por `InMemoryLedger` y verificadas en el test de integración
de extremo a extremo de la ceremonia de traspaso completa.

### §8 — Relación con el libro WORM

Describe la relación arquitectónica: el sustrato WORM
(`conventions/worm-ledger-design.md`) es la capa de almacenamiento de registros
fundamental. `service-fs` (Ring 1) es el consumidor a nivel de aplicación.
`system-ledger` es el consumidor a nivel de sustrato. Ambos usan el mismo
formato de nota firmada C2SP; ambos verifican firmas ed25519 del apex; la
diferencia es la posición de despliegue (kernel-adyacente vs. espacio de
usuario) y el presupuesto de latencia. El `system-core` compartido evita la
duplicación de la mecánica de pruebas entre consumidores.

### §9 — Referencias cruzadas

Afirmaciones doctrinales #33 y #34; `conventions/system-substrate-doctrine.md`;
`conventions/worm-ledger-design.md`; TOPIC compañero sobre pruebas de Merkle;
estado de implementación del cluster project-system Phase 1A (system-core
v0.2.0 con 51 pruebas; system-ledger v0.2.1 con 44 pruebas + 10 benchmarks).


---

*(El TOPIC canónico en inglés está en `topic-capability-ledger-substrate.md`.
Esta versión en español es un panorama estratégico, no una traducción
palabra-por-palabra, según DOCTRINE.md §XII.)*
