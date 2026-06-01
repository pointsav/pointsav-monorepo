#!/usr/bin/env python3
"""
build-clusters.py — Two-pass tight-first DBSCAN co-location engine.

Replaces the anchor-centric ring scan with the DBSCAN proximity graph
validated in simulate-dbscan-ab.py.  Reads per-chain JSONL files declared
in taxonomy.py.  Emits the §2 schema from BRIEF-BUILD-SPEC-2026-05-22.

Key changes vs the old code:
  - Tier = retailer-category COMPOSITION ONLY (no IoU, no demand, no civic gate)
  - Geometry = centroid (never an anchor pin)
  - cluster_id = centroid-derived: co_{iso}_{clat5}_{clon5}
  - Two-pass DBSCAN: TAU_TIGHT=1.0 km freezes nuclei; TAU_LOOSE=3.0 km expands
  - Civic stores (hospital/university) added to members[] but never gate tier
  - span_km = max pairwise diameter of retail members
  - tight_intact flag: True iff all retail members are within 1 km of each other

Reads:   service-fs/service-business/<chain_id>.jsonl  (per taxonomy.py BRAND_FILL)
         service-places/cleansed-civic-osm.jsonl        (hospitals + universities)
Writes:  work/clusters.geojson
"""

import json
import math
import re
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import (
    TOTEBOX_DATA_PATH, BOUNDARIES_DIR, WORK_DIR,
    CHAIN_FAMILIES, CHAIN_SUB_LABELS,
)
from taxonomy import (
    BRAND_FILL, ALL_DISPLAY_ISO, ISO_TO_CONTINENT,
    DISPLAY_NAMES, category_of, tier_of, ring_radius_km, all_chains_for_iso,
    slots_for,
)
from utils.region_engine import RegionEngine

WORK_DIR.mkdir(parents=True, exist_ok=True)

CHAIN_DIR        = TOTEBOX_DATA_PATH / "service-fs" / "service-business"
SERVICE_PLACES   = TOTEBOX_DATA_PATH / "service-places" / "cleansed-civic-osm.jsonl"

TAU_TIGHT_KM = 1.0
TAU_LOOSE_KM = 3.0
CIVIC_RADIUS_KM = 5.0   # civic stores added to members if within this radius of centroid

# VWH/PKS enrichment categories — attached to clusters post-clustering like civic.
# They NEVER affect tier (tier_of uses only the 6 retail cats) or cluster geometry.
ENRICH_CATS = [
    "auto_parts", "paint", "mro_industrial", "flooring", "tool_rental",
    "lumber", "plumbing", "electrical", "welding", "car_rental",
]
ENRICH_RADIUS_KM = 5.0   # enrichment stores added to members if within this radius


# ── HAVERSINE ─────────────────────────────────────────────────────────────────

def haversine_km(lat1, lon1, lat2, lon2) -> float:
    R = 6371.0
    ph1, ph2 = math.radians(lat1), math.radians(lat2)
    a = (math.sin(math.radians(lat2 - lat1) / 2) ** 2
         + math.cos(ph1) * math.cos(ph2)
         * math.sin(math.radians(lon2 - lon1) / 2) ** 2)
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


# ── SPATIAL GRID ──────────────────────────────────────────────────────────────

def build_grid(recs: list, size: float = 0.1) -> dict:
    g: dict = {}
    for idx, r in enumerate(recs):
        cell = (int(float(r["latitude"]) / size), int(float(r["longitude"]) / size))
        g.setdefault(cell, []).append(idx)
    return g


def neighbours_within(lat: float, lon: float, recs: list, grid: dict,
                       r_km: float, size: float = 0.1) -> list[tuple[int, float]]:
    deg = (r_km + 0.5) / 111.0
    result = []
    for la in range(int((lat - deg) / size), int((lat + deg) / size) + 1):
        for lo in range(int((lon - deg) / size), int((lon + deg) / size) + 1):
            for idx in grid.get((la, lo), []):
                d = haversine_km(lat, lon, float(recs[idx]["latitude"]),
                                 float(recs[idx]["longitude"]))
                if d <= r_km:
                    result.append((idx, d))
    return result


# ── UNION-FIND ────────────────────────────────────────────────────────────────

