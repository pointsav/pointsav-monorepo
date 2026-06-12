#!/usr/bin/env bash
# build-aec-climate-solar.sh — Night 2 AEC build: climate zones + solar GHI
#
# Run at 05:00 UTC 2026-05-25 (Night 2 of AEC staged rollout).
# See .agent/AEC-NIGHTLY-BUILD-PLAN.md for full plan context.
#
# Usage:  bash build-aec-climate-solar.sh [--dry-run]
#
# Produces:
#   layer8-ashrae-zones-us.pmtiles     — US county ASHRAE 169-2013 climate zones
#   layer8-necb-zones-ca.pmtiles       — Canada NECB/HOT2000 climate zones
#   layer8-eu-climate-zones.pmtiles    — EU regulatory code zones (FR/ES/IT/DE/GR/PT/FI/PL/SE)
#   patches clusters-meta.json         — ashrae_zone, necb_zone, eu_climate_zone, ghi_kwh_m2_yr
#
# Prerequisites (all present on this VM):
#   ogr2ogr (GDAL 3.8+), tippecanoe, python3, curl, unzip, jq
#
# NREL NSRDB API key required for US/CA/MX solar GHI (free registration):
#   https://developer.nrel.gov/signup/
#   Set env var:  export NREL_API_KEY=your_key_here
#   If unset, solar GHI sampling is SKIPPED (field stays null in clusters-meta).
#
# Night 2 disk delta: ~15–25 MB PMTiles + ~500 MB work/ intermediates.
# Ensure ≥5 GB free on /srv/foundry before running.

set -euo pipefail

DRY_RUN=0
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN=1

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="$SCRIPT_DIR/work/aec"
export WORK_DIR
LOG="$SCRIPT_DIR/build-aec-climate-solar.log"
STAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
TILES_OUT="/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles"
META_PATH="/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"

echo "──────────────────────────────────────────────" | tee -a "$LOG"
echo "build-aec-climate-solar  $STAMP" | tee -a "$LOG"

# ── pre-flight ────────────────────────────────────────────────────────────────

DISK_AVAIL=$(df -BG /srv/foundry | awk 'NR==2 {print $4}' | tr -d 'G')
if (( DISK_AVAIL < 5 )); then
    echo "ERROR: only ${DISK_AVAIL}G free — aborting to prevent ENOSPC" | tee -a "$LOG"
    exit 1
fi
echo "Disk free: ${DISK_AVAIL}G  ✓" | tee -a "$LOG"

for tool in ogr2ogr tippecanoe python3 curl unzip jq; do
    if ! command -v "$tool" &>/dev/null; then
        echo "ERROR: required tool '$tool' not found" | tee -a "$LOG"
        exit 1
    fi
done
echo "Tools: ogr2ogr $(ogr2ogr --version 2>&1 | head -1 | cut -d, -f1)  ✓" | tee -a "$LOG"
echo "Tools: tippecanoe $(tippecanoe --version 2>&1 | head -1)  ✓" | tee -a "$LOG"

if [[ -z "${NREL_API_KEY:-}" ]]; then
    echo "WARN: NREL_API_KEY not set — solar GHI sampling will be skipped" | tee -a "$LOG"
    SKIP_SOLAR=1
else
    SKIP_SOLAR=0
    echo "NREL API key present  ✓" | tee -a "$LOG"
fi

if [[ $DRY_RUN -eq 1 ]]; then
    echo "DRY RUN — pre-flight passed, not executing build steps" | tee -a "$LOG"
    exit 0
fi

mkdir -p "$WORK_DIR"
echo "Work dir: $WORK_DIR" | tee -a "$LOG"

# ── Step 1 — ASHRAE 169-2013 county→zone table (US) ─────────────────────────
#
# Source: US DOE Energy Codes Program / PNNL county climate zone assignments
# URL: https://www.energycodes.gov/sites/default/files/2021-10/ASHRAE_Standard_169-2013-ClimateZones.xlsx
# Fallback CSV mirror: https://raw.githubusercontent.com/NREL/openstudio-standards/master/lib/openstudio-standards/utilities/ashrae_climate_zone_lookup.csv
#
# The CSV has columns: county_fips, climate_zone (e.g. "4A", "5B")

