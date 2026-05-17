"""
ingest-statcan.py — Statistics Canada 2021 Census Journey-to-Work ingest
========================================================================
Ingests Statistics Canada 2021 Census commuter flow data (DA-level origin →
destination) and produces H3 res-7 WORK-reach origin distributions for
Canadian co-location clusters.

Source:
  Statistics Canada 2021 Census
  Table 98-400-X2021007:
    "Commuting flows for employed workers aged 15 and over by
     place of residence and place of work"
    https://www12.statcan.gc.ca/census-recensement/2021/dp-pd/dt-td/
      CompDataDownload.cfm?LANG=E&PID=141928&OFT=CSV

  DA population-weighted centroids (Figshare mirror — WORKING 2026-05-17):
    https://ndownloader.figshare.com/files/47126050
    Columns: DAUID, PRUID, y (lat), x (lon), Population

  NOTE: The StatCan official centroid URL is dead. The original CompDataDownload.cfm
  URL for 98-400-X2021007 is also permanently dead (redirects to 404 page).
  See COMMUTER_URL block below for the manual download procedure.

Fallback source (when bulk commuter-flow CSV is unavailable):
  Table 98-10-0494-01 — Place of work status by CT (work-reach proxy, not true OD)
    https://www150.statcan.gc.ca/n1/tbl/csv/98100494-eng.zip  [WORKING 2026-05-17]
  NOTE: This is CMA/CT-level, not DA-level. Granularity is coarser than the
  primary table. Records carry data_source="statcan_employment_proxy" + is_measured=False.

Pipeline:
  1. Load Canadian clusters (iso == "CA" or list containing "CA") from
     clusters-meta.json; build bucket-index for fast proximity lookup.
  2. Download CenPop2021_Mean_DA.csv (small, ~2MB) → {DAUID: (lat, lon)}.
  3. Download the Journey-to-Work bulk CSV (~500MB); fall back to the
     employment-by-DA proxy table if unavailable.
  4. For each origin-DA → dest-DA commuter row, if the destination DA is
     within 150 km of any CA cluster, accumulate the trip count into the
     origin DA's H3 res-7 cell.
  5. Emit per-H3 JSONL: h3, lat, lon, iso, visits_work_total,
     cluster_proximity, data_source, is_measured.

Usage:
  python3 ingest-statcan.py                   # full ingest
  python3 ingest-statcan.py --dry-run         # first 1000 rows, no output
  python3 ingest-statcan.py --province BC,ON  # filter by province code(s)

Dependencies (stdlib + h3):
  h3, urllib.request, csv, json, math, pathlib, sys, collections,
  argparse, zipfile, os
"""

import os
import sys
import csv
import json
import math
import zipfile
import argparse
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
RAW_STATCAN_DIR = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-mobility/raw/statcan"
)
OUTPUT_DIR = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-mobility"
)
OUTPUT_WORK    = OUTPUT_DIR / "statcan-work-od-ca.jsonl"
OUTPUT_SUMMARY = OUTPUT_DIR / "statcan-work-summary-ca.jsonl"

# H3 resolution used throughout the pipeline
H3_RES = 7

# ---------------------------------------------------------------------------
# Source URLs
# ---------------------------------------------------------------------------

# DA centroid mirror (Figshare) — VERIFIED WORKING 2026-05-17 (HTTP 302 → S3, real file).
#   DAUID, PRUID, y (lat), x (lon), Population — 57k DAs, pop-weighted centroids
CENTROIDS_URL = "https://ndownloader.figshare.com/files/47126050"
CENTROIDS_FILE = RAW_STATCAN_DIR / "Popwgt_DA_Cent.csv"

