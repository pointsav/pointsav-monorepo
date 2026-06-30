#!/usr/bin/env python3
"""
build-wind-join.py — NASA POWER climatological annual mean wind speed per cluster
Task G — Phase 22 2026-06-30

Source: NASA POWER v2.6, climatology endpoint (free, no API key required)
WS100M = annual mean wind speed at 100m height (m/s)
URL: https://power.larc.nasa.gov/api/temporal/climatology/point?parameters=WS100M&community=RE&longitude={lon}&latitude={lat}&format=JSON

Rate limit: gentle — 3 concurrent workers with 0.4s delay.
6,117 clusters → ~15-25 min.
Saves: wind_speed_ms (float, 2 decimal) to clusters-meta.json
"""
import json, time, urllib.request, fcntl
from pathlib import Path
from threading import Thread, Lock
from queue import Queue

META_PATH = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
POWER_URL = "https://power.larc.nasa.gov/api/temporal/climatology/point?parameters=WS100M&community=RE&longitude={lon}&latitude={lat}&format=JSON"
WORKERS      = 1    # NASA POWER rate-limited at >1 req/s; use 1 worker + 10s
BATCH_DELAY  = 10.0

def fetch_wind(lat, lon):
    url = POWER_URL.format(lat=round(lat, 4), lon=round(lon, 4))
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'gis-research/1.0'})
        with urllib.request.urlopen(req, timeout=30) as r:
            data = json.loads(r.read())
        # Annual mean is in properties.parameter.WS100M.ANN
        val = data.get('properties', {}).get('parameter', {}).get('WS100M', {}).get('ANN')
        if val is None or val < 0 or val > 50:
            return None
        return round(float(val), 2)
    except Exception:
        return None

def worker(q, results, lock, progress, total):
    while True:
        item = q.get()
        if item is None:
            break
        cluster_id, lat, lon = item
        val = fetch_wind(lat, lon)
        with lock:
            if val is not None:
                results[cluster_id] = val
            progress[0] += 1
            done = progress[0]
            if done % 300 == 0 or done == total:
                filled = len(results)
                print(f"  Progress: {done}/{total} fetched, {filled} with wind data", flush=True)
        time.sleep(BATCH_DELAY)
        q.task_done()

print("Building NASA POWER wind speed join...")
clusters = json.loads(META_PATH.read_text())
total    = len(clusters)
already  = sum(1 for c in clusters if c.get('wind_speed_ms') is not None)
print(f"  {total} clusters; {already} already have wind_speed_ms")

# Use cluster_id as key (not array index) so results survive a clusters-meta.json rebuild
pending = [(c['id'], c['lat'], c['lon']) for c in clusters
           if c.get('wind_speed_ms') is None and 'lat' in c and 'lon' in c and 'id' in c]
print(f"  Fetching {len(pending)} clusters from NASA POWER...")

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

n_new = 0
lock_path = META_PATH.with_suffix('.lock')
with open(lock_path, 'w') as lf:
    fcntl.flock(lf, fcntl.LOCK_EX)
    clusters = json.loads(META_PATH.read_text())
    for c in clusters:
        cid = c.get('id', '')
        if cid in results:
            c['wind_speed_ms'] = results[cid]
            n_new += 1
    META_PATH.write_text(json.dumps(clusters, separators=(',', ':')))
t_total = sum(1 for c in clusters if c.get('wind_speed_ms') is not None)
print(f"\nDone.")
print(f"  wind_speed_ms: {t_total}/{len(clusters)} ({n_new} new)")
