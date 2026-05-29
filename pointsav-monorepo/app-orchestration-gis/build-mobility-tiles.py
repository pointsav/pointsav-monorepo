#!/usr/bin/env python3
"""
build-mobility-tiles.py — Mobility (WORK + observed HOME) PMTile layers

Reads observed mobility JSONL outputs from ingest-lodes.py (US),
ingest-statcan.py (CA), and ingest-mitma.py (ES). Builds H3 res-7
hexagon polygon GeoJSONs, normalises visit volumes to 0–1 per layer,
and runs tippecanoe → PMTiles for the gateway.

Outputs (GeoJSON → tippecanoe → PMTiles in gateway tiles dir):
  work/mobility-work.geojson     → layer6-mobility-work.pmtiles
                                    (merged US LODES + CA StatCan + ES MITMA WORK reach)
  work/mobility-home-es.geojson  → layer7-mobility-home-measured.pmtiles
                                    (ES MITMA HOME reach — measured, MNO observed)

Inputs (under service-fs/service-mobility/):
  lodes-work-od-us.jsonl     — US LODES; visits_work_total
  statcan-work-od-ca.jsonl   — CA StatCan; visits_work_total
  mitma-work-od-es.jsonl     — ES MITMA; visits_work_total
  mitma-home-od-es.jsonl     — ES MITMA; visits_home_total (measured)

Schema: {h3, lat, lon, iso, visits_*_total, cluster_proximity,
         data_source, is_measured}

Side-effect: merges mobility_source / mobility_vintage into
clusters-meta.json for clusters touched by an observed source.

Provenance baseline: clusters without observed mobility default to
mobility_source='radius' / mobility_vintage='worldpop-2026' (set by
synthesize-od-study.py). This script only writes over that baseline
for clusters covered by LODES / StatCan / MITMA.
"""

import json
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

try:
    import h3 as h3lib
except ImportError:
    print("ERROR: h3 not installed.")
    sys.exit(1)

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------
DEPLOY        = Path("/srv/foundry/deployments/cluster-totebox-personnel-1")
MOBILITY_DIR  = DEPLOY / "service-fs/service-mobility"
TILES_DIR     = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles")
WORK_DIR      = Path(__file__).parent / "work"
CLUSTERS_META = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")

LODES_FILE      = MOBILITY_DIR / "lodes-work-od-us.jsonl"
STATCAN_FILE    = MOBILITY_DIR / "statcan-work-od-ca.jsonl"
MITMA_WORK_FILE = MOBILITY_DIR / "mitma-work-od-es.jsonl"
MITMA_HOME_FILE = MOBILITY_DIR / "mitma-home-od-es.jsonl"

WORK_GEOJSON     = WORK_DIR / "mobility-work.geojson"
HOME_ES_GEOJSON  = WORK_DIR / "mobility-home-es.geojson"
WORK_PMTILES     = TILES_DIR / "layer6-mobility-work.pmtiles"
HOME_ES_PMTILES  = TILES_DIR / "layer7-mobility-home-measured.pmtiles"

# Vintage labels — keep aligned with ingest scripts
SOURCE_VINTAGES = {
    "lodes":   "lodes-2021",
    "statcan": "statcan-2021",
    "mitma":   "mitma-2023",
}


def h3_hex_polygon(h3_idx: str) -> list:
    """Returns GeoJSON polygon ring from H3 cell boundary. h3 v4 returns (lat,lon)."""
    boundary = h3lib.cell_to_boundary(h3_idx)
    coords = [[round(lon, 5), round(lat, 5)] for lat, lon in boundary]
    coords.append(coords[0])
    return [coords]


def read_jsonl(path: Path) -> list:
    """Load JSONL into list of dicts. Returns [] if missing."""
    if not path.exists():
        print(f"  SKIP: {path.name} not found.")
        return []
    rows = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError as e:
                print(f"  WARN: {path.name} parse error: {e}")
    print(f"  Loaded {len(rows):,} rows from {path.name}.")
    return rows


def build_work_layer():
    """Merge LODES + StatCan + MITMA WORK reach into one GeoJSON."""
    print("\n=== Layer 6 — WORK reach (merged) ===")
    rows = []
    rows.extend(read_jsonl(LODES_FILE))
    rows.extend(read_jsonl(STATCAN_FILE))
    rows.extend(read_jsonl(MITMA_WORK_FILE))

    if not rows:
        print("  No WORK reach data available — skipping layer 6.")
        return False

    # Visit max for normalisation
    max_visits = max((r.get("visits_work_total", 0) or 0) for r in rows) or 1.0
    print(f"  WORK max visits (for normalisation): {max_visits:,.0f}")

    features = []
    skipped = 0
    for r in rows:
        h3_idx = r.get("h3")
        if not h3_idx:
            skipped += 1
            continue
        visits = r.get("visits_work_total", 0) or 0
        if visits <= 0:
            skipped += 1
            continue
        iso = r.get("iso")
        if isinstance(iso, list):
            iso = iso[0] if iso else ""
        features.append({
            "type": "Feature",
            "geometry": {"type": "Polygon",
                         "coordinates": h3_hex_polygon(h3_idx)},
            "properties": {
                "h3": h3_idx,
                "visits_work_total": round(visits),
                "visits_norm": round(visits / max_visits, 4),
                "iso": iso or "",
                "data_source": r.get("data_source", ""),
                "is_measured": bool(r.get("is_measured", True)),
            },
        })

    print(f"  WORK features: {len(features):,} kept / {skipped:,} skipped.")
    with open(WORK_GEOJSON, "w") as f:
        json.dump({"type": "FeatureCollection", "features": features},
                  f, separators=(",", ":"))
    return True


