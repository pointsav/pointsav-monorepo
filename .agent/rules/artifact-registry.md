---
schema: foundry-artifact-registry-v1
project: project-editorial
project: project-system
project: project-gis
project: project-knowledge
project: project-infrastructure
last_updated: 2026-06-14
last_updated: 2026-06-14 (session 8 — CODE C updated with commit 1a914564; entity_filter.rs added)
---

# project-infrastructure Artifact Registry

Persistent record of all CODE, SOFT, TOPIC, GUIDE, TEXT, and JOURNAL artifacts
produced by this archive. Updated as artifacts are staged, dispatched, or completed.

Routing:
- TOPIC / GUIDE / TEXT / PROSE-RESEARCH / JOURNAL → `project-editorial` via `.agent/drafts-outbound/`
- CODE → commit directly to archive git (self-contained; no drafts-outbound)
- SOFT → commit directly; marketplace listing via `app-privategit-marketplace`
- DATA → commit directly (none currently — architecture is stateless by design)

Bright-line rules (from CLAUDE.md):
- CODE: runs our systems; internal deploy; no customer license key
- SOFT: Ed25519 license key + marketplace listing + price at software.pointsav.com
- TOPIC: explains WHAT/WHY; bilingual EN+ES; survives decommission
- GUIDE: instructs HOW-NOW; English-only; tied to a specific deployment

---

## CODE Artifacts

Self-contained binaries deployed on vault-privategit-source-1 and PPN fleet nodes.
Committed directly; Stage 6 signals sent to Command.

| Crate | Binary / Port | Status | Last commit |
|---|---|---|---|
| `service-vm-fleet` | `local-vm-fleet.service` :9203 | Active — deployed; 14 tests | 5e851ecc |
| `service-vm-host` | `local-vm-host.service` :9220 (per node) | Active — deployed; 7 tests; .meta.json sidecar | 2717fbce |
| `service-vm-tenant` | `local-vm-tenant.service` :9221 | Active — A1 opaque bearer (TOKEN_MAP); A3 SLIRP host_ports; A4 service-fs audit route | dbf6a528 |
| `system-vm-fleet-types` | shared wire types (`no_std`-compatible) | Active — HostPortMapping + host_ports in VmRecord; backward-compat serde default | dbf6a528 |
| `app-network-admin` | CLI :8085 HTTP + :9206 UDP listen | Active — Phase S3: fleet watch loop; auto WireGuard peer-table + WORM ledger; 8 tests | 13ef4654 |

---

## SOFT Artifacts

Commercial products verified with Ed25519 license tokens. Sold at software.pointsav.com.
Cash register test: licensable + marketplace-listed = SOFT.

| Product ID | Crate | Port | Status |
|---|---|---|---|
| `soft-slm-orchestration` | `app-orchestration-slm` | `local-orchestration-slm.service` :9180 | Active — deployed; 15 tests; ORCHESTRATION_LICENSE_PUBKEY_HEX runtime load (C, 85e8c60f); marketplace listing pending operator action |

Notes:
- `REQUIRED_PRODUCT = "soft-slm-orchestration"` in `orchestration-slm/src/license.rs`
- Ed25519 offline verification — no network call for license check
- `ORCHESTRATION_LICENSE_PUBKEY_HEX` env var → runtime pubkey (C, wiring in progress)
- Marketplace listing in `app-privategit-marketplace` requires operator action

---

## TOPIC Artifacts

### A18 — BRIEF: Location Intelligence Archetypes (PRO / VWH / PKS)
- **File:** `.agent/briefs/BRIEF-location-intelligence-archetypes-2026-06-01.md`
- **Status:** ACTIVE — updated 2026-06-14; §10.13 Phase 5 MX coverage added; §10.11 version control resolved
- **Destination:** internal (BRIEF stays in archive); editorial drafts dispatched separately
- **Content:** Three-archetype Location Intelligence system (PRO/VWH/PKS). Definitions, co-location
  signals, full chain taxonomy with Wikidata IDs, airport/rail classification findings, service-parking
  architecture. §10: PKS production system — 7,045 clusters (Phase 5b); OXXO CONVENIENCE enrichment;
  MX=177 (was 183 Phase 5; 6 false US clusters removed; lon-scoped coord override); mode-group collapse; park-and-ride anchor; car rental + hotel + convenience enrichment.
  §11: VWH calibration — hardware profile (10,338 anchors, 45 chains), sim (73.4% hardware validation PASS),
  group-collapse tier rules, production results (6,368 clusters; T1=852/T2=1,327/T3=4,189; `retail_contamination` flag at 47.9%).
