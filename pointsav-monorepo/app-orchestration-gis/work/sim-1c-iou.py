#!/usr/bin/env python3
"""
sim-1c-iou.py — IoU sensitivity + spatial coverage analysis (T1 tier recalibration).

PART A — IoU sweep at P=0.20 (rank_pp_iso<=0.20), civic gate = hc_count_regional>=1.
         Sweep IoU thresholds [0.05, 0.10, 0.15] using the build-geometric-ranking.py
         lens-area formula (exact match: equal-radius circles, r=3.0 km).

PART B — 3km unified radius assessment for civic anchors.
         Uses 'd' (km) field in anchor_details cat='medical' to bin within-3km vs 3-5km.

PART C — Spatial coverage at P=0.20 (composition + rank_pp_iso<=0.20 +
         hc_count_regional>=1, no IoU). Per-state US, per-country EU; plus
         proximity-crowding flag (>=5 qualifiers within 200km radius).

Results -> work/sim-1c-results.txt (also printed).
"""

import json
import math
import sys
from collections import defaultdict, Counter
from pathlib import Path

# --- Pull constants and helpers from the project (so logic matches exactly) ---
sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from config import (
    ALPHA_HYPERMARKET, ALPHA_LIFESTYLE, ALPHA_HARDWARE, ALPHA_WAREHOUSE,
    SECONDARY_RADIUS_KM,
)

WORK = Path(__file__).resolve().parent
CLUSTERS = WORK / "clusters.geojson"
CATCH    = WORK / "catchment-data.json"
OUT      = WORK / "sim-1c-results.txt"

DISK_RADIUS_KM = SECONDARY_RADIUS_KM   # 3.0 km — matches build-geometric-ranking.py
P_THRESHOLD    = 0.20                  # relaxed P (vs P10 default)
CIVIC_RADIUS_3KM = 3.0
CIVIC_RADIUS_5KM = 5.0

EU_ISOS = {"ES","IT","PL","NORDICS","DK","FI","NO","IS","FR","DE","GB","AT","NL","PT","GR","CZ","SE"}
NA_ISOS = {"US","CA","MX"}

_hyper_ids     = set().union(*ALPHA_HYPERMARKET.values())
_lifestyle_ids = set().union(*ALPHA_LIFESTYLE.values())
_hw_ids        = set().union(*ALPHA_HARDWARE.values())
_wh_ids        = set().union(*ALPHA_WAREHOUSE.values())

# --- Helpers (copied verbatim from build-geometric-ranking.py) ---

def haversine_km(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat/2)**2
         + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2))
         * math.sin(dlon/2)**2)
    return 2.0 * R * math.asin(math.sqrt(max(0.0, min(1.0, a))))

def iou_equal_circles(dist_km, r_km):
    if dist_km >= 2.0 * r_km: return 0.0
    if dist_km <= 0.0:        return 1.0
    d, r = dist_km, r_km
    lens = 2.0*r*r*math.acos(d/(2.0*r)) - (d/2.0)*math.sqrt(4.0*r*r - d*d)
    circle = math.pi * r * r
    union = 2.0*circle - lens
    return lens/union if union > 0 else 0.0

def parse_list(v):
    if isinstance(v, list): return v
    if isinstance(v, str):
        try: return json.loads(v)
        except: return []
    return []

def anchor_classes(p):
    classes = set()
    anc        = p.get("primary_anchor") or ""
    hw_list    = parse_list(p.get("hw_list",    []))
    wh_list    = parse_list(p.get("wh_list",    []))
    hyper_list = parse_list(p.get("hyper_list", []))
    ls_list    = parse_list(p.get("ls_list",    []))
    if anc in _hyper_ids     or any(c in _hyper_ids     for c in hyper_list): classes.add("Hypermarket")
    if anc in _lifestyle_ids or any(c in _lifestyle_ids for c in ls_list):    classes.add("Lifestyle")
    if anc in _hw_ids        or any(c in _hw_ids        for c in hw_list):    classes.add("Hardware")
    if anc in _wh_ids        or any(c in _wh_ids        for c in wh_list):    classes.add("Warehouse")
    return classes

def t1_composition(p):
    cls = anchor_classes(p)
    return (("Warehouse" in cls and "Hypermarket" in cls)
            or ("Lifestyle" in cls and "Hypermarket" in cls))

