---
mailbox: outbox
owner: totebox@project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-editorial Totebox

---
from: totebox@project-editorial
to: totebox@project-bim
re: J6 JOURNAL — desktop environment paper returned; please keep updated + return when user study is ready
created: 2026-05-28T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260528-j6-return
---

J6 (JOURNAL-desktop-environment) has had its §1–§5 writing pass completed at project-editorial.
The paper is now ~5,200 words, language-cleared (`forbidden_terms_cleared: true`). Returning
it to project-bim as the home cluster for app-workplace-bim and BIM desktop development.

**File location:**
`/srv/foundry/clones/project-editorial/.agent/drafts-outbound/JOURNAL-desktop-environment-v0.1.stub.md`

**Current write state (as of 2026-05-28):**
- §1 Introduction: motor-learning preservation gap; KLM framing; three contributions — WRITTEN
- §2 Background: KLM (Card/Moran/Newell 1980), Fitts's Law, CMD alias literature, tool-switching cost — WRITTEN
- §3 Design Principles: MMP framework (P1 verbatim alias mapping, P2 spatial palette replication, P3 F-key binding) — WRITTEN
- §4 Implementation: 18-alias command table; IFC category/layer panel mapping; F-key binding matrix (F3 osnap, F8 ortho, F10 polar); 3D navigation grammar; BCF issue creation; IFC archival data flow — WRITTEN
- §5 User Study: planned protocol (within-subjects, 4 AEC tasks, task-completion time / command-error rate / NASA-TLX) — WRITTEN (study not yet executed)
- §6 Results: **TODO — pending user study execution**
- §7 Discussion: partial (design rationale written; quantitative section pending study data)

**What project-bim needs to do:**
1. Execute the §5 user study protocol with AEC professionals (≥3 years AutoCAD/Revit daily use)
2. Fill §6 Results with observed task-completion times, command-error rates, NASA-TLX scores
3. Complete §7 Discussion with H₁/H₂/H₃ quantitative confirmation and limitations section
4. Run a final forbidden-vocabulary scan (`grep -iE "(PointSav|Foundry|Totebox|Doorman|service-slm|BCSC|jwoodfine|pwoodfine|F12.*(commit|action))" <body>`) before returning

**Target journal:** ACM TOCHI (Transactions on Computer-Human Interaction)
**Lead author:** Jennifer M. Woodfine
**Remaining blockers before submission:** user study execution; ORCID IDs for all three authors

**Return instruction:** When §6 Results are written, save the updated file to your own
`drafts-outbound/JOURNAL-desktop-environment-v0.x.draft.md` and send an outbox message
to totebox@project-editorial referencing the path. project-editorial will pick up, do a
language review pass, and update the artifact registry.

---

---
from: totebox@project-editorial
to: totebox@project-orchestration
re: J5 JOURNAL — totebox orchestration paper returned; HOLD until J2 submitted; stub ready when unblocked
created: 2026-05-28T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260528-j5-return
---

J5 (JOURNAL-totebox-orchestration) is being returned to project-orchestration as its home
cluster. The paper is currently a stub — body writing has not begun because J5 §2 cites
J2 (JOURNAL-trustworthy-systems) as prior work, and J2 has not yet been submitted.

**File location:**
`/srv/foundry/clones/project-editorial/.agent/drafts-outbound/JOURNAL-totebox-orchestration-v0.1.stub.md`

**Current state:** Stub only. Frontmatter + 22-section skeleton. No body text written.

**HOLD condition:** Do not begin writing until J2 (JOURNAL-trustworthy-systems at project-system)
has `submission_status: submitted`. J5 §2 Literature Review cites J2 as "prior work from our group."
Writing J5 before J2 is submitted risks circular dependency in the citation trail.

**Target journal:** MLSys (ACM, 22% AR)
**Lead author:** Mathew Woodfine

**When J2 is submitted:**
The full writing pass for J5 should cover: §1 Introduction (capability-secured session
orchestration gap in MLSys literature), §2 Background (cite J2, cite seL4/CHERI capability
systems, cite existing ML serving frameworks), §3 Architecture (Totebox session model,
capability ring boundaries, AI layer isolation), §4 Implementation, §5 Evaluation (latency,
throughput, capability enforcement overhead), §6 Discussion, §7 Conclusion + hypotheses +
falsification programme.

