#!/usr/bin/env python3
"""
build-tiles.py — JSONL → GeoJSON → PMTiles (all three layers)

Reads from TOTEBOX_DATA_PATH (service-business + service-places).
Reads clusters.geojson and radius.geojson from work/.
Writes layer1-locations.pmtiles, layer2-clusters.pmtiles,
layer3-radius.pmtiles to www/tiles/.

Requires: tippecanoe v2.79.0+ on PATH.
"""

import json
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from config import TOTEBOX_DATA_PATH, SERVICE_BUSINESS, SERVICE_PLACES, TILES_DIR, WORK_DIR, WWW_DIR

# chain_id → brand family (drives circle-color in the locations layer)
# Seven families: Hypermarket | Lifestyle | Hardware | Warehouse | Food | Furniture | Pharmacy
# D5 2026-05-16: IKEA exits Hypermarket → Lifestyle class (separate anchor category)
# D1/Phase 1 2026-05-16: mercadona-es + tesco-uk + sainsburys-uk → Hypermarket
CHAIN_FAMILY = {
    # Hypermarket (anchor — general-merchandise + groceries, large-format)
    "walmart-us":               "Hypermarket",
    "walmart-ca":               "Hypermarket",
    "walmart-mx":               "Hypermarket",
    "bodega-aurrera-mx":        "Hypermarket",
    "real-canadian-superstore-ca": "Hypermarket",
    "target-us":                "Hypermarket",
    "soriana-mx":               "Hypermarket",
    "mercadona-es":             "Hypermarket",
    "tesco-uk":                 "Hypermarket",
    "sainsburys-uk":            "Hypermarket",
    "carrefour-hypermarket-es": "Hypermarket",
    "alcampo-es":               "Hypermarket",
    "leclerc-es":               "Hypermarket",
    "carrefour-hypermarket-it": "Hypermarket",
    "ipercoop-it":              "Hypermarket",
    "iper-it":                  "Hypermarket",
    "bennet-it":                "Hypermarket",
    "carrefour-hypermarket-pl": "Hypermarket",
    "leclerc-pl":               "Hypermarket",
    "auchan-pl":                "Hypermarket",
    "bilka-dk":                 "Hypermarket",
    "prisma-fi":                "Hypermarket",
    "k-citymarket-fi":          "Hypermarket",
    "obs-coop-no":              "Hypermarket",
    "hagkaup-is":               "Hypermarket",
    "coop-forum-se":            "Hypermarket",
    "maxi-ica-se":              "Hypermarket",
    # Lifestyle (anchor — destination furniture/home goods; IKEA family)
    "ikea-us":                  "Lifestyle",
    "ikea-ca":                  "Lifestyle",
    "ikea-mx":                  "Lifestyle",
    "ikea-es":                  "Lifestyle",
    "ikea-gr":                  "Lifestyle",
    "ikea-it":                  "Lifestyle",
    "ikea-se":                  "Lifestyle",
    "ikea-dk":                  "Lifestyle",
    "ikea-no":                  "Lifestyle",
    "ikea-fi":                  "Lifestyle",
    "ikea-pl":                  "Lifestyle",
    "ikea-fr":                  "Lifestyle",
    "ikea-de":                  "Lifestyle",
    "ikea-uk":                  "Lifestyle",
    "ikea-at":                  "Lifestyle",
    "ikea-nl":                  "Lifestyle",
    "ikea-pt":                  "Lifestyle",
    # Hardware / Home improvement
    "home-depot-us":            "Hardware",
    "home-depot-ca":            "Hardware",
    "home-depot-mx":            "Hardware",
    "lowes-us":                 "Hardware",
    "lowes-ca":                 "Hardware",
    "leroy-merlin-es":          "Hardware",
    "leroy-merlin-it":          "Hardware",
    "leroy-merlin-gr":          "Hardware",
    "leroy-merlin-pl":          "Hardware",
    "leroy-merlin-fr":          "Hardware",
    "leroy-merlin-pt":          "Hardware",
    "clas-ohlson-se":           "Hardware",
    "canadian-tire-ca":         "Hardware",
    "peavey-mart-ca":           "Hardware",
    "imerco-dk":                "Hardware",
    "k-rauta-fi":               "Hardware",
    "obs-bygg-no":              "Hardware",
    "gamma-nl":                 "Hardware",
    "karwei-nl":                "Hardware",
    "praxis-nl":                "Hardware",
    "castorama-fr":             "Hardware",
    "castorama-pl":             "Hardware",
    "husasmidjan-is":           "Hardware",
    "brico-depot-es":           "Hardware",
    "toom-baumarkt-de":         "Hardware",
    "bricocenter-it":           "Hardware",
    "silvan-dk":                "Hardware",
    "praktiker-gr":             "Hardware",
    "byko-is":                  "Hardware",
    "hagebaumarkt-de":          "Hardware",
    "hornbach-de":              "Hardware",
    "hornbach-at":              "Hardware",
    "bauhaus-se":               "Hardware",
    "bq-uk":                    "Hardware",
    # Warehouse Club (membership and cash & carry)
    "costco-us":                "Warehouse",
    "costco-ca":                "Warehouse",
    "costco-mx":                "Warehouse",
    "costco-es":                "Warehouse",
    "costco-se":                "Warehouse",
    "costco-is":                "Warehouse",
    "costco-uk":                "Warehouse",
    "costco-fr":                "Warehouse",
    "sams-club-us":             "Warehouse",
    "sams-club-mx":             "Warehouse",
    "bjs-wholesale-us":         "Warehouse",
    "makro-es":                 "Warehouse",
    "makro-nl":                 "Warehouse",
    "metro-it":                 "Warehouse",
    "metro-de":                 "Warehouse",
    "makro-pl":                 "Warehouse",
    "selgros-de":               "Warehouse",
    "selgros-pl":               "Warehouse",
    "the-mart-gr":              "Warehouse",
    # Food / Grocery (data layer — visible on map, not in algorithm)
    "carrefour-it":             "Food",
    "lidl-es":                  "Food",
    "lidl-gr":                  "Food",
    "lidl-it":                  "Food",
    "lidl-nordics":             "Food",
    "lidl-pl":                  "Food",
    "safeway-ca":               "Food",
    "safeway-us":               "Food",
    "save-on-foods-ca":         "Food",
    "whole-foods-us":           "Hypermarket",
    "chedraui-mx":              "Hypermarket",
    "asda-uk":                  "Hypermarket",
    "morrisons-uk":             "Hypermarket",
    "heb-us":                   "Hypermarket",
    "wegmans-us":               "Hypermarket",
    "winco-foods-us":           "Hypermarket",
    "sprouts-us":               "Hypermarket",
    "hipercor-es":              "Food",
    "famila-it":                "Food",
    "biedronka-pl":             "Food",
    "stokrotka-pl":             "Food",
    "netto-pl":                 "Food",
    "tiendas-3b-mx":            "Food",
    "sklavenitis-gr":           "Food",
    "lidl-uk":                  "Food",
    "lidl-de":                  "Food",
    "lidl-fr":                  "Food",
    "lidl-nl":                  "Food",
    "lidl-at":                  "Food",
    "lidl-pt":                  "Food",
    "aldi-de":                  "Food",
    "aldi-uk":                  "Food",
    "aldi-nl":                  "Food",
    "aldi-pl":                  "Food",
    "tienda-inglesa-uy":        "Food",
    "carrefour-fr":             "Food",
    # Furniture
    "conforama-es":             "Furniture",
    # Traditional department stores (not hypermarkets)
    "macys-us":                 "Department",
    "london-drugs-ca":          "Pharmacy",
}

