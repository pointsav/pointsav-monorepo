# NEXT.md — service-disclosure

> Last updated: 2026-04-27
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

Nothing in progress — Phase 1A and Phase 1C landed
(v0.1.0 and v0.2.0). Next pickup is Phase 1B from the Queue, but
Phase 1B is blocked on a cross-cluster decision (see Blocked).

## Queue

- Emit schema-stable signal to project-proofreader Task once
  Phase 1B lands AND Master ratifies the public surface. Until
  then, `service-proofreader` runs on hardcoded protocol templates.
- Add JSONL corpus-schema spec for editorial-apprenticeship tuples
  — either a `CORPUS-SCHEMA.md` at this directory root or an
  appended section in `ARCHITECTURE.md`. Coordinate with the
  apprenticeship-corpus directory scaffold (Phase 3 of the cluster
  brief).
- Author Phase 1B banned-vocabulary CFG once the library question
  resolves. Round-trip test: a synthetic prose sample containing
  `leverage` rejected at decode time; same sample without
  `leverage` passes. Document the CFG schema in `ARCHITECTURE.md`.
- Author worked-example documents demonstrating each genre
  template — useful for the Phase 2 style-guide TOPICs and for the
  apprenticeship-corpus seed.

## Blocked

- Phase 1B CFG-format choice — Blocked on: confirmation of which
  decode-time constraint library `service-slm` integrates with in
  the project-slm cluster's AS-2 implementation (`llguidance` or
  Outlines). Surfaced to Master via outbox 2026-04-27.

## Deferred

- TRANSLATE-family adapter list — Deferred: convention §2 names
  TRANSLATE as a meta-protocol layered on top of the other
  families, not a separate generation track. No genre variants are
  needed in `GenreTemplate` for it. Revisit only if a specific
  language-pair workflow surfaces a need.

## Recently done

- 2026-04-27: v0.2.0 — Phase 1C genre-template registry. 18 .toml +
  18 .md template pairs, `get_template` and
  `get_template_description` lookup functions, 7 new
  templates-module tests (26 total).
- 2026-04-27: v0.1.0 — Phase 1A scaffold landed (4-family taxonomy,
  18 genre templates, ProtocolRequest, Frontmatter, Register,
  validate_frontmatter, BANNED_VOCABULARY, 19 unit tests).