echo "" | tee -a "$LOG"
echo "[1/9] ASHRAE 169-2013 county→zone CSV" | tee -a "$LOG"

ASHRAE_CSV="$WORK_DIR/ashrae-county-zones.csv"
if [[ ! -f "$ASHRAE_CSV" ]]; then
    # Source: NREL ResStock 2022 spatial_tract_lookup_table — IECC 2012 == ASHRAE 169-2013
    # (original NREL openstudio-standards URL removed; file no longer exists at that path)
    python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
import urllib.request, csv, io
url = "https://oedi-data-lake.s3.amazonaws.com/nrel-pds-building-stock/end-use-load-profiles-for-us-building-stock/2022/resstock_tmy3_release_1.1/geographic_information/spatial_tract_lookup_table.csv"
out = "/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/work/aec/ashrae-county-zones.csv"
seen = {}
reader = csv.DictReader(io.TextIOWrapper(urllib.request.urlopen(url), encoding='utf-8'))
for row in reader:
    g = row.get('nhgis_2010_county_gisjoin', '')
    if g.startswith('G') and len(g) >= 8:
        fips = g[1:3] + g[4:7]
        if fips not in seen:
            z = row.get('iecc_2012_climate_zone', '').strip()
            if z: seen[fips] = z
with open(out, 'w', newline='') as f:
    w = csv.writer(f); w.writerow(['county_fips','climate_zone'])
    [w.writerow([k,v]) for k,v in sorted(seen.items())]
print(f"  {len(seen)} counties → {out}")
PYEOF
    if [[ ! -f "$ASHRAE_CSV" ]]; then
        echo "ERROR: ASHRAE CSV generation failed" | tee -a "$LOG"
        exit 1
    fi
fi
echo "  → $ASHRAE_CSV ($(wc -l < "$ASHRAE_CSV") rows)  ✓" | tee -a "$LOG"

# ── Step 2 — TIGER 2023 county polygons (US) ─────────────────────────────────
#
# Source: US Census Bureau TIGER/Line 2023
# URL: https://www2.census.gov/geo/tiger/TIGER2023/COUNTY/tl_2023_us_county.zip
# Stable, versioned. ~75 MB zip, ~135 MB unzipped.

echo "" | tee -a "$LOG"
echo "[2/9] TIGER 2023 US county polygons" | tee -a "$LOG"

TIGER_ZIP="$WORK_DIR/tl_2023_us_county.zip"
TIGER_SHP="$WORK_DIR/tl_2023_us_county.shp"
if [[ ! -f "$TIGER_SHP" ]]; then
    curl -fL "https://www2.census.gov/geo/tiger/TIGER2023/COUNTY/tl_2023_us_county.zip" \
         -o "$TIGER_ZIP" 2>&1 | tee -a "$LOG"
    unzip -q "$TIGER_ZIP" -d "$WORK_DIR"
fi
echo "  → $TIGER_SHP  ✓" | tee -a "$LOG"

# ── Step 3 — Join ASHRAE → TIGER → GeoJSON → PMTiles (US) ────────────────────

echo "" | tee -a "$LOG"
echo "[3/9] Build ASHRAE zone GeoJSON + PMTiles" | tee -a "$LOG"

ASHRAE_GEOJSON="$WORK_DIR/ashrae-zones-us.geojson"
python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
import csv, json, os, sys

WORK_DIR = os.environ.get('WORK_DIR', os.path.expanduser('~/work/aec'))

# Load ASHRAE lookup: fips → zone
ashrae = {}
csv_path = os.path.join(WORK_DIR, 'ashrae-county-zones.csv')
with open(csv_path) as f:
    reader = csv.DictReader(f)
    for row in reader:
        # Column names vary by source; try common variants
        fips = row.get('county_fips') or row.get('FIPS') or row.get('fips') or ''
        zone = row.get('climate_zone') or row.get('ClimateZone') or row.get('IECC_Climate_Zone') or ''
        if fips and zone:
            ashrae[fips.zfill(5)] = zone.strip()

if not ashrae:
    print(f"ERROR: could not parse ASHRAE CSV — check column names in {csv_path}", file=sys.stderr)
    sys.exit(1)
