#!/usr/bin/env python3
"""
build-transit-terminus.py — standalone Transit Terminus / parkade-gap candidate
output, productionizing sim-commuter-zone.py's validated 3-step Commuter Zone
pipeline (see BRIEF-gis-commuter-zone.md).

Output model (2026-07-01, corrected after review): writes a STANDALONE GeoJSON
(work/archetype-transit-terminus.geojson), one feature per qualifying terminus.
Does NOT touch clusters-meta.json or build-clusters.py's tier_of(). This
follows the same pattern already proven in production by build-pks-clusters.py
("Commuter"/PKS archetype) and the VWH ("Urban Fringe") archetype — both are
independent clustering/detection passes with their own output file and own
type/tier field, never merged into the retail co-location dataset.

Why not patch clusters-meta.json (the original approach, corrected here): a
lone terminus with no nearby retail — exactly the "underserved, growth-market"
case this whole exercise exists to surface (e.g. Anse-à-l'Orme in Montreal,
flagged in the BRIEF as "the clearest single candidate") has no cluster to
attach to. Verified empirically: 0/4 known Calgary termini matched an existing
cluster within any reasonable snap radius. Also, `tier_of()` requires 2+ retail
categories to produce any output at all (civic locations hit the identical
blind spot — confirmed by reading build-clusters.py's enrich_with_civic(),
which only ever adds civic points to a cluster that already qualified via
retail, never seeds one standalone). The BRIEFs had already locked in "separate
classification track, not a T4 tier" as the intended design.

Pipeline (per BRIEF, verified across 6 Canadian cities 2026-06-30):
  1. Route-relation-derived terminus topology (ground truth from
     TERMINI_BY_REGION — see note below on live route-relation detection).
  2. POI density vs. suburban mid-line baseline, per category (park_and_ride,
     parkade, drive_through_food, fuel_convenience, bank_branch,
     residential_anchor), 800m walk-shed.
  3. No existing parkade nearby -> confirmed parkade_gap_candidate.

Scope note: this script covers the 6 already-validated regions
(AB/BC/TORONTO/OTTAWA/EDMONTON/MONTREAL) using the same hand-verified
TERMINI_BY_REGION ground truth sim-commuter-zone.py uses — reused verbatim, not
reimplemented, since it's the proven-correct part. The BRIEF calls for live
route-relation queries (relation[route=light_rail/subway], first/last member)
so this generalizes to new cities without hand-curated ground truth — that's
deliberately NOT built here yet (untested code path, no way to verify
correctness against a new city in this session) and is tracked as a follow-up
in NEXT.md under Commuter Zone international expansion.

Open follow-up (not resolved here, needs operator input): whether this should
eventually merge into build-pks-clusters.py (the existing "Commuter" archetype,
which already DBSCANs metro_subway + other transit POIs but without topology/
terminus-awareness or parkade-gap detection) rather than staying a fully
separate archetype, and how to avoid a naming collision between this feature
and the existing shipped "Urban Fringe" (VWH) archetype if "Intercity Fringe"
also ships later. Also not done here: frontend integration (a new
activeArchetype-style mode in www/index.html, following the VWH/PKS pattern).

Usage:
    python3 build-transit-terminus.py --dry-run              # one region, print only
    python3 build-transit-terminus.py                        # all 6 validated regions
    python3 build-transit-terminus.py --region AB BC          # specific regions
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

OUTPUT_PATH = Path("work/archetype-transit-terminus.geojson")

OVERPASS = "https://overpass-api.de/api/interpreter"
QUERY_SLEEP = 4
POI_RADIUS_KM = 0.8       # walk-shed radius around each station (matches sim script)

# ── Ground truth, reused verbatim from sim-commuter-zone.py (validated 2026-06-30) ──

TERMINI_BY_REGION = {
    "AB": {"69 Street", "Saddletowne", "Somerset-Bridlewood",
           "Somerset–Bridlewood", "Tuscany"},
    "BC": {"Waterfront", "YVR-Airport", "YVR–Airport",
           "Richmond-Brighouse", "Richmond–Brighouse", "King George",
           "Production Way–University", "Production Way-University",
           "VCC-Clark", "VCC–Clark",
           "Lafarge Lake–Douglas", "Lafarge Lake-Douglas"},
    "TORONTO": {"Vaughan Metropolitan Centre", "Finch", "Kipling", "Kennedy",
                "Don Mills", "Sheppard-Yonge", "Sheppard–Yonge",
                "Mount Dennis", "Humber College", "Finch West"},
    "OTTAWA": {"Tunney's Pasture", "Blair", "Bayview", "Limebank",
               "South Keys", "Airport", "Airport / Aéroport"},
    "EDMONTON": {"Clareview", "Century Park", "Health Sciences/Jubilee",
                 "NAIT/Blatchford Market", "Mill Woods", "102 Street"},
    "MONTREAL": {"Honoré-Beaugrand", "Angrignon", "Montmorency", "Côte-Vertu",
                 "Berri-UQAM", "Longueuil–Université-de-Sherbrooke",
                 "Saint-Michel", "Snowdon", "Deux-Montagnes", "Brossard",
                 "Anse-à-l'Orme"},
}

INNER_CITY_TERMINI = {"Waterfront", "VCC-Clark", "VCC–Clark",
                       "Kennedy", "Berri-UQAM"}

REGIONS = {
    "AB": (50.75, -114.35, 51.20, -113.85),
    "BC": (49.00, -123.25, 49.35, -122.60),
    "TORONTO":  (43.55, -79.65, 43.85, -79.10),
    "OTTAWA":   (45.20, -75.90, 45.50, -75.45),
    "EDMONTON": (53.35, -113.75, 53.72, -113.25),
    "MONTREAL": (45.30, -73.95, 45.75, -73.40),
}

DOWNTOWN_CORRIDOR_BY_REGION = {
    "AB": {"Downtown West–Kerby", "6 Street Southwest", "City Hall/Bow Valley College",
           "Centre Street", "1 Street Southwest", "3 Street Southwest",
           "4 Street Southwest", "7 Street Southwest", "8 Street Southwest",
           "Erlton/Stampede", "Victoria Park/Stampede"},
    "BC": {"Waterfront", "Burrard", "Granville", "Stadium–Chinatown",
           "Main Street–Science World", "VCC-Clark", "VCC–Clark"},
    "TORONTO": {"Union", "King", "Queen", "Dundas", "College", "Wellesley",
                "Bloor-Yonge", "Bloor–Yonge", "St George", "St. George",
                "Osgoode", "St Andrew", "St. Andrew", "Museum", "Queen's Park",
                "St Patrick", "St. Patrick", "Kennedy"},
    "OTTAWA": {"Parliament", "Rideau", "Lyon", "Bayview"},
    "EDMONTON": {"Churchill", "Central", "Bay/Enterprise Square", "Corona",
                 "Grandin/Government Centre", "MacEwan", "Stadium"},
    "MONTREAL": {"Berri-UQAM", "McGill", "Peel", "Guy-Concordia",
                 "Lucien-L'Allier", "Bonaventure", "Square-Victoria",
                 "Place-des-Arts", "Saint-Laurent"},
}

ALL_CATS = ["park_and_ride", "parkade", "drive_through_food",
            "fuel_convenience", "bank_branch", "residential_anchor"]


def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2
         + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2))
         * math.sin(dlon / 2) ** 2)
    return R * 2 * math.asin(math.sqrt(max(0, min(1, a))))


def fetch_overpass(q, label, retries=3):
    data = urllib.parse.urlencode({"data": q}).encode()
    req = urllib.request.Request(OVERPASS, data=data, method="POST", headers={
        "Content-Type": "application/x-www-form-urlencoded",
        "User-Agent": "foundry-gis-build/1.0",
    })
    for attempt in range(retries):
        try:
            with urllib.request.urlopen(req, timeout=200) as r:
                return json.loads(r.read())
        except urllib.error.HTTPError as e:
            if e.code in (429, 504) and attempt < retries - 1:
                wait = 15 * (attempt + 1)
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


def fetch_stations(bbox_str):
    q = f"""
