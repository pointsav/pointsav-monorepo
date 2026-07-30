#!/usr/bin/env python3
"""
export-aec-coverage.py — AEC data coverage CSV export for J3 §6 Results.

Uses intermediate GeoJSON/raster files from AEC nightly builds (Nights 2–3)
to perform spatial joins against all current clusters.

Outputs:
  work/DATA-aec-clusters.csv          — per-cluster AEC annotations (6,493 rows)
  work/DATA-aec-coverage-summary.csv  — coverage % by layer × country for J3 §6

Layers covered:
  - ASHRAE 169-2013 climate zones (US only, from Night 2)
  - EU regulatory climate zones (EU countries, from Night 2)
  - Köppen-Geiger class (global, raster sampling from Night 3 TIF)
  - Ecoregion name + biome (global, from Night 3)

Layers NOT yet available (pending builds):
  - Seismic PGA (Night 4 — URL fix applied, re-run pending)
  - Flood hazard (Night 5 — scheduled 2026-05-31 05:00 UTC)

Run from:  app-orchestration-gis/
  python3 export-aec-coverage.py

Requires:  shapely 2.x, osgeo.gdal 3.x (both present on VM)
Estimated runtime:  3–5 minutes (ecoregion GeoJSON load dominates)
"""

import json
import csv
import sys
import struct
from pathlib import Path

from shapely.geometry import shape, Point
from shapely.strtree import STRtree
from osgeo import gdal

SCRIPT_DIR = Path(__file__).parent
WORK_DIR = SCRIPT_DIR / 'work'
AEC_DIR = WORK_DIR / 'aec'
META_PATH = Path('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json')

OUT_CLUSTERS = WORK_DIR / 'DATA-aec-clusters.csv'
OUT_SUMMARY  = WORK_DIR / 'DATA-aec-coverage-summary.csv'

# Beck et al. 2018 raster code → Köppen class
KOPPEN_MAP = {
    1:'Af', 2:'Am', 3:'Aw', 4:'BWh', 5:'BWk', 6:'BSh', 7:'BSk',
    8:'Csa', 9:'Csb', 10:'Csc', 11:'Cwa', 12:'Cwb', 13:'Cwc',
    14:'Cfa', 15:'Cfb', 16:'Cfc', 17:'Dsa', 18:'Dsb', 19:'Dsc', 20:'Dsd',
    21:'Dwa', 22:'Dwb', 23:'Dwc', 24:'Dwd', 25:'Dfa', 26:'Dfb', 27:'Dfc', 28:'Dfd',
    29:'ET', 30:'EF',
}

EU_ISOS = {'GB','DE','FR','ES','IT','PL','NL','AT','PT','GR','IS','SE','DK','FI','NO'}
COUNTRIES = ['US','CA','MX','GB','DE','FR','ES','IT','PL','NL','AT','PT','GR','IS','SE','DK','FI','NO']


def build_shapely_index(geojson_path, zone_field):
    """Load a GeoJSON and return (STRtree, geom_list, zone_list)."""
    size_mb = geojson_path.stat().st_size // 1_000_000
    print(f"  Loading {geojson_path.name} ({size_mb} MB)...", flush=True)
    with open(geojson_path) as f:
        gj = json.load(f)
    geoms, zones = [], []
    for feat in gj['features']:
        z = feat['properties'].get(zone_field)
        if not z:
            continue
        try:
            geoms.append(shape(feat['geometry']))
            zones.append(str(z))
        except Exception:
            pass
    print(f"    {len(geoms)} polygons indexed", flush=True)
    return STRtree(geoms), geoms, zones


def point_in_index(tree, geoms, zones, lat, lon):
    pt = Point(lon, lat)
    for idx in tree.query(pt):
        if geoms[idx].contains(pt):
            return zones[idx]
    return None


def open_koppen_raster(tif_path):
    """Open Köppen raster and return (dataset, geotransform, inv_geotransform)."""
    ds = gdal.Open(str(tif_path))
    if ds is None:
        return None, None, None
    gt = ds.GetGeoTransform()
    # Inverse geotransform: lon,lat → col,row
    inv_gt = gdal.InvGeoTransform(gt)
    return ds, gt, inv_gt


