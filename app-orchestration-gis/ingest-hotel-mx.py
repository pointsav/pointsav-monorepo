#!/usr/bin/env python3
"""
ingest-hotel-mx.py — Mexican hotel brand ingests for PKS enrichment.

City Express (Q109329542) by brand:wikidata.
Fiesta Inn, One Hotels by name query (not tagged with brand:wikidata in OSM MX).

Usage:
    python3 ingest-hotel-mx.py               # all brands
    python3 ingest-hotel-mx.py --brands city-express-mx fiesta-inn-mx
    python3 ingest-hotel-mx.py --dry-run
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

OVERPASS_URLS = [
    "https://overpass-api.de/api/interpreter",
    "https://overpass.kumi.systems/api/interpreter",
    "https://overpass.private.coffee/api/interpreter",
]

# Mexico bbox (includes some southern US fringe — all brands below are MX-only chains)
MX_BBOX = (14.5, -117.1, 32.5, -86.7)

BRANDS = {
    "city-express-mx": {
        "wikidata_id": "Q109329542",
        "display_name": "City Express by Marriott",
        "canonical_name": "City Express by Marriott",
        "naics_code": "721110",
        "top_category": "Hotels and Motels",
        "sub_category": "Hotel",
        "_iso_fallback": "MX",
        "_query_type": "wikidata",
    },
    "fiesta-inn-mx": {
        "display_name": "Fiesta Inn",
        "canonical_name": "Fiesta Inn (Grupo Posadas)",
        "naics_code": "721110",
        "top_category": "Hotels and Motels",
        "sub_category": "Hotel",
        "_iso_fallback": "MX",
        "_query_type": "name",
        "_name_query": "Fiesta Inn",
    },
    "one-hotels-mx": {
        "display_name": "One Hotels",
        "canonical_name": "One by City Express",
        "naics_code": "721110",
        "top_category": "Hotels and Motels",
        "sub_category": "Hotel",
        "_iso_fallback": "MX",
        "_query_type": "name",
        "_name_query": "One Hotels",
    },
}


def overpass_query_wikidata(wikidata_id: str, bbox: tuple, timeout: int = 90) -> list:
    lat_min, lon_min, lat_max, lon_max = bbox
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["brand:wikidata"="{wikidata_id}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["brand:wikidata"="{wikidata_id}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f");\n"
        f"out center tags;\n"
    )
    return _run_query(query, timeout)


def overpass_query_name(name: str, bbox: tuple, timeout: int = 90) -> list:
    # Exact-match queries are dramatically faster than regex on Overpass.
    lat_min, lon_min, lat_max, lon_max = bbox
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["tourism"="hotel"]["name"="{name}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["tourism"="hotel"]["name"="{name}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  node["tourism"="hotel"]["brand"="{name}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["tourism"="hotel"]["brand"="{name}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f");\n"
        f"out center tags;\n"
    )
    return _run_query(query, timeout)


def _run_query(query: str, timeout: int) -> list:
    last_err = None
    for url in OVERPASS_URLS:
        try:
            result = subprocess.run(
                ["curl", "-s", "--max-time", str(timeout + 60), "--data", f"data={query}", url],
                capture_output=True, text=True, timeout=timeout + 90,
            )
            if result.returncode != 0:
                last_err = f"curl exit {result.returncode}"
                continue
            if not result.stdout.strip().startswith("{"):
                last_err = "empty/non-JSON response (rate limited?)"
                time.sleep(5)
                continue
            data = json.loads(result.stdout)
            return data.get("elements", [])
        except (json.JSONDecodeError, subprocess.TimeoutExpired) as e:
            last_err = str(e)
            continue
    raise RuntimeError(f"All Overpass instances failed. Last: {last_err}")


def elem_to_record(elem: dict, chain_id: str, brand: dict) -> dict | None:
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
        "chain_id": chain_id,
        "brand_wikidata": brand.get("wikidata_id"),
        "location_name": tags.get("name") or tags.get("brand") or brand["display_name"],
        "brands": [{"brand_name": brand["display_name"], "brand_wikidata": brand.get("wikidata_id")}],
        "street_address": (tags.get("addr:housenumber", "") + " " + tags.get("addr:street", "")).strip() or None,
        "city": tags.get("addr:city"),
        "region": tags.get("addr:state") or tags.get("addr:region"),
        "postal_code": tags.get("addr:postcode"),
        "iso_country_code": tags.get("addr:country") or brand.get("_iso_fallback"),
        "latitude": lat,
        "longitude": lon,
        "polygon_wkt": None,
        "naics_code": brand["naics_code"],
        "top_category": brand["top_category"],
        "sub_category": brand["sub_category"],
        "operating_status": "open",
        "source": "osm",
        "confidence": 0.85,
        "last_updated": "2026-06-14",
        "osm_element_type": elem["type"],
    }


def ingest_brand(chain_id: str, brand: dict, dry_run: bool = False) -> int:
    qt = brand.get("_query_type", "wikidata")
    if qt == "wikidata":
        print(f"\n[{chain_id}] wikidata={brand['wikidata_id']}")
        print(f"  querying MX bbox by brand:wikidata...")
        try:
            elements = overpass_query_wikidata(brand["wikidata_id"], MX_BBOX)
        except RuntimeError as e:
            print(f"  ERROR: {e}")
            return 0
    else:
        name = brand["_name_query"]
        print(f"\n[{chain_id}] name_query='{name}'")
        print(f"  querying MX bbox by name/brand tag...")
        try:
            elements = overpass_query_name(name, MX_BBOX)
        except RuntimeError as e:
            print(f"  ERROR: {e}")
            return 0

    print(f"  → {len(elements)} elements")
    time.sleep(3)

    records = []
    for elem in elements:
        rec = elem_to_record(elem, chain_id, brand)
        if rec:
            records.append(rec)

    deduped = []
    seen = set()
    for rec in records:
        key = (round(rec["latitude"], 4), round(rec["longitude"], 4))
        if key not in seen:
            deduped.append(rec)
            seen.add(key)

    dropped = len(records) - len(deduped)
    if dropped:
        print(f"  deduped: dropped {dropped} duplicates")
    print(f"  records to write: {len(deduped)}")

    if dry_run:
        print(f"  DRY RUN — would write {len(deduped)} to {OUT_DIR / (chain_id + '.jsonl')}")
        return len(deduped)

    if not deduped:
        print(f"  no records — skipping write")
        return 0

    out_path = OUT_DIR / f"{chain_id}.jsonl"
    with open(out_path, "w") as f:
        for rec in deduped:
            f.write(json.dumps(rec) + "\n")
    print(f"  wrote {len(deduped)} records → {out_path}")
    return len(deduped)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--brands", nargs="+", choices=list(BRANDS.keys()))
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    brands_to_run = args.brands or list(BRANDS.keys())
    total = 0
    for chain_id in brands_to_run:
        n = ingest_brand(chain_id, BRANDS[chain_id], dry_run=args.dry_run)
        total += n
        time.sleep(4)

    print(f"\nTotal records: {total}")


if __name__ == "__main__":
    main()
