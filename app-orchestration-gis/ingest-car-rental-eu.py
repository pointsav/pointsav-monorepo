#!/usr/bin/env python3
"""
ingest-car-rental-eu.py — EU car rental brand ingests for PKS enrichment.

Queries Overpass by brand:wikidata=<ID> for each brand across EU country bboxes.
Writes to service-fs/service-business/<chain_id>.jsonl (flat, same format as existing files).

Usage:
    python3 ingest-car-rental-eu.py               # all brands
    python3 ingest-car-rental-eu.py --brands sixt hertz
    python3 ingest-car-rental-eu.py --dry-run
"""

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import SERVICE_BUSINESS as _SB_CONFIG

# build-pks-clusters.py uses service-fs/service-business/ directly; config.py points one level up
# Use service-fs/ subpath to match where all chain JSONLs actually live
_TOTEBOX = _SB_CONFIG.parent  # cluster-totebox-personnel-1/
OUT_DIR = _TOTEBOX / "service-fs" / "service-business"
if not OUT_DIR.exists():
    # fallback: use config path directly
    OUT_DIR = _SB_CONFIG

OVERPASS_URLS = [
    "https://overpass-api.de/api/interpreter",
    "https://overpass.kumi.systems/api/interpreter",
    "https://overpass.private.coffee/api/interpreter",
]

# EU country bboxes (lat_min, lon_min, lat_max, lon_max)
# Split W/E to avoid Overpass timeouts on large queries
EU_BBOXES = [
    ("EU-W", 34.0, -25.0, 72.0, 10.0),   # Western Europe (incl GB, IE, IS, FR, ES, PT, IT-W)
    ("EU-E", 34.0,  10.0, 72.0, 45.0),   # Eastern Europe (DE, AT, PL, SE, FI, IT-E, GR, etc.)
]

# Brands to ingest: (chain_id, wikidata_id, display_name, naics, approx_count)
BRANDS = {
    "sixt-eu": {
        "wikidata_id": "Q704156",
        "display_name": "Sixt",
        "canonical_name": "Sixt SE",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
    },
    "hertz-eu": {
        "wikidata_id": "Q379425",
        "display_name": "Hertz",
        "canonical_name": "The Hertz Corporation",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
    },
    "avis-eu": {
        "wikidata_id": "Q849144",
        "display_name": "Avis",
        "canonical_name": "Avis Budget Group, Inc.",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
    },
    "budget-eu": {
        "wikidata_id": "Q1004913",
        "display_name": "Budget",
        "canonical_name": "Budget Rent a Car",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
    },
    "europcar-eu": {
        "wikidata_id": "Q466704",
        "display_name": "Europcar",
        "canonical_name": "Europcar Mobility Group SA",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
    },
    "enterprise-eu": {
        "wikidata_id": "Q1307252",
        "display_name": "Enterprise Rent-A-Car",
        "canonical_name": "Enterprise Holdings, Inc.",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
    },
    # MX-only brands (Phase 3, 2026-06-14)
    # NOTE: OSM Mexico taggers use different Wikidata IDs than EU canonical brands.
    # These IDs are derived by inspecting amenity=car_rental tags in the MX bbox.
    "hertz-mx": {
        "wikidata_id": "Q1543874",  # Hertz as tagged in OSM MX (133 nodes)
        "display_name": "Hertz",
        "canonical_name": "Hertz de México",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
        "_na_bbox": (14.5, -118.4, 32.7, -86.7),
        "_iso_fallback": "MX",  # OSM MX nodes rarely have addr:country tag
    },
    "enterprise-mx": {
        "wikidata_id": "Q17085454",  # Enterprise as tagged in OSM MX (50 nodes)
        "display_name": "Enterprise",
        "canonical_name": "Enterprise Renta de Autos",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
        "_na_bbox": (14.5, -118.4, 32.7, -86.7),
        "_iso_fallback": "MX",
    },
    "avis-mx": {
        "wikidata_id": "Q791136",  # Avis Budget Group as tagged in OSM MX (40 nodes)
        "display_name": "Avis",
        "canonical_name": "Avis Renta de Autos",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
        "_na_bbox": (14.5, -118.4, 32.7, -86.7),
        "_iso_fallback": "MX",
    },
    "budget-mx": {
        "wikidata_id": "Q1001437",  # Budget as tagged in OSM MX (29 nodes)
        "display_name": "Budget",
        "canonical_name": "Budget Renta Car México",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
        "_na_bbox": (14.5, -118.4, 32.7, -86.7),
        "_iso_fallback": "MX",
    },
    # NA brands not yet ingested
    "budget-us": {
        "wikidata_id": "Q1004913",
        "display_name": "Budget",
        "canonical_name": "Budget Rent a Car",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
        "_na_bbox": (14.0, -141.0, 84.0, -52.0),  # US+CA+MX combined
    },
    "alamo-us": {
        "wikidata_id": "Q1005603",
        "display_name": "Alamo",
        "canonical_name": "Alamo Rent A Car",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
        "_na_bbox": (14.0, -141.0, 84.0, -52.0),
    },
    "national-us": {
        "wikidata_id": "Q1978597",
        "display_name": "National Car Rental",
        "canonical_name": "National Car Rental (Enterprise Holdings)",
        "naics_code": "532111",
        "top_category": "Passenger Car Rental",
        "sub_category": "Car Rental",
        "_na_bbox": (14.0, -141.0, 84.0, -52.0),
    },
}


