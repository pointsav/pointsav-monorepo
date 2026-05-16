#!/usr/bin/env python3
"""
generate-rankings.py — Enhanced 0-1000 co-tenancy scoring + multi-dimensional rankings.

Reads clusters.geojson produced by build-clusters.py and writes
clusters-ranked.geojson with new scoring and ranking fields.

Scoring formula V2 (see SCORING-METHODOLOGY.md):
    score_base            = (s_hw + s_wh + s_hc + s_he) × 1.5      [max ~600]
    score_count_bonus     = min(75, 15 × extra_brands_beyond_first)
    score_diversity_bonus = Shannon_entropy(chains) × 50            [max ~50]
    score_multi_anchor    = step(3+ chains=25, 2+ chains=15, else 0)
    score_civic_depth     = log(hc+he+1) × 10 if proximity>20 else 0
    score_overlap_penalty = -min(75, 15 × overlapping_higher_clusters)
    score_final           = sum of above, rounded to int

Tier thresholds (score-driven, not binary gate):
    T3 Apex:    score_final >= 700
    T2 Hub:     score_final >= 450
    T1 Valid:   score_final >= 150
    T0 Border:  score_final < 150

Four rank dimensions per cluster:
    national_rank / national_rank_of    — within ISO by score_final
    rank_national_pct                   — percentile (100=best, 1=worst)
    iso_market_rank / iso_market_rank_of — within US/CA/MX state; or country for EU
    rank_in_tier / rank_in_tier_of      — within tier + ISO
    percentile                          — global across all clusters

Country saturation guard: if any ISO would have >5% T3, raise threshold
until <=5% (prevents geographic concentration of apex tier).

Usage:
    python3 generate-rankings.py [--dry-run] [--threshold-t3 700] [--threshold-t2 450]
"""

import argparse
import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR

INPUT_FILE  = WORK_DIR / "clusters.geojson"
OUTPUT_FILE = WORK_DIR / "clusters.geojson"   # in-place update

DEFAULT_T3 = 625
DEFAULT_T2 = 400
DEFAULT_T1 = 150
MAX_T3_PCT = 0.05   # country saturation guard: no more than 5% T3 per ISO


# ── Scoring helpers ──────────────────────────────────────────────────────────

def _shannon_entropy(chain_ids: list[str]) -> float:
    """Shannon entropy of chain_id distribution in this cluster (0 = single chain)."""
    if not chain_ids:
        return 0.0
    counts = Counter(chain_ids)
    total = len(chain_ids)
    return -sum((c / total) * math.log2(c / total) for c in counts.values())


def _parse_list(val) -> list:
    """Parse a JSON-encoded list or return empty list."""
    if isinstance(val, list):
        return val
    if isinstance(val, str):
        try:
            return json.loads(val)
        except (json.JSONDecodeError, ValueError):
            return []
    return []


def _parse_details(val) -> list[dict]:
    """Parse anchor_details JSON into list of dicts."""
    raw = _parse_list(val)
    return [d for d in raw if isinstance(d, dict)]


