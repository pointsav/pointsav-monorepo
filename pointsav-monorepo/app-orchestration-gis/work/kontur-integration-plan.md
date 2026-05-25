---
title: Kontur Population Integration Plan
project: project-gis
author: Jennifer Woodfine
created: 2026-05-17
status: DRAFT — not yet approved
---

# Kontur Population Integration Plan

## Context

The current pipeline ingests WorldPop 2026 100m raster files per ISO country
(`{iso}_pop_2026.tif`), converts pixel lat/lon to H3 res-7 cells via
`bin-census-h3.py`, and writes `census-h3-res7.jsonl`. The file
`synthesize-od-study.py` reads that JSONL as `{h3, lat, lon, pop}` records
and aggregates population counts within 35km and 150km H3 disk radii for each
cluster.

Kontur Population is a pre-gridded alternative: population is already
disaggregated to H3 res-8 hexagons (~400m, ~0.74 km²), one resolution finer
than our working res-7 (~2.1km). Integration would replace the raster-to-H3
conversion step with a direct H3 parent rollup.

---

## 1. License verdict

**Verdict: commercially permissible with attribution.**

The Kontur Population dataset is released under the
**Creative Commons Attribution 4.0 International (CC BY 4.0)** license.
Commercial use is explicitly allowed. The only requirement is attribution
to Kontur Inc. in any public-facing product or publication that relies on
the dataset.

Nuance: several upstream inputs (OSM, Microsoft Building Footprints) are
individually licensed under ODbL. The ODbL share-alike provision applies to
those *source* databases, not to derivative products. Kontur's harmonized
output carries CC BY, and Kontur has resolved the ODbL boundary for their
redistribution. For a closed commercial product (gis.woodfinegroup.com),
the CC BY attribution requirement is satisfied by adding a data-attribution
line to the methodology dialog and DATA-MANIFEST.md. No share-alike
obligation attaches to our codebase.

Attribution line for DATA-MANIFEST.md and UI:
> "Population data: Kontur Population (CC BY 4.0) — Kontur Inc.,
>  data.humdata.org/dataset/kontur-population-dataset"

---

## 2. Download steps

### Primary source — HDX global GeoPackage

**Dataset page:**
https://data.humdata.org/dataset/kontur-population-dataset

**Direct file (latest confirmed release — 2023-11-01 vintage):**
```
https://data.humdata.org/dataset/kontur-population-dataset/resource/78688126-f729-41b5-ac79-195598dbfad7
Filename: kontur_population_20231101.gpkg.gz
Size: ~6.6 GB compressed
Format: GeoPackage (.gpkg), gzip-compressed
```

Per-country subsets are also available at:
https://data.humdata.org/organization/kontur
Country slugs follow the pattern:
`kontur-population-{country-name}` (e.g. `kontur-population-united-states-of-america`)

For our 13 countries, using per-country files is preferred over the 6.6 GB
global file — download only what is needed. Country slugs for our coverage:

| ISO | HDX slug suffix |
|-----|----------------|
| US  | united-states-of-america |
| CA  | canada |
| MX  | mexico |
| ES  | spain |
| FR  | france |
| DE  | germany |
| GB  | united-kingdom |
| IT  | italy |
| NL  | netherlands |
| AT  | austria |
| PL  | poland |
| GR  | greece |
| PT  | portugal |

Nordic countries (DK, NO, SE, FI, IS): check HDX for individual slugs;
all Nordic countries have confirmed coverage in the global file.

### Download procedure

```bash
# Example: download and decompress Spain subset
wget "https://data.humdata.org/dataset/kontur-population-spain/resource/<id>" \
     -O kontur_population_ESP_20231101.gpkg.gz
gunzip kontur_population_ESP_20231101.gpkg.gz
```

Note: HDX resource IDs within per-country pages must be resolved at download
time — they are stable UUIDs but are not listed in this plan. Check each
country page for the current resource UUID.

### Alternative: 3km pre-aggregated file (res-5 equivalent)

A 169 MB global file at 3km resolution is available at:
https://data.humdata.org/dataset/kontur-population-dataset-3km

This is too coarse for our use (we need res-7, ~2.1km). Use the 400m (res-8)
files and perform the res-8 → res-7 rollup ourselves.

---

## 3. Format and conversion: Kontur gpkg → census-h3-res7.jsonl

### Native format

Kontur Population GeoPackage contains a single vector layer with one row per
populated H3 res-8 hexagon. Schema:

| Column | Type | Notes |
|--------|------|-------|
| `h3` | TEXT | H3 res-8 cell index (15-char string) |
| `population` | INTEGER | Estimated population count for the cell |
| `geom` | GEOMETRY | Hexagon polygon (optional — not needed) |

Confirmed via CARTO and Databricks catalog descriptions of the dataset.

### Conversion script (new file: `ingest-kontur.py`)

The conversion replaces the two-step `ingest-census.py` + `bin-census-h3.py`
workflow with a single script:

