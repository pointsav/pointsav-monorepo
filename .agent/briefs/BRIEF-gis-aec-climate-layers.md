---
artifact: brief
schema: foundry-brief-v1
status: active
brief-id: project-gis-aec-climate-layers
owner: project-gis
created: 2026-06-25
updated: 2026-06-30
---

# BRIEF: AEC / Climate Layers — Manual Pickup Queue

Runbook for filling in the AEC/climate fields in `clusters-meta.json`.
Operator runs these manually, one task at a time, at their own pace — no scheduled builds.

**Working directory for all commands:** `~/Foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/`

**clusters-meta.json path:** `~/Foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json`

**After each patch: always push to prod** (`push-to-prod.sh gis --checksum`) so the updated data is live.

---

## Current Coverage (2026-06-30, post Phase 22)

| Field | Coverage | Source ready? | Script ready? |
|---|---|---|---|
| `wildfire_hazard` (= fwi) | ✅ 6102/6118 (99%) | ✅ | ✅ build-fwi-join.py |
| `flood_hazard` | ⚠️ 801/6118 (13%) | ✅ | ✅ (build-aec-flood.sh re-run) |
| `koppen_class` | ✅ 6116/6118 (99%) | ✅ | ✅ build-koppen-join.py |
| `ecoregion_name` | ✅ 6088/6118 (99%) | ✅ | ✅ build-ecoregion-join.py |
| `ecoregion_biome` | ✅ 6088/6118 (99%) | ✅ | ✅ build-ecoregion-join.py |
| `wetland_class` | ✅ 33/6118 (0.5%) | ✅ VRT in deployment work/aec/ | ✅ build-wetland-join.py |
| `fwi_class` | ✅ 6102/6118 (99%) | ✅ gwis-fwi-global.geojson | ✅ build-fwi-join.py |
| `ashrae_zone` | ✅ 6118/6118 (100%) | derived from koppen | ✅ build-ashrae-zone.py |
| `seismic_pga_g` | ❌ 0/6118 | ⚠️ DATA FORMAT ISSUE (see §Seismic below) | ❌ blocked |
| `temp_annual_mean_c` | ❌ 0/6118 | ❌ needs download | ❌ needs new script |
| `hdd18` | ❌ 0/6118 | ❌ needs download | ❌ needs new script |
| `cdd18` | ❌ 0/6118 | ❌ needs download | ❌ needs new script |
| `ghi_kwh_m2_yr` | ❌ 0/6118 | ❌ needs download | ❌ needs new script |
| `wind_speed_ms` | ❌ 0/6118 | ❌ needs download | ❌ needs new script |
| `necb_zone` | ❌ 0/6118 | depends on hdd | ❌ needs new script |
| `eu_climate_zone` | ❌ 0/6118 | ❌ needs download | ❌ needs new script |

**Disk:** 14 GB free (92% full) as of 2026-06-30. Monitor before large downloads.

## Seismic data format issue (discovered 2026-06-30)

`eshm20-eu.geojson` (110K, in deployment work/aec/) = seismotectonic zone INPUTS (ZONE_ID, MAXMAG fields) — NOT PGA hazard output. Cannot be used for PIP seismic classification.

`usgs-nshm-pga-us.geojson` (66M, in clone work/aec/) = contour LineStrings with `Contour: -1000000.0` — these are NSHM contour line exports, not zone polygons. PIP doesn't work on lines.

`build-aec-seismic.sh` also has a USGS_TIF unbound-variable bug at step 3 that kills the script.

**To unblock:**
- EU seismic: download ESHM20 PGA output rasters from EFEHR (separate release, not in the source tarball)
  URL pattern: `https://hazard.efehr.org/` or EFEHR Zenodo record
- US seismic: download the actual NSHM 2023 PGA raster from ScienceBase item `64ff886dd34ed30c2057b4d9`
  (not the contour shapefile — the raster interpolation grid)
- Fix: Use `band.ReadRaster()` instead of `ReadAsArray()` (numpy 2.x compat) as done in build-koppen-join.py

**Disk:** 15 GB free (91% full) as of 2026-06-25. Monitor before large downloads.

---

## Per-Country Cluster Counts (for context)

| ISO | Clusters | Notes |
|---|---|---|
| US | 3,104 | Largest; seismic important for CA/PNW/NM |
| DE | 602 | EU |
| FR | 500 | EU |
| GB | 457 | EU |
| CA | 375 | NECB zone instead of ASHRAE; high HDD |
| MX | 284 | Seismic important (CDMX region) |
| ES | 212 | Seismic in SE; wildfire high |
| PL | 158 | EU |
| IT | 134 | EU; significant seismic |
| DK–NO | 67–10 | Nordic; low seismic |
| GR | 20 | High seismic |
| IS | 3 | Volcanic; extreme climate |

---

