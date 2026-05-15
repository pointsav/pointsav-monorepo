"""
ingest-mitma.py — Spain MITMA Big Data Mobility Study ingest
=============================================================
Ingests MITMA OD matrix CSV files (Orange + Vodafone MNO data) and produces
H3 res-7 catchment signals for Spanish co-location clusters.

Source:
  Ministry of Transport (MITMA) Big Data Mobility Study
  Main page:  https://www.mitma.gob.es/ministerio/proyectos-singulares/estudio-de-movilidad-con-big-data
  Open data:  https://movilidad-opendata.mitma.es/

ZONE_SHAPEFILE_FORMATS note:
  MITMA has released zone geometries in multiple formats across study versions:
    v1 (2020–2021): shapefile (.shp) inside a zip, CRS EPSG:4326
    v2 (2022+):     GeoJSON (.geojson or .json), CRS EPSG:4326
    Fallback:       CSV with columns zona_id,lat,lon (centroid table)
  This script attempts GeoJSON first (glob *.geojson / *.json), then shapefile
  (.shp via fiona/geopandas if available), then CSV centroid fallback.
  If none is found the script exits with instructions.

OD CSV column schema (approximate; minor variation across study versions):
  origen, destino, actividad_origen, actividad_destino, residencia,
  edad, periodo, viajes
  - origen / destino: zone ID string
  - actividad_origen / actividad_destino: activity type at origin/destination
      trabajo = work, hogar = home, otros_motivos = other/retail
  - residencia: "1" if traveler's home zone matches origen
  - periodo: 0=night, 1=morning, 2=afternoon
  - viajes: trip count (float; values <100 may be suppressed/zero)

Usage:
  python3 ingest-mitma.py            # full ingest
  python3 ingest-mitma.py --dry-run  # first 1000 rows of first CSV, no output written

Dependencies (all available in this pipeline):
  h3, json, csv, pathlib, math, sys, collections, zipfile, os
"""

import os
import sys
import csv
import gzip
import json
import math
import zipfile
import collections
import urllib.request
import urllib.error
from pathlib import Path

# ---------------------------------------------------------------------------
# Path constants
# ---------------------------------------------------------------------------

CLUSTERS_META = (
    "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
)
RAW_MITMA_DIR = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-mobility/raw/mitma"
)
ZONES_DIR = RAW_MITMA_DIR / "zonas"
VIAJES_DIR = RAW_MITMA_DIR / "viajes"
OUTPUT_DIR = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-mobility"
)
OUTPUT_HOME = OUTPUT_DIR / "mitma-home-od-es.jsonl"
OUTPUT_WORK = OUTPUT_DIR / "mitma-work-od-es.jsonl"

# H3 resolution used throughout the pipeline
H3_RES = 7

# Activity-type string constants (MITMA v2 vocabulary)
# v1 used "trabajo"/"hogar"; v2 uses "frecuente"/"casa"
ACTIVIDAD_TRABAJO = "frecuente"        # work/frequent-destination activity
ACTIVIDAD_HOGAR   = "casa"             # home activity type

# MITMA portal (movilidad-opendata.mitma.es) returns HTTP 500.
# Data is accessible directly from the S3 buckets behind the portal.
S3_V2 = "https://mitma-movilidad-v2.s3.amazonaws.com"

# Shapefile components for the v2 district zonification (ETRS89 UTM30N → WGS84)
S3_ZONE_FILES = [
    ("zonificacion/zonificacion_distritos/zonificacion_distritos.shp", "zonificacion_distritos.shp"),
    ("zonificacion/zonificacion_distritos/zonificacion_distritos.dbf", "zonificacion_distritos.dbf"),
    ("zonificacion/zonificacion_distritos/zonificacion_distritos.shx", "zonificacion_distritos.shx"),
    ("zonificacion/zonificacion_distritos/zonificacion_distritos.prj", "zonificacion_distritos.prj"),
]

