#!/usr/bin/env python3
"""
sweep-archetype-parameters.py — comprehensive parameter sweep for the Urban Fringe
(VWH) and Commuter (PKS) archetypes.

Design: compute-once, sweep-cheap. The expensive work is distance computation
(nearest_metro over all transit anchors, nearest_cluster lookups). We precompute every
candidate's distances ONCE, cache them, then evaluate the full parameter grid in-memory
(milliseconds per combination).

Objective:
  Urban Fringe — maximise total subject to all-3-tiers populated and balance ≥ baseline.
  Commuter     — best balance-vs-Retail + tier-shape, with a soft pull toward a smaller
                 total than the current 2,969 ("less Commuter").

Outputs:
  work/archetype-precompute.json  — cached candidate pools (distances precomputed)
  work/sweep-results.json         — every combination, all metrics
  work/sweep-report.md            — ranked top-10 per archetype + auto-recommendation
"""

import importlib.util
import json
import math
from collections import Counter
from itertools import product
from pathlib import Path

BASE    = Path(__file__).parent
WORK    = BASE / "work"
TOTEBOX = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")
CLUSTERS_GEOJSON = WORK / "clusters.geojson"
PLACES_JSONL     = TOTEBOX / "service-places" / "cleansed-places.jsonl"
AIRPORTS_OSM     = TOTEBOX / "service-places" / "cleansed-civic-airports.jsonl"
RAILWAY_OSM      = TOTEBOX / "service-places" / "cleansed-civic-railway.jsonl"
PRECOMPUTE       = WORK / "archetype-precompute.json"

DISPLAY_ISO = {
    "US", "CA", "MX",
    "GB", "SE", "DK", "NO", "FI", "IS",
    "FR", "DE", "ES", "IT", "GR", "PL", "AT", "NL", "PT",
}
NA_ISO = {"US", "CA", "MX"}

# Retail reference tier split (global), from clusters-meta.json 2026-06-01
RETAIL_TIER_SPLIT = {1: 0.269, 2: 0.420, 3: 0.311}

_VWH_CATS = {"auto_parts", "paint", "mro_industrial", "flooring",
             "tool_rental", "lumber", "plumbing", "electrical", "welding"}

# ── helpers (replicated from test-cluster-archetypes.py) ──────────────────────

def _load_metro_lists():
    spec = importlib.util.spec_from_file_location(
        "score_regional_markets", BASE / "score-regional-markets.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod.NA_METROS, mod.EU_METROS

NA_METROS, EU_METROS = _load_metro_lists()
ALL_METROS = NA_METROS + EU_METROS


def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2 + math.cos(math.radians(lat1))
         * math.cos(math.radians(lat2)) * math.sin(dlon / 2) ** 2)
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def nearest_metro(lat, lon):
    best = float("inf")
    for mlat, mlon, _ in ALL_METROS:
        d = haversine(lat, lon, mlat, mlon)
        if d < best:
            best = d
    return best


def nearest_cluster_km(lat, lon, clusters, max_km):
    """Nearest cluster distance within max_km (bbox-prefiltered); None if none."""
    lat_margin = max_km / 111.0
    lon_margin = max_km / max(111.0 * math.cos(math.radians(lat)), 1.0)
    best = float("inf")
    for c in clusters:
        if abs(c["lat"] - lat) > lat_margin or abs(c["lon"] - lon) > lon_margin:
            continue
        d = haversine(lat, lon, c["lat"], c["lon"])
        if d < best:
            best = d
    return best if best <= max_km else None


def js_divergence(p: dict, q: dict) -> float:
    """Jensen-Shannon divergence between two count dicts (normalised to shares)."""
    keys = set(p) | set(q)
    ps = sum(p.values()) or 1
    qs = sum(q.values()) or 1
    P = {k: p.get(k, 0) / ps for k in keys}
    Q = {k: q.get(k, 0) / qs for k in keys}

    def _kl(a, b):
        s = 0.0
        for k in keys:
            if a[k] > 0 and b[k] > 0:
                s += a[k] * math.log2(a[k] / b[k])
        return s

    M = {k: 0.5 * (P[k] + Q[k]) for k in keys}
    return 0.5 * _kl(P, M) + 0.5 * _kl(Q, M)


def balance_score(country_counts: dict, retail_counts: dict) -> float:
    """1 - JS(archetype country shares ‖ retail country shares). 1.0 = perfect match."""
    return round(1.0 - js_divergence(country_counts, retail_counts), 4)