## TASK A — Köppen Class (Priority 1, fast, data already on disk)

**Why first:** `ashrae_zone` and `necb_zone` are derived from Köppen. All other climate
rows in the print report depend on it. Raster already downloaded.

**Source in work/aec/:**
- `koppen_geiger.tif` — Beck et al. 2018 raster (1 km resolution, CC BY 4.0)
- `koppen-simplified.geojson` — vector version already generated

**Script to write:** `build-koppen-join.py`

```python
#!/usr/bin/env python3
"""
build-koppen-join.py — Point-sample koppen_class for all clusters.
Reads koppen_geiger.tif, samples each cluster centroid, patches clusters-meta.json.

KOPPEN_RASTER values (Beck 2018):
  1=Af, 2=Am, 3=Aw, 4=BWh, 5=BWk, 6=BSh, 7=BSk,
  8=Csa, 9=Csb, 10=Csc, 11=Cwa, 12=Cwb, 13=Cwc,
  14=Cfa, 15=Cfb, 16=Cfc,
  17=Dsa, 18=Dsb, 19=Dsc, 20=Dsd,
  21=Dwa, 22=Dwb, 23=Dwc, 24=Dwd,
  25=Dfa, 26=Dfb, 27=Dfc, 28=Dfd,
  29=ET, 30=EF

Usage: python3 build-koppen-join.py [--dry-run] [--overwrite]
"""
import json, sys
from pathlib import Path
try:
    from osgeo import gdal, osr
    gdal.UseExceptions()
except ImportError:
    sys.exit("ERROR: osgeo/gdal not available — pip install gdal or apt-get install python3-gdal")

KOPPEN_INT_TO_CODE = {
    1:'Af', 2:'Am', 3:'Aw',
    4:'BWh', 5:'BWk', 6:'BSh', 7:'BSk',
    8:'Csa', 9:'Csb', 10:'Csc', 11:'Cwa', 12:'Cwb', 13:'Cwc', 14:'Cfa', 15:'Cfb', 16:'Cfc',
    17:'Dsa', 18:'Dsb', 19:'Dsc', 20:'Dsd', 21:'Dwa', 22:'Dwb', 23:'Dwc', 24:'Dwd',
    25:'Dfa', 26:'Dfb', 27:'Dfc', 28:'Dfd',
    29:'ET', 30:'EF',
}

SCRIPT_DIR = Path(__file__).parent
RASTER_PATH = SCRIPT_DIR / 'work/aec/koppen_geiger.tif'
META_PATH = Path('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json')

dry_run = '--dry-run' in sys.argv
overwrite = '--overwrite' in sys.argv

ds = gdal.Open(str(RASTER_PATH))
gt = ds.GetGeoTransform()
band = ds.GetRasterBand(1)

def sample_koppen(lon, lat):
    px = int((lon - gt[0]) / gt[1])
    py = int((lat - gt[3]) / gt[5])
    if px < 0 or py < 0 or px >= ds.RasterXSize or py >= ds.RasterYSize:
        return None
    val = band.ReadAsArray(px, py, 1, 1)
    if val is None: return None
    code = KOPPEN_INT_TO_CODE.get(int(val[0][0]))
    return code

with open(META_PATH) as f:
    clusters = json.load(f)

patched = 0
for c in clusters:
    if not overwrite and c.get('koppen_class'):
        continue
    v = sample_koppen(c['lon'], c['lat'])
    if v:
        c['koppen_class'] = v
        patched += 1

print(f"Patched {patched}/{len(clusters)} clusters with koppen_class")
if not dry_run:
    with open(META_PATH, 'w') as f:
        json.dump(clusters, f, separators=(',',':'))
    print("clusters-meta.json updated")
```

**Run:**
```bash
cd ~/Foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/
python3 build-koppen-join.py --dry-run   # preview
python3 build-koppen-join.py             # execute
```
Expected: ~6,100 clusters patched. Runtime < 2 minutes (in-memory raster read).

**Then immediately run ASHRAE zone:**
```bash
python3 build-ashrae-zone.py --dry-run
python3 build-ashrae-zone.py
```
This derives `ashrae_zone` from `koppen_class` (no download needed).

---

## TASK B — Ecoregion Name + Biome (Priority 1, data already on disk)

