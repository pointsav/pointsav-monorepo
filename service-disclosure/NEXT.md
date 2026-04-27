# NEXT.md — service-disclosure

> Last updated: 2026-04-27
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

Phase 1A, Phase 1C, and Phase 1B all landed. The
schema-stable-ratification trigger phrase has been emitted in the
cluster outbox awaiting Master's ratification pass. On
ratification, `service-disclosure` jumps v0.2.1 → v0.3.0 and the
upgrade procedure relays to `project-proofreader` Task.

## Queue

- Author worked-example documents demonstrating each genre
  template — useful for apprenticeship-corpus seed material.
- Phase 3 — apprenticeship corpus directory scaffold. Surface
  `.gitignore` patterns to Master before authoring (Master
  lands the workspace-tier `.gitignore` edit; Task commits the
  per-task-type / per-tenant tree on `cluster/project-language`).
- Phase 5 — factory-release-engineering propose-via-outbox.
  Read-mode only; never commit there. Pickup if a future TOPIC
  surfaces a needed governance edit candidate.

## Deferred

- TRANSLATE-family adapter list — Deferred: convention §2 names
  TRANSLATE as a meta-protocol layered on top of the other
  families, not a separate generation track. No genre variants are
  needed in `GenreTemplate` for it. Revisit only if a specific
  language-pair workflow surfaces a need.

## Recently done

- 2026-04-27: Phase 1B — `banned-vocab.lark` shipped at
  `service-content/schemas/banned-vocab.lark` (Lark EBNF +
  `llguidance` per Master's v0.1.26 spec). `validate.py` runs
  in dual mode (Lark when available; regex fallback otherwise).
  pass-fixture = 0 hits; fail-fixture = 10 hits. Schema-stable
  ratification trigger phrase emitted in cluster outbox.
- 2026-04-27: v0.2.1 — `CORPUS-SCHEMA.md` documents the JSONL
  tuple shape for editorial-apprenticeship records.
- 2026-04-27: v0.2.0 — Phase 1C genre-template registry. 18 .toml +
  18 .md template pairs, `get_template` and
  `get_template_description` lookup functions, 7 new
  templates-module tests (26 total).
- 2026-04-27: v0.1.0 — Phase 1A scaffold landed (4-family taxonomy,
  18 genre templates, ProtocolRequest, Frontmatter, Register,
  validate_frontmatter, BANNED_VOCABULARY, 19 unit tests).
