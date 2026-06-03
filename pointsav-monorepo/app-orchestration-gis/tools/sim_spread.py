#!/usr/bin/env python3
"""
sim_spread.py — VWH (Urban Fringe) category-combination spread simulation.

Clusters the VWH chains ONCE (the DBSCAN is independent of the co-location rule), caches every
raw cluster as (metro, iso, frozenset(categories), member_count), then evaluates any number of
candidate qualify/tier rules instantly over the cache and reports geographic-spread metrics.

Usage:
    from sim_spread import build_cache, evaluate, CATS, STRONG, BROAD
    cache = build_cache()
    m = evaluate(cache, qualify=lambda cats: len(cats) >= 2)
    print(m)
Run directly to print the baseline + a few reference rules.
"""
import json, math, sys
from pathlib import Path
from collections import Counter, defaultdict
import importlib.util

SCRIPT_DIR = Path(__file__).parent
APP_DIR    = SCRIPT_DIR.parent

# Load the canonical build modules (single source of truth for chains, helpers, constants).
sys.argv = ["sim"]
_spec = importlib.util.spec_from_file_location("bvc", APP_DIR / "build-vwh-clusters.py")
bvc = importlib.util.module_from_spec(_spec); _spec.loader.exec_module(bvc)
_specp = importlib.util.spec_from_file_location("bpc", APP_DIR / "build-pks-clusters.py")
bpc = importlib.util.module_from_spec(_specp); _specp.loader.exec_module(bpc)

CATS   = list(bvc.VWH_CHAINS.keys())
STRONG = set(bvc._STRONG_CATS)            # mro_industrial, tool_rental, electrical, plumbing, lumber, flooring, welding
ANCHOR = {"hardware"}
WEAK   = {"auto_parts", "paint"}
BROAD  = {"hardware", "mro_industrial", "tool_rental", "electrical"}   # present in many countries


_VWH_CACHE = SCRIPT_DIR / ".cache-vwh.json"
_PKS_CACHE = SCRIPT_DIR / ".cache-pks.json"


def _load_disk(path):
    if path.exists():
        rows = json.loads(path.read_text())
        for r in rows:
            r["cats"] = frozenset(r["cats"])
        return rows
    return None


def _save_disk(path, rows):
    path.write_text(json.dumps([{**r, "cats": sorted(r["cats"])} for r in rows]))


def build_cache(use_disk=True):
    """Cluster the VWH chains once; return list of dicts: {metro, iso, cats, n}."""
    if use_disk:
        cached = _load_disk(_VWH_CACHE)
        if cached is not None:
            return cached
    all_recs = []
    for cat, chain_ids in bvc.VWH_CHAINS.items():
        for cid in chain_ids:
            for r in bvc.load_jsonl(bvc.CHAIN_DIR / f"{cid}.jsonl"):
                r["category"] = cat
                all_recs.append(r)
    comps = bvc.two_pass_dbscan(all_recs)
    cache = []
    for comp in comps:
        cats = frozenset(all_recs[i]["category"] for i in comp)
        # span filter (same as build)
        span = 0.0
        for a in range(len(comp)):
            for b in range(a + 1, len(comp)):
                d = bvc.haversine(all_recs[comp[a]]["lat"], all_recs[comp[a]]["lon"],
                                  all_recs[comp[b]]["lat"], all_recs[comp[b]]["lon"])
                if d > span:
                    span = d
        if span > bvc.VWH_SPAN_MAX_KM:
            continue
        lats = [all_recs[i]["lat"] for i in comp]; lons = [all_recs[i]["lon"] for i in comp]
        clat, clon = sum(lats) / len(lats), sum(lons) / len(lons)
        iso = Counter(all_recs[i]["iso"] for i in comp).most_common(1)[0][0]
        _, metro = bvc.nearest_metro(clat, clon)
        cache.append({"metro": metro, "iso": iso, "cats": cats, "n": len(comp)})
    if use_disk:
        _save_disk(_VWH_CACHE, cache)
    return cache


