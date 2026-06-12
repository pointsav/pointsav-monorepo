#!/usr/bin/env python3
import json, math, sys, os
from pathlib import Path
import collections

# Import configuration
sys.path.insert(0, str(Path(__file__).parent))
from config import (
    SERVICE_BUSINESS_CLEANSED, SERVICE_PLACES_CLEANSED,
    ALPHA_ANCHORS, ALPHA_HARDWARE, ALPHA_WAREHOUSE,
    REGION_CONFIG, SECONDARY_RADIUS_KM, TERTIARY_RADIUS_KM
)

def haversine_km(lat1, lon1, lat2, lon2) -> float:
    R = 6371.0
    phi1, phi2 = math.radians(lat1), math.radians(lat2)
    dphi, dlam = math.radians(lat2 - lat1), math.radians(lon2 - lon1)
    a = math.sin(dphi/2)**2 + math.cos(phi1)*math.cos(phi2)*math.sin(dlam/2)**2
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))

def load_jsonl(path):
    res = []
    if path.exists():
        with open(path) as f:
            for line in f:
                try: res.append(json.loads(line))
                except: pass
    return res

def linear_score(dist_km, max_dist_km):
    if dist_km > max_dist_km: return 0.0
    return max(0.0, (max_dist_km - dist_km) / max_dist_km) * 100.0

def build_grid(records, grid_size=0.1):
    grid = collections.defaultdict(list)
    for r in records:
        lat, lon = float(r["latitude"]), float(r["longitude"])
        grid[(int(lat/grid_size), int(lon/grid_size))].append(r)
    return grid

def query_grid(plat, plon, grid, radius_km, grid_size=0.1):
    res = []
    cell_lat, cell_lon = int(plat/grid_size), int(plon/grid_size)
    for dlat in range(-5, 6):
        for dlon in range(-5, 6):
            for r in grid.get((cell_lat + dlat, cell_lon + dlon), []):
                dist = haversine_km(plat, plon, float(r["latitude"]), float(r["longitude"]))
                if dist <= radius_km: res.append(dist)
    return min(res) if res else 999.0

def main():
    biz = load_jsonl(SERVICE_BUSINESS_CLEANSED)
    places = load_jsonl(SERVICE_PLACES_CLEANSED)

    hw_recs = [r for r in biz if r.get("chain_id") in (ALPHA_HARDWARE["NA"] | ALPHA_HARDWARE["EU"])]
    wh_recs = [r for r in biz if r.get("chain_id") in (ALPHA_WAREHOUSE["NA"] | ALPHA_WAREHOUSE["EU"])]
    
    # Use category_id as discovered in inspection
    hc_recs = [r for r in places if r.get("category_id") == "hospital"]
    he_recs = [r for r in places if r.get("category_id") == "university"]

    print(f"HW:{len(hw_recs)} WH:{len(wh_recs)} HC:{len(hc_recs)} HE:{len(he_recs)}")

    hw_grid = build_grid(hw_recs)
    wh_grid = build_grid(wh_recs)
    hc_grid = build_grid(hc_recs)
    he_grid = build_grid(he_recs)

    results = {"NA": [], "EU": []}

    for r in biz:
        cid = r.get("chain_id")
        cont = "NA" if cid in ALPHA_ANCHORS["NA"] else "EU" if cid in ALPHA_ANCHORS["EU"] else None
        
        if cont:
            plat, plon = float(r["latitude"]), float(r["longitude"])
            d_hw = query_grid(plat, plon, hw_grid, SECONDARY_RADIUS_KM)
            d_wh = query_grid(plat, plon, wh_grid, SECONDARY_RADIUS_KM)
            d_hc = query_grid(plat, plon, hc_grid, TERTIARY_RADIUS_KM)
            d_he = query_grid(plat, plon, he_grid, TERTIARY_RADIUS_KM)
            
            s_hw = linear_score(d_hw, SECONDARY_RADIUS_KM)
            s_wh = linear_score(d_wh, SECONDARY_RADIUS_KM)
            s_hc = linear_score(d_hc, TERTIARY_RADIUS_KM)
            s_he = linear_score(d_he, TERTIARY_RADIUS_KM)
            
            total_score = s_hw + s_wh + s_hc + s_he
            if total_score > 0:
                results[cont].append({
                    "cid": cid, "city": r.get("city", "Unknown"), "score": total_score,
                    "shw": s_hw, "swh": s_wh, "shc": s_hc, "she": s_he
                })

    for cont in ["NA", "EU"]:
        results[cont].sort(key=lambda x: x["score"], reverse=True)
        top = results[cont][:400]
        print(f"\n--- {cont} TOP 400 ---")
        if not top: continue
        print(f"Max: {top[0]['score']:.1f}, Min: {top[-1]['score']:.1f}")
        
        has_all = len([x for x in top if x["shw"]>0 and x["swh"]>0 and x["shc"]>0 and x["she"]>0])
        has_3 = len([x for x in top if len([s for s in [x["shw"], x["swh"], x["shc"], x["she"]] if s>0]) == 3])
        has_2 = len([x for x in top if len([s for s in [x["shw"], x["swh"], x["shc"], x["she"]] if s>0]) == 2])
        
        print(f"Convergence: All-4: {has_all}, 3-of-4: {has_3}, 2-of-4: {has_2}")
        
        print("\nTop 5 Breakdown:")
        for x in top[:5]:
            print(f"- {x['cid']} @ {x['city']}: {x['score']:.1f} (HW:{x['shw']:.0f} WH:{x['swh']:.0f} HC:{x['shc']:.0f} HE:{x['she']:.0f})")

if __name__ == "__main__": main()