# ── PRECOMPUTE (once) ─────────────────────────────────────────────────────────

# precompute uses the largest threshold any grid combination will need
PRECOMPUTE_MAX_KM = 25.0


def build_precompute():
    print("Loading clusters.geojson …")
    with open(CLUSTERS_GEOJSON) as f:
        features = json.load(f)["features"]
    print(f"  {len(features):,} clusters")

    t1_index, t12_index = [], []
    vwh_pool = []
    retail_country = Counter()
    retail_tier = Counter()

    for feat in features:
        p = feat["properties"]
        lat, lon, tier, iso = p["seed_lat"], p["seed_lon"], p["tier"], p["iso"]
        retail_country[iso] += 1
        retail_tier[tier] += 1
        entry = {"lat": lat, "lon": lon, "id": p["cluster_id"]}
        if tier == 1:
            t1_index.append(entry)
        if tier in (1, 2):
            t12_index.append(entry)

        members_raw = p["members"]
        members = json.loads(members_raw) if isinstance(members_raw, str) else members_raw
        cats = {m["category"] for m in members}
        if "hardware" not in cats or "hypermarket" in cats:
            continue
        strength = len({m["category"] for m in members if m["category"] in _VWH_CATS})
        vwh_pool.append({
            "iso": iso, "span_km": round(p["span_km"], 2),
            "metro_dist": round(nearest_metro(lat, lon), 1),
            "vwh_strength": strength,
        })

    print(f"  T1={len(t1_index):,}  T1+T2={len(t12_index):,}  VWH-pool={len(vwh_pool):,}")

    # transit anchors
    def _load_jsonl(path, category):
        recs = []
        if not path.exists():
            return recs
        with open(path) as f:
            for line in f:
                try:
                    r = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if r.get("category_id") != category:
                    continue
                recs.append(r)
        return recs

    print("Loading transit anchors …")
    airports = _load_jsonl(AIRPORTS_OSM, "airport")
    railways = _load_jsonl(RAILWAY_OSM, "railway_station")
    for a in airports:
        a["transit_type"] = "airport"
    for r in railways:
        r["transit_type"] = "railway"
    transit = [t for t in (airports + railways) if t.get("iso_country_code") in DISPLAY_ISO]
    print(f"  {len(transit):,} anchors "
          f"({sum(1 for t in transit if t['transit_type']=='airport'):,} air / "
          f"{sum(1 for t in transit if t['transit_type']=='railway'):,} rail)")

    pks_pool = []
    n = len(transit)
    for i, ap in enumerate(transit):
        if i % 2000 == 0:
            print(f"  precompute … {i:,}/{n:,}", end="\r", flush=True)
        lat, lon = ap["latitude"], ap["longitude"]
        iso = ap.get("iso_country_code", "")
        md = nearest_metro(lat, lon)
        t1k = nearest_cluster_km(lat, lon, t1_index, PRECOMPUTE_MAX_KM)
        t12k = nearest_cluster_km(lat, lon, t12_index, PRECOMPUTE_MAX_KM)
        pks_pool.append({
            "iso": iso, "transit_type": ap["transit_type"],
            "metro_dist": round(md, 1),
            "t1_km": round(t1k, 1) if t1k is not None else None,
            "t12_km": round(t12k, 1) if t12k is not None else None,
        })
    print(f"\n  PKS-pool={len(pks_pool):,}")

    data = {
        "vwh_pool": vwh_pool,
        "pks_pool": pks_pool,
        "retail_country": dict(retail_country),
        "retail_tier": {str(k): v for k, v in retail_tier.items()},
    }
    WORK.mkdir(exist_ok=True)
    with open(PRECOMPUTE, "w") as f:
        json.dump(data, f)
    print(f"  cached → {PRECOMPUTE}")
    return data


def load_precompute(force=False):
    if PRECOMPUTE.exists() and not force:
        print(f"Using cached precompute {PRECOMPUTE} (delete to force rebuild)")
        with open(PRECOMPUTE) as f:
            return json.load(f)
    return build_precompute()

# ── SWEEP ─────────────────────────────────────────────────────────────────────

VWH_GRID = {
    "min_metro": [1.0, 3.0, 5.0],
    "max_metro": [80.0, 100.0, 120.0, 150.0],
    "max_span":  [5.0, 7.0, 10.0],
}

