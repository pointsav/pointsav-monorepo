#!/usr/bin/env bash
# build-aec-flood.sh — Night 5 AEC build: flood hazard + wildfire risk
#
# Run at 05:00 UTC 2026-05-28 (Night 5 of AEC staged rollout).
# See .agent/AEC-NIGHTLY-BUILD-PLAN.md for full plan context.
#
# Usage:  bash build-aec-flood.sh [--dry-run]
#
# Produces:
#   layer11-flood-global.pmtiles       — WRI AQUEDUCT 3.0 global 1-in-100yr riverine flood
#   layer12-fema-sfha-us.pmtiles       — FEMA NFHL Special Flood Hazard Areas (US, all 50 states)
#   layer12-flood-eu-regulatory.pmtiles — EU regulatory flood zones (GB/FR/ES/IT/DE INSPIRE WFS)
#   layer15-wildfire-global.pmtiles    — GWIS Fire Weather Index global wildfire risk
#   patches clusters-meta.json         — flood_hazard (categorical), wildfire_hazard (categorical)
#
# Prerequisites:
#   ogr2ogr (GDAL 3.8+), tippecanoe, python3, curl, unzip, jq, 7z (p7zip-full)
#
# !! DISK WARNING: FEMA NFHL state GDB downloads total ~30+ GB of temp data.
# !! AQUEDUCT 3.0 source GeoTIFF is ~5 GB.
# !! Ensure ≥35 GB free on /srv/foundry before running. !!
#
# Estimated runtime: 7–9 hours (dominated by FEMA GDB downloads + AQUEDUCT processing).
#
# Data licences:
#   WRI AQUEDUCT 3.0: CC BY 4.0 (Rentschler & Salhab 2020, WRI)
#   FEMA NFHL: public domain (US Government)
#   EU flood (INSPIRE): varies by country — all CC BY or equivalent OGL
#   GWIS FWI: public domain (JRC / EU Copernicus)

set -euo pipefail

DRY_RUN=0
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN=1

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="$SCRIPT_DIR/work/aec"
export WORK_DIR
LOG="$SCRIPT_DIR/build-aec-flood.log"
STAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
TILES_OUT="/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles"
META_PATH="/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"

echo "──────────────────────────────────────────────" | tee -a "$LOG"
echo "build-aec-flood  $STAMP" | tee -a "$LOG"

# ── pre-flight ────────────────────────────────────────────────────────────────

DISK_AVAIL=$(df -BG /srv/foundry | awk 'NR==2 {print $4}' | tr -d 'G')
if (( DISK_AVAIL < 35 )); then
    echo "ERROR: only ${DISK_AVAIL}G free — Night 5 requires ≥35 GB; aborting" | tee -a "$LOG"
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

# Check Night 4 complete marker (advisory)
if [[ ! -f "$WORK_DIR/.night4-complete" ]]; then
    echo "WARN: .night4-complete marker not found — Night 4 may not have run; proceeding anyway" | tee -a "$LOG"
else
    echo "Night 4 complete marker found  ✓" | tee -a "$LOG"
fi

if [[ $DRY_RUN -eq 1 ]]; then
    echo "DRY RUN — pre-flight passed, not executing build steps" | tee -a "$LOG"
    exit 0
fi

mkdir -p "$WORK_DIR/fema-state-gdbs"
echo "Work dir: $WORK_DIR" | tee -a "$LOG"

# ── Step 1 — WRI AQUEDUCT 3.0 global flood raster ────────────────────────────
#
# Source: World Resources Institute AQUEDUCT Floods 3.0
# Licence: CC BY 4.0
# Dataset: 1-in-100yr riverine inundation depth (inunriver), historical baseline
# Landing page: https://www.wri.org/data/aqueduct-floods-hazard-maps
#
# The 1-in-100yr historical riverine GeoTIFF is available from WRI's S3 bucket.
# Filename pattern: inunriver_historical_000000000WATCH_1980_rp00100.tif
# (~5 GB download, global 150m resolution)

