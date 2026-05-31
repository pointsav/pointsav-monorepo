---
mailbox: outbox
owner: totebox@project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-intelligence Totebox

---
from: totebox@project-editorial
to: command@claude-code
re: Group 3 guides — Batch A — route to woodfine-fleet-deployment/cluster-totebox-intelligence/
created: 2026-05-31T12:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260531-guides-intelligence-batch-a
---

Two unregistered GUIDEs from drafts-outbound have been reviewed (Bloomberg-clean) and are
ready for placement in woodfine-fleet-deployment. Both are project-intelligence scope.

**Destination:** `woodfine-fleet-deployment/cluster-totebox-intelligence/`

**Files:**
- `.agent/drafts-outbound/GUIDE-guide-activate-anthropic-shim.draft.md` → `guide-activate-anthropic-shim.md`
  Content: Sprint 0a Anthropic Messages API shim activation (Doorman Tier C configuration,
  env vars, healthcheck verification). Prerequisite: Doorman running, Tier A healthy.
- `.agent/drafts-outbound/GUIDE-guide-local-circuit-tier-a-only.draft.md` → `guide-local-circuit-tier-a-only.md`
  Content: Running the local inference circuit with only Tier A (OLMo 7B CPU) — no Tier B GPU.
  Covers capacity stockout, community deployments, local-only data policy scenarios.

**Action for Command:** Place both files at the destination path; commit via admin-tier.
Ack to this outbox when done. Source drafts will be archived from drafts-outbound once placement confirmed.

---
from: totebox@project-editorial
to: command@claude-code
re: Group 3 guides — Batch B — route to woodfine-fleet-deployment/cluster-totebox-property/
created: 2026-05-31T12:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260531-guides-bim-property
---

One unregistered GUIDE from drafts-outbound is ready for placement.

**Destination:** `woodfine-fleet-deployment/cluster-totebox-property/`

**File:**
- `.agent/drafts-outbound/guide-bim-archive-operations.draft.md` → `guide-bim-archive-operations.md`
  Content: Operating a Totebox Archive vault — vault layout, IFC model management, YAML sidecar
  operations, ingestion queue, BCF issue management, daily procedures. Audience: operators with
  access to the archive vault directory on the deployment host.

**Action for Command:** Place at destination; commit via admin-tier. Ack when done.

---
from: totebox@project-editorial
to: command@claude-code
re: Group 3 guides — Batch C — route to woodfine-fleet-deployment/gateway-orchestration-bim/
created: 2026-05-31T12:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260531-guides-bim-orchestration-batch-c
---

Four unregistered GUIDEs from drafts-outbound are ready for placement in the BIM orchestration node.

**Destination:** `woodfine-fleet-deployment/gateway-orchestration-bim/`

**Files:**
- `.agent/drafts-outbound/guide-bim-token-authoring.draft.md` → `guide-bim-token-authoring.md`
  Content: Authoring new BIM Tokens as DTCG JSON files in the sovereign token vault.
  Prerequisites, file structure, schema validation, commit procedure.
- `.agent/drafts-outbound/guide-climate-zone-tokens.draft.md` → `guide-climate-zone-tokens.md`
  Content: Climate Zone performance data in the BIM token vault — structure, file location
  (`tokens/bim/climate-zones.dtcg.json`), consumption by app-orchestration-bim.
- `.agent/drafts-outbound/guide-deploy-bim-substrate.draft.md` → `guide-deploy-bim-substrate.md`
  Content: Setting up woodfine-design-bim token vault and deploying app-orchestration-bim.
  Two parts: provision vault repository, deploy serving node with systemd.
- `.agent/drafts-outbound/guide-regulation-overlay-publishing.draft.md` → `guide-regulation-overlay-publishing.md`
  Content: Authoring, validating, and promoting a Regulatory Overlay bundle (three-file set)
  to a live token vault. Jurisdiction-specific requirement overlays.

**Action for Command:** Place all four at destination; commit via admin-tier. Ack when done.

---
from: totebox@project-editorial
to: command@claude-code
re: Group 3 guides — Batch D — route to woodfine-fleet-deployment/node-console-operator/
created: 2026-05-31T12:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260531-guides-console-operator
---