Explain WHAT and WHY. Bilingual EN+ES pair. Route to project-editorial.

| ID | File | Title | Status |
|---|---|---|---|
| T1 | `TOPIC-ppn-vm-architecture.draft.md` | PPN VM Resource Pool Architecture | STAGED — written 2026-06-14 (workflow D); ready for project-editorial |
| T2 | `TOPIC-ppn-tenant-vm-isolation.draft.md` | PPN Tenant VM Isolation | STAGED — written 2026-06-14; A1-A4 hardening stable; ready for project-editorial |

### A20 — TOPIC: Parking Structures (PKS)
- **File:** `.agent/drafts-outbound/TOPIC-parking-structures.draft.md`
- **Status:** STAGED — draft complete 2026-06-01; ready for project-editorial; draft references
  6,953 clusters (Phase 4) — editorial pass must update to Phase 5b production counts (see below)
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** PKS archetype definition (3-9 story transit car parks), regional-to-metro relationship,
  airport and rail station classification, car rental chains, parking operator directory. **Phase 5b
  production counts (update draft body):** 7,045 clusters (T1=692/T2=2,665/T3=3,688); MX=177;
  park-and-ride anchor (23,117 records); car rental + hotel enrichment; mode-group collapse tiers (see BRIEF §10).

### A21 — GUIDE: Location Intelligence Data Collection
- **File:** `.agent/drafts-outbound/GUIDE-location-intelligence-data-collection.draft.md`
- **Status:** STAGED — draft complete 2026-06-01; ready for project-editorial
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** 7-step operational runbook: run existing YAML ingests, add Würth + new taxonomy
  categories, add Tier A VWH chains, write ingest-osm-airports.py, write ingest-osm-railway.py,
  add PKS car rental chains, re-run test-cluster-archetypes.py and deploy updated GeoJSON.
  Updated 2026-06-12: all 7 steps now marked complete; Step 7 updated to production build scripts.

### A22 — TOPIC: Location Intelligence Co-location Archetypes
- **File:** `media-knowledge-projects/topic-location-intelligence-archetypes.md`
- **Paired:** `media-knowledge-projects/topic-location-intelligence-archetypes.es.md`
- **Status:** COMMITTED — commit d550b6b (Jennifer Woodfine, 2026-06-14); source drafts at project-gis/drafts-outbound/; editorial strip applied (H1 removed, code vars stripped from Map Integration section); Stage 6 pending
- **Destination:** media-knowledge-projects (committed)
- **Content:** Overview TOPIC covering all three co-location archetypes (PRO/VWH/PKS). Three-tier
  tier definitions, PRO Phase 23+Change B dataset (6,493 clusters), VWH production results
  (6,368 clusters; T1=852/T2=1,327/T3=4,189; retail_contamination flag), PKS production results
  (7,045 clusters; T1=692/T2=2,665/T3=3,688; MX=177; park-and-ride anchor; car rental+hotel enrichment; Phase 5b; false US clusters removed).
  Map integration (vwhActive/psActive toggle layer documentation). EN+ES pair.

### A23 — GUIDE: AEC Hazard Pipeline Repair
- **File:** `.agent/drafts-outbound/GUIDE-gis-aec-pipeline-repair.draft.md`
- **Status:** STAGED — draft complete 2026-06-11; ready for project-editorial
- **Destination:** project-editorial → woodfine-fleet-deployment/gateway-orchestration-gis-1/
- **Content:** Operational runbook for diagnosing and repairing failures in the AEC hazard data
  pipeline (atmospheric, environmental, climatic enrichment). Covers pre-flight verification,
  failure patterns from build-aec-flood.sh and build-aec-seismic.sh, URL fix reference (bd17a348).

### A24 — GUIDE: GIS Nightly Rebuild Operations
- **File:** `.agent/drafts-outbound/GUIDE-gis-nightly-build-operations.draft.md`
- **Status:** STAGED — draft complete 2026-06-11; ready for project-editorial
- **Destination:** project-editorial → woodfine-fleet-deployment/gateway-orchestration-gis-1/
- **Content:** Operational runbook for scheduled nightly cluster rebuild. Covers nightly-rebuild.sh,
  build-vwh-clusters.py and build-pks-clusters.py trigger sequence, overnight build timing policy
  (05:00 UTC minimum start), and post-build verification steps.

