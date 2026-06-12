#!/usr/bin/env python3
"""
ingest-osm-airports.py — OSM Overpass → cleansed-civic-airports.jsonl

Queries Overpass for commercial passenger airports by country bounding box.
Replaces the noisy Overture airport set (20,841 records including private
airstrips, heliports, military fields, glider clubs) with OSM-sourced
commercial airports only.

A commercial airport is identified by either:
  - aerodrome:type IN (public, international, regional, domestic), OR
  - presence of an iata=* tag (IATA codes are issued only to airports
    with scheduled commercial passenger service)

Explicitly excluded: aerodrome:type IN (private, military, glider),
aeroway IN (heliport, airstrip).

Writes to service-places/cleansed-civic-airports.jsonl.

Usage:
    python3 ingest-osm-airports.py --countries US CA MX
    python3 ingest-osm-airports.py --all
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

OUTPUT_FILE = TOTEBOX_DATA_PATH / "service-places" / "cleansed-civic-airports.jsonl"

COUNTRY_BBOX = {
    "US": (24.0,  -125.0,  50.0,  -65.0),
    "CA": (41.0,  -141.0,  84.0,  -52.0),
    "MX": (14.0,  -118.0,  33.0,  -86.0),
    "ES": (35.0,   -9.5,   44.0,    4.5),
    "IT": (36.0,    6.5,   47.5,   18.5),
    "GR": (34.5,   19.5,   42.0,   26.5),
    "PL": (49.0,   14.0,   55.0,   24.5),
    "SE": (55.0,   10.5,   69.5,   24.5),
    "NO": (57.0,    3.5,   71.5,   31.5),
    "DK": (54.5,    7.5,   58.0,   15.5),
    "FI": (59.5,   19.5,   70.5,   31.5),
    "IS": (63.0,  -25.0,   66.5,  -13.0),
    "FR": (41.0,   -5.5,   51.5,   10.0),
    "DE": (47.0,    6.0,   55.5,   15.5),
    "GB": (49.5,   -8.5,   61.5,    2.0),
    "AT": (46.5,    9.5,   49.0,   17.2),
    "NL": (50.7,    3.3,   53.7,    7.3),
    "PT": (36.5,   -9.5,   42.5,   -6.0),
}

# aerodrome:type values that indicate a commercial passenger airport
COMMERCIAL_TYPES = {"public", "international", "regional", "domestic"}
# aerodrome:type values to reject outright
REJECT_TYPES = {"private", "military", "glider", "airfield"}

# Name substrings that indicate a military airfield (drop even if it has an IATA code)
MILITARY_NAME_SIGNALS = [
    "air force base", "afb", "space force", "naval air", "nas ",
    "army airfield", "army air", "marine corps", "mcas", "raf ",
    "joint base", "air national guard", "air reserve", "military",
    "base aérea", "base aerea", "fliegerhorst", "luftwaffe",
]

# Large countries whose bbox must be tiled to avoid Overpass timeouts.
# value = (n_lat_tiles, n_lon_tiles)
TILED_COUNTRIES = {"US": (3, 5), "CA": (2, 4)}


def overpass_query(bbox: tuple, timeout: int = 120) -> list:
    """Query Overpass for commercial aerodromes within a bounding box.
    Pulls aerodromes with a commercial aerodrome:type OR an iata tag."""
    lat_min, lon_min, lat_max, lon_max = bbox
    bb = f"{lat_min},{lon_min},{lat_max},{lon_max}"
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["aeroway"="aerodrome"]["aerodrome:type"~"^(public|international|regional|domestic)$"]({bb});\n'
        f'  way["aeroway"="aerodrome"]["aerodrome:type"~"^(public|international|regional|domestic)$"]({bb});\n'
        f'  node["aeroway"="aerodrome"]["iata"]({bb});\n'
        f'  way["aeroway"="aerodrome"]["iata"]({bb});\n'
        f");\n"
        f"out center;\n"
    )
    last_err = None
    # 3 rounds across all endpoints, backing off to ride out throttle windows
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
            time.sleep(45)  # throttle cooldown before retrying all endpoints
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

    # Reject explicit non-commercial types even if an iata tag slipped through
    atype = (tags.get("aerodrome:type") or "").lower()
    if atype in REJECT_TYPES:
        return None
    if tags.get("aeroway") in ("heliport", "airstrip"):
        return None

    # Reject military airfields by name signal or explicit military tags
    name_l = (tags.get("name") or "").lower()
    if any(sig in name_l for sig in MILITARY_NAME_SIGNALS):
        return None
    if tags.get("military") or tags.get("landuse") == "military":
        return None

    iata = tags.get("iata")
    icao = tags.get("icao")
    # Require either a commercial type OR an iata code (belt and suspenders;
    # the query already enforced this, but guard against loose tags)
    if atype not in COMMERCIAL_TYPES and not iata:
        return None

    name = (tags.get("name") or tags.get("official_name") or "airport").strip()
    tag_iso = tags.get("addr:country") or tags.get("is_in:country_code")
    if tag_iso and tag_iso != iso:
        return None

    return {
        "placekey": None,
        "category_id": "airport",
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
        "naics_code": "488119",
        "top_category": "Other Airport Operations",
        "sub_category": "Other Airport Operations",
        "operating_status": "open",
        "source": "osm",
        "confidence": 0.90,
        "last_updated": "2026-06-01",
        "is_regional_anchor": True,
        "sub_department_count": 0,
        "iata_code": iata,
        "icao_code": icao,
        "aerodrome_type": atype or None,
        "wikidata_id": tags.get("wikidata"),
    }


def _tiles(bbox: tuple, n_lat: int, n_lon: int):
    lat_min, lon_min, lat_max, lon_max = bbox
    dlat = (lat_max - lat_min) / n_lat
    dlon = (lon_max - lon_min) / n_lon
    for i in range(n_lat):
        for j in range(n_lon):
            yield (lat_min + i * dlat, lon_min + j * dlon,
                   lat_min + (i + 1) * dlat, lon_min + (j + 1) * dlon)


def ingest_country(iso: str, bbox: tuple) -> list:
    # Tile large countries to avoid Overpass timeouts
    if iso in TILED_COUNTRIES:
        n_lat, n_lon = TILED_COUNTRIES[iso]
        print(f"  [{iso}] aerodrome — tiling {n_lat}×{n_lon} over {bbox}")
        elements = []
        tiles = list(_tiles(bbox, n_lat, n_lon))
        for ti, tb in enumerate(tiles):
            try:
                te = overpass_query(tb, timeout=90)
                elements.extend(te)
                print(f"    tile {ti+1}/{len(tiles)}: {len(te)} elements")
            except (RuntimeError, OSError) as e:
                print(f"    tile {ti+1}/{len(tiles)} ERROR: {e}")
            time.sleep(3)
    else:
        print(f"  [{iso}] aerodrome bbox={bbox}")
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

    # Coordinate-based de-duplication (~1km threshold — large airports map as
    # multiple nodes for terminal/runway/apron)
    deduped = []
    seen = set()
    for rec in records:
        key = (round(rec["latitude"], 2), round(rec["longitude"], 2))
        if key not in seen:
            deduped.append(rec)
            seen.add(key)
    dupes = len(records) - len(deduped)
    if dupes:
        print(f"    de-duplication: dropped {dupes} co-located nodes")

    with_iata = sum(1 for r in deduped if r.get("iata_code"))
    print(f"    → {len(deduped)} airports for {iso} ({with_iata} with IATA code)")
    return deduped


def main():
    parser = argparse.ArgumentParser(description="Ingest OSM commercial airports")
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
        print(f"ERROR: unknown country codes: {invalid}")
        sys.exit(1)

    all_records = []
    first = True
    for iso in countries:
        if not first:
            time.sleep(args.delay)
        first = False
        all_records.extend(ingest_country(iso, COUNTRY_BBOX[iso]))

    print(f"\nTotal commercial airports: {len(all_records)}")
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
