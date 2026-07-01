#!/usr/bin/env python3
"""
sim-aviation-fringe.py — Aviation Fringe detection VALIDATION (not production).

Tests the BRIEF's proposed rule (BRIEF-gis-commuter-zone.md §Aviation Fringe
Detection Framework) against 3 new reference zones plus the existing Springbank
baseline, before any build-aviation-fringe.py production script gets written.
Only Springbank (CYBW) has ever been profiled before this — this script is the
same validation step Commuter Zone went through across 6 cities before its
production script was written.

Rule under test: aerodrome_anchor (aeroway=aerodrome + aeroway=runway) +
hangar_cluster (>=5 aeroway=hangar ways) -> Aviation Fringe confirmed.

Sites (ICAO codes looked up live via Overpass — never hardcode lat/lon from
memory for a specific airport, since a wrong coordinate would silently produce
a wrong bbox and wrong results):
  CYBW  Springbank, Calgary AB       — existing baseline, re-verified here
  CYKF  Waterloo Regional, ON        — new, from NEXT.md carry-forward
  KFCM  Flying Cloud, MN             — new
  LFPN  Toussus-le-Noble, FR         — new

Usage:
    python3 sim-aviation-fringe.py                # all 4 sites
    python3 sim-aviation-fringe.py --icao CYKF     # one site
"""

import argparse
import json
import math
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

OVERPASS = "https://overpass-api.de/api/interpreter"
QUERY_SLEEP = 4
BBOX_RADIUS_KM = 3.0  # generous radius around the aerodrome center; GA airport
                       # properties + immediately adjacent fringe development
                       # should fit inside this. Not validated against a known
                       # Springbank query radius — flag if counts look truncated
                       # relative to the BRIEF's 478-element Springbank profile.

SITES = {
    "CYBW": "Springbank, Calgary AB (baseline, re-verify)",
    "CYKF": "Waterloo Regional, ON",
    "KFCM": "Flying Cloud, MN",
    "LFPN": "Toussus-le-Noble, FR",
}

HANGAR_THRESHOLD = 5  # per BRIEF: >=5 aeroway=hangar ways


def fetch_overpass(q, label, retries=3):
    data = urllib.parse.urlencode({"data": q}).encode()
    req = urllib.request.Request(OVERPASS, data=data, method="POST", headers={
        "Content-Type": "application/x-www-form-urlencoded",
        "User-Agent": "foundry-gis-sim/1.0",
    })
    for attempt in range(retries):
        try:
            with urllib.request.urlopen(req, timeout=200) as r:
                return json.loads(r.read())
        except urllib.error.HTTPError as e:
            if e.code in (429, 504) and attempt < retries - 1:
                wait = 15 * (attempt + 1)
                print(f"  {e.code}, retrying in {wait}s...", end=" ", flush=True)
                time.sleep(wait)
                continue
            print(f"  WARN {label}: {e}", file=sys.stderr)
            return {"elements": []}
        except Exception as e:
            print(f"  WARN {label}: {e}", file=sys.stderr)
            return {"elements": []}
    return {"elements": []}


def get_coord(el):
    if el["type"] == "node":
        return el.get("lat"), el.get("lon")
    c = el.get("center", {})
    return c.get("lat"), c.get("lon")


def find_aerodrome(icao):
    """Live lookup by ICAO code — never guess coordinates."""
    q = f"""
[out:json][timeout:60];
(
  node["aeroway"="aerodrome"]["icao"="{icao}"];
  way["aeroway"="aerodrome"]["icao"="{icao}"];
);
out center tags;
"""
    res = fetch_overpass(q, f"aerodrome:{icao}")
    els = res.get("elements", [])
    if not els:
        return None
    el = els[0]
    lat, lon = get_coord(el)
    return {"lat": lat, "lon": lon, "name": el.get("tags", {}).get("name", icao)}


