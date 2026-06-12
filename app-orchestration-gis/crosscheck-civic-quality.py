#!/usr/bin/env python3
"""
crosscheck-civic-quality.py — Cross-check OSM civic vs Overture Places for data quality.

Method:
  Spatial-match OSM hospitals/universities against Overture Places records
  within 200m. Records matched in both sources = high confidence.
  OSM-only = single-source. Overture-only = potential pipeline gaps.

Input:
  service-places/cleansed-civic-osm.jsonl  (OSM source)
  service-places/cleansed-places.jsonl     (Overture source)

Output: work/civic-quality-report.txt
"""
import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR

OSM_SRC = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1/service-places/cleansed-civic-osm.jsonl"
)
OVT_SRC = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1/service-places/cleansed-places.jsonl"
)
OUT = WORK_DIR / "civic-quality-report.txt"

MATCH_RADIUS_KM = 0.2   # 200m match threshold
GRID_STEP = 0.1         # ~11km grid bucket; we check adjacent buckets for fine matches

OVT_HOSP_CATS = {"hospital", "childrens_hospital"}
OVT_UNIV_CATS = {"college_university", "university"}


def haversine_km(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = math.sin(dlat / 2) ** 2 + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2)) * math.sin(dlon / 2) ** 2
    return R * 2 * math.asin(math.sqrt(a))


def grid_key(lat, lon):
    return (round(lat / GRID_STEP), round(lon / GRID_STEP))


def build_grid(records):
    grid = defaultdict(list)
    for i, r in enumerate(records):
        grid[grid_key(r["latitude"], r["longitude"])].append(i)
    return grid


def nearby(lat, lon, records, grid, radius_km):
    results = []
    gk = grid_key(lat, lon)
    for dlat in (-1, 0, 1):
        for dlon in (-1, 0, 1):
            for idx in grid[(gk[0] + dlat, gk[1] + dlon)]:
                r = records[idx]
                d = haversine_km(lat, lon, r["latitude"], r["longitude"])
                if d <= radius_km:
                    results.append((idx, d))
    return results


