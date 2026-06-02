#!/usr/bin/env python3
"""
ingest-osm-railway-commuter.py — OSM Overpass → cleansed-civic-railway-commuter.jsonl

Ingests commuter rail, suburban rail, and metro/subway terminal stations.
Complements ingest-osm-railway.py (which targets intercity stations only).

Target use case: stations where a traveller DRIVES, PARKS, then rides transit
into the city — the park-and-ride catchment. This includes:
  - Commuter rail (NJ Transit, LIRR, Metro-North, Metra, Caltrain, VRE, etc.)
  - Suburban S-Bahn / RER / Transilien (DE, FR, AT)
  - Metro/subway end-of-line and near-outer stations (8–35 km from metro core)
  - Light rail suburban termini (BART East Bay, MAX, TRAX, etc.)

NOT included:
  - Downtown subway/metro stations (< 8 km from city center) — no park-and-ride
  - Intercity mainline stations (covered by ingest-osm-railway.py)
  - Tram stops

Strategy:
  - Overpass query includes station=subway and station=light_rail (excluded in intercity script)
  - Operator filter is EXPANDED to include commuter/suburban operators per country
  - Post-load, geographic distance from metro center is used to exclude downtown stations
    (the PKS archetype script already applies a metro_dist range filter)

Writes to service-places/cleansed-civic-railway-commuter.jsonl (appends; run --all to replace).

Usage:
    python3 ingest-osm-railway-commuter.py --countries US DE FR
    python3 ingest-osm-railway-commuter.py --all
    python3 ingest-osm-railway-commuter.py --all --replace
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

OUTPUT_FILE = TOTEBOX_DATA_PATH / "service-places" / "cleansed-civic-railway-commuter.jsonl"

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

# Commuter / suburban operators by country.
# Substring match (case-insensitive). None = accept all non-intercity stations.
COMMUTER_OPERATORS = {
    "US": [
        "nj transit", "njt", "lirr", "long island rail road",
        "metro-north", "metro north",
        "metra", "caltrain", "bart", "marc", "shore line east",
        "mbta", "septa", "trimet", "sound transit", "sounder",
        "metrolink", "coaster", "sunrail", "brightline",
        "valley metro", "trax", "frontrunner",
        "mta", "wmata", "cta", "la metro",
        "amtrak",  # include amtrak (intercity also, but ok — geographic filter handles separation)
    ],
    "CA": [
        "go transit", "go train", "metrolinx",
        "west coast express", "réseau de transport métropolitain", "exo",
        "agence métropolitaine de transport", "amt",
        "translink",
        "o-train", "via rail", "via",
    ],
    "MX": [
        "stc metro", "metro", "metrobús", "metrobus",
        "tren ligero", "suburbano", "ferrocarriles suburbanos",
        "sistema de tren eléctrico", "siteur",
    ],
    "GB": None,  # All UK non-urban — commuter TOCs included by default
    "FR": [
        "sncf", "ter", "transilien", "ile-de-france mobilités", "idf mobilités",
        "ratp",  # RER A/B/C are RATP-operated; suburban fringe stations are valid PKS
        "sncf voyageurs", "sncf réseau",
        "navigo",
    ],
    "DE": [
        "db", "deutsche bahn", "db regio", "db fernverkehr", "db netz",
        "s-bahn", "sbahn", "s bahn",
        "vgo", "rhein-ruhr", "vrs", "zvv", "hvv", "mvv", "vbb", "vrr",
    ],
    "ES": [
        "renfe", "adif", "cercanías", "cercanias", "rodalies", "metro",
        "fgc", "ferrocarrils de la generalitat de catalunya",
        "metro de madrid", "emt",
    ],
    "IT": [
        "trenitalia", "italo", "ferrovie dello stato",
        "ferrovie del nord milano", "atm", "atac",
        "fs", "rfi", "trenord",
    ],
    "AT": [
        "öbb", "obb", "wiener linien", "wiener lokalbahnen",
        "s-bahn wien", "sbahn wien",
    ],
    "NL": [
        "ns", "nederlandse spoorwegen", "gvb", "ret", "htm",
        "connexxion", "arriva", "ov-chipkaart",
    ],
    "BE": [
        "nmbs", "sncb", "de lijn", "stib", "mivb", "tec",
    ],
    "PL": [
        "pkp", "pkp intercity", "pkp plk", "skm", "kkm",
        "szybka kolej miejska", "koleje mazowieckie",
        "warszawa", "metro warszawa",
    ],
    "SE": [
        "sj", "norrtåg", "trafikverket",
        "sl", "pendeltåg", "tunnelbana",
        "västtrafik", "skånetrafiken", "mtr",
    ],
    "DK": [
        "dsb", "banedanmark", "metro",
        "s-tog", "stog", "movia",
    ],
    "NO": [
        "vy", "nsb", "bane nor", "ruter",
        "t-bane", "tbane",
    ],
    "FI": [
        "vr", "vr group", "hsl", "helsingin seudun liikenne",
        "metro", "länsimetro",
    ],
    "GR": ["trainose", "hellenic train", "stasy", "metro"],
    "PT": [
        "cp", "comboios de portugal", "infraestruturas de portugal",
        "metro do porto", "metropolitano de lisboa",
    ],
    "CH": [
        "sbb", "cff", "ffs", "zvv", "bls",
        "s-bahn", "sbahn",
    ],
    "CZ": ["cd", "české dráhy", "pid", "metro"],
    "HU": ["máv", "mav", "bkk", "metro", "hév"],
    "RO": ["cfr", "metrorex", "metro"],
}

# Operators definitively downtown-only (drop even if they appear in commuter lists)
DOWNTOWN_ONLY_OPERATORS = [
    "underground", "metro central", "city tram", "tram ",
]


def overpass_query(bbox: tuple, timeout: int = 150) -> list:
    lat_min, lon_min, lat_max, lon_max = bbox
    bb = f"{lat_min},{lon_min},{lat_max},{lon_max}"
    # Include subway and light_rail station= values (excluded in intercity script)
    # Still exclude plain trams and monorail
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["railway"="station"]["station"!="tram"]["station"!="monorail"]({bb});\n'
        f'  way["railway"="station"]["station"!="tram"]["station"!="monorail"]({bb});\n'
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


def transit_class(tags: dict) -> str:
    """Classify station by OSM tags into intercity / suburban / subway / light_rail."""
    s = (tags.get("station") or "").lower()
    if s == "subway":
        return "subway"
    if s == "light_rail":
        return "light_rail"
    # heuristic: usage=main/branch for intercity vs suburban
    usage = (tags.get("usage") or "").lower()
    if usage in ("main",):
        return "intercity"
    return "suburban"


def keep_station(tags: dict, iso: str) -> bool:
    operator = (tags.get("operator") or "").lower()
    network  = (tags.get("network")  or "").lower()
    combined = f"{operator} {network}"

    if any(d in combined for d in DOWNTOWN_ONLY_OPERATORS):
        return False

    allowed = COMMUTER_OPERATORS.get(iso)
    if allowed is None:
        return True  # GB: accept all

    if not operator and not network:
        return True  # untagged — keep

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

    tag_iso = tags.get("addr:country") or tags.get("is_in:country_code")
    if tag_iso and tag_iso != iso:
        return None

    name = (tags.get("name") or tags.get("official_name") or "station").strip()

    return {
        "placekey":          None,
        "category_id":       "railway_station",
        "overture_id":       None,
        "location_name":     name,
        "street_address":    None,
        "city":              tags.get("addr:city") or tags.get("addr:town"),
        "region":            tags.get("addr:state") or tags.get("addr:province"),
        "postal_code":       tags.get("addr:postcode"),
        "iso_country_code":  iso,
        "latitude":          round(float(lat), 7),
        "longitude":         round(float(lon), 7),
        "polygon_wkt":       None,
        "naics_code":        "485112",
        "top_category":      "Commuter Rail Systems",
        "sub_category":      "Commuter Rail Station",
        "operating_status":  "open",
        "source":            "osm",
        "confidence":        0.80,
        "last_updated":      "2026-06-02",
        "is_regional_anchor": True,
        "sub_department_count": 0,
        "operator":          tags.get("operator"),
        "network":           tags.get("network"),
        "wikidata_id":       tags.get("wikidata"),
        "transit_class":     transit_class(tags),  # intercity|suburban|subway|light_rail
    }


def ingest_country(iso: str, bbox: tuple) -> list:
    print(f"  [{iso}] railway=station (commuter+subway) bbox={bbox}")
    try:
        elements = overpass_query(bbox)
    except (RuntimeError, OSError) as e:
        print(f"    ERROR: {e}")
        return []

    print(f"    OSM elements returned: {len(elements)}")
    records = []
    dropped = 0
    for elem in elements:
        rec = element_to_record(elem, iso)
        if rec:
            records.append(rec)
        else:
            dropped += 1

    # Coordinate de-dup (~110m)
    deduped = []
    seen = set()
    for rec in records:
        key = (round(rec["latitude"], 3), round(rec["longitude"], 3))
        if key not in seen:
            deduped.append(rec)
            seen.add(key)
    dupes = len(records) - len(deduped)

    by_class: dict[str, int] = {}
    for r in deduped:
        tc = r.get("transit_class", "unknown")
        by_class[tc] = by_class.get(tc, 0) + 1

    print(f"    dropped {dropped} (operator filter), {dupes} dupes → {len(deduped)} stations")
    print(f"    by class: {by_class}")
    return deduped


def main():
    parser = argparse.ArgumentParser(
        description="Ingest OSM commuter/suburban rail and metro end-of-line stations"
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

    print(f"\nTotal commuter stations: {len(all_records)}")

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
        tc = r.get("transit_class", "unknown")
        by_class[tc] = by_class.get(tc, 0) + 1

    print("\nBy country:")
    for iso, n in sorted(by_iso.items(), key=lambda x: -x[1]):
        print(f"  {iso}: {n}")
    print("\nBy transit class:")
    for tc, n in sorted(by_class.items(), key=lambda x: -x[1]):
        print(f"  {tc}: {n}")

    print("""
Next steps:
  1. Update test-cluster-archetypes.py to load cleansed-civic-railway-commuter.jsonl
     alongside cleansed-civic-railway.jsonl.
  2. The commuter/subway stations should use a tighter metro_dist range (5–35 km)
     rather than the intercity range (35–150 km). The 'transit_class' field
     distinguishes them: subway/light_rail use the tight range; suburban uses mid-range.
  3. Re-run test-cluster-archetypes.py to produce updated archetype-pks.geojson.
  4. Copy updated archetype-pks.geojson to www/data/ and sync to gateway.
""")


if __name__ == "__main__":
    main()