PKS_GRID = {
    "min_metro":  [15.0, 25.0, 35.0],
    "hub_excl":   [3.0, 5.0, 10.0],
    "integrated": [3.0, 5.0, 8.0],
    "t2_km":      [8.0, 12.0, 20.0],
    "t3_mode":    ["off", "airport-sweetspot", "any-sweetspot"],
}
PKS_MAX_METRO = 150.0
SWEETSPOT = (30.0, 100.0)


def eval_vwh(pool, p):
    res = {1: Counter(), 2: Counter(), 3: Counter()}
    for c in pool:
        if c["span_km"] > p["max_span"]:
            continue
        if not (p["min_metro"] <= c["metro_dist"] <= p["max_metro"]):
            continue
        s = c["vwh_strength"]
        tier = 1 if s >= 2 else (2 if s == 1 else 3)
        res[tier][c["iso"]] += 1
    return res


def eval_pks(pool, p):
    res = {1: Counter(), 2: Counter(), 3: Counter()}
    for a in pool:
        md = a["metro_dist"]
        if not (p["min_metro"] <= md <= PKS_MAX_METRO):
            continue
        # hub exclusion: T1 within hub_excl → skip
        if a["t1_km"] is not None and a["t1_km"] <= p["hub_excl"]:
            continue
        t12 = a["t12_km"]
        if t12 is not None and t12 <= p["integrated"]:
            res[1][a["iso"]] += 1
        elif t12 is not None and t12 <= p["t2_km"]:
            res[2][a["iso"]] += 1
        else:
            # standalone — T3 only under a bounded rule
            if p["t3_mode"] == "off":
                continue
            if not (SWEETSPOT[0] <= md <= SWEETSPOT[1]):
                continue
            if p["t3_mode"] == "airport-sweetspot" and a["transit_type"] != "airport":
                continue
            res[3][a["iso"]] += 1
    return res


def metrics(res, retail_country):
    t1 = sum(res[1].values())
    t2 = sum(res[2].values())
    t3 = sum(res[3].values())
    total = t1 + t2 + t3
    country = Counter()
    for tier in (1, 2, 3):
        country.update(res[tier])
    na = sum(v for k, v in country.items() if k in NA_ISO)
    eu = total - na
    bal = balance_score(dict(country), retail_country) if total else 0.0
    tshape = (1.0 - js_divergence({1: t1, 2: t2, 3: t3},
                                  {1: RETAIL_TIER_SPLIT[1], 2: RETAIL_TIER_SPLIT[2],
                                   3: RETAIL_TIER_SPLIT[3]})) if total else 0.0
    return {
        "total": total, "t1": t1, "t2": t2, "t3": t3,
        "na": na, "eu": eu,
        "na_pct": round(100 * na / total, 1) if total else 0.0,
        "balance": bal, "tier_shape": round(tshape, 4),
        "tier_ok": (t1 > 0 and t2 > 0 and t3 > 0),
        "country": dict(country),
    }


