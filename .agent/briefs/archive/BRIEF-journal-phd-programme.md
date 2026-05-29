---
artifact: brief
name: BRIEF-journal-phd-programme
status: active
version: "0.3"
created: 2026-05-27
updated: 2026-05-29
owner: totebox@project-editorial
---

# PhD-Level Journal Programme — Next-Level Plan

Six JOURNAL manuscripts on the `foundry-journal-v1` schema. All target the top-3 venues
in their field. All three authors (Jennifer M., Peter M., Mathew Woodfine) on every paper
for CRediT spreading. Double-blind anonymisation pass = `forbidden_terms_cleared` pass.

Rules: `.agent/rules/journal-artifact-discipline.md`
Registry: `.agent/rules/artifact-registry.md` §J

---

## Status banner (2026-05-29)

Author block updated 2026-05-29: all six papers now use RAND/Brookings Approach A — named
individuals first (`**Jennifer M. Woodfine, Peter M. Woodfine, and Mathew Woodfine**`),
institution on second line (`Woodfine Management Corp., New York, NY, USA`), institutional
email (`corporate.secretary@woodfinegroup.com`), full-name cite_as. Commits `c4a51814` + `1abc094e`.
J1+J3 re-post dispatched to project-gis (msg-id `project-editorial-20260529-journal-j1-j3-repost`).

| ID | File | Body state | Cleared | Next action |
|----|------|------------|---------|-------------|
| J1 | JOURNAL-retail-colocation-v0.1.draft.md | Complete v0.3 (~8,200w) | language-cleared | §7.2 primary spec pending Phase 24B; permutation test; F1–F5 at project-gis; **repost dispatched** |
| J2 | JOURNAL-trustworthy-systems-v0.1.draft.md | Complete (~8,800w) | language-cleared | Bench #9 quiet-VM re-run; ASPLOS short-version reformat (~6,000w) |
| J3 | JOURNAL-aec-data-layers-v0.1.draft.md | Full body v0.2 (~7,800w) | language-cleared | §6 Results pending AEC nightly metrics from project-gis; **repost dispatched** |
| J4 | JOURNAL-private-network-v0.1.stub.md | Full body v0.4 (~6,400w) | language-cleared (§4–§5 final pass pending) | Word count gap 6,400 → 9,000; ORCID IDs; §4–§5 final forbidden-terms sweep |
| J5 | JOURNAL-totebox-orchestration-v0.1.stub.md | Stub (0w) | — | HOLD until J2 submitted |
| J6 | JOURNAL-desktop-environment-v0.1.stub.md | §1-§5 written v0.2 (~5,200w) | language-cleared | §6 Results pending user study execution |

---

## J1 — Retail Anchor Co-location (Economic Geography)

**Primary:** Economic Geography (Wiley-Blackwell, IF 7.2)
**Alternates:** Journal of Economic Geography (OUP, IF 6.3); Regional Studies (T&F, IF 5.54)

### Current state (2026-05-28)

Body complete v0.3 (~8,200 words). Language pass COMPLETE (`forbidden_terms_cleared: true`).
New §7.0 "Preliminary Phase 22 Analysis" added with executable OLS results.

**§7.0 OLS results (executable — committed 37523014):**
- Model A: `log(span_km) ~ T1_dummy + T2_dummy + C(country)` — N=6,481, 17 countries, R²=0.121
  - T1 β=+0.489 [0.359, 0.619] p<0.001 → exp(0.489)=1.631, T1 clusters 63% larger than T3
  - T2 β=+0.008 p=0.833 (not significant — T2 spans same as T3)
- Model B: `T1_dummy ~ has_price_club + has_electronics + has_lifestyle + has_sport + log_span_km + tight + C(country)` — R²=0.503
  - has_price_club β=+0.639 p<0.001; has_electronics β=+0.489; has_lifestyle β=+0.311; has_sport β=+0.135
- F6 partial forest plot produced at `work/figures/F6-ols-coefficients.png`

**F1–F5 status:** Ready at project-gis `work/figures/` (commit 59e28780). Scripts at
`app-orchestration-gis/generate-figures-f1-f5.py`. Regenerate: run from project-gis.

### Remaining blockers

1. **§7.2 primary spec not executable.** Requires Kontur population join (Phase 24B) for
   `log(pop_150km)` covariate and O-D data for `log(od_work)`. Separate session after Phase 24B.

