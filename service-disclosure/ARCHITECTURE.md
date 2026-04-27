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

## Public surface — Phase 1A

```text
service_disclosure::
    Family               (enum, 4 variants)
    GenreTemplate        (enum, 18 variants)
    ProtocolRequest      (struct, 6 fields)
    Frontmatter          (struct, 8 fields)
    Register             (enum, 5 variants)
    ValidationError      (enum)
    validate_frontmatter (fn)
    BANNED_VOCABULARY    (&[&str], 8 entries)
    VERSION              (&str, from CARGO_PKG_VERSION)
```

Anything not listed above is implementation detail and may move
between modules across PATCH versions.

## Module layout

`src/lib.rs` re-exports the public surface and holds workspace-wide
constants (`BANNED_VOCABULARY`, `VERSION`). Per-concern modules:

- `genre.rs` — Family, GenreTemplate, partition test
- `request.rs` — ProtocolRequest, Register, round-trip tests
- `frontmatter.rs` — Frontmatter, optional-field elision tests
- `validate.rs` — validate_frontmatter, per-genre rule tests

The split lets a future Phase 1B add `cfg.rs` without touching the
type modules, and Phase 1C add `templates.rs` (or a `templates/`
subdirectory of compiled-in `.toml` fragments) without touching
either.

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

## Phase 1C — genre-template registry

Each `GenreTemplate` variant pairs with one `.toml` describing the
template parameters and one `.md` describing the genre to humans.
Both ship under `service-disclosure/templates/`. The crate exposes
`fn get_template(template: GenreTemplate) -> &'static str`
returning the rendered template fragment ready for Doorman
composition.

The TOML shape is anchored in convention §2.2 — required sections,
register parameters, bilingual-pair convention, frontmatter schema.

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