# --- Load data ---

print("Loading clusters.geojson + catchment-data.json ...")
with open(CLUSTERS) as f:
    g = json.load(f)
features = g["features"]
print(f"  {len(features):,} clusters")

with open(CATCH) as f:
    catch = json.load(f)
print(f"  {len(catch):,} catchment records")

# Merge per-ISO ranks (same merge build-geometric-ranking.py performs)
rank_fields = ["rank_pp_iso","rank_sp_iso","rank_pg_iso","rank_sg_iso",
               "rank_ph_iso","rank_sh_iso","rank_pw_iso","rank_sw_iso",
               "pp","sp"]
merged = 0
for feat in features:
    p = feat["properties"]
    cid = p.get("cluster_id") or p.get("id") or ""
    if cid in catch:
        for k in rank_fields:
            if k in catch[cid]: p[k] = catch[cid][k]
        merged += 1
print(f"  Merged {merged:,} catchment records onto cluster props")

# Build flat prop list with centroid (prefer Point geometry, else centroid_lat/lon)
clusters = []
for feat in features:
    p = dict(feat["properties"])
    geom = feat.get("geometry") or {}
    if geom.get("type") == "Point" and geom.get("coordinates"):
        lon, lat = geom["coordinates"][0], geom["coordinates"][1]
    else:
        lat = float(p.get("centroid_lat") or 0)
        lon = float(p.get("centroid_lon") or 0)
    p["_lat"], p["_lon"] = float(lat), float(lon)
    clusters.append(p)

# --- PART A: IoU sweep at P=0.20 -------------------------------------------

def qualifies_pre_iou(p):
    """T1 composition + rank_pp_iso<=P_THRESHOLD + hc_count_regional>=1."""
    if not t1_composition(p): return False
    if float(p.get("rank_pp_iso") or 1.0) > P_THRESHOLD: return False
    if int(p.get("hc_count_regional") or 0) < 1: return False
    return True

# Precompute pre-IoU candidates grouped by ISO
by_iso_all = defaultdict(list)
for p in clusters:
    by_iso_all[p.get("iso") or ""].append(p)

pre_iou_candidates = [p for p in clusters if qualifies_pre_iou(p)]
print(f"\nPART A — Pre-IoU candidates (composition + rank_pp_iso<=0.20 + hc_reg>=1): "
      f"{len(pre_iou_candidates):,}")

def t1_count_with_iou(iou_max):
    """Apply IoU dedup: drop candidate if any stronger peer in same ISO violates IoU<=iou_max.
    Stronger = higher pp. Matches build-geometric-ranking.py logic, P=0.20 instead of P10."""
    survivors = []
    removed   = []
    for p in pre_iou_candidates:
        iso = p.get("iso") or ""
        cid = p.get("cluster_id") or ""
        pp  = float(p.get("pp") or 0)
        # Iterate all ISO peers (entire by_iso_all[iso], same as build-geometric-ranking.py
        # which iterates iso_props — not just other candidates).
        dropped = False
        for peer in by_iso_all[iso]:
            if peer is p: continue
            if (peer.get("cluster_id") or "") == cid: continue
            if float(peer.get("pp") or 0) <= pp: continue
            d = haversine_km(p["_lat"], p["_lon"], peer["_lat"], peer["_lon"])
            if iou_equal_circles(d, DISK_RADIUS_KM) > iou_max:
                dropped = True
                break
        if dropped: removed.append(p)
        else:       survivors.append(p)
    return survivors, removed

part_a_results = {}
for thresh in [0.05, 0.10, 0.15]:
    surv, rem = t1_count_with_iou(thresh)
    na = sum(1 for p in surv if (p.get("iso") or "") in NA_ISOS)
    eu = sum(1 for p in surv if (p.get("iso") or "") in EU_ISOS)
    part_a_results[thresh] = {
        "survivors": len(surv), "removed": len(rem), "NA": na, "EU": eu,
        "by_iso": Counter(p.get("iso") for p in surv),
    }
    print(f"  IoU<={thresh:.2f}: T1={len(surv)}  (NA={na}  EU={eu})  removed={len(rem)}")

# --- PART B: 3km-vs-3-5km civic coverage -----------------------------------

