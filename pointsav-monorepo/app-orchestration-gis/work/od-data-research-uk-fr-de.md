# OD Data Research: UK / France / Germany

**Created:** 2026-05-17
**Author:** project-gis Totebox (research session)
**Purpose:** Evaluate free, machine-readable origin-destination commuter data
sources for UK, France, and Germany for integration into the mobility pipeline
at H3 resolution 7 (~2.1 km cells).

**Current pipeline coverage:** US (LODES LEHD), CA (StatCan — URL broken),
ES (MITMA MNO open data).

---

## Summary Table

| Country | Source | Status | Resolution | Format | License | Commercial OK |
|---|---|---|---|---|---|---|
| UK | ONS / Nomis — ODWP01EW | **AVAILABLE** | MSOA (~7,200 areas) | CSV zip | UK OGL v3 | Yes |
| FR | INSEE RP2021 — FD_MOBPRO | **AVAILABLE** | Commune (~35,000) | CSV / Parquet | Licence Ouverte 2.0 (Etalab) | Yes |
| DE | BA Statistik — Pendler Gemeinde | **PARTIAL** — Kreise-level OD confirmed; Gemeinde-level matrix requires portal navigation | Kreis (district, ~400) or Gemeinde (~11,000) | Excel (XLSX) | BA copyright; free reuse with attribution | Yes (with citation) |

---

## 1. United Kingdom — ONS Census 2021 Commuter OD Data

### Source

Office for National Statistics (ONS) via Nomis — the official census and
labour market statistics portal.

**Source page (confirmed live):**
`https://www.nomisweb.co.uk/sources/census_2021_od`

**Dataset series:** ODWP (Origin-Destination Workplace Population)

### Key Dataset: ODWP01EW

- **Full name:** Location of usual residence and place of work
- **Description:** Commuting flows between usual residence and place of work
  for people aged 16+ in employment in the week before Census Day (27 March 2021).
- **Download size:** ~77 MB (zip containing CSV files by geographic type)
- **Estimated URL pattern:**
  `https://www.nomisweb.co.uk/output/census/2021/census2021-odwp01ew.zip`
  (Verify on the Nomis OD page above — zip structure mirrors the 2011 bulk
  pattern `nomisweb.co.uk/census/2011/bulk/rOD1`.)
- **Additional univariate datasets:** ODWP03EW (sex, 21 MB), ODWP04EW (age,
  30 MB), ODWP12EW (hours worked, 26 MB), ODWP14EW (car availability, 27 MB).
  For the OD signal we need only ODWP01EW.

### Geographic Resolution

- Primary geography: **MSOA** (Middle Layer Super Output Area)
  — approximately 7,200 areas in England and Wales; average population ~8,000.
- Some tables are available at **local authority** level only.
- Scotland: Census 2022 (one year later). NRS commuter OD data at Datazone
  level is **not yet released as of 2026-05-17** — expected 2025/2026.
  Scottish flows from/to England and Wales are also partially excluded from
  the England & Wales dataset due to the timing gap.
- Northern Ireland: Published separately by NISRA. NISRA has its own
  Census 2021 OD release:
  `https://www.nisra.gov.uk/publications/census-2021-origin-destination-place-work-tables-uk`

### Caveat on Data Quality

The 2021 Census was conducted under COVID-19 lockdown conditions and
furlough schemes. Travel-to-work patterns are **not representative of
normal commuting** — remote working was heavily elevated. This data is
best used as a structural signal (direction of flows, home-cell
distributions) rather than an absolute flow volume. Label accordingly
in any UI disclosure.

### Format

Each zip contains:
- One CSV per geographic level (OA, LSOA, MSOA, LA).
- First row: column headers.
- Data from row 2 onward.
- A metadata XLS file.

Typical columns: `Output_Areas_code`, `MSOA_code` (for residence MSOA),
`MSOA_of_workplace_code`, `Count`.

### Mapping to H3 res-7

MSOAs are polygon geographies. Two approaches:

1. **Centroid approach (recommended for speed):** Use the ONS Open Geography
   Portal MSOA centroid lookup
   (`https://geoportal.statistics.gov.uk/`) to get lat/lon per MSOA code,
   then call `h3.latlng_to_cell(lat, lon, 7)`. This assigns each MSOA to
   the single H3 cell containing its centroid.
