---
schema: foundry-draft-v1
state: skeleton-pending-language-pass
originating_cluster: project-data
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-adr-07-zero-ai-in-ring-1.es.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
created: 2026-05-29
canonical_english: topic-adr-07-zero-ai-in-ring-1.draft.md
notes_for_editor: >
  Esqueleto en español conforme a la disciplina bilingüe del Tetrad. El
  contenido sustantivo está en el canónico inglés
  (topic-adr-07-zero-ai-in-ring-1.draft.md). La pasada de lenguaje debe
  producir un resumen orientado en español propio — no una traducción literal.
  Decisiones terminológicas para ratificar: (1) "Anillo 1/2/3" vs. "Ring 1/2/3"
  (se recomienda mantener "Ring" como término técnico no traducido, igual que
  MCP o SHA-256, pero la decisión es de project-language); (2) "ingestión de
  límite" para "boundary ingest" — posiblemente "ingestión en la frontera";
  (3) "cero IA" para "zero AI" (calco directo, probable aceptación);
  (4) "pista de auditoría" para "audit trail"; (5) "determinista" vs.
  "determinístico" (ambas formas son válidas en español técnico — ratificar
  una).
---

# Tema: SYS-ADR-07 — Cero IA en el Anillo 1

*(Esqueleto — pasada de lenguaje pendiente)*

SYS-ADR-07 es una decisión arquitectónica que prohíbe la inferencia de IA en
los servicios de ingestión del Anillo 1. No es una preferencia ni una
directriz — es una restricción estricta que los cuatro servicios del Anillo 1
implementan en código, sin excepciones.

---

## 1. Qué es el Anillo 1

*(Ver canónico inglés §1 — cuatro servicios de ingestión: service-fs,
service-people, service-email, service-input; rol de muro exterior del plano
de datos; permanencia de los registros.)*

---

## 2. Por qué solo operaciones deterministas

*(Ver canónico inglés §2 — tres razones: pista de auditoría verificable sin
estado de modelo; garantía de composabilidad para Anillos 2 y 3; postura
regulatoria.)*

---

## 3. Qué significa "Cero IA" en la práctica

*(Ver canónico inglés §3 — qué está prohibido (llamadas LLM, vectores de
incrustación, pesos de modelo) vs. qué está permitido (expresiones regulares,
SHA-256, UUIDv5, Ed25519, parseo estructural).)*

---

## 4. Instancias de implementación

*(Ver canónico inglés §4 — service-fs (criptografía pura), service-people
(regex + UUIDv5 + detección de conflictos), service-email (parseo SOAP XML),
service-input (parseo estructural de documentos).)*

---

## 5. La Frontera del Anillo 2

*(Ver canónico inglés §5 — la inteligencia entra en el Anillo 2;
service-extraction lee desde el Anillo 1 vía MCP; no existe ruta de
escritura de vuelta al Anillo 1 desde inferencia del Anillo 2.)*

---

## 6. SYS-ADR-10 (F12): Exponer Ambigüedad al Operador

*(Ver canónico inglés §6 — conflictos de identidad superficiados como error,
no resueltos por modelo; protocolo F12 / SYS-ADR-10.)*

---

## 7. Anillo 3: IA Compositiva

*(Ver canónico inglés §7 — Ring 3 (service-slm) es intended/planned como capa
de inteligencia opcional; no es una excepción al Anillo 1; consulta datos del
Anillo 2 que a su vez lee del Anillo 1 vía la misma interfaz MCP determinista.)*

---
