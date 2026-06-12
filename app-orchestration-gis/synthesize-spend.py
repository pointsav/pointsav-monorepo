import json
import os

# Configuration
CENSUS_DATA_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/"
SPEND_OUTPUT_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-spend/"

# Regional Spend Averages (proxy multipliers per capita)
# These would be derived from actual survey data (e.g. BLS / StatCan)
SPEND_MULTIPLIERS = {
    "USA": {"grocery": 1200, "hardware": 450, "wholesale": 800},
    "CAN": {"grocery": 1350, "hardware": 400, "wholesale": 750},
    "GBR": {"grocery": 1100, "hardware": 350, "wholesale": 600}
}

def synthesize_spend(iso):
    input_file = os.path.join(CENSUS_DATA_DIR, f"full-census-{iso}.jsonl")
    output_file = os.path.join(SPEND_OUTPUT_DIR, f"cleansed-spend-{iso}.jsonl")
    
    if not os.path.exists(input_file):
        print(f"Census data not found for {iso}. Skipping.")
        return

    print(f"Synthesizing spend for {iso}...")
    multipliers = SPEND_MULTIPLIERS.get(iso, {"grocery": 1000, "hardware": 300, "wholesale": 500})
    
    with open(input_file, 'r') as f, open(output_file, 'w') as out:
        for line in f:
            record = json.loads(line)
            pop = float(record.get('pop_count', 0))
            
            # Simple synthesis logic: pop * multiplier
            spend_record = {
                'grid_id': record.get('grid_id'),
                'spend_grocery': pop * multipliers['grocery'],
                'spend_hardware': pop * multipliers['hardware'],
                'spend_wholesale': pop * multipliers['wholesale']
            }
            out.write(json.dumps(spend_record) + '\n')
    print(f"  Synthesized spend for {iso} complete.")

if __name__ == "__main__":
    for iso in SPEND_MULTIPLIERS.keys():
        synthesize_spend(iso)