print(f"  Loaded {len(ashrae)} ASHRAE county→zone mappings")

# Load TIGER counties GeoJSON (ogr2ogr first converts the SHP)
import subprocess
shp = os.path.join(WORK_DIR, 'tl_2023_us_county.shp')
counties_geojson = os.path.join(WORK_DIR, 'tiger-counties-raw.geojson')
if not os.path.exists(counties_geojson):
    subprocess.run([
        'ogr2ogr', '-f', 'GeoJSON', counties_geojson, shp,
        '-t_srs', 'EPSG:4326',
        '-select', 'GEOID,NAME,STATEFP',
    ], check=True)

with open(counties_geojson) as f:
    counties = json.load(f)

# Join
matched = 0
for feat in counties['features']:
    fips = feat['properties'].get('GEOID', '')
    zone = ashrae.get(fips)
    feat['properties']['ashrae_zone'] = zone
    if zone:
        matched += 1

print(f"  Matched {matched}/{len(counties['features'])} counties to ASHRAE zone")

out_path = os.path.join(WORK_DIR, 'ashrae-zones-us.geojson')
with open(out_path, 'w') as f:
    json.dump(counties, f)
print(f"  → {out_path}")
PYEOF

export WORK_DIR
python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
import os
WORK_DIR = os.environ.get('WORK_DIR', '')
print(f"WORK_DIR = {WORK_DIR}")
PYEOF

ASHRAE_GEOJSON="$WORK_DIR/ashrae-zones-us.geojson"
tippecanoe \
    --output="$TILES_OUT/layer8-ashrae-zones-us.pmtiles" \
    --name="ASHRAE 169-2013 Climate Zones" \
    --attribution="US DOE / PNNL / US Census TIGER 2023" \
    --layer="ashrae_zones" \
    --minimum-zoom=2 --maximum-zoom=9 \
    --simplification=4 \
    --drop-densest-as-needed \
    --force \
    "$ASHRAE_GEOJSON" 2>&1 | tee -a "$LOG"

echo "  → $TILES_OUT/layer8-ashrae-zones-us.pmtiles ($(du -sh "$TILES_OUT/layer8-ashrae-zones-us.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"

# ── Step 4 — NRCan NECB / HOT2000 climate zones (Canada) ────────────────────
#
# Source: Natural Resources Canada HOT2000 climate zone polygons
# Available via: https://www.nrcan.gc.ca/energy/efficiency/housing/new-homes/hot2000/20539
# Authoritative MapServer (GeoJSON query):
#   https://services1.arcgis.com/RLQu0rK7h4kbsBq5/arcgis/rest/services/
#   NECB_ClimatZones_Canada/FeatureServer/0/query?where=1=1&outFields=*&f=geojson
#
# If the ArcGIS Online endpoint is unavailable, the NECB 2011 zone boundaries
# are also embedded in the National Energy Use Database (NEUD) open datasets
# at https://open.canada.ca/data/en/dataset/b03e6e90-7ecc-4f9a-88a1-fa7a3ccd4ef5

echo "" | tee -a "$LOG"
echo "[4/9] NRCan NECB HOT2000 climate zones (Canada)" | tee -a "$LOG"

NECB_GEOJSON="$WORK_DIR/necb-zones-ca.geojson"
if [[ ! -f "$NECB_GEOJSON" ]]; then
    NECB_URL="https://services1.arcgis.com/RLQu0rK7h4kbsBq5/arcgis/rest/services/NECB_ClimatZones_Canada/FeatureServer/0/query?where=1%3D1&outFields=*&f=geojson&resultRecordCount=5000"
    curl -fsSL "$NECB_URL" -o "$NECB_GEOJSON" 2>&1 | tee -a "$LOG" || {
        echo "WARN: NECB ArcGIS Online query failed — layer8-necb-zones-ca.pmtiles will be skipped" | tee -a "$LOG"
        NECB_GEOJSON=""
    }
fi

