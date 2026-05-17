# project-gis Outstanding To-Do (Backlog)

**Snapshot:** 2026-05-17 (Phase 13: Tesco/Sainsbury's re-ingest + MITMA + UK chain expansion)
**Purpose:** Cluster-level backlog of every open / deferred / held item not addressed in the current commit. Maintained alongside `tasks.md` (sprint history) and `outbox.md` (inter-session messages). Refreshed when items ship or new ones surface.

**Pipeline state:** 62,201 cleansed records · 13,164 clusters · T1=442 (US:303 ES:42 CA:32 MX:28 FR:13 DE:13 GB:10 NO:1) · T2=1,021 · T3=2,211 · T4=9,490 · P10=0.25 + Gate B (hc_count≥1) · live at gis.woodfinegroup.com.

---

## Categories

- **A** · Recent surfaces (cheap auto-execute candidates)
- **B** · Data Enrichment (In Progress)
- **C** · Operator-decision partials carried forward
- **D** · Substantial, focused-sprint scope
- **E** · Held / architecturally blocked
- **F** · Long-tail / nice-to-have
- **G** · Retired by operator decision

---

## A · Recent surfaces (3)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| A1 | **synthesize-od-study.py re-run after Phase 13** | Phase 13 close | S | 3,043 new clusters (from Tesco/Sainsbury's/ASDA/Morrisons expansion) lack rank_pp_iso in catchment-data.json. Re-running synthesize-od-study.py and build-geometric-ranking.py would give these clusters real percentile ranks and likely push GB T1 above 10. IN PROGRESS as of Phase 13. |
| A2 | **Tiendas 3B MX under-count** | Phase 13 audit | M | tiendas-3b-mx: 151→247 (expected ~1,700). name_query "Tiendas 3B" exact match in OSM returns only 247. OSM coverage is genuinely thin — needs OSM mapping campaign or alternative data source. Not a config fix. |
| A3 | **LODES (US) + StatCan (CA) OD ingest** | Phase 13 analysis | M | build-mobility-tiles.py skips US LODES + CA StatCan because lodes-work-od-us.jsonl and statcan-work-od-ca.jsonl are missing from service-mobility. CA StatCan: manual download required (URL dead — see E5). US LODES: ingest-lodes.py available; needs run. |

## B · Data Enrichment

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| B1 | **service-census Ingest** | Operator Directive | — | ✓ DONE Sprint 14. WorldPop 2026 100m → H3 res-7; 1,928,815 cells; 13 countries. |
| B2 | **service-spend Ingest** | Operator Directive | — | ✓ DONE Sprint 14. |
| B3 | **service-mobility Ingest — MITMA ES** | Operator Directive | — | ✓ DONE Phase 13. 58 ES clusters mobility_source=mitma. |
| B4 | **service-mobility Ingest — US LODES** | Phase 13 surface | M | ingest-lodes.py available; lodes raw data at service-fs/service-mobility/raw/ (check). Would populate lodes-work-od-us.jsonl and update ~4,000+ US clusters to mobility_source=lodes. |

## C · Operator-decision partials (3)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| C1 | **Auchan-FR ingest** | outbox Sprint 11 / 12 | M | Wikidata Q758603, ~600 stores. Already in ALPHA_HYPERMARKET; JSONL exists (auchan-fr.jsonl). Check count and promote to REGION_CONFIG FR anchor if strong. |
| C2 | **Path C composition — HW∧HM as T1** | Phase 11 analysis | M | Adding HW∧HM (e.g., HomeDepot+Walmart clusters) as T1-qualifying composition raises NA T1 by ~199 at P=0.20. Methodologically significant; operator review required before applying. |
| C3 | **More US chains for T1 density** | Phase 12 outbox | M | Wegmans (~110 stores, Q1182328), WinCo Foods (~130, Q2584339), Sprouts (~380, Q7580917) — all ALPHA_HYPERMARKET candidates. Would add clusters but mostly T2/T3 without LODES rank enrichment. |

## D · Focused-sprint scope (3)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| D1 | **Overture Addresses spatial join** | tasks.md Group B3 | L | S3 access not tested. extract-overture-addresses.py committed (21cf18df). Operator should verify DuckDB+S3 before scheduling. Expected ~50GB S3 read. |
| D2 | **UK/FR/DE OD ingest** | Phase 11 research | L | ONS ODWP01EW + INSEE FD_MOBPRO + BA Pendler all viable; all require manual download. See work/od-data-research-uk-fr-de.md. Would populate EU work-reach tiles. |
| D3 | **Kontur Population replacement** | Phase 11 research | M | H3 res-8 global dataset, CC BY 4.0, HDX download (~2.3GB). Better urban accuracy than WorldPop 100m. See work/kontur-integration-plan.md. 1 sprint estimate. |

## E · Held / architecturally blocked (3)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| E1 | **DataGraph entity writes (E2)** | tasks.md · inbox-archive 2026-05-06 | M | service-content graph live (127.0.0.1:9081). Held by Master pending schema-sync. |
| E3 | **G1 — Rust service-ingest crate** | tasks.md Group G | L | Hold per workspace strategy. |
| E5 | **StatCan commuter flow URL** | Sprint 16 | S | CompDataDownload.cfm dead. Figshare mirror documented in ingest-statcan.py (21cf18df). Manual download needed. |

## F · Long-tail / nice-to-have (4)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| F1 | **IPEDS EF2023A.zip integration** | inbox-archive 2026-05-05 | M | URLs live: nces.ed.gov/ipeds/datacenter/data/EF2023A.zip. enrich_university_enrollment.py already written. Low priority. |
| F2 | **5 draft open questions** | draft frontmatter | S each | Editorial gate (project-editorial Task scope). |
| F3 | **PRODUCT_VISION — Beacon + Opportunity Engine** | drafts-outbound | L | Vision-stage; no operator commitment. |
| F4 | **Blank-zone diagnostic z6–10** | tasks.md Sprint 5 | S | Lower priority follow-up. |

## G · Retired by operator decision

- **A3 Data Manifest & Legal Compliance** ✓ shipped 2026-05-12.
- **Service Scaffolding** ✓ shipped 2026-05-12.
- **A4 country expansion (BE / LU / IE / CH)** — operator answered "None this quarter."
- **A2 tienda-del-sol-mx** — operator answered "drop chain entirely."
- **Soriana-mx ALPHA promotion** ✓ shipped Sprint 12.
- **Mercadona-es ALPHA promotion** ✓ shipped Phase 1 2026-05-16 (D1).
- **whole-foods-us ALPHA promotion** ✓ shipped Phase 12 2026-05-17 (3b367a9f).
- **chedraui-mx ALPHA promotion** ✓ shipped Phase 12 2026-05-17 (3b367a9f).
- **ASDA-UK / Morrisons-UK ingest** ✓ shipped Phase 12 2026-05-17 (3b367a9f).
- **H-E-B ingest** ✓ shipped Phase 12 2026-05-17 (3b367a9f).
- **T1 threshold recalibration** ✓ shipped Phase 11/12 (P10=0.25 + Gate B; T1=442).
- **tesco-uk / sainsburys-uk name_query re-ingest** ✓ shipped Phase 13 2026-05-17.
- **MITMA ES mobility_source** ✓ shipped Phase 13 2026-05-17 (58 clusters updated).
- **Stale backup deletion** ✓ shipped Phase 13 2026-05-17 (~35M freed).

---

## Counts

**Total open: 14 items**
- A: 3 (od-study re-run, tiendas-3b, LODES)
- B: 1 (LODES)
- C: 3 (Auchan, Path-C, more US chains)
- D: 3 (Overture, EU OD, Kontur)
- E: 3 held
- F: 4 long-tail

---

## Maintenance protocol

- When an item ships, mark it ✓ shipped with the commit hash and date, leave it for one sprint, then move to G.
- When a new item surfaces, append to the appropriate category.
- Re-snapshot pipeline-state header on each meaningful rebuild.
