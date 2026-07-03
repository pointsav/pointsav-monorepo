#!/usr/bin/env python3
"""
discover-na-aerodromes.py — discover aerodromes directly from OSM across
Canada + US + Mexico, for the full Aviation Fringe NA sweep (Part C).

Why this exists (2026-07-03): the pre-existing cleansed-civic-airports.jsonl
dataset was checked directly and found to be missing CYBW (Springbank) — the
exact airport that established the >=5-hangar Aviation Fringe rule — and only
246 of 4,346 records lack an IATA code, meaning the dataset systematically
excludes small regional GA airports (exactly what Aviation Fringe targets).
This script discovers candidates directly from OSM instead, per NA sub-region
bbox (reusing sim-urban-fringe.py's REGIONS dict), so the candidate list is
genuinely complete rather than filtered by IATA/commercial status.

Query: aeroway=aerodrome + aeroway=airfield (nodes and ways), excluding
aeroway=heliport. Output: one JSON file per region under
work/na-sweep-logs/aerodromes-<region>.json, each a list of
{icao, name, lat, lon} — icao may be null if OSM doesn't tag one, since the
downstream Aviation Fringe profile only needs lat/lon.

Usage:
  python3 discover-na-aerodromes.py --region US_PACIFIC
  python3 discover-na-aerodromes.py --all-na    # all 18 Canada+US+Mexico regions
"""

import argparse
import json
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

# Reuse the exact NA region bboxes from sim-urban-fringe.py (Canada + US + Mexico
# only — the EU regions there are irrelevant to this NA-only discovery pass).
NA_REGIONS = {
    "AB":       (49.0, -120.0, 60.0, -110.0),
    "BC":       (48.3, -139.0, 60.0, -120.0),
    "SK":       (49.0, -110.0, 60.0, -101.5),
    "MB":       (49.0, -101.5, 60.0,  -95.0),
    "ON":       (41.6,  -95.2, 56.9,  -74.3),
    "QC":       (45.0,  -79.8, 62.6,  -57.1),
    "ATL":      (43.3,  -69.1, 60.4,  -52.6),
    "US_PACIFIC":       (32.5, -125.0, 49.0, -114.0),
    "US_MOUNTAIN":      (31.3, -120.0, 49.0, -102.0),
    "US_CENTRAL_NORTH": (37.0, -104.1, 49.4,  -89.5),
    "US_CENTRAL_SOUTH": (25.8, -106.6, 37.0,  -89.0),
    "US_MIDWEST":       (37.0,  -92.9, 48.3,  -80.5),
    "US_SOUTHEAST":     (24.5,  -91.7, 39.1,  -75.5),
    "US_NORTHEAST":     (38.9,  -80.5, 47.5,  -66.9),
    "US_MIDATLANTIC":   (36.5,  -83.7, 40.6,  -75.0),
    "MX_NORTH":   (24.0, -118.0, 32.7, -97.0),
    "MX_CENTRAL": (19.0, -105.5, 24.0, -96.0),
    "MX_SOUTH":   (14.5,  -99.5, 19.0, -86.7),
}

OVERPASS = "https://overpass-api.de/api/interpreter"
# 2026-07-03: throttled per operator direction — no daily cap, but substantially
# slower than any prior sweep this session, to avoid overloading Overpass.
QUERY_SLEEP = 25
OUT_DIR = Path("work/na-sweep-logs")


def fetch_overpass(q, label, retries=3):
    data = urllib.parse.urlencode({"data": q}).encode()
    req = urllib.request.Request(OVERPASS, data=data, method="POST", headers={
        "Content-Type": "application/x-www-form-urlencoded",
        "User-Agent": "foundry-gis-sim/1.0",
    })
    for attempt in range(retries):
        try:
            with urllib.request.urlopen(req, timeout=280) as r:
                return json.loads(r.read())
        except urllib.error.HTTPError as e:
            if e.code in (429, 502, 503, 504) and attempt < retries - 1:
                wait = 30 * (2 ** attempt)  # 30s/60s/120s
                print(f"  {e.code}, retrying in {wait}s...", end=" ", flush=True)
                time.sleep(wait)
                continue
            print(f"  WARN {label}: {e}", file=sys.stderr)
            return {"elements": []}
        except Exception as e:
            print(f"  WARN {label}: {e}", file=sys.stderr)
            return {"elements": []}
    return {"elements": []}


def get_coord(el):
    if el["type"] == "node":
        return el.get("lat"), el.get("lon")
    c = el.get("center", {})
    return c.get("lat"), c.get("lon")


def discover_region(region: str, bbox: tuple) -> list:
    bbox_str = f"{bbox[0]},{bbox[1]},{bbox[2]},{bbox[3]}"
    q = f"""
[out:json][timeout:240];
(
  node["aeroway"="aerodrome"]({bbox_str});
  way["aeroway"="aerodrome"]({bbox_str});
  node["aeroway"="airfield"]({bbox_str});
  way["aeroway"="airfield"]({bbox_str});
);
out center tags;
"""
    print(f"  querying aerodromes+airfields...", end=" ", flush=True)
    res = fetch_overpass(q, f"aerodromes:{region}")
    sites = []
    seen = set()
    for el in res.get("elements", []):
        tags = el.get("tags", {})
        if tags.get("aeroway") == "heliport":
            continue
        lat, lon = get_coord(el)
        if lat is None or lon is None:
            continue
        key = (round(lat, 4), round(lon, 4))
        if key in seen:
            continue
        seen.add(key)
        sites.append({
            "icao": tags.get("icao"),
            "iata": tags.get("iata"),
            "name": tags.get("name") or tags.get("icao") or f"unnamed_{key[0]}_{key[1]}",
            "lat": lat,
            "lon": lon,
            "aeroway": tags.get("aeroway"),
        })
    print(f"{len(sites)} aerodromes/airfields discovered")
    return sites


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--region", choices=list(NA_REGIONS.keys()))
    ap.add_argument("--all-na", action="store_true",
                     help="Run all 18 Canada+US+Mexico regions sequentially")
    args = ap.parse_args()

    if args.all_na:
        regions = list(NA_REGIONS.keys())
    elif args.region:
        regions = [args.region]
    else:
        ap.print_help()
        sys.exit(1)

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    total = 0
    for i, region in enumerate(regions):
        print(f"\n=== Region: {region} ===")
        sites = discover_region(region, NA_REGIONS[region])
        out = OUT_DIR / f"aerodromes-{region.lower()}.json"
        out.write_text(json.dumps(sites, indent=2))
        print(f"  -> {out}")
        total += len(sites)
        if i < len(regions) - 1:
            time.sleep(QUERY_SLEEP)

    print(f"\nTotal discovered across {len(regions)} region(s): {total}")


if __name__ == "__main__":
    main()
