#!/usr/bin/env python3
"""
build-pks-clusters.py — Independent Commuter (PKS) co-location clustering.

Builds PKS clusters from transit node data directly.
Does NOT reference clusters.geojson (retail co-location data).

Transit categories (first-class co-location signals):
  airport         — commercial passenger airports (IATA-filtered)
  intercity_rail  — mainline / intercity rail stations
  commuter_rail   — suburban S-Bahn, RER, NJ Transit, etc.
  metro_subway    — metro/subway end-of-line and outer stations

Commercial enrichment:
  car_rental      — confirms traveller-facing transit zone

T1/T2/T3 by category composition:
  T1: airport + any rail within 3 km (multi-modal hub)
      OR airport + car_rental within 2 km
      OR 3+ transit categories
  T2: airport alone (valid metro range, no co-located rail)
      OR any rail + car_rental within 2 km
      OR 2 rail categories without car_rental
  T3: intercity_rail alone, 15–80 km from metro
      OR commuter/metro rail alone, 5–35 km from metro

Excluded: transit nodes < 3 km from metro centre (downtown core, not suburban)
          commuter nodes outside 5–80 km range; intercity outside 15–150 km

Usage:
    python3 build-pks-clusters.py
    python3 build-pks-clusters.py --output work/archetype-pks.geojson
"""

import json
import math
import argparse
from pathlib import Path
from collections import defaultdict, Counter

SCRIPT_DIR = Path(__file__).parent
TOTEBOX    = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")
CHAIN_DIR  = TOTEBOX / "service-fs" / "service-business"
PLACES_DIR = TOTEBOX / "service-places"
WORK       = SCRIPT_DIR / "work"

DISPLAY_ISO = {
    "US", "CA", "MX",
    "GB", "SE", "DK", "NO", "FI", "IS",
    "FR", "DE", "ES", "IT", "GR", "PL", "AT", "NL", "PT",
}

# Metro distance ranges by transit class
RANGE_AIRPORT      = (15.0, 150.0)
RANGE_INTERCITY    = (15.0, 150.0)
RANGE_COMMUTER     = ( 5.0,  80.0)
RANGE_METRO        = ( 3.0,  35.0)

TRANSIT_GROUP_KM   = 3.0   # group adjacent transit nodes (multi-modal detection)
CAR_RENTAL_KM      = 2.0   # car rental within this → enriched

# T3 quality filter: limit intercity_rail T3 to 15–80 km to avoid rural noise
T3_INTERCITY_MAX_KM = 80.0

CAR_RENTAL_CHAINS = [
    "enterprise-us", "hertz-us", "avis-us", "europcar-fr", "sixt-de",
    "enterprise-ca", "hertz-mx",   # Canada + Mexico
]

