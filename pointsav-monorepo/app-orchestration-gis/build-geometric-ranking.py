#!/usr/bin/env python3
"""
build-geometric-ranking.py — Phase 2 pure-predicate tier engine.

Replaces generate-rankings.py V2 score-based tier assignment with a
binary-predicate gate system. Each cluster is assigned:
    tier                  1 (Regional) / 2 (District) / 3 (Local) / 4 (Fringe)
    tier_predicates_fired list of strings describing which gates passed

Predicate definitions:
    T1 Regional:  (Warehouse ∧ Hypermarket) OR (Lifestyle ∧ Hypermarket)
                  ∧ rank_pp_iso ≤ p10
                  ∧ hc_count_regional ≥ 1
                  ∧ IoU_max(cluster, any stronger-in-ISO) ≤ 0.10

    T2 District:  Hypermarket ∧ (Hardware OR Warehouse)
                  ∧ rank_pp_iso ≤ p25
                  ∧ (rank_pg_iso ≤ p25 OR rank_ph_iso ≤ p25 OR rank_pw_iso ≤ p25)
                  ∧ hc_count_regional + hc_count_district ≥ 1
                  ∧ IoU_max(cluster, T1) ≤ 0.25

    T3 Local:     (Hardware OR Warehouse) present
                  ∧ rank_pp_iso ≤ p50
                  ∧ hc_count ≥ 1

    T4 Fringe:    none of the above

Percentile thresholds (G6): p10=0.10, p20=0.20, p25=0.25, p50=0.50
IoU: closed-form lens-area for equal-radius circles (SECONDARY_RADIUS_KM=3.0 km disk).
Tiebreaker within tier+ISO: anchor-count desc → pp desc → cluster_id asc.

Per-ISO percentile ranks (rank_pp_iso etc.) are loaded from work/catchment-data.json
and merged into cluster properties before evaluation. Run synthesize-od-study.py first.

Reads:  work/clusters.geojson          (produced by build-clusters.py + generate-rankings.py)
        work/catchment-data.json        (produced by synthesize-od-study.py)
Writes: work/clusters.geojson          (in-place; adds tier + tier_predicates_fired)
        deployments/.../clusters-meta.json  (patches tier + tier_predicates_fired)
"""

import json
import math
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import (
    WORK_DIR, ALPHA_HYPERMARKET, ALPHA_LIFESTYLE, ALPHA_HARDWARE, ALPHA_WAREHOUSE,
    SECONDARY_RADIUS_KM,
)

INPUT_FILE    = WORK_DIR / "clusters.geojson"
OUTPUT_FILE   = WORK_DIR / "clusters.geojson"
CATCHMENT_IN  = WORK_DIR / "catchment-data.json"
CLUSTERS_META = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")

# Percentile thresholds (G6 — coarse set)
P10 = 0.10
P20 = 0.20
P25 = 0.25
P50 = 0.50

DISK_RADIUS_KM = SECONDARY_RADIUS_KM   # 3.0 km — primary-disk IoU baseline


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def haversine_km(lat1: float, lon1: float, lat2: float, lon2: float) -> float:
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2
         + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2))
         * math.sin(dlon / 2) ** 2)
    return 2.0 * R * math.asin(math.sqrt(max(0.0, min(1.0, a))))


def iou_equal_circles(dist_km: float, r_km: float) -> float:
    """IoU of two equal-radius circles separated by dist_km.
    Closed-form lens area formula. Returns 0 if circles do not overlap."""
    if dist_km >= 2.0 * r_km:
        return 0.0
    if dist_km <= 0.0:
        return 1.0
    d, r = dist_km, r_km
    lens = 2.0 * r * r * math.acos(d / (2.0 * r)) - (d / 2.0) * math.sqrt(4.0 * r * r - d * d)
    circle_area = math.pi * r * r
    union = 2.0 * circle_area - lens
    return lens / union if union > 0 else 0.0


def _parse_list(val) -> list:
    if isinstance(val, list):
        return val
    if isinstance(val, str):
        try:
            return json.loads(val)
        except (ValueError, TypeError):
            return []
    return []


# ---------------------------------------------------------------------------
# Anchor class detection from clusters.geojson property keys
# ---------------------------------------------------------------------------

_hyper_ids     = set().union(*ALPHA_HYPERMARKET.values())
_lifestyle_ids = set().union(*ALPHA_LIFESTYLE.values())
_hw_ids        = set().union(*ALPHA_HARDWARE.values())
_wh_ids        = set().union(*ALPHA_WAREHOUSE.values())


