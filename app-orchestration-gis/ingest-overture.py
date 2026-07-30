#!/usr/bin/env python3
"""
ingest-overture.py — Overture Maps Foundation → service-places JSONL

Downloads and queries the Overture Maps Foundation Places GeoParquet dataset
using DuckDB, clips to the union bounding box of all cluster catchment radii,
filters to hospital / university / airport categories, and writes
service-places/locations/*.jsonl in the Totebox.

Overture data licence: CDLA Permissive 2.0 (commercial use OK; attribution in
map footer required: "© Overture Maps Foundation").

Schema note: Overture deprecated `categories` in the 2025-11 release and removed
it in the 2026-06 release. This script uses the replacement `taxonomy` field
(taxonomy.primary / taxonomy.alternate) introduced in 2025-11. If pinned to a
pre-2025-11 release, revert filters to `categories.primary`.

Usage:
    python3 ingest-overture.py [--bbox "minlon,minlat,maxlon,maxlat"]
    python3 ingest-overture.py --category hospital
    python3 ingest-overture.py --all

The bounding box defaults to the union of all cluster catchment radii once
build-clusters.py has run. On first run (no clusters yet), pass --bbox manually
or use --all to ingest globally (large download).
"""

import argparse
import json
import sys
from pathlib import Path

import duckdb

sys.path.insert(0, str(Path(__file__).parent))
from config import SERVICE_PLACES, TOTEBOX_DATA_PATH

# Overture Maps Foundation S3 path (us-west-2, requester-pays = false for Places)
OVERTURE_PLACES_S3 = "s3://overturemaps-us-west-2/release/2026-04-15.0/theme=places/type=place/*.parquet"

CATEGORY_FILTERS = {
    "hospital":   "taxonomy.primary IN ('hospital', 'childrens_hospital')",
    "university": "taxonomy.primary = 'college_university'",
    "airport":    "taxonomy.primary IN ('airport', 'major_airports', 'domestic_airports')",
}

CATEGORY_NAICS = {
    "hospital":   {"naics_code": "622110",
                   "top_category": "General Medical and Surgical Hospitals",
                   "sub_category": "General Medical and Surgical Hospitals"},
    "university": {"naics_code": "611310",
                   "top_category": "Colleges, Universities, and Professional Schools",
                   "sub_category": "Colleges, Universities, and Professional Schools"},
    "airport":    {"naics_code": "488119",
                   "top_category": "Other Airport Operations",
                   "sub_category": "Other Airport Operations"},
}


def get_cluster_bbox():
    """Read clusters.geojson and return the union bounding box of all catchment radii."""
    clusters_path = Path(__file__).parent / "work" / "clusters.geojson"
    if not clusters_path.exists():
        return None
    with open(clusters_path) as f:
        fc = json.load(f)
    lons, lats = [], []
    for feat in fc.get("features", []):
        coord = feat["geometry"]["coordinates"]
        lons.append(coord[0])
        lats.append(coord[1])
        r_km = feat["properties"].get("catchment_radius_km", 75.0)
        deg = r_km / 111.0
        lons.extend([coord[0] - deg, coord[0] + deg])
        lats.extend([coord[1] - deg, coord[1] + deg])
    if not lons:
        return None
    return (min(lons) - 0.5, min(lats) - 0.5, max(lons) + 0.5, max(lats) + 0.5)


