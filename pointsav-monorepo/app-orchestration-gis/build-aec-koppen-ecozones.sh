#!/usr/bin/env bash
# build-aec-koppen-ecozones.sh — Night 3 AEC build: Köppen-Geiger + ecoregions + PVGIS solar
#
# Run at 05:00 UTC 2026-05-26 (Night 3 of AEC staged rollout).
# See .agent/AEC-NIGHTLY-BUILD-PLAN.md for full plan context.
#
# Usage:  bash build-aec-koppen-ecozones.sh [--dry-run]
#
# Produces:
#   layer9-koppen-global.pmtiles       — Beck et al. 2023 Köppen-Geiger 1km, global z2–z8
#   layer9-ecoregions-global.pmtiles   — Resolve Ecoregions 2017, global z2–z8
#   layer9-biogeographic-eu.pmtiles    — EEA Biogeographical Regions 2016, EU z3–z9
#   layer9-ecoregions-us.pmtiles       — EPA Level III Ecoregions, US z3–z9
#   patches clusters-meta.json         — koppen_class, ecoregion_name, ecoregion_biome,
#                                        eu_biogeo_region, ghi_kwh_m2_yr (EU clusters)
#
# Prerequisites:
#   ogr2ogr (GDAL 3.8+), gdal_polygonize.py, tippecanoe, python3, curl, unzip
#
# PVGIS (EU solar GHI): free, no key required. Sampled at EU cluster centroids.
# US/CA solar GHI was handled in Night 2 via NREL NSRDB.
#
# Night 3 disk delta: ~50–100 MB PMTiles + ~500 MB work/ intermediates.
# Ensure ≥5 GB free on /srv/foundry before running.
#
# Data sources:
#   Köppen-Geiger: Beck et al. 2023, Zenodo 10.5281/zenodo.7872081, CC BY 4.0
#   Resolve Ecoregions: Dinerstein et al. 2017, storage.googleapis.com/teow2016/, CC BY 4.0
#   EEA Biogeographical Regions: EEA, eea.europa.eu, CC BY 2.5 DK
#   EPA L3 Ecoregions: US EPA, epa.gov/eco-research, public domain
#   PVGIS: JRC/EU, pvgis.ec.europa.eu, free API

set -euo pipefail

DRY_RUN=0
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN=1

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="$SCRIPT_DIR/work/aec"
LOG="$SCRIPT_DIR/build-aec-koppen-ecozones.log"
STAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
TILES_OUT="/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles"
META_PATH="/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"

echo "──────────────────────────────────────────────" | tee -a "$LOG"
echo "build-aec-koppen-ecozones  $STAMP" | tee -a "$LOG"

# ── pre-flight ────────────────────────────────────────────────────────────────

DISK_AVAIL=$(df -BG /srv/foundry | awk 'NR==2 {print $4}' | tr -d 'G')
if (( DISK_AVAIL < 5 )); then
    echo "ERROR: only ${DISK_AVAIL}G free — aborting to prevent ENOSPC" | tee -a "$LOG"
    exit 1
fi
echo "Disk free: ${DISK_AVAIL}G  ✓" | tee -a "$LOG"

for tool in ogr2ogr gdal_polygonize.py tippecanoe python3 curl unzip; do
    if ! command -v "$tool" &>/dev/null; then
        echo "ERROR: required tool '$tool' not found" | tee -a "$LOG"
        exit 1
    fi
done
echo "Tools: ogr2ogr $(ogr2ogr --version 2>&1 | head -1 | cut -d, -f1)  ✓" | tee -a "$LOG"
echo "Tools: tippecanoe $(tippecanoe --version 2>&1 | head -1)  ✓" | tee -a "$LOG"

# Night 2 complete marker check (advisory — not hard-blocking to allow reruns)
NIGHT2_MARKER="$SCRIPT_DIR/work/aec/.night2-complete"
if [[ ! -f "$NIGHT2_MARKER" ]]; then
    echo "WARN: Night 2 complete marker not found — proceeding anyway (some EU GHI may be missing)" | tee -a "$LOG"
fi

