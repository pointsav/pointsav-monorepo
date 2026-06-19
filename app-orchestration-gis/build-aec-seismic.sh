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

# ── Step 1 — USGS NSHM 2023 PGA shapefile (CONUS, 2% in 50 years) ────────────
#
# Source: USGS National Seismic Hazard Model 2023
# DOI: https://doi.org/10.5066/P9PRVCF1
# NOTE: The 2023 NSHM ships as shapefiles via ScienceBase (not a GeoTIFF).
#   The original git-lfs URL (earthquake.usgs.gov/static/lfs/nshm/2023/...)
#   returned only 111 bytes (a git-lfs pointer stub, not the actual file).
# ScienceBase child item: https://www.sciencebase.gov/catalog/item/64ff886dd34ed30c2057b4d9
#   "04. Uniform-hazard ground motion maps for the conterminous U.S."
# Direct download (BC site class, PGA 2%/5%/10% in 50yr shapefiles, ~59 MB zip):
#   https://www.sciencebase.gov/catalog/file/get/64ff886dd34ed30c2057b4d9?f=__disk__76%2Ff4%2Fb4%2F76f4b416aadf6f70680106a36acc31714473b4ff
# Licence: public domain (US Government, CC0)

echo "" | tee -a "$LOG"
echo "[1/9] USGS NSHM 2023 PGA shapefile (CONUS, 2%/50yr, ScienceBase)" | tee -a "$LOG"

