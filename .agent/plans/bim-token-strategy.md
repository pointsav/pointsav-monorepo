# BIM Token Strategy — Research Synthesis

**Created:** 2026-05-17
**Owner:** project-editorial (research); operator (decision)
**Status:** Research complete (6 of 7 agents); commercial model section pending Agent 7

---

## Operator questions answered

Two questions from the operator dispatch:

1. **Is there validity in Woodfine using the BIM token system for its developments,
   whereby tenants come to Woodfine and use BIM tokens to do their interior layouts?**
   → **Yes, 7 out of 10.** The value is landlord-side (asset intelligence, ESG
   data, fit-out governance), not a tenant fee model. Precedent exists for the
   model; the revenue structure needs reframing.

2. **Is there an industry-wide need for BIM tokens that PointSav should produce —
   like an IBM Carbon for BIM?**
   → **Yes, 8 out of 10.** The gap is documented and real. No DTCG-format BIM
   token library exists. The timing window for capturing the de-facto standard
   slot closes around 2026–2027 (EU implementing acts). The recommended path is
   open library first, CMS second.

---

## Part I — The gap: is it real?

**Score: 8/10.** The gap is real and large.

### What exists

The AEC industry has layered property-definition infrastructure:

| Layer | Artifact | Problem |
|---|---|---|
| Schema | IFC 4.3 (ISO 16739-1:2024) — 800+ entities | Good schema, no DTCG consumption layer |
| Property catalogs | 645 IFC Psets in XML + bSDD REST API | XML-first, no offline distribution, poor DX |
| Classification | Uniclass 2015, OmniClass, ETIM | Codes, not values; no datatypes |
| Authoring tool format | Revit Shared Parameters (.txt, GUIDs) | Proprietary, no semver, no diff/merge |
| Information requirements | IDS (Information Delivery Specification) | Requirements spec, not value-bearing tokens |
| Commercial platforms | Autodesk Tandem, Siemens Building X, Azure ADT | Each builds its own ingestion pipeline; no shared authoring format |

### The precise gap

**No DTCG-format BIM token library exists.** Comprehensive searches return zero
results for projects combining "BIM/IFC" + "design tokens" + DTCG W3C format.

What all current BIM property infrastructure has in common: it is built for
*model authoring and validation* (does this Revit file comply?), not for
*developer consumption as data* (let me `import { wall, door, slab } from
"@bim/tokens"` and theme my digital-twin app or drive my IFC parser).

Carbon's insight — tokens are JSON files in git, semver-published on npm,
consumable by any build pipeline regardless of UI framework — has no AEC
analog. The DTCG specification reached its first stable version (2025.10) in
October 2025. The convergence of DTCG stable + IFC 5 JSON-first direction
creates an opening that did not exist 24 months ago.

### Who would use it

High-likelihood early adopters:
- **That Open Company ecosystem** (web-ifc, engine_components) — npm-native, JS-first, same "open the market" mission
- **Speckle Kit authors** — currently hand-define schemas; a DTCG vocabulary shortcut
- **Digital twin developers** (Autodesk Tandem, Azure ADT, Unity, Omniverse) — each currently builds its own ingestion pipeline from IFC/COBie/proprietary to internal model
- **Facilities management integrators** (Archibus, Planon, Maximo) — hand-map fields today; tokens standardize the destination schema
- **Portfolio ESG analysts** — need per-asset, per-tenant property data to satisfy Scope 3 Cat 13 disclosure by 2026 (mandatory)

Lower-likelihood early adopters: production Revit users in BIM-managed firms
(entrenched in proprietary shared-parameter files), Autodesk APS customers.

---

## Part II — Woodfine's use case validity

**Score: 7/10.** Valid, but the value framing requires adjustment.

### What works

The market gap is confirmed: no major landlord publishes DTCG BIM tokens for
tenant fit-outs. The white space is real.

**EPBD Renovation Passports** are mandatory EU-wide by 29 May 2026 — a strong
regulatory tailwind for structured building data in the landlord/tenant context.
In the UK, the Building Safety Act 2022 (Golden Thread requirement) places
statutory disclosure obligations directly on the Principal Accountable Person
(typically the landlord/freeholder) for higher-risk buildings — obligations that
can only be satisfied by aggregating tenant-level asset data.