def build_cache_pks(use_disk=True):
    """Cluster the PKS transit nodes once; return list of dicts: {metro, iso, cats, n, major, metro_d}.
    `cats` is the transit category set WITH car_rental appended when present (matches build)."""
    if use_disk:
        cached = _load_disk(_PKS_CACHE)
        if cached is not None:
            return cached
    nodes = []
    nodes += bpc.load_transit(bpc.PLACES_DIR / "cleansed-civic-airports.jsonl", "airport")
    nodes += bpc.load_transit(bpc.PLACES_DIR / "cleansed-civic-railway.jsonl", "intercity_rail")
    cp = bpc.PLACES_DIR / "cleansed-civic-railway-commuter.jsonl"
    if cp.exists():
        nodes += bpc.load_transit(cp, "commuter_rail",
                                  transit_class_map={"subway": "metro_subway", "light_rail": "metro_subway", "metro": "metro_subway"})
    car = []
    for cid in bpc.CAR_RENTAL_CHAINS:
        p = bpc.CHAIN_DIR / f"{cid}.jsonl"
        if not p.exists():
            continue
        for line in open(p):
            try:
                r = json.loads(line); lat = r.get("latitude"); lon = r.get("longitude")
                iso = (r.get("iso_country_code") or "")[:2].upper()
                if lat and lon and iso in bpc.DISPLAY_ISO:
                    car.append({"lat": float(lat), "lon": float(lon)})
            except Exception:
                pass
    cgrid = bpc.build_grid(car)
    comps = bpc.group_by_proximity(nodes, bpc.TRANSIT_GROUP_KM)
    cache = []
    for comp in comps:
        lats = [nodes[i]["lat"] for i in comp]; lons = [nodes[i]["lon"] for i in comp]
        clat, clon = sum(lats) / len(lats), sum(lons) / len(lons)
        iso = Counter(nodes[i]["iso"] for i in comp).most_common(1)[0][0]
        cats = set(nodes[i]["category"] for i in comp)
        major = any(nodes[i]["category"] == "airport" and nodes[i].get("is_major") for i in comp)
        if bpc.any_within(clat, clon, car, cgrid, bpc.CAR_RENTAL_KM):
            cats.add("car_rental")
        metro_d, metro = bpc.nearest_metro(clat, clon)
        cache.append({"metro": metro, "iso": iso, "cats": frozenset(cats), "n": len(comp),
                      "major": major, "metro_d": round(metro_d, 2)})
    if use_disk:
        _save_disk(_PKS_CACHE, cache)
    return cache


def _gini(values):
    vals = sorted(values)
    n = len(vals)
    if n == 0 or sum(vals) == 0:
        return 0.0
    cum = 0
    for i, v in enumerate(vals, 1):
        cum += i * v
    return (2 * cum) / (n * sum(vals)) - (n + 1) / n


def evaluate(cache, qualify, tier_fn=None, per_metro_cap=None):
    """qualify(c) and tier_fn(c) take the FULL cluster dict. Returns geographic + tier spread metrics."""
    kept = [c for c in cache if qualify(c)]
    if per_metro_cap:
        by_metro = defaultdict(list)
        for c in kept:
            by_metro[c["metro"]].append(c)
        kept = []
        for m, lst in by_metro.items():
            lst.sort(key=lambda c: -c["n"])           # keep the largest per metro
            kept.extend(lst[:per_metro_cap])
    total = len(kept)
    if total == 0:
        return {"total": 0, "us_pct": 0, "countries": 0, "metros": 0, "top10_pct": 0, "gini": 0,
                "tiers": None, "tier_gini": None, "tier_metros": None}
    iso = Counter(c["iso"] for c in kept)
    metro = Counter(c["metro"] for c in kept)
    top10 = sum(n for _, n in metro.most_common(10))
    out = {
        "total": total,
        "us_pct": round(100 * iso["US"] / total, 1),
        "countries": len(iso),
        "metros": len(metro),
        "top10_pct": round(100 * top10 / total, 1),
        "gini": round(_gini(list(metro.values())), 3),   # geographic evenness (0 even … 1 lumpy)
        "by_country": dict(iso.most_common(8)),
    }
    if tier_fn:
        tcount = Counter(); tmetro = defaultdict(set); tiso = defaultdict(set)
        for c in kept:
            t = tier_fn(c)
            tcount[t] += 1; tmetro[t].add(c["metro"]); tiso[t].add(c["iso"])
        out["tiers"] = {t: tcount[t] for t in (1, 2, 3)}
        out["tier_gini"] = round(_gini([tcount[1], tcount[2], tcount[3]]), 3)  # tier-balance (0 = even thirds)
        out["tier_metros"] = {t: len(tmetro[t]) for t in (1, 2, 3)}            # per-tier city spread
        out["tier_countries"] = {t: len(tiso[t]) for t in (1, 2, 3)}
    return out


