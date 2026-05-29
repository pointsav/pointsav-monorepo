"""
ingest-lodes.py — US Census LODES (LEHD) Origin-Destination ingest
====================================================================
Downloads US Census LEHD LODES8 OD main + block crosswalk files for all
50 states + DC, and produces WORK-reach H3 res-7 origin distributions
for US co-location clusters.

The LODES OD-main table describes job-counts by home-block × work-block
(S000 = total primary jobs). For each US cluster, this script:

  1. Identifies work blocks (w_geocode) whose centroid lies within 150 km
     of the cluster.
  2. For OD records terminating in those work blocks, accumulates the
     home-block trip counts (S000) onto H3 res-7 cells (using the block
     crosswalk lat/lon as the home-block centroid).
  3. Records cluster_proximity (the set of clusters the worker reaches).

Source:
  https://lehd.ces.census.gov/data/lodes/LODES8/

Files per state (2-letter lowercase abbreviation, e.g. 'al', 'tx'):
  od main:  https://lehd.ces.census.gov/data/lodes/LODES8/{st}/od/{st}_od_main_JT00_{year}.csv.gz
            Columns: w_geocode, h_geocode, S000, ... (15-digit FIPS block IDs)
  xwalk:    https://lehd.ces.census.gov/data/lodes/LODES8/{st}/{st}_xwalk.csv.gz
            Columns: tabblk2020, blklatdd, blklondd, ... (block centroids)

Output schema (one record per H3 res-7 cell, written to JSONL):
  {
    "h3": str,
    "lat": float, "lon": float,
    "iso": "US",
    "visits_work_total": float,
    "cluster_proximity": [cluster_id, ...],
    "data_source": "lodes_2021",
    "is_measured": true
  }

A per-cluster summary file is also written:
  {
    "cluster_id": str,
    "total_work_reach_35km":  float,
    "total_work_reach_150km": float
  }

Usage:
  python3 ingest-lodes.py                       # full ingest, all 50 + DC
  python3 ingest-lodes.py --dry-run             # first state only, first 10k OD rows
  python3 ingest-lodes.py --states AL,TX,CA     # subset of states
  python3 ingest-lodes.py --year 2021           # year selector (default 2021)

Dependencies (all stdlib + h3):
  h3, gzip, urllib.request, csv, json, math, pathlib, sys, collections, argparse
"""

import argparse
import collections
import csv
import gzip
import io
import json
import math
import sys
import urllib.error
import urllib.request
from pathlib import Path

# ---------------------------------------------------------------------------
# Path constants
# ---------------------------------------------------------------------------

CLUSTERS_META = (
    "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
)
RAW_LODES_DIR = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-mobility/raw/lodes"
)
OUTPUT_DIR = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-mobility"
)
OUTPUT_WORK = OUTPUT_DIR / "lodes-work-od-us.jsonl"
OUTPUT_SUMMARY = OUTPUT_DIR / "lodes-work-summary-us.jsonl"

# H3 resolution used throughout the pipeline
H3_RES = 7

# Distance bands
PRIMARY_KM   = 35.0
SECONDARY_KM = 150.0

# LODES portal base
LODES_BASE = "https://lehd.ces.census.gov/data/lodes/LODES8"

# Default year (LODES8 supports multiple vintages; 2021 is the most recent
# fully-released year as of pipeline build).
DEFAULT_YEAR = 2021

# Job-type code: JT00 = all jobs (primary + secondary).
JOB_TYPE = "JT00"

# All 50 states + DC (FIPS 2-letter lowercase abbreviations).
STATES = [
    "al", "ak", "az", "ar", "ca", "co", "ct", "de", "dc", "fl",
    "ga", "hi", "id", "il", "in", "ia", "ks", "ky", "la", "me",
    "md", "ma", "mi", "mn", "ms", "mo", "mt", "ne", "nv", "nh",
    "nj", "nm", "ny", "nc", "nd", "oh", "ok", "or", "pa", "ri",
    "sc", "sd", "tn", "tx", "ut", "vt", "va", "wa", "wv", "wi",
    "wy",
]

# Progress print cadence (OD rows).
PROGRESS_EVERY = 500_000

# ---------------------------------------------------------------------------
# H3 import guard
# ---------------------------------------------------------------------------

try:
    import h3
except ImportError:
    print("ERROR: h3 library not available. Install with: pip install h3")
    print("       This pipeline requires h3 >= 3.7.")
    sys.exit(1)

# h3 v3 / v4 compatibility shim (mirrors other ingest scripts).
if hasattr(h3, "latlng_to_cell"):
    _h3_from_latlng = h3.latlng_to_cell
