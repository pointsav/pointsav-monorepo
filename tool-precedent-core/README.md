# tool-precedent-core

The generic flat-file precedent shard. Zero dependencies. It holds what every `tool-precedent-*` needs, so each tool gets every addition: change log (`precedent/changes.csv`), version lineage (`precedent/versions.csv`), terms (`precedent/terms.csv`), Draw Register (`precedent/draws.csv`), and hash-fenced tokens (FNV-1a 64). No master register.

Staged in project-documents (built and tested 2026-10-23); the intended home is project-precedent, which takes it over. Document-specific code (`.psdoc`, the Annex, `stamp`) stays in tool-precedent-documents; chart-specific code stays in tool-precedent-orgcharts.

Commands (all take `--root DIR`, default `.`, and `--file` to name another CSV): `changes-add`, `changes`, `versions [--dir outputs] [--out F]`, `terms`, `terms-add`, `draws`, `draw-add`, `apply <file.html> [--check]`, `catalog`, `registry <check|classes|css|dtcg|lint>`, `shared-hash`.

Sibling: `tool-forecasting-core` (project-proforma) carries the same six modules (changes, csv, draws, fence, terms, versions), copied. `shared-hash` prints the FNV-1a 64 of each; the two lists must match line for line. When a shared module changes, copy it to the other core and compare.

Status: tool-precedent-documents and tool-precedent-orgcharts still carry their own copies of these modules. Moving them onto this core is a follow-up for project-precedent.

## registry module (2026-11-06)

`src/registry.rs` is the unified typesetting registry of project-documents (operator decisions P-34..P-51) in Rust. It reads the eight data tables `precedent/registry/ts-*.csv` (classes, geometry, labels, glyphs, sections, families, engine, changes), zero dependencies, read-only (it never writes into a template or a document):

- `registry check [--root DIR] [--registry DIR]` referential integrity: statuses from closed sets; every geometry class, alias, label and glyph link resolves in the closed class list; family names exist; `components.csv` files exist and their FNV-1a hash (over the trimmed file, as `tool-precedent-documents` tokens) matches; a generated own-file component (ts-engine key `css.own-file-components`, today `ca-classes`) still contains the block the table generates and its evidence line numbers point at the right rules. Exit 1 on any error.
- `registry classes [--family F]` the closed class list (tab-separated: class, element, kind, status, aliases, families).
- `registry css --component C | --out DIR` the generated geometry block / the generated component CSS files; byte-identical to `bin/script-typeset-reference.py css`.
- `registry dtcg [--family F] [--out FILE]` design-token JSON (W3C DTCG draft: dimension / typography / fontWeight tokens, pt and Word twips in `$extensions`); byte-identical to `script-typeset-reference.py dtcg`.
- `registry lint FILE... [--family F] [--legacy] [--strict] [--all-lines]` the class rules of `bin/script-typeset-lint.py` on authored HTML (unknown-class, retired-class including classes retired for one family by a `ts-changes.csv` row, inline-style, alias-use, strong-def-term, label-space, family-scope, proposed-class). Counts and line numbers equal the Python lint on all 745 HTML files of project-documents (2026-11-06). Python-only for now: label-seq, label-form, circle-outside-mark, bullet-outside-label, folio-dup, h-jump. `tool-precedent-documents lint-html` is the same command.

`registry.rs` is NOT one of the six shared modules (it is not copied to tool-forecasting-core; `shared-hash` still lists six and is unchanged). tool-precedent-documents uses it through its path dependency on this crate; it does not carry a copy.

Tests: `tests/fixtures/registry` (a small real subset of the registry, a generated own-file component, and goldens made by the Python scripts: css per component, dtcg all / canada / letter) and `tests/registry_parity.rs` (live parity against the real project-documents registry and Python scripts when present; skips elsewhere). The workspace build (`cargo build -p tool-precedent-core` from the archive root) is the one that resolves offline-free.