# Civic anchors live in anchor_details with cat='medical' (hospital). Distance 'd' is km
# from cluster centroid to that anchor; build-clusters.py only includes anchors within
# TERTIARY_RADIUS_KM (5.0).
#
# hc_count_regional (build-clusters.py) is "distinct regional-tier hospitals within 5km",
# but the geojson does not carry the per-anchor tier (regional/district), only the cat.
# So we cannot break hc_count_regional itself by 3km vs 3-5km without re-ingesting tier
# data. We instead measure on the *raw* medical anchors in anchor_details: what fraction
# of T1 clusters have ANY medical anchor within 3km vs only 3-5km? This is a valid proxy
# because a cluster whose only medicals sit 3-5km would lose its civic gate entirely
# under a unified 3km radius.

# Use the current T1 set (tier=1) plus pre-IoU P=0.20 candidates (broader audit).
current_t1 = [p for p in clusters if p.get("tier") == 1]
print(f"\nPART B — civic distance binning (3km vs 3-5km)")
print(f"  Current T1 clusters: {len(current_t1)}")

def bin_civic(t1_set, label):
    in_3km   = 0  # has at least one medical within 3km
    only_3_5 = 0  # has medicals but all >=3km (would lose civic gate at 3km)
    no_med   = 0  # no medical anchors recorded at all
    total_meds_3km   = 0
    total_meds_3_5km = 0
    for p in t1_set:
        ad = parse_list(p.get("anchor_details", []))
        meds = [e for e in ad if e.get("cat") == "medical" and e.get("d") is not None]
        if not meds:
            no_med += 1
            continue
        n_in   = sum(1 for m in meds if float(m["d"]) <= 3.0)
        n_band = sum(1 for m in meds if 3.0 < float(m["d"]) <= 5.0)
        total_meds_3km   += n_in
        total_meds_3_5km += n_band
        if n_in > 0: in_3km += 1
        else:        only_3_5 += 1
    print(f"  [{label}] n={len(t1_set)}")
    print(f"    >=1 medical within 3km        : {in_3km}  ({in_3km/max(1,len(t1_set)):.1%})")
    print(f"    only-medicals in 3-5km band   : {only_3_5}  ({only_3_5/max(1,len(t1_set)):.1%})  <-- would lose civic gate at unified 3km")
    print(f"    no medical anchors in details : {no_med}")
    print(f"    total medical anchors  <=3km  : {total_meds_3km}")
    print(f"    total medical anchors 3-5km   : {total_meds_3_5km}")
    return {"in_3km": in_3km, "only_3_5km": only_3_5, "no_med": no_med,
            "med_count_3km": total_meds_3km, "med_count_3_5km": total_meds_3_5km}

part_b_current = bin_civic(current_t1, "Current T1 (tier==1)")

# Also report on the P=0.20 pre-IoU candidates so the operator can see how many
# would still pass civic if radius were tightened to 3km.
part_b_p20 = bin_civic(pre_iou_candidates, "P=0.20 pre-IoU candidates")

print(
    "\n  Note: hc_count_regional is computed in build-clusters.py with "
    "TERTIARY_RADIUS_KM=5.0 *and* uses the per-anchor hospital_tier (regional/district) "
    "metadata that is NOT exposed through anchor_details in clusters.geojson. The "
    "binning above measures raw 'medical' anchors as a proxy. A unified 3km radius "
    "would require rerunning build-clusters.py with TERTIARY_RADIUS_KM=3.0; the "
    "'only-medicals in 3-5km band' counter is the upper bound on T1 clusters that "
    "would lose their civic gate under that change."
)

# --- PART C: Spatial coverage at P=0.20 (no IoU) ---------------------------

print(f"\nPART C — spatial coverage (composition + rank_pp_iso<=0.20 + hc_count_regional>=1; NO IoU)")
print(f"  Total qualifiers: {len(pre_iou_candidates):,}")

# US per-state
us_quals = [p for p in pre_iou_candidates if (p.get("iso") or "") == "US"]
us_state_ct = Counter()
us_no_state = 0
for p in us_quals:
    st = (p.get("state") or "").strip()
    if not st:
        # fallback: infer from region_name if state is empty
        rn = (p.get("region_name") or "").strip()
        st = f"(no-state, region={rn})" if rn else "(unknown)"
        us_no_state += 1
    us_state_ct[st] += 1