def sample_koppen(ds, band, inv_gt, lat, lon):
    """Sample Köppen raster at (lat, lon). Returns class string or None.

    Uses ReadRaster + struct.unpack instead of ReadAsArray to avoid the
    gdal_array / NumPy 2.x ABI incompatibility (_ARRAY_API not found).
    The Beck et al. 2018 TIF is GDT_Byte, so one unsigned byte per pixel.
    """
    col, row = gdal.ApplyGeoTransform(inv_gt, lon, lat)
    col, row = int(col), int(row)
    if col < 0 or row < 0 or col >= ds.RasterXSize or row >= ds.RasterYSize:
        return None
    val_bytes = band.ReadRaster(col, row, 1, 1)
    if not val_bytes:
        return None
    code = struct.unpack('B', val_bytes)[0]
    return KOPPEN_MAP.get(code)


def pct_str(n, d):
    return f"{100 * n / d:.1f}%" if d else "—"


# ── Load clusters ─────────────────────────────────────────────────────────────

print(f"Loading clusters from {META_PATH}...", flush=True)
with open(META_PATH) as f:
    clusters = json.load(f)
print(f"  {len(clusters)} clusters", flush=True)

# ── Build spatial indices ─────────────────────────────────────────────────────

print("\n[1/4] ASHRAE 169-2013 climate zones (US)", flush=True)
ashrae_path = AEC_DIR / 'ashrae-zones-us.geojson'
ashrae_tree = ashrae_geoms = ashrae_zones = None
if ashrae_path.exists():
    ashrae_tree, ashrae_geoms, ashrae_zones = build_shapely_index(ashrae_path, 'ashrae_zone')
else:
    print(f"  WARN: {ashrae_path} not found — ASHRAE layer skipped", flush=True)

print("\n[2/4] EU regulatory climate zones", flush=True)
eu_path = AEC_DIR / 'eu-climate-zones-merged.geojson'
eu_tree = eu_geoms = eu_zones = None
if eu_path.exists():
    eu_tree, eu_geoms, eu_zones = build_shapely_index(eu_path, 'zone_code')
else:
    print(f"  WARN: {eu_path} not found — EU climate layer skipped", flush=True)

print("\n[3/4] Köppen-Geiger (raster sampling)", flush=True)
koppen_path = AEC_DIR / 'koppen_geiger.tif'
koppen_ds = koppen_band = koppen_inv_gt = None
if koppen_path.exists():
    koppen_ds, _, koppen_inv_gt = open_koppen_raster(koppen_path)
    if koppen_ds:
        print(f"  Raster: {koppen_ds.RasterXSize}×{koppen_ds.RasterYSize} px", flush=True)
        koppen_band = koppen_ds.GetRasterBand(1)
    else:
        print(f"  WARN: could not open raster", flush=True)
else:
    print(f"  WARN: {koppen_path} not found — Köppen layer skipped", flush=True)

print("\n[4/4] WWF Ecoregions 2017", flush=True)
eco_path = AEC_DIR / 'ecoregions-global.geojson'
eco_tree = eco_geoms = eco_names = eco_biomes = None
if eco_path.exists():
    eco_tree, eco_geoms, eco_names_raw = build_shapely_index(eco_path, 'ECO_NAME')
    # Also pull biome names from same features (second pass avoided — rebuild from raw)
    with open(eco_path) as f:
        eco_gj = json.load(f)
    eco_biomes = []
    for feat in eco_gj['features']:
        if feat['properties'].get('ECO_NAME'):
            eco_biomes.append(str(feat['properties'].get('BIOME_NAME', '')))
    eco_names = eco_names_raw
    del eco_gj  # free memory
else:
    print(f"  WARN: {eco_path} not found — ecoregion layer skipped", flush=True)

# ── Annotate clusters ─────────────────────────────────────────────────────────

