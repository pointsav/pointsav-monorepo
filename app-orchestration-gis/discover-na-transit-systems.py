#!/usr/bin/env python3
"""
discover-na-transit-systems.py — discover rail-transit systems + candidate
termini directly from OSM route relations, across Canada + US + Mexico
(Part B of the full NA sweep, 2026-07-03).

Why this exists: sim-commuter-zone.py's TERMINI_BY_REGION dict is hand-curated
— the 6 Canadian cities in it were each hand-run once via a route-relation
Overpass query, with results manually transcribed. That query pattern is
documented in BRIEF-gis-commuter-zone.md but was never coded as a reusable
discovery step. Also, that dict only covers subway/light_rail — the operator
flagged this "misses a lot": tram/streetcar networks and commuter/suburban
rail (route=rail, filtered by operator) are just as valid Transit Terminus
candidates and were never covered at all.

Method: query relation[route=subway/light_rail/tram] directly (unambiguous
transit types), plus relation[route=rail] filtered post-hoc by operator/network
against the same COMMUTER_OPERATORS allowlist ingest-osm-railway-commuter.py
already built (imported directly, not duplicated) — to exclude freight/
intercity rail. For each relation, the first and last "stop"-role members are
the candidate termini (ground truth from the relation's own ordering, not
guesswork). Relations are grouped into distinct systems by operator/network
tag (falling back to geographic clustering for untagged relations, reusing
the same cluster() helper as sim-urban-fringe.py).

KNOWN LIMITATION (documented in the plan, not hidden): pure first/last-member
logic misclassifies some interchange stations as termini — this needed manual
judgment for Calgary/Toronto/Ottawa (e.g. Sheppard-Yonge, Bayview). Running
this across dozens of NA cities without per-city manual review means some
noise in the termini output is expected. Downstream consumers (i.e. whoever
merges this into sim-commuter-zone.py's TERMINI_BY_REGION) should treat this
as a candidate list needing spot-check review, not a final ground truth.

Usage:
  python3 discover-na-transit-systems.py --region US_NORTHEAST
  python3 discover-na-transit-systems.py --all-na
Output: work/na-sweep-logs/transit-systems-<region>.json — list of
  {system_name, transit_class, region, bbox, termini: [{name, lat, lon}], ...}
"""

import argparse
import importlib.util
import json
import math
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

BASE = Path(__file__).parent


