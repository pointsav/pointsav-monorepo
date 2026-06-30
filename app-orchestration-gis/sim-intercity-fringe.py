#!/usr/bin/env python3
"""
sim-intercity-fringe.py — Intercity Fringe zone detection simulation
Alberta, Canada — first pass

Queries Overpass for all 9 IF category signals, clusters at 600m,
scores against three threshold variants, cross-checks hardware proximity.

Usage:
  python3 sim-intercity-fringe.py [--region AB|BC|SK|MB|WEST]
  Output: work/sim-if-<region>.json + printed table
"""

import json, math, sys, time, urllib.request, urllib.parse, argparse
from pathlib import Path
from collections import defaultdict

OVERPASS = "https://overpass-api.de/api/interpreter"
CLUSTER_KM = 0.6   # merge signals within 600m into one IF candidate

SERVICE_BUSINESS = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-business"
)
HARDWARE_CHAINS = ["home-depot-ca", "canadian-tire-ca", "rona-ca"]

# Region bounding boxes  south, west, north, east
REGIONS = {
    "AB":   (49.0, -120.0, 60.0, -110.0),
    "BC":   (48.3, -139.0, 60.0, -114.0),
    "SK":   (49.0, -110.0, 60.0, -101.5),
    "MB":   (49.0, -102.0, 60.0,  -95.0),
    "WEST": (49.0, -139.0, 60.0,  -95.0),
    # Tighter urban-corridor sub-sets (faster)
    "AB_URBAN": (49.0, -115.0, 54.5, -110.5),
}

# IF category → Overpass query blocks (node + way where applicable)
def make_query(bbox_str: str, cat: str) -> str:
    S = {
        "industrial_land": f"""
  way["landuse"="industrial"]({bbox_str});""",

        "auto_service": f"""
  node["shop"~"^(car_repair|tyres|car_parts)$"]({bbox_str});
  way["shop"~"^(car_repair|tyres|car_parts)$"]({bbox_str});""",

        "auto_dealer": f"""
  node["shop"="car"]({bbox_str});
  way["shop"="car"]({bbox_str});""",

        "trade_supply": f"""
  node["shop"~"^(trade|wholesale)$"]({bbox_str});
  way["shop"~"^(trade|wholesale)$"]({bbox_str});""",

        "staffing_labour": f"""
  node["office"="employment_agency"]({bbox_str});
  way["office"="employment_agency"]({bbox_str});""",

        "craft_trade": f"""
  node["craft"~"^(signmaker|upholsterer|glaziery|printer)$"]({bbox_str});
  way["craft"~"^(signmaker|upholsterer|glaziery|printer)$"]({bbox_str});""",

        "self_storage": f"""
  node["shop"="storage_rental"]({bbox_str});
  way["shop"="storage_rental"]({bbox_str});""",

        "microbrewery": f"""
  node["craft"="brewery"]({bbox_str});
  way["craft"="brewery"]({bbox_str});""",

        "construction_office": f"""
  node["office"~"^(construction_company|engineer)$"]({bbox_str});
  way["office"~"^(construction_company|engineer)$"]({bbox_str});""",
    }
    block = S[cat]
    return f"[out:json][timeout:120];\n({block}\n);\nout center;"


def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2
         + math.cos(math.radians(lat1))
         * math.cos(math.radians(lat2))
         * math.sin(dlon / 2) ** 2)
    return R * 2 * math.asin(math.sqrt(max(0, min(1, a))))


def fetch_overpass(q: str, cat: str):
    data = urllib.parse.urlencode({"data": q}).encode()
    req  = urllib.request.Request(OVERPASS, data=data, method="POST", headers={
        "Content-Type": "application/x-www-form-urlencoded",
        "User-Agent":   "foundry-gis-sim/1.0",
    })
    try:
        with urllib.request.urlopen(req, timeout=150) as r:
            return json.loads(r.read())
    except Exception as e:
        print(f"  WARN {cat}: {e}", file=sys.stderr)
        return {"elements": []}