2. **Polygon cover (more accurate for large MSOAs):** Use MSOA boundary
   GeoJSON + `h3.geo_to_cells(polygon, 7)` to get all H3 cells touched,
   then distribute flow counts proportionally by area overlap.

For OD aggregation the centroid approach is sufficient; MSOAs are small
relative to H3 res-7 cells (~2.1 km), and most MSOAs contain 1–3 H3
cells at res-7.

### License

**UK Open Government Licence v3 (OGL v3).**

Source: `https://www.ons.gov.uk/methodology/geography/licences`

OGL v3 permits:
- Free use, copy, distribute, and adapt.
- Commercial and non-commercial use.
- No royalty or fee.
- Attribution required: "Contains National Statistics data © Crown copyright
  and database right [year]."

No additional application required.

### Ingest Plan

```
1. DOWNLOAD
   wget https://www.nomisweb.co.uk/output/census/2021/census2021-odwp01ew.zip
   unzip → census2021-odwp01ew-msoa.csv

2. PARSE
   For each row (residence_msoa, workplace_msoa, count):
     - Look up centroid lat/lon for residence_msoa → h3_home
     - Look up centroid lat/lon for workplace_msoa → h3_work
     - Accumulate count onto (h3_home, h3_work) pair

3. CLUSTER JOIN
   For each UK cluster:
     - Identify h3_work cells within 150 km radius
     - For OD records with h3_work in that set, accumulate h3_home counts
     - Output JSONL matching ingest-lodes.py schema:
       {"h3": h3_home, "iso": "GB", "visits_work_total": float,
        "cluster_proximity": [...], "data_source": "ons_census2021",
        "is_measured": false}   ← flag false due to COVID distortion

4. OUTPUT
   service-fs/service-mobility/od-gb.jsonl
```

**Script name (to be created):** `ingest-ons.py`
**Pattern:** Mirror `ingest-lodes.py` structure (MSOA centroid lookup
replaces block crosswalk; zip HTTP fetch replaces state-by-state GZ loop).

---

## 2. France — INSEE RP2021 Mobilités Professionnelles

### Source

Institut national de la statistique et des études économiques (INSEE).

**Source page (confirmed live, 2021 vintage):**
`https://www.insee.fr/fr/statistiques/8205896`

**Source page (2020 vintage — slightly larger file, 8.08M rows):**
`https://www.insee.fr/fr/statistiques/7637844`

**Source page (2022 vintage — newest available):**
`https://www.insee.fr/fr/statistiques/8589904`

Recommend **2021 vintage** as the best balance of recency and row count.
The 2022 vintage was released 2024 and is the most current available.

### Dataset

- **Full name:** Mobilités professionnelles des individus — déplacements
  commune de résidence / commune de travail
- **File code (internal):** FD_MOBPRO (Fichier Détail Mobilités
  Professionnelles)
- **Format:** CSV (.csv) or Parquet (.parquet) — both available for 2021+.
  2020 vintage is CSV/DBF only.
- **Rows:** ~7.24 million observations (2021); ~8.08 million (2020)
- **Variables:** 32 columns per row
- **Key columns:** `COMMUNE` (commune of residence), `DCLT` (commune of
  work/destination), `IPONDI` (individual weight/expansion factor).
  These are the OD pair. Flows < 200 are noted as orders of magnitude.
- **Geographic scope:** All French communes (mainland + DOM), excluding
  Mayotte. Cross-border workers (Belgium, Switzerland, etc.) are recorded
  with a special destination code.

This is a **microdata individual file**, not a pre-aggregated OD matrix.
Each row is one individual's commute record. Aggregation to commune-pair
flow counts requires a groupby on (COMMUNE, DCLT) with sum of IPONDI.

### Geographic Resolution

**Commune** — approximately 34,800 communes in Metropolitan France + DOM.
Average commune population: ~2,000. Median commune is smaller (~400 people).

INSEE commune codes follow the COG (Code Officiel Géographique) — a
5-character code (2-digit department + 3-digit commune number for mainland;
3-digit overseas department prefix for DOM).

### Mapping to H3 res-7

