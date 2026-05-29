import os
import json
import rasterio
from rasterio.windows import Window
import numpy as np
from utils.spatial_filter import ClusterFilter
from multiprocessing import Pool

# Configuration
CLUSTERS_META = "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
DATA_BASE_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/raw/"
OUTPUT_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/"

def process_iso(args):
    iso, cf = args
    input_file = os.path.join(DATA_BASE_DIR, f"{iso}_pop_2026.tif")
    output_file = os.path.join(OUTPUT_DIR, f"cleansed-census-{iso.upper()}.jsonl")
    
    if not os.path.exists(input_file):
        print(f"Skipping {iso}: Raw file not found.")
        return
        
    print(f"Ingesting {iso} 100m census data (150km filter)...")
    
    with rasterio.open(input_file) as src:
        transform = src.transform
        nodata = src.nodata
        
        kept_pixels = 0
        chunk_size = 2000 # Process in 2km x 2km blocks
        
        with open(output_file, 'w') as out:
            chunk_count = 0
            for r in range(0, src.height, chunk_size):
                for c in range(0, src.width, chunk_size):
                    h = min(chunk_size, src.height - r)
                    w = min(chunk_size, src.width - c)
                    
                    window = Window(c, r, w, h)
                    data = src.read(1, window=window)
                    
                    if np.all((data == nodata) | (data <= 0)):
                        continue
                        
                    lon_min, lat_max = transform * (c, r)
                    lon_max, lat_min = transform * (c + w, r + h)
                    
                    chunk_active = False
                    for lon_int in range(int(lon_min) - 3, int(lon_max) + 4):
                        for lat_int in range(int(lat_min) - 3, int(lat_max) + 4):
                            if (lon_int, lat_int) in cf.buckets:
                                chunk_active = True
                                break
                        if chunk_active: break
                        
                    if not chunk_active:
                        continue
                        
                    for row_idx in range(data.shape[0]):
                        for col_idx in range(data.shape[1]):
                            val = data[row_idx, col_idx]
                            if val > 0 and val != nodata:
                                lon, lat = transform * (c + col_idx + 0.5, r + row_idx + 0.5)
                                if cf.is_active(lon, lat):
                                    out.write(json.dumps({
                                        'lat': round(lat, 6),
                                        'lon': round(lon, 6),
                                        'pop': float(val)
                                    }) + '\n')
                                    kept_pixels += 1
                    
                    chunk_count += 1
                    if chunk_count % 10 == 0:
                        print(f"  [{iso.upper()}] Processed {chunk_count} active chunks...")
        
    print(f"  {iso.upper()}: Kept {kept_pixels} pixels.")

def ingest_census():
    cf = ClusterFilter(CLUSTERS_META, threshold_km=150.0)
    iso_codes = ["usa", "can", "mex", "gbr", "deu", "fra", "nld", "aut", "prt", "grc", "dnk", "isl", "pol"]
    
    with Pool(processes=4) as pool:
        pool.map(process_iso, [(iso, cf) for iso in iso_codes])

if __name__ == "__main__":
    ingest_census()
