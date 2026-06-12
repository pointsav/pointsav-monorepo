import json
import math
from pathlib import Path

TOTEBOX_DATA_PATH = Path("/srv/foundry/deployments/cluster-totebox-personnel-1/data")
SERVICE_BUSINESS = TOTEBOX_DATA_PATH / "service-business"
SERVICE_PLACES = TOTEBOX_DATA_PATH / "service-places"

ALPHA_ANCHORS = {
    "NA": {"walmart-us", "walmart-ca", "walmart-mx", "ikea-us", "ikea-ca", "ikea-mx"},
    "EU": {"ikea-es", "ikea-it", "ikea-gr", "ikea-pl", "ikea-nordics",
           "carrefour-hypermarket-es", "carrefour-hypermarket-it", "carrefour-hypermarket-pl",
           "alcampo-es", "leclerc-es", "ipercoop-it", "iper-it", "bennet-it", "leclerc-pl", "auchan-pl",
           "bilka-dk", "prisma-fi", "k-citymarket-fi", "obs-coop-no", "hagkaup-is"}
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
    has_hw = len(nhw) > 0
    has_wh = len(nwh) > 0
    has_hc = len(nhc) > 0
    has_he = len(nhe) > 0

    if not has_alpha_anchor: return None

    # T4: Anchor + (HW or WH) + (Hospital AND University)
    # T3: Anchor + (HW or WH) + (Hospital OR University)
    # T2: Anchor + HW + WH (No Civic) 
    # T1: Anchor + (HW or WH)
    
    if (has_hw or has_wh) and has_hc and has_he: return 4
    if (has_hw or has_wh) and (has_hc or has_he): return 3
    if has_hw and has_wh: return 2
    if has_hw or has_wh: return 1

    return None

import sys
sys.path.insert(0, "/srv/foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis")
from config import REGION_CONFIG

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