if [[ $DRY_RUN -eq 1 ]]; then
    echo "DRY RUN — pre-flight passed, not executing build steps" | tee -a "$LOG"
    exit 0
fi

mkdir -p "$WORK_DIR"
echo "Work dir: $WORK_DIR" | tee -a "$LOG"

# ── Step 1 — Beck et al. 2023 Köppen-Geiger GeoTIFF (global, 1km) ────────────
#
# Source: Beck, H.E., Zimmermann, N.E., McVicar, T.R., et al. (2018).
#         Present and future Köppen-Geiger climate classification maps at
#         1-km resolution. Scientific Data 5, 180214.
# URL:    https://figshare.com/articles/dataset/Present_and_future_K_ppen-Geiger_climate_classification_maps_at_1-km_resolution/6396959
# File:   Beck_KG_V1_present.tif (present-day 1980–2016 classification, inside Beck_KG_V1.zip)
# License: CC BY 4.0
#
# Class codes in the raster (integer 1–30) map to the standard KG symbols
# (Af, Am, Aw, BSh, ... EF). Reclassification table embedded below.

echo "" | tee -a "$LOG"
echo "[1/11] Beck et al. 2018 Köppen-Geiger GeoTIFF" | tee -a "$LOG"

KG_TIF="$WORK_DIR/koppen_geiger.tif"
if [[ ! -f "$KG_TIF" ]]; then
    # Download zip (Beck_KG_V1.zip ~70 MB), extract present-day TIF, delete zip
    KG_ZIP="$WORK_DIR/Beck_KG_V1.zip"
    KG_ZIP_URL="https://ndownloader.figshare.com/files/12407516"
    curl -fL "$KG_ZIP_URL" -o "$KG_ZIP" 2>&1 | tee -a "$LOG" || {
        echo "ERROR: Köppen-Geiger zip download failed" | tee -a "$LOG"
        exit 1
    }
    # File in zip is Beck_KG_V1_present_0p0083.tif (1km = 0.0083°); not Beck_KG_V1_present.tif
    unzip -p "$KG_ZIP" "Beck_KG_V1_present_0p0083.tif" > "$KG_TIF" || {
        echo "ERROR: Could not extract Beck_KG_V1_present_0p0083.tif from zip" | tee -a "$LOG"
        # Fallback: find the largest present-period TIF (the 1km file is ~22MB in the zip)
        PRESENT_NAME=$(unzip -l "$KG_ZIP" | grep -i 'present_0p0083' | awk '{print $NF}' | head -1)
        if [[ -z "$PRESENT_NAME" ]]; then
            PRESENT_NAME=$(unzip -l "$KG_ZIP" | grep -i present | grep '\.tif' | sort -k1 -rn | awk '{print $NF}' | head -1)
        fi
        if [[ -n "$PRESENT_NAME" ]]; then
            unzip -p "$KG_ZIP" "$PRESENT_NAME" > "$KG_TIF" || { echo "ERROR: fallback extract failed" | tee -a "$LOG"; exit 1; }
        else
            echo "ERROR: no present-period TIF found in zip" | tee -a "$LOG"
            exit 1
        fi
    }
    rm -f "$KG_ZIP"
fi
echo "  → $KG_TIF ($(du -sh "$KG_TIF" | cut -f1))  ✓" | tee -a "$LOG"

# ── Step 2 — Polygonize Köppen raster ────────────────────────────────────────
#
# gdal_polygonize.py converts the integer class raster to vector polygons.
# This is the slow step (~30–45 min globally). Clumping adjacent same-class
# pixels into one polygon before tippecanoe.

echo "" | tee -a "$LOG"
echo "[2/11] Polygonize Köppen raster (slow — 30–45 min)" | tee -a "$LOG"

KG_RAW_GPKG="$WORK_DIR/koppen-raw.gpkg"
if [[ ! -f "$KG_RAW_GPKG" ]]; then
    gdal_polygonize.py "$KG_TIF" -f GPKG "$KG_RAW_GPKG" koppen kg_code 2>&1 | tee -a "$LOG"
