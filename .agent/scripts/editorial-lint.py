#!/usr/bin/env python3
"""editorial-lint.py — Track D / D1 editorial linter for project-editorial.

Self-contained: Python 3 standard library only. No network calls, no
third-party packages. Reads on-disk content and the rule data files under
.agent/editorial-qa/.

Layer 0-1 of the editorial QA stack (editorial plan §5):
  - frontmatter / research-trail schema  (gate)
  - deterministic prose lint              (gate on errors + advisory)

Checks
  ERROR (gate — exit 1 if any):
    - missing or empty YAML frontmatter
    - foundry-draft-v1: the five research-trail fields must be present;
      research_provenance must be a valid enum value
    - foundry-doc-v1: title / slug / category must be present
    - banned vocabulary in body prose
    - a body H1 ('# ') — the title comes from frontmatter (content-contract
      §5.2); the body carries no H1
    - terminal sections (See also / References / External links) out of order
    - a TOPIC without its .es.md (or .md) bilingual pair
  WARN (advisory — never gates):
    - a sentence longer than the Gate-0 hard ceiling (~45 words)

Usage
  editorial-lint.py PATH [PATH ...]      # files or directories (recurses *.md)
  editorial-lint.py --help

Exit status: 0 = no errors; 1 = at least one ERROR; 2 = bad invocation.
"""

import argparse
import os
import re
import sys

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
EDITORIAL_QA_DIR = os.path.normpath(os.path.join(SCRIPT_DIR, "..", "editorial-qa"))
BANNED_VOCAB_FILE = os.path.join(EDITORIAL_QA_DIR, "banned-vocabulary.txt")
CITATIONS_YAML = os.path.expanduser("~/Foundry/citations.yaml")

RESEARCH_TRAIL_FIELDS = [
    "research_done_count",
    "research_suggested_count",
    "open_questions_count",
    "research_provenance",
    "research_inline",
]
RESEARCH_PROVENANCE_ENUM = {
    "direct-consultation", "sub-agent", "citation-registry",
    "mixed", "tacit", "none",
}
DOC_REQUIRED_FIELDS = ["title", "slug", "category"]
TERMINAL_SECTIONS = ["see also", "references", "external links"]
SENTENCE_HARD_CEILING = 45  # Gate-0 rule 1 — expansion-sentence ceiling
CONFIDENCE_VALUES = {"established", "reported", "projected", "contested", "structural"}
CLAIM_REQUIRED_PLANNED = {"planned", "intended", "may", "target"}
CLAIM_KNOWN_FIELDS = {"id", "confidence", "cites", "valid_at", "depends_on"}

CLAIM_OPEN_RE = re.compile(r"<!--claim\b([^>]*)-->", re.S)
CLAIM_CLOSE_RE = re.compile(r"<!--/claim-->")
CLAIM_FIELD_RE = re.compile(r"(\w+)=(\S+)")

# Fallback banned list if the data file is missing — keeps the linter
# self-contained. The data file, when present, is authoritative.
DEFAULT_BANNED = [
    "leverage", "empower", "next-generation", "industry-leading",
    "seamless", "robust", "cutting-edge", "world-class",
    "utilize", "facilitate",
]


def load_banned_vocab():
    """Return the banned-term list from the data file, or the fallback."""
    if not os.path.isfile(BANNED_VOCAB_FILE):
        return list(DEFAULT_BANNED)
    terms = []
    with open(BANNED_VOCAB_FILE, encoding="utf-8") as fh:
        for line in fh:
            line = line.strip()
            if line and not line.startswith("#"):
                terms.append(line.lower())
    return terms or list(DEFAULT_BANNED)


def load_citations_registry():
    """Return citation IDs from citations.yaml, or None if unavailable."""
    if not os.path.isfile(CITATIONS_YAML):
        return None
    ids = set()
    with open(CITATIONS_YAML, encoding="utf-8") as fh:
        for line in fh:
            m = re.match(r"^([a-z0-9][a-z0-9_.-]*):\s*$", line.strip())
            if m:
                ids.add(m.group(1))
    return ids or None


def _parse_claim_list(raw):
    """Parse a [a,b,c] or bareword value into a list of strings."""
    raw = raw.strip()
    if raw.startswith("[") and raw.endswith("]"):
        inner = raw[1:-1].strip()
        return [x.strip() for x in inner.split(",") if x.strip()] if inner else []
    return [raw] if raw else []


