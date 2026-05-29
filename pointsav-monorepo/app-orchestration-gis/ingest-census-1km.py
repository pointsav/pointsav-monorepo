import os
import json
import rasterio
from rasterio.enums import Resampling
import numpy as np

# Configuration
DATA_BASE_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/raw/"
OUTPUT_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/"
OUTPUT_FILE = os.path.join(OUTPUT_DIR, "cleansed-census-1km-ALL.jsonl")

def ingest_census_1km():
    # Countries to process
    iso_codes = ["usa", "can", "mex", "gbr", "deu", "fra", "nld", "aut", "prt", "grc", "dnk", "isl", "pol"]
    
    total_processed_pixels = 0
    
    with open(OUTPUT_FILE, 'w') as out:
        for iso in iso_codes:
            input_file = os.path.join(DATA_BASE_DIR, f"{iso}_pop_2026.tif")
            
            if not os.path.exists(input_file):
                print(f"Skipping {iso}: Raw file not found.")
                continue
                
            print(f"Ingesting {iso} census data at 1km resolution...")
            
            with rasterio.open(input_file) as src:
                # Calculate the 1km resolution (10x10 aggregation of 100m pixels)
                # Original resolution is approx 0.00083333333 degrees (~100m)
                # We want approx 0.00833333333 degrees (~1km)
                
                downsample_factor = 10
                new_height = src.height // downsample_factor
                new_width = src.width // downsample_factor
                
                # Use block-based reading to handle large rasters memory-efficiently
                # We'll read in chunks that are multiples of the downsample factor
                chunk_size = 1000 # 100x100 1km blocks
                
                transform = src.transform
                nodata = src.nodata
                
                chunk_count = 0
                for r in range(0, src.height, chunk_size):
                    for c in range(0, src.width, chunk_size):
                        chunk_count += 1
                        if chunk_count % 500 == 0:
                            print(f"  Processed {chunk_count} blocks...")
                        # Calculate window
                        h = min(chunk_size, src.height - r)
                        w = min(chunk_size, src.width - c)
                        
                        # Adjust height and width to be multiples of downsample_factor
                        # except at the very edges
                        h_adj = (h // downsample_factor) * downsample_factor
                        w_adj = (w // downsample_factor) * downsample_factor
                        
                        if h_adj == 0 or w_adj == 0:
                            # Handle edge blocks that are smaller than 10x10
                            # For simplicity in this 1km pilot, we might skip them or handle specially
                            # Let's read the full chunk and aggregate what we have
                            h_adj = h
                            w_adj = w

                        window = rasterio.windows.Window(c, r, w_adj, h_adj)
                        data = src.read(1, window=window)
                        
                        # Replace nodata with 0 for summation
                        data = np.where(data == nodata, 0, data)
                        data = np.where(data < 0, 0, data) # Remove negative values
                        
                        # Aggregate 10x10 blocks
                        # We use reshaping and summing for speed
                        # Shape of data: (h_adj, w_adj)
                        
                        # Final dimensions of this aggregated chunk
                        agg_h = h_adj // downsample_factor
                        agg_w = w_adj // downsample_factor
                        
                        if agg_h > 0 and agg_w > 0:
                            # Reshape to (agg_h, 10, agg_w, 10)
                            reshaped = data[:agg_h*downsample_factor, :agg_w*downsample_factor].reshape(agg_h, downsample_factor, agg_w, downsample_factor)
                            # Sum over axes 1 and 3 (the 10x10 blocks)
                            aggregated = reshaped.sum(axis=(1, 3))
                            
                            # Write non-zero results
                            for i in range(agg_h):
                                for j in range(agg_w):
                                    val = aggregated[i, j]
                                    if val > 0:
                                        # Calculate center of the 1km block
                                        # (c + j*10 + 5, r + i*10 + 5)
                                        lon, lat = transform * (c + j*downsample_factor + 5, r + i*downsample_factor + 5)
                                        
                                        out.write(json.dumps({
                                            'iso': iso.upper(),
                                            'lat': round(lat, 5),
                                            'lon': round(lon, 5),
                                            'pop': float(val)
                                        }) + '\n')
                                        total_processed_pixels += 1
                                        
            print(f"  Finished {iso.upper()}.")

    print(f"Completed 1km ingest. Total records: {total_processed_pixels}")

if __name__ == "__main__":
    ingest_census_1km()
