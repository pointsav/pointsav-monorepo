# ARCHITECTURE.md — service-disclosure

The crate boundary, the consumer contract, and the shape Phases 1B
and 1C will add. Read alongside
`~/Foundry/conventions/language-protocol-substrate.md`.

## Position in the editorial-write path

Three services compose the editorial-write path. Each one owns a
distinct shape:

| Service | Shape | Owner cluster |
|---|---|---|
| `service-content` | Data — taxonomy ledger + knowledge graph | project-slm |
| `service-slm` | Inference — Doorman + tier routing + audit ledger | project-slm |
| `service-disclosure` | Schema — types, validators, CFG, templates | project-language |
| `service-proofreader` | Operational — request-shaped HTTP write-assistant | project-proofreader |

`service-disclosure` is library-shaped on purpose. It opens no
sockets, reads no files, and depends on nothing in the workspace
besides `serde` and `serde_yaml`. The Doorman (`service-slm`)
imports it to compose prompts at request time;
`service-proofreader` imports it to validate inbound requests and
outbound diffs.

## Public surface

Phase 1A (v0.1.0) and Phase 1C (v0.2.0) shipped:

```text
service_disclosure::
    Family                       (enum, 4 variants)
    GenreTemplate                (enum, 18 variants)
    ProtocolRequest              (struct, 6 fields)
    Frontmatter                  (struct, 8 fields)
    Register                     (enum, 5 variants)
    ValidationError              (enum)
    validate_frontmatter         (fn)
    get_template                 (fn) -> &'static str  (TOML)
    get_template_description     (fn) -> &'static str  (Markdown)
    BANNED_VOCABULARY            (&[&str], 8 entries)
    VERSION                      (&str, from CARGO_PKG_VERSION)
    templates::ALL               (&[GenreTemplate], 18 entries)
```

Anything not listed above is implementation detail and may move
between modules across PATCH versions.

## Module layout

`src/lib.rs` re-exports the public surface and holds workspace-wide
constants (`BANNED_VOCABULARY`, `VERSION`). Per-concern modules:

- `genre.rs` — Family, GenreTemplate, partition test
- `request.rs` — ProtocolRequest, Register, round-trip tests
- `frontmatter.rs` — Frontmatter, optional-field elision tests
- `templates.rs` — get_template, get_template_description, ALL,
  registry-completeness and well-formedness tests
- `validate.rs` — validate_frontmatter, per-genre rule tests

The split lets a future Phase 1B add `cfg.rs` without touching the
existing modules.

## Phase 1B — banned-vocabulary CFG

Phase 1B will export `BANNED_VOCABULARY` as a CFG fragment in either
`llguidance` or Outlines format, chosen to match the decode-time
constraint library that `service-slm` already integrates with in
the project-slm cluster's AS-2 implementation. The CFG must:

- Allow all reasonable English prose except the banned tokens in
  standalone form
- Permit banned tokens inside backtick-quoted citations or
  prior-art examples (escape rule: backtick-quoted is fine,
  bare-word is refused at decode time)
- Be regenerated deterministically from `BANNED_VOCABULARY` so a
  future addition to the list propagates without manual CFG edits

The CFG schema and the regeneration entry-point land in this file
when Phase 1B ships.

## Phase 1C — genre-template registry (shipped v0.2.0)

Each `GenreTemplate` variant pairs with one `.toml` and one `.md`
under `service-disclosure/templates/`. Both files are baked into
the binary at compile time via `include_str!`; no runtime
filesystem access. `get_template` returns the TOML; the Doorman
parses it at request time using the `toml` crate.

TOML shape (anchored in convention §2.2):

| Field | Required | Notes |
|---|---|---|
| `name` | yes | Matches the kebab-case serde form of the variant. |
| `family` | yes | One of `prose` / `comms` / `legal` / `translate`. |
| `prompt_scaffolding` | yes | The template fragment the Doorman composes into the request prompt. |
| `required_sections` | optional | Array of section headings the genre demands. |
| `bilingual_pair_required` | optional | True for public-facing genres. |
| `target_reading_level` | optional | Style budget. |
| `mean_sentence_length` | optional | Style budget. |
| `max_sentence_length` | optional | Style budget. |
| `banned_vocabulary_inherits` | optional | Defaults to `@global`. |
| `default_register` | optional | Bloomberg / operational / technical / casual / legal. |
| `tenant_required` | optional | Genre needs a tenant slug to route per-tenant adapters. |
| `default_routing` | optional | LEGAL family carries `tier-c` to mean Doorman routes to external API by default. |
| `frontmatter_required` | optional | Document genre carries YAML frontmatter (TOPIC, MEMO). |
| `max_lines` | optional | Short-form length cap (e.g., chat). |
| `plain_text_first_30_lines` | optional | README convention — readable in nano/vim/less. |
| `ascii_compatible` | optional | Default `true`. |

The `name` and `family` fields are validated by `templates.rs`
unit tests: `name` must match the variant's kebab-case serde form,
and `family` must match the genre partition declared by
`GenreTemplate::family()`. New variants therefore land with their
template files in the same commit.

## Apprenticeship corpus schema

Editorial-apprenticeship corpus tuples (Phase 3 of the cluster
brief) carry the same wire shape as code-apprenticeship tuples per
`~/Foundry/conventions/apprenticeship-substrate.md`. The full JSONL
schema lands in this file (or a sibling `CORPUS-SCHEMA.md`) once
Phase 3 of the cluster brief is picked up.

## What this crate is not

- Not a prompt-runner. Prompt composition runs in the Doorman.
- Not a network surface. There is no HTTP, no IPC.
- Not a citation resolver. Citations resolve against
  `~/Foundry/citations.yaml`; this crate carries the IDs in
  `Frontmatter.cites` but does not load the registry.
- Not a per-tenant adapter loader. Adapter loading is the
  Doorman's responsibility.