# ---------------------------------------------------------------------------
# PRIMARY COMMUTER TABLE — MANUAL DOWNLOAD REQUIRED
# ---------------------------------------------------------------------------
# Table 98-400-X2021007 (DA-to-DA commuter flow, ~500 MB) was a StatCan
# Census Data Product served via CompDataDownload.cfm (PID=141928).
# That endpoint is PERMANENTLY DEAD as of 2026-05-17: all www12 paths for
# PID=141928 redirect to https://www12.statcan.gc.ca/census-recensement/
#   srvmsg/srvmsg404.html (4,099-byte HTML error page).
# No automated public replacement for DA-level OD flow data exists.
# The CMA/CA-level commuting table (98-10-0460-01) IS publicly available but
# is NOT a valid substitute — it contains no DA-level geography.
#
# MANUAL DOWNLOAD PROCEDURE (DA-level commuter flow):
#   1. Go to: https://www12.statcan.gc.ca/census-recensement/2021/dp-pd/dt-td/
#   2. Under "Commuting", look for product 98-400-X2021007
#      ("Commuting flows for employed workers aged 15 and over by
#       place of residence and place of work").
#      If not found there, contact StatCan order desk:
#        https://www150.statcan.gc.ca/n1/en/about/contact-contactez-nous
#        Reference: Table 98-400-X2021007, PID=141928
#   3. Download the bulk CSV file (~500 MB); it may come as a .zip.
#   4. Save the file as one of:
#        {commuter_file}
#        {commuter_zip}
#   5. Re-run this script. It will detect the local file and skip the download.
# ---------------------------------------------------------------------------
COMMUTER_URL = (
    "https://www12.statcan.gc.ca/census-recensement/2021/dp-pd/dt-td/"
    "CompDataDownload.cfm?LANG=E&PID=141928&OFT=CSV"
)
COMMUTER_FILE = RAW_STATCAN_DIR / "98-400-X2021007.csv"
COMMUTER_ZIP  = RAW_STATCAN_DIR / "98-400-X2021007.zip"

# ---------------------------------------------------------------------------
# FALLBACK EMPLOYMENT TABLE
# ---------------------------------------------------------------------------
# Table 98-10-0494-01 — "Place of work status by occupation broad category,
# work activity, age and gender": Census metropolitan areas, tracted census
# agglomerations and census tracts of work.
#
# IMPORTANT LIMITATION: This table is CMA/CT-level, not DA-level.
# It cannot produce DA-granularity WORK-reach estimates. Records produced
# from this fallback carry data_source="statcan_employment_proxy" and
# is_measured=False to flag this limitation downstream.
#
# URL STATUS 2026-05-17:
#   Old dtblDnld.action URL: DEAD (HTTP 404).
#   New n1/tbl/csv/ URL:     WORKING (HTTP 200, ~107 MB ZIP).
#     https://www150.statcan.gc.ca/n1/tbl/csv/98100494-eng.zip
#   CKAN record: https://open.canada.ca/data/api/3/action/package_show?id=...
#     (CKAN search: "Place of work status occupation 2021 census")
EMPLOYMENT_URL = (
    "https://www150.statcan.gc.ca/n1/tbl/csv/98100494-eng.zip"
)
EMPLOYMENT_FILE = RAW_STATCAN_DIR / "98-10-0494-01.csv"
EMPLOYMENT_ZIP  = RAW_STATCAN_DIR / "98-10-0494-01.zip"

# Download-failure instructions block
DOWNLOAD_INSTRUCTIONS = """
  -----------------------------------------------------------------------
  Statistics Canada bulk download failed or files not found.

  SITUATION (verified 2026-05-17):
    - DA-level commuter OD table (98-400-X2021007, ~500 MB): the StatCan
      CompDataDownload.cfm endpoint is permanently dead. No automated
      public download exists. MANUAL DOWNLOAD REQUIRED (see below).
    - DA centroid file (Figshare mirror, ~3.4 MB): automated download
      should work. If it fails, proceed to step 1.
    - Fallback employment table (98-10-0494-01, ~107 MB): automated
      download from the new StatCan URL should work. If it fails, see
      step 3.

  STEP 1 — Population-weighted DA centroids (Figshare mirror, ~3.4 MB):
       mkdir -p {raw_dir}
       curl -L -o {centroids_file} \\
         "{centroids_url}"
     Expected columns: DAUID, PRUID, y (lat), x (lon), Population

  STEP 2 — Journey-to-Work DA-level commuter flows (MANUAL REQUIRED):
     This is the primary data source. Table 98-400-X2021007 must be
     obtained directly from Statistics Canada.

     Option A — Public catalogue (may still work):
       1. Open: https://www12.statcan.gc.ca/census-recensement/2021/dp-pd/dt-td/
       2. Click "Commuting" in the left-hand topic list.
       3. Look for "Commuting flows for employed workers aged 15 and over
          by place of residence and place of work" (PID 141928).
       4. Click "Download data" and select CSV format.
       5. Save the downloaded file as:
            {commuter_zip}
          (or {commuter_file} if it comes as a plain CSV)

     Option B — StatCan order desk (if public download is unavailable):
       1. Go to: https://www150.statcan.gc.ca/n1/en/about/contact-contactez-nous
       2. Reference: "Table 98-400-X2021007, PID=141928, Census 2021,
          commuting flows by place of residence and place of work,
          DA-level, bulk CSV".
       3. Save received file as: {commuter_zip}

     Re-run this script once {commuter_zip} is in place.
     The script will skip the download and process the local file.

  STEP 3 — Fallback: Place of work status by CT (automated, ~107 MB):
     NOTE: This fallback produces CMA/CT-level estimates, not DA-level.
     Results carry data_source="statcan_employment_proxy", is_measured=False.

     Automated URL (working as of 2026-05-17):
       {employment_url}

     If automated download fails, download manually:
       1. Open: https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=9810049401
       2. Click "Download options" → "Download entire table (CSV)".
       3. Save as: {employment_zip}

  STEP 4 — Re-run this script once any of the above files is in place.
  -----------------------------------------------------------------------
""".format(
    raw_dir=RAW_STATCAN_DIR,
    centroids_url=CENTROIDS_URL,
    centroids_file=CENTROIDS_FILE,
    commuter_file=COMMUTER_FILE,
    commuter_zip=COMMUTER_ZIP,
    employment_url=EMPLOYMENT_URL,
    employment_file=EMPLOYMENT_FILE,
)

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
# Utility: haversine distance (km) — mirrors ingest-mitma.py
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
# Utility: cached HTTP download with progress
# ---------------------------------------------------------------------------

