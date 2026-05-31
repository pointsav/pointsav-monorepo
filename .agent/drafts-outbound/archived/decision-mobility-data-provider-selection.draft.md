---
foundry-draft-v1: true
type: DECISION-RECORD
language_protocol: PROSE-EN
destination: project-editorial
created: 2026-05-15
author: project-gis Totebox Session
research_trail:
  query: "Advan Research Dewey Data Patterns Plus; Replica mobility platform; StreetLight Data Jacobs InSight; Spectus Cuebiq mobility data; Huq Industries footfall EU coverage"
  sources_consulted: "Dewey Data docs, Advan Research product pages, Replica documentation, StreetLight / Jacobs marketing, Spectus PEM whitepaper, Huq Industries Datarade listing, alternative data provider reviews"
  gaps: "Replica Canada coverage unconfirmed; Huq suburban/exurban density outside major conurbations unconfirmed; Spectus NL/BE/AT/CH/Nordics commercial coverage unconfirmed; all commercial pricing enterprise-only"
  bcsc_posture: factual
  forward_looking: none
---

# Mobility Data Provider Comparison — Retail Catchment O-D

**Decision record** — for gis.woodfinegroup.com co-location cluster catchment rebuild
**Prepared:** 2026-05-15 — supersedes WorldPop+radius-only methodology for North American clusters
**Scope:** 6,815 clusters across CA, US, UK/IE, DE/FR/NL/BE/AT/CH, ES/IT/PT, Nordics

---

## 1. Executive Summary

There is no single provider that covers all six priority regions with comparable fidelity for observed retail-visit origins. The pragmatic build is a three-vendor mosaic: **Advan Research (via Dewey Data)** for US and Canada at CBG / Canadian Dissemination Area resolution, **Huq Industries** for UK/IE and Western European conurbations using first-party SDK data, and **Spectus (Cuebiq Group)** as a research-grade clean-room option where panel depth in IT/ES/FR/DE is needed. **Replica** is North-America-only and synthetic — a strong Phase 2 candidate to validate Advan's observed flows against a calibrated activity-based model, but not the primary observed-data source. **StreetLight (Jacobs)** is excellent for traffic-corridor analysis but is positioned for transportation engineering, not retail-visitor origin queries, and remains US/CA-only. The WorldPop gravity model is retained as the floor for all uncovered regions and as the smoothing prior where panel counts are thin.

---

## 2. Use-Case Requirements Matrix

| Requirement | Hard / Soft | Rationale |
|---|---|---|
| Observed device-derived visits (not synthetic-only) for primary product | Hard | Replaces radius model; needed for disclosure credibility |
| Origin geography filterable to ≤5 km² (CBG / DA / LAU / postal-sector) | Hard | Required to bucket into H3 res-7 (~5 km²) |
| Per-POI or per-polygon visitor home query | Hard | We query against 6,815 cluster polygons, not zone-zone OD |
| Bulk delivery (Parquet/CSV/GeoJSON) suitable for nightly pipeline | Hard | PMTiles pipeline is offline-batch, not real-time |
| Monthly or finer cadence | Soft | Quarterly acceptable for v1 |
| Aggregate publishing rights | Hard | Output is public on gis.woodfinegroup.com — needs explicit license |
| Work-origin layer | Soft (Phase 2) | Wanted for daytime-population overlays |
| Public/predictable pricing | Soft | Helps with BCSC-style disclosure of data-cost commitments |

---

## 3. Provider Profiles

### 3.1 Advan Research (Dewey Data marketplace) — `Patterns Plus`

**A. Sourcing:** GPS panel aggregated from multiple SDK partners. Trade-area calculations (`visitor_home_cbgs`, `visitor_daytime_cbgs`) draw from a permissioned panel distinct from the visitation panel — better for cross-sectional catchment than YoY deltas. Panel size [unconfirmed]; Advan does not publish a hard device count.

**B. Coverage:** US + Canada confirmed. No EU/UK product. Canadian output uses Statistics Canada Dissemination Areas (DA).

**C. POI-visit query:** Yes — `Patterns Plus` (replacing `Monthly Patterns`, retired 2026-03-01) is built around POI-level visit records with origin distributions attached.

**D. Home/work labeling:** Yes. `visitor_home_cbgs` = home; `visitor_daytime_cbgs` = inferred work/daytime origin. Both filterable per POI. Cells with <4 visitors suppressed.

