#!/usr/bin/env python3
"""
sim-intercity-fringe.py — Intercity Fringe zone detection simulation

Queries Overpass for 9 IF category signals, clusters at 600m,
scores against threshold variants, cross-checks hardware + T1/T2/T3 proximity.

Usage:
  python3 sim-intercity-fringe.py [--region AB_URBAN|GB|FR|DE|...]
  Output: work/sim-if-<region>.json + printed table
"""

import json, math, sys, time, urllib.request, urllib.parse, argparse
from pathlib import Path
from collections import defaultdict

OVERPASS = "https://overpass-api.de/api/interpreter"
CLUSTER_KM   = 0.6    # merge signals within 600m into one IF candidate
QUERY_SLEEP  = 4      # seconds between Overpass requests (avoid 429)

SERVICE_BUSINESS = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-business"
)
CLUSTERS_GEOJSON = Path(
    "/srv/foundry/clones/project-gis/app-orchestration-gis/work/clusters.geojson"
)

# Per-region hardware chains
HARDWARE_BY_REGION = {
    "CA":   ["home-depot-ca", "canadian-tire-ca", "rona-ca"],
    "GB":   ["bq-uk", "screwfix-uk", "wickes-uk"],
    "FR":   ["leroy-merlin-fr", "castorama-fr", "brico-depot-fr", "bricomarch-fr"],
    "DE":   ["obi-de", "hornbach-de"],
    "ES":   ["leroy-merlin-es", "brico-depot-es", "bricomart-es", "bricoman-es"],
}

# Region → ISO code for hardware lookup + cluster filter
REGION_ISO = {
    "AB": "CA", "BC": "CA", "SK": "CA", "MB": "CA",
    "AB_URBAN": "CA", "WEST": "CA", "ON": "CA", "QC": "CA", "ATL": "CA",
    "GB": "GB", "FR": "FR", "DE": "DE", "ES": "ES",
}

# Region bounding boxes  south, west, north, east
#
# 2026-07-01: BC's east edge was -114.0, overlapping ~6 degrees into Alberta
# (Calgary sits at ~-114.07 — 19/176 zones leaked into BC's sweep per the BRIEF's
# measurement). Tightened to -120.0 to match AB's real west edge. MB's west edge
# similarly overlapped SK's east edge by 0.5 degrees — tightened to match.
# ON/QC/ATL have much larger overlaps (QC's -79.8 sits deep inside ON's box which
# extends to -74.3; ATL's -69.1 sits deep inside QC's box which extends to -57.1 —
# Montreal itself falls inside ON's box) — NOT fixed here. These provinces' real
# borders are irregular (rivers, not meridians), so a rectangle edit can't resolve
# them without excluding real territory; needs actual province polygon boundaries
# (point-in-polygon on cluster centroids) in a future session. See NEXT.md.
REGIONS = {
    # Canada
    "AB":       (49.0, -120.0, 60.0, -110.0),
    "BC":       (48.3, -139.0, 60.0, -120.0),
    "SK":       (49.0, -110.0, 60.0, -101.5),
    "MB":       (49.0, -101.5, 60.0,  -95.0),
    "WEST":     (49.0, -139.0, 60.0,  -95.0),
    "AB_URBAN": (49.0, -115.0, 54.5, -110.5),
    "ON":       (41.6,  -95.2, 56.9,  -74.3),   # Ontario — NOT fixed, see note above
    "QC":       (45.0,  -79.8, 62.6,  -57.1),   # Quebec — NOT fixed, see note above
    "ATL":      (43.3,  -69.1, 60.4,  -52.6),   # NB + NS + PE + NL combined — NOT fixed, see note above
    # Europe
    "GB":  (50.0,  -6.0, 58.7,  1.8),   # England + Wales + Scottish lowlands
    "FR":  (42.3,  -5.2, 51.2,  8.2),   # Metropolitan France
    "DE":  (47.3,   5.9, 55.1, 15.0),   # Germany
    "ES":  (35.9,  -9.3, 43.8,  4.4),   # Spain
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
    return f"[out:json][timeout:240];\n({block}\n);\nout center;"


def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2
         + math.cos(math.radians(lat1))
         * math.cos(math.radians(lat2))
         * math.sin(dlon / 2) ** 2)
    return R * 2 * math.asin(math.sqrt(max(0, min(1, a))))


