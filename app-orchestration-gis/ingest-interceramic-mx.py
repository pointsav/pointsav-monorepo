#!/usr/bin/env python3
"""
ingest-interceramic-mx.py — Interceramic flooring warehouse ingest for VWH enrichment.

Interceramic (Internacional de Cerámica, S.A.B. de C.V.) operates ~150 ceramic tile
and flooring warehouse stores in Mexico. OSM brand:wikidata is Q113203429 (differs from
canonical Wikidata Q6040285 for parent company). Name query "Interceramic" is used as
primary fallback since many MX stores are not tagged with brand:wikidata.

Usage:
    python3 ingest-interceramic-mx.py               # full ingest
    python3 ingest-interceramic-mx.py --dry-run     # count without writing
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

# OSM uses Q113203429; canonical Wikidata for parent company is Q6040285
OSM_WIKIDATA_ID = "Q113203429"
CHAIN_ID = "interceramic-mx"
DISPLAY_NAME = "Interceramic"
NAICS_CODE = "444190"

OVERPASS_URLS = [
    "https://overpass-api.de/api/interpreter",
    "https://overpass.kumi.systems/api/interpreter",
    "https://overpass.private.coffee/api/interpreter",
]

# Mexico bbox (lat_min, lon_min, lat_max, lon_max)
MX_BBOX = (14.5, -117.1, 32.5, -86.7)


def _run_query(query: str, timeout: int = 120) -> list:
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
                last_err = "empty/non-JSON response"
                time.sleep(10)
                continue
            data = json.loads(result.stdout)
            return data.get("elements", [])
        except (json.JSONDecodeError, subprocess.TimeoutExpired) as e:
            last_err = str(e)
            time.sleep(5)
            continue
    raise RuntimeError(f"All Overpass instances failed. Last: {last_err}")


def query_by_wikidata(bbox: tuple, timeout: int = 120) -> list:
    lat_min, lon_min, lat_max, lon_max = bbox
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["brand:wikidata"="{OSM_WIKIDATA_ID}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["brand:wikidata"="{OSM_WIKIDATA_ID}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f");\n"
        f"out center tags;\n"
    )
    return _run_query(query, timeout)


def query_by_name(bbox: tuple, timeout: int = 120) -> list:
    lat_min, lon_min, lat_max, lon_max = bbox
    query = (
        f'[out:json][timeout:{timeout}];\n'
        f'(\n'
        f'  node["name"~"Interceramic",i]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["name"~"Interceramic",i]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f');\n'
        f'out center tags;\n'
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
    name = tags.get("name") or tags.get("brand") or DISPLAY_NAME
    # Filter name-query results: only keep Interceramic stores (exclude unrelated matches)
    if "interceramic" not in name.lower():
        return None
    return {
        "placekey": None,
        "chain_id": CHAIN_ID,
        "brand_wikidata": OSM_WIKIDATA_ID,
        "location_name": name,
        "brands": [{"brand_name": DISPLAY_NAME, "brand_wikidata": OSM_WIKIDATA_ID}],
        "street_address": (tags.get("addr:housenumber", "") + " " + tags.get("addr:street", "")).strip() or None,
        "city": tags.get("addr:city"),
        "region": tags.get("addr:state") or tags.get("addr:region"),
        "postal_code": tags.get("addr:postcode"),
        "iso_country_code": tags.get("addr:country") or "MX",
        "latitude": lat,
        "longitude": lon,
        "polygon_wkt": None,
        "naics_code": NAICS_CODE,
        "top_category": "Building Material and Garden Equipment and Supplies Dealers",
        "sub_category": "Contractor Flooring Warehouse (VWH signal)",
        "operating_status": "open",
        "source": "osm",
        "confidence": 0.88,
        "last_updated": "2026-06-15",
        "osm_element_type": elem["type"],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    print(f"[interceramic-mx] querying OSM by brand:wikidata={OSM_WIKIDATA_ID}")
    wikidata_elements = query_by_wikidata(MX_BBOX)
    print(f"  → {len(wikidata_elements)} elements from wikidata query")

    time.sleep(5)
    print(f"  Querying by name 'Interceramic' (many MX stores lack brand:wikidata tag)")
    name_elements = query_by_name(MX_BBOX)
    print(f"  → {len(name_elements)} elements from name query")

    all_elements = wikidata_elements + name_elements

    records = []
    seen: set = set()
    for elem in all_elements:
        rec = elem_to_record(elem)
        if rec is None:
            continue
        key = (round(rec["latitude"], 4), round(rec["longitude"], 4))
        if key in seen:
            continue
        seen.add(key)
        records.append(rec)

    print(f"\nTotal unique Interceramic records: {len(records)}")

    if args.dry_run:
        print("DRY RUN — no file written")
        return

    if not records:
        print("No records — skipping write")
        return

    out_path = OUT_DIR / f"{CHAIN_ID}.jsonl"
    with open(out_path, "w") as f:
        for rec in records:
            f.write(json.dumps(rec) + "\n")
    print(f"Wrote {len(records)} records → {out_path}")


if __name__ == "__main__":
    main()