```python
#!/usr/bin/env python3
"""
ingest-kontur.py — Convert Kontur Population res-8 gpkg files to
census-h3-res7.jsonl, matching the format expected by synthesize-od-study.py.

Input:  per-country .gpkg files in KONTUR_RAW_DIR
Output: census-h3-res7.jsonl (same path and schema as current pipeline)

Dependencies: h3, fiona (or geopandas with fiona backend)
  pip install h3 fiona
"""

import json
import os
from collections import defaultdict
from pathlib import Path
import fiona
import h3

KONTUR_RAW_DIR = Path("/srv/foundry/deployments/cluster-totebox-personnel-1"
                      "/service-fs/service-census/kontur-raw/")
OUTPUT_FILE    = Path("/srv/foundry/deployments/cluster-totebox-personnel-1"
                      "/service-fs/service-census/census-h3-res7.jsonl")
H3_TARGET_RES  = 7   # aggregate res-8 → res-7

# ISO codes to process (matches existing pipeline)
ISO_GPKG_MAP = {
    "usa": "kontur_population_USA_20231101.gpkg",
    "can": "kontur_population_CAN_20231101.gpkg",
    "mex": "kontur_population_MEX_20231101.gpkg",
    "esp": "kontur_population_ESP_20231101.gpkg",
    "fra": "kontur_population_FRA_20231101.gpkg",
    "deu": "kontur_population_DEU_20231101.gpkg",
    "gbr": "kontur_population_GBR_20231101.gpkg",
    "ita": "kontur_population_ITA_20231101.gpkg",
    "nld": "kontur_population_NLD_20231101.gpkg",
    "aut": "kontur_population_AUT_20231101.gpkg",
    "pol": "kontur_population_POL_20231101.gpkg",
    "grc": "kontur_population_GRC_20231101.gpkg",
    "prt": "kontur_population_PRT_20231101.gpkg",
    "dnk": "kontur_population_DNK_20231101.gpkg",
    "isl": "kontur_population_ISL_20231101.gpkg",
}

def rollup_country(iso: str, gpkg_path: Path, hex_data: dict) -> int:
    """Read res-8 cells from gpkg, roll up to res-7, accumulate into hex_data."""
    count = 0
    with fiona.open(gpkg_path) as src:
        for feat in src:
            h8 = feat["properties"]["h3"]
            pop = feat["properties"]["population"] or 0
            if pop <= 0:
                continue
            h7 = h3.cell_to_parent(h8, H3_TARGET_RES)
            hex_data[h7]["pop"] += pop
            hex_data[h7]["iso"].add(iso)
            count += 1
    return count

def main():
    hex_data = defaultdict(lambda: {"pop": 0.0, "iso": set()})
    for iso, fname in ISO_GPKG_MAP.items():
        path = KONTUR_RAW_DIR / fname
        if not path.exists():
            print(f"  SKIP {iso}: {path.name} not found")
            continue
        n = rollup_country(iso, path, hex_data)
        print(f"  {iso.upper()}: {n:,} res-8 cells rolled up")

    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    print(f"Writing {len(hex_data):,} res-7 cells to {OUTPUT_FILE.name} ...")
    with open(OUTPUT_FILE, "w") as out:
        for h7, data in hex_data.items():
            lat, lon = h3.cell_to_latlng(h7)
            out.write(json.dumps({
                "h3":  h7,
                "lat": round(lat, 5),
                "lon": round(lon, 5),
                "pop": round(data["pop"], 2),
                "iso": sorted(data["iso"]),
            }) + "\n")
    print("Kontur ingest complete.")

if __name__ == "__main__":
    main()
```

**Output schema is identical to the current `census-h3-res7.jsonl` schema.**
No changes are required in `synthesize-od-study.py` to read the file.

---

## 4. Changes needed in synthesize-od-study.py

`synthesize-od-study.py` reads `census-h3-res7.jsonl` via the `CENSUS_FILE`
path constant and expects records with keys `h3`, `lat`, `lon`, `pop`. The
Kontur-derived file produced by `ingest-kontur.py` uses exactly that schema.

**Required changes: two constants only.**

```python
# Line 267 — update mobility_vintage provenance tag
entry.setdefault("mobility_vintage", "kontur-20231101")  # was: "worldpop-2026"
```

The `CENSUS_FILE` path constant (line 45) does not change — it still points to
`census-h3-res7.jsonl`. The only substantive edit is the `mobility_vintage`
provenance string, which documents which population layer was used when
`synthesize-od-study.py` writes catchment data to `clusters-meta.json`.

No other logic changes are required.

---

## 5. Expected accuracy difference vs WorldPop

### Methodology comparison