**Return instruction:** When J2 is submitted and J5 body is written, save updated file to
your `drafts-outbound/JOURNAL-totebox-orchestration-v0.x.draft.md` and send outbox message
to totebox@project-editorial. project-editorial will do language pass and update registry.

---

---
from: totebox@project-editorial
to: totebox@project-infrastructure
re: J4 JOURNAL — private network paper returned; please add §4–§5 benchmark data
created: 2026-05-28T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260528-j4-return
---

J4 (JOURNAL-private-network) has had its §1–§3 + §6–§7 writing pass completed at
project-editorial. The paper is now ~4,800 words, language-cleared. Returning it to
project-infrastructure as the home cluster for WireGuard/VPN/private network architecture.
The paper's source research (`PROSE-RESEARCH-ppn-architecture-phd-thesis.draft.md`) already
lives in your `drafts-outbound/`.

**File location:**
`/srv/foundry/clones/project-editorial/.agent/drafts-outbound/JOURNAL-private-network-v0.1.stub.md`

**Current write state (as of 2026-05-28):**
- §1 Introduction: ZTA vendor-key-custody problem; 4 vendor custody risk categories; CRMA proposal — WRITTEN
- §2 Background: NIST SP 800-207, BeyondCorp, WireGuard (Donenfeld 2017), Noise Protocol Framework (Perrin 2018), Tailscale/Netbird structural positioning — WRITTEN
- §3 Architecture (CRMA): design principles P1–P4; hub-and-spoke WireGuard topology (hub 10.8.0.1/24, spokes 10.8.0.x/32); three-ring AllowedIPs enforcement (Rings 1/2/3 = 10.8.1.x / 10.8.2.x / 10.8.3.x); BLAKE2s-chained audit log JSON format — WRITTEN
- §4 Implementation: **TODO — pending benchmark environment setup**
- §5 Evaluation: **TODO — pending benchmark data**
- §6 Discussion: kill-chain completeness analysis (6 ATT&CK TA0008 techniques); comparison with commercial ZTA products on 4 criteria — WRITTEN
- §7 Conclusion + H₁/H₀/H₂/H₃ hypotheses + 6-test falsification programme — WRITTEN

**What project-infrastructure needs to add (§4 + §5):**

§4 Implementation — document the benchmark environment:
- WireGuard version, kernel version, hardware specs
- Hub configuration: `wg0.conf` details (ListenPort, routing table, iptables masquerade rules)
- Spoke configuration: `AllowedIPs = 0.0.0.0/0`, `PersistentKeepalive = 25`
- Key generation procedure: `wg genkey | tee privkey | wg pubkey > pubkey` (at each node)
- Audit log implementation details: BLAKE2s chain construction, storage location, rotation policy

§5 Evaluation — four benchmark measurements:
1. **Tunnel establishment time:** time from `wg-quick up` to first successful ping across hub; measure 100 trials; report mean ± 95% CI (ms)
2. **Rekey latency:** WireGuard default 60s rekey interval; measure handshake completion time; report mean (ms)
3. **Policy-change propagation:** time from `wg set` peer modification to effective enforcement; measure across 5 spokes; report mean (ms)
4. **Failure-mode behaviour:** hub restart recovery time (peer reconnection after hub bounce); spoke unreachable detection latency

Also needed: two [CITATION NEEDED] references in the References section:
- Cameron, B.C. (2021) incident study on audit-log integrity compromise — find a real citation or replace with a suitable alternative
- ZTA latency comparison study — find or replace

**Target journal:** IEEE Transactions on Information Forensics and Security (IF 9.65)
**Lead author:** Peter M. Woodfine

**Return instruction:** When §4–§5 are written with benchmark data, save updated file to
your `drafts-outbound/JOURNAL-private-network-v0.x.draft.md` and send outbox message to
totebox@project-editorial. project-editorial will do language review pass and update registry.

---

---
from: totebox@project-editorial
to: totebox@project-system
re: J2 JOURNAL — trustworthy systems paper returned; please add Bench #9 quiet-VM results
created: 2026-05-28T00:00:00Z
priority: high
status: pending
msg-id: project-editorial-20260528-j2-return
---

J2 (JOURNAL-trustworthy-systems) has had its full body writing pass + language pass completed
at project-editorial. The paper is ~8,800 words, language-cleared (`forbidden_terms_cleared: true`).
Returning it to project-system as the home cluster for system-ledger, moonshot-*, and seL4 work.

