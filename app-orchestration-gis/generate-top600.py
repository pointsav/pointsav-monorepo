#!/usr/bin/env python3
"""
generate-top600.py — Top-600 co-location index from clusters.geojson (§2 schema).

Sorts by two-stage lexicographic rank:
    primary:   dist_rank_in_tier DESC (geometric compactness; 1.0 = tightest)
    secondary: demand_rank_in_tier DESC (catchment population proxy; 1.0 = highest)

Tiers are preserved as-is (T1/T2/T3) — no Apex/Hub/Core/Valid bands.

Produces continent-separate Top-600 slices:
    NA: continent == "NA"
    EU: continent == "EU"

Each record carries the Regional Market column so downstream editorial
can group by RM when building the Top-600 wiki pages.

Reads:  work/clusters.geojson
Writes: work/top600_final.json
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR

INPUT_FILE  = WORK_DIR / "clusters.geojson"
OUTPUT_FILE = WORK_DIR / "top600_final.json"

TOP_N = 600


def main():
    print(f"Loading {INPUT_FILE} ...")
    with open(INPUT_FILE) as f:
        fc = json.load(f)
    features = fc.get("features", [])
    print(f"  {len(features):,} clusters")

    by_continent: dict = {"NA": [], "EU": []}

    for feat in features:
        p = feat.get("properties", {})
        coords = feat.get("geometry", {}).get("coordinates", [])
        if not coords or len(coords) < 2:
            continue
        cont = p.get("continent") or ""
        if cont not in by_continent:
            continue
        tier = int(p.get("tier") or 0)
        if tier == 0:
            continue
        by_continent[cont].append({
            "id":     p.get("cluster_id") or "",
            "lon":    round(float(coords[0]), 5),
            "lat":    round(float(coords[1]), 5),
            "tier":   tier,
            "td":     p.get("tier_descriptor") or "",
            "span":   round(float(p.get("span_km") or 0.0), 3),
            "dr":     float(p.get("dist_rank_in_tier") or 0.0),
            "dp":     int(p.get("dist_pctile") or 0),
            "dmr":    float(p.get("demand_rank_in_tier") or 0.5),
            "dmb":    p.get("demand_basis") or "interim-none",
            "rm":     p.get("regional_market") or "",
            "mkt":    p.get("market_name") or "",
            "mrgn":   p.get("market_region") or "",
            "metro":  p.get("metro_market") or "",
            "conf":   p.get("mkt_conf") or "low",
            "iso":    p.get("iso") or "",
            "mc":     int(p.get("member_count") or 0),
        })

    result = {}
    for cont, sites in by_continent.items():
        # Two-stage sort: tier ASC, then dist_rank DESC, then demand_rank DESC
        sites.sort(key=lambda x: (x["tier"], -x["dr"], -x["dmr"]))
        top = sites[:TOP_N]
        for i, s in enumerate(top):
            s["rank"] = i + 1
        result[cont] = top
        print(f"  {cont}: {len(top)} records "
              f"(T1={sum(1 for s in top if s['tier']==1)} "
              f"T2={sum(1 for s in top if s['tier']==2)} "
              f"T3={sum(1 for s in top if s['tier']==3)})")

    WORK_DIR.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_FILE, "w") as f:
        json.dump(result, f, separators=(",", ":"))
    print(f"Written {OUTPUT_FILE}")


if __name__ == "__main__":
    main()