def fetch_overpass(q: str, cat: str, retries: int = 3):
    data = urllib.parse.urlencode({"data": q}).encode()
    req  = urllib.request.Request(OVERPASS, data=data, method="POST", headers={
        "Content-Type": "application/x-www-form-urlencoded",
        "User-Agent":   "foundry-gis-sim/1.0",
    })
    for attempt in range(retries):
        try:
            with urllib.request.urlopen(req, timeout=280) as r:
                return json.loads(r.read())
        except urllib.error.HTTPError as e:
            if e.code in (429, 502, 503, 504) and attempt < retries - 1:
                wait = 15 * (attempt + 1)
                print(f"  {e.code}, retrying in {wait}s...", end=" ", flush=True)
                time.sleep(wait)
                continue
            print(f"  WARN {cat}: {e}", file=sys.stderr)
            return {"elements": []}
        except Exception as e:
            print(f"  WARN {cat}: {e}", file=sys.stderr)
            return {"elements": []}
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


def load_hardware(bbox, chains):
    s, w, n, e = bbox
    hw = []
    for chain in chains:
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


def fetch_transit_stations(bbox_str):
    """Passenger rail/metro/tram stations — used to detect CBD convergence points."""
    q = f"""
[out:json][timeout:240];
(
  node["railway"="station"]({bbox_str});
  node["railway"="halt"]({bbox_str});
  node["public_transport"="station"]({bbox_str});
);
out;
"""
    res = fetch_overpass(q, "transit_stations")
    pts = []
    for el in res.get("elements", []):
        lat, lon = get_coord(el)
        if lat and lon:
            pts.append((lat, lon, "station"))
    return pts


def find_cbd_anchors(stations, t_clusters, radius_km=1.5, min_stations=3):
    """CBD candidate = cluster of >=N distinct transit stations within radius_km
    of each other (multiple lines/routes converging on one downtown corridor).
    Label each anchor with the nearest existing T-cluster market name, if any."""
    clusters = cluster(stations, radius_km)
    anchors = []
    for cl in clusters:
        if len(cl["members"]) < min_stations:
            continue
        name = "?"
        if t_clusters:
            name = min(
                t_clusters,
                key=lambda t: haversine(cl["cx"], cl["cy"], t[0], t[1])
            )[3][:20]
        anchors.append({"lat": cl["cx"], "lon": cl["cy"],
                        "n_stations": len(cl["members"]), "name": name})
    return anchors


def load_clusters(bbox, iso):
    """Load existing T1/T2/T3 clusters for this country from clusters.geojson."""
    if not CLUSTERS_GEOJSON.exists():
        return []
    s, w, n, e = bbox
    data = json.loads(CLUSTERS_GEOJSON.read_text())
    pts  = []
    for f in data.get("features", []):
        p = f["properties"]
        if p.get("iso") != iso:
            continue
        c = f["geometry"]["coordinates"]
        lat, lon = c[1], c[0]
        if s <= lat <= n and w <= lon <= e:
            pts.append((lat, lon, p.get("tier", 0), p.get("market_name", "?")))
    return pts


# ── Threshold variants ────────────────────────────────────────────────────────

SERVICE_CATS   = {"auto_service", "trade_supply", "staffing_labour",
                  "craft_trade", "self_storage"}
NON_LAND_CATS  = SERVICE_CATS | {"auto_dealer", "microbrewery", "construction_office"}

