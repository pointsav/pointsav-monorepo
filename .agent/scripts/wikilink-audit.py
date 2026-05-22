#!/usr/bin/env python3
"""wikilink-audit.py — resolve every [[wikilink]] in a content wiki.

Self-contained: Python 3 standard library only, no network.

A wiki article links to another with `[[slug]]` or `[[slug|Display Text]]`
(content-contract §5.1). The slug resolver is flat and global within one wiki:
`[[slug]]` resolves when some file in the wiki has the filename stem `slug`,
or an article declares `slug` in an `aliases:` frontmatter list. An unresolved
slug renders as a red link — intentional for a genuinely unwritten article,
a defect when the target exists under a different slug.

Usage
  wikilink-audit.py WIKI_DIR [WIKI_DIR ...]

For each wiki it prints every unresolved `[[target]]`, grouped by source file,
and a per-wiki count. Exit status 0 always — red links are not a hard failure;
the report is for editorial judgement.
"""

import os
import re
import sys

WIKILINK = re.compile(r"\[\[([^\]|]+)(?:\|[^\]]*)?\]\]")
ALIASES_BLOCK = re.compile(r"^aliases:\s*\n((?:\s*-\s*.+\n)+)", re.M)


def collect_slugs(wiki_dir):
    """Return the set of resolvable slugs: every .md filename stem (with the
    `.es` of a Spanish pair stripped), plus every `aliases:` entry."""
    slugs = set()
    for root, _, files in os.walk(wiki_dir):
        if any(seg in root for seg in ("/.git", "/.agent", "/.claude", "/.github")):
            continue
        for f in files:
            if not f.endswith(".md"):
                continue
            stem = f[:-3]
            if stem.endswith(".es"):
                stem = stem[:-3]
            slugs.add(stem)
            path = os.path.join(root, f)
            try:
                with open(path, encoding="utf-8") as fh:
                    head = fh.read(4000)
            except OSError:
                continue
            m = ALIASES_BLOCK.search(head)
            if m:
                for line in m.group(1).splitlines():
                    alias = line.strip().lstrip("-").strip().strip('"').strip("'")
                    if alias:
                        slugs.add(alias)
    return slugs


def audit_wiki(wiki_dir):
    """Print unresolved wikilinks for one wiki. Return the unresolved count."""
    slugs = collect_slugs(wiki_dir)
    unresolved = 0
    for root, _, files in os.walk(wiki_dir):
        if any(seg in root for seg in ("/.git", "/.agent", "/.claude", "/.github")):
            continue
        for f in sorted(files):
            if not f.endswith(".md"):
                continue
            path = os.path.join(root, f)
            with open(path, encoding="utf-8") as fh:
                body = fh.read()
            # Strip fenced and inline code: a `[[slug]]` inside a code span is
            # the wikilink syntax being *documented*, not a live link.
            body = re.sub(r"```.*?```", "", body, flags=re.S)
            body = re.sub(r"`[^`\n]+`", "", body)
            misses = []
            for m in WIKILINK.finditer(body):
                target = m.group(1).strip()
                if target not in slugs:
                    misses.append(target)
            if misses:
                rel = os.path.relpath(path, wiki_dir)
                print("  %s" % rel)
                for t in sorted(set(misses)):
                    print("    [[%s]]" % t)
                unresolved += len(set(misses))
    return unresolved


def main(argv=None):
    args = (argv if argv is not None else sys.argv[1:])
    if not args:
        print("usage: wikilink-audit.py WIKI_DIR [WIKI_DIR ...]", file=sys.stderr)
        return 2
    grand = 0
    for wiki in args:
        if not os.path.isdir(wiki):
            print("skip (not a directory): %s" % wiki, file=sys.stderr)
            continue
        print("=== %s ===" % wiki)
        n = audit_wiki(wiki)
        print("  %d unresolved wikilink target(s)\n" % n)
        grand += n
    print("total: %d unresolved wikilink target(s)" % grand)
    return 0


if __name__ == "__main__":
    sys.exit(main())
