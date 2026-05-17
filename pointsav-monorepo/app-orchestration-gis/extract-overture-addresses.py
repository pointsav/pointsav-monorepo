#!/usr/bin/env python3
"""
extract-overture-addresses.py — Overture Maps Addresses → service-business backfill

Downloads and queries the Overture Maps Foundation Addresses theme from S3 using
DuckDB, then backfills null street_address fields in service-business JSONL records
using a spatial proximity lookup (H3 res-11, ~15m cell edge).

Overture Addresses licence: ODbL 1.0 (same as OSM).
Attribution required: © Overture Maps Foundation contributors.

S3 path: s3://overturemaps-us-west-2/release/{version}/theme=addresses/type=address/*.parquet

Usage:
    # Backfill all countries, write {chain_id}-addressed.jsonl per chain:
    python3 extract-overture-addresses.py

    # Backfill specific countries:
    python3 extract-overture-addresses.py --countries CA ES

    # Sample mode — query 1 country, report stats, no file writes:
    python3 extract-overture-addresses.py --sample --countries CA

    # Output a standalone lookup JSON instead of per-chain files:
    python3 extract-overture-addresses.py --lookup-only

    # Override Overture release version:
    python3 extract-overture-addresses.py --version 2026-04-15.0
"""

import argparse
import json
import sys
from pathlib import Path

import h3
import duckdb

sys.path.insert(0, str(Path(__file__).parent))
from config import TOTEBOX_DATA_PATH

# ── Overture release — keep in sync with ingest-overture.py ──────────────────
OVERTURE_RELEASE = "2026-04-15.0"
OVERTURE_ADDRESSES_S3 = (
    "s3://overturemaps-us-west-2/release/{version}/theme=addresses/type=address/*.parquet"
)

# ── H3 resolution ─────────────────────────────────────────────────────────────
# Res-11 average hexagon edge length ≈ 24m; cell-to-cell distance ≤ ~53m.
# Matching on the same cell + 1-ring neighbours gives an effective ~15m radius.
H3_RES = 11

# ── Target countries (ISO-2) ──────────────────────────────────────────────────
# Primary 13 markets + Nordic expansion markets.
PRIMARY_COUNTRIES = ["US", "CA", "MX", "ES", "FR", "DE", "GB", "IT", "NL", "AT", "PL", "GR", "PT"]
NORDIC_COUNTRIES  = ["SE", "NO", "DK", "FI", "IS"]
ALL_COUNTRIES     = PRIMARY_COUNTRIES + NORDIC_COUNTRIES

# Country → approximate bounding box (minlon, minlat, maxlon, maxlat)
# Aligns with ingest-osm.py COUNTRY_BBOX (lat_min, lon_min, lat_max, lon_max → reordered).
COUNTRY_BBOX = {
    "US": (-125.0,  24.0,  -65.0,  50.0),
    "CA": (-141.0,  41.0,  -52.0,  84.0),
    "MX": (-118.0,  14.0,  -86.0,  33.0),
    "ES": (  -9.5,  35.0,    4.5,  44.0),
    "FR": (  -5.5,  41.0,   10.0,  51.5),
    "DE": (   6.0,  47.0,   15.5,  55.5),
    "GB": (  -8.5,  49.5,    2.0,  61.5),
    "IT": (   6.5,  36.0,   18.5,  47.5),
    "NL": (   3.3,  50.7,    7.3,  53.7),
    "AT": (   9.5,  46.5,   17.2,  49.0),
    "PL": (  14.0,  49.0,   24.5,  55.0),
    "GR": (  19.5,  34.5,   26.5,  42.0),
    "PT": (  -9.5,  36.5,   -6.0,  42.5),
    "SE": (  10.5,  55.0,   24.5,  69.5),
    "NO": (   3.5,  57.0,   31.5,  71.5),
    "DK": (   7.5,  54.5,   15.5,  58.0),
    "FI": (  19.5,  59.5,   31.5,  70.5),
    "IS": ( -25.0,  63.0,  -13.0,  66.5),
}

