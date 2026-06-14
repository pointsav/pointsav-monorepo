#!/usr/bin/env python3
"""
build-pks-clusters.py — PKS Commuter co-location clustering.

Builds PKS clusters from transit node + commercial enrichment JSONL data.
Does NOT reference clusters.geojson (retail co-location data) — independent model.

Question: "Where do people leave their car to board transit or fly?"

Transit POIs (first-class co-location signal):
  airport        — commercial passenger airports
  intercity_rail — mainline / intercity rail stations
  commuter_rail  — suburban commuter rail (GO Transit, NJ Transit, RER, S-Bahn, etc.)
  metro_subway   — metro/subway/light-rail outer stations
  intercity_bus  — long-distance/intercity bus terminals (ADO, ALSA, FlixBus, National Express, etc.)

Commercial enrichment (co-location signal):
  car_rental     — confirms traveller-facing transit zone
  park_ride      — designated park-and-ride lots

  NOTE: self_storage removed (VWH logistics signal, not drive-to-transit evidence).
  parcel_depot removed (bridges transit lines through suburbs, creates false mega-clusters).

Co-location method: two-pass DBSCAN (1 km tight / 2.5 km loose) on all PKS POIs.
Ring = arithmetic mean of all member lat/lons (centroid). No metro-distance gate —
the category selection drives the location naturally.

Mode groups (ICR+CR collapse): intercity_rail and commuter_rail are the same
physical platform with two service levels — counted as one RAIL group, not two modes.
Groups: AIR (airport), RAIL (intercity_rail+commuter_rail), URBAN (metro_subway), BUS (intercity_bus).

Qualification gate: airport OR ≥2 mode groups OR (any mode group + any enrichment).
Pure single-mode transit nodes with no parking/rental evidence are walk-up urban
stops — not park-and-transit hubs; excluded.

T1/T2/T3 by mode-group composition and enrichment class:
  T1: (AIR + RENTAL), OR ≥3 groups, OR (≥2 groups + ≥2 enrich), OR (AIR + ≥1 enrich)
  T2: AIR alone, OR (≥2 groups + ≥1 enrich), OR (1 group + ≥2 enrich)
  T3: ≥1 group + ≥1 enrich (not qualifying above)

Usage:
    python3 build-pks-clusters.py
    python3 build-pks-clusters.py --output work/archetype-pks.geojson
"""

import json
import math
import re
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

EPS_TIGHT_KM    = 1.0
EPS_LOOSE_KM    = 2.5    # 3.0→2.5 prevents chaining through city-wide metro networks
PKS_SPAN_MAX_KM = 8.0    # max cluster diameter

TRANSIT_MODES = frozenset({"airport", "intercity_rail", "commuter_rail", "metro_subway", "intercity_bus"})

CAR_RENTAL_CHAINS = [
    # NA
    "enterprise-us", "hertz-us", "avis-us", "budget-us", "alamo-us", "national-us",
    "enterprise-ca", "hertz-mx", "avis-mx", "budget-mx", "enterprise-mx",
    # MX additional (Phase 5, 2026-06-14)
    "europcar-mx", "alamo-mx", "national-mx", "thrifty-mx", "sixt-mx", "payless-mx", "dollar-mx",
    # EU
    "sixt-eu", "hertz-eu", "avis-eu", "budget-eu", "europcar-eu", "enterprise-eu",
    # legacy single-country (kept for de-dup robustness; build tolerates missing JSONLs)
    "europcar-fr", "sixt-de",
]

HOTEL_CHAINS = [
    # EU
    "ibis-eu", "b-and-b-hotels-eu", "premier-inn-gb", "travelodge-gb", "motel-one-de",
    # NA
    "holiday-inn-express-us", "hampton-us", "courtyard-us",
    # MX (Phase 4, 2026-06-14)
    "city-express-mx", "fiesta-inn-mx", "one-hotels-mx",
]

CONVENIENCE_CHAINS = [
    # MX — OXXO is the dominant traveller-facing convenience signal at MX transit nodes (Phase 5, 2026-06-14)
    "oxxo-mx",
]

# self_storage removed: it is a VWH (urban-fringe logistics) signal, not drive-to-transit evidence.
# parcel_depot removed: retail UPS/FedEx/DHL locations bridge transit lines through suburbs.