def union_find(n: int, edges: set) -> list[list[int]]:
    parent = list(range(n))
    rank = [0] * n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(a, b):
        ra, rb = find(a), find(b)
        if ra == rb:
            return
        if rank[ra] < rank[rb]:
            ra, rb = rb, ra
        parent[rb] = ra
        if rank[ra] == rank[rb]:
            rank[ra] += 1

    for a, b in edges:
        union(a, b)
    comps: dict = defaultdict(list)
    for i in range(n):
        comps[find(i)].append(i)
    return list(comps.values())


# ── CLUSTER GEOMETRY ──────────────────────────────────────────────────────────

def component_diameter(indices: list[int], recs: list) -> float:
    max_d = 0.0
    for i in range(len(indices)):
        for j in range(i + 1, len(indices)):
            a, b = recs[indices[i]], recs[indices[j]]
            d = haversine_km(float(a["latitude"]), float(a["longitude"]),
                             float(b["latitude"]), float(b["longitude"]))
            if d > max_d:
                max_d = d
    return max_d


def split_greedy_tight(indices: list[int], recs: list, max_d: float,
                       atom_of: dict) -> list[list[int]]:
    """Greedy clique partition that never splits tight nuclei (Pass-1 atoms).

    Pathological case: dense city centres (e.g. London) produce a single tight
    atom spanning the whole city because 1-km chains connect every store.  When
    an atom's own diameter exceeds max_d it cannot form a valid cluster anyway,
    so we dissolve it into individual stores before the greedy pass runs.
    """
    raw_atoms: dict = {}
    for idx in indices:
        raw_atoms.setdefault(atom_of[idx], []).append(idx)

    # Dissolve any atom that already exceeds max_d — treat each store as its own atom.
    atoms: dict = {}
    next_id = max(raw_atoms.keys(), default=-1) + 1
    for aid, idxs in raw_atoms.items():
        if len(idxs) > 1 and component_diameter(idxs, recs) > max_d:
            for idx in idxs:
                atoms[next_id] = [idx]
                next_id += 1
        else:
            atoms[aid] = idxs

    remaining = sorted(atoms.keys(), key=lambda a: (-len(atoms[a]), a))
    groups = []
    while remaining:
        seed = remaining.pop(0)
        group = list(atoms[seed])
        still = []
        for cand in remaining:
            fits = all(
                haversine_km(float(recs[c]["latitude"]), float(recs[c]["longitude"]),
                             float(recs[g]["latitude"]), float(recs[g]["longitude"])) <= max_d
                for c in atoms[cand] for g in group
            )
            if fits:
                group.extend(atoms[cand])
            else:
                still.append(cand)
        remaining = still
        groups.append(group)
    return groups


def tight_intact(indices: list[int], recs: list) -> bool:
    for i in range(len(indices)):
        for j in range(i + 1, len(indices)):
            a, b = recs[indices[i]], recs[indices[j]]
            if haversine_km(float(a["latitude"]), float(a["longitude"]),
                            float(b["latitude"]), float(b["longitude"])) > TAU_TIGHT_KM:
                return False
    return True


# ── DATA LOADING ──────────────────────────────────────────────────────────────

def load_chain_jsonl(chain_id: str) -> list:
    path = CHAIN_DIR / f"{chain_id}.jsonl"
    if not path.exists():
        return []
    recs = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                r = json.loads(line)
                r["chain_id"] = chain_id
                recs.append(r)
            except Exception:
                pass
    return recs


def load_civic_jsonl() -> list:
    recs = []
    if not SERVICE_PLACES.exists():
        print(f"  WARNING: {SERVICE_PLACES} not found — civic data skipped")
        return recs
    with open(SERVICE_PLACES) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                recs.append(json.loads(line))
            except Exception:
                pass
    return recs


# ── CLUSTER ID ────────────────────────────────────────────────────────────────

def make_cluster_id(iso: str, clat: float, clon: float) -> str:
    lat5 = f"{abs(clat):.5f}".replace(".", "")
    lon5 = f"{abs(clon):.5f}".replace(".", "")
    lat_sign = "n" if clat >= 0 else "s"
    lon_sign = "e" if clon >= 0 else "w"
    return f"co_{iso.lower()}_{lat_sign}{lat5}_{lon_sign}{lon5}"


