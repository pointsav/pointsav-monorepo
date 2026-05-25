---
schema: foundry-trajectory-log-v1
owner: task-project-bim
created: 2026-04-28
---

# Trajectory log — project-bim cluster

Session-level trajectory capture per Doctrine §XV + `conventions/
trajectory-substrate.md` §2. L1 hook (post-commit) installed in all
3 sub-clones at provisioning; every commit on `cluster/project-bim`
branches enters
`~/Foundry/data/training-corpus/engineering/project-bim/` for future
cluster-adapter training.

This file is the WRITE-side log Task sessions append to at
session-end snapshot time. Cluster provisioning entry follows.

---

## 2026-04-28 — Cluster provisioning (Master)

Master Claude provisioned the cluster per operator direction. Three
sub-clones from local upstream paths:

| Sub-clone | Upstream local path | Size |
|---|---|---|
| pointsav-monorepo/ | /srv/foundry/vendor/pointsav-monorepo | ~446 MB |
| pointsav-design-system/ | /srv/foundry/vendor/pointsav-design-system | ~3.6 MB |
| woodfine-fleet-deployment/ | /srv/foundry/customer/woodfine-fleet-deployment | ~105 MB |

Each sub-clone configured with:
- `origin` → SSH alias for canonical (`pointsav-administrator` for
  pointsav/* repos; `woodfine-administrator` for woodfine/*)
- `origin-staging-j` → jwoodfine
- `origin-staging-p` → pwoodfine
- `cluster/project-bim` branch checked out from `main`
- L1 trajectory capture hook (`bin/install-capture-hook.sh`) installed

cluster/project-bim is at the same HEAD as main on all three sub-clones.

Cluster scope per operator (workspace v0.1.59 chat):
- 5 NEW projects in pointsav-monorepo: service-materials,
  service-buildings, app-orchestration-bim, app-workplace-bim,
  app-console-bim. (Possibly +service-codes pending research.)
- Building Design System tokens in pointsav-design-system (parallel
  to project-design's DTCG vault pattern).
- Deployment catalogs in woodfine-fleet-deployment: existing
  cluster-totebox-property/ (target instance for the per-property
  archive) + NEW gateway-orchestration-bim/ (target catalog for
  bim.woodfinegroup.com frontend).

Tetrad legs (per Doctrine claim #37):
- Vendor: pointsav-monorepo (5 NEW projects)
- Customer: woodfine-fleet-deployment (cluster-totebox-property +
  gateway-orchestration-bim catalog folders)
- Deployment: ~/Foundry/deployments/cluster-totebox-property-1/ +
  ~/Foundry/deployments/gateway-orchestration-bim-1/
- Wiki: drafts-outbound/ → project-language gateway

Three Sonnet research sub-agents dispatched in parallel from Master
session at 2026-04-28T20:08Z:
- A: BIM Design System prior art + token taxonomy + Revit muscle-memory
- B: City Code as Geometry leapfrog research
- C: US/EU regulatory acceptance for flat-file IFC architectures

Reports land at `~/Foundry/.claude/sub-agent-results/`.

Cluster manifest `.claude/manifest.md` will be authored by Master
once research returns — pending for ~30 min.

— Master Claude (provisioning), 2026-04-28

---

## 2026-04-28 — Sub-agent A returned (~9 min, 414 lines)

`A-bim-design-system-prior-art-2026-04-28.md` — three findings:

**1. The space is genuinely empty.** No prior art for "Carbon for AEC."
Closest candidates fragment across three layers: (a) Archilogic
Honeycomb (MIT, Vue 3, Dec 2025) — design system FOR an AEC web app,
not FOR AEC workflows; (b) Hypar Elements (Apache 2.0, C#) — element
data vocabulary; (c) Speckle Objects Kit — BuiltElements namespace.
The BIM-semantic UI layer (IFC tree browser, property inspector, 3D
viewport with element selection) has no open prior art and must be
built.

**2. Token taxonomy validated** — 8 primitives anchored to IFC 4.3:

| Token | IFC Anchor |
|---|---|
| SPATIAL | IfcSpatialElement |
| ELEMENTS | IfcBuiltElement |
| SYSTEMS | IfcDistributionElement (MEP) |
| MATERIALS | IfcMaterial + bSDD |
| ASSEMBLIES | IfcElementAssembly |
| PERFORMANCE | IfcPropertySet + IfcQuantitySet |
| IDENTITY+CODES | IfcClassificationReference + IfcConstraint |
| RELATIONSHIPS | IfcRel* relationship entities |

Operator's draft (Materials/Elements/Spaces/Systems/Connections/
Codes/Performance/Identity) refined: Connections → RELATIONSHIPS;
Identity + Codes merged on `IfcClassificationReference`. Uniclass
2015 recommended as the classification floor.

**3. City-Code-as-Geometry: REAL but staged.** IDS 1.0 (June 2024)
explicitly limited to alphanumeric — explicit gap. Solibri does
geometric checking proprietarily. Singapore CORENET X (mandatory
Oct 2026) is closest production city-as-publisher. CHEK (EU Horizon
Europe, concluded 2025) demonstrates open research. Realistic
roadmap: 2026-2028 ship IDS+bSDD-based codes; 2028-2032 leapfrog
to IfcConstraint geometric encoding + jurisdiction packs. Hyperscaler
moat is structurally incompatible because cities acting as neutral
publishers contradicts subscription tenancy.

**10 universal interface components** validated across Revit /
ArchiCAD / Bonsai / BricsCAD: SpatialTree, PropertiesPanel,
Viewport3D, ViewNavigator, Toolbar, StatusBar, SelectionFilter,
TypeBrowser, SectionPlane, AnnotationLayer. Plus 4 console-unique
(`GuidSearch`, `AuditLog`, `Dashboard`, `ExportPanel`) and 4
workplace-unique (`MaterialsBrowser`, `TypeEditor`, `ClashDetector`,
`VersionHistory`). Most components shared with `mode` prop.

— Master Claude (sub-agent A return), 2026-04-28

---

## 2026-04-28 — Sub-agent C returned (~9 min, 460 lines)

`C-bim-regulatory-acceptance-2026-04-28.md` — verdict
**STRUCTURALLY FAVORABLE.**

**Standards floor (cluster must declare + ship):**

1. IFC export: IFC 2x3 + IFC 4 + IFC 4.3 (ISO 16739-1:2024). Stable
   GUIDs, correct property sets, schema validation. IfcOpenShell
   0.8.5 sidecar.
2. COBie via `ifccsv` — mandatory for GSA, USACE, VA (US); UK BIM
   Level 2.
3. BCF 3.0 — mandatory for Spain (2025), Dubai (2024); accepted
   everywhere.
4. IDS 1.0 validation via `ifctester` — adoption growing post-June
   2024 official release.
5. ISO 19650-compliant metadata per information container —
   git-based flat-file repo qualifies as a CDE solution under the
   standard's technology-neutral language (UID = commit hash,
   Status = branch/tag, Revision = git history, Classification =
   directory + YAML, Change History = git log).
6. BEP (BIM Execution Plan) documentation aligning PointSav's role
   with project requirements.

**Structural advantages over hyperscalers (cannot be closed):**

| Dimension | Hyperscalers | PointSav |
|---|---|---|
| ITAR / air-gapped defense | Cannot host CUI on standard cloud (requires IL4-IL6 isolation) | NATIVE — data never leaves customer hardware |
| EU Data Act vs US CLOUD Act | US-origin clouds remain CLOUD Act-subject | NO CLOUD DEPENDENCY — structurally sovereign |
| GDPR data residency | Contractual (BCRs + SCCs + regional centers) | ARCHITECTURAL — data never leaves customer jurisdiction |
| HIPAA (VA healthcare BIM) | Requires Business Associate Agreement | No BAA — local storage |
| FedRAMP | Required for cloud BIM | EXEMPT — not a cloud service |
| EUPL-1.2 | Proprietary subscription | EU procurement preferred; FAR 12.212 compatible; DoD "prefer OSS" supportive |
| Subscription-lapse data access | "You will need to enter into a new Token Flex Term…" (Tandem) | OWNER HOLDS THE FILES — permanent and unconditional access |

**Certifications path (cluster pursues):**

- buildingSMART **IFC certification** — highest-leverage; recognized
  in international BIM project acceptance
- buildingSMART **openBIM software certification** — EU procurement
  scoring weight (Denmark, Norway, Netherlands)
- **CMMC Level 2 readiness documentation** — DoD subcontractor
  positioning (process docs, not a software certification)

**Per-country EU/UK mandate snapshot:** Germany (BMDV Dec 2020
infrastructure), Italy (all ≥€1M from Jan 2025), Spain (2024-2030
plan; BCF mandatory), Denmark (2007 — world's first), Norway (2010),
Netherlands (Open BIM Infrastructure 2012; uses CityJSONSeq for
3DBAG 10M+ buildings), UK BIM Framework (Level 2 mandatory; ISO
19650), Poland (€10M+ 2025 → all by 2030), Dubai (most comprehensive
mandate globally — names ISO 19650 + IFC + IDS + BCF together).

**6 open questions carry to operator-presence pass:**

1. IFC version pinning (default support 2x3 + 4 + 4.3)
2. CMMC Level 2 applicability per-project (operator + counsel)
3. FedRAMP carve-out for any future cloud sync feature
4. COBie v3 / IFC 4 mapping QA gap
5. ISO 19650-3 CDE qualification (formal review with bSI-recognized
   professional)
6. EUPL-1.2 ITAR/classified work review (legal counsel)

— Master Claude (sub-agent C return), 2026-04-28

---

## 2026-04-28 — Sub-agent B returned (~13 min, 376 lines)

`B-bim-city-code-as-geometry-2026-04-28.md` — verdict
**INVENTION IS REAL AND MATERIALLY NOVEL.**

**Headline:** Twenty years of prior art exists in **post-design
validation** (check finished IFC against rules after designer is
done). NO prior art addresses **compositional-first code-as-geometry**
(designer composes inside pre-constrained envelopes from the FIRST
placement, before any compliance check runs). That framing is
unoccupied.

**Closest prior art (3 systems):**

| System | Status | Closest match |
|---|---|---|
| Singapore CORENET X / IFC-SG | Gold standard government publication | City-as-publisher (validation, not composition) |
| EU ACCORD AEC3PO ontology | Most semantically rigorous (OWL, 80 classes, 4297 entities) | Rules-as-data format (research-grade) |
| Commercial zoning APIs (Gridics, Archistar, Symbium) | Production validators | Machine-readable zoning (post-design checking) |

**The core conceptual gap (validator vs composer):**

- **Code-as-validation (all prior art):** Designer creates freely;
  tool runs after; violations reported as a list; designer corrects.
  Code lives outside design environment.
- **Code-as-design-system-token (the invention):** City publishes
  building code as composable BIM design tokens. Designer assembles
  inside pre-constrained envelopes from the first placement.
  Violations become geometrically impossible by construction.
  IBM Carbon analogy is exact: "developers don't draw a button and
  then run accessibility checker — they compose from Carbon tokens
  that are already accessible by construction."

**Technical mechanism (4 layers):**

1. **bSDD city dictionary** — `bsdd-cityofwoodfine.json` published
   to buildingSMART Data Dictionary. Each zoning class + bylaw
   property gets a dereferenceable URI. Existing infrastructure;
   no new standard required.
2. **IDS constraint specification** — `woodfine-rs1.ids` XML
   referencing the city's bSDD URIs. Production-ready (IDS 1.0,
   June 2024).
3. **IFC geometric exclusion-zone tokens** — `woodfine-rs1-constraints.ifc`
   carrying `IfcSpace` entities classified as `WoodfineZoning.ExclusionZone.Front`,
   etc. Designer federates this `.ifc` fragment into site model;
   exclusion zones are PHYSICALLY PRESENT as volumes; IfcClash
   detects building-element intrusion in real time.
4. **Per-jurisdiction overlay** — Municipal + Provincial + Federal +
   Accessibility IDS files compose via sequential `ifctester` runs
   today; in IFC 5 / IFCX (alpha) becomes USD-style explicit
   sublayers with priority ordering.

**Structural moat against hyperscalers (3 layers, all structural):**

1. **Regulatory sovereignty** — Cities cannot award exclusive code-
   interpretation rights to a single vendor whose pricing they
   don't control. Cloud BIM cannot be the authoritative interpreter
   of a city's own bylaw without the city's cooperation, and cities
   structurally cannot grant that.
2. **Flat-file portability** — bSDD JSON + IDS XML + IFC-SPF plain
   text. Any IFC-capable tool can consume. No subscription gate.
3. **Per-jurisdiction overlay at flat-file scale** — Multi-tenant
   SaaS would need to hold all jurisdictions' codes as "tenants" in
   one vendor's database. Regulatory bodies will not accept this.

**First-deployment scope (6-8 weeks, no government approval needed):**

- Week 1-2: `bsdd-woodfine-zoning.json` published to bSDD
- Week 2-3: `woodfine-rs1.ids` IDS constraint file
- Week 3-5: `woodfine-rs1-constraints.ifc` geometric exclusion-zone
  fragments (front + side + rear setback volumes + height envelope
  surface)
- Week 5-6: Multi-jurisdiction overlay demonstration (Woodfine + BC
  Building Code + simplified NFC fire code = three IDS files in one
  validation run)
- Week 6-7: DESIGN-COMPONENT draft `component-bim-code-rs1.draft.md`
  for project-design pickup
- Week 7-8: PROSE-TOPIC draft `topic-city-code-as-geometry.draft.md`
  for project-language pickup; GUIDE runbook
  `guide-bim-code-encoding.md` in fleet-deployment catalog

All buildable with IfcOpenShell 0.8.5 + Bonsai today. No new
standards required. Phase 2 (after demonstration): approach Woodfine's
local BC municipality with the encoded bylaw, propose publishing
relationship.

**Doctrine claim candidates (operator-presence ratification):**

- **Claim #40 — Flat-File BIM Substrate** (broader; includes the 5
  hyperscaler-incompatible capabilities from `BIM_Buildable
  Architecture.md`: asset-anchored / offline-field / vendor-
  obsolescence-survivable / IoT-direct / BIM+lease+ledger
  convergence)
- **Claim #41 — City Code as Composable Geometry** (specific
  invention; cities as design-system publishers; compositional-first
  code-as-token model)

— Master Claude (sub-agent B return), 2026-04-28

---

## 2026-04-28 — First Task session: v0.0.1 baseline shipped

Task Claude opened the cluster on 2026-04-28 (operator green-lit "yes"
post-sweep). Session commits across all three sub-clones on
`cluster/project-bim` branch:

- `pointsav-monorepo` `3fb2759` (Peter Woodfine) — 48 files; 6 NEW
  projects scaffold-coded + registry transitions (3 Reserved-folder →
  Active; 1 retired with split into 3 sibling Active services).
- `pointsav-design-system` `6f2ceaa` (Jennifer Woodfine) — 21 files;
  Building Design System BIM extension (8 token categories anchored
  to IFC 4.3 + Uniclass classification floor + 3 universal AEC
  component recipes + 3 AI-readable research files).
- `woodfine-fleet-deployment` `05ccb19` (Peter Woodfine) — 5 files;
  gateway-orchestration-bim/ catalog folder NEW + cluster-totebox-property/
  GUIDE-bim-archive-operations.md.

All three commits via `bin/commit-as-next.sh` (staging-tier, signed,
Stage-6 hold preserved). L1 trajectory hook fired on each commit;
shadow brief dispatched to Doorman.

### Cluster-tier sub-agent research dispatched + returned

BB.1–BB.4 dispatched in parallel as background Sonnet sub-agents at
session start. All four returned within the session. Reports at
`.claude/sub-agent-results/`:

- BB.1 (947 lines) — IfcOpenShell Rust subprocess. Critical: ifctester
  exits 0 regardless of validation outcome; parse JSON.
- BB.2 (346 lines) — xeokit chosen over @thatopen for double-precision
  rendering. License correction: combined work AGPL-3.0 not EUPL-1.2.
- BB.3 (865 lines) — Tauri 2.10 BIM-scale. Never pipe IFC over IPC;
  convertFileSrc + Channel<T>; sidecar download + SHA-256 verify not
  bundle.
- BB.4 (525 lines) — Bonsai. SpatialTree storey-default expansion;
  build purpose-built widget, not Outliner-as-tree.

All four findings encoded directly in the relevant project CLAUDE.md
files at v0.0.1 commit.

### Tetrad legs

Per Doctrine claim #37 — vendor / customer / deployment satisfied;
wiki partial (1 substantive PROSE-TOPIC draft + DESIGN-INDEX
manifest). 7 PROSE + 15 individual DESIGN drafts queue for v0.0.2.

### Cross-cluster outbox messages

Four heads-up messages staged in `.claude/outbox.md` addressed to
project-design, project-data, project-language, project-slm. Plus
v0.0.1-handoff message to Master with workspace-tier action items
(systemd unit + nginx + DNS + certbot) and Doctrine claim
ratification proposals (#40 Flat-File BIM Substrate; #41 City Code
as Composable Geometry).

### Material licensing fact

`app-workplace-bim` is AGPL-3.0 (xeokit coupling). The other 5 NEW
projects remain EUPL-1.2.

— Task Claude (project-bim, first session), 2026-04-28T22:50Z

---

## 2026-04-28T23:46Z — bim.woodfinegroup.com LIVE on HTTP

Operator (sudo prompt) ran
`/srv/foundry/infrastructure/local-bim-orchestration/bootstrap.sh`
after Master mirrored the cluster compute/ files to workspace tier.
v0.0.1 binary serving on 127.0.0.1:9096; nginx vhost forwarding from
public 80 → 127.0.0.1:9096; HTTP probe `curl http://bim.woodfinegroup.com/healthz`
returned 200.

DNS A record `bim.woodfinegroup.com → 34.53.65.203` was operator-
provisioned 2026-04-28 between 23:00Z and 23:30Z. certbot ran later
(timestamp not exactly captured but verified 200 with valid cert by
2026-04-29T01:00Z).

— Task Claude (project-bim, second session), live-up landing

---

## 2026-04-29 — v0.0.2 visual upgrade — implementation cycle

Second Task session opened 2026-04-28T22:00Z+; ran through several
substantial threads before yielding.

### Operator-iterated framing

The session traversed multiple operator clarifications about what
`bim.woodfinegroup.com` IS:
- Initially proposed: PointSav-extension feel
- Operator clarified: Woodfine tenant of PointSav software (Stripe-
  customer-dashboard pattern)
- Operator further clarified: NOT the property dashboard (that's
  app-console-bim querying os-console); IS the BIM token catalog
  for Woodfine's architects + engineers + BIM operators
- Operator final: "original copy of the two best 'design system'
  websites" — Carbon-shape stays at design.pointsav.com; pick a
  different best-in-class for bim.woodfinegroup.com so bankers /
  collaborators / contributors can distinguish at a glance

### Three sub-agents dispatched (BB.13 + BB.14 in this session)

- BB.13 (cluster research, 640 lines) — Untitled UI 3 + design-system-
  showcase patterns survey + AEC vernacular markers; verdict: Inter +
  JetBrains Mono + drafting-blue palette + 5 mandatory AEC markers
- BB.14 (cluster research, 416 lines) — best-in-class design-system
  websites landscape; recommendation: Adobe Spectrum-shape + Source
  Serif 4 / Geist Sans / Geist Mono typography stack; 14/15 score on
  bankers' thumbnail-distinguishability test vs Carbon's 3/15

### Woodfine brand pull

Read /srv/foundry/customer/woodfine-media-assets/. Confirmed canonical
palette (woodfine-canvas #F7F9FA, white #FFFFFF, slate #111827,
slate-translucent rgba(17,24,39,0.15), grey-mid #6B7280, grey-light
#e9ecef, blue #164679, accent-secure #54924E in theme CSS only).
Logo + signet specs locked under DS-ADR-08 "Institutional Brutalism."

### Implementation — v0.0.2 commit ef0d974

14 files, +2167 / -202 lines in pointsav-monorepo:
- nav.rs (new) — sidebar IFC tree (8 token categories + components +
  research + code overlays)
- script.js (new, 103 lines) — copy/clipboard + tab disclosure + preview
  theme toggle (zero-JS baseline preserved via <details>)
- render.rs (rewrite, +934 net) — Spectrum-shape templates with
  isometric building-mass hero SVG (IFC anchor labels: IfcSite,
  IfcBuilding, IfcBuildingStorey, IfcSpace.ExclusionZone)
- style.css (rewrite, ~1023 lines, 18 sections) — full CSS-custom-
  property design-token layer mirroring Woodfine canonical
- main.rs — new routes /tokens (index), /tokens/<key> (per-category
  detail), /code-overlays (Doctrine claim #41 surface, empty-state
  at v0.0.2), /static (ServeDir)
- compute/{bootstrap.sh, local-bim-orchestration.service} — v0.0.2
  font-sync step 3b + BIM_STATIC_DIR env var
- 3 self-hosted WOFF2 fonts at static/fonts/ (~570 KB total): Geist
  Sans + Geist Mono + Source Serif 4 Variable Roman; all OFL 1.1

### 3 cross-repo / cross-cluster drafts staged

- WOODFINE-PALETTE-ADDITIONS.md (admin-only repo handoff) — 7 AEC
  semantic color additions + accent-secure→woodfine-green promotion
- DESIGN-GENERIC-COMPONENTS-INDEX.md — 9 domain-agnostic patterns
  flowing to project-design META-substrate
- topic-flat-file-bim-leapfrog.draft.md (already staged at v0.0.1) —
  pending project-language gateway sweep

### Local validation

cargo build --release -p app-orchestration-bim — clean, 47.08 s.
Binary smoke on port 9099 against live deployment instance: all 10
routes 200; all 3 fonts served via /static/fonts/; /readyz returns
{components_count:3, tokens_count:8, research_count:3}.

### Production state at session end

| | Status |
|---|---|
| DNS | ✅ live |
| HTTPS / TLS cert | ✅ valid (Master ran certbot) |
| v0.0.1 binary at /usr/local/bin/ | ✅ serving (the LIVE state visitors see right now) |
| v0.0.2 binary at target/release/ | ✅ ready (2.50 MB, mtime 2026-04-29T00:39Z) |
| Workspace mirror at /srv/foundry/infrastructure/local-bim-orchestration/ | ❌ still v0.0.1 (mtime 23:36Z) — pending Master re-mirror + bootstrap re-run |

Master messages in outbox at 00:50Z + 01:05Z name the exact two-command
sequence to land v0.0.2 (re-mirror cluster compute/ → infra/ +
sudo bootstrap.sh).

### Session yield — 2026-04-29T01:10Z

Cluster sub-clones all clean. Cluster history at session yield:
- pointsav-monorepo: `ef0d974` (v0.0.2 visual upgrade) <- `0a478a3`
  (v0.0.1.1 patch) <- `3fb2759` (v0.0.1 baseline)
- pointsav-design-system: `6f2ceaa` (v0.0.1 BDS extension)
- woodfine-fleet-deployment: `05ccb19` (v0.0.1 customer leg)

Workspace-tier mirroring + binary redeploy + manifest tetrad-status
flip pending Master action. Background routine `trig_01KY6e4wqYJtnrKYiN8EhFJF`
fires 2026-04-29T23:34Z to verify the v0.0.2 launch and report
VERDICT: LIVE / PARTIAL / NOT-LIVE.

— Task Claude (project-bim, second session yield), 2026-04-29T01:10Z

---

## 2026-05-07 — Leapfrog 2030 overhaul + business-side launch (Task Claude)

Three-session arc completed. bim.woodfinegroup.com is live on HTTP at port 9096.

**Commits this arc:**
- 5a034f7 — Phase 1+2: hero section, four-tab token detail, IBC 2024 overlay, /about page,
  OG meta tags, sidebar Overview section, plain-English category intros
- 1040c91 — Phase 1+2 continued: IBC nav item, footer consolidation, copyright line
- 58e2777 — Phase 3+4: footer copyright entity corrected (Woodfine Capital Projects Inc.),
  light code blocks (#F7F9FA), Token Format tab header strip, /about corporate contact

**Key decisions made this session:**
- Light code blocks: black (#0E0F12) → #F7F9FA; IBM Carbon institutional aesthetic over
  terminal aesthetic. Header strip with schema label for Token Format tab specifically.
- Copyright entity: confirmed against factory-release-engineering TRADEMARK.md — WCI, not WMC.
- "BIM Token" trademark: not registrable (too generic/descriptive); no referral sent.
- Token data license: unlicensed gap surfaced; CC BY 4.0 recommended, flagged to Master.

**Artifacts in drafts-outbound (26 files):**
- 14 DESIGN items → project-design
- 11 TOPIC/GUIDE items → project-editorial
- Routing request posted to cluster outbox for Master

**Still pending (Master scope):**
- certbot TLS for bim.woodfinegroup.com
- 4 factory-release-engineering defects (LICENSE-MATRIX, EUPL-1.2 recipe, SPDX headers,
  token data license)
- Stage-6 promotion of cluster/project-bim branch to staging remotes

— Task Claude (project-bim), 2026-05-07
