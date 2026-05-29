import os
import json
import h3
from collections import defaultdict

# Configuration
CLUSTERS_META = "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
H3_DATA_FILE = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/census-h3-res7.jsonl"
OUTPUT_REPORT = "trade-area-conflict-report.json"
H3_RESOLUTION = 7

# At Res 7, center-to-center is approx 2.11km
HEX_RAD_75KM = 36
HEX_RAD_150KM = 72

def verify_data_radius():
    print("Loading hexagonal census data...")
    hex_pops = {}
    with open(H3_DATA_FILE, 'r') as f:
        for line in f:
            record = json.loads(line)
            hex_pops[record['h3']] = record['pop']
    print(f"Loaded {len(hex_pops)} H3 cells.")

    print("Loading cluster centroids...")
    with open(CLUSTERS_META, 'r') as f:
        clusters = json.load(f)
    print(f"Loaded {len(clusters)} clusters.")

    report = []
    
    print(f"Analyzing trade area conflicts using H3 grid (disk radii: {HEX_RAD_75KM}, {HEX_RAD_150KM})...")
    for idx, cluster in enumerate(clusters):
        c_id = cluster.get('id', f"cluster_{idx}")
        c_lat = cluster['lat']
        c_lon = cluster['lon']
        
        c_hex = h3.latlng_to_cell(c_lat, c_lon, H3_RESOLUTION)
        
        # Get hexes in 75km disk
        inner_hexes = set(h3.grid_disk(c_hex, HEX_RAD_75KM))
        # Get hexes in 150km disk
        regional_hexes = set(h3.grid_disk(c_hex, HEX_RAD_150KM))
        
        # Population within 75km
        pop_75km = sum(hex_pops.get(h, 0) for h in inner_hexes)
        
        # Population within 150km (regional)
        # We only need to sum the difference
        outer_hexes = regional_hexes - inner_hexes
        pop_outer = sum(hex_pops.get(h, 0) for h in outer_hexes)
        pop_150km = pop_75km + pop_outer
        
        # Calculate conflict metrics
        if pop_150km > 0:
            ratio_75_to_150 = pop_75km / pop_150km
            diff_pop = pop_outer
            
            # If more than 25% of regional population is outside the 75km limit, flag it
            if ratio_75_to_150 < 0.75 and diff_pop > 100000: # Slightly higher threshold for "significant"
                report.append({
                    'id': c_id,
                    'lat': round(c_lat, 4),
                    'lon': round(c_lon, 4),
                    'pop_75km': int(pop_75km),
                    'pop_150km': int(pop_150km),
                    'clipping_ratio': round(1 - ratio_75_to_150, 3),
                    'clipped_pop': int(diff_pop)
                })
        
        if (idx + 1) % 500 == 0:
            print(f"  Analyzed {idx + 1} clusters...")

    print(f"Analysis complete. Found {len(report)} clusters with significant 75km clipping.")
    
    # Sort by clipped population
    report.sort(key=lambda x: x['clipped_pop'], reverse=True)
    
    with open(OUTPUT_REPORT, 'w') as out:
        json.dump(report, out, indent=2)
    
    print(f"Report saved to {OUTPUT_REPORT}.")

if __name__ == "__main__":
    verify_data_radius()
