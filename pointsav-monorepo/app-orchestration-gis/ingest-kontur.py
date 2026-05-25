#!/usr/bin/env python3
"""
ingest-kontur.py — Kontur Population res-8 GeoPackage → census-h3-res7.jsonl

Reads per-country kontur_population_{ISO2}_{date}.gpkg.gz files, rolls up
H3 res-8 cells to res-7, and writes census-h3-res7.jsonl — identical schema
to the WorldPop-derived file read by synthesize-od-study.py.

No fiona required: GeoPackage is SQLite; read directly with stdlib sqlite3.

Usage:
    python3 ingest-kontur.py            # all available countries
    python3 ingest-kontur.py --dry-run  # first country only, print stats

Output: /srv/foundry/deployments/cluster-totebox-personnel-1/
            service-fs/service-census/census-h3-res7.jsonl
"""

import argparse
import gzip
import json
import math
import shutil
import sqlite3
import tempfile
from collections import defaultdict
from pathlib import Path

import h3

KONTUR_RAW = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-census/kontur-raw"
)
OUTPUT_FILE = Path(
    "/srv/foundry/deployments/cluster-totebox-personnel-1"
    "/service-fs/service-census/census-h3-res7.jsonl"
)
H3_TARGET_RES = 7

# ISO2 → filename mapping (matches downloaded files)
ISO2_FILES = {
    "US": "kontur_population_US_20231101.gpkg.gz",
    "CA": "kontur_population_CA_20231101.gpkg.gz",
    "MX": "kontur_population_MX_20231101.gpkg.gz",
    "ES": "kontur_population_ES_20231101.gpkg.gz",
    "FR": "kontur_population_FR_20231101.gpkg.gz",
    "DE": "kontur_population_DE_20231101.gpkg.gz",
    "GB": "kontur_population_GB_20231101.gpkg.gz",
    "IT": "kontur_population_IT_20231101.gpkg.gz",
    "NL": "kontur_population_NL_20231101.gpkg.gz",
    "AT": "kontur_population_AT_20231101.gpkg.gz",
    "PL": "kontur_population_PL_20231101.gpkg.gz",
    "GR": "kontur_population_GR_20231101.gpkg.gz",
    "PT": "kontur_population_PT_20231101.gpkg.gz",
}


def rollup_country(iso2: str, gpkg_gz: Path, hex_data: dict) -> int:
    with tempfile.NamedTemporaryFile(suffix=".gpkg", delete=False) as tmp:
        tmppath = Path(tmp.name)
    try:
        with gzip.open(gpkg_gz, "rb") as f_in, open(tmppath, "wb") as f_out:
            shutil.copyfileobj(f_in, f_out)
        conn = sqlite3.connect(tmppath)
        rows = conn.execute("SELECT h3, population FROM population WHERE population > 0").fetchall()
        conn.close()
    finally:
        tmppath.unlink(missing_ok=True)

    count = 0
    for h8, pop in rows:
        if not pop:
            continue
        h7 = h3.cell_to_parent(h8, H3_TARGET_RES)
        hex_data[h7]["pop"] += pop
        hex_data[h7]["isos"].add(iso2)
        count += 1
    return count


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dry-run", action="store_true", help="First country only")
    args = parser.parse_args()

    hex_data: dict = defaultdict(lambda: {"pop": 0.0, "isos": set()})
    items = list(ISO2_FILES.items())
    if args.dry_run:
        items = items[:1]

    for iso2, fname in items:
        path = KONTUR_RAW / fname
        if not path.exists():
            print(f"  SKIP {iso2}: {fname} not found")
            continue
        print(f"  {iso2}: decompressing + reading ...", end="", flush=True)
        n = rollup_country(iso2, path, hex_data)
        print(f" {n:,} res-8 cells → {len(hex_data):,} res-7 cells so far")

    print(f"\nWriting {len(hex_data):,} res-7 cells → {OUTPUT_FILE.name} ...")
    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_FILE, "w") as out:
        for h7, data in hex_data.items():
            lat, lon = h3.cell_to_latlng(h7)
            out.write(json.dumps({
                "h3":  h7,
                "lat": round(lat, 5),
                "lon": round(lon, 5),
                "pop": round(data["pop"], 2),
                "iso": sorted(data["isos"]),
            }) + "\n")

    size_mb = OUTPUT_FILE.stat().st_size / 1024 / 1024
    print(f"Done. {len(hex_data):,} cells written ({size_mb:.1f} MB).")
    if args.dry_run:
        print("(dry-run: first country only — re-run without --dry-run for full ingest)")


if __name__ == "__main__":
    main()
