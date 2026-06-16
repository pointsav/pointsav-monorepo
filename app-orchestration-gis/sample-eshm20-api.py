#!/usr/bin/env python3
"""
sample-eshm20-api.py — Populate seismic_pga for EU clusters via EFEHR API

Queries the EFEHR Seismic Hazard Harmonized Endpoints API for each EU cluster
centroid and patches the `seismic_pga` field in clusters-meta.json in-place.
Replaces the broken ESHM20 tarball approach (build-aec-seismic.sh step [4/9]).

Usage:
    python3 sample-eshm20-api.py --dry-run          # test one cluster; no writes
    python3 sample-eshm20-api.py                    # patch all EU clusters
    python3 sample-eshm20-api.py --countries DE FR  # limit to specific countries
    python3 sample-eshm20-api.py --help

API endpoint (no auth required):
    https://maps.efehr.org/api/v1/calc/disaggregation?
        lon={lon}&lat={lat}&rp=475&imt=PGA&vs30=760

    Returns JSON. PGA value in `mean_hazard` field (g units).
    Return period 475yr corresponds to 10% probability in 50 years (standard seismic design).
    vs30=760 = rock site (NEHRP class B/C boundary).

Fallback (if API unreachable):
    Use GSHAP global raster:
    https://www.gfz.de/en/section/seismic-hazard-and-risk-dynamics/data-products-services/gshap/
    The GSHAP GeoTIFF covers all countries but uses 1999 data.
    To use: install rasterio, sample with gdallocationinfo or rasterio.
"""

import argparse
import json
import os
import sys
import time
import urllib.request
import urllib.error
from pathlib import Path

META_PATH = (
    "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
)

EU_ISOS = {"DE", "FR", "GB", "IT", "ES", "PL", "AT", "NL", "SE", "DK", "NO", "FI", "IS", "GR", "PT"}

# EFEHR API — no authentication required
EFEHR_BASE = "https://maps.efehr.org/api/v1/calc/disaggregation"
RATE_SLEEP = 0.6  # seconds between requests to be polite


def query_efehr(lon: float, lat: float, rp: int = 475) -> float | None:
    """Return mean PGA (g) for the given point, or None on error."""
    url = (
        f"{EFEHR_BASE}?lon={lon:.6f}&lat={lat:.6f}"
        f"&rp={rp}&imt=PGA&vs30=760"
    )
    try:
        req = urllib.request.Request(url, headers={"Accept": "application/json"})
        with urllib.request.urlopen(req, timeout=15) as resp:
            data = json.loads(resp.read().decode())
        # Try common response shapes
        if "mean_hazard" in data:
            return float(data["mean_hazard"])
        if "pga" in data:
            return float(data["pga"])
        if "data" in data and isinstance(data["data"], dict):
            return float(data["data"].get("mean_hazard", data["data"].get("pga", 0)))
        # Print full response once so the caller can adapt
        print(f"  WARN: unexpected API response shape: {json.dumps(data)[:200]}")
        return None
    except urllib.error.HTTPError as e:
        print(f"  HTTP {e.code} for ({lat:.4f}, {lon:.4f})")
        return None
    except Exception as e:
        print(f"  Error for ({lat:.4f}, {lon:.4f}): {e}")
        return None


def main():
    parser = argparse.ArgumentParser(
        description="Patch seismic_pga in clusters-meta.json via EFEHR API"
    )
    parser.add_argument("--meta", default=META_PATH, metavar="PATH",
                        help="Path to clusters-meta.json")
    parser.add_argument("--dry-run", action="store_true",
                        help="Query one cluster, print result, do not write")
    parser.add_argument("--countries", nargs="+", default=None,
                        metavar="ISO",
                        help="Limit to specific ISO codes (e.g. DE FR GB)")
    parser.add_argument("--overwrite", action="store_true",
                        help="Re-query clusters that already have seismic_pga set")
    parser.add_argument("--rp", type=int, default=475,
                        help="Return period in years (default 475 = 10%% in 50yr)")
    args = parser.parse_args()

    if not os.path.exists(args.meta):
        print(f"ERROR: {args.meta} not found", file=sys.stderr)
        sys.exit(1)

    with open(args.meta) as f:
        clusters = json.load(f)

    target_isos = set(args.countries) if args.countries else EU_ISOS
    targets = [
        c for c in clusters
        if c.get("iso") in target_isos
        and (args.overwrite or c.get("seismic_pga_g") is None)
    ]

    print(f"Loaded {len(clusters)} clusters")
    print(f"EU targets to sample: {len(targets)} (countries: {sorted(target_isos)})")

    if args.dry_run:
        if not targets:
            print("No eligible targets found.")
            sys.exit(0)
        c = targets[0]
        print(f"\nDRY RUN — querying one cluster: {c.get('id')} @ ({c['lat']}, {c['lon']}) iso={c.get('iso')}")
        pga = query_efehr(c["lon"], c["lat"], rp=args.rp)
        if pga is not None:
            print(f"Result: seismic_pga = {pga:.4f} g  (RP={args.rp}yr)")
        else:
            print("Result: None (API may be unreachable or endpoint format changed)")
            print("Fallback: download GSHAP global raster and sample with rasterio/gdallocationinfo")
        sys.exit(0)

    # Full run
    patched = 0
    errors = 0
    for i, c in enumerate(targets):
        pga = query_efehr(c["lon"], c["lat"], rp=args.rp)
        if pga is not None:
            c["seismic_pga_g"] = round(pga, 4)
            patched += 1
        else:
            errors += 1
        if (i + 1) % 50 == 0:
            print(f"  {i+1}/{len(targets)} — patched={patched} errors={errors}")
        time.sleep(RATE_SLEEP)

    print(f"\nPatched {patched}/{len(targets)} clusters; {errors} API errors")

    if patched > 0:
        out_path = args.meta
        tmp_path = args.meta + ".tmp"
        with open(tmp_path, "w") as f:
            json.dump(clusters, f, separators=(",", ":"))
        os.replace(tmp_path, out_path)
        print(f"Wrote {out_path}")
    else:
        print("No clusters patched — clusters-meta.json unchanged")
        if errors == len(targets):
            print("All queries failed. Check EFEHR API availability, or use GSHAP fallback.")
            sys.exit(1)


if __name__ == "__main__":
    main()
