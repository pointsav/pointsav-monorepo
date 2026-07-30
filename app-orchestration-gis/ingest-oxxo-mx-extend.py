#!/usr/bin/env python3
"""
ingest-oxxo-mx-extend.py — append OXXO stores from coverage gaps identified in Phase 5.

Gap analysis (2026-06-14): original 4-bbox ingest covered 6,427 stores but missed:
  GULF-MID:  lat 21-27°N, lon -100 to -96.5 (Tampico, Matamoros, Reynosa)  ~227
  GULF-N:    lat 27-32.5°N, lon -100 to -96.5 (north Tamaulipas)            ~23
  SIN-NAY:   lat 21.5-24°N, lon -107.5 to -104 (S Sinaloa, Nayarit)         ~43
  SOUTH:     lat 14.5-18°N, lon -96.5 to -91 (Oaxaca coast, Chiapas)       ~165

Total expected: ~458 additional stores.
Deduplicates by rounded (lat, lon) against existing oxxo-mx.jsonl before appending.
"""

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
OUT_PATH = OUT_DIR / f"{CHAIN_ID}.jsonl"

OVERPASS_URLS = [
    "https://overpass-api.de/api/interpreter",
    "https://overpass.kumi.systems/api/interpreter",
    "https://overpass.private.coffee/api/interpreter",
]

GAP_BBOXES = [
    ("GULF-MID", 21.0, -100.0, 27.0, -96.5),   # Tampico / Matamoros / Reynosa corridor
    ("GULF-N",   27.0, -100.0, 32.5, -96.5),   # Northern Tamaulipas + NE Coahuila
    ("SIN-NAY",  21.5, -107.5, 24.0, -104.0),  # Southern Sinaloa + Nayarit
    ("SOUTH",    14.5,  -96.5, 18.0,  -91.0),  # Oaxaca coast + interior Chiapas
]


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
    # Load existing records to build dedup set
    seen = set()
    existing_count = 0
    if OUT_PATH.exists():
        with open(OUT_PATH) as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    r = json.loads(line)
                    key = (round(r["latitude"], 4), round(r["longitude"], 4))
                    seen.add(key)
                    existing_count += 1
                except Exception:
                    pass
    print(f"Existing: {existing_count} records ({len(seen)} unique coords)")

    new_records = []
    for region, lat_min, lon_min, lat_max, lon_max in GAP_BBOXES:
        print(f"\n[oxxo-mx {region}] querying {lat_min},{lon_min},{lat_max},{lon_max}")
        query = (
            f"[out:json][timeout:120];\n"
            f"(\n"
            f'  node["brand:wikidata"="{WIKIDATA_ID}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
            f'  way["brand:wikidata"="{WIKIDATA_ID}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
            f");\n"
            f"out center tags;\n"
        )
        try:
            elements = _run_query(query)
        except RuntimeError as e:
            print(f"  ERROR: {e}")
            continue
        print(f"  → {len(elements)} elements")

        added = 0
        for elem in elements:
            rec = elem_to_record(elem)
            if rec is None:
                continue
            key = (round(rec["latitude"], 4), round(rec["longitude"], 4))
            if key in seen:
                continue
            seen.add(key)
            new_records.append(rec)
            added += 1
        print(f"  → {added} new unique records")
        time.sleep(6)

    print(f"\nNew unique records: {len(new_records)}")

    if not new_records:
        print("Nothing new — no changes to file.")
        return

    with open(OUT_PATH, "a") as f:
        for rec in new_records:
            f.write(json.dumps(rec) + "\n")
    total = existing_count + len(new_records)
    print(f"Appended → {OUT_PATH}")
    print(f"Total OXXO: {total}")


if __name__ == "__main__":
    main()