# ── TIER DESCRIPTOR ───────────────────────────────────────────────────────────

def tier_descriptor(cats: set[str]) -> str:
    retail = cats & {"hypermarket", "hardware", "price_club", "lifestyle", "sport", "electronics"}
    parts = []
    for k, label in [("hypermarket", "Hypermarket"), ("lifestyle", "Lifestyle"),
                     ("hardware", "Hardware"), ("price_club", "Price Club"),
                     ("sport", "Sport"), ("electronics", "Electronics")]:
        if k in retail:
            parts.append(label)
    return " + ".join(parts) if parts else "Unknown"


# ── TWO-PASS DBSCAN PER ISO ───────────────────────────────────────────────────

def run_dbscan_for_iso(iso: str, recs: list) -> list[dict]:
    """Run two-pass tight-first DBSCAN on recs, all from one ISO country.
    Returns list of raw cluster dicts (no civic, no geocoding yet)."""
    n = len(recs)
    if n == 0:
        return []

    grid = build_grid(recs)

    def graph_edges(radius):
        e = set()
        for i, r in enumerate(recs):
            lat, lon = float(r["latitude"]), float(r["longitude"])
            for j, _ in neighbours_within(lat, lon, recs, grid, radius):
                if j != i:
                    e.add((min(i, j), max(i, j)))
        return e

    # Pass 1 — freeze tight (≤1 km) nuclei
    tight_comps = union_find(n, graph_edges(TAU_TIGHT_KM))
    atom_of = {}
    for atom_id, comp in enumerate(tight_comps):
        for idx in comp:
            atom_of[idx] = atom_id

    # Pass 2 — group at loose (≤3 km); atoms never split
    loose_comps = union_find(n, graph_edges(TAU_LOOSE_KM))

    clusters = []
    for comp in loose_comps:
        if component_diameter(comp, recs) > TAU_LOOSE_KM:
            groups = split_greedy_tight(comp, recs, TAU_LOOSE_KM, atom_of)
        else:
            groups = [comp]

        for group in groups:
            members = [recs[i] for i in group]
            # Deduplicate same store appearing via multiple chain queries
            seen_locs: set = set()
            unique_members = []
            for m in members:
                key = (round(float(m["latitude"]), 4), round(float(m["longitude"]), 4))
                if key not in seen_locs:
                    seen_locs.add(key)
                    unique_members.append(m)
            members = unique_members
            group_indices = list(range(len(members)))  # re-index after dedup

            cats = {category_of(m["chain_id"]) for m in members
                    if category_of(m["chain_id"]) is not None}
            # Compute tight_intact and span before tier_of (both feed the tier rule)
            ti = all(
                haversine_km(float(members[i]["latitude"]), float(members[i]["longitude"]),
                             float(members[j]["latitude"]), float(members[j]["longitude"])) <= TAU_TIGHT_KM
                for i in range(len(members)) for j in range(i + 1, len(members))
            )
            span = round(component_diameter(list(range(len(members))), members), 3)
            tier = tier_of(cats, ti, span)
            if tier is None:
                continue

            lats = [float(m["latitude"])  for m in members]
            lons = [float(m["longitude"]) for m in members]
            clat = sum(lats) / len(lats)
            clon = sum(lons) / len(lons)

            clusters.append({
                "iso":        iso,
                "clat":       clat,
                "clon":       clon,
                "tier":       tier,
                "tier_descriptor": tier_descriptor(cats),
                "span_km":    span,
                "tight_intact": ti,
                "ring_radius_km": 1.0 if (tier == 1 and ti) else 3.0,
                "members":    members,
                "cats":       cats,
            })

    return clusters


# ── CIVIC ENRICHMENT ──────────────────────────────────────────────────────────

