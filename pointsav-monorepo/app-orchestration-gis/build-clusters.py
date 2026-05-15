#!/usr/bin/env python3
"""
build-clusters.py — 3-Tier Leapfrog 2030 (Alpha-Secondary Restriction)

Multi-radius output: rank_1km / rank_3km per cluster.
Tier gate is retail-only (Alpha HW + Alpha WH). Civic (hospital/university)
contributes to proximity score and info card only — not to tier gate.
Proximity score: linear sum HW + WH + HC + HE alpha secondaries, max 400.
Chain sets imported exclusively from config.py — no inline overrides.
"""
import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import (
    SERVICE_BUSINESS_CLEANSED, SERVICE_PLACES_CLEANSED, WORK_DIR,
    REGION_CONFIG, ISO_TO_REGION, BOUNDARIES_DIR,
    ALPHA_ANCHORS, ALPHA_HARDWARE, ALPHA_WAREHOUSE,
    GENERIC_HARDWARE, GENERIC_WAREHOUSE,
    CHAIN_FAMILIES, CHAIN_SUB_LABELS, ANCHOR_DISPLAY_NAMES,
    SECONDARY_RADIUS_KM, TERTIARY_RADIUS_KM,
    CALIBRATION_THRESHOLD, DEFAULT_CATCHMENT_KM, DENSE_CATCHMENT_KM,
)
from utils.region_engine import RegionEngine

WORK_DIR.mkdir(parents=True, exist_ok=True)

RADII_KM = [1.0, 3.0]

DENSE_URBAN_BOXES = [
    (-79.7, 43.5, -79.1, 43.9),   # Toronto
    (-88.0, 41.7, -87.5, 42.1),   # Chicago
    (-118.7, 33.7, -118.0, 34.2), # Los Angeles
    (-74.1, 40.5, -73.7, 40.9),   # New York
    (-123.3, 49.0, -122.9, 49.4), # Vancouver
    (-3.9, 40.2, -2.5, 40.7),     # Madrid
]


def haversine_km(lat1, lon1, lat2, lon2) -> float:
    R = 6371.0
    phi1, phi2 = math.radians(lat1), math.radians(lat2)
    dphi = math.radians(lat2 - lat1)
    dlam = math.radians(lon2 - lon1)
    a = math.sin(dphi / 2) ** 2 + math.cos(phi1) * math.cos(phi2) * math.sin(dlam / 2) ** 2
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def linear_score(dist_km: float, max_dist_km: float) -> float:
    return max(0.0, (max_dist_km - dist_km) / max_dist_km) * 100.0


def load_cleansed_jsonl(path: Path) -> list:
    res = []
    if not path.exists():
        print(f"  WARNING: {path} not found")
        return res
    with open(path) as f:
        for line in f:
            try:
                res.append(json.loads(line))
            except Exception:
                pass
    return res


def build_grid(recs, size=0.1):
    g = {}
    for r in recs:
        cell = (int(float(r["latitude"]) / size), int(float(r["longitude"]) / size))
        g.setdefault(cell, []).append(r)
    return g


def query_grid_with_dist(plat, plon, g, r_km, size=0.1):
    """Returns list of (record, dist_km) for all records within r_km."""
    res = []
    deg = (r_km + 0.5) / 111.0
    for lat_c in range(int((plat - deg) / size), int((plat + deg) / size) + 1):
        for lon_c in range(int((plon - deg) / size), int((plon + deg) / size) + 1):
            for r in g.get((lat_c, lon_c), []):
                d = haversine_km(plat, plon, float(r["latitude"]), float(r["longitude"]))
                if d <= r_km:
                    res.append((r, d))
    return res


def within(recs_with_dist, r_km):
    return [r for r, d in recs_with_dist if d <= r_km]


def count_distinct_institutions(recs_with_dist, r_km, cluster_km=0.3) -> int:
    """Count distinct institutions within r_km by grouping OSM nodes within cluster_km.
    A large campus (100 OSM nodes all within 300m) counts as 1 institution."""
    candidates = [(r, d) for r, d in recs_with_dist if d <= r_km]
    if not candidates:
        return 0
    # Greedy spatial clustering — assign each node to the nearest already-seen centroid
    centroids: list[tuple[float, float]] = []
    for r, _ in candidates:
        lat = float(r["latitude"])
        lon = float(r["longitude"])
        assigned = False
        for clat, clon in centroids:
            if haversine_km(lat, lon, clat, clon) <= cluster_km:
                assigned = True
                break
        if not assigned:
            centroids.append((lat, lon))
    return len(centroids)


