#!/usr/bin/env python3
"""
build-catchment-centroids.py — Per-cluster H3 centroid JSON files for heatmap layer

For each co-location cluster, writes a GeoJSON FeatureCollection of H3 res-7
cell centroids (Point features) within the primary catchment zone (≤35 km).
These are lazy-loaded client-side to drive the MapLibre heatmap layer without
ever rendering raw hexagon polygons on the map.

Outputs one file per cluster:
  gateway-orchestration-gis-1/www/data/catchment-cells/<cluster_id>.json

Each file is a GeoJSON FeatureCollection of Point features with properties:
  h3, pop, spend_total, spend_grocery, spend_hardware, spend_wholesale

Primary zone only (35 km / k=17 H3 rings) — secondary cells are visually
represented by the catchment-outer-fill polygon layer, not the heatmap.

Typical output: ~754 points per cluster, ~44 KB per file, ~300 MB total.
Run time: ~5–10 minutes for 6,815 clusters depending on disk I/O.
"""

import json
import math
import sys
from pathlib import Path
from collections import defaultdict

try:
    import h3 as h3lib
except ImportError:
    print("ERROR: h3 not installed. Run: pip install h3")
    sys.exit(1)

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------
DEPLOY_DATA = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")
CENSUS_FILE  = DEPLOY_DATA / "service-fs/service-census/census-h3-res7.jsonl"
SPEND_FILE   = DEPLOY_DATA / "service-fs/service-spend/cleansed-spend-h3-res7.jsonl"
CLUSTERS_META = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
OUTPUT_DIR   = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/catchment-cells")

PRIMARY_KM    = 35.0
PRIMARY_RINGS = 17     # H3 res-7: k=17 ≈ 36 km — matches synthesize-od-study.py
H3_RESOLUTION = 7

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def haversine_km(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = math.sin(dlat/2)**2 + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2)) * math.sin(dlon/2)**2
    return 2 * R * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def load_census(path):
    """Load census JSONL → dict: h3 → {lat, lon, pop}"""
    print(f"Loading census data from {path.name} ...")
    data = {}
    skipped = 0
    with open(path) as f:
        for line in f:
            r = json.loads(line)
            if r.get("pop", 0) <= 0:
                skipped += 1
                continue
            data[r["h3"]] = {"lat": r["lat"], "lon": r["lon"], "pop": r["pop"]}
    print(f"  Census: {len(data):,} cells loaded ({skipped:,} zero-pop skipped).")
    return data


def load_spend(path, census):
    """Load spend JSONL → dict: h3 → {spend_total, spend_grocery, spend_hardware, spend_wholesale}"""
    print(f"Loading spend data from {path.name} ...")
    data = {}
    if not path.exists():
        print("  Spend file not found — spend fields will be 0.")
        return data
    with open(path) as f:
        for line in f:
            r = json.loads(line)
            h = r["h3"]
            if h not in census:
                continue
            total = (r.get("spend_grocery", 0) + r.get("spend_hardware", 0)
                     + r.get("spend_wholesale", 0))
            if total <= 0:
                continue
            data[h] = {
                "spend_total":     round(total),
                "spend_grocery":   round(r.get("spend_grocery", 0)),
                "spend_hardware":  round(r.get("spend_hardware", 0)),
                "spend_wholesale": round(r.get("spend_wholesale", 0)),
            }
    print(f"  Spend: {len(data):,} cells loaded.")
    return data


def load_clusters(path):
    print(f"Loading clusters from {path.name} ...")
    with open(path) as f:
        clusters = json.load(f)
    print(f"  {len(clusters):,} clusters loaded.")
    return clusters


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    if not CENSUS_FILE.exists():
        print(f"ERROR: census file not found: {CENSUS_FILE}")
        sys.exit(1)
    if not CLUSTERS_META.exists():
        print(f"ERROR: clusters-meta not found: {CLUSTERS_META}")
        sys.exit(1)

    census  = load_census(CENSUS_FILE)
    spend   = load_spend(SPEND_FILE, census)
    clusters = load_clusters(CLUSTERS_META)

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    written  = 0
    empty    = 0
    skipped  = 0

    print(f"\nBuilding centroid files for {len(clusters):,} clusters → {OUTPUT_DIR}")
    print(f"  Primary zone: {PRIMARY_KM} km  (H3 res-7 k={PRIMARY_RINGS})")

    for i, c in enumerate(clusters):
        cid  = c["id"]
        lat  = c["lat"]
        lon  = c["lon"]

        out_path = OUTPUT_DIR / f"{cid}.json"

        # Skip if already written (re-run safety)
        if out_path.exists() and "--force" not in sys.argv:
            skipped += 1
            if i % 500 == 0:
                print(f"  [{i:,}/{len(clusters):,}] Skipping existing files... ({skipped} so far)")
            continue

        # Get H3 cell at cluster centre
        center_h3 = h3lib.latlng_to_cell(lat, lon, H3_RESOLUTION)

        # Cells within primary radius (grid_disk returns a set — h3 v4 API)
        primary_cells = h3lib.grid_disk(center_h3, PRIMARY_RINGS)

        features = []
        for h in primary_cells:
            if h not in census:
                continue
            cell = census[h]
            # Double-check with haversine to avoid ring over-approximation at edges
            if haversine_km(lat, lon, cell["lat"], cell["lon"]) > PRIMARY_KM + 2.5:
                continue
            props = {
                "h3":   h,
                "pop":  round(cell["pop"]),
            }
            sp = spend.get(h)
            if sp:
                props["spend_total"]     = sp["spend_total"]
                props["spend_grocery"]   = sp["spend_grocery"]
                props["spend_hardware"]  = sp["spend_hardware"]
                props["spend_wholesale"] = sp["spend_wholesale"]
            else:
                props["spend_total"] = 0
            features.append({
                "type": "Feature",
                "geometry": {
                    "type": "Point",
                    "coordinates": [round(cell["lon"], 5), round(cell["lat"], 5)],
                },
                "properties": props,
            })

        if not features:
            empty += 1
        else:
            fc = {"type": "FeatureCollection", "features": features}
            with open(out_path, "w") as f:
                json.dump(fc, f, separators=(",", ":"))
            written += 1

        if (i + 1) % 500 == 0 or (i + 1) == len(clusters):
            print(f"  [{i+1:,}/{len(clusters):,}]  written={written:,}  empty={empty}  skipped={skipped}")

    print(f"\nDone. {written:,} centroid files written to {OUTPUT_DIR}")
    if empty:
        print(f"  {empty} clusters had no census coverage within {PRIMARY_KM} km (sparse/ocean areas).")
    if skipped:
        print(f"  {skipped} files skipped (already existed). Use --force to overwrite.")


if __name__ == "__main__":
    main()