# All 50 US states + DC for gap detection
ALL_US_STATES = {
    "AL","AK","AZ","AR","CA","CO","CT","DE","FL","GA","HI","ID","IL","IN","IA","KS",
    "KY","LA","ME","MD","MA","MI","MN","MS","MO","MT","NE","NV","NH","NJ","NM","NY",
    "NC","ND","OH","OK","OR","PA","RI","SC","SD","TN","TX","UT","VT","VA","WA","WV",
    "WI","WY","DC"
}
us_states_with = {s for s in us_state_ct if s in ALL_US_STATES}
us_states_gap = sorted(ALL_US_STATES - us_states_with)

print(f"\n  US qualifiers: {len(us_quals)} (across {len(us_states_with)} states; "
      f"{us_no_state} with no state field)")
print(f"  Top US states:")
for st, ct in us_state_ct.most_common(15):
    print(f"    {st:>30}: {ct}")
print(f"  US states with 0 qualifiers ({len(us_states_gap)}): {', '.join(us_states_gap)}")

# EU per-country
eu_quals = [p for p in pre_iou_candidates if (p.get("iso") or "") in EU_ISOS]
eu_iso_ct = Counter(p.get("iso") for p in eu_quals)
eu_with_clusters = {iso for iso, lst in by_iso_all.items() if iso in EU_ISOS and len(lst) > 0}
eu_gap = sorted(eu_with_clusters - set(eu_iso_ct.keys()))
print(f"\n  EU qualifiers: {len(eu_quals)} (across {len(eu_iso_ct)} ISO codes)")
for iso, ct in sorted(eu_iso_ct.items(), key=lambda kv: -kv[1]):
    total = len(by_iso_all[iso])
    print(f"    {iso:>8}: {ct}  (of {total} clusters in ISO)")
print(f"  EU ISOs with clusters but 0 qualifiers: {', '.join(eu_gap) if eu_gap else '(none)'}")

# Other (CA, MX, etc. — NA non-US)
other_quals = [p for p in pre_iou_candidates if (p.get("iso") or "") in (NA_ISOS - {"US"})]
other_ct = Counter(p.get("iso") for p in other_quals)
print(f"\n  Other NA qualifiers: {len(other_quals)}  {dict(other_ct)}")

# Crowding: any 200km subregion with >=5 qualifiers
print(f"\n  Proximity crowding (>=5 qualifiers within 200km of a centroid):")
crowding_hotspots = []
for i, p in enumerate(pre_iou_candidates):
    near = 0
    near_ids = []
    for j, q in enumerate(pre_iou_candidates):
        if i == j: continue
        d = haversine_km(p["_lat"], p["_lon"], q["_lat"], q["_lon"])
        if d <= 200.0:
            near += 1
            near_ids.append(q.get("cluster_id"))
    if near >= 4:  # >=5 total qualifiers including self
        crowding_hotspots.append({
            "cluster_id": p.get("cluster_id"),
            "iso": p.get("iso"),
            "state_or_region": p.get("state") or p.get("region_name"),
            "lat": p["_lat"], "lon": p["_lon"],
            "neighbors_within_200km": near,
            "total_in_hotspot": near + 1,
        })

# Cluster the hotspots into rough "centers" by collapsing duplicates: each hotspot row
# is already a centroid of a >=5 group. Sort by density.
crowding_hotspots.sort(key=lambda h: -h["total_in_hotspot"])
# Greedy de-dup: drop a hotspot if it's within 100km of a stronger one we already kept.
unique_hot = []
for h in crowding_hotspots:
    too_close = False
    for u in unique_hot:
        if haversine_km(h["lat"], h["lon"], u["lat"], u["lon"]) <= 100:
            too_close = True; break
    if not too_close:
        unique_hot.append(h)

print(f"    {len(crowding_hotspots)} candidate centroids with >=5 in 200km; "
      f"{len(unique_hot)} unique hotspot centers (collapsed to nearest 100km).")
for h in unique_hot[:25]:
    print(f"    {h['iso']:>3} {h['state_or_region']:<45}  "
          f"({h['lat']:.3f},{h['lon']:.3f})  total={h['total_in_hotspot']}")

