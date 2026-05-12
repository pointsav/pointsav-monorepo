# project-gis — Outstanding Tasks

Last updated: 2026-05-07

---

## Group A — UI Fixes

### A1: Fix `all-locations` layer click handler [x] DONE
Dynamic cat badge from brand_family/category_id; address fields passed — committed f8e9c4c.

### A2: All-locations dots invisible at overview zoom [x] DONE — committed 56dfcff
- `build-tiles.py`: layer-1 `--minimum-zoom 4` → `2`
- `index.html`: `all-locations` circle-radius static 3px → zoom interpolation (2→1.5, 6→2.5, 10→3.5, 14→5)
- Layer-1 PMTile rebuilt; min_zoom confirmed 2 in header.

### A3: Residual circles after clicking off a cluster [x] DONE — committed 9126bd1
`e.defaultPrevented` guard was broken (MapLibre does not honour `stopPropagation` across generic/layer handlers).
Fixed with `queryRenderedFeatures` guard: generic click only calls `flyToOverview()` when no feature hit.

### A4: Neighbouring clusters not visible after drilling in [x] DONE — committed 56dfcff
`fitBounds` `maxZoom: adaptiveMax` → `Math.min(adaptiveMax, 12)`.
Single-secondary `flyTo` zoom 14 → 12. Dense-city clusters now keep neighbours in viewport.

---

## Group B — Pipeline / Schema Updates

### B1: Overture taxonomy migration [x] DONE
`categories.primary` → `taxonomy.primary` in ingest-overture.py — committed f8e9c4c.

### B2: `brand_wikidata` flat field [x] DONE
Added to ingest-osm.py + ingest-overture.py — committed f8e9c4c.

### B3: Address completeness — Overture Addresses spatial join [ ]
- For POIs with null `addr:*`, spatial-join against Overture Addresses theme (≤15 m)
- Back-fill `full_address`

### B4: Fix null `region_name` clusters [x] DONE 2026-05-07
Was 5 null regions. Fixed via 3 changes to region_engine.py:
1. `if hits:` numpy bug → `if len(hits) > 0:`
2. Nearest-polygon fallback (0.15° / ~15km) for coastal/fjord edge cases
3. "Nordics" composite ISO code → routes to EU NUTS-3
Result: 0 null regions in 3,498 clusters.

### B5: Re-run `download-boundaries.sh` with ogr2ogr [x] DONE (previous session)
Boundary files present at /srv/foundry/deployments/cluster-totebox-personnel-1/boundaries/.

---

## Group C — Chain Coverage

### C1: Castorama FR [x] DONE (data present; promoted to ALPHA)
118 records already ingested; promoted from GENERIC to ALPHA_HARDWARE + CHAIN_FAMILY updated.

### C2: Gamma NL [x] DONE
215 records ingested (QID Q1424817 returned 0; name_query="Gamma" fallback used). 205 cleansed.

### C3: Karwei NL [x] DONE
137 records ingested (name_query="Karwei"). 126 cleansed.

### C4: Toom Baumarkt DE [x] DONE
357 records ingested (QID Q2327717 returned 0; name_query="toom" partial fallback). Added to GENERIC_HARDWARE EU + REGION_CONFIG DE.

### C5: BJ's Wholesale US [ ] — NOTE: bjs-wholesale-us already has 232 records / 250 expected (OK)

### C6: Bricocenter IT [x] DONE
51 records ingested (QID Q3001985 returned 0; name_query="BricoCenter" exact fallback). Added to REGION_CONFIG IT.

### C7: Silvan DK [x] DONE
52 records ingested (QID Q7512977 returned 0; name_query="Silvan" partial fallback). Added to REGION_CONFIG NORDICS.

### C8: Praktiker GR [x] DONE
19 records ingested (QID Q1289591 returned 0; name_query="Praktiker" exact fallback). Added to REGION_CONFIG GR.

### C9: BYKO IS [x] DONE
12 records ingested (name_query="Byko" partial fallback). Added to REGION_CONFIG NORDICS.

### C10: Hagebaumarkt DE [x] DONE
394 records ingested (QID Q875399 returned 0; name_query="hagebaumarkt" partial fallback). Added to GENERIC_HARDWARE EU + REGION_CONFIG DE.

