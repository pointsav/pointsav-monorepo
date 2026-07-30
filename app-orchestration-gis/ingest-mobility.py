import os
import json
from utils.spatial_filter import ClusterFilter

# Configuration
CLUSTERS_META = "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
DATA_BASE_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-mobility/raw/"
OUTPUT_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-mobility/"

def ingest_mobility():
    # Using the same 150km Data Radius filter
    cf = ClusterFilter(CLUSTERS_META, threshold_km=150.0)
    
    input_file = os.path.join(DATA_BASE_DIR, "worldmove_trajectories.jsonl")
    output_file = os.path.join(OUTPUT_DIR, "cleansed-mobility.jsonl")
    
    if not os.path.exists(input_file):
        print(f"Mobility raw data not found at {input_file}. Implementation ready for data arrival.")
        return

    print("Ingesting WorldMove mobility data (150km filter)...")
    
    processed_count = 0
    kept_count = 0
    
    with open(input_file, 'r') as f, open(output_file, 'w') as out:
        for line in f:
            record = json.loads(line)
            # Expecting 'lat', 'lon' in the trajectory points or centroid
            lat = record.get('lat')
            lon = record.get('lon')
            
            if lat and lon:
                if cf.is_active(lon, lat):
                    out.write(json.dumps(record) + '\n')
                    kept_count += 1
                processed_count += 1
                
    print(f"Processed {processed_count} mobility records. Kept {kept_count} within 150km radius.")

if __name__ == "__main__":
    ingest_mobility()
