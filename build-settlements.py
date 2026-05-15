#!/usr/bin/env python3
"""
build-settlements.py — Download and build settlement-level boundary files.

Produces three files in BOUNDARIES_DIR:
  us_places.geojson       — US Census TIGER Places (incorporated cities/towns/villages)
  eu_municipalities.geojson — GISCO LAU 2021 (EU+EEA municipalities) + GADM GBR admin-3
  ca_places_osm.geojson   — Nominatim batch results for CA cluster centroids
                             (fills the gap where CSD is a county, e.g. Strathcona County
                             → resolves to "Sherwood Park" at settlement level)

These feed resolve_market() in utils/region_engine.py. Point-in-polygon only —
geometries are simplified aggressively (tolerance=0.005°, ~500m) to keep files small.

Run once to build; rerun when adding new countries or refreshing boundaries.
"""

import json
import sys
import time
import zipfile
import io
import urllib.request
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent / "pointsav-monorepo" / "app-orchestration-gis"))
from config import BOUNDARIES_DIR, TOTEBOX_DATA_PATH

try:
    import geopandas as gpd
    from shapely.geometry import Point
except ImportError:
    print("ERROR: geopandas / shapely not available")
    sys.exit(1)

BOUNDARIES_DIR = Path(BOUNDARIES_DIR)
WORK_DIR = TOTEBOX_DATA_PATH / "boundaries-work"
WORK_DIR.mkdir(parents=True, exist_ok=True)

SIMPLIFY_TOL = 0.005  # degrees ≈ 500m — sufficient for point-in-polygon


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def download(url: str, dest: Path, label: str) -> bool:
    if dest.exists():
        print(f"  {label}: already cached at {dest.name}")
        return True
    print(f"  {label}: downloading {url} ...", flush=True)
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "foundry-gis-build/1.0"})
        with urllib.request.urlopen(req, timeout=120) as r:
            data = r.read()
        dest.write_bytes(data)
        print(f"  {label}: saved {len(data)//1024//1024} MB → {dest.name}")
        return True
    except Exception as e:
        print(f"  {label}: FAILED — {e}")
        return False


def simplify_and_save(gdf: gpd.GeoDataFrame, name_col: str, out_path: Path, label: str):
    gdf = gdf[[name_col, "geometry"]].copy()
    gdf = gdf.rename(columns={name_col: "name"})
    gdf = gdf[gdf["name"].notna() & (gdf["name"].str.strip() != "")]
    gdf["geometry"] = gdf["geometry"].simplify(SIMPLIFY_TOL, preserve_topology=True)
    gdf = gdf[~gdf["geometry"].is_empty]
    gdf.to_file(out_path, driver="GeoJSON")
    size_kb = out_path.stat().st_size // 1024
    print(f"  {label}: {len(gdf)} features → {out_path.name} ({size_kb} KB)")


# ---------------------------------------------------------------------------
# 1. US — Census TIGER Places (incorporated places + CDPs), 2023 500k scale
# ---------------------------------------------------------------------------

def build_us_places():
    print("\n=== US Places (TIGER 2023) ===")
    zip_path = WORK_DIR / "cb_2023_us_place_500k.zip"
    url = "https://www2.census.gov/geo/tiger/GENZ2023/shp/cb_2023_us_place_500k.zip"
    out = BOUNDARIES_DIR / "us_places.geojson"

    if out.exists():
        print(f"  us_places.geojson exists ({out.stat().st_size//1024} KB) — skipping")
        return

    if not download(url, zip_path, "TIGER Places"):
        return

    print("  reading shapefile from zip ...", flush=True)
    gdf = gpd.read_file(f"zip://{zip_path}")
    print(f"  {len(gdf)} raw features, cols: {list(gdf.columns)}")
    simplify_and_save(gdf, "NAME", out, "US Places")


# ---------------------------------------------------------------------------
# 2. EU + EEA — GISCO LAU 2021 (municipality level, 1:1M scale)
#    Covers: all EU27 + NO, IS, LI, CH (EEA/EFTA subset)
#    Then supplements with GADM GBR admin-3 for UK (not in GISCO post-Brexit)
# ---------------------------------------------------------------------------

