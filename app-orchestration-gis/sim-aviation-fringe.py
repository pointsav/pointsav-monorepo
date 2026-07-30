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
    python3 sim-aviation-fringe.py                # all 4 validation sites
    python3 sim-aviation-fringe.py --icao CYKF     # one validation site
    python3 sim-aviation-fringe.py --batch-file work/na-sweep-logs/aerodromes-ab.json
        # 2026-07-03 Part C batch mode: profile every site in a
        # discover-na-aerodromes.py output file directly by lat/lon, skipping
        # the ICAO-lookup query (halves query count for large batches).
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
# 2026-07-03: throttled per operator direction for the full NA sweep — no daily
# cap, but substantially slower than the original 4s pace.
QUERY_SLEEP = 25
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
            if e.code in (429, 502, 503, 504) and attempt < retries - 1:
                wait = 30 * (2 ** attempt)  # 30s/60s/120s — 2026-07-03 throttling
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


def profile_and_score(lat, lon, radius_km):
    """Profile a site and apply the confirmed >=5-hangar rule. Shared by both modes."""
    profile = profile_site(lat, lon, radius_km)
    aerodrome_anchor = profile["has_runway"]
    hangar_cluster = profile["hangar_count"] >= HANGAR_THRESHOLD
    confirmed = aerodrome_anchor and hangar_cluster
    return profile, aerodrome_anchor, hangar_cluster, confirmed


def run_batch(batch_file: Path):
    """2026-07-03 Part C mode: profile every discovered site directly by
    lat/lon (from discover-na-aerodromes.py output) — skips find_aerodrome()'s
    ICAO-lookup query entirely since coordinates are already known.

    2026-07-09: checkpoints after every site (not just at the end) and
    resumes from an existing partial output file — a VM crash mid-region
    used to lose the entire region's Overpass queries (all-or-nothing write
    at loop end); now at most one in-flight site's query is lost."""
    sites = json.loads(batch_file.read_text())
    out_dir = Path("work/na-sweep-logs")
    out_dir.mkdir(parents=True, exist_ok=True)
    out = out_dir / f"aviation-fringe-{batch_file.stem.replace('aerodromes-', '')}.json"

    results = []
    start_i = 0
    if out.exists():
        results = json.loads(out.read_text())
        start_i = len(results)
        print(f"Resuming {out}: {start_i}/{len(sites)} already profiled")

    print(f"Batch mode: {len(sites)} sites from {batch_file}")
    for i in range(start_i, len(sites)):
        site = sites[i]
        name = site.get("name") or site.get("icao") or "unnamed"
        print(f"\n[{i+1}/{len(sites)}] {name} @ ({site['lat']:.4f}, {site['lon']:.4f})")
        profile, aerodrome_anchor, hangar_cluster, confirmed = profile_and_score(
            site["lat"], site["lon"], BBOX_RADIUS_KM)
        print(f"  hangars={profile['hangar_count']}  runway={aerodrome_anchor}  "
              f"confirmed={confirmed}")
        results.append({
            "icao": site.get("icao"), "iata": site.get("iata"), "found": True,
            "lat": site["lat"], "lon": site["lon"], "name": name, **profile,
            "aerodrome_anchor": aerodrome_anchor,
            "hangar_cluster": hangar_cluster,
            "confirmed": confirmed,
        })
        out.write_text(json.dumps(results, indent=2))
        if i < len(sites) - 1:
            time.sleep(QUERY_SLEEP)

    confirmed_n = sum(1 for r in results if r["confirmed"])
    print(f"\n{'='*70}\n{confirmed_n}/{len(results)} sites confirmed "
          f"(>=  {HANGAR_THRESHOLD} hangars + runway)\n{'='*70}")
    print(f"Saved -> {out}\n")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--icao", nargs="+", default=None, choices=list(SITES.keys()),
                     metavar="ICAO", help="Limit to specific sites (default: all 4)")
    ap.add_argument("--batch-file", type=Path, default=None,
                     help="Profile every site in a discover-na-aerodromes.py "
                          "output JSON file (Part C NA sweep mode)")
    args = ap.parse_args()

    if args.batch_file:
        run_batch(args.batch_file)
        return

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
        profile, aerodrome_anchor, hangar_cluster, confirmed = profile_and_score(
            site["lat"], site["lon"], BBOX_RADIUS_KM)

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