# Metro refs — enrichment output only; NOT used as a filter gate.
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

    # Metro-subway atoms that contain no non-metro element don't participate in
    # loose-phase bridging with other metro-only atoms.  This prevents city-wide
    # metro/LRT networks (Canada Line, etc.) from chaining into giant clusters that
    # exceed PKS_SPAN_MAX_KM and get filtered.  A metro-only atom CAN still connect
    # to an airport or commuter-rail atom in the loose phase.
    atom_is_metro_only = {
        comp[0]: all(recs[i].get("tight_only", False) for i in comp)
        for comp in tight_comps
    }

    loose_edges = set()
    for a, b in edges_within(eps_loose):
        ra, rb = atom_of[a], atom_of[b]
        if ra != rb:
            if atom_is_metro_only.get(ra) and atom_is_metro_only.get(rb):
                continue  # metro-only ↔ metro-only loose bridge blocked
            loose_edges.add((min(ra, rb), max(ra, rb)))

    atom_ids = sorted(set(atom_of.values()))
    atom_idx = {v: i for i, v in enumerate(atom_ids)}
    loose_comps = union_find(len(atom_ids), {(atom_idx[a], atom_idx[b]) for a, b in loose_edges})

    clusters = []
    for comp in loose_comps:
        member_atoms = [atom_ids[i] for i in comp]
        member_set = set(member_atoms)
        member_indices = [orig_idx for orig_idx, ai in atom_of.items() if ai in member_set]
        clusters.append(member_indices)
    return clusters


_AIRSTRIP_RE   = re.compile(r"airstrip|airfield|ultralight|glid|heliport|seaplane|balloon|model ", re.I)
_COMMERCIAL_RE = re.compile(r"international|regional|municipal|airport", re.I)


def is_significant_airport(rec: dict) -> bool:
    """Commercial/regional airport — not a GA strip, heliport, or ultralight field."""
    name = rec.get("location_name") or rec.get("name") or ""
    if _AIRSTRIP_RE.search(name):
        return False
    atype = (rec.get("aerodrome_type") or "")
    if "international" in atype or "regional" in atype:
        return True
    if (rec.get("iata_code") or rec.get("iata")) and _COMMERCIAL_RE.search(name):
        return True
    return False


def _coord_override_iso(lat, lon, stored_iso=None, iata_code=None):
    """Return the correct ISO-2 country code from coordinates for North American transit data.
    Overrides erroneous iso_country_code values caused by bbox-based OSM ingest contamination.
    Returns None when no override is needed (trust the stored iso_country_code).

    For airports: pass iata_code to enable the CA Y-prefix heuristic. Genuine Canadian
    airports below 49°N almost universally use IATA codes starting with 'Y' (Transport
    Canada convention). US airports near the border (DTW, GTF, MVW, DSM, ROC, BUF, etc.)
    do not. This disambiguates cases like Windsor YQG vs Detroit DTW that share the same
    latitude and cannot be separated by coordinate bounds alone."""
    if lat is None or lon is None:
        return None
    lat, lon = float(lat), float(lon)
    if 18.9 <= lat <= 28.5 and -178.5 <= lon <= -154.8:   # Hawaii → US
        return "US"
    if lat >= 54.0 and -168.5 <= lon <= -130.0:            # Alaska → US
        return "US"
    if lon < -114.0 and 22.0 <= lat <= 32.7:               # Baja California peninsula → MX
        return "MX"
    # MX interior: stepped lat thresholds approximate the US-Mexico border east of Baja.
    # Conservative to avoid US border cities: El Paso (31.8°N), Laredo (27.5°N), McAllen (26.2°N).
    # Known gaps (border cities, intentionally excluded): Cd. Juárez (31.7°N), Nuevo Laredo (27.5°N).
    if -117.0 <= lon < -104.5 and 14.5 <= lat < 30.5:     # Sonora / Chihuahua interior → MX
        return "MX"
    if -104.5 <= lon < -99.0 and 14.5 <= lat < 26.5:      # Coahuila / Nuevo León → MX
        return "MX"
    if -99.0 <= lon <= -86.5 and 14.5 <= lat < 25.5:      # Tamaulipas / Gulf coast → MX
        return "MX"
    # Airport-specific CA/US confusion: CA bbox overlaps the entire CONUS northeast/midwest.
    # Airports ingested as CA but physically below 49°N (the US-Canada border) that have
    # non-Y IATA codes are US airports. Known exceptions: ZBF (Bathurst NB, 47.6°N).
    # Only fires when iata_code is supplied (transit records without IATA retained as-is).
    if stored_iso == "CA" and lat < 49.0 and iata_code:
        iata = iata_code.upper()
        _CA_NON_Y = {"ZBF"}  # Bathurst NB — genuine CA airport with non-Y code below 49°N
        if not iata.startswith("Y") and iata not in _CA_NON_Y:
            return "US"
    return None