def _format_bytes(n: int) -> str:
    for unit in ("B", "KB", "MB", "GB"):
        if n < 1024:
            return f"{n:.1f} {unit}"
        n /= 1024
    return f"{n:.1f} TB"


def download_with_progress(url: str, dest: Path, label: str) -> bool:
    """
    Download `url` to `dest`, showing periodic progress.
    Skips re-download if dest already exists and is non-empty.
    Returns True on success, False on any failure.
    """
    if dest.exists() and dest.stat().st_size > 0:
        print(f"  [cache] {label}: {dest.name} present ({_format_bytes(dest.stat().st_size)})")
        return True

    dest.parent.mkdir(parents=True, exist_ok=True)
    print(f"  [download] {label}: {url}")
    print(f"             → {dest}")

    tmp = dest.with_suffix(dest.suffix + ".part")
    try:
        req = urllib.request.Request(
            url,
            headers={"User-Agent": "Mozilla/5.0 (compatible; foundry-statcan-ingest/1)"},
        )
        with urllib.request.urlopen(req, timeout=120) as resp:
            total = int(resp.headers.get("Content-Length", "0") or "0")
            chunk = 1024 * 256  # 256 KB
            read = 0
            last_pct = -1
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
                            print(f"    {pct:>3}%  {_format_bytes(read)} / {_format_bytes(total)}")
                            last_pct = pct
                    elif read % (chunk * 40) == 0:
                        print(f"    ... {_format_bytes(read)}")
        tmp.rename(dest)
        print(f"  [done]   {label}: {_format_bytes(dest.stat().st_size)}")
        return True
    except (urllib.error.URLError, urllib.error.HTTPError, TimeoutError, OSError) as e:
        print(f"  [FAIL]   {label}: {e}")
        if tmp.exists():
            try:
                tmp.unlink()
            except OSError:
                pass
        return False

# ---------------------------------------------------------------------------
# Step 1: Load Canadian clusters (iso == "CA")
# ---------------------------------------------------------------------------

def load_canadian_clusters(threshold_km: float = 150.0):
    """
    Load all Canadian clusters from clusters-meta.json.
    Returns (clusters_list, bucket_index, threshold_km).
    Each cluster dict: {id, lat, lon}.
    """
    if not Path(CLUSTERS_META).exists():
        print(f"ERROR: clusters-meta.json not found at {CLUSTERS_META}")
        sys.exit(1)

    with open(CLUSTERS_META, "r") as f:
        data = json.load(f)

    canadian = []
    for c in data:
        iso = c.get("iso", "")
        if isinstance(iso, list):
            iso_match = "CA" in iso
        else:
            iso_match = iso == "CA"
        if iso_match:
            canadian.append({"id": c["id"], "lat": c["lat"], "lon": c["lon"]})

    if not canadian:
        print("ERROR: No Canadian clusters (iso=CA) found in clusters-meta.json")
        sys.exit(1)

    print(f"Loaded {len(canadian)} Canadian clusters from clusters-meta.json.")

    # Build 1°×1° bucket index (same pattern as ingest-mitma.py)
    buffer = math.ceil(threshold_km / 50.0)
    if buffer < 2:
        buffer = 2

    buckets = collections.defaultdict(list)
    for c in canadian:
        lon_i = int(c["lon"])
        lat_i = int(c["lat"])
        for i in range(lon_i - buffer, lon_i + buffer + 1):
            for j in range(lat_i - buffer, lat_i + buffer + 1):
                buckets[(i, j)].append(c)

    return canadian, buckets, threshold_km


