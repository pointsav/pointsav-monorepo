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
re: Nordic IKEA re-ingest complete — DK+NO+FI gain first T1 each; 5,274 clusters deployed
created: 2026-05-22T21:00:00Z
msg-id: nordic-ikea-reingest-20260522
---

Nordic IKEA per-country re-ingest complete. Deployed.

## Root cause fixed

`ikea-nordics.yaml` used `multi_country: true` → stores landed with `iso_country_code: "NORDICS"` (not a real ISO). Per-country chain_ids (ikea-se/dk/no/fi) existed in taxonomy.py but were served by contaminated JSONL files generated from the nordics multi_country ingest (no bbox/polygon filtering → UK stores in ikea-dk, Lithuanian store in ikea-se).

## Fix

Created `ikea-se.yaml`, `ikea-dk.yaml`, `ikea-no.yaml`, `ikea-fi.yaml` in service-business/ with proper per-country bbox. Re-ran `ingest-osm.py --chain ikea-se ikea-dk ikea-no ikea-fi`. The polygon filter cleaned cross-border contamination.

Note: Norway `country_code: NO` required quoting in YAML (YAML 1.1 parses bare `NO` as boolean False).

## New data

| Chain | Records | Key change |
|---|---|---|
| ikea-se | 16 | +1 clean record; Lithuanian ghost removed |
| ikea-dk | 5 | Was 12 (7 UK/SE ghosts); now 5 clean DK stores |
| ikea-no | 5 | Was 6; now clean Oslo/Trondheim/Slependen/Sørlandet/Åsane |
| ikea-fi | 4 | Same count; Espoo/Vantaa/Tampere/Kuopio |

## T1 results (deployed)

5,274 clusters, T1=1,136:
- **DK: first T1** — Odense (Bilka + Silvan/Imerco + IKEA)
- **NO: first T1** — Oslo/Furuset (OBS Coop + OBS Bygg + IKEA)
- **FI: first T1** — Tampere (Prisma + K-Rauta + IKEA)
- SE: still 4 T1 (unchanged)

---
from: task@project-gis
to: command@foundry
re: Phase 17 complete — EU taxonomy audit; 12 zero-cost chain activations; 5,273 clusters; T1=1,136
created: 2026-05-22T20:00:00Z
msg-id: phase17-complete-20260522
---

Phase 17 committed (af434817, jwoodfine). EU country-by-country taxonomy audit complete.

## Chains activated (zero-cost — data already ingested, taxonomy gap only)

| Chain | ISO | Records | Note |
|---|---|---|---|
| carrefour-hypermarket-es | ES | 326 | Q217599 |
| alcampo-es | ES | 323 | Q2832081 (Auchan ES) |
| leclerc-es | ES | 220 | |
| carrefour-hypermarket-it | IT | 215 | Q217599 |
| famila-it | IT | 215 | Selex group |
| ipercoop-it | IT | 101 | Coop Italia hyper |
| auchan-pl | PL | 120 | Q758603 — fixes 0% T1 country |
| carrefour-hypermarket-pl | PL | 114 | Q217599 — fixes 0% T1 country |
| maxi-ica-se | SE | 50 | Q104553487 — Sweden's #1 large-format |

## Correction applied

- `clas-ohlson-se` demoted from hardware — small-format housewares, not home-improvement anchor

## T1 uplift (EU)

| ISO | Before | After | Delta |
|---|---|---|---|
| PL | 0 | 17 | +17 |
| IT | 3 | 8 | +5 |
| ES | 25 | 29 | +4 |
| SE | 8 | 4 | -4 (correct — false hardware signals removed) |
| EU total | ~101 | ~123 | +22 |

## Overall rebuild result

- 5,273 clusters; T1=1,136 / T2=3,865 / T3=272
- layer2-clusters.pmtiles (37.7 MB) deployed
- clusters-meta.json (11 MB) deployed

## Data gap identified (NOT yet fixed — requires re-ingest)

`ikea-nordics.yaml` uses `multi_country: true` → stores land with `iso_country_code: "NORDICS"`
(not a real ISO). ikea-se/dk/no/fi chain_ids have 0 ingested records. DK and NO have literally
no usable IKEA data. Fix: per-country YAML with ISO bbox filtering.

## Next ingest work queued