def build_eu_municipalities():
    print("\n=== EU Municipalities (GISCO LAU 2021 + GADM GBR) ===")
    out = BOUNDARIES_DIR / "eu_municipalities.geojson"

    if out.exists():
        print(f"  eu_municipalities.geojson exists ({out.stat().st_size//1024} KB) — skipping")
        return

    # GISCO LAU 2021 — 1:1M scale (manageable size, covers most of our EU markets)
    lau_path = WORK_DIR / "LAU_RG_01M_2021_4326.geojson"
    lau_url = ("https://gisco-services.ec.europa.eu/distribution/v2/lau/geojson/"
               "LAU_RG_01M_2021_4326.geojson")

    lau_ok = download(lau_url, lau_path, "GISCO LAU")

    # GADM GBR admin-3 — districts (Salford, Manchester, Trafford, etc.)
    gbr_path = WORK_DIR / "gadm41_GBR_3.json"
    gbr_url = "https://geodata.ucdavis.edu/gadm/gadm4.1/json/gadm41_GBR_3.json"
    gbr_ok = download(gbr_url, gbr_path, "GADM GBR admin-3")

    frames = []

    if lau_ok and lau_path.exists():
        print("  loading GISCO LAU ...", flush=True)
        try:
            lau = gpd.read_file(lau_path)
            # Name column varies by year — find it
            name_col = next(
                (c for c in ["LAU_NAME", "GISCO_ID", "NAME_LATN", "LAU_NAME_LATIN"] if c in lau.columns),
                None
            )
            print(f"  GISCO LAU cols: {list(lau.columns)[:10]} → using name_col={name_col}")
            if name_col:
                lau = lau[[name_col, "geometry"]].rename(columns={name_col: "name"})
                lau["geometry"] = lau["geometry"].simplify(SIMPLIFY_TOL, preserve_topology=True)
                lau = lau[lau["name"].notna() & ~lau["geometry"].is_empty]
                frames.append(lau)
                print(f"  GISCO LAU: {len(lau)} municipalities loaded")
        except Exception as e:
            print(f"  GISCO LAU load failed: {e}")

    if gbr_ok and gbr_path.exists():
        print("  loading GADM GBR admin-3 ...", flush=True)
        try:
            gbr = gpd.read_file(gbr_path)
            name_col = next(
                (c for c in ["NAME_3", "NAME_2", "NAME_1"] if c in gbr.columns),
                None
            )
            print(f"  GADM GBR cols: {list(gbr.columns)} → using name_col={name_col}")
            if name_col:
                gbr = gbr[[name_col, "geometry"]].rename(columns={name_col: "name"})
                gbr["geometry"] = gbr["geometry"].simplify(SIMPLIFY_TOL, preserve_topology=True)
                gbr = gbr[gbr["name"].notna() & ~gbr["geometry"].is_empty]
                frames.append(gbr)
                print(f"  GADM GBR: {len(gbr)} districts loaded")
        except Exception as e:
            print(f"  GADM GBR load failed: {e}")

    if not frames:
        print("  ERROR: no data to combine for EU municipalities")
        return

    import pandas as pd
    combined = pd.concat(frames, ignore_index=True)
    combined = gpd.GeoDataFrame(combined, crs="EPSG:4326")
    combined.to_file(out, driver="GeoJSON")
    size_kb = out.stat().st_size // 1024
    print(f"  EU municipalities: {len(combined)} total features → {out.name} ({size_kb} KB)")


# ---------------------------------------------------------------------------
# 3. Canada — Nominatim batch lookup for cluster centroids
#    For clusters where CSD gives a county-level name, Nominatim resolves to
#    the settlement (e.g. Strathcona County → Sherwood Park).
#    Saves a mapping: cluster_id → settlement_name in ca_places_nominatim.json
# ---------------------------------------------------------------------------