def nearby_clusters(lon: float, lat: float, buckets, threshold_km: float):
    """Return list of cluster IDs within threshold_km of (lon, lat)."""
    key = (int(lon), int(lat))
    candidates = buckets.get(key, [])
    result = []
    for c in candidates:
        if haversine_km(lon, lat, c["lon"], c["lat"]) <= threshold_km:
            result.append(c["id"])
    return result

# ---------------------------------------------------------------------------
# Step 2: DA centroid loader
# ---------------------------------------------------------------------------

def load_da_centroids(provinces_filter=None) -> dict:
    """
    Parse CenPop2021_Mean_DA.csv → {DAUID (str): (lat, lon, pruid_str)}.

    Schema (StatCan official): DAUID, PRUID, PRNAME, CDUID, CSDUID,
    CSDNAME, ERUID, CMAUID, CMANAME, ALandDA, POPULATION, LATITUDE, LONGITUDE.

    `provinces_filter`: optional set of province codes (e.g. {"BC","ON"})
    in their two-letter form. PRUID mapping per StatCan:
        10 NL, 11 PE, 12 NS, 13 NB, 24 QC, 35 ON, 46 MB,
        47 SK, 48 AB, 59 BC, 60 YT, 61 NT, 62 NU
    """
    pruid_to_iso = {
        "10": "NL", "11": "PE", "12": "NS", "13": "NB",
        "24": "QC", "35": "ON", "46": "MB", "47": "SK",
        "48": "AB", "59": "BC", "60": "YT", "61": "NT", "62": "NU",
    }

    if not CENTROIDS_FILE.exists():
        print(f"ERROR: DA centroid file missing: {CENTROIDS_FILE}")
        return {}

    print(f"Loading DA centroids from {CENTROIDS_FILE.name} ...")
    centroids = {}
    skipped = 0

    # File is Windows-1252 in some StatCan releases; try utf-8 first.
    encodings = ("utf-8-sig", "utf-8", "cp1252", "latin-1")
    fh = None
    for enc in encodings:
        try:
            fh = open(CENTROIDS_FILE, "r", encoding=enc, newline="")
            fh.read(1)
            fh.seek(0)
            break
        except UnicodeDecodeError:
            if fh:
                fh.close()
            fh = None
    if fh is None:
        print(f"ERROR: could not decode {CENTROIDS_FILE} with any known encoding.")
        return {}

    try:
        reader = csv.DictReader(fh)
        # StatCan sometimes uses upper or mixed case header — normalize.
        if reader.fieldnames is None:
            print(f"ERROR: {CENTROIDS_FILE.name} has no header row.")
            return {}
        # Map normalized → original column name
        col_map = {c.strip().upper(): c for c in reader.fieldnames}

        def col(row, name):
            key = col_map.get(name.upper())
            return row[key].strip() if key and row.get(key) is not None else ""

        for row in reader:
            dauid = col(row, "DAUID")
            pruid = col(row, "PRUID")
            lat_s = col(row, "LATITUDE") or col(row, "Y")
            lon_s = col(row, "LONGITUDE") or col(row, "X")
            if not dauid or not lat_s or not lon_s:
                skipped += 1
                continue
            try:
                lat = float(lat_s)
                lon = float(lon_s)
            except ValueError:
                skipped += 1
                continue
            iso_prov = pruid_to_iso.get(pruid, "")
            if provinces_filter and iso_prov not in provinces_filter:
                continue
            centroids[dauid] = (lat, lon, iso_prov)
    finally:
        fh.close()

    print(f"  Loaded {len(centroids):,} DA centroids ({skipped} skipped).")
    if provinces_filter:
        print(f"  Province filter active: {sorted(provinces_filter)}")
    return centroids

# ---------------------------------------------------------------------------
# Step 3: Build DA → H3 / cluster-proximity index
# ---------------------------------------------------------------------------

def build_da_index(centroids: dict, buckets, threshold_km: float) -> dict:
    """
    For each DA centroid within 150 km of any CA cluster, compute its
    H3 res-7 cell and record the nearby cluster IDs.

    Returns: {DAUID: {h3, lat, lon, cluster_proximity:list, pr:iso2}}.
    """
    print(f"Building DA → H3 index (filtering to {threshold_km} km of CA clusters)...")
    index = {}
    kept = 0
    total = len(centroids)

    for dauid, (lat, lon, pr) in centroids.items():
        nearby = nearby_clusters(lon, lat, buckets, threshold_km)
        if not nearby:
            continue
        try:
            cell = h3.geo_to_h3(lat, lon, H3_RES)
        except AttributeError:
            # h3 v4 API
            cell = h3.latlng_to_cell(lat, lon, H3_RES)
        index[dauid] = {
            "h3": cell,
            "lat": round(lat, 6),
            "lon": round(lon, 6),
            "cluster_proximity": nearby,
            "pr": pr,
        }
        kept += 1

    print(f"  DA index: processed {total:,}, kept {kept:,} within {threshold_km} km.")
    return index

