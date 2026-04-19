# NEXT.md — service-extraction

Session close: 2026-04-18

## Accomplished today

- Wrote `samples/expected.yaml` — ground-truth extraction targets for all
  10 `.eml` files in `samples/`, using the five-value classification enum
  (Newsletter, Marketing, Business, Transactional, Personal) and real
  filesystem filenames as keys.
- Created `CLEANUP_LOG.md` in this directory and recorded the `ask_Ryan`
  sample-description correction.
- Corrected the primary "Ryan Nguyen" → "Ryan Rumsey" citation in
  `VALIDATION.md` §"Extraction targets per email" (line 40) and
  `ROADMAP.md` §"The gap service-extraction is closing" (line 35).
- No changes to `src/main.rs`, `Cargo.toml`, or any technique.
  Audit + documentation only.

## Current state

- `samples/expected.yaml` — present, 10 entries, ready to be consumed.
- `CLEANUP_LOG.md` — present, one entry dated 2026-04-18.
- `VALIDATION.md` — §"Extraction targets per email" corrected. Three
  stale "Ryan Nguyen" mentions remain at lines 26, 85, 134
  (see open decisions below).
- `ROADMAP.md` — §"The gap service-extraction is closing" corrected.
  No other occurrences of the stale name in this file.

## Single next command when you return

> "Write `scripts/score_against_expected.sh` per VALIDATION.md
> §Reproducing measurements, then run it against v0.2 output to
> establish the baseline score before any v0.4 technique is added."

Rationale: `samples/expected.yaml` has no utility until a script consumes
it, and the v0.2 baseline must be measured before any v0.4 technique can
be credited with an improvement.

## Open decisions you still owe me

1. Whether to apply the same "Ryan Nguyen" → "Ryan Rumsey" correction to
   the three remaining mentions in `VALIDATION.md` (lines 26, 85, 134).
   The CLEANUP_LOG entry says "Corrected in both docs" — leaving these
   as-is makes that statement partly inaccurate. Yes or no.
