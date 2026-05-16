#!/usr/bin/env python3
"""
ingest-osm.py — OSM Overpass API → service-business JSONL

Queries Overpass for each chain using brand:wikidata=<id> filtered by country
bounding box. Applies format_exclude_names to drop sub-format records.
Writes locations/<chain_id>.jsonl.

Usage:
    python3 ingest-osm.py --chain costco-us
    python3 ingest-osm.py --chain costco-us costco-ca costco-mx
    python3 ingest-osm.py --all-pending    # all chains with no JSONL file yet
"""

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).parent))
from config import TOTEBOX_DATA_PATH

OVERPASS_URLS = [
    "https://overpass-api.de/api/interpreter",
    "https://overpass.kumi.systems/api/interpreter",
    "https://overpass.private.coffee/api/interpreter",
]

# Chain YAMLs and output JSONLs live together in service-fs/service-business/
CHAIN_DIR = TOTEBOX_DATA_PATH / "service-fs" / "service-business"
LOC_DIR   = TOTEBOX_DATA_PATH / "service-fs" / "service-business"

# Country code → (lat_min, lon_min, lat_max, lon_max) bounding boxes
COUNTRY_BBOX = {
    "US":      (24.0,  -125.0,  50.0,  -65.0),
    "CA":      (41.0,  -141.0,  84.0,  -52.0),
    "MX":      (14.0,  -118.0,  33.0,  -86.0),
    "ES":      (35.0,   -9.5,   44.0,    4.5),
    "IT":      (36.0,    6.5,   47.5,   18.5),
    "GR":      (34.5,   19.5,   42.0,   26.5),  # tightened to exclude Istanbul/Turkey
    "PL":      (49.0,   14.0,   55.0,   24.5),
    "SE":      (55.0,   10.5,   69.5,   24.5),
    "IS":      (63.0,  -25.0,   66.5,  -13.0),
    "NO":      (57.0,    3.5,   71.5,   31.5),
    "DK":      (54.5,    7.5,   58.0,   15.5),
    "FI":      (59.5,   19.5,   70.5,   31.5),
    "Nordics": (54.5,  -25.0,   71.5,   31.5),
    "FR":      (41.0,   -5.5,   51.5,   10.0),
    "DE":      (47.0,    6.0,   55.5,   15.5),
    "GB":      (49.5,   -8.5,   61.5,    2.0),
    "AT":      (46.5,    9.5,   49.0,   17.2),
    "NL":      (50.7,    3.3,   53.7,    7.3),
    "PT":      (36.5,   -9.5,   42.5,   -6.0),
    "UY":      (-35.0, -58.5,  -30.0,  -53.0),
}

# Sprint 11 D3 — country-polygon containment filter to fix bbox contamination
# (e.g., CA bbox leaking US border stores). Toggle to False to disable for diagnosis.
COUNTRY_POLYGON_FILTER = True
_COUNTRY_POLYGON_CACHE: dict = {}


def _load_country_polygon(iso_code: str):
    """Lazy-load the union polygon for a country from fallback_ne_admin1.geojson.
    Returns None if shapely unavailable or country not found. Result cached per ISO."""
    if iso_code in _COUNTRY_POLYGON_CACHE:
        return _COUNTRY_POLYGON_CACHE[iso_code]
    try:
        from shapely.geometry import shape
        from shapely.ops import unary_union
    except ImportError:
        _COUNTRY_POLYGON_CACHE[iso_code] = None
        return None
    boundaries = Path("/srv/foundry/deployments/cluster-totebox-personnel-1/boundaries/fallback_ne_admin1.geojson")
    if not boundaries.exists():
        _COUNTRY_POLYGON_CACHE[iso_code] = None
        return None
    with open(boundaries) as f:
        fc = json.load(f)
    geoms = []
    for feat in fc.get("features", []):
        if feat.get("properties", {}).get("iso_a2") == iso_code:
            try:
                geoms.append(shape(feat["geometry"]))
            except Exception:
                continue
    if not geoms:
        _COUNTRY_POLYGON_CACHE[iso_code] = None
        return None
    poly = unary_union(geoms)
    _COUNTRY_POLYGON_CACHE[iso_code] = poly
    return poly


