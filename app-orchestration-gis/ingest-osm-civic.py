#!/usr/bin/env python3
"""
ingest-osm-civic.py — OSM Overpass → cleansed-civic-osm.jsonl

Queries Overpass for amenity=hospital and amenity=university by country
bounding box. OSM applies these tags conservatively: amenity=hospital
means a real hospital building (not clinics, urgent care, or doctors'
offices); amenity=university means an actual university campus (not
satellite offices or training centers).

Writes to service-places/cleansed-civic-osm.jsonl.

Usage:
    python3 ingest-osm-civic.py --countries US CA MX
    python3 ingest-osm-civic.py --all
    python3 ingest-osm-civic.py --amenity hospital --countries ES IT
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
    "https://overpass-api.de/api/interpreter",
    "https://overpass.kumi.systems/api/interpreter",
    "https://overpass.private.coffee/api/interpreter",
]

OUTPUT_FILE = TOTEBOX_DATA_PATH / "service-places" / "cleansed-civic-osm.jsonl"

# Same bounding boxes used in ingest-osm.py (individual country codes only)
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
}

AMENITIES = ["hospital", "university"]

# Hospital name substrings that indicate non-hospital facilities (quick pre-filter)
HOSPITAL_SKIP_NAMES = [
    "urgent care", "urgentcare", "clinica del dolor", "pain clinic",
    "dental", "optom", "optician", "pharmacy", "farmacia", "podol",
    "beauty", "cosmet", "tattoo", "veterinar",
    "walk-in", "walkin", "walk in", "family practice", "family medicine",
    "dialysis", "rehabilitation", "rehab center", "psychiatric",
    "mental health center", "nursing home", "care home", "rest home",
    "surgery center", "outpatient",
]

# University name substrings that indicate non-campus facilities (quick pre-filter)
UNIVERSITY_SKIP_NAMES = [
    "driving school", "auto school", "music school", "dance school",
    "beauty school", "cosmetology", "cooking school", "culinary",
    "language school", "gym", "fitness", "yoga",
    "vocational", "trade school", "bible college", "bible university",
    "preparatoria", "preparatory school", "middle school", "high school",
    "primary school", "secondary school", "escuela secundaria",
    "escuela preparatoria",
]

# Regional hospital name signals (substring match → "regional" tier)
_HOSPITAL_REGIONAL_SIGNALS = [
    "general hospital", "regional hospital", "regional medical",
    "memorial hospital", "medical centre", "medical center",
    "university hospital", "university medical", "centre hospitalier",
    "hospital general", "hospital regional", "hospital universitario",
    "centro medico", "krankenhaus", "universitätsklinikum",
    "hopital central", "hopital regional", "klinikum",
]

# Clinic name signals (override to "clinic" tier when no bed data)
_HOSPITAL_CLINIC_SIGNALS = [
    "walk-in", "walkin", "walk in", "family practice", "family medicine",
    "family health", "dialysis", "rehabilitation", "rehab center",
    "psychiatric", "mental health center", "nursing home", "care home",
    "rest home", "surgery center", "outpatient", "health centre",
    "health center", "dental", "eye care", "vision center",
]


def _classify_hospital(name: str, bed_count, emergency: str, operator_type: str) -> str:
    """Return 'regional', 'district', or 'clinic'."""
    n = name.lower()
    if any(sig in n for sig in _HOSPITAL_REGIONAL_SIGNALS):
        return "regional"
    if bed_count is not None:
        if bed_count >= 100:
            return "regional"
        if bed_count >= 50:
            return "district"
        return "clinic"
    if emergency == "yes":
        return "regional"
    if any(sig in n for sig in _HOSPITAL_CLINIC_SIGNALS):
        return "clinic"
    return "district"  # uncertain — keep at lower tier


def _classify_university_name(name: str) -> str:
    """Return 'regional', 'small', or 'excluded'."""
    n = name.lower()
    excluded_signals = [
        "vocational", "trade school", "bible college", "bible university",
        "preparatoria", "preparatory", "middle school", "high school",
        "primary school", "secondary school", "escuela secundaria",
        "escuela preparatoria", "driving school", "beauty school",
        "cosmetology", "music school", "dance school", "culinary",
        "cooking school", "language school", "gym", "fitness", "yoga",
        "auto school",
    ]
    if any(sig in n for sig in excluded_signals):
        return "excluded"
    regional_signals = [
        "university", "universidad", "universität", "université",
        "universidade", "polytechnic", "institute of technology",
        "school of medicine", "school of law", "school of engineering",
        "college of arts", "college of science",
    ]
    if any(sig in n for sig in regional_signals):
        return "regional"
    # community/junior/city colleges stay in pipeline but flagged small
    return "small"


def overpass_query(amenity: str, bbox: tuple, timeout: int = 120) -> list:
    lat_min, lon_min, lat_max, lon_max = bbox
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["amenity"="{amenity}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["amenity"="{amenity}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  relation["amenity"="{amenity}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f");\n"
        f"out center;\n"
    )
    last_err = None
    for url in OVERPASS_URLS:
        try:
            result = subprocess.run(
                ["curl", "-s", "--max-time", str(timeout + 60),
                 "--data", f"data={query}", url],
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


def is_skip_facility(amenity: str, elem: dict) -> bool:
    tags = elem.get("tags", {})
    name = (tags.get("name") or "").lower()
    if amenity == "hospital":
        return any(sub in name for sub in HOSPITAL_SKIP_NAMES)
    elif amenity == "university":
        return any(sub in name for sub in UNIVERSITY_SKIP_NAMES)
    return False


def element_to_record(elem: dict, amenity: str, iso: str) -> dict | None:
    if elem["type"] == "node":
        lat, lon = elem.get("lat"), elem.get("lon")
    else:
        center = elem.get("center", {})
        lat, lon = center.get("lat"), center.get("lon")
    if lat is None or lon is None:
        return None

    tags = elem.get("tags", {})
    name = (tags.get("name") or tags.get("official_name") or amenity).strip()

    # Reject elements explicitly tagged to a different country
    tag_iso = tags.get("addr:country") or tags.get("is_in:country_code")
    if tag_iso and tag_iso != iso:
        return None

    # Extract civic quality signals from OSM tags
    bed_count_raw = tags.get("beds") or tags.get("capacity") or tags.get("bed_count")
    bed_count = None
    if bed_count_raw:
        try:
            bed_count = int(str(bed_count_raw).strip().split(";")[0])
        except (ValueError, AttributeError):
            pass

    emergency    = tags.get("emergency") or tags.get("emergency_service")
    operator_type = tags.get("operator:type") or tags.get("operator")
    wikidata_id  = tags.get("wikidata")

    # Classify and drop low-quality records at ingest (fail-closed)
    if amenity == "hospital":
        hospital_tier = _classify_hospital(name, bed_count, emergency, operator_type)
        if hospital_tier == "clinic":
            return None
        university_tier = None
    else:
        university_tier = _classify_university_name(name)
        if university_tier == "excluded":
            return None
        hospital_tier = None

    category_id = amenity  # "hospital" or "university"

    return {
        "placekey": None,
        "category_id": category_id,
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
        "naics_code": "622110" if amenity == "hospital" else "611310",
        "top_category": "General Medical and Surgical Hospitals" if amenity == "hospital"
                        else "Colleges, Universities, and Professional Schools",
        "sub_category": "General Medical and Surgical Hospitals" if amenity == "hospital"
                        else "Colleges, Universities, and Professional Schools",
        "operating_status": "open",
        "source": "osm",
        "confidence": 0.90,
        "last_updated": "2026-05-05",
        "is_regional_anchor": True,
        "sub_department_count": 0,
        "bed_count": bed_count,
        "emergency": emergency,
        "operator_type": operator_type,
        "hospital_tier": hospital_tier,
        "university_tier": university_tier,
        "wikidata_id": wikidata_id,
    }


def ingest_country_amenity(amenity: str, iso: str, bbox: tuple) -> list:
    print(f"  [{iso}] amenity={amenity} bbox={bbox}")
    try:
        elements = overpass_query(amenity, bbox)
    except (RuntimeError, OSError) as e:
        print(f"    ERROR: {e}")
        return []

    print(f"    OSM elements returned: {len(elements)}")
    records = []
    skipped = 0
    for elem in elements:
        if is_skip_facility(amenity, elem):
            skipped += 1
            continue
        rec = element_to_record(elem, amenity, iso)
        if rec:
            records.append(rec)

    if skipped:
        print(f"    skipped {skipped} non-{amenity} facilities")

    # Coordinate-based de-duplication (~11m threshold)
    deduped = []
    seen = set()
    for rec in records:
        key = (round(rec["latitude"], 4), round(rec["longitude"], 4))
        if key not in seen:
            deduped.append(rec)
            seen.add(key)
    dupes = len(records) - len(deduped)
    if dupes:
        print(f"    de-duplication: dropped {dupes} duplicates")
    print(f"    → {len(deduped)} {amenity} records for {iso}")
    return deduped


def main():
    parser = argparse.ArgumentParser(description="Ingest OSM civic places (hospital + university)")
    parser.add_argument("--countries", nargs="+", metavar="ISO",
                        help="ISO country codes (e.g. US CA MX)")
    parser.add_argument("--amenity", choices=AMENITIES, help="Ingest only this amenity type")
    parser.add_argument("--all", action="store_true", help="All countries in COUNTRY_BBOX")
    parser.add_argument("--delay", type=float, default=5.0,
                        help="Seconds between Overpass requests (default: 5)")
    args = parser.parse_args()

    if args.all:
        countries = list(COUNTRY_BBOX.keys())
    elif args.countries:
        countries = args.countries
    else:
        parser.print_help()
        sys.exit(1)

    amenities = [args.amenity] if args.amenity else AMENITIES

    invalid = [c for c in countries if c not in COUNTRY_BBOX]
    if invalid:
        print(f"ERROR: unknown country codes: {invalid}")
        print(f"Known: {sorted(COUNTRY_BBOX.keys())}")
        sys.exit(1)

    # Load existing records from output file (if any) to merge
    existing = []
    if OUTPUT_FILE.exists():
        with open(OUTPUT_FILE) as f:
            for line in f:
                try:
                    existing.append(json.loads(line))
                except Exception:
                    pass
        print(f"Loaded {len(existing)} existing records from {OUTPUT_FILE.name}")

    # Build set of (iso, amenity) already present — skip re-fetching those
    existing_keys = set()
    for r in existing:
        existing_keys.add((r.get("iso_country_code",""), r.get("category_id","")))

    new_records = []
    first = True
    for iso in countries:
        bbox = COUNTRY_BBOX[iso]
        for amenity in amenities:
            if (iso, amenity) in existing_keys:
                print(f"  [{iso}] amenity={amenity} — already in file, skipping")
                continue
            if not first:
                time.sleep(args.delay)
            first = False
            recs = ingest_country_amenity(amenity, iso, bbox)
            new_records.extend(recs)

    all_records = existing + new_records
    print(f"\nTotal records: {len(all_records)} ({len(existing)} existing + {len(new_records)} new)")

    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_FILE, "w") as f:
        for rec in all_records:
            f.write(json.dumps(rec) + "\n")
    print(f"Written → {OUTPUT_FILE}")

    # Summary by category
    by_cat = {}
    for r in all_records:
        k = (r.get("iso_country_code","?"), r.get("category_id","?"))
        by_cat[k] = by_cat.get(k, 0) + 1
    print("\nBreakdown:")
    for (iso, cat), n in sorted(by_cat.items()):
        print(f"  {iso} {cat}: {n}")


if __name__ == "__main__":
    main()