### A26 — BRIEF: GIS Nightly Rebuild + AEC Layer Infrastructure
- **File:** `.agent/briefs/BRIEF-gis-nightly-rebuild-aec-2026-06-12.md`
- **Status:** ACTIVE — internal research record; not dispatched to project-editorial
- **Destination:** internal (BRIEF stays in archive)
- **Content:** 3-audit investigation (2× Opus, 1× Fable) 2026-06-12. Documents: cross-archive
  gateway contamination (project-orgcharts was active cron job overwriting gateway nightly),
  AEC wipe mechanism (build-tiles.py regenerates clusters-meta.json from scratch — 21 fields, no
  merge), cron timezone error (0 5 fires at 12:00 UTC not 05:00 UTC), missing scripts in
  project-gis (build-clusters.py / build-tiles.py / taxonomy.py). Fable governance design:
  A+D hybrid (pairings.yaml registry + .owner files + deploy-guard + cron-audit.sh). Fix sequence:
  Phases 0–5 with operator action callouts. §4 checkboxes updated as work completed 2026-06-12.

### A25 — BRIEF: PKS Commuter Archetype — Fable Model Analysis
- **File:** `.agent/briefs/BRIEF-pks-fable-analysis-2026-06-11.md`
- **Status:** ACTIVE — internal research record; not dispatched to project-editorial
- **Destination:** internal (BRIEF stays in archive)
- **Content:** Fable model consultation (claude-fable-5) on PKS tier calibration. Documents root
  causes of original 57% fake bimodal issue (ICR+CR double-counting), mode-group collapse fix,
  EPS_LOOSE=2.5km anti-chaining rationale, and park-and-ride anchor design decision. Reference
  for PKS methodology decisions in BRIEF §10 and sim-pks-colocation.py.

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
T1 covers: service-vm-fleet / service-vm-host / service-vm-tenant / app-orchestration-slm /
WireGuard mesh / planned seL4 + Firecracker extensions. ~900 words. ES sibling added by project-editorial.

T2 covers: namespace isolation, process isolation, per-VM network containment, opaque bearer
tokens, WORM audit trail, quota serialization, what's not guaranteed (network-level, node operator),
path to Phase S3 + seL4 Mode B. ~1,000 words. ES sibling added by project-editorial.

Backlog:
- TOPIC: os-network-admin as PPN Control Plane — after Phase S3 ships

---

## GUIDE Artifacts

Instruct HOW-NOW. English-only. Tied to a named deployment. Route to project-editorial.

| ID | File | Title | Status |
|---|---|---|---|
| G1 | `GUIDE-ppn-fleet-operations.draft.md` | PPN Fleet Operations | STAGED — written 2026-06-14 (workflow D); ready for project-editorial |
| G2 | `GUIDE-ppn-node-setup.draft.md` | PPN Node Setup | STAGED — written 2026-06-11 (claude-fable-5); frontmatter updated 2026-06-14; ready for project-editorial |

G1 covers: check fleet status, add/reserve a node, spawn VM (operator + tenant paths),
destroy VM, check orchestration-slm. Concrete curl examples on actual ports.

G2 covers: installing os-infrastructure on a new machine and joining the PPN mesh.
Current path is manual (3 commands on node); target is single bootable ISO.

Destination: woodfine-fleet-deployment catalog entry for this infrastructure (pending creation).

Backlog:
- GUIDE: Adding a PPN Node (operator) — G2 update once Genesis Protocol ISO boots bare-metal
- GUIDE: SLM Orchestration Setup — once C (license pubkey) and marketplace listing are done

---

## TEXT Artifacts

Short-form copy for UI, website, or marketing surfaces.

| ID | File | Title | Status |
|---|---|---|---|
| TX1 | `TEXT-ppn-any-hardware-sovereign-compute.draft.md` | Any Hardware, Sovereign Compute | STAGED — written 2026-06-14; ~350 words; ready for project-editorial; target: pointsav.com product page |

---

## JOURNAL / PROSE-RESEARCH Artifacts

Long-form research drafts. Named natural-person authors only. Route to project-editorial.

| ID | File | Title | Status |
|---|---|---|---|
| PR1 | `PROSE-RESEARCH-ppn-architecture-phd-thesis.draft.md` | PointSav Private Network: A Formally-Isolated Sovereign Virtualization Platform | STAGED — v0.2 (43KB); frontmatter updated 2026-06-14; 61 citations; benchmark placeholders [T][N][L] pending empirical data; ready for project-editorial review pass |
| J1 | `JOURNAL-ppn-pooled-compute-v0.1.draft.md` | Pooled Compute from Heterogeneous Hardware | STAGED — lead: Peter M. Woodfine; frontmatter fixed 2026-06-14 (removed incorrect email); target: IEEE TCC (IF 6.49); ORCID IDs for all 3 authors required before submission |

