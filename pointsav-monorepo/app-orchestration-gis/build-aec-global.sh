#!/usr/bin/env bash
# build-aec-global.sh — weekly AEC metadata enrichment (global-only pipeline)
#
# Patches clusters-meta.json with four globally-available fields:
#   koppen_class    — Köppen-Geiger 2023 (Beck et al., CC BY 4.0)    US+CA+MX+EU
#   ecoregion_name  — Resolve Ecoregions 2017 (Dinerstein, CC BY 4.0) US+CA+MX+EU
#   ecoregion_biome — same source                                       US+CA+MX+EU
#   wetland_class   — GWL_FCS30 (Liu et al. 2022, CC BY 4.0)          US+CA+MX+EU
#   ghi_kwh_m2_yr   — NREL NSRDB (NA: US+CA+MX) + PVGIS (EU)          US+CA+MX+EU
#
# Regional-only sources (ASHRAE, EU climate codes, seismic, FEMA) are excluded:
# no Mexico data source exists for those datasets.
#
# Solar GHI requires NREL_API_KEY for the NA batch (free at developer.nrel.gov).
# If not set, the NA solar step is skipped; EU solar via PVGIS runs without a key.
#
# Cron: 0 5 * * 1  (Monday 05:00 UTC — after OSM ingest, before business hours)
#
# Source data (must exist in work/aec/):
#   koppen-simplified.geojson   (255 MB) — produced by build-aec-koppen-ecozones.sh
#   ecoregions-global.geojson   (631 MB) — produced by build-aec-koppen-ecozones.sh
#   gwl-fcs30-global.tif        (15 MB)  — produced by build-aec-seismic.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK="$SCRIPT_DIR/work/aec"
DEPLOY="/srv/foundry/deployments/gateway-orchestration-gis-1"
META="$DEPLOY/www/data/clusters-meta.json"
INDEX_HTML="$DEPLOY/www/index.html"
LOG="$SCRIPT_DIR/aec-global.log"
DATE_TAG="$(date -u +%Y%m%d)"

echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ── build-aec-global START ──────────────────────────" | tee -a "$LOG"

# Verify required source files
for f in "$WORK/koppen-simplified.geojson" "$WORK/ecoregions-global.geojson" "$WORK/gwl-fcs30-global.tif" "$META"; do
    if [[ ! -f "$f" ]]; then
        echo "ERROR: Required file missing: $f" | tee -a "$LOG"; exit 1
    fi
done
echo "[1/5] Source files verified  ✓" | tee -a "$LOG"

# ── Steps 2+3+4 — Köppen + Ecoregion + Wetland spatial sampling ──────────────
echo "[2/5] Spatial sampling: Köppen class + ecoregion + wetland" | tee -a "$LOG"

WORK_DIR="$WORK" META_PATH="$META" python3 - <<'PYEOF'
import json, os, sys, subprocess, math
from pathlib import Path

WORK    = Path(os.environ["WORK_DIR"])
META_P  = Path(os.environ["META_PATH"])

print("  Loading clusters-meta.json …")
with open(META_P) as f:
    clusters = json.load(f)
print(f"  {len(clusters)} clusters")

# ── Köppen-Geiger ─────────────────────────────────────────────────────────────
print("  Loading koppen-simplified.geojson (255 MB) …")
with open(WORK / "koppen-simplified.geojson") as f:
    kg_fc = json.load(f)

# Build bounding-box index: list of (minx,miny,maxx,maxy, properties)
print("  Building Köppen bbox index …")
kg_index = []
for feat in kg_fc["features"]:
    props = feat["properties"]
    geom  = feat["geometry"]
    if geom["type"] == "Polygon":
        rings = geom["coordinates"]
    else:  # MultiPolygon
        rings = [r for poly in geom["coordinates"] for r in poly]
    for ring in rings:
        xs = [p[0] for p in ring]; ys = [p[1] for p in ring]
        kg_index.append((min(xs), min(ys), max(xs), max(ys), props, feat["geometry"]))
print(f"  Köppen index: {len(kg_index)} polygons")

def point_in_polygon(px, py, polygon_coords):
    """Ray-casting point-in-polygon for a single ring."""
    inside = False
    n = len(polygon_coords)
    j = n - 1
    for i in range(n):
        xi, yi = polygon_coords[i][0], polygon_coords[i][1]
        xj, yj = polygon_coords[j][0], polygon_coords[j][1]
        if ((yi > py) != (yj > py)) and (px < (xj - xi) * (py - yi) / (yj - yi + 1e-12) + xi):
            inside = not inside
        j = i
    return inside

def point_in_geom(px, py, geom):
    if geom["type"] == "Polygon":
        return point_in_polygon(px, py, geom["coordinates"][0])
    for poly in geom["coordinates"]:
        if point_in_polygon(px, py, poly[0]):
            return True
    return False

