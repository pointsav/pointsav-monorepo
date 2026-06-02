#!/usr/bin/env python3
"""
analyze-archetype-categories.py — Distribution analysis for independent VWH and PKS clustering.

Runs BEFORE build-vwh-clusters.py and build-pks-clusters.py to inform tier boundary decisions.
Prints only — no files written.

Usage:
    python3 analyze-archetype-categories.py
    python3 analyze-archetype-categories.py --vwh-only
    python3 analyze-archetype-categories.py --pks-only
"""

import json
import math
import sys
import argparse
from pathlib import Path
from collections import defaultdict, Counter

SCRIPT_DIR  = Path(__file__).parent
TOTEBOX     = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")
CHAIN_DIR   = TOTEBOX / "service-fs" / "service-business"
PLACES_DIR  = TOTEBOX / "service-places"

DISPLAY_ISO = {
    "US", "CA", "MX",
    "GB", "SE", "DK", "NO", "FI", "IS",
    "FR", "DE", "ES", "IT", "GR", "PL", "AT", "NL", "PT",
}

# ── METRO REFERENCE POINTS (same list as test-cluster-archetypes.py) ──────────
METRO_REFS = [
    ("New York",     40.7128, -74.0060), ("Los Angeles",  34.0522,-118.2437),
    ("Chicago",      41.8781, -87.6298), ("Dallas",       32.7767, -96.7970),
    ("Houston",      29.7604, -95.3698), ("Phoenix",      33.4484,-112.0740),
    ("Philadelphia", 39.9526, -75.1652), ("San Antonio",  29.4241, -98.4936),
    ("San Diego",    32.7157,-117.1611), ("San Jose",     37.3382,-121.8863),
    ("Austin",       30.2672, -97.7431), ("Jacksonville", 30.3322, -81.6557),
    ("Columbus",     39.9612, -82.9988), ("Indianapolis", 39.7684, -86.1581),
    ("Seattle",      47.6062,-122.3321), ("Denver",       39.7392,-104.9903),
    ("Nashville",    36.1627, -86.7816), ("Oklahoma City",35.4676, -97.5164),
    ("Portland",     45.5051,-122.6750), ("Las Vegas",    36.1699,-115.1398),
    ("Memphis",      35.1495, -90.0490), ("Louisville",   38.2527, -85.7585),
    ("Baltimore",    39.2904, -76.6122), ("Milwaukee",    43.0389, -87.9065),
    ("Albuquerque",  35.0844,-106.6504), ("Tucson",       32.2226,-110.9747),
    ("Fresno",       36.7378,-119.7871), ("Sacramento",   38.5816,-121.4944),
    ("Kansas City",  39.0997, -94.5786), ("Mesa",         33.4152,-111.8315),
    ("Atlanta",      33.7490, -84.3880), ("Miami",        25.7617, -80.1918),
    ("Minneapolis",  44.9778, -93.2650), ("Cleveland",    41.4993, -81.6944),
    ("Raleigh",      35.7796, -78.6382), ("Omaha",        41.2565, -95.9345),
    ("Colorado Springs",38.8339,-104.8214),("Virginia Beach",36.8529,-75.9780),
    ("Long Beach",   33.7701,-118.1937), ("Tampa",        27.9506, -82.4572),
    ("New Orleans",  29.9511, -90.0715), ("Arlington",    32.7357, -97.1081),
    ("Wichita",      37.6872, -97.3301), ("Bakersfield",  35.3733,-119.0187),
    ("Olympia",      47.0379,-122.9007), ("Fargo",        46.8772, -96.7898),
    ("State College",40.7934, -77.8600), ("Baton Rouge",  30.4515, -91.1871),
    ("Vancouver",    49.2827,-123.1207), ("Toronto",      43.6532, -79.3832),
    ("Montreal",     45.5017, -73.5673), ("Calgary",      51.0447,-114.0719),
    ("Ottawa",       45.4215, -75.6972), ("Edmonton",     53.5461,-113.4938),
    ("Winnipeg",     49.8951, -97.1384), ("Quebec City",  46.8139, -71.2080),
    ("Kitchener",    43.4516, -80.4925), ("Halifax",      44.6488, -63.5752),
    ("Mexico City",  19.4326, -99.1332), ("Guadalajara",  20.6597,-103.3496),
    ("Monterrey",    25.6866, -100.3161),("Puebla",       19.0414, -98.2063),
    ("Tijuana",      32.5149,-117.0382), ("Cancún",       21.1619, -86.8515),
    ("London",       51.5074,  -0.1278), ("Birmingham",   52.4862,  -1.8904),
    ("Manchester",   53.4808,  -2.2426), ("Glasgow",      55.8642,  -4.2518),
    ("Liverpool",    53.4084,  -2.9916), ("Bristol",      51.4545,  -2.5879),
    ("Edinburgh",    55.9533,  -3.1883), ("Leeds",        53.8008,  -1.5491),
    ("Sheffield",    53.3811,  -1.4701), ("Cardiff",      51.4816,  -3.1791),
    ("Stockholm",    59.3293,  18.0686), ("Gothenburg",   57.7089,  11.9746),
    ("Malmö",        55.6050,  13.0038), ("Uppsala",      59.8586,  17.6389),
    ("Copenhagen",   55.6761,  12.5683), ("Aarhus",       56.1629,  10.2039),
    ("Odense",       55.4038,  10.4024), ("Oslo",         59.9139,  10.7522),
    ("Bergen",       60.3913,   5.3221), ("Trondheim",    63.4305,  10.3951),
    ("Helsinki",     60.1699,  24.9384), ("Tampere",      61.4978,  23.7610),
    ("Turku",        60.4518,  22.2666), ("Reykjavik",    64.1355, -21.8954),
    ("Paris",        48.8566,   2.3522), ("Marseille",    43.2965,   5.3698),
    ("Lyon",         45.7640,   4.8357), ("Toulouse",     43.6047,   1.4442),
    ("Nice",         43.7102,   7.2620), ("Nantes",       47.2184,  -1.5536),
    ("Strasbourg",   48.5734,   7.7521), ("Montpellier",  43.6119,   3.8772),
    ("Bordeaux",     44.8378,  -0.5792), ("Lille",        50.6292,   3.0573),
    ("Rennes",       48.1173,  -1.6778), ("Reims",        49.2583,   4.0317),
    ("Berlin",       52.5200,  13.4050), ("Hamburg",      53.5753,   9.9950),
    ("Munich",       48.1351,  11.5820), ("Cologne",      50.9333,   6.9500),
    ("Frankfurt",    50.1109,   8.6821), ("Stuttgart",    48.7758,   9.1829),
    ("Düsseldorf",   51.2217,   6.7762), ("Dortmund",     51.5136,   7.4653),
    ("Essen",        51.4556,   7.0116), ("Leipzig",      51.3397,  12.3731),
    ("Bremen",       53.0793,   8.8017), ("Dresden",      51.0504,  13.7373),
    ("Hannover",     52.3759,   9.7320), ("Nuremberg",    49.4521,  11.0767),
    ("Regensburg",   49.0134,  12.1016), ("Halle",        51.4825,  11.9675),
    ("Magdeburg",    52.1205,  11.6276), ("Karlsruhe",    49.0069,   8.4037),
    ("Wuppertal",    51.2562,   7.1508), ("Bielefeld",    52.0302,   8.5325),
    ("Madrid",       40.4168,  -3.7038), ("Barcelona",    41.3851,   2.1734),
    ("Valencia",     39.4699,  -0.3763), ("Seville",      37.3891,  -5.9845),
    ("Zaragoza",     41.6488,  -0.8891), ("Málaga",       36.7213,  -4.4214),
    ("Murcia",       37.9922,  -1.1307), ("Palma",        39.5696,   2.6502),
    ("Rome",         41.9028,  12.4964), ("Milan",        45.4654,   9.1866),
    ("Naples",       40.8518,  14.2681), ("Turin",        45.0703,   7.6869),
    ("Palermo",      38.1157,  13.3615), ("Genoa",        44.4056,   8.9463),
    ("Bologna",      44.4949,  11.3426), ("Florence",     43.7696,  11.2558),
    ("Bari",         41.1171,  16.8719), ("Athens",       37.9838,  23.7275),
    ("Thessaloniki", 40.6401,  22.9444), ("Warsaw",       52.2297,  21.0122),
    ("Kraków",       50.0647,  19.9450), ("Łódź",         51.7592,  19.4560),
    ("Wrocław",      51.1079,  17.0385), ("Poznań",       52.4064,  16.9252),
    ("Gdańsk",       54.3520,  18.6466), ("Szczecin",     53.4289,  14.5530),
    ("Bydgoszcz",    53.1235,  18.0084), ("Lublin",       51.2465,  22.5684),
    ("Katowice",     50.2649,  19.0238), ("Kielce",       50.8661,  20.6286),
    ("Olsztyn",      53.7784,  20.4801), ("Vienna",       48.2082,  16.3738),
    ("Graz",         47.0707,  15.4395), ("Linz",         48.3069,  14.2858),
    ("Amsterdam",    52.3676,   4.9041), ("Rotterdam",    51.9244,   4.4777),
    ("The Hague",    52.0705,   4.3007), ("Utrecht",      52.0907,   5.1214),
    ("Lisbon",       38.7169,  -9.1399), ("Porto",        41.1579,  -8.6291),
    ("Braga",        41.5454,  -8.4265), ("Coimbra",      40.2033,  -8.4103),
    ("Tallinn",      59.4370,  24.7536), ("Riga",         56.9496,  24.1052),
]