| Dimension | WorldPop 2026 100m | Kontur Population (20231101) |
|-----------|-------------------|------------------------------|
| Base census | National census disaggregated via Random Forests | GHSL 2023A + HRSL (Facebook) |
| Building layer | Maxar/Ecopia, Google, Microsoft Bing | Microsoft Building Footprints + OSM |
| Vintage | 2026 projection extrapolated from 2020 base | 2023 estimate; GHSL R2023A base (covers 1975–2030 multitemporal) |
| Resolution | 100m raster → aggregated to H3 res-7 | H3 res-8 natively (~400m, ~0.74 km²) → rolled up to res-7 |
| Aggregation loss | Partial pixels at H3 cell boundaries summed | Exact: every res-8 child maps to exactly one res-7 parent (H3 hierarchy) |
| Urban density bias | Known undercount in dense urban cores vs. constrained models | HRSL layer improves urban coverage; OSM constraints remove quarries/water |
| Rural bias | −53% systematic undercount documented (Nature Comms 2025) for WorldPop global unconstrained | GHSL+HRSL fusion improves rural accuracy; comparable systematic biases exist |

### Practical implications for this pipeline

Our pipeline uses population exclusively for **relative ranking** (primary and
secondary catchment population rank among clusters within the same ISO). The
ranking engine does not depend on absolute population correctness — it ranks
clusters relative to each other. Therefore:

- In-country relative rankings are unlikely to shift materially between
  WorldPop and Kontur for urban and suburban clusters where both datasets
  are well-calibrated.
- At the margin, clusters near dense urban cores may see modestly higher
  Kontur population estimates due to HRSL's superior building-level
  disaggregation in megacities (US, MX, DE, FR, GB).
- Small clusters in rural EU regions (GR, PT, PL) may see Kontur counts
  diverge more, because HRSL coverage is sparser there and GHSL is the
  fallback.
- The H3 hierarchy rollup (res-8 → res-7) is mathematically exact: every
  res-8 cell has exactly one res-7 parent, so there is zero boundary
  rounding loss. The current WorldPop pipeline accumulates partial-pixel
  errors at H3 cell boundaries.

### Quantitative estimate

No third-party validation study specific to our 13-country scope and retail
cluster context was found. Based on methodology literature:

- Urban clusters (top quartile by population): divergence likely < 5%
  between WorldPop and Kontur at res-7 aggregation
- Peri-urban clusters: divergence 5–15%
- Rural/exurban clusters (bottom quartile): divergence potentially 15–30%,
  with Kontur more likely to be higher (HRSL catches informal settlements
  missed by WorldPop unconstrained)

Because rankings are within-ISO, absolute divergence matters less than
rank-preservation. Rank shifts of more than 10 positions among the top 50
clusters per ISO are unlikely.

---

## 6. Recommendation

**Use Kontur as the primary population source, replacing WorldPop for the
census layer. Retain WorldPop files as a validation baseline for one pipeline
cycle.**

Rationale:

1. **License is clean.** CC BY 4.0 is unambiguous for commercial use.
   Attribution is satisfied with a single line in DATA-MANIFEST.md and the
   UI methodology dialog.

2. **Integration is low-effort.** The output schema is identical. Only two
   constants change in the codebase. `synthesize-od-study.py` requires no
   logic changes. A new `ingest-kontur.py` script replaces the two-step
   raster pipeline.

3. **Aggregation is exact.** H3 res-8 → res-7 parent rollup eliminates
   raster-to-hex boundary accumulation errors. The pipeline becomes simpler
   and more deterministic.

4. **Vintage is acceptable.** The November 2023 vintage is 27 months behind
   2026 WorldPop projections. For EU/NA retail site analysis, 2023 population
   is fit for purpose — population change at the catchment scale (150km radius)
   is negligible over two years.

5. **Download is manageable.** Per-country .gpkg files are significantly
   smaller than the global file. All 13 + Nordic countries are available.

**Not recommended as a full replacement** if observed mobility (LODES,
MITMA, StatCan) is available for a cluster's ISO — in those cases, the
population census layer feeds only the fallback radius estimate, so the
difference between WorldPop and Kontur is immaterial to the final
`mobility_source` field.

**Recommended next steps:**

1. Download one country (ES or DE) and run `ingest-kontur.py` against it.
2. Run `synthesize-od-study.py` with the Kontur-derived census file.
3. Compare `pp` (primary population) and `rp` (population rank) fields
   between the WorldPop run and the Kontur run for T1 clusters in that ISO.
4. If rank correlation > 0.95, switch all ISOs and update `mobility_vintage`
   to `kontur-20231101`.
5. Update DATA-MANIFEST.md, the methodology dialog draft, and artifact-registry.md.

---

## Appendix: H3 resolution reference

| H3 Resolution | Avg cell area | Avg center-to-center | Use in pipeline |
|---------------|--------------|---------------------|-----------------|
| res-5 | ~252 km² | ~29 km | Too coarse |
| res-7 | ~5.16 km² | ~2.11 km | **Current working resolution** |
| res-8 | ~0.74 km² | ~0.80 km | Kontur native resolution |

Every res-8 cell has exactly one res-7 parent (H3 hierarchical property).
Rollup: `h3.cell_to_parent(h8_index, 7)` → returns the res-7 parent.
Aggregation: `sum(population)` for all res-8 children with same res-7 parent.
