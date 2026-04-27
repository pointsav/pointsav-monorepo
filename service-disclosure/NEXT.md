# NEXT.md — service-disclosure

> Last updated: 2026-04-27
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

Nothing in progress — Phase 1A landed in v0.1.0. Next pickup is
Phase 1B from the Queue.

## Queue

- Author Phase 1B banned-vocabulary CFG — pick `llguidance` or
  Outlines based on what `service-slm` (Doorman) already integrates
  with in the project-slm cluster's AS-2 implementation. Round-trip
  test: a synthetic prose sample containing `leverage` rejected at
  decode time; same sample without `leverage` passes. Document the
  CFG schema in `ARCHITECTURE.md`.
- Author Phase 1C genre-template registry — one `.toml` + one `.md`
  per `GenreTemplate` variant. Eighteen templates total. Author all
  PROSE templates first, then COMMS, then LEGAL. Wire
  `fn get_template(template: GenreTemplate) -> &str` once at least
  one template lands.
- Emit schema-stable signal to project-proofreader Task — append an
  outbox entry when Phase 1B and Phase 1C are testable and Master
  ratifies the public surface. Until then `service-proofreader`
  runs on hardcoded protocol templates.
- Add JSONL corpus-schema spec for editorial-apprenticeship tuples
  — either a `CORPUS-SCHEMA.md` at this directory root or an
  appended section in `ARCHITECTURE.md`. Coordinate with the
  apprenticeship-corpus directory scaffold (Phase 3 of the cluster
  brief).

## Blocked

- Phase 1B CFG-format choice — Blocked on: confirmation of which
  decode-time constraint library `service-slm` already integrates
  with. Surface to Master via outbox if the project-slm cluster has
  not landed AS-2 yet.

## Deferred

- TRANSLATE-family adapter list — Deferred: convention §2 names
  TRANSLATE as a meta-protocol layered on top of the other
  families, not a separate generation track. No genre variants are
  needed in `GenreTemplate` for it. Revisit only if a specific
  language-pair workflow surfaces a need.

## Recently done

- 2026-04-27: v0.1.0 — Phase 1A scaffold landed (4-family taxonomy,
  18 genre templates, ProtocolRequest, Frontmatter, Register,
  validate_frontmatter, BANNED_VOCABULARY, 19 unit tests).
