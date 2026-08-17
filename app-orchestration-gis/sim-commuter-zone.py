#!/usr/bin/env python3
# SPDX-License-Identifier: LicenseRef-PointSav-ARR
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
#
# This file is proprietary material of Woodfine Capital Projects Inc.
# See the LICENSE file in this repository for the full terms.
# Unauthorized use, reproduction, or distribution is prohibited.

"""
sim-commuter-zone.py — Commuter Zone (Transit Terminus) detection simulation

Tests whether "terminus" (last stop on a line) is actually a necessary
condition for the Transit Terminus co-location pattern, or whether any
station with a large park-and-ride shows the same signature. Also profiles
existing parkades (multi-storey car parks) against nearby transit + POI to
reverse-engineer candidate sites for new parkade construction.

Usage:
  python3 sim-commuter-zone.py [--region AB|BC]
  Output: work/sim-commuter-<region>.json + printed table
"""

import json, math, sys, time, urllib.request, urllib.parse, argparse
from pathlib import Path

OVERPASS = "https://overpass-api.de/api/interpreter"
QUERY_SLEEP = 4
POI_RADIUS_KM = 0.8   # walk-shed radius around each station

# Known line termini per region, from route-relation queries (ground truth, not
# guesswork) — see BRIEF-gis-commuter-zone.md for the queries used. Region keys are
# historically AB/BC for Calgary/Vancouver (kept for continuity with committed work);
# new cities use full names since Ottawa/Edmonton would otherwise collide with ON/AB.
TERMINI_BY_REGION = {
    # Calgary CTrain (2026-06-30)
    "AB": {"69 Street", "Saddletowne", "Somerset-Bridlewood",
           "Somerset–Bridlewood", "Tuscany"},
    # Vancouver SkyTrain (2026-06-30) — Waterfront and VCC-Clark are inner-city/
    # downtown termini (unlike any Calgary terminus); flagged separately in analysis.
    # OSM station names use en-dash (–, U+2013), not hyphen — both variants included.
    "BC": {"Waterfront", "YVR-Airport", "YVR–Airport",
           "Richmond-Brighouse", "Richmond–Brighouse", "King George",
           "Production Way–University", "Production Way-University",
           "VCC-Clark", "VCC–Clark",
           "Lafarge Lake–Douglas", "Lafarge Lake-Douglas"},
    # Toronto TTC (2026-06-30) — Line 1/2/4/5/6. Kennedy is shared Line2/Line5
    # terminus + GO interchange (major hub, flagged inner-city). Sheppard-Yonge is a
    # Line 4 terminus but a Line 1 mid-line interchange — treated as terminus.
    "TORONTO": {"Vaughan Metropolitan Centre", "Finch", "Kipling", "Kennedy",
                "Don Mills", "Sheppard-Yonge", "Sheppard–Yonge",
                "Mount Dennis", "Humber College", "Finch West"},
    # Ottawa O-Train (2026-06-30) — Lines 1/2/4. Bayview is Line 2 terminus + Line 1
    # interchange — treated as terminus (same pattern as Sheppard-Yonge).
    "OTTAWA": {"Tunney's Pasture", "Blair", "Bayview", "Limebank",
               "South Keys", "Airport", "Airport / Aéroport"},
    # Edmonton LRT (2026-06-30) — Capital/Metro/Valley Lines.
    "EDMONTON": {"Clareview", "Century Park", "Health Sciences/Jubilee",
                 "NAIT/Blatchford Market", "Mill Woods", "102 Street"},
    # Montreal Metro + REM (2026-06-30) — Green/Orange/Yellow/Blue + REM branches.
    # Berri-UQAM is the 3-line downtown interchange (Yellow terminus) — inner-city.
    "MONTREAL": {"Honoré-Beaugrand", "Angrignon", "Montmorency", "Côte-Vertu",
                 "Berri-UQAM", "Longueuil–Université-de-Sherbrooke",
                 "Saint-Michel", "Snowdon", "Deux-Montagnes", "Brossard",
                 "Anse-à-l'Orme"},
}

# Inner-city/downtown termini — excluded from the "suburban terminus" baseline
# comparison the same way Calgary's downtown corridor was excluded, but reported
# separately since a downtown terminus is itself an interesting test case.
INNER_CITY_TERMINI = {"Waterfront", "VCC-Clark", "VCC–Clark",
                       "Kennedy", "Berri-UQAM"}