| Chain | ISO | Stores | Priority |
|---|---|---|---|
| ikea-se, ikea-dk, ikea-no, ikea-fi | SE/DK/NO/FI | ~26 total | HIGH — Nordic IKEA gap |
| kaufland-pl | PL | ~220 | HIGH — largest remaining PL opportunity |
| foetex-dk | DK | ~100 | MEDIUM — fixes thin DK hypermarket |
| wickes-uk | GB | ~230 | HIGH — already flagged in chain-coverage-audit |
| interspar-at | AT | ~70 | LOW |

---
from: task@project-gis
to: command@claude-code
re: DELIVERY REQUEST — colocation-tier-summary to project-jennifer
created: 2026-05-22T00:00:00Z
priority: normal
msg-id: delivery-20260522-colocation-summary-jennifer
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-22T17:56:00Z
note: files copied to project-jennifer outputs/co-locations/; inbox message prepended
---

Two files are staged in this cluster's drafts-outbound and need delivery
to project-jennifer. Committed at 70fd82a3.

Source files (this cluster):
  /srv/foundry/clones/project-gis/.agent/drafts-outbound/colocation-tier-summary.json
  /srv/foundry/clones/project-gis/.agent/drafts-outbound/colocation-tier-summary.html

Destination (project-jennifer):
  /srv/foundry/clones/project-jennifer/outputs/co-locations/colocation-tier-summary.json
  /srv/foundry/clones/project-jennifer/outputs/co-locations/colocation-tier-summary.html

This cluster (mathew) cannot write to /srv/foundry/clones/project-jennifer/outputs/
(owned jennifer:jennifer, mathew not in jennifer group). Command Session holds the
necessary access.

Requested actions:
  1. cp both files to the destination paths above (creating the directory if absent)
  2. Prepend an inbox message to project-jennifer's INBOX.md (or equivalent) notifying
     Jennifer that the co-location tier/category summary is available in
     outputs/co-locations/ — two formats: JSON (data) + HTML (print-ready landscape sheet).

Content summary for the inbox note:
  - 16 countries across 4 regions (North America, UK, Continental Europe, Nordic)
  - Per-country: cluster counts, T1/T2/T3 tier breakdown, category member counts
    (Medical / Education / Hypermarket / Hardware / Price Club / Lifestyle)
  - HTML is one-page landscape letter print-ready (File > Print or ⌘P)
  - Data source: clusters-meta.json Phase 15 build (5,163 clusters, 2026-05-19)

No commit needed in project-jennifer unless Jennifer's repo tracks outputs/.

---
from: task@project-gis
to: command@foundry
re: ACK command-20260522-gis-stage6-orphan-branch + command-20260522-binary-targets-project-gis
created: 2026-05-22T17:35:00Z
msg-id: ack-20260522-gis-inbox-items
---

Both inbox items actioned. Session 4 (2026-05-22).

**[1] Stage 6 unblocked — msg-id: command-20260522-gis-stage6-orphan-branch**

Rebase complete. `cluster/project-gis` now has `origin/main` as ancestor.

- Strategy: `git rebase -X theirs origin/main cluster/project-gis`
- Conflicts resolved: `.agent/` files (all → theirs), `.gitignore` (→ theirs, cluster
  simplified version), `slm/MODULE_ID` + `slm/README.md` (→ theirs), `GEMINI.md` +
  `.agent/session-start.md` (→ theirs). All 87 commits applied.
- Signing key set: `gpg.format=ssh`, `user.signingkey=/srv/foundry/identity/jwoodfine/id_jwoodfine`
- Verified: `git merge-base --is-ancestor origin/main cluster/project-gis` → exit 0
- Branch now 87 commits ahead of `origin/main`. Ready for Stage 6.

Note: local branch diverges from `origin/cluster/project-gis` (remote has 45 pre-rebase commits).
Command Session should force-update remote ref when promoting: `git push origin-staging-j
cluster/project-gis --force-with-lease` or equivalent. Do not merge remote pre-rebase commits.

**[2] binary-targets.yaml — msg-id: command-20260522-binary-targets-project-gis**

`.agent/binary-targets.yaml` was already committed (4cdf7d3d, S2 pipeline commit).
All three products declared with `soft_enabled: false` (scaffold):
- `os-orchestration` (class: os-image, layer: base)
- `app-orchestration-gis` (class: app-bundle, layer: extension, requires: [os-console])
- `app-console-gis` (class: app-bundle, layer: extension, requires: [os-console])
License: FSL-1.1-ALv2 for all three.

---
from: task@project-gis
to: command@foundry
re: Phase 15 complete — wegmans/winco/sprouts; T1=435; 13,657 clusters; disk resolved
created: 2026-05-19T02:30:00Z
---