### CX: Zero-count chains — status after 2026-05-07 ingest pass
| Chain | Expected | Raw | Cleansed | Status |
|---|---|---|---|---|
| `husasmidjan-is` | 20 | 18 | 13 | [x] ingested |
| `imerco-dk` | 100 | 148 | 137 | [x] ingested (1.5x — possibly correct) |
| `lowes-ca` | 64 | 354 | 349 | [x] ingested (5.5x — bbox contamination) |
| `maxi-ica-se` | 94 | 50 | 43 | [x] ingested (partial; try wider query) |
| `peavey-mart-ca` | 190 | 26 | 26 | [x] ingested (0.1x — poor OSM coverage) |
| `sklavenitis-gr` | 300 | 418 | 408 | [x] ingested (1.4x — OK) |
| `tiendas-3b-mx` | 1700 | 151 | 151 | [x] ingested (QID Q113217378 confirmed; OSM coverage ~9%) |
| `topaz-pl` | 200 | 82 | 82 | [x] ingested (QID Q11837058 confirmed; OSM coverage ~41%) |
| `tienda-del-sol-mx` | 50 | 0 | 0 | [ ] NEEDS OPERATOR INPUT — multiple chains use this name |
| `tienda-inglesa-mx` | 35 | 0 | 0 | [ ] ANOMALY — Uruguayan chain (Q7794716); 0 records in MX bbox; needs country reassignment |

---

## Group D — Data Quality (NEW: cross-check revealed)

### D0: Sub-location inflation — OVER-count chains [ ] REBUILD NEEDED
After re-ingest (fix format_exclude_names + cluster-entities.py rewrite), run full pipeline rebuild.
Root causes:
1. Sub-location OSM elements (same brand_wikidata, different name) → fixed by cluster-entities.py v2
2. Bbox overlap (canada bbox captures US border stores) → requires Overpass `addr:country` filter
3. YAML format_exclude_names gaps → fixed for costco-ca (2026-05-06)

Worst offenders (raw count / expected):
| Chain | Expected | Raw | Root cause |
|---|---|---|---|
| `leclerc-es` | 18 | 220 | store_count_approx likely wrong (E.Leclerc Spain >> 18); verify |
| `carrefour-hypermarket-it` | 41 | 215 | Arabic-named records (Tunisia bbox overlap); tighten Italy bbox |
| `alcampo-es` | 80 | 323 | sub-location + name variants |
| `home-depot-ca` | 182 | 719 | US stores captured in Canada bbox (addr:country missing) |
| `walmart-ca` | 409 | 1417 | US stores captured in Canada bbox |
| `walmart-mx` | 250 | 1072 | sub-location + bbox |
| `ikea-*` | varies | 2-6x | IKEA Restaurant, Café, Småland, Planning Studio |
| `costco-ca` | 107 | 258 | US bbox contamination (~146 US stores) + cross-QID dedup absorbs costco-us records |

### D1: Parent-child sub-location model [x] DONE — pipeline rebuilt 2026-05-07
cluster-entities.py v2 ran: 36,091 raw → 34,582 parent records, 1,509 sub-entities.
build-clusters.py: 3,486 clusters. All PMTiles live at gis.woodfinegroup.com.
Second rebuild 2026-05-07: 37,209 raw → 35,555 parent, 1,654 sub. 3,498 clusters. T3=10 T2=290 T1=1546 T0=1652.

### D4: Region name cleanup [x] DONE 2026-05-07
region_engine.py: comprehensive _REGION_CLEAN override dict (Greek → English, Finnish, Danish,
Norwegian, Dutch, German suffix strip, Belgian bilingual, Icelandic, Polish, Czech, etc.).
_format_ca_cma: removed "CMA" suffix. US double-hyphen CBSA names simplified.
Result: 1,015 unique region names — all clean English, no non-Latin script, no CMA/KreisfreieStadt.

### D2: Bbox overlap ingest fix [x] CODE DONE (partial)
Belt-and-suspenders post-filter added to ingest-osm.py (drops records with wrong
explicit iso_country_code). However: OSM stores without addr:country tag get the
chain's country_code as fallback in element_to_record() — so bbox-contaminated US
stores still pass the filter.
Root cause: `"iso_country_code": iso_country or chain.get("country_code")` line 178.
Full fix requires polygon-based country containment check (B4/B5 territory).
Affected chains: home-depot-ca (693 vs 182), lowes-ca (354 vs 64), walmart-ca (315 vs 409
was worse before D2).