# One representative week of v2 daily viajes (Jan 3–9 2022: Mon–Sun)
S3_VIAJES_SAMPLE = [
    ("estudios_basicos/por-distritos/viajes/ficheros-diarios/2022-01/20220103_Viajes_distritos.csv.gz", "20220103_Viajes_distritos.csv.gz"),
    ("estudios_basicos/por-distritos/viajes/ficheros-diarios/2022-01/20220104_Viajes_distritos.csv.gz", "20220104_Viajes_distritos.csv.gz"),
    ("estudios_basicos/por-distritos/viajes/ficheros-diarios/2022-01/20220105_Viajes_distritos.csv.gz", "20220105_Viajes_distritos.csv.gz"),
    ("estudios_basicos/por-distritos/viajes/ficheros-diarios/2022-01/20220106_Viajes_distritos.csv.gz", "20220106_Viajes_distritos.csv.gz"),
    ("estudios_basicos/por-distritos/viajes/ficheros-diarios/2022-01/20220107_Viajes_distritos.csv.gz", "20220107_Viajes_distritos.csv.gz"),
    ("estudios_basicos/por-distritos/viajes/ficheros-diarios/2022-01/20220108_Viajes_distritos.csv.gz", "20220108_Viajes_distritos.csv.gz"),
    ("estudios_basicos/por-distritos/viajes/ficheros-diarios/2022-01/20220109_Viajes_distritos.csv.gz", "20220109_Viajes_distritos.csv.gz"),
]

# ---------------------------------------------------------------------------
# H3 import guard
# ---------------------------------------------------------------------------

try:
    import h3
except ImportError:
    print("ERROR: h3 library not available. Install with: pip install h3")
    print("       This pipeline requires h3 >= 3.7.")
    sys.exit(1)

# ---------------------------------------------------------------------------
# Utility: haversine distance (km) — mirrors spatial_filter.py
# ---------------------------------------------------------------------------

def haversine_km(lon1: float, lat1: float, lon2: float, lat2: float) -> float:
    R = 6371.0
    phi1, phi2 = math.radians(lat1), math.radians(lat2)
    dphi = math.radians(lat2 - lat1)
    dlambda = math.radians(lon2 - lon1)
    a = (math.sin(dphi / 2) ** 2
         + math.cos(phi1) * math.cos(phi2) * math.sin(dlambda / 2) ** 2)
    return 2 * R * math.atan2(math.sqrt(a), math.sqrt(1 - a))

# ---------------------------------------------------------------------------
# Step 1: Auto-download raw data from MITMA S3
# ---------------------------------------------------------------------------

def _dl(s3_path: str, dest: Path, label: str) -> bool:
    """Download one file from S3_V2 to dest; skip if already cached."""
    if dest.exists() and dest.stat().st_size > 0:
        size_mb = dest.stat().st_size / 1_048_576
        print(f"  [cache] {dest.name} ({size_mb:.1f} MB)")
        return True
    dest.parent.mkdir(parents=True, exist_ok=True)
    url = f"{S3_V2}/{s3_path}"
    print(f"  Downloading {label}: {url}")
    tmp = dest.with_suffix(dest.suffix + ".part")
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "foundry-mitma-ingest/1"})
        with urllib.request.urlopen(req, timeout=300) as resp:
            total = int(resp.headers.get("Content-Length", "0") or "0")
            chunk, read, last_pct = 256 * 1024, 0, -1
            with open(tmp, "wb") as out:
                while True:
                    buf = resp.read(chunk)
                    if not buf:
                        break
                    out.write(buf)
                    read += len(buf)
                    if total > 0:
                        pct = int(read * 100 / total)
                        if pct >= last_pct + 5:
                            print(f"    {pct:>3}%  {read/1_048_576:.1f} / {total/1_048_576:.1f} MB")
                            last_pct = pct
        tmp.rename(dest)
        print(f"  -> {dest.name} ({dest.stat().st_size / 1_048_576:.1f} MB)")
        return True
    except (urllib.error.URLError, OSError) as e:
        print(f"  [FAIL] {label}: {e}")
        if tmp.exists():
            tmp.unlink()
        return False


def ensure_raw_data() -> bool:
    """Download MITMA zone shapefile and sample viajes files if not cached."""
    print("Checking MITMA raw data...")
    ok = True

    # Zone shapefile
    for s3_path, fname in S3_ZONE_FILES:
        if not _dl(s3_path, ZONES_DIR / fname, fname):
            ok = False

    # Sample viajes (one week Jan 2022)
    for s3_path, fname in S3_VIAJES_SAMPLE:
        if not _dl(s3_path, VIAJES_DIR / fname, fname):
            ok = False

    return ok