echo "" | tee -a "$LOG"
echo "[1/17] WRI AQUEDUCT 3.0 — 1-in-100yr riverine flood (global, CC BY 4.0)" | tee -a "$LOG"

AQUEDUCT_TIF="$WORK_DIR/aqueduct-flood-100yr.tif"
AQUEDUCT_URL="https://wri-projects.s3.amazonaws.com/AqueductFloodTool/download/v2/inunriver_historical_000000000WATCH_1980_rp00100.tif"

if [[ ! -f "$AQUEDUCT_TIF" || $(stat -c%s "$AQUEDUCT_TIF") -lt 100000000 ]]; then
    echo "  Downloading AQUEDUCT raster (~5 GB, may take 20–40 min)..." | tee -a "$LOG"
    curl -L --retry 3 --retry-delay 30 -o "$AQUEDUCT_TIF" "$AQUEDUCT_URL" \
        2>&1 | tee -a "$LOG"
fi
if [[ ! -f "$AQUEDUCT_TIF" || $(stat -c%s "$AQUEDUCT_TIF") -lt 100000000 ]]; then
    echo "ERROR: AQUEDUCT raster download failed or too small — check URL:" | tee -a "$LOG"
    echo "  $AQUEDUCT_URL" | tee -a "$LOG"
    echo "  Landing page: https://www.wri.org/data/aqueduct-floods-hazard-maps" | tee -a "$LOG"
    exit 1
fi
echo "  → $AQUEDUCT_TIF ($(du -sh "$AQUEDUCT_TIF" | cut -f1))  ✓" | tee -a "$LOG"

# ── Step 2 — Classify + build AQUEDUCT flood PMTiles ─────────────────────────
#
# Classify raster values into categorical flood depth bins:
#   0        = no flood
#   0–0.1 m  = very_low
#   0.1–0.5 m = low
#   0.5–1.5 m = medium
#   1.5–5 m   = high
#   >5 m      = very_high
# Use gdal_calc.py to produce a byte-class raster, then gdal_polygonize to
# convert to vector polygons, then tippecanoe for PMTiles.

echo "" | tee -a "$LOG"
echo "[2/17] Build layer11-flood-global.pmtiles from AQUEDUCT" | tee -a "$LOG"

AQUEDUCT_CLASSIFIED="$WORK_DIR/aqueduct-flood-classified.tif"
AQUEDUCT_GEOJSON="$WORK_DIR/aqueduct-flood-global.geojson"

if [[ ! -f "$AQUEDUCT_GEOJSON" ]]; then
    # Classify into 0–5 byte raster (0=no flood, 1=very_low, 2=low, 3=medium, 4=high, 5=very_high)
    gdal_calc.py \
        -A "$AQUEDUCT_TIF" \
        --outfile="$AQUEDUCT_CLASSIFIED" \
        --calc="(A>0)*(A<=0.1)*1 + (A>0.1)*(A<=0.5)*2 + (A>0.5)*(A<=1.5)*3 + (A>1.5)*(A<=5.0)*4 + (A>5.0)*5" \
        --type=Byte \
        --NoDataValue=0 \
        --co="COMPRESS=DEFLATE" \
        2>&1 | tee -a "$LOG"

    gdal_polygonize.py "$AQUEDUCT_CLASSIFIED" -f GeoJSON "$AQUEDUCT_GEOJSON" flood_zones flood_class \
        2>&1 | tee -a "$LOG"

    # Map integer class to label
    python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
import json, pathlib
LABELS = {1:'very_low',2:'low',3:'medium',4:'high',5:'very_high'}
p = pathlib.Path("$AQUEDUCT_GEOJSON")
gj = json.loads(p.read_text())
for f in gj['features']:
    cls = f['properties'].get('flood_class',0)
    f['properties']['flood_depth_cat'] = LABELS.get(cls, 'unknown')
p.write_text(json.dumps(gj))
PYEOF
fi