def _anchor_classes(p: dict) -> set[str]:
    """Return set of alpha classes present in this cluster."""
    classes: set[str] = set()
    anc        = p.get("primary_anchor") or ""
    hw_list    = _parse_list(p.get("hw_list",    []))
    wh_list    = _parse_list(p.get("wh_list",    []))
    hyper_list = _parse_list(p.get("hyper_list", []))
    ls_list    = _parse_list(p.get("ls_list",    []))

    if anc in _hyper_ids     or any(c in _hyper_ids     for c in hyper_list):
        classes.add("Hypermarket")
    if anc in _lifestyle_ids or any(c in _lifestyle_ids for c in ls_list):
        classes.add("Lifestyle")
    if anc in _hw_ids        or any(c in _hw_ids        for c in hw_list):
        classes.add("Hardware")
    if anc in _wh_ids        or any(c in _wh_ids        for c in wh_list):
        classes.add("Warehouse")
    return classes


def _clat(p: dict) -> float:
    return float(p.get("centroid_lat") or p.get("lat") or 0)


def _clon(p: dict) -> float:
    return float(p.get("centroid_lon") or p.get("lon") or 0)


def _cid(p: dict) -> str:
    return str(p.get("cluster_id") or p.get("id") or "")


# ---------------------------------------------------------------------------
# Tier predicates
# ---------------------------------------------------------------------------

def _eval_t1(p: dict, iso_peers: list[dict]) -> tuple[bool, list[str]]:
    fired: list[str] = []
    classes = _anchor_classes(p)

    comp_ok = (("Warehouse" in classes and "Hypermarket" in classes) or
               ("Lifestyle" in classes and "Hypermarket" in classes))
    if not comp_ok:
        return False, []
    fired.append("T1:composition")

    rpp = float(p.get("rank_pp_iso") or 1.0)
    if rpp > P10:
        return False, []
    fired.append("T1:rank_pp")

    hc_reg = int(p.get("hc_count_regional") or 0)
    if hc_reg < 1:
        return False, []
    fired.append("T1:civic_regional")

    cid = _cid(p)
    pp  = float(p.get("pp") or 0)
    for peer in iso_peers:
        if _cid(peer) == cid:
            continue
        if float(peer.get("pp") or 0) <= pp:
            continue
        dist = haversine_km(_clat(p), _clon(p), _clat(peer), _clon(peer))
        if iou_equal_circles(dist, DISK_RADIUS_KM) > 0.10:
            return False, []
    fired.append("T1:iou")

    return True, fired


def _eval_t2(p: dict, t1_clusters: list[dict]) -> tuple[bool, list[str]]:
    fired: list[str] = []
    classes = _anchor_classes(p)

    comp_ok = ("Hypermarket" in classes and ("Hardware" in classes or "Warehouse" in classes))
    if not comp_ok:
        return False, []
    fired.append("T2:composition")

    if float(p.get("rank_pp_iso") or 1.0) > P25:
        return False, []
    fired.append("T2:rank_pp")

    rpg = float(p.get("rank_pg_iso") or 1.0)
    rph = float(p.get("rank_ph_iso") or 1.0)
    rpw = float(p.get("rank_pw_iso") or 1.0)
    if not (rpg <= P25 or rph <= P25 or rpw <= P25):
        return False, []
    fired.append("T2:rank_spend")

    hc_reg = int(p.get("hc_count_regional") or 0)
    hc_dst = int(p.get("hc_count_district") or 0)
    if hc_reg + hc_dst < 1:
        return False, []
    fired.append("T2:civic_hospital")

    for t1p in t1_clusters:
        dist = haversine_km(_clat(p), _clon(p), _clat(t1p), _clon(t1p))
        if iou_equal_circles(dist, DISK_RADIUS_KM) > 0.25:
            return False, []
    fired.append("T2:iou")

    return True, fired


def _eval_t3(p: dict) -> tuple[bool, list[str]]:
    classes = _anchor_classes(p)
    fired: list[str] = []

    if not ("Hardware" in classes or "Warehouse" in classes):
        return False, []
    fired.append("T3:composition")

    if float(p.get("rank_pp_iso") or 1.0) > P50:
        return False, []
    fired.append("T3:rank_pp")

    if int(p.get("hc_count") or 0) < 1:
        return False, []
    fired.append("T3:civic_any")

    return True, fired


# ---------------------------------------------------------------------------
# Main tier assignment
# ---------------------------------------------------------------------------