TILES_DIR.mkdir(parents=True, exist_ok=True)
WORK_DIR.mkdir(parents=True, exist_ok=True)


def load_jsonl(path: Path) -> list:
    records = []
    if not path.exists():
        return records
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                try:
                    records.append(json.loads(line))
                except json.JSONDecodeError:
                    pass
    return records


def record_to_feature(rec: dict, id_field: str = "chain_id") -> dict | None:
    lat = rec.get("latitude")
    lon = rec.get("longitude")
    if lat is None or lon is None:
        return None
    try:
        lat, lon = float(lat), float(lon)
    except (TypeError, ValueError):
        return None
    props = {k: v for k, v in rec.items()
             if k not in ("latitude", "longitude", "polygon_wkt")
             and v is not None}
    return {
        "type": "Feature",
        "geometry": {"type": "Point", "coordinates": [lon, lat]},
        "properties": props,
    }


def build_layer1():
    """Merge all service-business + service-places JSONL → layer1-locations.pmtiles"""
    print("\n[Layer 1] Building base locations tile...")
    features = []

    # service-business (raw locations in service-fs branch)
    loc_dir = TOTEBOX_DATA_PATH / "service-fs" / "service-business"
    if loc_dir.exists():
        for jsonl_file in sorted(loc_dir.glob("*.jsonl")):
            recs = load_jsonl(jsonl_file)
            for rec in recs:
                feat = record_to_feature(rec, "chain_id")
                if feat:
                    chain_id = feat["properties"].get("chain_id", "")
                    bf = CHAIN_FAMILY.get(chain_id)
                    if bf:
                        feat["properties"]["brand_family"] = bf
                    features.append(feat)
    print(f"  service-business: {len(features)} features")

    # service-places (raw locations in service-fs branch)
    sp_start = len(features)
    sp_dir = TOTEBOX_DATA_PATH / "service-fs" / "service-places"
    if sp_dir.exists():
        for jsonl_file in sorted(sp_dir.glob("*.jsonl")):
            recs = load_jsonl(jsonl_file)
            for rec in recs:
                feat = record_to_feature(rec, "category_id")
                if feat:
                    features.append(feat)
    print(f"  service-places: {len(features) - sp_start} features")

    # service-places (OSM civic ingest — hospitals + universities)
    civic_path = TOTEBOX_DATA_PATH / "service-places" / "cleansed-civic-osm.jsonl"
    civic_start = len(features)
    if civic_path.exists():
        for rec in load_jsonl(civic_path):
            feat = record_to_feature(rec, "category_id")
            if feat:
                features.append(feat)
    print(f"  service-places (civic OSM): {len(features) - civic_start} features")
    print(f"  Total: {len(features)} features")

    if not features:
        print("  No features found. Check TOTEBOX_DATA_PATH.")
        return False

    fc = {"type": "FeatureCollection", "features": features}
    geojson_path = WORK_DIR / "layer1-locations.geojson"
    with open(geojson_path, "w") as f:
        json.dump(fc, f)

    out = TILES_DIR / "layer1-locations.pmtiles"
    cmd = [
        "tippecanoe",
        "--output", str(out),
        "--force",
        "--layer", "locations",
        "--minimum-zoom", "2",
        "--maximum-zoom", "17",
        "--base-zoom", "4",
        "--no-feature-limit",
        "--no-tile-size-limit",
        str(geojson_path),
    ]
    print(f"  Running tippecanoe → {out.name}")
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"  ERROR: {result.stderr[-500:]}")
        return False
    size_mb = out.stat().st_size / 1_000_000
    print(f"  Done: {out.name} ({size_mb:.1f} MB)")
    return True