def score_cluster(p: dict) -> dict:
    """
    Compute V2 score for a cluster feature properties dict.
    Returns a dict of new scoring fields to merge in.
    """
    s_hw = float(p.get("score_hw", 0) or 0)
    s_wh = float(p.get("score_wh", 0) or 0)
    s_hc = float(p.get("score_hc", 0) or 0)
    s_he = float(p.get("score_he", 0) or 0)
    hc_count = int(p.get("hc_count", 0) or 0)
    he_count = int(p.get("he_count", 0) or 0)

    hw_ids = _parse_list(p.get("hw_list", "[]"))
    wh_ids = _parse_list(p.get("wh_list", "[]"))
    all_chain_ids = hw_ids + wh_ids

    # Component 1: Base score (scaled existing proximity scores)
    score_base = (s_hw + s_wh + s_hc + s_he) * 1.5

    # Component 2: Count bonus (rewards multiple distinct brands)
    unique_brands = len(set(all_chain_ids))
    extra_brands  = max(0, unique_brands - 1)
    score_count_bonus = min(75.0, 15.0 * extra_brands)

    # Component 3: Diversity bonus (Shannon entropy of chain mix)
    entropy = _shannon_entropy(all_chain_ids)
    score_diversity_bonus = entropy * 50.0

    # Component 4: Multi-anchor bonus (step function)
    if unique_brands >= 3:
        score_multi_anchor = 25.0
    elif unique_brands >= 2:
        score_multi_anchor = 15.0
    else:
        score_multi_anchor = 0.0

    # Component 5: Civic depth (log-scaled, gated on minimum proximity)
    civic_total = hc_count + he_count
    has_civic_proximity = (s_hc + s_he) > 20.0
    score_civic_depth = (math.log(civic_total + 1) * 10.0) if has_civic_proximity else 0.0
    score_civic_depth = min(50.0, score_civic_depth)

    # Components 6+7: Regional hospital/university placeholders (activated after Item 1 data)
    score_regional_hospital   = 0.0
    score_regional_university = 0.0

    # Overlap penalty computed in post-pass (set to 0 here; updated below)
    score_overlap_penalty = 0.0

    score_final = (
        score_base
        + score_count_bonus
        + score_diversity_bonus
        + score_multi_anchor
        + score_civic_depth
        + score_regional_hospital
        + score_regional_university
        + score_overlap_penalty
    )

    return {
        "score_final":             round(score_final),
        "score_base":              round(score_base, 1),
        "score_count_bonus":       round(score_count_bonus, 1),
        "score_diversity_bonus":   round(score_diversity_bonus, 1),
        "score_multi_anchor":      round(score_multi_anchor, 1),
        "score_civic_depth":       round(score_civic_depth, 1),
        "score_regional_hospital": round(score_regional_hospital, 1),
        "score_regional_university": round(score_regional_university, 1),
        "score_overlap_penalty":   round(score_overlap_penalty, 1),
        "unique_brands":           int(unique_brands),
    }


def haversine_km(lat1, lon1, lat2, lon2) -> float:
    R = 6371.0
    phi1, phi2 = math.radians(lat1), math.radians(lat2)
    dphi = math.radians(lat2 - lat1)
    dlam = math.radians(lon2 - lon1)
    a = math.sin(dphi / 2) ** 2 + math.cos(phi1) * math.cos(phi2) * math.sin(dlam / 2) ** 2
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def deduplicate_clusters(features: list[dict]) -> list[dict]:
    """
    Remove clusters whose anchor store is within 0.15 km of a higher-count cluster anchor.
    Suppressed clusters' anchor identities are attached to the survivor as merged_zones
    so the BentoBox can show "consolidated within 150 m" transparency.
    """
    DEDUP_KM = 0.15
    ranked = sorted(features, key=lambda f: _rank_key(f["properties"]), reverse=True)
    kept_coords: list[tuple[float, float]] = []
    kept_features: list[dict] = []
    result = []
    for feat in ranked:
        lon, lat = feat["geometry"]["coordinates"][:2]
        survivor_idx = None
        for i, (klat, klon) in enumerate(kept_coords):
            if haversine_km(lat, lon, klat, klon) <= DEDUP_KM:
                survivor_idx = i
                break
        if survivor_idx is not None:
            survivor = kept_features[survivor_idx]
            p = feat["properties"]
            zones = survivor["properties"].setdefault("merged_zones", [])
            zones.append({
                "anchor": p.get("anchor_label") or p.get("primary_anchor") or "",
                "cluster_id": p.get("cluster_id") or "",
            })
            continue
        kept_coords.append((lat, lon))
        kept_features.append(feat)
        result.append(feat)
    return result


