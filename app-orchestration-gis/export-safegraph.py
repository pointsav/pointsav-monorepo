#!/usr/bin/env python3
"""
export-safegraph.py — Export service-business JSONL records as SafeGraph-formatted CSV.

The service-business JSONL files already use SafeGraph field names
(location_name, latitude, longitude, placekey, naics_code, etc.) because
ingest-osm.py / ingest-overture.py write in that schema. This script
merges all chain JSONLs into a single flat CSV and adds chain_class.

Usage:
  python3 export-safegraph.py [--sample N] [--output PATH] [--iso CC ...]

Options:
  --sample N      Write only the first N records (test mode).
  --output PATH   Output CSV path (default: work/export-safegraph.csv).
  --iso CC        Filter to one or more ISO country codes (repeatable).

Output fields (SafeGraph schema):
  placekey, location_name, brands, street_address, city, region,
  postal_code, iso_country_code, latitude, longitude, naics_code,
  top_category, sub_category, polygon_wkt, parent_placekey,
  chain_id, chain_class, operating_status, confidence
"""

import argparse
import csv
import json
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

SRC_DIR = Path("/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-business")
WORK_DIR = Path(__file__).resolve().parent / "work"

SAFEGRAPH_FIELDS = [
    "placekey", "location_name", "brands", "street_address", "city",
    "region", "postal_code", "iso_country_code", "latitude", "longitude",
    "naics_code", "top_category", "sub_category", "polygon_wkt",
    "parent_placekey", "chain_id", "chain_class", "operating_status", "confidence",
]

# Chain class lookup — derive from chain_id prefix or naics_code
# Populated by ingest scripts; fall back to naics heuristic.
NAICS_TO_CLASS = {
    "452311": "hypermarket",
    "452910": "warehouse",
    "444110": "hardware",
    "442110": "lifestyle",
    "445110": "grocery",
    "445131": "grocery",
    "452319": "hypermarket",
}


def classify(rec: dict) -> str:
    naics = str(rec.get("naics_code", "") or "")
    return NAICS_TO_CLASS.get(naics, "other")


def export(args: argparse.Namespace) -> int:
    jsonl_files = sorted(SRC_DIR.glob("*.jsonl"))
    if not jsonl_files:
        print(f"ERROR: no JSONL files found in {SRC_DIR}", file=sys.stderr)
        return 1

    iso_filter = set(args.iso) if args.iso else None
    out_path = Path(args.output)
    out_path.parent.mkdir(parents=True, exist_ok=True)

    written = 0
    skipped_iso = 0
    skipped_inactive = 0

    with out_path.open("w", newline="", encoding="utf-8") as fh:
        writer = csv.DictWriter(fh, fieldnames=SAFEGRAPH_FIELDS, extrasaction="ignore")
        writer.writeheader()

        for jsonl in jsonl_files:
            with jsonl.open(encoding="utf-8") as jf:
                for raw in jf:
                    raw = raw.strip()
                    if not raw:
                        continue
                    rec = json.loads(raw)

                    # ISO filter
                    iso = rec.get("iso_country_code") or ""
                    if iso_filter and iso not in iso_filter:
                        skipped_iso += 1
                        continue

                    # Skip explicitly closed/removed locations
                    status = rec.get("operating_status") or ""
                    if status.lower() in ("closed", "removed", "inactive"):
                        skipped_inactive += 1
                        continue

                    out_rec = {f: rec.get(f) for f in SAFEGRAPH_FIELDS}
                    out_rec["chain_class"] = classify(rec)

                    writer.writerow(out_rec)
                    written += 1

                    if args.sample and written >= args.sample:
                        break
            if args.sample and written >= args.sample:
                break

    print(f"Wrote {written:,} records → {out_path}")
    if skipped_iso:
        print(f"  Skipped {skipped_iso:,} records (ISO filter)")
    if skipped_inactive:
        print(f"  Skipped {skipped_inactive:,} records (inactive)")
    return 0


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--sample", type=int, default=0, metavar="N")
    parser.add_argument("--output", default=str(WORK_DIR / "export-safegraph.csv"))
    parser.add_argument("--iso", action="append", metavar="CC")
    args = parser.parse_args()
    sys.exit(export(args))


if __name__ == "__main__":
    main()