# ---------------------------------------------------------------------------
# Step 2: Load Spanish clusters (iso == "ES")
# ---------------------------------------------------------------------------

def load_spanish_clusters(threshold_km: float = 150.0):
    """
    Load all Spanish clusters from clusters-meta.json.
    Returns a list of dicts: {id, lat, lon} filtered to iso == "ES".
    Also returns a ClusterFilter-style bucket index for fast proximity lookup.
    """
    if not Path(CLUSTERS_META).exists():
        print(f"ERROR: clusters-meta.json not found at {CLUSTERS_META}")
        sys.exit(1)

    with open(CLUSTERS_META, "r") as f:
        data = json.load(f)

    spanish = []
    for c in data:
        iso = c.get("iso", "")
        if isinstance(iso, list):
            iso_match = "ES" in iso
        else:
            iso_match = iso == "ES"
        if iso_match:
            spanish.append({"id": c["id"], "lat": c["lat"], "lon": c["lon"]})

    if not spanish:
        print("ERROR: No Spanish clusters (iso=ES) found in clusters-meta.json")
        sys.exit(1)

    print(f"Loaded {len(spanish)} Spanish clusters from clusters-meta.json.")

    # Build bucket index for O(1) proximity lookup
    buffer = math.ceil(threshold_km / 50.0)
    if buffer < 2:
        buffer = 2

    buckets = collections.defaultdict(list)
    for c in spanish:
        lon_i = int(c["lon"])
        lat_i = int(c["lat"])
        for i in range(lon_i - buffer, lon_i + buffer + 1):
            for j in range(lat_i - buffer, lat_i + buffer + 1):
                buckets[(i, j)].append(c)

    return spanish, buckets, threshold_km

def nearby_clusters(lon: float, lat: float, buckets, threshold_km: float):
    """
    Return list of cluster IDs within threshold_km of (lon, lat).
    Uses the same bucket strategy as ClusterFilter.
    """
    key = (int(lon), int(lat))
    candidates = buckets.get(key, [])
    result = []
    for c in candidates:
        if haversine_km(lon, lat, c["lon"], c["lat"]) <= threshold_km:
            result.append(c["id"])
    return result

# ---------------------------------------------------------------------------
# Step 3: Load zone geometries → zone_id: (lat, lon) centroid map
# ---------------------------------------------------------------------------

def _centroid_of_geojson_feature(feature: dict):
    """Compute a simple centroid for a GeoJSON Polygon or MultiPolygon feature."""
    geom = feature.get("geometry", {})
    gtype = geom.get("type", "")
    coords = geom.get("coordinates", [])

    if gtype == "Point":
        return coords[1], coords[0]

    # Flatten all coordinate rings into a single list
    flat = []
    if gtype == "Polygon":
        for ring in coords:
            flat.extend(ring)
    elif gtype == "MultiPolygon":
        for poly in coords:
            for ring in poly:
                flat.extend(ring)

    if not flat:
        return None, None

    lon_sum = sum(pt[0] for pt in flat)
    lat_sum = sum(pt[1] for pt in flat)
    n = len(flat)
    return lat_sum / n, lon_sum / n


def _try_load_geojson(zones_dir: Path):
    """Attempt to load zone centroids from GeoJSON files under zones_dir."""
    candidates = (
        list(zones_dir.rglob("*.geojson"))
        + list(zones_dir.rglob("*.json"))
    )
    if not candidates:
        return None

    zone_map = {}
    for path in candidates:
        try:
            with open(path, "r", encoding="utf-8") as f:
                gj = json.load(f)
        except (json.JSONDecodeError, OSError) as e:
            print(f"  Warning: could not parse {path.name}: {e}")
            continue

        features = gj.get("features", [])
        for feat in features:
            props = feat.get("properties", {})
            # MITMA uses 'ID', 'id', 'ZONA', 'zona', 'distrito', 'DISTRITO'
            zone_id = (
                props.get("ID")
                or props.get("id")
                or props.get("ZONA")
                or props.get("zona")
                or props.get("DISTRITO")
                or props.get("distrito")
            )
            if zone_id is None:
                continue
            zone_id = str(zone_id).strip()
            lat, lon = _centroid_of_geojson_feature(feat)
            if lat is not None:
                zone_map[zone_id] = (lat, lon)

    return zone_map if zone_map else None


