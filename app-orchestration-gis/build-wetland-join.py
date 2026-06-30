#!/usr/bin/env python3
"""
build-wetland-join.py — GWL_FCS30 wetland classification join
Phase 22 2026-06-29 — samples VRT mosaic (408 tiles, 30m resolution).
Uses band.ReadRaster() to avoid numpy 2.x / GDAL incompatibility.
"""
import json, struct
from pathlib import Path
from osgeo import gdal
# Do NOT call gdal.UseExceptions() — crashes under numpy 2.x

VRT_PATH  = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/work/aec/gwl-fcs30-mosaic.vrt")
META_PATH = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")

WETLAND_CODES = {
    100: 'permanent_water',
    181: 'permanent_water',
    182: 'seasonal_water',
    183: 'permanent_wetland',
    184: 'seasonal_wetland',
    185: 'mangrove',
    186: 'salt_marsh',
    190: 'wetland',
}

print("Opening GWL_FCS30 VRT mosaic...")
ds   = gdal.Open(str(VRT_PATH))
band = ds.GetRasterBand(1)
gt   = ds.GetGeoTransform()   # (origin_x, pixel_w, 0, origin_y, 0, pixel_h)
print(f"  Size: {ds.RasterXSize} x {ds.RasterYSize}; GT: origin=({gt[0]:.2f},{gt[3]:.2f}) px=({gt[1]:.6f},{gt[5]:.6f})")

def sample_wetland(lon, lat):
    px = int((lon - gt[0]) / gt[1])
    py = int((lat - gt[3]) / gt[5])
    if px < 0 or py < 0 or px >= ds.RasterXSize or py >= ds.RasterYSize:
        return None
    raw = band.ReadRaster(px, py, 1, 1)
    if not raw:
        return None
    val = struct.unpack('B', raw)[0]
    return WETLAND_CODES.get(val)

print("Loading clusters-meta.json...")
clusters = json.loads(META_PATH.read_text())
already  = sum(1 for c in clusters if c.get('wetland_class') is not None)
print(f"  {len(clusters)} clusters; {already} already have wetland_class")

n = 0
for i, c in enumerate(clusters):
    if c.get('wetland_class') is not None:
        continue
    cls = sample_wetland(c.get('lon', 0), c.get('lat', 0))
    if cls is not None:
        c['wetland_class'] = cls
        n += 1
    if (i + 1) % 1000 == 0:
        print(f"  Progress: {i+1}/{len(clusters)}, wetlands found: {n}")

META_PATH.write_text(json.dumps(clusters, separators=(',', ':')))
total = sum(1 for c in clusters if c.get('wetland_class') is not None)
print(f"\nDone. wetland_class: {total}/{len(clusters)} clusters patched ({n} new).")