def build_layer2():
    """clusters.geojson → layer2-clusters.pmtiles"""
    print("\n[Layer 2] Building co-location clusters tile...")
    clusters_path = WORK_DIR / "clusters.geojson"
    if not clusters_path.exists():
        print("  clusters.geojson not found. Run build-clusters.py first.")
        return False

    with open(clusters_path) as f:
        fc = json.load(f)
    print(f"  {len(fc.get('features', []))} clusters")

    out = TILES_DIR / "layer2-clusters.pmtiles"
    cmd = [
        "tippecanoe",
        "--output", str(out),
        "--force",
        "--layer", "clusters",
        "--minimum-zoom", "2",
        "--maximum-zoom", "14",
        "--no-feature-limit",
        "--no-tile-size-limit",
        "--base-zoom", "2",
        str(clusters_path),
    ]
    print(f"  Running tippecanoe → {out.name}")
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"  ERROR: {result.stderr[-500:]}")
        return False
    size_mb = out.stat().st_size / 1_000_000
    print(f"  Done: {out.name} ({size_mb:.1f} MB)")
    return True


def build_clusters_meta():
    """Extract cluster records → www/data/clusters-meta.json (§2 schema)."""
    print("\n[Meta] Building clusters-meta.json...")
    clusters_path = WORK_DIR / "clusters.geojson"
    if not clusters_path.exists():
        print("  clusters.geojson not found.")
        return False
    with open(clusters_path) as f:
        fc = json.load(f)

    meta = []
    tier_counts: dict = {}
    for feat in fc.get("features", []):
        p = feat.get("properties", {})
        coords = feat.get("geometry", {}).get("coordinates", [])
        if not coords or len(coords) < 2:
            continue
        tier = int(p.get("tier") or 0)
        tier_counts[tier] = tier_counts.get(tier, 0) + 1
        members = p.get("members") or []
        if isinstance(members, str):
            try:
                members = json.loads(members)
            except Exception:
                members = []
        meta.append({
            "id":   p.get("cluster_id", ""),
            "lon":  round(float(coords[0]), 5),
            "lat":  round(float(coords[1]), 5),
            # Tier + geometry
            "t":    tier,
            "td":   p.get("tier_descriptor") or "",
            "span": round(float(p.get("span_km") or 0.0), 3),
            "tight": 1 if p.get("tight_intact") else 0,
            "rr":   round(float(p.get("ring_radius_km") or 0.0), 2),
            # Ranking
            "dr":   round(float(p.get("dist_rank_in_tier") or 0.0), 4),
            "dp":   int(p.get("dist_pctile") or 0),
            "dmr":  round(float(p.get("demand_rank_in_tier") or 0.5), 4),
            "dmb":  p.get("demand_basis") or "interim-none",
            # Market identity
            "rm":   p.get("regional_market") or "",
            "mkt":  p.get("market_name") or "",
            "mrgn": p.get("market_region") or "",
            "metro": p.get("metro_market") or "",
            "conf": p.get("mkt_conf") or "low",
            "iso":  p.get("iso") or "",
            "cont": p.get("continent") or "",
            # Members
            "mc":   int(p.get("member_count") or len(members)),
            "members": members,
        })
    out = WWW_DIR / "data" / "clusters-meta.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    with open(out, "w") as f:
        json.dump(meta, f, separators=(',', ':'))
    size_kb = out.stat().st_size / 1000
    print(f"  Done: {len(meta)} clusters, {size_kb:.0f} KB → {out}")
    for t in sorted(tier_counts):
        print(f"  T{t}: {tier_counts[t]}")
    return True