METRO_LATS = [m[1] for m in METRO_REFS]
METRO_LONS = [m[2] for m in METRO_REFS]


def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = math.sin(dlat / 2) ** 2 + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2)) * math.sin(dlon / 2) ** 2
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def nearest_metro(lat, lon):
    best_d, best_name = 9999, "?"
    margin = 3.0
    for name, mlat, mlon in METRO_REFS:
        if abs(mlat - lat) > margin or abs(mlon - lon) > margin * 1.5:
            continue
        d = haversine(lat, lon, mlat, mlon)
        if d < best_d:
            best_d, best_name = d, name
    return best_d, best_name


# ── UNION-FIND (same as build-clusters.py) ────────────────────────────────────

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


def two_pass_dbscan(recs, eps_tight=1.0, eps_loose=3.0):
    """Two-pass tight-first DBSCAN. recs must have lat/lon keys."""
    n = len(recs)
    if n == 0:
        return []

    # Build grid for fast neighbour lookup
    grid_size = 0.05
    grid = defaultdict(list)
    for i, r in enumerate(recs):
        cell = (int(r["lat"] / grid_size), int(r["lon"] / grid_size))
        grid[cell].append(i)

    def edges_within(eps):
        deg = (eps + 0.3) / 111.0
        result = set()
        for i, r in enumerate(recs):
            clat, clon = r["lat"], r["lon"]
            for la in range(int((clat - deg) / grid_size), int((clat + deg) / grid_size) + 1):
                for lo in range(int((clon - deg) / grid_size), int((clon + deg) / grid_size) + 1):
                    for j in grid.get((la, lo), []):
                        if j <= i: continue
                        if abs(recs[j]["lat"] - clat) > deg: continue
                        if haversine(clat, clon, recs[j]["lat"], recs[j]["lon"]) <= eps:
                            result.add((i, j))
        return result

    # Pass 1: tight nuclei
    tight_comps = union_find(n, edges_within(eps_tight))
    atom_of = {}
    for comp in tight_comps:
        for idx in comp:
            atom_of[idx] = comp[0]

    # Pass 2: loose expansion; respect atom boundaries
    loose_edges = set()
    for a, b in edges_within(eps_loose):
        ra, rb = atom_of[a], atom_of[b]
        if ra != rb:
            loose_edges.add((min(ra, rb), max(ra, rb)))

    atom_ids = sorted(set(atom_of.values()))
    atom_idx = {v: i for i, v in enumerate(atom_ids)}
    loose_comps = union_find(len(atom_ids), {(atom_idx[a], atom_idx[b]) for a, b in loose_edges})

    clusters = []
    for comp in loose_comps:
        member_atoms = [atom_ids[i] for i in comp]
        member_indices = []
        for a in member_atoms:
            for orig_idx, ai in atom_of.items():
                if ai == a:
                    member_indices.append(orig_idx)
        clusters.append(member_indices)
    return clusters


