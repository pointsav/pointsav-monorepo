#!/usr/bin/env python3
"""
check-chain-counts.py — Cross-check ingested record counts against YAML expectations.

For each chain YAML in service-business/, compares store_count_approx (expected)
against the actual line count in the corresponding JSONL file, then against the
parent-record count in cleansed-clusters.jsonl.

Flags:
  OVER   — actual raw count > 1.3× expected (sub-location inflation)
  UNDER  — actual raw count < 0.5× expected (missing data)
  EMPTY  — JSONL missing or zero records
  OK     — within acceptable range

Usage:
    python3 check-chain-counts.py
    python3 check-chain-counts.py --over     # only show over-count chains
    python3 check-chain-counts.py --empty    # only show empty chains
    python3 check-chain-counts.py --all      # include chains with no store_count_approx
"""

import argparse
import json
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).parent))
from config import TOTEBOX_DATA_PATH

RAW_DIR      = TOTEBOX_DATA_PATH / "service-fs" / "service-business"
CLEANSED_FILE = TOTEBOX_DATA_PATH / "service-business" / "cleansed-clusters.jsonl"

OVER_THRESHOLD  = 1.30  # raw > expected × this → OVER
UNDER_THRESHOLD = 0.50  # raw < expected × this → UNDER


def count_cleansed_by_chain() -> dict[str, int]:
    if not CLEANSED_FILE.exists():
        return {}
    counts: dict[str, int] = {}
    with open(CLEANSED_FILE) as f:
        for line in f:
            try:
                rec = json.loads(line)
                cid = rec.get("chain_id") or rec.get("category_id") or "unknown"
                counts[cid] = counts.get(cid, 0) + 1
            except Exception:
                pass
    return counts


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--over",  action="store_true", help="Show only OVER chains")
    parser.add_argument("--empty", action="store_true", help="Show only EMPTY chains")
    parser.add_argument("--under", action="store_true", help="Show only UNDER chains")
    parser.add_argument("--all",   action="store_true", help="Include chains with no expected count")
    args = parser.parse_args()

    show_all    = not (args.over or args.empty or args.under)
    cleansed    = count_cleansed_by_chain()
    results     = []

    for yf in sorted(RAW_DIR.glob("*.yaml")):
        chain_id = yf.stem
        try:
            cfg = yaml.safe_load(yf.read_text())
        except Exception:
            continue

        expected = cfg.get("store_count_approx")
        jf = RAW_DIR / f"{chain_id}.jsonl"
        raw_count = sum(1 for _ in open(jf)) if jf.exists() and jf.stat().st_size > 0 else 0
        clean_count = cleansed.get(chain_id, 0)

        if expected is None:
            if args.all or show_all:
                results.append((chain_id, None, raw_count, clean_count, "?"))
            continue

        if raw_count == 0:
            status = "EMPTY"
        elif raw_count > expected * OVER_THRESHOLD:
            status = "OVER"
        elif raw_count < expected * UNDER_THRESHOLD:
            status = "UNDER"
        else:
            status = "OK"

        if args.over  and status != "OVER":  continue
        if args.empty and status != "EMPTY": continue
        if args.under and status != "UNDER": continue

        results.append((chain_id, expected, raw_count, clean_count, status))

    hdr = f"{'chain_id':<38} {'expected':>9} {'raw':>7} {'cleansed':>9} {'ratio':>7}  status"
    print(hdr)
    print("-" * len(hdr))
    for chain_id, expected, raw, clean, status in results:
        ratio = f"{raw/expected:.1f}x" if expected else "?"
        flag  = f"  <<< {status}" if status not in ("OK", "?") else ""
        exp_s = str(expected) if expected is not None else "?"
        print(f"{chain_id:<38} {exp_s:>9} {raw:>7} {clean:>9} {ratio:>7}{flag}")

    totals = {"OVER": 0, "UNDER": 0, "EMPTY": 0, "OK": 0, "?": 0}
    for *_, status in results:
        totals[status] = totals.get(status, 0) + 1
    print()
    print(f"Summary: {totals['OK']} OK  |  {totals['OVER']} OVER  |  {totals['UNDER']} UNDER  |  {totals['EMPTY']} EMPTY")


if __name__ == "__main__":
    main()