**The tenant-change / fit-out lifecycle is the most operationally defensible
use case.** Space management is the single most turbulent use case in commercial
real estate — tenants move in/out, partitions shift, furniture rotates, occupancy
categories change. Today: the property manager accepts tenant fit-out drawings
as a one-off and manually updates the IWMS. With BIM tokens: the tenant fit-out
is delivered as a token override file, the digital twin recomposes the affected
spaces, and IWMS/BMS/ESG-reporting systems all pick up the change from one source.

### What doesn't work (as originally framed)

Direct tenant fees for BIM token access have no precedent. Tenants do not
currently pay for "access to BIM data infrastructure." The value flows upstream —
to the landlord who uses the data for AIM (Asset Information Management), ESG
disclosure, and capital planning.

**Reframing required:** Woodfine's benefit is not a tenant revenue line. It is:

1. **Operational cost savings** — eliminates manual IWMS re-entry at each fit-out
2. **ESG compliance** — satisfies Scope 3 Cat 13 disclosure without integrator labour
3. **AIM value** — building performance data is captured in real time; informs
   capital allocation and renovation timing
4. **Tenant attraction** — landlords who offer structured digital services to
   tenants compete on more than location and rate

The commercial model that works: **Woodfine deploys for its own balance sheet;
PointSav licenses the same system to other landlords as SaaS.** Woodfine's
operational use is the reference deployment; the SaaS revenue accrues to PointSav.

### Competitive position

No major landlord in Canada or the UK has published an open BIM token standard
for tenant fit-outs. Woodfine's position as the reference deployment of
`pointsav-bim-system` gives it the credibility of being the first adopter without
the cost of being the standards publisher. It is structurally identical to IBM's
position with Carbon: IBM deploys Carbon internally; external developers adopt it
because IBM's scale makes it credible; IBM benefits from the ecosystem of developers
skilled in Carbon-based interfaces.

---

## Part III — The industry gap: seven acute pain points, ranked

From the AEC workflows research (Agent 4):

| Pain point | Who bears the cost | Adoption pathway for BIM tokens | Composite score |
|---|---|---|---|
| **Client BIM requirement compliance** (hospitals, gov, university) | Owner-operators | Mandate in BEP; strong contractual lever | **Highest** |
| **FM handover** — 30% of building data lost at handover | Facility operators | Owner mandate flowing up contract chain | **Highest** |
| **Cross-firm collaboration** — parameter mapping chaos | Every multi-firm project | Professional-association-driven, then large firm early adoption | **Strong** |
| **Landlord/tenant design standard compliance** | Landlords (Scope 3 Cat 13 disclosure) | Lease-level requirement; small tractable niche | **Moderate** |
| **Computational design parametrization** | BIM-tech teams | GitHub/npm-native community; credibility building | **Moderate** |
| **IFC data fidelity** through software transitions | Everyone | Requires authoring tool vendor cooperation | Lower |
| **Portfolio ESG rollup** | Institutional CRE investors | 2026 mandatory Scope 1/2 disclosure removes discretion | **Highest** — 2026 forcing function |

**The single highest-leverage technical deliverable** (confirmed by both AEC
workflows and digital twin research): a **Revit add-in that converts the JSON
token catalog into a Shared Parameters `.txt` file + IFC mapping table**. This
is the bridge between the open standard and the authoring-tool world practitioners
actually inhabit. Without it, the token catalog is documentation; with it, it
is tooling.

---

## Part IV — Regulatory tailwind

**Verdict:** Regulation creates a substantial but indirect forcing function.
No jurisdiction currently mandates a specific BIM token vocabulary. Multiple
mandate the structured data those tokens would provide.

### Near-term forcing functions (2026–2027 window)