def profile_site(lat, lon, radius_km):
    dlat = radius_km / 111.0
    dlon = radius_km / (111.0 * math.cos(math.radians(lat)))
    bbox = f"{lat-dlat},{lon-dlon},{lat+dlat},{lon+dlon}"
    q = f"""
[out:json][timeout:120];
(
  way["aeroway"="runway"]({bbox});
  way["aeroway"="taxiway"]({bbox});
  way["aeroway"="hangar"]({bbox});
  node["aeroway"="hangar"]({bbox});
  way["aeroway"="terminal"]({bbox});
  node["aeroway"="terminal"]({bbox});
  way["aeroway"="apron"]({bbox});
  node["aeroway"="parking_position"]({bbox});
  node["aeroway"="helipad"]({bbox});
  node["office"="company"]({bbox});
  node["craft"]({bbox});
  way["landuse"="industrial"]({bbox});
  node["aeroway"="aerodrome"]({bbox});
  way["aeroway"="aerodrome"]({bbox});
);
out center tags;
"""
    res = fetch_overpass(q, "profile")
    counts = {}
    operators = []
    for el in res.get("elements", []):
        tags = el.get("tags", {})
        key = tags.get("aeroway")
        if key:
            counts[f"aeroway={key}"] = counts.get(f"aeroway={key}", 0) + 1
        elif "office" in tags:
            counts["office=company"] = counts.get("office=company", 0) + 1
            if tags.get("name"):
                operators.append(tags["name"])
        elif "craft" in tags:
            counts[f"craft={tags['craft']}"] = counts.get(f"craft={tags['craft']}", 0) + 1
        elif tags.get("landuse") == "industrial":
            counts["landuse=industrial"] = counts.get("landuse=industrial", 0) + 1
    return {
        "total_elements": len(res.get("elements", [])),
        "counts": counts,
        "operators": operators,
        "has_runway": counts.get("aeroway=runway", 0) > 0,
        "hangar_count": counts.get("aeroway=hangar", 0),
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--icao", nargs="+", default=None, choices=list(SITES.keys()),
                     metavar="ICAO", help="Limit to specific sites (default: all 4)")
    args = ap.parse_args()

    icaos = args.icao or list(SITES.keys())
    results = []
    for icao in icaos:
        print(f"\n{'='*70}\n{icao} — {SITES[icao]}\n{'='*70}")
        site = find_aerodrome(icao)
        if site is None:
            print(f"  NOT FOUND — no aeroway=aerodrome node/way tagged icao={icao} in OSM")
            results.append({"icao": icao, "found": False})
            continue
        print(f"  Found: {site['name']!r} @ ({site['lat']:.4f}, {site['lon']:.4f})")
        time.sleep(QUERY_SLEEP)
        profile = profile_site(site["lat"], site["lon"], BBOX_RADIUS_KM)
        aerodrome_anchor = profile["has_runway"]
        hangar_cluster = profile["hangar_count"] >= HANGAR_THRESHOLD
        confirmed = aerodrome_anchor and hangar_cluster

        print(f"  Total elements: {profile['total_elements']}")
        for k, v in sorted(profile["counts"].items(), key=lambda kv: -kv[1]):
            print(f"    {k:<25} {v}")
        if profile["operators"]:
            print(f"  Named operators ({len(profile['operators'])}): "
                  f"{', '.join(profile['operators'][:10])}")
        print(f"  aerodrome_anchor={aerodrome_anchor}  "
              f"hangar_cluster={hangar_cluster} (count={profile['hangar_count']}, "
              f"threshold={HANGAR_THRESHOLD})")
        print(f"  >>> AVIATION FRINGE {'CONFIRMED' if confirmed else 'NOT CONFIRMED'} <<<")

        results.append({
            "icao": icao, "found": True, "lat": site["lat"], "lon": site["lon"],
            "name": site["name"], **profile,
            "aerodrome_anchor": aerodrome_anchor,
            "hangar_cluster": hangar_cluster,
            "confirmed": confirmed,
        })
        time.sleep(QUERY_SLEEP)

    print(f"\n{'='*70}\nSUMMARY — does the >=5-hangar threshold hold across all sites?\n{'='*70}")
    for r in results:
        if not r.get("found"):
            print(f"  {r['icao']:6s} NOT FOUND IN OSM")
            continue
        print(f"  {r['icao']:6s} hangars={r['hangar_count']:3d}  "
              f"runway={r['has_runway']}  confirmed={r['confirmed']}")

    out = Path("work/sim-aviation-fringe.json")
    out.parent.mkdir(exist_ok=True)
    out.write_text(json.dumps(results, indent=2))
    print(f"\nSaved -> {out}\n")


if __name__ == "__main__":
    main()