**E. Spatial output:** US CBG (~600–3,000 people); Canada DA (~400–700 people). Both smaller than H3 res-7 — clean upward aggregation.

**F. Delivery:** Dewey Data marketplace — Parquet/CSV bulk download or AWS S3 sync; no per-query API required.

**G. Cadence:** Weekly + monthly rollups; ~2–4 week vintage lag.

**H. Pricing:** Dewey subscription tiers (commercial vs academic). Academic licenses prohibit commercial/derivative use and public republication — commercial-tier required for gis.woodfinegroup.com. No public commercial price; quote-driven.

**I. Disclosure posture:** Aggregation thresholds (4-visitor floor) align with privacy-safe republication. Commercial terms govern publication rights — confirm aggregate-publish clause before going live.

---

### 3.2 Replica (replicahq.com) — Seasonal Trip Table / Places

**A. Sourcing:** Activity-based synthetic population simulation calibrated against LBS + mobile-network + ground-truth count data. Not a raw GPS panel — a model output. Synthetic agents carry home, work, income, mode, and trip purpose.

**B. Coverage:** United States, all 48 contiguous + DC, packaged in megaregions. Canada [unconfirmed — no Canadian megaregions documented]. No EU/UK coverage found.

**C. POI-visit query:** Yes — Places product surfaces per-POI visitor trip summaries with disaggregate breakdowns by day/time.

**D. Home/work labeling:** Yes — every synthetic agent has explicit home and work locations; both queryable.

**E. Spatial output:** Block-group / parcel resolution; aggregates cleanly to H3 res-7.

**F. Delivery:** Enterprise SaaS platform; data exports available. Not a self-serve marketplace.

**G. Cadence:** Seasonal (four releases per year); ~1 season vintage lag.

**H. Pricing:** Enterprise-only; no public pricing.

**I. Disclosure posture:** Synthetic-data output is inherently aggregate; republication terms governed by enterprise contract. Synthetic provenance must be disclosed clearly — cannot be presented as observed device data.

---

### 3.3 StreetLight Data (a Jacobs company) — InSight platform

**A. Sourcing:** Multi-source — GPS LBS, connected-vehicle, navigation-GPS, IoT. Calibrated against thousands of permanent counters.

**B. Coverage:** US (lower 48) + all Canadian provinces confirmed, including 2,500+ specific Canadian study zones. No UK/EU coverage found.

**C. POI-visit query:** Possible but secondary — the product is built around transportation OD between zones. Retail site-selection is marketed, but "who visited POI X and where do they live" is not the primary query shape.

**D. Home/work labeling:** Home and work zone tags are derivable, but the platform vocabulary is transportation-engineering, not retail-visitor.

**E. Spatial output:** Custom polygons, block group, ZIP. Compatible with H3 via post-processing.

**F. Delivery:** Web platform (InSight) with API and project exports. Not bulk-data-feed-shaped.

**G. Cadence:** Rolling; supports historical pulls back several years.

**H. Pricing:** Enterprise project-based; no public list price.

**I. Disclosure posture:** Publication via Esri partner integrations exists, so aggregate republication is supported under contract.

---

### 3.4 Spectus (Cuebiq Group)

**A. Sourcing:** First-party SDK GPS data from opted-in mobile-app users; privacy-enhanced ("PEM") with differential-privacy primitives.

**B. Coverage:** US, UK, Italy, Spain, France, Germany confirmed via documentation. Netherlands, Belgium, Austria, Switzerland, Portugal, Nordics — [unconfirmed] for commercial coverage at retail-relevant panel density. Italian panel is documented as North-skewed.

**C. POI-visit query:** Yes via clean-room SQL/Python over device trajectories and detected stops; not a turnkey POI dashboard — engineering effort required.

**D. Home/work labeling:** Yes — `Device Recurring Areas` product labels home, work, and other recurring stop clusters per device.

**E. Spatial output:** Geohash-3 / Geohash-5 in aggregated products; arbitrary polygon support in the clean-room. Geohash-5 (~4.9 km × 4.9 km) is near 1:1 with H3 res-7.

**F. Delivery:** Spectus Data Clean Room (AWS Marketplace); SQL/Python access plus CSV export.

**G. Cadence:** Continuous; monthly product updates.

**H. Pricing:** Enterprise; clean-room subscription. No public commercial price. Academic/Data-for-Good tier is gratis but non-commercial — would not cover public republication.