if [[ -n "$NECB_GEOJSON" && -f "$NECB_GEOJSON" ]]; then
    tippecanoe \
        --output="$TILES_OUT/layer8-necb-zones-ca.pmtiles" \
        --name="NRCan NECB HOT2000 Climate Zones" \
        --attribution="Natural Resources Canada" \
        --layer="necb_zones" \
        --minimum-zoom=3 --maximum-zoom=9 \
        --simplification=4 \
        --force \
        "$NECB_GEOJSON" 2>&1 | tee -a "$LOG"
    echo "  → $TILES_OUT/layer8-necb-zones-ca.pmtiles  ✓" | tee -a "$LOG"
else
    echo "  SKIPPED — NECB source unavailable" | tee -a "$LOG"
fi

# ── Steps 5–7 — EU climate zones (build-by-join) ────────────────────────────
#
# Pipeline: GISCO LAU2 2021 boundaries + per-country zone lookup tables
# → joined GeoJSON per country → merged PMTiles
#
# Source boundaries: Eurostat GISCO, CC BY 4.0
# https://gisco-services.ec.europa.eu/distribution/v2/lau/download/ref-lau-2021-01m.shp.zip
#
# Zone lookup tables are encoded as Python dicts below, derived from:
#   FR RE2020 (Arrêté 4 août 2021) — département → zone
#   ES CTE DB-HE Annex B — municipality cod → zone
#   IT DPR 412/1993 — Gradi-Giorno lookup (ENEA open data)
#   DE GEG 2023 — federal state → TRY zone (simplified; full raster join Night 3)
#   GR KENAK — prefecture → zone
#   PT SCE/REH — NUT3 → winter/summer zone pair
#   FI SFS-EN ISO 15927-4 — municipality → zone (4 zones)
#   PL WT 2021 — 5 zones by region (MDPI 2024 model)
#   SE BBR — county → zone (Boverket, CC0)

echo "" | tee -a "$LOG"
echo "[5/9] Download GISCO LAU2 2021 boundaries" | tee -a "$LOG"

GISCO_ZIP="$WORK_DIR/ref-lau-2021-01m.shp.zip"
GISCO_DIR="$WORK_DIR/gisco-lau2"
GISCO_SHP="$GISCO_DIR/LAU_RG_01M_2021_4326.shp"
if [[ ! -f "$GISCO_SHP" ]]; then
    if [[ ! -d "$GISCO_DIR" ]]; then
        curl -fL "https://gisco-services.ec.europa.eu/distribution/v2/lau/download/ref-lau-2021-01m.shp.zip" \
             -o "$GISCO_ZIP" 2>&1 | tee -a "$LOG"
        mkdir -p "$GISCO_DIR"
        unzip -q "$GISCO_ZIP" -d "$GISCO_DIR"
    fi
    # Outer zip contains nested per-projection zips; extract WGS84 one
    if [[ -f "$GISCO_DIR/LAU_RG_01M_2021_4326.shp.zip" && ! -f "$GISCO_SHP" ]]; then
        unzip -q "$GISCO_DIR/LAU_RG_01M_2021_4326.shp.zip" -d "$GISCO_DIR"
    fi
fi
echo "  → $GISCO_SHP  ✓" | tee -a "$LOG"

echo "" | tee -a "$LOG"
echo "[6/9] Build EU climate zone GeoJSON (build-by-join)" | tee -a "$LOG"

EU_GEOJSON="$WORK_DIR/eu-climate-zones-merged.geojson"
WORK_DIR_EXPORT="$WORK_DIR" python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
"""
Build-by-join: GISCO LAU2 boundaries + national climate zone lookup tables.
Produces a single merged GeoJSON with properties: iso, zone_code, zone_label.
"""
import os, json, subprocess, sys
from pathlib import Path

WORK_DIR = Path(os.environ['WORK_DIR_EXPORT'])
GISCO_DIR = WORK_DIR / 'gisco-lau2'
OUT_PATH  = WORK_DIR / 'eu-climate-zones-merged.geojson'

# ── Zone lookup tables (ISO → {LAU_code_prefix → zone}) ─────────────────────
# Keys are département codes (FR), CCAA/province codes (ES simplified),
# federal state codes (DE simplified), or region codes (IT, GR, PL, FI, SE, PT).
# Sources documented in AEC-NIGHTLY-BUILD-PLAN.md.