fi
echo "  → $KG_RAW_GPKG  ✓" | tee -a "$LOG"

# ── Step 3 — Simplify + reclassify to KG symbol strings ─────────────────────

echo "" | tee -a "$LOG"
echo "[3/11] Simplify + reclassify Köppen polygons" | tee -a "$LOG"

KG_GEOJSON="$WORK_DIR/koppen-simplified.geojson"
if [[ ! -f "$KG_GEOJSON" ]]; then
    # First simplify with ogr2ogr (-simplify tolerance in degrees; 0.01° ≈ 1km)
    KG_SIMPLIFIED_GPKG="$WORK_DIR/koppen-simplified.gpkg"
    ogr2ogr -f GPKG "$KG_SIMPLIFIED_GPKG" "$KG_RAW_GPKG" \
        -simplify 0.01 \
        -dialect SQLite \
        -sql "SELECT kg_code, geom FROM koppen WHERE kg_code > 0" \
        2>&1 | tee -a "$LOG"

    # Reclassify integer codes to standard KG symbols in Python
    python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
import json, subprocess, os, sys

# Beck et al. 2023 integer class → KG symbol mapping
# Reference: Table 1 in Beck et al. 2023 (doi:10.1038/s41597-023-02549-6)
KG_CODES = {
    1: "Af",  2: "Am",  3: "Aw",  4: "BWh", 5: "BWk",
    6: "BSh", 7: "BSk", 8: "Csa", 9: "Csb", 10: "Csc",
    11: "Cwa", 12: "Cwb", 13: "Cwc", 14: "Cfa", 15: "Cfb",
    16: "Cfc", 17: "Dsa", 18: "Dsb", 19: "Dsc", 20: "Dsd",
    21: "Dwa", 22: "Dwb", 23: "Dwc", 24: "Dwd", 25: "Dfa",
    26: "Dfb", 27: "Dfc", 28: "Dfd", 29: "ET",  30: "EF",
}

WORK_DIR = os.environ.get('WORK_DIR', '')
gpkg = os.path.join(WORK_DIR, 'koppen-simplified.gpkg')
out  = os.path.join(WORK_DIR, 'koppen-simplified.geojson')

# Read via ogr2ogr to GeoJSON
result = subprocess.run(
    ['ogr2ogr', '-f', 'GeoJSON', out, gpkg],
    capture_output=True, text=True
)
if result.returncode != 0:
    print(f"ERROR: ogr2ogr GeoJSON export failed: {result.stderr}", file=sys.stderr)
    sys.exit(1)

with open(out) as f:
    fc = json.load(f)

# Add koppen_class string field
for feat in fc['features']:
    code = feat['properties'].get('kg_code')
    feat['properties']['koppen_class'] = KG_CODES.get(code, 'Unknown')

with open(out, 'w') as f:
    json.dump(fc, f)

print(f"  Reclassified {len(fc['features'])} polygons → {out}")
PYEOF
fi
echo "  → $KG_GEOJSON  ✓" | tee -a "$LOG"

# ── Step 4 — Köppen → PMTiles ─────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[4/11] Köppen GeoJSON → PMTiles" | tee -a "$LOG"

export WORK_DIR
tippecanoe \
    --output="$TILES_OUT/layer9-koppen-global.pmtiles" \
    --name="Köppen-Geiger Climate Classification 2023" \
    --attribution="Beck et al. 2023, Sci Data, CC BY 4.0" \
    --layer="koppen" \
    --minimum-zoom=2 --maximum-zoom=8 \
    --simplification=6 \
    --drop-densest-as-needed \
    --force \
    "$KG_GEOJSON" 2>&1 | tee -a "$LOG"

echo "  → $TILES_OUT/layer9-koppen-global.pmtiles ($(du -sh "$TILES_OUT/layer9-koppen-global.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"

# ── Step 5 — Resolve Ecoregions 2017 (global, CC BY 4.0) ─────────────────────
#
# Source: Dinerstein et al. 2017, BioScience 67(6):534–545
# URL:    https://storage.googleapis.com/teow2016/Ecoregions2017.zip
# Fields: ECO_NAME, BIOME_NAME, REALM, ECO_ID, BIOME_NUM
# License: CC BY 4.0