| Mandate | Jurisdiction | In force | BIM token relevance |
|---|---|---|---|
| EU CPR Article 76 — Digital Product Passport for construction products | EU | 8 Jan 2026 (implementing acts 2025–2027) | Per-product structured property data; implementing-act vocabulary TBD — **open for capture** |
| EPBD Article 19 — Digital Building Logbook | EU/Member States | 29 May 2026 (Member State implementations) | Per-building lifecycle data; reference vocabulary TBD — **open for capture** |
| ESRS E1-5/E1-6 (Scope 3 Cat 13) | EU/global ISSB S2 adopters | FY2024+ mandatory for large PIEs | Per-asset, per-tenant carbon/energy data; BIM tokens are the collection plumbing |
| UK Building Safety Act — Golden Thread | UK (HRBs ≥18m) | Duty-holder regime in force Oct 2023 | Principal Accountable Person (landlord) holds structured asset data; format TBD |
| California SB 253 | California | Reporting 2026–2027 | Scope 3 at entity level; drives asset-level data infrastructure demand |
| Italy — D.M. 560/2017 | Italy | All public works ≥€1M from 1 Jan 2025 | BIM mandatory; supply-chain pressure on private sector |

### Critical timing insight

**The window for an open standard to capture the EU implementing-act vocabulary
closes around 2027.** The CPR Article 76 DPP-CP implementing acts (DG GROW)
and the EPBD Article 19 DBL Member State implementations are both under active
development now. A credible vendor-published open standard with reference
implementations and bSDD registration has a plausible path to citation in these
implementing acts — but only if it exists and has demonstrable adoption before
the implementing-act drafting concludes.

Canada (including BC) has no live BIM mandate and no near-term pipeline. This
is a green-field market — no regulatory headwind, but also no regulatory tailwind.

### Who benefits most from mandates

The mandate analysis confirms the structural insight from the Woodfine use-case
research: **the entity with statutory disclosure obligations on occupied buildings
is the landlord, not the tenant, not the software vendor, not the standards
body.** The EU DBL, UK Golden Thread, and Scope 3 Cat 13 all flow to the
Principal Accountable Person / property owner. BIM tokens give landlords a way
to satisfy those obligations without hiring integrators per building.

---

## Part V — Digital twin convergence

**Verdict:** The digital twin use case substantially strengthens the BIM token
argument, more so than the architect-design use case.

### The architect-design use case (saturated)

BIM authoring is a saturated market. Revit, Archicad, Tekla, and their
ecosystem already produce IFC. The standards bodies (buildingSMART, ISO/TC 59)
own the field. IFC 5 is in alpha and moving toward JSON-native serialization.
A BIM token standard that competes with IFC would be ignored; one that composes
with it has a path.

### The digital twin / operational use case (growing)

Digital twin adoption is running at 300–400% YoY growth in commercial building
portfolios (2025 inflection). The growth is in operational twins (BMS/IoT
integration) more than BIM-anchored geometry twins — but ESG compliance pressure
is forcing BIM data into the operational layer.

**Positioning statement (architectural):** DTCG BIM tokens are not a replacement
for IFC, DTDL, Brick, or RealEstateCore. They are the *human-authorable,
version-controllable, git-native upstream feed* to all of those formats — the
same architectural position that DTCG tokens for UI occupy relative to CSS,
Sass, and platform-native styling systems.

| Platform | DTCG token → platform format | Bridge status |
|---|---|---|
| Azure Digital Twins | DTCG → DTDL | DTDL is JSON-LD; mapping is one schema conversion |
| Autodesk Tandem | DTCG → classification templates | Tandem's template model is compatible; no public bridge exists |
| Brick/RealEstateCore | DTCG → RDF/Turtle Brick instances | RealEstateCore is the reference; bridge is a SPARQL transform |
| NVIDIA Omniverse (USD) | DTCG → USD attributes | NVIDIA is encoding IFC in USD; DTCG → IFC PropertySet → USD attribute |
| ESRI ArcGIS Indoors | DTCG → geodatabase attributes | ArcGIS BIM File to Geodatabase tool is the intermediate |
| IFC files | DTCG → IFC PropertySet | The core bridge; IDS validation from the same token set |

