#!/usr/bin/env python3
"""
build-temperature-join.py — Open-Meteo historical API temperature + HDD/CDD join
Task E — Phase 22 2026-06-30

Uses Open-Meteo archive API (free, no key needed) to fetch 2000-2020 daily
temperature_2m_mean, then computes:
  temp_annual_mean_c  — 2000-2020 mean (°C, 1 decimal)
  hdd18               — cumulative heating degree-days base 18°C (integer)
  cdd18               — cumulative cooling degree-days base 18°C (integer)

3 concurrent workers; 1.5s inter-request delay per worker.
6,117 clusters → ~50-60 min.
"""
import json, time, urllib.request, urllib.error, fcntl
from pathlib import Path
from threading import Thread, Lock
from queue import Queue

META_PATH   = Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
OM_URL      = ("https://archive-api.open-meteo.com/v1/archive"
               "?latitude={lat}&longitude={lon}"
               "&start_date=2000-01-01&end_date=2020-12-31"
               "&daily=temperature_2m_mean&timezone=UTC")
WORKERS     = 1     # single worker — Open-Meteo free tier: ~10k/day, 1 req per 7s safe
DELAY       = 7.0   # seconds between requests

def fetch_temp(lat, lon):
    """Returns (mean_c, hdd18, cdd18) or None on error."""
    url = OM_URL.format(lat=round(lat, 4), lon=round(lon, 4))
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'gis-research/1.0'})
        with urllib.request.urlopen(req, timeout=45) as r:
            data = json.loads(r.read())
        vals = data.get('daily', {}).get('temperature_2m_mean', [])
        vals = [v for v in vals if v is not None]
        if not vals:
            return None
        mean_c   = round(sum(vals) / len(vals), 1)
        n_years  = 21  # 2000–2020 inclusive
        hdd18    = int(sum(max(0.0, 18.0 - t) for t in vals) / n_years)
        cdd18    = int(sum(max(0.0, t - 18.0) for t in vals) / n_years)
        return mean_c, hdd18, cdd18
    except Exception:
        return None

def worker(q, results, lock, progress, total):
    while True:
        item = q.get()
        if item is None:
            break
        cluster_id, lat, lon = item
        result = fetch_temp(lat, lon)
        with lock:
            if result is not None:
                results[cluster_id] = result
            progress[0] += 1
            done = progress[0]
            if done % 200 == 0 or done == total:
                filled = len(results)
                print(f"  Progress: {done}/{total} fetched, {filled} with temperature data", flush=True)
        time.sleep(DELAY)
        q.task_done()

print("Building Open-Meteo temperature join (2000-2020 mean)...")
clusters = json.loads(META_PATH.read_text())
total    = len(clusters)
already  = sum(1 for c in clusters if c.get('temp_annual_mean_c') is not None)
print(f"  {total} clusters; {already} already have temp_annual_mean_c")

# Use cluster_id as key (not array index) so results survive a clusters-meta.json rebuild
pending = [(c['id'], c['lat'], c['lon']) for c in clusters
           if c.get('temp_annual_mean_c') is None and 'lat' in c and 'lon' in c and 'id' in c]
print(f"  Fetching {len(pending)} clusters from Open-Meteo archive (~{len(pending)*7//3600+1}h)...")

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
# Exclusive file lock: atomic read-modify-write; re-read file so we survive a mid-run rebuild
lock_path = META_PATH.with_suffix('.lock')
with open(lock_path, 'w') as lf:
    fcntl.flock(lf, fcntl.LOCK_EX)
    clusters = json.loads(META_PATH.read_text())
    for c in clusters:
        cid = c.get('id', '')
        if cid in results:
            mean_c, hdd, cdd = results[cid]
            c['temp_annual_mean_c'] = mean_c
            c['hdd18']              = hdd
            c['cdd18']              = cdd
            n_new += 1
    META_PATH.write_text(json.dumps(clusters, separators=(',', ':')))
t_total = sum(1 for c in clusters if c.get('temp_annual_mean_c') is not None)
print(f"\nDone.")
print(f"  temp_annual_mean_c: {t_total}/{len(clusters)} ({n_new} new)")