# ── Reference tier functions (current builds) — take full cluster dict ───────
def tier_baseline_vwh(c):
    cats = c["cats"]; strong = cats & STRONG; has_hw = "hardware" in cats
    if len(cats) >= 3: return 1
    if has_hw and len(strong) >= 2: return 1
    if has_hw and len(strong) >= 1: return 2
    if len(strong) >= 2: return 2
    return 3

def tier_baseline_pks(c):
    cats = c["cats"]; rail = cats & {"intercity_rail", "commuter_rail", "metro_subway"}
    has_air = "airport" in cats
    if has_air and len(cats) >= 2: return 1
    if len(cats) >= 3: return 1
    if len(rail) >= 2: return 2
    if has_air: return 2
    return 3

def qualify_vwh_baseline(c):
    return len(c["cats"]) >= 2

def qualify_pks_baseline(c):
    cats = c["cats"]
    if len(cats) >= 2: return True
    if cats == frozenset({"airport"}) and c["major"]: return True
    if cats == frozenset({"intercity_rail"}) and 3.0 <= c["metro_d"] <= 60.0: return True
    return False


def fmt(label, m):
    t = m.get("tiers") or {}
    tm = m.get("tier_metros") or {}
    return (f"{label:<44} total={m['total']:>5} US={m['us_pct']:>5}% metros={m['metros']:>3} "
            f"ctry={m['countries']:>2} geo_gini={m['gini']:<5} "
            f"T1/2/3={t.get(1,'-')}/{t.get(2,'-')}/{t.get(3,'-')} "
            f"tier_gini={m.get('tier_gini')} tierMetros={tm.get(1,'-')}/{tm.get(2,'-')}/{tm.get(3,'-')}")


if __name__ == "__main__":
    print("Clustering VWH once …")
    vwh = build_cache()
    print(f"  VWH raw clusters: {len(vwh):,}")
    print("Clustering PKS once …")
    pks = build_cache_pks()
    print(f"  PKS raw groups:   {len(pks):,}\n")

    print("=== URBAN FRINGE (VWH) ===")
    print(fmt("baseline", evaluate(vwh, qualify_vwh_baseline, tier_baseline_vwh)))
    print(fmt("require >=1 strong", evaluate(vwh, lambda c: len(c['cats']) >= 2 and bool(c['cats'] & STRONG), tier_baseline_vwh)))
    print(fmt("baseline + per-metro cap 20", evaluate(vwh, qualify_vwh_baseline, tier_baseline_vwh, per_metro_cap=20)))
    print(fmt("require strong + cap 25", evaluate(vwh, lambda c: len(c['cats']) >= 2 and bool(c['cats'] & STRONG), tier_baseline_vwh, per_metro_cap=25)))

    print("\n=== COMMUTER (PKS) ===")
    print(fmt("baseline", evaluate(pks, qualify_pks_baseline, tier_baseline_pks)))
    print(fmt("baseline + per-metro cap 20", evaluate(pks, qualify_pks_baseline, tier_baseline_pks, per_metro_cap=20)))
