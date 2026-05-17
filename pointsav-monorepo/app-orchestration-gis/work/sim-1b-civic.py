#!/usr/bin/env python3
"""
sim-1b-civic.py — T1 tier recalibration simulation.

PART A — Civic gate sensitivity matrix (P-threshold x civic gate option).
PART B — Composition path expansion at P=0.20 with current civic gate.

Reads:  work/clusters.geojson + work/catchment-data.json
Writes: work/sim-1b-results.txt
"""

import json
import math
import sys
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
APP  = HERE.parent
sys.path.insert(0, str(APP))

from config import (
    ALPHA_HYPERMARKET, ALPHA_LIFESTYLE, ALPHA_HARDWARE, ALPHA_WAREHOUSE,
    SECONDARY_RADIUS_KM,
)

CLUSTERS_GEOJSON = APP / "work" / "clusters.geojson"
CATCHMENT_JSON   = APP / "work" / "catchment-data.json"
RESULTS_FILE     = APP / "work" / "sim-1b-results.txt"

DISK_RADIUS_KM = SECONDARY_RADIUS_KM  # 3.0

# -----------------------------------------------------------------
# Anchor class sets (mirrors build-geometric-ranking.py)
# -----------------------------------------------------------------
_hyper_ids     = set().union(*ALPHA_HYPERMARKET.values())
_lifestyle_ids = set().union(*ALPHA_LIFESTYLE.values())
_hw_ids        = set().union(*ALPHA_HARDWARE.values())
_wh_ids        = set().union(*ALPHA_WAREHOUSE.values())

NA_ISO = {"US", "CA", "MX"}


def _parse_list(val):
    if isinstance(val, list):
        return val
    if isinstance(val, str):
        try:
            return json.loads(val)
        except (ValueError, TypeError):
            return []
    return []


def _anchor_classes(p):
    classes = set()
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


def haversine_km(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2
         + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2))
         * math.sin(dlon / 2) ** 2)
    return 2.0 * R * math.asin(math.sqrt(max(0.0, min(1.0, a))))


def iou_equal_circles(dist_km, r_km):
    if dist_km >= 2.0 * r_km:
        return 0.0
    if dist_km <= 0.0:
        return 1.0
    d, r = dist_km, r_km
    lens = 2.0 * r * r * math.acos(d / (2.0 * r)) - (d / 2.0) * math.sqrt(4.0 * r * r - d * d)
    union = 2.0 * math.pi * r * r - lens
    return lens / union if union > 0 else 0.0


def _clat(p): return float(p.get("centroid_lat") or p.get("lat") or 0)
def _clon(p): return float(p.get("centroid_lon") or p.get("lon") or 0)
def _cid(p):  return str(p.get("cluster_id") or p.get("id") or "")


# -----------------------------------------------------------------
# Load + merge
# -----------------------------------------------------------------
print(f"Loading {CLUSTERS_GEOJSON.name} ...")
with open(CLUSTERS_GEOJSON) as f:
    geojson = json.load(f)
features = geojson["features"]
print(f"  {len(features):,} clusters")

print(f"Merging per-ISO ranks from {CATCHMENT_JSON.name} ...")
with open(CATCHMENT_JSON) as f:
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
print(f"  merged {merged:,}")

# Snapshot props list and per-iso index
props_list = [f["properties"] for f in features]
by_iso = defaultdict(list)
for p in props_list:
    by_iso[p.get("iso") or ""].append(p)


def continent_of(iso):
    return "NA" if iso in NA_ISO else "EU"


# -----------------------------------------------------------------
# Reusable T1 evaluator: returns (qualifies_bool, reason)
# -----------------------------------------------------------------

def composition_current(classes):
    return (("Warehouse" in classes and "Hypermarket" in classes) or
            ("Lifestyle" in classes and "Hypermarket" in classes))


def composition_path_c(classes):
    # Hardware + Hypermarket (currently a T2 pattern)
    return ("Hardware" in classes and "Hypermarket" in classes)


def composition_path_d(classes):
    # Pure lifestyle, no hypermarket requirement
    return ("Lifestyle" in classes)


def civic_gate_a(p):  # current
    return int(p.get("hc_count_regional") or 0) >= 1


def civic_gate_b(p):  # any hospital
    return int(p.get("hc_count") or 0) >= 1


def civic_gate_c(p):  # off
    return True


def passes_iou(p, iso_peers, threshold=0.10):
    cid = _cid(p)
    pp  = float(p.get("pp") or 0)
    plat, plon = _clat(p), _clon(p)
    for peer in iso_peers:
        if _cid(peer) == cid:
            continue
        if float(peer.get("pp") or 0) <= pp:
            continue
        dist = haversine_km(plat, plon, _clat(peer), _clon(peer))
        if iou_equal_circles(dist, DISK_RADIUS_KM) > threshold:
            return False
    return True