def _try_load_csv_centroids(zones_dir: Path):
    """
    Fallback: load zone centroids from a CSV file.
    Expected columns: zona_id (or id/ID), lat, lon.
    """
    candidates = list(zones_dir.rglob("*.csv"))
    if not candidates:
        return None

    zone_map = {}
    for path in candidates:
        try:
            with open(path, "r", encoding="utf-8") as f:
                reader = csv.DictReader(f, delimiter=";")
                # Also try comma if semicolon yields a single-column header
                first_row = None
                for row in reader:
                    first_row = row
                    break
                if first_row is None:
                    continue
                # Re-open to read from the beginning
                f.seek(0)
                # Detect delimiter
                sample = f.read(512)
                f.seek(0)
                delim = ";" if ";" in sample else ","
                reader2 = csv.DictReader(f, delimiter=delim)
                for row in reader2:
                    zone_id = (
                        row.get("zona_id")
                        or row.get("id")
                        or row.get("ID")
                        or row.get("ZONA")
                    )
                    if zone_id is None:
                        continue
                    try:
                        lat = float(row.get("lat") or row.get("LAT") or "")
                        lon = float(row.get("lon") or row.get("LON") or "")
                    except ValueError:
                        continue
                    zone_map[str(zone_id).strip()] = (lat, lon)
        except OSError as e:
            print(f"  Warning: could not read {path.name}: {e}")
            continue

    return zone_map if zone_map else None


def load_zone_map(zones_dir: Path) -> dict:
    """
    Load MITMA zone centroids, trying formats in priority order:
      1. GeoJSON / JSON (v2 study format)
      2. CSV centroid table (fallback)
      3. Shapefile via geopandas/fiona (optional; graceful skip if unavailable)

    Returns dict: zone_id_str → (lat, lon).
    Exits with an error message if no geometry source is found.
    """
    print("Loading MITMA zone geometries...")

    zone_map = _try_load_geojson(zones_dir)
    if zone_map:
        print(f"  Loaded {len(zone_map)} zones from GeoJSON.")
        return zone_map

    print("  No GeoJSON zone file found; trying CSV centroid table...")
    zone_map = _try_load_csv_centroids(zones_dir)
    if zone_map:
        print(f"  Loaded {len(zone_map)} zones from CSV centroid table.")
        return zone_map

    # Optional: try geopandas/fiona for shapefile support
    print("  No CSV centroid table found; trying shapefile (requires geopandas)...")
    shp_candidates = list(zones_dir.rglob("*.shp"))
    if shp_candidates:
        try:
            import geopandas as gpd  # not in base requirements; graceful failure
            zone_map = {}
            for shp_path in shp_candidates:
                gdf = gpd.read_file(shp_path).to_crs("EPSG:4326")
                for _, row in gdf.iterrows():
                    centroid = row.geometry.centroid
                    zone_id = (
                        str(row.get("ID") or row.get("id") or row.get("ZONA")
                            or row.get("DISTRITO") or "").strip()
                    )
                    if zone_id:
                        zone_map[zone_id] = (centroid.y, centroid.x)
            if zone_map:
                print(f"  Loaded {len(zone_map)} zones from shapefile via geopandas.")
                return zone_map
        except ImportError:
            print("  geopandas not available; shapefile support skipped.")
        except Exception as e:
            print(f"  Warning: shapefile load failed: {e}")

    print(
        "ERROR: No MITMA zone geometry source found in:\n"
        f"  {zones_dir}\n"
        "Expected one of:\n"
        "  *.geojson or *.json  (GeoJSON with feature properties containing ID/zona)\n"
        "  *.csv                (columns: zona_id, lat, lon)\n"
        "  *.shp                (shapefile, requires geopandas)\n"
        + DOWNLOAD_INSTRUCTIONS
    )
    sys.exit(1)

# ---------------------------------------------------------------------------
# Step 4: Build zone → H3 + cluster proximity index
# ---------------------------------------------------------------------------

