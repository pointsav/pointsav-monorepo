#!/usr/bin/env python3
"""
synthesize-od-study.py — B1/B2/B3: O-D catchment computation

For each co-location cluster, computes primary (≤35km) and secondary (35–150km)
catchment zones using crow-flies distance over H3 res-7 cells. Aggregates
census population and spend (grocery/hardware/wholesale) within each zone.
Ranks clusters by combined primary+secondary totals (no weights).

Writes:
  service-fs/service-mobility/od-summary.jsonl   — B3 artifact; one record per cluster
  work/catchment-data.json                        — full stats + ranks for merge

Distance model: crow-flies (haversine). No drive-time, no gravity weighting.
Primary radius 35km is provisional — adjustable via PRIMARY_KM below.

H3 ring approximations (res-7, ~2.11km center-to-center):
  k=17 → ~36km  (covers 35km primary)
  k=72 → ~152km (covers 150km secondary bound)
"""

import json
import math
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

# ---------------------------------------------------------------------------
# Parameters — adjust PRIMARY_KM to tune primary catchment radius
# ---------------------------------------------------------------------------
PRIMARY_KM   = 35.0   # crow-flies; provisional
SECONDARY_KM = 150.0  # fixed — matches data radius
H3_RESOLUTION = 7

# H3 res-7: center-to-center ≈ 2.11km → ring counts
PRIMARY_RINGS   = 17   # covers ~35km
SECONDARY_RINGS = 72   # covers ~150km

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------
DEPLOY = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")
CENSUS_FILE    = DEPLOY / "service-fs/service-census/census-h3-res7.jsonl"
SPEND_FILE     = DEPLOY / "service-fs/service-spend/cleansed-spend-h3-res7.jsonl"
OD_SUMMARY_OUT = DEPLOY / "service-fs/service-mobility/od-summary.jsonl"
CLUSTERS_META  = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
CATCHMENT_OUT  = Path(__file__).parent / "work/catchment-data.json"

try:
    import h3 as h3lib
except ImportError:
    print("ERROR: h3 library not installed. Run: pip install h3")
    sys.exit(1)


def haversine_km(lat1: float, lon1: float, lat2: float, lon2: float) -> float:
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2
         + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2))
         * math.sin(dlon / 2) ** 2)
    return 2.0 * R * math.asin(math.sqrt(max(0.0, min(1.0, a))))


def load_census(path: Path) -> dict:
    """Returns {h3_idx: (lat, lon, pop)} for all cells with pop > 0."""
    print(f"Loading census H3 data from {path.name} ...")
    cells = {}
    with open(path) as f:
        for line in f:
            r = json.loads(line)
            if r.get("pop", 0) > 0:
                cells[r["h3"]] = (r["lat"], r["lon"], r["pop"])
    print(f"  Loaded {len(cells):,} populated cells.")
    return cells


def load_spend(path: Path) -> dict:
    """Returns {h3_idx: (gro, hw, whs)} spend totals."""
    print(f"Loading spend H3 data from {path.name} ...")
    spend = {}
    with open(path) as f:
        for line in f:
            r = json.loads(line)
            spend[r["h3"]] = (
                r.get("spend_grocery", 0.0),
                r.get("spend_hardware", 0.0),
                r.get("spend_wholesale", 0.0),
            )
    print(f"  Loaded {len(spend):,} spend cells.")
    return spend