# ── Paths ─────────────────────────────────────────────────────────────────────
SERVICE_BUSINESS_DIR = TOTEBOX_DATA_PATH / "service-fs" / "service-business"
LOOKUP_OUTPUT        = SERVICE_BUSINESS_DIR / "overture-address-lookup.json"

# ── Address confidence assigned to Overture-backfilled records ────────────────
ADDRESS_CONFIDENCE = 0.90


# ── DuckDB helpers ────────────────────────────────────────────────────────────

def _init_con() -> duckdb.DuckDBPyConnection:
    """Create a DuckDB connection with httpfs + spatial extensions loaded."""
    con = duckdb.connect()
    con.execute("INSTALL httpfs; LOAD httpfs;")
    con.execute("INSTALL spatial; LOAD spatial;")
    con.execute("SET s3_region='us-west-2';")
    return con


def fetch_addresses_for_country(con: duckdb.DuckDBPyConnection,
                                country: str,
                                version: str,
                                sample_limit: int | None = None) -> list[dict]:
    """
    Query Overture Addresses parquet for a single country within its bounding box.

    Returns a list of dicts with keys:
        id, latitude, longitude, street, number, postcode, city, region, country
    """
    bbox = COUNTRY_BBOX.get(country)
    if bbox is None:
        print(f"  [WARN] No bounding box for {country} — skipping")
        return []

    minlon, minlat, maxlon, maxlat = bbox
    s3_path = OVERTURE_ADDRESSES_S3.format(version=version)
    limit_clause = f"LIMIT {sample_limit}" if sample_limit else ""

    # Overture Addresses schema (2026-04-15.0):
    #   street      VARCHAR        — street name (top-level column)
    #   number      VARCHAR        — house number (top-level column)
    #   postcode    VARCHAR        — postal code
    #   postal_city VARCHAR        — city (often null for rural; fall back to address_levels)
    #   address_levels STRUCT(value VARCHAR)[]  — [0] = locality/municipality
    #   country     VARCHAR        — ISO-2 country code
    # There is no dedicated region/state column in this release.
    query = f"""
    SELECT
        id,
        ST_Y(geometry)                                  AS latitude,
        ST_X(geometry)                                  AS longitude,
        street,
        number,
        postcode,
        COALESCE(postal_city, address_levels[1].value)  AS city,
        NULL                                            AS region,
        country
    FROM read_parquet('{s3_path}', hive_partitioning=false)
    WHERE country = '{country}'
      AND ST_X(geometry) BETWEEN {minlon} AND {maxlon}
      AND ST_Y(geometry) BETWEEN {minlat} AND {maxlat}
    {limit_clause}
    """

    try:
        rows = con.execute(query).fetchall()
    except Exception as e:
        print(f"  [ERROR] DuckDB query for {country} failed: {e}")
        return []

    cols = ["id", "latitude", "longitude", "street", "number", "postcode", "city", "region", "country"]
    return [dict(zip(cols, row)) for row in rows]


# ── Spatial lookup builders ───────────────────────────────────────────────────

def build_h3_lookup(addresses: list[dict]) -> dict[str, dict]:
    """
    Index addresses by H3 res-11 cell string.

    When multiple addresses share a cell (possible in dense areas), the first
    one with a non-null street value is retained.

    Returns {h3_cell: address_dict}.
    """
    lookup: dict[str, dict] = {}
    for addr in addresses:
        lat = addr.get("latitude")
        lon = addr.get("longitude")
        if lat is None or lon is None:
            continue
        try:
            cell = h3.latlng_to_cell(lat, lon, H3_RES)
        except Exception:
            continue
        if cell not in lookup:
            lookup[cell] = addr
        elif not lookup[cell].get("street") and addr.get("street"):
            # Prefer the entry that carries a street name
            lookup[cell] = addr
    return lookup