# Tram/streetcar operators with no park-and-ride or park-and-fly relevance.
# Records from these operators are skipped in load_transit_jsonl to avoid
# inflating PKS cluster counts for walkable urban streetcar networks.
_TRAM_EXCLUDE_OPERATORS = frozenset({
    "MATA",          # Memphis — suspended since 2020
    "RTA New Orleans", "New Orleans RTA",  # streetcar, no airport
    "KC Streetcar",
    "Atlanta Streetcar",
    "DC Streetcar", "DDOT",
    "TECO Line Streetcar", "HARTline",
})


def load_transit_jsonl(path, category, transit_class_map=None):
    """Load transit-place JSONL (airports, railway) into the PKS POI pool."""
    recs = []
    if not path.exists():
        print(f"  WARN: {path.name} not found")
        return recs
    with open(path) as f:
        for line in f:
            try:
                r = json.loads(line)
                lat = r.get("latitude") or r.get("lat")
                lon = r.get("longitude") or r.get("lon")
                if lat is None or lon is None: continue
                iso = (r.get("iso_country_code") or r.get("country_code") or "")[:2].upper()
                # Override with coordinate-derived ISO for North American records where
                # the JSONL field is contaminated (e.g. CTA L labeled CA, Monterrey labeled US).
                coord_iso = _coord_override_iso(lat, lon, stored_iso=iso, iata_code=r.get("iata_code"))
                if coord_iso is not None:
                    iso = coord_iso
                if iso not in DISPLAY_ISO: continue
                # Skip suspended or urban-only tram operators that add noise to PKS.
                operator = r.get("operator") or r.get("network") or ""
                if operator in _TRAM_EXCLUDE_OPERATORS: continue
                tc = (r.get("transit_class") or "").lower()
                if tc == "tram": continue  # catch remaining street-running trams by class
                cat = category
                if transit_class_map:
                    cat = transit_class_map.get(tc, category)
                if cat == "airport" and not is_significant_airport(r):
                    continue
                recs.append({
                    "lat":        float(lat),
                    "lon":        float(lon),
                    "iso":        iso,
                    "category":   cat,
                    "name":       r.get("location_name") or r.get("name") or "",
                    "tight_only": (cat == "metro_subway"),
                })
            except (json.JSONDecodeError, ValueError):
                continue
    return recs


def load_chain_jsonl(path, category):
    """Load chain-store JSONL (car rental, self-storage, parcel depot) into PKS POI pool."""
    recs = []
    if not path.exists():
        print(f"  WARN: {path.name} not found — chain listed but JSONL missing (silent no-op)")
        return recs
    with open(path) as f:
        for line in f:
            try:
                r = json.loads(line)
                lat = r.get("latitude")
                lon = r.get("longitude")
                if lat is None or lon is None: continue
                iso = (r.get("iso_country_code") or "")[:2].upper()
                coord_iso = _coord_override_iso(lat, lon, stored_iso=iso)
                if coord_iso is not None:
                    iso = coord_iso
                if iso not in DISPLAY_ISO: continue
                recs.append({
                    "lat":      float(lat),
                    "lon":      float(lon),
                    "iso":      iso,
                    "category": category,
                    "name":     r.get("location_name") or "",
                })
            except (json.JSONDecodeError, ValueError):
                continue
    return recs


def load_park_ride(path):
    """Load park-and-ride lots from civic-parking JSONL."""
    recs = []
    if not path.exists():
        print(f"  WARN: {path.name} not found")
        return recs
    with open(path) as f:
        for line in f:
            try:
                r = json.loads(line)
                if r.get("parking_class") != "park_ride":
                    continue
                lat = r.get("latitude") or r.get("lat")
                lon = r.get("longitude") or r.get("lon")
                if lat is None or lon is None: continue
                iso = (r.get("iso_country_code") or r.get("country_code") or "")[:2].upper()
                coord_iso = _coord_override_iso(lat, lon, stored_iso=iso)
                if coord_iso is not None:
                    iso = coord_iso
                if iso not in DISPLAY_ISO: continue
                recs.append({
                    "lat":      float(lat),
                    "lon":      float(lon),
                    "iso":      iso,
                    "category": "park_ride",
                    "name":     r.get("location_name") or r.get("name") or "",
                })
            except (json.JSONDecodeError, ValueError):
                continue
    return recs


def _mode_groups(cats: frozenset) -> frozenset:
    """Collapse transit categories into mode groups.

    ICR + CR at the same station are one RAIL group (same physical platform,
    two service levels — not genuinely bimodal). Counting them separately
    inflates multi-modal scores and distorts tier assignment.
    """
    g = set()
    if "airport" in cats:                                              g.add("AIR")
    if "intercity_rail" in cats or "commuter_rail" in cats:           g.add("RAIL")
    if "metro_subway" in cats:                                         g.add("URBAN")
    if "intercity_bus" in cats:                                        g.add("BUS")
    return frozenset(g)