# ---------------------------------------------------------------------------
# Step 4: Commuter-flow CSV iterator (handles .csv and .zip)
# ---------------------------------------------------------------------------

def iter_commuter_rows(path: Path):
    """
    Yield (delimiter, header_list, row_iter) for each CSV under `path`.
    Handles plain .csv and .zip (with one or more .csv members).
    Decoded as utf-8 with cp1252 fallback (StatCan vintage).
    """
    def _open_text(rawbytes):
        for enc in ("utf-8-sig", "utf-8", "cp1252", "latin-1"):
            try:
                return rawbytes.decode(enc)
            except UnicodeDecodeError:
                continue
        return rawbytes.decode("latin-1", errors="replace")

    if path.suffix.lower() == ".zip":
        try:
            with zipfile.ZipFile(path, "r") as zf:
                for member in sorted(zf.namelist()):
                    if not member.lower().endswith(".csv"):
                        continue
                    print(f"  Reading zip member: {member}")
                    with zf.open(member) as zh:
                        text = _open_text(zh.read())
                    yield from _split_csv_text(text, label=f"{path.name}/{member}")
        except (zipfile.BadZipFile, OSError) as e:
            print(f"  ERROR: cannot open zip {path.name}: {e}")
        return

    if path.suffix.lower() == ".csv":
        try:
            with open(path, "rb") as fh:
                text = _open_text(fh.read())
        except OSError as e:
            print(f"  ERROR: cannot read {path.name}: {e}")
            return
        yield from _split_csv_text(text, label=path.name)


def _split_csv_text(text: str, label: str):
    """Yield (label, delimiter, header_list, row_iter)."""
    lines = text.splitlines()
    if not lines:
        return
    header_line = lines[0]
    # Detect delimiter
    for d in (",", ";", "\t", "|"):
        if d in header_line:
            delim = d
            break
    else:
        delim = ","
    header = [c.strip() for c in header_line.split(delim)]
    yield label, delim, header, lines[1:]

# ---------------------------------------------------------------------------
# Step 5: Aggregate Journey-to-Work commuter flows
# ---------------------------------------------------------------------------

# Tolerant header lookups — StatCan column names vary slightly across
# release vintages and language flavours.
ORIG_DA_KEYS = (
    "GEO_CODE_RES", "GEO_RES", "DAUID_RES", "DA_RES",
    "RES_DAUID", "RES_GEO_CODE", "ORIGIN_DAUID", "ORIGIN_DA",
    "GEOGRAPHY_RES", "PLACE_OF_RESIDENCE_DAUID",
)
DEST_DA_KEYS = (
    "GEO_CODE_POW", "GEO_POW", "DAUID_POW", "DA_POW",
    "POW_DAUID", "POW_GEO_CODE", "DEST_DAUID", "DEST_DA",
    "GEOGRAPHY_POW", "PLACE_OF_WORK_DAUID",
)
COUNT_KEYS = (
    "VALUE", "COUNT", "OBS_VALUE", "FLOW", "COMMUTERS",
    "C1_COUNT_TOTAL", "T_DATA_DONNEE",
)


def _find_col(header_upper: list, candidates) -> int:
    """Return first matching column index, or -1."""
    for i, name in enumerate(header_upper):
        if name in candidates:
            return i
    return -1


