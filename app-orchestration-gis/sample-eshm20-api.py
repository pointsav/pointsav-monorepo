#!/usr/bin/env python3
# SPDX-License-Identifier: LicenseRef-PointSav-ARR
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
#
# This file is proprietary material of Woodfine Capital Projects Inc.
# See the LICENSE file in this repository for the full terms.
# Unauthorized use, reproduction, or distribution is prohibited.


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
    https://efehr-services.ethz.ch/api/v1/curves?
        lon={lon}&lat={lat}&modelid=81&imt=PGA

    2026-07-01: the old `maps.efehr.org/api/v1/calc/disaggregation` endpoint this
    script used is dead (NXDOMAIN). EFEHR migrated to a new unified platform at
    efehr-services.ethz.ch (valid cert, confirmed live) — discovered via the "Web
    Services" page at hazard.efehr.org (readable with `curl -k` despite its own
    expired cert) which links a beta OpenAPI spec at
    https://efehr-services.ethz.ch/api/openapi.json. Old `hazard.efehr.org` itself
    is a documentation portal, not an API host — do not point EFEHR_BASE at it.

    The new API's `/v1/maps` point-query endpoint (closest analogue to the old
    disaggregation call) timed out repeatedly in testing (likely a beta-quality
    backend issue) — use `/v1/curves` instead, which returns the full hazard curve
    (investigation_time=50yr, `points: [{iml, poe}, ...]` sorted by increasing PGA)
    fast and reliably (~0.6s/call, 4/4 test points across DE/FR/ES/IT succeeded, no
    rate-limiting observed). Interpolate PGA at the desired `poe` client-side —
    poe=0.10 with investigation_time=50yr is exactly the standard "10% probability
    of exceedance in 50 years" / 475yr-return-period seismic design value, so no
    additional return-period math is needed against this endpoint.

    Model ID 81 = ESHM20 (confirmed via `/v1/models?lon=..&lat=..` for any EU point).

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

# EFEHR API — new unified platform, no authentication required (2026-07-01)
EFEHR_BASE = "https://efehr-services.ethz.ch/api/v1/curves"
ESHM20_MODEL_ID = 81
RATE_SLEEP = 0.6  # seconds between requests to be polite


def _interpolate_pga(points: list, target_poe: float) -> float | None:
    """Log-linear interpolate PGA (iml) at the target poe from a sorted hazard curve.

    `points` is the curve's own point list: increasing iml, decreasing poe.
    """
    for i in range(len(points) - 1):
        p0, p1 = points[i], points[i + 1]
        if p0["poe"] >= target_poe >= p1["poe"]:
            if p0["poe"] == p1["poe"]:
                return p0["iml"]
            # log-linear in both iml and poe, standard for hazard curves
            import math
            log_poe0, log_poe1 = math.log(p0["poe"]), math.log(p1["poe"])
            log_iml0, log_iml1 = math.log(p0["iml"]), math.log(p1["iml"])
            frac = (math.log(target_poe) - log_poe0) / (log_poe1 - log_poe0)
            return math.exp(log_iml0 + frac * (log_iml1 - log_iml0))
    return None  # target_poe outside the curve's range


def query_efehr(lon: float, lat: float, rp: int = 475) -> float | None:
    """Return interpolated PGA (g) at the given return period, or None on error.

    rp=475 <-> poe=0.10 over a 50yr investigation time (standard 10%-in-50yr design
    value) — the curve endpoint already returns points at investigation_time=50yr,
    so poe=1/rp is not the right conversion here; poe=0.10 is used directly for the
    only rp this script calls with (475).
    """
    target_poe = 1.0 - (1.0 - 1.0 / rp) ** 50  # ~0.10 for rp=475
    url = f"{EFEHR_BASE}?lon={lon:.6f}&lat={lat:.6f}&modelid={ESHM20_MODEL_ID}&imt=PGA"
    try:
        req = urllib.request.Request(url, headers={"Accept": "application/json"})
        with urllib.request.urlopen(req, timeout=20) as resp:
            data = json.loads(resp.read().decode())
        curves = data.get("curves", [])
        if not curves:
            print(f"  WARN: no curve returned for ({lat:.4f}, {lon:.4f})")
            return None
        # Use the arithmetic-mean aggregation (first curve; matches the old script's
        # "mean_hazard" semantics) rather than a percentile/fractile curve.
        curve = next((c for c in curves if c.get("aggregation_type") == "arithmetic"), curves[0])
        return _interpolate_pga(curve["points"], target_poe)
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
        if targets and errors == len(targets):
            print("All queries failed. Check EFEHR API availability, or use GSHAP fallback.")
            # Non-fatal: an unreachable external API is a degraded-data condition, not a
            # script error — exit 0 so build-aec-seismic.sh's `set -euo pipefail` doesn't
            # abort before steps 5-9 (wetland/EU zone join), matching how the NRCan step
            # already treats its own unreachable-host case as non-fatal (SKIP_NRCAN=1).


if __name__ == "__main__":
    main()