### D5: Cluster radius gap — Sherwood Park Costco [ ] KNOWN LIMITATION
Costco at (53.5467, -113.3175) exists in costco-ca.jsonl and in layer1-locations (All Locations dot present).
Absent from Sherwood Park cluster bento because anchor (Walmart at 53.5111) is 3.96 km away — just outside
`max_r = max(RADII_KM) = 3.0 km`. No second cluster anchors the Emerald Hills retail node.
Options: (a) raise secondary radius 3→5 km (risk: cross-cluster pollution in dense cities);
         (b) document as 3 km methodology limit; Costco visible via All Locations toggle.
Recommendation: leave 3 km; note in guide.

### D3: Bbox contamination — polygon country filter [ ] DEFERRED → B4/B5
When B5 (boundary download) + B4 (region fix) land, add a spatial containment
filter in ingest-osm.py using the country boundary polygons. Until then, CA/MX
chains will have inflated counts due to US border-store capture.

---

## Group E — Infrastructure

### E1: Stage 6 preparation [x] DONE
Committed f8e9c4c (v0.1.118) + ba5fe38 (v0.1.119) + pending (v0.1.120+).

### E2: DataGraph entity writes [ ] HOLD
Coordinate schema with Master; defer until D1 rebuild is clean.

### E3: Full pipeline rebuild [x] DONE 2026-05-07 (×2)
First rebuild: 3,486 clusters; T3=10 T2=289 T1=1543 T0=1644; score range 0–697.
Second rebuild (with 8 new chains + region fix): 3,498 clusters; T3=10 T2=290 T1=1546 T0=1652; score range 0–697.
PMTiles: layer1 398MB (229,054 features), layer2 22.5MB, layer3 100.6MB. All live at gis.woodfinegroup.com.

---

## Group F — Editorial

### F1: `topic-poi-data-schema.draft.md` [x] DONE
Staged + dispatched to project-editorial ba5fe38.

### F2: `guide-gis-adding-a-chain.md` updated [x] DONE
Staged + dispatched to project-editorial ba5fe38.

### F3: `topic-co-location-methodology.md` V2 tier update [x] DONE
Dispatched to project-editorial earlier session.

---

## Group G — Deferred

- **G1:** Rust `service-ingest` crate — hold
- **G2:** OD Study layer — hold

---

## Tools added this session

- `check-chain-counts.py` — persistent cross-check; run any time after ingest
- `cluster-entities.py` v2 — improved D1 parent-child dedup; run before full pipeline

## Session 4 close — 2026-05-07

### Session 4 completed work (commit 1dcc52e)

**Anchor tier redesign — Fortune-scale criterion applied:**
- Home Depot elevated: ALPHA_HARDWARE → ALPHA_ANCHORS (dual membership in hardware for secondary scoring)
- Costco elevated globally: ALPHA_WAREHOUSE → ALPHA_ANCHORS (dual membership in warehouse)
- Makro elevated globally (ES/NL/PL): ALPHA_WAREHOUSE → ALPHA_ANCHORS (dual membership in warehouse)
- Carrefour: deferred — data ingest required (C11–C14 below)
- Fred Meyer: kept for now; flag for removal review (92–132 stores, PNW-regional only)
- Pipeline rebuilt: 7,594 clusters; T3=28 T2=1316; score 0–730; tiles live at gis.woodfinegroup.com

**UI fixes (index.html):**
- BentoBox: "All Catchment Areas" → "Data Radius"; "OD Study" → "Catchment Areas"; "catchment radius" → "co-location radius"
- Radius toggle: bubble sizing decoupled from radius; `score_final` used (0–730 range, stable); `setPaintProperty` removed from `setRadius()`
- Cluster zoom UX: `drillIntoCluster()` hides nodes layer entirely, shows all radius rings + All Locations; `radius-fill` click handler resolves cluster via `querySourceFeatures`; `all-locations` click delegates to enclosing radius polygon; Back-only navigation via `flyToOverview()`
- Map south bound: −5.0° (was +5.0°) for Mexico breathing room

**Pipeline tier-1 rate note:** 11.5% Tier-1 (just above 10% calibration threshold) — acceptable given Fortune-scale elevation adding new cluster initiators.

### C11–C14: Carrefour ingest (new — deferred to next session)
Carrefour approved for re-addition; data ingest required before config.py entry:
- `carrefour-hypermarket-fr` — France
- `carrefour-hypermarket-es` — Spain
- `carrefour-hypermarket-pl` — Poland
- `carrefour-hypermarket-pt` — Portugal

Create YAML chain files → ingest → verify counts → add to ALPHA_ANCHORS + REGION_CONFIG → rebuild.

### Session 5 close — 2026-05-08 (commit 75a853d)

