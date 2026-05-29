#!/usr/bin/env python3
"""
build-radius.py — catchment radius polygons for each co-location cluster

Reads work/clusters.geojson.
For each cluster, generates a circular buffer at catchment_radius_km.
Writes work/radius.geojson.

The radius polygon serves three purposes:
  1. Visual: shows the service area on the map
  2. Procurement spec: the exact geometry handed to a mobility data provider
     ("we need OD pings from origins within this radius")
  3. Compute boundary: device pings outside this radius are deleted from
     OD study processing (cost and compute scope control)

When Valhalla is provisioned, replace ST_Buffer with a Valhalla
90-minute drive-time isochrone. The WKT field updates in place;
no map rebuild required.

Requires: shapely
"""

import json
import math
import sys
from pathlib import Path

from shapely.geometry import Point, mapping

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR, DENSE_CATCHMENT_KM, DEFAULT_CATCHMENT_KM

WORK_DIR.mkdir(parents=True, exist_ok=True)

# OD study placeholder label
OD_PLACEHOLDER = "OD data pending — contact data provider with this polygon"


def km_to_degrees(km: float, latitude: float) -> float:
    """Approximate degree offset for a given km at a given latitude."""
    lat_deg = km / 111.0
    lon_deg = km / (111.0 * math.cos(math.radians(latitude)))
    return (lat_deg + lon_deg) / 2


def buffer_circle(lon: float, lat: float, radius_km: float, n_points: int = 64) -> dict:
    """Generate a circular polygon approximation using raw math (no projection)."""
    coords = []
    for i in range(n_points):
        angle = 2 * math.pi * i / n_points
        dlat = (radius_km / 111.0) * math.sin(angle)
        dlon = (radius_km / (111.0 * math.cos(math.radians(lat)))) * math.cos(angle)
        coords.append([round(lon + dlon, 5), round(lat + dlat, 5)])
    coords.append(coords[0])  # close ring
    return {"type": "Polygon", "coordinates": [coords]}


def main():
    clusters_path = WORK_DIR / "clusters.geojson"
    if not clusters_path.exists():
        print("clusters.geojson not found. Run build-clusters.py first.")
        sys.exit(1)

    with open(clusters_path) as f:
        fc = json.load(f)

    features = []
    for feat in fc.get("features", []):
        lon, lat = feat["geometry"]["coordinates"]
        props = feat["properties"]
        radius_km = props.get("catchment_radius_km", DEFAULT_CATCHMENT_KM)

        geom = buffer_circle(lon, lat, radius_km)

        radius_feat = {
            "type": "Feature",
            "geometry": geom,
            "properties": {
                "cluster_id":        props.get("cluster_id"),
                "rank":              props.get("rank"),
                "city":              props.get("city", ""),
                "iso_country_code":  props.get("iso_country_code", ""),
                "catchment_radius_km": radius_km,
                "radius_type":       "circular_buffer",
                "od_status":         "pending",
                "od_label":          OD_PLACEHOLDER,
                "home_catchment":    "Home catchment (OD data pending)",
                "work_catchment":    "Work catchment (OD data pending)",
                "valhalla_ready":    False,
            },
        }
        features.append(radius_feat)

    out = WORK_DIR / "radius.geojson"
    with open(out, "w") as f:
        json.dump({"type": "FeatureCollection", "features": features}, f)

    print(f"Written {len(features)} radius polygons to {out}")
    print("Run build-tiles.py --layer 3 to generate layer3-radius.pmtiles")


if __name__ == "__main__":
    main()