def lookup_koppen(lon, lat):
    candidates = [(minx, miny, maxx, maxy, props, geom)
                  for (minx, miny, maxx, maxy, props, geom) in kg_index
                  if minx <= lon <= maxx and miny <= lat <= maxy]
    for (_, _, _, _, props, geom) in candidates:
        if point_in_geom(lon, lat, geom):
            return props.get("koppen_class")
    return None

# ── Resolve Ecoregions ────────────────────────────────────────────────────────
print("  Loading ecoregions-global.geojson (631 MB) …")
with open(WORK / "ecoregions-global.geojson") as f:
    eco_fc = json.load(f)

print("  Building ecoregion bbox index …")
eco_index = []
for feat in eco_fc["features"]:
    props = feat["properties"]
    geom  = feat["geometry"]
    if geom["type"] == "Polygon":
        rings = geom["coordinates"]
    else:
        rings = [r for poly in geom["coordinates"] for r in poly]
    for ring in rings:
        xs = [p[0] for p in ring]; ys = [p[1] for p in ring]
        if xs and ys:
            eco_index.append((min(xs), min(ys), max(xs), max(ys), props, feat["geometry"]))
print(f"  Ecoregion index: {len(eco_index)} polygons")

def lookup_ecoregion(lon, lat):
    candidates = [(minx, miny, maxx, maxy, props, geom)
                  for (minx, miny, maxx, maxy, props, geom) in eco_index
                  if minx <= lon <= maxx and miny <= lat <= maxy]
    for (_, _, _, _, props, geom) in candidates:
        if point_in_geom(lon, lat, geom):
            return props.get("ECO_NAME"), props.get("BIOME_NAME")
    return None, None

# ── Wetland sampling via gdallocationinfo ─────────────────────────────────────
GWL_TIF = str(WORK / "gwl-fcs30-global.tif")
GWL_CODES = {
    10: "Cropland", 20: "Forest", 30: "Grassland", 40: "Shrubland",
    60: "Bare soil", 80: "Water body", 90: "Tundra", 100: "Snow/ice",
    181: "Permanent wetland", 182: "Seasonal wetland",
    183: "Permanent herbaceous wetland", 184: "Seasonal herbaceous wetland",
    185: "Mangrove", 186: "Salt marsh", 190: "Other wetland",
}

def sample_wetland(lon, lat):
    try:
        result = subprocess.run(
            ["gdallocationinfo", "-valonly", "-wgs84", GWL_TIF, str(lon), str(lat)],
            capture_output=True, text=True, timeout=5
        )
        val = result.stdout.strip()
        if val:
            code = int(float(val))
            return GWL_CODES.get(code)
    except Exception:
        pass
    return None

# ── Patch all clusters ────────────────────────────────────────────────────────
print("  Sampling all clusters …")
kg_hits = eco_hits = wl_hits = 0
total = len(clusters)

for i, c in enumerate(clusters):
    if i % 500 == 0:
        print(f"    {i}/{total} …", flush=True)
    lon, lat = c.get("lon", 0), c.get("lat", 0)

    if not c.get("koppen_class"):
        klass = lookup_koppen(lon, lat)
        if klass:
            c["koppen_class"] = klass
            kg_hits += 1

    if not c.get("ecoregion_name"):
        eco_name, biome = lookup_ecoregion(lon, lat)
        if eco_name:
            c["ecoregion_name"]  = eco_name
            c["ecoregion_biome"] = biome
            eco_hits += 1

    if not c.get("wetland_class"):
        wl = sample_wetland(lon, lat)
        if wl:
            c["wetland_class"] = wl
            wl_hits += 1

print(f"  Köppen hits:    {kg_hits}/{total}")
print(f"  Ecoregion hits: {eco_hits}/{total}")
print(f"  Wetland hits:   {wl_hits}/{total}")

# Atomic write
tmp = str(META_P) + ".tmp"
with open(tmp, "w") as f:
    json.dump(clusters, f, separators=(",", ":"))
import os; os.replace(tmp, str(META_P))
print(f"  clusters-meta.json updated  ✓")
PYEOF

echo "[2/5] Spatial sampling complete  ✓" | tee -a "$LOG"

# ── Step 5a — Solar GHI: EU clusters via PVGIS ───────────────────────────────
echo "[3/5] Solar GHI — EU clusters via PVGIS (no key required)" | tee -a "$LOG"

META_PATH="$META" python3 - <<'PYEOF'
import json, os, time, urllib.request, urllib.parse

META_P = os.environ["META_PATH"]
with open(META_P) as f:
    clusters = json.load(f)

targets = [c for c in clusters if c.get("cont") == "EU" and c.get("ghi_kwh_m2_yr") is None]
print(f"  {len(targets)} EU clusters without GHI")

