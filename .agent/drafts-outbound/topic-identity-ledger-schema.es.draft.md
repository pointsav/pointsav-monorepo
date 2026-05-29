---
schema: foundry-draft-v1
state: skeleton-pending-language-pass
originating_cluster: project-data
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-identity-ledger-schema.es.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
created: 2026-05-29
canonical_english: topic-identity-ledger-schema.draft.md
notes_for_editor: >
  Esqueleto en español conforme a la disciplina bilingüe del Tetrad. El
  contenido sustantivo está en el canónico inglés (topic-identity-ledger-schema.draft.md).
  La pasada de lenguaje debe producir un resumen orientado en español propio —
  no una traducción literal del inglés. Decisiones terminológicas para ratificar:
  (1) "ledger de identidades" vs. "registro de identidades"; (2) "primitiva de
  identidad" (calco directo, aceptable); (3) "ancla" para Anchor (sustantivo
  técnico — verificar que no colisione con terminología interna); (4) "demanda"
  o "afirmación" para Claim (ambas tienen matices distintos en contexto jurídico
  vs. técnico — ratificar); (5) "puntuación de confianza" para confidence_score
  (calco descriptivo, probable aceptación); (6) "inquilino" para tenant
  (alternativas: "cliente", "dominio").
---

# Tema: Esquema del Registro de Identidades

*(Esqueleto — pasada de lenguaje pendiente)*

El registro de identidades es la primitiva de identidad determinista en el
centro de la ingestión de datos en el Anillo 1. Define tres tipos de registros
— Persona, Ancla y Afirmación — que en conjunto representan quién es conocido
por el sistema, cómo fue observada esa identidad y qué atributos se han
registrado sobre ella.

---

## 1. La Primitiva de Identidad

*(Ver canónico inglés §1 — derivación UUIDv5 sobre correo electrónico en
minúsculas; propiedad de determinismo y composibilidad.)*

---

## 2. El Registro de Persona

*(Ver canónico inglés §2 — estructura Person, campos, invariantes de
construcción.)*

---

## 3. El Registro de Ancla

*(Ver canónico inglés §3 — estructura Anchor, semántica de observación
inmutable.)*

---

## 4. El Registro de Afirmación

*(Ver canónico inglés §4 — estructura Claim, claim_id UUIDv4, puntuación de
confianza, procedencia.)*

---

## 5. Detección de Conflictos

*(Ver canónico inglés §5 — ConflictingIdentity, protocolo F12 / SYS-ADR-10.)*

---

## 6. Cumplimiento de SYS-ADR-07: Cero IA en el Anillo 1

*(Ver canónico inglés §6 — extracción por expresión regular, derivación UUIDv5,
ausencia de pesos de modelo, sin llamadas de inferencia.)*

---

## 7. Persistencia WORM

*(Ver canónico inglés §7 — ruta de escritura FsClient, cabecera
X-Foundry-Module-ID, garantía append-only.)*

---

## 8. Perspectiva: Extensión de Identidad en el Anillo 2

*(Ver canónico inglés §8 — compartición de identidades entre inquilinos,
coincidencia difusa basada en vectores — planned/intended; no presente en la
implementación actual.)*

---