**Key insight:** every digital twin platform vendor currently builds its own
ingestion pipeline from IFC/COBie to its internal model. A DTCG token layer
positioned as the platform-agnostic authoring surface shortens every vendor's
ingestion pipeline to one token-format-to-platform-format transformation. That
is a real, billable service for PointSav to offer (bridge serializers + hosted
platform connectors).

### Operational data is where the institutional money is

Portfolio ESG reporting under 2026 mandatory Scope 1/2 disclosure is where
institutional CRE investors have non-discretionary spend. Structured BIM tokens
that roll up to portfolio-level ESG metrics are directly in this spend category.
Architects do not control construction budgets; facility owners control operating
budgets. The operational layer is where the commercial pull is strongest.

---

## Part VI — Library-first vs CMS-first

**Recommendation: open library first (Phase 1), CMS second (Phase 2), only if
Phase 1 achieves ≥100 firm adoption.**

| Model | Description | Risk |
|---|---|---|
| **A — Open library only** | `@pointsav/bim-tokens` Apache 2.0, on npm, Style Dictionary transforms | No commercial capture; community risk without governance |
| **B — CMS only** | Proprietary token management platform; tokens are an entry-point | Zero adoption; no open ecosystem to pull from |
| **C — Both (recommended)** | Open library establishes the format; CMS provides the operational workflow surface | Requires discipline to not build the CMS before the library is proven |

**Phase 1 (open library):**
- Publish `@pointsav/bim-tokens` on npm, Apache 2.0, IFC 4.3 anchored, DTCG 2025.10 format
- Ship reference serializers: DTCG → IFC PropertySet, DTCG → DTDL, DTCG → Brick instance
- Ship Revit add-in: JSON token catalog → Shared Parameters `.txt` + IFC mapping table
- Register token identifiers with bSDD as custom dictionary (not a fork — a publication)
- Engage OSArch, Speckle, That Open Company communities

**Phase 2 (hosted dashboard — free, lightweight CMS):**
- Web UI for browsing and searching the token catalog
- Hosted per-project token override files (tenant fit-out variant of base token set)
- IDS validation: upload an IFC file, validate against the token set, get a report
- Targeting small-firm and single-project adoption (the tail of the AEC market)

**Phase 3 (productized CMS — paid):**
- Full workflow: token lifecycle management, approval gating, audit trail
- Portfolio rollup for ESG reporting (Scope 3 Cat 13 — the money)
- Multi-tenant architecture: one token catalog per landlord, token override files per tenant
- Only if Phase 1 achieves ≥100 firm adoption (proving the open library works)

**Critical anti-patterns (from Library vs CMS research):**
- Do not lead with the CMS pitch — the open library must prove itself first
- Do not build a closed token catalog — it must be openly forkable for adoption
- Must explicitly position relative to bSDD — not a competing catalog, a DTCG consumption layer

---

## Part VII — Governance and standards positioning

**How to position relative to buildingSMART:**

The relationship to bSDD and IFC must be explicitly *complementary*, not
competitive. Every token in `pointsav-bim-system` that has an equivalent IFC
Pset property should carry the bSDD URI as a canonical identifier in its
`$extensions.bsdd` field. Every token set should ship with an IDS export so
the same vocabulary that defines the tokens also validates IFC files.

**Positioning statement (for README, press, and standards-body engagement):**
> `pointsav-bim-system` is a DTCG-format developer-consumption layer for IFC
> 4.3 property sets and bSDD-registered properties. It does not fork or
> compete with IFC or bSDD. It provides the git-native, npm-distributable,
> CI-checkable authoring surface that IFC and bSDD currently lack.

**Governance options:**

| Option | Description | Fit |
|---|---|---|
| PointSav stewardship | PointSav governs the repo; open contributions via PRs and GitHub Discussions | Fast to launch; credibility risk if seen as vendor capture |
| buildingSMART liaison | Formally register with buildingSMART; participate in IDS/bSDD working groups | Highest credibility; slow; requires ongoing membership investment |
| **Linux Foundation project** | Donate the repo to a neutral foundation; PointSav retains technical lead role | Best long-term governance; precedent: CNCF (cloud-native), ASWF (media/entertainment), OpenSSF |
| New foundation | Create "AEC Tokens Foundation" as a new entity | Most control; most setup cost; little credibility without major co-founders |

