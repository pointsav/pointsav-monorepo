#!/usr/bin/env python3
"""
build-koppen-join.py — Point-sample koppen_class for all clusters.
Reads koppen_geiger.tif, samples each cluster centroid, patches clusters-meta.json.

KOPPEN_RASTER values (Beck 2018):
  1=Af, 2=Am, 3=Aw, 4=BWh, 5=BWk, 6=BSh, 7=BSk,
  8=Csa, 9=Csb, 10=Csc, 11=Cwa, 12=Cwb, 13=Cwc,
  14=Cfa, 15=Cfb, 16=Cfc,
  17=Dsa, 18=Dsb, 19=Dsc, 20=Dsd,
  21=Dwa, 22=Dwb, 23=Dwc, 24=Dwd,
  25=Dfa, 26=Dfb, 27=Dfc, 28=Dfd,
  29=ET, 30=EF

Usage: python3 build-koppen-join.py [--dry-run] [--overwrite]
"""
import json, sys, struct
from pathlib import Path
try:
    from osgeo import gdal
    # Do NOT call gdal.UseExceptions() — triggers lazy gdal_array import, crashes under numpy 2.x
except ImportError:
    sys.exit("ERROR: osgeo/gdal not available — pip install gdal or apt-get install python3-gdal")

KOPPEN_INT_TO_CODE = {
    1:'Af', 2:'Am', 3:'Aw',
    4:'BWh', 5:'BWk', 6:'BSh', 7:'BSk',
    8:'Csa', 9:'Csb', 10:'Csc', 11:'Cwa', 12:'Cwb', 13:'Cwc', 14:'Cfa', 15:'Cfb', 16:'Cfc',
    17:'Dsa', 18:'Dsb', 19:'Dsc', 20:'Dsd', 21:'Dwa', 22:'Dwb', 23:'Dwc', 24:'Dwd',
    25:'Dfa', 26:'Dfb', 27:'Dfc', 28:'Dfd',
    29:'ET', 30:'EF',
}

SCRIPT_DIR = Path(__file__).parent
RASTER_PATH = SCRIPT_DIR / 'work/aec/koppen_geiger.tif'
META_PATH = Path('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json')

dry_run = '--dry-run' in sys.argv
overwrite = '--overwrite' in sys.argv

ds = gdal.Open(str(RASTER_PATH))
gt = ds.GetGeoTransform()
band = ds.GetRasterBand(1)

def sample_koppen(lon, lat):
    px = int((lon - gt[0]) / gt[1])
    py = int((lat - gt[3]) / gt[5])
    if px < 0 or py < 0 or px >= ds.RasterXSize or py >= ds.RasterYSize:
        return None
    raw = band.ReadRaster(px, py, 1, 1)
    if not raw: return None
    val = struct.unpack('B', raw)[0]  # Byte band — no numpy needed
    return KOPPEN_INT_TO_CODE.get(val)

with open(META_PATH) as f:
    clusters = json.load(f)

patched = 0
for c in clusters:
    if not overwrite and c.get('koppen_class'):
        continue
    v = sample_koppen(c['lon'], c['lat'])
    if v:
        c['koppen_class'] = v
        patched += 1

print(f"Patched {patched}/{len(clusters)} clusters with koppen_class")
if not dry_run:
    with open(META_PATH, 'w') as f:
        json.dump(clusters, f, separators=(',',':'))
    print("clusters-meta.json updated")