# --- Persist results -------------------------------------------------------

OUT.write_text("")  # truncate
with OUT.open("w") as f:
    def w(s=""): f.write(s + "\n"); print(s)
    w("=" * 78)
    w("sim-1c-iou.py — T1 IoU sensitivity + spatial coverage")
    w("Config: DISK_RADIUS_KM=3.0, P_THRESHOLD=0.20, civic gate=hc_count_regional>=1")
    w("IoU formula: closed-form lens area for equal-radius circles (matches build-geometric-ranking.py)")
    w("=" * 78)

    w("\nPART A — IoU sweep")
    w(f"  Pre-IoU candidates: {len(pre_iou_candidates):,}")
    w(f"  {'IoU<=':>8}  {'survivors':>10}  {'removed':>8}  {'NA':>6}  {'EU':>6}")
    for thr in [0.05, 0.10, 0.15]:
        r = part_a_results[thr]
        w(f"  {thr:>8.2f}  {r['survivors']:>10}  {r['removed']:>8}  {r['NA']:>6}  {r['EU']:>6}")
    w("\n  By-ISO breakdown of survivors:")
    for thr in [0.05, 0.10, 0.15]:
        r = part_a_results[thr]
        rows = ", ".join(f"{k}={v}" for k, v in r["by_iso"].most_common())
        w(f"    IoU<={thr:.2f}: {rows}")

    w("\nPART B — civic distance binning (3km vs 3-5km, raw medical anchors)")
    for label, d in [("Current T1 (tier==1)", part_b_current),
                     ("P=0.20 pre-IoU candidates", part_b_p20)]:
        w(f"  [{label}]")
        w(f"    >=1 medical within 3km        : {d['in_3km']}")
        w(f"    only-medicals in 3-5km band   : {d['only_3_5km']}  <-- T1 candidates that would")
        w(f"                                                       lose civic gate at unified 3km radius")
        w(f"    no medical anchors recorded   : {d['no_med']}")
        w(f"    total medical anchors <=3km   : {d['med_count_3km']}")
        w(f"    total medical anchors 3-5km   : {d['med_count_3_5km']}")
    w("\n  CAVEAT: hc_count_regional uses a per-anchor hospital_tier classification (regional/district)")
    w("  loaded from the hospital ingest, which is NOT exposed through anchor_details. The numbers")
    w("  above are an upper bound on T1 clusters whose civic gate is satisfied ONLY by hospitals in")
    w("  the 3-5km band. Re-running build-clusters.py with TERTIARY_RADIUS_KM=3.0 (and re-running")
    w("  build-geometric-ranking.py) is the only way to get exact numbers.")

    w("\nPART C — spatial coverage at P=0.20 (no IoU)")
    w(f"  Total qualifiers: {len(pre_iou_candidates):,}")
    w(f"  US qualifiers: {len(us_quals)} (across {len(us_states_with)} states)")
    w("  Top 15 US states:")
    for st, ct in us_state_ct.most_common(15):
        w(f"    {st:>32}: {ct}")
    w(f"  US states with 0 qualifiers ({len(us_states_gap)}):")
    w(f"    {', '.join(us_states_gap)}")
    w(f"\n  EU qualifiers: {len(eu_quals)} (across {len(eu_iso_ct)} ISO codes)")
    for iso, ct in sorted(eu_iso_ct.items(), key=lambda kv: -kv[1]):
        total = len(by_iso_all[iso])
        w(f"    {iso:>8}: {ct}  (of {total} clusters in ISO)")
    w(f"  EU ISOs with clusters but 0 qualifiers: {', '.join(eu_gap) if eu_gap else '(none)'}")
    w(f"\n  Other NA (CA, MX) qualifiers: {len(other_quals)}  {dict(other_ct)}")

    w("\n  Proximity crowding (>=5 qualifiers within 200km radius):")
    w(f"    {len(crowding_hotspots)} candidate centroids meet threshold; "
      f"{len(unique_hot)} unique hotspots after 100km collapse.")
    for h in unique_hot[:25]:
        w(f"    {h['iso']:>3} {(h['state_or_region'] or '')[:45]:<45} "
          f"({h['lat']:.3f},{h['lon']:.3f})  total={h['total_in_hotspot']}")

print(f"\nResults written to {OUT}")