def evaluate_t1(p, iso_peers, p_threshold, composition_fn, civic_fn):
    classes = _anchor_classes(p)
    if not composition_fn(classes):
        return False
    if float(p.get("rank_pp_iso") or 1.0) > p_threshold:
        return False
    if not civic_fn(p):
        return False
    if not passes_iou(p, iso_peers, threshold=0.10):
        return False
    return True


# -----------------------------------------------------------------
# PART A — civic-gate sensitivity matrix
# -----------------------------------------------------------------

P_THRESHOLDS = [0.20, 0.25]
CIVIC_GATES  = [("A_current", civic_gate_a),
                ("B_any_hospital", civic_gate_b),
                ("C_off", civic_gate_c)]

print("\nPART A — civic gate sensitivity matrix")
matrix_rows = []
for p_thr in P_THRESHOLDS:
    for gate_name, gate_fn in CIVIC_GATES:
        na_count = 0
        eu_count = 0
        qualifiers = []
        for iso, iso_props in by_iso.items():
            cont = continent_of(iso)
            for p in iso_props:
                if evaluate_t1(p, iso_props, p_thr, composition_current, gate_fn):
                    qualifiers.append(p)
                    if cont == "NA":
                        na_count += 1
                    else:
                        eu_count += 1
        matrix_rows.append((p_thr, gate_name, na_count, eu_count, qualifiers))
        print(f"  P={p_thr:.2f}  gate={gate_name:<16}  NA={na_count:4d}  EU={eu_count:4d}  total={na_count+eu_count:4d}")

# Gate C deltas: clusters qualifying ONLY when civic is off
# Baseline = current gate (Gate A) at same P-threshold
print("\nPART A — Gate C (no civic) only qualifiers (samples)")
gate_c_only_samples = {}
for p_thr in P_THRESHOLDS:
    # Find Gate A qualifier ids and Gate C qualifier ids at this P
    gate_a_ids = set()
    gate_c_ids = set()
    gate_c_props = {}
    for (thr, name, _na, _eu, quals) in matrix_rows:
        if thr != p_thr:
            continue
        if name == "A_current":
            gate_a_ids = {_cid(p) for p in quals}
        elif name == "C_off":
            gate_c_ids = {_cid(p) for p in quals}
            gate_c_props = {_cid(p): p for p in quals}
    only_c = sorted(gate_c_ids - gate_a_ids)
    print(f"\n  P={p_thr:.2f}: Gate-C-only count = {len(only_c)}")
    # Sample by-continent: take up to 10 mixed, sort by pp desc
    samples = [gate_c_props[cid] for cid in only_c]
    samples.sort(key=lambda x: float(x.get("pp") or 0), reverse=True)
    take = samples[:10]
    gate_c_only_samples[p_thr] = take
    for p in take:
        cid = _cid(p)
        iso = p.get("iso")
        disp = p.get("display_name") or ""
        pp = float(p.get("pp") or 0)
        rpp = float(p.get("rank_pp_iso") or 1.0)
        hc  = int(p.get("hc_count") or 0)
        hc_reg = int(p.get("hc_count_regional") or 0)
        print(f"    {iso} pp={pp:>10,.0f} rank_pp={rpp:.3f} hc={hc} hc_reg={hc_reg}  {cid}  | {disp}")

# -----------------------------------------------------------------
# PART B — composition path expansion (P=0.20, civic=current)
# -----------------------------------------------------------------

print("\nPART B — Composition path expansion at P=0.20, civic=current")

# First compute current T1 cluster_ids (composition_current, P=0.20, civic A)
# Note user said "current T1" which means baseline composition; use 0.20 here
# per spec: "at P=0.20"
current_t1_ids = set()
for iso, iso_props in by_iso.items():
    for p in iso_props:
        if evaluate_t1(p, iso_props, 0.20, composition_current, civic_gate_a):
            current_t1_ids.add(_cid(p))
print(f"  baseline (current composition, P=0.20, civic A): {len(current_t1_ids)} T1 clusters")

# Also compute the operational current T1 set (file-baked tier == 1)
op_t1_ids = {_cid(p) for p in props_list if int(p.get("tier") or 0) == 1}
op_tier_of = {_cid(p): int(p.get("tier") or 0) for p in props_list}
print(f"  operational current T1 (file tier==1): {len(op_t1_ids)}")

paths = [("C_hw_plus_hyper", composition_path_c),
         ("D_pure_lifestyle", composition_path_d)]

