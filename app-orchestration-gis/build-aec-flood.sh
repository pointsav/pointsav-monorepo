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
# !! DISK: AQUEDUCT GeoTIFF ~90 MB + IT GeoPackage ~1 GB + GFWED monthly NetCDFs
# !! (12 × ~128 MB, deleted after averaging). Peak transient ~2 GB. Ensure ≥10 GB free. !!
#
# Estimated runtime: 1–2 hours (FEMA/GB REST queries + IT GeoPackage + GFWED averaging).
# (FEMA no longer downloads 30 GB of state GDBs — replaced by market-windowed REST query.)
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
if (( DISK_AVAIL < 10 )); then
    echo "ERROR: only ${DISK_AVAIL}G free — Night 5 requires ≥10 GB; aborting" | tee -a "$LOG"
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

# Self-heal stale markers from prior failed runs:
#  - .aqueduct.skip wrongly suppressed the (now-working) AQUEDUCT download
#  - a tiny gwis-fwi-global.tif is a 404 error page, not a raster
[[ -f "$WORK_DIR/.aqueduct.skip" ]] && { rm -f "$WORK_DIR/.aqueduct.skip"; echo "  cleared stale .aqueduct.skip" | tee -a "$LOG"; }
[[ -f "$WORK_DIR/gwis-fwi-global.tif" && $(stat -c%s "$WORK_DIR/gwis-fwi-global.tif") -lt 100000 ]] && rm -f "$WORK_DIR/gwis-fwi-global.tif"

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
AQUEDUCT_SKIP="$WORK_DIR/.aqueduct.skip"
# Primary URL (WRI S3 v2). If this redirects to a landing page, the --fail flag
# causes curl to exit non-zero rather than downloading HTML content.
AQUEDUCT_URLS=(
    "https://wri-projects.s3.amazonaws.com/AqueductFloodTool/download/v2/inunriver_historical_000000000WATCH_1980_rp00100.tif"
    "/vsis3/wri-projects/AqueductFloodTool/download/v2/inunriver_historical_000000000WATCH_1980_rp00100.tif"
)

_aqueduct_valid() {
    [[ -f "$AQUEDUCT_TIF" ]] \
        && (( $(stat -c%s "$AQUEDUCT_TIF") >= 100000000 )) \
        && file "$AQUEDUCT_TIF" 2>/dev/null | grep -qi "TIFF\|GeoTIFF"
}

if [[ -f "$AQUEDUCT_SKIP" ]]; then
    echo "  SKIP: .aqueduct.skip marker present — skipping AQUEDUCT download" | tee -a "$LOG"
elif ! _aqueduct_valid; then
    echo "  Downloading AQUEDUCT raster (~5 GB, may take 20–40 min)..." | tee -a "$LOG"
    for AQUEDUCT_URL in "${AQUEDUCT_URLS[@]}"; do
        echo "  Trying: $AQUEDUCT_URL" | tee -a "$LOG"
        if curl --fail -L --retry 3 --retry-delay 30 -o "$AQUEDUCT_TIF" "$AQUEDUCT_URL" \
                2>&1 | tee -a "$LOG" && _aqueduct_valid; then
            echo "  → $AQUEDUCT_TIF ($(du -sh "$AQUEDUCT_TIF" | cut -f1))  ✓" | tee -a "$LOG"
            break
        fi
        rm -f "$AQUEDUCT_TIF"
    done
    if ! _aqueduct_valid; then
        echo "WARN: AQUEDUCT raster unavailable — skipping step 1 and any steps that require it." | tee -a "$LOG"
        echo "  Primary URL: ${AQUEDUCT_URLS[0]}" | tee -a "$LOG"
        echo "  Landing page: https://www.wri.org/data/aqueduct-floods-hazard-maps" | tee -a "$LOG"
        echo "  To fix: find the correct raster URL and place the GeoTIFF at:" | tee -a "$LOG"
        echo "    $AQUEDUCT_TIF" | tee -a "$LOG"
        touch "$AQUEDUCT_SKIP"
    fi
else
    echo "  → $AQUEDUCT_TIF ($(du -sh "$AQUEDUCT_TIF" | cut -f1))  ✓ (cached)" | tee -a "$LOG"
fi

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