def enrich_with_civic(clusters: list[dict], civic_recs: list) -> None:
    """Add nearby civic stores (hospital/university) to each cluster's members.
    Civic never affects tier — descriptor / info only."""
    if not civic_recs:
        return
    civic_grid = build_grid(civic_recs)
    for c in clusters:
        clat, clon = c["clat"], c["clon"]
        nearby = neighbours_within(clat, clon, civic_recs, civic_grid, CIVIC_RADIUS_KM)
        for idx, d in nearby:
            r = civic_recs[idx]
            cat = r.get("category_id") or r.get("category", "")
            if cat not in ("hospital", "university"):
                continue
            c["members"].append({
                "chain_id":    cat,
                "latitude":    r["latitude"],
                "longitude":   r["longitude"],
                "location_name": r.get("location_name") or cat.title(),
                "_civic":      True,
                "_dist_km":    round(d, 2),
            })


# ── VWH/PKS ENRICHMENT ────────────────────────────────────────────────────────

def load_enrich_recs(iso: str) -> list:
    """Load all VWH/PKS enrichment-category chain records for an ISO.
    These attach to clusters as members but never gate tier or geometry."""
    recs = []
    for cat in ENRICH_CATS:
        for cid in slots_for(iso, cat):
            for r in load_chain_jsonl(cid):
                r["iso_country_code"] = iso
                recs.append(r)
    return recs


def enrich_with_vwh(clusters: list[dict], enrich_recs: list) -> None:
    """Attach nearby VWH/PKS enrichment stores (auto_parts, mro_industrial,
    car_rental, etc.) to each cluster's members. Marked _enrich; never tier."""
    if not enrich_recs:
        return
    grid = build_grid(enrich_recs)
    for c in clusters:
        clat, clon = c["clat"], c["clon"]
        for idx, d in neighbours_within(clat, clon, enrich_recs, grid, ENRICH_RADIUS_KM):
            r = enrich_recs[idx]
            c["members"].append({
                "chain_id":    r["chain_id"],
                "latitude":    r["latitude"],
                "longitude":   r["longitude"],
                "location_name": r.get("location_name") or r["chain_id"],
                "city":        r.get("city"),
                "_enrich":     True,
                "_dist_km":    round(d, 2),
            })


# ── GEOCODING + SCHEMA ASSEMBLY ───────────────────────────────────────────────

def assemble_feature(c: dict, engine: RegionEngine) -> dict:
    iso   = c["iso"]
    clat  = round(c["clat"], 5)
    clon  = round(c["clon"], 5)

    city, mkt_conf = engine.resolve_market(clat, clon, iso)
    region_name    = engine.resolve(clat, clon, iso) or ""

    cluster_id = make_cluster_id(iso, clat, clon)

    if city:
        iso_str = c.get("iso") or iso
        state_strs = {m.get("region") or "" for m in c["members"] if not m.get("_civic")}
        state_str = next((s for s in state_strs if s), "")
        market_name = f"{city}, {state_str}" if state_str else city
    else:
        market_name = region_name or iso

    regional_market = (
        "rm_" + iso.lower() + "_" +
        re.sub(r"[^a-z0-9]+", "_", (city or region_name or iso).lower()).strip("_")
    )

    retail_members = [m for m in c["members"] if not m.get("_civic") and not m.get("_enrich")]
    civic_members  = [m for m in c["members"] if m.get("_civic")]
    enrich_members = [m for m in c["members"] if m.get("_enrich")]

    members_out = []
    for m in retail_members:
        members_out.append({
            "chain_id":    m["chain_id"],
            "category":    category_of(m["chain_id"]) or "unknown",
            "name":        DISPLAY_NAMES.get(m["chain_id"], m["chain_id"]),
            "lat":         round(float(m["latitude"]), 5),
            "lon":         round(float(m["longitude"]), 5),
            "addr":        (m.get("street_address") or "").strip() or None,
            "city":        (m.get("city") or "").strip() or None,
        })
    for m in civic_members:
        members_out.append({
            "chain_id":    m["chain_id"],
            "category":    "medical" if m["chain_id"] == "hospital" else "education",
            "name":        m.get("location_name", m["chain_id"].title()),
            "lat":         round(float(m["latitude"]), 5),
            "lon":         round(float(m["longitude"]), 5),
            "dist_km":     m.get("_dist_km"),
            "addr":        None,
            "city":        (m.get("city") or "").strip() or None,
        })
    # VWH/PKS enrichment members (auto_parts, mro_industrial, car_rental, …)
    for m in enrich_members:
        members_out.append({
            "chain_id":    m["chain_id"],
            "category":    category_of(m["chain_id"]) or "unknown",
            "name":        DISPLAY_NAMES.get(m["chain_id"], m["chain_id"]),
            "lat":         round(float(m["latitude"]), 5),
            "lon":         round(float(m["longitude"]), 5),
            "dist_km":     m.get("_dist_km"),
            "_enrich":     True,
            "addr":        None,
            "city":        (m.get("city") or "").strip() or None,
        })

    props = {
        "cluster_id":        cluster_id,
        "tier":              c["tier"],
        "tier_descriptor":   c["tier_descriptor"],
        "span_km":           c["span_km"],
        "tight_intact":      c["tight_intact"],
        "ring_radius_km":    c["ring_radius_km"],
        "dist_rank_in_tier": 0.0,   # set by build-geometric-ranking.py
        "dist_pctile":       50,    # set by build-geometric-ranking.py
        "demand_rank_in_tier": 0.5, # set by build-demand-ranking.py
        "demand_basis":      "interim-none",
        "regional_market":   regional_market,
        "market_name":       market_name,
        "market_region":     region_name,
        "metro_market":      "",    # set by build-regional-markets.py if metro catalog present
        "mkt_conf":          mkt_conf,
        "iso":               iso,
        "continent":         ISO_TO_CONTINENT.get(iso, "?"),
        "members":           json.dumps(members_out),
        "member_count":      len(retail_members),
        "seed_lat":          round(float(retail_members[0]["latitude"]), 5) if retail_members else clat,
        "seed_lon":          round(float(retail_members[0]["longitude"]), 5) if retail_members else clon,
        "last_computed":     "2026-05-23",
    }

    return {
        "type": "Feature",
        "geometry": {"type": "Point", "coordinates": [clon, clat]},
        "properties": props,
    }


