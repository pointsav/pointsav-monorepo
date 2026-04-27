# CLAUDE.md — service-disclosure

> **State:** Active  —  **Last updated:** 2026-04-27
> **Version:** 0.1.0  (per `~/Foundry/CLAUDE.md` §7 and DOCTRINE.md §VIII)
> **Registry row:** `pointsav-monorepo/.claude/rules/project-registry.md`
>
> When state changes, update this header AND the registry row in the
> same commit. Drift between the two is a documentation defect.

---

## What this project is

Rust library crate that holds the schema substrate for Foundry
editorial work — the 4-family adapter taxonomy, the genre-template
enumeration, the request and frontmatter types, and the validator
that enforces per-genre rules. Sibling to `service-content` (data
substrate) and `service-slm` (inference substrate); the three
together compose the editorial-write path.

Operational anchor: `~/Foundry/conventions/language-protocol-substrate.md`.

## Current state

Phase 1A landed in v0.1.0:

- `Family` enum with four variants (Prose, Comms, Legal, Translate)
- `GenreTemplate` enum with eighteen variants partitioned across
  Prose (10), Comms (4), and Legal (4)
- `ProtocolRequest` and `Frontmatter` structs with full serde
  round-trip coverage
- `Register` enum with five variants
- `validate_frontmatter` with universal and per-genre rules,
  returning every error in one pass
- `BANNED_VOCABULARY` constant with eight cross-genre prohibited
  terms

Nineteen unit tests pass. `cargo check -p service-disclosure` and
`cargo check --workspace` are green.

Phase 1B (banned-vocabulary CFG export for `llguidance` or Outlines)
and Phase 1C (genre-template registry with `.toml` + `.md`
fragments per template) are queued — see `NEXT.md`.

The schema-stable signal that unblocks `service-proofreader` from
its hardcoded-protocol-templates stub has NOT yet been emitted.
That signal lands when Phase 1B and Phase 1C ship and Master
ratifies the public surface.

## Build and test

```sh
cargo check -p service-disclosure
cargo test -p service-disclosure
```

Both commands run from the `pointsav-monorepo` repo root.
`service-disclosure` is the ninth declared workspace member as of
v0.1.0.

## File layout

```
service-disclosure/
├── Cargo.toml          # crate manifest; serde + serde_yaml deps
├── README.md           # English entry point
├── README.es.md        # Spanish overview (bilingual pair, CLAUDE.md §6)
├── CLAUDE.md           # this file
├── AGENTS.md           # vendor-neutral pointer to CLAUDE.md
├── NEXT.md             # Phase 1B / 1C queue
├── ARCHITECTURE.md     # crate boundary, consumer contract, future shape
├── CHANGELOG.md        # one PATCH line per accepted commit
├── src/
│   ├── lib.rs          # crate root; pub uses; BANNED_VOCABULARY
│   ├── genre.rs        # Family + GenreTemplate
│   ├── request.rs      # ProtocolRequest + Register
│   ├── frontmatter.rs  # Frontmatter
│   └── validate.rs     # validate_frontmatter + ValidationError
└── templates/          # Phase 1C target (empty until then)
```

## Hard constraints — do not violate

- Public types must round-trip cleanly through both `serde_yaml` and
  `serde_json`. The frontmatter wire format is YAML; project-
  proofreader request payloads will be JSON. Any new field must
  carry test coverage for both.
- Banned-vocabulary list growth is a semver-MINOR change. Update the
  baseline assertion in the same commit.
- New `GenreTemplate` variants require updating the partition test
  in `genre.rs` and the per-genre rules in `validate.rs` in the
  same commit.
- This crate has no I/O. It is a pure type and validator library;
  the Doorman (`service-slm`) is the only component that opens
  network or filesystem handles. Do not introduce `tokio`, `reqwest`,
  or any I/O dependency here.

## Dependencies on other projects

- **Consumed by:** `service-proofreader` (project-proofreader
  cluster), via Cargo dependency. Cross-cluster contract is in the
  cluster manifest at `~/Foundry/clones/project-language/.claude/manifest.md`.
- **Consumes:** none (zero workspace-internal dependencies). Direct
  external dependencies are `serde` and `serde_yaml`.

## Commit convention

`service-disclosure: <what changed>` on cluster branch
`cluster/project-language`. Per `~/Foundry/CLAUDE.md` §8, commits
land via `~/Foundry/bin/commit-as-next.sh`; the PATCH increments
per accepted commit and the message ends with `Version: M.m.P`.

## What not to do

- Do not add a binary target. This crate is library-only.
- Do not embed AI calls or prompt construction here. Prompt
  scaffolding lives in the genre-template registry (Phase 1C
  `.toml`/`.md` fragments); the Doorman composes prompts at request
  time.
- Do not vendor citation registry data here. Citation IDs are
  resolved against `~/Foundry/citations.yaml`.

---

## Inherited rules — do not duplicate, do not silently override

- **Repo-level:** `pointsav-monorepo/CLAUDE.md`
- **Workspace-level:** `~/Foundry/CLAUDE.md`
- **Cluster-level:** `~/Foundry/clones/project-language/.claude/manifest.md`