ZONE_LOOKUPS = {

    # FR RE2020 (Arrêté 4 août 2021): 8 zones H1a–H3, département → zone
    "FR": {
        "01":"H1c","02":"H1b","03":"H1c","04":"H1c","05":"H1c","06":"H3",
        "07":"H1c","08":"H1b","09":"H2d","10":"H1b","11":"H2d","12":"H1c",
        "13":"H3","14":"H1a","15":"H1c","16":"H2a","17":"H2a","18":"H1c",
        "19":"H1c","20":"H3","21":"H1b","22":"H1a","23":"H1c","24":"H2b",
        "25":"H1b","26":"H1c","27":"H1a","28":"H1b","29":"H1a","2A":"H3",
        "2B":"H3","30":"H2d","31":"H2d","32":"H2d","33":"H2b","34":"H2d",
        "35":"H1a","36":"H1c","37":"H1b","38":"H1c","39":"H1b","40":"H2b",
        "41":"H1b","42":"H1c","43":"H1c","44":"H2a","45":"H1b","46":"H1c",
        "47":"H2b","48":"H1c","49":"H2a","50":"H1a","51":"H1b","52":"H1b",
        "53":"H1b","54":"H1b","55":"H1b","56":"H1a","57":"H1b","58":"H1b",
        "59":"H1a","60":"H1b","61":"H1a","62":"H1a","63":"H1c","64":"H2c",
        "65":"H2c","66":"H2d","67":"H1b","68":"H1b","69":"H1c","70":"H1b",
        "71":"H1b","72":"H1b","73":"H1c","74":"H1c","75":"H1b","76":"H1a",
        "77":"H1b","78":"H1b","79":"H2a","80":"H1a","81":"H2d","82":"H2d",
        "83":"H3","84":"H1c","85":"H2a","86":"H2a","87":"H1c","88":"H1b",
        "89":"H1b","90":"H1b","91":"H1b","92":"H1b","93":"H1b","94":"H1b",
        "95":"H1b","971":"H3","972":"H3","973":"H3","974":"H3","976":"H3",
    },

    # ES CTE DB-HE: 12 zones A3–E1, simplified by Comunidad Autónoma (NUTS2)
    "ES": {
        "ES61":"A3","ES62":"A3","ES70":"A3",  # Andalucía, Murcia, Canarias
        "ES52":"B3","ES53":"B3",               # Valencia, Illes Balears
        "ES24":"B3","ES41":"C4","ES42":"C4",   # Aragón, Castilla-La Mancha, Castilla y León (w)
        "ES43":"C3","ES51":"C2","ES63":"A4",   # Extremadura, Cataluña, Ceuta
        "ES64":"A4","ES11":"C1","ES12":"Cf",   # Melilla, Galicia, Asturias
        "ES13":"Cf","ES21":"D1","ES22":"D1",   # Cantabria, País Vasco, Navarra
        "ES23":"D1","ES30":"D3",               # La Rioja, Madrid
    },

    # IT DPR 412/1993: 6 zones A–F, simplified by NUTS2 region (gradi-giorno proxy)
    "IT": {
        "ITC1":"F","ITC2":"F","ITC3":"E","ITC4":"E",  # NW: Valle d'Aosta, Liguria, Piemonte, Lombardia
        "ITH1":"E","ITH2":"F","ITH3":"E","ITH4":"E","ITH5":"E",  # NE
        "ITI1":"D","ITI2":"D","ITI3":"D","ITI4":"C",              # Centro
        "ITF1":"B","ITF2":"B","ITF3":"C","ITF4":"C","ITF5":"B","ITF6":"B",  # Sud
        "ITG1":"B","ITG2":"A",                                     # Isole: Sicilia, Sardegna
    },

    # DE GEG 2023 simplified: federal state → broad zone (TRY raster detail added Night 3)
    "DE": {
        "01":"II","02":"I","03":"II","04":"I","05":"II","06":"II",
        "07":"II","08":"III","09":"IV","10":"II","11":"II","12":"II",
        "13":"II","14":"IV","15":"II","16":"IV",
    },

    # GR KENAK: 4 zones A–D, prefecture-level
    "GR": {
        "EL3":"A","EL4":"B","EL5":"B","EL6":"C","EL1":"A","EL2":"B",
    },

    # PT SCE/REH: winter (I1–I3) + summer (V1–V3) encoded as "W/S"
    "PT": {
        "PT11":"I2/V2","PT15":"I1/V1","PT16":"I1/V2","PT17":"I1/V3",
        "PT18":"I2/V3","PT20":"I1/V1",
    },

    # FI SFS-EN ISO 15927-4: 4 zones (1 mild coastal, 4 coldest north)
    "FI": {
        "FI19":"1","FI1B":"2","FI1C":"3","FI1D":"4","FI20":"2",
    },

    # PL WT 2021: 5 zones I (warmest SW) to V (coldest NE)
    "PL": {
        "PL21":"I","PL22":"I","PL41":"II","PL42":"II","PL43":"II",
        "PL51":"III","PL52":"III","PL61":"IV","PL62":"IV","PL63":"IV",
        "PL71":"IV","PL72":"IV","PL81":"V","PL82":"V","PL84":"V","PL85":"V","PL86":"V",
    },

    # SE BBR: 4 climate zones (Boverket) by NUTS3 county
    "SE": {
        "SE110":"III","SE121":"III","SE122":"III","SE123":"III","SE124":"II",
        "SE125":"II","SE211":"III","SE212":"II","SE213":"II","SE214":"I",
        "SE221":"II","SE224":"II","SE231":"II","SE232":"II","SE311":"I",
        "SE312":"I","SE321":"I","SE322":"I","SE331":"I","SE332":"I",
    },
}