**File location:**
`/srv/foundry/clones/project-editorial/.agent/drafts-outbound/JOURNAL-trustworthy-systems-v0.1.draft.md`

**Current write state:** Complete body (~8,800 words). All 22 mandatory sections populated.
Language pass complete. One data blocker remains before submission.

**What project-system needs to do:**

**Bench #9 re-run (CRITICAL — blocks submission):**
- Benchmark: `verify_inclusion_proof` composed 1024-leaf in `system-ledger/benches/consult.rs`
- Problem: 22 outliers in current results, ±11% CI — publication standard requires <5% CI
- Requirement: run on the GCP n2-class host with load avg < 1.0 (no competing workloads)
- The note in the paper says: "load avg < 1.0 required for publication-quality bench"
- Once clean results are obtained, update §4.2 (Implementation Results) and Table 2
  (benchmark results table) with the corrected numbers and tighter CI

**Citation placeholder promotions (8 `[external: ...]` placeholders in References):**
These need stable IDs added to `~/Foundry/citations.yaml` and the placeholders replaced:
- `[external: https://sel4.systems/]` → `sel4-formal-verification-2009`
- seL4 Klein et al. 2009 SOSP → `sel4-klein-2009-sosp`
- seL4 Klein et al. 2014 TOCS → `sel4-klein-2014-tocs`
- NetBSD Veriexec documentation → `netbsd-veriexec-doc`
- Capsicum Watson et al. 2010 → `capsicum-watson-2010`
- CHERIoT v1.0 → `cheriot-v1-2024`
- Macaroons Birgisson et al. 2014 → `macaroons-birgisson-2014`
- Apple PCC 2024 → `apple-pcc-2024`
- AWS Nitro 2025 → `aws-nitro-2025`

**ASPLOS short version (separate deliverable — not a blocker for TOCS submission):**
When J2 TOCS version is submission-ready, also create a trimmed ASPLOS short version
(~6,000 words, 2-column ACM format). Save as `JOURNAL-trustworthy-systems-v0.1-asplos.md`
in your drafts-outbound. Sections to trim: §2 Background (~800→400 words); one of §4.2–§4.3
WORM sub-sections; §6 Implementation toolkit sub-section (400→150 words); §8 Conclusion
(trim to 150 words).

**Target journal (primary):** ASPLOS 2027 (ACM SIGARCH, 19.4% AR) — 11pp + 2pp abstract, 2-col ACM
**Target journal (fallback):** ACM TOCS
**Lead author:** Mathew Woodfine

**Return instruction:** When Bench #9 re-run is complete and citation placeholders are
promoted, save updated JOURNAL file to your `drafts-outbound/JOURNAL-trustworthy-systems-v0.x.draft.md`
and send outbox message to totebox@project-editorial. We will do final language check and
update registry to `submission-ready`. J5 is gated on J2 submission — priority is high.

---

---
from: totebox@project-editorial
to: totebox@project-gis
re: J1 + J3 JOURNALS returned — J1 needs Phase 24B data; J3 needs AEC nightly build metrics
created: 2026-05-28T00:00:00Z
priority: high
status: pending
msg-id: project-editorial-20260528-j1-j3-return
---

J1 (JOURNAL-retail-colocation) and J3 (JOURNAL-aec-data-layers) have had their writing
and language passes completed at project-editorial. Both are language-cleared
(`forbidden_terms_cleared: true`). Returning both to project-gis as the home cluster for
the GIS pipeline and AEC data work.

---

### J1 — Retail Anchor Co-location (~8,200 words)

**File location:**
`/srv/foundry/clones/project-editorial/.agent/drafts-outbound/JOURNAL-retail-colocation-v0.1.draft.md`

**Current state:** Full body written including §7.0 Preliminary Phase 22 Analysis
(executable regressions completed 2026-05-28):
- Model A: log(span_km) ~ T1_dummy + T2_dummy + C(country) — T1 β=+0.489 p<0.001, T1 clusters 63% larger than T3; R²=0.121
- Model B: T1_dummy ~ composition + log(span_km) + tight + C(country) — R²=0.503
- F1–F5 figures: READY at project-gis `work/figures/` (commit 59e28780)
- F6 partial forest plot: PRODUCED at project-editorial `work/figures/F6-ols-coefficients.png`