Pre-submission blockers:
- PR1: academic register review; citation completeness (61 entries; gaps flagged); BCSC disclosure pass; ORCID IDs
- J1: benchmark placeholders [T][N][L] need empirical data; Peter + Mathew + Jennifer ORCID IDs; Peter email address unknown (do not use jmwoodfine@gmail.com)

---

## Contaminated files in .agent/drafts-outbound/

These do not belong to project-infrastructure. Do not route as infrastructure artifacts.
Flag to Command for correct routing when next Command sweep runs.

| Artifact | File | Status |
|---|---|---|
| PKS profiling script | `app-orchestration-gis/analyze-parkade-colocation.py` | DONE (2026-06-11; profiles all POI categories within 3km of 140,201 existing parking structures) |
| PKS calibration sim | `app-orchestration-gis/sim-pks-colocation.py` | DONE (2026-06-11; 5 iterations; transit-first, commercial-first, EPS sensitivity; calibration converged to production) |
| Parkade co-location profile | `work/parkade-colocation-profile.json` | DONE (2026-06-11; commuter_rail 64.7% / car_rental 44.7% / hotel 36.2% / airport 4.1% within 3km of built structures) |
| archetype-pks.geojson (calibrated) | `gateway-orchestration-gis-1/www/data/archetype-pks.geojson` | DONE (2026-06-14 Phase 5b; 7,045 features; T1=692/T2=2,665/T3=3,688; MX=177; false US clusters removed; lon-scoped coord override; deployed) |

### PKS Phase 5b — MX Fix + OXXO GULF-MID (2026-06-14)

Bug-fix pass + OXXO gap coverage. See BRIEF §10.14 for methodology. Commits a18f9240, f8e8f7f5, 469c10b9, 74f6e782.

| Artifact | File | Status |
|---|---|---|
| _coord_override_iso US rules | `app-orchestration-gis/build-pks-clusters.py` | DONE (a18f9240 + f8e8f7f5; 3 US-detection rules with lon bounds [-118.5, -86.0]; removes Fort Worth TX / Shreveport LA / Tucson AZ / San Diego CA false MX clusters) |
| OXXO GULF-MID gap ingest | `app-orchestration-gis/ingest-oxxo-gulfmid.py` | DONE (469c10b9; 6 sub-bboxes; 30s back-off; 110 new Tampico/Cd.Victoria records appended) |
| OXXO spatial audit script | `app-orchestration-gis/ingest-oxxo-mx-extend.py` | DONE (469c10b9; 4-bbox gap audit; confirmed most areas already covered) |
| oxxo-mx.jsonl (extended) | `service-fs/service-business/oxxo-mx.jsonl` | DONE (6,427 → 6,537 records; +110 GULF-MID; total ≈29% of ~22,000 MX stores) |

**EU car rental chain ingests (2026-06-11):**

| File | Records |
|---|---|
| `service-business/hertz-eu.jsonl` | 687 |
| `service-business/avis-eu.jsonl` | 741 |
| `service-business/budget-eu.jsonl` | 130 |
| `service-business/europcar-eu.jsonl` | 1,021 |
| `service-business/sixt-eu.jsonl` | 246 |
| `service-business/budget-us.jsonl` | 278 |
| `service-business/alamo-us.jsonl` | 110 |
| `service-business/national-us.jsonl` | 2 |

**Hotel chain ingests (2026-06-11):**

| File | Records |
|---|---|
| `service-business/ibis-eu.jsonl` | 708 |
| `service-business/b-and-b-hotels-eu.jsonl` | 797 |
| `service-business/premier-inn-gb.jsonl` | 817 |
| `service-business/travelodge-gb.jsonl` | 580 |
| `service-business/motel-one-de.jsonl` | 24 |
| `service-business/holiday-inn-express-us.jsonl` | 2,021 |
| `service-business/hampton-us.jsonl` | 240 |
| `service-business/courtyard-us.jsonl` | 1,020 |

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

## A — Active / In-Progress (PPN + Totebox Pipeline)

### A1–A8 — PPN / OS surface staged drafts