**I. Disclosure posture:** GDPR-compliant collection; PEM differential-privacy framework strong for EU publication. Per-engagement terms govern aggregate republication.

---

### 3.5 Huq Industries

**A. Sourcing:** First-party SDK collected directly from end-user apps (claims status as the only 1st-party-only provider in the space). ~61M unique consumer profiles in panel history; ~4.5B monthly real-world interaction events.

**B. Coverage:** UK confirmed; active footfall reporting in Netherlands, Belgium, Germany confirmed via 2025 benchmark publications. 11-country "major-conurbation" footprint advertised. Spain, Italy, Portugal, Nordics — [unconfirmed at retail-park / power-centre density]; suburban/exurban coverage outside major conurbations is the key gap.

**C. POI-visit query:** Yes — productised as "Footfall" and "Location Intelligence Reports" with per-site visitor origin geographies.

**D. Home/work labeling:** Yes — behavioural product labels home, work, and "lifestyle" recurring locations per device.

**E. Spatial output:** Postal sector / LSOA / custom polygon in UK; equivalent admin units (LAU, postal) in EU. Maps to H3 res-7 via weighted area-overlap conversion.

**F. Delivery:** SaaS platform, API, and CSV/GeoJSON exports per Datarade listing.

**G. Cadence:** Weekly benchmarks; near-real-time available for enterprise.

**H. Pricing:** Datarade-listed (quote-based); free sample reports offered. Mid-market scale — negotiable.

**I. Disclosure posture:** First-party-SDK provenance is a clean privacy story for UK/EU GDPR. Note: Oxford City Council withdrew Huq data in a prior episode — surface in due-diligence Q&A; withdrawal concerned interpretive use, not data collection.

---

## 4. Side-by-Side Comparison

| Dimension | Advan / Dewey | Replica | StreetLight | Spectus | Huq |
|---|---|---|---|---|---|
| **Coverage — Canada** | Yes (DA) | [unconfirmed] | Yes | — | — |
| **Coverage — US** | Yes (CBG) | Yes (48+DC) | Yes | Yes | — |
| **Coverage — UK / IE** | — | — | — | Yes | Yes (strongest) |
| **Coverage — DE / FR / NL / BE** | — | — | — | DE/FR yes; NL/BE [unconfirmed] | NL/BE/DE yes; FR [unconfirmed] |
| **Coverage — ES / IT / PT** | — | — | — | ES/IT yes; PT [unconfirmed] | [unconfirmed] |
| **Coverage — Nordics (SE/NO/DK/FI)** | — | — | — | [unconfirmed] | [unconfirmed] |
| **Home-origin product** | Yes (`visitor_home_cbgs`) | Yes (synthetic) | Derivable | Yes (Recurring Areas) | Yes |
| **Work-origin product** | Yes (`visitor_daytime_cbgs`) | Yes | Derivable | Yes | Yes |
| **Per-POI / per-polygon visit query** | Yes | Yes | Indirect | Yes (clean-room SQL) | Yes |
| **H3-native output** | No (CBG/DA → H3) | No (BG → H3) | No (custom → H3) | Partial (geohash) | No (postal → H3) |
| **Bulk delivery (Parquet/CSV)** | Yes (Dewey S3) | Export | Export | Clean-room export | Yes |
| **Monthly cadence** | Yes (+ weekly) | Seasonal (4/yr) | Rolling | Continuous | Weekly |
| **Observed vs synthetic** | Observed | Synthetic | Observed | Observed | Observed |
| **Public pricing signal** | Subscription tiers (no $) | Enterprise-only | Enterprise-only | Enterprise-only | Datarade quote |
| **Fit for this use case (1–5)** | **5 (NA)** | 3 (NA Phase 2) | 2 | **4 (EU research)** | **4 (UK/EU)** |

---

## 5. Recommended Build Architecture

**Primary — US + CA:** Advan Research via Dewey Data (`Patterns Plus`). Migration off `Monthly Patterns` is time-sensitive — that product retired 2026-03-01. Commercial tier required for public republication.

**Primary — UK/IE + Western EU conurbations:** Huq Industries. Strongest first-party panel and clean publication story for UK/IE/NL/BE/DE. Start with UK and confirm Tier-2/3 city density before committing to continental EU.