def _filter_records_by_country_polygon(records: list, iso_code: str) -> tuple:
    """Drop records whose lat/lon falls outside the country polygon.
    Returns (kept_records, dropped_count). No-op if shapely unavailable or
    polygon not loadable."""
    if not COUNTRY_POLYGON_FILTER or not iso_code:
        return records, 0
    polygon = _load_country_polygon(iso_code)
    if polygon is None:
        return records, 0
    try:
        from shapely.geometry import Point
    except ImportError:
        return records, 0
    kept = []
    dropped = 0
    for rec in records:
        lat = rec.get("latitude")
        lon = rec.get("longitude")
        if lat is None or lon is None:
            kept.append(rec)
            continue
        if polygon.contains(Point(lon, lat)):
            kept.append(rec)
        else:
            dropped += 1
    return kept, dropped

# Tags that indicate non-store sub-facilities — always drop at element level
SKIP_AMENITY_TAGS = {"fuel", "atm", "pharmacy"}
# Name substrings that indicate a fuel/gas facility
SKIP_NAME_SUBSTRINGS = ["gasoline", " gas", "fuel", "petrol", "pharmacy", "vision center", "garden center", "photo center", "tire & lube", "tire and lube", "food court", "optical", "hearing aid", "moneycenter"]


def load_chain_yaml(chain_id: str) -> dict:
    p = CHAIN_DIR / f"{chain_id}.yaml"
    if not p.exists():
        raise FileNotFoundError(f"Chain YAML not found: {p}")
    with open(p) as f:
        return yaml.safe_load(f)


