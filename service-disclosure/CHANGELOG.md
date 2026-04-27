# CHANGELOG — service-disclosure

Newest on top. PATCH per accepted commit; MINOR per feature; MAJOR
per breaking change. Per `~/Foundry/CLAUDE.md` §7.

## v0.3.0 — 2026-04-27 (schema-stable)

- **Schema-stable contract ratified at workspace v0.1.26.** Master's
  ratification commit names the public surface as locked: `Family`,
  `GenreTemplate` (18 variants), `ProtocolRequest`, `Frontmatter`,
  `Register`, `validate_frontmatter`, `BANNED_VOCABULARY`,
  `templates::ALL`, `get_template`, `get_template_description`.
- **Phase 1B banned-vocab grammar shipped** at
  `pointsav-monorepo/service-content/schemas/banned-vocab.lark`
  (Lark EBNF + `llguidance` decode-time enforcement). The grammar's
  artefact path is now part of the contract that consumers may rely
  on; the `.lark` lives with the data substrate (multi-tenant
  shared) rather than in this crate (per-tenant code).
- **Validation confirmed end-to-end.** Master installed
  `python3-lark` 1.3.1 on the workspace VM and ran
  `service-content/schemas/validate.py` in full Lark mode:
  pass-fixture parses cleanly; fail-fixture rejected with
  `UnexpectedCharacters`. Production-grade per spec.
- **`project-proofreader` Task may upgrade Cargo dep on this version.**
  Their hardcoded protocol templates in `service-proofreader/src/templates/`
  upgrade to consume the published crate at their convenience.
- No code change in this crate this commit; the `.lark` grammar
  shipped in commit `374d192` and is the load-bearing artefact for
  the v0.3.0 ratification.

## v0.2.1 — 2026-04-27

- Added `CORPUS-SCHEMA.md` documenting the JSONL tuple shape for
  the editorial-apprenticeship corpus. Specialises the substrate
  convention's code-shaped tuple to the eight editorial task-types
  declared by the cluster manifest, adding `language_protocol` and
  `banned_vocabulary_hits` fields without forking the record schema.
  Includes the closed `doctrine_violation_tag` set for DPO triples
  on `refine` / `reject` verdicts. Documents the migration from the
  v0.1 stub (`-v1-stub` schemas remain valid until schema-stable
  signal lands).
- No code change — PATCH bump per `~/Foundry/CLAUDE.md` §7.

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