def load_jsonl(path, lat_key="latitude", lon_key="longitude", iso_key="iso_country_code"):
    recs = []
    if not path.exists():
        return recs
    with open(path) as f:
        for line in f:
            try:
                r = json.loads(line)
                lat = r.get(lat_key)
                lon = r.get(lon_key)
                if lat is None or lon is None:
                    continue
                iso = (r.get(iso_key) or "")[:2].upper()
                if iso not in DISPLAY_ISO:
                    continue
                recs.append({"lat": float(lat), "lon": float(lon), "iso": iso,
                             "name": r.get("location_name") or r.get("name") or "",
                             "city": r.get("city") or ""})
            except (json.JSONDecodeError, ValueError):
                continue
    return recs


# ── A. URBAN FRINGE ANALYSIS ──────────────────────────────────────────────────

VWH_CHAINS = {
    "hardware":      ["home-depot-us", "home-depot-ca", "home-depot-mx",
                      "lowes-us", "lowes-ca",
                      "bq-uk", "wickes-uk",
                      "leroy-merlin-fr", "leroy-merlin-es", "leroy-merlin-it",
                      "leroy-merlin-gr", "leroy-merlin-pl", "leroy-merlin-pt",
                      "castorama-fr", "castorama-pl",
                      "obi-de", "obi-at", "obi-it", "obi-pl",
                      "hornbach-de", "bauhaus-de", "bauhaus-at",
                      "bauhaus-dk", "bauhaus-fi", "bauhaus-no",
                      "hellweg-de", "biltema-se", "praxis-nl", "gamma-nl",
                      "mr-bricolage-fr", "alaska-industrial-hardware-us"],
    "mro_industrial":["wurth-de", "fastenal-us", "grainger-us", "hilti-ch",
                      "princess-auto-ca"],
    "tool_rental":   ["united-rentals-us", "sunbelt-rentals-us",
                      "loxam-fr", "kiloutou-fr", "boels-rental-nl",
                      "ramirent-fi", "cramo-fi", "hss-hire-uk", "speedy-hire-uk",
                      "ahlsell-se"],
    "auto_parts":    ["autozone-us", "oreilly-auto-us", "napa-us",
                      "napa-ca", "advance-auto-us", "autozone-mx",
                      "halfords-uk"],
    "electrical":    ["cef-uk", "rexel-fr", "ahlsell-se"],
    "flooring":      ["floor-decor-us", "topps-tiles-uk"],
    "lumber":        ["84-lumber-us", "builders-firstsource-us",
                      "kent-building-supplies-ca"],
    "plumbing":      ["ferguson-us", "wolseley-uk"],
    "paint":         ["sherwin-williams-us"],
    "welding":       ["boc-uk"],
}

