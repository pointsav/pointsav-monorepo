#!/usr/bin/env python3
"""
build-vwh-clusters.py — Independent Urban Fringe (VWH) co-location clustering.

Builds VWH clusters directly from industrial chain JSONL data.
Does NOT reference clusters.geojson (retail co-location data).

Categories: hardware, mro_industrial, tool_rental, auto_parts, electrical,
            flooring, lumber, plumbing, paint, welding

T1/T2/T3 by category composition:
  T1: 3+ distinct categories  OR  hardware + 2+ non-hardware categories
  T2: 2 distinct categories
  T3: 1 category, 2+ store locations (same or different chains)

Excluded: singletons (single chain at single location — not a co-location)
          hypermarket within 1 km (retail park, not industrial fringe)
          outside 3–150 km metro distance

Usage:
    python3 build-vwh-clusters.py
    python3 build-vwh-clusters.py --output work/archetype-vwh.geojson
"""

import json
import math
import argparse
from pathlib import Path
from collections import defaultdict, Counter

SCRIPT_DIR = Path(__file__).parent
TOTEBOX    = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")
CHAIN_DIR  = TOTEBOX / "service-fs" / "service-business"
WORK       = SCRIPT_DIR / "work"

DISPLAY_ISO = {
    "US", "CA", "MX",
    "GB", "SE", "DK", "NO", "FI", "IS",
    "FR", "DE", "ES", "IT", "GR", "PL", "AT", "NL", "PT",
}

VWH_MIN_METRO_KM    = 3.0
VWH_MAX_METRO_KM    = 150.0
VWH_SPAN_MAX_KM     = 8.0    # max cluster diameter — wider than retail (3 km) to catch spread industrial parks
HYPER_DISQUALIFY_KM = 1.0    # hypermarket within this → retail park, exclude
MIN_RECORDS         = 2      # minimum store locations to qualify (same or different chains)

EPS_TIGHT_KM = 1.0
EPS_LOOSE_KM = 3.0

# ── CHAIN TAXONOMY ────────────────────────────────────────────────────────────

VWH_CHAINS: dict[str, list[str]] = {
    "hardware": [
        "home-depot-us", "home-depot-ca", "home-depot-mx",
        "lowes-us", "lowes-ca",
        "bq-uk", "wickes-uk",
        "leroy-merlin-fr", "leroy-merlin-es", "leroy-merlin-it",
        "leroy-merlin-gr", "leroy-merlin-pl", "leroy-merlin-pt",
        "castorama-fr", "castorama-pl",
        "obi-de", "obi-at", "obi-it", "obi-pl",
        "hornbach-de", "bauhaus-de", "bauhaus-at",
        "bauhaus-dk", "bauhaus-fi", "bauhaus-no",
        "hellweg-de", "biltema-se", "praxis-nl", "gamma-nl",
        "mr-bricolage-fr", "alaska-industrial-hardware-us",
    ],
    "mro_industrial": [
        "wurth-de", "fastenal-us", "grainger-us", "hilti-ch",
        "princess-auto-ca",
    ],
    "tool_rental": [
        "united-rentals-us", "sunbelt-rentals-us",
        "loxam-fr", "kiloutou-fr", "boels-rental-nl",
        "ramirent-fi", "cramo-fi",
        "hss-hire-uk", "speedy-hire-uk",
    ],
    "auto_parts": [
        "autozone-us", "oreilly-auto-us", "napa-us",
        "napa-ca", "advance-auto-us", "autozone-mx",
        "halfords-uk",
    ],
    "electrical": [
        "cef-uk", "rexel-fr", "ahlsell-se",
    ],
    "flooring": [
        "floor-decor-us", "topps-tiles-uk",
    ],
    "lumber": [
        "84-lumber-us", "builders-firstsource-us",
        "kent-building-supplies-ca",
    ],
    "plumbing": [
        "ferguson-us", "wolseley-uk",
    ],
    "paint": [
        "sherwin-williams-us",
    ],
    "welding": [
        "boc-uk",
    ],
}