| ID | File | Title | Destination | Status |
|----|------|-------|-------------|--------|
| A1 | `drafts-outbound/GUIDE-ppn-node-setup.draft.md` | PPN Node Setup Guide | project-editorial | staged |
| A2 | `drafts-outbound/TOPIC-ppn-small-business-compute.draft.md` | PPN Small-Business Compute | project-editorial | staged — EN+ES pair confirmed 2026-06-14 |
| A3 | `drafts-outbound/TOPIC-os-console-architecture.draft.md` | OS Console Architecture | project-editorial | staged — EN+ES pair confirmed 2026-06-14 |
| A4 | `drafts-outbound/TOPIC-software-distribution-substrate.draft.md` | Software Distribution Substrate | project-editorial | staged — EN+ES pair confirmed 2026-06-14 |
| A5 | `drafts-outbound/TOPIC-crypto-license-sales-architecture.draft.md` | Crypto License Sales Architecture | project-editorial | staged — EN+ES pair confirmed 2026-06-14 |
| A6 | `drafts-outbound/TOPIC-private-git-paid-customer-endpoint.draft.md` | Private Git Paid Customer Endpoint | project-editorial | staged — EN+ES pair confirmed 2026-06-14 |
| A7 | `drafts-outbound/JOURNAL-ppn-pooled-compute-v0.1.draft.md` | PPN Pooled Compute (JOURNAL) | project-editorial | staged |
| A8 | `drafts-outbound/PROSE-RESEARCH-ppn-architecture-phd-thesis.draft.md` | PPN Architecture PhD Thesis prose | project-editorial | staged |

### A9 — TOPIC: Dual-Tier Entity Extraction Architecture
- **File:** `drafts-outbound/TOPIC-dual-tier-extraction-architecture.draft.md`
- **Paired:** `drafts-outbound/TOPIC-dual-tier-extraction-architecture.es.draft.md`
- **Status:** STAGED 2026-06-14 — ready for project-editorial
- **Destination:** project-editorial → media-knowledge-documentation
- **Content:** Tier A (OLMo 7B local CPU, LadybugDB), Tier B (OLMo 32B L4 GPU), ALLOWED_CLASSIFICATIONS guard, EXTRACTION_SYSTEM_PROMPT hardening, drain-hold predicate (SLM_DRAIN_PAUSED), DPO enrichment loop (chosen=B / rejected=A), flush_tier_a() behaviour. 78% Tier B > Tier A enrichment signal confirmed 2026-06-14 (n=32 pairs).

### A10 — GUIDE: jennifer-2 Migration Stack Operation
- **File:** `drafts-outbound/GUIDE-jennifer-2-migration-stack.draft.md`
- **Status:** STAGED 2026-06-14 — ready for project-editorial
- **Destination:** project-editorial → woodfine-fleet-deployment (operator runbook)
- **Content:** Start service-fs :9103 (j2 WORM), service-extraction j2, service-input :9106; manual migration batch commands; nightly cron driver (nightly-jennifer-migrate.sh health gate + DPO loss guard); VM-reboot restart procedure; SLM_DRAIN_PAUSED usage; drop-file lifecycle (watch → processed/).

---

## B — Backlog

*(No backlog items at this time.)*

---

## C — Code Artifacts (jennifer-2 Ingest Pipeline, 2026-06-14)

Committed to `pointsav-monorepo` feature branch; Stage 6 pending.

| Artifact | Commit | File(s) | Notes |
|---|---|---|---|
| SFT corpus generator | 4ddff37f | `service-input/scripts/build-extraction-sft.py` | RAFT-style 182 SFT pairs from 461 human-curated YAMLs; normalize_reference_yaml handles heterogeneous schema (metric_name, theme_alignment, wrapper keys); people.csv pipe-delimited positional; provenance: human-curated |
| Nightly migration driver | 4ddff37f | `service-input/scripts/nightly-jennifer-migrate.sh` | /readyz health gate; tier_a bool + tier_b.<node>.circuit; go_no_go at summary.go_no_go (nested); DPO loss guard (skip if Tier A=true + Tier B=open); night window 05:00 UTC |
| Extraction quality fixes | 22c57822 | `service-content/src/main.rs` | ALLOWED_CLASSIFICATIONS const; enum guard in raw_entities_to_graph(); EXTRACTION_SYSTEM_PROMPT hardened: Location negative examples ("EXCLUDE: retail anchor location, downtown core"), SPDX/licence identifiers added to omit list |
| Drop-file lifecycle fix | 22c57822 | `service-extraction/src/main.rs` | After successful process_payload(), move drop file to watch_dir/processed/ (both startup drain site and inotify event site). Preserves audit trail vs. delete. |
| cargo fmt pre-promote | 28c69356 + f40f922c | `service-content/src/main.rs`, `service-extraction/src/main.rs` | Style-only; required by Stage 6 gate |
| Entity quality hardening (6 changes) | 1a914564 | `service-content/src/entity_filter.rs` (new), `service-content/src/main.rs`, `service-content/src/graph.rs`, `service-content/src/http.rs` | Changes 1-5: EXTRACTION_SYSTEM_PROMPT + is_noise_entity_name + clean_dpo_side + coerce_classification + word-count gate; Change 6: GET /v1/graph/cleanup endpoint + delete_entity on GraphStore trait; 30/30 tests; Stage 6 pending |