if [[ -f "$AQUEDUCT_GEOJSON" ]]; then
    tippecanoe \
        --output="$TILES_OUT/layer11-flood-global.pmtiles" \
        --name="AQUEDUCT 3.0 Global Flood Hazard (1-in-100yr)" \
        --attribution="WRI AQUEDUCT Floods 3.0, CC BY 4.0" \
        --layer="flood_zones" \
        --minimum-zoom=3 \
        --maximum-zoom=8 \
        --simplification=6 \
        --drop-densest-as-needed \
        --force \
        "$AQUEDUCT_GEOJSON" \
        2>&1 | tee -a "$LOG"
    echo "  → $TILES_OUT/layer11-flood-global.pmtiles ($(du -sh "$TILES_OUT/layer11-flood-global.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"
else
    echo "  WARN: AQUEDUCT GeoJSON not produced — skipping global flood PMTiles" | tee -a "$LOG"
fi

# ── Steps 3–8 — FEMA NFHL Special Flood Hazard Areas (US, all 50 states) ─────
#
# Source: FEMA National Flood Hazard Layer (NFHL)
# Licence: public domain (US Government)
# API: https://msc.fema.gov/portal/downloadProduct?productTypeID=NFHL
# Bulk state downloads (GDB zip, ~100–500 MB each state):
#   https://hazards.fema.gov/nfhl/rest/services/public/NFHL/MapServer/exts/LargeFIRMDownloadControllerSoe/DownloadService/downloadFile?countycode={FIPS5}&filetype=&state={STATE_FIPS}
#
# Alternative bulk download for all states (S_FLD_HAZ_AR layer, SFHA zones):
#   State-by-state NFHL GDB at:
#   https://www.fema.gov/flood-maps/national-flood-hazard-layer

echo "" | tee -a "$LOG"
echo "[3/17] FEMA NFHL — US flood zones (50 states, ~30 GB download)" | tee -a "$LOG"
echo "  WARNING: This step downloads ~30 GB of GDB files. ETA: 2–4 hours." | tee -a "$LOG"

FEMA_MERGED="$WORK_DIR/fema-sfha-merged.geojson"

# State FIPS codes 01–56 (excluding 03, 07, 11 unassigned/DC handled separately, 14, 43, 52)
FEMA_STATES=(
    "01" "02" "04" "05" "06" "08" "09" "10" "11" "12"
    "13" "15" "16" "17" "18" "19" "20" "21" "22" "23"
    "24" "25" "26" "27" "28" "29" "30" "31" "32" "33"
    "34" "35" "36" "37" "38" "39" "40" "41" "42" "44"
    "45" "46" "47" "48" "49" "50" "51" "53" "54" "55" "56"
)

# SFHA flood zone codes (Special Flood Hazard Area = 1-in-100yr)
SFHA_ZONES="'A','AE','AH','AO','AR','A99','V','VE'"