print(f"\nAnnotating {len(clusters)} clusters...", flush=True)
rows = []
for i, c in enumerate(clusters):
    if i % 1000 == 0:
        print(f"  {i}/{len(clusters)}...", flush=True)

    lat = c['lat']
    lon = c['lon']
    iso = c.get('iso', '')

    # ASHRAE (US only)
    ashrae_zone = None
    if iso == 'US' and ashrae_tree:
        ashrae_zone = point_in_index(ashrae_tree, ashrae_geoms, ashrae_zones, lat, lon)

    # EU climate (EU countries only)
    eu_climate_zone = None
    if iso in EU_ISOS and eu_tree:
        eu_climate_zone = point_in_index(eu_tree, eu_geoms, eu_zones, lat, lon)

    # Köppen raster
    koppen_class = None
    if koppen_ds and koppen_band:
        koppen_class = sample_koppen(koppen_ds, koppen_band, koppen_inv_gt, lat, lon)

    # Ecoregion
    ecoregion_name = ecoregion_biome = None
    if eco_tree:
        pt = Point(lon, lat)
        for idx in eco_tree.query(pt):
            if eco_geoms[idx].contains(pt):
                ecoregion_name = eco_names[idx]
                ecoregion_biome = eco_biomes[idx] if idx < len(eco_biomes) else ''
                break

    rows.append({
        'cluster_id':      c.get('id', ''),
        'iso':             iso,
        'tier':            c.get('t', ''),
        'lat':             lat,
        'lon':             lon,
        'ashrae_zone':     ashrae_zone or '',
        'eu_climate_zone': eu_climate_zone or '',
        'koppen_class':    koppen_class or '',
        'ecoregion_name':  ecoregion_name or '',
        'ecoregion_biome': ecoregion_biome or '',
    })

print(f"  Done.", flush=True)

# ── Write per-cluster CSV ─────────────────────────────────────────────────────

CLUSTER_FIELDS = [
    'cluster_id','iso','tier','lat','lon',
    'ashrae_zone','eu_climate_zone','koppen_class','ecoregion_name','ecoregion_biome',
]
print(f"\nWriting {OUT_CLUSTERS}...", flush=True)
with open(OUT_CLUSTERS, 'w', newline='') as f:
    w = csv.DictWriter(f, fieldnames=CLUSTER_FIELDS)
    w.writeheader()
    w.writerows(rows)
print(f"  {len(rows)} rows", flush=True)

# ── Write coverage summary ────────────────────────────────────────────────────

print(f"\nWriting {OUT_SUMMARY}...", flush=True)
by_iso = {iso: [r for r in rows if r['iso'] == iso] for iso in COUNTRIES}

def coverage_row(scope, iso, subset):
    n = len(subset)
    return {
        'scope':           scope,
        'iso':             iso,
        'n_clusters':      n,
        'ashrae_n':        sum(1 for r in subset if r['ashrae_zone']),
        'ashrae_pct':      pct_str(sum(1 for r in subset if r['ashrae_zone']), n),
        'eu_climate_n':    sum(1 for r in subset if r['eu_climate_zone']),
        'eu_climate_pct':  pct_str(sum(1 for r in subset if r['eu_climate_zone']), n),
        'koppen_n':        sum(1 for r in subset if r['koppen_class']),
        'koppen_pct':      pct_str(sum(1 for r in subset if r['koppen_class']), n),
        'ecoregion_n':     sum(1 for r in subset if r['ecoregion_name']),
        'ecoregion_pct':   pct_str(sum(1 for r in subset if r['ecoregion_name']), n),
    }

SUMMARY_FIELDS = [
    'scope','iso','n_clusters',
    'ashrae_n','ashrae_pct',
    'eu_climate_n','eu_climate_pct',
    'koppen_n','koppen_pct',
    'ecoregion_n','ecoregion_pct',
]

summary_rows = [coverage_row('All', 'ALL', rows)]
for iso in COUNTRIES:
    subset = by_iso.get(iso, [])
    if subset:
        summary_rows.append(coverage_row('Country', iso, subset))

# Continent subtotals
na_rows = [r for r in rows if r['iso'] in {'US','CA','MX'}]
eu_rows = [r for r in rows if r['iso'] in EU_ISOS]
summary_rows.insert(1, coverage_row('Continent', 'NA', na_rows))
summary_rows.insert(2, coverage_row('Continent', 'EU', eu_rows))

with open(OUT_SUMMARY, 'w', newline='') as f:
    w = csv.DictWriter(f, fieldnames=SUMMARY_FIELDS)
    w.writeheader()
    w.writerows(summary_rows)
print(f"  {len(summary_rows)} rows", flush=True)

print(f"\n── AEC Coverage Export Complete ──")
print(f"  Per-cluster: {OUT_CLUSTERS}")
print(f"  Summary:     {OUT_SUMMARY}")
print(f"\nLayers NOT yet available (pending nightly builds):")
print(f"  seismic_pga_g  — Night 4 re-run needed (URL fix: bd17a348)")
print(f"  flood_hazard   — Night 5 scheduled 2026-05-31 05:00 UTC")