---

## D — Data / SFT Artifacts

| Artifact | Location | Count | Notes |
|---|---|---|---|
| Extraction SFT pairs (human-curated) | `/srv/foundry/data/training-corpus/extraction/jennifer-sft-*.jsonl` | 182 pairs | provenance: human-curated; RAFT-style entity candidates injected from people.csv (9,575 names); metric/theme labels from jennifer-1 human-curated YAML ledgers (461 files); entity labels sparse (YAML corpus is metric/theme-dominant) |
| DPO enrichment pairs (Tier B vs Tier A) | `cluster-totebox-jennifer/service-fs/data/training-corpus/feedback/enrichment-DOC_*.jsonl` | 4 pairs (2026-06-14) | chosen=Tier B (OLMo 32B), rejected=Tier A (OLMo 7B); P1+P3 clean, P2 contaminated (ops(slm) commit-prefix in chosen), P4 ambiguous; do NOT train as-is; pre-save validator needed; minimum 200–300 genre-diverse pairs before LoRA; provenance: olmo-self |
| Apprenticeship SFT corpus (git-commit activity) | `/srv/foundry/data/training-corpus/feedback/apprenticeship-git-commit-*.jsonl` | 834 pairs | Separate artifact type from DPO enrichment pairs; SFT from commit activity; provenance: human-curated (commits are human-authored) |
## D — Design Artifacts (staged → project-design)

DESIGN-* artifacts in `.agent/drafts-outbound/` routing to project-design.
All carry `foundry-draft-v1` frontmatter. TOKEN-CHANGE artifacts require Master co-sign.

### D1 — Knowledge Platform design artifacts (native to this archive)

| ID | File | Type | Subject | Status |
|----|------|------|---------|--------|
| D1a | `DESIGN-knowledge-platform-fresh-slate-analysis.draft.md` | DESIGN-RESEARCH | Visual + information architecture analysis for knowledge wiki engine — benchmarks against Wikipedia Vector 2022, Stripe Docs, Linear, MDN | STAGED — pending dispatch to project-design |
| D1b | `knowledge-platform-rewrite-analysis.draft.md` | DESIGN-RESEARCH | Companion analysis — schema: foundry-draft-v1; artifact_type: DESIGN-RESEARCH | STAGED — unclassified filename (no DESIGN- prefix); pending rename + dispatch |
| D1c | `DESIGN-doc-header-component.draft.md` | DESIGN-COMPONENT | Document header component spec for knowledge platform chrome | STAGED — pending dispatch to project-design |
| D1d | `DESIGN-docs-sidenav-component.draft.md` | DESIGN-COMPONENT | Sidebar navigation component spec for knowledge platform | STAGED — pending dispatch to project-design |
| D1e | `DESIGN-wireframe-home-header-v2c.draft.html` | DESIGN-COMPONENT | Home page header wireframe v2c (HTML) | STAGED — pending dispatch to project-design |

### D2 — Org chart design artifacts (originating_cluster: project-orgcharts / project-proforma)

These files landed in this archive's drafts-outbound from project-orgcharts work. They belong to project-orgcharts / project-design pipeline. Tracked here only for completeness; relay to correct archive or project-design directly.

