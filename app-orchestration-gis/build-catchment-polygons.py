#!/usr/bin/env python3
"""
build-catchment-polygons.py — Catchment ring polygons for PMTile layer3-catchment

Reads clusters-meta.json (which must already have catchment fields from
synthesize-od-study.py) and generates two circular polygon features per cluster:

  zone=primary   — ≤35km crow-flies fill (solid, low opacity)
  zone=secondary — 35–150km crow-flies stroke (lighter, annulus effect)

Both zones go into one GeoJSON → tippecanoe → layer3-catchment.pmtiles.

MapLibre style uses match["get","zone"] to colour primary vs secondary differently.
HOME/AWAY sub-toggle is wired in index.html; both use the same geometry for v1
(AWAY will differ once daytime population data is available).

Outputs:
  work/catchment-polygons.geojson
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

PRIMARY_KM   = 35.0
SECONDARY_KM = 150.0

CLUSTERS_META  = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
OUTPUT         = Path(__file__).parent / "work/catchment-polygons.geojson"


def circle_polygon(lon: float, lat: float, radius_km: float, n: int = 64) -> dict:
    coords = []
    for i in range(n):
        angle = 2 * math.pi * i / n
        dlat = (radius_km / 111.0) * math.sin(angle)
        dlon = (radius_km / (111.0 * math.cos(math.radians(lat)))) * math.cos(angle)
        coords.append([round(lon + dlon, 5), round(lat + dlat, 5)])
    coords.append(coords[0])
    return {"type": "Polygon", "coordinates": [coords]}


def main():
    print(f"Loading {CLUSTERS_META.name} ...")
    with open(CLUSTERS_META) as f:
        clusters = json.load(f)
    print(f"  {len(clusters):,} clusters loaded.")

    features = []
    for c in clusters:
        cid  = c["id"]
        lat  = c["lat"]
        lon  = c["lon"]

        # Properties shared by both rings
        base = {
            "cluster_id": cid,
            "display_name": c.get("dn", ""),
            "anchor": c.get("anc", ""),
            "score": c.get("sc", 0),
            "primary_pop":    c.get("pp", 0),
            "secondary_pop":  c.get("sp", 0),
            "primary_grocery": c.get("pg", 0),
            "primary_hardware": c.get("ph", 0),
            "catchment_rank_pop": c.get("rp", 0),
            "catchment_rank_of":  c.get("rn", 0),
        }

        # Primary ring (≤35km)
        features.append({
            "type": "Feature",
            "geometry": circle_polygon(lon, lat, PRIMARY_KM),
            "properties": {**base, "zone": "primary", "radius_km": PRIMARY_KM},
        })

        # Secondary ring (≤150km — full outer circle; CSS/style makes it look like annulus)
        features.append({
            "type": "Feature",
            "geometry": circle_polygon(lon, lat, SECONDARY_KM),
            "properties": {**base, "zone": "secondary", "radius_km": SECONDARY_KM},
        })

    fc = {"type": "FeatureCollection", "features": features}
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT, "w") as f:
        json.dump(fc, f, separators=(",", ":"))

    print(f"Written {len(features):,} polygon features ({len(clusters):,} clusters × 2 zones)")
    print(f"Output: {OUTPUT}")


if __name__ == "__main__":
    main()
