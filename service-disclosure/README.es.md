# service-disclosure (resumen)

Sustrato de esquema y gramática libre de contexto (CFG) para el
trabajo editorial en Foundry. Define la taxonomía de adaptadores en
cuatro familias (PROSE / COMMS / LEGAL / TRANSLATE), la enumeración
de plantillas por género, el tipo de portada documental
(*frontmatter*) y la lista de vocabulario prohibido común a todos
los géneros.

Convención operativa de referencia:
`~/Foundry/conventions/language-protocol-substrate.md`.

## Qué expone este crate

| Elemento | Propósito |
|---|---|
| `Family` | Las cuatro familias de adaptadores. |
| `GenreTemplate` | Las dieciocho plantillas de género actualmente en uso. |
| `ProtocolRequest` | Forma de petición que consumen `service-slm` y `service-proofreader`. |
| `Frontmatter` | Tipo de portada documental según la disciplina de citación (CLAUDE.md §16). |
| `Register` | Selector de registro editorial (Bloomberg / Operacional / Técnico / Coloquial / Legal). |
| `validate_frontmatter` | Validador de paso único que devuelve todos los errores detectados. |
| `BANNED_VOCABULARY` | Lista de términos prohibidos compartida entre géneros. |

## Estado

Solo Fase 1A: tipos y validadores. La Fase 1B (exportación CFG con
`llguidance` u Outlines) y la Fase 1C (registro de plantillas con
fragmentos `.toml` + `.md`) están en cola en `NEXT.md`.

## Licencia

Apache-2.0. Hereda el archivo `LICENSE` raíz del monorepo.

[Documento canónico en inglés: `README.md`.]