def main():
    import sys
    force = "--rebuild" in sys.argv
    data = load_precompute(force=force)
    retail_country = data["retail_country"]
    vwh_pool, pks_pool = data["vwh_pool"], data["pks_pool"]

    # ── VWH sweep ──
    vwh_results = []
    for mn, mx, sp in product(VWH_GRID["min_metro"], VWH_GRID["max_metro"], VWH_GRID["max_span"]):
        p = {"min_metro": mn, "max_metro": mx, "max_span": sp}
        m = metrics(eval_vwh(vwh_pool, p), retail_country)
        m["params"] = p
        vwh_results.append(m)

    # ── PKS sweep ──
    pks_results = []
    for mn, hx, ig, t2, t3m in product(
        PKS_GRID["min_metro"], PKS_GRID["hub_excl"], PKS_GRID["integrated"],
        PKS_GRID["t2_km"], PKS_GRID["t3_mode"]):
        if t2 <= ig:
            continue  # T2 boundary must exceed T1 boundary
        p = {"min_metro": mn, "hub_excl": hx, "integrated": ig, "t2_km": t2, "t3_mode": t3m}
        m = metrics(eval_pks(pks_pool, p), retail_country)
        m["params"] = p
        pks_results.append(m)

    # ── recommend ──
    # VWH: maximise total among tier_ok with decent balance; tie-break balance
    vwh_ok = [r for r in vwh_results if r["tier_ok"]]
    vwh_rec = max(vwh_ok, key=lambda r: (r["total"], r["balance"])) if vwh_ok else None

    # PKS: best 0.6*balance + 0.4*tier_shape among tier_ok, soft pull below 2969
    def pks_obj(r):
        size_pen = 0.15 if r["total"] > 2969 else 0.0
        return 0.6 * r["balance"] + 0.4 * r["tier_shape"] - size_pen
    pks_ok = [r for r in pks_results if r["tier_ok"]]
    pks_rec = max(pks_ok, key=pks_obj) if pks_ok else None

    with open(WORK / "sweep-results.json", "w") as f:
        json.dump({"vwh": vwh_results, "pks": pks_results,
                   "vwh_rec": vwh_rec, "pks_rec": pks_rec}, f, indent=2)

    # ── report ──
    def fmt_country(country):
        return "  ".join(f"{k}={v}" for k, v in
                         sorted(country.items(), key=lambda x: -x[1])[:10])

    lines = ["# Archetype Parameter Sweep — Report", ""]
    lines.append(f"Retail reference (balance target): "
                 f"{fmt_country(retail_country)}")
    lines.append("")

    lines.append("## Urban Fringe — top 10 by total (tier_ok only)")
    lines.append("| min | max | span | total | T1 | T2 | T3 | NA% | bal |")
    lines.append("|--|--|--|--|--|--|--|--|--|")
    for r in sorted(vwh_ok, key=lambda r: (-r["total"], -r["balance"]))[:10]:
        p = r["params"]
        lines.append(f"| {p['min_metro']} | {p['max_metro']} | {p['max_span']} | "
                     f"{r['total']} | {r['t1']} | {r['t2']} | {r['t3']} | "
                     f"{r['na_pct']} | {r['balance']} |")
    lines.append("")
    if vwh_rec:
        p = vwh_rec["params"]
        lines.append(f"**VWH recommendation:** VW_MIN={p['min_metro']} VW_MAX={p['max_metro']} "
                     f"SPAN={p['max_span']} → total={vwh_rec['total']} "
                     f"(T1={vwh_rec['t1']} T2={vwh_rec['t2']} T3={vwh_rec['t3']}) "
                     f"NA%={vwh_rec['na_pct']} balance={vwh_rec['balance']}")
        lines.append(f"  per-country: {fmt_country(vwh_rec['country'])}")
    lines.append("")

    lines.append("## Commuter — top 10 by objective (tier_ok only)")
    lines.append("| min | hub | T1≤ | T2≤ | t3mode | total | T1 | T2 | T3 | NA% | bal | shape |")
    lines.append("|--|--|--|--|--|--|--|--|--|--|--|--|")
    for r in sorted(pks_ok, key=pks_obj, reverse=True)[:10]:
        p = r["params"]
        lines.append(f"| {p['min_metro']} | {p['hub_excl']} | {p['integrated']} | "
                     f"{p['t2_km']} | {p['t3_mode']} | {r['total']} | {r['t1']} | "
                     f"{r['t2']} | {r['t3']} | {r['na_pct']} | {r['balance']} | "
                     f"{r['tier_shape']} |")
    lines.append("")
    if pks_rec:
        p = pks_rec["params"]
        lines.append(f"**PKS recommendation:** TN_MIN={p['min_metro']} HUB_EXCL={p['hub_excl']} "
                     f"INTEGRATED={p['integrated']} T2_KM={p['t2_km']} T3_MODE={p['t3_mode']} → "
                     f"total={pks_rec['total']} (T1={pks_rec['t1']} T2={pks_rec['t2']} "
                     f"T3={pks_rec['t3']}) NA%={pks_rec['na_pct']} balance={pks_rec['balance']}")
        lines.append(f"  per-country: {fmt_country(pks_rec['country'])}")
    lines.append("")
    lines.append("## Baseline (2026-06-01)")
    lines.append("- VWH: total=428 T1=58 T2=122 T3=248  NA%≈42")
    lines.append("- PKS: total=2969 T1=1148 T2=1821 T3=0  NA%≈26")

    report = "\n".join(lines)
    with open(WORK / "sweep-report.md", "w") as f:
        f.write(report)
    print("\n" + report)
    print(f"\nWrote {WORK/'sweep-report.md'} and {WORK/'sweep-results.json'}")


if __name__ == "__main__":
    main()