INSEE publishes commune centroid files via the Code Officiel Géographique:
`https://www.insee.fr/fr/information/2560452`

The IGN also publishes ADMIN-EXPRESS-COG shapefiles with commune polygons and
pre-computed centroids (WGS84). Source: `https://geoservices.ign.fr/`

Approach:
```
commune_code → lat/lon centroid → h3.latlng_to_cell(lat, lon, 7)
```

Most rural communes are smaller than one H3 res-7 cell; urban communes
(Paris arrondissements, Lyon, Marseille) each map to a handful of cells.
For Paris/Lyon/Marseille, INSEE provides data at arrondissement municipal
(ARM) level, which should be used directly.

### License

**Licence Ouverte / Open Licence 2.0 (Etalab).**

Source: `https://www.data.gouv.fr/pages/legal/licences/etalab-2.0`

This is the French government's standard open data licence. It is
compatible with CC BY 2.0 and the OGL v2. Permits:
- Free use, reproduction, distribution.
- Commercial and non-commercial use.
- Adaptation and derived works.
- Attribution required: "Source: INSEE, [year]."

No registration or application required.

### Ingest Plan

```
1. DOWNLOAD
   # From INSEE statistics page — direct download link on page:
   wget "https://www.insee.fr/fr/statistiques/fichier/8205896/FD_MOBPRO21.zip"
   # Exact filename varies by vintage; verify on source page.
   # 2022 vintage: https://www.insee.fr/fr/statistiques/fichier/8589904/...

2. PARSE (microdata aggregation)
   import csv, collections
   flows = collections.Counter()
   for row in csv.DictReader(fd):
       origin_commune = row['COMMUNE']
       dest_commune   = row['DCLT']
       weight         = float(row['IPONDI'])
       flows[(origin_commune, dest_commune)] += weight
   # Result: ~300k–800k unique commune-pair flows after suppression

3. GEO JOIN
   Load commune centroid lookup: commune_code → (lat, lon)
   For each (origin, dest, count):
       h3_home = h3.latlng_to_cell(lat_origin, lon_origin, 7)
       h3_work = h3.latlng_to_cell(lat_dest, lon_dest, 7)
       accumulate count onto (h3_home, h3_work)

4. CLUSTER JOIN
   For each FR cluster:
       Identify h3_work cells within 150 km
       Aggregate h3_home counts
       Output JSONL:
         {"h3": h3_home, "iso": "FR", "visits_work_total": float,
          "cluster_proximity": [...], "data_source": "insee_rp2021",
          "is_measured": true}

5. OUTPUT
   service-fs/service-mobility/od-fr.jsonl
```

**Script name (to be created):** `ingest-insee.py`
**Pattern:** Microdata groupby aggregation → same H3 output schema as
ingest-lodes.py. The IPONDI expansion factor makes this a weighted count.

---

## 3. Germany — Bundesagentur für Arbeit (BA) Pendler Statistics

### Source

Statistik der Bundesagentur für Arbeit (Federal Employment Agency Statistics).

**Portal (confirmed live):**
`https://statistik.arbeitsagentur.de/DE/Navigation/Statistiken/Interaktive-Statistiken/Pendler/Pendler-Nav.html`

**Pendleratlas (interactive visualisation):**
`https://statistik.arbeitsagentur.de/DE/Navigation/Statistiken/Interaktive-Statistiken/Pendleratlas/Pendleratlas-Nav.html`

**Gemeindedaten download page:**
`https://statistik.arbeitsagentur.de/DE/Navigation/Footer/Top-Produkte/Gemeindedaten-sozialversicherungspflichtig-Beschaeftigter-Nav.html`

### Dataset

- **Population:** Sozialversicherungspflichtig Beschäftigte (employees
  subject to social insurance contributions). This covers roughly 80% of
  employed persons; civil servants (Beamte), self-employed, and
  mini-jobbers below the marginal employment threshold are excluded or
  handled separately.
- **Reference date:** June 30 of each year (annual snapshot).
- **Most recent:** June 2024 data (published late 2024).
- **OD content:** Origin (Wohnort = place of residence) × destination
  (Arbeitsort = place of work) commuter flows.

### Two Data Products — Key Distinction

**Product A — Gemeindedaten (confirmed free bulk download):**