# ── METRO REFERENCE POINTS ────────────────────────────────────────────────────
METRO_REFS = [
    ("New York",40.7128,-74.0060),("Los Angeles",34.0522,-118.2437),
    ("Chicago",41.8781,-87.6298),("Dallas",32.7767,-96.7970),
    ("Houston",29.7604,-95.3698),("Phoenix",33.4484,-112.0740),
    ("Philadelphia",39.9526,-75.1652),("Seattle",47.6062,-122.3321),
    ("Denver",39.7392,-104.9903),("Atlanta",33.7490,-84.3880),
    ("Miami",25.7617,-80.1918),("Minneapolis",44.9778,-93.2650),
    ("Portland",45.5051,-122.6750),("Las Vegas",36.1699,-115.1398),
    ("Sacramento",38.5816,-121.4944),("Kansas City",39.0997,-94.5786),
    ("Nashville",36.1627,-86.7816),("Tampa",27.9506,-82.4572),
    ("Columbus",39.9612,-82.9988),("Indianapolis",39.7684,-86.1581),
    ("San Antonio",29.4241,-98.4936),("Austin",30.2672,-97.7431),
    ("Baltimore",39.2904,-76.6122),("Milwaukee",43.0389,-87.9065),
    ("Raleigh",35.7796,-78.6382),("Omaha",41.2565,-95.9345),
    ("Vancouver",49.2827,-123.1207),("Toronto",43.6532,-79.3832),
    ("Montreal",45.5017,-73.5673),("Calgary",51.0447,-114.0719),
    ("Ottawa",45.4215,-75.6972),("Edmonton",53.5461,-113.4938),
    ("Mexico City",19.4326,-99.1332),("Guadalajara",20.6597,-103.3496),
    ("Monterrey",25.6866,-100.3161),
    ("London",51.5074,-0.1278),("Birmingham",52.4862,-1.8904),
    ("Manchester",53.4808,-2.2426),("Glasgow",55.8642,-4.2518),
    ("Edinburgh",55.9533,-3.1883),("Leeds",53.8008,-1.5491),
    ("Cardiff",51.4816,-3.1791),
    ("Stockholm",59.3293,18.0686),("Gothenburg",57.7089,11.9746),
    ("Copenhagen",55.6761,12.5683),("Aarhus",56.1629,10.2039),
    ("Oslo",59.9139,10.7522),("Helsinki",60.1699,24.9384),
    ("Tampere",61.4978,23.7610),("Reykjavik",64.1355,-21.8954),
    ("Paris",48.8566,2.3522),("Lyon",45.7640,4.8357),
    ("Marseille",43.2965,5.3698),("Toulouse",43.6047,1.4442),
    ("Bordeaux",44.8378,-0.5792),("Lille",50.6292,3.0573),
    ("Berlin",52.5200,13.4050),("Hamburg",53.5753,9.9950),
    ("Munich",48.1351,11.5820),("Cologne",50.9333,6.9500),
    ("Frankfurt",50.1109,8.6821),("Stuttgart",48.7758,9.1829),
    ("Düsseldorf",51.2217,6.7762),("Dortmund",51.5136,7.4653),
    ("Leipzig",51.3397,12.3731),("Dresden",51.0504,13.7373),
    ("Hannover",52.3759,9.7320),("Nuremberg",49.4521,11.0767),
    ("Madrid",40.4168,-3.7038),("Barcelona",41.3851,2.1734),
    ("Valencia",39.4699,-0.3763),("Seville",37.3891,-5.9845),
    ("Rome",41.9028,12.4964),("Milan",45.4654,9.1866),
    ("Naples",40.8518,14.2681),("Turin",45.0703,7.6869),
    ("Bologna",44.4949,11.3426),("Athens",37.9838,23.7275),
    ("Warsaw",52.2297,21.0122),("Kraków",50.0647,19.9450),
    ("Wrocław",51.1079,17.0385),("Gdańsk",54.3520,18.6466),
    ("Katowice",50.2649,19.0238),("Vienna",48.2082,16.3738),
    ("Amsterdam",52.3676,4.9041),("Rotterdam",51.9244,4.4777),
    ("Lisbon",38.7169,-9.1399),("Porto",41.1579,-8.6291),
]


def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2
         + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2))
         * math.sin(dlon / 2) ** 2)
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def nearest_metro(lat, lon):
    best_d, best_name = 9999.0, "?"
    margin = 4.0
    for name, mlat, mlon in METRO_REFS:
        if abs(mlat - lat) > margin or abs(mlon - lon) > margin * 1.5:
            continue
        d = haversine(lat, lon, mlat, mlon)
        if d < best_d:
            best_d, best_name = d, name
    return best_d, best_name


def union_find(n, edges):
    parent = list(range(n))
    rank = [0] * n
    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]; x = parent[x]
        return x
    def union(a, b):
        ra, rb = find(a), find(b)
        if ra == rb: return
        if rank[ra] < rank[rb]: ra, rb = rb, ra
        parent[rb] = ra
        if rank[ra] == rank[rb]: rank[ra] += 1
    for a, b in edges:
        union(a, b)
    comps = defaultdict(list)
    for i in range(n):
        comps[find(i)].append(i)
    return list(comps.values())


def group_by_proximity(nodes, eps_km):
    """Group transit nodes within eps_km using single-linkage union-find."""
    n = len(nodes)
    if n == 0:
        return []
    gs = 0.05
    grid = defaultdict(list)
    for i, nd in enumerate(nodes):
        cell = (int(nd["lat"] / gs), int(nd["lon"] / gs))
        grid[cell].append(i)

    edges = set()
    deg = (eps_km + 0.3) / 111.0
    for i, nd in enumerate(nodes):
        clat, clon = nd["lat"], nd["lon"]
        for la in range(int((clat - deg) / gs), int((clat + deg) / gs) + 1):
            for lo in range(int((clon - deg) / gs), int((clon + deg) / gs) + 1):
                for j in grid.get((la, lo), []):
                    if j <= i: continue
                    if abs(nodes[j]["lat"] - clat) > deg: continue
                    if haversine(clat, clon, nodes[j]["lat"], nodes[j]["lon"]) <= eps_km:
                        edges.add((i, j))
    return union_find(n, edges)


