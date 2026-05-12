# project-gis Outstanding To-Do (Backlog)

**Snapshot:** 2026-05-12 (post-Sprint 13 follow-on)
**Purpose:** Cluster-level backlog of every open / deferred / held item not addressed in the current commit. Maintained alongside `tasks.md` (which carries sprint-close history) and `outbox.md` (which carries inter-session messages). Refreshed when items ship or new ones surface.

**Pipeline state:** 48,468 cleansed records · 6,815 deduplicated clusters · T3=28 · T2=1,309 · T1=3,374 · T0=2,104 · score 0–730 · live at gis.woodfinegroup.com.

**Recent commits:** Sprint 12 `8f09505` · search v2 `cfda0f4` · Sprint 13 follow-on `5f96ca0` (supercentre filter fix + civic OSM tile path). Code commits now in project-gis git (cluster/project-gis) via `git add -f`.

---

## Categories

- **A** · New surface from recent sprint (cheap auto-execute candidates)
- **B** · Operator-decision partials carried forward
- **C** · Substantial, focused-sprint scope
- **D** · Held / architecturally blocked
- **E** · Long-tail / nice-to-have

---

## A · Recent surfaces (2)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| A1 | **Carrefour-FR sub-format coverage** | outbox Sprint 12 close | S | Wikidata Q217599 returned 1,835 elements; final ingest 509 records (10% of expected ~5,200). Sub-formats Carrefour Express / City / Market / Contact likely use different brand:wikidata or none. Apply Aldi-NL precedent: `wikidata_id: ~` + `name_query: "Carrefour"` + `name_query_partial: true`. Re-ingest single chain; pipeline rebuild. |
| A2 | **Search highlight refinement** ✓ shipped 2026-05-09 | operator note | — | Two iterations: `074c34b` shipped initial dot-level highlight on `all-locations`; `cfda0f4` shipped v2 dual-mode (cluster-bubble stroke at Co-location Level + retailer-dot stroke at Retail Level) preserving the user's zoom mode. Operator's "stay in the mode" + "yellow circle around the searched retailer" both addressed. |

## B · Operator A3 partial carry-forward (2)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| B1 | **Auchan-FR ingest** | outbox Sprint 11 / 12 | M | Wikidata Q758603, ~600 stores. Mechanical pattern matches Sprint 12 Carrefour-FR. Operator chose Carrefour first; Auchan still queued. |
| B2 | **Mercadona-ES anchor question** | outbox Sprint 12 | S | Already ingested as Food family (1,603 records, verified Sprint 12 B2). Open question: should Mercadona promote to ALPHA in ES (parallel to Soriana in MX)? Mercadona is Spain's flagship hypermarket. Not yet asked. |

## C · Focused-sprint scope (2)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| C1 | **Overture Addresses spatial join** | tasks.md Group B3 | L | For POIs with null `addr:*`, spatial-join against Overture Addresses theme (≤15 m). Back-fill `full_address`. New ingest-pipeline step; would require its own sprint. |
| C2 | **Fred Meyer ALPHA removal review** | tasks.md Sprint 4 close | M | `fred-meyer-us` (92–132 PNW-regional stores) currently in ALPHA_ANCHORS. Operator flagged for review. Re-ingest + rebuild + tier audit to measure impact. |

## D · Held / architecturally blocked (4)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| D1 | **DataGraph entity writes (E2)** | tasks.md · inbox-archive 2026-05-06 | M | service-content graph live (127.0.0.1:9081, 10,414 entities, module_id=woodfine). Held by Master pending schema-sync. Awaiting Master ratification. |
| D2 | **Sherwood Park Costco / 3 km radius gap (D5)** | tasks.md Group D5 | S (doc note) or M (recalibration) | Costco 3.96 km from Walmart anchor — outside max_r=3.0 km. Visible in All Locations layer; absent from cluster bento. Options: (a) raise to 5 km (cross-cluster pollution risk); (b) document as methodology limit. Sprint 11 lean: leave at 3 km. |
| D3 | **G1 — Rust service-ingest crate** | tasks.md Group G | L | Hold per workspace strategy. |
| D4 | **G2 — OD Study layer** | tasks.md Group G | L | Hold per workspace strategy. |

## E · Long-tail / nice-to-have (4)

| # | Item | Source | Effort | Notes |
|---|---|---|---|---|
| E1 | **IPEDS EF2023A.zip integration** | inbox-archive 2026-05-05 | M | US higher-education enrolment fixture. Master ranked below D3. Low priority. |
| E2 | **5 draft open questions still open** | draft frontmatter inspection | S each | Five staged drafts carry one open_questions field each (location-intelligence-ux, ring-retailer-click-ux, zoom-prefetch-pattern, gis-as-bim-substrate, uk-eu-food-retail-coverage). Editorial gate (project-editorial Task scope, not project-gis). |
| E3 | **PRODUCT_VISION — Beacon + Opportunity Engine** | drafts-outbound/PRODUCT_VISION.md | L | Aspirational features (Cluster DNA radar chart, dynamic re-scoring lenses, Opportunity Zones). No scoping; vision-stage. Not in scope until operator commits. |
| E4 | **Blank-zone diagnostic z6–10** | tasks.md Sprint 5 close | S | Possible bubble flicker mid-zoom. Sprint 9 prefetch + threshold-9 likely resolved; lower priority follow-up. |

## F · Retired by operator decision (for reference)

- **A4 country expansion (BE / LU / IE / CH)** — operator answered "None this quarter" in Sprint 12 prelude.
- **A2 tienda-del-sol-mx** — operator answered "drop chain entirely" in Sprint 12 (executed `8f09505`).
- **Soriana-mx ALPHA promotion** — operator approved in Sprint 12 (`8f09505`).

---

## Counts

**Total open: 13 items** (was 14 before this commit; A2 search-highlight shipped).
- A: 1 (Carrefour-FR sub-format)
- B: 2 (Auchan-FR, Mercadona-ES anchor question)
- C: 2 (Overture join, Fred Meyer review)
- D: 4 held
- E: 4 long-tail

**Effort distribution:** S (under 30 min): 5 · M (1–4 h): 5 · L (multi-session): 3.

---

## Recommended sequencing (Sprint 13 candidate)

The cheapest forward path:

1. **Auto-execute A1** (Carrefour-FR sub-format refinement — single-chain re-ingest + pipeline rebuild; ~1 hour).
2. **Ask operator B2** (Mercadona-ES anchor promotion — same shape as Sprint 12 Soriana decision).
3. **Optionally execute B1** (Auchan-FR ingest) and **C2** (Fred Meyer review).

D + E carried indefinitely; revisit when Master architectural decisions land or when long-tail urgency rises.

---

## Maintenance protocol

- When an item ships, mark it ✓ shipped with the commit hash and date, leave it in the file for one sprint, then move to a "Retired" section (F) on the next refresh.
- When a new item surfaces, append to the appropriate category. Re-count.
- Re-snapshot the pipeline-state header on each meaningful pipeline rebuild.
- Cross-reference `tasks.md` sprint-close sections for full historical context.