2. **Permutation test (§7.1) not yet implemented.** Script `sim-tier-permutation.py` needs
   writing — shuffle T1/T2/T3 labels 10,000 times, one-tailed p-value. Cluster coordinates
   available in `work/clusters-ols.csv`.

3. **F6 update pending.** Current partial F6 shows composition predictors only. Full F6
   (including population + O-D covariates) needs §7.2 primary spec first.

4. **ORCID IDs** for all three authors (operator action).

5. **Bilingual ES sibling** required before journal submission.

### Figure production specifications

Full specs in PROSE-RESEARCH-geometric-site-selection.draft.md `figures_required:` block
and in project-gis outbox message `project-gis-20260527-a6-thesis-journal-handoff`.

| Figure | Tool | Blocked? | Spec summary |
|--------|------|---------|--------------|
| F1 Tier Decision Tree | graphviz dot | No | 3 decision nodes → T1(1,747)/T2(3,393)/T3(1,353) leaf nodes |
| F2 DBSCAN Schematic | geopandas + matplotlib | No | 2 panels: abstract ε/minPts + Edmonton real example on OSM |
| F3 Continental Dot Map | geopandas + matplotlib | No | NA (Albers) + EU (LAEA); tier colour; NO WEB MERCATOR |
| F4 Per-Country T1 Bar | matplotlib/seaborn | No | 13 countries, horizontal paired bars, NA/EU mean lines |
| F5 Span_km Violin | seaborn | No | log Y-axis; Kruskal-Wallis H in caption |
| F6 OLS Coefficient | statsmodels + matplotlib | **YES (regression)** | Forest plot + inset partial scatter |
| F7 Co-occurrence Heatmap | seaborn | No (enhancing) | 6×6 lift matrix, diverging palette |
| F8 T1 vs Pop Density | geopandas + matplotlib | No (supplement) | 2×3 metro grid, 600 DPI |

---

## J2 — Composing Trustworthy Systems (ASPLOS / EuroSys / ACM TOCS)

**Primary:** ASPLOS 2027 (ACM SIGARCH, 19.4% AR) — 11pp + 2pp abstract, 2-col ACM
**Alternates:** EuroSys 2027 (14.7% AR); SOSP 2027 (~20% AR); ACM TOCS (journal fallback)

### Current state (2026-05-28)

Body complete (~8,800 words). Language pass COMPLETE (`forbidden_terms_cleared: true`).
All forbidden vocabulary removed per language-pass session.

**Remaining blockers before submission:**

### Bench #9 re-run

`verify_inclusion_proof` composed 1024-leaf: 22 outliers at ±11% CI. The note says
"load avg < 1.0 required for publication-quality". Run from `system-ledger/benches/consult.rs`
on the GCP n2-class host during an idle period. Coordinate via project-system outbox.

### ASPLOS short version

ASPLOS 2027: 11 content pages + 2 abstract pages, 2-column ACM format (~6,000–7,000 words).
Current body is ~8,800 words — need to cut ~2,000–2,800 words. Create as a separate file:
`JOURNAL-trustworthy-systems-v0.1-asplos.md` (do not modify the TOCS-target version).

Sections to compress for ASPLOS: §2 Background (~800 words, cut to 400); §4.2–§4.3 (cut
one of the WORM layer sub-sections); §6 Implementation (cut the `moonshot-toolkit` sub-section
from 400 words to 150); §8 Conclusion (trim to 150 words).

---

## J3 — Open-Source AEC Data Layers (Automation in Construction)

**Primary:** Automation in Construction (Elsevier, IF 12.0)
**Alternates:** Journal of Computing in Civil Engineering (ASCE, IF 6.04);
  Journal of Information Technology in Construction (ITcon, IF 3.8, open access)

### Current state (2026-05-28)

Full body written v0.2 (~7,800 words). Committed `02117825`. Language pass COMPLETE
(`forbidden_terms_cleared: true`).

**What was written:** §1 Introduction (AEC site analysis gap, 4 regulatory data categories,
3 contributions), §2 Background (ASHRAE 169-2020/IECC, Floods Directive 2007/60/EC, FEMA
NFHL, USGS NSHM, NECB, RESOLVE 2017 ecoregions, H3 spatial indexing), §3 AEC Data Layer
Taxonomy (8-row table with regulatory standing, scope, license, complexity), §4 Per-Country
Coverage Assessment (16 ISO parity scorecard from AEC-DATA-PARITY-RESEARCH.md), §5 Integration
Pipeline (H3 res-7 approach, PMTiles, API/point enrichment, build-by-join), §6 Results
(structured TODO pending nightly build data), §7 Discussion + Formal Hypotheses + Falsification
Programme (7 tests), §8 Conclusion.