# ── MAIN ──────────────────────────────────────────────────────────────────────

def main():
    print("build-clusters.py — two-pass DBSCAN, composition-only tier (Phase 22)")
    print("Loading boundary engine...")
    engine = RegionEngine(BOUNDARIES_DIR)

    print("Loading civic data...")
    civic_recs = load_civic_jsonl()
    print(f"  {len(civic_recs)} civic records")

    all_features = []
    total_singletons = 0
    tier_counts = {1: 0, 2: 0, 3: 0}

    for iso in ALL_DISPLAY_ISO:
        chain_map = {}
        for cid in all_chains_for_iso(iso):
            recs = load_chain_jsonl(cid)
            for r in recs:
                # normalise iso_country_code to this ISO (handles legacy NORDICS codes)
                r["iso_country_code"] = iso
                chain_map.setdefault(cid, []).append(r)

        all_recs = [r for recs in chain_map.values() for r in recs]
        if not all_recs:
            continue

        print(f"  {iso}: {len(all_recs)} retail records from {len(chain_map)} chains")
        clusters = run_dbscan_for_iso(iso, all_recs)
        enrich_with_civic(clusters, civic_recs)
        enrich_recs = load_enrich_recs(iso)
        if enrich_recs:
            enrich_with_vwh(clusters, enrich_recs)

        for c in clusters:
            feat = assemble_feature(c, engine)
            all_features.append(feat)
            tier_counts[c["tier"]] = tier_counts.get(c["tier"], 0) + 1

    print(f"\n{len(all_features)} co-locations total")
    for t in sorted(tier_counts):
        print(f"  T{t}: {tier_counts[t]}")

    # Deduplicate clusters by cluster_id (centroid collision across ISO boundaries)
    seen_ids: set = set()
    deduped = []
    for f in all_features:
        cid = f["properties"]["cluster_id"]
        if cid not in seen_ids:
            seen_ids.add(cid)
            deduped.append(f)
    if len(deduped) < len(all_features):
        print(f"  Deduplicated {len(all_features) - len(deduped)} centroid collisions")
    all_features = deduped

    out = WORK_DIR / "clusters.geojson"
    with open(out, "w") as f:
        json.dump({"type": "FeatureCollection", "features": all_features}, f)
    print(f"\nWritten {len(all_features)} clusters → {out}")


if __name__ == "__main__":
    main()
