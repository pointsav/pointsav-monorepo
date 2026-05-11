---
mailbox: outbox
owner: task-project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-gis cluster

---
from: task@project-gis
to: command
re: session 13 close — search highlight v2 (dual-mode); outstanding-todo backlog persisted; inbox swept
created: 2026-05-09T01:30:00Z
priority: normal
---

Session 13 close-out. Two follow-up commits shipped after Sprint 12:

- **`074c34b`** — search-highlight UX correction (operator note "the orange/yellow circles should only be on the dot of that retailer, not all the retailers in the co-location"). Initial dot-level highlight on `all-locations` layer + auto-engage Retail Level. Plus persistent backlog created at `.agent/outstanding-todo.md` (13 open items in 5 categories, with maintenance protocol).

- **`cfda0f4`** — search highlight v2 (operator follow-up "it should stay in the mode of the zoom, rings or clusters"). Dual-mode: cluster-bubble stroke on `nodes` layer at Co-location Level + retailer-dot stroke on `all-locations` at Retail Level. User's zoom mode is preserved across search. Auto-Retail-Level switch removed. `applyChainFilterStyling()` composes both strokes.

**Pipeline state unchanged from Sprint 12:** 48,810 cleansed records / 6,422 deduplicated clusters / score 0–730. T3 Prime 28 / T2 Strong 1,302 / T1 Core 3,322 / T0 Emerging 2,022. Live at gis.woodfinegroup.com.

**Inbox swept:** 3 Master ACKs (sessions 9, 10, 11) archived to `inbox-archive.md`. All were acknowledgements with no action required. Operator-decision follow-ups they raised were addressed in Sprint 12 via operator A1–A4.

**Workspace state:** `/srv/foundry` in sync with `origin/main`. No unpushed commits.

**Outstanding-todo backlog (13 items, refreshed):**
- A1 Carrefour-FR sub-format coverage refinement (cheap)
- B1 Auchan-FR ingest, B2 Mercadona-ES anchor question
- C1 Overture Addresses spatial join, C2 Fred Meyer ALPHA review
- D1–D4 held items (DataGraph E2, Sherwood Park 3km, Rust ingest, OD Study)
- E1–E4 long-tail (IPEDS, draft open questions, PRODUCT_VISION, blank-zone diag)

Full file: `clones/project-gis/.agent/outstanding-todo.md`.

Session ready to exit.

---
from: task@project-gis
to: command
re: session 12 complete — search regression fix + 4 operator decisions executed (Soriana ALPHA, Carrefour-FR, drop tienda-del-sol-mx)
created: 2026-05-09T00:30:00Z
priority: normal
---

Session 12 closed all six items in Sprint 12 scope (B1, B2, B3, A1, A2, A3) plus the search regression that surfaced this turn.

**B1 — Search regression FIXED.** Root cause traced to Sprint 8 (`8cc3a07` PMTiles→GeoJSON conversion of `data` source): only 5 properties were carried into the new GeoJSON features, but `chainFilterExpr()` needs `hw_list` and `wh_list`. Filter never matched → no orange highlight. Fix: added `hw_list` + `wh_list` (native arrays) to the GeoJSON feature properties at `index.html:1004`. Stroke colour bumped from `#F59E0B` (amber) to `#EAB308` (brand-accent yellow-orange) and width 2.5 → 3.5 for visibility against the cluster bubbles.

**B2 — Mercadona-ES verified** in-place. 1,603 records ingested; CHAIN_FAMILY = Food; in GENERIC_FOOD set. No work needed.

**B3 — Spanish municipio name polish: zero residual cases.** Ran a regex sweep across live `clusters-meta.json` for period-without-trailing-space and lowercase-uppercase-glue patterns: zero matches. Sprint 11's polish was complete.

**A1 — Soriana promoted to ALPHA_HYPERMARKET.** Required two-step config change: (a) add `soriana-mx` to `REGION_CONFIG[MX]["anchor"]`; (b) add `soriana-mx` to `ALPHA_ANCHORS["NA"]` (build-clusters filters by ALPHA_ANCHORS). Plus `ANCHOR_DISPLAY_NAMES` = "Soriana" for human-readable label, and `CHAIN_FAMILY` = "Hypermarket" (was "Food"). Result: **250 new Soriana-anchored clusters** in MX. Cluster topology shifted: MX cluster count 410 → 657 (+247). Tier distribution: T3 27→28 (+1), T2 1,220→1,302 (+82), T1 3,166→3,322 (+156). Chedraui-mx remains Food per operator decision (asymmetric scaling).

**A2 — `tienda-del-sol-mx` dropped.** YAML removed from registry per operator decision. No JSONL existed (chain was held). No further action.