ghi_map = {}
for i, c in enumerate(targets):
    if i % 50 == 0:
        print(f"    {i}/{len(targets)} …", flush=True)
    lon, lat = c["lon"], c["lat"]
    params = urllib.parse.urlencode({
        "lat": lat, "lon": lon,
        "raddatabase": "PVGIS-SARAH2",
        "outputformat": "json",
        "pvcalculation": 0,
        "angle": 0, "aspect": 0,
        "components": 1,
    })
    url = f"https://re.jrc.ec.europa.eu/api/v5_2/seriescalc?{params}"
    try:
        with urllib.request.urlopen(url, timeout=20) as resp:
            data = json.loads(resp.read())
        hourly = data.get("outputs", {}).get("hourly", [])
        if hourly:
            n_years = len(hourly) / 8760          # PVGIS returns full dataset range
            annual_ghi = sum(h.get("Gb(i)", 0) + h.get("Gd(i)", 0)
                             for h in hourly) / 1000.0 / n_years
            ghi_map[c["id"]] = round(annual_ghi, 1)
    except Exception as e:
        print(f"  WARN: PVGIS error for {c.get('id','?')}: {e}")
    time.sleep(0.5)

for c in clusters:
    if c["id"] in ghi_map:
        c["ghi_kwh_m2_yr"] = ghi_map[c["id"]]

tmp = META_P + ".tmp"
with open(tmp, "w") as f:
    json.dump(clusters, f, separators=(",", ":"))
os.replace(tmp, META_P)
print(f"  EU solar GHI: {len(ghi_map)} clusters updated  ✓")
PYEOF

echo "[3/5] EU solar GHI complete  ✓" | tee -a "$LOG"

# ── Step 5b — Solar GHI: NA clusters via PVGIS-NSRDB ─────────────────────────
# PVGIS-NSRDB covers Americas (lon -170 to -30, lat -20 to 60). No API key required.
# Replaces former NREL NSRDB integration (developer.nrel.gov DNS failure Jun 2026).
echo "[4/5] Solar GHI — NA clusters (US+CA+MX) via PVGIS-NSRDB (no key required)" | tee -a "$LOG"
META_PATH="$META" python3 - <<'PYEOF'
import json, os, time, urllib.request, urllib.parse

META_P = os.environ["META_PATH"]
with open(META_P) as f:
    clusters = json.load(f)

targets = [c for c in clusters if c.get("cont") == "NA" and c.get("ghi_kwh_m2_yr") is None]
print(f"  {len(targets)} NA clusters without GHI (US+CA+MX)")

ghi_map = {}
skipped = 0
for i, c in enumerate(targets):
    if i % 50 == 0:
        print(f"    {i}/{len(targets)} …", flush=True)
    lon, lat = c["lon"], c["lat"]
    if lat > 60.0:          # PVGIS-NSRDB coverage ceiling
        skipped += 1
        continue
    params = urllib.parse.urlencode({
        "lat": lat, "lon": lon,
        "raddatabase": "PVGIS-NSRDB",
        "outputformat": "json",
        "pvcalculation": 0,
        "angle": 0, "aspect": 0,
        "components": 1,
    })
    url = f"https://re.jrc.ec.europa.eu/api/v5_2/seriescalc?{params}"
    try:
        with urllib.request.urlopen(url, timeout=20) as resp:
            data = json.loads(resp.read())
        hourly = data.get("outputs", {}).get("hourly", [])
        if hourly:
            n_years = len(hourly) / 8760          # PVGIS returns full dataset range
            annual_ghi = sum(h.get("Gb(i)", 0) + h.get("Gd(i)", 0)
                             for h in hourly) / 1000.0 / n_years
            ghi_map[c["id"]] = round(annual_ghi, 1)
    except Exception as e:
        print(f"  WARN: PVGIS-NSRDB error for {c.get('id','?')}: {e}")
    time.sleep(0.5)

for c in clusters:
    if c["id"] in ghi_map:
        c["ghi_kwh_m2_yr"] = ghi_map[c["id"]]

tmp = META_P + ".tmp"
with open(tmp, "w") as f:
    json.dump(clusters, f, separators=(",", ":"))
os.replace(tmp, META_P)
print(f"  NA solar GHI: {len(ghi_map)} clusters updated  {skipped} skipped (lat>60)  ✓")
PYEOF
echo "[4/5] NA solar GHI complete  ✓" | tee -a "$LOG"

# ── Step 6 — Cache-bust index.html ───────────────────────────────────────────
echo "[5/5] Cache-bust index.html (?v= token)" | tee -a "$LOG"
# Update all ?v=YYYYMMDD occurrences (cluster data URLs)
sed -i "s/?v=[0-9]\{8\}[a-z]*/?v=${DATE_TAG}/g" "$INDEX_HTML"
echo "  ?v=${DATE_TAG} applied  ✓" | tee -a "$LOG"

# ── Summary ───────────────────────────────────────────────────────────────────
python3 - <<PYEOF
import json
with open("$META") as f:
    d = json.load(f)
total = len(d)
print(f"\n  === build-aec-global summary ===")
for field in ["koppen_class", "ecoregion_name", "wetland_class", "ghi_kwh_m2_yr"]:
    n = sum(1 for c in d if c.get(field) not in (None, ""))
    print(f"  {field:20s}  {n:5d}/{total}")
PYEOF

echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ── build-aec-global DONE ───────────────────────────" | tee -a "$LOG"