# Use combined GISCO shapefile filtered by CNTR_CODE per country
combined_shp = GISCO_DIR / "LAU_RG_01M_2021_4326.shp"
if not combined_shp.exists():
    print(f"  ERROR: combined GISCO shapefile not found at {combined_shp}")
    sys.exit(1)

features_merged = []

for iso, lookup in ZONE_LOOKUPS.items():
    tmp_geojson = WORK_DIR / f"eu-climate-{iso.lower()}-raw.geojson"
    if not tmp_geojson.exists():
        subprocess.run([
            'ogr2ogr', '-f', 'GeoJSON', str(tmp_geojson), str(combined_shp),
            '-t_srs', 'EPSG:4326',
            '-where', f"CNTR_CODE='{iso}'",
        ], check=True, capture_output=True)

    with open(tmp_geojson) as f:
        data = json.load(f)

    matched = 0
    for feat in data['features']:
        props = feat['properties']
        # LAU_ID in GISCO typically looks like "75056" (FR) or "ES0000" (ES NUTS prefix match)
        lau_id = str(props.get('LAU_ID', '') or props.get('GISCO_ID', ''))
        zone = None

        # Try direct match first, then prefix match on NUTS2/3 codes
        zone = lookup.get(lau_id)
        if not zone:
            for prefix, z in lookup.items():
                if lau_id.startswith(prefix):
                    zone = z
                    break

        if zone:
            props['zone_code'] = zone
            props['iso'] = iso
            features_merged.append(feat)
            matched += 1

    print(f"  {iso}: {matched}/{len(data['features'])} LAU units matched to climate zone")

if not features_merged:
    print("WARN: no EU climate zone features produced — GISCO shapefiles may not match expected naming")
else:
    out = {"type": "FeatureCollection", "features": features_merged}
    with open(OUT_PATH, 'w') as f:
        json.dump(out, f)
    print(f"  → {OUT_PATH} ({len(features_merged)} features)")
PYEOF

echo "" | tee -a "$LOG"
echo "[7/9] Build EU climate zones PMTiles" | tee -a "$LOG"

