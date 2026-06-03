#!/usr/bin/env python3
"""
ingest-osm-parking.py — OSM Overpass → cleansed-civic-parking.jsonl

Ingests the parking layer used to test the Commuter (park-and-ride) thesis:
"a regional airport or the outer terminus of a commuter rail line that does NOT
yet have the parking structure a major metro hub already has."

To classify each Commuter candidate we need to know, per location, whether a
built parking STRUCTURE already exists nearby, or only surface parking (or
nothing). So we ingest two relevant subsets of amenity=parking — NOT all
surface parking (which is millions of nodes and would overwhelm Overpass):

  1. Park-and-ride lots          park_ride=yes|train|bus|metro|tram|ferry
  2. Structured / built parkades parking=multi-storey|underground|garage_boxes

The downstream join (in build-pks-clusters.py / test-cluster-archetypes.py)
then labels each candidate terminus:
  - within ~800 m of a multi-storey/underground parkade   → BUILT  (exclude / down-weight)
  - within ~800 m of a surface park_ride only             → PARTIAL (develop-up opportunity)
  - nothing nearby                                        → GREENFIELD (prime opportunity)

Writes to service-places/cleansed-civic-parking.jsonl (appends; --replace to overwrite).

Usage:
    python3 ingest-osm-parking.py --countries US DE GB
    python3 ingest-osm-parking.py --all
    python3 ingest-osm-parking.py --all --replace
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

OUTPUT_FILE = TOTEBOX_DATA_PATH / "service-places" / "cleansed-civic-parking.jsonl"

# Same country bounding boxes as ingest-osm-railway-commuter.py (kept in sync).
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
    "GR": (34.5,   19.5,   42.0,   26.5),
    "PT": (36.5,   -9.5,   42.5,   -6.0),
    "CH": (45.8,    5.9,   47.8,   10.5),
    "CZ": (48.5,   12.1,   51.1,   18.9),
    "HU": (45.7,   16.1,   48.6,   22.9),
    "RO": (43.6,   20.2,   48.3,   30.0),
}

# Structured-parking values that mean a parkade is already BUILT.
BUILT_STRUCTURE = {"multi-storey", "underground", "garage_boxes", "rooftop"}


def overpass_query(bbox: tuple, timeout: int = 180) -> list:
    lat_min, lon_min, lat_max, lon_max = bbox
    bb = f"{lat_min},{lon_min},{lat_max},{lon_max}"
    # Only the two relevant subsets: park-and-ride lots, and structured parking.
    # This keeps the result set bounded (surface parking generally is excluded
    # unless it is explicitly tagged park_ride).
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["amenity"="parking"]["park_ride"]({bb});\n'
        f'  way["amenity"="parking"]["park_ride"]({bb});\n'
        f'  node["amenity"="parking"]["parking"~"multi-storey|underground|garage_boxes|rooftop"]({bb});\n'
        f'  way["amenity"="parking"]["parking"~"multi-storey|underground|garage_boxes|rooftop"]({bb});\n'
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


def parking_class(tags: dict) -> str:
    """built = structured parkade exists; park_ride = surface P+R; other = surface/unknown."""
    parking = (tags.get("parking") or "").lower()
    if parking in BUILT_STRUCTURE:
        return "built"
    if tags.get("park_ride") and tags.get("park_ride").lower() not in ("no",):
        return "park_ride"
    return "surface"


def element_to_record(elem: dict, iso: str) -> dict | None:
    if elem["type"] == "node":
        lat, lon = elem.get("lat"), elem.get("lon")
    else:
        center = elem.get("center", {})
        lat, lon = center.get("lat"), center.get("lon")
    if lat is None or lon is None:
        return None

    tags = elem.get("tags", {})
    cls = parking_class(tags)
    capacity = tags.get("capacity")
    try:
        capacity = int(capacity) if capacity is not None else None
    except (ValueError, TypeError):
        capacity = None

    return {
        "placekey":          None,
        "category_id":       "parking",
        "overture_id":       None,
        "location_name":     (tags.get("name") or "parking").strip(),
        "iso_country_code":  iso,
        "latitude":          round(float(lat), 7),
        "longitude":         round(float(lon), 7),
        "polygon_wkt":       None,
        "top_category":      "Parking",
        "sub_category":      "Park and Ride" if cls == "park_ride" else (
                              "Parking Structure" if cls == "built" else "Parking Lot"),
        "operating_status":  "open",
        "source":            "osm",
        "confidence":        0.80,
        "last_updated":      "2026-06-03",
        "parking_class":     cls,            # built | park_ride | surface
        "park_ride":         tags.get("park_ride"),
        "parking_form":      tags.get("parking"),
        "capacity":          capacity,
        "access":            tags.get("access"),
        "operator":          tags.get("operator"),
        "network":           tags.get("network"),
        "wikidata_id":       tags.get("wikidata"),
    }


def ingest_country(iso: str, bbox: tuple) -> list:
    print(f"  [{iso}] amenity=parking (park_ride + structured) bbox={bbox}")
    try:
        elements = overpass_query(bbox)
    except (RuntimeError, OSError) as e:
        print(f"    ERROR: {e}")
        return []

    print(f"    OSM elements returned: {len(elements)}")
    records = []
    for elem in elements:
        rec = element_to_record(elem, iso)
        if rec:
            records.append(rec)

    # Coordinate de-dup (~110 m)
    deduped, seen = [], set()
    for rec in records:
        key = (round(rec["latitude"], 3), round(rec["longitude"], 3))
        if key not in seen:
            deduped.append(rec)
            seen.add(key)

    by_class: dict[str, int] = {}
    for r in deduped:
        by_class[r["parking_class"]] = by_class.get(r["parking_class"], 0) + 1
    print(f"    {len(records) - len(deduped)} dupes → {len(deduped)} lots; by class: {by_class}")
    return deduped


def main():
    parser = argparse.ArgumentParser(
        description="Ingest OSM park-and-ride + structured parking for the Commuter thesis"
    )
    parser.add_argument("--countries", nargs="+", metavar="ISO")
    parser.add_argument("--all",     action="store_true")
    parser.add_argument("--replace", action="store_true",
                        help="Overwrite output file instead of appending")
    parser.add_argument("--delay", type=float, default=6.0)
    args = parser.parse_args()

    if args.all:
        countries = list(COUNTRY_BBOX.keys())
    elif args.countries:
        countries = args.countries
    else:
        parser.print_help()
        sys.exit(1)

    invalid = [c for c in countries if c not in COUNTRY_BBOX]
    if invalid:
        print(f"ERROR: unknown country codes: {invalid}")
        print(f"Known: {sorted(COUNTRY_BBOX.keys())}")
        sys.exit(1)

    all_records: list[dict] = []
    first = True
    for iso in countries:
        if not first:
            time.sleep(args.delay)
        first = False
        all_records.extend(ingest_country(iso, COUNTRY_BBOX[iso]))

    print(f"\nTotal parking lots: {len(all_records)}")

    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    mode = "w" if args.replace else "a"
    action = "Written" if args.replace else "Appended"
    with open(OUTPUT_FILE, mode) as f:
        for rec in all_records:
            f.write(json.dumps(rec) + "\n")
    print(f"{action} → {OUTPUT_FILE}")

    by_iso: dict[str, int] = {}
    by_class: dict[str, int] = {}
    for r in all_records:
        by_iso[r["iso_country_code"]] = by_iso.get(r["iso_country_code"], 0) + 1
        by_class[r["parking_class"]] = by_class.get(r["parking_class"], 0) + 1

    print("\nBy country:")
    for iso, n in sorted(by_iso.items(), key=lambda x: -x[1]):
        print(f"  {iso}: {n}")
    print("\nBy parking class:")
    for cls, n in sorted(by_class.items(), key=lambda x: -x[1]):
        print(f"  {cls}: {n}")

    print("""
Next steps:
  1. Join cleansed-civic-parking.jsonl to the geometric Commuter candidates:
     for each terminus/regional-airport candidate, find the nearest parking lot.
       - within ~800 m of a parking_class=built lot   → BUILT      (exclude / down-weight)
       - within ~800 m of a park_ride lot only         → PARTIAL    (develop-up opportunity)
       - nothing within ~800 m                         → GREENFIELD (prime opportunity)
  2. Use that label as a geometric tier input (GREENFIELD > PARTIAL > BUILT).
  3. Re-run the Commuter build → archetype-pks.geojson; copy to www/data/; sync gateway.
""")


if __name__ == "__main__":
    main()
