import json
import math
from pathlib import Path

# Paths
TOTEBOX_DATA_PATH = Path("/srv/foundry/deployments/cluster-totebox-personnel-1/data")
SERVICE_BUSINESS = TOTEBOX_DATA_PATH / "service-business"
SERVICE_PLACES = TOTEBOX_DATA_PATH / "service-places"

# Define the Alpha Brands per region
ALPHA_ANCHORS = {
    "NA": {"walmart-us", "walmart-ca", "walmart-mx", "ikea-us", "ikea-ca", "ikea-mx"},
    "EU": {"ikea-es", "ikea-it", "ikea-gr", "ikea-pl", "ikea-nordics",
           "carrefour-hypermarket-es", "carrefour-hypermarket-it", "carrefour-hypermarket-pl",
           "alcampo-es", "leclerc-es", "ipercoop-it", "iper-it", "bennet-it", "leclerc-pl", "auchan-pl",
           "bilka-dk", "prisma-fi", "k-citymarket-fi", "obs-coop-no", "hagkaup-is"}
}

ALPHA_HARDWARE = {
    "NA": {"home-depot-us", "home-depot-ca", "home-depot-mx"},
    "EU": {"leroy-merlin-es", "leroy-merlin-it", "leroy-merlin-gr", "leroy-merlin-pl", "clas-ohlson-se", "brico-depot-es"}
}

ALPHA_WAREHOUSE = {
    "NA": {"costco-us", "costco-ca", "costco-mx"},
    "EU": {"costco-es", "costco-se", "costco-is", "makro-es", "makro-pl"} 
}

# Generic category mapping
GENERIC_WAREHOUSE = {"sams-club-us", "sams-club-mx", "bjs-wholesale-us", "metro-it", "the-mart-gr"}
GENERIC_HARDWARE = {"lowes-us", "lowes-ca", "canadian-tire-ca", "peavey-mart-ca", "imerco-dk", "k-rauta-fi", "obs-bygg-no", "husasmidjan-is"}
GENERIC_FOOD = {"lidl-es", "mercadona-es", "safeway-ca", "whole-foods-us", "soriana-mx", "biedronka-pl", "real-canadian-superstore-ca", "target-us"}

# Config from gateway
REGION_CONFIG = {
    "US": {"anchor": ["walmart-us", "ikea-us", "target-us"], "hardware": ["home-depot-us", "lowes-us"], "warehouse": ["costco-us", "sams-club-us", "bjs-wholesale-us"]},
    "CA": {"anchor": ["walmart-ca", "ikea-ca", "real-canadian-superstore-ca"], "hardware": ["home-depot-ca", "lowes-ca", "canadian-tire-ca"], "warehouse": ["costco-ca"]},
    "MX": {"anchor": ["walmart-mx", "ikea-mx"], "hardware": ["home-depot-mx"], "warehouse": ["costco-mx", "sams-club-mx"]},
    "ES": {"anchor": ["ikea-es", "carrefour-hypermarket-es", "alcampo-es", "leclerc-es"], "hardware": ["leroy-merlin-es", "brico-depot-es"], "warehouse": ["costco-es", "makro-es"]},
    "IT": {"anchor": ["ikea-it", "carrefour-hypermarket-it", "ipercoop-it", "iper-it", "bennet-it"], "hardware": ["leroy-merlin-it"], "warehouse": ["metro-it"]},
    "GR": {"anchor": ["ikea-gr"], "hardware": ["leroy-merlin-gr"], "warehouse": ["the-mart-gr"]},
    "PL": {"anchor": ["ikea-pl", "carrefour-hypermarket-pl", "leclerc-pl", "auchan-pl"], "hardware": ["leroy-merlin-pl"], "warehouse": ["makro-pl"]},
    "NORDICS": {"anchor": ["ikea-nordics", "bilka-dk", "prisma-fi", "k-citymarket-fi", "obs-coop-no", "hagkaup-is"], "hardware": ["clas-ohlson-se", "k-rauta-fi", "imerco-dk", "obs-bygg-no", "husasmidjan-is"], "warehouse": ["costco-se", "costco-is"]},
}

def haversine_km(lat1, lon1, lat2, lon2):
    R = 6371.0
    phi1, phi2 = math.radians(lat1), math.radians(lat2)
    dphi, dlam = math.radians(lat2 - lat1), math.radians(lon2 - lon1)
    a = math.sin(dphi/2)**2 + math.cos(phi1)*math.cos(phi2)*math.sin(dlam/2)**2
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))

def load_jsonl(path):
    res = []
    if not path.exists(): return res
    with open(path) as f:
        for line in f:
            try:
                rec = json.loads(line)
                if rec.get("latitude") and rec.get("longitude"): res.append(rec)
            except: pass
    return res

def build_grid(recs, size=0.1):
    g = {}
    for r in recs:
        cell = (int(float(r["latitude"])/size), int(float(r["longitude"])/size))
        g.setdefault(cell, []).append(r)
    return g

