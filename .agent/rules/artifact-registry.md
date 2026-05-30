---
schema: foundry-artifact-registry-v1
project: project-gis
last_updated: 2026-05-30
---

# project-editorial Artifact Registry

Persistent record of all editorial, design, data, and JOURNAL artifacts being built
for this project. Updated as artifacts are staged, dispatched, or completed.
Companion to `tasks.md` (sprint history) and `outstanding-todo.md` (backlog).

Routing:
- TOPIC / GUIDE / TEXT / PROSE → `project-editorial`
- DESIGN-RESEARCH / DESIGN-TOKEN / COMPONENT → `project-design`
- DATA artifacts remain in this archive
- JOURNAL → `drafts-outbound/` (staged for external journal submission)

---

## J — JOURNAL Artifacts (PhD Thesis Programme)

Academic papers under the `foundry-journal-v1` schema. Named natural-person
authors only. No internal Foundry vocabulary. Rules: `.agent/rules/journal-artifact-discipline.md`.

Status values: `stub` → `scaffolded` → `language-cleared` → `submission-ready` → `submitted` → `published`

| ID | File | Title (working) | Target Journal | Lead Author | Status |
|----|------|-----------------|----------------|-------------|--------|
| J1 | `JOURNAL-retail-colocation-v0.1.draft.md` | Retail Anchor Co-location Composition as a Spatial Leading Indicator of Commercial Activity | Economic Geography (Wiley, IF 7.2) | Jennifer M. Woodfine | v0.5 dispatched to project-editorial 2026-05-31 |
| J2 | `JOURNAL-trustworthy-systems-v0.1.draft.md` | Composing Trustworthy Systems from Verified Primitives | ASPLOS (ACM, 19.4% AR) | Mathew Woodfine | language-cleared |
| J3 | `JOURNAL-aec-data-layers-v0.1.draft.md` | Open-Source Building-Systems Data Layers for Urban-Scale Site Analysis | Automation in Construction (Elsevier, IF 12.0) | Jennifer M. Woodfine | language-cleared |
| J4 | `JOURNAL-private-network-v0.4.draft.md` | Customer-Rooted Mesh Architecture for Distributed Operational Systems: Zero-Trust Isolation Without Vendor Key Custody | IEEE TIFS (IEEE, IF 9.65) | Peter M. Woodfine | v0.4 synced to JOURNAL/; dispatched to project-editorial 2026-05-31 |
| J5 | `JOURNAL-totebox-orchestration-v0.1.stub.md` | Capability-Secured Session Orchestration | MLSys (ACM, 22% AR) | Mathew Woodfine | stub |
| J6 | `JOURNAL-desktop-environment-v0.1.stub.md` | Muscle-Memory-Preserving Desktop Environments for Professional AEC Software Migration | ACM TOCHI | Jennifer M. Woodfine | language-cleared (v0.3; history-language pass 2026-05-30; Phase labels stripped; §6 Results pending user study) |

### Pre-submission blockers by paper

**J1 — Retail Co-location:**
- ~~Language pass~~ — COMPLETE 2026-05-28 (`forbidden_terms_cleared: true`; body scanned clean)
- ~~F1–F5 figures~~ — READY at project-gis `work/figures/` (produced 2026-05-28)
- ~~Phase 22 CSV~~ — READY at project-gis `work/clusters-ols.csv` (6,493 rows, 2026-05-28)
- ~~§7.0 preliminary OLS~~ — COMPLETE 2026-05-28: Model A (T1 span β=+0.489, p<0.001) + Model B (R²=0.503); F6 partial produced
- ~~§5.1 table corrected~~ — DONE 2026-05-31 v0.5: NA/EU Phase 23+Change B actuals (NA T1=1,021/T2=1,831/T3=913; EU T1=725/T2=895/T3=1,108)
- ~~§5.4 Regional Market Discovery~~ — DONE 2026-05-31 v0.5: isolation-first scoring, Top 400 country distribution, H₄ hypothesis
- ~~Appendix B~~ — DONE 2026-05-31 v0.5: 18-country T1/T2/T3 table from Phase 23+Change B clusters-meta.json
- Language pass on §5.4 (new section; `forbidden_terms_cleared` reset to false) — AT project-editorial
- §7.2 primary spec (catchment_entropy ~ tier + log[pop_150km] + country FE) — pending Phase 24B (Kontur population join + O-D data)
- F6 update with §7.2 spec results — pending Phase 24B
- §5.3 LODES employment join — v0.6 item (executable once `build-geometric-ranking.py` run)
- Appendix C data-flow diagram — v0.6 item
- Permutation test (`sim-tier-permutation.py`) — to be written
- Word count trim (~800 words; from 9,300 to 8,500 target) — AT project-editorial
- CBRE/JLL leasing-data acquisition (Year 2 research)
- ORCID IDs for all three authors