def main():
    # Load data
    census = load_census(CENSUS_FILE)
    spend  = load_spend(SPEND_FILE)

    print(f"Loading cluster centroids from {CLUSTERS_META.name} ...")
    with open(CLUSTERS_META) as f:
        clusters = json.load(f)
    print(f"  Loaded {len(clusters):,} clusters.")

    OD_SUMMARY_OUT.parent.mkdir(parents=True, exist_ok=True)
    CATCHMENT_OUT.parent.mkdir(parents=True, exist_ok=True)

    catchment = {}   # cluster_id → stats dict
    total = len(clusters)

    print(f"\nComputing O-D catchment zones (primary ≤{PRIMARY_KM}km, secondary ≤{SECONDARY_KM}km) ...")
    print(f"H3 resolution {H3_RESOLUTION}: primary_rings={PRIMARY_RINGS}, secondary_rings={SECONDARY_RINGS}")

    with open(OD_SUMMARY_OUT, "w") as od_out:
        for idx, c in enumerate(clusters):
            cid  = c["id"]
            clat = c["lat"]
            clon = c["lon"]

            if idx % 500 == 0:
                print(f"  [{idx}/{total}] {cid}")

            # Get cluster's H3 center cell
            c_hex = h3lib.latlng_to_cell(clat, clon, H3_RESOLUTION)

            # Compute primary and full disks (h3 v4 returns list — convert to set)
            primary_set   = set(h3lib.grid_disk(c_hex, PRIMARY_RINGS))
            full_set      = set(h3lib.grid_disk(c_hex, SECONDARY_RINGS))
            secondary_set = full_set - primary_set

            # Aggregate primary zone
            p_pop = p_gro = p_hw = p_whs = 0.0
            p_cells = 0
            for h in primary_set:
                if h in census:
                    _, _, pop = census[h]
                    p_pop += pop
                    p_cells += 1
                if h in spend:
                    gro, hw, whs = spend[h]
                    p_gro += gro
                    p_hw  += hw
                    p_whs += whs

            # Aggregate secondary zone
            s_pop = s_gro = s_hw = s_whs = 0.0
            s_cells = 0
            for h in secondary_set:
                if h in census:
                    _, _, pop = census[h]
                    s_pop += pop
                    s_cells += 1
                if h in spend:
                    gro, hw, whs = spend[h]
                    s_gro += gro
                    s_hw  += hw
                    s_whs += whs

            catchment[cid] = {
                "pp": round(p_pop),
                "sp": round(s_pop),
                "pg": round(p_gro),
                "sg": round(s_gro),
                "ph": round(p_hw),
                "sh": round(s_hw),
                "pw": round(p_whs),
                "sw": round(s_whs),
                "pc": p_cells,
                "sc_": s_cells,
            }

            # Write compact B3 summary record
            od_out.write(json.dumps({
                "cluster_id": cid,
                "lat": clat,
                "lon": clon,
                "primary_cells":  p_cells,
                "secondary_cells": s_cells,
                "primary_pop":    round(p_pop),
                "secondary_pop":  round(s_pop),
                "primary_grocery": round(p_gro),
                "secondary_grocery": round(s_gro),
            }) + "\n")

    print(f"\nO-D computation complete. {len(catchment):,} clusters processed.")

    # Rank by combined (primary + secondary) totals — no weights
    print("Computing catchment rankings (combined primary+secondary) ...")

    ranked_pop = sorted(catchment.keys(),
                        key=lambda cid: catchment[cid]["pp"] + catchment[cid]["sp"],
                        reverse=True)
    ranked_gro = sorted(catchment.keys(),
                        key=lambda cid: catchment[cid]["pg"] + catchment[cid]["sg"],
                        reverse=True)
    ranked_hw  = sorted(catchment.keys(),
                        key=lambda cid: catchment[cid]["ph"] + catchment[cid]["sh"],
                        reverse=True)
    ranked_whs = sorted(catchment.keys(),
                        key=lambda cid: catchment[cid]["pw"] + catchment[cid]["sw"],
                        reverse=True)

    n = len(catchment)
    for rank, cid in enumerate(ranked_pop, 1):
        catchment[cid]["rp"] = rank
    for rank, cid in enumerate(ranked_gro, 1):
        catchment[cid]["rg"] = rank
    for rank, cid in enumerate(ranked_hw, 1):
        catchment[cid]["rh"] = rank
    for rank, cid in enumerate(ranked_whs, 1):
        catchment[cid]["rw"] = rank
    for cid in catchment:
        catchment[cid]["rn"] = n

    # Write catchment-data.json
    print(f"Writing {CATCHMENT_OUT} ...")
    with open(CATCHMENT_OUT, "w") as f:
        json.dump(catchment, f)

    # Merge into clusters-meta.json
    print(f"Merging catchment data into {CLUSTERS_META} ...")
    with open(CLUSTERS_META) as f:
        meta = json.load(f)

    merged = 0
    for entry in meta:
        cid = entry["id"]
        if cid in catchment:
            entry.update(catchment[cid])
            merged += 1

    with open(CLUSTERS_META, "w") as f:
        json.dump(meta, f, separators=(",", ":"))

    print(f"  Merged {merged:,} / {len(meta):,} cluster entries.")

    # Quick sanity check
    top5_pop = sorted(catchment.items(),
                      key=lambda kv: kv[1]["pp"] + kv[1]["sp"],
                      reverse=True)[:5]
    print("\nTop 5 clusters by combined catchment population:")
    for cid, stats in top5_pop:
        total_pop = stats["pp"] + stats["sp"]
        print(f"  [{stats['rp']:>5}] {cid:<55} pop={total_pop:>12,.0f}")

    print(f"\nB3 summary written to: {OD_SUMMARY_OUT}")
    print(f"Catchment data written to: {CATCHMENT_OUT}")
    print("clusters-meta.json updated.")
    print("\nB1 (census), B2 (spend), B3 (O-D) — DONE.")


if __name__ == "__main__":
    main()
