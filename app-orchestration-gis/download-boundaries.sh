#!/usr/bin/env bash
# download-boundaries.sh — download offline reverse-geocoding boundary files
#
# Downloads 5 GeoJSON files to cluster-totebox-personnel-1/boundaries/.
# Total size ~10 MB. Run once before the first build-clusters.py execution.
#
# Sources (all open-data, no API key required):
#   US:       Census TIGER GENZ2023 CBSA boundaries (Metropolitan Statistical Areas)
#   Canada:   Statistics Canada 2021 Census Metropolitan Areas
#   Mexico:   INEGI 2018 Zonas Metropolitanas
#   EU/rest:  Eurostat GISCO NUTS-3 2021 level-3 (1:3M)
#   Fallback: Natural Earth Admin-1 10m (global)

set -euo pipefail

DEST="/srv/foundry/deployments/cluster-totebox-personnel-1/boundaries"
mkdir -p "$DEST"

# Convert SHP → GeoJSON using ogr2ogr (gdal-bin) or Geopandas
convert_shp() {
  local src_dir="$1"
  local dest_file="$2"
  if command -v ogr2ogr >/dev/null 2>&1; then
    ogr2ogr -f GeoJSON "$dest_file" "$src_dir"/*.shp \
      -t_srs EPSG:4326 -lco COORDINATE_PRECISION=5
    echo "  Written: $dest_file (via ogr2ogr)"
  else
    echo "  WARNING: ogr2ogr not found — trying Geopandas fallback"
    python3 - "$src_dir" "$dest_file" <<'PYEOF'
import sys, glob, os
try:
    import geopandas as gpd
    shp_files = glob.glob(os.path.join(sys.argv[1], "*.shp"))
    if not shp_files:
        print(f"  ERROR: No .shp files found in {sys.argv[1]}")
        sys.exit(1)
    df = gpd.read_file(shp_files[0])
    # Ensure WGS84
    if df.crs and df.crs.to_epsg() != 4326:
        df = df.to_crs(epsg=4326)
    df.to_file(sys.argv[2], driver='GeoJSON')
    print(f"  Written: {sys.argv[2]} (via Geopandas)")
except ImportError:
    print("  ERROR: Geopandas not found. Install: pip install geopandas")
    sys.exit(1)
except Exception as e:
    print(f"  ERROR: {e}")
    sys.exit(1)
PYEOF
  fi
}

echo "Downloading US CBSA boundaries (Census TIGER GENZ2023)..."
curl -fsSL \
  "https://www2.census.gov/geo/tiger/GENZ2023/shp/cb_2023_us_cbsa_20m.zip" \
  -o /tmp/us_cbsa.zip
rm -rf /tmp/us_cbsa_shp && mkdir -p /tmp/us_cbsa_shp
unzip -o /tmp/us_cbsa.zip -d /tmp/us_cbsa_shp
convert_shp "/tmp/us_cbsa_shp" "$DEST/us_cbsa.geojson"

echo "Downloading Canada CMA boundaries (Statistics Canada 2021)..."
curl -fsSL \
  "https://www12.statcan.gc.ca/census-recensement/2021/geo/sip-pis/boundary-limites/files-fichiers/lcma000b21a_e.zip" \
  -o /tmp/ca_cma.zip
rm -rf /tmp/ca_cma_shp && mkdir -p /tmp/ca_cma_shp
unzip -o /tmp/ca_cma.zip -d /tmp/ca_cma_shp
convert_shp "/tmp/ca_cma_shp" "$DEST/ca_cma.geojson"

echo "Downloading Mexico admin-2 (Municipio) via GADM 4.1..."
curl -fsSL --max-time 60 \
  "https://geodata.ucdavis.edu/gadm/gadm4.1/json/gadm41_MEX_2.json.zip" \
  -o /tmp/mx_municipio.zip
rm -rf /tmp/mx_municipio_dir && mkdir -p /tmp/mx_municipio_dir
unzip -o /tmp/mx_municipio.zip -d /tmp/mx_municipio_dir
# GADM ships as a .json file inside the zip; rename to standard layer name
mv /tmp/mx_municipio_dir/*.json "$DEST/mx_municipio.geojson"
echo "  Written: $DEST/mx_municipio.geojson (GADM 4.1 admin-2 — Municipio)"

echo "Downloading Canada admin-3 (Census Subdivision proxy) via GADM 4.1..."
# GADM admin-2 for CA is generic "DivisionNo.X" — useless for Sherwood Park / Edmonton
# distinction. Admin-3 has the named subdivisions (Edmonton, Strathcona County, etc.)
curl -fsSL --max-time 90 \
  "https://geodata.ucdavis.edu/gadm/gadm4.1/json/gadm41_CAN_3.json.zip" \
  -o /tmp/ca_csd.zip
rm -rf /tmp/ca_csd_dir && mkdir -p /tmp/ca_csd_dir
unzip -o /tmp/ca_csd.zip -d /tmp/ca_csd_dir
mv /tmp/ca_csd_dir/*.json "$DEST/ca_csd.geojson"
echo "  Written: $DEST/ca_csd.geojson (GADM 4.1 admin-3 — Census Subdivision proxy)"

echo "Downloading EU NUTS-3 2021 boundaries (Eurostat GISCO, 1:3M)..."
curl -fsSL \
  "https://gisco-services.ec.europa.eu/distribution/v2/nuts/geojson/NUTS_RG_03M_2021_4326_LEVL_3.geojson" \
  -o "$DEST/eu_nuts3.geojson"
echo "  Written: $DEST/eu_nuts3.geojson"

echo "Downloading Natural Earth Admin-1 fallback (10m)..."
curl -fsSL \
  "https://raw.githubusercontent.com/nvkelso/natural-earth-vector/master/geojson/ne_10m_admin_1_states_provinces.geojson" \
  -o "$DEST/fallback_ne_admin1.geojson"
echo "  Written: $DEST/fallback_ne_admin1.geojson"

echo ""
echo "All boundary files ready in $DEST/"
ls -lh "$DEST/"