**J2 — Trustworthy Systems:**
- ~~Language pass~~ — COMPLETE 2026-05-28 (`forbidden_terms_cleared: true`)
- Bench #9 quiet-VM re-run (22 outliers, ±11% CI — explicitly flagged)
- Promote all `[external: ...]` citation placeholders to `citations.yaml` stable IDs
- ASPLOS short version (~6,000 words, 2-column ACM format)
- ORCID IDs for all three authors

**J3 — AEC Data Layers:**
- ~~Full body writing pass~~ — COMPLETE 2026-05-28 (~7,800 words; §1–§5 + §7–§8 written; §6 Results structured TODO)
- ~~Language pass~~ — COMPLETE 2026-05-28 (`forbidden_terms_cleared: true`; body scanned clean)
- §6 Results — pending AEC nightly build coverage metrics from project-gis (H3 cells covered vs. total per country per layer; Nights 2–5)
- ORCID IDs for all three authors

**J4 — Private Network / CRMA:**
- ~~§1–§3 writing pass~~ — COMPLETE 2026-05-28 (~4,800 words; §1 Introduction, §2 Background, §3 Architecture written; §6 Discussion + §7 Conclusion written)
- ~~Language pass~~ — COMPLETE 2026-05-28 (`forbidden_terms_cleared: true`)
- ~~§4 Implementation + §5 Evaluation~~ — COMPLETE 2026-05-29 v0.3 (commit 149a8b39): empirical benchmarks on GCP e2-standard-8; B1 n=30 mean=44ms; B2 n=10 mean=59ms; B3 wg set=8ms; B4 bimodal {1s,11-16s}
- ~~[CITATION NEEDED] x2~~ — RESOLVED v0.4 (b3e8190a): Birge-Lee 2024 DOI:10.1007/978-3-031-85960-1_14 + Mackey 2020 DOI:10.1145/3374664.3379532
- ~~JOURNAL/ sync~~ — DONE 2026-05-31: v0.4 copied to JOURNAL/JOURNAL-private-network-v0.4.draft.md; stale v0.1 stub removed
- §4–§5 language pass — AT project-editorial (outbox 952b2b09; `forbidden_terms_cleared` conservatively false for new sections)
- ORCID IDs for all three authors (operator action required)

**J6 — Desktop Environment:**
- §1–§4 writing pass (Introduction, Background, Design Principles, Implementation) — in progress
- §5–§6 pending user study data
- ORCID IDs for all three authors

**J5:** HOLD until J2 submitted

**J7 — Urban Fringe pre-submission blockers:**
- Full chain ingestion (MRO, flooring, tool-rental, lumber YAMLs) — pending Overpass ingest
- Full literature review (§2) — stub only
- Validation section (§4.4) — stub only
- OLS regression (§5.3 / §7.1) — executable once full dataset available
- ORCID IDs for all three authors

**J8 — Commuter pre-submission blockers:**
- Literature review (§2) — stub only
- Validation (§4.4) — stub only
- Integration rate regression (§5.3 / §7.2) — requires rail-frequency external data
- ORCID IDs for all three authors

---

## S — Practitioner Summaries (CBRE/JLL white paper format)

Self-contained market briefs. Audience: commercial real estate investors, developers, planners. No academic citations. Published to `gis.woodfinegroup.com/research/` alongside companion JOURNAL papers.

| ID | File | Title | Companion | Status |
|---|---|---|---|---|
| S1 | `SUMMARY/SUMMARY-urban-fringe.draft.md` | Urban Fringe: The Industrial Co-location Layer in the Metropolitan Ring | J7 | stub — 2026-06-01 |
| S2 | `SUMMARY/SUMMARY-commuter.draft.md` | Commuter: Transit-Adjacent Commercial Co-location at Regional Rail Stations and Airports | J8 | stub — 2026-06-01 |

**S1 pre-completion requirements:** Full chain ingestion → updated §2 country table; §5 integration analysis; §6 investment thesis
**S2 pre-completion requirements:** §5 integration rate analysis written; §6 investment thesis; map figures from work/archetype-pks.geojson

---

## A — Active / In-Progress