[out:json][timeout:120];
(
  node["railway"="station"]["station"="light_rail"]({bbox_str});
  node["public_transport"="station"]["light_rail"="yes"]({bbox_str});
  node["railway"="station"]["station"="subway"]({bbox_str});
  node["public_transport"="station"]["subway"="yes"]({bbox_str});
);
out;
"""
    res = fetch_overpass(q, "stations")
    out, seen = [], set()
    for el in res.get("elements", []):
        lat, lon = get_coord(el)
        name = el.get("tags", {}).get("name", "?")
        if lat and lon and name not in seen:
            seen.add(name)
            out.append({"name": name, "lat": lat, "lon": lon})
    return out


def fetch_poi_near(lat, lon, radius_km):
    dlat = radius_km / 111.0
    dlon = radius_km / (111.0 * math.cos(math.radians(lat)))
    s, n = lat - dlat, lat + dlat
    w, e = lon - dlon, lon + dlon
    bbox = f"{s},{w},{n},{e}"
    q = f"""
[out:json][timeout:90];
(
  node["amenity"="parking"]({bbox});
  way["amenity"="parking"]({bbox});
  way["parking"="multi-storey"]({bbox});
  node["parking"="multi-storey"]({bbox});
  way["building"="parking"]({bbox});
  node["amenity"="fast_food"]["drive_through"="yes"]({bbox});
  node["amenity"="cafe"]["drive_through"="yes"]({bbox});
  node["amenity"="fuel"]({bbox});
  node["shop"="convenience"]({bbox});
  node["amenity"="bank"]({bbox});
  node["amenity"="atm"]({bbox});
  node["shop"="supermarket"]({bbox});
  node["amenity"="pharmacy"]({bbox});
);
out center tags;
"""
    res = fetch_overpass(q, f"poi@{lat:.3f},{lon:.3f}")
    pts = []
    for el in res.get("elements", []):
        elat, elon = get_coord(el)
        if elat is None:
            continue
        d = haversine(lat, lon, elat, elon)
        if d > radius_km:
            continue
        tags = el.get("tags", {})
        cat = None
        if tags.get("amenity") == "parking" or tags.get("parking") == "multi-storey" or tags.get("building") == "parking":
            cat = "parkade" if (tags.get("parking") == "multi-storey" or tags.get("building") == "parking") else "park_and_ride"
        elif tags.get("amenity") in ("fast_food", "cafe") and tags.get("drive_through") == "yes":
            cat = "drive_through_food"
        elif tags.get("amenity") == "fuel" or tags.get("shop") == "convenience":
            cat = "fuel_convenience"
        elif tags.get("amenity") in ("bank", "atm"):
            cat = "bank_branch"
        elif tags.get("shop") == "supermarket" or tags.get("amenity") == "pharmacy":
            cat = "residential_anchor"
        if cat:
            pts.append({"cat": cat, "lat": elat, "lon": elon, "dist_km": round(d, 2)})
    return pts


def classify_terminus_type(station_ratios):
    """QUIET/MIXED/DESTINATION per BRIEF's city-profile finding: quiet networks'
    termini run <1.0 on most categories, destination networks >1.0, mixed in between.
    Applied per-station here (mean ratio across categories with a mid-line signal)
    rather than per-region, so individual stations can differ from their city's
    overall tendency."""
    vals = [r for r in station_ratios if r is not None and r != float("inf")]
    if not vals:
        return "unknown"
    mean_ratio = sum(vals) / len(vals)
    if mean_ratio < 0.85:
        return "quiet"
    if mean_ratio > 1.15:
        return "destination"
    return "mixed"


def matches_colocation_pattern(cat_counts):
    """Same heuristic sim-commuter-zone.py validated: park_and_ride present plus
    at least 2 of the 3 secondary categories — used here as the parkade-gap test's
    "strong co-location signal" precondition, combined with no nearby parkade."""
    return (cat_counts.get("park_and_ride", 0) >= 1
            and sum(1 for k in ["drive_through_food", "fuel_convenience", "bank_branch"]
                    if cat_counts.get(k, 0) >= 1) >= 2)


def process_region(region):
    bbox = REGIONS[region]
    bbox_str = f"{bbox[0]},{bbox[1]},{bbox[2]},{bbox[3]}"
    print(f"\n[{region}] Fetching stations ...", flush=True)
    stations = fetch_stations(bbox_str)
    print(f"  {len(stations)} stations found")

    termini_set = TERMINI_BY_REGION[region]
    downtown_set = DOWNTOWN_CORRIDOR_BY_REGION.get(region, set())
    for st in stations:
        st["is_terminus"] = st["name"] in termini_set
        st["is_inner_city_terminus"] = st["name"] in INNER_CITY_TERMINI
        st["is_downtown"] = st["name"] in downtown_set

    for i, st in enumerate(stations, 1):
        print(f"  [{i:2d}/{len(stations)}] {st['name']:30s} ...", end=" ", flush=True)
        poi = fetch_poi_near(st["lat"], st["lon"], POI_RADIUS_KM)
        cats = {}
        for p in poi:
            cats.setdefault(p["cat"], []).append(p)
        st["cat_counts"] = {k: len(v) for k, v in cats.items()}
        nearest_parkade = min((p for p in poi if p["cat"] == "parkade"),
                               key=lambda p: p["dist_km"], default=None)
        st["nearest_parkade_km"] = nearest_parkade["dist_km"] if nearest_parkade else None
        print(f"cats={len(cats)}")
        time.sleep(QUERY_SLEEP)

    # Suburban mid-line baseline (excludes downtown corridor + inner-city termini,
    # same exclusion sim-commuter-zone.py validated to avoid CBD confounding)
    suburban = [s for s in stations if not s["is_downtown"]]
    midline = [s for s in suburban if not s["is_terminus"]]
    baseline = {}
    for cat in ALL_CATS:
        vals = [s["cat_counts"].get(cat, 0) for s in midline]
        baseline[cat] = sum(vals) / len(vals) if vals else 0

    termini = [s for s in suburban if s["is_terminus"] and not s["is_inner_city_terminus"]]
    features = []
    for st in termini:
        ratios = [
            (st["cat_counts"].get(cat, 0) / baseline[cat]) if baseline[cat] > 0
            else (float("inf") if st["cat_counts"].get(cat, 0) > 0 else None)
            for cat in ALL_CATS
        ]
        terminus_type = classify_terminus_type(ratios)
        gap_candidate = (
            st["nearest_parkade_km"] is None
            and matches_colocation_pattern(st["cat_counts"])
        )
        features.append({
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [st["lon"], st["lat"]]},
            "properties": {
                "name": st["name"],
                "region": region,
                "terminus_type": terminus_type,
                "parkade_gap_candidate": gap_candidate,
                "nearest_parkade_km": st["nearest_parkade_km"],
                "cat_counts": st["cat_counts"],
            },
        })
        print(f"  {st['name']!r} type={terminus_type} gap={gap_candidate} "
              f"nearest_parkade_km={st['nearest_parkade_km']}")

    return features


def main():
    ap = argparse.ArgumentParser(
        description="Build a standalone Transit Terminus / parkade-gap candidate GeoJSON "
                     "(does not touch clusters-meta.json)"
    )
    ap.add_argument("--out", default=str(OUTPUT_PATH), metavar="PATH")
    ap.add_argument("--region", nargs="+", default=None, choices=list(REGIONS.keys()),
                     metavar="REGION", help="Limit to specific regions (default: all 6)")
    ap.add_argument("--dry-run", action="store_true",
                     help="Run one region, print results, do not write the output file")
    args = ap.parse_args()

    regions = args.region or list(REGIONS.keys())
    if args.dry_run:
        regions = regions[:1]
        print(f"DRY RUN — {regions[0]} only, no writes")

    all_features = []
    for region in regions:
        all_features.extend(process_region(region))

    n_gap = sum(1 for f in all_features if f["properties"]["parkade_gap_candidate"])
    print(f"\nTotal: {len(all_features)} termini processed, {n_gap} parkade-gap candidates")

    if args.dry_run:
        print("Dry run — output file NOT written")
        return

    out_path = Path(args.out)
    out_path.parent.mkdir(exist_ok=True)
    out_path.write_text(json.dumps(
        {"type": "FeatureCollection", "features": all_features}, indent=2
    ))
    print(f"Wrote {out_path}")


if __name__ == "__main__":
    main()
