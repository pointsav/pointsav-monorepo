#!/usr/bin/env python3
"""
test-cluster-archetypes.py — Probe existing GIS data for two new co-location archetypes.

Archetype 1 — Urban Fringe (VWH)
  3–6 story urban logistics / light-manufacturing building.
  Proxy signal: hardware cluster with NO hypermarket anchor, at urban fringe (5–80 km
  from nearest major metro), cluster span < 5 km.
  Not a retail zone — serves contractors, manufacturers, JIT delivery.

Archetype 2 — Commuter (PKS)
  Regional airport 15–150 km from a major metro, co-located with a T1/T2 cluster
  (the Regional Market whose population generates park-and-fly / park-and-train demand).
  Major hub proxy: airports with a T1 cluster within 5 km are excluded (major hubs have
  T1 retail directly adjacent; regional airports typically do not).

Outputs:
  work/archetype-test-results.json
  work/archetype-vwh-candidates.geojson
  work/archetype-pks-candidates.geojson
"""

import importlib.util
import json
import math
import subprocess
import sys
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

# ── PATHS ─────────────────────────────────────────────────────────────────────

BASE    = Path(__file__).parent
WORK    = BASE / "work"
TOTEBOX = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")

CLUSTERS_GEOJSON = WORK / "clusters.geojson"
PLACES_JSONL     = TOTEBOX / "service-places" / "cleansed-places.jsonl"

# ── IMPORT METRO LISTS FROM score-regional-markets.py ────────────────────────

def _load_metro_lists():
    spec = importlib.util.spec_from_file_location(
        "score_regional_markets",
        BASE / "score-regional-markets.py",
    )
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod.NA_METROS, mod.EU_METROS

NA_METROS, EU_METROS = _load_metro_lists()
ALL_METROS = NA_METROS + EU_METROS

# ── DISPLAY COUNTRIES ─────────────────────────────────────────────────────────

DISPLAY_ISO = {
    "US", "CA", "MX",
    "GB", "SE", "DK", "NO", "FI", "IS",
    "FR", "DE", "ES", "IT", "GR", "PL", "AT", "NL", "PT",
}

# ── GEOMETRY HELPERS ──────────────────────────────────────────────────────────

def haversine(lat1: float, lon1: float, lat2: float, lon2: float) -> float:
    R = 6371.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (math.sin(dlat / 2) ** 2
         + math.cos(math.radians(lat1)) * math.cos(math.radians(lat2))
         * math.sin(dlon / 2) ** 2)
    return R * 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))


def nearest_metro(lat: float, lon: float) -> tuple[float, str]:
    best, name = float("inf"), ""
    for mlat, mlon, mname in ALL_METROS:
        d = haversine(lat, lon, mlat, mlon)
        if d < best:
            best, name = d, mname
    return best, name


def nearest_cluster(
    lat: float,
    lon: float,
    clusters: list[dict],
    max_km: float = 20.0,
) -> tuple[float | None, str | None]:
    """Return (dist_km, cluster_id) for nearest cluster within max_km."""
    lat_margin = max_km / 111.0
    lon_margin = max_km / max(111.0 * math.cos(math.radians(lat)), 1.0)
    best_d, best_id = float("inf"), None
    for c in clusters:
        if abs(c["lat"] - lat) > lat_margin or abs(c["lon"] - lon) > lon_margin:
            continue
        d = haversine(lat, lon, c["lat"], c["lon"])
        if d < best_d:
            best_d, best_id = d, c["id"]
    if best_d <= max_km:
        return best_d, best_id
    return None, None



# ── ARCHETYPE DETECTION (independent clustering) ─────────────────────────────

print("\nBuilding Urban Fringe clusters …")
subprocess.run([sys.executable, str(BASE / "build-vwh-clusters.py")], check=True)

print("\nBuilding Commuter clusters …")
subprocess.run([sys.executable, str(BASE / "build-pks-clusters.py")], check=True)

print("\nDone.")