def _enrich_classes(cats: frozenset) -> frozenset:
    """Enrichment evidence classes (distinct from transit modes)."""
    ec = set()
    if "car_rental" in cats:   ec.add("RENTAL")
    if "park_ride" in cats:    ec.add("PARK")
    if "hotel" in cats:        ec.add("HOTEL")
    if "convenience" in cats:  ec.add("CONVENIENCE")
    return frozenset(ec)


def qualify_pks(cats: frozenset) -> bool:
    """Admit as PKS if: airport, OR ≥2 mode groups, OR any mode group + any enrichment.

    Pure single-mode transit nodes with no parking/rental evidence are walk-up urban
    stops — not park-and-transit hubs; excluded to keep the dataset focused.
    """
    mg = _mode_groups(cats)
    if not mg:
        return False
    ec = _enrich_classes(cats)
    return "AIR" in mg or len(mg) >= 2 or len(ec) >= 1


def tier_pks(cats: frozenset) -> int:
    """Mode-group and enrichment tier.

    T1: (AIR + RENTAL), OR ≥3 groups, OR (≥2 groups + ≥2 enrich), OR (AIR + ≥1 enrich)
    T2: AIR alone, OR (≥2 groups + ≥1 enrich), OR (1 group + ≥2 enrich)
    T3: ≥1 group + ≥1 enrich (not qualifying above)
    """
    mg = _mode_groups(cats)
    ec = _enrich_classes(cats)
    n_g = len(mg); n_e = len(ec)
    if ("AIR" in mg and ("RENTAL" in ec or "HOTEL" in ec)) or n_g >= 3 or (n_g >= 2 and n_e >= 2) or ("AIR" in mg and n_e >= 1):
        return 1
    if "AIR" in mg or (n_g >= 2 and n_e >= 1) or (n_g == 1 and n_e >= 2):
        return 2
    return 3  # n_g>=1 and n_e>=1, not above