| ID | File | Type | Subject | Status |
|----|------|------|---------|--------|
| D2a | `DESIGN-COMPONENT-orgchart-canvas.draft.md` | DESIGN-COMPONENT | Org chart canvas component | STAGED — relay to project-design (originated project-orgcharts) |
| D2b | `DESIGN-COMPONENT-orgchart-connector.draft.md` | DESIGN-COMPONENT | Org chart connector component | STAGED — relay to project-design |
| D2c | `DESIGN-COMPONENT-orgchart-node.draft.md` | DESIGN-COMPONENT | Org chart node component | STAGED — relay to project-design |
| D2d | `DESIGN-RESEARCH-orgchart-carbon-token-map.draft.md` | DESIGN-RESEARCH | Carbon token mapping for org chart | STAGED — relay to project-design |
| D2e | `DESIGN-RESEARCH-orgchart-token-system.draft.md` | DESIGN-RESEARCH | Org chart token system design | STAGED — relay to project-design |
| D2f | `DESIGN-RESEARCH-orgchart-woodfine-brand-spec.draft.md` | DESIGN-RESEARCH | Woodfine brand spec for org chart | STAGED — relay to project-design |
| D2g | `DESIGN-TOKEN-CHANGE-orgchart-layout-type.draft.md` | DESIGN-TOKEN-CHANGE | Org chart layout type tokens | STAGED — relay to project-design |
| D2h | `DESIGN-TOKEN-CHANGE-orgchart-primitives.draft.md` | DESIGN-TOKEN-CHANGE | Org chart primitive tokens | STAGED — relay to project-design |
| D2i | `DESIGN-TOKEN-CHANGE-woodfine-chart-css.draft.md` | DESIGN-TOKEN-CHANGE | Woodfine chart CSS tokens | STAGED — relay to project-design |
| D2j | `DESIGN-TOKEN-CHANGE-woodfine-yellow-magenta.draft.md` | DESIGN-TOKEN-CHANGE | Woodfine yellow/magenta palette tokens | STAGED — relay to project-design |
| D2k | `DESIGN-TOKEN-CHANGE-wp-tokens-20260602.draft.md` | DESIGN-TOKEN-CHANGE | Woodfine platform token set 2026-06-02; `state: draft-pending-master-cosign` (originated project-workplace) | STAGED — relay to project-design |
| D2l | `DESIGN-COMPONENT-financial-report-layout.draft.md` | DESIGN-COMPONENT | Financial report layout component (originated project-proforma) | STAGED — relay to project-design |
| D2m | `GUIDE-orgchart-authoring.draft.md` | GUIDE | Org chart authoring runbook (originated project-orgcharts) | STAGED — relay to project-editorial |

---

## X — Cross-Archive Contamination (staged here from other archives)

Files in `.agent/drafts-outbound/` that explicitly declare `originating_cluster: project-orgcharts`.
These are NOT artifacts of this archive. Tracked here to surface for cleanup.
Action: relay to project-orgcharts outbox or delete after confirming project-orgcharts has copies.

| File | Originating Archive | Type | Action |
|------|---------------------|------|--------|
| `COMMS-bencal-nature-of-business.md` (+ .html, -copy.html, .pdf) | project-orgcharts | COMMS | Relay to project-orgcharts or project-editorial; confirm originator has canonical copy |
| `RESEARCH-bencal-naming-conventions.md` (+ .pdf) | project-orgcharts | RESEARCH | Relay to project-orgcharts; confirm originator has canonical copy |

---

## E — Engine Code Artifacts (app-mediakit-knowledge)

CODE artifacts that run the knowledge wiki engine. Committed to sub-clone `pointsav-monorepo/`
via `commit-as-next.sh`. All require Stage 6 promotion by Command Session before going live.
Binary rebuild + service restart required after each Stage 6 (see BRIEF §4 Phase 9 notes).

| ID | Commit | Author | Description | Stage 6 | Live |
|----|--------|--------|-------------|---------|------|
| E1 | `9a1326df` | jwoodfine | Phase 0 gate: `scripts/stage6-gate.sh` xtask runner; red-link render path removed (L18 complete); `inject_wiki_prefixes` cross-mount wiring confirmed | CONFIRMED (origin/main) | Yes (binary `e5e899...`) |
| E2 | `bd435cc3` | pwoodfine | Phase 0 code: tokens.css regen; blueprints relates_to rail; slug 301 redirect (topic- prefix + ES locale); `/edit/{slug}` stub; CodeMirror 6 + toc-persistence.js; conditional chrome load | CONFIRMED | Yes |
| E3 | `7a2b9b42` | jwoodfine | Phase 0 mobile: M8 drawer CSS (transform+transition); tap-popover viewport flip; Cmd+K trigger + `window.openCmdK` exposure | CONFIRMED | Yes |
| E4 | `eeb60cbb` | pwoodfine | Phase 0 AppState: `mounts: Vec<Mount>` refactor; hardcoded content_dir/guide_dir removed; `blueprints.rs` wired | CONFIRMED | Yes |
| E5 | `6d554ec6` | jwoodfine | Phase 7 scaffold: `PeerConfig`; `peers: Vec<PeerConfig>` in AppState; federated MCP search; `activitypub.rs` stub; reqwest 0.12 + rustls-tls | Stage 6 PENDING | No (binary rebuild pending) |
| E6 | `3106b2e1` | pwoodfine | Audit repair Sprint A: WCAG 2.2 focus outline (C3 — navy 9.1:1 vs gold 2.26:1); article body link underline (M15) | Stage 6 PENDING | No |
| E7 | `48bfa7e7` | jwoodfine | Audit repair Sprint B: sitemap absolute URLs via `canonical_url` TOML field (M1); brand-instance from TOML `[site].instance` (M14); ES tab/breadcrumb i18n in wiki_handlers via inline match (M12); dead POST form removed from `/edit` view (M11) | Stage 6 PENDING | No |