def build_ca_nominatim():
    print("\n=== Canada — Nominatim settlement lookup ===")
    out = BOUNDARIES_DIR / "ca_places_nominatim.json"

    if out.exists():
        print(f"  ca_places_nominatim.json exists — skipping")
        return

    # Load CA cluster centroids
    meta_path = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
    if not meta_path.exists():
        print("  clusters-meta.json not found — skipping CA nominatim")
        return

    with open(meta_path) as f:
        clusters = json.load(f)

    ca_clusters = [c for c in clusters if c.get("iso") == "CA"]
    print(f"  {len(ca_clusters)} CA clusters to look up")

    # Check which ones have CSD names that are county-level
    # (contains "County", "Region", "District", "Municipality")
    # We load the CSD data to pre-check
    from shapely.geometry import Point
    from shapely.strtree import STRtree
    from shapely.geometry import shape

    csd_path = BOUNDARIES_DIR / "ca_csd.geojson"
    county_keywords = ("County", "Region", "District", "Municipality", "Township",
                       "Parish", "Rural")

    csd_features = []
    if csd_path.exists():
        with open(csd_path) as f:
            fc = json.load(f)
        for feat in fc.get("features", []):
            try:
                geom = shape(feat["geometry"])
                csd_features.append((geom, feat.get("properties", {})))
            except Exception:
                continue

    csd_tree = STRtree([f[0] for f in csd_features]) if csd_features else None

    def get_csd_name(lat, lon):
        if not csd_tree:
            return None
        pt = Point(lon, lat)
        hits = csd_tree.query(pt, predicate="within")
        if len(hits) > 0:
            props = csd_features[hits[0]][1]
            raw = props.get("NAME_3") or ""
            import re
            s = re.sub(r"(?<=[a-z])(?=[A-Z])", " ", raw)
            return s.strip()
        return None

    # Only query Nominatim for clusters where CSD name is county-like
    to_lookup = []
    for c in ca_clusters:
        csd_name = get_csd_name(c["lat"], c["lon"])
        if csd_name and any(kw in csd_name for kw in county_keywords):
            to_lookup.append(c)
        elif not csd_name:
            to_lookup.append(c)

    print(f"  {len(to_lookup)} clusters need Nominatim lookup (county CSD or no CSD)")

    if not to_lookup:
        out.write_text("{}")
        print("  all CA clusters have city-level CSD names — no Nominatim needed")
        return

    # Nominatim batch at 1 rps (public API policy)
    results = {}
    base = "https://nominatim.openstreetmap.org/reverse"
    headers = {"User-Agent": "foundry-gis-build/1.0 (contact: open.source@pointsav.com)"}

    print(f"  querying Nominatim at 1 rps for {len(to_lookup)} clusters ...")
    for i, c in enumerate(to_lookup):
        cid = c["id"]
        lat, lon = c["lat"], c["lon"]
        url = f"{base}?lat={lat}&lon={lon}&zoom=10&format=jsonv2&accept-language=en"
        try:
            req = urllib.request.Request(url, headers=headers)
            with urllib.request.urlopen(req, timeout=15) as r:
                data = json.loads(r.read())
            addr = data.get("address", {})
            # Priority: city > town > village > hamlet > municipality > county
            name = (addr.get("city") or addr.get("town") or addr.get("village")
                    or addr.get("hamlet") or addr.get("municipality")
                    or addr.get("county") or "")
            results[cid] = name.strip()
            if i % 10 == 0:
                print(f"    {i+1}/{len(to_lookup)}: {cid} → {name}")
        except Exception as e:
            print(f"    {cid}: error — {e}")
            results[cid] = ""
        time.sleep(1.0)  # 1 rps — Nominatim public policy

    out.write_text(json.dumps(results, indent=2, ensure_ascii=False))
    print(f"  saved {len(results)} results → {out.name}")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

if __name__ == "__main__":
    import argparse
    p = argparse.ArgumentParser()
    p.add_argument("--us", action="store_true")
    p.add_argument("--eu", action="store_true")
    p.add_argument("--ca", action="store_true")
    p.add_argument("--all", action="store_true")
    args = p.parse_args()

    run_all = args.all or not (args.us or args.eu or args.ca)

    if run_all or args.us:
        build_us_places()
    if run_all or args.eu:
        build_eu_municipalities()
    if run_all or args.ca:
        build_ca_nominatim()

    print("\nDone.")
