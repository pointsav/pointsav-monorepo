#!/usr/bin/env python3
"""
enrich_university_enrollment.py — IPEDS HD2023 enrollment enrichment

Downloads IPEDS HD2023.csv (US university directory from NCES), fuzzy-matches
US university names against cleansed-civic-osm.jsonl, and enriches records with:
  - enrollment: 4-year IPEDS enrollment count
  - university_tier: "regional" (>=5000), "small" (1000-4999), "excluded" (<1000)

Non-US universities use the name-based heuristic already applied at ingest.
Runs in-place on the JSONL (replaces file).

Usage:
    python3 enrich_university_enrollment.py [--dry-run]
"""

import argparse
import csv
import io
import json
import re
import sys
import urllib.request
import zipfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import TOTEBOX_DATA_PATH

IPEDS_HD_URL = "https://nces.ed.gov/ipeds/datacenter/data/HD2023.zip"
IPEDS_EF_URL = "https://nces.ed.gov/ipeds/datacenter/data/EF2023A.zip"

OSM_CIVIC_FILE = TOTEBOX_DATA_PATH / "service-places" / "cleansed-civic-osm.jsonl"

# Fuzzy match — strip common suffixes before comparing
_STRIP_SUFFIXES = re.compile(
    r"\s+(university|college|institute|school|of technology|polytechnic|campus|system)$",
    re.IGNORECASE,
)


def _normalize(name: str) -> str:
    n = name.lower().strip()
    n = re.sub(r"[^a-z0-9 ]", "", n)
    n = _STRIP_SUFFIXES.sub("", n)
    n = re.sub(r"\s+", " ", n).strip()
    return n


def download_ipeds() -> dict[str, int]:
    """Download IPEDS HD (names) and EF (enrollment) and return {normalized_name: enrollment}."""
    print(f"Downloading IPEDS HD2023 (directory) ...")
    try:
        with urllib.request.urlopen(IPEDS_HD_URL, timeout=120) as resp:
            hd_data = resp.read()
    except Exception as e:
        print(f"  WARNING: IPEDS HD download failed ({e}). Skipping enrichment.")
        return {}

    print(f"Downloading IPEDS EF2023A (enrollment) ...")
    try:
        with urllib.request.urlopen(IPEDS_EF_URL, timeout=120) as resp:
            ef_data = resp.read()
    except Exception as e:
        print(f"  WARNING: IPEDS EF download failed ({e}). Skipping enrichment.")
        return {}

    # 1. Map UNITID -> Name from HD
    id_to_name: dict[str, str] = {}
    with zipfile.ZipFile(io.BytesIO(hd_data)) as z:
        csv_name = next((n for n in z.namelist() if n.lower().endswith(".csv")), None)
        if csv_name:
            with z.open(csv_name) as f:
                reader = csv.DictReader(io.TextIOWrapper(f, encoding="utf-8-sig"))
                for row in reader:
                    # ICLEVEL=1 → 4-year institutions only
                    if row.get("ICLEVEL", "").strip() == "1":
                        id_to_name[row["UNITID"]] = row["INSTNM"]

    # 2. Map Name -> Enrollment from EF (joined via UNITID)
    lookup: dict[str, int] = {}
    with zipfile.ZipFile(io.BytesIO(ef_data)) as z:
        csv_name = next((n for n in z.namelist() if n.lower().endswith(".csv")), None)
        if csv_name:
            with z.open(csv_name) as f:
                reader = csv.DictReader(io.TextIOWrapper(f, encoding="utf-8-sig"))
                for row in reader:
                    unitid = row.get("UNITID")
                    if unitid not in id_to_name:
                        continue
                    # EF2023A uses EFTOTLT for Fall Total Enrollment (EFYTOTLT is 12-month)
                    enroll_raw = row.get("EFTOTLT", "").strip()
                    if not enroll_raw:
                        continue
                    try:
                        enroll = int(enroll_raw)
                    except ValueError:
                        continue
                    
                    name = id_to_name[unitid]
                    key = _normalize(name)
                    if key:
                        # Use max in case of multiple rows per UNITID (though EF2023A should be unique per UNITID+EFALEVEL)
                        # EFALEVEL=1 is "All students total"
                        if row.get("EFALEVEL", "").strip() == "1":
                            lookup[key] = max(lookup.get(key, 0), enroll)

    print(f"  Loaded {len(lookup)} 4-year US institutions from IPEDS (joined HD+EF).")
    return lookup


def _tier_from_enrollment(enrollment: int) -> str:
    if enrollment >= 5000:
        return "regional"
    if enrollment >= 1000:
        return "small"
    return "excluded"


def enrich(dry_run: bool = False) -> None:
    if not OSM_CIVIC_FILE.exists():
        print(f"ERROR: {OSM_CIVIC_FILE} not found. Run ingest-osm-civic.py first.")
        sys.exit(1)

    ipeds = download_ipeds()
    if not ipeds:
        print("No IPEDS data. Skipping enrichment (university_tier values unchanged).")
        return

    records = []
    with open(OSM_CIVIC_FILE) as f:
        for line in f:
            try:
                records.append(json.loads(line))
            except json.JSONDecodeError:
                pass

    enriched = 0
    reclassified = 0
    for r in records:
        if r.get("category_id") != "university":
            continue
        if r.get("iso_country_code") != "US":
            continue
        key = _normalize(r.get("location_name", ""))
        if not key or key not in ipeds:
            continue
        enrollment = ipeds[key]
        new_tier   = _tier_from_enrollment(enrollment)
        old_tier   = r.get("university_tier")
        r["enrollment"]     = enrollment
        r["university_tier"] = new_tier
        enriched += 1
        if old_tier != new_tier:
            reclassified += 1

    print(f"  Enriched {enriched} US universities with IPEDS enrollment.")
    print(f"  Reclassified {reclassified} records (tier changed).")

    # Drop "excluded" universities surfaced by IPEDS (enrollment < 1000)
    before = len(records)
    records = [
        r for r in records
        if not (r.get("category_id") == "university" and r.get("university_tier") == "excluded")
    ]
    dropped = before - len(records)
    if dropped:
        print(f"  Dropped {dropped} universities with enrollment < 1000.")

    if dry_run:
        print("  Dry run — not writing changes.")
        return

    with open(OSM_CIVIC_FILE, "w") as f:
        for r in records:
            f.write(json.dumps(r) + "\n")
    print(f"  Written → {OSM_CIVIC_FILE} ({len(records)} records)")


def main():
    parser = argparse.ArgumentParser(description="Enrich university records with IPEDS enrollment")
    parser.add_argument("--dry-run", action="store_true", help="Report changes without writing")
    args = parser.parse_args()
    enrich(dry_run=args.dry_run)


if __name__ == "__main__":
    main()
