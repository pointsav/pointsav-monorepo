---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: pointsav-monorepo
target_path: system-core/
target_filename: README.es.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-README
authored: 2026-04-28T02:00:00Z
authored_by: task-project-system (session 181c94d9ca0491c5, ps-administrator identity)
authored_with: sonnet-4-6
references:
  - commit 9b5e4fd (Phase 1A.4 InclusionProof)
  - commit 82b659f (Phase 1A.5 ConsistencyProof)
  - DOCTRINE.md claim #33
  - RFC 9162 §2 (Certificate Transparency 2.0)
notes_for_editor: |
  Spanish overview per DOCTRINE.md §XII strategic-adaptation pattern.
  NOT a 1:1 translation of the English canonical. Covers purpose,
  type families, and composed primitives at a strategic level;
  points to the English README for full technical detail.
---

# system-core

[ Read this document in English ](./README.md)

---

## Resumen

`system-core` es el cajón de primitivas del **Sustrato del Libro Mayor
de Capacidades** (afirmación doctrinal #33). Define los tipos de datos
puros y las primitivas criptográficas que utilizan todos los demás
cajones `system-*` y `moonshot-*`. No depende de ningún otro cajón del
espacio de trabajo; todos los que gestionan capacidades dependen de él.

**Estado:** Versión 0.2.0 — estructuralmente completo. 51 pruebas
unitarias pasan en Rust stable.

---

## Qué contiene

El cajón organiza su superficie pública en tres grupos.

**Primitivas de capacidad.** El tipo `Capability` es el token de
autorización mediado por el núcleo (kernel) que lleva el árbol de
derivación de capacidades de seL4. Cada capacidad incluye su tipo de
recurso, los derechos permitidos, una marca de tiempo de expiración
opcional, y un `LedgerAnchor` que la ancla a una posición específica en
el registro Merkle del cliente. El tipo `WitnessRecord` extiende una
capacidad más allá de su expiración — el núcleo verifica la prueba de
inclusión Merkle en el registro antes de aceptar la extensión.

**Punto de control con firma criptográfica (C2SP signed-note).**
`SignedCheckpoint` implementa el formato de cable completo de la
especificación C2SP: análisis, representación y verificación ed25519.
Se admiten múltiples firmantes sobre el mismo cuerpo del punto de
control, lo que realiza el mecanismo de rotación de autoridad apical:
al transferir la propiedad del registro, el titular saliente y el
entrante firman conjuntamente el mismo punto de control; el núcleo
acepta la transferencia solo cuando ambas firmas son válidas.

**Pruebas Merkle según RFC 9162.** El módulo `inclusion_proof`
comprueba que un registro concreto aparece en la raíz Merkle de un
punto de control determinado. El módulo `consistency_proof` comprueba
que una raíz anterior es un prefijo honesto de una raíz más reciente —
primitiva de seguridad para la replicación del registro.

---

## Primitivas compuestas

Los métodos de verificación compuestos en `SignedCheckpoint` son las
interfaces orientadas al núcleo:

- `verify_signer` — verifica la firma de un firmante específico.
- `verify_apex_handover` — exige que tanto el titular saliente como el
  entrante hayan firmado el mismo cuerpo; una única firma no es
  suficiente.
- `verify_inclusion_proof` — combina en una sola operación atómica la
  verificación de firma y la prueba de inclusión Merkle.
- `verify_consistency_proof` — combina la verificación de ambas firmas
  (antiguo y nuevo punto de control) con la prueba de consistencia
  Merkle.

Llamar a los verificadores de pruebas en bruto sin autenticar
primero la raíz contra la que se verifican equivale a confiar en
una raíz no autenticada. Los métodos compuestos eliminan esa clase
de error.

---

## Lo que viene a continuación

El cajón `system-ledger` (Fase 1A incremento 3, activo) es el
consumidor de estas primitivas: implementa la caché de puntos de
control, el conjunto de revocaciones, el historial de apices, y el
rasgo `LedgerConsumer` con su implementación `InMemoryLedger`. La
llamada operativa al núcleo — `consult_capability(cap, current_root)
-> Verdict` — vive en `system-ledger`, no aquí.

La documentación técnica completa — esquemas de tipos, firmas de
métodos, restricciones de diseño, instrucciones de compilación y
prueba, y referencias cruzadas — se encuentra en el documento
canónico en inglés ([README.md](./README.md)), conforme a la
convención bilingüe del espacio de trabajo (`~/Foundry/CLAUDE.md`
§6: el español es panorámica estratégica, no traducción literal).