HYPERMARKET_CHAINS = [
    "walmart-us", "walmart-ca", "walmart-mx", "target-us",
    "whole-foods-us", "kroger-us", "heb-us", "fred-meyer-us",
    "asda-uk", "morrisons-uk", "tesco-uk", "sainsburys-uk",
    "aldi-uk", "lidl-uk", "lidl-de", "lidl-fr", "lidl-es", "lidl-pt",
    "lidl-it", "lidl-pl", "lidl-at", "lidl-nl", "lidl-be",
    "aldi-de", "aldi-fr", "aldi-nl", "aldi-at",
    "rewe-de", "edeka-de", "kaufland-de", "real-de", "ecenter-de", "globus-de",
    "carrefour-hypermarket-fr", "carrefour-hypermarket-es",
    "carrefour-hypermarket-it", "carrefour-hypermarket-pl",
    "geant-casino-fr", "intermarche-hyper-fr",
    "leclerc-fr", "leclerc-pl",
    "auchan-fr", "auchan-pl", "auchan-pt",
    "alcampo-es", "hipercor-es",
    "interspar-at", "billa-plus-at",
    "bilka-dk", "foetex-dk",
    "albert-heijn-xl-nl", "jumbo-nl",
    "k-citymarket-fi",
    "coop-forum-se",
    "continente-pt",
    "esselunga-it", "ipercoop-it", "famila-it",
    "sklavenitis-gr",
    "chedraui-mx",
    "costco-us", "costco-ca", "costco-mx",
    "sams-club-us",
    "global-fr",
    "e-leclerc-drive-fr",
    "hagkaup-is",
]

VWH_MIN_METRO_KM = 3.0
VWH_MAX_METRO_KM = 150.0
HYPER_DISQUALIFY_KM = 1.0