def main():
    lines = []

    def pr(s=""):
        print(s)
        lines.append(s)

    pr("=" * 72)
    pr("CIVIC DATA QUALITY REPORT — OSM vs Overture cross-check")
    pr("=" * 72)
    pr()

    # --- Load OSM source ---
    if not OSM_SRC.exists():
        pr(f"ERROR: OSM source not found: {OSM_SRC}")
        return
    print(f"Loading OSM civic: {OSM_SRC} ...")
    osm_recs = []
    with open(OSM_SRC) as f:
        for line in f:
            line = line.strip()
            if line:
                try:
                    osm_recs.append(json.loads(line))
                except Exception:
                    pass
    osm_hosp = [r for r in osm_recs if r.get("category_id") == "hospital"]
    osm_univ = [r for r in osm_recs if r.get("category_id") == "university"]
    print(f"  OSM: {len(osm_hosp):,} hospitals, {len(osm_univ):,} universities")

    # --- Load Overture source (filter to civic) ---
    if not OVT_SRC.exists():
        pr(f"ERROR: Overture source not found: {OVT_SRC}")
        return
    print(f"Loading Overture places: {OVT_SRC} ...")
    ovt_hosp = []
    ovt_univ = []
    with open(OVT_SRC) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                r = json.loads(line)
                cat = r.get("category_id", "")
                if cat in OVT_HOSP_CATS:
                    ovt_hosp.append(r)
                elif cat in OVT_UNIV_CATS:
                    ovt_univ.append(r)
            except Exception:
                pass
    print(f"  Overture: {len(ovt_hosp):,} hospitals, {len(ovt_univ):,} universities")
    pr(f"OSM source:     {len(osm_hosp):,} hospitals  {len(osm_univ):,} universities")
    pr(f"Overture source:{len(ovt_hosp):,} hospitals  {len(ovt_univ):,} universities")
    pr()

    # --- Spatial match function ---
    def run_match(osm_list, ovt_list, label):
        pr(f"  {label} ({len(osm_list):,} OSM vs {len(ovt_list):,} Overture)")
        pr(f"  Match radius: {MATCH_RADIUS_KM*1000:.0f}m")
        pr()

        ovt_grid = build_grid(ovt_list)

        matched_osm = set()
        matched_ovt = set()
        match_by_country = Counter()
        total_by_country = Counter(r.get("iso_country_code", "?") for r in osm_list)

        for i, r in enumerate(osm_list):
            if i % 5000 == 0:
                print(f"    Matching {label}: {i:,}/{len(osm_list):,} ...", end="\r")
            hits = nearby(r["latitude"], r["longitude"], ovt_list, ovt_grid, MATCH_RADIUS_KM)
            if hits:
                matched_osm.add(i)
                for j, _ in hits:
                    matched_ovt.add(j)
                match_by_country[r.get("iso_country_code", "?")] += 1
        print()

        n_osm = len(osm_list)
        n_ovt = len(ovt_list)
        n_matched = len(matched_osm)
        n_osm_only = n_osm - n_matched
        n_ovt_only = n_ovt - len(matched_ovt)

        pr(f"  Results:")
        pr(f"    Both sources (high confidence):  {n_matched:>6,}  ({n_matched/n_osm*100:.1f}% of OSM)")
        pr(f"    OSM-only (single source):         {n_osm_only:>6,}  ({n_osm_only/n_osm*100:.1f}% of OSM)")
        pr(f"    Overture-only (pipeline gap):     {n_ovt_only:>6,}  ({n_ovt_only/n_ovt*100:.1f}% of Overture)")
        pr()

        pr(f"  Match rate by country (Top 15):")
        pr(f"  {'ISO':<5} {'OSM':>6}  {'matched':>7}  {'match%':>6}")
        for iso, total in sorted(total_by_country.items(), key=lambda x: -match_by_country[x[0]]):
            matched = match_by_country[iso]
            if total < 5:
                continue
            pr(f"  {iso:<5} {total:>6,}  {matched:>7,}  {matched/total*100:>5.1f}%")

        pr()

        # Overture-only high-value records (not in OSM)
        ovt_only_list = [
            ovt_list[j] for j in range(n_ovt) if j not in matched_ovt
        ]
        # Filter to regional or high-confidence
        ovt_only_hv = [
            r for r in ovt_only_list
            if r.get("is_regional_anchor") or (r.get("confidence", 0) or 0) >= 0.9
        ]
        pr(f"  Overture-only high-value (is_regional_anchor or confidence≥0.9): {len(ovt_only_hv):,}")
        pr(f"  By country:")
        for iso, n in Counter(r.get("iso_country_code", "?") for r in ovt_only_hv).most_common(15):
            pr(f"    {iso}: {n:,}")
        pr()
        return matched_osm, ovt_only_list

    # --- Run match for hospitals ---
    pr("-" * 72)
    pr("HOSPITALS")
    pr("-" * 72)
    matched_hosp, ovt_hosp_only = run_match(osm_hosp, ovt_hosp, "hospitals")

    # --- Run match for universities ---
    pr("-" * 72)
    pr("UNIVERSITIES")
    pr("-" * 72)
    matched_univ, ovt_univ_only = run_match(osm_univ, ovt_univ, "universities")

    # --- Quality tiers for hospitals in OSM ---
    pr("-" * 72)
    pr("OSM HOSPITAL QUALITY TIERS")
    pr("-" * 72)
    pr()
    pr("  Tier definitions:")
    pr("    High:   beds≥150 AND emergency=yes AND matched in Overture")
    pr("    Medium: emergency=yes OR beds≥150 (at least one strong signal)")
    pr("    Low:    neither beds nor emergency signal (name-heuristic only)")
    pr()

    h_high = sum(
        1 for i, r in enumerate(osm_hosp)
        if i in matched_hosp
        and (r.get("bed_count") or 0) >= 150
        and r.get("emergency") == "yes"
    )
    h_med = sum(
        1 for r in osm_hosp
        if r.get("emergency") == "yes" or (r.get("bed_count") or 0) >= 150
    )
    h_low = len(osm_hosp) - h_med

    pr(f"  High confidence:   {h_high:>6,}  ({h_high/len(osm_hosp)*100:.1f}%)")
    pr(f"  Medium confidence: {h_med:>6,}  ({h_med/len(osm_hosp)*100:.1f}%)")
    pr(f"  Low confidence:    {h_low:>6,}  ({h_low/len(osm_hosp)*100:.1f}%)")
    pr()
    pr("  Implication for Rule A/B:")
    pr(f"    If only high-confidence hospitals gate T1/T2, ~{h_high:,} records are reliable.")
    pr(f"    Low-confidence records ({h_low:,}) should NOT gate tier — name-heuristic only.")

    # --- Summary ---
    pr()
    pr("=" * 72)
    pr("SUMMARY — Recommendations for taxonomy changes")
    pr("=" * 72)
    pr()
    pr("  Data readiness by use case:")
    pr()

    hosp_match_rate = len(matched_hosp) / len(osm_hosp) * 100
    univ_match_rate = len(matched_univ) / len(osm_univ) * 100

    pr(f"  Hospital cross-source match rate: {hosp_match_rate:.1f}%")
    pr(f"  University cross-source match rate: {univ_match_rate:.1f}%")
    pr()

    if hosp_match_rate >= 60:
        pr("  Hospital data: ADEQUATE for tier gating (≥60% cross-source confirmation)")
        pr("  Recommended gate: hospital_tier='regional' + (emergency=yes OR beds≥100)")
    else:
        pr("  Hospital data: CAUTION — cross-source match rate below 60%")
        pr("  Recommend: restrict tier gates to records with emergency=yes AND Overture match")

    if univ_match_rate >= 60:
        pr()
        pr("  University data: ADEQUATE for tier gating (≥60% cross-source confirmation)")
        pr("  Recommended gate: university_tier='regional' (≥5,000 students for T1/T2)")
    else:
        pr()
        pr("  University data: CAUTION — cross-source match rate below 60%")
        pr("  EU universities lack enrollment data; gating T1/T2 on EU universities is risky")
        pr("  Recommend: US-only university gating until ETER data is integrated")

    pr()
    pr("  EU university enrichment gap:")
    pr("  ETER (European Tertiary Education Register) provides enrollment for ~6,000 EU")
    pr("  universities. Download: https://www.eter-project.com/#/data (CC BY)")
    pr("  Integrating ETER would bring EU university quality on par with US IPEDS.")

    WORK_DIR.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines) + "\n")
    print(f"\nWrote: {OUT}")


if __name__ == "__main__":
    main()