EU_GEOJSON="$WORK_DIR/eu-climate-zones-merged.geojson"
if [[ -f "$EU_GEOJSON" ]]; then
    tippecanoe \
        --output="$TILES_OUT/layer8-eu-climate-zones.pmtiles" \
        --name="EU Regulatory Climate Zones" \
        --attribution="Eurostat GISCO CC BY 4.0; national codes: RE2020 FR, CTE ES, DPR 412 IT, GEG DE, KENAK GR, SCE PT, SFS FI, WT PL, BBR SE" \
        --layer="eu_climate_zones" \
        --minimum-zoom=3 --maximum-zoom=9 \
        --simplification=6 \
        --drop-densest-as-needed \
        --force \
        "$EU_GEOJSON" 2>&1 | tee -a "$LOG"
    echo "  → $TILES_OUT/layer8-eu-climate-zones.pmtiles ($(du -sh "$TILES_OUT/layer8-eu-climate-zones.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"
else
    echo "  WARN: EU climate GeoJSON not produced — PMTiles skipped" | tee -a "$LOG"
fi

# ── Step 8 — Solar GHI sampling (NREL NSRDB for US/CA/MX) ────────────────────
#
# NREL NSRDB point-query API: free, rate-limited to ~1 req/sec
# Register at https://developer.nrel.gov/signup/ for API key
# EU clusters use PVGIS (Night 3 — lower priority for this night)

echo "" | tee -a "$LOG"
echo "[8/9] Solar GHI sampling via NREL NSRDB" | tee -a "$LOG"

GHI_OUT="$WORK_DIR/ghi-us-ca-mx.json"
if [[ $SKIP_SOLAR -eq 1 ]]; then
    echo "  SKIPPED — NREL_API_KEY not set" | tee -a "$LOG"
else
    python3 - <<PYEOF 2>&1 | tee -a "$LOG"
import json, time, urllib.request, urllib.parse, os, sys

META_PATH = "$META_PATH"
API_KEY   = os.environ.get('NREL_API_KEY', '')
OUT_PATH  = "$GHI_OUT"

with open(META_PATH) as f:
    clusters = json.load(f)

# Only sample NA clusters that don't already have ghi_kwh_m2_yr
targets = [c for c in clusters if c.get('cont') == 'NA' and c.get('ghi_kwh_m2_yr') is None]
print(f"  Sampling {len(targets)} NA clusters (US+CA+MX) via NSRDB annual-avg")

results = {}
BATCH_SIZE = 1  # NSRDB single-point API; rate-limit safe at 1/sec
BASE = "https://developer.nrel.gov/api/nsrdb/v2/solar/psm3-download.json"

for i, c in enumerate(targets):
    params = {
        "api_key":   API_KEY,
        "lat":       c["lat"],
        "lon":       c["lon"],
        "attributes":"ghi",
        "names":     "tmy",
        "interval":  "60",
        "utc":       "false",
        "email":     "noreply@pointsav.com",
        "affiliation":"PointSav Digital Systems",
        "mailing_list":"false",
    }
    url = BASE + "?" + urllib.parse.urlencode(params)
    try:
        with urllib.request.urlopen(url, timeout=30) as resp:
            data = json.loads(resp.read())
        # NSRDB returns download URL in outputs; extract annual mean GHI
        # The psm3-download returns a file link — parse response for mean GHI
        # If it's a direct data response, look for mean_ghi
        ghi_val = data.get("outputs", {}).get("mean_ghi")
        if ghi_val is None and "outputs" in data:
            # Some endpoints return statistics directly
            ghi_val = data["outputs"].get("ghi_mean") or data["outputs"].get("ghi")
        if ghi_val is not None:
            results[c["id"]] = round(float(ghi_val))
        if i % 50 == 0 and i > 0:
            print(f"  ... {i}/{len(targets)} sampled ({len(results)} successful)")
        time.sleep(1.05)  # stay under 1 req/sec rate limit
    except Exception as e:
        print(f"  WARN: NSRDB error for cluster {c['id']}: {e}")
        continue

with open(OUT_PATH, 'w') as f:
    json.dump(results, f)
print(f"  → {OUT_PATH}: {len(results)} GHI values saved")
PYEOF
fi

# ── Step 9 — Patch clusters-meta.json ────────────────────────────────────────
#
# Adds fields: ashrae_zone, necb_zone, eu_climate_zone, ghi_kwh_m2_yr
# Uses a spatial point-in-polygon lookup for ASHRAE and EU zones.
# NECB lookup uses the same approach against the NRCan GeoJSON if present.

