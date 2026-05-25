#!/usr/bin/env python3
"""
build-demand-ranking.py — Stage 2: demand rank within tier.

Assigns demand_rank_in_tier to each co-location using an interim demand
proxy (catchment population at 35/150 km).  Target demand: US LODES O-D
primary catchments + ES MITMA.

Interim source: catchment-data.json produced by synthesize-od-study.py.
Clusters are matched by spatial proximity (nearest old cluster within 2 km)
since cluster IDs changed from anchor-pin to centroid-based.

demand_basis values:
    "od-us"           matched to US LODES O-D data
    "od-es"           matched to ES MITMA O-D data
    "catchment-35-150" matched to interim catchment population (no O-D)
    "interim-none"    no catchment data available for this cluster

demand_rank_in_tier:
    Inverted percentile of demand proxy within (tier) pool, continent-wide.
    0.0 = lowest demand; 1.0 = highest demand.

Reads:  work/clusters.geojson
        work/catchment-data.json   (optional; produced by synthesize-od-study.py)
Writes: work/clusters.geojson     (in-place; updates demand_rank_in_tier + demand_basis)
"""
import json
import math
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR

INPUT_FILE   = WORK_DIR / "clusters.geojson"
CATCHMENT_IN = WORK_DIR / "catchment-data.json"
OUTPUT_FILE  = WORK_DIR / "clusters.geojson"

SPATIAL_MATCH_KM = 2.0   # max distance to old cluster for spatial join


def haversine_km(lat1, lon1, lat2, lon2) -> float:
    R = 6371.0
    ph1, ph2 = math.radians(lat1), math.radians(lat2)
    a = (math.sin(math.radians(lat2 - lat1) / 2) ** 2
         + math.cos(ph1) * math.cos(ph2)
         * math.sin(math.radians(lon2 - lon1) / 2) ** 2)
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def _inverted_pctile_map(items: list[tuple]) -> dict:
    """items = [(feat_idx, demand_value), ...] sorted ascending.
    Returns {feat_idx: inverted_pctile} where 1.0 = highest demand."""
    indexed = sorted(items, key=lambda x: x[1])
    m = len(indexed)
    result = {}
    for pos, (idx, _) in enumerate(indexed):
        frac = pos / (m - 1) if m > 1 else 0.0
        result[idx] = round(frac, 4)   # 0=lowest demand, 1=highest demand
    return result


def main():
    print(f"Loading {INPUT_FILE} ...")
    with open(INPUT_FILE) as f:
        geojson = json.load(f)
    features = geojson.get("features", [])
    print(f"  {len(features):,} clusters")

    # Load catchment-data.json for spatial matching
    demand_by_cid: dict = {}
    if CATCHMENT_IN.exists():
        print(f"Loading {CATCHMENT_IN.name} ...")
        with open(CATCHMENT_IN) as f:
            raw = json.load(f)
        # Build spatial index of old clusters: {cluster_id: {lat, lon, pp, od_basis}}
        for cid, entry in raw.items():
            lat = float(entry.get("lat") or entry.get("_lat") or 0)
            lon = float(entry.get("lon") or entry.get("_lon") or 0)
            pp  = float(entry.get("pp") or entry.get("population_proxy") or 0)
            basis = entry.get("demand_basis") or "catchment-35-150"
            demand_by_cid[cid] = {"lat": lat, "lon": lon, "pp": pp, "basis": basis}
        print(f"  {len(demand_by_cid):,} entries in catchment-data.json")
    else:
        print(f"  WARNING: {CATCHMENT_IN} not found. demand_rank_in_tier will be 0.5 for all.")

    # Spatial join: match each new cluster to nearest old cluster
    demand_entries = list(demand_by_cid.values()) if demand_by_cid else []
    matched = 0
    unmatched = 0

    for i, feat in enumerate(features):
        p = feat["properties"]
        clon, clat = feat["geometry"]["coordinates"]

        best_pp, best_basis = 0.0, "interim-none"
        if demand_entries:
            best_d = 9999.0
            for entry in demand_entries:
                if not (entry["lat"] and entry["lon"]):
                    continue
                d = haversine_km(clat, clon, entry["lat"], entry["lon"])
                if d < best_d:
                    best_d = d
                    best_pp = entry["pp"]
                    best_basis = entry["basis"]
            if best_d <= SPATIAL_MATCH_KM:
                matched += 1
            else:
                best_pp, best_basis = 0.0, "interim-none"
                unmatched += 1
        else:
            unmatched += 1

        p["_pp_raw"] = best_pp
        p["demand_basis"] = best_basis

    if demand_by_cid:
        print(f"  Spatial match: {matched} matched, {unmatched} unmatched within {SPATIAL_MATCH_KM} km")

    # Compute demand_rank_in_tier: inverted percentile within tier (continent-wide)
    by_tier: dict = defaultdict(list)
    for i, feat in enumerate(features):
        p = feat["properties"]
        tier = int(p.get("tier") or 0)
        by_tier[tier].append((i, p.get("_pp_raw") or 0.0))

    demand_rank: dict[int, float] = {}
    for tier, items in by_tier.items():
        pctiles = _inverted_pctile_map(items)
        demand_rank.update(pctiles)

    for i, feat in enumerate(features):
        p = feat["properties"]
        p["demand_rank_in_tier"] = demand_rank.get(i, 0.5)
        p.pop("_pp_raw", None)

    counts = {b: 0 for b in ("od-us", "od-es", "catchment-35-150", "interim-none")}
    for feat in features:
        b = feat["properties"].get("demand_basis") or "interim-none"
        counts[b] = counts.get(b, 0) + 1
    print("  demand_basis distribution:")
    for b, n in counts.items():
        if n:
            print(f"    {b}: {n}")

    print(f"Writing {OUTPUT_FILE} ...")
    with open(OUTPUT_FILE, "w") as f:
        json.dump(geojson, f)
    print("  Done.")


if __name__ == "__main__":
    main()