def analyze_vwh():
    print("=" * 70)
    print("URBAN FRINGE (VWH) — Category Distribution Analysis")
    print("=" * 70)

    # Load all VWH chains
    all_recs = []
    chain_counts = {}
    for cat, chain_ids in VWH_CHAINS.items():
        for cid in chain_ids:
            p = CHAIN_DIR / f"{cid}.jsonl"
            recs = load_jsonl(p)
            if recs:
                for r in recs:
                    r["category"] = cat
                    r["chain_id"] = cid
                all_recs.extend(recs)
                chain_counts[cid] = len(recs)

    print(f"\nRecords loaded: {len(all_recs):,} across {len(chain_counts)} chains")

    by_cat = Counter(r["category"] for r in all_recs)
    print("\nRecords by category:")
    for cat, n in sorted(by_cat.items(), key=lambda x: -x[1]):
        print(f"  {cat:<20} {n:5,}")

    # Load hypermarket records for disqualification
    hyper_recs = []
    for cid in HYPERMARKET_CHAINS:
        recs = load_jsonl(CHAIN_DIR / f"{cid}.jsonl")
        hyper_recs.extend(recs)
    print(f"\nHypermarket records for disqualifier: {len(hyper_recs):,}")

    # Build hypermarket grid
    hgrid = defaultdict(list)
    gs = 0.05
    for i, r in enumerate(hyper_recs):
        cell = (int(r["lat"] / gs), int(r["lon"] / gs))
        hgrid[cell].append(i)

    def has_nearby_hypermarket(lat, lon, radius_km=HYPER_DISQUALIFY_KM):
        deg = (radius_km + 0.3) / 111.0
        for la in range(int((lat - deg) / gs), int((lat + deg) / gs) + 1):
            for lo in range(int((lon - deg) / gs), int((lon + deg) / gs) + 1):
                for j in hgrid.get((la, lo), []):
                    if haversine(lat, lon, hyper_recs[j]["lat"], hyper_recs[j]["lon"]) <= radius_km:
                        return True
        return False

    # Run two-pass DBSCAN
    print("\nRunning two-pass DBSCAN (eps_tight=1.0 km, eps_loose=3.0 km)...")
    comp_list = two_pass_dbscan(all_recs, eps_tight=1.0, eps_loose=3.0)
    print(f"  Raw clusters: {len(comp_list):,}")

    # Analyse each cluster
    clusters = []
    skipped_metro = skipped_hyper = skipped_single = 0

    for comp in comp_list:
        if len(comp) < 1:
            continue
        lats = [all_recs[i]["lat"] for i in comp]
        lons = [all_recs[i]["lon"] for i in comp]
        clat = sum(lats) / len(lats)
        clon = sum(lons) / len(lons)
        iso_counts = Counter(all_recs[i]["iso"] for i in comp)
        iso = iso_counts.most_common(1)[0][0]

        metro_d, metro_name = nearest_metro(clat, clon)
        if not (VWH_MIN_METRO_KM <= metro_d <= VWH_MAX_METRO_KM):
            skipped_metro += 1
            continue

        if has_nearby_hypermarket(clat, clon):
            skipped_hyper += 1
            continue

        cats = set(all_recs[i]["category"] for i in comp)
        chains = set(all_recs[i]["chain_id"] for i in comp)

        # Span
        span = 0.0
        if len(comp) > 1:
            for i in range(len(comp)):
                for j in range(i + 1, len(comp)):
                    d = haversine(all_recs[comp[i]]["lat"], all_recs[comp[i]]["lon"],
                                  all_recs[comp[j]]["lat"], all_recs[comp[j]]["lon"])
                    if d > span:
                        span = d

        clusters.append({
            "lat": clat, "lon": clon, "iso": iso,
            "categories": frozenset(cats), "chains": frozenset(chains),
            "metro_d": metro_d, "metro_name": metro_name,
            "span_km": span, "member_count": len(comp),
        })

    print(f"  After metro filter:         skipped {skipped_metro:,}")
    print(f"  After hypermarket disqualifier: skipped {skipped_hyper:,}")
    print(f"  Valid clusters: {len(clusters):,}")

    # Category combination frequency
    print("\n── Category combinations (top 30) ──")
    combo_counts = Counter(frozenset(c["categories"]) for c in clusters)
    for combo, n in sorted(combo_counts.items(), key=lambda x: -x[1])[:30]:
        cats_str = " + ".join(sorted(combo))
        print(f"  {n:5d}  {cats_str}")

    # Per-country distribution
    print("\n── Clusters per country ──")
    iso_counts = Counter(c["iso"] for c in clusters)
    for iso, n in sorted(iso_counts.items(), key=lambda x: -x[1]):
        print(f"  {iso}: {n}")

    # T1/T2/T3 under different rule sets
    print("\n── T1/T2/T3 tier rules — sensitivity ──")
    print(f"  {'Rule set':<55}  T1    T2    T3   Total")
    print(f"  {'-'*55}  ----  ----  ----  -----")

    rule_sets = [
        ("T1=hw+2+, T2=hw+1|2cats, T3=hw|1cat",
         lambda c: (1 if ("hardware" in c and len(c - {"hardware"}) >= 2) or len(c) >= 3
                    else 2 if ("hardware" in c and len(c - {"hardware"}) >= 1) or len(c) >= 2
                    else 3)),
        ("T1=hw+2+|3cats, T2=hw+1|2cats, T3=rest",
         lambda c: (1 if ("hardware" in c and len(c - {"hardware"}) >= 2) or (len(c) >= 3 and "hardware" not in c)
                    else 2 if ("hardware" in c and len(c - {"hardware"}) >= 1) or len(c) >= 2
                    else 3)),
        ("T1=3+cats, T2=2cats, T3=1cat",
         lambda c: 1 if len(c) >= 3 else 2 if len(c) == 2 else 3),
        ("T1=2+cats, T2=1cat+hw, T3=bare-hw",
         lambda c: (1 if len(c) >= 2 else 2 if "hardware" in c else 3)),
        ("T1=hw+A(2+)|4cats, T2=hw+A(1)|3cats, T3=rest",
         lambda c: (
             1 if ("hardware" in c and len(c & {"mro_industrial","tool_rental","auto_parts","electrical","flooring","lumber"}) >= 2) or len(c) >= 4
             else 2 if ("hardware" in c and len(c & {"mro_industrial","tool_rental","auto_parts","electrical","flooring","lumber"}) >= 1) or len(c) >= 3
             else 3)),
    ]

    for label, tier_fn in rule_sets:
        tiers = Counter(tier_fn(c["categories"]) for c in clusters)
        total = sum(tiers.values())
        print(f"  {label:<55}  {tiers[1]:4d}  {tiers[2]:4d}  {tiers[3]:4d}  {total:5d}")

    # Single-category breakdown
    print("\n── Single-category clusters (T3 candidates) ──")
    single = [c for c in clusters if len(c["categories"]) == 1]
    single_cat_counts = Counter(list(c["categories"])[0] for c in single)
    for cat, n in sorted(single_cat_counts.items(), key=lambda x: -x[1]):
        print(f"  {cat:<20} {n:4d} single-category clusters")

    print(f"\n  Total clusters: {len(clusters):,}  |  "
          f"Multi-cat: {sum(1 for c in clusters if len(c['categories']) > 1):,}  |  "
          f"Single-cat: {len(single):,}")


