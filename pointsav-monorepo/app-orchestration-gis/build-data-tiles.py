#!/usr/bin/env python3
"""
build-data-tiles.py — Census and spend PMTile layers masked to catchment areas

Reads census-h3-res7.jsonl and spend-h3-res7.jsonl, keeps only H3 cells that
fall within 150km of at least one co-location cluster (using ClusterFilter),
and writes H3 hexagon polygon GeoJSONs for tippecanoe ingestion.

Outputs (GeoJSON → tippecanoe → PMTiles in gateway tiles dir):
  work/census-catchment.geojson   → layer4-census.pmtiles
  work/spend-catchment.geojson    → layer5-spend.pmtiles

RULE: Only H3 cells within a catchment area (≤150km of any cluster) are
included. The 150km data radius is the ingest boundary, NOT a display boundary.

HOME layer only (v1). AWAY (daytime population) deferred pending data source.
"""

import json
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from utils.spatial_filter import ClusterFilter

try:
    import h3 as h3lib
except ImportError:
    print("ERROR: h3 not installed.")
    sys.exit(1)

DEPLOY = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")
CENSUS_FILE = DEPLOY / "service-fs/service-census/census-h3-res7.jsonl"
SPEND_FILE  = DEPLOY / "service-fs/service-spend/cleansed-spend-h3-res7.jsonl"
CLUSTERS_META = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
TILES_DIR   = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles")
WORK_DIR    = Path(__file__).parent / "work"

CENSUS_GEOJSON = WORK_DIR / "census-catchment.geojson"
SPEND_GEOJSON  = WORK_DIR / "spend-catchment.geojson"
CENSUS_PMTILES = TILES_DIR / "layer4-census.pmtiles"
SPEND_PMTILES  = TILES_DIR / "layer5-spend.pmtiles"


def h3_hex_polygon(h3_idx: str) -> list:
    """Returns GeoJSON polygon ring from H3 cell boundary. h3 v4 returns (lat,lon)."""
    boundary = h3lib.cell_to_boundary(h3_idx)
    coords = [[round(lon, 5), round(lat, 5)] for lat, lon in boundary]
    coords.append(coords[0])
    return [coords]


def main():
    WORK_DIR.mkdir(parents=True, exist_ok=True)

    print("Initialising ClusterFilter (150km threshold) ...")
    cf = ClusterFilter(str(CLUSTERS_META), threshold_km=150.0)

    # --- Census layer ---
    print(f"\nBuilding census catchment GeoJSON from {CENSUS_FILE.name} ...")
    census_features = []
    kept = skipped = 0
    with open(CENSUS_FILE) as f:
        for line in f:
            r = json.loads(line)
            if r.get("pop", 0) <= 0:
                continue
            if not cf.is_active(r["lon"], r["lat"]):
                skipped += 1
                continue
            census_features.append({
                "type": "Feature",
                "geometry": {"type": "Polygon",
                             "coordinates": h3_hex_polygon(r["h3"])},
                "properties": {
                    "h3": r["h3"],
                    "pop": round(r["pop"]),
                    "iso": r.get("iso", [""])[0] if isinstance(r.get("iso"), list) else r.get("iso", ""),
                },
            })
            kept += 1
            if kept % 100000 == 0:
                print(f"  census: {kept:,} kept, {skipped:,} skipped ...")

    print(f"  Census: {kept:,} cells kept / {skipped:,} outside catchment skipped.")
    with open(CENSUS_GEOJSON, "w") as f:
        json.dump({"type": "FeatureCollection", "features": census_features},
                  f, separators=(",", ":"))
    del census_features

    # --- Spend layer ---
    print(f"\nBuilding spend catchment GeoJSON from {SPEND_FILE.name} ...")
    # Need lat/lon from census — build a quick lookup
    print("  Loading H3 lat/lon index ...")
    h3_coords = {}
    with open(CENSUS_FILE) as f:
        for line in f:
            r = json.loads(line)
            h3_coords[r["h3"]] = (r["lat"], r["lon"])

    spend_features = []
    kept = skipped = 0
    with open(SPEND_FILE) as f:
        for line in f:
            r = json.loads(line)
            h3_idx = r["h3"]
            coords = h3_coords.get(h3_idx)
            if coords is None:
                skipped += 1
                continue
            lat, lon = coords
            if not cf.is_active(lon, lat):
                skipped += 1
                continue
            total_spend = (r.get("spend_grocery", 0)
                           + r.get("spend_hardware", 0)
                           + r.get("spend_wholesale", 0))
            if total_spend <= 0:
                skipped += 1
                continue
            spend_features.append({
                "type": "Feature",
                "geometry": {"type": "Polygon",
                             "coordinates": h3_hex_polygon(h3_idx)},
                "properties": {
                    "h3": h3_idx,
                    "spend_total": round(total_spend),
                    "spend_grocery": round(r.get("spend_grocery", 0)),
                    "spend_hardware": round(r.get("spend_hardware", 0)),
                    "spend_wholesale": round(r.get("spend_wholesale", 0)),
                },
            })
            kept += 1
            if kept % 100000 == 0:
                print(f"  spend: {kept:,} kept ...")

    print(f"  Spend: {kept:,} cells kept / {skipped:,} skipped.")
    with open(SPEND_GEOJSON, "w") as f:
        json.dump({"type": "FeatureCollection", "features": spend_features},
                  f, separators=(",", ":"))
    del spend_features

    # --- Run tippecanoe ---
    print("\nBuilding PMTiles ...")

    def run_tippecanoe(input_geojson: Path, output_pmtiles: Path, layer_name: str):
        cmd = [
            "tippecanoe",
            "-o", str(output_pmtiles),
            "--force",
            "--layer", layer_name,
            "--minimum-zoom", "4",
            "--maximum-zoom", "12",
            "--drop-densest-as-needed",
            "--extend-zooms-if-still-dropping",
            str(input_geojson),
        ]
        print(f"  {' '.join(cmd[:6])} ... {input_geojson.name} → {output_pmtiles.name}")
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            print(f"  ERROR: {result.stderr[-500:]}")
        else:
            size_mb = output_pmtiles.stat().st_size / 1024 / 1024
            print(f"  Done: {output_pmtiles.name} ({size_mb:.1f} MB)")

    TILES_DIR.mkdir(parents=True, exist_ok=True)
    run_tippecanoe(CENSUS_GEOJSON, CENSUS_PMTILES, "census")
    run_tippecanoe(SPEND_GEOJSON,  SPEND_PMTILES,  "spend")

    print("\nlayer4-census.pmtiles and layer5-spend.pmtiles ready.")
    print("These layers display census/spend H3 hexes ONLY within catchment areas (≤150km).")


if __name__ == "__main__":
    main()