def aggregate_commuter_flows(da_index: dict, dry_run: bool = False):
    """
    Iterate the Journey-to-Work CSV (Table 98-400-X2021007) and accumulate
    work-trip volume into origin DA H3 cells.

    Logic:
      - origin DA = place of residence
      - dest DA   = place of work
      - keep row if dest DA is in da_index (i.e. dest within 150 km of a CA cluster)
      - map origin DA to its H3 res-7 cell and accumulate the flow count

    Returns: {h3_origin: {visits, lat, lon, clusters:set, pr:str}}
    """
    src_path = COMMUTER_FILE if COMMUTER_FILE.exists() else COMMUTER_ZIP
    if not src_path.exists():
        return None  # caller falls back to employment proxy

    print(f"Aggregating commuter flows from {src_path.name} ...")

    work_acc = collections.defaultdict(
        lambda: {"visits": 0.0, "lat": 0.0, "lon": 0.0, "clusters": set(), "pr": ""}
    )

    files_processed   = 0
    rows_processed    = 0
    rows_kept         = 0
    rows_no_dest      = 0
    rows_no_origin    = 0
    rows_bad          = 0
    dry_run_limit     = 1000
    dry_run_done      = False

    for label, delim, header, row_iter in iter_commuter_rows(src_path):
        files_processed += 1
        header_upper = [c.upper() for c in header]
        idx_orig  = _find_col(header_upper, ORIG_DA_KEYS)
        idx_dest  = _find_col(header_upper, DEST_DA_KEYS)
        idx_count = _find_col(header_upper, COUNT_KEYS)

        if idx_orig < 0 or idx_dest < 0 or idx_count < 0:
            print(
                f"  Warning: {label} has unexpected columns "
                f"(orig_idx={idx_orig}, dest_idx={idx_dest}, count_idx={idx_count})"
            )
            print(f"           Header sample: {header[:8]}")
            continue

        print(f"  Processing {label} (columns: orig={header[idx_orig]}, "
              f"dest={header[idx_dest]}, count={header[idx_count]}) ...")

        for line in row_iter:
            if not line:
                continue

            parts = line.split(delim)
            if max(idx_orig, idx_dest, idx_count) >= len(parts):
                rows_bad += 1
                continue

            orig_da = parts[idx_orig].strip().strip('"')
            dest_da = parts[idx_dest].strip().strip('"')
            count_s = parts[idx_count].strip().strip('"').replace(",", "")

            try:
                count = float(count_s)
            except ValueError:
                rows_bad += 1
                rows_processed += 1
                if dry_run and rows_processed >= dry_run_limit:
                    dry_run_done = True
                    break
                continue

            rows_processed += 1
            if count <= 0:
                if dry_run and rows_processed >= dry_run_limit:
                    dry_run_done = True
                    break
                continue

            dest_info = da_index.get(dest_da)
            if dest_info is None:
                rows_no_dest += 1
                if dry_run and rows_processed >= dry_run_limit:
                    dry_run_done = True
                    break
                continue

            orig_info = da_index.get(orig_da)
            if orig_info is None:
                rows_no_origin += 1
                if dry_run and rows_processed >= dry_run_limit:
                    dry_run_done = True
                    break
                continue

            rec = work_acc[orig_info["h3"]]
            rec["visits"] += count
            rec["lat"]     = orig_info["lat"]
            rec["lon"]     = orig_info["lon"]
            rec["pr"]      = orig_info.get("pr", "")
            rec["clusters"].update(dest_info["cluster_proximity"])
            rows_kept += 1

            if dry_run and rows_processed >= dry_run_limit:
                dry_run_done = True
                break

        if dry_run and dry_run_done:
            break

    print(
        f"  Commuter aggregation: {files_processed} file(s), "
        f"{rows_processed:,} rows read, {rows_kept:,} kept, "
        f"{rows_no_dest:,} dest outside envelope, "
        f"{rows_no_origin:,} origin outside envelope, "
        f"{rows_bad:,} malformed."
    )
    return work_acc

# ---------------------------------------------------------------------------
# Step 5b: Fallback — employment-by-DA proxy
# ---------------------------------------------------------------------------

EMP_DA_KEYS = ("DGUID", "GEO_CODE", "GEOGRAPHY", "DAUID", "GEO_NAME")
EMP_VALUE_KEYS = ("VALUE", "OBS_VALUE", "T_DATA_DONNEE", "COUNT")


