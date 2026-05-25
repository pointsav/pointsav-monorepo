---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: pointsav-monorepo
target_path: system-ledger/
target_filename: README.es.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-README
authored: 2026-04-28T06:00:00Z
authored_by: task-project-system (session 181c94d9ca0491c5, ps-administrator identity)
authored_with: sonnet-4-6
references:
  - commit 2b9ca9c (Phase 1A.4 trait change v0.2.0)
  - commit b0dba5e (end-to-end §4 ceremony)
  - DOCTRINE.md claim #33
  - system-substrate-doctrine.md §4
notes_for_editor: |
  Spanish overview per DOCTRINE.md §XII strategic-adaptation pattern.
  NOT a 1:1 translation of the English canonical. Headings mirror the
  English structure; prose is overview-level for a reader who follows
  Spanish but reaches the English canonical for technical detail.
---

<div align="center">

# system-ledger

[ Read this document in English ](./README.md)

</div>

---

## Resumen

`system-ledger` es la **máquina de estados del lado del kernel** que
consume las primitivas criptográficas de `system-core` para decidir
si honrar una invocación de capacidad. Implementa el patrón de
consulta al libro mayor descrito en la Afirmación #33 de DOCTRINE.md
(El Sustrato del Libro Mayor de Capacidades) y en
`conventions/system-substrate-doctrine.md` §3.1 + §4.

**Versión:** 0.2.1 — 44 pruebas + 10 benchmarks de criterion — v0.2.x
estructuralmente completo.

La documentación técnica detallada — especificaciones del trait, flujo
de decisión, números de benchmarks, instrucciones de compilación —
vive en el documento canónico en inglés ([README.md](./README.md)) por
convención bilingüe del espacio de trabajo. El español es panorámica
estratégica.

---

## I. Qué provee

La crate expone el trait `LedgerConsumer` y la implementación de
referencia `InMemoryLedger`. Los módulos internos son:

- `cache::CheckpointCache` — caché LRU de checkpoints recientes
  (capacidad predeterminada: 64 entradas)
- `revocation::RevocationSet` — conjunto de capacidades revocadas con
  membresía en O(1)
- `apex::ApexHistory` — historial de ápices append-only con invariante
  post-traspaso
- `witness::verify_witness_signature` — wrapper de `ssh-keygen -Y
  verify` con espacio de nombres `capability-witness-v1`

---

## II. Estado

Versión 0.2.1. El alcance de v0.2.x está estructuralmente completo.
La versión 0.2.0 introdujo un cambio incompatible en la firma del
trait `LedgerConsumer` (§IV más abajo).

---

## III. Trait LedgerConsumer

El kernel llama a `consult_capability` para cada invocación de
capacidad. El veredicto puede ser `Allow`, `Refuse(RefuseReason)` o
`ExtendThenAllow { new_expiry_t }` (extensión via registro testigo).

El flujo de decisión:

1. Verificar el checkpoint actual contra el historial de ápices.
2. Consultar el conjunto de revocaciones.
3. Verificar la expiración de la capacidad.
4. Si hay testigo: validar firma SSH + presencia en el árbol Merkle.

---

## IV. Ceremonia de traspaso de ápice (§4 N+3+)

`system-ledger` implementa la ceremonia de transferencia de propiedad
especificada en `system-substrate-doctrine.md` §4:

```
altura N      ápice anterior P-viejo; checkpoints de firma única
altura N+1    P-viejo firma entrada de revocación
altura N+2    P-viejo Y P-nuevo co-firman el checkpoint de traspaso
altura N+3+   solo P-nuevo aceptado; la firma de P-viejo es RECHAZADA
```

El invariante post-traspaso — "la firma de P-viejo en checkpoints de
altura N+3 en adelante es rechazada" — lo impone `ApexHistory`
junto con `consult_capability`. La prueba integral
`full_handover_ceremony_end_to_end` en `src/lib.rs` verifica las
cuatro fases de la ceremonia de extremo a extremo.

La ceremonia es atómica: no requiere migración de estado, tiempo de
inactividad ni intervención del proveedor. El nuevo ápice hereda todo
el historial de capacidades y auditoría.

---

## V. `apply_witness_record` requiere prueba de inclusión

En la versión 0.2.0, `apply_witness_record` recibe un `InclusionProof`
adicional. El llamador debe demostrar que el hash de la hoja del
registro testigo (RFC 9162 §2.1, `SHA-256(0x00 || bytes)`) está
presente en la raíz Merkle del checkpoint actual. Un fallo retorna
`Err(LedgerError::WitnessNotInRoot(_))`.

El método de acceso directo para pruebas — `apply_witness_record_unchecked`
— está disponible solo bajo `#[cfg(test)]`.

---

## VI. Disciplina de caché

La caché de checkpoints evita la verificación completa de firma
ed25519 en la ruta crítica de `consult_capability`. Un acierto de
caché cuesta aproximadamente 8 ns frente a los 3.40 ms de una
verificación ed25519 — una diferencia de ~420,000 veces. La caché y
las pruebas de inclusión son complementarias: operan sobre datos
distintos con propósitos distintos.

---

## VII. Espacio de nombres de la firma testigo

El espacio de nombres para firmas de registros testigo es
`capability-witness-v1`. El mismo primitivo `ssh-keygen` se usa en
tres contextos del espacio de trabajo con espacios de nombres
distintos, lo que previene ataques de reproducción entre contextos.

---

## VIII. Compilación y pruebas

```
cargo build -p system-ledger
cargo test  -p system-ledger
cargo bench -p system-ledger
```

---

## IX. Referencias cruzadas

- Crate hermana `../system-core/` — posee las primitivas de datos.
- DOCTRINE.md §II afirmación #33 — ancla constitucional.
- `conventions/system-substrate-doctrine.md` §3.1 + §4 + §5.
- `conventions/worm-ledger-design.md` §2 — pila de cuatro capas.
- `topic-merkle-proofs-as-substrate-primitive.md` (previsto,
  `content-wiki-documentation`) — contexto narrativo sobre las
  pruebas RFC 9162.

---

## X. Licencia

Hereda la `LICENSE` del monorepo en la raíz del repositorio.