def apply_overlap_penalty(features: list[dict], radius_km: float = 75.0) -> None:
    """
    In-place: for each cluster, count how many higher-ranked clusters
    overlap its catchment and subtract the overlap penalty.
    O(N²) but N ≤ ~5000 clusters so acceptable (<2s).
    """
    # Sort descending by score_final so we can count "higher-ranked" easily
    ranked = sorted(features, key=lambda f: f["properties"].get("score_final", 0), reverse=True)
    coords = {
        f["properties"].get("cluster_id"): f["geometry"]["coordinates"]
        for f in features
    }

    for i, feat in enumerate(ranked):
        p = feat["properties"]
        cid = p.get("cluster_id")
        lon, lat = coords.get(cid, [0, 0])
        overlap_count = 0
        for j, other in enumerate(ranked):
            if i == j:
                continue
            if other["properties"].get("score_final", 0) <= p.get("score_final", 0):
                break  # sorted desc — all remaining are lower or equal
            other_cid = other["properties"].get("cluster_id")
            other_lon, other_lat = coords.get(other_cid, [0, 0])
            dist = haversine_km(lat, lon, other_lat, other_lon)
            if dist <= radius_km:
                overlap_count += 1

        penalty = -min(75.0, 15.0 * overlap_count)
        p["score_overlap_penalty"] = round(penalty, 1)
        p["score_final"] = max(0, p["score_final"] + int(penalty))


def assign_tiers(features: list[dict], t3: int, t2: int, t1: int) -> tuple[int, int, int]:
    """Apply score-driven tiers. Saturation guard removed — Phase 2 uses pure-predicate
    geometric engine (build-geometric-ranking.py); top-decile-within-ISO is self-bounding.
    Returns (t3, t2, t1) used (unchanged thresholds)."""
    for f in features:
        p = f["properties"]
        s = p.get("score_final", 0)
        if s >= t3:
            p["rank_v2"] = 3
        elif s >= t2:
            p["rank_v2"] = 2
        elif s >= t1:
            p["rank_v2"] = 1
        else:
            p["rank_v2"] = 0

    return t3, t2, t1


def _rank_key(p: dict) -> tuple:
    """Primary sort key: count_3km desc, tie-break count_1km desc."""
    return (int(p.get("count_3km") or 0), int(p.get("count_1km") or 0))


def assign_rankings(features: list[dict]) -> None:
    """Assign rank dimensions to all clusters (in-place). Ranked by count_3km (pure geometry)."""
    total = len(features)

    # 1. Global percentile (count-based)
    features.sort(key=lambda f: _rank_key(f["properties"]))
    for i, f in enumerate(features):
        f["properties"]["percentile"] = round((i + 1) / total * 100, 1)

    # 2. National rank within ISO — pure geometry: count_3km desc, count_1km as tie-break
    by_iso: dict[str, list] = defaultdict(list)
    for f in features:
        by_iso[f["properties"].get("iso", "XX")].append(f)
    for iso, iso_feats in by_iso.items():
        iso_feats.sort(key=lambda f: _rank_key(f["properties"]), reverse=True)
        n = len(iso_feats)
        for i, f in enumerate(iso_feats):
            p = f["properties"]
            p["national_rank"]     = i + 1
            p["national_rank_of"]  = n
            p["rank_national_pct"] = round((n - i) / n * 100, 1)

    # 3. North America rank (US + CA + MX combined)
    na_isos = {"US", "CA", "MX"}
    na_feats = [f for f in features if f["properties"].get("iso", "") in na_isos]
    na_feats.sort(key=lambda f: _rank_key(f["properties"]), reverse=True)
    na_total = len(na_feats)
    for i, f in enumerate(na_feats):
        f["properties"]["na_rank"]    = i + 1
        f["properties"]["na_rank_of"] = na_total

    # 4. Regional rank (US/CA/MX by state; EU by ISO country)
    def regional_key(p: dict) -> str:
        iso = p.get("iso", "XX")
        if iso in ("US", "CA", "MX"):
            return f"{iso}:{p.get('state', '')}"
        return iso

    by_region: dict[str, list] = defaultdict(list)
    for f in features:
        by_region[regional_key(f["properties"])].append(f)
    for rk, rfeats in by_region.items():
        rfeats.sort(key=lambda f: _rank_key(f["properties"]), reverse=True)
        n = len(rfeats)
        for i, f in enumerate(rfeats):
            p = f["properties"]
            p["iso_market_rank"]    = i + 1
            p["iso_market_rank_of"] = n

    # 5. Rank within tier + ISO
    by_tier_iso: dict[str, list] = defaultdict(list)
    for f in features:
        p = f["properties"]
        key = f"{p.get('iso','XX')}:T{p.get('rank_v2', 0)}"
        by_tier_iso[key].append(f)
    for tk, tfeats in by_tier_iso.items():
        tfeats.sort(key=lambda f: _rank_key(f["properties"]), reverse=True)
        n = len(tfeats)
        for i, f in enumerate(tfeats):
            p = f["properties"]
            p["rank_in_tier"]    = i + 1
            p["rank_in_tier_of"] = n

    # 6. Co-location N: when multiple clusters share region_name, append ordinal
    #    Sort by count_3km desc so Co-location 1 = densest cluster in that region
    by_region_name: dict[str, list] = defaultdict(list)
    for f in features:
        key = f"{f['properties'].get('iso','XX')}:{f['properties'].get('region_name','')}"
        by_region_name[key].append(f)
    for key, rfeats in by_region_name.items():
        if len(rfeats) <= 1:
            continue
        rfeats.sort(key=lambda f: _rank_key(f["properties"]), reverse=True)
        region_name = rfeats[0]["properties"].get("region_name", "")
        for i, f in enumerate(rfeats):
            f["properties"]["display_name"] = f"{region_name} — Co-location {i + 1}"