REGIONS = {
    # Calgary metro only — terminus dictionary is Calgary-specific; a wider AB bbox
    # would pull in unlabeled Edmonton LRT stations and corrupt the comparison.
    "AB": (50.75, -114.35, 51.20, -113.85),
    # Metro Vancouver only (SkyTrain network extent) — same scoping logic.
    "BC": (49.00, -123.25, 49.35, -122.60),
    "TORONTO":  (43.55, -79.65, 43.85, -79.10),
    "OTTAWA":   (45.20, -75.90, 45.50, -75.45),
    "EDMONTON": (53.35, -113.75, 53.72, -113.25),
    "MONTREAL": (45.30, -73.95, 45.75, -73.40),
}

# Downtown transit-mall corridor per region — excluded from suburban-vs-terminus
# comparison (confounds it with CBD parking/banking density, same issue found in AB).
DOWNTOWN_CORRIDOR_BY_REGION = {
    "AB": {"Downtown West–Kerby", "6 Street Southwest", "City Hall/Bow Valley College",
           "Centre Street", "1 Street Southwest", "3 Street Southwest",
           "4 Street Southwest", "7 Street Southwest", "8 Street Southwest",
           "Erlton/Stampede", "Victoria Park/Stampede"},
    "BC": {"Waterfront", "Burrard", "Granville", "Stadium–Chinatown",
           "Main Street–Science World", "VCC-Clark", "VCC–Clark"},
    # Toronto downtown core — Yonge-University financial/entertainment district loop.
    "TORONTO": {"Union", "King", "Queen", "Dundas", "College", "Wellesley",
                "Bloor-Yonge", "Bloor–Yonge", "St George", "St. George",
                "Osgoode", "St Andrew", "St. Andrew", "Museum", "Queen's Park",
                "St Patrick", "St. Patrick", "Kennedy"},
    # Ottawa downtown core — Confederation Line underground/central segment.
    "OTTAWA": {"Parliament", "Rideau", "Lyon", "Bayview"},
    # Edmonton downtown core — Churchill/Central/Bay-Enterprise Square corridor.
    "EDMONTON": {"Churchill", "Central", "Bay/Enterprise Square", "Corona",
                 "Grandin/Government Centre", "MacEwan", "Stadium"},
    # Montreal downtown core — Green/Orange central segment.
    "MONTREAL": {"Berri-UQAM", "McGill", "Peel", "Guy-Concordia",
                 "Lucien-L'Allier", "Bonaventure", "Square-Victoria",
                 "Place-des-Arts", "Saint-Laurent"},
}


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
        "User-Agent": "foundry-gis-sim/1.0",
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
    # Covers light_rail (Calgary CTrain) and subway (Vancouver SkyTrain — grade-
    # separated automated guideway, tagged station=subway/subway=yes, not light_rail).
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
    # bbox approx for the radius (generous; caller distance-filters)
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
            pts.append({"cat": cat, "lat": elat, "lon": elon, "dist_km": round(d, 2),
                        "name": tags.get("name", "")})
    return pts


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--region", default="AB", choices=list(REGIONS.keys()))
    args = ap.parse_args()
    bbox = REGIONS[args.region]
    bbox_str = f"{bbox[0]},{bbox[1]},{bbox[2]},{bbox[3]}"

    print(f"\nCommuter Zone simulation — {args.region}")
    print("Fetching LRT stations ...", flush=True)
    stations = fetch_stations(bbox_str)
    print(f"  {len(stations)} stations found\n")

    termini_set = TERMINI_BY_REGION[args.region]
    downtown_set = DOWNTOWN_CORRIDOR_BY_REGION.get(args.region, set())
    for st in stations:
        st["is_terminus"] = st["name"] in termini_set
        st["is_inner_city_terminus"] = st["name"] in INNER_CITY_TERMINI
        st["is_downtown"] = st["name"] in downtown_set

    results = []
    for i, st in enumerate(stations, 1):
        print(f"  [{i:2d}/{len(stations)}] {st['name']:30s} "
              f"({'TERMINUS' if st['is_terminus'] else 'mid-line'}) ...", end=" ", flush=True)
        poi = fetch_poi_near(st["lat"], st["lon"], POI_RADIUS_KM)
        cats = {}
        for p in poi:
            cats.setdefault(p["cat"], []).append(p)
        st["poi"] = poi
        st["cat_counts"] = {k: len(v) for k, v in cats.items()}
        st["n_cats"] = len(cats)
        nearest_parkade = min((p for p in poi if p["cat"] == "parkade"),
                               key=lambda p: p["dist_km"], default=None)
        st["nearest_parkade_km"] = nearest_parkade["dist_km"] if nearest_parkade else None
        results.append(st)
        print(f"cats={st['n_cats']} " + " ".join(f"{k}={v}" for k, v in st["cat_counts"].items()))
        time.sleep(QUERY_SLEEP)

    # ── Terminus vs mid-line comparison (suburban-only — excludes downtown corridor
    #    and inner-city termini, which are confounded by CBD parking/banking density) ──
    suburban = [s for s in results if not s["is_downtown"]]
    termini  = [s for s in suburban if s["is_terminus"] and not s["is_inner_city_terminus"]]
    midline  = [s for s in suburban if not s["is_terminus"]]
    inner_termini = [s for s in results if s["is_inner_city_terminus"]]

    print(f"\n{'═'*80}")
    print(f"SUBURBAN TERMINUS vs SUBURBAN MID-LINE — {len(termini)} termini, "
          f"{len(midline)} mid-line stations (downtown corridor + inner-city "
          f"termini excluded)")
    print(f"{'═'*80}")

    all_cats = ["park_and_ride", "parkade", "drive_through_food",
                "fuel_convenience", "bank_branch", "residential_anchor"]
    print(f"{'Category':<22} {'Terminus avg':>14} {'Mid-line avg':>14} {'Ratio T/M':>16}")
    for cat in all_cats:
        t_vals = [s["cat_counts"].get(cat, 0) for s in termini]
        m_vals = [s["cat_counts"].get(cat, 0) for s in midline]
        t_avg = sum(t_vals) / len(t_vals) if t_vals else 0
        m_avg = sum(m_vals) / len(m_vals) if m_vals else 0
        ratio = (t_avg / m_avg) if m_avg > 0 else (float("inf") if t_avg > 0 else 0)
        print(f"{cat:<22} {t_avg:>14.2f} {m_avg:>14.2f} {ratio:>16.2f}")

    if inner_termini:
        print(f"\nInner-city/downtown termini (excluded above, reported separately — "
              f"{len(inner_termini)}):")
        for s in inner_termini:
            print(f"    {s['name']:30s} cats={s['n_cats']}  " +
                  " ".join(f"{k}={v}" for k, v in s["cat_counts"].items()))

    # ── Mid-line stations that match the terminus pattern anyway ───────────
    def matches_pattern(s):
        c = s["cat_counts"]
        return (c.get("park_and_ride", 0) >= 1
                and sum(1 for k in ["drive_through_food", "fuel_convenience", "bank_branch"]
                        if c.get(k, 0) >= 1) >= 2)

    midline_matches = [s for s in midline if matches_pattern(s)]
    print(f"\n{'─'*80}")
    print(f"Mid-line stations matching the Transit Terminus co-location pattern anyway:")
    print(f"{'─'*80}")
    print(f"  {len(midline_matches)} / {len(midline)} mid-line stations qualify "
          f"({100*len(midline_matches)//max(1,len(midline))}%)")
    for s in sorted(midline_matches, key=lambda s: -s["n_cats"]):
        print(f"    {s['name']:30s} cats={s['n_cats']}  "
              f"parkade={s['nearest_parkade_km']}km  " +
              " ".join(f"{k}={v}" for k, v in s["cat_counts"].items()))

    # ── Parkade reverse-engineering ──────────────────────────────────────────
    print(f"\n{'─'*80}")
    print("PARKADE SITE CANDIDATES — stations with strong co-location pattern, no parkade nearby")
    print(f"{'─'*80}")
    candidates = sorted(
        [s for s in results if s["nearest_parkade_km"] is None and s["n_cats"] >= 3],
        key=lambda s: -s["n_cats"]
    )
    for s in candidates:
        print(f"    {s['name']:30s} ({'TERMINUS' if s['is_terminus'] else 'mid-line'})  "
              f"cats={s['n_cats']}  " + " ".join(f"{k}={v}" for k, v in s["cat_counts"].items()))
    if not candidates:
        print("    (none — every station with 3+ POI categories already has a nearby parkade)")

    # ── Existing parkade profile ─────────────────────────────────────────────
    print(f"\n{'─'*80}")
    print("STATIONS WITH AN EXISTING PARKADE — profile of what's around them")
    print(f"{'─'*80}")
    with_parkade = sorted([s for s in results if s["nearest_parkade_km"] is not None],
                           key=lambda s: s["nearest_parkade_km"])
    for s in with_parkade:
        print(f"    {s['name']:30s} ({'TERMINUS' if s['is_terminus'] else 'mid-line'})  "
              f"parkade={s['nearest_parkade_km']}km  cats={s['n_cats']}  " +
              " ".join(f"{k}={v}" for k, v in s["cat_counts"].items()))

    # ── Save JSON ──────────────────────────────────────────────────────────
    out = Path(f"work/sim-commuter-{args.region.lower()}.json")
    out.parent.mkdir(exist_ok=True)
    out.write_text(json.dumps(results, indent=2))
    print(f"\nSaved → {out}\n")


if __name__ == "__main__":
    main()
