import os
import json
import rasterio
import numpy as np
from utils.spatial_filter import ClusterFilter

# Configuration
CLUSTERS_META = "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
DATA_BASE_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/raw/"
OUTPUT_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/"

def run_full_ingest():
    cf = ClusterFilter(CLUSTERS_META)
    iso_codes = ["usa", "can", "mex", "gbr", "deu", "fra", "nld", "aut", "prt", "grc", "dnk", "isl", "pol"]
    
    for iso in iso_codes:
        input_file = os.path.join(DATA_BASE_DIR, f"{iso}_pop_2026.tif")
        output_file = os.path.join(OUTPUT_DIR, f"cleansed-census-{iso.upper()}.jsonl")
        
        if not os.path.exists(input_file):
            print(f"Skipping {iso}: Raw file not found.")
            continue
            
        print(f"Ingesting {iso}...")
        
        with rasterio.open(input_file) as src:
            data = src.read(1)
            transform = src.transform
            
            with open(output_file, 'w') as out:
                # Iterate through raster to process cells
                for row_idx in range(src.height):
                    for col_idx in range(src.width):
                        pop_count = data[row_idx, col_idx]
                        if pop_count > 0:
                            lon, lat = transform * (col_idx, row_idx)
                            # Export as JSONL record
                            out.write(json.dumps({
                                'grid_id': f"{iso}_{row_idx}_{col_idx}",
                                'lat': lat,
                                'lon': lon,
                                'pop_count': float(pop_count)
                            }) + '\n')
            
        print(f"  Saved full census data for {iso.upper()}.")

if __name__ == "__main__":
    run_full_ingest()