def nearest_dist(recs_with_dist, fallback):
    dists = [d for _, d in recs_with_dist]
    return min(dists) if dists else fallback


def evaluate_tier(nhw_a, nwh_a, nhw_g, nwh_g) -> int | None:
    """
    Returns tier 1–3 or None. Civic data is not a gate — info card only.

    T3 Apex:  Alpha_HW AND Alpha_WH both present
    T2 Hub:   Alpha_HW OR Alpha_WH (one alpha secondary)
    T1 Valid: Generic_HW or Generic_WH only (no alpha)
    None:     No secondary at all — not a cluster
    """
    h_a = len(nhw_a) > 0
    w_a = len(nwh_a) > 0
    h_g = len(nhw_g) > 0
    w_g = len(nwh_g) > 0

    if h_a and w_a:
        return 3
    if h_a or w_a:
        return 2
    if h_g or w_g:
        return 1
    return None


def compute_clusters():
    print("Loading cleansed data layers...")
    all_locs   = load_cleansed_jsonl(SERVICE_BUSINESS_CLEANSED)
    all_places = load_cleansed_jsonl(SERVICE_PLACES_CLEANSED)

    # Reverse-geocoding engine (offline, no runtime API calls)
    region_engine = RegionEngine(BOUNDARIES_DIR)
    print(f"  business: {len(all_locs)} records, places: {len(all_places)} records")

    print("Building spatial grids...")
    # Deduplicate civic records by coordinate (cross-country bbox overlap causes duplicates)
    def dedup_places(recs):
        seen, out = set(), []
        for r in recs:
            key = (round(float(r["latitude"]), 4), round(float(r["longitude"]), 4))
            if key not in seen:
                seen.add(key)
                out.append(r)
        return out

    hc_raw = [r for r in all_places if r.get("category_id") == "hospital"]
    he_raw = [r for r in all_places if r.get("category_id") == "university"]
    hc_g = build_grid(dedup_places(hc_raw))
    he_g = build_grid(dedup_places(he_raw))
    print(f"  hospitals: {sum(len(v) for v in hc_g.values())} (raw {len(hc_raw)}), "
          f"universities: {sum(len(v) for v in he_g.values())} (raw {len(he_raw)})")

    # Canonicalize sub-entities to their parent chain_id
    def canonical_cid(r):
        cid = r.get("chain_id", "")
        return CHAIN_FAMILIES.get(cid, cid)

    locs_by_cid = {}
    for r in all_locs:
        cid = canonical_cid(r)
        if cid:
            locs_by_cid.setdefault(cid, []).append(r)

    # Also index sub-entity chain_ids separately for info-card display
    sub_locs_by_cid = {}
    for r in all_locs:
        raw_cid = r.get("chain_id", "")
        if raw_cid in CHAIN_FAMILIES:
            sub_locs_by_cid.setdefault(raw_cid, []).append(r)

    max_r = max(RADII_KM)
    clusters = []

    for r_key, r_roles in REGION_CONFIG.items():
        cont = "NA" if r_key in ("US", "CA", "MX") else "EU"

        hw_a_recs = [r for cid in r_roles.get("hardware",  []) if cid in ALPHA_HARDWARE[cont]   for r in locs_by_cid.get(cid, [])]
        hw_g_recs = [r for cid in r_roles.get("hardware",  []) if cid in GENERIC_HARDWARE[cont]  for r in locs_by_cid.get(cid, [])]
        wh_a_recs = [r for cid in r_roles.get("warehouse", []) if cid in ALPHA_WAREHOUSE[cont]   for r in locs_by_cid.get(cid, [])]
        wh_g_recs = [r for cid in r_roles.get("warehouse", []) if cid in GENERIC_WAREHOUSE[cont] for r in locs_by_cid.get(cid, [])]

        hw_a_grid = build_grid(hw_a_recs)
        hw_g_grid = build_grid(hw_g_recs)
        wh_a_grid = build_grid(wh_a_recs)
        wh_g_grid = build_grid(wh_g_recs)

        for anchor_cid in r_roles.get("anchor", []):
            if anchor_cid not in ALPHA_ANCHORS[cont]:
                continue
            for pri in locs_by_cid.get(anchor_cid, []):
                plat = float(pri["latitude"])
                plon = float(pri["longitude"])

                # Single query at max radius (3km); sub-radii filter from results
                hw_a_wd = query_grid_with_dist(plat, plon, hw_a_grid, max_r)
                hw_g_wd = query_grid_with_dist(plat, plon, hw_g_grid, max_r)
                wh_a_wd = query_grid_with_dist(plat, plon, wh_a_grid, max_r)
                wh_g_wd = query_grid_with_dist(plat, plon, wh_g_grid, max_r)
                nhc_wd  = query_grid_with_dist(plat, plon, hc_g, TERTIARY_RADIUS_KM)
                nhe_wd  = query_grid_with_dist(plat, plon, he_g, TERTIARY_RADIUS_KM)

                nhc = within(nhc_wd, TERTIARY_RADIUS_KM)
                nhe = within(nhe_wd, TERTIARY_RADIUS_KM)
                # Count distinct institutions (300m cluster radius collapses campus sub-nodes)
                hc_distinct = count_distinct_institutions(nhc_wd, TERTIARY_RADIUS_KM, 0.3)
                he_distinct = count_distinct_institutions(nhe_wd, TERTIARY_RADIUS_KM, 0.3)

                # Evaluate tier at each secondary radius (civics not in gate)
                ranks = {}
                for r_km in RADII_KM:
                    tier = evaluate_tier(
                        within(hw_a_wd, r_km), within(wh_a_wd, r_km),
                        within(hw_g_wd, r_km), within(wh_g_wd, r_km),
                    )
                    ranks[r_km] = tier or 0

                if not any(ranks.values()):
                    continue

                # Decomposed proximity scores (alpha secondaries, max 400 total)
                d_hw = nearest_dist(hw_a_wd, max_r)
                d_wh = nearest_dist(wh_a_wd, max_r)
                d_hc = nearest_dist(nhc_wd, TERTIARY_RADIUS_KM)
                d_he = nearest_dist(nhe_wd, TERTIARY_RADIUS_KM)
                s_hw = linear_score(d_hw, max_r)
                s_wh = linear_score(d_wh, max_r)
                s_hc = linear_score(d_hc, TERTIARY_RADIUS_KM)
                s_he = linear_score(d_he, TERTIARY_RADIUS_KM)
                score = s_hw + s_wh + s_hc + s_he

                # Generic bonus score (informational, not used for national_rank)
                d_hw_g = nearest_dist(hw_g_wd, max_r)
                d_wh_g = nearest_dist(wh_g_wd, max_r)
                generic_score = linear_score(d_hw_g, max_r) + linear_score(d_wh_g, max_r)

                hw_ids = list({r.get("chain_id", "") for r, _ in hw_a_wd + hw_g_wd})
                wh_ids = list({r.get("chain_id", "") for r, _ in wh_a_wd + wh_g_wd})

                # Anchor details for click-to-inspect (per category, with cat field for dot coloring)
                anchor_details = []
                for r, d in sorted(hw_a_wd + hw_g_wd, key=lambda x: x[1]):
                    anchor_details.append({
                        "lat": round(float(r["latitude"]), 5),
                        "lon": round(float(r["longitude"]), 5),
                        "n": r.get("chain_id", ""),
                        "d": round(d, 2),
                        "cat": "hardware",
                        "addr": (r.get("street_address") or "").strip() or None,
                        "city": (r.get("city") or "").strip() or None,
                        "rgn":  (r.get("region") or "").strip() or None,
                    })
                for r, d in sorted(wh_a_wd + wh_g_wd, key=lambda x: x[1]):
                    anchor_details.append({
                        "lat": round(float(r["latitude"]), 5),
                        "lon": round(float(r["longitude"]), 5),
                        "n": r.get("chain_id", ""),
                        "d": round(d, 2),
                        "cat": "warehouse",
                        "addr": (r.get("street_address") or "").strip() or None,
                        "city": (r.get("city") or "").strip() or None,
                        "rgn":  (r.get("region") or "").strip() or None,
                    })
                for r, d in sorted(nhc_wd, key=lambda x: x[1])[:6]:
                    anchor_details.append({
                        "lat": round(float(r["latitude"]), 5),
                        "lon": round(float(r["longitude"]), 5),
                        "n": r.get("location_name", "Hospital"),
                        "d": round(d, 2),
                        "cat": "medical",
                        "addr": None,
                        "city": (r.get("city") or "").strip() or None,
                        "rgn":  (r.get("region") or "").strip() or None,
                    })
                for r, d in sorted(nhe_wd, key=lambda x: x[1])[:6]:
                    anchor_details.append({
                        "lat": round(float(r["latitude"]), 5),
                        "lon": round(float(r["longitude"]), 5),
                        "n": r.get("location_name", "University"),
                        "d": round(d, 2),
                        "cat": "academic",
                        "addr": None,
                        "city": (r.get("city") or "").strip() or None,
                        "rgn":  (r.get("region") or "").strip() or None,
                    })

                # Geometric centroid of commercial co-location stores (anchor + hardware + warehouse)
                _commercial = [(s['lat'], s['lon']) for s in anchor_details
                               if s.get('cat') in ('hardware', 'warehouse')]
                if _commercial:
                    _all_lats = [plat] + [s[0] for s in _commercial]
                    _all_lons = [plon] + [s[1] for s in _commercial]
                    centroid_lat = round(sum(_all_lats) / len(_all_lats), 5)
                    centroid_lon = round(sum(_all_lons) / len(_all_lons), 5)
                else:
                    centroid_lat, centroid_lon = round(plat, 5), round(plon, 5)

                # Commercial store counts at each radius (from anchor, hardware + warehouse only)
                count_1km = sum(1 for s in anchor_details
                                if s.get('cat') in ('hardware', 'warehouse') and s['d'] <= 1.0) + 1
                count_3km = sum(1 for s in anchor_details
                                if s.get('cat') in ('hardware', 'warehouse') and s['d'] <= 3.0) + 1

                # Tier descriptor: categorical composition (which families are present)
                _all_hw_ids = set().union(*ALPHA_HARDWARE.values()) | set().union(*GENERIC_HARDWARE.values())
                _all_wh_ids = set().union(*ALPHA_WAREHOUSE.values()) | set().union(*GENERIC_WAREHOUSE.values())
                if anchor_cid in _all_wh_ids:
                    _anchor_cat = 'Warehouse'
                elif anchor_cid in _all_hw_ids:
                    _anchor_cat = 'Hardware'
                else:
                    _anchor_cat = 'Hypermarket'
                _cats = set()
                _cats.add(_anchor_cat)
                if any(s.get('cat') == 'hardware' for s in anchor_details):
                    _cats.add('Hardware')
                if any(s.get('cat') == 'warehouse' for s in anchor_details):
                    _cats.add('Warehouse')
                # Plain-English tier nomenclature (Sprint 9): accessibility-friendly,
                # neurodivergent-clear; old labels (Full Complement / Home + Bulk Hub) used
                # compound nouns and "+" symbols that increased cognitive load.
                if 'Hypermarket' in _cats and 'Hardware' in _cats and 'Warehouse' in _cats:
                    tier_descriptor = 'Prime'
                elif 'Hypermarket' in _cats and 'Hardware' in _cats:
                    tier_descriptor = 'Strong (Retail)'
                elif 'Hypermarket' in _cats and 'Warehouse' in _cats:
                    tier_descriptor = 'Strong (Bulk)'
                elif 'Hardware' in _cats and 'Warehouse' in _cats:
                    tier_descriptor = 'Strong (Hub)'
                elif 'Hypermarket' in _cats:
                    tier_descriptor = 'Core (Hyper)'
                elif 'Hardware' in _cats:
                    tier_descriptor = 'Core (Hardware)'
                elif 'Warehouse' in _cats:
                    tier_descriptor = 'Core (Wholesale)'
                else:
                    tier_descriptor = 'Emerging'

                # Sub-entity display labels (within 200m of anchor)
                sub_labels = []
                anchor_sub_map = CHAIN_SUB_LABELS.get(anchor_cid, {})
                for sub_cid, label in anchor_sub_map.items():
                    for sub_rec in sub_locs_by_cid.get(sub_cid, []):
                        d = haversine_km(plat, plon, float(sub_rec["latitude"]), float(sub_rec["longitude"]))
                        if d <= 0.200:
                            sub_labels.append(label)
                            break

                is_dense = any(
                    b[0] <= plon <= b[2] and b[1] <= plat <= b[3]
                    for b in DENSE_URBAN_BOXES
                )

                # Geographic display name + offline reverse-geocoded region
                iso_str      = (pri.get("iso_country_code", "") or "").strip()
                anchor_label = ANCHOR_DISPLAY_NAMES.get(anchor_cid, anchor_cid)
                region_name  = region_engine.resolve(plat, plon, iso_str)

                # Settlement-level market name — derived from centroid, not POI addr:city
                cluster_id_tmp = (
                    f"c_{anchor_cid}_{round(plat, 3)}_{round(plon, 3)}"
                    .replace(".", "x").replace("-", "_")
                )
                city_str, mkt_conf = region_engine.resolve_market(
                    plat, plon, iso_str, cluster_id=cluster_id_tmp
                )
                state_str = (pri.get("region", "") or "").strip()

                if city_str:
                    display_name = f"{city_str}, {state_str}" if state_str else f"{city_str}, {iso_str}"
                else:
                    display_name = f"{anchor_label} ({iso_str})"

                clusters.append({
                    "cluster_id": (
                        f"c_{anchor_cid}_{round(plat, 3)}_{round(plon, 3)}"
                        .replace(".", "x").replace("-", "_")
                    ),
                    "rank_1km":   ranks[1.0],
                    "rank_3km":   ranks[3.0],
                    "score":      round(score, 1),
                    "score_hw":   round(s_hw, 1),
                    "score_wh":   round(s_wh, 1),
                    "score_hc":   round(s_hc, 1),
                    "score_he":   round(s_he, 1),
                    "generic_score": round(generic_score, 1),
                    "primary_anchor": anchor_cid,
                    "anchor_label": anchor_label,
                    "hw_list":    json.dumps(hw_ids),
                    "wh_list":    json.dumps(wh_ids),
                    "hc_count":   hc_distinct,
                    "he_count":   he_distinct,
                    "anchor_details": json.dumps(anchor_details),
                    "sub_entities_display": json.dumps(sub_labels),
                    "display_name": display_name,
                    "region_name": region_name,
                    "city":       city_str,
                    "mkt_conf":   mkt_conf,
                    "state":      state_str,
                    "iso":        iso_str,
                    "catchment_radius_km": DENSE_CATCHMENT_KM if is_dense else DEFAULT_CATCHMENT_KM,
                    "last_computed": "2026-05-08",
                    "count_1km":  count_1km,
                    "count_3km":  count_3km,
                    "centroid_lat": centroid_lat,
                    "centroid_lon": centroid_lon,
                    "tier_descriptor": tier_descriptor,
                    "_lat": plat,
                    "_lon": plon,
                })

    return clusters