def build_layer3():
    """radius.geojson → layer3-radius.pmtiles"""
    print("\n[Layer 3] Building catchment radius tile...")
    radius_path = WORK_DIR / "radius.geojson"
    if not radius_path.exists():
        print("  radius.geojson not found. Run build-radius.py first.")
        return False

    with open(radius_path) as f:
        fc = json.load(f)
    print(f"  {len(fc.get('features', []))} radius polygons")

    out = TILES_DIR / "layer3-radius.pmtiles"
    cmd = [
        "tippecanoe",
        "--output", str(out),
        "--force",
        "--layer", "radius",
        "--minimum-zoom", "3",
        "--maximum-zoom", "12",
        "--no-feature-limit",
        "--simplification", "4",
        str(radius_path),
    ]
    print(f"  Running tippecanoe → {out.name}")
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"  ERROR: {result.stderr[-500:]}")
        return False
    size_mb = out.stat().st_size / 1_000_000
    print(f"  Done: {out.name} ({size_mb:.1f} MB)")
    return True


def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--layer", choices=["1", "2", "3", "all"], default="all")
    args = parser.parse_args()

    results = {}
    if args.layer in ("1", "all"):
        results["layer1"] = build_layer1()
    if args.layer in ("2", "all"):
        results["layer2"] = build_layer2()
        results["meta"]   = build_clusters_meta()
    if args.layer in ("3", "all"):
        results["layer3"] = build_layer3()

    print("\n── Summary ──")
    for k, v in results.items():
        print(f"  {k}: {'OK' if v else 'SKIP/ERROR'}")


if __name__ == "__main__":
    main()