**Recommended path:** Start with PointSav stewardship (fast, no overhead). Within
18 months of launch, if ≥5 non-PointSav contributors are active, pursue Linux
Foundation project donation. Engage buildingSMART as a liaison from day one
(attend IDS and bSDD working group calls; do not wait for formal membership).

**buildingSMART membership:** General membership is open; fees range from
approximately €500–€5,000/year depending on tier (participant vs. associate vs.
technical). Startup/innovator tiers may be available. Membership gives access to
working group participation, early standard drafts, and co-marketing with bSDD.
Worth pursuing in Year 1 alongside the library launch.

---

## Part VIII — Commercial model

Research basis: Agent 7 deep research on IBM Carbon, open-core commercial models,
buildingSMART membership, and first-mover dynamics in open AEC standards.

### The Carbon analogy corrected

**IBM does not monetize Carbon directly.** Carbon is Apache 2.0, fully funded by
IBM's design organization as overhead — not a profit center. There is no IBM Carbon
support contract, no "Carbon Enterprise" tier, no documented attribution of Cloud
ARR to Carbon adoption. Carbon's commercial value to IBM is: internal cost reduction
at scale (47% faster to build with Carbon than from scratch per Sparkbox study),
talent acquisition signal, and indirect alignment of IBM Cloud products.

**Direct implication:** PointSav should not expect Apache 2.0 publication of
`pointsav-bim-system` to generate direct revenue. The Carbon analogy works only if
PointSav has a portfolio of products benefiting from a common BIM token vocabulary
(which it does) and a recruiting/credibility motive (which it may). But the revenue
model is *not* IBM/Carbon. The closer analogs are Confluent/Kafka, RealEstateCore,
and the Brick Consortium.

### Three commercial model families (applicable to BIM tokens)

**Open-core / managed service (Confluent/Kafka, HashiCorp/Terraform, Redis):**
The standard is Apache 2.0; a managed cloud product captures operational complexity.
Confluent reached $11B valuation in seven years. The cautionary tale: HashiCorp
moved to BSL in 2023 when hyperscalers resold Terraform without contributing —
destroying trust. Lesson: if PointSav weakens the Apache 2.0 license, adoption
collapses and the standard gets forked.

**Standard-setting consortium (buildingSMART, Brick Consortium, RealEstateCore):**
The standard is owned by a non-profit; members pay dues and receive governance
votes, conformance certification, and co-marketing. buildingSMART membership:
€8K (Standard Small) to €100K (Strategic). ODA: $7,500/$4,500 (Sustaining).
Revenue covers staff; it does not fund growth. Gives legitimacy, not cash flow.

**Data network / aggregator (Willow/RealEstateCore, co-location intelligence):**
The open standard creates a population of compliant data assets; the steward
aggregates and resells benchmarking or analytics. This is PointSav's existing
co-location intelligence business extended to BIM data. Each token-compliant
building contributes to the aggregated signal; the signal is the competitive moat.

### Recommended model: three-layer

**Layer 1 — The open specification (no revenue, maximum adoption)**
Apache 2.0, hosted at a neutral foundation (start at W3C Community Group, migrate
to Linux Foundation directed fund within 24 months once 3+ co-founders committed).
PointSav as founding editor. Governance: foundation board, no PointSav majority.
Cost: approximately $500K–$800K/year in spec maintenance staff.

*Why this must be neutral:* AEC has a 30-year trust deficit from vendor-controlled
formats (Autodesk RVT vs IFC history). A DTCG BIM token standard from "an unknown
Canadian fintech" without neutral governance will not be adopted by competing AEC
vendors — the same vendors whose authoring tools need to consume the tokens.

**Layer 2 — Managed token platform (primary recurring revenue)**
Closed-source SaaS that runs `pointsav-bim-system` at portfolio scale: token
lifecycle, IFC anchoring service, multi-stakeholder governance, WORM provenance,
deployment to fleet. Pricing model: per-building per-month + per-anchored-commit
+ per-tenant-seat for tenant-facing applications. Target: institutional landlords
(Woodfine reference), then mid-market property owners, then AEC consultancies.
This is the Confluent play: spec is the funnel, running the spec at scale is the moat.

