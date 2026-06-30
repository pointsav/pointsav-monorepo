#!/usr/bin/env python3
"""
build-necb-zone.py — NECB 2020 climate zone for Canadian clusters
Task H — Phase 22 2026-06-30

Derives necb_zone from hdd18 (heating degree-days base 18°C) set by Task E.
NECB 2020 Table C-1 thresholds:
  Zone 4 : HDD < 3000
  Zone 5 : 3000 ≤ HDD < 4000
  Zone 6 : 4000 ≤ HDD < 5000
  Zone 7A: 5000 ≤ HDD < 6000
  Zone 7B: 6000 ≤ HDD < 7000
  Zone 8 : HDD ≥ 7000
"""
import json
from pathlib import Path

META_PATH = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")

def hdd_to_necb(hdd):
    if hdd < 3000: return 'NECB-4'
    if hdd < 4000: return 'NECB-5'
    if hdd < 5000: return 'NECB-6'
    if hdd < 6000: return 'NECB-7A'
    if hdd < 7000: return 'NECB-7B'
    return 'NECB-8'

clusters = json.loads(META_PATH.read_text())
already  = sum(1 for c in clusters if c.get('necb_zone') is not None)
print(f"  {len(clusters)} clusters; {already} already have necb_zone")

n_new = n_skip = 0
for c in clusters:
    if c.get('necb_zone') is not None:
        continue
    # Only apply to Canada clusters (identified by country field or lon/lat range)
    country = c.get('country', '') or c.get('iso', '')
    lat = c.get('lat', 0)
    lon = c.get('lon', 0)
    is_canada = (country.upper() == 'CA') or (41 < lat < 84 and -141 < lon < -52)
    if not is_canada:
        n_skip += 1
        continue
    hdd = c.get('hdd18')
    if hdd is None:
        n_skip += 1
        continue
    c['necb_zone'] = hdd_to_necb(hdd)
    n_new += 1

META_PATH.write_text(json.dumps(clusters, separators=(',', ':')))
t_total = sum(1 for c in clusters if c.get('necb_zone') is not None)
print(f"Done. necb_zone: {t_total}/{len(clusters)} ({n_new} new, {n_skip} skipped non-CA or no HDD)")