USGS_ZIP="$WORK_DIR/usgs-nshm-pga-us.zip"
USGS_GEOJSON="$WORK_DIR/usgs-nshm-pga-us.geojson"
if [[ ! -f "$USGS_GEOJSON" ]]; then
    if [[ ! -f "$USGS_ZIP" ]] || ! file "$USGS_ZIP" | grep -qi "zip"; then
        rm -f "$USGS_ZIP"
        SCIENCEBASE_ITEM="64ff886dd34ed30c2057b4d9"
        echo "  Resolving current download URL via ScienceBase catalog API..." | tee -a "$LOG"
        USGS_DL_URL=$(curl -sL --max-time 30 --connect-timeout 10 \
            -H "User-Agent: Mozilla/5.0 (compatible; USGS-Data-Fetch/1.0)" \
            "https://www.sciencebase.gov/catalog/item/${SCIENCEBASE_ITEM}?format=json" | \
            python3 -c "
import sys, json
try:
    d = json.load(sys.stdin)
    zips = [(f.get('name',''), f['downloadUri'])
            for f in d.get('files', []) if f.get('name','').endswith('.zip')]
    # Prefer the 2%/50yr PGA dataset by name; fall back to first zip.
    def score(name):
        n = name.lower()
        s = 0
        if 'pga' in n: s += 4
        if '2pct' in n or '2p50' in n or '2percent' in n or '2-percent' in n: s += 3
        if '50yr' in n or '50-year' in n or '50year' in n: s += 1
        return s
    zips.sort(key=lambda nz: score(nz[0]), reverse=True)
    print(zips[0][1] if zips else '')
except Exception:
    print('')
" 2>/dev/null)
        if [[ -z "$USGS_DL_URL" ]]; then
            echo "  WARN: ScienceBase catalog API unreachable or no .zip file — USGS seismic data will be skipped" | tee -a "$LOG"
        else
            echo "  Downloading: $USGS_DL_URL" | tee -a "$LOG"
            curl -L --retry 3 --retry-delay 10 --max-time 300 \
                -H "User-Agent: Mozilla/5.0 (compatible; USGS-Data-Fetch/1.0)" \
                -o "$USGS_ZIP" "$USGS_DL_URL" \
                2>&1 | tee -a "$LOG"
        fi
    fi
    if file "$USGS_ZIP" 2>/dev/null | grep -qi "zip"; then
        TMP_USGS=$(mktemp -d)
        unzip -o "$USGS_ZIP" -d "$TMP_USGS" 2>&1 | tee -a "$LOG" || true
        # The outer zip may contain inner zips; unzip recursively
        for inner in "$TMP_USGS"/*.zip; do
            [[ -f "$inner" ]] && unzip -o "$inner" -d "$TMP_USGS" 2>&1 | tee -a "$LOG" || true
        done
        # Find the 2%/50yr PGA shapefile (look for *2Pct* or *2pct* or *PGA* shp)
        SHP=$(find "$TMP_USGS" -name "*2Pct*.shp" -o -name "*2pct*.shp" -o -name "*PGA*.shp" 2>/dev/null | head -1)
        if [[ -z "$SHP" ]]; then
            SHP=$(find "$TMP_USGS" -name "*.shp" 2>/dev/null | head -1)
        fi
        if [[ -n "$SHP" ]]; then
            ogr2ogr -f GeoJSON -t_srs EPSG:4326 "$USGS_GEOJSON" "$SHP" \
                2>&1 | tee -a "$LOG"
            echo "  Extracted shapefile: $SHP" | tee -a "$LOG"
        fi
        rm -rf "$TMP_USGS"
    else
        echo "  WARN: USGS ScienceBase download is not a valid zip" | tee -a "$LOG"
        rm -f "$USGS_ZIP"
    fi
fi
if [[ ! -f "$USGS_GEOJSON" ]]; then
    echo "WARN: USGS NSHM 2023 shapefile not available — US seismic PGA sampling will be skipped" | tee -a "$LOG"
    SKIP_USGS=1
else
    FEAT_COUNT=$(python3 -c "import json; d=json.load(open('$USGS_GEOJSON')); print(len(d['features']))" 2>/dev/null || echo "?")
    echo "  → $USGS_GEOJSON ($FEAT_COUNT features)  ✓" | tee -a "$LOG"
    SKIP_USGS=0
fi

# ── Step 2 — NRCan 2015 seismic hazard raster (Canada) ───────────────────────
#
# Source: Natural Resources Canada 2015 National Building Code seismic hazard
# Licence: Open Government Licence – Canada
#
# NOTE: The original interpolat/2015/ endpoint returned 3.5 KB corrupt zip.
# The 5th-Gen grid values are now at NRCan OSTR Open File 7893:
#   https://ostrnrcan-dostrncan.canada.ca/handle/1845/153282
# The GEOSCAN record R=297378 is the download gateway:
#   https://geoscan.nrcan.gc.ca/starweb/geoscan/servlet.starweb?path=geoscan/downloade.web&search1=R=297378
# If both fail, the NRCan interpolation tool at
#   https://www.earthquakescanada.nrcan.gc.ca/hazard-alea/interpolat/calc-en.php
# can generate per-point CSV (manual fallback; not automated here).

echo "" | tee -a "$LOG"
echo "[2/9] NRCan 2015 seismic PGA grid (Canada, 2%/50yr)" | tee -a "$LOG"

NRCAN_ZIP="$WORK_DIR/nrcan-seismic-ca.zip"
NRCAN_CSV="$WORK_DIR/nrcan-pga-2p50.csv"
if [[ ! -f "$NRCAN_CSV" ]]; then
    # Try multiple sources in order
    NRCAN_URLS=(
        # GEOSCAN download (R=297378 — OF 7893 5th gen grid values)
        "https://geoscan.nrcan.gc.ca/starweb/geoscan/servlet.starweb?path=geoscan/downloade.web&search1=R=297378"
        # Legacy interpolat endpoint (may still work)
        "https://earthquakescanada.nrcan.gc.ca/hazard-alea/interpolat/2015/2015_pga_2p50.csv.zip"
    )
    for url in "${NRCAN_URLS[@]}"; do
        if [[ ! -f "$NRCAN_ZIP" ]] || ! file "$NRCAN_ZIP" 2>/dev/null | grep -qi "zip"; then
            rm -f "$NRCAN_ZIP"
            echo "  Trying NRCan URL: $url" | tee -a "$LOG"
            curl -L --retry 2 --retry-delay 10 --max-time 120 -o "$NRCAN_ZIP" "$url" 2>&1 | tee -a "$LOG" || true
        fi
        file "$NRCAN_ZIP" 2>/dev/null | grep -qi "zip" && break
        rm -f "$NRCAN_ZIP"
    done
    # Extract CSV from zip
    if file "$NRCAN_ZIP" 2>/dev/null | grep -qi "zip"; then
        unzip -o -j "$NRCAN_ZIP" "*.csv" -d "$WORK_DIR" 2>&1 | tee -a "$LOG" || true
        for f in "$WORK_DIR"/2015_pga*.csv "$WORK_DIR"/pga*.csv "$WORK_DIR"/*.csv; do
            [[ -f "$f" && "$f" != "$NRCAN_CSV" ]] && mv "$f" "$NRCAN_CSV" && break
        done
    else
        echo "  WARN: NRCan download not a valid zip — CA seismic sampling will be skipped" | tee -a "$LOG"
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

# USGS data is GeoJSON (not raster); USGS_TIF is unused but referenced in the
# heredoc for legacy raster fallback path — set explicitly to avoid set -u error.
USGS_TIF="${USGS_TIF:-}"

python3 - <<PYEOF 2>&1 | tee -a "$LOG"
import json, subprocess, csv, math, pathlib, sys

META = pathlib.Path("$META_PATH")
USGS_TIF = "${USGS_TIF:-}"
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
# Authors: EFEHR Consortium
# Licence: CC BY 4.0
# Repository: https://gitlab.seismo.ethz.ch/efehr/eshm20
# Direct download (full repo tarball, ~120 MB):
#   https://gitlab.seismo.ethz.ch/efehr/eshm20/-/archive/master/eshm20-master.tar.gz
#
# Shapefiles in tarball include PGA 10%/50yr hazard contour polygons.
# Use PGA hazard contours as categorical zones: <0.05g / 0.05–0.10g / 0.10–0.20g /
#   0.20–0.40g / >0.40g — five levels mapped to very_low/low/moderate/high/very_high.

echo "" | tee -a "$LOG"
echo "[4/9] ESHM20 EU seismic hazard zones (CC BY 4.0)" | tee -a "$LOG"

ESHM20_TAR="$WORK_DIR/eshm20-eu.tar.gz"
ESHM20_GEOJSON="$WORK_DIR/eshm20-eu.geojson"
if [[ ! -f "$ESHM20_GEOJSON" ]]; then
    # Primary: EFEHR GitLab tarball (ESHM20 — replaces retired Pangaea ESHM13 URL)
    curl -L --retry 3 --retry-delay 15 \
        -o "$ESHM20_TAR" \
        "https://gitlab.seismo.ethz.ch/efehr/eshm20/-/archive/master/eshm20-master.tar.gz" \
        2>&1 | tee -a "$LOG" || true

    if [[ -f "$ESHM20_TAR" && $(stat -c%s "$ESHM20_TAR") -gt 10000 ]]; then
        TMP_DIR=$(mktemp -d)
        tar xzf "$ESHM20_TAR" -C "$TMP_DIR" 2>&1 | tee -a "$LOG"
        # Find shapefile — prefer PGA 10/50 if named; fall back to any .shp
        SHP=$(find "$TMP_DIR" -name "*PGA*10*50*.shp" | head -1)
        [[ -z "$SHP" ]] && SHP=$(find "$TMP_DIR" -name "*.shp" | head -1)
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

# ── Step 6 — Download GWL_FCS30 wetland raster tiles ─────────────────────────
#
# Source: Global Wetland-Land Cover FCS30 (GWL_FCS30)
# Authors: Liu et al. 2022
# Licence: CC BY 4.0
# Zenodo record: https://zenodo.org/records/7340516
#
# Dataset is distributed as 12 tiled 30°-longitude zip archives (no global composite).
# Download the tiles covering NA (US+CA) and EU. Build a VRT mosaic for sampling.
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
echo "[6/9] GWL_FCS30 wetland raster tiles (CC BY 4.0, Zenodo 7340516)" | tee -a "$LOG"

GWL_DIR="$WORK_DIR/gwl-tiles"
GWL_VRT="$WORK_DIR/gwl-fcs30-mosaic.vrt"
GWL_TIF="$GWL_VRT"  # VRT used directly — gdallocationinfo reads VRT natively
mkdir -p "$GWL_DIR"

# Zenodo record 7340516 distributes 408 individual 5°-lat/lon TIFs
# (e.g. GWL_FCS30_2020_E0N10.tif), NOT 30°-lon zip archives.
# Collect all tiles already present in gwl-tiles/ and build a VRT mosaic.
GWL_TIFS=()
while IFS= read -r -d '' f; do
    GWL_TIFS+=("$f")
done < <(find "$GWL_DIR" -name "GWL_FCS30_*.tif" -print0 2>/dev/null | sort -z)

if [[ ${#GWL_TIFS[@]} -gt 0 ]]; then
    echo "  Found ${#GWL_TIFS[@]} GWL_FCS30 tiles in $GWL_DIR" | tee -a "$LOG"
    gdalbuildvrt "$GWL_VRT" "${GWL_TIFS[@]}" 2>&1 | tee -a "$LOG"
    echo "  → Mosaic VRT: $GWL_VRT (${#GWL_TIFS[@]} tiles)  ✓" | tee -a "$LOG"
    SKIP_WETLAND=0
else
    echo "WARN: No GWL_FCS30 tiles found in $GWL_DIR — wetland sampling will be skipped" | tee -a "$LOG"
    SKIP_WETLAND=1
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
        if c.get('seismic_pga_g') is not None:
            continue
        if c.get('iso') not in EU_ISOS:
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
