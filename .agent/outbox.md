---
mailbox: outbox
owner: task@project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-gis

---
from: task@project-gis
to: command@foundry
re: Phase 11 complete — T1 recalibration + analysis sprint results
created: 2026-05-17T19:40:00Z
---

Phase 11 committed (adbb5d42, jwoodfine). All autonomous workstreams complete.
Summary of findings and operator decisions needed:

## 1. T1 threshold — DONE; 500 target needs operator decision

Applied P10=0.20 (was 0.10). **T1 now 305** (from 157).

Simulation summary with IoU enforced (sim-1b):
| P | Civic gate | NA T1 | EU T1 | Total |
|---|---|---|---|---|
| 0.20 | current (hc_count_regional≥1) | 245 | 60 | **305** ← APPLIED |
| 0.20 | any hospital (hc_count≥1) | 278 | 63 | 341 |
| 0.25 | current | 310 | 75 | 385 |
| 0.25 | any hospital | 358 | 79 | **437** |
| 0.25 | no civic | 439 | 80 | 519 |

**To reach ~500 NA T1, operator choices:**
- **Option A** — P=0.25 + civic Gate B (any hospital): NA≈358, total≈437. Clean; retains civic context.
- **Option B** — Add composition Path C (HW∧HM as T1-qualifying): adds ~199 NA + 103 EU at P=0.20.
  Promotes hardware+hypermarket clusters (Lowe's+Walmart, HomeDepot+Tesco) to T1 alongside Costco+Walmart.
  Methodologically a significant change — requires review of Path C samples.
- **Option C** — Combination A+B: likely reaches 500+.

Gate C (no civic) samples look problematic: pure retail strips in NJ/LA with hc=0. Do not recommend.

Spatial coverage gap at P=0.20: 37 US states produce 0 T1 clusters. US T1 is concentrated in
CA (165), TX (64), IL (54), NY (42) — four states = 62% of all pre-IoU US qualifiers.

## 2. Chain count audit — notable anomalies

`work/chain-count-audit.txt` has full results. Key items:

| Chain | Expected | Actual | Ratio | Note |
|---|---|---|---|---|
| tesco-uk | 3,300 | 784 | 0.24× | SEVERE UNDER — ALPHA_HYPERMARKET chain |
| sainsburys-uk | 1,400 | 672 | 0.48× | UNDER — ALPHA_HYPERMARKET chain |
| tiendas-3b-mx | 1,700 | 151 | 0.09× | SEVERE UNDER — Food family |
| soriana-mx | 800 | 489 | 0.61× | UNDER — ALPHA_HYPERMARKET chain |
| walmart-mx | 250 | 462 | 1.85× | OVER (sub-formats inflating) |
| sams-club-mx | 170 | 254 | 1.49× | OVER |

**Tesco/Sainsbury's UNDER is the highest-priority fix.** Both are ALPHA_HYPERMARKET in GB.
Low GB T1 (currently 3) likely partly explained by thin Tesco/Sainsbury's OSM coverage.
Re-ingest or wikidata_id cross-check recommended next GB session.

## 3. Chain coverage audit — top gap candidates

`work/chain-coverage-audit.md` has full detail. Recommended additions:

| Chain | ISO | Stores | Recommended placement | QID |
|---|---|---|---|---|
| Whole Foods Market | US | ~530 | ALPHA_HYPERMARKET (premium grocery) | Q1758180 |
| ASDA | GB | ~600 | ALPHA_HYPERMARKET | Q297410 |
| Morrisons | GB | ~500 | ALPHA_HYPERMARKET | Q922344 |
| Chedraui | MX | ~280 | ALPHA_HYPERMARKET | Q2336803 |
| Real GmbH | DE | ~280 | ALPHA_HYPERMARKET | Q695602 |
| Casino Hypermarché | FR | ~200 | ALPHA_HYPERMARKET | Q1630621 |
| H-E-B | US | ~340 | ALPHA_HYPERMARKET (regional, TX+MX) | Q1665088 |

**IT, GR, AT, PT, NL are IKEA-only anchor regions** — structural gap. No ALPHA_HYPERMARKET
chain exists in config for these countries. Adding Conad-IT, Esselunga-IT, Spar-AT, Continente-PT,
Albert Heijn-NL (Ahold) would unblock T1 clusters in these markets.

## 4. OD data — UK/FR/DE all available

`work/od-data-research-uk-fr-de.md` has full detail. Summary:
- **UK ONS ODWP01EW**: Available at Nomis (nomisweb.co.uk), MSOA-level, ~77MB zip, OGL v3.
- **France INSEE FD_MOBPRO21**: Available, commune-level, Licence Ouverte 2.0. Navigate:
  insee.fr → Données → Mobilités professionnelles → RP2021 MOBPRO.
- **Germany BA Statistik Pendler**: Kreis-level OD available, XLSX, free with attribution.

All three are free and commercially usable. Ingest pipeline exists (uses H3 res-7 spatial join).
Manual download required for all three (no S3/direct URL).

## 5. Kontur Population — viable WorldPop alternative

`work/kontur-integration-plan.md` has full detail.
- HDX download: `data.humdata.org/dataset/kontur-population-dataset` (global H3 res-8, ~2.3GB)
- License: CC BY 4.0 (compatible with our pipeline)
- H3 res-8 → res-7 aggregation: straightforward (res-8 child cells → sum to res-7 parent)
- Estimated accuracy improvement vs WorldPop 100m: better for dense urban areas (uses building footprints)
- Sprint effort: 1 session (download + aggregate + replace WorldPop source in ingest-census.py)

## 6. MITMA — ES records missing from od-summary.jsonl

od-summary.jsonl has 10,213 entries (one per cluster) but ES records = 0.
`mitma-home-od-es.jsonl` (5.4M) and `mitma-work-od-es.jsonl` (4.6M) exist in service-mobility.
synthesize-od-study.py likely needs to be re-run after MITMA ingest, or MITMA needs
a different integration path. Needs investigation — not a blocker for map UI.

## 7. Storage — no action needed today

Root at 65% (28G free). Secondary at 79% (7.9G free).
Stale backup GeoJSON files (clusters.geojson.pre-sprint9, .new, .bak = ~35M) can be deleted
without approval. Large GeoJSON move to secondary not recommended (secondary already at 79%).

## 8. Overture Addresses — not tested (DuckDB check deferred)

S3 connectivity test was not run. `extract-overture-addresses.py` committed (21cf18df).
Operator should verify DuckDB+S3 is accessible before scheduling the run
(expected: ~50GB S3 read, H3-join, writes backfilled JSONL).

---