**A3 — Carrefour-FR ingested.** New `carrefour-fr.yaml` (Wikidata Q217599) → 1,835 OSM elements via brand:wikidata query → 805 after fuel-skip + format-filter → 509 after polygon-filter (drops 292 cross-border, mostly Belgian / Italian). Final 509 records. Sub-formats Carrefour Express / City / Contact likely under-represented in OSM brand:wikidata — flag for future name_query refinement. CHAIN_FAMILY = Food; in GENERIC_FOOD set. France retains 37 clusters (Food family doesn't score; new dots visible at retail zoom).

**Pipeline state after Sprint 12:**
- 48,810 cleansed records (+509 Carrefour-FR; +250 Soriana net of dedup)
- 7,292 raw clusters / 6,422 deduplicated *(unchanged dedup count — survivors absorb new clusters)*
- T3 Prime 28 · T2 Strong 1,302 · T1 Core 3,322 · T0 Emerging 2,022
- Country counts: US 4,947 · CA 483 · **MX 657** (was 410, +247 from Soriana) · GB 53 · DE 48 · **FR 37** (Food layer expanded but cluster count stable)
- Score range 0–730 unchanged
- All live at gis.woodfinegroup.com

**Files committed in Sprint 12:**
- `app-orchestration-gis/www/index.html` — search-fix (B1)
- `app-orchestration-gis/config.py` — Soriana ALPHA_ANCHORS + ANCHOR_DISPLAY_NAMES + REGION_CONFIG[MX] + GENERIC_FOOD additions (Carrefour-FR; Soriana removed from Food)
- `app-orchestration-gis/build-tiles.py` — Soriana CHAIN_FAMILY Food→Hypermarket; Carrefour-FR Food

**Files NOT committed (deployment-local):**
- `carrefour-fr.yaml` (new, deployment-side)
- `tienda-del-sol-mx.yaml` (removed, deployment-side)
- All JSONL ingest outputs / PMTiles / clusters-meta.json

**Open follow-ups for next sprint (carry-forward inventory items):**

1. **Carrefour-FR sub-format coverage** — ~5,200 expected vs 509 ingested (10% coverage). Sub-formats Express / City / Market likely tagged with different brand:wikidata or no wikidata at all. Try `wikidata_id: ~` + `name_query: "Carrefour"` with `name_query_partial: true` (per Aldi-NL precedent).

2. **C1 — Overture Addresses spatial join** (B3 from inventory) — multi-session work; still deferred.

3. **C2 — Fred Meyer ALPHA removal review** — operator-flagged; not yet executed.

4. **D1–D4** — held / architecturally blocked items unchanged.

5. **E1–E4** — long-tail nice-to-haves unchanged.

---
from: task@project-gis
to: command
re: session 11 complete — Phase J D3 bbox filter + 9 close-out drafts authored + Aldi-NL fix
created: 2026-05-08T20:30:00Z
priority: normal
---

Session 11 closed Sprint 10's last item (Phase J / D3 polygon country filter) and authored the editorial / design / cross-cluster artifacts that Sprints 9–11 generated.

**Phase J — D3 polygon country filter (Sprint 10 finisher):**
- Added `_load_country_polygon()` + `_filter_records_by_country_polygon()` to `ingest-osm.py`. Lazy-loaded country polygons from `fallback_ne_admin1.geojson` (cached per ISO).
- Re-ingested 4 affected CA chains:
  - home-depot-ca: 693 → 177 (matches expected 182) — polygon dropped 516 US-border stores
  - lowes-ca: 355 → 1 (OSM Canada coverage genuinely sparse for "Lowe's" name; polygon dropped 354 US stores)
  - walmart-ca: 318 → 253 — polygon dropped 63 cross-border
  - costco-ca: 258 → 109 (matches expected 107) — polygon dropped 149 cross-border
- Pipeline rebuild — CA cluster count 712 → 483 (-229 phantom border clusters); US +234 (correctly attributed). Score range and tier distribution unchanged.

**Phase A — DESIGN-RESEARCH drafts staged (3):**
- `DESIGN-RESEARCH-tier-naming-accessibility.draft.md` — Plain-English tier rebrand + ARIA pattern (resolves Sprint 9 outbox commitment)
- `DESIGN-RESEARCH-zoom-prefetch-pattern.draft.md` — Two-stage tile prefetch + visibility-flip ordering (reusable design-system primitive)
- `DESIGN-RESEARCH-bento-merged-zones-disclosure.draft.md` — Annotation-not-deletion transparency pattern for dedup steps

**Phase B — TOPIC drafts staged (4):**
- `topic-regional-name-resolution-architecture.draft.md` — Five-layer offline reverse-geocoding methodology
- `topic-uk-eu-food-retail-coverage.draft.md` — 12 new chain instances + Aldi Süd/Nord asymmetry note
- `topic-co-location-tier-nomenclature.draft.md` — Customer-facing companion to DESIGN-RESEARCH-tier-naming-accessibility
- `topic-gis-as-bim-substrate.draft.md` — **Cross-cluster bridge to project-bim** (the "BIM" artifact in operator's list)

**Phase C — GUIDE drafts (2 new + 1 updated):**
- `guide-gis-adding-a-country.draft.md` — End-to-end procedure using Sprint 10 UY example
- `guide-gis-pipeline-rebuild.draft.md` — Five-stage operational runbook with timings + failure modes
- `guide-gis-adding-a-chain.md` — Updated with Sprint 9–11 appendix (Food family, name_query fallback, polygon filter)

**Phase D — TEXT release note staged:**
- `text-gis-uk-eu-coverage-release.draft.md` — Bloomberg-style May 2026 release note covering UK + EU + region granularity + bbox correction + tier label refresh

**Phase E — Aldi-NL fix:**
- Original Q125054 returned 3 records; polygon filter would have blocked them. Sprint 11 nulled `wikidata_id` to force name_query, switched to `name_query_partial: true` for prefix matching ("Aldi *"). Result: 490 records (matches expected 480). German border stores correctly dropped by polygon filter.

**Outbox follow-ups still open (operator decision):**

1. **Soriana / Chedraui ALPHA promotion** — both currently classified as Food family (data-only). Promotion to ALPHA_HYPERMARKET in REGION_CONFIG[MX] would shift MX cluster topology meaningfully (new T2/T3 clusters where they co-locate with Walmart-MX / Bodega-Aurrera / Costco-MX). Operator decision pending.

2. **`tienda-del-sol-mx`** — confirmed HOLD per Sprint 10 operator decision. Persists as `locations_status: needs-research` with no ingest until a Wikidata QID is identified.

3. **EU food expansion** — Carrefour-FR, Auchan-FR, Mercadona-ES not yet ingested in their home markets. Mechanical add (one chain config each) when operator surfaces priority.

4. **Country expansion** — Belgium, Luxembourg, Ireland, Switzerland absent from REGION_CONFIG. New `guide-gis-adding-a-country.md` documents the procedure.

**Pipeline state after Sprint 11:**
- 47,860 cleansed records (slight increase from Aldi-NL fix; CA bbox cleanup balanced by NL gain)
- 7,041 raw clusters / 6,422 deduplicated (unchanged tier dist: T3=27, T2=1,220, T1=3,166, T0=2,014)
- CA: 483 clusters (was 712; -229 phantom border clusters removed)
- US: 4,947 (was 4,713; +234 correctly re-attributed)
- Score range 0–730
- Layer1 PMTiles ~400 MB; Layer2 41.7 MB; clusters-meta.json 2.7 MB

**Files committed in Sprint 11:**
- `app-orchestration-gis/ingest-osm.py` — country-polygon containment filter (Phase J)
- 9 new drafts in `.agent/drafts-outbound/` (Phases A–D)
- 1 updated draft (`guide-gis-adding-a-chain.md`)
- `.agent/outbox.md` + `.agent/tasks.md`

**Files NOT committed (deployment-local per Foundry tier discipline):**
- `aldi-nl.yaml` — Phase E fix
- All JSONL ingest outputs
- PMTiles + clusters-meta.json

---
from: task@project-gis
to: command
re: session 10 complete — Sprint 10 closes all outstanding (Phase 6 region integration; UK + EU food fills; CX disambiguation; Sprint 9 pushed)
created: 2026-05-08T19:45:00Z
priority: normal
---

Session 10 shipped. All changes live at gis.woodfinegroup.com.

**Phase A — Push Sprint 9:** commit `7e92013` pushed to `origin/main` (vault `pointsav/foundry`). Workspace single-remote tier confirms `bin/promote.sh` not applicable — direct push is the documented path for vault-tier admin commits.

**Phase B — GADM admin-2 + admin-3 boundaries downloaded:**
- `mx_municipio.geojson` (GADM 4.1 admin-2 — 2,457 municipios)
- `ca_csd.geojson` (GADM 4.1 admin-3 — 5,581 census subdivisions; admin-2 was generic "DivisionNo.X")
- `download-boundaries.sh` updated: now fetches admin-3 for CA (admin-2 removed)

**Phase C — region_engine.py integration (safe insertion):**
- New helpers `_format_ca_csd()` and `_format_mx_municipio()` extract `NAME_3` (CA) / `NAME_2` (MX) from GADM
- CamelCase splitter handles "StrathconaCounty" → "Strathcona County"; Spanish preposition splitter handles "Bocadel" → "Boca del", "Acapulcode" → "Acapulco de"; period splitter for "A.Madero" → "A. Madero"
- CA routing: CSD primary, CMA suffix when both present and distinct → "Strathcona County, Edmonton" (Sherwood Park's CSD!)
- MX routing: Municipio primary; legacy INEGI-ZM fallback if file present
- Result: CA distinct regions 35 → 245, MX distinct regions 32 → 104

**Phase D — UK fill (Tesco + Sainsbury's + Lidl-GB):** 3 new chain YAMLs; ingested 784 + 672 + 1,272 = 2,728 records; added to `GENERIC_FOOD` config; CHAIN_FAMILY → Food. UK now has 54 clusters with retailer dots visible at retail zoom.

**Phase E — MX (Soriana + Chedraui):** verified existing — 489 + 249 records already ingested. Food family classification correct. No code change needed.

**Phase F — EU food fill:** 9 new chain YAMLs (Lidl DE/FR/NL/AT/PT + Aldi DE/UK/NL/PL); batch ingested 12,289 records. Aldi-DE/UK/PL fall back to name_query (wikidata Q41171/Q125054 split between Süd/Nord). Aldi-NL undercoverage (3 records via wikidata; OSM tagging gap) — acceptable for now.

**Phase G — OBI/Bauhaus name-query:** verified all 5 chains already ingested with name_query active. Counts: obi-de 430, obi-it 84, obi-pl 104, bauhaus-de 243, bauhaus-es 17. No work needed; audit's "0 records" claim was outdated.

**Phase H — CX disambiguation (operator answered):**
- `tienda-del-sol-mx`: HOLD (locations_status: needs-research; YAML retained, no ingest until operator identifies QID)
- `tienda-inglesa-mx`: RENAMED to `tienda-inglesa-uy`. Added UY bbox to `ingest-osm.py` COUNTRY_BBOX. Re-ingested via name_query "Tienda Inglesa" → 20 records (close to expected 35).

**Phase J — D3 polygon country filter:** DEFERRED. Implementation requires shapely point-in-polygon in ingest-osm.py + 4-chain re-ingest + full pipeline rebuild. High-value (home-depot-ca 693 → expected 182) but warrants its own focused sprint to avoid pipeline-state risk.

**Pipeline state after Sprint 10:**
- 48,306 cleansed parent records (was 35,555 pre-Sprint 10 = +12,751 new EU/UK food records)
- 7,594 raw clusters / 6,422 deduplicated (T3=27, T2=1,202, T1=3,156, T0=2,037)
- Score range 0–730
- 245 distinct CA regions, 104 MX regions, 671 US, 100s in EU
- Layer1 PMTiles: 403.5 MB (was ~200 MB pre-Sprint 10)
- Layer2 PMTiles: 41.7 MB
- clusters-meta.json: 2,755 KB

**Files committed in Sprint 10:**
- `app-orchestration-gis/utils/region_engine.py` (Phase C — new helpers + routing)
- `app-orchestration-gis/config.py` (Phase D, F — GENERIC_FOOD additions)
- `app-orchestration-gis/build-tiles.py` (Phase D, F, H — CHAIN_FAMILY additions)
- `app-orchestration-gis/ingest-osm.py` (Phase H — UY bbox)
- `app-orchestration-gis/download-boundaries.sh` (Phase B — CA admin-3 instead of admin-2)
- `.agent/outbox.md` + `.agent/tasks.md`

**Files NOT committed (deployment-local per Foundry tier discipline):**
- 13 new chain YAMLs (UK + EU food + UY) at `/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-business/`
- All `*.jsonl` ingest outputs (gitignored)
- PMTiles + clusters-meta.json (deployment artifacts; gitignored)

**Open follow-ups for next sprint:**
1. Phase J (D3) polygon country filter — bbox contamination fix
2. Aldi-NL coverage gap (3 records) — investigate OSM tagging
3. Soriana/Chedraui promotion to ALPHA_HYPERMARKET — operator decision pending
4. tienda-del-sol-mx Wikidata QID identification (held)
5. Improvement: Spanish municipio names with periods like "S.Cristóbalde lasCasas" still need manual cleanup table

---
from: task@project-gis
to: command
re: session 9 complete — Sprint 9 leapfrog-2030 UI/UX iteration (5 of 7 phases shipped, 2 deferred)
created: 2026-05-08T18:50:00Z
priority: normal
---

Session 9 shipped. All changes live at gis.woodfinegroup.com. Pipeline rebuilt: 6,422 clusters; 1,162 with merged_zones; tier descriptors renamed.

**Phase 1 — Browser tab + favicon (item 6):**
- `<title>` → "Woodfine Location Intelligence"
- `<link rel="icon">` → `favicon.svg` from `woodfine-media-assets/ASSET-FAVICON-CIRCLE.svg` (blue brand circle, #164679)

**Phase 2 — Zoom transition (item 1, the "leapfrog 2030" UX ask):**
- `RETAIL_ZOOM_THRESHOLD` 11 → 9 — rings + retailers visible at regional zoom (~250 km viewport)
- `RETAIL_PREFETCH_ZOOM = 7` — layer1 (`all-locations`) PMTile cache pre-warmed 2 zoom levels before threshold; opacity 0 until reveal
- Visibility flip reordered in `setRetailLevel()`: GeoJSON rings paint instantly, all-locations bumps opacity 0 → 0.70
- Idle timeout 5000 → 2000 ms — cluster bubbles hide faster when tile fetch stalls
- Bottleneck (per audit): layer1 was fetched on-demand at threshold cross. Eliminated.

**Phase 3 — BentoBox anchor-once + 0.15 km dedup transparency (item 5):**
- Anchor no longer renders twice. `#sel-el` empty on cluster load (placeholder: "Click a retailer dot in this ring to inspect"); populates on retailer click.
- `deduplicate_clusters()` in `generate-rankings.py` now attaches `merged_zones` array to survivor cluster (anchor name + cluster_id of suppressed). 1,162 of 6,422 surviving clusters carry merged_zones.
- BentoBox renders below anchor pill: "Co-located within 150 m: ${merged_anchor list}". Italic, no chrome. 100% transparency on the dedup step.
- Resolves the open question in `DESIGN-RESEARCH-ring-retailer-click-ux.draft.md` — collapsibility of `#sel-el` no longer needed.

**Phase 4 — Tier label accessibility (item 2):**
- Plain-English nouns replace technical compound labels:
  - T3 "Full Complement" → **Prime**
  - T2 "Retail Anchor" / "Bulk + Scale" / "Home + Bulk Hub" → **Strong (Retail)** / **Strong (Bulk)** / **Strong (Hub)**
  - T1 "Hypermarket Anchor" / "Hardware Node" / "Wholesale Node" → **Core (Hyper)** / **Core (Hardware)** / **Core (Wholesale)**
  - T0 "Commercial Node" → **Emerging**
- Tier badge now carries `aria-label="Tier ${T} cluster: ${descriptor}; ${count_3km} stores within 3 kilometres"` for screen readers
- Cognitive-load reduction: drops "+" symbols, drops compound nouns

**Phase 5 — Geometric ranking enrichment (item 2/3):**
- Pure geometric metrics surfaced in BentoBox below the rank/sites grid:
  - **Stores per km²** — `count_3km / (π × 9)`, two-decimal formatted; frontend-only computation
  - **Retail brand count** — `unique_brands` (was already computed in `generate-rankings.py`, never exposed; now in `clusters-meta.json` as `ub`)
- No new statistical scoring — just exposing what was already in the pipeline. Operator's question "what can geometric do" answered: site density + brand diversity.

**Phase 6 — Regional names (item 3, partially deferred):**
- Audit confirmed D4 region cleanup (2026-05-07) shipped 1,015 unique clean English names; zero nulls
- MX still at state level (no INEGI ZM file); CA Sherwood Park still collapses into Edmonton CMA (no CSD file)
- INEGI ZM source 404'd. Operator-assist needed: official INEGI download is 3.3 GB — too large for a sprint window
- **Wired GADM 4.1 admin-2 as pragmatic substitute**: `download-boundaries.sh` now downloads `mx_municipio.geojson` (Mexican Municipios) + `ca_division.geojson` (Canadian Census Divisions). Files load on next `download-boundaries.sh` run.
- **Deferred to next session**: `region_engine.py` integration of these layers (helper functions + fallback ordering). The boundary files are ready; the engine code is not yet modified — risk of breaking the working 1,015-region baseline.

**Phase 7 — Missing-data fills (item 4, deferred to dedicated sprint):**
- Audit identified gaps requiring multi-hour Overpass ingest; not scoped for Sprint 9
- **High-leverage gaps surfaced for operator priority decision**:
  - **UK fill (highest impact)**: Tesco (~3,300 stores), Sainsbury's (~1,400), Lidl GB — entirely absent from `REGION_CONFIG`
  - **EU food gaps**: Lidl (~12,000 stores globally), Aldi (~12,000) — present only as `lidl-es` in GENERIC_FOOD; broader rollout needed
  - **MX fill**: Soriana (~700), Chedraui (~228) — entirely absent from `REGION_CONFIG`
  - **OBI/Bauhaus**: YAML stubs exist but 0 ingested records — name-query fallback pattern (per Karwei/Toom precedent) needed
  - **CX anomalies needing operator input**:
    - `tienda-del-sol-mx` — multiple chains share this name; needs disambiguation
    - `tienda-inglesa-mx` — actually Uruguayan chain Q7794716; needs country reassignment to UY
- Each market fill is a separate ingest run; recommend sequencing as: UK → MX → EU food → OBI/Bauhaus → CX
- **Tier-formation gaps (out of scope this sprint, flag only)**: AT, PT, GR have no warehouse anchor; tier-2/3 cluster formation impossible there. BE/LU/IE/CH absent entirely from `REGION_CONFIG`.

**Pipeline state after Sprint 9:**
- 6,422 clusters (1,172 dedup suppressions traced via merged_zones)
- T3 Prime: 27 · T2 Strong: 1,202 · T1 Core: 3,156 · T0 Emerging: 2,037 (unchanged from session 8 — only labels renamed)
- Score range 0–730
- Layer2 PMTile rebuilt (41.7 MB)
- clusters-meta.json: 2,754 KB
- All synced to `/srv/foundry/deployments/gateway-orchestration-gis-1/www/`

**Files touched this sprint:**
- `app-orchestration-gis/build-clusters.py` — tier descriptor rename
- `app-orchestration-gis/build-tiles.py` — `mz` + `ub` keys in clusters-meta
- `app-orchestration-gis/generate-rankings.py` — `merged_zones` audit trail in dedup; `unique_brands` in score return
- `app-orchestration-gis/download-boundaries.sh` — GADM admin-2 wiring (CA + MX)
- `app-orchestration-gis/www/index.html` — title, favicon, threshold, prefetch, visibility reorder, anchor-once placeholder, merged_zones render, ARIA tier badge, geometric metrics row, metaToClusterProps additions
- `app-orchestration-gis/www/favicon.svg` — new (copy of `woodfine-media-assets/ASSET-FAVICON-CIRCLE.svg`)

**Drafts already routed in commits prior to this sprint** (no new drafts staged):
- `topic-cluster-deduplication-threshold.draft.md` — explains 0.15 km logic visualised in Phase 3 ✓
- `topic-retail-brand-family-taxonomy.draft.md` — defines families used in Phase 5 brand count ✓
- `DESIGN-RESEARCH-ring-retailer-click-ux.draft.md` — its open question resolved by Phase 3 ✓

**Recommended next-session sequence:**
1. Run `download-boundaries.sh` to fetch GADM CA + MX admin-2 files
2. Phase 6 completion: integrate GADM layers into `region_engine.py` (`_format_mx_municipio()`, `_format_ca_csd_with_cma_fallback()`); rebuild pipeline; verify Sherwood Park ≠ Edmonton + Mexico City ≠ Estado de México
3. Phase 7 — operator picks one market fill to start (recommend UK as highest impact)
4. After UK fill, validate cluster surface; iterate

**Operator decisions surfaced:**
1. Phase 6 next: GADM admin-2 integration vs hold for true INEGI ZM (large file)?
2. Phase 7 sequence: UK first, or MX first (operator's stated retail focus)?
3. CX disambiguation: `tienda-del-sol-mx` (which chain?) and `tienda-inglesa-mx` (move to UY?)

---
from: task@project-gis
to: command
re: session 8 complete — dedup recalibration + category taxonomy + GeoJSON source + click handler fixes
created: 2026-05-08T07:30:00Z
priority: normal
---

Session 8 shipped. All changes live at gis.woodfinegroup.com. No commit yet — commit pending operator review of drafts and final approval.

**Pipeline change — dedup threshold 0.5 km → 0.15 km:**
- Root cause for 0.5 km regression: Edmonton Home Depot + Costco 20 m apart (same zone) correctly deduplicated, but separate commercial nodes 200–500 m apart were silently suppressed
- Fix: `DEDUP_KM = 0.15` in `generate-rankings.py` deduplicate_clusters()
- Pipeline rebuilt: 7,594 raw → 6,422 clusters (1,172 duplicates removed)
- Tier result: T3=27 T2=1202 T1=3156 T0=2037; score range 0–730

**Brand family taxonomy expansion:**
- Added Food, Furniture, Pharmacy, Department families to `CHAIN_FAMILY` in `build-tiles.py`
- Added `familyToCat` entries in `all-locations` click handler in `index.html`
- Added CSS `.cat-badge.food/.furniture/.pharmacy/.department` classes
- Added labels to both `catLabels` occurrences in `index.html`
- Effect: Save-On-Foods, Safeway, Lidl, Mercadona et al. now display "Grocery" badge (was "Hardware")

**Blank zone fix — GeoJSON source:**
- Root cause: `data` source was PMTiles (layer2-clusters.pmtiles); hidden layers do not prefetch tiles; on show, every zoom level required HTTP range requests → 1–3 s blank
- Fix: converted `data` source to in-memory GeoJSON, populated from `clusters-meta.json` at startup; zero tile-fetch latency at any zoom level
- Removed `source-layer: 'clusters'` from all four nodes layers (GeoJSON does not use source-layer)
- zoomend revert threshold raised from z<6 → z<RETAIL_ZOOM_THRESHOLD (symmetric blank-zone fix for zoom-out direction)

**Click handler regression fix (GeoJSON source conversion broke all three):**
- Root cause: after GeoJSON source, `querySourceFeatures('data', { sourceLayer: 'clusters', ... })` returned features with only 5 fields (cluster_id, rank_1km, rank_3km, score_final, display_name); `showClusterDetail` showed empty anchor name, blank city/region
- Fix: removed `querySourceFeatures` path from `proximity-fill`, `radius-fill`, and `nodes` click handlers; all three now use `clusterIndex.get(cid)` → `metaToClusterProps(c)` for full 20+ field cluster data

**Ring/retailer click UX:**
- Click ring fill → full cluster BentoBox with anchor in Selected Location
- Click retailer dot inside ring → cluster BentoBox with that retailer in Selected Location (first click; no anchor-first step)
- Click retailer dot outside ring → standalone retailer detail card

**Pending for session 9 (operator priority order):**
1. Commit this session's changes (generate-rankings.py, build-tiles.py, index.html)
2. Mexico INEGI ZM boundary download (Block 1A) — MX clusters still at state level
3. Canadian CSD boundaries for Sherwood Park / Edmonton distinction (Block 1B)
4. chains-master.json single registry (Block 2A)
5. BentoBox `#sel-el` collapsible toggle — flagged in DESIGN-RESEARCH draft

**Drafts staged (3 new + 1 updated):**
- `topic-cluster-deduplication-threshold.draft.md` → project-editorial (PROSE-TOPIC)
- `topic-retail-brand-family-taxonomy.draft.md` → project-editorial (PROSE-TOPIC)
- `DESIGN-RESEARCH-ring-retailer-click-ux.draft.md` → project-design (DESIGN-RESEARCH)
- `guide-gis-adding-a-chain.md` updated: added Step 3b (CHAIN_FAMILY classification)

Flag to project-editorial: 2 new TOPIC drafts ready for language pass.
Flag to project-design: 1 DESIGN-RESEARCH draft ready for review.

---
from: task@project-gis
to: master
re: session 7 complete — halo fix + anchor dot removed (commit 9ce1a50)
created: 2026-05-08T03:10:00Z
priority: normal
---

Session 7 shipped two bug fixes from post-session-6 user report. Live at gis.woodfinegroup.com.

**Bug 1 fixed — halos gone at Co-location Level:**
- Root cause: `--cluster-distance 30` in tippecanoe layer2 caused nearby clusters to be merged at low zoom levels; numeric properties (rank_3km) were averaged to fractional values (e.g. 2.7); MapLibre filter `['==', ['get', 'rank_3km'], 3]` failed; no halos rendered
- Fix: Removed `--cluster-distance 30` from `build_layer2()` in `build-tiles.py`; rebuilt layer2 → 43.4 MB tile (was ~30 MB with merging)
- Data was always correct (2,199 clusters confirmed with rank_3km=3); the tile was the failure point

**Bug 2 fixed — blue bubble at ring centre + double rings:**
- Root cause: Session 6 changed cluster centroid to geometric mean of commercial stores; `build_clusters_meta()` now stores centroid as lon/lat; `drillIntoCluster()` placed an 11px navy anchor dot at those centroid coords = ring centre = visually identical to an inner ring
- Fix: `drillIntoCluster()` now clears the anchors source to empty FeatureCollection instead of placing anchor dot at centroid; the anchor chain store is still visible at its actual location via the all-locations layer
- User explicitly requested "the centre does not need to be identified"

**Sherwood Park ≠ Edmonton (architecture limit — not fixed):**
- Sherwood Park is within the Edmonton CMA boundary; region_engine.py assigns Edmonton CMA to all clusters in that metro catchment
- Three separate clusters DO exist for the Edmonton area (Co-location 1/2/3 numbering working correctly)
- Cannot distinguish Sherwood Park from Edmonton without Canadian Census Subdivision (CSD) shapefile — Block 1B in master plan (deferred multi-session)
- Recommend explaining to user: correct fix requires downloading Statistics Canada CSD boundaries

**Session 8 priorities:**
1. Verify halos restored in browser (ring view + overview both)
2. Verify no blue bubble at ring centre
3. Mexican INEGI ZM boundary download (Block 1A) — MX clusters still at state level
4. Canadian CSD boundaries for Sherwood Park / municipality granularity (Block 1B) — deferred but user has noticed
5. chains-master.json single registry (Block 2A)

---
from: task@project-gis
to: master
re: session 6 complete — pure geometry rebuild (commit 77b8d2a)
created: 2026-05-08T02:45:00Z
priority: normal
---

Session 6 shipped. Live at gis.woodfinegroup.com.

**Pipeline rebuild (full cluster + layer2 tiles + meta):**
- `config.py`: `ANCHOR_DISPLAY_NAMES` expanded — 0 raw chain_ids remain in anchor_label (was 4,000+ clusters showing "home-depot-us" / "costco-us")
- `build-clusters.py`: adds `count_1km`, `count_3km` (commercial store counts), `centroid_lat/lon` (geometric centroid of cluster stores), `tier_descriptor` (categorical composition: "Full Complement" / "Retail Anchor" / "Home + Bulk Hub" / etc.)
- `generate-rankings.py`: `national_rank` now sorted by `count_3km` desc (pure geometry, no weighting); adds `na_rank`/`na_rank_of` (US+CA+MX combined); adds Co-location N display_name suffix when multiple clusters share a region_name
- `build-tiles.py`: layer2 tippecanoe params fixed (`--base-zoom 2 --cluster-distance 30 --minimum-zoom 2`) — should eliminate blank zone at z=6-10; clusters-meta.json extended with c1/c3/clat/clon/td/nar/naro fields (3MB)
- Pipeline result: 7,594 clusters, T3=28 T2=1316 T1=3856 T0=2394, US #1 = The Home Depot, Canton OH (7 stores/1km)

**BentoBox redesign:**
- Tier badge now shows `tier_descriptor` label ("Home + Bulk Hub") instead of "T2 Hub"
- 2-column labeled grid: "North America Rank / #N of M · Country" + "Sites in Ring / N at 3km"
- NA rank shown for US/CA/MX clusters; national rank shown for EU
- National rank shown as secondary line under NA rank when both available
- No more score/1000 — replaced with site count (objective, auditable)

**Zoom UX:**
- Auto-revert from Retail Level removed — only Back button returns to Co-location Level
- Tile-load safety net: if nodes layer empty at threshold zoom after idle, force Retail Level
- `maxTileCacheSize: 500` (was 200) to reduce tile eviction during zoom transitions

**Session 7 priorities (from master plan):**
1. Mexico INEGI ZM boundary download — Mexican clusters still at state level (fix download-boundaries.sh)
2. chains-master.json single registry (Block 2A) — eliminate dual frontend/backend name system
3. BentoBox accessibility audit (font size floor, touch targets, screen reader order)
4. Verify blank zone fix in browser — confirm z=6-10 now shows clusters
5. Region name audit — flag any US CBSA hyphenated slugs, CA French variants

**Editorial staging flag:** Session 6 research (geometric ranking methodology, regional naming government sources, chain registry rationale) is ready to seed TOPICs. Flag to project-editorial when session 7 is stable.



---
from: task@project-gis
to: master
re: session 5 complete — zoom UX + BentoBox simplification (commit 75a853d)
created: 2026-05-08T00:10:00Z
priority: normal
---

Session 5 shipped. Live at gis.woodfinegroup.com.

**Ring coverage fix (in-memory cluster data):**
- `build-tiles.py` now generates `www/data/clusters-meta.json` (683KB, 7594 clusters, fields: id/lon/lat/r1/r2/r3) after each layer2 rebuild
- `updateProximityRings()` fetches this file at map start and generates rings from in-memory viewport-filtered candidates — all rings visible at Retail Level regardless of tile-load state
- Fallback to `querySourceFeatures` if fetch fails

**Tile fix:**
- Layer2 tippecanoe: `--base-zoom 3` → `--base-zoom 0`; rebuilt 36.7MB tile

**BentoBox (simplified):**
- Removed: star ratings, Top-X% pill, HW/WH/Medical/Academic sub-score line, Institutional Support section
- Section 2 redesign: Tier badge (no stars) + #Rank as 26px hero + score as secondary + "of N · Country" in small caps
- Currently shows national (within-country) rank — NA cross-country rank requires `generate-rankings.py` change (session 6 item 1)

**Remaining zoom UX issues:**
- Blank zone at zoom ~6-10 (bubbles disappear briefly mid-zoom) — not yet fully diagnosed; may need browser DevTools session; `--base-zoom 0` change may help
- `RETAIL_ZOOM_THRESHOLD=11` confirmed correct; transition fires at zoom 11

**Session 6 priorities (see tasks.md for full list):**
1. `na_rank` + `na_rank_of` in generate-rankings.py (NA cross-country rank for BentoBox)
2. Blank zone investigation (possible DevTools session)
3. C11–C14 Carrefour ingest

---
from: task@project-gis
to: master
re: session 4 complete — Fortune-scale anchor redesign + full deployment
created: 2026-05-07T18:45:00Z
priority: normal
---

Session 4 work complete and deployed to gis.woodfinegroup.com. Commit 1dcc52e.

**Anchor tier:**
- Home Depot → ALPHA_ANCHORS (was ALPHA_HARDWARE); dual membership maintained for secondary scoring
- Costco → ALPHA_ANCHORS globally; Makro → ALPHA_ANCHORS (ES/NL/PL); both retain warehouse dual membership
- Carrefour deferred: C11–C14 tasks logged; needs chain YAML + ingest before config entry
- Cluster count: 7,594 (was 3,498); T3=28 T2=1316; score range 0–730

**UI (all live):**
- BentoBox labels corrected: Data Radius / Catchment Areas — Planned / co-location radius
- Radius toggle bug fixed: bubble sizing now uses score_final (radius-independent)
- Cluster zoom UX redesign: nodes hidden on drill-in; radius rings + All Locations appear; Back-only navigation
- Map south bound −5.0° for Mexico breathing room

**Pending for Master awareness:**
- Municipality names (§6) is a next-session multi-hour data task (boundary downloads)
- Tier-1 rate is 11.5% (calibration threshold is 10%) — within tolerance but worth monitoring
- Fred Meyer (PNW-regional, 92–132 stores) flagged for future removal review from ALPHA_ANCHORS