def ingest_category(con, category: str, bbox: tuple, append: bool = False):
    """Query Overture for one category within bbox, write JSONL."""
    out_path = SERVICE_PLACES / "locations" / f"{category}.jsonl"
    out_path.parent.mkdir(parents=True, exist_ok=True)

    cat_filter = CATEGORY_FILTERS[category]
    naics = CATEGORY_NAICS[category]
    minlon, minlat, maxlon, maxlat = bbox

    print(f"  Querying Overture for {category} in bbox {bbox}...")

    query = f"""
    INSTALL httpfs; LOAD httpfs;
    INSTALL spatial; LOAD spatial;
    SET s3_region='us-west-2';

    SELECT
        id AS overture_id,
        COALESCE(brand.names.primary, '{category}') AS location_name,
        brand.wikidata AS brand_wikidata,
        addresses[1].freeform AS street_address,
        addresses[1].locality AS city,
        addresses[1].region AS region,
        addresses[1].postcode AS postal_code,
        addresses[1].country AS iso_country_code,
        ST_X(geometry) AS longitude,
        ST_Y(geometry) AS latitude,
        confidence,
        TRY_CAST(
            json_extract_string(to_json(taxonomy), '$.alternate[0].attributes.beds')
        AS INTEGER) AS beds
    FROM read_parquet('{OVERTURE_PLACES_S3}')
    WHERE {cat_filter}
      AND ST_X(geometry) BETWEEN {minlon} AND {maxlon}
      AND ST_Y(geometry) BETWEEN {minlat} AND {maxlat}
      AND confidence > 0.5
    LIMIT 50000
    """

    try:
        rows = con.execute(query).fetchall()
        cols = ["overture_id", "location_name", "brand_wikidata", "street_address",
                "city", "region", "postal_code", "iso_country_code", "longitude",
                "latitude", "confidence", "beds"]
    except Exception as e:
        print(f"  ERROR querying Overture: {e}")
        print("  Hint: Overture GeoParquet requires S3 access (no auth needed).")
        print("  Ensure the VM has outbound HTTPS to s3.amazonaws.com.")
        return 0

    count = 0
    mode = "a" if append else "w"
    with open(out_path, mode) as f:
        for row in rows:
            rec = dict(zip(cols, row))
            record = {
                "placekey": None,
                "category_id": category,
                "overture_id": rec["overture_id"],
                "brand_wikidata": rec.get("brand_wikidata") or None,
                "location_name": rec["location_name"] or "Unknown",
                "street_address": rec["street_address"],
                "city": rec["city"],
                "region": rec["region"],
                "postal_code": rec["postal_code"],
                "iso_country_code": rec["iso_country_code"],
                "latitude": rec["latitude"],
                "longitude": rec["longitude"],
                "polygon_wkt": None,
                "naics_code": naics["naics_code"],
                "top_category": naics["top_category"],
                "sub_category": naics["sub_category"],
                "operating_status": "open",
                "source": "overture",
                "confidence": rec["confidence"],
                "last_updated": "2026-04-30",
                "bed_count": rec.get("beds"),
            }
            if rec["latitude"] and rec["longitude"]:
                f.write(json.dumps(record) + "\n")
                count += 1

    print(f"  Written {count} records to {out_path}")
    return count


def main():
    parser = argparse.ArgumentParser(description="Ingest Overture service-places data")
    parser.add_argument("--bbox", help="minlon,minlat,maxlon,maxlat")
    parser.add_argument("--category", choices=list(CATEGORY_FILTERS.keys()))
    parser.add_argument("--all", action="store_true", help="Global ingest (large)")
    parser.add_argument("--append", action="store_true", help="Append to existing JSONL (don't overwrite)")
    args = parser.parse_args()

    if args.bbox:
        bbox = tuple(float(x) for x in args.bbox.split(","))
    elif args.all:
        bbox = (-180.0, -90.0, 180.0, 90.0)
    else:
        bbox = get_cluster_bbox()
        if not bbox:
            print("No clusters found. Run build-clusters.py first, or pass --bbox.")
            sys.exit(1)

    categories = [args.category] if args.category else list(CATEGORY_FILTERS.keys())

    con = duckdb.connect()
    total = 0
    for cat in categories:
        print(f"\n[{cat}]")
        count = ingest_category(con, cat, bbox, append=args.append)
        total += count

    print(f"\nDone. {total} total records written to {SERVICE_PLACES}/locations/")

    # Update registry counts
    registry_path = SERVICE_PLACES / "registry.yaml"
    if registry_path.exists():
        text = registry_path.read_text()
        text = text.replace("ingest_status: pending", "ingest_status: partial")
        registry_path.write_text(text)


if __name__ == "__main__":
    main()
