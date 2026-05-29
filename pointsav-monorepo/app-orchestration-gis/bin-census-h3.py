import os
import json
import h3
from collections import defaultdict

# Configuration
INPUT_FILE = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/cleansed-census-1km-ALL.jsonl"
OUTPUT_FILE = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/census-h3-res7.jsonl"
H3_RESOLUTION = 7

def bin_census_h3():
    print(f"Binning census data into H3 Resolution {H3_RESOLUTION}...")
    
    # We'll use a dictionary to aggregate population by H3 cell
    # Key: H3 index, Value: {pop: sum, iso: set of countries contributing}
    hex_data = defaultdict(lambda: {'pop': 0.0, 'iso': set()})
    
    if not os.path.exists(INPUT_FILE):
        print(f"Input file not found: {INPUT_FILE}")
        return

    processed_count = 0
    with open(INPUT_FILE, 'r') as f:
        for line in f:
            record = json.loads(line)
            lat = record['lat']
            lon = record['lon']
            pop = record['pop']
            iso = record.get('iso', 'UNKNOWN')
            
            # Get H3 index for the coordinates
            h3_index = h3.latlng_to_cell(lat, lon, H3_RESOLUTION)
            
            hex_data[h3_index]['pop'] += pop
            hex_data[h3_index]['iso'].add(iso)
            
            processed_count += 1
            if processed_count % 1000000 == 0:
                print(f"  Processed {processed_count} records...")

    print(f"Total points processed: {processed_count}")
    print(f"Total unique H3 cells (Res {H3_RESOLUTION}): {len(hex_data)}")

    print(f"Writing aggregated hexagonal data to {OUTPUT_FILE}...")
    with open(OUTPUT_FILE, 'w') as out:
        for h3_index, data in hex_data.items():
            # Get center coords for the hexagon for reference/visualization
            lat, lon = h3.cell_to_latlng(h3_index)
            
            out.write(json.dumps({
                'h3': h3_index,
                'lat': round(lat, 5),
                'lon': round(lon, 5),
                'pop': round(data['pop'], 2),
                'iso': list(data['iso'])
            }) + '\n')

    print("Hexagonal binning complete.")

if __name__ == "__main__":
    bin_census_h3()
