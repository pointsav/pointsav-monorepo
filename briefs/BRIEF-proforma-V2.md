---
artifact: brief
name: proforma-V2
status: archived
archived: 2026-05-23
superseded_by: BRIEF-tool-proforma-leapfrog-2030
created: 2026-05-22
updated: 2026-05-22
archive: project-proforma
---

## Implementation Status (2026-05-22)

All three deliverables implemented in `tool-proforma-engine` (Rust):
- **D1 — Development Classes** (`dev-classes` subcommand): 4-class 10-year IS/BS/CF from TitleCo 3 Excel.
- **D2 — Direct-Hold Solution** (`direct-hold` subcommand): 10-year IS/CF/BS + Financial Forecast from PCLP 1 Excel.
- **D3 — WCP Inc.** (`wcp` subcommand): 10-year IS/BS + Revenue Generator + Valuation Matrix from WCP 42M Excel.

Outputs at `/srv/foundry/clones/project-proforma/outputs/` (markdown + HTML).
Stage 6 pending — commits `cd29776b` + `f76d3e87` on `cluster/project-proforma`.

**Open items:**
- Number audit: verify engine-computed values match Excel tab-by-tab.
- LP name abbreviation in D3 Revenue Generator table.
- D1 entity name override (TitleCo file entity vs. target report header).

---


# Brief — Proforma V2

We need to take a step back on the RUST ENGINE

I am looking for this to look like a markdown file even after it has been rendered.

This is for internal use only with management to look at the numbers to make adjustments to the RUST ENGINE rather then the on the report or marketing, which is pointless if the numbers are not being generated from the RUST ENGINE and/or are not correct.

We are going to audit the numbers by hand to make sure the RUST ENGINE is calculating the numbers. We don't want to just be rendering the numbers from the inputs, the number must be computing from the RUST ENGINE. 

DELIVERABLE #1 - Development Classes

We need the RUST ENGINE to generate a one page: 10 year Balance Sheet, Income Statement, Cash Flow Statement for each development class.

This needs to be simple. 
Four sheets of Landsacpe Paper schema - one sheet per development class.
No notes

Development Classes:
Professional Centres
Suburban Office
Tech Industrial
Retail Select

Floor plate assumptions: 
Suburban Office = 19,000 sqft per floor  
Professional Centres = 21,000 sqft per floor  
Tech Industrial = single-storey (7,200 or 8,400 sqft per building) Tech Industrail buildings are always constructed and leased in pairs.
Retail Select = single-storey (4,500 / 6,700 / 7,700 sqft per building)

*need to determine if there is a way to show the different sizes for Tech Industrial and Retail Select without separate balance sheets.

Input files: 
DUE DILIGENCE_TitleCo 3_2026_01_06_Forecast_Development Proforma_Test Site_Grande Prairie_FIN


DELIVERABLE #2 - Direct-Hold Solution
We need the RUST ENGINE to generate a one page: 10 year Balance Sheet, Income Statement, Cash Flow Statement for direct-hold solutions

Extra Page: Financial Forecast
needs to be 100% match, verbatum, to AA12:AN35

No sensivity analysis needed.

Input file:
DUE DILIGENCE_PCLP 1_2026_01_06_Forecast_250M_Cash Flow and Valuation_FIN


DELIVERABLE #3 - Woodfine Capital Projects Inc.

We need the RUST ENGINE to generate a one page: 10 year Balance Sheet, Income Statement, Cash Flow Statement for Woodfine Capital Projects Inc.

Extra Page 1: Revenue Generator
shows the number of funds
When each fund starts to cash flow

Extra Page 2: Valuation Matrix

This needs to be simple. 
1 sheet of Landsacpe Paper schema + extra revenue generator + valuation matrix
No notes

Input file:
COMPLIANCE_WCP_2026_01_08_Forecast_42M_Cash Flow and Valuation_FIN