else:
    _h3_from_latlng = h3.latlng_to_cell

# ---------------------------------------------------------------------------
# Utility: haversine distance (km)
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
# Step 1: Load US clusters + build bucket index
# ---------------------------------------------------------------------------

def load_us_clusters(threshold_km: float = SECONDARY_KM):
    """
    Load all US clusters from clusters-meta.json.
    Returns:
      us_list  — list of {id, lat, lon}
      buckets  — defaultdict((int_lon, int_lat) → [cluster, ...]) for O(1) proximity
      threshold_km — passed through
    """
    if not Path(CLUSTERS_META).exists():
        print(f"ERROR: clusters-meta.json not found at {CLUSTERS_META}")
        sys.exit(1)

    with open(CLUSTERS_META, "r") as f:
        data = json.load(f)

    us = []
    for c in data:
        iso = c.get("iso", "")
        if isinstance(iso, list):
            iso_match = "US" in iso
        else:
            iso_match = iso == "US"
        if iso_match:
            us.append({"id": c["id"], "lat": c["lat"], "lon": c["lon"]})

    if not us:
        print("ERROR: No US clusters (iso=US) found in clusters-meta.json")
        sys.exit(1)

    print(f"Loaded {len(us)} US clusters from clusters-meta.json.")

    # Bucket index for proximity lookup. Buckets are 1°×1° (≈111 km at the
    # equator, less at higher latitudes — buffer compensates).
    buffer = math.ceil(threshold_km / 50.0)
    if buffer < 2:
        buffer = 2

    buckets = collections.defaultdict(list)
    for c in us:
        lon_i = int(c["lon"])
        lat_i = int(c["lat"])
        for i in range(lon_i - buffer, lon_i + buffer + 1):
            for j in range(lat_i - buffer, lat_i + buffer + 1):
                buckets[(i, j)].append(c)

    return us, buckets, threshold_km


def nearby_clusters(lon: float, lat: float, buckets, threshold_km: float):
    """
    Return list of cluster IDs within threshold_km of (lon, lat).
    Uses the bucket strategy from ingest-mitma.py.
    """
    key = (int(lon), int(lat))
    candidates = buckets.get(key, [])
    result = []
    for c in candidates:
        if haversine_km(lon, lat, c["lon"], c["lat"]) <= threshold_km:
            result.append(c["id"])
    return result


def nearby_clusters_with_dist(lon: float, lat: float, buckets, threshold_km: float):
    """
    Return list of (cluster_id, distance_km) tuples within threshold_km.
    Used for primary/secondary banding in the summary file.
    """
    key = (int(lon), int(lat))
    candidates = buckets.get(key, [])
    result = []
    for c in candidates:
        d = haversine_km(lon, lat, c["lon"], c["lat"])
        if d <= threshold_km:
            result.append((c["id"], d))
    return result

# ---------------------------------------------------------------------------
# Step 2: Lazy file downloader (urllib.request, stdlib only)
# ---------------------------------------------------------------------------

def _download(url: str, dest: Path) -> bool:
    """
    Download `url` to `dest`. Returns True on success, False on HTTP error
    (logged as a warning — caller decides whether to skip the state).

    Files are downloaded gzipped (the LODES portal serves them that way)
    and stay gzipped on disk; readers use gzip.open() directly.
    """
    if dest.exists() and dest.stat().st_size > 0:
        return True

    dest.parent.mkdir(parents=True, exist_ok=True)
    tmp = dest.with_suffix(dest.suffix + ".part")

    print(f"  Downloading {url}")
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "foundry-gis-pipeline/1.0"})
        with urllib.request.urlopen(req, timeout=120) as resp:
            total = resp.getheader("Content-Length")
            try:
                total = int(total) if total else 0
            except ValueError:
                total = 0

            bytes_read = 0
            chunk = 64 * 1024
            with open(tmp, "wb") as out:
                while True:
                    buf = resp.read(chunk)
                    if not buf:
                        break
                    out.write(buf)
                    bytes_read += len(buf)
                    if total:
                        pct = 100.0 * bytes_read / total
                        # Print sparingly: every ~5 MB
                        if bytes_read % (5 * 1024 * 1024) < chunk:
                            print(f"    {bytes_read / 1e6:8.1f} MB / "
                                  f"{total / 1e6:8.1f} MB ({pct:5.1f}%)")
        tmp.rename(dest)
        size_mb = dest.stat().st_size / 1e6
        print(f"    -> {dest.name} ({size_mb:.1f} MB)")
        return True

    except urllib.error.HTTPError as e:
        print(f"  Warning: HTTP {e.code} for {url} — skipping.")
        if tmp.exists():
            tmp.unlink()
        return False
    except urllib.error.URLError as e:
        print(f"  Warning: URL error for {url}: {e.reason} — skipping.")
        if tmp.exists():
            tmp.unlink()
        return False
    except OSError as e:
        print(f"  Warning: I/O error downloading {url}: {e} — skipping.")
        if tmp.exists():
            tmp.unlink()
        return False