def get_coord(el):
    if el["type"] == "node":
        return el.get("lat"), el.get("lon")
    c = el.get("center", {})
    return c.get("lat"), c.get("lon")


def cluster(points, radius_km):
    """Simple greedy clustering — fast enough for a few thousand points."""
    clusters = []   # {"cx","cy","members":[(lat,lon,cat)]}
    for lat, lon, cat in points:
        best = None
        best_d = radius_km + 1
        for cl in clusters:
            d = haversine(lat, lon, cl["cx"], cl["cy"])
            if d < best_d:
                best_d = d
                best = cl
        if best is not None:
            best["members"].append((lat, lon, cat))
            # recompute centroid
            lats = [m[0] for m in best["members"]]
            lons = [m[1] for m in best["members"]]
            best["cx"] = sum(lats) / len(lats)
            best["cy"] = sum(lons) / len(lons)
        else:
            clusters.append({"cx": lat, "cy": lon,
                             "members": [(lat, lon, cat)]})
    return clusters


def load_hardware(bbox):
    s, w, n, e = bbox
    hw = []
    for chain in HARDWARE_CHAINS:
        p = SERVICE_BUSINESS / f"{chain}.jsonl"
        if not p.exists():
            continue
        for line in p.read_text().splitlines():
            if not line.strip():
                continue
            try:
                r   = json.loads(line)
                lat = float(r.get("latitude", 0))
                lon = float(r.get("longitude", 0))
                if s <= lat <= n and w <= lon <= e:
                    hw.append((lat, lon, chain))
            except Exception:
                pass
    return hw


# ── Threshold variants ────────────────────────────────────────────────────────

SERVICE_CATS = {"auto_service", "trade_supply", "staffing_labour",
                "craft_trade", "self_storage"}

