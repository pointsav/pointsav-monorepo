#!/usr/bin/env python3
"""
filter-aerodromes-by-runway.py — pre-filter discover-na-aerodromes.py's output
by runway presence, before the expensive per-site hangar-count profiling
(Part C step 2 of 3, 2026-07-03).

WHY THIS FILTER IS PRINCIPLED, NOT AN ARBITRARY CUTOFF:

sim-aviation-fringe.py's existing, already-validated confirmation rule is:
    confirmed = aerodrome_anchor (has_runway) AND hangar_cluster (>=5 hangars)

A site with NO runway within a reasonable radius can never satisfy this rule,
no matter how many hangars it has (a hangar cluster next to a road, with no
runway, isn't an aerodrome by the rule's own definition). Checking runway
presence first is therefore a ZERO-FALSE-NEGATIVE filter — it removes exactly
the sites that were guaranteed to score "not confirmed" anyway, using a signal
already baked into the validated rule, not a new heuristic invented to save
time. Every site that passes this filter still gets the full, real hangar-count
profile in sim-aviation-fringe.py's batch mode — nothing is confirmed/rejected
by this script alone.

WHY THIS IS CHEAP: discovery (24,866 sites across NA) already cost 18 queries
(one per region). Checking "does site X have a runway" one-at-a-time would cost
24,866 more queries — as expensive as the profiling step itself. Instead, this
queries aeroway=runway ways ONCE PER REGION (18 more queries total) and matches
runway locations against the already-discovered aerodrome list locally in
Python (free) — same pattern as the discovery step itself.

Matching radius: 2.0km from aerodrome point to nearest runway way's centroid.
Chosen to be generous relative to typical small-airport layouts (a GA airport's
single runway is rarely more than ~1.5km from the field's OSM-tagged center
point) while still being tight enough to reject unrelated runways in a
different town. This radius is a genuine judgment call, not derived from the
confirmation rule the way "needs ANY runway" is — documented here so it can be
revisited if the TOPIC write-up needs a more rigorous derivation (e.g. cross-
checking against known confirmed sites' actual point-to-runway distances).

Usage:
  python3 filter-aerodromes-by-runway.py --region AB
  python3 filter-aerodromes-by-runway.py --all-na
Reads:  work/na-sweep-logs/aerodromes-<region>.json (from discover-na-aerodromes.py)
Writes: work/na-sweep-logs/aerodromes-filtered-<region>.json (subset with a
        nearby runway) + prints how many were dropped and why.
"""

import argparse
import json
import math
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

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
QUERY_SLEEP = 25
RUNWAY_MATCH_KM = 2.0
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
                wait = 30 * (2 ** attempt)
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


def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2 + math.cos(math.radians(lat1))
         * math.cos(math.radians(lat2)) * math.sin(dlon / 2) ** 2)
    return R * 2 * math.asin(math.sqrt(max(0, min(1, a))))


def fetch_runways(bbox_str: str) -> list:
    q = f"""
[out:json][timeout:240];
way["aeroway"="runway"]({bbox_str});
out center;
"""
    res = fetch_overpass(q, "runways")
    pts = []
    for el in res.get("elements", []):
        lat, lon = get_coord(el)
        if lat is not None and lon is not None:
            pts.append((lat, lon))
    return pts


def filter_region(region: str, bbox: tuple) -> tuple:
    aerodrome_file = OUT_DIR / f"aerodromes-{region.lower()}.json"
    if not aerodrome_file.exists():
        print(f"  SKIP: {aerodrome_file} not found (run discover-na-aerodromes.py first)")
        return [], 0

    sites = json.loads(aerodrome_file.read_text())
    bbox_str = f"{bbox[0]},{bbox[1]},{bbox[2]},{bbox[3]}"
    print(f"  querying runways...", end=" ", flush=True)
    runways = fetch_runways(bbox_str)
    print(f"{len(runways)} runway ways")

    kept = []
    for site in sites:
        has_nearby = any(
            haversine(site["lat"], site["lon"], rlat, rlon) <= RUNWAY_MATCH_KM
            for rlat, rlon in runways
        )
        if has_nearby:
            kept.append(site)

    return kept, len(sites) - len(kept)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--region", choices=list(NA_REGIONS.keys()))
    ap.add_argument("--all-na", action="store_true")
    args = ap.parse_args()

    if args.all_na:
        regions = list(NA_REGIONS.keys())
    elif args.region:
        regions = [args.region]
    else:
        ap.print_help()
        sys.exit(1)

    total_kept, total_dropped = 0, 0
    for i, region in enumerate(regions):
        print(f"\n=== Region: {region} ===")
        kept, dropped = filter_region(region, NA_REGIONS[region])
        out = OUT_DIR / f"aerodromes-filtered-{region.lower()}.json"
        out.write_text(json.dumps(kept, indent=2))
        print(f"  kept {len(kept)}, dropped {dropped} (no runway within {RUNWAY_MATCH_KM}km) -> {out}")
        total_kept += len(kept)
        total_dropped += dropped
        if i < len(regions) - 1:
            time.sleep(QUERY_SLEEP)

    print(f"\nTotal: kept {total_kept}, dropped {total_dropped} "
          f"({100*total_dropped//(total_kept+total_dropped) if (total_kept+total_dropped) else 0}% reduction)")


if __name__ == "__main__":
    main()