if [[ -f "$AQUEDUCT_SKIP" ]]; then
    echo "  SKIP: .aqueduct.skip marker present — no AQUEDUCT raster to process" | tee -a "$LOG"
elif [[ ! -f "$AQUEDUCT_GEOJSON" ]]; then
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
    python3 - <<PYEOF 2>&1 | tee -a "$LOG"
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
# Source: FEMA National Flood Hazard Layer (NFHL), public domain (US Government).
# The legacy state-GDB bulk download (LargeFIRMDownloadControllerSoe) was RETIRED
# (404, 2026). Current source: the NFHL ArcGIS REST service, layer 28 "Flood
# Hazard Zones". Full-national REST export is impractical, so we query SFHA
# polygons in market windows around US cluster centroids — the display layer
# exactly where the markets are.
#   Service: https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer/28
# Migration record: .agent/AEC-ENDPOINT-MIGRATION-2026-06-01.md

echo "" | tee -a "$LOG"
echo "[3/17] FEMA NFHL flood zones (US) — ArcGIS REST, market-windowed" | tee -a "$LOG"

FEMA_MERGED="$WORK_DIR/fema-sfha-merged.geojson"
CLUSTERS_GEOJSON="$SCRIPT_DIR/work/clusters.geojson"

if [[ ! -f "$FEMA_MERGED" && -f "$CLUSTERS_GEOJSON" ]]; then
    python3 - "$CLUSTERS_GEOJSON" "$FEMA_MERGED" <<'PYEOF' 2>&1 | tee -a "$LOG"
import sys, json, time, urllib.request, urllib.parse

clusters_path, out_path = sys.argv[1], sys.argv[2]
SERVICE = "https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer/28/query"
SFHA = "'A','AE','AH','AO','AR','A99','V','VE'"   # Special Flood Hazard Area (1-in-100yr)
HALF = 0.10   # ~11 km half-box around each market centroid
UA = {"User-Agent": "Mozilla/5.0 (compatible; gis-aec-fetch/1.0)"}

d = json.load(open(clusters_path))
cells = set()   # dedup overlapping markets to ~0.2° cells
for f in d["features"]:
    p = f["properties"]
    if p.get("iso") != "US":
        continue
    lat, lon = p.get("seed_lat"), p.get("seed_lon")
    if lat is None or lon is None:
        continue
    cells.add((round(lat * 5) / 5, round(lon * 5) / 5))
print(f"  US market cells to query: {len(cells)}")

feats, seen = [], set()
for i, (lat, lon) in enumerate(sorted(cells)):
    if i % 100 == 0:
        print(f"    … {i}/{len(cells)} cells, {len(feats)} polygons", flush=True)
    env = f"{lon-HALF},{lat-HALF},{lon+HALF},{lat+HALF}"
    params = {
        "where": f"FLD_ZONE IN ({SFHA})",
        "geometry": env, "geometryType": "esriGeometryEnvelope", "inSR": "4326",
        "spatialRel": "esriSpatialRelIntersects",
        "outFields": "FLD_ZONE", "returnGeometry": "true", "outSR": "4326",
        "f": "geojson", "resultRecordCount": "2000",
    }
    url = SERVICE + "?" + urllib.parse.urlencode(params)
    try:
        with urllib.request.urlopen(urllib.request.Request(url, headers=UA), timeout=40) as r:
            gj = json.loads(r.read().decode())
        for ft in gj.get("features", []):
            key = json.dumps(ft.get("geometry"), sort_keys=True)[:240]
            if key in seen:
                continue
            seen.add(key)
            feats.append(ft)
    except Exception:
        pass
    time.sleep(0.15)   # polite to FEMA

json.dump({"type": "FeatureCollection", "features": feats}, open(out_path, "w"))
print(f"  FEMA SFHA polygons collected: {len(feats)} → {out_path}")
PYEOF
fi

if [[ ! -f "$FEMA_MERGED" ]]; then
    echo "  WARN: FEMA flood data not collected (clusters.geojson missing or REST failed) — FEMA pmtiles skipped" | tee -a "$LOG"
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
echo "[10/17] EU flood zones — GB (OGC API Features) + IT (ISPRA GeoPackage)" | tee -a "$LOG"

EU_FLOOD_PARTS=()