# ── B. COMMUTER ANALYSIS ─────────────────────────────────────────────────────

CAR_RENTAL_CHAINS = [
    "enterprise-us", "hertz-us", "avis-us", "europcar-fr", "sixt-de",
]

PKS_PER_CLASS_RANGE = {
    "airport":       (15.0, 150.0),
    "intercity_rail":(15.0, 150.0),
    "commuter_rail": ( 5.0,  80.0),
    "metro_subway":  ( 3.0,  35.0),
}

TRANSIT_GROUP_KM = 3.0   # group adjacent transit nodes within this distance
CAR_RENTAL_KM    = 2.0   # car rental within this distance → enriched


def load_transit_nodes():
    nodes = []

    # Airports
    airports_path = PLACES_DIR / "cleansed-civic-airports.jsonl"
    if airports_path.exists():
        with open(airports_path) as f:
            for line in f:
                try:
                    r = json.loads(line)
                    lat = r.get("latitude") or r.get("lat")
                    lon = r.get("longitude") or r.get("lon")
                    if lat is None or lon is None: continue
                    iso = (r.get("iso_country_code") or "")[:2].upper()
                    if iso not in DISPLAY_ISO: continue
                    nodes.append({"lat": float(lat), "lon": float(lon), "iso": iso,
                                  "category": "airport",
                                  "name": r.get("location_name") or r.get("name") or ""})
                except (json.JSONDecodeError, ValueError): continue

    # Intercity rail
    rail_path = PLACES_DIR / "cleansed-civic-railway.jsonl"
    if rail_path.exists():
        with open(rail_path) as f:
            for line in f:
                try:
                    r = json.loads(line)
                    lat = r.get("latitude") or r.get("lat")
                    lon = r.get("longitude") or r.get("lon")
                    if lat is None or lon is None: continue
                    iso = (r.get("iso_country_code") or r.get("country_code") or "")[:2].upper()
                    if iso not in DISPLAY_ISO: continue
                    nodes.append({"lat": float(lat), "lon": float(lon), "iso": iso,
                                  "category": "intercity_rail",
                                  "name": r.get("location_name") or r.get("name") or ""})
                except (json.JSONDecodeError, ValueError): continue

    # Commuter / metro (if available)
    commuter_path = PLACES_DIR / "cleansed-civic-railway-commuter.jsonl"
    if commuter_path.exists():
        with open(commuter_path) as f:
            for line in f:
                try:
                    r = json.loads(line)
                    lat = r.get("latitude") or r.get("lat")
                    lon = r.get("longitude") or r.get("lon")
                    if lat is None or lon is None: continue
                    iso = (r.get("iso_country_code") or r.get("country_code") or "")[:2].upper()
                    if iso not in DISPLAY_ISO: continue
                    tc = r.get("transit_class") or ""
                    cat = "metro_subway" if tc in ("subway", "light_rail", "metro") else "commuter_rail"
                    nodes.append({"lat": float(lat), "lon": float(lon), "iso": iso,
                                  "category": cat,
                                  "name": r.get("location_name") or r.get("name") or ""})
                except (json.JSONDecodeError, ValueError): continue

    return nodes