def build_grid(recs, gs=0.05):
    g = defaultdict(list)
    for i, r in enumerate(recs):
        cell = (int(r["lat"] / gs), int(r["lon"] / gs))
        g[cell].append(i)
    return g


def any_within(lat, lon, recs, grid, radius_km, gs=0.05):
    deg = (radius_km + 0.3) / 111.0
    for la in range(int((lat - deg) / gs), int((lat + deg) / gs) + 1):
        for lo in range(int((lon - deg) / gs), int((lon + deg) / gs) + 1):
            for j in grid.get((la, lo), []):
                if haversine(lat, lon, recs[j]["lat"], recs[j]["lon"]) <= radius_km:
                    return True
    return False


def load_transit(path, category, transit_class_map=None):
    """Load transit node JSONL. transit_class_map: override category by transit_class field."""
    nodes = []
    if not path.exists():
        return nodes
    with open(path) as f:
        for line in f:
            try:
                r = json.loads(line)
                lat = r.get("latitude") or r.get("lat")
                lon = r.get("longitude") or r.get("lon")
                if lat is None or lon is None: continue
                iso = (r.get("iso_country_code") or r.get("country_code") or "")[:2].upper()
                if iso not in DISPLAY_ISO: continue
                cat = category
                if transit_class_map:
                    tc = r.get("transit_class") or ""
                    cat = transit_class_map.get(tc, category)
                nodes.append({
                    "lat": float(lat), "lon": float(lon), "iso": iso,
                    "category": cat,
                    "name": r.get("location_name") or r.get("name") or "",
                    "iata": r.get("iata") or r.get("iata_code") or "",
                    "operator": r.get("operator") or "",
                })
            except (json.JSONDecodeError, ValueError):
                continue
    return nodes


def tier_pks(all_cats: frozenset) -> int:
    """
    Strict co-location tiering — mirrors the Retail model. A Commuter co-location
    requires 2+ DISTINCT categories (enforced in build(); single-category transit
    nodes are dropped). `all_cats` includes car_rental when present.

    Categories: airport, intercity_rail, commuter_rail, metro_subway, car_rental

    T1: airport + any other category   (major intermodal hub)
        OR 3+ distinct categories
    T2: 2+ distinct RAIL types         (intercity + commuter, intercity + metro, …)
    T3: single rail type + car_rental  (basic park-and-ride / station car hire)
    """
    rail_cats = all_cats & {"intercity_rail", "commuter_rail", "metro_subway"}
    has_airport = "airport" in all_cats

    # T1 — airport-anchored intermodal hub, or richly multi-modal
    if has_airport and len(all_cats) >= 2:
        return 1
    if len(all_cats) >= 3:
        return 1
    # T2 — multi-modal rail
    if len(rail_cats) >= 2:
        return 2
    # T3 — single rail type + car rental
    return 3