HYPERMARKET_CHAINS = [
    "walmart-us", "walmart-ca", "walmart-mx", "target-us",
    "whole-foods-us", "kroger-us", "heb-us", "fred-meyer-us",
    "asda-uk", "morrisons-uk", "tesco-uk", "sainsburys-uk",
    "aldi-uk", "lidl-de", "lidl-fr", "lidl-es", "lidl-pt",
    "lidl-it", "lidl-pl", "lidl-at", "lidl-nl",
    "aldi-de", "aldi-fr", "aldi-nl", "aldi-at",
    "rewe-de", "edeka-de", "kaufland-de", "real-de", "ecenter-de", "globus-de",
    "carrefour-hypermarket-fr", "carrefour-hypermarket-es",
    "carrefour-hypermarket-it", "carrefour-hypermarket-pl",
    "geant-casino-fr", "intermarche-hyper-fr",
    "leclerc-fr", "leclerc-pl", "auchan-fr", "auchan-pl", "auchan-pt",
    "alcampo-es", "interspar-at", "billa-plus-at",
    "bilka-dk", "foetex-dk", "albert-heijn-xl-nl", "jumbo-nl",
    "k-citymarket-fi", "coop-forum-se", "continente-pt",
    "esselunga-it", "ipercoop-it", "famila-it", "sklavenitis-gr",
    "chedraui-mx", "costco-us", "costco-ca", "costco-mx", "sams-club-us",
    "hagkaup-is",
]

# ── METRO REFERENCE POINTS ────────────────────────────────────────────────────
# Abbreviated — full list in analyze-archetype-categories.py
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
    ("Oklahoma City",35.4676,-97.5164),("Tucson",32.2226,-110.9747),
    ("Fresno",36.7378,-119.7871),("Albuquerque",35.0844,-106.6504),
    ("Colorado Springs",38.8339,-104.8214),("Virginia Beach",36.8529,-75.9780),
    ("Long Beach",33.7701,-118.1937),("Wichita",37.6872,-97.3301),
    ("Bakersfield",35.3733,-119.0187),("Baton Rouge",30.4515,-91.1871),
    ("New Orleans",29.9511,-90.0715),("Fargo",46.8772,-96.7898),
    ("Vancouver",49.2827,-123.1207),("Toronto",43.6532,-79.3832),
    ("Montreal",45.5017,-73.5673),("Calgary",51.0447,-114.0719),
    ("Ottawa",45.4215,-75.6972),("Edmonton",53.5461,-113.4938),
    ("Winnipeg",49.8951,-97.1384),("Kitchener",43.4516,-80.4925),
    ("Mexico City",19.4326,-99.1332),("Guadalajara",20.6597,-103.3496),
    ("Monterrey",25.6866,-100.3161),("Puebla",19.0414,-98.2063),
    ("London",51.5074,-0.1278),("Birmingham",52.4862,-1.8904),
    ("Manchester",53.4808,-2.2426),("Glasgow",55.8642,-4.2518),
    ("Liverpool",53.4084,-2.9916),("Bristol",51.4545,-2.5879),
    ("Edinburgh",55.9533,-3.1883),("Leeds",53.8008,-1.5491),
    ("Sheffield",53.3811,-1.4701),("Cardiff",51.4816,-3.1791),
    ("Stockholm",59.3293,18.0686),("Gothenburg",57.7089,11.9746),
    ("Malmö",55.6050,13.0038),("Copenhagen",55.6761,12.5683),
    ("Aarhus",56.1629,10.2039),("Odense",55.4038,10.4024),
    ("Oslo",59.9139,10.7522),("Bergen",60.3913,5.3221),
    ("Helsinki",60.1699,24.9384),("Tampere",61.4978,23.7610),
    ("Reykjavik",64.1355,-21.8954),
    ("Paris",48.8566,2.3522),("Marseille",43.2965,5.3698),
    ("Lyon",45.7640,4.8357),("Toulouse",43.6047,1.4442),
    ("Nantes",47.2184,-1.5536),("Bordeaux",44.8378,-0.5792),
    ("Lille",50.6292,3.0573),("Strasbourg",48.5734,7.7521),
    ("Berlin",52.5200,13.4050),("Hamburg",53.5753,9.9950),
    ("Munich",48.1351,11.5820),("Cologne",50.9333,6.9500),
    ("Frankfurt",50.1109,8.6821),("Stuttgart",48.7758,9.1829),
    ("Düsseldorf",51.2217,6.7762),("Dortmund",51.5136,7.4653),
    ("Essen",51.4556,7.0116),("Leipzig",51.3397,12.3731),
    ("Dresden",51.0504,13.7373),("Hannover",52.3759,9.7320),
    ("Nuremberg",49.4521,11.0767),("Karlsruhe",49.0069,8.4037),
    ("Madrid",40.4168,-3.7038),("Barcelona",41.3851,2.1734),
    ("Valencia",39.4699,-0.3763),("Seville",37.3891,-5.9845),
    ("Zaragoza",41.6488,-0.8891),("Málaga",36.7213,-4.4214),
    ("Rome",41.9028,12.4964),("Milan",45.4654,9.1866),
    ("Naples",40.8518,14.2681),("Turin",45.0703,7.6869),
    ("Bologna",44.4949,11.3426),("Florence",43.7696,11.2558),
    ("Athens",37.9838,23.7275),("Thessaloniki",40.6401,22.9444),
    ("Warsaw",52.2297,21.0122),("Kraków",50.0647,19.9450),
    ("Łódź",51.7592,19.4560),("Wrocław",51.1079,17.0385),
    ("Poznań",52.4064,16.9252),("Gdańsk",54.3520,18.6466),
    ("Szczecin",53.4289,14.5530),("Katowice",50.2649,19.0238),
    ("Vienna",48.2082,16.3738),("Graz",47.0707,15.4395),
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