def validate_claims(body, citations_registry):
    """Claim-validation pass per claim-authoring-convention §9.

    Returns (errors, warns). citations_registry is a set of valid IDs from
    citations.yaml, or None if the file is unavailable (skips cites resolution).
    """
    errors = []
    warns = []

    # Collect all markers in source order.
    markers = []
    for m in CLAIM_OPEN_RE.finditer(body):
        markers.append(("open", m.start(), m.end(), m.group(1)))
    for m in CLAIM_CLOSE_RE.finditer(body):
        markers.append(("close", m.start(), m.end(), ""))
    markers.sort(key=lambda x: x[1])

    # Match opens to closes, build claim list.
    claims = []
    open_stack = []
    for kind, start, end, attrs in markers:
        if kind == "open":
            open_stack.append((start, end, attrs))
        else:
            if not open_stack:
                errors.append("claim: unmatched <!--/claim--> at body offset %d" % start)
            else:
                o_start, o_end, raw_attrs = open_stack.pop()
                fields = {fm.group(1): fm.group(2)
                          for fm in CLAIM_FIELD_RE.finditer(raw_attrs)}
                claims.append({
                    "span_start": o_start, "span_end": end,
                    "text": body[o_end:start],
                    "id": fields.get("id"),
                    "confidence": fields.get("confidence"),
                    "cites_raw": fields.get("cites", "[]"),
                    "depends_on_raw": fields.get("depends_on", "[]"),
                    "unknown": [k for k in fields if k not in CLAIM_KNOWN_FIELDS],
                })
    for o_start, _, _ in open_stack:
        errors.append("claim: unclosed <!--claim--> at body offset %d" % o_start)

    if not claims:
        return errors, warns

    # Overlap check.
    by_start = sorted(claims, key=lambda c: c["span_start"])
    for i in range(len(by_start) - 1):
        if by_start[i + 1]["span_start"] < by_start[i]["span_end"]:
            errors.append("claim: %r overlaps %r" % (
                by_start[i].get("id") or "?",
                by_start[i + 1].get("id") or "?"))

    seen_ids = {}
    for idx, claim in enumerate(claims):
        cid = claim["id"]
        conf = claim["confidence"]
        cites = _parse_claim_list(claim["cites_raw"])
        text = claim["text"]
        label = cid or ("claim #%d" % (idx + 1))

        if not cid:
            errors.append("claim #%d: missing required field: id" % (idx + 1))
        if not conf:
            errors.append("claim %r: missing required field: confidence" % label)
        elif conf not in CONFIDENCE_VALUES:
            errors.append("claim %r: unknown confidence value %r" % (label, conf))

        if cid:
            if cid in seen_ids:
                errors.append("claim: duplicate id %r (first at claim #%d)" % (cid, seen_ids[cid]))
            else:
                seen_ids[cid] = idx + 1

        if conf and conf != "structural" and not cites:
            # WARN rather than ERROR: citations.yaml grows over time; empty cites
            # on a non-structural claim is a gap to fill, not a blocking defect.
            warns.append("claim %r: cites is empty — add a citations.yaml ID "
                         "when one exists (confidence=%s)" % (label, conf or "?"))

        if citations_registry is not None:
            for cite_id in cites:
                if cite_id not in citations_registry:
                    errors.append("claim %r: cites ID %r not in citations.yaml" % (label, cite_id))

        # Skip the English-word check on .es.md files — Spanish articles use
        # their own equivalents (planificado, previsto, etc.) for BCSC posture.
        if conf == "projected" and not path.endswith(".es.md"):
            if not any(w in text.lower() for w in CLAIM_REQUIRED_PLANNED):
                errors.append(
                    "claim %r: confidence=projected but text lacks "
                    "planned/intended/may/target language" % label)

        for uf in claim["unknown"]:
            warns.append("claim %r: unknown field %r (ignored)" % (label, uf))

    # depends_on resolution (same-file only; cross-file references get a warn).
    for claim in claims:
        label = claim["id"] or "?"
        for dep in _parse_claim_list(claim["depends_on_raw"]):
            if ":" in dep:
                warns.append("claim %r: cross-file depends_on %r cannot be validated at lint time" % (label, dep))
            elif dep not in seen_ids:
                errors.append("claim %r: depends_on %r not found in this file" % (label, dep))

    return errors, warns