**Source:** `work/aec/ecoregions-global.geojson` — WWF Terrestrial Ecoregions (Olson 2001, CC BY 4.0).
File already in work/aec/ (it's used by `layer9-ecoregions-global.pmtiles`).

**Script to write:** `build-ecoregion-join.py`

```python
#!/usr/bin/env python3
"""
build-ecoregion-join.py — Point-in-polygon ecoregion join for all clusters.
Reads ecoregions-global.geojson (Shapely PIP), patches ecoregion_name + ecoregion_biome.

Usage: python3 build-ecoregion-join.py [--dry-run] [--overwrite]
"""
import json, sys
from pathlib import Path
try:
    from shapely.geometry import Point, shape
except ImportError:
    sys.exit("ERROR: shapely not available — pip install shapely")

SCRIPT_DIR = Path(__file__).parent
ECO_PATH = SCRIPT_DIR / 'work/aec/ecoregions-global.geojson'
META_PATH = Path('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json')

dry_run = '--dry-run' in sys.argv
overwrite = '--overwrite' in sys.argv

print("Loading ecoregions...", flush=True)
with open(ECO_PATH) as f:
    eco = json.load(f)
polys = [(shape(feat['geometry']), feat['properties']) for feat in eco['features']]
print(f"  {len(polys)} ecoregion polygons loaded")

with open(META_PATH) as f:
    clusters = json.load(f)

patched = 0
for i, c in enumerate(clusters):
    if not overwrite and c.get('ecoregion_name'):
        continue
    pt = Point(c['lon'], c['lat'])
    for poly, props in polys:
        if poly.contains(pt):
            c['ecoregion_name']  = props.get('ECO_NAME', '')
            c['ecoregion_biome'] = props.get('BIOME_NAME', '')
            patched += 1
            break

print(f"Patched {patched}/{len(clusters)} clusters with ecoregion")
if not dry_run:
    with open(META_PATH, 'w') as f:
        json.dump(clusters, f, separators=(',',':'))
    print("clusters-meta.json updated")
```

**Run:**
```bash
cd ~/Foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/
python3 build-ecoregion-join.py --dry-run
python3 build-ecoregion-join.py
```
Expected: ~5,800–6,000 clusters patched (ocean clusters = no ecoregion).
Runtime: 5–15 minutes (PIP against ~800 polygons, full set).

---

## TASK C — Seismic + Wetland (Priority 1, run existing script)

**Script:** `build-aec-seismic.sh`

**Known blockers:**
1. **ESHM20 EU seismic** — `maps.efehr.org` NXDOMAIN. Tarball `work/aec/eshm20-eu.tar.gz` may already be downloaded. Try:
   ```bash
   ls -lh work/aec/eshm20-eu.tar.gz
   tar -tzf work/aec/eshm20-eu.tar.gz | head -10   # check contents
   ```
   If tarball is intact, the script may proceed past the download step.
   Alternative source: `https://gitlab.seismo.ethz.ch/efehr/eshm20/-/archive/main/eshm20-main.tar.gz`

2. **USGS NSHM 2023** — already downloaded (`work/aec/usgs-nshm-pga-us.geojson` exists). Step 1 will be skipped (file check).

3. **NRCan CA seismic** — download is still live (NRCan Open Gov Licence).

**Run (after checking ESHM20 status):**
```bash
cd ~/Foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/
bash build-aec-seismic.sh --dry-run    # pre-flight check
bash build-aec-seismic.sh              # execute
```
Patches: `seismic_pga_g`, `wetland_class` in clusters-meta.json.
Produces: `layer10-seismic-eu.pmtiles` (improved if ESHM20 fix works).
Runtime: 20–40 minutes.

---

## TASK D — Flood Completeness (Priority 2)

**Why only 13%:** FEMA script ran with a `clusters.geojson` path issue. EU regulatory tiles are very small (151 KB) — likely incomplete.

**Run:**
```bash
cd ~/Foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/
bash build-aec-flood.sh --dry-run
bash build-aec-flood.sh
```
Disk requirement: ≥10 GB free. Check first: `df -h /srv/foundry`.
Runtime: 1–2 hours (FEMA REST queries + EU WFS).

Wildfire is already 99.7% complete — this re-run will not hurt it.

---

## TASK E — Temperature + HDD/CDD (Priority 2, download required ~800 MB)

**Source:** WorldClim v2.1 — free, CC BY 4.0.
- Mean annual temp: `https://geodata.ucdavis.edu/climate/worldclim/2_1/base/wc2.1_10m_tavg.zip` (~35 MB, 10-minute resolution)
- Monthly rasters needed for HDD18/CDD18: 12 × monthly mean temp layers

**Disk note:** Download ~35 MB, expand ~35 MB. Low footprint.

**Script to write:** `build-temperature-join.py`

Logic:
1. Download `wc2.1_10m_tavg.zip` (or `wc2.1_10m_tmin.zip` + `wc2.1_10m_tmax.zip` for full range)
2. Sample `tavg` band for each cluster centroid → `temp_annual_mean_c`
3. For HDD18: for each monthly mean temp T_m, contribution = max(0, 18 - T_m) × days_in_month; sum 12 months
4. For CDD18: sum max(0, T_m - 18) × days_in_month

**Canada note:** Canadian clusters would get `necb_zone` derived from HDD18:
- HDD < 3000 → NECB Zone 4
- HDD 3000–4000 → Zone 5
- HDD 4000–5000 → Zone 6
- HDD 5000–6000 → Zone 7A
- HDD > 6000 → Zone 7B/8

Once temperature data is in, run `build-ashrae-zone.py` again (it uses HDD as a tiebreaker).

---

## TASK F — Solar GHI (Priority 3, download required)

**Source:** PVGIS API (EU JRC, free, no API key needed):
```
https://re.jrc.ec.europa.eu/api/v5_2/seriescalc?lat={lat}&lon={lon}&outputformat=json
```
One API call per cluster = 6,117 calls. Rate limit ~1 req/s → ~2 hours.

Alternative: Global Solar Atlas bulk GeoTIFF download (~3 GB, once).
`https://globalsolaratlas.info/download/world` (free registration).

**Script to write:** `build-solar-join.py`

Recommend the Atlas bulk download to avoid rate-limit exposure. Save to `work/aec/gsa-ghi-global.tif`, then point-sample — same pattern as Köppen.

---

## TASK G — Wind Speed (Priority 3, download required)

**Source:** Global Wind Atlas 250m v3 (DTU/WB, free CC BY 4.0):
- Annual mean wind speed at 100m hub height
- Bulk GeoTIFF: `https://globalwindatlas.info/api/gis/global/wind-speed/100` (~2 GB)

**Script to write:** `build-wind-join.py` — same raster point-sampling pattern as Köppen.

---

## TASK H — NECB Zone for Canada (Priority 2, derived from Task E)

Run after Task E (HDD18 available).

**Script to write:** `build-necb-zone.py`

```python
NECB_ZONES = [
    (3000, '4'),
    (4000, '5'),
    (5000, '6'),
    (6000, '7A'),
    (float('inf'), '7B'),
]
for c in clusters:
    if c.get('iso') != 'CA': continue
    hdd = c.get('hdd18')
    if hdd is None: continue
    for threshold, zone in NECB_ZONES:
        if hdd < threshold:
            c['necb_zone'] = zone
            break
```

---

## TASK I — EU Climate Zone (Priority 3)

**Source:** EU Energy Performance Buildings Directive climate zones (Peel 2007 CEN).
Available as a shapefile from the JRC: `https://data.jrc.ec.europa.eu/dataset/jrc-10111-10001`

Only relevant for EU clusters (DE/FR/GB/ES/IT/PL/DK/NL/FI/PT/SE/GR/AT/NO/IS).

---

## After Each Task — Update + Push

```bash
# 1. Verify patch count
python3 -c "
import json
with open('/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json') as f:
    d = json.load(f)
fields = ['koppen_class','ecoregion_name','seismic_pga_g','wetland_class',
          'temp_annual_mean_c','hdd18','ghi_kwh_m2_yr','wildfire_hazard','flood_hazard']
for k in fields:
    n = sum(1 for c in d if c.get(k))
    print(f'{k}: {n}/{len(d)}')
"

# 2. Push to prod (tiles unchanged, only clusters-meta.json)
~/Foundry/bin/push-to-prod.sh gis --checksum
```

---

## Recommended Sequence

| Order | Task | Disk req | Time | Data on disk? |
|---|---|---|---|---|
| 1 | **A — Köppen join** | none | < 2 min | ✅ yes |
| 1 | **A — ASHRAE derive** | none | < 1 min | ✅ yes |
| 2 | **B — Ecoregion join** | none | 10–15 min | ✅ yes |
| 3 | **C — Seismic + Wetland** | < 1 GB | 20–40 min | ✅ mostly |
| 4 | **D — Flood re-run** | ≥10 GB | 1–2 hr | ✅ mostly |
| 5 | **E — Temperature/HDD** | ~100 MB | 30 min | ❌ download |
| 5 | **H — NECB zone (CA)** | none | < 1 min | depends on E |
| 6 | **F — Solar GHI** | ~3 GB | 30 min + build | ❌ download |
| 7 | **G — Wind speed** | ~2 GB | 30 min + build | ❌ download |
| 8 | **I — EU climate zone** | ~100 MB | 20 min | ❌ download |

Tasks A + B are the best "pick at before bed" sessions — no downloads, no large disk, fast results visible immediately in the print report.

---

## Research Trail

- Coverage audit: `python3` inline query 2026-06-25 — confirmed all AEC fields 0 except wildfire (99.7%) + flood (13%)
- work/aec/ inventory: confirmed koppen_geiger.tif, ecoregions-global.geojson, eshm20-eu.tar.gz, usgs-nshm-pga-us.geojson, gwl-fcs30-mosaic.vrt all present
- Scripts: build-aec-seismic.sh + build-aec-flood.sh + build-ashrae-zone.py all exist and correct
- Missing scripts: Köppen join, ecoregion join, temperature, solar, wind, NECB zone
