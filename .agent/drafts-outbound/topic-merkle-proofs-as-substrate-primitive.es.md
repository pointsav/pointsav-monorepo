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
authored_by: task-project-system (session 181c94d9ca0491c5, ps-administrator identity)
authored_with: opus-4-7
references:
  - topic-merkle-proofs-as-substrate-primitive.md (English canonical)
notes_for_editor: |
  Spanish overview per DOCTRINE.md §XII strategic-adaptation pattern: NOT a
  1:1 translation. Captures the essential framing for a Spanish-reading
  contributor who needs to know what the English TOPIC is about and decide
  whether to read it in full.

  Skeleton stage: section headings + draft-pending markers, mirroring the
  English structure. Substantive Spanish overview follows when project-
  language refines the English canonical bulk in milestone N+1.
---

# Pruebas de Merkle como Primitiva de Sustrato

*(borrador-pendiente — la sustancia sigue en el hito N+1)*

## Resumen

*(borrador-pendiente — la sustancia sigue en el hito N+1)*

El cluster project-system entregó las pruebas de inclusión RFC 9162
§2.1.3 (Fase 1A.4) y las pruebas de consistencia §2.1.4 (Fase 1A.5)
como primitivas a nivel de sustrato. Junto con el formato de
checkpoint firmado C2SP, forman el piso criptográfico del Sustrato
del Libro de Capacidades (afirmación doctrinal #33).

## Secciones del TOPIC en inglés

1. Qué son las pruebas de Merkle
2. Dos sabores: inclusión y consistencia
3. Pruebas de inclusión en `system-core`
4. Pruebas de consistencia en `system-core`
5. Primitivas compuestas sobre `SignedCheckpoint`
6. Integración del consumidor en `system-ledger`
7. Por qué esto importa como primitiva de sustrato
8. Referencias cruzadas

*(El TOPIC canónico en inglés está en `topic-merkle-proofs-as-substrate-
primitive.md`. Esta versión en español es un panorama, no una
traducción palabra-por-palabra, según DOCTRINE.md §XII.)*