### A18 — BRIEF: Location Intelligence Archetypes (PRO / VWH / PKS)
- **File:** `.agent/briefs/BRIEF-location-intelligence-archetypes-2026-06-01.md`
- **Status:** ACTIVE — updated 2026-06-01 with §8 full data collection inventory and §9 sequencing
- **Destination:** internal (BRIEF stays in archive); editorial drafts dispatched separately
- **Content:** Three-archetype Location Intelligence system (PRO/VWH/PKS). Definitions, co-location
  signals, test results (360 VWH / 6,640 PKS / 1,803 integrated), full chain taxonomy with Wikidata
  IDs, data collection sequencing, airport/rail classification findings, service-parking architecture.

### A19 — TOPIC: Vertical Warehouse (VWH)
- **File:** `.agent/drafts-outbound/TOPIC-vertical-warehouse.draft.md`
- **Status:** STAGED — draft complete 2026-06-01; ready for project-editorial
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** VWH archetype definition (3-6 story urban logistics/light-mfg), spatial signature,
  site selection signals, 360 test candidates, full Tier A/B chain taxonomy (Würth, Floor & Decor,
  United Rentals, Fastenal, etc.), data collection plan with Wikidata IDs.

### A20 — TOPIC: Parking Structures (PKS)
- **File:** `.agent/drafts-outbound/TOPIC-parking-structures.draft.md`
- **Status:** STAGED — draft complete 2026-06-01; ready for project-editorial
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** PKS archetype definition (3-9 story transit car parks), regional-to-metro relationship,
  airport and rail station classification, 6,640 test candidates (1,803 integrated), commercial
  co-location pattern from Overpass spot checks, car rental chains, parking operator directory.

### A21 — GUIDE: Location Intelligence Data Collection
- **File:** `.agent/drafts-outbound/GUIDE-location-intelligence-data-collection.draft.md`
- **Status:** STAGED — draft complete 2026-06-01; ready for project-editorial
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** 7-step operational runbook: run existing YAML ingests, add Würth + new taxonomy
  categories, add Tier A VWH chains, write ingest-osm-airports.py, write ingest-osm-railway.py,
  add PKS car rental chains, re-run test-cluster-archetypes.py and deploy updated GeoJSON.

### A7 — BRIEF: Regional Markets System
- **File:** `BRIEF-regional-markets-system.draft.md`
- **Status:** REVISED + STAGED — dispatched to project-editorial 2026-05-30 (msg-id: project-gis-20260530-rm-corrected-dispatch)
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** System BRIEF updated v2: executive summary adds competitive positioning (Oxford Economics/CBRE/Colliers cover metro cores; this dataset addresses the suburban ring gap). §3 Regional Markets adds three-type table (metro-core/suburban-regional/standalone-secondary) and coherence constraint. §4 Top 400 formula corrected (metro_distance_multiplier removed; rationale explains why). Current top results cited. Phase 23+Change B dataset (6,493 clusters; T1=1,746/T2=2,726/T3=2,021).

### A8 — TOPIC: Top 400 Regional Markets — North America
- **File:** `TOPIC-top-400-regional-markets-na.draft.md`
- **Status:** REVISED + STAGED — dispatched to project-editorial 2026-05-30 (msg-id: project-gis-20260530-rm-corrected-dispatch)
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** REVISED v2 — corrected methodology: suburban-regional definition (15–80 km from major metro), no province aggregations, no metro_multiplier. US 369 / CA 23 / MX 8. Rank 1 = Plano TX (suburb of Dallas, score 25.5). Score range 25.5–4.0.

### A9 — TOPIC: Top 400 Regional Markets — Europe
- **File:** `TOPIC-top-400-regional-markets-eu.draft.md`
- **Status:** REVISED + STAGED — dispatched to project-editorial 2026-05-30 (msg-id: project-gis-20260530-rm-corrected-dispatch)
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** REVISED v2 — corrected methodology: suburban-regional definition (15–80 km), no metro_multiplier. DE 124 / FR 102 / GB 81 / ES 23 / IT 21 / PL 16 / NL 14 / AT 7 / DK 5 / SE 4 / PT 3 = 11 countries. Rank 1 = Chemnitz (suburb of Dresden, score 18.0). Score range 18.0–3.0. Suburb_of column in top-25 table.

### A10 — TOPIC: Wichita, Kansas Regional Market
- **File:** `TOPIC-rm-wichita-ks.draft.md`
- **Status:** SUPERSEDED — hold pending revision (msg-id: project-gis-20260530-rm-methodology-correction). Wichita is itself a metro reference; its suburbs (Derby, Andover, Maize) are regional markets, not Wichita proper.

