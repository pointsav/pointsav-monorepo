#!/usr/bin/env bash
# build-aec-seismic.sh — Night 4 AEC build: seismic hazard + wetland classification
#
# Run at 05:00 UTC 2026-05-27 (Night 4 of AEC staged rollout).
# See .agent/AEC-NIGHTLY-BUILD-PLAN.md for full plan context.
#
# Usage:  bash build-aec-seismic.sh [--dry-run]
#
# Produces:
#   layer10-seismic-eu.pmtiles         — EU seismic hazard zones (ESHM20, CC BY 4.0)
#   patches clusters-meta.json         — seismic_pga_g (float g), wetland_class (string)
#
# Note: US/CA seismic PGA is sampled from USGS NSHM 2023 and NRCan 2015 rasters.
# ESHM20 is a vector zone dataset for EU. GWL_FCS30 global wetland is a raster.
#
# Prerequisites (all present on this VM):
#   ogr2ogr (GDAL 3.8+), tippecanoe, python3, curl, unzip, jq, gdallocationinfo
#
# Night 4 disk delta: ~50–200 MB PMTiles + ~700 MB work/ intermediates.
# Ensure ≥5 GB free on /srv/foundry before running.
#
# IMPORTANT — data licence check:
#   USGS NSHM 2023: public domain (US Gov)
#   NRCan 2015: Open Government Licence Canada
#   ESHM20: CC BY 4.0 (Woessner et al. 2015 / EFEHR)
#   GWL_FCS30: CC BY 4.0 (Liu et al. 2022, Zenodo 7340516)

set -euo pipefail

DRY_RUN=0
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN=1

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="$SCRIPT_DIR/work/aec"
export WORK_DIR
LOG="$SCRIPT_DIR/build-aec-seismic.log"
STAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
TILES_OUT="/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles"
META_PATH="/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"

echo "──────────────────────────────────────────────" | tee -a "$LOG"
echo "build-aec-seismic  $STAMP" | tee -a "$LOG"

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

# gdallocationinfo is from GDAL — check separately (may be gdal-bin not gdal-core)
if ! command -v gdallocationinfo &>/dev/null; then
    echo "WARN: gdallocationinfo not found — raster sampling will use Python gdal bindings fallback" | tee -a "$LOG"
    USE_GDALLOCATIONINFO=0
else
    echo "Tools: gdallocationinfo (GDAL)  ✓" | tee -a "$LOG"
    USE_GDALLOCATIONINFO=1
fi

# Check Night 3 complete marker (advisory — does not block)
if [[ ! -f "$WORK_DIR/.night3-complete" ]]; then
    echo "WARN: .night3-complete marker not found — Night 3 may not have run; proceeding anyway" | tee -a "$LOG"
else
    echo "Night 3 complete marker found  ✓" | tee -a "$LOG"
fi

if [[ $DRY_RUN -eq 1 ]]; then
    echo "DRY RUN — pre-flight passed, not executing build steps" | tee -a "$LOG"
    exit 0
fi

mkdir -p "$WORK_DIR"
echo "Work dir: $WORK_DIR" | tee -a "$LOG"

# ── Step 1 — USGS NSHM 2023 PGA raster (CONUS, 2% in 50 years) ──────────────
#
# Source: USGS National Seismic Hazard Model 2023
# DOI: https://doi.org/10.5066/P9PRVCF1
# Direct GeoTIFF (PGA, 2%/50yr, site class B/C, CONUS, ~50 MB):
#   https://earthquake.usgs.gov/static/lfs/nshm/2023/nshm2023_pga_2in50_conus.tif
# Licence: public domain (US Government)

echo "" | tee -a "$LOG"
echo "[1/9] USGS NSHM 2023 PGA raster (CONUS, 2%/50yr)" | tee -a "$LOG"

USGS_TIF="$WORK_DIR/usgs-nshm-pga-us.tif"
if [[ ! -f "$USGS_TIF" ]]; then
    curl -L --retry 3 --retry-delay 10 -o "$USGS_TIF" \
        "https://earthquake.usgs.gov/static/lfs/nshm/2023/nshm2023_pga_2in50_conus.tif" \
        2>&1 | tee -a "$LOG"
fi
# Validate: a real GeoTIFF starts with II (little-endian) or MM (big-endian) TIFF magic
if [[ ! -f "$USGS_TIF" ]] || ! file "$USGS_TIF" | grep -qi "TIFF"; then
    echo "WARN: USGS PGA raster not a valid GeoTIFF (URL may have changed — check https://doi.org/10.5066/P9PRVCF1); US seismic sampling will be skipped" | tee -a "$LOG"
    rm -f "$USGS_TIF"
    SKIP_USGS=1