def query_grid(plat, plon, g, r_km, size=0.1):
    res = []
    deg = (r_km + 0.5) / 111.0
    for lat_c in range(int((plat-deg)/size), int((plat+deg)/size)+1):
        for lon_c in range(int((plon-deg)/size), int((plon+deg)/size)+1):
            for r in g.get((lat_c, lon_c), []):
                if haversine_km(plat, plon, float(r["latitude"]), float(r["longitude"])) <= r_km: res.append(r)
    return res

def evaluate_tier(anchor_id, nhw, nwh, nhc, nhe, cont):
    has_alpha_anchor = anchor_id in ALPHA_ANCHORS[cont]
    has_alpha_hw = any(r["chain_id"] in ALPHA_HARDWARE[cont] for r in nhw)
    has_alpha_wh = any(r["chain_id"] in ALPHA_WAREHOUSE[cont] for r in nwh)
    
    has_gen_hw = len(nhw) > 0  # Broadened: ANY hardware counts as generic if not Alpha
    has_gen_wh = len(nwh) > 0  # Broadened: ANY warehouse counts as generic if not Alpha
    has_civic = len(nhc) > 0 and len(nhe) > 0
    has_med_or_edu = len(nhc) > 0 or len(nhe) > 0

    # Apex: Need Alpha Anchor + ANY Hardware + ANY Warehouse + Civic
    # We relax the strict "Alpha HW/WH" requirement at the top tier because Europe's hardware/warehouse density is different.
    if has_alpha_anchor and has_gen_hw and has_gen_wh and has_civic:
        return 4
        
    # Tier 3: Alpha Anchor + Generic HW + Generic WH (No Civic)
    if has_alpha_anchor and has_gen_hw and has_gen_wh:
        return 3
        
    # Tier 2: Anchor Inversion (Generic Food primary + Alpha WH + Gen HW)
    # OR Alpha Anchor + HW + Medical/Edu
    if (anchor_id in GENERIC_FOOD and has_alpha_wh and has_gen_hw) or \
       (has_alpha_anchor and has_gen_hw and has_med_or_edu):
        return 2
    
    # Tier 1: Baseline. Must have at least ONE Alpha of any kind.
    if has_alpha_anchor or has_alpha_hw or has_alpha_wh:
        # Only return Tier 1 if we actually have some co-location (e.g. at least a hardware store or medical)
        # We don't want a lone anchor with literally nothing else showing up.
        if has_gen_hw or has_gen_wh or has_med_or_edu:
            return 1
        
    return None

def run_test(sec_r):
    ids = set()
    for roles in REGION_CONFIG.values():
        for lst in roles.values():
            ids.update(lst)
    
    locs = {cid: load_jsonl(SERVICE_BUSINESS / "locations" / f"{cid}.jsonl") for cid in ids}
    hc_g = build_grid(load_jsonl(SERVICE_PLACES / "locations" / "hospital.jsonl"))
    he_g = build_grid(load_jsonl(SERVICE_PLACES / "locations" / "university.jsonl"))
    
    counts = {"NA": {1:0, 2:0, 3:0, 4:0}, "EU": {1:0, 2:0, 3:0, 4:0}}
    
    for r_key, r_roles in REGION_CONFIG.items():
        cont = "NA" if r_key in ["US", "CA", "MX"] else "EU"
        hw_recs = [r for cid in r_roles.get("hardware", []) for r in locs.get(cid, [])]
        wh_recs = [r for cid in r_roles.get("warehouse", []) for r in locs.get(cid, [])]
        hw_g, wh_g = build_grid(hw_recs), build_grid(wh_recs)
        
        for anchor_cid in r_roles.get("anchor", []):
            for pri in locs.get(anchor_cid, []):
                plat, plon = float(pri["latitude"]), float(pri["longitude"])
                nhw = query_grid(plat, plon, hw_g, sec_r)
                nwh = query_grid(plat, plon, wh_g, sec_r)
                nhc = query_grid(plat, plon, hc_g, 5.0)
                nhe = query_grid(plat, plon, he_g, 5.0)
                
                tier = evaluate_tier(anchor_cid, nhw, nwh, nhc, nhe, cont)
                if tier:
                    counts[cont][tier] += 1
                    
    return counts

print(f"{'REGION':<10} | {'T1':<6} | {'T2':<6} | {'T3':<6} | {'T4':<6} | {'TOTAL':<6}")
print("-" * 55)

for r in [1.0, 2.0, 3.0]:
    print(f"\nTHRESHOLD: {r}km")
    counts = run_test(r)
    for reg in ["NA", "EU"]:
        c = counts[reg]
        total = sum(c.values())
        print(f"{reg:<10} | {c[1]:<6} | {c[2]:<6} | {c[3]:<6} | {c[4]:<6} | {total:<6}")