for path_name, comp_fn in paths:
    new_qualifiers = []
    for iso, iso_props in by_iso.items():
        for p in iso_props:
            if evaluate_t1(p, iso_props, 0.20, comp_fn, civic_gate_a):
                cid = _cid(p)
                if cid not in current_t1_ids:
                    new_qualifiers.append(p)
    print(f"\n  Path {path_name}: {len(new_qualifiers)} NEW clusters (not in baseline current-composition T1)")

    # Breakdown by iso
    by_iso_count = defaultdict(int)
    for p in new_qualifiers:
        by_iso_count[p.get("iso") or "?"] += 1
    if by_iso_count:
        line = "    iso breakdown: " + ", ".join(
            f"{iso}={n}" for iso, n in sorted(by_iso_count.items(), key=lambda kv: -kv[1]))
        print(line)

    # Sample listing (up to 15)
    new_qualifiers.sort(key=lambda x: float(x.get("pp") or 0), reverse=True)
    take = new_qualifiers[:15]
    print(f"    samples (pp desc, current_tier in []):")
    for p in take:
        cid = _cid(p)
        iso = p.get("iso")
        disp = p.get("display_name") or ""
        pp = float(p.get("pp") or 0)
        cur = op_tier_of.get(cid, "?")
        print(f"      {iso} pp={pp:>10,.0f} [cur=T{cur}] {cid} | {disp}")


# -----------------------------------------------------------------
# Save summary to file
# -----------------------------------------------------------------

with open(RESULTS_FILE, "w") as fh:
    fh.write("sim-1b-civic — T1 recalibration simulation\n")
    fh.write("=" * 60 + "\n\n")

    fh.write("PART A — civic gate sensitivity matrix\n")
    fh.write("(composition = current WH∧HM or LS∧HM; IoU ≤ 0.10)\n\n")
    fh.write(f"  {'P':>6}  {'Gate':<18}  {'NA':>6}  {'EU':>6}  {'Total':>6}\n")
    for (p_thr, name, na, eu, _q) in matrix_rows:
        fh.write(f"  {p_thr:>6.2f}  {name:<18}  {na:>6d}  {eu:>6d}  {na+eu:>6d}\n")
    fh.write("\n")

    fh.write("Gate C (no civic) — clusters qualifying ONLY because civic is off\n")
    for p_thr, take in gate_c_only_samples.items():
        fh.write(f"\n  P={p_thr:.2f} samples (top by pp):\n")
        for p in take:
            cid = _cid(p)
            iso = p.get("iso")
            disp = p.get("display_name") or ""
            pp = float(p.get("pp") or 0)
            rpp = float(p.get("rank_pp_iso") or 1.0)
            hc  = int(p.get("hc_count") or 0)
            hc_reg = int(p.get("hc_count_regional") or 0)
            fh.write(f"    {iso} pp={pp:>10,.0f} rank_pp={rpp:.3f} hc={hc} hc_reg={hc_reg}  {cid}  | {disp}\n")

    fh.write("\n" + "=" * 60 + "\n")
    fh.write("PART B — composition path expansion (P=0.20, civic=current)\n")
    fh.write(f"baseline current-composition T1 = {len(current_t1_ids)}\n")
    fh.write(f"operational T1 (file tier==1)  = {len(op_t1_ids)}\n\n")

    for path_name, comp_fn in paths:
        new_qualifiers = []
        for iso, iso_props in by_iso.items():
            for p in iso_props:
                if evaluate_t1(p, iso_props, 0.20, comp_fn, civic_gate_a):
                    cid = _cid(p)
                    if cid not in current_t1_ids:
                        new_qualifiers.append(p)
        by_iso_count = defaultdict(int)
        for p in new_qualifiers:
            by_iso_count[p.get("iso") or "?"] += 1
        fh.write(f"\nPath {path_name}: {len(new_qualifiers)} NEW clusters\n")
        if by_iso_count:
            fh.write("  iso breakdown: " + ", ".join(
                f"{iso}={n}" for iso, n in sorted(by_iso_count.items(), key=lambda kv: -kv[1])) + "\n")
        new_qualifiers.sort(key=lambda x: float(x.get("pp") or 0), reverse=True)
        for p in new_qualifiers[:15]:
            cid = _cid(p)
            iso = p.get("iso")
            disp = p.get("display_name") or ""
            pp = float(p.get("pp") or 0)
            cur = op_tier_of.get(cid, "?")
            fh.write(f"    {iso} pp={pp:>10,.0f} [cur=T{cur}] {cid} | {disp}\n")

print(f"\nWrote {RESULTS_FILE}")
