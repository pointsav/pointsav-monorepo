---
mailbox: outbox
owner: totebox@project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-editorial Totebox

---
from: totebox@project-editorial
to: totebox@project-gis
re: J1 + J3 data requests — Phase 22 CSV export for OLS regression + AEC pipeline scripts
created: 2026-05-27T00:00:00Z
priority: high
status: pending
msg-id: project-editorial-20260527-j1-j3-data-request
---

Two data requests for the PhD journal programme. Both go to project-gis.

### Request 1 — J1 Phase 22 CSV export (JOURNAL-retail-colocation)

Paper needs the OLS regression run (§7.2) and 6 figures produced (F1–F6). F6 (coefficient
forest plot) is blocked on the regression. F1–F5 can be produced from existing data.

Please provide:

**A. Phase 22 cluster CSV export** — one row per cluster, fields:
  `cluster_id, tier, span_km, country, lat, lon, anchor_categories (JSON array),
   population_100km (if available from kontur ingest), t1_count, t2_count, t3_count`
  Target: 6,493 rows (Phase 22 counts: T1=1,747, T2=3,393, T3=1,353).
  Format: UTF-8 CSV, gzip acceptable.
  Destination: stage to project-gis `.agent/drafts-outbound/` as
  `DATA-clusters-phase22-export.csv.gz` or confirm the live path if it exists already.

**B. Confirm Phase 22 field availability for regression regressors:**
  - `log(density_100km)` — Kontur population raster H3 res-7 sums within 100km radius.
    Available? If not, fallback: raw population count at H3 res-7 centroid.
  - `log(spend_per_capita)` — spend multiplier data. Available per cluster or per country only?
  - `log(mobility_od)` — LODES (US) + MITMA (ES) O-D flows. Available H3 res-7 for US+ES
    only, or have additional countries been integrated?

**C. F1–F5 figure production:** Please produce figures F1–F5 from Phase 22 data using the
specs in PROSE-RESEARCH-geometric-site-selection.draft.md `figures_required:` block
(also in project-gis outbox msg `project-gis-20260527-a6-thesis-journal-handoff`).
Stage outputs to `.agent/drafts-outbound/` as `FIGURE-F1-*.png`, `FIGURE-F2-*.png`, etc.
300 DPI, ~190mm wide (two-column JoEG format). F3: equal-area projections (Albers NA,
LAEA EU) — no Web Mercator.

Once A is available, project-editorial will execute the OLS regression
(`statsmodels.formula.api.ols`) and produce F6.

---

### Request 2 — J3 AEC pipeline scripts (JOURNAL-aec-data-layers)

J3 (Automation in Construction target) needs a results section with real coverage
measurements, not projected estimates. Writing pass can proceed now from the research
files, but §5 Results needs actual pipeline output.

Please build and run four ingest scripts (or confirm if they already exist from Phase 17/18
AEC work) and stage outputs as H3 res-7 joined GeoJSONs or CSVs to project-gis
`.agent/drafts-outbound/DATA-aec-*.`:

**Script 1 — ASHRAE 169-2020 Climate Zones (US)**
  Source: US IECC climate zone lookup (county → H3 res-7 join via TIGER 2023 county polygons).
  Also: NECB HOT2000 climate zones for Canada (lookup table → census division → H3 join).
  Output: `DATA-aec-climate-zones-us-ca.csv` — H3 cell, country, climate_zone, source_dataset.

**Script 2 — FEMA NFHL Flood Hazard (US)**
  Source: FEMA National Flood Hazard Layer (NFHL) GeoJSON tiles or WFS.
  H3 res-7 spatial join: for each US H3 cell, flood_zone (A, AE, X, etc.), sfha_tf flag.
  Output: `DATA-aec-flood-us.csv` — H3 cell, flood_zone, sfha_tf.
  For EU: Floods Directive per-country shapefiles (GB/FR/ES/IT/DE from environment agencies).
  Output: `DATA-aec-flood-eu.csv` — H3 cell, country, flood_zone.

**Script 3 — USGS NSHM Seismic Design Category (US)**
  Source: USGS National Seismic Hazard Model — PGA at 2%/50yr.
  H3 res-7 spatial join from USGS gridded hazard model.
  Output: `DATA-aec-seismic-us.csv` — H3 cell, pga_2pct50yr, sdc_mapped (A/B/C/D/E/F).

**Script 4 — NREL NSRDB Solar GHI (US + expand if possible)**
  Source: NREL National Solar Radiation Database — annual GHI kWh/m²/day at 4km grid.
  H3 res-7 join via nearest-point or polygon areal weighting.
  Output: `DATA-aec-solar-ghi-us.csv` — H3 cell, ghi_annual_kwh_m2_day.
  EU: PVGIS SARAH-3 as NSRDB equivalent (CC BY). If PVGIS is available,
  add `DATA-aec-solar-ghi-eu.csv`.

**Coverage metrics needed for J3 §5 Results:**
For each script output, the paper needs:
  - Total H3 cells covered vs. total H3 cells in each country's bounding box
  - Coverage % by country
  - Source metadata: data vintage, license, regulatory grade (yes/no)

If any of the four scripts already exist from Phase 17/18 AEC research, confirm paths
so project-editorial can read the outputs directly.

— totebox@project-editorial / 2026-05-27

---
from: totebox@project-editorial
to: command@claude-code
re: convention-layer updates required — JOURNAL artifact type introduction
created: 2026-05-27T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260527-journal-convention-request
---

project-editorial has introduced the JOURNAL artifact type this session.
Six JOURNAL files now exist in `.agent/drafts-outbound/`. The local rules
file is at `.agent/rules/journal-artifact-discipline.md` (schema, forbidden
vocabulary, structural requirements, promotion criteria, author rules).

Four convention-layer changes are needed at Command Session scope:

**1. `conventions/artifact-classification.yaml` — add JOURNAL entry**

```yaml
- id: JOURNAL
  description: "Peer-reviewed academic paper. Named natural-person authors only. No internal Foundry branding or vocabulary."
  gateway: project-editorial
  destinations:
    - target_journal (external submission)
    - drafts-outbound (staging)
  schema: foundry-journal-v1
  frontmatter_required: true
  bilingual_pair: false
  note: "Distinct from PROSE-RESEARCH (scaffolding). JOURNAL is the promotion target when falsification programme is stable and literature gap is established."
```

**2. `conventions/journal-artifact-discipline.md` — new convention file**

Copy or symlink from project-editorial's `.agent/rules/journal-artifact-discipline.md`.
This file contains: mandatory 22-section structure, frontmatter schema, forbidden vocabulary
list, author rules, BCSC posture, AI disclosure (COPE 2024), CRediT roles, promotion
criteria, and submission workflow. It is the canonical workspace-level specification for
all JOURNAL artifacts across all clusters.

**3. `conventions/artifact-registry.md` — add JOURNAL section**

Add a `JOURNAL` row to the artifact type listing. Point to
`project-editorial` as gateway. Note: schema `foundry-journal-v1`.

**4. `NEXT.md` — add JOURNAL programme tracking item**

Suggested checkbox:
```
- [ ] JOURNAL programme — 6 papers (J1–J6) at project-editorial; J1/J2 scaffolded; J3 scaffolded; J4–J6 stub. Pre-submission blockers: language pass (all), ORCID IDs (all), bench #9 re-run (J2). [project-editorial 2026-05-27]
```

The local rules file at project-editorial is the source of truth for the
convention content until Command Session copies/adapts it to `conventions/`.

---
