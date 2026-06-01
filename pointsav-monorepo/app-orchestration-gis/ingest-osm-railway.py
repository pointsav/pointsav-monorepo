#!/usr/bin/env python3
"""
ingest-osm-railway.py — OSM Overpass → cleansed-civic-railway.jsonl

Queries Overpass for intercity / regional railway stations by country
bounding box, for the PKS (Parking Structures) archetype. The target is
stations where a traveller would drive, park, and take a 30–150 km train
ride to a major metro — NOT subway/metro stops, trams, or light rail.

Two-step filter:
  1. Overpass: railway=station, excluding station=subway|light_rail|tram|monorail
  2. Python: keep stations operated by national intercity rail operators;
     drop stations whose operator is a known metro/urban transit operator.
     Stations with no operator tag are kept (often valid, just untagged).

MX and IS are skipped — no intercity passenger rail.

Writes to service-places/cleansed-civic-railway.jsonl.

Usage:
    python3 ingest-osm-railway.py --countries US DE FR
    python3 ingest-osm-railway.py --all
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

OUTPUT_FILE = TOTEBOX_DATA_PATH / "service-places" / "cleansed-civic-railway.jsonl"

COUNTRY_BBOX = {
    "US": (24.0,  -125.0,  50.0,  -65.0),
    "CA": (41.0,  -141.0,  84.0,  -52.0),
    "ES": (35.0,   -9.5,   44.0,    4.5),
    "IT": (36.0,    6.5,   47.5,   18.5),
    "GR": (34.5,   19.5,   42.0,   26.5),
    "PL": (49.0,   14.0,   55.0,   24.5),
    "SE": (55.0,   10.5,   69.5,   24.5),
    "NO": (57.0,    3.5,   71.5,   31.5),
    "DK": (54.5,    7.5,   58.0,   15.5),
    "FI": (59.5,   19.5,   70.5,   31.5),
    "FR": (41.0,   -5.5,   51.5,   10.0),
    "DE": (47.0,    6.0,   55.5,   15.5),
    "GB": (49.5,   -8.5,   61.5,    2.0),
    "AT": (46.5,    9.5,   49.0,   17.2),
    "NL": (50.7,    3.3,   53.7,    7.3),
    "PT": (36.5,   -9.5,   42.5,   -6.0),
    # MX and IS intentionally absent — no intercity passenger rail
}

# Known national intercity operators (substring match, case-insensitive).
# None = accept all non-urban stations (GB: all Network Rail TOCs qualify).
INTERCITY_OPERATORS = {
    "US": ["amtrak"],
    "CA": ["via rail", "via"],
    "GB": None,
    "FR": ["sncf", "ter"],
    "DE": ["deutsche bahn", "db ", "db regio", "db fernverkehr", "db netz"],
    "ES": ["renfe", "adif"],
    "IT": ["trenitalia", "italo", "ferrovie dello stato", "rfi"],
    "AT": ["öbb", "obb", "österreichische bundesbahnen"],
    "NL": ["ns", "nederlandse spoorwegen", "prorail"],
    "SE": ["sj", "norrtåg", "trafikverket"],
    "DK": ["dsb", "banedanmark"],
    "NO": ["vy", "nsb", "bane nor"],
    "FI": ["vr", "vr group"],
    "PT": ["cp", "comboios de portugal", "infraestruturas de portugal"],
    "PL": ["pkp", "pkp intercity", "regiojet"],
    "GR": ["trainose", "hellenic train", "ose"],
}

# Operators that are definitively urban transit (drop even if station= is clean).
METRO_OPERATORS = [
    "bvg", "ratp", "tfl", "transport for london", "bart", "cta", "mta",
    "wmata", "septa", "metro de madrid", "metro de", "u-bahn", "s-bahn berlin",
    "vienna u-bahn", "wiener linien", "stib", "mivb", "gvb", "ttc",
    "métro", "metro ", "underground", "subway", "tramlink", "light rail",
]


def overpass_query(bbox: tuple, timeout: int = 120) -> list:
    lat_min, lon_min, lat_max, lon_max = bbox
    bb = f"{lat_min},{lon_min},{lat_max},{lon_max}"
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["railway"="station"]["station"!="subway"]["station"!="light_rail"]'
        f'["station"!="tram"]["station"!="monorail"]({bb});\n'
        f'  way["railway"="station"]["station"!="subway"]["station"!="light_rail"]'
        f'["station"!="tram"]["station"!="monorail"]({bb});\n'
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


def keep_station(tags: dict, iso: str) -> bool:
    """Operator-based keep/drop. Returns True to keep."""
    operator = (tags.get("operator") or "").lower()
    network = (tags.get("network") or "").lower()
    combined = f"{operator} {network}"

    # Drop explicit metro/urban transit operators
    if any(m in combined for m in METRO_OPERATORS):
        return False

    allowed = INTERCITY_OPERATORS.get(iso)
    if allowed is None:
        return True  # GB: accept all non-urban stations

    if not operator and not network:
        return True  # untagged operator — keep (often a valid station)

    return any(op in combined for op in allowed)


def element_to_record(elem: dict, iso: str) -> dict | None:
    if elem["type"] == "node":
        lat, lon = elem.get("lat"), elem.get("lon")
    else:
        center = elem.get("center", {})
        lat, lon = center.get("lat"), center.get("lon")
    if lat is None or lon is None:
        return None

    tags = elem.get("tags", {})
    if not keep_station(tags, iso):
        return None

    name = (tags.get("name") or tags.get("official_name") or "railway station").strip()
    tag_iso = tags.get("addr:country") or tags.get("is_in:country_code")
    if tag_iso and tag_iso != iso:
        return None

    return {
        "placekey": None,
        "category_id": "railway_station",
        "overture_id": None,
        "location_name": name,
        "street_address": None,
        "city": (tags.get("addr:city") or tags.get("addr:town") or tags.get("addr:municipality")),
        "region": (tags.get("addr:state") or tags.get("addr:province") or tags.get("addr:region")),
        "postal_code": tags.get("addr:postcode"),
        "iso_country_code": iso,
        "latitude": round(float(lat), 7),
        "longitude": round(float(lon), 7),
        "polygon_wkt": None,
        "naics_code": "482111",
        "top_category": "Line-Haul Railroads",
        "sub_category": "Passenger Rail Station",
        "operating_status": "open",
        "source": "osm",
        "confidence": 0.85,
        "last_updated": "2026-06-01",
        "is_regional_anchor": True,
        "sub_department_count": 0,
        "operator": tags.get("operator"),
        "network": tags.get("network"),
        "wikidata_id": tags.get("wikidata"),
    }


def ingest_country(iso: str, bbox: tuple) -> list:
    print(f"  [{iso}] railway=station bbox={bbox}")
    try:
        elements = overpass_query(bbox)
    except (RuntimeError, OSError) as e:
        print(f"    ERROR: {e}")
        return []

    print(f"    OSM elements returned: {len(elements)}")
    records = []
    dropped_op = 0
    for elem in elements:
        rec = element_to_record(elem, iso)
        if rec:
            records.append(rec)
        else:
            dropped_op += 1

    # Coordinate-based de-duplication (~110m threshold)
    deduped = []
    seen = set()
    for rec in records:
        key = (round(rec["latitude"], 3), round(rec["longitude"], 3))
        if key not in seen:
            deduped.append(rec)
            seen.add(key)
    dupes = len(records) - len(deduped)

    print(f"    dropped {dropped_op} (metro/operator filter), {dupes} dupes "
          f"→ {len(deduped)} intercity stations for {iso}")
    return deduped


def main():
    parser = argparse.ArgumentParser(description="Ingest OSM intercity railway stations")
    parser.add_argument("--countries", nargs="+", metavar="ISO")
    parser.add_argument("--all", action="store_true")
    parser.add_argument("--delay", type=float, default=5.0)
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
        print(f"ERROR: unknown / no-rail country codes: {invalid}")
        print(f"Known: {sorted(COUNTRY_BBOX.keys())} (MX, IS excluded — no intercity rail)")
        sys.exit(1)

    all_records = []
    first = True
    for iso in countries:
        if not first:
            time.sleep(args.delay)
        first = False
        all_records.extend(ingest_country(iso, COUNTRY_BBOX[iso]))

    print(f"\nTotal intercity stations: {len(all_records)}")
    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_FILE, "w") as f:
        for rec in all_records:
            f.write(json.dumps(rec) + "\n")
    print(f"Written → {OUTPUT_FILE}")

    by_iso = {}
    for r in all_records:
        by_iso[r["iso_country_code"]] = by_iso.get(r["iso_country_code"], 0) + 1
    print("\nBreakdown:")
    for iso, n in sorted(by_iso.items(), key=lambda x: -x[1]):
        print(f"  {iso}: {n}")


if __name__ == "__main__":
    main()
