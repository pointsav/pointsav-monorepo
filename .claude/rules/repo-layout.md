# Repo Layout — pointsav-monorepo

Canonical rule for what files belong at the monorepo root and at each
project-directory root. Read at session start. Update only when the
rule itself changes; drift is closed by moving files, not by loosening
the rule.

Last updated: 2026-04-23.

---

## Monorepo root — allowed files

Only the following files may appear at the monorepo root. Any other
file at root is a defect and must be moved to its proper home.

| File | Purpose |
|---|---|
| `Cargo.toml` | Workspace manifest |
| `Cargo.lock` | Lockfile |
| `.gitignore` | Required |
| `.gitattributes` | Optional; only if needed |
| `LICENSE` | Repo licence |
| `README.md` | English README (bilingual pair per CLAUDE.md §6) |
| `README.es.md` | Spanish README (bilingual pair per CLAUDE.md §6) |
| `NEXT.md` | Repo-scope open items |

**Not allowed at root:** shell scripts, Python scripts, ad-hoc build
helpers, user guides, operations guides, one-off memos, zip/tar/iso
artefacts, loose data files.

Where such items belong instead:

- Scripts → inside the project directory they operate on, under `scripts/`
- User-facing documentation → `content-wiki-documentation/` (sibling repo)
- Design-system / UI material → `pointsav-design-system/` (sibling repo)
- Architecture topics / ADRs → `content-wiki-documentation/`
- Large binaries → build-time fetch; never checked in

---

## Project directory root — allowed files

Inside each top-level project directory (e.g. `service-slm/`,
`app-console-people/`), only the following files may appear at the
project root.

| File | When present |
|---|---|
| `Cargo.toml` | Rust crate |
| `README.md` | Required |
| `README.es.md` | Required (bilingual, CLAUDE.md §6) |
| `CHANGELOG.md` | When meaningful |
| `CLAUDE.md` | Active-state projects (framework §8) |
| `NEXT.md` | Active-state projects (framework §8) |
| `ARCHITECTURE.md` | When a project warrants a dedicated architecture note |
| `DEVELOPMENT.md` | When a project warrants build/run instructions |
| `SECURITY.md` | When a project warrants a security posture note |

Anything else at project root — scripts, stray markdown, data files —
is a defect. Move to a subfolder or delete.

## Project directory subfolders — conventional names

| Subfolder | Purpose |
|---|---|
| `src/` | Source code |
| `scripts/` | Shell / Python helpers targeting this project |
| `tools/` | Reusable utilities scoped to this project |
| `docs/` | Detailed documentation |
| `tests/` | Tests |
| `build/` | Build output (gitignored) |

`engine/`, `relay/`, or other names are acceptable when they carry
domain meaning (e.g. `os-console/engine/`, `os-console/relay/`). The
table is a floor, not a ceiling.

---

## Sibling repos — where cross-cutting content lives

| Repo | What belongs there |
|---|---|
| `content-wiki-documentation` | User guides, operations guides, architecture topics, ADRs, glossaries, research memos |
| `content-wiki-corporate` | Corporate / business-side documentation |
| `content-wiki-projects` | Per-project narrative references |
| `pointsav-design-system` | UI components, templates, themes, tokens, design-system guidelines |

When a file fits one of these sibling-repo categories, it moves to
that sibling rather than accumulating in the monorepo.

---

## How defects are closed

1. Identify the file that violates this rule.
2. Determine its correct home (project subfolder or sibling repo).
3. Move with `git mv` for intra-repo moves; for cross-repo moves, add
   in the destination and remove in the source with one commit each.
4. Update any callers that reference the old path.
5. Log the change in `.claude/rules/cleanup-log.md`.

Do not loosen this rule to accommodate a misplaced file — move the file.