def build(output_path: Path):
    all_recs: list[dict] = []

    # ── Transit nodes ─────────────────────────────────────────────────────────
    airports = load_transit_jsonl(PLACES_DIR / "cleansed-civic-airports.jsonl", "airport")
    print(f"  airports:       {len(airports):,}")
    all_recs.extend(airports)

    intercity = load_transit_jsonl(PLACES_DIR / "cleansed-civic-railway.jsonl", "intercity_rail")
    print(f"  intercity rail: {len(intercity):,}")
    all_recs.extend(intercity)

    commuter = load_transit_jsonl(
        PLACES_DIR / "cleansed-civic-railway-commuter.jsonl",
        "commuter_rail",
        transit_class_map={"subway": "metro_subway", "light_rail": "metro_subway", "metro": "metro_subway"},
    )
    print(f"  commuter/metro: {len(commuter):,}")
    all_recs.extend(commuter)

    bus_terminals = load_transit_jsonl(
        PLACES_DIR / "cleansed-civic-bus-terminal.jsonl",
        "intercity_bus",
    )
    print(f"  intercity bus:  {len(bus_terminals):,}")
    all_recs.extend(bus_terminals)

    # ── Commercial enrichment ─────────────────────────────────────────────────
    n_car = 0
    for cid in CAR_RENTAL_CHAINS:
        recs = load_chain_jsonl(CHAIN_DIR / f"{cid}.jsonl", "car_rental")
        all_recs.extend(recs); n_car += len(recs)
    print(f"  car rental:     {n_car:,}")

    park_ride = load_park_ride(PLACES_DIR / "cleansed-civic-parking.jsonl")
    print(f"  park-and-ride:  {len(park_ride):,}")
    all_recs.extend(park_ride)

    n_hotel = 0
    for cid in HOTEL_CHAINS:
        recs = load_chain_jsonl(CHAIN_DIR / f"{cid}.jsonl", "hotel")
        all_recs.extend(recs); n_hotel += len(recs)
    print(f"  hotels:         {n_hotel:,}")

    n_conv = 0
    for cid in CONVENIENCE_CHAINS:
        recs = load_chain_jsonl(CHAIN_DIR / f"{cid}.jsonl", "convenience")
        all_recs.extend(recs); n_conv += len(recs)
    if n_conv:
        print(f"  convenience:    {n_conv:,}")

    # self_storage removed: VWH logistics signal, not park-and-transit evidence.
    # parcel_depot removed: retail courier locations bridge transit lines, creating false mega-clusters.

    print(f"PKS POI total: {len(all_recs):,}")

    # ── Two-pass DBSCAN ───────────────────────────────────────────────────────
    print(f"Running two-pass DBSCAN (tight={EPS_TIGHT_KM} km, loose={EPS_LOOSE_KM} km)...")
    comp_list = two_pass_dbscan(all_recs)
    print(f"  Raw clusters: {len(comp_list):,}")

    # ── Filter and tier ───────────────────────────────────────────────────────
    features = []
    n_skipped_no_transit = n_skipped_span = n_skipped_no_enrich = 0

    for comp in comp_list:
        cats = frozenset(all_recs[i]["category"] for i in comp)

        if not (cats & TRANSIT_MODES):
            n_skipped_no_transit += 1
            continue

        if not qualify_pks(cats):
            n_skipped_no_enrich += 1
            continue

        lats = [all_recs[i]["lat"] for i in comp]
        lons = [all_recs[i]["lon"] for i in comp]
        clat = sum(lats) / len(lats)
        clon = sum(lons) / len(lons)

        iso_counter = Counter(all_recs[i]["iso"] for i in comp)
        iso = iso_counter.most_common(1)[0][0]

        span = 0.0
        for ii in range(len(comp)):
            for jj in range(ii + 1, len(comp)):
                d = haversine(all_recs[comp[ii]]["lat"], all_recs[comp[ii]]["lon"],
                              all_recs[comp[jj]]["lat"], all_recs[comp[jj]]["lon"])
                if d > span:
                    span = d

        if span > PKS_SPAN_MAX_KM:
            n_skipped_span += 1
            continue

        metro_d, metro_name = nearest_metro(clat, clon)
        t = tier_pks(cats)

        transit_cats    = sorted(cats & TRANSIT_MODES)
        enrichment_cats = sorted(cats - TRANSIT_MODES)
        mode_grps       = _mode_groups(cats)

        # Airports anchor PKS clusters: use airport name + airport centroid when present.
        airport_members = [i for i in comp if all_recs[i]["category"] == "airport"]
        if airport_members:
            # Centroid anchored to airport(s) — prevents metro stations from pulling
            # the representative point away from the airport.
            clat = sum(all_recs[i]["lat"] for i in airport_members) / len(airport_members)
            clon = sum(all_recs[i]["lon"] for i in airport_members) / len(airport_members)
            airport_names = sorted(set(all_recs[i]["name"] for i in airport_members if all_recs[i].get("name")))
            display_name = airport_names[0] if airport_names else ""
        else:
            transit_names = sorted(set(
                all_recs[i]["name"] for i in comp
                if all_recs[i]["category"] in TRANSIT_MODES and all_recs[i].get("name")
            ))
            display_name = transit_names[0] if transit_names else ""

        features.append({
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [round(clon, 6), round(clat, 6)]},
            "properties": {
                "id":                f"pks-{iso.lower()}-{round(clat, 4)}-{round(clon, 4)}",
                "name":              display_name,
                "lat":               round(clat, 6),
                "lon":               round(clon, 6),
                "iso":               iso,
                "commuter_tier":     t,
                "span_km":           round(span, 2),
                "metro_dist_km":     round(metro_d, 1),
                "nearest_metro":     metro_name,
                "node_count":        len(comp),
                "transit_categories": transit_cats,
                "multi_modal":       len(mode_grps) >= 2,
                "car_rental":        "car_rental" in cats,
                "hotel":             "hotel" in cats,
                "pks_signal":        sorted(cats),
                "archetype":         "commuter",
            },
        })

    n_t1 = sum(1 for f in features if f["properties"]["commuter_tier"] == 1)
    n_t2 = sum(1 for f in features if f["properties"]["commuter_tier"] == 2)
    n_t3 = sum(1 for f in features if f["properties"]["commuter_tier"] == 3)

    print(f"  Skipped — no transit mode (not PKS): {n_skipped_no_transit:,}")
    print(f"  Skipped — transit only, no enrichment (walk-up stop): {n_skipped_no_enrich:,}")
    print(f"  Skipped — span > {PKS_SPAN_MAX_KM} km: {n_skipped_span:,}")
    print(f"  Valid PKS co-locations: {len(features):,}  (T1={n_t1} T2={n_t2} T3={n_t3})")

    iso_counts = Counter(f["properties"]["iso"] for f in features)
    print("\n  ISO   Count")
    for iso, n in sorted(iso_counts.items(), key=lambda x: -x[1]):
        print(f"  {iso}     {n}")

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