def analyze_pks():
    print("\n" + "=" * 70)
    print("COMMUTER (PKS) — Category Distribution Analysis")
    print("=" * 70)

    nodes = load_transit_nodes()
    print(f"\nTransit nodes loaded: {len(nodes):,}")
    cat_counts = Counter(n["category"] for n in nodes)
    for cat, n in sorted(cat_counts.items(), key=lambda x: -x[1]):
        print(f"  {cat:<20} {n:6,}")

    if not nodes:
        print("  No transit data available.")
        return

    # Load car rental records
    car_recs = []
    for cid in CAR_RENTAL_CHAINS:
        recs = load_jsonl(CHAIN_DIR / f"{cid}.jsonl")
        car_recs.extend(recs)
    print(f"\nCar rental records: {len(car_recs):,}")

    # Build car rental grid
    cgrid = defaultdict(list)
    gs = 0.05
    for i, r in enumerate(car_recs):
        cell = (int(r["lat"] / gs), int(r["lon"] / gs))
        cgrid[cell].append(i)

    def has_car_rental(lat, lon, radius_km=CAR_RENTAL_KM):
        deg = (radius_km + 0.3) / 111.0
        for la in range(int((lat - deg) / gs), int((lat + deg) / gs) + 1):
            for lo in range(int((lon - deg) / gs), int((lon + deg) / gs) + 1):
                for j in cgrid.get((la, lo), []):
                    if haversine(lat, lon, car_recs[j]["lat"], car_recs[j]["lon"]) <= radius_km:
                        return True
        return False

    # Group adjacent transit nodes (multi-modal proximity)
    print(f"\nGrouping transit nodes within {TRANSIT_GROUP_KM} km (multi-modal)...")
    comp_list = two_pass_dbscan(nodes, eps_tight=0.5, eps_loose=TRANSIT_GROUP_KM)
    print(f"  Transit clusters: {len(comp_list):,} (from {len(nodes):,} individual nodes)")

    multi_modal = sum(1 for c in comp_list
                      if len(set(nodes[i]["category"] for i in c)) > 1)
    print(f"  Multi-modal (2+ transit types): {multi_modal:,}")

    # Score each transit cluster
    clusters = []
    skipped_metro = 0
    for comp in comp_list:
        lats = [nodes[i]["lat"] for i in comp]
        lons = [nodes[i]["lon"] for i in comp]
        clat = sum(lats) / len(lats)
        clon = sum(lons) / len(lons)
        iso_counter = Counter(nodes[i]["iso"] for i in comp)
        iso = iso_counter.most_common(1)[0][0]

        metro_d, metro_name = nearest_metro(clat, clon)
        transit_cats = set(nodes[i]["category"] for i in comp)

        # Per-class metro distance filter — use the most permissive range that applies
        ranges = [PKS_PER_CLASS_RANGE[cat] for cat in transit_cats]
        min_d = min(r[0] for r in ranges)
        max_d = max(r[1] for r in ranges)
        if not (min_d <= metro_d <= max_d):
            skipped_metro += 1
            continue

        enriched = has_car_rental(clat, clon)
        if enriched:
            transit_cats.add("car_rental")

        clusters.append({
            "lat": clat, "lon": clon, "iso": iso,
            "categories": frozenset(transit_cats),
            "metro_d": metro_d, "metro_name": metro_name,
            "enriched": enriched,
        })

    print(f"  After metro filter: skipped {skipped_metro:,}")
    print(f"  Valid transit clusters: {len(clusters):,}")

    # Category combination frequency
    print("\n── Transit category combinations (top 20) ──")
    combo_counts = Counter(frozenset(c["categories"]) for c in clusters)
    for combo, n in sorted(combo_counts.items(), key=lambda x: -x[1])[:20]:
        cats_str = " + ".join(sorted(combo))
        print(f"  {n:5d}  {cats_str}")

    # Car rental enrichment rate
    enriched_n = sum(1 for c in clusters if "car_rental" in c["categories"])
    print(f"\n  Car rental co-location: {enriched_n:,} of {len(clusters):,} clusters "
          f"({100*enriched_n/max(len(clusters),1):.1f}%)")

    # Per-country
    print("\n── Clusters per country ──")
    iso_cnts = Counter(c["iso"] for c in clusters)
    for iso, n in sorted(iso_cnts.items(), key=lambda x: -x[1]):
        print(f"  {iso}: {n}")

    # T1/T2/T3 sensitivity
    print("\n── T1/T2/T3 tier rules — sensitivity ──")
    print(f"  {'Rule set':<65}  T1    T2    T3   Total")
    print(f"  {'-'*65}  ----  ----  ----  -----")

    transit_only_cats = {"airport", "intercity_rail", "commuter_rail", "metro_subway"}

    rule_sets = [
        ("T1=2+transit|transit+rental, T2=1transit+rental, T3=1transit",
         lambda c: (1 if len(c & transit_only_cats) >= 2 or ("car_rental" in c and bool(c & transit_only_cats))
                    else 2 if "car_rental" in c
                    else 3)),
        ("T1=2+transit, T2=1transit+rental|airport, T3=rest",
         lambda c: (1 if len(c & transit_only_cats) >= 2
                    else 2 if "car_rental" in c or "airport" in c
                    else 3)),
        ("T1=airport+rail, T2=airport|rail+rental, T3=rail-alone",
         lambda c: (1 if "airport" in c and bool(c & {"intercity_rail","commuter_rail","metro_subway"})
                    else 2 if "airport" in c or "car_rental" in c
                    else 3)),
        ("T1=airport+rental|3cats, T2=2cats, T3=1cat",
         lambda c: (1 if ("airport" in c and "car_rental" in c) or len(c) >= 3
                    else 2 if len(c) >= 2
                    else 3)),
    ]

    for label, tier_fn in rule_sets:
        tiers = Counter(tier_fn(c["categories"]) for c in clusters)
        total = sum(tiers.values())
        print(f"  {label:<65}  {tiers[1]:4d}  {tiers[2]:4d}  {tiers[3]:4d}  {total:5d}")

    # Metro distance profile
    print("\n── Metro distance profile (km from nearest large city) ──")
    bins = [(0,15),(15,30),(30,50),(50,80),(80,120),(120,150),(150,999)]
    dist_counts = Counter()
    for lo, hi in bins:
        n = sum(1 for c in clusters if lo <= c["metro_d"] < hi)
        dist_counts[f"{lo}–{hi}"] = n
        bar = "█" * min(n // 20, 40)
        print(f"  {lo:3d}–{hi:3d} km: {n:5d}  {bar}")


# ── MAIN ──────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--vwh-only", action="store_true")
    parser.add_argument("--pks-only", action="store_true")
    args = parser.parse_args()

    if not args.pks_only:
        analyze_vwh()
    if not args.vwh_only:
        analyze_pks()


if __name__ == "__main__":
    main()