def _run_overpass(query: str, timeout: int) -> list:
    """Execute an Overpass QL query via curl, trying multiple instances.
    Uses --data-urlencode so special chars (& in B&Q etc.) are safe."""
    last_err = None
    for url in OVERPASS_URLS:
        try:
            result = subprocess.run(
                ["curl", "-s", "--max-time", str(timeout + 60),
                 "-H", "User-Agent: project-gis/ingest-osm",
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
    raise RuntimeError(f"All Overpass instances failed. Last error: {last_err}")


def overpass_query(wikidata_id: str, bbox: tuple, timeout: int = 120) -> list:
    """Query Overpass for brand:wikidata=<id> within a bounding box."""
    lat_min, lon_min, lat_max, lon_max = bbox
    query = (
        f"[out:json][timeout:{timeout}];\n(\n"
        f'  node["brand:wikidata"="{wikidata_id}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["brand:wikidata"="{wikidata_id}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  relation["brand:wikidata"="{wikidata_id}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f");\nout center;\n"
    )
    return _run_overpass(query, timeout)


def overpass_query_by_name(name: str, bbox: tuple, timeout: int = 120,
                           partial: bool = False) -> list:
    """Query Overpass for name=<name> within a bounding box.
    partial=True uses regex prefix match (catches "Brand CityName" variants).
    Used as fallback when brand:wikidata tag coverage is sparse."""
    lat_min, lon_min, lat_max, lon_max = bbox
    escaped = name.replace('"', '\\"').replace("&", "\\&")
    if partial:
        op = f'~"^{escaped}",i'   # regex: name starts with the query string
    else:
        op = f'="{escaped}"'
    query = (
        f"[out:json][timeout:{timeout}];\n(\n"
        f'  node["name"{op}]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["name"{op}]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  relation["name"{op}]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f");\nout center;\n"
    )
    return _run_overpass(query, timeout)


def is_fuel_facility(elem: dict) -> bool:
    """Return True if this element is a gas station / fuel point, not a store."""
    tags = elem.get("tags", {})
    if tags.get("amenity") in SKIP_AMENITY_TAGS:
        return True
    name = (tags.get("name") or "").lower()
    return any(sub in name for sub in SKIP_NAME_SUBSTRINGS)


def element_to_record(elem: dict, chain: dict) -> dict | None:
    if elem["type"] == "node":
        lat, lon = elem.get("lat"), elem.get("lon")
    else:
        center = elem.get("center", {})
        lat, lon = center.get("lat"), center.get("lon")
    if lat is None or lon is None:
        return None

    tags = elem.get("tags", {})
    name = tags.get("name") or tags.get("brand") or chain.get("retailer", "")
    housenumber = tags.get("addr:housenumber", "")
    street = tags.get("addr:street", "")
    street_address = f"{housenumber} {street}".strip() or None
    city = (tags.get("addr:city")
            or tags.get("addr:town")
            or tags.get("addr:municipality"))
    state_region = (tags.get("addr:state")
                    or tags.get("addr:province")
                    or tags.get("addr:region"))
    postal_code = tags.get("addr:postcode")
    iso_country = tags.get("addr:country") or tags.get("is_in:country_code")

    # Reject records explicitly tagged as belonging to a different country
    expected_cc = chain.get("country_code", "")
    if iso_country and iso_country != expected_cc:
        return None

    return {
        "placekey": None,
        "chain_id": chain["chain_id"],
        "brand_wikidata": chain.get("wikidata_id") or None,
        "location_name": name,
        "brands": [{"brand_name": chain.get("retailer", ""),
                    "brand_wikidata": chain.get("wikidata_id", "")}],
        "street_address": street_address,
        "city": city,
        "region": state_region,
        "postal_code": postal_code,
        "iso_country_code": iso_country or chain.get("country_code"),
        "latitude": round(lat, 7),
        "longitude": round(lon, 7),
        "polygon_wkt": None,
        "naics_code": chain.get("naics_code", ""),
        "top_category": chain.get("top_category", ""),
        "sub_category": chain.get("sub_category", ""),
        "operating_status": "open",
        "source": "osm",
        "confidence": 0.85,
        "last_updated": "2026-05-01",
    }


def apply_format_filter(records: list, chain: dict) -> list:
    """Drop records whose location_name matches any format_exclude_names substring."""
    exclude = chain.get("format_exclude_names") or []
    if not exclude:
        return records
    out = []
    for rec in records:
        name = rec.get("location_name") or ""
        if any(ex.lower() in name.lower() for ex in exclude):
            continue
        out.append(rec)
    dropped = len(records) - len(out)
    if dropped:
        print(f"    format_filter: dropped {dropped} sub-format records")
    return out


def ingest_chain(chain_id: str) -> int:
    print(f"\n[{chain_id}]")
    try:
        chain = load_chain_yaml(chain_id)
    except FileNotFoundError as e:
        print(f"  ERROR: {e}")
        return 0

    wikidata_id = chain.get("wikidata_id")
    name_query  = chain.get("name_query")
    if not wikidata_id and not name_query:
        print(f"  No wikidata_id or name_query — skip")
        return 0

    country_code = chain.get("country_code") or chain.get("country", "")
    bbox = COUNTRY_BBOX.get(country_code) or COUNTRY_BBOX.get(country_code.title())
    if bbox is None:
        print(f"  No bbox for country_code '{country_code}' — skip")
        return 0

    elements = []
    if wikidata_id:
        print(f"  brand:wikidata={wikidata_id}, country={country_code}, bbox={bbox}")
        try:
            elements = overpass_query(wikidata_id, bbox)
        except (RuntimeError, OSError) as e:
            print(f"  ERROR querying Overpass: {e}")
            return 0
        print(f"  OSM elements returned: {len(elements)}")

    if not elements and name_query:
        partial = bool(chain.get("name_query_partial"))
        mode = "prefix regex" if partial else "exact"
        print(f"  wikidata returned 0 — falling back to name=\"{name_query}\" ({mode})")
        try:
            elements = overpass_query_by_name(name_query, bbox, partial=partial)
        except (RuntimeError, OSError) as e:
            print(f"  ERROR querying Overpass (name fallback): {e}")
            return 0
        print(f"  OSM elements returned (name query): {len(elements)}")

    records = []
    fuel_skipped = 0
    for elem in elements:
        if is_fuel_facility(elem):
            fuel_skipped += 1
            continue
        rec = element_to_record(elem, chain)
        if rec:
            records.append(rec)

    if fuel_skipped:
        print(f"    fuel/gas facilities skipped: {fuel_skipped}")

    records = apply_format_filter(records, chain)
    print(f"  Records after format filter: {len(records)}")

    # Country-code validation: belt-and-suspenders pass after element_to_record filter.
    # Rejects any record explicitly tagged with a country other than expected_cc.
    # Records with no iso_country_code (untagged in OSM) are kept — they may be valid
    # local stores that simply lack the addr:country OSM tag.
    expected_cc = chain.get("country_code", "")
    if expected_cc:
        before_cc = len(records)
        records = [r for r in records
                   if not r.get("iso_country_code") or r["iso_country_code"] == expected_cc]
        wrong_cc = before_cc - len(records)
        if wrong_cc:
            print(f"    country filter: dropped {wrong_cc} records tagged {expected_cc!r} mismatch")

    # Sprint 11 D3 — polygon containment filter (catches bbox leak when records
    # have no addr:country tag and inherit the chain's country_code as fallback).
    if expected_cc:
        records, polygon_dropped = _filter_records_by_country_polygon(records, expected_cc)
        if polygon_dropped:
            print(f"    polygon-filter: dropped {polygon_dropped} cross-border records")

    if not records:
        print(f"  No records — skipping write")
        return 0

    # Coordinate-based de-duplication (~11m threshold — same building)
    deduped = []
    seen_coords = set()
    for rec in records:
        coord_key = (round(rec['latitude'], 4), round(rec['longitude'], 4))
        if coord_key not in seen_coords:
            deduped.append(rec)
            seen_coords.add(coord_key)

    dupes_dropped = len(records) - len(deduped)
    if dupes_dropped:
        print(f"    de-duplication: dropped {dupes_dropped} coordinate collisions")

    records = deduped
    print(f"  Records after all filters: {len(records)}")

    LOC_DIR.mkdir(parents=True, exist_ok=True)
    out = LOC_DIR / f"{chain_id}.jsonl"
    with open(out, "w") as f:
        for rec in records:
            f.write(json.dumps(rec) + "\n")
    size_kb = out.stat().st_size // 1024
    print(f"  Written {len(records)} records → {out.name} ({size_kb} KB)")
    return len(records)


def main():
    parser = argparse.ArgumentParser(description="Ingest service-business from OSM Overpass")
    parser.add_argument("--chain", nargs="+", metavar="CHAIN_ID",
                        help="One or more chain IDs to ingest")
    parser.add_argument("--all-pending", action="store_true",
                        help="Ingest all chains with no JSONL file yet")
    parser.add_argument("--delay", type=float, default=4.0,
                        help="Seconds between Overpass requests (default: 4)")
    args = parser.parse_args()

    if args.all_pending:
        chain_ids = []
        for yaml_file in sorted(CHAIN_DIR.glob("*.yaml")):
            cid = yaml_file.stem
            if not (LOC_DIR / f"{cid}.jsonl").exists():
                chain_ids.append(cid)
        print(f"All-pending: {len(chain_ids)} chains without JSONL")
        for cid in chain_ids:
            print(f"  {cid}")
    else:
        chain_ids = args.chain or []

    if not chain_ids:
        parser.print_help()
        sys.exit(1)

    total = 0
    for i, cid in enumerate(chain_ids):
        if i > 0:
            time.sleep(args.delay)
        total += ingest_chain(cid)

    print(f"\nDone. {total} total records written.")


if __name__ == "__main__":
    main()