Annual municipality-level tables of socially insured employees by Wohnort
and Arbeitsort. These are **marginal totals per Gemeinde** (in-commuter
count, out-commuter count, net balance) — not a full origin × destination
matrix.

Format: Excel (XLSX), multiple tabs. Available per district (Kreis) or
for all of Germany.

Known URL pattern (Bundesland × Bundesland aggregate, June 2020):
```
https://statistik.arbeitsagentur.de/Statistikdaten/Detail/202006/iiia6/
beschaeftigung-pendler-blxbl/blxbl-d-0-202006-xls.xlsx
```
The `202006` segment is YYYYMM. For June 2024, the analogous URL would be:
```
https://statistik.arbeitsagentur.de/Statistikdaten/Detail/202406/iiia6/
beschaeftigung-pendler-blxbl/blxbl-d-0-202406-xls.xlsx
```
(Not directly verified via WebFetch; pattern inferred from confirmed 2020
URL in search results.)

**Product B — Kreismatrizen / Kleinräumige Pendelrelationen (OD matrix):**

A true origin × destination commuter matrix at Kreis (district) level
(~400 districts). Documented in the BA's own methodology paper:
`https://statistik.arbeitsagentur.de/DE/Statischer-Content/Service/Statistik-angewendet/Statistische-Woche-2023/Generische-Publikationen/kleinraeumige-Pendelrelationen.pdf`

Full Gemeinde-level OD matrices (11,000+ municipalities) exist internally
but their **public bulk download format is not confirmed** via search. The
Pendleratlas provides municipality-level query access but does not expose
a flat OD CSV download. An independently maintained scraper exists at:
`https://git.nroo.de/norwin/pendleratlas` (not an official BA product).

### Assessment

| Product | OD matrix? | Resolution | Bulk CSV? | Status |
|---|---|---|---|---|
| Gemeindedaten (blxbl Excel) | No — marginal totals only | Bundesland × Bundesland | Yes — Excel | Confirmed free |
| Kreismatrizen | Yes — true OD | Kreis (~400) | Uncertain — may require portal | Partially confirmed |
| Gemeinde-level OD | Yes | Gemeinde (~11,000) | Not confirmed public | Likely portal-only |

### Geographic Resolution

- **Kreis (Landkreis/Stadtkreis):** ~401 districts. Average area ~900 km².
  Coarser than MSOA or commune. At H3 res-7 (~2.1 km cells), one Kreis
  typically spans 200–2,000 H3 cells.
- **Gemeinde:** ~11,000 municipalities. Average area ~32 km². More
  comparable to French commune resolution and more useful for H3 mapping.

Gemeindeschlüssel (AGS): 8-digit official municipality key, usable for
joins with BKG geodata:
`https://www.destatis.de/EN/Service/OpenData/maps-geodata.html`

BKG provides Verwaltungsgebiete shapefiles (VG250) at no charge under
dl-de/by-2-0, including Gemeinde boundaries with AGS codes.

### License

BA Statistik terms (from official citation guidance document):

> "Data and tables may be used without restriction. Information may be
> stored and passed on, duplicated, and distributed with source attribution.
> The contents may not be altered or falsified."

Attribution required: "© Statistik der Bundesagentur für Arbeit"
with a link to `statistik.arbeitsagentur.de`.

Commercial use is permitted. This is consistent with the
Datenlizenz Deutschland – Namensnennung – Version 2.0 (dl-de/by-2-0)
framework used by German federal agencies, though the BA has not formally
adopted the DL-DE label — their own terms govern.

### Ingest Plan (Kreis-level OD — recommended first step)

