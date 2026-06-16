#!/usr/bin/env python3
"""
build-ashrae-zone.py — Patch ashrae_zone into clusters-meta.json

Primary path (--from-fields, default): derives ASHRAE 169-2020 zone from the
cluster's existing koppen_class + hdd18 + cdd18 fields.  No network required.

Secondary path (--from-geojson): downloads the DOE/PNNL climate zone GeoJSON
and does a point-in-polygon spatial join (requires shapely + network).

ASHRAE 169-2020 zones: 1A–8A, 1B–7B, 3C, 4C, 5C, 6C
  A=Humid  B=Dry  C=Marine

Usage:
    python3 build-ashrae-zone.py               # from-fields (default, fast)
    python3 build-ashrae-zone.py --from-geojson
    python3 build-ashrae-zone.py --dry-run
    python3 build-ashrae-zone.py --overwrite --countries DE FR
"""

import argparse
import json
import os
import sys
import urllib.request
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import WORK_DIR

META_PATH = Path(
    "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
)
ASHRAE_GEOJSON = WORK_DIR / "aec" / "ashrae-zones.geojson"
ASHRAE_URL = (
    "https://energyplus.net/assets/nrel_custom/epw_climate_zones/EPW_CZ_Map.geojson"
)

# ── Köppen → ASHRAE moisture-regime mapping ──────────────────────────────────
# B (Dry): arid and semi-arid climates; dry-summer subtypes of C/D
_DRY_PREFIXES = ("BW", "BS")
_DRY_SUBTYPES = ("Csa", "Csb", "Csc", "Dsa", "Dsb", "Dsc", "Dsd")
# C (Marine): oceanic Cfb/Cfc zones with no dry season and mild winters
# — assigned only when zone number is 3–6 (temperature range consistent with marine)
_MARINE_SUBTYPES = ("Cfb", "Cfc")


def moisture_regime(koppen: str, zone_num: int) -> str:
    """Return ASHRAE moisture suffix (A/B/C) from Köppen class and zone number."""
    if not koppen:
        return "A"
    if koppen[:2] in _DRY_PREFIXES or koppen in _DRY_SUBTYPES:
        return "B"
    if koppen in _MARINE_SUBTYPES and 3 <= zone_num <= 6:
        return "C"
    return "A"


def ashrae_from_fields(hdd18, cdd18, koppen_class: str) -> str | None:
    """
    Derive ASHRAE 169-2020 zone from HDD/CDD (base 18 °C) and Köppen class.

    Zone-number boundaries (ASHRAE Table B-1, converted to base-18 °C):
      1: CDD ≥ 5000
      2: CDD ≥ 3000
      3: CDD ≥ 1200  or  HDD < 1200
      4: HDD < 2600
      5: HDD < 4000
      6: HDD < 5500
      7: HDD < 8000
      8: HDD ≥ 8000
    """
    try:
        hdd = float(hdd18 or 0)
        cdd = float(cdd18 or 0)
    except (TypeError, ValueError):
        return None

    if cdd >= 5000:
        n = 1
    elif cdd >= 3000:
        n = 2
    elif cdd >= 1200 or hdd < 1200:
        n = 3
    elif hdd < 2600:
        n = 4
    elif hdd < 4000:
        n = 5
    elif hdd < 5500:
        n = 6
    elif hdd < 8000:
        n = 7
    else:
        n = 8

    m = moisture_regime(koppen_class or "", n)
    return f"{n}{m}"


# ── GeoJSON spatial-join path ─────────────────────────────────────────────────

def download_ashrae(path: Path) -> bool:
    path.parent.mkdir(parents=True, exist_ok=True)
    print(f"Downloading ASHRAE climate zones → {path}")
    try:
        req = urllib.request.Request(
            ASHRAE_URL,
            headers={"User-Agent": "Mozilla/5.0 (compatible; gis-ashrae/1.0)"},
        )
        with urllib.request.urlopen(req, timeout=60) as r:
            data = r.read()
        gj = json.loads(data)
        if "features" not in gj or len(gj["features"]) < 10:
            print(f"  WARN: unexpected GeoJSON ({len(gj.get('features', []))} features)")
            return False
        path.write_bytes(data)
        print(f"  → {path} ({len(gj['features'])} zone polygons)  ✓")
        return True
    except Exception as e:
        print(f"  ERROR: {e}")
        return False


