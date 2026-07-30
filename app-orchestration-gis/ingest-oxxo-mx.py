#!/usr/bin/env python3
"""
ingest-oxxo-mx.py — OXXO convenience store ingest for PKS enrichment.

OXXO (brand:wikidata Q1342538) has ~22,000 locations in Mexico and is co-located
at virtually every ADO bus terminal. It is the highest-precision CONVENIENCE signal
for Mexican transit co-location.

The ingest splits Mexico into 4 sub-bboxes to avoid Overpass timeout on large queries.
All records get _iso_fallback='MX' since OSM MX nodes rarely carry addr:country.

Usage:
    python3 ingest-oxxo-mx.py               # all sub-regions
    python3 ingest-oxxo-mx.py --dry-run     # count without writing
"""

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import SERVICE_BUSINESS as _SB_CONFIG

_TOTEBOX = _SB_CONFIG.parent
OUT_DIR = _TOTEBOX / "service-fs" / "service-business"
if not OUT_DIR.exists():
    OUT_DIR = _SB_CONFIG

WIKIDATA_ID = "Q1342538"
CHAIN_ID = "oxxo-mx"
DISPLAY_NAME = "OXXO"

OVERPASS_URLS = [
    "https://overpass-api.de/api/interpreter",
    "https://overpass.kumi.systems/api/interpreter",
    "https://overpass.private.coffee/api/interpreter",
]

# Split into 4 quadrants to avoid timeout on 22,000-record query
MX_BBOXES = [
    ("NW", 24.0, -118.4, 32.7, -104.0),  # Baja, Sonora, Chihuahua, Sinaloa (north)
    ("NE", 24.0, -104.0, 32.7, -86.7),   # Coahuila, Nuevo León, Tamaulipas (north)
    ("SW", 14.5, -118.4, 24.0, -104.0),  # Jalisco, Nayarit, Colima, Michoacán, Guerrero
    ("SE", 14.5, -104.0, 24.0, -86.7),   # CDMX, Veracruz, Oaxaca, Yucatán, Quintana Roo
]


def _run_query(query: str, timeout: int) -> list:
    last_err = None
    for url in OVERPASS_URLS:
        try:
            result = subprocess.run(
                ["curl", "-s", "--max-time", str(timeout + 60),
                 "--data", f"data={query}", url],
                capture_output=True, text=True, timeout=timeout + 90,
            )
            if result.returncode != 0:
                last_err = f"curl exit {result.returncode}"
                time.sleep(5)
                continue
            if not result.stdout.strip().startswith("{"):
                last_err = "empty/non-JSON response (rate limited?)"
                time.sleep(10)
                continue
            data = json.loads(result.stdout)
            return data.get("elements", [])
        except (json.JSONDecodeError, subprocess.TimeoutExpired) as e:
            last_err = str(e)
            time.sleep(5)
            continue
    raise RuntimeError(f"All Overpass instances failed. Last: {last_err}")


def query_bbox(region: str, lat_min: float, lon_min: float, lat_max: float, lon_max: float, timeout: int = 150) -> list:
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["brand:wikidata"="{WIKIDATA_ID}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["brand:wikidata"="{WIKIDATA_ID}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f");\n"
        f"out center tags;\n"
    )
    return _run_query(query, timeout)


def elem_to_record(elem: dict) -> dict | None:
    if elem["type"] == "node":
        lat, lon = elem.get("lat"), elem.get("lon")
    else:
        center = elem.get("center", {})
        lat, lon = center.get("lat"), center.get("lon")
    if lat is None or lon is None:
        return None
    tags = elem.get("tags", {})
    return {
        "placekey": None,
        "chain_id": CHAIN_ID,
        "brand_wikidata": WIKIDATA_ID,
        "location_name": tags.get("name") or tags.get("brand") or DISPLAY_NAME,
        "brands": [{"brand_name": DISPLAY_NAME, "brand_wikidata": WIKIDATA_ID}],
        "street_address": (tags.get("addr:housenumber", "") + " " + tags.get("addr:street", "")).strip() or None,
        "city": tags.get("addr:city"),
        "region": tags.get("addr:state") or tags.get("addr:region"),
        "postal_code": tags.get("addr:postcode"),
        "iso_country_code": tags.get("addr:country") or "MX",
        "latitude": lat,
        "longitude": lon,
        "polygon_wkt": None,
        "naics_code": "445131",
        "top_category": "Convenience Stores",
        "sub_category": "Convenience Store",
        "operating_status": "open",
        "source": "osm",
        "confidence": 0.90,
        "last_updated": "2026-06-14",
        "osm_element_type": elem["type"],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    all_records = []
    seen = set()

    for region, lat_min, lon_min, lat_max, lon_max in MX_BBOXES:
        print(f"\n[oxxo-mx {region}] querying {lat_min},{lon_min},{lat_max},{lon_max}")
        try:
            elements = query_bbox(region, lat_min, lon_min, lat_max, lon_max)
        except RuntimeError as e:
            print(f"  ERROR: {e}")
            continue
        print(f"  → {len(elements)} elements")

        for elem in elements:
            rec = elem_to_record(elem)
            if rec is None:
                continue
            key = (round(rec["latitude"], 4), round(rec["longitude"], 4))
            if key in seen:
                continue
            seen.add(key)
            all_records.append(rec)

        time.sleep(5)

    print(f"\nTotal unique OXXO records: {len(all_records)}")

    if args.dry_run:
        print("DRY RUN — no file written")
        return

    if not all_records:
        print("No records — skipping write")
        return

    out_path = OUT_DIR / f"{CHAIN_ID}.jsonl"
    with open(out_path, "w") as f:
        for rec in all_records:
            f.write(json.dumps(rec) + "\n")
    print(f"Wrote {len(all_records)} records → {out_path}")


if __name__ == "__main__":
    main()