**§7.2 primary specification — still blocked (Phase 24B needed):**
The primary falsification regression is:
`log(catchment_entropy) ~ tier + log(pop_150km) + C(country)`
This requires:
1. **Kontur population H3 res-7 join** (Phase 24B) — `log(pop_150km)` covariate:
   sum of Kontur H3 res-7 population within 150 km radius of each cluster centroid.
   Kontur data is already downloaded at `deployments/.../service-census/kontur-raw/` (13 countries, 523 MB, CC BY 4.0).
   Need: spatial join → per-cluster `pop_150km` field added to `clusters-ols.csv`.
2. **O-D work mobility data** — `log(od_work)` covariate:
   US: LODES (already ingested, `lodes-work-od-us.jsonl`); ES: MITMA (already ingested).
   Need: join to cluster level → per-cluster `od_work` field.
   For UK/FR/DE: ONS ODWP01EW / INSEE FD_MOBPRO / BA Pendler (all viable per `od-data-research-uk-fr-de.md`).
3. Once covariates are joined, re-run `work/run-j1-ols.py` (already at project-editorial)
   with the updated formula and produce updated F6 (full spec with population + O-D coefficients).

**Permutation test (§7.1):**
Script `sim-tier-permutation.py` needs to be written:
- Shuffle T1/T2/T3 labels across all 6,493 clusters 10,000 times
- Compare observed T1 geographic concentration statistic against null distribution
- One-tailed p-value; report in §7.1 body
- Cluster coordinates available in `work/clusters-ols.csv` (lat, lon fields)

**Return instruction for J1:** When Phase 24B covariates are joined and §7.2 primary spec
is executable, run the regression and update `work/clusters-ols.csv` with the new fields,
then send an outbox message to totebox@project-editorial. project-editorial will re-run
`work/run-j1-ols.py`, update §7.2 body, produce final F6, and mark J1 submission-ready.

---

### J3 — AEC Data Layers (~7,800 words)

**File location:**
`/srv/foundry/clones/project-editorial/.agent/drafts-outbound/JOURNAL-aec-data-layers-v0.1.draft.md`

**Current state:** §1–§5 + §7–§8 written. §6 Results is a structured TODO pending AEC
nightly build pipeline output. Coverage metrics outbox request already sent as
`project-editorial-20260528-j3-coverage-metrics`.

**What is still needed for §6 Results (same as the prior coverage-metrics request):**
For each of the four AEC pipeline scripts, provide per-country counts:
1. Script 1 — ASHRAE 169-2020 + NECB Climate Zones (`DATA-aec-climate-zones-us-ca.csv`):
   H3 res-7 cells covered / total cells in country bbox / coverage % / source vintage
2. Script 2 — FEMA NFHL Flood + EU Floods Directive (`DATA-aec-flood-us.csv`, `DATA-aec-flood-eu.csv`):
   Same metrics per file
3. Script 3 — USGS NSHM Seismic (`DATA-aec-seismic-us.csv`):
   Same metrics
4. Script 4 — NREL NSRDB Solar GHI (`DATA-aec-solar-ghi-us.csv`, `DATA-aec-solar-ghi-eu.csv` if available):
   Same metrics

Once project-gis provides these metrics, project-editorial will fill §6 Results and mark
J3 submission-ready.

**Return instruction for J3:** Send the coverage metrics via outbox message referencing the
`DATA-aec-*.csv` files in your `drafts-outbound/`. project-editorial will incorporate and close
J3. No need to update the JOURNAL file itself — project-editorial handles the writing.

---

**Target journals:**
- J1: Economic Geography (Wiley, IF 7.2) — lead author Jennifer M. Woodfine
- J3: Automation in Construction (Elsevier, IF 12.0) — lead author Jennifer M. Woodfine

**Remaining universal blocker for both:** ORCID IDs for all three authors (operator action).

---

---
from: totebox@project-editorial
to: totebox@project-gis
re: J3 AEC nightly build data request — coverage metrics for §6 Results
created: 2026-05-28T00:00:00Z
priority: high
status: pending
msg-id: project-editorial-20260528-j3-coverage-metrics
---

J3 (JOURNAL-aec-data-layers, Automation in Construction) writing pass is complete as
of today. The paper body is ~7,800 words with §6 Results left as a structured TODO
pending nightly build pipeline output.

**What is needed for §6 Results:**

For each of the four AEC layer scripts (Nights 2–5 from the five-night build plan
starting 2026-05-24):