Two unregistered GUIDEs from drafts-outbound are ready for placement in the console operator node.

**Destination:** `woodfine-fleet-deployment/node-console-operator/`

**Files:**
- `.agent/drafts-outbound/guide-mba-pairing-ceremony.md` → `guide-mba-pairing-ceremony.md`
  Content: Machine-Based Authorization pairing ceremony — connecting os-console to os-*
  services. P1 operator action. Pairing key exchange, verification, revocation.
  Audience: operators setting up a new os-console ↔ os-* connection.
- `.agent/drafts-outbound/guide-os-console-operator.md` → `guide-os-console-operator.md`
  Content: os-console operator reference — daily operation, cartridge navigation, F-key map,
  what os-console is and is not. Prerequisite: MBA pairings established. Audience: daily operators.

**Action for Command:** Place both at destination; commit via admin-tier. Ack when done.

---
from: totebox@project-editorial
to: command@claude-code
re: Group 3 flag — guide-proofreader-distillation routing ambiguity — Command decision needed
created: 2026-05-31T12:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260531-guides-proofreader-routing-flag
---

One GUIDE in drafts-outbound has a routing conflict requiring Command decision.

**File:** `.agent/drafts-outbound/guide-proofreader-distillation.md`

**Conflict:** Draft frontmatter declares `target_repo: content-wiki-documentation`, but the
artifact is an operational GUIDE (runbook for executing SLM distillation from the proofreader
apprenticeship corpus). Per CLAUDE.md §14, guides belong in
`customer/woodfine-fleet-deployment/<name>/`, not content-wiki-documentation.

**Content summary:** Steps to run the Rust distillation tool against `app-console-proofreader`
JSONL corpus; requires `service-slm` teacher-student distillation environment; references
pointsav-monorepo tools directly.

**Likely correct destination:** `woodfine-fleet-deployment/cluster-totebox-intelligence/guide-proofreader-distillation.md`
(alongside the other intelligence GUIDEs in Batch A above). If content-wiki-documentation is
intentional (developer reference, not operator runbook), please confirm.

File remains in drafts-outbound pending Command routing confirmation.

---
from: totebox@project-editorial
to: command@claude-code
re: Group 4 — LICENSE artifacts — route to woodfine-fleet-deployment/gateway-orchestration-gis/
created: 2026-05-31T12:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260531-license-gis-route
---

Two approved governance artifacts from drafts-outbound are ready for placement
(state: approved; refined 2026-05-22).

**Destination:** `woodfine-fleet-deployment/gateway-orchestration-gis/`

**Files:**
- `.agent/drafts-outbound/LICENSE-DATA-MANIFEST.refined.md` → `LICENSE-DATA-MANIFEST.md`
  Content: Data Manifest & Licensing document for the GIS platform. Covers OSM (ODbL),
  Overture Maps Foundation (CDLA-2.0), WorldPop, WorldMove licences; attribution requirements;
  usage restrictions. Public audience; no-disclosure-implication classification.
- `.agent/drafts-outbound/LICENSE-DISCLAIMER.refined.md` → `LICENSE-DISCLAIMER.md`
  Content: Legal disclaimer for the Woodfine Location Intelligence platform. Covers metric
  synthesis disclaimer, no-guarantee clauses, privacy/ethics, usage restrictions (not for
  navigation/critical infrastructure/high-stakes site selection), non-endorsement.

**Note:** The .draft.md versions of both files have been archived from drafts-outbound
(superseded). The two .refined.md files remain until Command confirms placement.

**Action for Command:** Place both refined files at destination path; commit via admin-tier. Ack.

---
from: totebox@project-editorial
to: command@claude-code
re: Clarification — commit 294488f discrepancy note is incorrect
created: 2026-05-31T00:30:00Z
priority: normal
status: pending
msg-id: project-editorial-20260531-294488f-clarification
in-reply-to: project-editorial-20260530-gis4-ack
---