**Zoom + ring fixes:**
- build-tiles.py: `--base-zoom 3` → `--base-zoom 0` for layer2; rebuilt layer2-clusters.pmtiles
- build_clusters_meta(): generates `www/data/clusters-meta.json` (683KB, 7594 entries: id/lon/lat/r1/r2/r3)
- updateProximityRings(): loads cluster centroids into memory at map startup; ring generation uses
  in-memory viewport-padded lookup — guaranteed all rings visible regardless of tile-load state
- RETAIL_ZOOM_THRESHOLD: 11 (confirmed, reverted from 14 in earlier session)

**BentoBox simplification:**
- Removed: stars from tier badge, Top-X% pill, HW/WH/Medical/Academic sub-score line, Institutional Support section
- Redesigned section 2: Tier badge → #Rank (26px hero) + Score (secondary) + "of N · Country" label
- Currently shows national (within-country) rank; NA cross-country rank requires pipeline change (see below)

**Live at:** gis.woodfinegroup.com — commit 75a853d

### Next execution sequence (session 6+)

1. **NA rank**: Add `na_rank` + `na_rank_of` fields to `generate-rankings.py` (rank among US+CA+MX clusters by score_final); update BentoBox to show as primary rank labelled "North America"
2. **Blank zone investigation**: At zoom ~6-10, bubbles may disappear briefly — may need browser DevTools session to diagnose; `--base-zoom 0` change may help; secondary theory: tile-load lag
3. **C11–C14**: Carrefour ingest (France/Spain/Poland/Portugal) → add to anchor tier → rebuild
4. **Municipality names (§6)**: US Census TIGER Places + Canadian CSDs + EU LAU-2/OSM admin-8 boundary downloads; update `region_engine.py` to prioritise municipality over CBSA/CMA (multi-session data task)
5. **Fred Meyer review**: Verify cluster impact of removing `fred-meyer-us` from ALPHA_ANCHORS
6. **D3**: Polygon country filter (bbox contamination for home-depot-ca, lowes-ca, walmart-ca, costco-ca)
7. **CX anomalies**: tienda-del-sol-mx, tienda-inglesa-mx (country wrong — UY not MX)
8. **B3**: Address completeness via Overture Addresses spatial join (lower priority)
9. **E2**: DataGraph entity writes (hold, needs operator input)

Last updated: 2026-05-08 (session 9)

## check-chain-counts.py results (2026-05-07)
Summary: 63 OK | 41 OVER | 7 UNDER | 5 EMPTY
Notable: home-depot-ca 693 vs 182 (bbox contamination), lowes-ca 354 vs 64 (same),
peavey-mart-ca 26 vs 190 (poor OSM coverage), maxi-ica-se 43 vs 94 (partial coverage).

## Sprint 9 close — 2026-05-08

5 of 7 plan phases shipped to gis.woodfinegroup.com; 2 deferred with forward path.

### Shipped
- **Phase 1** — browser tab title `Woodfine Location Intelligence` + `favicon.svg` (Woodfine blue circle)
- **Phase 2** — zoom transition: `RETAIL_ZOOM_THRESHOLD` 11→9; `RETAIL_PREFETCH_ZOOM=7` warms layer1 PMTiles 2 levels early; visibility flip reorder (rings instant, all-locations bumps opacity 0→0.70); idle timeout 5000→2000ms
- **Phase 3** — BentoBox: anchor-once (`#sel-el` empty until retailer click); 0.15 km dedup audit trail (`merged_zones` array on survivor); 1,162/6,422 clusters carry merged_zones
- **Phase 4** — tier label rename: Prime / Strong (Retail|Bulk|Hub) / Core (Hyper|Hardware|Wholesale) / Emerging; `aria-label` on tier badge
- **Phase 5** — geometric metrics surfaced: stores/km² + retail brand count via `clusters-meta.json` `ub` field

