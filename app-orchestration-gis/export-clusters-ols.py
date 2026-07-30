#!/usr/bin/env python3
"""
export-clusters-ols.py — Export Phase 22 cluster data as CSV for §7.2 OLS regression.

Reads clusters-meta.json (deployed gateway copy).
Writes:
  work/clusters-ols.csv       — all 6,493 clusters
  work/clusters-ols-na.csv    — NA subset (US/CA/MX)
  work/clusters-ols-eu.csv    — EU subset

Fields:
  cluster_id, tier, tier_label, t1_dummy, t2_dummy, span_km, tight,
  country, continent, lat, lon, member_count,
  has_hypermarket, has_hardware, has_price_club, has_lifestyle,
  has_electronics, has_sport, anchor_composition,
  regional_market, metro_region, ashrae_zone
"""
import csv
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import TOTEBOX_DATA_PATH, WORK_DIR

SOURCE = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")

FIELDNAMES = [
    "cluster_id", "tier", "tier_label", "t1_dummy", "t2_dummy",
    "span_km", "tight", "country", "continent", "lat", "lon", "member_count",
    "has_hypermarket", "has_hardware", "has_price_club", "has_lifestyle",
    "has_electronics", "has_sport", "anchor_composition",
    "regional_market", "metro_region", "ashrae_zone",
]

ANCHOR_CATS = {"hypermarket", "hardware", "price_club", "lifestyle", "electronics", "sport"}


def parse_cluster(c: dict) -> dict:
    tier = int(c.get("t", 0))
    members = c.get("members", [])
    cats = {m.get("category", "") for m in members} & ANCHOR_CATS
    return {
        "cluster_id":        c.get("id", ""),
        "tier":              tier,
        "tier_label":        {1: "T1", 2: "T2", 3: "T3"}.get(tier, f"T{tier}"),
        "t1_dummy":          1 if tier == 1 else 0,
        "t2_dummy":          1 if tier == 2 else 0,
        "span_km":           c.get("span", ""),
        "tight":             c.get("tight", 0),
        "country":           c.get("iso", ""),
        "continent":         c.get("cont", ""),
        "lat":               c.get("lat", ""),
        "lon":               c.get("lon", ""),
        "member_count":      c.get("mc", len(members)),
        "has_hypermarket":   1 if "hypermarket"  in cats else 0,
        "has_hardware":      1 if "hardware"     in cats else 0,
        "has_price_club":    1 if "price_club"   in cats else 0,
        "has_lifestyle":     1 if "lifestyle"    in cats else 0,
        "has_electronics":   1 if "electronics"  in cats else 0,
        "has_sport":         1 if "sport"        in cats else 0,
        "anchor_composition": ",".join(sorted(cats)),
        "regional_market":   c.get("rm", ""),
        "metro_region":      c.get("mrgn", ""),
        "ashrae_zone":       c.get("ashrae_zone", ""),
    }


def write_csv(path: Path, rows: list[dict]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=FIELDNAMES)
        w.writeheader()
        w.writerows(rows)
    print(f"  → {path} ({len(rows):,} rows)")


def main() -> None:
    print(f"Loading {SOURCE} ...")
    with open(SOURCE) as f:
        raw = json.load(f)
    clusters = raw if isinstance(raw, list) else list(raw.values())
    print(f"  {len(clusters):,} clusters")

    rows = [parse_cluster(c) for c in clusters]

    write_csv(WORK_DIR / "clusters-ols.csv",    rows)
    write_csv(WORK_DIR / "clusters-ols-na.csv", [r for r in rows if r["continent"] == "NA"])
    write_csv(WORK_DIR / "clusters-ols-eu.csv", [r for r in rows if r["continent"] == "EU"])

    # Quick summary
    from collections import Counter
    tier_region = Counter((r["tier_label"], r["continent"]) for r in rows)
    print("\nTier × Region:")
    for (tier, reg), n in sorted(tier_region.items()):
        print(f"  {tier} {reg}: {n:,}")


if __name__ == "__main__":
    main()