def _load_commuter_operators():
    spec = importlib.util.spec_from_file_location(
        "ingest_osm_railway_commuter", BASE / "ingest-osm-railway-commuter.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod.COMMUTER_OPERATORS, mod.DOWNTOWN_ONLY_OPERATORS


COMMUTER_OPERATORS, DOWNTOWN_ONLY_OPERATORS = _load_commuter_operators()

# Same 18 Canada+US+Mexico regions as sim-urban-fringe.py / discover-na-aerodromes.py
NA_REGIONS = {
    "AB":       (49.0, -120.0, 60.0, -110.0),
    "BC":       (48.3, -139.0, 60.0, -120.0),
    "SK":       (49.0, -110.0, 60.0, -101.5),
    "MB":       (49.0, -101.5, 60.0,  -95.0),
    "ON":       (41.6,  -95.2, 56.9,  -74.3),
    "QC":       (45.0,  -79.8, 62.6,  -57.1),
    "ATL":      (43.3,  -69.1, 60.4,  -52.6),
    "US_PACIFIC":       (32.5, -125.0, 49.0, -114.0),
    "US_MOUNTAIN":      (31.3, -120.0, 49.0, -102.0),
    "US_CENTRAL_NORTH": (37.0, -104.1, 49.4,  -89.5),
    "US_CENTRAL_SOUTH": (25.8, -106.6, 37.0,  -89.0),
    "US_MIDWEST":       (37.0,  -92.9, 48.3,  -80.5),
    "US_SOUTHEAST":     (24.5,  -91.7, 39.1,  -75.5),
    "US_NORTHEAST":     (38.9,  -80.5, 47.5,  -66.9),
    "US_MIDATLANTIC":   (36.5,  -83.7, 40.6,  -75.0),
    "MX_NORTH":   (24.0, -118.0, 32.7, -97.0),
    "MX_CENTRAL": (19.0, -105.5, 24.0, -96.0),
    "MX_SOUTH":   (14.5,  -99.5, 19.0, -86.7),
}

REGION_ISO = {
    "AB": "CA", "BC": "CA", "SK": "CA", "MB": "CA", "ON": "CA", "QC": "CA", "ATL": "CA",
    "US_PACIFIC": "US", "US_MOUNTAIN": "US", "US_CENTRAL_NORTH": "US",
    "US_CENTRAL_SOUTH": "US", "US_MIDWEST": "US", "US_SOUTHEAST": "US",
    "US_NORTHEAST": "US", "US_MIDATLANTIC": "US",
    "MX_NORTH": "MX", "MX_CENTRAL": "MX", "MX_SOUTH": "MX",
}

OVERPASS = "https://overpass-api.de/api/interpreter"
QUERY_SLEEP = 25  # 2026-07-03 throttling, matches Part A/C
CLUSTER_KM = 3.0  # geographic grouping radius for untagged relations


def fetch_overpass(q, label, retries=3):
    data = urllib.parse.urlencode({"data": q}).encode()
    req = urllib.request.Request(OVERPASS, data=data, method="POST", headers={
        "Content-Type": "application/x-www-form-urlencoded",
        "User-Agent": "foundry-gis-sim/1.0",
    })
    for attempt in range(retries):
        try:
            with urllib.request.urlopen(req, timeout=280) as r:
                return json.loads(r.read())
        except urllib.error.HTTPError as e:
            if e.code in (429, 502, 503, 504) and attempt < retries - 1:
                wait = 30 * (2 ** attempt)
                print(f"  {e.code}, retrying in {wait}s...", end=" ", flush=True)
                time.sleep(wait)
                continue
            print(f"  WARN {label}: {e}", file=sys.stderr)
            return {"elements": []}
        except Exception as e:
            print(f"  WARN {label}: {e}", file=sys.stderr)
            return {"elements": []}
    return {"elements": []}


def haversine(lat1, lon1, lat2, lon2):
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2 + math.cos(math.radians(lat1))
         * math.cos(math.radians(lat2)) * math.sin(dlon / 2) ** 2)
    return R * 2 * math.asin(math.sqrt(max(0, min(1, a))))


def keep_relation(tags: dict, iso: str) -> bool:
    """Same operator-allowlist logic as ingest-osm-railway-commuter.py's
    keep_station(), applied to route=rail relations (subway/light_rail/tram
    are always kept — unambiguous transit types)."""
    operator = (tags.get("operator") or "").lower()
    network = (tags.get("network") or "").lower()
    combined = f"{operator} {network}"
    if any(d in combined for d in DOWNTOWN_ONLY_OPERATORS):
        return False
    allowed = COMMUTER_OPERATORS.get(iso)
    if allowed is None:
        return True
    if not operator and not network:
        return True
    return any(op in combined for op in allowed)


def fetch_relations(bbox_str: str, route_type: str) -> dict:
    q = f"""
[out:json][timeout:240];
relation["route"="{route_type}"]({bbox_str});
out body;
>;
out skel qt;
"""
    return fetch_overpass(q, f"relations:{route_type}")


def extract_termini(res: dict, iso: str, route_type: str) -> list:
    """For each relation in the response, find the first/last 'stop'-role
    member and resolve its coordinates from the recursed node/way skeleton.
    Returns a list of {relation_id, name, operator, network, termini:[{name,lat,lon}]}."""
    nodes = {}
    relations = []
    for el in res.get("elements", []):
        if el["type"] == "node":
            nodes[el["id"]] = (el.get("lat"), el.get("lon"), el.get("tags", {}).get("name"))
        elif el["type"] == "relation":
            relations.append(el)

    systems = []
    for rel in relations:
        tags = rel.get("tags", {})
        if route_type == "rail" and not keep_relation(tags, iso):
            continue
        members = rel.get("members", [])
        stop_members = [m for m in members
                        if m.get("type") == "node"
                        and m.get("role") in ("stop", "stop_entry_only", "stop_exit_only", "platform", "")]
        if len(stop_members) < 2:
            continue
        first_id, last_id = stop_members[0]["ref"], stop_members[-1]["ref"]
        first, last = nodes.get(first_id), nodes.get(last_id)
        termini = []
        for node in (first, last):
            if node and node[0] is not None and node[1] is not None:
                termini.append({"name": node[2] or "?", "lat": node[0], "lon": node[1]})
        if not termini:
            continue
        systems.append({
            "relation_id": rel["id"],
            "name": tags.get("name", "?"),
            "operator": tags.get("operator"),
            "network": tags.get("network"),
            "route_type": route_type,
            "termini": termini,
        })
    return systems


