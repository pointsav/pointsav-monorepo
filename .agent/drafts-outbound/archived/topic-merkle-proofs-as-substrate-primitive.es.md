---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-merkle-proofs-as-substrate-primitive.es.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: TRANSLATE-ES
authored: 2026-04-28T01:00:00Z
updated: 2026-05-20T00:00:00Z
authored_by: task-project-system (session 181c94d9ca0491c5 skeleton; 37ac0f6b sustancia del panorama)
authored_with: opus-4-7 (esqueleto), claude-sonnet-4-6 (panorama)
references:
  - topic-merkle-proofs-as-substrate-primitive.md (canónico en inglés)
notes_for_editor: |
  Panorama estratégico per DOCTRINE.md §XII: adaptación, no traducción 1:1.
  El objetivo es que un lector en español pueda entender el alcance del TOPIC
  en inglés y decidir si leerlo completo. Substantive Spanish prose follows
  the strategic-adaptation pattern — shorter and more narrative than the
  English canonical.

  The English canonical has been fully substantiated (milestone N+1 complete
  2026-05-20). This Spanish overview should reflect that completed state.
---

# Pruebas de Merkle como Primitiva de Sustrato

Las pruebas de Merkle son el mecanismo criptográfico que permite al sustrato
de Foundry garantizar — a cualquier tercero, sin necesidad de confianza previa
— que un registro específico forma parte de un registro (log) de sólo-adición,
y que ese log no ha sido reescrito entre dos puntos de observación. Estas dos
garantías fundamentan el Sustrato del Libro de Capacidades (afirmación
doctrinal #33) y el Sustrato Soberano de Dos Bases (afirmación doctrinal #34).


## Resumen

La función hash SHA-256 produce una huella digital de 32 bytes a partir de
cualquier secuencia de bytes. Un árbol de Merkle organiza una secuencia de
registros en un árbol binario de hashes: cada hoja es el hash de un registro;
cada nodo interno es el hash de sus dos hijos. La raíz — un único valor de 32
bytes — es un *compromiso* con toda la secuencia. Cualquier modificación a
cualquier registro cambia la raíz.

Una prueba de inclusión para un registro específico es una lista de hashes
hermanos a lo largo del camino desde esa hoja hasta la raíz. Un verificador
que posee el hash de la hoja y la lista de hermanos puede reconstruir la raíz.
Si coincide con la raíz afirmada, el registro está en el árbol.

El tamaño de la prueba crece logarítmicamente con el tamaño del log: unas
10 firmas para un log de 1.000 registros, 20 para un millón. El costo de
verificación en el crate `system-core` es de 5–18 microsegundos para tamaños
típicos de árbol.

El mismo árbol de Merkle admite dos tipos distintos de prueba que responden
a preguntas diferentes: las **pruebas de inclusión** (RFC 9162 §2.1.3)
responden "¿está este registro en el log con esta raíz?", y las **pruebas de
consistencia** (RFC 9162 §2.1.4) responden "¿es este nuevo estado del log
una extensión válida del estado anterior, o se ha reescrito el historial?".


## Secciones del TOPIC en inglés

### §1 — Qué son las pruebas de Merkle

Explica la construcción del árbol hash (nodos hoja con prefijo 0x00, nodos
internos con prefijo 0x01 per RFC 9162 §2.1), la separación de dominio que
previene ataques de segunda preimagen, el coste logarítmico de las pruebas, y
la distinción entre *comprometerse con* una secuencia (producir la raíz) y
*probar la pertenencia* a esa secuencia (proveer el camino de hermanos).

### §2 — Dos sabores: inclusión y consistencia

Describe cuándo se necesita cada tipo de prueba. Las pruebas de inclusión
validan escrituras individuales: antes de registrar un testigo en el libro de
capacidades, se verifica que su hash aparece en el árbol cubierto por el
checkpoint firmado actual. Las pruebas de consistencia validan la replicación:
una réplica que avanza del checkpoint A al checkpoint B puede verificar que la
extensión no contiene reescrituras antes de aceptar el nuevo estado.

### §3 — Pruebas de inclusión en `system-core`

Documenta la estructura `InclusionProof { leaf_index, tree_size, sibling_hashes }`
y el algoritmo de verificación per RFC 9162 §2.1.3 verbatim (variables `fn_`
y `sn`). Cubre la suite de pruebas de 11 casos incluyendo la cuadrícula
completa de tamaños 1..=8, y los números de rendimiento medidos: 5,37 µs para
un árbol de 8 hojas, 17,74 µs para 1.024 hojas.

### §4 — Pruebas de consistencia en `system-core`

Documenta la estructura `ConsistencyProof { hashes }` y el algoritmo de dos
acumuladores (`old_hash`, `new_hash`) sembrados desde `hashes[0]` per RFC
9162 §2.1.4. Explica los 9 variantes de error que distinguen fallos
estructurales (longitud del camino) de fallos criptográficos (raíz incorrecta).
Describe los casos extremos: `old_size == 0`, `old_size == new_size` (prueba
de identidad vacía), y árboles no potencia-de-dos. Incluye la cuadrícula
completa 1..=8.

### §5 — Primitivas compuestas sobre `SignedCheckpoint`

Explica el formato de nota firmada C2SP (origen, tamaño del árbol, hash raíz,
líneas de firma ed25519 con prefijo de 4 bytes) y cómo las llamadas compuestas
`verify_inclusion_proof` y `verify_consistency_proof` encadenan tres verificaciones
en secuencia: invariante de tamaño de árbol → verificación de firma → verificación
de prueba cruda. Argumenta por qué las pruebas crudas **no** son la API de cara
al kernel: verificar una prueba cruda contra una raíz no autenticada no ofrece
ninguna garantía de seguridad.

### §6 — Integración del consumidor en `system-ledger`

Describe el rasgo `LedgerConsumer` y el cambio en Phase 1A.4: `apply_witness_record`
pasó de aceptar registros sin verificación a requerir una prueba de inclusión
válida (cambio de firma v0.1.x → v0.2.0). Explica la arquitectura de caché:
un acierto de caché tarda 11,2 ns frente a los 4 ms de una verificación ed25519
— una diferencia de 358.000× que hace que el caché sea una parte crítica de
la arquitectura, no una optimización opcional.

### §7 — Por qué esto importa como primitiva de sustrato

Sintetiza tres propiedades que el sustrato adquiere mediante las pruebas de
Merkle: (1) auditabilidad sin custodia — cualquier parte puede verificar un
registro con sólo el checkpoint firmado y la prueba, sin acceso al log
completo; (2) inmutabilidad del historial — las pruebas de consistencia hacen
que la reescritura del log sea detectable por cualquiera que haya registrado
dos checkpoints; (3) replicación sin confianza — una réplica puede avanzar de
estado sin confiar en el operador del log. Señala que el crate es elegible para
compilación `no_std`, habilitando el consumo directo desde el kernel sin frontera
FFI, y que la novedad es la composición, no los algoritmos individuales.

### §8 — Referencias cruzadas

Enumera las referencias normativas: RFC 9162, especificación C2SP signed-note,
`conventions/worm-ledger-design.md`, afirmaciones doctrinales #33 y #34,
`conventions/system-substrate-doctrine.md`, y los commits de implementación del
cluster project-system Phase 1A.4 y 1A.5.


---

*(El TOPIC canónico en inglés está en `topic-merkle-proofs-as-substrate-primitive.md`.
Esta versión en español es un panorama estratégico, no una traducción
palabra-por-palabra, según DOCTRINE.md §XII.)*