### A11 — TOPIC: Colorado Springs, Colorado Regional Market
- **File:** `TOPIC-rm-colorado-springs-co.draft.md`
- **Status:** SUPERSEDED — hold pending revision. Colorado Springs is ~90 km from Denver = standalone-secondary (>80 km threshold). Excluded from Top 400 under corrected methodology.

### A12 — TOPIC: Nürnberg, Germany Regional Market
- **File:** `TOPIC-rm-nurnberg-de.draft.md`
- **Status:** SUPERSEDED — hold pending revision. Nürnberg is a metro reference (added to EU_METROS list). Classified as metro-core (dist=1.6 km from itself). Excluded from Top 400 under corrected methodology.

### A15 — TOPIC: Plano, Texas Regional Market
- **File:** `TOPIC-rm-plano-tx.draft.md`
- **Status:** STAGED in drafts-outbound/ — dispatched to project-editorial 2026-05-30 (msg-id: project-gis-20260530-rm-corrected-dispatch)
- **Destination:** project-editorial → media-knowledge-documentation (projects.woodfinegroup.com)
- **Content:** Rank 1 NA. Suburb of Dallas (28.1 km). 3×T1 + 2×T2 + 1×T3, civic=true. Score=25.5. Civic anchors: Texas Health Presbyterian, Baylor Scott & White, Medical City Plano, UT Dallas, Texas A&M AgriLife. ASHRAE=3A, Köppen=Cfa. ~1,050 words body.

### A16 — TOPIC: Mississauga, Ontario Regional Market
- **File:** `TOPIC-rm-mississauga-on.draft.md`
- **Status:** STAGED in drafts-outbound/ — dispatched to project-editorial 2026-05-30 (msg-id: project-gis-20260530-rm-corrected-dispatch)
- **Destination:** project-editorial → media-knowledge-documentation (projects.woodfinegroup.com)
- **Content:** Rank 4 NA. Suburb of Toronto (24.9 km). 2×T1 + 1×T2 + 2×T3, civic=true. Score=18.0. Civic anchors: Mississauga Hospital, Credit Valley Hospital (Trillium Health Partners), University of Toronto Mississauga. Köppen=Dfa. ~900 words body.

### A17 — TOPIC: Krefeld, Germany Regional Market
- **File:** `TOPIC-rm-krefeld-de.draft.md`
- **Status:** STAGED in drafts-outbound/ — dispatched to project-editorial 2026-05-30 (msg-id: project-gis-20260530-rm-corrected-dispatch)
- **Destination:** project-editorial → media-knowledge-documentation (projects.woodfinegroup.com)
- **Content:** Rank 5 EU. Suburb of Düsseldorf (19.4 km). 2×T1, civic=true. Score=12.0. Civic anchors: Helios Klinikum Krefeld, Hochschule Niederrhein. EU Climate=II (Atlantic), Köppen=Cfb. ~950 words body.

### A13 — DESIGN-RESEARCH: Regional Market TOPIC Template
- **File:** `DESIGN-regional-market-topic-template.draft.md`
- **Status:** STAGED in drafts-outbound/ — dispatched to project-design 2026-05-30 (msg-id: project-gis-20260530-regional-markets-dispatch)
- **Destination:** project-design → pointsav-design-system
- **Content:** Visual layout spec for Regional Market TOPIC wiki article type. Two-column layout, infobox card, co-location table with tier colour-coding, AEC data grid, CSS score breakdown bar, Wikipedia attribution footer. HTML skeleton + named CSS classes. 5 open questions for project-design.

### A14 — GUIDE: Regional Market TOPIC Production
- **File:** `GUIDE-regional-market-topic-production.draft.md`
- **Status:** STAGED in drafts-outbound/ — dispatched to project-editorial 2026-05-30 (msg-id: project-gis-20260530-regional-markets-dispatch)
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** 11-section operational guide covering: prerequisites, running score-regional-markets.py, running export-aec-coverage.py, market selection criteria, Wikipedia API lookup procedure, TOPIC frontmatter checklist, AEC data join, TOPIC body structure, review/dispatch, artifact registry, batch production workflow.

### A6 — PROSE-RESEARCH: Geometric Site Selection (JoEG preparation)
- **File:** `PROSE-RESEARCH-geometric-site-selection.draft.md`
- **Status:** DISPATCHED — v0.4 (2026-05-27) — at project-gis drafts-outbound, dispatched to project-editorial
- **Destination:** project-editorial → content-wiki-documentation/research/
- **Content:** Continental-scale cluster analysis paper; geometric co-location as spatial leading indicator.
  v0.4: 8 sections, §3.7 mobility catchments, §7.2 OLS regression, Bloomberg register, banned vocabulary clean.