echo "" | tee -a "$LOG"
echo "[5/11] Resolve Ecoregions 2017 (Dinerstein et al.)" | tee -a "$LOG"

ECO_ZIP="$WORK_DIR/Ecoregions2017.zip"
ECO_SHP="$WORK_DIR/Ecoregions2017.shp"
if [[ ! -f "$ECO_SHP" ]]; then
    curl -fL "https://storage.googleapis.com/teow2016/Ecoregions2017.zip" \
         -o "$ECO_ZIP" 2>&1 | tee -a "$LOG"
    unzip -q "$ECO_ZIP" -d "$WORK_DIR"
fi
echo "  → $ECO_SHP  ✓" | tee -a "$LOG"

# ── Step 6 — Resolve Ecoregions → GeoJSON → PMTiles ─────────────────────────

echo "" | tee -a "$LOG"
echo "[6/11] Resolve Ecoregions → PMTiles" | tee -a "$LOG"

ECO_GEOJSON="$WORK_DIR/ecoregions-global.geojson"
if [[ ! -f "$ECO_GEOJSON" ]]; then
    ogr2ogr -f GeoJSON "$ECO_GEOJSON" "$ECO_SHP" \
        -t_srs EPSG:4326 \
        -select "ECO_NAME,BIOME_NAME,REALM,ECO_ID,BIOME_NUM" \
        2>&1 | tee -a "$LOG"
fi

tippecanoe \
    --output="$TILES_OUT/layer9-ecoregions-global.pmtiles" \
    --name="RESOLVE Ecoregions 2017" \
    --attribution="Dinerstein et al. 2017, BioScience, CC BY 4.0" \
    --layer="ecoregions" \
    --minimum-zoom=2 --maximum-zoom=8 \
    --simplification=4 \
    --drop-densest-as-needed \
    --force \
    "$ECO_GEOJSON" 2>&1 | tee -a "$LOG"

echo "  → $TILES_OUT/layer9-ecoregions-global.pmtiles ($(du -sh "$TILES_OUT/layer9-ecoregions-global.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"

# ── Step 7 — EEA Biogeographical Regions 2016 (EU only) ──────────────────────
#
# Source: European Environment Agency
# URL:    https://www.eea.europa.eu/data-and-maps/data/biogeographical-regions-europe-3
# Direct: https://www.eea.europa.eu/data-and-maps/data/biogeographical-regions-europe-3/zipped-shapefile-format-vector-polygon/zipped-shapefile-format-vector-polygon/at_download/file
# Fields: short_name, long_name  (e.g. "ALP" = Alpine, "MED" = Mediterranean)
# License: CC BY 2.5 DK

echo "" | tee -a "$LOG"
echo "[7/11] EEA Biogeographical Regions 2016 (EU)" | tee -a "$LOG"

EEA_ZIP="$WORK_DIR/eea-biogeographic-regions.zip"
EEA_GEOJSON="$WORK_DIR/eea-biogeographic-regions.geojson"
if [[ ! -f "$EEA_GEOJSON" ]]; then
    EEA_URL="https://www.eea.europa.eu/data-and-maps/data/biogeographical-regions-europe-3/zipped-shapefile-format-vector-polygon/zipped-shapefile-format-vector-polygon/at_download/file"
    curl -fL "$EEA_URL" -o "$EEA_ZIP" 2>&1 | tee -a "$LOG" || {
        echo "WARN: EEA download failed — layer9-biogeographic-eu.pmtiles will be skipped" | tee -a "$LOG"
        EEA_GEOJSON=""
    }
    if [[ -n "$EEA_GEOJSON" ]]; then
        unzip -q "$EEA_ZIP" -d "$WORK_DIR/eea-biogeo"
        EEA_SHP=$(find "$WORK_DIR/eea-biogeo" -name "*.shp" | head -1)
        ogr2ogr -f GeoJSON "$EEA_GEOJSON" "$EEA_SHP" \
            -t_srs EPSG:4326 \
            -select "short_name,long_name" \
            2>&1 | tee -a "$LOG" || { EEA_GEOJSON=""; }
    fi