def aggregate_employment_proxy(da_index: dict, dry_run: bool = False):
    """
    Fallback when the commuter-flow table is unavailable.

    Loads Table 98-10-0494-01 / 98100494 (place of work status by CMA/CT)
    and treats the employment count at each in-envelope geography as a
    *work-reach proxy*. This is not true origin-destination and is
    CMA/CT-level, not DA-level — granularity is coarser than the primary
    table. Workers are represented at their workplace geography, not their
    residence DA. Records carry data_source="statcan_employment_proxy"
    and is_measured=False.
    """
    src_path = EMPLOYMENT_FILE if EMPLOYMENT_FILE.exists() else EMPLOYMENT_ZIP
    if not src_path.exists():
        return None

    print(
        "FALLBACK: Journey-to-Work commuter table unavailable.\n"
        f"  Using employment-by-DA proxy from {src_path.name}.\n"
        "  Note: workers will be attributed to their workplace DA, not\n"
        "        their residence DA. Records flagged is_measured=False."
    )

    proxy_acc = collections.defaultdict(
        lambda: {"visits": 0.0, "lat": 0.0, "lon": 0.0, "clusters": set(), "pr": ""}
    )

    rows_processed = 0
    rows_kept = 0
    dry_run_limit = 1000

    for label, delim, header, row_iter in iter_commuter_rows(src_path):
        header_upper = [c.upper() for c in header]
        idx_da    = _find_col(header_upper, EMP_DA_KEYS)
        idx_value = _find_col(header_upper, EMP_VALUE_KEYS)

        if idx_da < 0 or idx_value < 0:
            print(f"  Warning: {label} unexpected columns; header: {header[:8]}")
            continue

        for line in row_iter:
            if not line:
                continue
            parts = line.split(delim)
            if max(idx_da, idx_value) >= len(parts):
                continue
            raw_da = parts[idx_da].strip().strip('"')
            # DGUID is e.g. "2021S0512..." — DAUID is trailing 8 digits.
            dauid = raw_da[-8:] if raw_da.isdigit() is False else raw_da
            try:
                count = float(parts[idx_value].strip().strip('"').replace(",", ""))
            except ValueError:
                continue

            rows_processed += 1
            if count <= 0:
                if dry_run and rows_processed >= dry_run_limit:
                    break
                continue

            info = da_index.get(dauid)
            if info is None:
                if dry_run and rows_processed >= dry_run_limit:
                    break
                continue

            rec = proxy_acc[info["h3"]]
            rec["visits"] += count
            rec["lat"]     = info["lat"]
            rec["lon"]     = info["lon"]
            rec["pr"]      = info.get("pr", "")
            rec["clusters"].update(info["cluster_proximity"])
            rows_kept += 1

            if dry_run and rows_processed >= dry_run_limit:
                break

        if dry_run and rows_processed >= dry_run_limit:
            break

    print(f"  Employment proxy: {rows_processed:,} rows, {rows_kept:,} kept.")
    return proxy_acc

# ---------------------------------------------------------------------------
# Step 6: Output writers
# ---------------------------------------------------------------------------

def write_output(work_acc: dict, data_source: str, is_measured: bool) -> None:
    """
    Write per-H3 JSONL output and a per-cluster summary JSONL.
    """
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    # Per-H3 origin records
    per_h3_count = 0
    cluster_totals = collections.defaultdict(lambda: {"visits": 0.0, "cells": 0})

    with open(OUTPUT_WORK, "w", encoding="utf-8") as f:
        for h3_cell, rec in sorted(work_acc.items()):
            record = {
                "h3": h3_cell,
                "lat": round(rec["lat"], 6),
                "lon": round(rec["lon"], 6),
                "iso": "CA",
                "visits_work_total": round(rec["visits"], 2),
                "cluster_proximity": sorted(rec["clusters"]),
                "data_source": data_source,
                "is_measured": is_measured,
            }
            if rec.get("pr"):
                record["pr"] = rec["pr"]
            f.write(json.dumps(record) + "\n")
            per_h3_count += 1

            for cid in rec["clusters"]:
                cluster_totals[cid]["visits"] += rec["visits"]
                cluster_totals[cid]["cells"]  += 1

    print(f"  Per-H3 work records:  {per_h3_count:,} → {OUTPUT_WORK}")

    # Per-cluster summary
    with open(OUTPUT_SUMMARY, "w", encoding="utf-8") as f:
        for cid in sorted(cluster_totals):
            t = cluster_totals[cid]
            f.write(json.dumps({
                "cluster_id":         cid,
                "iso":                "CA",
                "h3_origin_cells":    t["cells"],
                "visits_work_total":  round(t["visits"], 2),
                "data_source":        data_source,
                "is_measured":        is_measured,
            }) + "\n")

    print(f"  Per-cluster summary:  {len(cluster_totals):,} → {OUTPUT_SUMMARY}")

# ---------------------------------------------------------------------------
# Step 7: Dry-run statistics
# ---------------------------------------------------------------------------

def print_dry_run_stats(work_acc: dict, data_source: str) -> None:
    print("\n--- DRY-RUN STATISTICS (first 1000 data rows, no files written) ---")
    print(f"  Data source: {data_source}")
    print(f"  Unique H3 origin cells with WORK signal : {len(work_acc):,}")
    if work_acc:
        total = sum(r["visits"] for r in work_acc.values())
        print(f"  Total work trip-volume accumulated      : {total:,.1f}")
        sample_h3 = next(iter(work_acc))
        rec = work_acc[sample_h3]
        print("\n  Sample WORK record:")
        print(f"    h3={sample_h3}, lat={rec['lat']}, lon={rec['lon']}, "
              f"visits={rec['visits']:.1f}, "
              f"clusters={sorted(rec['clusters'])[:3]}...")
    print("-------------------------------------------------------------------\n")