### Remaining blockers

1. **§6 Results — concrete coverage metrics.** Needs AEC nightly build pipeline output:
   - Script 1: ASHRAE 169-2020 + NECB climate zones → `DATA-aec-climate-zones-us-ca.csv`
   - Script 2: FEMA NFHL flood (US) + EU Floods Directive → `DATA-aec-flood-us.csv` + `DATA-aec-flood-eu.csv`
   - Script 3: USGS NSHM seismic → `DATA-aec-seismic-us.csv`
   - Script 4: NREL NSRDB solar GHI → `DATA-aec-solar-ghi-us.csv`
   Outbox message `project-editorial-20260528-j3-coverage-metrics` sent to project-gis.
   Metrics needed per script: H3 cells covered vs. total per country, coverage %, source vintage.

2. **ORCID IDs** for all three authors (operator action).

---

## J4 — Zero-Trust Private Network Architecture (IEEE TIFS)

**Primary:** IEEE Transactions on Information Forensics and Security (IF 9.65)
**Alternates:** Computers & Security (Elsevier, IF 7.98);
  IEEE Transactions on Network and Service Management (IF 6.44)

### Current state (2026-05-28)

§1–§3 + §6 Discussion + §7 Conclusion written v0.2 (~4,800 words). Committed `67eb9a37`.
Language pass COMPLETE (`forbidden_terms_cleared: true`). Title updated to CRMA (Customer-Rooted
Mesh Architecture).

**Full title:** "Customer-Rooted Mesh Architecture for Distributed Operational Systems:
Zero-Trust Isolation Without Vendor Key Custody"

**What was written:**
- §1 Introduction: ZTA vendor-key-custody problem; 4 risk categories; CRMA proposal
- §2 Background: NIST SP 800-207, BeyondCorp, WireGuard (Donenfeld 2017), Noise Protocol
  Framework (Perrin 2018), Tailscale/Netbird structural positioning
- §3 Architecture: CRMA design principles P1–P4 (node-local key generation, public-key-only
  coordinator, topology-derived AllowedIPs, customer-controlled audit log); hub-and-spoke
  WireGuard topology; three-ring AllowedIPs enforcement (Ring 1=10.8.1.x, Ring 2=10.8.2.x,
  Ring 3=10.8.3.x); BLAKE2s-chained audit log JSON format
- §6 Discussion: kill-chain completeness analysis (6 ATT&CK TA0008 techniques), practical
  limitations, comparison with commercial ZTA products on 4 criteria
- §7 Conclusion: H₁/H₀/H₂/H₃ hypotheses + 6-test falsification programme

**H₁:** Customer-rooted WireGuard mesh achieves equivalent lateral-movement resistance to
vendor-managed ZTA products, with customer-held routing keys and transparent audit logs,
measurable via kill-chain completeness metric under a defined threat model.

### Remaining blockers

1. **§4 Implementation:** Benchmark setup needs documenting — performance measurements of
   tunnel establishment time, rekey latency (60s default), policy-change propagation, failure-mode
   behaviour (peer unreachable, hub restart, key rotation).

2. **§5 Evaluation:** Benchmark data (tunnel establishment time, rekey latency, policy-change
   propagation, failure-mode behaviour) required for quantified results.

3. **[CITATION NEEDED] placeholders in References:** Cameron CA incident study (audit-log
   integrity) + ZTA latency comparisons need verified citations with DOIs.

4. **ORCID IDs** for all three authors (operator action).

---

## J5 — Capability-Secured Session Orchestration (MLSys / ASPLOS / OSDI)

**Status: HOLD.** Explicitly depends on J2 (JOURNAL-trustworthy-systems) being published
or at minimum under-review. J5 §2 cites J2 as prior work. Do not begin body writing until
J2 submission status updates to `submitted` or `under-review`.

---

## J6 — Muscle-Memory-Preserving Desktop Environments (ACM TOCHI / IJHCS)

**Primary:** ACM Transactions on Computer-Human Interaction (TOCHI, ACM, Q1 HCI)
**Alternates:** International Journal of Human-Computer Studies (Elsevier, IF 6.96);
  Human-Computer Interaction (T&F, IF ~4.5)

### Current state (2026-05-28)

§1–§5 written v0.2 (~5,200 words). Committed `da4925a4`. Language pass COMPLETE
(`forbidden_terms_cleared: true`).

