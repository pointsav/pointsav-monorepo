---
artifact: brief
name: BRIEF-journal-phd-programme
status: active
version: "0.1"
created: 2026-05-27
owner: totebox@project-editorial
---

# PhD-Level Journal Programme — Next-Level Plan

Six JOURNAL manuscripts on the `foundry-journal-v1` schema. All target the top-3 venues
in their field. All three authors (Jennifer M., Peter M., Mathew Woodfine) on every paper
for CRediT spreading. Double-blind anonymisation pass = `forbidden_terms_cleared` pass.

Rules: `.agent/rules/journal-artifact-discipline.md`
Registry: `.agent/rules/artifact-registry.md` §J

---

## Status banner (2026-05-27)

| ID | File | Body state | Blocked on | Next action |
|----|------|------------|------------|-------------|
| J1 | JOURNAL-retail-colocation-v0.1.draft.md | Complete (~7,800w) | OLS regression data (project-gis) | Send data request → execute regression → 6 figures |
| J2 | JOURNAL-trustworthy-systems-v0.1.draft.md | Complete (~8,800w) | Language pass (25+ violations) | Execute language pass this session |
| J3 | JOURNAL-aec-data-layers-v0.1.draft.md | Scaffold (0w body) | Writing pass + pipeline data | Begin §1–§3 writing now from research files |
| J4 | JOURNAL-private-network-v0.1.stub.md | Stub (0w) | Writing pass | Begin §1–§2 writing now |
| J5 | JOURNAL-totebox-orchestration-v0.1.stub.md | Stub (0w) | J2 must be under-review first | HOLD |
| J6 | JOURNAL-desktop-environment-v0.1.stub.md | Stub (0w) | Writing pass | Begin §1/§3/§5 writing with BIM docs |

---

## J1 — Retail Anchor Co-location (Economic Geography)

**Primary:** Economic Geography (Wiley-Blackwell, IF 7.2)
**Alternates:** Journal of Economic Geography (OUP, IF 6.3); Regional Studies (T&F, IF 5.54)

### Current state

Body is complete — Abstract, §1–§8 written from PROSE-RESEARCH v0.4.1. The `word_count_body: 0`
frontmatter field is stale; actual word count is ~7,800. Body needs updating.

### Blockers

1. **§7.2 OLS regression not executed.** Requires Phase 22 cluster CSV (6,493 rows, 13 countries)
   from project-gis. Model: `log(T1_share) = α + β₁·log(density_100km) + β₂·log(spend_per_capita)
   + β₃·log(mobility_od) + Σδ_c·country_c + ε`. Country fixed effects. Results go into §7.2 body
   and produce F6 (forest plot).

2. **Permutation test (§7.1) not implemented.** Spatial random reassignment test: shuffle T1/T2/T3
   labels across all 6,493 clusters 10,000 times; compare observed T1 geographic concentration
   statistic against null distribution. One-tailed p-value. Requires cluster coordinates from
   Phase 22 CSV.

3. **6 must-have figures not produced** (F1–F6). F1–F5 can be produced from existing Phase 22
   data now. F6 blocked pending regression.

4. **Language pass** not yet done (`forbidden_terms_cleared: false`). Body has minimal internal
   vocabulary — quick pass expected.

5. **`word_count_body: 0` frontmatter** — update to actual count after language pass.

### Execution order

1. Send outbox message to project-gis: Phase 22 CSV export request + figure-production script
   stubs (F1–F5 can be produced immediately from existing data) — DONE in this session's outbox.
2. project-gis returns CSV + confirms data schema.
3. project-editorial executes OLS regression (statsmodels) + produces F6.
4. project-editorial produces F1–F5 (matplotlib/seaborn, specifications in PROSE-RESEARCH
   frontmatter `figures_required:` block and in project-gis outbox message).
5. Language pass: scan for internal vocabulary; update `forbidden_terms_cleared: true`.
6. Update `word_count_body` frontmatter.
7. Send to operator for byline review.

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

### Current state

Body complete (~8,800 words). Strong academic content. Forbidden vocabulary scattered through
body — requires a systematic language pass.

### Forbidden vocabulary to remove (body text only; frontmatter is clean)