THRESHOLDS = {
    "strict":   lambda c: "industrial_land" in c and len(c & SERVICE_CATS) >= 3,
    "standard": lambda c: "industrial_land" in c and len(c & SERVICE_CATS) >= 2,
    "loose":    lambda c: len(c & (SERVICE_CATS | {"auto_dealer", "microbrewery"})) >= 3,
    # Hardware-anchored: industrial_land within 3km of hardware is implicit
    "hw_adjacent": lambda c: "industrial_land" in c and len(c & SERVICE_CATS) >= 1,
    # standard's rule OR a stricter 4-of-8 non-land co-location bar — catches
    # commercial-zoned parcels that show the IF pattern but lack the OSM tag.
    "commercial_fallback": lambda c: (
        ("industrial_land" in c and len(c & SERVICE_CATS) >= 2)
        or len(c & NON_LAND_CATS) >= 4
    ),
}


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--region", default="AB_URBAN",
                    choices=list(REGIONS.keys()),
                    help=f"Region to simulate. Options: {', '.join(REGIONS)}")
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
        time.sleep(QUERY_SLEEP)

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

    # ── Step 3: hardware + T1/T2/T3 proximity ────────────────────────────────
    iso          = REGION_ISO.get(args.region, "CA")
    hw_chains    = HARDWARE_BY_REGION.get(iso, HARDWARE_BY_REGION["CA"])

    print("Loading hardware ...", flush=True)
    hw = load_hardware(bbox, hw_chains)
    print(f"  {len(hw)} hardware locations in region ({', '.join(hw_chains)})")

    print("Loading T1/T2/T3 clusters ...", flush=True)
    t_clusters = load_clusters(bbox, iso)
    print(f"  {len(t_clusters)} retail clusters ({iso}): "
          f"T1={sum(1 for c in t_clusters if c[2]==1)} "
          f"T2={sum(1 for c in t_clusters if c[2]==2)} "
          f"T3={sum(1 for c in t_clusters if c[2]==3)}")

    for cl in raw_clusters:
        # Hardware proximity
        if hw:
            dists = sorted(
                (haversine(cl["cx"], cl["cy"], hlat, hlon), hchain)
                for hlat, hlon, hchain in hw
            )
            cl["hw_km"]    = round(dists[0][0], 2)
            cl["hw_chain"] = dists[0][1].split("-")[0]   # strip country suffix
        else:
            cl["hw_km"]    = None
            cl["hw_chain"] = None

        # Nearest T1/T2/T3 cluster
        if t_clusters:
            tdists = sorted(
                (haversine(cl["cx"], cl["cy"], tlat, tlon), tier, name)
                for tlat, tlon, tier, name in t_clusters
            )
            cl["t_km"]     = round(tdists[0][0], 2)
            cl["t_tier"]   = tdists[0][1]
            cl["t_market"] = tdists[0][2][:25]
        else:
            cl["t_km"]     = None
            cl["t_tier"]   = None
            cl["t_market"] = None

    # ── Step 3b: CBD proximity ────────────────────────────────────────────────
    # CBD anchor = cluster of >=3 distinct transit stations within 1.5km
    # (multiple lines/routes converging on one downtown corridor).
    print("Loading transit stations (CBD detection) ...", flush=True)
    time.sleep(QUERY_SLEEP)
    stations = fetch_transit_stations(bbox_str)
    print(f"  {len(stations)} station nodes in region")
    cbd_anchors = find_cbd_anchors(stations, t_clusters)
    print(f"  {len(cbd_anchors)} CBD anchor(s) detected "
          f"(>=3 stations within 1.5km): "
          + ", ".join(f"{a['name']}({a['n_stations']})" for a in cbd_anchors[:10]))

    for cl in raw_clusters:
        if cbd_anchors:
            cdists = sorted(
                (haversine(cl["cx"], cl["cy"], a["lat"], a["lon"]), a["name"])
                for a in cbd_anchors
            )
            cl["cbd_km"]   = round(cdists[0][0], 2)
            cl["cbd_name"] = cdists[0][1]
        else:
            cl["cbd_km"]   = None
            cl["cbd_name"] = None

        # Nearest SINGLE transit station (any station, not just CBD-convergence
        # clusters) — tests whether ordinary transit access, not just downtown
        # proximity, is itself a co-location signal for IF zones.
        if stations:
            sdists = sorted(
                haversine(cl["cx"], cl["cy"], slat, slon)
                for slat, slon, _ in stations
            )
            cl["station_km"] = round(sdists[0], 2)
        else:
            cl["station_km"] = None

    # ── Step 4: print results ─────────────────────────────────────────────────
    print("\n" + "═" * 72)
    print("CATEGORY SIGNAL COUNTS")
    print("─" * 40)
    for cat in categories:
        bar = "█" * min(40, cat_counts[cat] // 5)
        print(f"  {cat:25s} {cat_counts[cat]:4d}  {bar}")

    print()
    for thresh in ["strict", "standard", "loose", "commercial_fallback"]:
        zones = sorted(
            [cl for cl in raw_clusters if cl[f"match_{thresh}"]],
            key=lambda c: (-c["n_cats"], -c["n_sig"])
        )
        print(f"\n{'─'*80}")
        print(f"THRESHOLD '{thresh}' — {len(zones)} IF candidate zones")
        print(f"{'─'*80}")
        for i, cl in enumerate(zones[:25], 1):
            cats_str = " ".join(c[:4] for c in sorted(cl["cats"]))
            hw_str   = f"hw={cl['hw_chain']} {cl['hw_km']}km" if cl["hw_km"] else "hw=none"
            t_str    = (f"T{cl['t_tier']} {cl['t_km']}km"
                        if cl["t_tier"] else "T=none")
            cbd_str  = (f"cbd={cl['cbd_name']} {cl['cbd_km']}km"
                        if cl["cbd_km"] is not None else "cbd=none")
            print(f"  {i:3d}. ({cl['cx']:.4f},{cl['cy']:.4f})  "
                  f"cats={cl['n_cats']}  sig={cl['n_sig']:3d}  "
                  f"{hw_str:<22}  {t_str:<14}  {cbd_str:<22}  [{cats_str}]")
        if len(zones) > 25:
            print(f"       ... {len(zones)-25} more")

    # CBD-closest ranking (standard threshold) — the zones this simulation exists to find
    std_for_cbd = [cl for cl in raw_clusters
                   if cl["match_standard"] and cl["cbd_km"] is not None]
    if std_for_cbd:
        closest = sorted(std_for_cbd, key=lambda c: c["cbd_km"])
        print(f"\n{'─'*80}")
        print(f"CLOSEST TO CBD — standard threshold, ranked by cbd_km "
              f"({len(std_for_cbd)} zones)")
        print(f"{'─'*80}")
        for i, cl in enumerate(closest[:20], 1):
            cats_str = " ".join(c[:4] for c in sorted(cl["cats"]))
            hw_str   = f"hw={cl['hw_chain']} {cl['hw_km']}km" if cl["hw_km"] else "hw=none"
            print(f"  {i:3d}. ({cl['cx']:.4f},{cl['cy']:.4f})  "
                  f"cbd={cl['cbd_name']} {cl['cbd_km']}km  "
                  f"cats={cl['n_cats']}  sig={cl['n_sig']:3d}  {hw_str}  [{cats_str}]")

    # Proximity summaries (standard threshold)
    std_zones = [cl for cl in raw_clusters if cl["match_standard"]]
    if std_zones:
        print(f"\n{'─'*80}")
        print(f"PROXIMITY SUMMARY — standard threshold ({len(std_zones)} zones)")

        if hw:
            hw_dists = [cl["hw_km"] for cl in std_zones if cl["hw_km"] is not None]
            if hw_dists:
                w3  = sum(1 for d in hw_dists if d <= 3.0)
                w5  = sum(1 for d in hw_dists if d <= 5.0)
                med = sorted(hw_dists)[len(hw_dists) // 2]
                print(f"  Hardware  within 3km: {w3:3d} ({100*w3//len(hw_dists)}%)  "
                      f"within 5km: {w5:3d} ({100*w5//len(hw_dists)}%)  "
                      f"median: {med:.1f}km")

        if t_clusters:
            for tier in [1, 2, 3]:
                td = [cl["t_km"] for cl in std_zones
                      if cl["t_tier"] == tier and cl["t_km"] is not None]
                # zones whose nearest cluster is this tier
                nt = [cl for cl in std_zones if cl["t_tier"] == tier]
                print(f"  T{tier} nearest    count: {len(nt):3d}  "
                      + (f"median dist: {sorted(td)[len(td)//2]:.1f}km" if td else ""))
            all_td = [cl["t_km"] for cl in std_zones if cl["t_km"] is not None]
            if all_td:
                t_w3 = sum(1 for d in all_td if d <= 3.0)
                t_w5 = sum(1 for d in all_td if d <= 5.0)
                print(f"  Any T-cluster within 3km: {t_w3} ({100*t_w3//len(all_td)}%)  "
                      f"within 5km: {t_w5} ({100*t_w5//len(all_td)}%)")

        if stations:
            st_dists = [cl["station_km"] for cl in std_zones if cl["station_km"] is not None]
            if st_dists:
                s1  = sum(1 for d in st_dists if d <= 1.0)
                s2  = sum(1 for d in st_dists if d <= 2.0)
                med = sorted(st_dists)[len(st_dists) // 2]
                print(f"  Nearest transit station (any, not just CBD anchors):")
                print(f"    within 1km: {s1:3d} ({100*s1//len(st_dists)}%)  "
                      f"within 2km: {s2:3d} ({100*s2//len(st_dists)}%)  "
                      f"median: {med:.1f}km")

        if cbd_anchors:
            cbd_dists = [cl["cbd_km"] for cl in std_zones if cl["cbd_km"] is not None]
            if cbd_dists:
                bands = [
                    ("0-5km   (inner-ring)",   0, 5),
                    ("5-10km  (near-ring)",    5, 10),
                    ("10-20km (outer-ring)",  10, 20),
                    ("20km+   (exurban)",     20, 10**6),
                ]
                med = sorted(cbd_dists)[len(cbd_dists) // 2]
                print(f"  CBD distance  median: {med:.1f}km  "
                      f"(n={len(cbd_dists)}, {len(cbd_anchors)} anchors detected)")
                for label, lo, hi in bands:
                    n = sum(1 for d in cbd_dists if lo <= d < hi)
                    print(f"    {label:<22} {n:4d} ({100*n//len(cbd_dists)}%)")

    # ── Step 5: save JSON ─────────────────────────────────────────────────────
    out = Path(f"work/sim-if-{args.region.lower()}.json")
    out.parent.mkdir(exist_ok=True)
    payload = {
        "region":          args.region,
        "iso":             iso,
        "bbox":            bbox,
        "category_counts": cat_counts,
        "total_signals":   len(all_signals),
        "total_clusters":  len(raw_clusters),
        "t_cluster_counts": {
            "T1": sum(1 for c in t_clusters if c[2] == 1),
            "T2": sum(1 for c in t_clusters if c[2] == 2),
            "T3": sum(1 for c in t_clusters if c[2] == 3),
        },
        "cbd_anchors": cbd_anchors,
        "thresholds": {},
    }
    for thresh in THRESHOLDS:
        payload["thresholds"][thresh] = [
            {
                "lat":        round(cl["cx"], 5),
                "lon":        round(cl["cy"], 5),
                "n_cats":     cl["n_cats"],
                "categories": sorted(cl["cats"]),
                "n_signals":  cl["n_sig"],
                "hw_km":      cl["hw_km"],
                "hw_chain":   cl["hw_chain"],
                "t_km":       cl["t_km"],
                "t_tier":     cl["t_tier"],
                "t_market":   cl["t_market"],
                "cbd_km":     cl["cbd_km"],
                "cbd_name":   cl["cbd_name"],
                "station_km": cl["station_km"],
            }
            for cl in raw_clusters if cl[f"match_{thresh}"]
        ]
    out.write_text(json.dumps(payload, indent=2))
    print(f"\nSaved → {out}\n")


if __name__ == "__main__":
    main()
