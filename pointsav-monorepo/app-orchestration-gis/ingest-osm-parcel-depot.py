#!/usr/bin/env python3
"""
ingest-osm-parcel-depot.py — OSM Overpass → service-business/parcel-depot-osm.jsonl

Last-mile logistics / parcel-courier DEPOTS as an Urban Fringe (VWH) enrichment
signal. These sit on light-industrial estates near demand — a marker of the
just-in-time delivery land-use the archetype targets. Tag-based (not brand-keyed),
because the carrier landscape is fragmented across many regional couriers.

Targets (depots/sorting on industrial land — NOT residential parcel lockers):
  - amenity=post_depot           (Royal Mail / Deutsche Post delivery offices, sorting depots)
  - office=logistics             (carrier logistics offices on estates)
  - landuse=industrial nodes tagged with courier brands are NOT queried (too noisy)

Deliberately EXCLUDED (anti-thesis — residential/retail convenience points):
  - amenity=parcel_locker, parcel_pickup, vending=parcel_pickup

Output schema matches the service-business chain JSONL the VWH build reads
(latitude/longitude/iso_country_code/location_name/city), so it can be referenced
as a single pseudo-chain `parcel-depot-osm` under a VWH `parcel_depot` category.

Usage:
    python3 ingest-osm-parcel-depot.py --countries US DE GB
    python3 ingest-osm-parcel-depot.py --all
    python3 ingest-osm-parcel-depot.py --all --replace
"""

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import TOTEBOX_DATA_PATH

OVERPASS_URLS = [
    "https://overpass.private.coffee/api/interpreter",
    "https://overpass.kumi.systems/api/interpreter",
    "https://overpass-api.de/api/interpreter",
]

OUTPUT_FILE = TOTEBOX_DATA_PATH / "service-fs" / "service-business" / "parcel-depot-osm.jsonl"

COUNTRY_BBOX = {
    "US": (24.0, -125.0,  50.0,  -65.0),
    "CA": (41.0, -141.0,  84.0,  -52.0),
    "MX": (14.5,  -118.0,  33.0,  -86.5),
    "GB": (49.5,   -8.5,   61.5,    2.0),
    "FR": (41.0,   -5.5,   51.5,   10.0),
    "DE": (47.0,    6.0,   55.5,   15.5),
    "ES": (35.0,   -9.5,   44.0,    4.5),
    "IT": (36.0,    6.5,   47.5,   18.5),
    "AT": (46.5,    9.5,   49.0,   17.2),
    "NL": (50.7,    3.3,   53.7,    7.3),
    "BE": (49.4,    2.5,   51.6,    6.5),
    "PL": (49.0,   14.0,   55.0,   24.5),
    "SE": (55.0,   10.5,   69.5,   24.5),
    "NO": (57.0,    3.5,   71.5,   31.5),
    "DK": (54.5,    7.5,   58.0,   15.5),
    "FI": (59.5,   19.5,   70.5,   31.5),
    "PT": (36.5,   -9.5,   42.5,   -6.0),
}


def overpass_query(bbox: tuple, timeout: int = 180) -> list:
    lat_min, lon_min, lat_max, lon_max = bbox
    bb = f"{lat_min},{lon_min},{lat_max},{lon_max}"
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["amenity"="post_depot"]({bb});\n'
        f'  way["amenity"="post_depot"]({bb});\n'
        f'  node["office"="logistics"]({bb});\n'
        f'  way["office"="logistics"]({bb});\n'
        f");\n"
        f"out center;\n"
    )
    last_err = None
    for attempt in range(3):
        for url in OVERPASS_URLS:
            try:
                result = subprocess.run(
                    ["curl", "-s", "--max-time", str(timeout + 60),
                     "--data-urlencode", f"data={query}", url],
                    capture_output=True, text=True, timeout=timeout + 90,
                )
                if result.returncode != 0:
                    last_err = f"curl exit {result.returncode}: {result.stderr[:200]}"
                    continue
                data = json.loads(result.stdout)
                return data.get("elements", [])
            except (json.JSONDecodeError, subprocess.TimeoutExpired) as e:
                last_err = str(e)
                continue
        if attempt < 2:
            time.sleep(45)
    raise RuntimeError(f"All Overpass instances failed. Last error: {last_err}")