| Location | Forbidden term | Replace with |
|----------|----------------|-------------|
| §1.2 line 144 | "Foundry continuous-disclosure posture [conventions/bcsc-disclosure-posture.md]" | Remove the parenthetical entirely — "language per our internal research governance standards" or just delete the clause |
| §1.2 line 146 | "scaffold-coded state" | "prototype stage" or "not yet production-deployed" |
| §2.2 line 176 | "Doctrine Invention #7 [conventions/worm-ledger-design.md]" | Remove the bracketed reference; retain "making the customer's ledger externally timestamped" |
| §3.1 line 207 | "Totebox Archive" | "customer-controlled deployment instance" (throughout) |
| §4.1 (table) | `service-fs/src/http.rs`, `service-fs/src/mcp.rs` etc. | Replace file path references with "implementation module" or describe by function |
| §4.1 line 304 | "Substrate Substitution [DOCTRINE.md claim #29]" | Remove bracketed doctrine reference; retain the concept description |
| §4.2 reference | `service-fs` throughout | "the ledger server" or "the WORM ledger service" |
| §5 and elsewhere | `moonshot-kernel` | "the planned no_std capability kernel" |
| §5 and elsewhere | `moonshot-toolkit` | "the build orchestrator" (when referring to the tool) |
| §5 and elsewhere | `moonshot-database` | "the planned storage backend" |
| §6.1 | `vendor-sel4-kernel` | "the vendored seL4 kernel source tree" |
| §7.2 body | "Honest We Own It scoresheet [conventions/...]" | Remove entirely; replace with the actual property description |
| §7.2 | `ps-administrator SSH key` | "the operator's apex signing key" |
| Appendix | Research Trail section (lines 692–733) | Delete entirely — internal-only artifact |
| Throughout | `[conventions/...md]` internal path references | Remove bracketed paths; retain surrounding text |
| Throughout | `os-*` in context of product family name | Replace with "the operating system runtime family" where generic; specific product names (os-console, os-network-admin) must be removed |
| Throughout | `[external: https://...]` citation placeholders | Promote to stable citations.yaml IDs (see checklist below) |

### Citation promotions needed (8 `[external: ...]` placeholders)

| Placeholder | Stable ID to create in citations.yaml |
|-------------|--------------------------------------|
| `[external: https://sel4.systems/]` | `sel4-formal-verification-2009` |
| seL4 Klein et al. 2009 SOSP | `sel4-klein-2009-sosp` |
| seL4 2014 TOCS | `sel4-klein-2014-tocs` |
| NetBSD Veriexec documentation | `netbsd-veriexec-doc` |
| Capsicum Watson et al. 2010 | `capsicum-watson-2010` |
| CHERIoT v1.0 | `cheriot-v1-2024` |
| Macaroons Birgisson et al. 2014 | `macaroons-birgisson-2014` |
| Apple PCC (2024) | `apple-pcc-2024` |
| AWS Nitro (2025) | `aws-nitro-2025` |

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

### Current state

Frontmatter and scaffold complete. All body sections are TODO stubs. Research material is
available in two files:
- Source 1: `AEC-LAYERS-RESEARCH.md` (in project-gis `.agent/drafts-outbound/` or accessible
  via project-gis archive — contains the Phase 17/18 pipeline research and country-by-country
  data source audit)
- Source 2: `AEC-DATA-PARITY-RESEARCH.md` (in project-gis `.agent/drafts-outbound/` — contains
  parity scorecard, §7 data table)

### Writing plan (can start now)

§1 Introduction — write from research frontmatter notes_for_editor "contribution angle":
  Three contributions already drafted in frontmatter. Gap statement: no systematic comparative
  assessment of open-license AEC geospatial data across North America and Europe exists.

§2 Literature Review — AEC site analysis tools, GIS data quality literature, H3 spatial
  indexing (Brodsky 2018), OpenStreetMap quality for infrastructure analysis (Haklay 2010).

§3 Methodology — H3 res-7 pipeline; eight data layer categories; 16 countries; source
  evaluation criteria (regulatory grade, license class, spatial resolution, last-updated).

§4 Data Sources — write from AEC-LAYERS-RESEARCH.md key findings in notes_for_editor:
  US all-four-Tier-1-complete, EU floods Directive 2007/60/EC, Canada NECB HOT2000 open,
  Mexico CONABIO CC BY-NC conflict, ASCE 7 copyright, Solargis ShareAlike conflict,
  Resolve 2017 eco-regions baseline.

§5 Results — parity scorecard table from AEC-DATA-PARITY-RESEARCH.md §7 (already exists);
  copy in with proper academic framing. Quantify: "X of 8 layer categories achieve
  regulatory-grade open coverage across Y of 16 countries."

§6 Discussion — coverage gaps (Canada flood, Mexico CC BY-NC, pan-EU climate zones);
  integration pattern recommendations; limitations (static snapshot, OSM data quality).

§7–§8 Formal hypotheses + falsification programme + conclusion (write from contribution angle).

### Pipeline data needed (request to project-gis)

To populate §5 Results with real measurements (not projected estimates), need the
Phase 17/18 AEC pipeline scripts run against actual data:
- ASHRAE 169-2020 climate zones (US IECC + NECB HOT2000 for Canada)
- FEMA NFHL flood hazard H3 join
- USGS NSHM seismic design category (SDC) H3 join
- NREL NSRDB solar GHI H3 join

Send outbox message to project-gis with full script specs — DONE in this session.

---

## J4 — Zero-Trust Private Network Architecture (IEEE TIFS)