- **Editorial gates (project-editorial to resolve before publication):**
  - Appendix B country-by-country T1 table (Phase 22 data available — run taxonomy.py export)
  - §5.3 LODES employment medians (placeholder or "v0.5" note)
  - Appendix C data-flow diagram (placeholder or defer to v0.5)
  - BCSC disclosure pass (bcsc_class: public-disclosure-safe in frontmatter; verify no active Foundation language)
  - Paper NOT submitted to any journal — draft notice must read "in preparation for intended submission to JEG (OUP)"
  - Bilingual ES sibling required before journal submission
- **Research tasks pending:** CBRE/JLL acquisition (Year 2); permutation test implementation

### A1 — TOPIC: O-D Catchment Methodology
- **File:** `topic-od-catchment-methodology.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial
- **Destination:** project-editorial
- **Content:** Crow-flies O-D model; 35/150km ring rationale; H3 res-7; provisional language; HOME vs AWAY distinction

### A2 — TOPIC: Trade Area Data Sources
- **File:** `topic-trade-area-data-sources.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial
- **Destination:** project-editorial
- **Content:** WorldPop 2026 100m raster → H3 res-7 aggregation; 13 countries; per-capita spend multipliers; data vintage; BLS/StatCan/Eurostat proxies

### A3 — TOPIC: Catchment Ranking Methodology
- **File:** `topic-catchment-ranking-methodology.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial
- **Destination:** project-editorial
- **Content:** Combined primary+secondary rank dimensions; no-weights rationale; future weighting roadmap

### A4 — TEXT: Data Methodology Dialog
- **File:** `text-gis-data-methodology-dialog.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial
- **Content:** Copy for the "Data" button modal on gis.woodfinegroup.com; all data source attributions; methodology notes; link to DATA-MANIFEST.md

### A5 — GUIDE: Pipeline Rebuild (Phase 1/2 appended)
- **File:** `guide-gis-pipeline-rebuild.draft.md`
- **Status:** DISPATCHED fe5148fd (2026-05-16) — at project-editorial; Phase 1/2 sections appended
- **Destination:** project-editorial
- **Content:** Full rebuild procedure including Phase 1 taxonomy rebuild steps and Phase 2 build-geometric-ranking.py future pipeline

---

## B — Backlog (queued for future sessions)

### B1 — TOPIC: Co-location Ranking System (full update)
- **Status:** BACKLOG — existing draft dispatched; needs update for catchment rank fields
- **Destination:** project-editorial

### B2 — TOPIC: POI Data Schema
- **Status:** DISPATCHED sprint 13 (ba5fe38) — at project-editorial

### B3 — GUIDE: Adding a Chain
- **Status:** DISPATCHED sprint 11/13 — at project-editorial; appendix added

### B4 — GUIDE: Adding a Country
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B5 — TEXT: Canada/Walmart Supercentre + Hospital Coverage
- **Status:** STAGED in drafts-outbound/ (text-gis-canada-walmart-hospital-coverage.draft.md)

### B6 — DESIGN-RESEARCH: Bento Merged Zones Disclosure
- **Status:** IMPLEMENTED 21cf18df (2026-05-17) — merged-ring UX shipped in index.html (Union-Find groupOverlappingClusters, showMergedGroupPanel). Editorial draft still at project-design for write-up.

### B7 — DESIGN-RESEARCH: Location Intelligence UX
- **Status:** STAGED in drafts-outbound/ — at project-design

### B8 — DESIGN-RESEARCH: Ring Retailer Click UX
- **Status:** STAGED in drafts-outbound/ — at project-design

### B9 — DESIGN-RESEARCH: Tier Naming Accessibility
- **Status:** STAGED in drafts-outbound/ — at project-design

### B10 — DESIGN-RESEARCH: Zoom Prefetch Pattern
- **Status:** STAGED in drafts-outbound/ — at project-design

### B11 — TEXT: Nordic/UK Coverage Release
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B12 — TEXT: UK/EU Coverage Release
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B13 — TOPIC: Regional Name Resolution Architecture
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B14 — TOPIC: Co-location Tier Nomenclature
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B15 — TOPIC: GIS as BIM Substrate
- **Status:** STAGED in drafts-outbound/ — at project-editorial

### B16 — TOPIC: UK/EU Food Retail Coverage
- **Status:** STAGED in drafts-outbound/ — at project-editorial

---

## C — Data Artifacts (pipeline outputs, not editorial)