def split_frontmatter(text):
    """Split a document into (frontmatter_keys, body).

    frontmatter_keys is the set of top-level YAML keys present (lowercased
    values kept for the scalar keys we inspect). Returns (None, text) when
    there is no '---' delimited frontmatter block at file start.
    """
    if not text.startswith("---"):
        return None, text
    lines = text.splitlines()
    end = None
    for i in range(1, len(lines)):
        if lines[i].strip() == "---":
            end = i
            break
    if end is None:
        return None, text
    keys = {}
    for raw in lines[1:end]:
        if not raw.strip() or raw.startswith((" ", "\t", "#", "-")):
            continue  # nested / list-item / comment line
        m = re.match(r"([A-Za-z0-9_]+)\s*:\s*(.*)$", raw)
        if m:
            keys[m.group(1)] = m.group(2).strip().strip('"').strip("'")
    body = "\n".join(lines[end + 1:])
    return keys, body


def strip_code_blocks(body):
    """Remove fenced code, inline code, and HTML comments before prose checks.

    A term inside a code span is being *mentioned* (e.g. a style guide
    quoting a banned word), not *used* — it must not trip the prose checks.
    HTML comments — including claim-authoring markers (`<!--claim ...-->`,
    claim-authoring-convention §3) — are metadata, not rendered prose: they
    are stripped so they neither count toward sentence length nor scan for
    banned vocabulary.
    """
    body = re.sub(r"```.*?```", "", body, flags=re.S)
    body = re.sub(r"<!--.*?-->", "", body, flags=re.S)
    body = re.sub(r"`[^`\n]+`", "", body)
    return body


def find_body_h1(body):
    """Return line numbers (1-based, body-relative) of any '# ' H1 lines,
    ignoring fenced code blocks."""
    hits = []
    in_fence = False
    for n, line in enumerate(body.splitlines(), start=1):
        if line.lstrip().startswith("```"):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        if re.match(r"#\s+\S", line):
            hits.append(n)
    return hits


def section_headings(body):
    """Return the ordered list of H2 heading texts (lowercased)."""
    out = []
    in_fence = False
    for line in body.splitlines():
        if line.lstrip().startswith("```"):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        m = re.match(r"##\s+(.*\S)\s*$", line)
        if m:
            out.append(m.group(1).strip().lower())
    return out


def check_terminal_order(headings):
    """Return an error string if terminal sections are out of canonical
    order or not at the document tail; else None."""
    present = [(i, h) for i, h in enumerate(headings) if h in TERMINAL_SECTIONS]
    if not present:
        return None
    got = [h for _, h in present]
    want = [s for s in TERMINAL_SECTIONS if s in got]
    if got != want:
        return ("terminal sections out of order: found %s, expected %s"
                % (got, want))
    first_terminal_idx = present[0][0]
    tail = headings[first_terminal_idx:]
    if any(h not in TERMINAL_SECTIONS for h in tail):
        return ("non-terminal section appears after a terminal section "
                "(See also / References / External links must be last)")
    return None


def long_sentences(body):
    """Return (sentence_excerpt, word_count) for sentences over the ceiling.
    Heuristic: prose paragraphs only — headings, tables, lists, code skipped."""
    prose = []
    in_fence = False
    for line in strip_code_blocks(body).splitlines():
        s = line.strip()
        if s.startswith("```"):
            in_fence = not in_fence
            continue
        if in_fence or not s:
            continue
        if s.startswith(("#", "|", ">", "-", "*", "+")) or re.match(r"\d+\.", s):
            continue
        prose.append(s)
    text = " ".join(prose)
    flagged = []
    for sentence in re.split(r"(?<=[.!?])\s+", text):
        words = re.findall(r"\S+", sentence)
        if len(words) > SENTENCE_HARD_CEILING:
            excerpt = sentence if len(sentence) <= 80 else sentence[:77] + "..."
            flagged.append((excerpt, len(words)))
    return flagged


def banned_hits(body, banned):
    """Return (term, count) for each banned term found in the body."""
    hits = []
    text = strip_code_blocks(body)
    for term in banned:
        pat = re.compile(r"\b" + re.escape(term) + r"[a-z]*\b", re.I)
        n = len(pat.findall(text))
        if n:
            hits.append((term, n))
    return hits