def build_home_es_layer():
    """MITMA ES HOME reach (measured) — single source for now."""
    print("\n=== Layer 7 — HOME reach measured (MITMA ES) ===")
    rows = read_jsonl(MITMA_HOME_FILE)
    if not rows:
        print("  No HOME reach data available — skipping layer 7.")
        return False

    max_visits = max((r.get("visits_home_total", 0) or 0) for r in rows) or 1.0
    print(f"  HOME max visits (for normalisation): {max_visits:,.0f}")

    features = []
    skipped = 0
    for r in rows:
        h3_idx = r.get("h3")
        if not h3_idx:
            skipped += 1
            continue
        visits = r.get("visits_home_total", 0) or 0
        if visits <= 0:
            skipped += 1
            continue
        iso = r.get("iso")
        if isinstance(iso, list):
            iso = iso[0] if iso else ""
        features.append({
            "type": "Feature",
            "geometry": {"type": "Polygon",
                         "coordinates": h3_hex_polygon(h3_idx)},
            "properties": {
                "h3": h3_idx,
                "visits_home_total": round(visits),
                "visits_norm": round(visits / max_visits, 4),
                "iso": iso or "",
                "data_source": r.get("data_source", "mitma"),
                "is_measured": bool(r.get("is_measured", True)),
            },
        })

    print(f"  HOME features: {len(features):,} kept / {skipped:,} skipped.")
    with open(HOME_ES_GEOJSON, "w") as f:
        json.dump({"type": "FeatureCollection", "features": features},
                  f, separators=(",", ":"))
    return True


def run_tippecanoe(input_geojson: Path, output_pmtiles: Path, layer_name: str):
    """Match the build-data-tiles.py invocation pattern (z4–z12, drop densest)."""
    cmd = [
        "tippecanoe",
        "-o", str(output_pmtiles),
        "--force",
        "--layer", layer_name,
        "--minimum-zoom", "4",
        "--maximum-zoom", "12",
        "--drop-densest-as-needed",
        "--extend-zooms-if-still-dropping",
        str(input_geojson),
    ]
    print(f"  {' '.join(cmd[:6])} ... {input_geojson.name} → {output_pmtiles.name}")
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"  ERROR: {result.stderr[-500:]}")
        return False
    size_mb = output_pmtiles.stat().st_size / 1024 / 1024
    print(f"  Done: {output_pmtiles.name} ({size_mb:.1f} MB)")
    return True


def merge_mobility_provenance():
    """Update clusters-meta.json with mobility_source / mobility_vintage
    for clusters covered by an observed mobility source.

    Strategy: read JSONL files, group cluster_proximity → data_source
    (preferring measured > modelled when both touch a cluster), then
    merge into clusters-meta.json.

    Does NOT overwrite the radius baseline written by
    synthesize-od-study.py for clusters with no observed coverage.
    """
    print("\n=== Merging mobility provenance into clusters-meta.json ===")
    if not CLUSTERS_META.exists():
        print(f"  SKIP: {CLUSTERS_META} not found.")
        return

    # cluster_id → (source, is_measured)
    cluster_source = {}
    files = [
        (LODES_FILE,      "lodes"),
        (STATCAN_FILE,    "statcan"),
        (MITMA_WORK_FILE, "mitma"),
        (MITMA_HOME_FILE, "mitma"),
    ]
    for path, source in files:
        if not path.exists():
            continue
        with open(path) as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    r = json.loads(line)
                except json.JSONDecodeError:
                    continue
                cids = r.get("cluster_proximity")
                if not cids:
                    continue
                if isinstance(cids, str):
                    cids = [cids]
                # Tighter source wins (mitma measured > lodes/statcan modelled)
                for cid in cids:
                    cur = cluster_source.get(cid)
                    if cur is None or (r.get("is_measured") and not cur[1]):
                        cluster_source[cid] = (source, bool(r.get("is_measured", True)))

    if not cluster_source:
        print("  No observed mobility records — keeping radius baseline.")
        return

    print(f"  {len(cluster_source):,} clusters have observed mobility coverage.")

    with open(CLUSTERS_META) as f:
        meta = json.load(f)

    merged = 0
    for entry in meta:
        cid = entry.get("id")
        if cid in cluster_source:
            source, _measured = cluster_source[cid]
            entry["mobility_source"]  = source
            entry["mobility_vintage"] = SOURCE_VINTAGES.get(source, source)
            merged += 1
        else:
            # Establish radius baseline for any cluster missing it
            entry.setdefault("mobility_source",  "radius")
            entry.setdefault("mobility_vintage", "worldpop-2026")

    with open(CLUSTERS_META, "w") as f:
        json.dump(meta, f, separators=(",", ":"))

    print(f"  Updated {merged:,} / {len(meta):,} cluster entries with observed source.")


def main():
    WORK_DIR.mkdir(parents=True, exist_ok=True)
    TILES_DIR.mkdir(parents=True, exist_ok=True)

    have_work = build_work_layer()
    have_home = build_home_es_layer()

    print("\n=== Building PMTiles ===")
    if have_work:
        run_tippecanoe(WORK_GEOJSON, WORK_PMTILES, "mobility_work")
    else:
        print("  Skipping layer6-mobility-work.pmtiles (no input).")
    if have_home:
        run_tippecanoe(HOME_ES_GEOJSON, HOME_ES_PMTILES, "mobility_home")
    else:
        print("  Skipping layer7-mobility-home-measured.pmtiles (no input).")

    merge_mobility_provenance()

    print("\nMobility tile build complete.")
    if have_work:
        print(f"  layer6-mobility-work.pmtiles            ← LODES + StatCan + MITMA WORK")
    if have_home:
        print(f"  layer7-mobility-home-measured.pmtiles   ← MITMA ES HOME (measured)")


if __name__ == "__main__":
    main()