def group_into_systems(relations: list) -> dict:
    """Group relations by operator/network tag; untagged ones fall back to
    geographic clustering on their termini centroid."""
    groups = {}
    untagged = []
    for rel in relations:
        key = (rel.get("network") or rel.get("operator") or "").strip().lower()
        if key:
            groups.setdefault(key, []).append(rel)
        else:
            untagged.append(rel)

    # Geographic clustering for untagged relations (reuses the same greedy
    # nearest-centroid approach as sim-urban-fringe.py's cluster()).
    clusters = []
    for rel in untagged:
        pts = [(t["lat"], t["lon"]) for t in rel["termini"]]
        if not pts:
            continue
        cx = sum(p[0] for p in pts) / len(pts)
        cy = sum(p[1] for p in pts) / len(pts)
        best, best_d = None, CLUSTER_KM + 1
        for cl in clusters:
            d = haversine(cx, cy, cl["cx"], cl["cy"])
            if d < best_d:
                best_d, best = d, cl
        if best is not None:
            best["rels"].append(rel)
        else:
            clusters.append({"cx": cx, "cy": cy, "rels": [rel]})
    for i, cl in enumerate(clusters):
        groups[f"untagged_cluster_{i}"] = cl["rels"]

    return groups


def discover_region(region: str, bbox: tuple) -> list:
    bbox_str = f"{bbox[0]},{bbox[1]},{bbox[2]},{bbox[3]}"
    iso = REGION_ISO[region]
    all_relations = []
    for route_type in ("subway", "light_rail", "tram", "rail"):
        print(f"  querying route={route_type}...", end=" ", flush=True)
        res = fetch_relations(bbox_str, route_type)
        rels = extract_termini(res, iso, route_type)
        print(f"{len(rels)} relations")
        all_relations.extend(rels)
        time.sleep(QUERY_SLEEP)

    groups = group_into_systems(all_relations)
    systems = []
    for key, rels in groups.items():
        all_termini = []
        for r in rels:
            all_termini.extend(r["termini"])
        # Dedup termini by rounded coordinate
        seen, deduped = set(), []
        for t in all_termini:
            k = (round(t["lat"], 4), round(t["lon"], 4))
            if k not in seen:
                seen.add(k)
                deduped.append(t)
        lats = [t["lat"] for t in deduped]
        lons = [t["lon"] for t in deduped]
        systems.append({
            "system_key": key,
            "region": region,
            "route_types": sorted(set(r["route_type"] for r in rels)),
            "n_relations": len(rels),
            "bbox": [min(lats), min(lons), max(lats), max(lons)] if deduped else None,
            "termini": deduped,
        })
    return systems


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--region", choices=list(NA_REGIONS.keys()))
    ap.add_argument("--all-na", action="store_true")
    args = ap.parse_args()

    if args.all_na:
        regions = list(NA_REGIONS.keys())
    elif args.region:
        regions = [args.region]
    else:
        ap.print_help()
        sys.exit(1)

    out_dir = Path("work/na-sweep-logs")
    out_dir.mkdir(parents=True, exist_ok=True)
    for i, region in enumerate(regions):
        print(f"\n=== Region: {region} ===")
        systems = discover_region(region, NA_REGIONS[region])
        out = out_dir / f"transit-systems-{region.lower()}.json"
        out.write_text(json.dumps(systems, indent=2))
        n_termini = sum(len(s["termini"]) for s in systems)
        print(f"  {len(systems)} systems, {n_termini} candidate termini -> {out}")
        if i < len(regions) - 1:
            time.sleep(QUERY_SLEEP)


if __name__ == "__main__":
    main()