THRESHOLDS = {
    "strict":   lambda c: "industrial_land" in c and len(c & SERVICE_CATS) >= 3,
    "standard": lambda c: "industrial_land" in c and len(c & SERVICE_CATS) >= 2,
    "loose":    lambda c: len(c & (SERVICE_CATS | {"auto_dealer", "microbrewery"})) >= 3,
    # Hardware-anchored: industrial_land within 3km of hardware is implicit
    "hw_adjacent": lambda c: "industrial_land" in c and len(c & SERVICE_CATS) >= 1,
}


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--region", default="AB_URBAN",
                    choices=list(REGIONS.keys()),
                    help="Region to simulate")
    args = ap.parse_args()

    bbox     = REGIONS[args.region]
    bbox_str = f"{bbox[0]},{bbox[1]},{bbox[2]},{bbox[3]}"

    print(f"\nIntercity Fringe simulation — {args.region}")
    print(f"Bbox: {bbox}\n")

    # ── Step 1: fetch category signals ───────────────────────────────────────
    all_signals  = []
    cat_counts   = {}
    categories   = [
        "industrial_land", "auto_service", "auto_dealer",
        "trade_supply", "staffing_labour", "craft_trade",
        "self_storage", "microbrewery", "construction_office",
    ]

    for cat in categories:
        print(f"  [{cat:25s}] querying...", end=" ", flush=True)
        q   = make_query(bbox_str, cat)
        res = fetch_overpass(q, cat)
        pts = []
        for el in res.get("elements", []):
            lat, lon = get_coord(el)
            if lat and lon:
                pts.append((lat, lon, cat))
        cat_counts[cat] = len(pts)
        all_signals.extend(pts)
        print(f"{len(pts):4d} signals")
        time.sleep(2)

    print(f"\nTotal signals : {len(all_signals)}")

    # ── Step 2: cluster ───────────────────────────────────────────────────────
    print(f"Clustering at {CLUSTER_KM} km ...", flush=True)
    raw_clusters = cluster(all_signals, CLUSTER_KM)
    print(f"  {len(raw_clusters)} raw clusters")

    for cl in raw_clusters:
        cl["cats"] = set(m[2] for m in cl["members"])
        cl["n_cats"] = len(cl["cats"])
        cl["n_sig"]  = len(cl["members"])
        for name, fn in THRESHOLDS.items():
            cl[f"match_{name}"] = fn(cl["cats"])

    # ── Step 3: hardware proximity ────────────────────────────────────────────
    print("Loading hardware ...", flush=True)
    hw = load_hardware(bbox)
    print(f"  {len(hw)} hardware locations in region")

    for cl in raw_clusters:
        if hw:
            dists = sorted(
                (haversine(cl["cx"], cl["cy"], hlat, hlon), hchain)
                for hlat, hlon, hchain in hw
            )
            cl["hw_km"]    = round(dists[0][0], 2)
            cl["hw_chain"] = dists[0][1]
        else:
            cl["hw_km"]    = None
            cl["hw_chain"] = None

    # ── Step 4: print results ─────────────────────────────────────────────────
    print("\n" + "═" * 72)
    print("CATEGORY SIGNAL COUNTS")
    print("─" * 40)
    for cat in categories:
        bar = "█" * min(40, cat_counts[cat] // 5)
        print(f"  {cat:25s} {cat_counts[cat]:4d}  {bar}")

    print()
    for thresh in ["strict", "standard", "loose", "hw_adjacent"]:
        zones = sorted(
            [cl for cl in raw_clusters if cl[f"match_{thresh}"]],
            key=lambda c: (-c["n_cats"], -c["n_sig"])
        )
        print(f"\n{'─'*72}")
        print(f"THRESHOLD '{thresh}' — {len(zones)} IF candidate zones")
        print(f"{'─'*72}")
        for i, cl in enumerate(zones[:25], 1):
            cats_str = " ".join(sorted(cl["cats"]))
            hw_str   = (f"  ← {cl['hw_chain'].replace('-ca','')} "
                        f"{cl['hw_km']}km")  if cl["hw_km"] else ""
            print(f"  {i:3d}. ({cl['cx']:.4f},{cl['cy']:.4f})  "
                  f"cats={cl['n_cats']}  sig={cl['n_sig']:3d}  "
                  f"[{cats_str}]{hw_str}")
        if len(zones) > 25:
            print(f"       ... {len(zones)-25} more")

    # Hardware proximity summary (standard threshold)
    std_zones = [cl for cl in raw_clusters if cl["match_standard"]]
    if std_zones and hw:
        hw_dists = [cl["hw_km"] for cl in std_zones if cl["hw_km"] is not None]
        if hw_dists:
            w3 = sum(1 for d in hw_dists if d <= 3.0)
            w5 = sum(1 for d in hw_dists if d <= 5.0)
            med = sorted(hw_dists)[len(hw_dists) // 2]
            print(f"\n{'─'*72}")
            print(f"HARDWARE PROXIMITY — standard threshold ({len(std_zones)} zones)")
            print(f"  Within 3 km : {w3:3d}  ({100*w3//len(hw_dists)}%)")
            print(f"  Within 5 km : {w5:3d}  ({100*w5//len(hw_dists)}%)")
            print(f"  Median dist : {med:.1f} km")

    # ── Step 5: save JSON ─────────────────────────────────────────────────────
    out = Path(f"work/sim-if-{args.region.lower()}.json")
    out.parent.mkdir(exist_ok=True)
    payload = {
        "region": args.region,
        "bbox": bbox,
        "category_counts": cat_counts,
        "total_signals": len(all_signals),
        "total_clusters": len(raw_clusters),
        "thresholds": {},
    }
    for thresh in THRESHOLDS:
        payload["thresholds"][thresh] = [
            {
                "lat":       round(cl["cx"], 5),
                "lon":       round(cl["cy"], 5),
                "n_cats":    cl["n_cats"],
                "categories": sorted(cl["cats"]),
                "n_signals": cl["n_sig"],
                "hw_km":     cl["hw_km"],
                "hw_chain":  cl["hw_chain"],
            }
            for cl in raw_clusters if cl[f"match_{thresh}"]
        ]
    out.write_text(json.dumps(payload, indent=2))
    print(f"\nSaved → {out}\n")


if __name__ == "__main__":
    main()