def build_zone_index(zone_map: dict, buckets, threshold_km: float) -> dict:
    """
    For each MITMA zone centroid within 150 km of any Spanish cluster,
    compute its H3 res-7 cell and record nearby cluster IDs.

    Returns dict: zone_id → {h3, lat, lon, cluster_proximity}
    Zones outside the 150 km envelope are excluded.
    """
    print(f"Building zone → H3 index (filtering to {threshold_km} km radius)...")
    index = {}
    total = len(zone_map)
    kept = 0

    for zone_id, (lat, lon) in zone_map.items():
        nearby = nearby_clusters(lon, lat, buckets, threshold_km)
        if not nearby:
            continue
        h3_cell = h3.latlng_to_cell(lat, lon, H3_RES)
        index[zone_id] = {
            "h3": h3_cell,
            "lat": round(lat, 6),
            "lon": round(lon, 6),
            "cluster_proximity": nearby,
        }
        kept += 1

    print(f"  Zone index: processed {total}, kept {kept} within {threshold_km} km.")
    return index

# ---------------------------------------------------------------------------
# Step 5: Discover OD CSV files (may be zipped)
# ---------------------------------------------------------------------------

def iter_od_csvs(viajes_dir: Path):
    """
    Yield (filename_hint, file_object) for every OD CSV found under viajes_dir.
    Handles:
      - plain .csv files
      - .zip archives containing .csv files (one per day, zipped by month)
    Yields tuples: (label: str, lines: iterable-of-str)
    """
    for path in sorted(viajes_dir.rglob("*")):
        if path.suffix.lower() == ".csv":
            try:
                with open(path, "r", encoding="utf-8", errors="replace") as f:
                    yield str(path.name), f.readlines()
            except OSError as e:
                print(f"  Warning: cannot read {path.name}: {e}")

        elif path.suffix.lower() == ".zip":
            try:
                with zipfile.ZipFile(path, "r") as zf:
                    for member in sorted(zf.namelist()):
                        if member.lower().endswith(".csv"):
                            with zf.open(member) as zcsv:
                                lines = zcsv.read().decode("utf-8", errors="replace").splitlines(keepends=True)
                                yield f"{path.name}/{member}", lines
            except (zipfile.BadZipFile, OSError) as e:
                print(f"  Warning: cannot open zip {path.name}: {e}")

        elif path.name.lower().endswith(".csv.gz"):
            try:
                with gzip.open(path, "rb") as gz:
                    lines = gz.read().decode("utf-8", errors="replace").splitlines(keepends=True)
                yield path.name, lines
            except (OSError, EOFError) as e:
                print(f"  Warning: cannot read gz {path.name}: {e}")

# ---------------------------------------------------------------------------
# Step 6: Aggregate OD flows
# ---------------------------------------------------------------------------

def _detect_delimiter(header_line: str) -> str:
    """Detect CSV delimiter from header (MITMA uses pipe '|' or semicolon ';' or comma)."""
    for delim in ("|", ";", ",", "\t"):
        if delim in header_line:
            return delim
    return "|"  # MITMA default for most releases


