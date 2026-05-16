# project-gis Outstanding To-Do (Backlog)

**Snapshot:** 2026-05-15 (Sprint 14: O-D Catchment + Phase C EU Hardware)
**Purpose:** Cluster-level backlog of every open / deferred / held item not addressed in the current commit. Maintained alongside `tasks.md` (which carries sprint-close history) and `outbox.md` (which carries inter-session messages). Refreshed when items ship or new ones surface.

**Pipeline state:** 52,362 raw records · 48,468 cleansed · 6,815 deduplicated clusters · T3=28 · T2=1,309 · T1=3,374 · T0=2,104 · score 0–730 · B1/B2/B3 DONE · catchment layers live · live at gis.woodfinegroup.com.

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

## A · Recent surfaces (1)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| A1 | **Carrefour-FR sub-format coverage** | outbox Sprint 12 close | S | Wikidata Q217599 returned 1,835 elements; final ingest 509 records (10% of expected ~5,200). Sub-formats Carrefour Express / City / Market / Contact likely use different brand:wikidata or none. Apply Aldi-NL precedent: `wikidata_id: ~` + `name_query: "Carrefour"` + `name_query_partial: true`. Re-ingest single chain; pipeline rebuild. |

## B · Data Enrichment (4 of 4 complete)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| B1 | **service-census Ingest** | Operator Directive | M | [x] DONE Sprint 14. WorldPop 2026 100m → H3 res-7; 1,928,815 cells; 13 countries. |
| B2 | **service-spend Ingest** | Operator Directive | M | [x] DONE Sprint 14. Grocery/hardware/wholesale per-capita multipliers applied; 1,988,375 cells. |
| B3 | **service-mobility Ingest** | Operator Directive | M | [x] DONE Sprint 14. Synthesized via crow-flies O-D rings (35km primary / 150km secondary). od-summary.jsonl = B3 artifact. |
| B4 | **Europe Data Parity Audit** | Operator Directive | S | Sprint 14: EU hardware chains ingested (toom/hageb/brico/silvan/praktiker/byko). Ongoing: Carrefour-FR sub-format coverage (A1). |

## C · Operator A3 partial carry-forward (2)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| C1 | **Auchan-FR ingest** | outbox Sprint 11 / 12 | M | Wikidata Q758603, ~600 stores. Mechanical pattern matches Sprint 12 Carrefour-FR. Operator chose Carrefour first; Auchan still queued. |
| C2 | **Mercadona-ES anchor question** | outbox Sprint 12 | S | Already ingested as Food family (1,603 records, verified Sprint 12 B2). Open question: should Mercadona promote to ALPHA in ES (parallel to Soriana in MX)? Mercadona is Spain's flagship hypermarket. Not yet asked. |

## D · Focused-sprint scope (2)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| D1 | **Overture Addresses spatial join** | tasks.md Group B3 | L | For POIs with null `addr:*`, spatial-join against Overture Addresses theme (≤15 m). Back-fill `full_address`. New ingest-pipeline step; would require its own sprint. |
| D2 | **Fred Meyer ALPHA placement — ingest pending** | tasks.md Sprint 4 close | M | `fred-meyer-us` now in ALPHA_HYPERMARKET (Phase 1 taxonomy). Verified 2026-05-16: 0 locations in service-business-cleansed → 0 clusters → zero tier impact. Semantically correct placement; no removal needed. Dormant until fred-meyer-us ingest is completed. |