fi

if [[ -n "${EEA_GEOJSON:-}" && -f "$EEA_GEOJSON" ]]; then
    tippecanoe \
        --output="$TILES_OUT/layer9-biogeographic-eu.pmtiles" \
        --name="EEA Biogeographical Regions 2016" \
        --attribution="European Environment Agency, CC BY 2.5 DK" \
        --layer="biogeographic_eu" \
        --minimum-zoom=3 --maximum-zoom=9 \
        --simplification=4 \
        --force \
        "$EEA_GEOJSON" 2>&1 | tee -a "$LOG"
    echo "  → $TILES_OUT/layer9-biogeographic-eu.pmtiles ($(du -sh "$TILES_OUT/layer9-biogeographic-eu.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"
else
    echo "  SKIPPED — EEA download failed or file missing" | tee -a "$LOG"
fi

# ── Step 8 — EPA Level III Ecoregions (US) ───────────────────────────────────
#
# Source: US EPA Eco-Research Program (Omernik/Griffith 2014)
# URL:    https://gaftp.epa.gov/EPADataCommons/ORD/Ecoregions/us/us_eco_l3.zip
# Fields: US_L3NAME, US_L3CODE, NA_L2NAME, NA_L1NAME
# License: public domain

echo "" | tee -a "$LOG"
echo "[8/11] EPA Level III Ecoregions (US)" | tee -a "$LOG"

EPA_ZIP="$WORK_DIR/us_eco_l3.zip"
EPA_GEOJSON="$WORK_DIR/us-ecoregions-l3.geojson"
if [[ ! -f "$EPA_GEOJSON" ]]; then
    EPA_URL="https://gaftp.epa.gov/EPADataCommons/ORD/Ecoregions/us/us_eco_l3.zip"
    curl -fL "$EPA_URL" -o "$EPA_ZIP" 2>&1 | tee -a "$LOG" || {
        echo "WARN: EPA L3 ecoregions download failed — layer9-ecoregions-us.pmtiles will be skipped" | tee -a "$LOG"
        EPA_GEOJSON=""
    }
    if [[ -n "$EPA_GEOJSON" ]]; then
        unzip -q "$EPA_ZIP" -d "$WORK_DIR/epa-eco"
        EPA_SHP=$(find "$WORK_DIR/epa-eco" -name "us_eco_l3.shp" | head -1)
        if [[ -z "$EPA_SHP" ]]; then
            EPA_SHP=$(find "$WORK_DIR/epa-eco" -name "*.shp" | head -1)
        fi
        ogr2ogr -f GeoJSON "$EPA_GEOJSON" "$EPA_SHP" \
            -t_srs EPSG:4326 \
            -select "US_L3NAME,US_L3CODE,NA_L2NAME,NA_L1NAME" \
            2>&1 | tee -a "$LOG" || { EPA_GEOJSON=""; }
    fi
fi

if [[ -n "${EPA_GEOJSON:-}" && -f "$EPA_GEOJSON" ]]; then
    tippecanoe \
        --output="$TILES_OUT/layer9-ecoregions-us.pmtiles" \
        --name="EPA Level III Ecoregions" \
        --attribution="US EPA Eco-Research Program, public domain" \
        --layer="ecoregions_us" \
        --minimum-zoom=3 --maximum-zoom=9 \
        --simplification=3 \
        --force \
        "$EPA_GEOJSON" 2>&1 | tee -a "$LOG"
    echo "  → $TILES_OUT/layer9-ecoregions-us.pmtiles ($(du -sh "$TILES_OUT/layer9-ecoregions-us.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"
else
    echo "  SKIPPED — EPA download failed or file missing" | tee -a "$LOG"
fi

