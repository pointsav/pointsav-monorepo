---
schema: foundry-draft-v1
type: TEXT
slug: text-gis-data-methodology-dialog
title: "Data Methodology Dialog — gis.woodfinegroup.com"
language_protocol: project-editorial
status: draft
created: 2026-05-14
author: task@project-gis
destination: project-editorial
open_questions:
  - "Should the 35km primary radius note say 'provisional' or 'subject to calibration'?"
  - "Should spend multiplier sources (BLS/StatCan/Eurostat) be cited by name or described generically?"
research_trail:
  method: "Copy for the 'Data' button modal in index.html; mirrors DATA-MANIFEST.md in plain language"
  sources: ["DATA-MANIFEST.md", "topic-od-catchment-methodology.draft.md", "topic-trade-area-data-sources.draft.md"]
  assumptions: "Dialog is concise (modal); links to DATA-MANIFEST.md for full detail"
  gaps: "AWAY layer not yet available — noted in dialog"
  date: 2026-05-14
cites: []
---

# Data Methodology Dialog Copy

*Copy for the "Data" button modal on gis.woodfinegroup.com. Should be concise
enough for a modal dialog. Full detail is in DATA-MANIFEST.md.*

---

## Data Sources

**Business Locations**
Co-location retailers are sourced from OpenStreetMap via the Overpass API,
matched to Wikidata brand identifiers. Data is updated per pipeline run.

**Places (Airports, Hospitals, Universities)**
Civic point-of-interest data is sourced from Overture Maps and OpenStreetMap.

**Population**
Population estimates use the WorldPop 2026 100-metre global population grid.
Data is aggregated to H3 resolution 7 (≈5 km² hexagonal cells) and filtered
to the areas relevant to each co-location cluster.
Countries: United States, Canada, Mexico, Great Britain, Germany, France,
Netherlands, Austria, Portugal, Greece, Denmark, Iceland, Poland.

**Spend Potential**
Estimated annual retail spend is derived by applying per-capita expenditure
multipliers (sourced from national household expenditure surveys) to the
population grid. Spend is estimated for three categories: grocery, home
improvement / hardware, and wholesale / warehouse club. Values are in local
currency; cross-country comparisons are directional only.

---

## Trade Area Methodology

Each co-location cluster is assigned two catchment zones based on crow-flies
distance from the cluster centre:

- **Primary catchment** — within 35 km. Represents the immediate trade area.
- **Secondary catchment** — 35 km to 150 km. Represents the wider regional draw.

The 35 km primary boundary is a provisional threshold based on retail geography
conventions. It is subject to refinement as origin-destination data becomes
available.

Census and spend data is aggregated within each zone and used to rank
co-locations against each other.

---

## Catchment Views

**HOME** — catchment areas based on residential population (where people live).
This is the default view and uses WorldPop data.

**AWAY** — catchment areas based on daytime population (where people work).
This view is planned and will become available when workplace distribution
data is integrated.

---

## What the Map Shows

- **Census layer** — H3 hexagonal cells showing population density,
  displayed only within catchment areas (not the full data collection radius).
- **Spend layer** — H3 hexagonal cells showing estimated retail spend potential,
  displayed only within catchment areas.

The 150 km data radius shown on the map marks the boundary of data collection,
not the boundary of display.

---

[Full data manifest →](DATA-MANIFEST.md)