def overpass_query(wikidata_id: str, bbox: tuple, timeout: int = 90) -> list:
    lat_min, lon_min, lat_max, lon_max = bbox
    query = (
        f"[out:json][timeout:{timeout}];\n"
        f"(\n"
        f'  node["brand:wikidata"="{wikidata_id}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  way["brand:wikidata"="{wikidata_id}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
        f'  relation["brand:wikidata"="{wikidata_id}"]({lat_min},{lon_min},{lat_max},{lon_max});\n'
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
        "brand_wikidata": brand["wikidata_id"],
        "location_name": tags.get("name") or tags.get("brand") or brand["display_name"],
        "brands": [{"brand_name": brand["display_name"], "brand_wikidata": brand["wikidata_id"]}],
        "street_address": tags.get("addr:housenumber", "") + " " + tags.get("addr:street", "") or None,
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
        "last_updated": "2026-06-11",
        "osm_element_type": elem["type"],
    }


def ingest_brand(chain_id: str, brand: dict, dry_run: bool = False) -> int:
    print(f"\n[{chain_id}] wikidata={brand['wikidata_id']}")
    out_path = OUT_DIR / f"{chain_id}.jsonl"

    # Determine bboxes to query
    na_bbox = brand.get("_na_bbox")
    if na_bbox:
        bboxes = [("NA", *na_bbox)]
    else:
        bboxes = EU_BBOXES

    all_elements = []
    for region_name, lat_min, lon_min, lat_max, lon_max in bboxes:
        print(f"  querying {region_name} bbox ({lat_min},{lon_min},{lat_max},{lon_max})...")
        try:
            elems = overpass_query(brand["wikidata_id"], (lat_min, lon_min, lat_max, lon_max))
            print(f"    → {len(elems)} elements")
            all_elements.extend(elems)
            time.sleep(2)
        except RuntimeError as e:
            print(f"    ERROR: {e}")
            return 0

    print(f"  total elements: {len(all_elements)}")

    # Convert to records
    records = []
    for elem in all_elements:
        rec = elem_to_record(elem, chain_id, brand)
        if rec:
            records.append(rec)

    # Dedup by coordinate (~11m)
    deduped = []
    seen = set()
    for rec in records:
        key = (round(rec["latitude"], 4), round(rec["longitude"], 4))
        if key not in seen:
            deduped.append(rec)
            seen.add(key)

    dropped = len(records) - len(deduped)
    if dropped:
        print(f"  deduped: dropped {dropped} duplicate coordinates")
    print(f"  records to write: {len(deduped)}")

    if dry_run:
        print(f"  DRY RUN — would write {len(deduped)} records to {out_path}")
        return len(deduped)

    if not deduped:
        print(f"  no records — skipping write")
        return 0

    with open(out_path, "w") as f:
        for rec in deduped:
            f.write(json.dumps(rec) + "\n")
    print(f"  wrote {len(deduped)} records → {out_path}")
    return len(deduped)


def main():
    parser = argparse.ArgumentParser(description="Ingest EU car rental brands from OSM")
    parser.add_argument("--brands", nargs="+", choices=list(BRANDS.keys()),
                        help="Specific brands to ingest (default: all)")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    brands_to_run = args.brands or list(BRANDS.keys())
    total = 0
    for chain_id in brands_to_run:
        brand = BRANDS[chain_id]
        n = ingest_brand(chain_id, brand, dry_run=args.dry_run)
        total += n

    print(f"\nTotal records ingested: {total}")


if __name__ == "__main__":
    main()