# GB: Environment Agency "Flood Map for Planning" — OGC API Features (FZ2/3).
# Old EA ArcGIS REST path retired; OGC API Features is the current source.
GB_GEOJSON="$WORK_DIR/flood-eu-gb.geojson"
if [[ ! -f "$GB_GEOJSON" ]]; then
    echo "  [GB] EA Flood Zones 2/3 (OGC API Features, paginated)..." | tee -a "$LOG"
    python3 - "$GB_GEOJSON" <<'PYEOF' 2>&1 | tee -a "$LOG"
import sys, json, time, urllib.request
out = sys.argv[1]
BASE = ("https://environment.data.gov.uk/spatialdata/"
        "flood-map-for-planning-flood-zones/ogc/features/v1/"
        "collections/Flood_Zones_2_3_Rivers_and_Sea/items")
UA = {"User-Agent": "Mozilla/5.0 (compatible; gis-aec-fetch/1.0)"}
feats = []; offset = 0; LIMIT = 2000; MAX = 300000
while offset < MAX:
    url = f"{BASE}?f=json&limit={LIMIT}&offset={offset}"
    try:
        with urllib.request.urlopen(urllib.request.Request(url, headers=UA), timeout=120) as r:
            gj = json.loads(r.read().decode())
    except Exception as e:
        print(f"  GB page offset={offset} failed: {e}"); break
    fs = gj.get("features", [])
    if not fs:
        break
    feats.extend(fs)
    print(f"    GB offset={offset} +{len(fs)} (total {len(feats)})", flush=True)
    if len(fs) < LIMIT:
        break
    offset += LIMIT
    time.sleep(0.3)
if feats:
    json.dump({"type": "FeatureCollection", "features": feats}, open(out, "w"))
    print(f"  GB flood zones: {len(feats)} → {out}")
PYEOF
fi
[[ -f "$GB_GEOJSON" && $(stat -c%s "$GB_GEOJSON") -gt 10000 ]] && EU_FLOOD_PARTS+=("$GB_GEOJSON") || true

# IT: ISPRA IdroGEO national flood-hazard GeoPackage (aree pericolosità idraulica).
IT_GPKG="$WORK_DIR/it-flood-hazard.gpkg"
IT_GEOJSON="$WORK_DIR/flood-eu-it.geojson"
if [[ ! -f "$IT_GEOJSON" ]]; then
    echo "  [IT] ISPRA IdroGEO flood-hazard GeoPackage (~1 GB download)..." | tee -a "$LOG"
    if [[ ! -f "$IT_GPKG" ]]; then
        curl -L --no-progress-meter --show-error --retry 2 --max-time 2400 \
            -o "$IT_GPKG" \
            "https://sdi.isprambiente.it/download_ogc/alluvioni/aree_pericolosita_idraulica_v5_2020_4258.gpkg" \
            2>&1 | tee -a "$LOG" || true
    fi
    if [[ -f "$IT_GPKG" && $(stat -c%s "$IT_GPKG") -gt 1000000 ]]; then
        ogr2ogr -f GeoJSON -t_srs EPSG:4326 "$IT_GEOJSON" "$IT_GPKG" 2>&1 | tee -a "$LOG" || true
    else
        echo "    WARN: IT GeoPackage download failed" | tee -a "$LOG"
    fi
    rm -f "$IT_GPKG"   # reclaim ~1 GB
fi
[[ -f "$IT_GEOJSON" && $(stat -c%s "$IT_GEOJSON") -gt 10000 ]] && EU_FLOOD_PARTS+=("$IT_GEOJSON") || true

echo "" | tee -a "$LOG"
echo "[11/17] EU flood zones — FR / ES / DE: DEFERRED (fragmented sources)" | tee -a "$LOG"
echo "  FR Géorisques API returns attributes only (no geometry); ES is WMS/bulk-only;" | tee -a "$LOG"
echo "  DE is decentralised per-Land (no federal WFS). Bulk-ingest follow-up documented in" | tee -a "$LOG"
echo "  .agent/AEC-ENDPOINT-MIGRATION-2026-06-01.md. Skipping (no silent truncation)." | tee -a "$LOG"

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
        --attribution="GB Environment Agency (OGC API) / IT ISPRA IdroGEO — FR/ES/DE deferred" \
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

