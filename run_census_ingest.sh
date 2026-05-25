export PYTHONPATH=$PYTHONPATH:/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
for iso in usa can mex gbr deu fra nld aut prt grc dnk isl pol; do
  echo "Processing $iso..."
  python3 -c "
import os
import json
import rasterio
from rasterio.windows import Window
import numpy as np
from utils.spatial_filter import ClusterFilter
CLUSTERS_META = '/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json'
DATA_BASE_DIR = '/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/raw/'
OUTPUT_DIR = '/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/'
iso = '$iso'
cf = ClusterFilter(CLUSTERS_META, threshold_km=150.0)
input_file = os.path.join(DATA_BASE_DIR, f'{iso}_pop_2026.tif')
output_file = os.path.join(OUTPUT_DIR, f'cleansed-census-{iso.upper()}.jsonl')
with rasterio.open(input_file) as src:
    transform = src.transform
    nodata = src.nodata
    with open(output_file, 'w') as out:
        chunk_size = 2000
        for r in range(0, src.height, chunk_size):
            for c in range(0, src.width, chunk_size):
                h = min(chunk_size, src.height - r)
                w = min(chunk_size, src.width - c)
                window = Window(c, r, w, h)
                data = src.read(1, window=window)
                if np.all((data == nodata) | (data <= 0)): continue
                lon_min, lat_max = transform * (c, r)
                lon_max, lat_min = transform * (c + w, r + h)
                chunk_active = False
                for lon_int in range(int(lon_min) - 3, int(lon_max) + 4):
                    for lat_int in range(int(lat_min) - 3, int(lat_max) + 4):
                        if (lon_int, lat_int) in cf.buckets:
                            chunk_active = True
                            break
                    if chunk_active: break
                if not chunk_active: continue
                for row_idx in range(data.shape[0]):
                    for col_idx in range(data.shape[1]):
                        val = data[row_idx, col_idx]
                        if val > 0 and val != nodata:
                            lon, lat = transform * (c + col_idx + 0.5, r + row_idx + 0.5)
                            if cf.is_active(lon, lat):
                                out.write(json.dumps({'lat': round(lat, 6), 'lon': round(lon, 6), 'pop': float(val)}) + '\n')
"
done
