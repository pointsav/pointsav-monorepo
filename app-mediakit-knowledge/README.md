# app-mediakit-knowledge

Wikipedia-pattern HTTP knowledge wiki for `os-mediakit`. Serves the
`content-wiki-documentation` repository as a fully navigable wiki at
`documentation.pointsav.com`. Built in Rust. No database. No runtime
dependencies beyond the compiled binary.

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

The wiki engine for the PointSav knowledge platform. A single Rust
binary that serves a directory of CommonMark-with-wikilinks files
as a Wikipedia-shaped read-and-edit surface.

## Status

Phase 1 — render. The engine reads a content directory, parses
Markdown with the comrak wikilinks extension, and serves rendered
pages over HTTP on a loopback address.

See [`ARCHITECTURE.md`](./ARCHITECTURE.md) for the build-phase plan
through Phase 8.

## Design principle

**Markdown files in a Git tree are the source of truth.** Every
database, index, and cache is derived state, rebuildable from
`git checkout && reindex`. Page identity is a path; revision
history is `git log`; metadata is YAML frontmatter; multi-content
slots are sibling files. There is no schema migration ladder
because there is no canonical schema in the database — the
database is a regenerable index of the file tree.

## Run (Phase 1)

```
cargo run -- serve --content-dir tests/fixtures/content
```

The server binds `127.0.0.1:9090` by default. Override with
`--bind` or `WIKI_BIND`.

## Build phases (planned)

| Phase | Adds | Status |
|---|---|---|
| 1 | render one TOPIC | in flight |
| 2 | edit endpoint | planned |
| 3 | search (tantivy in-process) | planned |
| 4 | git sync, history, blame, diff | planned |
| 5 | auth (local + OIDC) | planned |
| 6 | wikilink resolution + backlinks | planned |
| 7 | federation seam (blake3 content addressing) | planned |
| 8 | disclosure mode (iXBRL, OpenTimestamps, MediaWiki XML import) | planned |

Phase 8 is the planned moat — the combination of Markdown-native
authorship, structured-data extraction for regulator-required
financial-statement blocks, cryptographic timestamping, and
per-jurisdiction export adapters. Forward-looking; subject to
material assumptions and operator decisions.

## Cluster context

This crate is part of the `project-knowledge` cluster
(per `~/Foundry/PROJECT-CLONES.md`), alongside:

- [`content-wiki-documentation`](../../../content-wiki-documentation/) —
  TOPIC content the engine renders
- [`pointsav-fleet-deployment/media-knowledge-documentation/`](../../../pointsav-fleet-deployment/) —
  catalog runbooks for deployment

## Conventions

- Single-binary deployment, systemd unit, no Docker
  (per `conventions/zero-container-runtime.md`)
- Bilingual READMEs (per workspace `CLAUDE.md` §6)
- Citation discipline (per `conventions/citation-substrate.md`)
- BCSC continuous-disclosure posture for content
  (per `conventions/bcsc-disclosure-posture.md`)

## Licence

Apache-2.0.