def lint_file(path, banned, citations_registry=None):
    """Lint one markdown file. Return (errors, warns) as lists of strings."""
    errors, warns = [], []
    with open(path, encoding="utf-8") as fh:
        text = fh.read()

    keys, body = split_frontmatter(text)
    if keys is None:
        errors.append("no YAML frontmatter block")
        return errors, warns

    schema = keys.get("schema", "")
    is_es = path.endswith(".es.md")

    if schema == "foundry-draft-v1":
        for field in RESEARCH_TRAIL_FIELDS:
            if field not in keys:
                errors.append("draft missing research-trail field: %s" % field)
        prov = keys.get("research_provenance")
        if prov is not None and prov not in RESEARCH_PROVENANCE_ENUM:
            errors.append("research_provenance not a valid value: %r" % prov)
    elif schema == "foundry-doc-v1":
        for field in DOC_REQUIRED_FIELDS:
            if field not in keys:
                errors.append("doc missing required frontmatter field: %s" % field)
    else:
        warns.append("unrecognized schema: %r (generic lint only)" % schema)

    # Body H1 is a published-content rule (content-contract §5.2 — the title
    # comes from frontmatter). A foundry-draft-v1 working document legitimately
    # carries a document title; the H1 is stripped when the draft is published.
    if schema != "foundry-draft-v1":
        for n in find_body_h1(body):
            errors.append("body H1 at body line %d — title comes from "
                           "frontmatter (content-contract §5.2)" % n)

    for term, count in banned_hits(body, banned):
        errors.append("banned vocabulary: %r x%d" % (term, count))

    order_err = check_terminal_order(section_headings(body))
    if order_err:
        errors.append(order_err)

    # Bilingual pair — TOPICs require an .es.md / .md counterpart.
    # GUIDEs are English-only; skip when the slug or type says guide.
    is_guide = (keys.get("type", "") == "guide"
                or os.path.basename(path).startswith("guide-"))
    if not is_guide and schema in ("foundry-doc-v1", "foundry-draft-v1"):
        if is_es:
            counterpart = path[:-len(".es.md")] + ".md"
        else:
            counterpart = path[:-len(".md")] + ".es.md"
        if not os.path.isfile(counterpart):
            errors.append("missing bilingual pair: %s" % os.path.basename(counterpart))

    for excerpt, wc in long_sentences(body):
        warns.append("sentence is %d words (ceiling ~%d): %s"
                     % (wc, SENTENCE_HARD_CEILING, excerpt))

    claim_errors, claim_warns = validate_claims(body, citations_registry)
    errors.extend(claim_errors)
    warns.extend(claim_warns)

    return errors, warns


def collect_markdown(paths):
    """Expand files and directories into a sorted list of .md files."""
    out = []
    for p in paths:
        if os.path.isdir(p):
            for root, _, files in os.walk(p):
                for f in files:
                    if f.endswith(".md"):
                        out.append(os.path.join(root, f))
        elif os.path.isfile(p) and p.endswith(".md"):
            out.append(p)
        else:
            print("skip (not a .md file or directory): %s" % p, file=sys.stderr)
    return sorted(set(out))


def main(argv=None):
    parser = argparse.ArgumentParser(
        description="Editorial linter (Track D / D1) — project-editorial.")
    parser.add_argument("paths", nargs="+", help="files or directories to lint")
    args = parser.parse_args(argv)

    banned = load_banned_vocab()
    citations_registry = load_citations_registry()
    if citations_registry is None:
        print("note: citations.yaml not found — cites resolution skipped", file=sys.stderr)
    files = collect_markdown(args.paths)
    if not files:
        print("no markdown files to lint", file=sys.stderr)
        return 2

    total_err = total_warn = 0
    for path in files:
        errors, warns = lint_file(path, banned, citations_registry)
        total_err += len(errors)
        total_warn += len(warns)
        if errors or warns:
            print(path)
            for e in errors:
                print("  ERROR  %s" % e)
            for w in warns:
                print("  WARN   %s" % w)

    print("\n%d file(s) linted — %d error(s), %d warning(s)"
          % (len(files), total_err, total_warn))
    return 1 if total_err else 0


if __name__ == "__main__":
    sys.exit(main())