def main():
    parser = argparse.ArgumentParser(description="Enhanced 0-1000 co-tenancy ranking engine")
    parser.add_argument("--dry-run", action="store_true", help="Report only, do not write output")
    parser.add_argument("--threshold-t3", type=int, default=DEFAULT_T3)
    parser.add_argument("--threshold-t2", type=int, default=DEFAULT_T2)
    parser.add_argument("--threshold-t1", type=int, default=DEFAULT_T1)
    args = parser.parse_args()

    if not INPUT_FILE.exists():
        print(f"ERROR: {INPUT_FILE} not found. Run build-clusters.py first.")
        sys.exit(1)

    print(f"Loading {INPUT_FILE} ...")
    with open(INPUT_FILE) as f:
        fc = json.load(f)
    features = fc.get("features", [])
    print(f"  {len(features)} clusters")

    # Phase 1: compute V2 scores (per-cluster)
    print("Computing V2 scores ...")
    for feat in features:
        new_scores = score_cluster(feat["properties"])
        feat["properties"].update(new_scores)

    # Phase 2: overlap penalty (cross-cluster)
    print("Applying overlap penalties ...")
    apply_overlap_penalty(features)

    # Phase 2b: deduplicate same-zone clusters (anchor stores within 0.5 km of each other)
    print("Deduplicating same-zone clusters (0.15 km threshold) ...")
    before = len(features)
    features = deduplicate_clusters(features)
    print(f"  {before - len(features)} duplicates removed → {len(features)} clusters")

    # Phase 3: score-driven tiers with saturation guard
    print(f"Assigning tiers (T3>={args.threshold_t3}, T2>={args.threshold_t2}, T1>={args.threshold_t1}) ...")
    effective_t3, _, _ = assign_tiers(features, args.threshold_t3, args.threshold_t2, args.threshold_t1)
    if effective_t3 != args.threshold_t3:
        print(f"  Saturation guard raised T3 threshold: {args.threshold_t3} → {effective_t3}")

    # Phase 4: multi-dimensional rankings
    print("Assigning rankings ...")
    assign_rankings(features)

    # Summary
    tier_counts = Counter(f["properties"].get("rank_v2", 0) for f in features)
    print(f"  T3 Apex: {tier_counts[3]}  T2 Hub: {tier_counts[2]}  "
          f"T1 Valid: {tier_counts[1]}  T0 Border: {tier_counts[0]}")
    score_min = min(f["properties"].get("score_final", 0) for f in features)
    score_max = max(f["properties"].get("score_final", 0) for f in features)
    print(f"  Score range: {score_min}–{score_max}")

    if args.dry_run:
        print("Dry run — not writing output.")
        return

    with open(OUTPUT_FILE, "w") as f:
        json.dump({"type": "FeatureCollection", "features": features}, f, indent=2)
    print(f"Written → {OUTPUT_FILE}")


if __name__ == "__main__":
    main()
