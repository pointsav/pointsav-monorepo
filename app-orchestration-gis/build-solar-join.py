#!/usr/bin/env python3
"""
build-solar-join.py — PVGIS monthly irradiation → annual GHI per cluster
Task F — Phase 22 2026-06-30

Uses EU JRC PVGIS v5.2 monthly radiation API (free, no API key).
Endpoint: https://re.jrc.ec.europa.eu/api/v5_2/MRstc?lat={lat}&lon={lon}&outputformat=json

Returns monthly GHI (Wh/m²/month). Sum → convert to kWh/m²/yr.
Rate limit: 1-2 req/s sustainable. Uses 3 concurrent workers + 0.4s delay between batches.
6,117 clusters → ~15-25 min.

Saves: ghi_kwh_m2_yr (float, 1 decimal) to clusters-meta.json
"""
import json, time, urllib.request, urllib.error, fcntl
from pathlib import Path
from threading import Thread, Lock
from queue import Queue

META_PATH = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
# PVcalc with angle=0 (horizontal) gives H(i)_y = annual GHI in kWh/m²/yr
PVGIS_URL = "https://re.jrc.ec.europa.eu/api/v5_2/PVcalc?lat={lat}&lon={lon}&peakpower=1&loss=0&angle=0&aspect=0&outputformat=json"
WORKERS   = 2
BATCH_DELAY = 0.6

def fetch_ghi(lat, lon):
    """Returns annual GHI in kWh/m²/yr, or None on error."""
    url = PVGIS_URL.format(lat=round(lat, 4), lon=round(lon, 4))
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'gis-research/1.0'})
        with urllib.request.urlopen(req, timeout=30) as r:
            data = json.loads(r.read())
        # H(i)_y = annual global irradiation on horizontal plane (kWh/m²/yr)
        val = data.get('outputs', {}).get('totals', {}).get('fixed', {}).get('H(i)_y')
        if val is None or val <= 0:
            return None
        return round(float(val), 1)
    except Exception:
        return None

def worker(q, results, lock, progress, total):
    while True:
        item = q.get()
        if item is None:
            break
        idx, lat, lon = item
        ghi = fetch_ghi(lat, lon)
        with lock:
            if ghi is not None:
                results[idx] = ghi
            progress[0] += 1
            done = progress[0]
            if done % 200 == 0 or done == total:
                filled = sum(1 for v in results.values() if v is not None)
                print(f"  Progress: {done}/{total} fetched, {filled} with GHI data", flush=True)
        time.sleep(BATCH_DELAY)
        q.task_done()

print("Building PVGIS solar GHI join...")
clusters = json.loads(META_PATH.read_text())
total    = len(clusters)
already  = sum(1 for c in clusters if c.get('ghi_kwh_m2_yr') is not None)
print(f"  {total} clusters; {already} already have ghi_kwh_m2_yr")

# Build work queue (only clusters without GHI yet)
pending = [(i, c['lat'], c['lon']) for i, c in enumerate(clusters)
           if c.get('ghi_kwh_m2_yr') is None and 'lat' in c and 'lon' in c]
print(f"  Fetching {len(pending)} clusters from PVGIS...")

results  = {}
lock     = Lock()
progress = [0]
q        = Queue(maxsize=50)

threads = [Thread(target=worker, args=(q, results, lock, progress, len(pending)), daemon=True)
           for _ in range(WORKERS)]
for t in threads:
    t.start()

for item in pending:
    q.put(item)
for _ in threads:
    q.put(None)
for t in threads:
    t.join()

# Patch clusters
n_new = 0
lock_path = META_PATH.with_suffix('.lock')
with open(lock_path, 'w') as lf:
    fcntl.flock(lf, fcntl.LOCK_EX)
    clusters = json.loads(META_PATH.read_text())
    for idx, ghi in results.items():
        if ghi is not None:
            clusters[idx]['ghi_kwh_m2_yr'] = ghi
            n_new += 1
    META_PATH.write_text(json.dumps(clusters, separators=(',', ':')))

t_total = sum(1 for c in clusters if c.get('ghi_kwh_m2_yr') is not None)
print(f"\nDone.")
print(f"  ghi_kwh_m2_yr: {t_total}/{total} ({n_new} new)")