# ---------------------------------------------------------------------------
# Main entry point
# ---------------------------------------------------------------------------

def _parse_provinces(arg: str):
    if not arg:
        return None
    return {p.strip().upper() for p in arg.split(",") if p.strip()}


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Ingest StatCan 2021 Census commuter flows → "
                    "H3 res-7 work-reach origins for Canadian co-location clusters.",
    )
    parser.add_argument(
        "--dry-run", action="store_true",
        help="Process first 1000 rows, print stats, write no output.",
    )
    parser.add_argument(
        "--province", type=str, default="",
        help="Comma-separated list of province codes to retain "
             "(e.g. BC,ON,AB). Applied to DA centroids.",
    )
    args = parser.parse_args()

    if args.dry_run:
        print("=== ingest-statcan.py (DRY RUN — first 1000 rows, no output written) ===")
    else:
        print("=== ingest-statcan.py: Statistics Canada 2021 commuter ingest ===")

    RAW_STATCAN_DIR.mkdir(parents=True, exist_ok=True)
    provinces = _parse_provinces(args.province)

    # ---- Downloads (with on-disk cache) -----------------------------------
    print("\n[1/5] Acquiring source files ...")
    have_centroids = download_with_progress(
        CENTROIDS_URL, CENTROIDS_FILE, "DA centroids (CenPop2021_Mean_DA)"
    )
    if not have_centroids:
        print(DOWNLOAD_INSTRUCTIONS)
        sys.exit(1)

    def _valid_zip(p: Path) -> bool:
        return p.exists() and p.stat().st_size > 10_000  # <10 KB = HTML error page

    have_commuter = _valid_zip(COMMUTER_FILE) or _valid_zip(COMMUTER_ZIP)
    if not have_commuter:
        have_commuter = (
            download_with_progress(COMMUTER_URL, COMMUTER_ZIP, "Journey-to-Work (98-400-X2021007)")
            and _valid_zip(COMMUTER_ZIP)
        )
        if not have_commuter:
            print("  Journey-to-Work bulk download unavailable; trying employment fallback ...")

    have_employment = _valid_zip(EMPLOYMENT_FILE) or _valid_zip(EMPLOYMENT_ZIP)
    if not have_employment and not have_commuter:
        have_employment = download_with_progress(
            EMPLOYMENT_URL, EMPLOYMENT_ZIP, "Place of work status by CT (98100494 / 98-10-0494-01)"
        ) and _valid_zip(EMPLOYMENT_ZIP)

    if not (have_commuter or have_employment):
        print(DOWNLOAD_INSTRUCTIONS)
        sys.exit(1)

    # ---- Cluster proximity index ------------------------------------------
    print("\n[2/5] Loading Canadian clusters ...")
    _canadian, buckets, threshold_km = load_canadian_clusters(threshold_km=150.0)

    # ---- DA centroids and spatial index ----------------------------------
    print("\n[3/5] Loading DA centroids ...")
    centroids = load_da_centroids(provinces_filter=provinces)
    if not centroids:
        print("ERROR: no DA centroids loaded.")
        sys.exit(1)

    print("\n[4/5] Building DA → H3 + cluster-proximity index ...")
    da_index = build_da_index(centroids, buckets, threshold_km)
    if not da_index:
        print(
            "ERROR: no DAs fall within 150 km of any Canadian cluster.\n"
            "       Check clusters-meta.json coverage."
        )
        sys.exit(1)

    # ---- Aggregate flows --------------------------------------------------
    print("\n[5/5] Aggregating work-trip flows ...")
    work_acc = None
    data_source = "statcan_2021"
    is_measured = True

    if have_commuter:
        work_acc = aggregate_commuter_flows(da_index, dry_run=args.dry_run)

    if not work_acc:
        # Empty result OR primary file missing → try fallback
        if have_employment:
            work_acc = aggregate_employment_proxy(da_index, dry_run=args.dry_run)
            data_source = "statcan_employment_proxy"
            is_measured = False
        else:
            print(
                "ERROR: commuter table yielded no rows and no employment\n"
                "       fallback table is present."
            )
            sys.exit(1)

    if not work_acc:
        print("WARNING: No qualifying rows accumulated. Output not written.")
        return

    # ---- Output / dry-run summary ----------------------------------------
    if args.dry_run:
        print_dry_run_stats(work_acc, data_source)
        return

    print(f"\nWriting JSONL output (data_source={data_source}, is_measured={is_measured}) ...")
    write_output(work_acc, data_source=data_source, is_measured=is_measured)
    print("ingest-statcan.py complete.")


if __name__ == "__main__":
    main()