| Artifact | File | Status |
|---|---|---|
| O-D Summary (B3) | `service-fs/service-mobility/od-summary.jsonl` | DONE (2026-05-15) |
| Catchment Data | `work/catchment-data.json` | DONE (2026-05-15) |
| Catchment Polygons | `work/catchment-polygons.geojson` → `layer3-catchment.pmtiles` (87 MB) | DONE (2026-05-15) |
| Census Catchment | `work/census-catchment.geojson` → `layer4-census.pmtiles` (373 MB) | DONE (2026-05-15) |
| Spend Catchment | `work/spend-catchment.geojson` → `layer5-spend.pmtiles` (635 MB) | DONE (2026-05-15) |
| DATA-MANIFEST.md | project root | DONE (2026-05-12) |
| clusters-meta.json | gateway www/data/ | DONE (2026-05-22; 5,702 clusters; T1=1,157/T2=4,283/T3=262; Phase 18; 570bda53) |
| regional-markets.json | gateway www/data/ | DONE (2026-05-15; 2,986 Regional Markets, 2,942 high-conf) |
| us_places.geojson | deployments/boundaries/ | DONE (2026-05-15; TIGER 2023, 32K US places) |
| eu_municipalities.geojson | deployments/boundaries/ | DONE (2026-05-15; GISCO LAU 2021 + GADM GBR, 98.6K entries) |
| ca_places_nominatim.json | deployments/boundaries/ | DONE (2026-05-15; 12 county-CSD overrides, e.g. Sherwood Park) |

---

### Phase 13 Re-Ingests + Mobility Update (2026-05-17)

| Artifact | File | Status |
|---|---|---|
| tesco-uk re-ingest | `service-business/tesco-uk.jsonl` | DONE (2026-05-17; 784→3,872 records; name_query partial; Phase 13) |
| sainsburys-uk re-ingest | `service-business/sainsburys-uk.jsonl` | DONE (2026-05-17; 672→1,903 records; name_query partial; Phase 13) |
| tiendas-3b-mx re-ingest | `service-business/tiendas-3b-mx.jsonl` | DONE (2026-05-17; 151→247 records; name_query Tiendas 3B; Phase 13) |
| MITMA ES mobility_source | clusters-meta.json (58 ES clusters) | DONE (2026-05-17; build-mobility-tiles.py; Phase 13) |

---

### Phase 15 Chain Ingests (2026-05-18)

| Artifact | File | Status |
|---|---|---|
| wegmans-us JSONL | `service-business/wegmans-us.jsonl` | DONE (2026-05-18; 114 records; Q1182328; name_query; 4952dfaf) |
| winco-foods-us JSONL | `service-business/winco-foods-us.jsonl` | DONE (2026-05-18; 145 records; Q2584339; name_query; 4952dfaf) |
| sprouts-us JSONL | `service-business/sprouts-us.jsonl` | DONE (2026-05-18; 450 records; Q7580917; name_query; 4952dfaf) |
| build-tiles CHAIN_FAMILY fix | `build-tiles.py` | DONE (2026-05-19; wegmans/winco/sprouts/whole-foods/chedraui/asda/morrisons/heb brand_family fixed) |
| layer2-clusters.pmtiles rebuild | gateway tiles/ | DONE (2026-05-19; 13,657 clusters; 76.7 MB) |

---

### Phase 12 Chain Ingests (2026-05-17)

| Artifact | File | Status |
|---|---|---|
| ASDA-UK JSONL | `service-business/asda-uk.jsonl` | DONE (2026-05-17; 1,051 records; Q297410; 3b367a9f) |
| Morrisons-UK JSONL | `service-business/morrisons-uk.jsonl` | DONE (2026-05-17; 620 records; Q922344; 3b367a9f) |
| H-E-B JSONL | `service-business/heb-us.jsonl` | DONE (2026-05-17; 301 records; Q1665088; name_query fallback; 3b367a9f) |
| whole-foods-us ALPHA promo | config.py | DONE (2026-05-17; promoted from GENERIC_FOOD; 528 records pre-existing; 3b367a9f) |
| chedraui-mx ALPHA promo | config.py | DONE (2026-05-17; promoted from generic; 249 records pre-existing; 3b367a9f) |

---

### Phase 16 Chain Ingests + Infrastructure (2026-05-19)