**Primary:** IEEE Transactions on Information Forensics and Security (IF 9.65)
**Alternates:** Computers & Security (Elsevier, IF 7.98);
  IEEE Transactions on Network and Service Management (IF 6.44)

### Current state

Stub only. All body sections are TODO. No blocking dependencies on other papers.

### Writing plan (can start now)

§1 Introduction — Research gap: Zero Trust Architecture (Kindervag 2010; NIST SP 800-207, 2020)
  literature focuses on access control policy, not on the data-plane implementation of private
  network overlays that carry application traffic between Zero-Trust subjects. WireGuard
  (Donenfeld 2017) has become the dominant open-source private network protocol since its
  Linux kernel merge (v5.6, March 2020), but its integration with customer-sovereign key
  management and capability-based access control has not been systematically characterised.
  Three contributions: (1) a composability model for WireGuard-based private network overlays
  and ZTA policy engines; (2) a key distribution architecture that roots WireGuard peer keys
  in a customer-controlled signing infrastructure; (3) a formal threat model showing which
  classes of lateral movement attacks the architecture prevents by construction.

§2 Background — Kindervag 2010 ZTA original; NIST SP 800-207 (Rose et al. 2020) formal spec;
  BeyondCorp (Ward and Beyer 2014); WireGuard cryptographic design (Donenfeld 2017, IEEE S&P);
  Noise Protocol Framework (Perrin 2018); existing ZTA overlay implementations (Tailscale,
  Netbird).

H₁ (formal): A private network overlay architecture that roots WireGuard peer-key distribution
in a customer-controlled signing infrastructure and enforces capability-based access policy
at the network-layer achieves lateral-movement resistance that is measurable via a kill-chain
completeness metric under a defined threat model, and achieves this without a trusted
third-party key server at runtime.

H₀ (null): WireGuard-based private networks with third-party key management do not exhibit
systematically different lateral-movement resistance compared to customer-sovereign key
management under the same threat model.

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

### Current state

Stub only. Venue confirmed as TOCHI (journal, full article). BIM muscle-memory
documentation available in `.agent/rules/bim-product-family.md` and
`app-workplace-bim/CLAUDE.md` — readable from this archive.

### Writing plan (can start now)

§1 Introduction — Research gap: professional AEC software migration studies focus on feature
  parity and training, not on motor-learning preservation. Card, Moran, Newell (1980)
  Keystroke-Level Model established that expert performance is dominated by physical-motor
  execution time, not cognitive planning time. Expert AutoCAD users have years of embedded
  command aliases (L→LINE, PL→PLINE, TR→TRIM) and spatial-motor paths to toolbars; a
  new interface that breaks these requires re-learning at the motor, not just cognitive, level.
  Three contributions: (1) formal taxonomy of muscle-memory preservation strategies for
  professional software migration; (2) implementation of three preservation mechanisms
  (alias mapping, palette layout replication, F-key binding); (3) comparative user study
  measuring task-completion time and error rate for experienced AutoCAD users on the
  new system vs. a blank-slate alternative.

§3 Design Principles — write from bim-product-family.md §"Muscle memory targets":
  Phase 1 core alias set (L, PL, C, M, CO, TR, O, F, LA), F3/F8/F10 toggle preservation,
  layer-panel-to-IFC-element-category mapping. Generalise to three principles:
  (1) Command-alias mapping verbatim; (2) Spatial-layout replication for toolbars and panels;
  (3) F-key and Ctrl- shortcut preservation.

§5 User Study — write protocol sketch: experienced AutoCAD users (≥3 years daily use),
  within-subjects design (counterbalanced), four standard AEC tasks (draw perimeter, trim
  to boundary, offset element, assign layer), measures: task-completion time (TLM-predicted
  vs. actual), command-error rate (wrong alias invocations per task), SUS satisfaction score.
  Comparative condition: blank-slate interface with same geometry engine, no alias mapping.

Source for implementation details: `.agent/rules/bim-product-family.md` §Phase 1 muscle
memory targets; app-workplace-bim CLAUDE.md Phase 1 scope.

---

## Execution order (recommended)

1. **This session:** J2 language pass (all substitutions per table above). Execute now.
2. **Coordinate project-gis:** Phase 22 CSV + F1–F5 figure production scripts + J3 AEC pipeline.
   Outbox message sent this session.
3. **Next session:** J3 §1–§5 writing pass (all research material available now).
4. **Next session:** J4 §1–§2 writing (no blocking dependencies).
5. **Next session:** J6 §1/§3/§5 writing (BIM docs available now).
6. **When project-gis returns Phase 22 CSV:** J1 OLS regression → F6 → J1 language pass → J1 submission-ready.
7. **After J2 language pass + ASPLOS reformat:** J2 submission-ready (pending Bench #9 re-run).
8. **After J3 writing + pipeline data:** J3 submission-ready.
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