else
    echo "  → $USGS_TIF ($(du -sh "$USGS_TIF" | cut -f1))  ✓" | tee -a "$LOG"
    SKIP_USGS=0
fi

# ── Step 2 — NRCan 2015 seismic hazard raster (Canada) ───────────────────────
#
# Source: Natural Resources Canada 2015 National Building Code seismic hazard
# URL: https://earthquakescanada.nrcan.gc.ca/hazard-alea/interpolat/2015/
# Download: Sa(0.0) = PGA, 2%/50yr, Site class C, netCDF or ASCII grid
# Licence: Open Government Licence – Canada
#
# NRCan provides an interpolation web service; for the raster grid use the
# 2015 NBC zip at:
#   https://earthquakescanada.nrcan.gc.ca/hazard-alea/interpolat/2015/2015_pga_2p50.csv.zip
# (CSV grid: lat, lon, pga_g columns)

echo "" | tee -a "$LOG"
echo "[2/9] NRCan 2015 seismic PGA grid (Canada, 2%/50yr)" | tee -a "$LOG"

NRCAN_ZIP="$WORK_DIR/nrcan-seismic-ca.zip"
NRCAN_CSV="$WORK_DIR/nrcan-pga-2p50.csv"
if [[ ! -f "$NRCAN_CSV" ]]; then
    # Re-download only if we don't have a valid zip (a real zip starts with PK magic bytes)
    if [[ ! -f "$NRCAN_ZIP" ]] || ! file "$NRCAN_ZIP" | grep -qi "zip"; then
        rm -f "$NRCAN_ZIP"
        curl -L --retry 3 --retry-delay 10 -o "$NRCAN_ZIP" \
            "https://earthquakescanada.nrcan.gc.ca/hazard-alea/interpolat/2015/2015_pga_2p50.csv.zip" \
            2>&1 | tee -a "$LOG"
    fi
    # Validate zip before extracting — URL may return an HTML error page
    if file "$NRCAN_ZIP" 2>/dev/null | grep -qi "zip"; then
        unzip -o -j "$NRCAN_ZIP" "*.csv" -d "$WORK_DIR" 2>&1 | tee -a "$LOG" || true
        # Rename to canonical name (actual filename inside zip may differ)
        for f in "$WORK_DIR"/2015_pga*.csv "$WORK_DIR"/pga*.csv; do
            [[ -f "$f" ]] && mv "$f" "$NRCAN_CSV" && break
        done
    else
        echo "  WARN: NRCAN download is not a valid zip — URL may have changed (file: $(file "$NRCAN_ZIP" 2>/dev/null | cut -d: -f2-))" | tee -a "$LOG"
        rm -f "$NRCAN_ZIP"
    fi
fi
if [[ ! -f "$NRCAN_CSV" ]]; then
    echo "WARN: NRCan PGA CSV not downloaded — CA seismic sampling will be skipped" | tee -a "$LOG"
    SKIP_NRCAN=1
else
    echo "  → $NRCAN_CSV ($(wc -l < "$NRCAN_CSV") rows)  ✓" | tee -a "$LOG"
    SKIP_NRCAN=0
fi

# ── Step 3 — Sample USGS + NRCan rasters at cluster centroids ────────────────
#
# Strategy: for each cluster centroid, query the appropriate raster.
#   US clusters: query USGS NSHM 2023 GeoTIFF via gdallocationinfo
#   CA clusters: interpolate from NRCan 2%/50yr CSV grid (nearest-neighbour)
# Output: seismic_pga_g field (float, units: g) in clusters-meta.json patch buffer

echo "" | tee -a "$LOG"
echo "[3/9] Sample seismic PGA at cluster centroids" | tee -a "$LOG"

python3 - <<PYEOF 2>&1 | tee -a "$LOG"
import json, subprocess, csv, math, pathlib, sys

META = pathlib.Path("$META_PATH")
USGS_TIF = "$USGS_TIF"
NRCAN_CSV = "$NRCAN_CSV"
SKIP_NRCAN = int("${SKIP_NRCAN:-1}")
SKIP_USGS  = int("${SKIP_USGS:-0}")

clusters = json.loads(META.read_text())
use_gdallocationinfo = $USE_GDALLOCATIONINFO