| Artifact | File | Status |
|---|---|---|
| layer3-catchment.pmtiles rebuild | gateway tiles/ | DONE (2026-05-19; 30MB vs 1.7GB bad build; max-zoom 8, drop-densest, simplification 8) |
| Kontur Population downloads | deployments/cluster-totebox-personnel-1/service-fs/service-census/kontur-raw/ | DONE (2026-05-19; 13 countries; 523MB; CC BY 4.0) |
| esselunga-it JSONL | `service-business/esselunga-it.jsonl` | DONE (2026-05-19; 259 records; name_query fallback; Q1377048) |
| sklavenitis-gr JSONL | `service-business/sklavenitis-gr.jsonl` | DONE (2026-05-19; 406 records; Greek name_query Σκλαβενίτης; Q7536996) |
| billa-plus-at JSONL | `service-business/billa-plus-at.jsonl` | DONE (2026-05-19; 139 records; name_query fallback; Q806085) |
| continente-pt JSONL | `service-business/continente-pt.jsonl` | DONE (2026-05-19; 57 records; name_query fallback; Q5164541) |
| albert-heijn-xl-nl JSONL | `service-business/albert-heijn-xl-nl.jsonl` | DONE (2026-05-19; 43 records; name_query "Albert Heijn XL"; no wikidata) |
| config.py Phase 16 update | `config.py` | DONE (2026-05-19; 5 chains → ALPHA_HYPERMARKET EU + REGION_CONFIG anchors + ANCHOR_DISPLAY_NAMES) |
| US LODES full ingest | `service-fs/service-mobility/lodes-work-od-us.jsonl` | DONE (2026-05-20; 50 states/AK skipped; 684,334 H3 cells; 7,577 US clusters; 5.3GB) |
| layer6-mobility-work.pmtiles rebuild | gateway tiles/ | DONE (2026-05-20; 164MB; full US LODES + MITMA ES; 49af6829) |
| ingest-kontur.py | `app-orchestration-gis/ingest-kontur.py` | DONE (2026-05-20; committed 49af6829; sqlite3 stdlib, no fiona; 13 countries) |

---

### Phase 17 EU Taxonomy Audit + Rebuild (2026-05-22)

