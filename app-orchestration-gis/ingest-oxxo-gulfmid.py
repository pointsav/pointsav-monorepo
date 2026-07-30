#!/usr/bin/env python3
"""
ingest-oxxo-gulfmid.py — OXXO stores in the Gulf coast corridor gap.

Coverage: lat 21-27°N, lon -100 to -96.5°W (Tampico, Cd. Victoria, Reynosa,
Matamoros, Monterrey east). This area was rate-limited during the main Phase 5
ingest and GULF-MID extension attempts. 6 sub-bboxes with aggressive back-off.

Appends unique records (dedup by rounded coord) to oxxo-mx.jsonl.
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

# 6 sub-regions covering the GULF-MID corridor
BBOXES = [
    ("GM-SW",  21.0, -100.0, 23.0,  -98.0),  # Tampico / Cd. Valles
    ("GM-SE-S",21.0,  -98.0, 23.0,  -96.5),  # Tampico east coast
    ("GM-SE-N",23.0,  -98.0, 24.0,  -96.5),  # Cd. Victoria area
    ("GM-CW",  23.0, -100.0, 24.0,  -98.0),  # Cd. Victoria west
    ("GM-NW",  24.0, -100.0, 27.0,  -98.0),  # Monterrey east / Linares
    ("GM-NE",  24.0,  -98.0, 27.0,  -96.5),  # Reynosa / Matamoros
]


def _query_bbox(lat_min, lon_min, lat_max, lon_max, timeout=120) -> list | None:
    """Returns element list or None if all URLs failed."""
    q = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["brand:wikidata"="{WIKIDATA_ID}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["brand:wikidata"="{WIKIDATA_ID}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f");\n"
        f"out center tags;\n"
    )
    for attempt, url in enumerate(OVERPASS_URLS):
        try:
            r = subprocess.run(
                ["curl", "-s", "--max-time", str(timeout + 60), "--data", f"data={q}", url],
                capture_output=True, text=True, timeout=timeout + 90,
            )
            if r.returncode != 0:
                print(f"    curl exit {r.returncode}, trying next")
                time.sleep(15)
                continue
            if not r.stdout.strip().startswith("{"):
                print(f"    rate limited on {url.split('/')[2]}, waiting 30s")
                time.sleep(30)
                continue
            return json.loads(r.stdout).get("elements", [])
        except (json.JSONDecodeError, subprocess.TimeoutExpired) as e:
            print(f"    error: {e}, trying next")
            time.sleep(10)
            continue
    return None


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
    # Load existing
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
                    seen.add((round(r["latitude"], 4), round(r["longitude"], 4)))
                    existing_count += 1
                except Exception:
                    pass
    print(f"Existing: {existing_count} records ({len(seen)} unique coords)")

    new_records = []
    failed = []

    for region, lat_min, lon_min, lat_max, lon_max in BBOXES:
        print(f"\n[{region}] {lat_min},{lon_min} → {lat_max},{lon_max}")
        # Up to 2 attempts per bbox
        elems = None
        for attempt in range(2):
            if attempt > 0:
                print(f"  retry {attempt+1}...")
                time.sleep(45)
            elems = _query_bbox(lat_min, lon_min, lat_max, lon_max)
            if elems is not None:
                break
        if elems is None:
            print(f"  FAILED — all URLs exhausted, skipping")
            failed.append(region)
            time.sleep(20)
            continue

        added = 0
        print(f"  {len(elems)} elements")
        for elem in elems:
            rec = elem_to_record(elem)
            if rec is None:
                continue
            key = (round(rec["latitude"], 4), round(rec["longitude"], 4))
            if key in seen:
                continue
            seen.add(key)
            new_records.append(rec)
            added += 1
        print(f"  {added} new unique")
        time.sleep(12)

    print(f"\nNew unique records: {len(new_records)}")
    if failed:
        print(f"FAILED regions: {failed} — re-run to retry")

    if new_records:
        with open(OUT_PATH, "a") as f:
            for rec in new_records:
                f.write(json.dumps(rec) + "\n")
        total = existing_count + len(new_records)
        print(f"Appended → {OUT_PATH}")
        print(f"Total OXXO: {total}")
    else:
        print("Nothing new written.")


if __name__ == "__main__":
    main()