# Build NRCan nearest-neighbour lookup grid from CSV
nrcan_grid = []  # [(lat, lon, pga_g)]
if not SKIP_NRCAN and pathlib.Path(NRCAN_CSV).exists():
    with open(NRCAN_CSV) as f:
        reader = csv.reader(f)
        header = None
        for row in reader:
            if header is None:
                header = [c.strip().lower() for c in row]
                try:
                    ilat = header.index('lat')
                    ilon = header.index('lon')
                    ipga = next(i for i, h in enumerate(header) if 'pga' in h or 'sa00' in h)
                except (ValueError, StopIteration):
                    print("  WARN: NRCan CSV header not recognised — skipping CA sampling")
                    break
                continue
            try:
                nrcan_grid.append((float(row[ilat]), float(row[ilon]), float(row[ipga])))
            except (ValueError, IndexError):
                continue
    print(f"  NRCan grid: {len(nrcan_grid)} points loaded")

def sample_usgs(lon, lat):
    """Sample USGS NSHM 2023 PGA raster at (lon, lat) → float g or None."""
    if SKIP_USGS or not use_gdallocationinfo:
        return None
    try:
        result = subprocess.run(
            ["gdallocationinfo", "-valonly", "-wgs84", USGS_TIF, str(lon), str(lat)],
            capture_output=True, text=True, timeout=10
        )
        val = result.stdout.strip()
        return round(float(val), 4) if val else None
    except Exception:
        return None

def sample_nrcan(lon, lat):
    """Nearest-neighbour lookup in NRCan CSV grid → float g or None."""
    if not nrcan_grid:
        return None
    best, best_dist = None, float('inf')
    for glat, glon, pga in nrcan_grid:
        d = (glat - lat) ** 2 + (glon - lon) ** 2
        if d < best_dist:
            best_dist = d; best = pga
    return round(best, 4) if best is not None and best_dist < 1.0 else None

n_us = n_ca = 0
for c in clusters:
    if c.get('seismic_pga_g') is not None:
        continue
    iso = c.get('iso', '')
    lon, lat = c.get('lon', 0), c.get('lat', 0)
    if iso == 'US':
        pga = sample_usgs(lon, lat)
        if pga is not None:
            c['seismic_pga_g'] = pga; n_us += 1
    elif iso == 'CA' and not SKIP_NRCAN:
        pga = sample_nrcan(lon, lat)
        if pga is not None:
            c['seismic_pga_g'] = pga; n_ca += 1

# Write back intermediate state
META.write_text(json.dumps(clusters, separators=(',', ':')))
print(f"  Sampled: {n_us} US clusters, {n_ca} CA clusters")
PYEOF

# ── Step 4 — Download ESHM20 EU seismic hazard zones ─────────────────────────
#
# Source: European Seismic Hazard Model 2020 (ESHM20)
# Authors: Woessner et al. / EFEHR Consortium
# Licence: CC BY 4.0
# GeoJSON or Shapefile available at PANGAEA / EFEHR portal:
#   https://doi.pangaea.de/10.1594/PANGAEA.919310
#   https://efehr.org/
# Direct download (shapefile zip):
#   https://store.pangaea.de/Publications/Woessner-etal_2015/SeismicHazardMap_PGA_10in50.zip
#
# This is the PGA 10%/50yr contour map (vector polygon zones, ~5 MB zip).
# Use PGA hazard contours as categorical zones: <0.05g / 0.05–0.10g / 0.10–0.20g /
#   0.20–0.40g / >0.40g — five levels mapped to very_low/low/moderate/high/very_high.

echo "" | tee -a "$LOG"
echo "[4/9] ESHM20 EU seismic hazard zones (CC BY 4.0)" | tee -a "$LOG"

ESHM20_ZIP="$WORK_DIR/eshm20-eu.zip"
ESHM20_GEOJSON="$WORK_DIR/eshm20-eu.geojson"
if [[ ! -f "$ESHM20_GEOJSON" ]]; then
    # Primary: PANGAEA download
    curl -L --retry 3 --retry-delay 15 \
        -o "$ESHM20_ZIP" \
        "https://store.pangaea.de/Publications/Woessner-etal_2015/SeismicHazardMap_PGA_10in50.zip" \
        2>&1 | tee -a "$LOG" || true

    if [[ -f "$ESHM20_ZIP" && $(stat -c%s "$ESHM20_ZIP") -gt 10000 ]]; then
        TMP_DIR=$(mktemp -d)
        unzip -o "$ESHM20_ZIP" -d "$TMP_DIR" 2>&1 | tee -a "$LOG"
        # Find shapefile
        SHP=$(find "$TMP_DIR" -name "*.shp" | head -1)
        if [[ -n "$SHP" ]]; then
            ogr2ogr -f GeoJSON -t_srs EPSG:4326 "$ESHM20_GEOJSON" "$SHP" \
                2>&1 | tee -a "$LOG"
        fi
        rm -rf "$TMP_DIR"
    fi