def element_to_record(elem: dict, iso: str) -> dict | None:
    if elem["type"] == "node":
        lat, lon = elem.get("lat"), elem.get("lon")
    else:
        center = elem.get("center", {})
        lat, lon = center.get("lat"), center.get("lon")
    if lat is None or lon is None:
        return None
    tags = elem.get("tags", {})
    kind = "post_depot" if tags.get("amenity") == "post_depot" else "logistics"
    return {
        "placekey":          None,
        "category_id":       "parcel_depot",
        "overture_id":       None,
        "location_name":     (tags.get("name") or tags.get("operator") or "parcel depot").strip(),
        "city":              tags.get("addr:city") or tags.get("addr:town"),
        "region":            tags.get("addr:state") or tags.get("addr:province"),
        "postal_code":       tags.get("addr:postcode"),
        "iso_country_code":  iso,
        "latitude":          round(float(lat), 7),
        "longitude":         round(float(lon), 7),
        "polygon_wkt":       None,
        "naics_code":        "492110",
        "top_category":      "Couriers and Express Delivery",
        "sub_category":      "Parcel Depot",
        "operating_status":  "open",
        "source":            "osm",
        "confidence":        0.75,
        "last_updated":      "2026-06-03",
        "depot_kind":        kind,
        "operator":          tags.get("operator"),
        "brand":             tags.get("brand"),
        "wikidata_id":       tags.get("brand:wikidata") or tags.get("wikidata"),
    }


def ingest_country(iso: str, bbox: tuple) -> list:
    print(f"  [{iso}] post_depot + office=logistics bbox={bbox}")
    try:
        elements = overpass_query(bbox)
    except (RuntimeError, OSError) as e:
        print(f"    ERROR: {e}")
        return []
    print(f"    OSM elements returned: {len(elements)}")
    records = [r for r in (element_to_record(e, iso) for e in elements) if r]
    deduped, seen = [], set()
    for rec in records:
        key = (round(rec["latitude"], 3), round(rec["longitude"], 3))
        if key not in seen:
            deduped.append(rec)
            seen.add(key)
    print(f"    {len(records) - len(deduped)} dupes → {len(deduped)} depots")
    return deduped


def main():
    parser = argparse.ArgumentParser(description="Ingest OSM parcel/courier depots (VWH enrichment)")
    parser.add_argument("--countries", nargs="+", metavar="ISO")
    parser.add_argument("--all",     action="store_true")
    parser.add_argument("--replace", action="store_true")
    parser.add_argument("--delay", type=float, default=6.0)
    args = parser.parse_args()

    if args.all:
        countries = list(COUNTRY_BBOX.keys())
    elif args.countries:
        countries = args.countries
    else:
        parser.print_help(); sys.exit(1)

    invalid = [c for c in countries if c not in COUNTRY_BBOX]
    if invalid:
        print(f"ERROR: unknown country codes: {invalid}"); sys.exit(1)

    all_records: list[dict] = []
    first = True
    for iso in countries:
        if not first:
            time.sleep(args.delay)
        first = False
        all_records.extend(ingest_country(iso, COUNTRY_BBOX[iso]))

    print(f"\nTotal parcel depots: {len(all_records)}")
    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    mode = "w" if args.replace else "a"
    with open(OUTPUT_FILE, mode) as f:
        for rec in all_records:
            f.write(json.dumps(rec) + "\n")
    print(f"{'Written' if args.replace else 'Appended'} → {OUTPUT_FILE}")

    by_iso: dict[str, int] = {}
    for r in all_records:
        by_iso[r["iso_country_code"]] = by_iso.get(r["iso_country_code"], 0) + 1
    print("\nBy country:")
    for iso, n in sorted(by_iso.items(), key=lambda x: -x[1]):
        print(f"  {iso}: {n}")


if __name__ == "__main__":
    main()
