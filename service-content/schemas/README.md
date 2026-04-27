# service-content/schemas/

Decode-time constraint grammars consumed by `service-slm`'s Doorman
when serving `service-proofreader` and other write-assistant
clients. The grammars live with the data substrate
(`service-content`) rather than the schema-definition crate
(`service-disclosure`) because they are **shared data** across all
tenants under `moduleId` namespacing — they are not per-tenant
code.

The Phase 1B grammar in this directory enforces the cross-genre
banned-vocabulary list from
`pointsav-monorepo/service-disclosure/src/lib.rs::BANNED_VOCABULARY`.

## Files

| File | Purpose |
|---|---|
| `banned-vocab.lark` | Lark EBNF grammar refusing any generation that would emit one of the eight banned terms as a bare word. Loaded by `llguidance` at inference time on Tier B (Yo-Yo) and Tier A (local OLMo). |
| `validate.py` | Validation harness. Uses Python `lark` package when available; falls back to a regex-equivalent check otherwise. Run before shipping any grammar edit. |
| `test-prose-pass.txt` | Synthetic prose that contains no banned words. Validation parses cleanly. |
| `test-prose-fail.txt` | Synthetic prose that contains every banned word. Validation rejects each occurrence outside backtick quotes. |

## Top-level rule

The grammar's entry rule is `response`. A response is any sequence
of allowed tokens, whitespace, punctuation, and backtick-quoted
segments. The grammar is intentionally permissive outside the
banned-word constraint — the goal is to allow all reasonable
English prose, not to enforce sentence-level structure.

## Banned terms (v0.1.0 baseline)

The eight terms locked in v0.1.0 of `service-disclosure`:

`leverage`, `empower`, `next-generation`, `industry-leading`,
`seamless`, `robust`, `cutting-edge`, `world-class`.

Matching is **case-insensitive** and **word-boundary-anchored**.
`Leverage`, `LEVERAGE`, and `leverage` are all rejected; substrings
inside other words (e.g. a hypothetical `leverages`) are not in
scope of the exact-form baseline.

The list is the editorial-grade marketing-vocabulary set the
language-protocol substrate convention §2.2 documents. Adding
terms is a semver-MINOR change in `service-disclosure` and updates
this grammar in lockstep.

## The escape rule — backtick-quoted citations

A banned word inside a backtick-quoted segment is **permitted**.
The CORPUS-SCHEMA at
`pointsav-monorepo/service-disclosure/CORPUS-SCHEMA.md` §5
documents the editorial invariant: an attempt that quotes a
banned term as prior-art example does not count as a hit. The
grammar implements this by permitting any non-backtick content
inside `` ` ` `` and any content inside ``` ``` ```.

This is what makes the grammar usable for editorial work that
must occasionally reference banned terms — for example, a
TOPIC about anti-homogenization discipline that quotes `leverage`
as an example of the diction it warns against.

## Validation procedure

```sh
python3 validate.py
```

`validate.py` runs in two modes:

1. **Lark mode** (preferred): if the Python `lark` package is
   importable, the grammar is loaded via `lark.Lark` and each
   test fixture is parsed. Pass-fixture must parse cleanly;
   fail-fixture must produce a `lark.exceptions.UnexpectedInput`
   error at every banned-word occurrence outside backtick quotes.
2. **Regex fallback**: if `lark` is not available on the host,
   the validator extracts the negative-lookahead pattern from
   `banned-vocab.lark` directly and applies it via Python's
   built-in `re` module. This is conceptually equivalent for the
   banned-vocab use case because Lark itself uses `re` for
   terminal matching; any pattern that the regex fallback rejects
   the Lark validator also rejects.

The regex fallback is sufficient for editorial-grade validation
during development. Production use on Tier A and Tier B requires
the full Lark grammar loaded by `llguidance` at inference time —
the validation harness does not replace `llguidance`'s decode-time
enforcement, only confirms the grammar text is well-formed.

## Cross-references

| Document | What it adds |
|---|---|
| `pointsav-monorepo/service-disclosure/CORPUS-SCHEMA.md` | JSONL tuple shape that records `attempt.banned_vocabulary_hits` for any banned word the grammar lets through (not expected once decode-time enforcement is live, but the field stays available for the v0.1 stub-schema migration window). |
| `pointsav-monorepo/service-disclosure/src/lib.rs::BANNED_VOCABULARY` | Authoritative list. The grammar in this directory mirrors that constant; drift between the two is a defect. |
| `~/Foundry/conventions/language-protocol-substrate.md` §2.2 | The editorial rationale for the banned-vocabulary list. |
| `vendor/pointsav-monorepo/service-slm/router/` | The Doorman that loads the grammar at request time and passes it to the inference path. |

## Versioning

The grammar's version is implicit — it tracks `service-disclosure`'s
version because the two artefacts must agree. Edits to the banned
list in either place trigger a paired commit that touches both.
The grammar is not versioned independently because no consumer
needs to pin against an old version of this artefact.

## Status

**v0.1.0 — initial Phase 1B ship.** Eight banned words,
case-insensitive word-boundary matching, backtick escape rule.
Validation harness ships in dual-mode (Lark + regex fallback)
because the workspace VM does not yet have the Python `lark`
package installed; that gap is surfaced to Master in the
project-language Task outbox.

## License

Apache-2.0. The `.lark` grammar and the validation harness inherit
the monorepo `LICENSE` file at the repository root.