```
1. DOWNLOAD (Kreis OD matrix — confirm download URL via portal)
   Navigate: statistik.arbeitsagentur.de → Pendler → Kreismatrizen
   Download Excel or request via Datenanforderung form.
   Alternative: use the pendleratlas.de scraper as reference for API
   endpoint structure (https://statistik.arbeitsagentur.de/PendlerDaten).

2. PARSE
   Columns expected: Wohnort_ARS (AGS of residence), Arbeitsort_ARS
   (AGS of workplace), Einpendler (in-commuter count), Auspendler
   (out-commuter count).
   For OD matrix: (residence_AGS, workplace_AGS, count).

3. GEO JOIN
   Load BKG VG250 Kreis centroid table: AGS → (lat, lon)
   h3_home = h3.latlng_to_cell(lat, lon, 7)
   h3_work = h3.latlng_to_cell(lat, lon, 7)
   Note: At Kreis resolution, many Kreise map to the same H3 cell.
   The OD signal is coarser than for UK/FR — acceptable for catchment
   sizing but not for fine-grained home-cell distributions.

4. CLUSTER JOIN
   For each DE cluster:
       h3_work cells within 150 km → accumulate h3_home counts
       Output JSONL:
         {"h3": h3_home, "iso": "DE", "visits_work_total": float,
          "cluster_proximity": [...], "data_source": "ba_pendler_2024",
          "is_measured": true,
          "resolution_note": "Kreis-level; centroid assignment"}

5. OUTPUT
   service-fs/service-mobility/od-de.jsonl
```

**Script name (to be created):** `ingest-ba-pendler.py`

### Fallback Recommendation

If Kreis-level OD matrix download cannot be confirmed as a bulk file:
use **Kontur Population** (`https://www.kontur.io/data/population_density/`)
as a gravity model input — compute synthetic OD flows using
population-weighted distance decay, matching the approach available in
`synthesize-od-study.py`. The Kontur dataset is already at H3 res-8 (400m)
and is trivially aggregatable to res-7. This should be labeled
`"is_measured": false` and `"data_source": "kontur_gravity_model"`.

---

## 4. Priority Order and Implementation Recommendation

1. **France first (highest confidence):** The INSEE FD_MOBPRO file is a
   direct bulk download, commune-level, with clear licensing. France has
   3 T1 clusters (FR T1=3 as of Phase 7). The ingest can be completed in
   a single session.

2. **UK second (high confidence, coverage caveat):** ODWP01EW is a
   confirmed free bulk download from Nomis, MSOA-level, OGL-licensed.
   UK has 3 T1 clusters (GB T1=3). The COVID distortion flag must be
   set — `"is_measured": false` — or a companion note added in the
   methodology dialog. Scotland flows are unavailable.

3. **Germany third (medium confidence):** BA Gemeindedaten confirms
   free data exists, but the full Gemeinde-level OD matrix format and
   bulk download URL need verification via direct portal navigation.
   Start with Kreis-level OD if available, or use gravity model fallback.
   DE has T1=9 clusters after Phase 8.

---

## 5. Shared Infrastructure Needed

All three ingests benefit from a shared **EU/UK commune-centroid lookup
table** built once and reused:

```
build-eu-centroid-lookup.py
  Inputs:
    - UK: ONS Open Geography MSOA centroids (CSV from geoportal.statistics.gov.uk)
    - FR: IGN ADMIN-EXPRESS-COG commune centroids (Parquet/CSV from geoservices.ign.fr)
    - DE: BKG VG250 Gemeinde shapefile centroids (from destatis.de open geodata)
  Output:
    work/eu-centroid-lookup.json
    { "MSOA2021_code": {"lat": ..., "lon": ..., "country": "GB"},
      "75056": {"lat": ..., "lon": ..., "country": "FR"},   # Paris INSEE code
      "08111000": {"lat": ..., "lon": ..., "country": "DE"} # Stuttgart AGS
    }
```

This lookup is then used by `ingest-ons.py`, `ingest-insee.py`, and
`ingest-ba-pendler.py` identically, mirroring the block crosswalk
(`_xwalk.csv.gz`) pattern from `ingest-lodes.py`.

---

## 6. Source URLs Reference