| Artifact | File | Status |
|---|---|---|
| taxonomy.py Phase 17 | `app-orchestration-gis/taxonomy.py` | DONE (2026-05-22; af434817; 12 zero-cost EU hypermarket chains activated) |
| config.py Phase 17 | `app-orchestration-gis/config.py` | DONE (2026-05-22; af434817; ALPHA_HYPERMARKET EU + REGION_CONFIG anchors sync'd) |
| generate-rm-topics.py | `app-orchestration-gis/generate-rm-topics.py` | DONE (2026-05-22; af434817; 225 lines; generates TOPIC drafts per Regional Market) |
| layer2-clusters.pmtiles Phase 17 | gateway tiles/ | DONE (2026-05-22; 37.7 MB; 5,273 clusters; two-pass DBSCAN §2 schema) |
| clusters-meta.json Phase 17 | gateway www/data/ | DONE (2026-05-22; 11 MB; T1=1,136/T2=3,865/T3=272; PL 0→17 T1; IT 3→8; ES 25→29; SE 8→4) |
| ikea-se.yaml / ikea-dk.yaml / ikea-no.yaml / ikea-fi.yaml | deployments/service-business/ | DONE (2026-05-22; per-country YAMLs replacing ikea-nordics multi_country; proper bbox+polygon filter) |
| ikea-se.jsonl | deployments/service-business/ | DONE (2026-05-22; 16 records; clean SE only; Lithuanian contamination removed) |
| ikea-dk.jsonl | deployments/service-business/ | DONE (2026-05-22; 5 records; clean DK only; 7 UK/SE ghost stores removed) |
| ikea-no.jsonl | deployments/service-business/ | DONE (2026-05-22; 5 records; Oslo/Trondheim/Slependen/Sørlandet/Åsane) |
| ikea-fi.jsonl | deployments/service-business/ | DONE (2026-05-22; 4 records; Espoo/Vantaa/Tampere/Kuopio) |
| layer2-clusters.pmtiles Nordic IKEA fix | gateway tiles/ | DONE (2026-05-22; 37.7 MB; 5,274 clusters; DK+NO+FI each gain first T1) |
| clusters-meta.json Nordic IKEA fix | gateway www/data/ | DONE (2026-05-22; 11 MB; T1=1,136/T2=3,866/T3=272; Odense DK T1; Oslo NO T1; Tampere FI T1) |

---

### Phase 18 Chain Ingests (2026-05-22)

| Artifact | File | Status |
|---|---|---|
| kaufland-pl JSONL | `service-business/kaufland-pl.jsonl` | DONE (2026-05-22; 253 records; Q685967; PL T1=17 unchanged — joins existing clusters) |
| foetex-dk JSONL | `service-business/foetex-dk.jsonl` | DONE (2026-05-22; 103 records; Q3093871 Salling Group) |
| wickes-uk JSONL | `service-business/wickes-uk.jsonl` | DONE (2026-05-22; 236 records; Q7998350 Travis Perkins hardware) |
| bauhaus-dk JSONL | `service-business/bauhaus-dk.jsonl` | DONE (2026-05-22; 20 records; Q532716) |
| bauhaus-no JSONL | `service-business/bauhaus-no.jsonl` | DONE (2026-05-22; 2 records; Q532716; OSM sparse in NO) |
| interspar-at JSONL | `service-business/interspar-at.jsonl` | DONE (2026-05-22; 85 records; Q1364056 SPAR Austria; Q1473279 rejected = Turmöl fuel) |
| jumbo-nl JSONL | `service-business/jumbo-nl.jsonl` | DONE (2026-05-22; 8 records; Q14716185 Jumbo Foodmarkt large-format) |
| leclerc-pl JSONL | `service-business/leclerc-pl.jsonl` | DONE (2026-05-22; 36 records; Q1273376) |
| bricomarch-fr JSONL | `service-business/bricomarch-fr.jsonl` | DONE (2026-05-22; 497 records; Q2896882 Les Mousquetaires hardware) |
| brico-depot-fr JSONL | `service-business/brico-depot-fr.jsonl` | DONE (2026-05-22; 137 records; Q3007003 Kingfisher hardware) |
| bauhaus-fi JSONL | `service-business/bauhaus-fi.jsonl` | DONE (2026-05-22; 6 records; Q532716) |
| globus-de JSONL | `service-business/globus-de.jsonl` | DONE (2026-05-22; 125 records; Q528681 Globus Holding) |
| geant-casino-fr JSONL | `service-business/geant-casino-fr.jsonl` | DONE (2026-05-22; 10 records; Q2901839 Casino Group) |
| intermarche-hyper-fr JSONL | `service-business/intermarche-hyper-fr.jsonl` | DONE (2026-05-22; 56 records; Q2029154 Les Mousquetaires) |
| taxonomy.py Phase 18 | `app-orchestration-gis/taxonomy.py` | DONE (2026-05-22; 570bda53; 14 chains added across PL/DK/GB/AT/NL/FR/DE/FI) |
| layer2-clusters.pmtiles Phase 18 | gateway tiles/ | DONE (2026-05-22; 40.8 MB; 5,702 clusters; T1=1,157/T2=4,283/T3=262) |
| clusters-meta.json Phase 18 | gateway www/data/ | DONE (2026-05-22; 570bda53; +17 T2 from FR hardware bricomarch+brico-depot; +4 T1/+19 T2 from globus-de) |

---

### Phase 11 Analysis Artifacts (2026-05-17)

| Artifact | File | Status |
|---|---|---|
| T1 threshold sweep (pre-IoU) | `work/sim-1a-results.txt` | DONE (2026-05-17; 5 thresholds; NA@P=0.20: 476 pre-IoU, 245 post-IoU) |
| T1 civic/composition sensitivity | `work/sim-1b-results.txt` | DONE (2026-05-17; civic B: NA=278@P=0.20; Path C adds 302 clusters) |
| T1 IoU + spatial coverage | `work/sim-1c-results.txt` | DONE (2026-05-17; IoU=0.10@P=0.20: NA=226 EU=57; 37 US states at 0) |
| Chain count audit | `work/chain-count-audit.txt` | DONE (2026-05-17; 91 OK, 35 OVER, 14 UNDER, 1 EMPTY) |
| Chain coverage audit | `work/chain-coverage-audit.md` | DONE (2026-05-17; gap candidates per country; see Section 2) |
| OD data research (UK/FR/DE) | `work/od-data-research-uk-fr-de.md` | DONE (2026-05-17; ONS ODWP01EW + INSEE FD_MOBPRO + BA Pendler all viable) |
| Kontur integration plan | `work/kontur-integration-plan.md` | DONE (2026-05-17; H3 res-8 available; CC BY 4.0; HDX download) |
| Storage report | `work/storage-report.md` | DONE (2026-05-17; root 65%; stale backups 35M removable) |
| SafeGraph export | `export-safegraph.py` | DONE adbb5d42 (2026-05-17; --sample 100 verified) |

---

## Maintenance

- When an artifact is dispatched to project-editorial/project-design, update status to DISPATCHED + commit hash
- When an artifact is returned (approved/rejected), update status
- Add new artifacts here at the time they are planned — do not wait until staging