# ── Step 13 — NASA GFWED FWI wildfire raster ─────────────────────────────────
#
# Source: NASA Global Fire WEather Database (GFWED), GEOS-5/GPM.LATE.v5
# Open NCCS datashare (no API key). Replaces the retired GWIS S3 raster (404).
# We download the 12 monthly FWI NetCDFs for the last complete year and average
# them into a mean-FWI GeoTIFF (global ~0.25°). Output keeps the historic
# filename so the classification (Step 14) and sampling (Step 16) are unchanged.
# Portal: https://portal.nccs.nasa.gov/datashare/GlobalFWI/
#
# FWI classification thresholds (Canadian Forest Service — universal FWI scale):
#   0–5.2  → none/very_low
#   5.2–11.2 → low
#   11.2–21.3 → moderate
#   21.3–38.0 → high
#   >38.0     → very_high / extreme

echo "" | tee -a "$LOG"
echo "[13/17] NASA GFWED FWI wildfire raster (global mean, GEOS-5/GPM)" | tee -a "$LOG"

GWIS_TIF="$WORK_DIR/gwis-fwi-global.tif"   # filename kept; downstream Step 14/16 unchanged
rm -f "$GWIS_TIF"                          # drop any stale/error-page file
GFWED_BASE="https://portal.nccs.nasa.gov/datashare/GlobalFWI/v2.0/fwiCalcs.GEOS-5/Default/GPM.LATE.v5"
GFWED_YEAR=2024   # last complete year of GPM.LATE.v5 monthly aggregates
GFWED_TIFS=()
for m in 01 02 03 04 05 06 07 08 09 10 11 12; do
    mtif="$WORK_DIR/gfwed-fwi-${GFWED_YEAR}${m}.tif"
    if [[ ! -f "$mtif" ]]; then
        mnc="$WORK_DIR/gfwed-${GFWED_YEAR}${m}.nc"
        url="$GFWED_BASE/$GFWED_YEAR/FWI.GPM.LATE.v5.Monthly.Default.${GFWED_YEAR}${m}.nc"
        curl -L --no-progress-meter --show-error --retry 3 --retry-delay 10 --max-time 600 \
            -o "$mnc" "$url" 2>&1 | tee -a "$LOG" || true
        if [[ -f "$mnc" ]] && gdalinfo "NETCDF:${mnc}:FWI" &>/dev/null; then
            gdal_translate -q -a_srs EPSG:4326 "NETCDF:${mnc}:FWI" "$mtif" 2>&1 | tee -a "$LOG" || true
        else
            echo "  WARN: GFWED ${GFWED_YEAR}-${m} download/read failed" | tee -a "$LOG"
        fi
        rm -f "$mnc"
    fi
    [[ -f "$mtif" ]] && GFWED_TIFS+=("$mtif")
done