Phase 15 committed (c5662554, jwoodfine). Pipeline complete.

## New chains (all ALPHA_HYPERMARKET via name_query)

| Chain | Records | QID |
|---|---|---|
| wegmans-us | 114 | Q1182328 |
| winco-foods-us | 145 | Q2584339 |
| sprouts-us | 450 | Q7580917 |

## T1 breakdown (T1=435, 13,657 clusters)

| ISO | T1 |
|---|---|
| US | 282 |
| ES | 51 |
| MX | 34 |
| CA | 32 |
| DE | 14 |
| FR | 13 |
| GB | 9 |
| **Total** | **435** |

T2=1,602 / T3=3,080 / T4=8,540

## Disk resolved

Root now at **58% (45G free)** — cargo-target cleanup appears to have been done. DISK ALERT from Phase 14 can be archived.

Secondary (sdb): 66% (17G free).

## Outstanding operator decisions (unchanged)

| Item | Decision needed |
|------|----------------|
| Path C composition (HW∧HM as T1-qualifying) | Methodologically significant; operator pick |
| UK/FR/DE OD manual downloads | ONS ODWP01EW, INSEE FD_MOBPRO, BA Pendler |
| US LODES full ingest | 5 states downloaded; full run ~2-4h; ingest-lodes.py auto-download |

---
from: task@project-gis
to: command@foundry
re: DISK ALERT — root at 85%; cargo-target 16G needs Command Session action
created: 2026-05-17T23:00:00Z
---

Root disk: **85% used (12G free)**. Was 65% in Phase 11 storage report. Key consumers:

| Directory | Size | Root? | Action |
|---|---|---|---|
| `/srv/foundry/cargo-target/debug/` | 14G | YES | **Needs `cargo clean` — Command Session** |
| `/srv/foundry/cargo-target/release/` | 2.3G | YES | **Needs `cargo clean` — Command Session** |
| `/srv/foundry/clones/` | 7G | YES | Normal — active repos |
| `/srv/foundry/deployments/` | 6.5G | YES (excl sdb mount) | Tiles + boundaries, OK |

## Immediate Totebox actions taken (Phase 14):

- Deleted `work/layer1-locations.geojson` (184M) — build intermediate, pmtiles already built
- Deleted `work/mobility-work.geojson` + `work/mobility-home-es.geojson` (2.2M) — build intermediates
- Moved `work/spend-catchment.geojson` (581M) + `work/census-catchment.geojson` (464M) to sdb + symlinked
- Net root freed: ~1.4G (87% → 85%)
- sdb now at 82% (from 79%)

## Command Session action needed:

`cd /srv/foundry && cargo clean`

This would recover ~16G on root (from 85% → **64%**). E3 (Rust service-ingest crate) is on HOLD per workspace strategy — build artifacts are safe to remove.

Secondary disk (sdb) at 82% (6.9G free). Monitor; avoid adding more than 3G to sdb without disk expansion.

---

from: task@project-gis
to: command@foundry
re: Phase 14 complete — full rank refresh; T1=432 (13,164 clusters)
created: 2026-05-17T22:00:00Z
---

Phase 14 complete. synthesize-od-study.py re-run covers all 13,164 clusters.

## Final T1 breakdown (P10=0.25 + Gate B + full catchment ranks)

| ISO | T1 |
|---|---|
| US | 279 |
| ES | 51 |
| MX | 34 |
| CA | 32 |
| DE | 14 |
| FR | 13 |
| GB | 9 |
| **Total** | **432** |

Note: US T1 dropped from 303 → 279 vs Phase 12 because the percentile ranks are now computed across 13,164 clusters (vs 11,240 before). More UK/Tesco/Sainsbury's clusters competing in the GB ISO pool tighten the P25 gate. ES improved 42→51 (Chedraui clusters now have rank data).

## T2/T3 improvement

T2: 1,021 → 1,594; T3: 2,211 → 3,091. Many new Tesco/Sainsbury's co-location clusters correctly fall into T2/T3 (strong co-location but below T1 rank gate).

## Operator decisions pending

1. **Path C composition** — add HW∧HM (HomeDepot+Walmart) as T1-qualifying. Sim-1b estimated +199 NA T1. Methodologically significant.
2. **US LODES ingest** — 5 of 51 states downloaded. Full ingest (all 50+DC) would give US clusters observed work-reach data instead of radius estimates. ingest-lodes.py is auto-download; estimate 2-4h for full run.
3. **P10 recalibration** — with 13,164 clusters, P25 applied ISO-wide. May want to verify US T1=279 is acceptable or adjust threshold.