**Post-Stage-6 Command actions required (E5/E6/E7):**
- Add `instance = "documentation"` / `"projects"` / `"corporate"` to each `/etc/local-knowledge/*.toml` (M14)
- Add `canonical_url = "https://documentation.pointsav.com"` (and peers) to TOML where applicable (M1)
- Rebuild binary from promoted canonical tip
- `sudo systemctl restart local-knowledge-{documentation,projects,corporate}.service`

---

## F — Content Repair Requests (dispatched to project-editorial 2026-06-14)

Outgoing requests generated from the 12-agent external audit (BRIEF §7). Not artifacts produced
here — these become TOPIC/GUIDE/TEXT artifacts at project-editorial. Tracked here for status
visibility.

| ID | Audit Ref | Priority | Subject | Dispatched | Status |
|----|-----------|----------|---------|------------|--------|
| F1 | C2 | CRITICAL | Tier semantics reconciliation — rewrite `co-location-ranking-system` so T1 = highest (matches all other articles); add authoritative `[[co-location-tier-nomenclature]]` wikilink from every tier-using article | 2026-06-14 outbox | Pending project-editorial |
| F2 | M2 | High | Author 4 start-here TOPICs per instance OR repoint onboarding chips to extant slugs (all 4 chips 404 on projects + corporate) | 2026-06-14 outbox | Pending project-editorial |
| F3 | M5 | High | Add hatnote to guide catalog: "These guides are accessible to Woodfine operators; they are not public wiki articles." Do not present unresolvable GUIDE slugs as live links | 2026-06-14 outbox | Pending project-editorial |
| F4 | M6 + M3 | High | Split transient operational/research content out of archetype TOPICs (vertical-warehouse, ranking-system) into GUIDE or BRIEF artifacts; keep TOPICs to durable declarative content; audit `category:` frontmatter on articles (M3 root cause: missing frontmatter, not engine bug) | 2026-06-14 outbox | Pending project-editorial |
| F5 | M7 | Medium | Add dated data-snapshot line to every article containing cluster/country counts ("Data as of YYYY-MM-DD build"); reconcile home headline to one snapshot or label superset | 2026-06-14 outbox | Pending project-editorial |
| F6 | M8 | High (legal) | Add "Data Sources" section naming © OpenStreetMap contributors / ODbL in every article whose methodology rests on OSM + Wikidata; surface Wikidata Q-IDs in brand-family taxonomy article | 2026-06-14 outbox | Pending project-editorial |
| F7 | M9 | High | EN/ES parity sweep: `co-location-ranking-system.es.md` is ~25% of EN length — full parallel translation required per L4; sweep all EN/ES pairs for lagging articles | 2026-06-14 outbox | Pending project-editorial |

---

## Maintenance

- Add artifacts when planned; update status when dispatched or returned.
- Cross-check with `.agent/briefs/README.md` for BRIEF-linked artifact chains.
- When an artifact is dispatched to project-editorial/project-design, update status to DISPATCHED + commit hash
- When an artifact is returned (approved/rejected), update status
- Add new artifacts here at the time they are planned — do not wait until staging
- E-series: update Stage 6 column when Command confirms promotion; update Live column when binary rebuild deployed
| File | Likely owner |
|---|---|
| `GUIDE-orgchart-authoring.draft.md` | project-orgcharts |
| `JOURNAL-NOTES-j3-20260602.draft.md` | project-gis (J3 = AEC Data Layers) |
| `JOURNAL-NOTES-j6-20260602.draft.md` | project-gis (J6 = Desktop Environment) |
| `RESEARCH-bencal-naming-conventions.{md,pdf}` | project-proforma |
| `knowledge-platform-rewrite-analysis.draft.md` | project-knowledge |
| `COMMS-bencal-*.{md,html,pdf}` | project-proforma or project-editorial |
| `DESIGN-*.draft.md` | project-design (orgchart / docs components) |
| `BRIEF-REVIEW-old-brief-verdict.md` | origin unknown |