def two_pass_dbscan(recs, eps_tight=EPS_TIGHT_KM, eps_loose=EPS_LOOSE_KM):
    n = len(recs)
    if n == 0:
        return []
    gs = 0.05
    grid = defaultdict(list)
    for i, r in enumerate(recs):
        cell = (int(r["lat"] / gs), int(r["lon"] / gs))
        grid[cell].append(i)

    def edges_within(eps):
        deg = (eps + 0.3) / 111.0
        result = set()
        for i, r in enumerate(recs):
            clat, clon = r["lat"], r["lon"]
            for la in range(int((clat - deg) / gs), int((clat + deg) / gs) + 1):
                for lo in range(int((clon - deg) / gs), int((clon + deg) / gs) + 1):
                    for j in grid.get((la, lo), []):
                        if j <= i: continue
                        if abs(recs[j]["lat"] - clat) > deg: continue
                        if haversine(clat, clon, recs[j]["lat"], recs[j]["lon"]) <= eps:
                            result.add((i, j))
        return result

    tight_comps = union_find(n, edges_within(eps_tight))
    atom_of = {}
    for comp in tight_comps:
        for idx in comp:
            atom_of[idx] = comp[0]

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
        for orig_idx, ai in atom_of.items():
            if ai in set(member_atoms):
                member_indices.append(orig_idx)
        clusters.append(member_indices)
    return clusters


def load_jsonl(path):
    recs = []
    if not path.exists():
        return recs
    with open(path) as f:
        for line in f:
            try:
                r = json.loads(line)
                lat = r.get("latitude")
                lon = r.get("longitude")
                if lat is None or lon is None:
                    continue
                iso = (r.get("iso_country_code") or "")[:2].upper()
                if iso not in DISPLAY_ISO:
                    continue
                recs.append({
                    "lat": float(lat), "lon": float(lon), "iso": iso,
                    "name": r.get("location_name") or "",
                    "city": r.get("city") or "",
                })
            except (json.JSONDecodeError, ValueError):
                continue
    return recs


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


_STRONG_CATS = frozenset({
    "mro_industrial", "tool_rental", "electrical",
    "plumbing", "lumber", "flooring", "welding",
})


def tier_vwh(cats: frozenset) -> int:
    """
    Strong enrichment: mro_industrial, tool_rental, electrical,
                       plumbing, lumber, flooring, welding
    Weak signal:       auto_parts, paint  (common in US suburban strips)

    T1: hardware + 1+ strong  (contractor supply zone)
        OR 2+ strong without hardware  (pure industrial estate)
        OR 3+ categories with at least 1 strong
    T2: 2+ categories, none strong  (hardware + auto_parts/paint combos)
        OR single strong + single weak
    T3: single category  (homogeneous cluster, 2+ distinct chains required)
    """
    strong = cats & _STRONG_CATS
    has_hw = "hardware" in cats

    if has_hw and strong:
        return 1
    if len(strong) >= 2:
        return 1
    if len(cats) >= 3 and strong:
        return 1
    if len(cats) >= 2:
        return 2
    return 3