### Deferred (next session)
- **Phase 6 — Regional names**: GADM 4.1 admin-2 download wired in `download-boundaries.sh` (CA Census Division + MX Municipio); `region_engine.py` integration not yet done — boundary files load on next script run, engine ignores them until helper functions added
- **Phase 7 — Missing data**: 5 high-leverage gaps surfaced in outbox (UK Tesco/Sainsbury's/Lidl-GB; EU Lidl/Aldi; MX Soriana/Chedraui; OBI/Bauhaus name-query fix; CX disambiguation). Each requires Overpass ingest. Operator priority decision needed before sequencing.

### Pipeline state
- 6,422 clusters; T3 Prime: 27 · T2 Strong: 1,202 · T1 Core: 3,156 · T0 Emerging: 2,037
- Score range 0–730
- `clusters-meta.json` 2,754 KB; `layer2-clusters.pmtiles` 41.7 MB
- Live at gis.woodfinegroup.com

### Open operator decisions
1. Phase 6 next: GADM admin-2 integration vs hold for true INEGI ZM (3.3 GB)?
2. Phase 7 sequence: UK first, MX first, or different order?
3. CX disambiguation: `tienda-del-sol-mx` (which chain?) and `tienda-inglesa-mx` (reassign to UY?)

## Sprint 10 close — 2026-05-08

All operator-listed outstanding work either shipped or surfaced for follow-up. Sprint 9 commit pushed.

### Shipped
- **Phase A** — pushed Sprint 9 commit `7e92013` to vault (`origin/main`)
- **Phase B** — GADM 4.1 boundaries downloaded: ca_csd.geojson (admin-3, 5,581 features), mx_municipio.geojson (admin-2, 2,457 features)
- **Phase C** — region_engine.py integration: new helpers + CamelCase/Spanish-preposition/period splitters; CA distinct regions 35→245, MX 32→104; "Strathcona County, Edmonton" works
- **Phase D** — UK food fill: tesco-uk (784), sainsburys-uk (672), lidl-uk (1,272); 54 GB clusters live
- **Phase E** — verified Soriana (489) + Chedraui (249) already ingested + classified
- **Phase F** — EU food fill: 9 chains (Lidl DE/FR/NL/AT/PT + Aldi DE/UK/NL/PL); 12,289 records added
- **Phase G** — verified OBI/Bauhaus all 5 chains have name_query + records (430/84/104/243/17)
- **Phase H** — tienda-inglesa-mx → tienda-inglesa-uy (UY bbox added; 20 records ingested); tienda-del-sol-mx HELD per operator

### Deferred
- **Phase J (D3) polygon country filter**: bbox contamination fix (home-depot-ca 693 → 182). Multi-step risk; dedicated sprint.

### Pipeline state
- 48,306 cleansed records (+12,751 from Sprint 10 fills)
- 6,422 clusters; T3=27, T2=1,202, T1=3,156, T0=2,037; score 0–730
- Layer1 PMTiles 403.5 MB, Layer2 41.7 MB

### Open operator follow-ups
1. Phase J (D3) bbox-contamination polygon filter
2. Aldi-NL OSM coverage gap (3 records vs ~480 expected)
3. Soriana/Chedraui promotion to ALPHA_HYPERMARKET (decision pending)
4. tienda-del-sol-mx Wikidata QID identification

## Sprint 11 close — 2026-05-08

Finished Sprint 10's deferred Phase J + authored close-out artifact drafts.

### Shipped
- **Phase J** — D3 polygon country filter in `ingest-osm.py`. Re-ingested home-depot-ca (693→177), lowes-ca (355→1), walmart-ca (318→253), costco-ca (258→109). CA cluster count 712→483 (-229 phantom border clusters). Score / tier distribution unchanged.
- **Phase A** — 3 DESIGN-RESEARCH drafts: tier-naming-accessibility, zoom-prefetch-pattern, bento-merged-zones-disclosure
- **Phase B** — 4 TOPIC drafts: regional-name-resolution-architecture, uk-eu-food-retail-coverage, co-location-tier-nomenclature, gis-as-bim-substrate
- **Phase C** — 2 new GUIDEs (adding-a-country, pipeline-rebuild) + 1 update (adding-a-chain Sprint 9-11 appendix)
- **Phase D** — TEXT release note (uk-eu-coverage-release)
- **Phase E** — Aldi-NL fix: `wikidata_id: ~` + `name_query_partial: true` → 3→490 records

### Pipeline state
- 47,860 cleansed records
- 7,041 raw clusters / 6,422 deduplicated
- CA 483 / US 4,947 / MX 410 / GB 53 / DE 48 / FR 37 / NL (Aldi now visible)

### Open operator follow-ups (after Sprint 11)
1. Soriana/Chedraui promotion to ALPHA_HYPERMARKET (decision pending)
2. tienda-del-sol-mx Wikidata QID identification (held)
3. EU food expansion (Carrefour-FR, Auchan-FR, Mercadona-ES home markets)
4. Country expansion (BE, LU, IE, CH absent from REGION_CONFIG)
5. INEGI ZM Mexican shapefile (3.3 GB) integration when bandwidth permits

## Sprint 12 close — 2026-05-09

Six scoped items shipped. Operator answered all four decision-blocking items from the consolidated outstanding-items master list.

### Shipped
- **B1** — Search regression fixed: data GeoJSON source now carries `hw_list` + `wh_list` arrays for filter matching; ring stroke `#EAB308` (brand-accent) + width 3.5 for visibility
- **B2** — Mercadona-ES verified in-place (1,603 records, Food classification correct)
- **B3** — Spanish municipio name polish: zero residual issues (Sprint 11 cleanup complete)
- **A1** — Soriana promoted to ALPHA_HYPERMARKET; 250 new MX clusters; tier dist shifted
- **A2** — `tienda-del-sol-mx` dropped from registry
- **A3** — Carrefour-FR ingested (509 records); flagged sub-format coverage gap for next sprint

### Pipeline state
- 48,810 cleansed records (+509 Carrefour-FR; +250 Soriana)
- 7,292 raw clusters / 6,422 deduplicated
- MX 657 (was 410, +247 from Soriana) · FR 37 · GB 53 · CA 483 · US 4,947
- T3 Prime 28 · T2 Strong 1,302 · T1 Core 3,322 · T0 Emerging 2,022
- Score range 0–730

### Open follow-ups (carry-forward)
1. Carrefour-FR sub-format coverage gap (10% — needs name_query refinement)
2. C1 Overture Addresses spatial join (multi-session)
3. C2 Fred Meyer ALPHA removal review
4. D1–D4 held items (DataGraph E2, Sherwood Park 3km, Rust ingest, OD Study)
5. E1–E4 long-tail (IPEDS, draft open questions, PRODUCT_VISION, blank-zone diag)

## Sprint 13 follow-on — 2026-05-12

Two global pipeline bugs identified and fixed. Triggered by operator-supplied Google Maps URLs for missing Sherwood Park Walmart Supercentre and Strathcona Community Hospital.

### Shipped — commit `5f96ca0` (Peter Woodfine)

- **Bug 1 — `"supercentre"` in SKIP_NAME_SUBSTRINGS** (`ingest-osm.py:129`): Canadian-spelling filter was silently dropping "Walmart Supercentre" records at ingest. US spelling "supercenter" was not matched. Fix: removed `"supercentre"`. walmart-ca re-ingested: 253 → 453 records. New cluster: "Strathcona County, Edmonton — Co-location 4" at (53.5689, -113.2790), anchored by walmart-ca with Canadian Tire as ALPHA_HW secondary (0.73 km). Affects all of Canada — every "Walmart Supercentre" OSM element now ingests correctly.

- **Bug 2 — civic OSM data absent from tiles** (`build-tiles.py`, `build_layer1()`): `service-places/cleansed-civic-osm.jsonl` (60,756 records: hospitals + universities across US, CA, FR, MX, DE, IT, ES, GB, PL, Nordics) was never read by the tile builder. Only Overture service-fs data was used, which had essentially zero valid Canadian records. Fix: added third read block for civic OSM path. All 60,756 records now flow into layer1-locations.pmtiles. Strathcona Community Hospital (53.5682, -113.2767, Sherwood Park AB) now renders.

### Pipeline state after Sprint 13 follow-on
- 48,468 cleansed business records (cluster-entities dedup applied)
- 6,815 clusters (was 6,422); T3 Apex: 28 · T2 Hub: 1,309 · T1 Valid: 3,374 · T0 Border: 2,104
- Score range 0–730
- layer1-locations.pmtiles: 500.6 MB (was ~400 MB; +60,756 civic OSM records)
- layer2-clusters.pmtiles: 43.7 MB · clusters-meta.json: 2,876 KB
- Strathcona County: 4 clusters (Co-location 1–4); Co-location 4 is the new Walmart Supercentre anchor
- Live at gis.woodfinegroup.com

### Commit-flow note for Command Session
Code files (ingest-osm.py + build-tiles.py) staged via `git add -f` in project-gis git (cluster/project-gis branch). The 4f7b0b0 workspace commit added `pointsav-monorepo/` to clones/project-gis/.gitignore, leaving code tracking in limbo between workspace git and project-gis git. Command Session should clarify the intended mechanism and update NEXT.md.

### Open follow-ups (unchanged backlog)
Same 13 items in outstanding-todo.md (A1, B1, B2, C1, C2, D1–D4, E1–E4).
