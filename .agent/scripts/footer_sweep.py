#!/usr/bin/env python3
"""
footer_sweep.py — IP footer sweep for content-wiki-documentation and woodfine-fleet-deployment.

Applies canonical three-variant footer to all TOPIC-* (EN + ES) and GUIDE-* files.

Modes:
  --dry-run   Print counts and sample changes; write nothing.
  --apply     Write changes to disk.
  --report    Print every file that would be changed (verbose dry-run).

Usage:
  python3 footer_sweep.py --dry-run
  python3 footer_sweep.py --apply
"""

import argparse
import os
import re
import sys
from pathlib import Path

FOUNDRY = Path("/srv/foundry")
WIKI = FOUNDRY / "clones/project-language/content-wiki-documentation"
FLEET = FOUNDRY / "customer/woodfine-fleet-deployment"

# ---------------------------------------------------------------------------
# Canonical footer blocks (no trailing newline — appended with \n\n)
# ---------------------------------------------------------------------------

FOOTER_TOPIC_EN = """\
---

*Copyright © 2026 Woodfine Capital Projects Inc. Licensed under [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/).*

*Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™, Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital Projects Inc., used in Canada, the United States, Latin America, and Europe. All other trademarks are the property of their respective owners.*"""

FOOTER_TOPIC_ES = """\
---

*Copyright © 2026 Woodfine Capital Projects Inc. Licenciado bajo [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/).*

*Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™, Totebox Orchestration™ y Totebox Archive™ son marcas comerciales de Woodfine Capital Projects Inc., utilizadas en Canadá, los Estados Unidos, América Latina y Europa. Todas las demás marcas comerciales son propiedad de sus respectivos titulares.*"""

FOOTER_GUIDE_EN = """\
---

*Copyright © 2026 Woodfine Management Corp. All rights reserved.*

*Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™, Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital Projects Inc., used in Canada, the United States, Latin America, and Europe. All other trademarks are the property of their respective owners.*"""

# Regex to detect any existing copyright footer (old WS5 one-liners or partial blocks)
COPYRIGHT_LINE_RE = re.compile(r"^\*Copyright © 2026 Woodfine", re.MULTILINE)


def strip_existing_footer(text: str) -> str:
    """Remove existing copyright footer and any immediately preceding --- separator."""
    # Find the copyright line
    m = COPYRIGHT_LINE_RE.search(text)
    if not m:
        return text

    # Walk back to strip the preceding --- separator + blank lines
    before = text[: m.start()]
    # Strip trailing whitespace/newlines then check for a trailing ---
    stripped = before.rstrip()
    if stripped.endswith("---"):
        stripped = stripped[: -3].rstrip()

    # The footer itself: take everything from the copyright line to end of file
    # (handles multi-line new-style footers too)
    return stripped


def build_new_text(original: str, footer: str) -> str:
    """Return file text with old footer stripped and new footer appended."""
    body = strip_existing_footer(original)
    return body + "\n\n" + footer + "\n"


def collect_wiki_files():
    """Yield (path, footer) tuples for all content-wiki-documentation .md files."""
    for p in sorted(WIKI.rglob("*.md")):
        # Skip .git, skip zip contents, skip CLAUDE.md / NEXT.md / etc.
        rel = str(p.relative_to(WIKI))
        if ".git" in rel:
            continue
        if ".claude" in rel or ".agent" in rel:  # operational rules files, not TOPIC content
            continue
        if "app-mediakit" in rel:  # binary zip contents
            continue
        name = p.name
        # README files use a different footer variant; exclude from this sweep
        if name.lower().startswith("readme"):
            continue
        # Only TOPIC-* and similar wiki content files; skip operational files
        # Operational files that stay English-only per CLAUDE.md §6:
        operational = {
            "CLAUDE.md", "NEXT.md", "CHANGELOG.md", "MANIFEST.md",
            "NOTAM.md", "BUDGET.md", "TRADEMARK.md", "CODE_OF_CONDUCT.md",
            "SECURITY.md", "RESEARCH-BIM-MARKET.md",
        }
        if name in operational:
            continue
        # _index files are renderer stubs — skip
        if name.startswith("_index"):
            continue
        # glossary CSV and yaml — not markdown content
        if not name.endswith(".md"):
            continue

        # Determine footer variant
        if name.endswith(".es.md"):
            footer = FOOTER_TOPIC_ES
        else:
            footer = FOOTER_TOPIC_EN

        yield p, footer


def collect_guide_files():
    """Yield (path, footer) tuples for all guide-*.md files in woodfine-fleet-deployment."""
    for p in sorted(FLEET.rglob("*.md")):
        rel = str(p.relative_to(FLEET))
        if ".git" in rel:
            continue
        name = p.name
        # Only guide-* and GUIDE-* files
        lower = name.lower()
        if not lower.startswith("guide-"):
            continue
        yield p, FOOTER_GUIDE_EN


def process_file(path: Path, footer: str, apply: bool, verbose: bool):
    """Read, compute new content, optionally write. Return (changed: bool, sample: str)."""
    original = path.read_text(encoding="utf-8")
    new_text = build_new_text(original, footer)

    changed = original != new_text

    sample = None
    if changed and verbose:
        # Show last 6 lines of new file as sample
        lines = new_text.rstrip().split("\n")
        sample = "\n".join(lines[-6:])

    if changed and apply:
        path.write_text(new_text, encoding="utf-8")

    return changed, sample


def main():
    parser = argparse.ArgumentParser(description="IP footer sweep")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--dry-run", action="store_true", help="Count changes, show samples")
    mode.add_argument("--apply", action="store_true", help="Write changes to disk")
    mode.add_argument("--report", action="store_true", help="List every changed file")
    args = parser.parse_args()

    verbose = args.dry_run or args.report
    do_write = args.apply

    wiki_changed = 0
    wiki_total = 0
    guide_changed = 0
    guide_total = 0
    samples = []

    print("=== content-wiki-documentation sweep ===")
    for path, footer in collect_wiki_files():
        wiki_total += 1
        changed, sample = process_file(path, footer, do_write, verbose)
        if changed:
            wiki_changed += 1
            rel = str(path.relative_to(WIKI))
            if args.report:
                print(f"  CHANGE  {rel}")
            if args.dry_run and wiki_changed <= 3 and sample:
                samples.append((rel, sample))

    print(f"  files scanned : {wiki_total}")
    print(f"  files changed : {wiki_changed}")
    if do_write:
        print(f"  written       : {wiki_changed}")

    print()
    print("=== woodfine-fleet-deployment guide sweep ===")
    for path, footer in collect_guide_files():
        guide_total += 1
        changed, sample = process_file(path, footer, do_write, verbose)
        if changed:
            guide_changed += 1
            rel = str(path.relative_to(FLEET))
            if args.report:
                print(f"  CHANGE  {rel}")
            if args.dry_run and guide_changed <= 3 and sample:
                samples.append((rel, sample))

    print(f"  files scanned : {guide_total}")
    print(f"  files changed : {guide_changed}")
    if do_write:
        print(f"  written       : {guide_changed}")

    if samples:
        print()
        print("=== sample output (up to 3 files) ===")
        for rel, s in samples:
            print(f"\n--- {rel} (last 6 lines) ---")
            print(s)

    total_changed = wiki_changed + guide_changed
    total_scanned = wiki_total + guide_total
    print()
    print(f"Total: {total_changed}/{total_scanned} files {'written' if do_write else 'would change'}")

    if not do_write and total_changed > 0:
        print()
        print("Run with --apply to write changes.")


if __name__ == "__main__":
    main()