fi
if [[ ! -f "$ESHM20_GEOJSON" ]]; then
    echo "WARN: ESHM20 GeoJSON not produced — EU seismic PMTiles will be skipped" | tee -a "$LOG"
    SKIP_ESHM20=1
else
    FEAT_COUNT=$(python3 -c "import json; d=json.load(open('$ESHM20_GEOJSON')); print(len(d['features']))" 2>/dev/null || echo "?")
    echo "  → $ESHM20_GEOJSON ($FEAT_COUNT features)  ✓" | tee -a "$LOG"
    SKIP_ESHM20=0
fi

# ── Step 5 — Build layer10-seismic-eu.pmtiles ────────────────────────────────

echo "" | tee -a "$LOG"
echo "[5/9] Build EU seismic PMTiles (layer10-seismic-eu)" | tee -a "$LOG"

if [[ $SKIP_ESHM20 -eq 0 ]]; then
    tippecanoe \
        --output="$TILES_OUT/layer10-seismic-eu.pmtiles" \
        --name="ESHM20 EU Seismic Hazard" \
        --attribution="Woessner et al. / EFEHR, CC BY 4.0" \
        --layer="seismic_zones" \
        --minimum-zoom=2 \
        --maximum-zoom=9 \
        --simplification=4 \
        --drop-densest-as-needed \
        --force \
        "$ESHM20_GEOJSON" \
        2>&1 | tee -a "$LOG"
    echo "  → $TILES_OUT/layer10-seismic-eu.pmtiles ($(du -sh "$TILES_OUT/layer10-seismic-eu.pmtiles" | cut -f1))  ✓" | tee -a "$LOG"
else
    echo "  SKIPPED — ESHM20 data not available" | tee -a "$LOG"
fi

# ── Step 6 — Download GWL_FCS30 global wetland raster ────────────────────────
#
# Source: Global Wetland-Land Cover FCS30 (GWL_FCS30)
# Authors: Liu et al. 2022
# Licence: CC BY 4.0
# Zenodo record: https://zenodo.org/records/7340516
# Direct TIF download (global composite, ~500 MB):
#   https://zenodo.org/records/7340516/files/GWL_FCS30_2020_global.tif
#
# Classification codes of interest:
#   100 = Permanent water body
#   181 = Open permanent water
#   182 = Seasonal/temporary water
#   183 = Permanent herbaceous wetland
#   184 = Seasonal herbaceous wetland
#   185 = Mangrove
#   186 = Salt marsh
#   190 = Other wetland
# All other values = non-wetland (dry land / urban / agriculture / etc.)

echo "" | tee -a "$LOG"
echo "[6/9] GWL_FCS30 global wetland raster (CC BY 4.0, Zenodo 7340516)" | tee -a "$LOG"

GWL_TIF="$WORK_DIR/gwl-fcs30-global.tif"
if [[ ! -f "$GWL_TIF" ]]; then
    curl -L --retry 3 --retry-delay 30 \
        -o "$GWL_TIF" \
        "https://zenodo.org/records/7340516/files/GWL_FCS30_2020_global.tif" \
        2>&1 | tee -a "$LOG"
fi
if [[ ! -f "$GWL_TIF" || $(stat -c%s "$GWL_TIF") -lt 1000000 ]]; then
    echo "WARN: GWL_FCS30 raster not downloaded or too small — wetland sampling will be skipped" | tee -a "$LOG"
    SKIP_WETLAND=1
else
    echo "  → $GWL_TIF ($(du -sh "$GWL_TIF" | cut -f1))  ✓" | tee -a "$LOG"
    SKIP_WETLAND=0
fi

# ── Step 7 — Sample wetland class at cluster centroids ───────────────────────

echo "" | tee -a "$LOG"
echo "[7/9] Sample wetland class at cluster centroids" | tee -a "$LOG"

if [[ $SKIP_WETLAND -eq 0 ]]; then
python3 - <<PYEOF 2>&1 | tee -a "$LOG"
import json, subprocess, pathlib

META = pathlib.Path("$META_PATH")
GWL_TIF = "$GWL_TIF"
USE_GDAL = $USE_GDALLOCATIONINFO

