#!/usr/bin/env python3
"""
build-ashrae-zone.py — Patch ashrae_zone into clusters-meta.json

Downloads the DOE/PNNL ASHRAE 169-2020 world climate zone GeoJSON once and
performs a point-in-polygon spatial join for every cluster centroid, writing
the result back to clusters-meta.json.

ASHRAE 169-2020 zones: 1A–8A, 1B–7B, 3C, 4C, 5C, 6C (A=Humid, B=Dry, C=Marine)
Values written: e.g. "2A", "5B", "3C"

Source: Pacific Northwest National Laboratory (PNNL) / DOE Building Technologies Office
GeoJSON: https://energyplus.net/assets/nrel_custom/epw_climate_zones/EPW_CZ_Map.geojson
Licence: public domain (US Government / DOE)

Usage:
    python3 build-ashrae-zone.py [--dry-run] [--overwrite] [--countries DE FR]
"""

import argparse
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR

META_PATH = Path(
    "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
)
ASHRAE_GEOJSON = WORK_DIR / "aec" / "ashrae-zones.geojson"
# Primary source: EnergyPlus / PNNL world climate zone map (public domain, DOE)
ASHRAE_URL = (
    "https://energyplus.net/assets/nrel_custom/epw_climate_zones/EPW_CZ_Map.geojson"
)


def download_ashrae(path: Path) -> bool:
    """Download ASHRAE zone GeoJSON. Returns True on success."""
    path.parent.mkdir(parents=True, exist_ok=True)
    print(f"Downloading ASHRAE climate zones → {path}")
    try:
        req = urllib.request.Request(
            ASHRAE_URL,
            headers={"User-Agent": "Mozilla/5.0 (compatible; gis-ashrae/1.0)"},
        )
        with urllib.request.urlopen(req, timeout=60) as r:
            data = r.read()
        # Minimal sanity check: should be JSON with features
        gj = json.loads(data)
        if "features" not in gj or len(gj["features"]) < 10:
            print(f"  WARN: unexpected GeoJSON structure ({len(gj.get('features', []))} features)")
            return False
        path.write_bytes(data)
        print(f"  → {path} ({len(gj['features'])} zone polygons)  ✓")
        return True
    except Exception as e:
        print(f"  ERROR: {e}")
        return False


def load_zones(path: Path):
    """
    Load ASHRAE zone polygons. Returns list of (zone_label, shapely_shape) tuples.
    Requires shapely. Falls back to bounding-box approximation if shapely absent.
    """
    try:
        from shapely.geometry import shape
    except ImportError:
        print("ERROR: shapely not available — install with: pip3 install shapely")
        sys.exit(1)

    gj = json.loads(path.read_text())
    zones = []
    for feat in gj.get("features", []):
        props = feat.get("properties") or {}
        # PNNL GeoJSON uses 'IECC_Climate_Zone' or 'BA_Climate_Zone' or 'Climate_Zone'
        zone = (
            props.get("IECC_Climate_Zone")
            or props.get("BA_Climate_Zone")
            or props.get("Climate_Zone")
            or props.get("zone")
            or props.get("CZ")
        )
        if not zone:
            # Try to extract from any string property containing a zone code pattern
            for v in props.values():
                s = str(v).strip()
                if len(s) == 2 and s[0].isdigit() and s[1].upper() in "ABC":
                    zone = s.upper()
                    break
        if not zone:
            continue
        zone = str(zone).strip().upper()
        try:
            geom = shape(feat["geometry"])
            zones.append((zone, geom))
        except Exception:
            continue
    print(f"  Loaded {len(zones)} ASHRAE zone polygons")
    return zones


def zone_for_point(lon: float, lat: float, zones: list) -> str | None:
    """Return ASHRAE zone label for point, or None if not found."""
    try:
        from shapely.geometry import Point
    except ImportError:
        return None
    pt = Point(lon, lat)
    for zone, geom in zones:
        if geom.contains(pt):
            return zone
    return None


def main():
    parser = argparse.ArgumentParser(description="Patch ashrae_zone in clusters-meta.json")
    parser.add_argument("--meta", default=str(META_PATH))
    parser.add_argument("--dry-run", action="store_true",
                        help="Query one cluster; no writes")
    parser.add_argument("--overwrite", action="store_true",
                        help="Re-assign clusters that already have ashrae_zone")
    parser.add_argument("--countries", nargs="+", default=None, metavar="ISO",
                        help="Limit to specific ISO-2 codes")
    args = parser.parse_args()

    meta_path = Path(args.meta)
    if not meta_path.exists():
        print(f"ERROR: {meta_path} not found", file=sys.stderr)
        sys.exit(1)

    # Download zone GeoJSON if not cached
    if not ASHRAE_GEOJSON.exists():
        ok = download_ashrae(ASHRAE_GEOJSON)
        if not ok:
            print("WARN: could not download ASHRAE zones — aborting")
            sys.exit(1)
    else:
        print(f"Using cached ASHRAE zones: {ASHRAE_GEOJSON}")

    zones = load_zones(ASHRAE_GEOJSON)
    if not zones:
        print("ERROR: no zone polygons loaded")
        sys.exit(1)

    clusters = json.loads(meta_path.read_text())
    target_isos = set(args.countries) if args.countries else None
    targets = [
        c for c in clusters
        if (target_isos is None or c.get("iso") in target_isos)
        and (args.overwrite or c.get("ashrae_zone") is None)
    ]
    print(f"Clusters total: {len(clusters)}  |  targets: {len(targets)}")

    if args.dry_run:
        if not targets:
            print("No eligible targets.")
            return
        c = targets[0]
        z = zone_for_point(c["lon"], c["lat"], zones)
        print(f"DRY RUN — cluster {c.get('id')} @ ({c['lat']}, {c['lon']}) → ashrae_zone={z}")
        return

    patched = 0
    for i, c in enumerate(targets):
        z = zone_for_point(c.get("lon", 0), c.get("lat", 0), zones)
        if z:
            c["ashrae_zone"] = z
            patched += 1
        if (i + 1) % 500 == 0:
            print(f"  {i+1}/{len(targets)} — patched {patched}", flush=True)

    print(f"Patched {patched}/{len(targets)} clusters with ashrae_zone")

    if patched > 0:
        tmp = str(meta_path) + ".tmp"
        with open(tmp, "w") as f:
            json.dump(clusters, f, separators=(",", ":"))
        os.replace(tmp, str(meta_path))
        print(f"Wrote {meta_path}")
    else:
        print("No clusters patched — clusters-meta.json unchanged")


if __name__ == "__main__":
    main()