# ── Step 9 — PVGIS solar GHI for EU clusters ─────────────────────────────────
#
# PVGIS (Photovoltaic Geographical Information System) — JRC / European Commission
# API endpoint: https://re.jrc.ec.europa.eu/api/v5_2/seriescalc
# Free, no API key required. Rate limit: ~1 req/sec.
# Samples annual GHI (H_sun, kWh/m²/yr) at each EU cluster centroid.
# US/CA clusters were handled in Night 2 via NREL NSRDB.

echo "" | tee -a "$LOG"
echo "[9/11] PVGIS solar GHI sampling (EU clusters)" | tee -a "$LOG"

GHI_EU_OUT="$WORK_DIR/ghi-eu.json"
if [[ ! -f "$GHI_EU_OUT" ]]; then
    python3 - <<PYEOF 2>&1 | tee -a "$LOG"
import json, time, urllib.request, urllib.parse, os, sys

META_PATH = "$META_PATH"
OUT_PATH  = "$GHI_EU_OUT"

with open(META_PATH) as f:
    clusters = json.load(f)

# EU clusters that don't already have ghi_kwh_m2_yr
EU_CONTS = {"EU"}
targets = [
    c for c in clusters
    if c.get("continent") in EU_CONTS and c.get("ghi_kwh_m2_yr") is None
]
print(f"  Sampling {len(targets)} EU clusters via PVGIS")

results = {}
BASE = "https://re.jrc.ec.europa.eu/api/v5_2/seriescalc"

for i, c in enumerate(targets):
    lat = c.get("lat") or c.get("seed_lat") or c.get("clat")
    lon = c.get("lon") or c.get("seed_lon") or c.get("clon")
    if lat is None or lon is None:
        continue
    params = {
        "lat":         lat,
        "lon":         lon,
        "raddatabase": "PVGIS-SARAH3",
        "outputformat":"json",
        "pvcalculation":"0",
        "pvtechchoice": "crystSi",
        "mountingplace": "free",
        "loss":         "14",
        "angle":        "35",
        "aspect":       "0",
        "browser":      "0",
        "usehorizon":   "1",
        "startyear":    "2005",
        "endyear":      "2023",
    }
    url = BASE + "?" + urllib.parse.urlencode(params)
    try:
        with urllib.request.urlopen(url, timeout=30) as resp:
            data = json.loads(resp.read())
        # PVGIS seriescalc returns hourly data; extract annual H_sun sum
        hourly = data.get("outputs", {}).get("hourly", [])
        if hourly:
            annual_ghi = sum(h.get("H(h)_sun", 0) for h in hourly)
            results[c["id"]] = round(annual_ghi)
        if i % 100 == 0 and i > 0:
            print(f"  ... {i}/{len(targets)} sampled ({len(results)} successful)")
        time.sleep(1.1)  # stay under 1 req/sec
    except Exception as e:
        print(f"  WARN: PVGIS error for cluster {c.get('id','?')}: {e}")
        continue

with open(OUT_PATH, 'w') as f:
    json.dump(results, f)
print(f"  → {OUT_PATH}: {len(results)} EU GHI values saved")
PYEOF
else
    echo "  → $GHI_EU_OUT (cached)  ✓" | tee -a "$LOG"
fi

# ── Step 10 — Centroid point-in-polygon for cluster metadata fields ───────────
#
# Assigns koppen_class, ecoregion_name, ecoregion_biome, eu_biogeo_region
# to each cluster by testing its centroid against the vector layers.

echo "" | tee -a "$LOG"
echo "[10/11] Spatial lookup — patch cluster metadata fields" | tee -a "$LOG"

python3 - <<PYEOF 2>&1 | tee -a "$LOG"
import json, os, sys
from pathlib import Path

META_PATH   = "$META_PATH"
WORK_DIR    = "$WORK_DIR"
GHI_EU_PATH = "$GHI_EU_OUT"

with open(META_PATH) as f:
    clusters = json.load(f)

# Load GHI values
ghi_map = {}
if os.path.exists(GHI_EU_PATH):
    with open(GHI_EU_PATH) as f:
        ghi_map = json.load(f)
print(f"  GHI map: {len(ghi_map)} EU values loaded")

# ── Spatial point-in-polygon using shapely ────────────────────────────────────
try:
    from shapely.geometry import shape, Point
    HAVE_SHAPELY = True
