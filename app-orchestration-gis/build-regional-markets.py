#!/usr/bin/env python3
"""
build-regional-markets.py — Build regional-markets.json from clusters.geojson.

Groups co-locations by their regional_market slug.  Produces one record per
Regional Market (RM) — the product publishing unit.

Schema (per RM):
    rm_id          slug: rm_{iso}_{settlement}
    market         display name (city + region)
    iso            country code
    continent      NA or EU
    mkt_conf       geocode confidence of the best cluster in this RM
    cluster_count  number of co-locations in this RM
    cluster_ids    list of cluster_id values
    centroid       [lon, lat] mean centroid of member cluster centroids
    best_tier      lowest tier number present (T1 > T2 > T3)
    metro_market   MSA/CMA name if available (nullable)

Reads:  work/clusters.geojson
Writes: deployments/gateway-orchestration-gis-1/www/data/regional-markets.json
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR, WWW_DIR

INPUT_FILE  = WORK_DIR / "clusters.geojson"
OUTPUT_FILE = WWW_DIR / "data" / "regional-markets.json"

CONF_RANK = {"high": 3, "medium": 2, "low": 1, "": 0}


def main():
    print(f"Loading {INPUT_FILE} ...")
    with open(INPUT_FILE) as f:
        geojson = json.load(f)
    features = geojson.get("features", [])
    print(f"  {len(features):,} clusters")

    # Group by regional_market slug
    rm_groups: dict = defaultdict(list)
    for feat in features:
        p = feat["properties"]
        rm_id = p.get("regional_market") or ""
        if rm_id:
            rm_groups[rm_id].append(p)

    records = []
    for rm_id, props_list in rm_groups.items():
        first = props_list[0]

        lons = [feat["geometry"]["coordinates"][0] for feat in features
                if feat["properties"].get("regional_market") == rm_id]
        lats = [feat["geometry"]["coordinates"][1] for feat in features
                if feat["properties"].get("regional_market") == rm_id]
        centroid = [
            round(sum(lons) / len(lons), 5),
            round(sum(lats) / len(lats), 5),
        ]

        best_conf = max(props_list, key=lambda p: CONF_RANK.get(p.get("mkt_conf") or "", 0))
        best_tier = min(int(p.get("tier") or 3) for p in props_list)

        records.append({
            "rm_id":         rm_id,
            "market":        first.get("market_name") or "",
            "iso":           first.get("iso") or "",
            "continent":     first.get("continent") or "",
            "region":        first.get("market_region") or "",
            "mkt_conf":      best_conf.get("mkt_conf") or "low",
            "cluster_count": len(props_list),
            "cluster_ids":   [p["cluster_id"] for p in props_list],
            "centroid":      centroid,
            "best_tier":     best_tier,
            "metro_market":  first.get("metro_market") or "",
        })

    # Sort: best_tier asc, cluster_count desc
    records.sort(key=lambda r: (r["best_tier"], -r["cluster_count"]))

    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_FILE, "w") as f:
        json.dump(records, f, separators=(",", ":"))

    high_conf = sum(1 for r in records if r["mkt_conf"] == "high")
    print(f"Written {len(records)} Regional Markets → {OUTPUT_FILE}")
    print(f"  high-conf: {high_conf}  multi-cluster: {sum(1 for r in records if r['cluster_count'] > 1)}")
    for t in [1, 2, 3]:
        print(f"  T{t} best: {sum(1 for r in records if r['best_tier'] == t)}")


if __name__ == "__main__":
    main()