**Layer 3 — Co-location intelligence network (long-term strategic moat)**
Every building in the `pointsav-bim-system` network produces normalized data feeding
PointSav's co-location product. Aggregated benchmarking and ESG analytics sold to
landlords, brokers, lenders, and regulators. RealEstateCore + Willow + Brookfield
demonstrates this pattern operates in adjacent space. Only PointSav has the
cumulative cross-portfolio data network — this is the genuinely defensible position.

### Woodfine's role and economics

Founding governance seat on the foundation. Reference implementation ("Pilot 1")
co-marketed. Tenant-services revenue options:

- **Base-lease inclusion:** 2-5% rent uplift on token-enabled floors (Vasakronan
  and Brookfield precedent with RealEstateCore digital twins)
- **Tenant portal premium tier:** paid per-seat or per-sq-ft for comfort control,
  booking, telemetry access to "their" tokens
- **Sovereign data residency premium:** Canadian-jurisdiction, machine-bound data
  as a differentiator for tenants concerned about US CLOUD Act exposure

**Open vs. proprietary trade-off:** An open standard expands the supplier ecosystem
(mechanical contractors, BIM consultants, IoT vendors become competent in it),
which lowers Woodfine's operating cost on its own portfolio. For a portfolio
Woodfine's size, the supply-side compression dominates the tenant-portability risk.
BOMA/IFMA-style precedent: major landlords cooperate on data definitions because
it makes the supply chain cheaper, even though it makes tenants more portable.

### Governance pathway

| Phase | Vehicle | Cost | What it buys |
|---|---|---|---|
| Year 1 | W3C Community Group (DTCG analog) | $0 | Fast launch; low credibility |
| Years 1-2 | buildingSMART Standard membership | €8K | Legitimacy, access to IDS/bSDD working groups |
| Years 2-3 | Linux Foundation directed fund | $250K-$500K/year from co-founders | Maximum credibility; neutral governance; CNCF/ASWF precedent |
| Optional | ODA Sustaining membership | $4,500-$7,500/year | Technical integration with CAD/BIM SDKs |

**Academy Software Foundation (ASWF)** is the most precise analog: neutral forum
for open media/entertainment standards, founded by Academy + Linux Foundation,
founding members include Autodesk, Adobe, Apple, Disney, Nvidia. An "AEC Tokens
Foundation" or "Built Environment Tokens Project" on the same model is a viable
target for years 2-3.

### Revenue trajectory (realistic 5-year horizon)

| Period | Revenue | Cost |
|---|---|---|
| Years 1-2 | $0 platform revenue | $500K-$1M/year spec maintenance |
| Year 3 | $1-3M ARR (Woodfine + 2-5 reference customers) | Partially offset by foundation dues |
| Year 5 | $10-25M ARR with regulatory citation; $3-8M without | Platform + co-location intelligence |
| Year 7+ | $50-150M ARR if data network effect holds | Defensible via network, not features |

### Conditions for success

1. **At least one credible non-Woodfine co-founder within 12 months** (institutional
   owner, established AEC consultancy, or academic group). Without it, the standard
   is "vendor-controlled by a Canadian fintech nobody has heard of."

2. **Apache 2.0 is never weakened.** The HashiCorp BSL conversion in 2023 destroyed
   trust and triggered the OpenTofu fork. Commit to Apache 2.0 in the foundation
   charter in writing.

3. **The EU/Canadian regulatory window is actively exploited.** EU BIM Task Group
   (November 2025 Position Paper), Statsbygg (Norway), PSPC Canada — explicit
   citation of `pointsav-bim-system` in any procurement standard is worth more
   than 100 vendor adoptions.

4. **No major vendor (Autodesk/Trimble/Bentley) publishes a competing JSON token
   spec before Phase 1 ships.** Watch for "Forma tokens," "Tekla tokens,"
   "iTwin tokens." If a vendor moves first, PointSav must either accelerate or
   pivot to the neutral interop layer between vendor formats.