def aggregate_od_flows(viajes_dir: Path, zone_index: dict, dry_run: bool = False):
    """
    Iterate over all OD CSV files and accumulate:
      - home_od: origin H3 cells for trips TO cluster-zone destinations where
                 the traveler originates from a home zone
                 (actividad_origen == 'hogar' OR residencia == '1')
      - work_od: origin H3 cells for trips TO cluster-zone destinations where
                 actividad_destino == 'trabajo'

    Accumulator schema:
      {h3_origin: {lat, lon, visits_total, cluster_proximity: set, iso}}

    dry_run=True: process first 1000 data rows of the first CSV and return
    stats without writing any output.
    """
    # accumulators: keyed by (signal, h3_origin)
    # value: {lat, lon, visits, cluster_ids_set}
    home_acc = collections.defaultdict(lambda: {"visits": 0.0, "lat": 0.0, "lon": 0.0, "clusters": set()})
    work_acc = collections.defaultdict(lambda: {"visits": 0.0, "lat": 0.0, "lon": 0.0, "clusters": set()})

    files_processed = 0
    rows_processed = 0
    rows_home = 0
    rows_work = 0
    rows_skipped_no_dest = 0
    dry_run_limit = 1000
    dry_run_done = False

    for label, lines in iter_od_csvs(viajes_dir):
        if dry_run and dry_run_done:
            break

        if not lines:
            continue

        header_line = lines[0].rstrip("\n\r")
        delimiter = _detect_delimiter(header_line)

        # Parse header to column index map (case-insensitive strip)
        raw_cols = [c.strip().lower() for c in header_line.split(delimiter)]
        col_idx = {name: i for i, name in enumerate(raw_cols)}

        # Required columns
        required = {"origen", "destino", "viajes"}
        missing = required - col_idx.keys()
        if missing:
            print(f"  Warning: {label} missing columns {missing}; skipping file.")
            continue

        files_processed += 1
        print(f"  Processing {label} ({len(lines) - 1} data rows)...")

        for line in lines[1:]:
            line = line.rstrip("\n\r")
            if not line:
                continue

            parts = line.split(delimiter)
            if len(parts) <= max(col_idx.values()):
                continue

            def get(col, default=""):
                idx = col_idx.get(col)
                if idx is None or idx >= len(parts):
                    return default
                return parts[idx].strip()

            origen   = get("origen")
            destino  = get("destino")
            viajes_s = get("viajes", "0")
            act_orig = get("actividad_origen", "")
            act_dest = get("actividad_destino", "")
            residencia = get("residencia", "")

            try:
                viajes = float(viajes_s.replace(",", "."))
            except ValueError:
                continue

            if viajes <= 0:
                continue

            # Destination must be a zone within our cluster envelope
            dest_info = zone_index.get(destino)
            if dest_info is None:
                rows_skipped_no_dest += 1
                rows_processed += 1
                if dry_run and rows_processed >= dry_run_limit:
                    dry_run_done = True
                    break
                continue

            # Origin zone must also be indexed (within envelope) to have an H3 cell
            orig_info = zone_index.get(origen)
            if orig_info is None:
                rows_processed += 1
                if dry_run and rows_processed >= dry_run_limit:
                    dry_run_done = True
                    break
                continue

            orig_h3  = orig_info["h3"]
            orig_lat = orig_info["lat"]
            orig_lon = orig_info["lon"]
            dest_clusters = dest_info["cluster_proximity"]

            rows_processed += 1

            # HOME signal: trip from a home zone (residential origin)
            # v2: actividad_origen == "casa"; residencia is a province code, not used
            is_home_trip = act_orig.lower() == ACTIVIDAD_HOGAR
            if is_home_trip:
                rec = home_acc[orig_h3]
                rec["visits"] += viajes
                rec["lat"] = orig_lat
                rec["lon"] = orig_lon
                rec["clusters"].update(dest_clusters)
                rows_home += 1

            # WORK signal: trip whose destination activity is trabajo (work)
            if act_dest.lower() == ACTIVIDAD_TRABAJO:
                rec = work_acc[orig_h3]
                rec["visits"] += viajes
                rec["lat"] = orig_lat
                rec["lon"] = orig_lon
                rec["clusters"].update(dest_clusters)
                rows_work += 1

            if dry_run and rows_processed >= dry_run_limit:
                dry_run_done = True
                break

        if dry_run and dry_run_done:
            break

    print(
        f"  OD aggregation complete: {files_processed} files, "
        f"{rows_processed} rows read, "
        f"{rows_home} home-signal rows, "
        f"{rows_work} work-signal rows, "
        f"{rows_skipped_no_dest} rows with destination outside envelope."
    )
    return home_acc, work_acc

# ---------------------------------------------------------------------------
# Step 7: Write output JSONL files
# ---------------------------------------------------------------------------

def write_output(home_acc: dict, work_acc: dict) -> None:
    """
    Serialize home and work OD accumulators to JSONL.
    Each record includes lat, lon, iso, h3, visits_home_total (or
    visits_work_total), cluster_proximity, data_source, is_measured.
    """
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    # -- HOME --
    home_count = 0
    with open(OUTPUT_HOME, "w", encoding="utf-8") as f:
        for h3_cell, rec in sorted(home_acc.items()):
            record = {
                "h3": h3_cell,
                "lat": round(rec["lat"], 6),
                "lon": round(rec["lon"], 6),
                "iso": "ES",
                "visits_home_total": round(rec["visits"], 2),
                "cluster_proximity": sorted(rec["clusters"]),
                "data_source": "mitma_mno",
                "is_measured": True,
            }
            f.write(json.dumps(record) + "\n")
            home_count += 1
    print(f"  Home OD: {home_count} H3 origin cells → {OUTPUT_HOME}")

    # -- WORK --
    work_count = 0
    with open(OUTPUT_WORK, "w", encoding="utf-8") as f:
        for h3_cell, rec in sorted(work_acc.items()):
            record = {
                "h3": h3_cell,
                "lat": round(rec["lat"], 6),
                "lon": round(rec["lon"], 6),
                "iso": "ES",
                "visits_work_total": round(rec["visits"], 2),
                "cluster_proximity": sorted(rec["clusters"]),
                "data_source": "mitma_mno",
                "is_measured": True,
            }
            f.write(json.dumps(record) + "\n")
            work_count += 1
    print(f"  Work OD: {work_count} H3 origin cells → {OUTPUT_WORK}")