| Resource | URL | Status |
|---|---|---|
| Nomis Census 2021 OD landing page | https://www.nomisweb.co.uk/sources/census_2021_od | Confirmed live |
| ONS Census 2021 OD flow description | https://www.ons.gov.uk/census/aboutcensus/censusproducts/origindestinationflowdata | Confirmed live |
| ONS OD user guide | https://www.ons.gov.uk/peoplepopulationandcommunity/populationandmigration/populationestimates/methodologies/userguidetocensus2021origindestinationdataenglandandwales | Confirmed live |
| ONS OGL licence page | https://www.ons.gov.uk/methodology/geography/licences | Confirmed live |
| NISRA NI Census 2021 OD | https://www.nisra.gov.uk/publications/census-2021-origin-destination-place-work-tables-uk | Confirmed live |
| ONS Open Geography Portal (MSOA centroids) | https://geoportal.statistics.gov.uk/ | Confirmed live |
| INSEE FD_MOBPRO 2021 | https://www.insee.fr/fr/statistiques/8205896 | Confirmed live |
| INSEE FD_MOBPRO 2022 (newest) | https://www.insee.fr/fr/statistiques/8589904 | Confirmed live |
| INSEE FD_MOBPRO 2020 | https://www.insee.fr/fr/statistiques/7637844 | Confirmed live |
| INSEE FD_MOBPRO documentation | https://www.insee.fr/fr/information/2383243 | Confirmed live |
| INSEE COG commune codes | https://www.insee.fr/fr/information/2560452 | Confirmed live |
| Etalab Licence Ouverte 2.0 | https://www.data.gouv.fr/pages/legal/licences/etalab-2.0 | Confirmed live |
| BA Statistik Pendler portal | https://statistik.arbeitsagentur.de/DE/Navigation/Statistiken/Interaktive-Statistiken/Pendler/Pendler-Nav.html | Confirmed live |
| BA Pendleratlas | https://statistik.arbeitsagentur.de/DE/Navigation/Statistiken/Interaktive-Statistiken/Pendleratlas/Pendleratlas-Nav.html | Confirmed live |
| BA Gemeindedaten page | https://statistik.arbeitsagentur.de/DE/Navigation/Footer/Top-Produkte/Gemeindedaten-sozialversicherungspflichtig-Beschaeftigter-Nav.html | Confirmed live |
| BA blxbl Excel (June 2020, pattern ref) | https://statistik.arbeitsagentur.de/Statistikdaten/Detail/202006/iiia6/beschaeftigung-pendler-blxbl/blxbl-d-0-202006-xls.xlsx | Confirmed (2020) |
| BA kleinräumige Pendelrelationen PDF | https://statistik.arbeitsagentur.de/DE/Statischer-Content/Service/Statistik-angewendet/Statistische-Woche-2023/Generische-Publikationen/kleinraeumige-Pendelrelationen.pdf | Confirmed live |
| Destatis maps/geodata (BKG VG250) | https://www.destatis.de/EN/Service/OpenData/maps-geodata.html | Confirmed live |
| Kontur Population (H3 res-8, fallback) | https://www.kontur.io/data/population_density/ | Confirmed live |
| pendleratlas.de scraper (unofficial) | https://git.nroo.de/norwin/pendleratlas | Confirmed live |

---

## 7. Open Questions Before Implementation

1. **UK ODWP01EW exact zip URL:** Verify the direct download URL on
   `nomisweb.co.uk/sources/census_2021_od`. The 2011 pattern was
   `nomisweb.co.uk/census/2011/bulk/rOD1`; the 2021 pattern may differ.
   A curl HEAD request will confirm without downloading the full 77 MB.

2. **INSEE FD_MOBPRO exact file URL:** The INSEE statistics page embeds a
   JavaScript download widget. Inspect network requests on the page to
   extract the direct ZIP URL (likely:
   `https://www.insee.fr/fr/statistiques/fichier/8205896/FD_MOBPRO21.zip`
   or similar pattern).

3. **BA Pendler OD matrix bulk format:** Navigate to
   `statistik.arbeitsagentur.de` → Beschäftigung → Pendler → Tabellen,
   and confirm whether a Kreis × Kreis OD matrix is downloadable as a
   single flat file. If only the interactive Pendleratlas is available,
   the scraper at `git.nroo.de/norwin/pendleratlas` documents the API
   call pattern.

4. **Scotland timing:** NRS has not released Census 2022 commuter OD data
   as of this research date. Flag GB ingest as England + Wales only.
   Revisit when NRS publishes (estimated 2025/2026).

5. **COVID flag strategy:** Decide whether to apply `"is_measured": false`
   globally to UK data, or to use UK data only for structural/directional
   signals (cluster catchment shape, not volume). The methodology dialog
   copy (artifact A4) should be updated when UK OD is ingested.