5. **The Revit add-in ships with Phase 1.** Without the JSON → Shared Parameters
   `.txt` + IFC mapping table bridge, the token catalog is documentation. With it,
   it is tooling that practitioners can adopt without changing their authoring workflow.

---

## Part IX — Recommended action sequence

All items ordered by impact. Items marked [OPERATOR] require operator approval.

### Immediate (no operator approval needed — research and planning)

1. Read `woodfine-design-bim` README and audit current token content — confirm
   which token categories exist and which are missing relative to the AEC
   adoption pathway above.
2. Draft `pointsav-bim-system` README v1 using the institutional register from
   `github-presence-elevation.md` + the positioning statements above.
3. Draft `NOTICE` file documenting the Woodfine authorship + Apache 2.0 relicense
   chain.
4. Confirm `@pointsav/bim-tokens` namespace is available on npm.

### Near-term [OPERATOR] — requires operator decision and Command Session

5. **Org transfer + rename + relicense** — `woodfine/woodfine-design-bim` →
   `pointsav/pointsav-bim-system`, EUPL-1.2 → Apache 2.0. See
   `github-presence-elevation.md` § "What changes (full move, not just rename)"
   for the full file change list. This is the highest-impact single action.

6. **Phase 1 tooling decision** — which bridge serializer ships first? Recommendation:
   Revit add-in (JSON → Shared Parameters .txt + IFC mapping table) — highest
   practitioner adoption leverage; can be scoped to project-bim cluster.

7. **buildingSMART engagement** — designate a liaison from the PointSav team to
   attend IDS and bSDD working group calls. This is a person-hour commitment,
   not a software commitment.

### Medium-term (6–12 months post-launch)

8. First anchor adopter outreach — target one institutional CRE owner (university
   capital projects office, healthcare system, or a BC government agency) to
   include `@pointsav/bim-tokens` in their BIM Execution Plan template.

9. Phase 2 hosted dashboard — scoped only after Phase 1 token catalog has ≥10
   external GitHub stars and ≥3 non-PointSav contributors.

10. Linux Foundation project donation evaluation — if ≥5 non-PointSav contributors
    are active contributors at 18 months post-launch.

---

## Synthesis: the two-question answer

### Question 1: Should Woodfine use the BIM system for tenant fit-outs?

**Yes, and the value is landlord-side.** Woodfine should deploy `pointsav-bim-system`
(once it exists in the pointsav org) for its own portfolio operations — not as
a tenant fee service, but as the data infrastructure that makes ESG compliance,
AIM, and capital planning tractable at portfolio scale. Tenants benefit from
frictionless fit-out acceptance (no manual BIM consultant required); Woodfine
benefits from the data. The commercial model has Woodfine paying PointSav for
the SaaS platform, not tenants paying Woodfine for token access.

### Question 2: Should PointSav produce industry-wide open BIM tokens?

**Yes, and the timing is now.** The gap is real (8/10). The EU implementing-act
window closes around 2027. No incumbent occupies the DTCG-format BIM token slot.
IFC 5 is in alpha and moving toward JSON-native serialization — this is the best
possible moment to publish a JSON-first BIM token standard that aligns with IFC 5's
direction. Apache 2.0 is mandatory for adoption by commercial AEC vendors.

**The play is:** open library first (proving the format), Revit add-in second
(proving the practitioner adoption path), EU regulatory engagement third (proving
the standards-body credibility), CMS fourth (capturing the commercial value).
Woodfine is the anchor tenant, PointSav is the standards publisher, and the two
roles must be structurally visible as separate entities — same as IBM (Carbon
publisher) and IBM (Carbon deployer) is the same entity, but the open-source
posture makes the Carbon-for-everyone-to-use framing credible.

---

---

*Research basis: 7 OPUS agent reports, completed 2026-05-16/17.*
*All agents ran as sub-agents of the project-editorial Totebox session.*
*Synthesis written by project-editorial Root Claude session 2026-05-17.*