def build(output_path: Path):
    # ── Load all VWH chain records ────────────────────────────────────────────
    all_recs: list[dict] = []
    chain_counts: dict[str, int] = {}
    for cat, chain_ids in VWH_CHAINS.items():
        for cid in chain_ids:
            recs = load_jsonl(CHAIN_DIR / f"{cid}.jsonl")
            for r in recs:
                r["category"] = cat
                r["chain_id"] = cid
            all_recs.extend(recs)
            if recs:
                chain_counts[cid] = len(recs)

    print(f"VWH chain records: {len(all_recs):,} across {len(chain_counts)} chains")

    # ── Load hypermarket records for disqualification ─────────────────────────
    hyper_recs: list[dict] = []
    for cid in HYPERMARKET_CHAINS:
        hyper_recs.extend(load_jsonl(CHAIN_DIR / f"{cid}.jsonl"))
    hyper_grid = build_grid(hyper_recs)
    print(f"Hypermarket disqualifier records: {len(hyper_recs):,}")

    # ── Two-pass DBSCAN ───────────────────────────────────────────────────────
    print(f"Running two-pass DBSCAN (tight={EPS_TIGHT_KM} km, loose={EPS_LOOSE_KM} km)...")
    comp_list = two_pass_dbscan(all_recs)
    print(f"  Raw clusters: {len(comp_list):,}")

    # ── Filter and tier ───────────────────────────────────────────────────────
    features = []
    n_skipped_single = n_skipped_metro = n_skipped_hyper = n_skipped_span = 0

    for comp in comp_list:
        chains_in = set(all_recs[i]["chain_id"] for i in comp)

        # Must have 2+ store locations (same or different chains) — single-location singletons excluded
        if len(comp) < MIN_RECORDS:
            n_skipped_single += 1
            continue

        lats = [all_recs[i]["lat"] for i in comp]
        lons = [all_recs[i]["lon"] for i in comp]
        clat = sum(lats) / len(lats)
        clon = sum(lons) / len(lons)

        iso_counter = Counter(all_recs[i]["iso"] for i in comp)
        iso = iso_counter.most_common(1)[0][0]

        # Span
        span = 0.0
        for i in range(len(comp)):
            for j in range(i + 1, len(comp)):
                d = haversine(all_recs[comp[i]]["lat"], all_recs[comp[i]]["lon"],
                              all_recs[comp[j]]["lat"], all_recs[comp[j]]["lon"])
                if d > span:
                    span = d

        if span > VWH_SPAN_MAX_KM:
            n_skipped_span += 1
            continue

        # Metro distance
        metro_d, metro_name = nearest_metro(clat, clon)
        if not (VWH_MIN_METRO_KM <= metro_d <= VWH_MAX_METRO_KM):
            n_skipped_metro += 1
            continue

        # Hypermarket disqualifier
        if any_within(clat, clon, hyper_recs, hyper_grid, HYPER_DISQUALIFY_KM):
            n_skipped_hyper += 1
            continue

        cats = frozenset(all_recs[i]["category"] for i in comp)
        hardware_chains = [all_recs[i]["chain_id"] for i in comp
                           if all_recs[i]["category"] == "hardware"]
        enrichment_chains = [all_recs[i]["chain_id"] for i in comp
                             if all_recs[i]["category"] != "hardware"]

        t = tier_vwh(cats)
        cid_str = f"vwh-{iso.lower()}-{round(clat, 4)}-{round(clon, 4)}"

        features.append({
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [round(clon, 6), round(clat, 6)]},
            "properties": {
                "id":               cid_str,
                "lat":              round(clat, 6),
                "lon":              round(clon, 6),
                "iso":              iso,
                "vwh_tier":         t,
                "span_km":          round(span, 2),
                "metro_dist_km":    round(metro_d, 1),
                "nearest_metro":    metro_name,
                "member_count":     len(comp),
                "chain_count":      len(chains_in),
                "hardware_chains":  sorted(set(hardware_chains)),
                "enrichment_chains":sorted(set(enrichment_chains)),
                "vwh_signal":       sorted(cats),
                "vwh_strength":     len(cats),
                "archetype":        "urban_fringe",
            },
        })

    # ── Summary ───────────────────────────────────────────────────────────────
    n_t1 = sum(1 for f in features if f["properties"]["vwh_tier"] == 1)
    n_t2 = sum(1 for f in features if f["properties"]["vwh_tier"] == 2)
    n_t3 = sum(1 for f in features if f["properties"]["vwh_tier"] == 3)

    print(f"  Skipped — single chain:     {n_skipped_single:,}")
    print(f"  Skipped — span > {VWH_SPAN_MAX_KM} km:    {n_skipped_span:,}")
    print(f"  Skipped — metro distance:   {n_skipped_metro:,}")
    print(f"  Skipped — hypermarket:      {n_skipped_hyper:,}")
    print(f"  Valid VWH clusters: {len(features):,}  (T1={n_t1} T2={n_t2} T3={n_t3})")

    iso_counts = Counter(f["properties"]["iso"] for f in features)
    print("\n  ISO   Count")
    for iso, n in sorted(iso_counts.items(), key=lambda x: -x[1]):
        print(f"  {iso}     {n}")

    # ── Write GeoJSON ─────────────────────────────────────────────────────────
    output_path.parent.mkdir(parents=True, exist_ok=True)
    geojson = {"type": "FeatureCollection", "features": features}
    with open(output_path, "w") as f:
        json.dump(geojson, f, separators=(",", ":"))
    print(f"\nWrote {output_path}  ({len(features):,} features: T1={n_t1} T2={n_t2} T3={n_t3})")
    return features


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", default=str(WORK / "archetype-vwh.geojson"))
    args = parser.parse_args()
    build(Path(args.output))


if __name__ == "__main__":
    main()
