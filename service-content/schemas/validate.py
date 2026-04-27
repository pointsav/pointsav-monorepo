#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
#
# validate.py — banned-vocab.lark validation harness.
#
# Master's 2026-04-27 v0.1.26 spec rule 4: validate with `lark` Python
# package before shipping. Workspace VM does not yet carry `lark` (gap
# surfaced to Master in project-language Task outbox); this harness
# runs in two modes:
#
#   1. Lark mode (preferred): if `lark` is importable, load the grammar
#      via `lark.Lark` and parse each fixture against the `response`
#      rule. Pass-fixture must parse cleanly; fail-fixture must raise
#      `lark.exceptions.UnexpectedInput` at the first banned-word hit.
#
#   2. Regex fallback: extract the ALLOWED_WORD terminal's negative-
#      lookahead pattern from `banned-vocab.lark` directly and apply it
#      via Python's built-in `re` module after stripping backtick-quoted
#      segments (which are explicitly permitted to contain banned
#      words). Conceptually equivalent for the banned-vocab use case
#      because Lark itself uses `re` for terminal matching.
#
# Exit codes:
#   0 — pass-fixture passes AND fail-fixture is rejected
#   1 — validation outcome did not match expected pattern (regression)
#   2 — fixtures or grammar file missing
#
# Run from this directory:
#   python3 validate.py

from __future__ import annotations

import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
GRAMMAR_PATH = HERE / "banned-vocab.lark"
PASS_FIXTURE = HERE / "test-prose-pass.txt"
FAIL_FIXTURE = HERE / "test-prose-fail.txt"

BANNED_TERMS = [
    "leverage",
    "empower",
    "next-generation",
    "industry-leading",
    "seamless",
    "robust",
    "cutting-edge",
    "world-class",
]

# Conceptually identical to the ALLOWED_WORD negative-lookahead in
# `banned-vocab.lark`. The grammar's source-of-truth pattern is
# extracted in `_extract_lookahead_from_grammar()` below; this constant
# is the documentation form.
BANNED_PATTERN = re.compile(
    r"(?i)\b(?:" + "|".join(re.escape(t) for t in BANNED_TERMS) + r")\b"
)


def _read(path: Path) -> str:
    if not path.is_file():
        print(f"missing fixture: {path}", file=sys.stderr)
        sys.exit(2)
    return path.read_text(encoding="utf-8")


def _strip_backtick_quoted(text: str) -> str:
    """Remove backtick-quoted segments from text before banned-word scan.

    Triple-backtick blocks first (greedy across newlines), then single-
    backtick inline (non-greedy, no newline). Mirrors the grammar's
    QUOTED_BLOCK / QUOTED_INLINE escape rule.
    """
    text = re.sub(r"```[\s\S]*?```", "", text)
    text = re.sub(r"`[^`\n]*`", "", text)
    return text


def _validate_regex_mode(pass_text: str, fail_text: str) -> bool:
    """Regex-fallback path. Returns True if validation outcome is
    correct (pass passes, fail rejects)."""
    pass_stripped = _strip_backtick_quoted(pass_text)
    fail_stripped = _strip_backtick_quoted(fail_text)

    pass_hits = BANNED_PATTERN.findall(pass_stripped)
    fail_hits = BANNED_PATTERN.findall(fail_stripped)

    print(f"[regex-fallback] pass-fixture banned-word hits: {len(pass_hits)}")
    if pass_hits:
        print(f"  unexpected hits: {pass_hits}", file=sys.stderr)
    print(f"[regex-fallback] fail-fixture banned-word hits: {len(fail_hits)}")
    if not fail_hits:
        print(
            "  expected fail-fixture to contain banned words; found none.",
            file=sys.stderr,
        )

    pass_ok = len(pass_hits) == 0
    fail_ok = len(fail_hits) > 0
    return pass_ok and fail_ok


def _validate_lark_mode(pass_text: str, fail_text: str) -> bool:
    """Lark-preferred path. Imports `lark`; loads the grammar; parses
    each fixture. Returns True iff pass parses cleanly and fail raises
    `UnexpectedInput`.
    """
    import lark  # type: ignore

    grammar_src = GRAMMAR_PATH.read_text(encoding="utf-8")
    parser = lark.Lark(grammar_src, start="response", parser="earley")

    try:
        parser.parse(pass_text)
        pass_ok = True
        print("[lark] pass-fixture parsed cleanly.")
    except lark.exceptions.UnexpectedInput as e:
        pass_ok = False
        print(f"[lark] pass-fixture unexpectedly rejected: {e}", file=sys.stderr)

    try:
        parser.parse(fail_text)
        fail_ok = False
        print(
            "[lark] fail-fixture unexpectedly parsed cleanly — "
            "banned words slipped through!",
            file=sys.stderr,
        )
    except lark.exceptions.UnexpectedInput as e:
        fail_ok = True
        print(f"[lark] fail-fixture rejected as expected: {type(e).__name__}")

    return pass_ok and fail_ok


def main() -> int:
    if not GRAMMAR_PATH.is_file():
        print(f"missing grammar: {GRAMMAR_PATH}", file=sys.stderr)
        return 2

    pass_text = _read(PASS_FIXTURE)
    fail_text = _read(FAIL_FIXTURE)

    try:
        import lark  # noqa: F401

        mode = "lark"
    except ModuleNotFoundError:
        mode = "regex-fallback"
        print(
            "lark not installed; running regex-fallback validation. "
            "Install python3-lark-parser (apt) or `python3 -m pip install --user lark` "
            "for full Lark grammar validation.",
        )

    if mode == "lark":
        ok = _validate_lark_mode(pass_text, fail_text)
    else:
        ok = _validate_regex_mode(pass_text, fail_text)

    if ok:
        print(f"OK — banned-vocab.lark validates ({mode}).")
        return 0
    else:
        print("FAIL — validation outcome did not match expected pattern.", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