except ImportError:
    HAVE_SHAPELY = False
    print("  WARN: shapely not available — skipping spatial lookups (GHI patch only)")

def load_features(path):
    if not os.path.exists(path):
        return []
    with open(path) as f:
        fc = json.load(f)
    return fc.get("features", [])

def build_index(features):
    """Return list of (geometry, properties) for point-in-polygon tests."""
    items = []
    for feat in features:
        try:
            geom = shape(feat["geometry"])
            items.append((geom, feat["properties"]))
        except Exception:
            pass
    return items

if HAVE_SHAPELY:
    print("  Loading Köppen polygons...")
    kg_index  = build_index(load_features(os.path.join(WORK_DIR, "koppen-simplified.geojson")))
    print(f"  Köppen: {len(kg_index)} polygons")

    print("  Loading Resolve Ecoregion polygons...")
    eco_index = build_index(load_features(os.path.join(WORK_DIR, "ecoregions-global.geojson")))
    print(f"  Ecoregions: {len(eco_index)} polygons")

    print("  Loading EEA Biogeographical Regions...")
    eea_path  = os.path.join(WORK_DIR, "eea-biogeographic-regions.geojson")
    eea_index = build_index(load_features(eea_path)) if os.path.exists(eea_path) else []
    print(f"  EEA biogeo: {len(eea_index)} polygons")

def find_containing(index, pt):
    for geom, props in index:
        try:
            if geom.contains(pt):
                return props
        except Exception:
            pass
    return None

patched_kg = patched_eco = patched_eea = patched_ghi = 0

for c in clusters:
    cid = c.get("id")
    lat = c.get("lat") or c.get("seed_lat") or c.get("clat")
    lon = c.get("lon") or c.get("seed_lon") or c.get("clon")
    if lat is None or lon is None:
        continue

    if HAVE_SHAPELY:
        pt = Point(float(lon), float(lat))

        if c.get("koppen_class") is None and kg_index:
            props = find_containing(kg_index, pt)
            if props:
                c["koppen_class"] = props.get("koppen_class")
                patched_kg += 1

        if c.get("ecoregion_name") is None and eco_index:
            props = find_containing(eco_index, pt)
            if props:
                c["ecoregion_name"]  = props.get("ECO_NAME")
                c["ecoregion_biome"] = props.get("BIOME_NAME")
                patched_eco += 1

        if c.get("eu_biogeo_region") is None and eea_index:
            props = find_containing(eea_index, pt)
            if props:
                c["eu_biogeo_region"] = props.get("short_name")
                patched_eea += 1

    # GHI patch (pre-computed, no shapely needed)
    if cid in ghi_map and c.get("ghi_kwh_m2_yr") is None:
        c["ghi_kwh_m2_yr"] = ghi_map[cid]
        patched_ghi += 1

print(f"  Patched: {patched_kg} Köppen, {patched_eco} ecoregion, {patched_eea} EEA biogeo, {patched_ghi} EU GHI")

with open(META_PATH, 'w') as f:
    json.dump(clusters, f, separators=(',', ':'))
print(f"  → {META_PATH}  ✓")
PYEOF

# ── Step 11 — Summary + phase marker ─────────────────────────────────────────

echo "" | tee -a "$LOG"
TILE3_COUNT=$(ls "$TILES_OUT"/layer9-*.pmtiles 2>/dev/null | wc -l)
echo "── Night 3 Complete: $(date -u '+%Y-%m-%dT%H:%M:%SZ') ──" | tee -a "$LOG"
echo "   layer9 PMTiles produced: $TILE3_COUNT" | tee -a "$LOG"
echo "   clusters-meta.json: $(wc -c < "$META_PATH") bytes" | tee -a "$LOG"
echo "   Next: Night 4 at 05:00 UTC 2026-05-27 — build-aec-seismic.sh" | tee -a "$LOG"

touch "$WORK_DIR/.night3-complete"
echo "   Phase marker: $WORK_DIR/.night3-complete  ✓" | tee -a "$LOG"