**Full title:** "Muscle-Memory-Preserving Desktop Environments for Professional AEC Software Migration"

**What was written:**
- §1 Introduction: motor-learning preservation gap; KLM framing; 3 contributions
- §2 Background: KLM (Card, Moran, Newell 1980), Fitts's Law (Fitts 1954), CMD alias
  literature, HCI studies of CAD users, prior tool-switching cost work
- §3 Design Principles: MMP framework — 3 principles: (P1) verbatim command-alias mapping,
  (P2) spatial palette replication (Fitts's Law justification), (P3) F-key binding preservation
- §4 Implementation: Table 1 (18 AutoCAD aliases → new environment); IFC category/layer panel
  mapping (IfcWall/Door/Window/Slab etc.); F-key binding matrix (F3 osnap, F8 ortho, F10 polar,
  F12 archival); 3D navigation grammar (Navisworks orbit/pan/zoom equivalence); BCF 3.0 issue
  creation workflow; §4.6 IFC archival data flow
- §5 User Study: planned study protocol — within-subjects design; 4 AEC tasks; measures:
  task-completion time, command-error rate, NASA-TLX, qualitative interview; comparative
  condition: blank-slate interface

### Remaining blockers

1. **§6 Results — pending user study execution.** Participants: AEC professionals with ≥3
   years AutoCAD/Revit daily use. Measures per §5 protocol.

2. **§7 Discussion (partial).** Design rationale written. Quantitative H₁/H₂/H₃ confirmation
   and limitations section need user study data to complete.

3. **ORCID IDs** for all three authors (operator action).

---

## Execution order — completed vs. remaining

### COMPLETE (2026-05-27 to 2026-05-28)
- ✅ J1 language pass — `forbidden_terms_cleared: true`
- ✅ J2 language pass — `forbidden_terms_cleared: true`
- ✅ J3 full writing pass (~7,800 words) + language pass
- ✅ J4 §1–§3 + §6–§7 writing pass (~4,800 words) + language pass
- ✅ J6 §1–§5 writing pass (~5,200 words) + language pass
- ✅ J1 §7.0 OLS preliminary regressions (Model A + B, Phase 22 data)
- ✅ F6 partial forest plot produced (`work/figures/F6-ols-coefficients.png`)
- ✅ OLS regression script committed (`work/run-j1-ols.py`)
- ✅ Outbox to project-gis: J3 AEC nightly build coverage metrics (2026-05-28)
- ✅ F1–F5 confirmed ready at project-gis (commit 59e28780)

### NEXT — data-gated
1. **When project-gis returns AEC nightly build data:** Fill J3 §6 Results with H3 coverage
   metrics → J3 submission-ready (modulo ORCID IDs).
2. **Phase 24B (Kontur population join + O-D data):** Execute J1 §7.2 primary specification
   (`log[od_work] ~ tier + log[pop_150km]`) → update F6 → J1 submission-ready.
3. **Benchmark data (J4):** Fill §4 Implementation + §5 Evaluation with tunnel/rekey/policy-change
   measurements → J4 submission-ready.
4. **User study execution (J6):** Fill §6 Results + §7 Discussion quantitative section → J6 submission-ready.

### NEXT — independent (can do any time)
5. **J1 permutation test:** Write `sim-tier-permutation.py` (shuffle T1/T2/T3 labels 10,000 times,
   one-tailed p-value). Cluster coordinates at `work/clusters-ols.csv`.
6. **J2 ASPLOS short version:** Separate trimming session → ~6,000 words, 2-column ACM format.
   Create `JOURNAL-trustworthy-systems-v0.1-asplos.md`. Cut: §2 Background, §4.2–§4.3, §6 moonshot-toolkit.
7. **J2 Bench #9 re-run:** Coordinate via project-system outbox; quiet VM, load avg < 1.0.
8. **All papers: ORCID IDs** — operator action required for all three authors.

### HOLD
9. **J5:** Hold until J2 submitted.

---

## Pre-submission universal checklist (per journal-artifact-discipline.md)

All 6 papers need before any submission:
- [ ] `forbidden_terms_cleared: true` (language pass complete)
- [ ] ORCID IDs for all three authors
- [ ] `word_count_body` updated to actual count
- [ ] All 22 mandatory structural sections populated (see rules file §Mandatory 22)
- [ ] AI disclosure statement per COPE 2024 in §18 of each manuscript
- [ ] Target journal impact factor confirmed; `impact_factor` field filled
- [ ] `submission_status` updated when actually submitted