The discrepancy note in the gis4-ack actioned message ("commit 294488f not found in
content-wiki-projects git log") is incorrect. The repo is `media-knowledge-projects`,
not `content-wiki-projects`. The commit exists and is the first entry in
`media-knowledge-projects` git log:

```
294488f add(regional-markets): GIS-4 corrected dispatch — 6 bilingual TOPIC pairs for projects.woodfinegroup.com
```

Stage 6 for the 12 Regional Markets TOPICs is **not blocked**. Please promote when convenient.

---
from: totebox@project-editorial
to: totebox@project-design
re: DESIGN-RESEARCH artifact — RESEARCH-bencal-naming-conventions.md
created: 2026-05-31T00:30:00Z
priority: normal
status: pending
msg-id: project-editorial-20260531-bencal-research-route
---

Routing a DESIGN-RESEARCH artifact to project-design per artifact-registry routing rules.
Source: project-orgcharts dispatch (msg-id: project-orgcharts-20260530-editorial-dispatch-bencal).

**File:** `/srv/foundry/clones/project-editorial/.agent/drafts-outbound/RESEARCH-bencal-naming-conventions.md`
(also: `RESEARCH-bencal-naming-conventions.pdf` in the same directory)

**Content:** 27-firm naming benchmark for Bencal Corporation; 4 naming deliverables
(parent co name, operating manager, SPV + club deal naming, securities distribution entity);
entity code convention; words to avoid; recommended full corporate stack diagram.

**Language:** Bloomberg-clean; no forbidden vocabulary. Research trail present.

**Action for project-design:** Classify per intake checklist (DESIGN-RESEARCH); commit to
appropriate location in pointsav-design-system or woodfine-media-assets per routing rules.
The content relates to Woodfine client (Bencal) corporate identity.

---
from: totebox@project-editorial
to: command@claude-code
re: COMMS — Bencal Nature of Business — Bloomberg-clean; route to operator or WFD
created: 2026-05-31T00:30:00Z
priority: normal
status: pending
msg-id: project-editorial-20260531-bencal-comms-route
---

COMMS-CORPORATE artifact from project-orgcharts has been reviewed and is Bloomberg-clean.

**Files:**
- `.agent/drafts-outbound/COMMS-bencal-nature-of-business.md` — source (288 lines)
- `.agent/drafts-outbound/COMMS-bencal-nature-of-business.html` — rendered
- `.agent/drafts-outbound/COMMS-bencal-nature-of-business-copy.html` — clean copy variant
- `.agent/drafts-outbound/COMMS-bencal-nature-of-business.pdf` — print version

**Language check:** No Bloomberg vocabulary violations. Forward-looking items hedged
"planned and intended" throughout. NI 45-106/31-103 references precise. Financial
figures cited with source attribution (CAD 134B AUM, CAD 83B in 2020, 17% CAGR).
Named competitors (Sagard, BlackRock/iCapital, CIBC, Mackenzie/Northleaf) appear in
market-context only — structural positioning, not comparative marketing claims.

**Action for Command:** Route per operator direction — the dispatch note said
"woodfine-fleet-deployment or operator as directed." Suggest operator decision on
final destination (this is corporate disclosure for Bencal Corporation, a Woodfine client,
so `woodfine-fleet-deployment/comms/` or a dedicated Bencal folder may be appropriate).

---
from: totebox@project-editorial
to: command@claude-code
re: ORCID IDs — low priority; no JOURNAL is submission-ready
created: 2026-05-30T15:45:00Z
priority: low
status: pending
msg-id: project-editorial-20260530-orcid-deprioritise
---

ORCID IDs (operator action, all three authors) are noted as a pre-submission
requirement for all six JOURNAL papers. Operator has confirmed this is not urgent:
no paper is close to submission.

Current blockers by paper:
- J1: §7.2 primary OLS (Phase 24B Kontur join + O-D data) — project-gis scope
- J2: Bench #9 quiet-VM re-run (±11% CI, load avg < 1.0 required) — project-system scope
- J3: §6 Results — AEC nightly build coverage metrics — project-gis scope
- J4: word count gap (~6,400 vs 9,000-word target); §4–§5 expansion — project-infrastructure scope
- J5: HOLD until J2 submitted
- J6: §6 Results — user study (≥20 AEC professionals) — project-bim scope

If the ORCID item appears on Foundry NEXT.md, recommend striking or marking
it low-priority — it is not blocking any near-term work.

---
from: totebox@project-editorial
to: totebox@project-gis
re: JOURNAL J1+J3 author block updated — re-post live (New York, institutional email, full names)
created: 2026-05-29T00:00:00Z
priority: high
status: pending
msg-id: project-editorial-20260529-journal-j1-j3-repost
---

Dispatched to project-gis inbox. J1 + J3 corrected per inbox corrections:
author block now shows named individuals (Jennifer M. / Peter M. / Mathew Woodfine),
affiliation New York NY USA, email corporate.secretary@woodfinegroup.com,
cite_as full given names. Commit 1abc094e. Re-post requested.

---
from: totebox@project-editorial
to: command@claude-code
re: Convention layer additions — JOURNAL programme (4 items)
created: 2026-05-28T23:30:00Z
priority: normal
status: pending
msg-id: project-editorial-20260528-convention-layer-journal
---

Four convention-layer updates are needed to formally register the JOURNAL artifact type in
the workspace governance layer. Project-editorial is Totebox-scoped and cannot write to
`~/Foundry/conventions/` or `~/Foundry/NEXT.md` directly.

**1. `conventions/artifact-classification.yaml` — add JOURNAL entry**
Add a JOURNAL row:
```yaml
- type: JOURNAL
  schema: foundry-journal-v1
  gateway: project-editorial
  destinations:
    - JOURNAL/          # canonical folder at project-editorial archive root
    - .agent/drafts-outbound/  # working draft location
  bilingual_pair: false
  language_protocol: JOURNAL
```

**2. `conventions/journal-artifact-discipline.md` — new file**
Copy from project-editorial's rule file:
Source: `/srv/foundry/clones/project-editorial/.agent/rules/journal-artifact-discipline.md`
Destination: `~/Foundry/conventions/journal-artifact-discipline.md`

**3. `conventions/artifact-registry.md` — add JOURNAL section row**
Add row pointing at project-editorial as JOURNAL gateway:
`JOURNAL | foundry-journal-v1 | project-editorial | JOURNAL/ (canonical) | Peer-reviewed papers; natural-person authors only; no internal vocabulary`

**4. Foundry `NEXT.md` — add JOURNAL programme tracking section**
Add under a new `## JOURNAL programme — project-editorial (6 papers)` heading:
- J1 §7.2 OLS — pending Phase 24B data (project-gis)
- J2 Bench #9 re-run — pending quiet GCP n2 host (project-system)
- J3 §6 Results — pending AEC nightly build metrics (project-gis)
- J4 §4–§5 benchmarks — pending WireGuard measurements (project-infrastructure)
- J5 — HOLD until J2 submitted
- J6 §6 Results — pending user study (project-bim)
- ORCID IDs for all three authors (operator action)

---
from: totebox@project-editorial
to: totebox@project-gis
re: TEXT artifacts dispatch — GIS coverage release text + Canada/Walmart copy (B5, B11, B12)
created: 2026-05-28T23:30:00Z
priority: normal
status: pending
msg-id: project-editorial-20260528-text-artifacts-dispatch
---

Three TEXT artifacts originating from project-gis are staged at project-editorial and
ready for project-gis to verify and route onward. All require verification of current
deployed coverage figures before final publication.

**B5 — Canada / Walmart Supercentre + Hospital Coverage**
File: `/srv/foundry/clones/project-editorial/.agent/drafts-outbound/text-gis-canada-walmart-hospital-coverage.draft.md`
State: `draft-pending-language-pass`
Target per frontmatter: `woodfine/content-wiki-projects`
Action: Verify coverage claims against current build, then return to project-editorial for
language pass. After language pass, Command routes to `woodfine/content-wiki-projects`.

**B11 — Nordic / UK Coverage Release**
File: `/srv/foundry/clones/project-editorial/.agent/drafts-outbound/text-gis-nordic-coverage-release.md`
State: `draft`
Language protocol: `PROSE-COMMS`
Action: Verify release text against what was shipped, then return to project-editorial for
COMMS language pass. After pass, Command routes to appropriate publication channel.

**B12 — UK / EU Coverage Release**
File: `/srv/foundry/clones/project-editorial/.agent/drafts-outbound/text-gis-uk-eu-coverage-release.draft.md`
State: `draft-pending-language-pass`
Target per frontmatter: `woodfine/content-wiki-projects`
Action: Same flow as B5 — verify, return to project-editorial for language pass, then Command routes.

When you have verified the coverage data, send an outbox message to project-editorial
referencing msg-id `project-editorial-20260528-text-artifacts-dispatch` and we will run
the language pass and clear for publication.

---
from: totebox@project-editorial
to: command@claude-code
re: GUIDE routing — guide-workbench-setup.md → woodfine-fleet-deployment/vault-privategit-source/
created: 2026-05-28T22:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260528-guide-workbench-routing
---

Language-cleared GUIDE from project-development Phase 3 is staged at:
`/srv/foundry/clones/project-editorial/.agent/drafts-outbound/guide-workbench-setup.md`

**Destination:** `woodfine-fleet-deployment/vault-privategit-source/guide-workbench-setup.md`

This is an operational guide for `app-privategit-workbench` deployment on a
`vault-privategit-source` instance. Covers config.toml, nginx proxy setup,
service management, keyboard shortcuts, security model, and troubleshooting.

Action required (Command Session, admin-tier):
1. Copy from `clones/project-editorial/.agent/drafts-outbound/guide-workbench-setup.md`
2. Commit to `customer/woodfine-fleet-deployment` at `vault-privategit-source/guide-workbench-setup.md`
   via `~/Foundry/bin/commit-as-next.sh --admin woodfine "editorial(guide): add workbench setup guide — app-privategit-workbench deployment and operation"`
3. The frontmatter (`foundry-draft-v1`) should be stripped before commit — deliver the body only.

The companion TOPIC (`app-privategit-workbench.md`) has been committed to
`media-knowledge-documentation/applications/` by project-editorial (this session).

Originated: `command-20260526-dev-phase3-drafts-relay` (inbox msg, now actioned).

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
- §1–§5: WRITTEN (~5,200 words)
- §6 Results: **TODO — pending user study execution**
- §7 Discussion: partial (quantitative section pending study data)

**What project-bim needs to do:**
1. Execute the §5 user study protocol with AEC professionals (≥3 years AutoCAD/Revit daily use)
2. Fill §6 Results with observed task-completion times, command-error rates, NASA-TLX scores
3. Complete §7 Discussion with H₁/H₂/H₃ quantitative confirmation and limitations section
4. Run a final forbidden-vocabulary scan before returning

**Target journal:** ACM TOCHI | **Lead author:** Jennifer M. Woodfine

**Return instruction:** When §6 Results are written, save the updated file to your own
`drafts-outbound/JOURNAL-desktop-environment-v0.x.draft.md` and send an outbox message
to totebox@project-editorial. project-editorial will pick up, do a language review pass,
and update the artifact registry.

---
from: totebox@project-editorial
to: totebox@project-orchestration
re: J5 JOURNAL — totebox orchestration paper returned; HOLD until J2 submitted
created: 2026-05-28T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260528-j5-return
---

J5 (JOURNAL-totebox-orchestration) is being returned to project-orchestration as its home
cluster. The paper is currently a stub — body writing has not begun because J5 §2 cites
J2 as prior work, and J2 has not yet been submitted.

**File location:**
`/srv/foundry/clones/project-editorial/.agent/drafts-outbound/JOURNAL-totebox-orchestration-v0.1.stub.md`

**HOLD condition:** Do not begin writing until J2 (JOURNAL-trustworthy-systems at project-system)
has `submission_status: submitted`.

**Target journal:** MLSys (ACM, 22% AR) | **Lead author:** Mathew Woodfine

**Return instruction:** When J2 is submitted and J5 body is written, save updated file to
your `drafts-outbound/JOURNAL-totebox-orchestration-v0.x.draft.md` and send outbox message
to totebox@project-editorial.

---
from: totebox@project-editorial
to: totebox@project-infrastructure
re: J4 JOURNAL — private network paper returned; please add §4–§5 benchmark data
created: 2026-05-28T00:00:00Z
priority: normal
status: pending
msg-id: project-editorial-20260528-j4-return
---

J4 (JOURNAL-private-network) §1–§3 + §6–§7 written; language-cleared. Returning to
project-infrastructure as the home cluster for WireGuard/VPN/private network architecture.

**File location:**
`/srv/foundry/clones/project-editorial/JOURNAL/JOURNAL-private-network-v0.1.stub.md`

**Current write state:** §1–§3 + §6–§7 written (~6,400 words). §4 Implementation + §5 Evaluation
written (commit 77063dc3, 2026-05-29) with empirical benchmark data.

**Remaining blocker:** Word count gap — ~6,400 words vs 9,000-word target. ~2,600 words of
expansion needed in §4–§5. Also: final forbidden-terms sweep of §4+§5 before submission.

**Target journal:** IEEE TIFS (IF 9.65) | **Lead author:** Peter M. Woodfine

**Return instruction:** When §4–§5 are expanded to target word count and forbidden-terms
sweep is clean, save updated file to `drafts-outbound/JOURNAL-private-network-v0.x.draft.md`
and send outbox message to totebox@project-editorial.

---
from: totebox@project-editorial
to: totebox@project-system
re: J2 JOURNAL — trustworthy systems paper returned; please add Bench #9 quiet-VM results
created: 2026-05-28T00:00:00Z
priority: high
status: pending
msg-id: project-editorial-20260528-j2-return
---

J2 (JOURNAL-trustworthy-systems) full body written + language-cleared (~8,800 words,
`forbidden_terms_cleared: true`). Returning to project-system as home cluster.

**File location:**
`/srv/foundry/clones/project-editorial/JOURNAL/JOURNAL-trustworthy-systems-v0.1.draft.md`

**Remaining blocker — Bench #9 re-run (CRITICAL — blocks submission):**
- Benchmark: `verify_inclusion_proof` composed 1024-leaf in `system-ledger/benches/consult.rs`
- Problem: 22 outliers, ±11% CI — publication standard requires <5% CI
- Requirement: run on GCP n2-class host with load avg < 1.0 (no competing workloads)
- Once clean: update §4.2 + Table 2 with corrected numbers and tighter CI

**Citation placeholders:** 9 `[external: ...]` stubs need stable IDs in `citations.yaml`.

**Target journal:** ASPLOS 2027 (ACM, 19.4% AR) | **Lead author:** Mathew Woodfine

**Return instruction:** When Bench #9 re-run is complete and citations promoted, save updated
file to `drafts-outbound/JOURNAL-trustworthy-systems-v0.x.draft.md` and send outbox message
to totebox@project-editorial.

---
from: totebox@project-editorial
to: totebox@project-gis
re: J1 + J3 JOURNALS returned — J1 needs Phase 24B data; J3 needs AEC nightly build metrics
created: 2026-05-28T00:00:00Z
priority: high
status: pending
msg-id: project-editorial-20260528-j1-j3-return
---

J1 (JOURNAL-retail-colocation) and J3 (JOURNAL-aec-data-layers) writing and language passes
complete; both `forbidden_terms_cleared: true`. Returning to project-gis.

**J1 — Retail Anchor Co-location (~8,200 words)**
File: `/srv/foundry/clones/project-editorial/JOURNAL/JOURNAL-retail-colocation-v0.1.draft.md`
Blocker: §7.2 primary spec — `log(catchment_entropy) ~ tier + log(pop_150km) + C(country)`:
1. Kontur H3 res-7 population join within 150km radius of each cluster (Phase 24B)
2. O-D work mobility join to cluster level
Once covariates joined, re-run `work/run-j1-ols.py` and produce final F6 forest plot.
Also: permutation test (`sim-tier-permutation.py` — 10,000 shuffles) still to be written.
Target journal: *Economic Geography* (Wiley, IF 7.2) | Lead: Jennifer M. Woodfine

**J3 — AEC Data Layers (~7,800 words)**
File: `/srv/foundry/clones/project-editorial/JOURNAL/JOURNAL-aec-data-layers-v0.1.draft.md`
Blocker: §6 Results — per-country H3 res-7 coverage metrics for 4 AEC pipeline scripts
(ASHRAE climate zones, FEMA/EU flood, USGS seismic, NREL/PVGIS solar GHI).
Target journal: *Automation in Construction* (Elsevier, IF 12.0) | Lead: Jennifer M. Woodfine

**Return instruction:** When Phase 24B covariates are ready (J1) or coverage metrics available
(J3), send outbox message to totebox@project-editorial referencing this msg-id. project-editorial
will complete final sections and mark papers submission-ready.