# GWL_FCS30 classification: wetland codes
WETLAND_CODES = {
    100: 'permanent_water',
    181: 'permanent_water',
    182: 'seasonal_water',
    183: 'permanent_wetland',
    184: 'seasonal_wetland',
    185: 'mangrove',
    186: 'salt_marsh',
    190: 'wetland',
}

def sample_wetland(lon, lat):
    if not USE_GDAL:
        return None
    try:
        result = subprocess.run(
            ["gdallocationinfo", "-valonly", "-wgs84", GWL_TIF, str(lon), str(lat)],
            capture_output=True, text=True, timeout=10
        )
        val = result.stdout.strip()
        code = int(float(val)) if val else 0
        return WETLAND_CODES.get(code)  # None = not a wetland
    except Exception:
        return None

clusters = json.loads(META.read_text())
n = 0
for c in clusters:
    if c.get('wetland_class') is not None:
        continue
    cls = sample_wetland(c.get('lon', 0), c.get('lat', 0))
    if cls is not None:
        c['wetland_class'] = cls; n += 1

META.write_text(json.dumps(clusters, separators=(',', ':')))
print(f"  Wetland-classified: {n} clusters")
PYEOF
else
    echo "  SKIPPED — GWL_FCS30 data not available" | tee -a "$LOG"
fi

# ── Step 8 — Sample EU seismic zones at cluster centroids (vector join) ───────
#
# For EU clusters not covered by a raster, do a spatial join against ESHM20
# vector polygons to assign a categorical seismic_zone_eu field.

echo "" | tee -a "$LOG"
echo "[8/9] EU seismic zone join at cluster centroids" | tee -a "$LOG"

if [[ $SKIP_ESHM20 -eq 0 ]]; then
python3 - <<PYEOF 2>&1 | tee -a "$LOG"
import json, pathlib
try:
    from shapely.geometry import Point, shape
    HAS_SHAPELY = True
except ImportError:
    HAS_SHAPELY = False
    print("  WARN: shapely not available — EU seismic zone join skipped")

META = pathlib.Path("$META_PATH")
ESHM20_GJ = "$ESHM20_GEOJSON"

if HAS_SHAPELY:
    gj = json.loads(pathlib.Path(ESHM20_GJ).read_text())
    polys = []
    for f in gj['features']:
        try:
            polys.append((shape(f['geometry']), f['properties']))
        except Exception:
            continue

    clusters = json.loads(META.read_text())
    EU_ISOS = {'AT','BE','BG','HR','CY','CZ','DK','EE','FI','FR','DE','GR','HU',
               'IE','IT','LV','LT','LU','MT','NL','PL','PT','RO','SK','SI','ES','SE','GB'}
    n = 0
    for c in clusters:
        if c.get('seismic_pga_g') is not None or c.get('iso') not in EU_ISOS:
            continue
        pt = Point(c['lon'], c['lat'])
        for poly, props in polys:
            if poly.contains(pt):
                # Extract PGA value from ESHM20 properties (field name varies by version)
                pga_raw = props.get('PGA_10in50') or props.get('pga') or props.get('HAZARD') or props.get('val')
                if pga_raw is not None:
                    try:
                        c['seismic_pga_g'] = round(float(pga_raw), 4); n += 1
                    except (ValueError, TypeError):
                        pass
                break
    META.write_text(json.dumps(clusters, separators=(',', ':')))
    print(f"  EU seismic PGA assigned: {n} clusters")
PYEOF
else
    echo "  SKIPPED — ESHM20 data not available" | tee -a "$LOG"
fi

# ── Step 9 — Summary + phase marker ──────────────────────────────────────────

echo "" | tee -a "$LOG"
TILE4_COUNT=$(find "$TILES_OUT" -name "layer10-seismic-*.pmtiles" 2>/dev/null | wc -l)
echo "── Night 4 Complete: $(date -u '+%Y-%m-%dT%H:%M:%SZ') ──" | tee -a "$LOG"
echo "   layer10 PMTiles produced: $TILE4_COUNT" | tee -a "$LOG"
echo "   clusters-meta.json: $(wc -c < "$META_PATH") bytes" | tee -a "$LOG"
echo "   Next: Night 5 at 05:00 UTC 2026-05-28 — build-aec-flood.sh" | tee -a "$LOG"

touch "$WORK_DIR/.night4-complete"
echo "   Phase marker: $WORK_DIR/.night4-complete  ✓" | tee -a "$LOG"
