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

## Phase 1B — banned-vocabulary CFG (shipped 2026-04-27)

Phase 1B exports `BANNED_VOCABULARY` as a Lark EBNF grammar
consumed by `llguidance` at decode time. Library and dialect
chosen by `project-slm` Task in the AS-2 cross-cluster relay
(`llguidance`, Microsoft Research Rust crate, Lark EBNF;
vLLM Multi-LoRA at Tier B accepts it natively).

The grammar lives with the data substrate, not this crate:

```
pointsav-monorepo/service-content/schemas/
├── banned-vocab.lark        # Lark EBNF; top-level rule `response`
├── README.md                # usage docs + escape rule + cross-refs
├── test-prose-pass.txt      # synthetic clean prose; parses cleanly
├── test-prose-fail.txt      # contains every banned word; rejected
└── validate.py              # dual-mode validator (Lark + regex fallback)
```

Path scope: the grammar is shared **data** across all tenants
under `moduleId` namespacing — not per-tenant code — so it lives
with `service-content` (the data substrate), not with
`service-disclosure` (the schema crate). `service-disclosure`'s
`BANNED_VOCABULARY` constant remains the authoritative list; the
`.lark` grammar mirrors it. Drift between the two is a defect
closed by editing both in lockstep.

The escape rule (backtick-quoted citations may contain banned
words) implements the `attempt.banned_vocabulary_hits` invariant
declared in `CORPUS-SCHEMA.md` §5.

The validation harness runs in two modes — Lark when the Python
`lark` package is installed on the host, regex-fallback otherwise.
The fallback is conceptually equivalent for the banned-vocab use
case because Lark itself uses Python's `re` module for terminal
matching. Production use on Tier A and Tier B requires the full
Lark grammar loaded by `llguidance` at inference time.

Schema-stable ratification: when Master ratifies in the next
Master pass, `service-disclosure` jumps v0.2.1 → v0.3.0 (semver
MINOR for the public contract addition); the `project-proofreader`
Task receives the Cargo dep upgrade procedure in the same
relay.

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