def load_zones(path: Path):
    try:
        from shapely.geometry import shape
    except ImportError:
        print("ERROR: shapely not available — install with: pip3 install shapely")
        sys.exit(1)
    gj = json.loads(path.read_text())
    zones = []
    for feat in gj.get("features", []):
        props = feat.get("properties") or {}
        zone = (
            props.get("IECC_Climate_Zone")
            or props.get("BA_Climate_Zone")
            or props.get("Climate_Zone")
            or props.get("zone")
            or props.get("CZ")
        )
        if not zone:
            for v in props.values():
                s = str(v).strip()
                if len(s) == 2 and s[0].isdigit() and s[1].upper() in "ABC":
                    zone = s.upper()
                    break
        if not zone:
            continue
        try:
            geom = shape(feat["geometry"])
            zones.append((str(zone).strip().upper(), geom))
        except Exception:
            continue
    print(f"  Loaded {len(zones)} ASHRAE zone polygons")
    return zones


def zone_for_point_geojson(lon: float, lat: float, zones: list) -> str | None:
    try:
        from shapely.geometry import Point
    except ImportError:
        return None
    pt = Point(lon, lat)
    for zone, geom in zones:
        if geom.contains(pt):
            return zone
    return None


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Patch ashrae_zone in clusters-meta.json")
    parser.add_argument("--meta", default=str(META_PATH))
    parser.add_argument("--dry-run", action="store_true",
                        help="Process one cluster; no writes")
    parser.add_argument("--overwrite", action="store_true",
                        help="Re-assign clusters that already have ashrae_zone")
    parser.add_argument("--countries", nargs="+", default=None, metavar="ISO")
    parser.add_argument("--from-geojson", action="store_true",
                        help="Use GeoJSON spatial join instead of field derivation")
    args = parser.parse_args()

    meta_path = Path(args.meta)
    if not meta_path.exists():
        print(f"ERROR: {meta_path} not found", file=sys.stderr)
        sys.exit(1)

    clusters = json.loads(meta_path.read_text())
    target_isos = set(args.countries) if args.countries else None
    targets = [
        c for c in clusters
        if (target_isos is None or c.get("iso") in target_isos)
        and (args.overwrite or c.get("ashrae_zone") is None)
    ]
    print(f"Clusters total: {len(clusters)}  |  targets: {len(targets)}")

    if args.from_geojson:
        if not ASHRAE_GEOJSON.exists():
            ok = download_ashrae(ASHRAE_GEOJSON)
            if not ok:
                print("WARN: could not download ASHRAE zones — falling back to field derivation")
                args.from_geojson = False
        if args.from_geojson:
            zones = load_zones(ASHRAE_GEOJSON)
            zone_fn = lambda c: zone_for_point_geojson(c.get("lon", 0), c.get("lat", 0), zones)

    if not args.from_geojson:
        missing = sum(1 for c in targets if c.get("koppen_class") is None)
        if missing:
            print(f"  WARN: {missing}/{len(targets)} targets missing koppen_class — moisture defaults to 'A'")
        zone_fn = lambda c: ashrae_from_fields(c.get("hdd18"), c.get("cdd18"), c.get("koppen_class"))

    if args.dry_run:
        if not targets:
            print("No eligible targets.")
            return
        c = targets[0]
        z = zone_fn(c)
        print(f"DRY RUN — cluster {c.get('id')} @ ({c.get('lat')}, {c.get('lon')}) "
              f"koppen={c.get('koppen_class')} hdd={c.get('hdd18')} cdd={c.get('cdd18')} "
              f"→ ashrae_zone={z}")
        return

    patched = 0
    for i, c in enumerate(targets):
        z = zone_fn(c)
        if z:
            c["ashrae_zone"] = z
            patched += 1
        if (i + 1) % 500 == 0:
            print(f"  {i+1}/{len(targets)} — patched {patched}", flush=True)

    print(f"Patched {patched}/{len(targets)} clusters with ashrae_zone")

    if patched > 0:
        tmp = str(meta_path) + ".tmp"
        with open(tmp, "w") as f:
            json.dump(clusters, f, separators=(",", ":"))
        os.replace(tmp, str(meta_path))
        print(f"Wrote {meta_path}")
    else:
        print("No clusters patched — clusters-meta.json unchanged")


if __name__ == "__main__":
    main()