# ---------------------------------------------------------------------------
# Step 8: Dry-run statistics printer
# ---------------------------------------------------------------------------

def print_dry_run_stats(home_acc: dict, work_acc: dict) -> None:
    print("\n--- DRY-RUN STATISTICS (first 1000 data rows, no files written) ---")
    print(f"  Unique H3 origin cells with HOME signal : {len(home_acc)}")
    print(f"  Unique H3 origin cells with WORK signal : {len(work_acc)}")
    if home_acc:
        total_home = sum(r["visits"] for r in home_acc.values())
        print(f"  Total home trip-volume accumulated      : {total_home:,.1f}")
    if work_acc:
        total_work = sum(r["visits"] for r in work_acc.values())
        print(f"  Total work trip-volume accumulated      : {total_work:,.1f}")

    # Sample records
    if home_acc:
        sample_h3 = next(iter(home_acc))
        rec = home_acc[sample_h3]
        print(f"\n  Sample HOME record:")
        print(f"    h3={sample_h3}, lat={rec['lat']}, lon={rec['lon']}, "
              f"visits={rec['visits']:.1f}, clusters={sorted(rec['clusters'])[:3]}...")
    if work_acc:
        sample_h3 = next(iter(work_acc))
        rec = work_acc[sample_h3]
        print(f"\n  Sample WORK record:")
        print(f"    h3={sample_h3}, lat={rec['lat']}, lon={rec['lon']}, "
              f"visits={rec['visits']:.1f}, clusters={sorted(rec['clusters'])[:3]}...")
    print("-------------------------------------------------------------------\n")

# ---------------------------------------------------------------------------
# Main entry point
# ---------------------------------------------------------------------------

def main() -> None:
    dry_run = "--dry-run" in sys.argv

    if dry_run:
        print("=== ingest-mitma.py (DRY RUN — first 1000 rows, no output written) ===")
    else:
        print("=== ingest-mitma.py: Spain MITMA MNO mobility ingest ===")

    # Step 1: download raw data if not cached
    if not ensure_raw_data():
        print("ERROR: One or more MITMA downloads failed; check network and retry.")
        sys.exit(1)

    # Step 2: load Spanish clusters and build proximity index
    spanish_clusters, buckets, threshold_km = load_spanish_clusters(threshold_km=150.0)

    # Step 3: load zone centroids from geometry files
    zone_map = load_zone_map(ZONES_DIR)

    # Step 4: build zone → H3 + cluster index (filtered to 150 km envelope)
    zone_index = build_zone_index(zone_map, buckets, threshold_km)

    if not zone_index:
        print(
            "ERROR: No MITMA zones fall within 150 km of any Spanish cluster.\n"
            "       Check that zone geometries cover mainland Spain."
        )
        sys.exit(1)

    # Step 5–6: aggregate OD flows
    home_acc, work_acc = aggregate_od_flows(VIAJES_DIR, zone_index, dry_run=dry_run)

    if dry_run:
        print_dry_run_stats(home_acc, work_acc)
        return

    # Step 7: write output JSONL
    if not home_acc and not work_acc:
        print(
            "WARNING: No qualifying OD rows found. Output files not written.\n"
            "         Check that viajes CSV files are present and non-empty in:\n"
            f"         {VIAJES_DIR}"
        )
        return

    print("Writing output JSONL files...")
    write_output(home_acc, work_acc)
    print("ingest-mitma.py complete.")


if __name__ == "__main__":
    main()