**Bridge / research-grade — Southern + Central EU (IT, ES, FR, DE):** Spectus clean room for clusters where Huq panel density is insufficient. Engineering-heavier integration; reserve for top-N clusters in these markets initially.

**Fallback — Nordics + uncovered regions + sparse rural clusters everywhere:** Retain WorldPop + radius-catchment model (existing 35/150 km rings). Mark catchments derived from radius-only with `model_source: radius` in `clusters-meta.json` so the UI can label them "modelled" rather than "observed."

**Phase 2 validation — US:** Replica Places product as a calibration cross-check against Advan flows for a representative megaregion (e.g., Texas Triangle). Use to compute a decay-coefficient correction applicable to radius-only clusters in uncovered regions.

**Deferred:** StreetLight — revisit only if a highway-flow or transportation-corridor overlay is added.

---

## 6. Integration Notes

**CBG → H3 res-7 (Advan US):** Use TIGER 2023 CBG boundaries; distribute each CBG's visitor count across overlapping H3 res-7 cells weighted by area-of-intersection × WorldPop population density. H3 res-7 (~5 km²) is coarser than most CBGs — most aggregate cleanly up.

**DA → H3 res-7 (Advan CA):** Same pattern with Statistics Canada DA boundaries (`lda_000b21a_e`). DAs are smaller than CBGs — mostly 1-to-1 aggregation.

**Postal sector / LAU → H3 res-7 (Huq UK + EU):** Area-weighted reapportionment via GeoPandas overlay; cache per-postal-unit H3 weights.

**Geohash-5 → H3 res-7 (Spectus):** Geohash-5 ≈ 4.9 km × 4.9 km — near 1:1 by area; use centroid-in-hex assignment with light area weighting.

**Per-cluster BentoBox stats:** Sum visitor counts across H3 cells inside each cluster's catchment polygon; report `home_pop`, `work_pop`, `top5_home_h3`, `top5_home_market` (resolved via existing Regional Market layer).

**PMTiles pipeline:** Add `layer6-mobility-home` and (Phase 2) `layer6-mobility-work` PMTiles built from per-cell visitor counts. Add a `source` attribute on each H3 feature: `advan | huq | spectus | radius` so the front-end can label gravity-only cells as modelled.

**Provenance discipline:** Every cluster row in `clusters-meta.json` carries `mobility_source` and `mobility_vintage` fields. Data methodology dialog (artifact A4) updated to disclose mixed sourcing.

---

## 7. Open Questions / Due-Diligence Items

1. **Advan commercial-tier price for full US+CA `Patterns Plus`** — quote needed from Dewey; confirm aggregate-publication clause explicitly.
2. **`Patterns Plus` schema parity with `Monthly Patterns`** — confirm `visitor_home_cbgs` / `visitor_daytime_cbgs` field names carry over before the 2026-03-01 sunset.
3. **Replica Canadian coverage** — confirm with Replica directly; published megaregions are US-focused.
4. **Huq subnational coverage outside major conurbations** — request panel-density map for UK retail parks beyond M25, and for German Tier-2/3 cities.
5. **Huq Oxford withdrawal episode** — request Huq's written response to the 2023 Oxford City Council methodology critique to attach to due-diligence file.
6. **Spectus commercial-vs-Data-for-Good** — Data-for-Good is non-commercial; commercial clean-room cost for IT+ES+FR+DE must be scoped separately.
7. **GDPR republication clause** — for every EU source, confirm aggregate-publication on a public website is permitted under the upstream SDK collection consent.
8. **Pricing tier for 6,815-cluster nightly refresh** — most vendors price by query/POI count; estimate annual run-rate before any commitment.
9. **Synthetic vs observed labeling in BCSC disclosure** — Replica outputs must be labeled "modelled" wherever surfaced publicly; align with `conventions/bcsc-disclosure-posture.md`.
10. **Spectus NL/BE/AT/CH/Nordics commercial density** — request coverage heatmap before relying on these markets.

---

*Sources: Dewey Data docs (docs.deweydata.io), Advan Research product pages (advanresearch.com/products/patternsplus), Replica documentation (documentation.replicahq.com), StreetLight/Jacobs marketing (streetlightdata.com, jacobs.com/technology/streetlight-insight), Spectus PEM Whitepaper (cuebiq.com), Spectus AWS Marketplace listing, Huq Industries Datarade listing (datarade.ai/data-providers/huq), Huq retail benchmarks (huq.io).*
