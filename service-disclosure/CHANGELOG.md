# CHANGELOG — service-disclosure

Newest on top. PATCH per accepted commit; MINOR per feature; MAJOR
per breaking change. Per `~/Foundry/CLAUDE.md` §7.

## v0.2.0 — 2026-04-27

- Phase 1C — genre-template registry. Eighteen `.toml` + `.md`
  pairs under `templates/`, one per `GenreTemplate` variant. New
  public functions `get_template` and `get_template_description`
  return `&'static str` via `include_str!`. New
  `templates::ALL` constant lists every variant in declaration
  order. Seven new templates-module tests verify completeness,
  TOML well-formedness, required-field presence, and
  family/name consistency with the genre partition. `toml = "0.8"`
  added to dev-dependencies. 26 unit tests pass overall.

## v0.1.0 — 2026-04-27

- Phase 1A scaffold — 4-family adapter taxonomy (Prose, Comms,
  Legal, Translate), 18 genre templates, `ProtocolRequest`,
  `Frontmatter`, `Register`, `validate_frontmatter`, and the
  cross-genre `BANNED_VOCABULARY` list. 19 unit tests pass.
  Declared as the ninth member of `pointsav-monorepo` workspace.
  Registered as Active in `.claude/rules/project-registry.md`.