def find_nearest_address(lat: float, lon: float,
                         lookup: dict[str, dict]) -> dict | None:
    """
    Return the nearest Overture address to (lat, lon) using H3 k-ring search.

    Strategy:
      1. Exact cell match (same ~24m hex).
      2. k=1 ring (6 neighbours; effective radius ≤ ~53m edge-to-edge).

    Returns the address dict or None.
    """
    try:
        cell = h3.latlng_to_cell(lat, lon, H3_RES)
    except Exception:
        return None

    # Exact hit
    if cell in lookup:
        return lookup[cell]

    # Neighbours (k=1)
    for neighbour in h3.grid_disk(cell, 1):
        if neighbour in lookup:
            return lookup[neighbour]

    return None


# ── JSONL I/O ─────────────────────────────────────────────────────────────────

def load_jsonl(path: Path) -> list[dict]:
    records = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                try:
                    records.append(json.loads(line))
                except json.JSONDecodeError:
                    pass
    return records


def write_jsonl(path: Path, records: list[dict]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w") as f:
        for rec in records:
            f.write(json.dumps(rec) + "\n")


# ── Address field formatting ──────────────────────────────────────────────────

def format_street_address(addr: dict) -> str | None:
    """Combine number + street into a single street_address string."""
    number = (addr.get("number") or "").strip()
    street = (addr.get("street") or "").strip()
    if number and street:
        return f"{number} {street}"
    if street:
        return street
    if number:
        return number
    return None


# ── Core backfill logic ───────────────────────────────────────────────────────

def backfill_jsonl_file(jsonl_path: Path,
                        lookup: dict[str, dict],
                        sample: bool = False) -> dict:
    """
    Read a service-business JSONL file, backfill null street_address records,
    and write {chain_id}-addressed.jsonl alongside the original.

    Returns stats dict: {total, null_before, backfilled, null_after}.
    """
    records = load_jsonl(jsonl_path)
    if not records:
        return {"total": 0, "null_before": 0, "backfilled": 0, "null_after": 0}

    null_before = sum(1 for r in records if not r.get("street_address"))
    backfilled  = 0
    sample_pairs: list[dict] = []  # before/after examples for --sample output

    for rec in records:
        if rec.get("street_address"):
            continue  # already has an address — skip

        lat = rec.get("latitude")
        lon = rec.get("longitude")
        if lat is None or lon is None:
            continue

        addr = find_nearest_address(lat, lon, lookup)
        if addr is None:
            continue

        street_address = format_street_address(addr)

        before_snapshot = {k: rec.get(k) for k in
                           ("street_address", "city", "region", "postal_code")}

        rec["street_address"] = street_address
        if not rec.get("city"):
            rec["city"] = addr.get("city") or rec.get("city")
        if not rec.get("region"):
            rec["region"] = addr.get("region") or rec.get("region")
        if not rec.get("postal_code"):
            rec["postal_code"] = addr.get("postcode") or rec.get("postal_code")
        rec["address_source"]     = "overture_addresses"
        rec["address_confidence"] = ADDRESS_CONFIDENCE

        backfilled += 1
        if len(sample_pairs) < 3:
            sample_pairs.append({
                "chain_id":   rec.get("chain_id"),
                "location":   rec.get("location_name"),
                "lat":        lat,
                "lon":        lon,
                "before":     before_snapshot,
                "after": {
                    "street_address": rec["street_address"],
                    "city":           rec["city"],
                    "region":         rec["region"],
                    "postal_code":    rec["postal_code"],
                },
            })

    null_after = sum(1 for r in records if not r.get("street_address"))

    if not sample:
        out_path = jsonl_path.parent / (jsonl_path.stem + "-addressed.jsonl")
        write_jsonl(out_path, records)

    return {
        "total":       len(records),
        "null_before": null_before,
        "backfilled":  backfilled,
        "null_after":  null_after,
        "sample_pairs": sample_pairs,
    }


# ── Lookup-only output ────────────────────────────────────────────────────────

def emit_lookup_json(lookup: dict[str, dict], path: Path) -> None:
    """
    Write a flat JSON keyed by H3 res-11 cell string.

    Format:
      { "<h3cell>": { "street": "...", "number": "...", "postcode": "...",
                      "city": "...", "region": "...", "country": "...",
                      "lat": <float>, "lon": <float> }, ... }

    This file can be consumed by other pipeline scripts without re-querying S3.
    """
    out = {}
    for cell, addr in lookup.items():
        out[cell] = {
            "street":  addr.get("street"),
            "number":  addr.get("number"),
            "postcode": addr.get("postcode"),
            "city":    addr.get("city"),
            "region":  addr.get("region"),
            "country": addr.get("country"),
            "lat":     addr.get("latitude"),
            "lon":     addr.get("longitude"),
        }
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w") as f:
        json.dump(out, f, separators=(",", ":"))
    print(f"  Lookup written → {path} ({len(out):,} cells)")


# ── Main ──────────────────────────────────────────────────────────────────────

def main() -> None:
    parser = argparse.ArgumentParser(
        description="Backfill null street_address fields using Overture Addresses theme"
    )
    parser.add_argument(
        "--countries", nargs="+", default=ALL_COUNTRIES,
        metavar="CC",
        help="ISO-2 country codes to process (default: all 18 markets)"
    )
    parser.add_argument(
        "--version", default=OVERTURE_RELEASE,
        help=f"Overture release version (default: {OVERTURE_RELEASE})"
    )
    parser.add_argument(
        "--sample", action="store_true",
        help="Dry-run: query S3, report stats and before/after examples, no file writes"
    )
    parser.add_argument(
        "--sample-limit", type=int, default=50_000,
        metavar="N",
        help="Max rows to fetch per country in --sample mode (default: 50000)"
    )
    parser.add_argument(
        "--lookup-only", action="store_true",
        help="Build and write overture-address-lookup.json only; skip JSONL backfill"
    )
    args = parser.parse_args()

    countries  = [c.upper() for c in args.countries]
    version    = args.version
    sample     = args.sample
    limit      = args.sample_limit if sample else None

    print(f"Overture Addresses backfill — release {version}")
    print(f"Countries: {', '.join(countries)}")
    if sample:
        print(f"Mode: SAMPLE (limit {limit:,} rows/country; no file writes)")
    elif args.lookup_only:
        print("Mode: LOOKUP-ONLY")
    else:
        print("Mode: FULL BACKFILL")

    # ── Phase 1: fetch addresses from Overture S3 ─────────────────────────────
    print("\n── Phase 1: Fetch Overture Addresses ────────────────────────────────")
    try:
        con = _init_con()
    except Exception as e:
        print(f"[FATAL] Cannot initialise DuckDB: {e}")
        print("  Install: pip install duckdb")
        sys.exit(1)

    combined_lookup: dict[str, dict] = {}
    country_counts: dict[str, int] = {}

    for country in countries:
        print(f"\n  [{country}] querying S3...", flush=True)
        addresses = fetch_addresses_for_country(con, country, version, sample_limit=limit)
        print(f"  [{country}] fetched {len(addresses):,} address records")
        country_counts[country] = len(addresses)

        # Build / merge into combined lookup
        country_lookup = build_h3_lookup(addresses)
        combined_lookup.update(country_lookup)
        print(f"  [{country}] {len(country_lookup):,} unique H3 cells indexed")

    con.close()
    print(f"\nTotal address cells in lookup: {len(combined_lookup):,}")

    # ── Phase 2: lookup-only output ───────────────────────────────────────────
    if args.lookup_only:
        print("\n── Phase 2: Write lookup JSON ────────────────────────────────────────")
        emit_lookup_json(combined_lookup, LOOKUP_OUTPUT)
        print("\nDone.")
        return

    # ── Phase 2: backfill JSONL files ─────────────────────────────────────────
    print("\n── Phase 2: Backfill service-business JSONL ─────────────────────────")
    jsonl_files = sorted(SERVICE_BUSINESS_DIR.glob("*.jsonl"))

    # In sample mode we also skip files already suffixed with -addressed
    jsonl_files = [p for p in jsonl_files if not p.name.endswith("-addressed.jsonl")]

    if not jsonl_files:
        print(f"  No JSONL files found in {SERVICE_BUSINESS_DIR}")
        print("  Run ingest-osm.py first to populate service-business/.")
        return

    # Filter to files whose chain_id country suffix matches requested countries
    # chain_id convention: <chain>-<iso2>  (e.g. ikea-ca, costco-us)
    def country_matches(path: Path) -> bool:
        stem = path.stem  # e.g. "ikea-ca"
        parts = stem.rsplit("-", 1)
        if len(parts) < 2:
            return True  # no country suffix — include
        suffix = parts[-1].upper()
        return suffix in countries

    target_files = [p for p in jsonl_files if country_matches(p)]
    skip_count   = len(jsonl_files) - len(target_files)
    if skip_count:
        print(f"  Skipping {skip_count} file(s) not in requested country set")

    total_stats = {
        "files":       0,
        "total":       0,
        "null_before": 0,
        "backfilled":  0,
        "null_after":  0,
    }
    all_sample_pairs: list[dict] = []

    for path in target_files:
        print(f"  {path.name}...", end=" ", flush=True)
        stats = backfill_jsonl_file(path, combined_lookup, sample=sample)
        pct = (stats["backfilled"] / stats["null_before"] * 100
               if stats["null_before"] else 0.0)
        print(
            f"total={stats['total']}  null_before={stats['null_before']}  "
            f"backfilled={stats['backfilled']} ({pct:.0f}%)  "
            f"null_after={stats['null_after']}"
        )
        total_stats["files"]       += 1
        total_stats["total"]       += stats["total"]
        total_stats["null_before"] += stats["null_before"]
        total_stats["backfilled"]  += stats["backfilled"]
        total_stats["null_after"]  += stats["null_after"]
        all_sample_pairs.extend(stats.get("sample_pairs", []))

    # ── Summary ───────────────────────────────────────────────────────────────
    print("\n── Summary ───────────────────────────────────────────────────────────")
    print(f"  Files processed : {total_stats['files']}")
    print(f"  Total records   : {total_stats['total']:,}")
    print(f"  Null before     : {total_stats['null_before']:,}")
    print(f"  Backfilled      : {total_stats['backfilled']:,}")
    fill_rate = (total_stats["backfilled"] / total_stats["null_before"] * 100
                 if total_stats["null_before"] else 0.0)
    print(f"  Fill rate       : {fill_rate:.1f}%")
    print(f"  Still null      : {total_stats['null_after']:,}")

    if sample and all_sample_pairs:
        print("\n── Sample before/after comparisons ──────────────────────────────────")
        for i, pair in enumerate(all_sample_pairs[:6], 1):
            print(f"\n  [{i}] {pair.get('chain_id')} — {pair.get('location')} "
                  f"({pair.get('lat'):.5f}, {pair.get('lon'):.5f})")
            before = pair.get("before", {})
            after  = pair.get("after",  {})
            for field in ("street_address", "city", "region", "postal_code"):
                b = before.get(field)
                a = after.get(field)
                if b != a:
                    print(f"    {field:15s}  {str(b)!r:30s} → {str(a)!r}")

    if sample:
        print("\n[SAMPLE MODE — no files written]")
    else:
        print(f"\nOutput: {SERVICE_BUSINESS_DIR}/<chain>-addressed.jsonl")
        print("  address_source='overture_addresses'  address_confidence=0.90")

    print("\nDone.")


if __name__ == "__main__":
    main()