def assign_tiers(features: list[dict]) -> None:
    props_list = [f["properties"] for f in features]

    by_iso: dict[str, list[dict]] = defaultdict(list)
    for p in props_list:
        by_iso[p.get("iso") or ""].append(p)

    # Pass 1 — T1 per ISO
    t1_props: list[dict] = []
    for iso, iso_props in by_iso.items():
        for p in iso_props:
            ok, fired = _eval_t1(p, iso_props)
            if ok:
                p["_t"] = 1
                p["tier_predicates_fired"] = fired
                t1_props.append(p)

    # Pass 2 — T2 per ISO, IoU against T1
    for iso, iso_props in by_iso.items():
        iso_t1 = [p for p in t1_props if (p.get("iso") or "") == iso]
        for p in iso_props:
            if "_t" in p:
                continue
            ok, fired = _eval_t2(p, iso_t1)
            if ok:
                p["_t"] = 2
                p["tier_predicates_fired"] = fired

    # Pass 3 — T3
    for p in props_list:
        if "_t" in p:
            continue
        ok, fired = _eval_t3(p)
        if ok:
            p["_t"] = 3
            p["tier_predicates_fired"] = fired

    # Pass 4 — Fringe
    for p in props_list:
        if "_t" not in p:
            p["_t"] = 4
            p["tier_predicates_fired"] = []

    # Finalise
    counts = {1: 0, 2: 0, 3: 0, 4: 0}
    for p in props_list:
        t = p.pop("_t")
        p["tier"] = t
        fired = p["tier_predicates_fired"]
        p["tier_predicates_fired"] = json.dumps(fired) if isinstance(fired, list) else fired
        counts[t] += 1

    print(f"  T1 Regional: {counts[1]}  T2 District: {counts[2]}  "
          f"T3 Local: {counts[3]}  T4 Fringe: {counts[4]}")

    # Emit contingency table (rank_v2 × tier) for diff harness
    table: dict[str, dict[str, int]] = {}
    for p in props_list:
        v2 = str(p.get("rank_v2") or "?")
        t  = str(p.get("tier", "?"))
        table.setdefault(v2, {}).setdefault(t, 0)
        table[v2][t] += 1
    print("\n  Contingency: rank_v2 (rows) × tier (cols)")
    print(f"  {'v2\\tier':>10} {'1':>8} {'2':>8} {'3':>8} {'4':>8}")
    for v2 in sorted(table, key=lambda x: int(x) if x.isdigit() else -1):
        row = table[v2]
        print(f"  {'v2='+v2:>10} {row.get('1',0):>8} {row.get('2',0):>8} "
              f"{row.get('3',0):>8} {row.get('4',0):>8}")


def main():
    print(f"Loading {INPUT_FILE} ...")
    with open(INPUT_FILE) as f:
        geojson = json.load(f)
    features = geojson.get("features", [])
    print(f"  {len(features):,} clusters")

    # Merge per-ISO percentile ranks from catchment-data.json
    if CATCHMENT_IN.exists():
        print(f"Merging per-ISO ranks from {CATCHMENT_IN.name} ...")
        with open(CATCHMENT_IN) as f:
            catchment = json.load(f)
        rank_fields = ["rank_pp_iso", "rank_sp_iso", "rank_pg_iso", "rank_sg_iso",
                       "rank_ph_iso", "rank_sh_iso", "rank_pw_iso", "rank_sw_iso",
                       "pp", "sp"]
        merged = 0
        for feat in features:
            cid = feat["properties"].get("cluster_id") or feat["properties"].get("id") or ""
            if cid in catchment:
                for k in rank_fields:
                    if k in catchment[cid]:
                        feat["properties"][k] = catchment[cid][k]
                merged += 1
        print(f"  Merged {merged:,} / {len(features):,} clusters")
    else:
        print(f"WARNING: {CATCHMENT_IN} not found. Per-ISO rank predicates will fail; "
              f"run synthesize-od-study.py first.")

    print("Assigning tiers (pure-predicate engine V3) ...")
    assign_tiers(features)

    print(f"Writing {OUTPUT_FILE} ...")
    with open(OUTPUT_FILE, "w") as f:
        json.dump(geojson, f)
    print("  Done.")

    # Patch clusters-meta.json
    if CLUSTERS_META.exists():
        print(f"Patching {CLUSTERS_META.name} ...")
        with open(CLUSTERS_META) as f:
            meta = json.load(f)
        props_by_cid = {
            (feat["properties"].get("cluster_id") or feat["properties"].get("id") or ""): feat["properties"]
            for feat in features
        }
        patched = 0
        for entry in meta:
            cid = entry.get("id")
            if cid and cid in props_by_cid:
                p = props_by_cid[cid]
                entry["tier"] = p["tier"]
                entry["tier_predicates_fired"] = p["tier_predicates_fired"]
                patched += 1
        with open(CLUSTERS_META, "w") as f:
            json.dump(meta, f, separators=(",", ":"))
        print(f"  Patched {patched:,} / {len(meta):,} entries.")


if __name__ == "__main__":
    main()