def fetch_state_files(state: str, year: int):
    """
    Ensure both the OD-main and crosswalk gzipped CSVs are cached locally.
    Returns (od_path, xwalk_path) on success, or (None, None) on failure
    (any HTTP error for either file).
    """
    state_dir = RAW_LODES_DIR / state
    od_url    = f"{LODES_BASE}/{state}/od/{state}_od_main_{JOB_TYPE}_{year}.csv.gz"
    xwalk_url = f"{LODES_BASE}/{state}/{state}_xwalk.csv.gz"
    od_path    = state_dir / f"{state}_od_main_{JOB_TYPE}_{year}.csv.gz"
    xwalk_path = state_dir / f"{state}_xwalk.csv.gz"

    if not _download(od_url, od_path):
        return None, None
    if not _download(xwalk_url, xwalk_path):
        return None, None

    return od_path, xwalk_path

# ---------------------------------------------------------------------------
# Step 3: Block crosswalk loader → {block_fips: (lat, lon)}
# ---------------------------------------------------------------------------

def load_block_xwalk(xwalk_path: Path) -> dict:
    """
    Parse the state crosswalk and return {tabblk2020: (lat, lon)}.

    LODES crosswalks contain >40 columns; we only need the block ID and
    its centroid. Coordinates are decimal degrees in WGS84.
    """
    blocks = {}
    rows = 0
    with gzip.open(xwalk_path, "rt", encoding="utf-8", errors="replace", newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            rows += 1
            blk = row.get("tabblk2020") or row.get("tabblk")
            if not blk:
                continue
            try:
                lat = float(row.get("blklatdd", ""))
                lon = float(row.get("blklondd", ""))
            except ValueError:
                continue
            blocks[blk] = (lat, lon)
    print(f"    crosswalk: {rows:,} rows, {len(blocks):,} blocks with centroids.")
    return blocks

# ---------------------------------------------------------------------------
# Step 4: Process a single state's OD-main file
# ---------------------------------------------------------------------------

def process_state(
    state: str,
    od_path: Path,
    xwalk_path: Path,
    buckets,
    threshold_km: float,
    work_acc: dict,
    summary_acc: dict,
    dry_run_rows: int = 0,
):
    """
    Accumulate work-reach signals from one state's OD-main file.

    Mutates `work_acc` (keyed by home-block H3 cell) and `summary_acc`
    (keyed by cluster_id).

    work_acc[h3] = {"visits": float, "lat": float, "lon": float, "clusters": set}
    summary_acc[cid] = {"work_35km": float, "work_150km": float}

    Returns a per-state summary dict:
      {"od_rows": int, "kept": int, "h3_cells_new": int, "clusters_updated": int}
    """
    print(f"  Loading block crosswalk for {state.upper()} ...")
    blocks = load_block_xwalk(xwalk_path)
    if not blocks:
        print(f"  Warning: empty crosswalk for {state}; skipping.")
        return {"od_rows": 0, "kept": 0, "h3_cells_new": 0, "clusters_updated": 0}

    # Pre-compute the set of WORK blocks within threshold_km of any US cluster,
    # mapping each to its proximate cluster_ids. Blocks outside the envelope
    # can be skipped at the OD-row level in O(1).
    print(f"  Indexing work-block proximity (≤{threshold_km:.0f} km) ...")
    work_block_clusters = {}    # w_block_fips → [cluster_ids within 150 km]
    work_block_dist     = {}    # w_block_fips → [(cluster_id, distance_km)]
    for blk, (lat, lon) in blocks.items():
        nearby = nearby_clusters_with_dist(lon, lat, buckets, threshold_km)
        if nearby:
            work_block_clusters[blk] = [cid for cid, _ in nearby]
            work_block_dist[blk] = nearby
    print(f"    {len(work_block_clusters):,} of {len(blocks):,} blocks within envelope.")

    if not work_block_clusters:
        print(f"  No US clusters within {threshold_km:.0f} km of any {state.upper()} block; "
              f"skipping OD scan.")
        return {"od_rows": 0, "kept": 0, "h3_cells_new": 0, "clusters_updated": 0}

    h3_cells_before = len(work_acc)
    clusters_before = set(summary_acc.keys())
    od_rows = 0
    kept    = 0

    print(f"  Streaming OD file: {od_path.name}")
    with gzip.open(od_path, "rt", encoding="utf-8", errors="replace", newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            od_rows += 1

            if dry_run_rows and od_rows > dry_run_rows:
                break

            if od_rows % PROGRESS_EVERY == 0:
                print(f"    {od_rows:>12,} rows scanned, {kept:>10,} kept ...")

            w = row.get("w_geocode")
            if not w:
                continue
            cluster_ids = work_block_clusters.get(w)
            if cluster_ids is None:
                continue

            h = row.get("h_geocode")
            if not h:
                continue
            home_centroid = blocks.get(h)
            if home_centroid is None:
                continue

            try:
                jobs = float(row.get("S000", "0"))
            except ValueError:
                continue
            if jobs <= 0:
                continue

            home_lat, home_lon = home_centroid
            h3_cell = _h3_from_latlng(home_lat, home_lon, H3_RES)

            rec = work_acc.get(h3_cell)
            if rec is None:
                rec = {
                    "visits": 0.0,
                    "lat": home_lat,
                    "lon": home_lon,
                    "clusters": set(),
                }
                work_acc[h3_cell] = rec
            rec["visits"] += jobs
            rec["clusters"].update(cluster_ids)

            # Per-cluster summary — banded by primary / secondary distance
            # measured from the cluster centroid to the WORK block centroid.
            for cid, dist_km in work_block_dist[w]:
                s = summary_acc.get(cid)
                if s is None:
                    s = {"work_35km": 0.0, "work_150km": 0.0}
                    summary_acc[cid] = s
                if dist_km <= PRIMARY_KM:
                    s["work_35km"] += jobs
                s["work_150km"] += jobs

            kept += 1

    h3_cells_new = len(work_acc) - h3_cells_before
    clusters_updated = len(set(summary_acc.keys()) - clusters_before)

    print(f"  {state.upper()} done: {od_rows:,} OD rows, {kept:,} kept, "
          f"{h3_cells_new:,} new H3 cells, {clusters_updated:,} new clusters touched.")

    return {
        "od_rows": od_rows,
        "kept": kept,
        "h3_cells_new": h3_cells_new,
        "clusters_updated": clusters_updated,
    }

# ---------------------------------------------------------------------------
# Step 5: Write output JSONL files
# ---------------------------------------------------------------------------

def write_output(work_acc: dict, summary_acc: dict, year: int) -> None:
    """
    Serialize work_acc to OUTPUT_WORK and summary_acc to OUTPUT_SUMMARY.
    """
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    data_source = f"lodes_{year}"

    work_count = 0
    with open(OUTPUT_WORK, "w", encoding="utf-8") as f:
        for h3_cell, rec in sorted(work_acc.items()):
            record = {
                "h3": h3_cell,
                "lat": round(rec["lat"], 6),
                "lon": round(rec["lon"], 6),
                "iso": "US",
                "visits_work_total": round(rec["visits"], 2),
                "cluster_proximity": sorted(rec["clusters"]),
                "data_source": data_source,
                "is_measured": True,
            }
            f.write(json.dumps(record) + "\n")
            work_count += 1
    print(f"  Work OD: {work_count:,} H3 origin cells → {OUTPUT_WORK}")

    summary_count = 0
    with open(OUTPUT_SUMMARY, "w", encoding="utf-8") as f:
        for cid, s in sorted(summary_acc.items()):
            record = {
                "cluster_id": cid,
                "total_work_reach_35km":  round(s["work_35km"], 2),
                "total_work_reach_150km": round(s["work_150km"], 2),
            }
            f.write(json.dumps(record) + "\n")
            summary_count += 1
    print(f"  Cluster summary: {summary_count:,} clusters → {OUTPUT_SUMMARY}")

# ---------------------------------------------------------------------------
# Step 6: Dry-run statistics printer
# ---------------------------------------------------------------------------

def print_dry_run_stats(work_acc: dict, summary_acc: dict) -> None:
    print("\n--- DRY-RUN STATISTICS (first state, first 10k OD rows, no files written) ---")
    print(f"  Unique H3 origin cells with WORK signal : {len(work_acc):,}")
    print(f"  Clusters touched                        : {len(summary_acc):,}")
    if work_acc:
        total_work = sum(r["visits"] for r in work_acc.values())
        print(f"  Total work trip-volume accumulated      : {total_work:,.1f}")
        sample_h3 = next(iter(work_acc))
        rec = work_acc[sample_h3]
        print(f"\n  Sample WORK record:")
        print(f"    h3={sample_h3}, lat={rec['lat']:.4f}, lon={rec['lon']:.4f}, "
              f"visits={rec['visits']:.1f}, clusters={sorted(rec['clusters'])[:3]}...")
    if summary_acc:
        sample_cid = next(iter(summary_acc))
        s = summary_acc[sample_cid]
        print(f"\n  Sample SUMMARY record:")
        print(f"    cluster_id={sample_cid}, "
              f"work_35km={s['work_35km']:.1f}, work_150km={s['work_150km']:.1f}")
    print("------------------------------------------------------------------------------\n")

# ---------------------------------------------------------------------------
# CLI + main
# ---------------------------------------------------------------------------

def parse_args(argv):
    p = argparse.ArgumentParser(
        description="Ingest US Census LODES OD data into H3 res-7 work-reach JSONL."
    )
    p.add_argument("--dry-run", action="store_true",
                   help="Process first state only, first 10,000 OD rows, no output written.")
    p.add_argument("--states", default="",
                   help="Comma-separated 2-letter state codes (e.g. AL,TX,CA). "
                        "Default: all 50 states + DC.")
    p.add_argument("--year", type=int, default=DEFAULT_YEAR,
                   help=f"LODES vintage year (default {DEFAULT_YEAR}).")
    return p.parse_args(argv)


def resolve_states(raw: str):
    """Validate and lower-case the --states selector against STATES."""
    if not raw:
        return list(STATES)
    requested = [s.strip().lower() for s in raw.split(",") if s.strip()]
    unknown = [s for s in requested if s not in STATES]
    if unknown:
        print(f"ERROR: unknown state code(s): {unknown}")
        print(f"       Valid codes: {', '.join(STATES)}")
        sys.exit(1)
    return requested


def main(argv=None) -> None:
    args = parse_args(argv if argv is not None else sys.argv[1:])
    dry_run = args.dry_run
    year = args.year
    states = resolve_states(args.states)

    if dry_run:
        print("=== ingest-lodes.py (DRY RUN — first state, first 10k rows, no output) ===")
        states = states[:1]
        dry_run_rows = 10_000
    else:
        print("=== ingest-lodes.py: US Census LODES work-reach ingest ===")
        dry_run_rows = 0

    print(f"States to process: {len(states)} → {', '.join(s.upper() for s in states)}")
    print(f"Year: {year}    Job type: {JOB_TYPE}    H3 resolution: {H3_RES}")
    print(f"Raw cache: {RAW_LODES_DIR}")

    # Step 1: load US clusters and build proximity buckets
    _us_clusters, buckets, threshold_km = load_us_clusters(threshold_km=SECONDARY_KM)

    RAW_LODES_DIR.mkdir(parents=True, exist_ok=True)

    # Step 2–4: process state-by-state, accumulating in-memory.
    work_acc    = {}
    summary_acc = {}

    states_done   = 0
    states_failed = []

    for st in states:
        print(f"\n--- {st.upper()} ---")
        od_path, xwalk_path = fetch_state_files(st, year)
        if od_path is None:
            states_failed.append(st.upper())
            continue

        try:
            process_state(
                state=st,
                od_path=od_path,
                xwalk_path=xwalk_path,
                buckets=buckets,
                threshold_km=threshold_km,
                work_acc=work_acc,
                summary_acc=summary_acc,
                dry_run_rows=dry_run_rows,
            )
        except (OSError, EOFError, gzip.BadGzipFile) as e:
            print(f"  ERROR processing {st.upper()}: {e} — continuing with next state.")
            states_failed.append(st.upper())
            continue

        states_done += 1

    print(f"\nStates processed: {states_done}/{len(states)}")
    if states_failed:
        print(f"States skipped/failed: {', '.join(states_failed)}")

    if dry_run:
        print_dry_run_stats(work_acc, summary_acc)
        return

    if not work_acc:
        print(
            "WARNING: No qualifying OD rows found. Output files not written.\n"
            "         Check that LODES data exists for the requested year.\n"
            f"         Raw cache: {RAW_LODES_DIR}"
        )
        return

    print("\nWriting output JSONL files...")
    write_output(work_acc, summary_acc, year)
    print("ingest-lodes.py complete.")


if __name__ == "__main__":
    main()