if [[ ! -f "$FEMA_MERGED" ]]; then
    FEMA_PARTS=()

    for STATE_FIPS in "${FEMA_STATES[@]}"; do
        STATE_GDB_ZIP="$WORK_DIR/fema-state-gdbs/nfhl_${STATE_FIPS}.zip"
        STATE_GEOJSON="$WORK_DIR/fema-state-gdbs/sfha_${STATE_FIPS}.geojson"

        if [[ -f "$STATE_GEOJSON" ]]; then
            FEMA_PARTS+=("$STATE_GEOJSON")
            continue
        fi

        # Download state NFHL GDB
        if [[ ! -f "$STATE_GDB_ZIP" ]]; then
            echo "    Downloading state $STATE_FIPS..." | tee -a "$LOG"
            curl -L --retry 3 --retry-delay 15 --max-time 1800 \
                -o "$STATE_GDB_ZIP" \
                "https://hazards.fema.gov/nfhl/rest/services/public/NFHL/MapServer/exts/LargeFIRMDownloadControllerSoe/DownloadService/downloadFile?state=${STATE_FIPS}&filetype=gdb" \
                2>&1 | grep -E "^[0-9]|Error|error" | tee -a "$LOG" || true
        fi

        if [[ ! -f "$STATE_GDB_ZIP" || $(stat -c%s "$STATE_GDB_ZIP") -lt 50000 ]]; then
            echo "    WARN: State $STATE_FIPS GDB download failed — skipping" | tee -a "$LOG"
            continue
        fi

        # Extract S_FLD_HAZ_AR layer, filter to SFHA zones only
        TMP_GDB_DIR=$(mktemp -d)
        unzip -q "$STATE_GDB_ZIP" -d "$TMP_GDB_DIR" 2>/dev/null || true
        GDB_PATH=$(find "$TMP_GDB_DIR" -name "*.gdb" -maxdepth 2 | head -1)

        if [[ -n "$GDB_PATH" ]]; then
            ogr2ogr -f GeoJSON \
                -t_srs EPSG:4326 \
                -where "FLD_ZONE IN ($SFHA_ZONES)" \
                -select "FLD_ZONE,SFHA_TF" \
                "$STATE_GEOJSON" \
                "$GDB_PATH" \
                "S_FLD_HAZ_AR" \
                2>&1 | tee -a "$LOG" || true
        fi

        rm -rf "$TMP_GDB_DIR"

        [[ -f "$STATE_GDB_ZIP" ]] && rm -f "$STATE_GDB_ZIP"  # reclaim space

        [[ -f "$STATE_GEOJSON" ]] && FEMA_PARTS+=("$STATE_GEOJSON")

        # Progress marker every 10 states
        DONE_COUNT=$(ls "$WORK_DIR/fema-state-gdbs/sfha_"*.geojson 2>/dev/null | wc -l)
        echo "    Progress: $DONE_COUNT / ${#FEMA_STATES[@]} states done" | tee -a "$LOG"
    done

    # Merge all state GeoJSONs
    if [[ ${#FEMA_PARTS[@]} -gt 0 ]]; then
        ogr2ogr -f GeoJSON "$FEMA_MERGED" "${FEMA_PARTS[0]}" 2>&1 | tee -a "$LOG"
        for i in "${!FEMA_PARTS[@]}"; do
            [[ $i -eq 0 ]] && continue
            ogr2ogr -f GeoJSON -update -append "$FEMA_MERGED" "${FEMA_PARTS[$i]}" \
                2>&1 | tee -a "$LOG" || true
        done
    fi
fi

# ── Step 9 — Build FEMA SFHA PMTiles ─────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[9/17] Build layer12-fema-sfha-us.pmtiles" | tee -a "$LOG"

if [[ -f "$FEMA_MERGED" ]]; then
    tippecanoe \
        --output="$TILES_OUT/layer12-fema-sfha-us.pmtiles" \
        --name="FEMA NFHL Special Flood Hazard Areas" \
        --attribution="FEMA National Flood Hazard Layer, public domain" \
        --layer="fema_sfha" \
        --minimum-zoom=5 \
        --maximum-zoom=12 \
        --simplification=3 \
        --drop-densest-as-needed \
        --force \
        "$FEMA_MERGED" \
        2>&1 | tee -a "$LOG"
    echo "  → $TILES_OUT/layer12-fema-sfha-us.pmtiles ($(du -sh "$TILES_OUT/layer12-fema-sfha-us.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"
else
    echo "  WARN: FEMA merged GeoJSON not produced — skipping FEMA PMTiles" | tee -a "$LOG"
fi

# ── Steps 10–12 — EU regulatory flood zones (INSPIRE WFS) ────────────────────
#
# Sources: EU Member State INSPIRE Flood Risk data services
#   GB: Environment Agency WFS — RiskZoneArea (1-in-100yr)
#   FR: Géorisques GASPAR API — PPRI flood plans
#   ES: SNCZI (MITERD) — Zonas inundables (T100)
#   DE: LAWA INSPIRE WFS — HQ100 flood zones (aggregated 16 Länder)
#   IT: IdroGEO ISPRA — P3 high-hazard flood zones
#
# Note: These are INSPIRE-compliant WFS endpoints. Responses may be large
# (~50–200 MB per country). Each is fetched with pagination where required.

echo "" | tee -a "$LOG"
echo "[10/17] EU flood zones (GB, FR, ES) — INSPIRE WFS" | tee -a "$LOG"

EU_FLOOD_PARTS=()

# GB: Environment Agency INSPIRE Flood Map for Planning
GB_GEOJSON="$WORK_DIR/flood-eu-gb.geojson"
if [[ ! -f "$GB_GEOJSON" ]]; then
    echo "  [GB] Environment Agency INSPIRE Flood Zones (RoFRS 1-in-100yr)..." | tee -a "$LOG"
    # EA INSPIRE WFS for Flood Risk Zones (RoFRS):
    curl -L --retry 2 --max-time 900 \
        -o "$GB_GEOJSON" \
        "https://environment.data.gov.uk/arcgis/rest/services/EA/FloodMapForPlanning/MapServer/4/query?where=1%3D1&outFields=*&f=geojson&returnGeometry=true&resultRecordCount=1000&outSR=4326" \
        2>&1 | tee -a "$LOG" || echo "    WARN: GB flood data request failed" | tee -a "$LOG"
    [[ -f "$GB_GEOJSON" && $(stat -c%s "$GB_GEOJSON") -gt 10000 ]] && EU_FLOOD_PARTS+=("$GB_GEOJSON")
fi
[[ -f "$GB_GEOJSON" && $(stat -c%s "$GB_GEOJSON") -gt 10000 ]] && EU_FLOOD_PARTS+=("$GB_GEOJSON") || true

# FR: Géorisques — GASPAR flood plans (T100 inundation zones)
FR_GEOJSON="$WORK_DIR/flood-eu-fr.geojson"
if [[ ! -f "$FR_GEOJSON" ]]; then
    echo "  [FR] Géorisques GASPAR flood zones (T100)..." | tee -a "$LOG"
    curl -L --retry 2 --max-time 900 \
        -o "$FR_GEOJSON" \
        "https://www.georisques.gouv.fr/api/v1/gaspar/azi?rayon=0&page=1&page_size=1000&format=geojson" \
        2>&1 | tee -a "$LOG" || echo "    WARN: FR flood data request failed" | tee -a "$LOG"
    [[ -f "$FR_GEOJSON" && $(stat -c%s "$FR_GEOJSON") -gt 10000 ]] && EU_FLOOD_PARTS+=("$FR_GEOJSON") || true
fi

# ES: SNCZI Zonas inundables (MITERD) — T100 flood zones
ES_GEOJSON="$WORK_DIR/flood-eu-es.geojson"
if [[ ! -f "$ES_GEOJSON" ]]; then
    echo "  [ES] SNCZI T100 flood zones (MITERD)..." | tee -a "$LOG"
    curl -L --retry 2 --max-time 900 \
        -o "$ES_GEOJSON" \
        "https://servicios.idee.es/wfs-inspire/riesgos-naturales/inundaciones?SERVICE=WFS&VERSION=2.0.0&REQUEST=GetFeature&TYPENAMES=hy-p:RiskZone&SRSNAME=EPSG:4326&outputFormat=application/json&count=5000" \
        2>&1 | tee -a "$LOG" || echo "    WARN: ES flood data request failed" | tee -a "$LOG"
    [[ -f "$ES_GEOJSON" && $(stat -c%s "$ES_GEOJSON") -gt 10000 ]] && EU_FLOOD_PARTS+=("$ES_GEOJSON") || true
fi

echo "" | tee -a "$LOG"
echo "[11/17] EU flood zones (DE, IT) — INSPIRE WFS" | tee -a "$LOG"

# DE: LAWA — HQ100 flood zones (all 16 Länder, aggregated WMS/WFS)
DE_GEOJSON="$WORK_DIR/flood-eu-de.geojson"
if [[ ! -f "$DE_GEOJSON" ]]; then
    echo "  [DE] LAWA HQ100 flood zones (16 Länder, UBA INSPIRE)..." | tee -a "$LOG"
    curl -L --retry 2 --max-time 900 \
        -o "$DE_GEOJSON" \
        "https://gis.uba.de/arcgis/rest/services/wasser/Hochwasserrisikokarten/MapServer/1/query?where=1%3D1&outFields=*&f=geojson&returnGeometry=true&outSR=4326&resultRecordCount=2000" \
        2>&1 | tee -a "$LOG" || echo "    WARN: DE flood data request failed" | tee -a "$LOG"
    [[ -f "$DE_GEOJSON" && $(stat -c%s "$DE_GEOJSON") -gt 10000 ]] && EU_FLOOD_PARTS+=("$DE_GEOJSON") || true
fi

# IT: IdroGEO ISPRA — P3 high-hazard flood zones
IT_GEOJSON="$WORK_DIR/flood-eu-it.geojson"
if [[ ! -f "$IT_GEOJSON" ]]; then
    echo "  [IT] IdroGEO ISPRA P3 flood hazard zones..." | tee -a "$LOG"
    curl -L --retry 2 --max-time 900 \
        -o "$IT_GEOJSON" \
        "https://idrogeo.isprambiente.it/api/geoserver/risk/ows?service=WFS&version=2.0.0&request=GetFeature&typeName=risk:pau_ifs&srsName=EPSG:4326&outputFormat=application/json&count=2000" \
        2>&1 | tee -a "$LOG" || echo "    WARN: IT flood data request failed" | tee -a "$LOG"
    [[ -f "$IT_GEOJSON" && $(stat -c%s "$IT_GEOJSON") -gt 10000 ]] && EU_FLOOD_PARTS+=("$IT_GEOJSON") || true
fi

# ── Step 12 — Merge + build EU flood PMTiles ──────────────────────────────────

echo "" | tee -a "$LOG"
echo "[12/17] Merge EU flood zones + build layer12-flood-eu-regulatory.pmtiles" | tee -a "$LOG"

EU_FLOOD_MERGED="$WORK_DIR/flood-eu-regulatory.geojson"
if [[ ${#EU_FLOOD_PARTS[@]} -gt 0 && ! -f "$EU_FLOOD_MERGED" ]]; then
    ogr2ogr -f GeoJSON "$EU_FLOOD_MERGED" "${EU_FLOOD_PARTS[0]}" 2>&1 | tee -a "$LOG"
    for i in "${!EU_FLOOD_PARTS[@]}"; do
        [[ $i -eq 0 ]] && continue
        ogr2ogr -f GeoJSON -update -append "$EU_FLOOD_MERGED" "${EU_FLOOD_PARTS[$i]}" \
            2>&1 | tee -a "$LOG" || true
    done
fi

if [[ -f "$EU_FLOOD_MERGED" ]]; then
    tippecanoe \
        --output="$TILES_OUT/layer12-flood-eu-regulatory.pmtiles" \
        --name="EU Regulatory Flood Zones (INSPIRE)" \
        --attribution="GB EA / FR Géorisques / ES MITERD / DE LAWA / IT ISPRA — INSPIRE WFS" \
        --layer="flood_zones_eu" \
        --minimum-zoom=3 \
        --maximum-zoom=10 \
        --simplification=4 \
        --drop-densest-as-needed \
        --force \
        "$EU_FLOOD_MERGED" \
        2>&1 | tee -a "$LOG"
    echo "  → $TILES_OUT/layer12-flood-eu-regulatory.pmtiles ($(du -sh "$TILES_OUT/layer12-flood-eu-regulatory.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"
else
    echo "  WARN: No EU flood data collected — skipping EU flood PMTiles" | tee -a "$LOG"
fi

# ── Step 13 — GWIS FWI wildfire raster ───────────────────────────────────────
#
# Source: Global Wildfire Information System (GWIS) — Fire Weather Index
# Provider: JRC / EU Copernicus Emergency Management Service
# Licence: public domain (JRC / EU Copernicus)
# Download: historical mean annual FWI raster (global, 0.25° resolution)
# URL: https://effis-gwis-cms.s3.eu-west-1.amazonaws.com/apps/country.profile/mean_fwi_m_1981_2010.tif
#
# FWI classification thresholds (Canadian Forest Service):
#   0–5.2  → none/very_low
#   5.2–11.2 → low
#   11.2–21.3 → moderate
#   21.3–38.0 → high
#   >38.0     → very_high / extreme

echo "" | tee -a "$LOG"
echo "[13/17] GWIS FWI wildfire raster (JRC / EU Copernicus)" | tee -a "$LOG"

GWIS_TIF="$WORK_DIR/gwis-fwi-global.tif"
GWIS_URL="https://effis-gwis-cms.s3.eu-west-1.amazonaws.com/apps/country.profile/mean_fwi_m_1981_2010.tif"

if [[ ! -f "$GWIS_TIF" || $(stat -c%s "$GWIS_TIF") -lt 1000000 ]]; then
    curl -L --retry 3 --retry-delay 15 -o "$GWIS_TIF" "$GWIS_URL" \
        2>&1 | tee -a "$LOG"
fi
if [[ ! -f "$GWIS_TIF" || $(stat -c%s "$GWIS_TIF") -lt 1000000 ]]; then
    echo "  WARN: GWIS FWI raster not downloaded — wildfire layer will be skipped" | tee -a "$LOG"
    SKIP_WILDFIRE=1
else
    echo "  → $GWIS_TIF ($(du -sh "$GWIS_TIF" | cut -f1))  ✓" | tee -a "$LOG"
    SKIP_WILDFIRE=0
fi

# ── Step 14 — Classify + build wildfire PMTiles ───────────────────────────────

echo "" | tee -a "$LOG"
echo "[14/17] Build layer15-wildfire-global.pmtiles" | tee -a "$LOG"

GWIS_CLASSIFIED="$WORK_DIR/gwis-fwi-classified.tif"
GWIS_GEOJSON="$WORK_DIR/gwis-fwi-global.geojson"

if [[ $SKIP_WILDFIRE -eq 0 && ! -f "$GWIS_GEOJSON" ]]; then
    # Classify FWI into 5 levels (1=low, 5=extreme)
    gdal_calc.py \
        -A "$GWIS_TIF" \
        --outfile="$GWIS_CLASSIFIED" \
        --calc="(A>0)*(A<=5.2)*1 + (A>5.2)*(A<=11.2)*2 + (A>11.2)*(A<=21.3)*3 + (A>21.3)*(A<=38.0)*4 + (A>38.0)*5" \
        --type=Byte \
        --NoDataValue=0 \
        --co="COMPRESS=DEFLATE" \
        2>&1 | tee -a "$LOG"

    gdal_polygonize.py "$GWIS_CLASSIFIED" -f GeoJSON "$GWIS_GEOJSON" wildfire_zones fwi_class \
        2>&1 | tee -a "$LOG"

    python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
import json, pathlib
LABELS = {1:'low',2:'moderate',3:'high',4:'very_high',5:'extreme'}
p = pathlib.Path("$GWIS_GEOJSON")
gj = json.loads(p.read_text())
for f in gj['features']:
    cls = f['properties'].get('fwi_class',0)
    f['properties']['wildfire_risk'] = LABELS.get(cls, 'unknown')
p.write_text(json.dumps(gj))
PYEOF
fi

if [[ $SKIP_WILDFIRE -eq 0 && -f "$GWIS_GEOJSON" ]]; then
    tippecanoe \
        --output="$TILES_OUT/layer15-wildfire-global.pmtiles" \
        --name="GWIS FWI Global Wildfire Risk (1981–2010 mean)" \
        --attribution="GWIS / JRC / EU Copernicus, public domain" \
        --layer="wildfire_zones" \
        --minimum-zoom=2 \
        --maximum-zoom=8 \
        --simplification=6 \
        --drop-densest-as-needed \
        --force \
        "$GWIS_GEOJSON" \
        2>&1 | tee -a "$LOG"
    echo "  → $TILES_OUT/layer15-wildfire-global.pmtiles ($(du -sh "$TILES_OUT/layer15-wildfire-global.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"
fi

# ── Steps 15–16 — Patch clusters-meta.json ───────────────────────────────────

echo "" | tee -a "$LOG"
echo "[15/17] Canada AQUEDUCT flood sampling (supplemental)" | tee -a "$LOG"

python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
import json, subprocess, pathlib

META = pathlib.Path("$META_PATH")
AQUEDUCT_TIF = "$AQUEDUCT_TIF"

USE_GDAL = 1 if subprocess.run(["which", "gdallocationinfo"],
                                 capture_output=True).returncode == 0 else 0

def sample_depth(lon, lat):
    if not USE_GDAL:
        return None
    try:
        r = subprocess.run(
            ["gdallocationinfo", "-valonly", "-wgs84", AQUEDUCT_TIF, str(lon), str(lat)],
            capture_output=True, text=True, timeout=15
        )
        v = r.stdout.strip()
        return float(v) if v else 0.0
    except Exception:
        return None

def depth_to_cat(d):
    if d is None or d <= 0:      return None
    elif d <= 0.1:               return 'very_low'
    elif d <= 0.5:               return 'low'
    elif d <= 1.5:               return 'medium'
    elif d <= 5.0:               return 'high'
    else:                        return 'very_high'

clusters = json.loads(META.read_text())
n = 0
for c in clusters:
    if c.get('flood_hazard') is not None:
        continue
    depth = sample_depth(c.get('lon', 0), c.get('lat', 0))
    cat = depth_to_cat(depth)
    if cat:
        c['flood_hazard'] = cat; n += 1
META.write_text(json.dumps(clusters, separators=(',', ':')))
print(f"  AQUEDUCT flood_hazard assigned: {n} clusters")
PYEOF

echo "" | tee -a "$LOG"
echo "[16/17] Patch wildfire_hazard into clusters-meta.json" | tee -a "$LOG"

if [[ $SKIP_WILDFIRE -eq 0 ]]; then
python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
import json, subprocess, pathlib

META = pathlib.Path("$META_PATH")
GWIS_TIF = "$GWIS_TIF"

USE_GDAL = 1 if subprocess.run(["which", "gdallocationinfo"],
                                 capture_output=True).returncode == 0 else 0

FWI_LABELS = {1:'low',2:'moderate',3:'high',4:'very_high',5:'extreme'}

def sample_fwi_class(lon, lat):
    if not USE_GDAL:
        return None
    try:
        r = subprocess.run(
            ["gdallocationinfo", "-valonly", "-wgs84", GWIS_TIF, str(lon), str(lat)],
            capture_output=True, text=True, timeout=15
        )
        v = r.stdout.strip()
        fwi = float(v) if v else 0.0
        if fwi <= 0:    return None
        elif fwi <= 5.2:  return 'low'
        elif fwi <= 11.2: return 'moderate'
        elif fwi <= 21.3: return 'high'
        elif fwi <= 38.0: return 'very_high'
        else:             return 'extreme'
    except Exception:
        return None

clusters = json.loads(META.read_text())
n = 0
for c in clusters:
    if c.get('wildfire_hazard') is not None:
        continue
    cat = sample_fwi_class(c.get('lon', 0), c.get('lat', 0))
    if cat:
        c['wildfire_hazard'] = cat; n += 1
META.write_text(json.dumps(clusters, separators=(',', ':')))
print(f"  GWIS wildfire_hazard assigned: {n} clusters")
PYEOF
else
    echo "  SKIPPED — wildfire raster not available" | tee -a "$LOG"
fi

# ── Step 17 — Summary + phase marker ─────────────────────────────────────────

echo "" | tee -a "$LOG"
TILE_COUNT=$(ls "$TILES_OUT"/layer11-*.pmtiles "$TILES_OUT"/layer12-*.pmtiles "$TILES_OUT"/layer15-*.pmtiles 2>/dev/null | wc -l)
echo "── Night 5 Complete: $(date -u '+%Y-%m-%dT%H:%M:%SZ') ──" | tee -a "$LOG"
echo "   Night 5 PMTiles produced: $TILE_COUNT" | tee -a "$LOG"
echo "   clusters-meta.json: $(wc -c < "$META_PATH") bytes" | tee -a "$LOG"
echo "   AEC rollout complete — all 5 nights done" | tee -a "$LOG"

touch "$WORK_DIR/.night5-complete"
echo "   Phase marker: $WORK_DIR/.night5-complete  ✓" | tee -a "$LOG"