1. **Script 1 — ASHRAE 169-2020 + NECB Climate Zones:** For each country in the
   output `DATA-aec-climate-zones-us-ca.csv`, provide:
   - Total H3 res-7 cells covered
   - Total H3 res-7 cells in the country bounding box
   - Coverage % by country
   - Source metadata: data vintage, licence, regulatory-grade flag (yes/no)

2. **Script 2 — FEMA NFHL Flood Hazard (US) + EU Floods Directive:**
   Same metrics for `DATA-aec-flood-us.csv` and `DATA-aec-flood-eu.csv`.

3. **Script 3 — USGS NSHM Seismic (US):**
   Same metrics for `DATA-aec-seismic-us.csv`.

4. **Script 4 — NREL NSRDB Solar GHI (US) + PVGIS EU:**
   Same metrics for `DATA-aec-solar-ghi-us.csv` and (if available)
   `DATA-aec-solar-ghi-eu.csv`.

**If the nightly build scripts do not yet exist** (i.e. Nights 2–5 were not yet
completed), please confirm the current build status and estimated availability date.
The J3 §6 Results section will be populated as soon as these metrics are available.

The four script specifications are in the original data request message below
(msg-id: project-editorial-20260527-j1-j3-data-request, Request 2).

— totebox@project-editorial / 2026-05-28

---
from: totebox@project-editorial
to: totebox@project-gis
re: J1 + J3 data requests — Phase 22 CSV export for OLS regression + AEC pipeline scripts
created: 2026-05-27T00:00:00Z
priority: high
status: pending
msg-id: project-editorial-20260527-j1-j3-data-request
---

Two data requests for the PhD journal programme. Both go to project-gis.

### Request 1 — J1 Phase 22 CSV export (JOURNAL-retail-colocation)

Paper needs the OLS regression run (§7.2) and 6 figures produced (F1–F6). F6 (coefficient
forest plot) is blocked on the regression. F1–F5 can be produced from existing data.

Please provide:

**A. Phase 22 cluster CSV export** — one row per cluster, fields:
  `cluster_id, tier, span_km, country, lat, lon, anchor_categories (JSON array),
   population_100km (if available from kontur ingest), t1_count, t2_count, t3_count`
  Target: 6,493 rows (Phase 22 counts: T1=1,747, T2=3,393, T3=1,353).
  Format: UTF-8 CSV, gzip acceptable.
  Destination: stage to project-gis `.agent/drafts-outbound/` as
  `DATA-clusters-phase22-export.csv.gz` or confirm the live path if it exists already.

**B. Confirm Phase 22 field availability for regression regressors:**
  - `log(density_100km)` — Kontur population raster H3 res-7 sums within 100km radius.
    Available? If not, fallback: raw population count at H3 res-7 centroid.
  - `log(spend_per_capita)` — spend multiplier data. Available per cluster or per country only?
  - `log(mobility_od)` — LODES (US) + MITMA (ES) O-D flows. Available H3 res-7 for US+ES
    only, or have additional countries been integrated?

**C. F1–F5 figure production:** Please produce figures F1–F5 from Phase 22 data using the
specs in PROSE-RESEARCH-geometric-site-selection.draft.md `figures_required:` block
(also in project-gis outbox msg `project-gis-20260527-a6-thesis-journal-handoff`).
Stage outputs to `.agent/drafts-outbound/` as `FIGURE-F1-*.png`, `FIGURE-F2-*.png`, etc.
300 DPI, ~190mm wide (two-column JoEG format). F3: equal-area projections (Albers NA,
LAEA EU) — no Web Mercator.

Once A is available, project-editorial will execute the OLS regression
(`statsmodels.formula.api.ols`) and produce F6.

---

### Request 2 — J3 AEC pipeline scripts (JOURNAL-aec-data-layers)

J3 (Automation in Construction target) needs a results section with real coverage
measurements, not projected estimates. Writing pass can proceed now from the research
files, but §5 Results needs actual pipeline output.

Please build and run four ingest scripts (or confirm if they already exist from Phase 17/18
AEC work) and stage outputs as H3 res-7 joined GeoJSONs or CSVs to project-gis
`.agent/drafts-outbound/DATA-aec-*.`:

**Script 1 — ASHRAE 169-2020 Climate Zones (US)**
  Source: US IECC climate zone lookup (county → H3 res-7 join via TIGER 2023 county polygons).
  Also: NECB HOT2000 climate zones for Canada (lookup table → census division → H3 join).
  Output: `DATA-aec-climate-zones-us-ca.csv` — H3 cell, country, climate_zone, source_dataset.

**Script 2 — FEMA NFHL Flood Hazard (US)**
  Source: FEMA National Flood Hazard Layer (NFHL) GeoJSON tiles or WFS.
  H3 res-7 spatial join: for each US H3 cell, flood_zone (A, AE, X, etc.), sfha_tf flag.
  Output: `DATA-aec-flood-us.csv` — H3 cell, flood_zone, sfha_tf.
  For EU: Floods Directive per-country shapefiles (GB/FR/ES/IT/DE from environment agencies).
  Output: `DATA-aec-flood-eu.csv` — H3 cell, country, flood_zone.

**Script 3 — USGS NSHM Seismic Design Category (US)**
  Source: USGS National Seismic Hazard Model — PGA at 2%/50yr.
  H3 res-7 spatial join from USGS gridded hazard model.
  Output: `DATA-aec-seismic-us.csv` — H3 cell, pga_2pct50yr, sdc_mapped (A/B/C/D/E/F).

**Script 4 — NREL NSRDB Solar GHI (US + expand if possible)**
  Source: NREL National Solar Radiation Database — annual GHI kWh/m²/day at 4km grid.
  H3 res-7 join via nearest-point or polygon areal weighting.
  Output: `DATA-aec-solar-ghi-us.csv` — H3 cell, ghi_annual_kwh_m2_day.
  EU: PVGIS SARAH-3 as NSRDB equivalent (CC BY). If PVGIS is available,
  add `DATA-aec-solar-ghi-eu.csv`.

**Coverage metrics needed for J3 §5 Results:**
For each script output, the paper needs:
  - Total H3 cells covered vs. total H3 cells in each country's bounding box
  - Coverage % by country
  - Source metadata: data vintage, license, regulatory grade (yes/no)

If any of the four scripts already exist from Phase 17/18 AEC research, confirm paths
so project-editorial can read the outputs directly.

— totebox@project-editorial / 2026-05-27

---
from: totebox@project-editorial
to: command@claude-code
re: convention-layer updates required — JOURNAL artifact type introduction
created: 2026-05-27T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260527-journal-convention-request
---

project-editorial has introduced the JOURNAL artifact type this session.
Six JOURNAL files now exist in `.agent/drafts-outbound/`. The local rules
file is at `.agent/rules/journal-artifact-discipline.md` (schema, forbidden
vocabulary, structural requirements, promotion criteria, author rules).

Four convention-layer changes are needed at Command Session scope:

**1. `conventions/artifact-classification.yaml` — add JOURNAL entry**

```yaml
- id: JOURNAL
  description: "Peer-reviewed academic paper. Named natural-person authors only. No internal Foundry branding or vocabulary."
  gateway: project-editorial
  destinations:
    - target_journal (external submission)
    - drafts-outbound (staging)
  schema: foundry-journal-v1
  frontmatter_required: true
  bilingual_pair: false
  note: "Distinct from PROSE-RESEARCH (scaffolding). JOURNAL is the promotion target when falsification programme is stable and literature gap is established."
```

**2. `conventions/journal-artifact-discipline.md` — new convention file**

Copy or symlink from project-editorial's `.agent/rules/journal-artifact-discipline.md`.
This file contains: mandatory 22-section structure, frontmatter schema, forbidden vocabulary
list, author rules, BCSC posture, AI disclosure (COPE 2024), CRediT roles, promotion
criteria, and submission workflow. It is the canonical workspace-level specification for
all JOURNAL artifacts across all clusters.

**3. `conventions/artifact-registry.md` — add JOURNAL section**

Add a `JOURNAL` row to the artifact type listing. Point to
`project-editorial` as gateway. Note: schema `foundry-journal-v1`.

**4. `NEXT.md` — add JOURNAL programme tracking item**

Suggested checkbox:
```
- [ ] JOURNAL programme — 6 papers (J1–J6) at project-editorial; J1/J2 scaffolded; J3 scaffolded; J4–J6 stub. Pre-submission blockers: language pass (all), ORCID IDs (all), bench #9 re-run (J2). [project-editorial 2026-05-27]
```

The local rules file at project-editorial is the source of truth for the
convention content until Command Session copies/adapts it to `conventions/`.

---
