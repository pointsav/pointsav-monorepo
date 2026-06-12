#!/usr/bin/env python3
"""
build-geometric-ranking.py — Stage 1: geometric compactness rank.

Reads clusters.geojson (produced by build-clusters.py).
Tier is already set — this script only adds:
    dist_rank_in_tier   inverted span_km percentile within (tier, ISO)
    dist_pctile         integer 0–100 (100 = tightest in tier-ISO pool)

Shrinkage blending: if a (tier, ISO) pool has fewer than MIN_POOL clusters,
blend toward the (tier, continent) pool using a linear weight.
    alpha = min(1.0, pool_size / MIN_POOL)
    dist_rank = alpha * country_rank + (1 - alpha) * continent_rank

Writes: work/clusters.geojson (in-place, adds dist_rank_in_tier + dist_pctile)
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR
from taxonomy import ISO_TO_CONTINENT

INPUT_FILE  = WORK_DIR / "clusters.geojson"
OUTPUT_FILE = WORK_DIR / "clusters.geojson"

MIN_POOL = 20   # blend toward continent when country pool < 20 per tier


def _inverted_pctile(ranks: list[float]) -> dict[int, float]:
    """For a sorted list of (idx, span_km), return {idx: inverted_pctile}.
    Smallest span_km → rank 1.0 (tightest); largest → rank 0.0."""
    indexed = sorted(enumerate(ranks), key=lambda x: x[1])
    m = len(indexed)
    result = {}
    for pos, (orig_idx, _) in enumerate(indexed):
        frac = pos / (m - 1) if m > 1 else 0.0   # 0 = tightest
        result[orig_idx] = round(1.0 - frac, 4)   # invert: 1.0 = tightest
    return result


def main():
    print(f"Loading {INPUT_FILE} ...")
    with open(INPUT_FILE) as f:
        geojson = json.load(f)
    features = geojson.get("features", [])
    print(f"  {len(features):,} clusters")

    # Group by (tier, iso) and (tier, continent) for shrinkage
    by_tier_iso: dict = defaultdict(list)
    by_tier_cont: dict = defaultdict(list)
    for i, feat in enumerate(features):
        p = feat["properties"]
        tier = int(p.get("tier") or 0)
        iso  = p.get("iso") or ""
        cont = ISO_TO_CONTINENT.get(iso, "?")
        span = float(p.get("span_km") or 0.0)
        by_tier_iso[(tier, iso)].append((i, span))
        by_tier_cont[(tier, cont)].append((i, span))

    # Compute country-level and continent-level inverted percentiles
    iso_rank:  dict[int, float] = {}
    cont_rank: dict[int, float] = {}

    for (tier, iso), items in by_tier_iso.items():
        indices = [x[0] for x in items]
        spans   = [x[1] for x in items]
        pctiles = _inverted_pctile(spans)
        for pos, orig_idx in enumerate(indices):
            iso_rank[orig_idx] = pctiles[pos]

    for (tier, cont), items in by_tier_cont.items():
        indices = [x[0] for x in items]
        spans   = [x[1] for x in items]
        pctiles = _inverted_pctile(spans)
        for pos, orig_idx in enumerate(indices):
            cont_rank[orig_idx] = pctiles[pos]

    # Shrinkage blend
    applied = 0
    for i, feat in enumerate(features):
        p = feat["properties"]
        tier = int(p.get("tier") or 0)
        iso  = p.get("iso") or ""

        pool_size = len(by_tier_iso.get((tier, iso), []))
        alpha = min(1.0, pool_size / MIN_POOL)

        ir = iso_rank.get(i, 0.5)
        cr = cont_rank.get(i, 0.5)
        blended = alpha * ir + (1.0 - alpha) * cr

        p["dist_rank_in_tier"] = round(blended, 4)
        p["dist_pctile"]       = int(round(blended * 100))
        applied += 1

    print(f"  Set dist_rank_in_tier for {applied:,} clusters (MIN_POOL={MIN_POOL})")

    # Tier distribution
    tier_counts: dict = defaultdict(int)
    for f in features:
        tier_counts[f["properties"].get("tier", 0)] += 1
    for t in sorted(tier_counts):
        print(f"  T{t}: {tier_counts[t]}")

    print(f"Writing {OUTPUT_FILE} ...")
    with open(OUTPUT_FILE, "w") as f:
        json.dump(geojson, f)
    print("  Done.")


if __name__ == "__main__":
    main()