def main():
    print("Computing clusters (1km / 3km secondary radii)...")
    cls = compute_clusters()
    print(f"  {len(cls)} clusters with at least one valid tier")

    # Tier-1 calibration check
    tier1_count = sum(1 for c in cls if c["rank_3km"] == 1)
    tier1_rate  = tier1_count / len(cls) if cls else 0
    print(f"  Tier-1 rate at 3km: {tier1_rate:.1%} (threshold {CALIBRATION_THRESHOLD:.0%})")
    if tier1_rate > CALIBRATION_THRESHOLD:
        print("  WARNING: Tier-1 rate exceeds calibration threshold — consider tightening secondary radius")

    # Tier distribution summary
    for t in [3, 2, 1]:
        n = sum(1 for c in cls if c["rank_3km"] == t)
        print(f"  T{t}: {n} clusters")

    # National rank by proximity score within each ISO country
    by_iso = {}
    for c in cls:
        by_iso.setdefault(c["iso"], []).append(c)
    for iso, country_cls in by_iso.items():
        country_cls.sort(key=lambda x: x["score"], reverse=True)
        for i, c in enumerate(country_cls):
            c["national_rank"] = f"{i + 1} of {len(country_cls)}"

    # National rank within tier (e.g., "3 of 113 T4 sites in US")
    by_iso_tier = {}
    for c in cls:
        key = (c["iso"], c["rank_3km"])
        by_iso_tier.setdefault(key, []).append(c)
    for (iso, tier), tier_cls in by_iso_tier.items():
        tier_cls.sort(key=lambda x: x["score"], reverse=True)
        for i, c in enumerate(tier_cls):
            c["national_rank_in_tier"] = f"{i + 1} of {len(tier_cls)} T{tier} in {iso}"

    # display_name collision detection: if two clusters share the same display_name,
    # prefix with anchor brand to disambiguate
    name_counts: dict = {}
    for c in cls:
        name_counts[c["display_name"]] = name_counts.get(c["display_name"], 0) + 1
    for c in cls:
        if name_counts[c["display_name"]] > 1:
            c["display_name"] = f"{c['anchor_label']} {c['display_name']}"

    features = [
        {
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [c["_lon"], c["_lat"]]},
            "properties": {k: v for k, v in c.items() if not k.startswith("_")},
        }
        for c in cls
    ]

    out = WORK_DIR / "clusters.geojson"
    with open(out, "w") as f:
        json.dump({"type": "FeatureCollection", "features": features}, f, indent=2)
    print(f"Written {len(cls)} clusters → {out}")


if __name__ == "__main__":
    main()