if [[ ${#GFWED_TIFS[@]} -ge 1 ]]; then
    echo "  Averaging ${#GFWED_TIFS[@]} monthly FWI rasters → annual mean..." | tee -a "$LOG"
    python3 - "$GWIS_TIF" "${GFWED_TIFS[@]}" <<'PYEOF' 2>&1 | tee -a "$LOG"
import sys
import numpy as np
import rasterio
out, tifs = sys.argv[1], sys.argv[2:]
acc = None; cnt = 0; prof = None; ref = None
for t in tifs:
    try:
        with rasterio.open(t) as src:
            if prof is None:
                prof = src.profile; ref = (src.height, src.width)
            for b in range(1, src.count + 1):   # robust to 1-band monthly or N-band daily
                a = src.read(b, masked=True).astype('float32')
                if a.shape != ref:
                    continue
                acc = a.filled(0.0) if acc is None else acc + a.filled(0.0)
                cnt += 1
    except Exception as e:
        print(f"  skip {t}: {e}")
if cnt and prof is not None:
    mean = (acc / cnt).astype('float32')
    prof.update(dtype='float32', count=1, compress='deflate', nodata=0)
    with rasterio.open(out, 'w', **prof) as dst:
        dst.write(mean, 1)
    print(f"  mean FWI written: {out} (averaged {cnt} bands)")
else:
    print("  ERROR: no readable FWI bands — mean not written")
PYEOF
fi

# clean monthly intermediates to reclaim space
rm -f "$WORK_DIR"/gfwed-fwi-*.tif

if [[ ! -f "$GWIS_TIF" || $(stat -c%s "$GWIS_TIF") -lt 100000 ]]; then
    echo "  WARN: GFWED mean FWI raster not produced — wildfire layer will be skipped" | tee -a "$LOG"
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
    # Classify FWI + polygonize using pure Python GDAL API (avoids numpy 2.x ABI crash
    # that breaks gdal_calc.py and gdal_polygonize.py compiled against numpy 1.x).
    python3 - <<PYEOF 2>&1 | tee -a "$LOG"
import json, struct, pathlib, sys
from osgeo import gdal, ogr, osr

gdal.UseExceptions()
LABELS = {1:'low', 2:'moderate', 3:'high', 4:'very_high', 5:'extreme'}

src_path     = "$GWIS_TIF"
cls_path     = "$GWIS_CLASSIFIED"
geojson_path = "$GWIS_GEOJSON"

# ── Classify FWI float raster → byte raster ──────────────────────────────────
src_ds = gdal.Open(src_path, gdal.GA_ReadOnly)
if src_ds is None:
    print(f"ERROR: cannot open {src_path}", file=sys.stderr)
    sys.exit(1)

band   = src_ds.GetRasterBand(1)
nd     = band.GetNoDataValue()
cols   = src_ds.RasterXSize
rows   = src_ds.RasterYSize
BLOCK  = 512

drv    = gdal.GetDriverByName("GTiff")
cls_ds = drv.Create(cls_path, cols, rows, 1, gdal.GDT_Byte,
                    ["COMPRESS=DEFLATE", "TILED=YES"])
cls_ds.SetGeoTransform(src_ds.GetGeoTransform())
cls_ds.SetProjection(src_ds.GetProjection())
cls_band = cls_ds.GetRasterBand(1)
cls_band.SetNoDataValue(0)

for y in range(0, rows, BLOCK):
    ysize = min(BLOCK, rows - y)
    for x in range(0, cols, BLOCK):
        xsize  = min(BLOCK, cols - x)
        data   = band.ReadRaster(x, y, xsize, ysize, buf_type=gdal.GDT_Float32)
        floats = struct.unpack(f"{xsize*ysize}f", data)
        out    = bytearray(xsize * ysize)
        for i, v in enumerate(floats):
            if nd is not None and abs(v - nd) < 1e-6:
                out[i] = 0
            elif v <= 0:
                out[i] = 0
            elif v <= 5.2:
                out[i] = 1
            elif v <= 11.2:
                out[i] = 2
            elif v <= 21.3:
                out[i] = 3
            elif v <= 38.0:
                out[i] = 4
            else:
                out[i] = 5
        cls_band.WriteRaster(x, y, xsize, ysize, bytes(out), buf_type=gdal.GDT_Byte)

cls_band.FlushCache()
cls_ds = None
src_ds = None
print("  Classified FWI → byte raster  ✓")

# ── Polygonize ────────────────────────────────────────────────────────────────
cls_ds   = gdal.Open(cls_path, gdal.GA_ReadOnly)
cls_band = cls_ds.GetRasterBand(1)

mem_drv  = ogr.GetDriverByName("Memory")
mem_ds   = mem_drv.CreateDataSource("out")
srs      = osr.SpatialReference()
srs.ImportFromEPSG(4326)
mem_lyr  = mem_ds.CreateLayer("wildfire_zones", srs=srs)
fd       = ogr.FieldDefn("fwi_class", ogr.OFTInteger)
mem_lyr.CreateField(fd)

gdal.Polygonize(cls_band, cls_band, mem_lyr, 0, [], callback=None)
cls_ds = None

# Add wildfire_risk label and write GeoJSON
feats = []
for feat in mem_lyr:
    cls = feat.GetField("fwi_class")
    if not cls:
        continue
    geom = feat.GetGeometryRef()
    if geom is None:
        continue
    feats.append({
        "type": "Feature",
        "properties": {"fwi_class": cls, "wildfire_risk": LABELS.get(cls, "unknown")},
        "geometry": json.loads(geom.ExportToJson()),
    })
mem_ds = None

gj = {"type": "FeatureCollection", "features": feats}
pathlib.Path(geojson_path).write_text(json.dumps(gj))
print(f"  Polygonized {len(feats)} features → {geojson_path}  ✓")
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

python3 - <<PYEOF 2>&1 | tee -a "$LOG"
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
python3 - <<PYEOF 2>&1 | tee -a "$LOG"
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