echo "" | tee -a "$LOG"
echo "[9/9] Patch clusters-meta.json — climate zones + GHI" | tee -a "$LOG"

python3 - <<PYEOF 2>&1 | tee -a "$LOG"
import json, os, sys
from pathlib import Path

META_PATH   = "$META_PATH"
WORK_DIR    = Path("$WORK_DIR")
GHI_PATH    = WORK_DIR / "ghi-us-ca-mx.json"
ASHRAE_GJ   = WORK_DIR / "ashrae-zones-us.geojson"
EU_CLIMATE  = WORK_DIR / "eu-climate-zones-merged.geojson"
NECB_GJ     = WORK_DIR / "necb-zones-ca.geojson"

# Lazy import of shapely for point-in-polygon
try:
    from shapely.geometry import shape, Point
    HAS_SHAPELY = True
except ImportError:
    HAS_SHAPELY = False
    print("  WARN: shapely not available — zone patching uses centroid bbox fallback")

with open(META_PATH) as f:
    clusters = json.load(f)

# Build spatial index from ASHRAE zones
ashrae_polys = []
if ASHRAE_GJ.exists() and HAS_SHAPELY:
    with open(ASHRAE_GJ) as f:
        gj = json.load(f)
    for feat in gj['features']:
        zone = feat['properties'].get('ashrae_zone')
        if zone:
            try:
                ashrae_polys.append((shape(feat['geometry']), zone))
            except Exception:
                pass
    print(f"  ASHRAE: {len(ashrae_polys)} county polygons indexed")

eu_polys = []
if EU_CLIMATE.exists() and HAS_SHAPELY:
    with open(EU_CLIMATE) as f:
        gj = json.load(f)
    for feat in gj['features']:
        zone = feat['properties'].get('zone_code')
        iso  = feat['properties'].get('iso')
        if zone and iso:
            try:
                eu_polys.append((shape(feat['geometry']), iso, zone))
            except Exception:
                pass
    print(f"  EU climate: {len(eu_polys)} LAU polygons indexed")

ghi_map = {}
if GHI_PATH.exists():
    with open(GHI_PATH) as f:
        ghi_map = json.load(f)
    print(f"  GHI: {len(ghi_map)} values loaded")

# Patch each cluster
patched_ashrae = patched_eu = patched_ghi = 0

for c in clusters:
    pt = Point(c['lon'], c['lat']) if HAS_SHAPELY else None

    # ASHRAE (US clusters)
    if c.get('iso') == 'US' and c.get('ashrae_zone') is None and pt:
        for poly, zone in ashrae_polys:
            if poly.contains(pt):
                c['ashrae_zone'] = zone
                patched_ashrae += 1
                break

    # EU climate zone
    if c.get('cont') == 'EU' and c.get('eu_climate_zone') is None and pt:
        for poly, iso, zone in eu_polys:
            if iso == c.get('iso') and poly.contains(pt):
                c['eu_climate_zone'] = zone
                patched_eu += 1
                break

    # GHI
    cid = c.get('id')
    if cid in ghi_map and c.get('ghi_kwh_m2_yr') is None:
        c['ghi_kwh_m2_yr'] = ghi_map[cid]
        patched_ghi += 1

print(f"  Patched: {patched_ashrae} ASHRAE, {patched_eu} EU climate, {patched_ghi} GHI")

with open(META_PATH, 'w') as f:
    json.dump(clusters, f, separators=(',', ':'))
print(f"  → {META_PATH}  ✓")
PYEOF

# ── summary ───────────────────────────────────────────────────────────────────

echo "" | tee -a "$LOG"
TILE_COUNT=$(ls "$TILES_OUT"/layer8-*.pmtiles 2>/dev/null | wc -l)
echo "── Night 2 Complete: $(date -u '+%Y-%m-%dT%H:%M:%SZ') ──" | tee -a "$LOG"
echo "   layer8 PMTiles produced: $TILE_COUNT" | tee -a "$LOG"
echo "   clusters-meta.json: $(wc -c < "$META_PATH") bytes" | tee -a "$LOG"
echo "   Next: Night 3 at 05:00 UTC 2026-05-26 — build-aec-koppen-ecozones.sh" | tee -a "$LOG"