def build(output_path: Path):
    # ── Load transit nodes ────────────────────────────────────────────────────
    all_nodes: list[dict] = []

    airports = load_transit(PLACES_DIR / "cleansed-civic-airports.jsonl", "airport")
    print(f"  airports:       {len(airports):,}")
    all_nodes.extend(airports)

    intercity = load_transit(PLACES_DIR / "cleansed-civic-railway.jsonl", "intercity_rail")
    print(f"  intercity rail: {len(intercity):,}")
    all_nodes.extend(intercity)

    commuter_path = PLACES_DIR / "cleansed-civic-railway-commuter.jsonl"
    if commuter_path.exists():
        commuter = load_transit(commuter_path, "commuter_rail",
                                transit_class_map={"subway": "metro_subway",
                                                   "light_rail": "metro_subway",
                                                   "metro": "metro_subway"})
        print(f"  commuter/metro: {len(commuter):,}")
        all_nodes.extend(commuter)
    else:
        print(f"  commuter/metro: 0 (run ingest-osm-railway-commuter.py --all first)")

    print(f"Transit nodes total: {len(all_nodes):,}")

    # ── Load car rental records ───────────────────────────────────────────────
    car_recs: list[dict] = []
    for cid in CAR_RENTAL_CHAINS:
        p = CHAIN_DIR / f"{cid}.jsonl"
        if not p.exists(): continue
        with open(p) as f:
            for line in f:
                try:
                    r = json.loads(line)
                    lat, lon = r.get("latitude"), r.get("longitude")
                    if lat is None or lon is None: continue
                    iso = (r.get("iso_country_code") or "")[:2].upper()
                    if iso not in DISPLAY_ISO: continue
                    car_recs.append({"lat": float(lat), "lon": float(lon)})
                except (json.JSONDecodeError, ValueError):
                    continue
    car_grid = build_grid(car_recs)
    print(f"Car rental records: {len(car_recs):,}")

    # ── Group adjacent transit nodes (multi-modal detection) ─────────────────
    print(f"\nGrouping transit nodes within {TRANSIT_GROUP_KM} km ...")
    comp_list = group_by_proximity(all_nodes, TRANSIT_GROUP_KM)
    print(f"  Transit groups: {len(comp_list):,}")

    # ── Score each transit group ──────────────────────────────────────────────
    features = []
    n_skipped = 0

    for comp in comp_list:
        lats = [all_nodes[i]["lat"] for i in comp]
        lons = [all_nodes[i]["lon"] for i in comp]
        clat = sum(lats) / len(lats)
        clon = sum(lons) / len(lons)

        iso_counter = Counter(all_nodes[i]["iso"] for i in comp)
        iso = iso_counter.most_common(1)[0][0]

        transit_cats = frozenset(all_nodes[i]["category"] for i in comp)

        # Metro distance kept for display/context only — NOT a filter (matches Retail model)
        metro_d, metro_name = nearest_metro(clat, clon)

        # Car rental enrichment
        car_rental = any_within(clat, clon, car_recs, car_grid, CAR_RENTAL_KM)

        # Full category set (transit + car_rental)
        all_cats = set(transit_cats)
        if car_rental:
            all_cats.add("car_rental")

        # Strict co-location: require 2+ DISTINCT categories (mirrors Retail tier_of n>=2)
        if len(all_cats) < 2:
            n_skipped += 1
            continue

        t = tier_pks(frozenset(all_cats))

        # Representative name: prefer airport name > largest intercity station
        names = [all_nodes[i]["name"] for i in comp if all_nodes[i]["name"]]
        iatas = [all_nodes[i]["iata"] for i in comp if all_nodes[i]["iata"]]
        display_name = (iatas[0] if iatas else "") or (names[0] if names else "")

        cats_list = sorted(transit_cats)
        if car_rental:
            cats_list.append("car_rental")

        features.append({
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [round(clon, 6), round(clat, 6)]},
            "properties": {
                "name":              display_name,
                "lat":               round(clat, 6),
                "lon":               round(clon, 6),
                "iso":               iso,
                "commuter_tier":     t,
                "transit_categories":cats_list,
                "multi_modal":       len(transit_cats) > 1,
                "car_rental":        car_rental,
                "metro_dist_km":     round(metro_d, 1),
                "nearest_metro":     metro_name,
                "node_count":        len(comp),
                "archetype":         "commuter",
            },
        })

    n_t1 = sum(1 for f in features if f["properties"]["commuter_tier"] == 1)
    n_t2 = sum(1 for f in features if f["properties"]["commuter_tier"] == 2)
    n_t3 = sum(1 for f in features if f["properties"]["commuter_tier"] == 3)

    print(f"  Skipped — single category (not a co-location): {n_skipped:,}")
    print(f"  Valid PKS clusters: {len(features):,}  (T1={n_t1} T2={n_t2} T3={n_t3})")

    iso_counts = Counter(f["properties"]["iso"] for f in features)
    print("\n  ISO   Total  T1  T2  T3")
    for iso, total in sorted(iso_counts.items(), key=lambda x: -x[1]):
        t1 = sum(1 for f in features if f["properties"]["iso"] == iso and f["properties"]["commuter_tier"] == 1)
        t2 = sum(1 for f in features if f["properties"]["iso"] == iso and f["properties"]["commuter_tier"] == 2)
        t3 = sum(1 for f in features if f["properties"]["iso"] == iso and f["properties"]["commuter_tier"] == 3)
        print(f"  {iso}   {total:5d}  {t1:3d}  {t2:3d}  {t3:3d}")

    # ── Write GeoJSON ─────────────────────────────────────────────────────────
    output_path.parent.mkdir(parents=True, exist_ok=True)
    geojson = {"type": "FeatureCollection", "features": features}
    with open(output_path, "w") as f:
        json.dump(geojson, f, separators=(",", ":"))
    print(f"\nWrote {output_path}  ({len(features):,} features: T1={n_t1} T2={n_t2} T3={n_t3})")
    return features


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", default=str(WORK / "archetype-pks.geojson"))
    args = parser.parse_args()
    build(Path(args.output))


if __name__ == "__main__":
    main()
