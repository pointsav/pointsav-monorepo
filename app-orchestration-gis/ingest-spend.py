import os
import json
import h3
from pathlib import Path

# Configuration
# Use the H3-based census grid we just ingested
H3_CENSUS_FILE = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/census-h3-res7.jsonl"
OUTPUT_DIR = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-spend/"

# Regional Expenditure Multipliers (Annual per capita estimated)
# Best Practice: Keep these decoupled from spatial logic for easier maintenance
SPEND_MULTIPLIERS = {
    "USA": {"grocery": 3500, "hardware": 1200, "wholesale": 1500, "currency": "USD"},
    "CAN": {"grocery": 3200, "hardware": 1100, "wholesale": 1300, "currency": "CAD"},
    "MEX": {"grocery": 18000, "hardware": 3500, "wholesale": 2500, "currency": "MXN"},
    "GBR": {"grocery": 2800, "hardware": 850, "wholesale": 900, "currency": "GBP"},
    "DEU": {"grocery": 2900, "hardware": 950, "wholesale": 1000, "currency": "EUR"},
    "FRA": {"grocery": 3100, "hardware": 900, "wholesale": 1000, "currency": "EUR"},
    "NLD": {"grocery": 2700, "hardware": 1000, "wholesale": 1100, "currency": "EUR"},
    "AUT": {"grocery": 3000, "hardware": 950, "wholesale": 1000, "currency": "EUR"},
    "PRT": {"grocery": 2400, "hardware": 600, "wholesale": 700, "currency": "EUR"},
    "GRC": {"grocery": 2200, "hardware": 500, "wholesale": 600, "currency": "EUR"},
    "DNK": {"grocery": 3500, "hardware": 1200, "wholesale": 1100, "currency": "EUR"},
    "ISL": {"grocery": 4000, "hardware": 1500, "wholesale": 1500, "currency": "EUR"},
    "POL": {"grocery": 8000, "hardware": 2000, "wholesale": 2500, "currency": "PLN"},
}

def synthesize_spend_h3():
    print("Synthesizing spend data from H3 census grids...")
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    
    # Validation counters
    total_pop_input = 0.0
    total_spend_output = 0.0
    
    output_file = os.path.join(OUTPUT_DIR, "cleansed-spend-h3-res7.jsonl")
    
    with open(H3_CENSUS_FILE, 'r') as f_in, open(output_file, 'w') as f_out:
        for line in f_in:
            record = json.loads(line)
            h3_idx = record['h3']
            pop = float(record['pop'])
            # H3 census data stores ISO as a list like ["USA"]
            iso_list = record.get('iso', ["USA"])
            iso = iso_list[0] if isinstance(iso_list, list) else iso_list
            
            multipliers = SPEND_MULTIPLIERS.get(iso)
            if not multipliers:
                continue
            
            # Synthesize Spend
            spend_record = {
                'h3': h3_idx,
                'pop': pop,
                'spend_grocery': round(pop * multipliers['grocery'], 2),
                'spend_hardware': round(pop * multipliers['hardware'], 2),
                'spend_wholesale': round(pop * multipliers['wholesale'], 2),
                'currency': multipliers['currency']
            }
            
            f_out.write(json.dumps(spend_record) + '\n')
            
            total_pop_input += pop
            total_spend_output += spend_record['spend_grocery'] + \
                                  spend_record['spend_hardware'] + \
                                  spend_record['spend_wholesale']
    
    print(f"Synthesis complete.")
    print(f"Total Population processed: {total_pop_input:,.0f}")
    print(f"Output saved to: {output_file}")

if __name__ == "__main__":
    synthesize_spend_h3()
