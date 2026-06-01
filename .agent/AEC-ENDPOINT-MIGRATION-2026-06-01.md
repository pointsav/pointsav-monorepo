# AEC endpoint migration — verified current URLs (2026-06-01)

The flood/wildfire AEC builds failed because upstream endpoints moved (404) — not a
transient outage. This file records the **verified-from-VM** replacements. Probed live
from the workspace VM 2026-06-01; HTTP codes are this VM's actual results.

Companion: build-aec-flood.sh, build-aec-seismic.sh. Crons run from the clone path.

---

## SEISMIC — RESOLVED (endpoint recovered + fix already live)

- USGS ScienceBase recovered: catalog API HTTP 200. The dynamic-URL fix (commit 96385fca,
  promoted) resolves 9 zips for item `64ff886dd34ed30c2057b4d9`.
- **Refinement needed:** resolver takes `zips[0]`; prefer the zip whose name matches the
  2%/50yr PGA dataset (downstream `*2Pct*/*PGA*.shp` filter is only a partial safety net).
- **Action:** reschedule the one-shot seismic cron (the 2026-06-01 05:00 run fired with the
  pre-fix script and failed). Should succeed now.

## FLOOD — FEMA endpoint MIGRATED (verified working replacement)

Old (404): `hazards.fema.gov/nfhl/rest/services/public/NFHL/MapServer/exts/LargeFIRMDownloadControllerSoe/DownloadService/downloadFile?state=NN&filetype=gdb`

**NEW — verified 200, returns flood-zone polygons as GeoJSON:**
- Service root: `https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer` (v11.1)
- Flood-zone layer: **28** ("Flood Hazard Zones"); layer 27 = Flood Hazard Boundaries
- Query pattern (per-state via bbox, paginate with resultOffset):
  ```
  https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer/28/query
    ?where=1%3D1&geometry=<W,S,E,N>&geometryType=esriGeometryEnvelope&inSR=4326
    &outFields=FLD_ZONE,ZONE_SUBTY&returnGeometry=true&f=geojson&resultRecordCount=2000
  ```
- **Model change:** this is a REST polygon query (GeoJSON), NOT a GDB bulk download. The
  flood script's state-loop must change from "download GDB → ogr2ogr" to "bbox query →
  paginate → GeoJSON". US states iterated by bounding box (reuse metro/state bbox table).
- Legacy `hazards.fema.gov/nfhlv2/output/State/NFHL_NN_*.zip` = 404 (dead).
- MSC `msc.fema.gov/portal/downloadProduct?productID=NFHL_NNS` returned HTML, not a zip —
  product-ID format unconfirmed; do NOT use without verifying the real state product ID.

## WILDFIRE (JRC GWIS FWI) — no drop-in raster; needs operator decision

Old (404): `effis-gwis-cms.s3.eu-west-1.amazonaws.com/apps/country.profile/mean_fwi_m_1981_2010.tif`

The single mean-FWI GeoTIFF is gone. Current options (none is a no-auth drop-in GeoTIFF):
- **Copernicus CEMS Fire historical** (`ewds.climate.copernicus.eu`, DOI 10.24381/cds.0e89c522)
  — NetCDF, **requires free CDS account + API key (operator action)**; convert via gdal_translate.
- **CWFIS WCS** (`cwfis.cfs.nrcan.gc.ca/geoserver/public/wcs`, layer `public:fwi`) — GeoTIFF,
  no auth, but **Canada-only** coverage.
- **EFFIS/GWIS data-request form** — manual, ~1–2 wk turnaround.
- **Fallback:** keep the existing cached wildfire raster already on disk (usable; just not refreshed).
- **Recommendation:** register a Copernicus CDS key (operator) for a real global refresh, or
  accept the cached raster until then.

## EU INSPIRE flood (5 countries) — fragmented; only GB + IT are clean programmatic

| Country | Status | Verified endpoint |
|---|---|---|
| **GB** | ✅ clean (OGC API Features) | `https://environment.data.gov.uk/spatialdata/flood-map-for-planning-flood-zones/ogc/features/v1/collections/Flood_Zones_2_3_Rivers_and_Sea/items?f=json&limit=2000` (collection id verified) |
| **IT** | ✅ GeoPackage (1.05 GB) | `https://sdi.isprambiente.it/download_ogc/alluvioni/aree_pericolosita_idraulica_v5_2020_4258.gpkg` (HTTP 200, 1,127,239,680 B). WFS `sdi.isprambiente.it/geoserver/nz1/wfs` exposes risk-to-asset layers, not hazard polygons directly. |
| **FR** | ⚠ attributes only | `https://georisques.gouv.fr/api/v1/gaspar/azi?code_insee=NNNNN` (HTTP 200) returns JSON attributes, NOT geometry. Polygons via bulk SHP at georisques.gouv.fr/donnees/bases-de-donnees or WMS. |
| **ES** | ⚠ WMS / bulk only | SNCZI moved to `sig.miteco.gob.es`; WMS `wms.mapama.gob.es/sig/agua/...` for raster; feature-level WFS typenames unconfirmed. Use bulk SHP. |
| **DE** | ⚠ decentralized | No federal WFS; per-state services (e.g. Saxony `luis.sachsen.de/arcgis/services/wasser/...WFSServer`). BfG portal `geoportal.bafg.de/karten/HWRM_Aktuell/`. |

---

## Implementation note

build-aec-flood.sh needs a rewrite of the fetch layer (not just a URL swap):
1. FEMA: GDB-download loop → ArcGIS REST bbox GeoJSON query (layer 28), per state, paginated.
2. GB: swap to the OGC API Features items endpoint above.
3. IT: download the GeoPackage once (1 GB — fits the overnight window), read with ogr/fiona.
4. FR/ES/DE: bulk SHP/GeoPackage downloads or WMS — fragmented; lower priority. Document
   what's skipped (no silent truncation).
5. Wildfire: gated on CDS key decision (operator) or cached-raster fallback.

Verified by: totebox@project-gis, 2026-06-01 (live VM probes).
