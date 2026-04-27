# service-disclosure

Schema and CFG substrate for Foundry editorial work. Defines the
4-family adapter taxonomy (PROSE / COMMS / LEGAL / TRANSLATE), the
genre-template enumeration, the document-frontmatter type, and the
banned-vocabulary list shared across genres.

Operational anchor: `~/Foundry/conventions/language-protocol-substrate.md`.

## What this crate exports

| Item | Purpose |
|---|---|
| `Family` | The four adapter families. |
| `GenreTemplate` | The eighteen genre templates currently in scope. |
| `ProtocolRequest` | Request shape consumed by `service-slm` and `service-proofreader`. |
| `Frontmatter` | Document-frontmatter type per CLAUDE.md §16 citation discipline. |
| `Register` | Editorial register selector (Bloomberg / Operational / Technical / Casual / Legal). |
| `validate_frontmatter` | One-pass validator returning every detected error. |
| `BANNED_VOCABULARY` | Cross-genre prohibited word list. |

## Status

Phase 1A only — types and validators. Phase 1B (`llguidance` or
Outlines CFG export) and Phase 1C (genre-template registry with
`.toml` + `.md` fragments) are queued in `NEXT.md`.

## Consumers

`service-proofreader` (project-proofreader cluster) consumes this
crate via Cargo dependency. Until `service-disclosure` ships a
schema-stable signal, the consumer operates with hardcoded
protocol templates.

## Build and test

```sh
cargo check -p service-disclosure
cargo test -p service-disclosure
```

## License

Apache-2.0. Inherits the monorepo `LICENSE` file at the repository
root.

## See also

- `~/Foundry/conventions/language-protocol-substrate.md`
- `ARCHITECTURE.md` (this directory)
- `~/Foundry/DOCTRINE.md` claims #21, #22, #25, #31, #32

[Spanish overview: `README.es.md`.]
