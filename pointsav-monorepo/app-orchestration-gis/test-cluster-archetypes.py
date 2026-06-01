#!/usr/bin/env python3
"""
test-cluster-archetypes.py — Probe existing GIS data for two new co-location archetypes.

Archetype 1 — Vertical Warehouse (VW)
  3–6 story urban logistics / light-manufacturing building.
  Proxy signal: hardware cluster with NO hypermarket anchor, at urban fringe (5–80 km
  from nearest major metro), cluster span < 5 km.
  Not a retail zone — serves contractors, manufacturers, JIT delivery.

Archetype 2 — Transit Node (TN)
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


# ── PARAMETERS ────────────────────────────────────────────────────────────────

# Vertical Warehouse
VW_MIN_METRO_KM = 5.0   # not downtown core
VW_MAX_METRO_KM = 80.0  # urban fringe, not exurban
VW_MAX_SPAN_KM  = 5.0   # tight commercial node

# Transit Node
TN_MIN_METRO_KM     = 15.0   # not a suburb
TN_MAX_METRO_KM     = 150.0  # regional sweet spot
TN_HUB_EXCL_KM      = 5.0    # T1 within this → likely major hub → exclude
TN_INTEGRATED_KM    = 10.0   # T1/T2 within this → "integrated" node
TN_CLUSTER_SEARCH_KM = 20.0  # outer search radius for any nearby cluster

# ── LOAD CLUSTERS ─────────────────────────────────────────────────────────────

print("Loading clusters.geojson …")
with open(CLUSTERS_GEOJSON) as f:
    geojson = json.load(f)

features = geojson["features"]
print(f"  {len(features):,} clusters")

# Build spatial indices: T1-only list (for hub exclusion) and T1+T2 list (for linkage)
t1_index:  list[dict] = []
t12_index: list[dict] = []

vw_candidates: list[dict] = []

for feat in features:
    p = feat["properties"]
    lat  = p["seed_lat"]
    lon  = p["seed_lon"]
    tier = p["tier"]
    iso  = p["iso"]

    entry = {"lat": lat, "lon": lon, "id": p["cluster_id"],
             "tier": tier, "iso": iso, "market": p.get("market_name", "")}
    if tier == 1:
        t1_index.append(entry)
    if tier in (1, 2):
        t12_index.append(entry)

    # ── Vertical Warehouse filter ──────────────────────────────────────────────
    members_raw = p["members"]
    members = json.loads(members_raw) if isinstance(members_raw, str) else members_raw

    cats = {m["category"] for m in members}
    if "hardware" not in cats or "hypermarket" in cats:
        continue
    if p["span_km"] > VW_MAX_SPAN_KM:
        continue

    metro_dist, metro_name = nearest_metro(lat, lon)
    if not (VW_MIN_METRO_KM <= metro_dist <= VW_MAX_METRO_KM):
        continue

    _RETAIL_CATS = {"hypermarket", "hardware", "price_club", "lifestyle", "sport", "electronics"}
    # VWH enrichment categories — their presence strengthens the VWH signal
    _VWH_CATS = {"auto_parts", "paint", "mro_industrial", "flooring",
                 "tool_rental", "lumber", "plumbing", "electrical", "welding"}
    vwh_signal = sorted({m["category"] for m in members if m["category"] in _VWH_CATS})
    vw_candidates.append({
        "id":             p["cluster_id"],
        "lat":            lat,
        "lon":            lon,
        "iso":            iso,
        "tier":           tier,
        "span_km":        round(p["span_km"], 2),
        "metro_dist_km":  round(metro_dist, 1),
        "nearest_metro":  metro_name,
        "member_count":   p["member_count"],
        "hardware_chains": [m["chain_id"] for m in members if m["category"] == "hardware"],
        "other_retail":   [m["chain_id"] for m in members
                           if m["category"] in _RETAIL_CATS and m["category"] != "hardware"],
        "vwh_signal":     vwh_signal,     # industrial enrichment categories present
        "vwh_strength":   len(vwh_signal),  # 0 = bare hardware; higher = stronger VWH
        "market_name":    p.get("market_name", ""),
    })

print(f"  T1 clusters: {len(t1_index):,}  |  T1+T2: {len(t12_index):,}")
print(f"  Vertical Warehouse candidates: {len(vw_candidates):,}")

# ── LOAD TRANSIT ANCHORS (airports + railway stations) ────────────────────────
# Prefer the IATA-filtered OSM airport set; fall back to raw Overture if absent.

AIRPORTS_OSM = TOTEBOX / "service-places" / "cleansed-civic-airports.jsonl"
RAILWAY_OSM  = TOTEBOX / "service-places" / "cleansed-civic-railway.jsonl"

def _load_jsonl(path: Path, category: str | None = None) -> list[dict]:
    recs = []
    if not path.exists():
        return recs
    with open(path) as f:
        for line in f:
            try:
                r = json.loads(line)
            except json.JSONDecodeError:
                continue
            if category and r.get("category_id") != category:
                continue
            recs.append(r)
    return recs

print("\nLoading transit anchors …")
airports = _load_jsonl(AIRPORTS_OSM, "airport")
if airports:
    print(f"  airports: {len(airports):,} from cleansed-civic-airports.jsonl (IATA-filtered)")
    airport_dedup = False  # OSM airport ingest already deduplicated
else:
    airports = _load_jsonl(PLACES_JSONL, "airport")
    print(f"  airports: {len(airports):,} from cleansed-places.jsonl (Overture fallback)")
    airport_dedup = True

railways = _load_jsonl(RAILWAY_OSM, "railway_station")
print(f"  railway stations: {len(railways):,} from cleansed-civic-railway.jsonl")

# Tag transit type and filter to display countries
for a in airports:
    a["transit_type"] = "airport"
for r in railways:
    r["transit_type"] = "railway"

transit = [t for t in (airports + railways) if t.get("iso_country_code") in DISPLAY_ISO]
print(f"  {len(transit):,} transit anchors in display countries "
      f"({sum(1 for t in transit if t['transit_type']=='airport'):,} airports, "
      f"{sum(1 for t in transit if t['transit_type']=='railway'):,} railway)")

# Deduplicate only the Overture-fallback airports (raw Overture has duplicate nodes).
if airport_dedup:
    DEDUP_KM = 5.0
    print(f"  Deduplicating Overture airports (merging within {DEDUP_KM} km) …")
    deduped: list[dict] = []
    used = [False] * len(transit)
    for i, a in enumerate(transit):
        if used[i]:
            continue
        used[i] = True
        if a["transit_type"] != "airport":
            deduped.append(a)
            continue
        best = a
        for j in range(i + 1, len(transit)):
            if used[j] or transit[j]["transit_type"] != "airport":
                continue
            b = transit[j]
            if abs(b["latitude"] - a["latitude"]) > DEDUP_KM / 80.0:
                continue
            if haversine(a["latitude"], a["longitude"], b["latitude"], b["longitude"]) <= DEDUP_KM:
                used[j] = True
                if b.get("confidence", 0) > best.get("confidence", 0):
                    best = b
        deduped.append(best)
    print(f"  {len(transit):,} → {len(deduped):,} after dedup")
    transit = deduped

# ── TRANSIT NODE PASS (PKS) ───────────────────────────────────────────────────

print("\nScoring transit anchors as Parking Structure (PKS) candidates …")
tn_candidates: list[dict] = []
skipped_range = skipped_hub = 0

for i, ap in enumerate(transit):
    if i % 2000 == 0:
        print(f"  … {i:,}/{len(transit):,}", end="\r", flush=True)

    lat = ap["latitude"]
    lon = ap["longitude"]
    iso = ap.get("iso_country_code", "")

    # Step 1: metro distance filter
    metro_dist, metro_name = nearest_metro(lat, lon)
    if not (TN_MIN_METRO_KM <= metro_dist <= TN_MAX_METRO_KM):
        skipped_range += 1
        continue

    # Step 2: hub exclusion — T1 within TN_HUB_EXCL_KM → skip
    hub_d, _ = nearest_cluster(lat, lon, t1_index, max_km=TN_HUB_EXCL_KM)
    if hub_d is not None:
        skipped_hub += 1
        continue

    # Step 3: find nearest T1/T2 cluster
    cluster_d, cluster_id = nearest_cluster(lat, lon, t12_index, max_km=TN_CLUSTER_SEARCH_KM)
    integrated = cluster_d is not None and cluster_d <= TN_INTEGRATED_KM

    # Step 4: score
    ideal_lo, ideal_hi = 30.0, 100.0
    if metro_dist < ideal_lo:
        metro_score = metro_dist / ideal_lo
    elif metro_dist <= ideal_hi:
        metro_score = 1.0
    else:
        metro_score = max(0.0, 1.0 - (metro_dist - ideal_hi) / (TN_MAX_METRO_KM - ideal_hi))

    integration_score = (1.0 if integrated
                         else 0.5 if cluster_d is not None
                         else 0.0)

    # Multi-modal bonus: airport with a railway station nearby (or vice versa)
    # is a higher-value PKS site. Computed in a second pass below; placeholder here.
    tn_candidates.append({
        "transit_type":       ap.get("transit_type", "airport"),
        "name":               ap.get("location_name") or ap.get("city") or "",
        "iata":               ap.get("iata_code"),
        "operator":           ap.get("operator"),
        "city":               ap.get("city") or "",
        "lat":                lat,
        "lon":                lon,
        "iso":                iso,
        "confidence":         round(ap.get("confidence", 0), 3),
        "metro_dist_km":      round(metro_dist, 1),
        "nearest_metro":      metro_name,
        "nearest_cluster_id": cluster_id,
        "nearest_cluster_km": round(cluster_d, 1) if cluster_d is not None else None,
        "integrated":         integrated,
        "transit_node_score": round(metro_score + integration_score, 3),
    })

print(f"  Skipped (metro distance out of range): {skipped_range:,}")
print(f"  Skipped (T1 within {TN_HUB_EXCL_KM} km — major hub): {skipped_hub:,}")
print(f"  Transit Node candidates: {len(tn_candidates):,}")

integrated_count   = sum(1 for t in tn_candidates if t["integrated"])
linked_count       = sum(1 for t in tn_candidates if t["nearest_cluster_km"] is not None)
standalone_count   = len(tn_candidates) - linked_count

print(f"    Integrated  (T1/T2 ≤ {TN_INTEGRATED_KM} km):  {integrated_count:,}")
print(f"    Linked      (T1/T2 ≤ {TN_CLUSTER_SEARCH_KM} km): {linked_count:,}")
print(f"    Standalone  (no cluster within {TN_CLUSTER_SEARCH_KM} km): {standalone_count:,}")

# ── SORT ──────────────────────────────────────────────────────────────────────

vw_candidates.sort(key=lambda x: x["metro_dist_km"])
tn_candidates.sort(key=lambda x: -x["transit_node_score"])

# ── BY-COUNTRY BREAKDOWN ──────────────────────────────────────────────────────

def by_country(candidates: list[dict], key: str = "iso") -> dict:
    counts: dict[str, int] = defaultdict(int)
    for c in candidates:
        counts[c[key]] += 1
    return dict(sorted(counts.items(), key=lambda x: -x[1]))

# ── WRITE JSON RESULTS ────────────────────────────────────────────────────────

results = {
    "generated": datetime.now(timezone.utc).isoformat(),
    "vertical_warehouse": {
        "count": len(vw_candidates),
        "by_country": by_country(vw_candidates),
        "parameters": {
            "min_metro_km": VW_MIN_METRO_KM,
            "max_metro_km": VW_MAX_METRO_KM,
            "max_span_km":  VW_MAX_SPAN_KM,
        },
        "candidates": vw_candidates,
    },
    "transit_node": {
        "count":            len(tn_candidates),
        "integrated_count": integrated_count,
        "linked_count":     linked_count,
        "standalone_count": standalone_count,
        "by_country":       by_country(tn_candidates),
        "parameters": {
            "min_metro_km":      TN_MIN_METRO_KM,
            "max_metro_km":      TN_MAX_METRO_KM,
            "hub_exclusion_km":  TN_HUB_EXCL_KM,
            "integrated_km":     TN_INTEGRATED_KM,
            "cluster_search_km": TN_CLUSTER_SEARCH_KM,
        },
        "candidates": tn_candidates,
    },
}

out_json = WORK / "archetype-test-results.json"
with open(out_json, "w") as f:
    json.dump(results, f, indent=2)
print(f"\nWrote {out_json}")

# ── WRITE GEOJSON ─────────────────────────────────────────────────────────────

def to_geojson(candidates: list[dict], archetype: str) -> dict:
    feats = []
    for c in candidates:
        props = dict(c)
        props["archetype"] = archetype
        feats.append({
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [c["lon"], c["lat"]]},
            "properties": props,
        })
    return {"type": "FeatureCollection", "features": feats}

out_vwh = WORK / "archetype-vwh-candidates.geojson"
out_pks = WORK / "archetype-pks-candidates.geojson"

with open(out_vwh, "w") as f:
    json.dump(to_geojson(vw_candidates, "vertical_warehouse"), f)
print(f"Wrote {out_vwh}  ({len(vw_candidates):,} features)")

with open(out_pks, "w") as f:
    json.dump(to_geojson(tn_candidates, "parking_structure"), f)
print(f"Wrote {out_pks}  ({len(tn_candidates):,} features)")

# ── PRINT SUMMARY ─────────────────────────────────────────────────────────────

W = 70

print("\n" + "=" * W)
print("VERTICAL WAREHOUSE — Top 25 candidates (sorted by proximity to metro)")
print("=" * W)
for c in vw_candidates[:25]:
    hw     = ", ".join(c["hardware_chains"])
    others = ", ".join(c["other_retail"]) if c["other_retail"] else ""
    suffix = f"  + {others}" if others else ""
    print(f"  {c['iso']:2} | {(c['market_name'] or c['id'])[:28]:28} | "
          f"{c['metro_dist_km']:5.1f}km {c['nearest_metro'][:16]:16} | {hw}{suffix}")

print(f"\n  {'ISO':4} {'Count':>6}")
for iso, cnt in list(by_country(vw_candidates).items())[:17]:
    print(f"  {iso:4}  {cnt:>6}")

print("\n" + "=" * W)
print("TRANSIT NODE — Top 25 candidates (by score)")
print("=" * W)
for c in tn_candidates[:25]:
    cluster = (c["nearest_cluster_id"] or "none")[:20]
    dist_s  = f"{c['nearest_cluster_km']:.1f}km" if c["nearest_cluster_km"] is not None else "—"
    integ   = "✓" if c["integrated"] else " "
    print(f"  {integ} {c['iso']:2} | {(c['city'] or '?')[:22]:22} | "
          f"{c['metro_dist_km']:5.1f}km {c['nearest_metro'][:16]:16} | "
          f"score={c['transit_node_score']:.2f} | cluster@{dist_s}")

print(f"\n  {'ISO':4} {'Total':>6} {'Integrated':>12} {'Linked':>8}")
for iso, cnt in list(by_country(tn_candidates).items())[:17]:
    integ = sum(1 for t in tn_candidates if t["iso"] == iso and t["integrated"])
    linked = sum(1 for t in tn_candidates if t["iso"] == iso and t["nearest_cluster_km"] is not None)
    print(f"  {iso:4}  {cnt:>6}  {integ:>12}  {linked:>8}")

print("\nDone.")
