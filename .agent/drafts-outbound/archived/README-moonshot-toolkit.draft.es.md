---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: pointsav-monorepo
target_path: moonshot-toolkit/
target_filename: README.es.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: TRANSLATE-ES
authored: 2026-04-28T00:00:00Z
authored_by: task-project-system (session 181c94d9ca0491c5, ps-administrator identity)
authored_with: sonnet-4-6
references:
  - clones/project-system/.claude/drafts-outbound/README-moonshot-toolkit.draft.md
  - DOCTRINE.md §XII (strategic-adaptation pattern for Spanish overviews)
notes_for_editor: |
  Spanish overview per DOCTRINE.md §XII strategic-adaptation pattern.
  NOT a 1:1 translation. The canonical technical depth lives in README.md
  (English). This file is strategic overview + section headings translated
  + closing pointer.

  Section headings translated literally where clean; "protection domain"
  and "BuildPlan" retained in English as technical proper names throughout
  the platform (consistent with prior Spanish READMEs in this codebase).
  "Rust-Only Toolchain" retained in English as a MEMO §7 named mandate.
---

<div align="center">

# moonshot-toolkit

[ Read in English ](./README.md)

</div>

---

## Resumen

`moonshot-toolkit` es el orquestador de construcción exclusivo en Rust
para las imágenes de microkernel seL4 de Foundry. Reemplaza la cadena
de herramientas Microkit (Python + CMake) según el mandato MEMO §7
("Microkit (Python/CMake) → moonshot-toolkit (Rust-Only Toolchain)").

El flujo de trabajo es el siguiente: el crate lee un `system-spec.toml`
que declara los dominios de protección, canales entre ellos, regiones de
memoria física y asignaciones de interrupciones de hardware; valida todos
los invariantes en tiempo de lectura; y genera un `BuildPlan` determinista
con direccionamiento por contenido. El `plan_hash` del plan — SHA-256 del
manifiesto completo — es el artefacto que la cosignatura del apex del
cliente compromete, habilitando la verificación reproducible sobre el metal
del cliente según `conventions/system-substrate-doctrine.md` §6.

Sin `moonshot-toolkit`, ningún otro proyecto `moonshot-*` puede producir
un artefacto ejecutable. Es la pieza fundacional de la Fase 1B.

**Estado:** Versión 0.1.3 — alcance v0.1.x de la Fase 1B cerrado. El
subcomando `build` es un stub intencional en v0.1.x; la compilación cruzada
real para seL4 + arranque en QEMU AArch64 es trabajo futuro (tarea #14).

---

## Secciones — ver documento canónico en inglés

La documentación técnica detallada vive en el README canónico en inglés
([README.md](./README.md)) por convención bilingüe del espacio de trabajo
(`~/Foundry/CLAUDE.md` §6: el español es panorámica estratégica, no
traducción literal).

| Sección | Contenido |
|---|---|
| I. Qué es | Propósito del crate y posición en la familia `moonshot-*` |
| II. Qué hace | Parser de SystemSpec, generador de BuildPlan, subcomandos CLI |
| III. Estado | v0.1.3, alcance Fase 1B cerrado, conteo de pruebas |
| IV. Formato de SystemSpec | Esquema TOML, reglas de validación, ejemplo mínimo |
| V. Formato de BuildPlan | Estructura JSON, `spec_hash`, `plan_hash`, pasos |
| VI. Compilación y pruebas | Comandos `cargo build`, `cargo test`, ejecución de subcomandos |
| VII. Qué está diferido | Tarea #14: compilación cruzada seL4, estrategia de toolchain |
| VIII. Restricciones rígidas | Rust-Only Toolchain, generación determinista, sin red en `build` |
| IX. Referencias cruzadas | DOCTRINE.md claims #33 y #34, MEMO §7, system-substrate-doctrine §6 |
| X. Licencia | Hereda la licencia del repositorio raíz |

---

## Anclas constitucionales

Las afirmaciones #33 y #34 de `DOCTRINE.md` (The Capability Ledger
Substrate y The Two-Bottoms Sovereign Substrate) son el fundamento
constitucional de este crate. La especificación operativa es
`conventions/system-substrate-doctrine.md` §6:
Verificación-Reproducible-Sobre-Metal-del-Cliente.