## E · Held / architecturally blocked (5)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| E1 | **DataGraph entity writes (E2)** | tasks.md · inbox-archive 2026-05-06 | M | service-content graph live (127.0.0.1:9081, 10,414 entities, module_id=woodfine). Held by Master pending schema-sync. Awaiting Master ratification. |
| E2 | **Sherwood Park Costco / 3 km radius gap (D5)** | tasks.md Group D5 | CLOSED | Verified 2026-05-16 (Phase 2 diff harness): Costco and HomeDepot both generate T3 Local clusters (rank_pp≈0.44, hc=1). The Costco is 3.96 km from nearest Walmart anchor and correctly forms its own cluster. Under Phase 2 predicate engine, Sherwood Park clusters correctly score T3 (no Hypermarket co-tenant within 3 km radius). Not a T1 gap — the 3 km radius limit is methodology-correct. E2 is closed: not a defect. |
| E3 | **G1 — Rust service-ingest crate** | tasks.md Group G | L | Hold per workspace strategy. |
| E4 | **G2 — OD Study layer** | tasks.md Group G | — | [x] DONE Sprint 14. synthesize-od-study.py ships primary/secondary catchment. AWAY sub-toggle stubs to HOME pending daytime population data. |
| E5 | **StatCan commuter flow URL (98-400-X2021007)** | Sprint 16 | S | StatCan CompDataDownload.cfm and 98-10-0494-01 fallback both returning dead links (2026-05-15). ingest-statcan.py exits with instructions. Needs: new working URL for Journey-to-Work bulk CSV, or operator manual download. Blocks CA WORK reach in layer6. |

## F · Long-tail / nice-to-have (4)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| F1 | **IPEDS EF2023A.zip integration** | inbox-archive 2026-05-05 | M | US higher-education enrolment fixture. URLs verified live 2026-05-16: nces.ed.gov/ipeds/datacenter/data/EF2023A.zip (2.8 MB) + HD2023.zip (1.1 MB). enrich_university_enrollment.py already written. Run when Phase 3 (civic enrichment) is prioritised. Low priority. |
| F2 | **5 draft open questions still open** | draft frontmatter inspection | S each | Five staged drafts carry one open_questions field each (location-intelligence-ux, ring-retailer-click-ux, zoom-prefetch-pattern, gis-as-bim-substrate, uk-eu-food-retail-coverage). Editorial gate (project-editorial Task scope, not project-gis). |
| F3 | **PRODUCT_VISION — Beacon + Opportunity Engine** | drafts-outbound/PRODUCT_VISION.md | L | Aspirational features (Cluster DNA radar chart, dynamic re-scoring lenses, Opportunity Zones). No scoping; vision-stage. Not in scope until operator commits. |
| F4 | **Blank-zone diagnostic z6–10** | tasks.md Sprint 5 close | S | Possible bubble flicker mid-zoom. Sprint 9 prefetch + threshold-9 likely resolved; lower priority follow-up. |

## G · Retired by operator decision (for reference)

- **A3 Data Manifest & Legal Compliance** ✓ shipped 2026-05-12 (index.html buttons + DATA-MANIFEST.md + drafts-outbound).
- **Service Scaffolding** ✓ shipped 2026-05-12 (monorepo READMEs + deployment folder creation).
- **A4 country expansion (BE / LU / IE / CH)** — operator answered "None this quarter" in Sprint 12 prelude.
- **A2 tienda-del-sol-mx** — operator answered "drop chain entirely" in Sprint 12 (executed `8f09505`).
- **Soriana-mx ALPHA promotion** — operator approved in Sprint 12 (`8f09505`).

---

## Counts

**Total open: 17 items** (was 13 before Data Enrichment phase).
- A: 1 (Carrefour-FR)
- B: 4 (Census, Spend, Mobility, EU Audit)
- C: 2 (Auchan, Mercadona)
- D: 2 (Overture, Fred Meyer)
- E: 4 held
- F: 4 long-tail

---

## Maintenance protocol

- When an item ships, mark it ✓ shipped with the commit hash and date, leave it in the file for one sprint, then move to a "Retired" section (G) on the next refresh.
- When a new item surfaces, append to the appropriate category. Re-count.
- Re-snapshot the pipeline-state header on each meaningful pipeline rebuild.
- Cross-reference `tasks.md` sprint-close sections for full historical context.