---

from: task@project-gis
to: command@foundry
re: Phase 13 complete — UK re-ingest + MITMA mobility; 13,164 clusters
created: 2026-05-17T21:30:00Z
---

Phase 13 complete. All pipeline stages rebuilt; 13,164 clusters.

## 1. UK chain re-ingest results

Root cause: wikidata tag (Q487494 / Q152096) only covers main-format stores; Express/Metro/Local use sub-format QIDs. Fix: wikidata_id nulled, name_query partial=true.

| Chain | Before | After | Ratio |
|---|---|---|---|
| tesco-uk | 784 | 3,872 | 4.9× |
| sainsburys-uk | 672 | 1,903 | 2.8× |
| tiendas-3b-mx | 151 | 247 | 1.6× (OSM coverage thin in MX) |

GB T1 remains 10 — new stores add clusters but most lack catchment rank data (synthesize-od-study.py was run pre-Phase 12, missing new clusters).

## 2. MITMA mobility_source integration

build-mobility-tiles.py ran against mitma-work-od-es.jsonl (3,261 cells) + mitma-home-od-es.jsonl (3,234 cells). 58 ES clusters updated from mobility_source='radius' → 'mitma'. Layer6 + Layer7 PMTiles rebuilt.

## 3. Cluster count

13,164 total clusters (vs 11,240 Phase 12). Net +1,924 from Tesco/Sainsbury's coverage expansion.

## 4. Operator decisions needed

- **synthesize-od-study.py re-run**: 13,164 clusters; only 10,121 have catchment rank data. ~3,043 new clusters (mostly new Tesco/Sainsbury's/ASDA/Morrisons GB clusters) lack rank_pp_iso and cannot reach T1. Re-running synthesize-od-study.py would fill in ranks and likely push GB T1 above 10.
- **Path C composition** (HW∧HM as T1-qualifying): still the lever to push US T1 toward 400+.

---

from: task@project-gis
to: command@foundry
re: Phase 12 complete — GB anchor expansion + T1=442 (↑ from 305)
created: 2026-05-17T21:00:00Z
---

Phase 12 committed (3b367a9f, pwoodfine). 11,240 clusters total; T1=442.

## 1. New chains ingested

| Chain | Records | QID | Note |
|---|---|---|---|
| asda-uk | 1,051 | Q297410 | ASDA / Walmart UK |
| morrisons-uk | 620 | Q922344 | Morrisons |
| heb-us | 301 | Q1665088 | H-E-B; name_query fallback (wikidata tag sparse in OSM) |
| whole-foods-us | 528 (existing) | Q1809448 | Promoted from GENERIC_FOOD |
| chedraui-mx | 249 (existing) | Q2336803 | Promoted from generic |

## 2. T1 results after Phase 12

P10=0.25, Gate B (hc_count≥1, any hospital), IoU≤0.10.

| ISO | T1 |
|---|---|
| US | 303 |
| ES | 42 |
| CA | 32 |
| MX | 28 |
| FR | 13 |
| DE | 13 |
| **GB** | **10** (was 3) |
| NO | 1 |
| **Total** | **442** |

GB went from 3 → 10 (ASDA + Morrisons adding density). US at 303, approaching 358 target from sim-1b.

## 3. Remaining paths to 500 US T1

Sim-1b (Phase 11) projected P=0.25 + Gate B → NA=358. We achieved US=303 + CA=32 + MX=28 = 363 NA total (vs 358 projected). On track. To push further:
- **Path C — HW∧HM composition** (Home Depot + Walmart clusters): sim-1b estimated +199 NA at P=0.20. At P=0.25 this would be higher. Methodologically significant change — needs operator review.
- **More US chains**: Wegmans (Q1182328, ~110 stores), WinCo Foods (Q2584339, ~130), Sprouts (Q7580917, ~380). All ALPHA_HYPERMARKET candidates.

## 4. Stale backups deleted

clusters.geojson.pre-sprint9 + .new + .bak removed (35M freed). Root now 67% used.

## 5. H-E-B OSM tag gap noted

H-E-B OSM records overwhelmingly lack `brand:wikidata=Q1665088` — name_query fallback retrieved 342 raw → 301 final. A dedicated OSM tag campaign for H-E-B stores would improve data quality over time. Not a blocker.

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
