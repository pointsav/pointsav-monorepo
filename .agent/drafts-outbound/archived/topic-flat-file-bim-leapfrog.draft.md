---
schema: foundry-draft-v1
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-documentation
target_path: topic-flat-file-bim-leapfrog.md
audience: vendor-public
bcsc_class: vendor-public
language_protocol: PROSE-TOPIC
authored: 2026-04-28T22:30:00Z
authored_by: task-cluster-project-bim
authored_with: claude-opus-4-7
research_done_count: 4
research_suggested_count: 3
open_questions_count: 2
research_provenance:
  - workspace-tier sub-agent A — `~/Foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md`
  - workspace-tier sub-agent B — `~/Foundry/.claude/sub-agent-results/B-bim-city-code-as-geometry-2026-04-28.md`
  - workspace-tier sub-agent C — `~/Foundry/.claude/sub-agent-results/C-bim-regulatory-acceptance-2026-04-28.md`
  - strategic source — `~/Foundry/BIM_Buildable Architecture.md`
research_inline: true
references:
  - cites: [iso-16739-1-2024, ifc-43, ids-1-0, bcf-3-0, eupl-1-2, ni-51-102, osc-sn-51-721]
notes_for_editor: |
  This is the substrate-explainer batch foundational draft.
  Doctrine claim #40 narrative — "Flat-File BIM Substrate".
  vendor-public BCSC class — applies the "we describe what Foundry
  does, not competitive contrast" rule per workspace CLAUDE.md §6.
  The five hyperscaler-incompatible capabilities are described
  structurally; competing platforms are referenced only when the
  fact is technical (e.g., Tandem's contract language about
  subscription lapse) and material to the customer's risk model.
---

# The flat-file BIM leapfrog

PointSav's Building Design System is built on five architectural
constraints that, individually, are mild inconveniences for any
single feature comparison and, together, define a product category
that hyperscalers cannot occupy without cannibalising their own
revenue model. The constraints are flat-file storage, open standards,
Rust + Tauri, offline-first, and EUPL-licensed. The open BIM standards
stack matured between 2023 and 2025 into the infrastructure that
makes this strategy viable; the property-manager market gap is
documented in peer-reviewed literature; and the open toolchain is
commercial-grade today.

This document explains what flat-file BIM is, what it is not, and why
five specific capabilities follow from the architecture rather than
needing to be added on top.

## The standards stack reached production maturity in 2024

The foundation is that the standards exist, specify plain-text
encodings, and sit inside ISO. IFC 4.3 was formally published as ISO
16739-1:2024 in April 2024, extending IFC from buildings to bridges,
roads, rail, ports, and waterways. The canonical serialisation,
IFC-SPF, is ISO 10303-21 clear-text — readable in any text editor.
IDS 1.0 became the official buildingSMART standard on 1 June 2024.
BCF 3.0 is a ZIP of XML markup files plus PNG snapshots — unzip it
and the per-topic directory tree is git-friendly diff-able prose.
CityJSON 2.0 is an OGC community standard, with CityJSONSeq used at
national scale by TU Delft's 3DBAG dataset for 10M+ Dutch buildings.

What is not yet production-ready matters as much. ifcJSON remains a
community draft. IFC 5 is alpha, with a JSON-based IFCX serialisation
borrowing USD-like composition from Pixar's OpenUSD; breaking changes
are expected. The pragmatic conclusion: canonicalise on IFC-SPF
today, mirror to ifcJSON opportunistically, and architect the object
model so an IFC 5 / IFCX migration is a serialisation swap, not a
rewrite.

## What "flat-file" means

A directory of plain-text and standardised-binary files that an
ordinary text editor or SVG viewer can open without a proprietary
SDK, decades after the software vendor that produced it is gone.

| Format | ISO / publisher | Role |
|---|---|---|
| IFC-SPF (`.ifc`) | ISO 16739-1:2024 | Authoritative geometry + semantics |
| IDS 1.0 | buildingSMART (June 2024) | Validation contract |
| BCF 3.0 | buildingSMART | Per-topic collaboration history |
| COBie via ifccsv | NIST | Asset handover |
| Per-element YAML sidecars | local convention | Pset_* + sensor + work-order data |
| Hash-addressed object store | local convention; Speckle-inspired | Versioned Merkle DAG |
| glTF 2.0 | ISO/IEC 12113:2022 | Visualisation cache (regenerable) |
| SVG | ISO/IEC 14496-22:2019 | 2D drawings (regenerable) |
| CityJSONSeq | OGC | Portfolio / urban context |

The `.ifc` file is the authoritative spatial and semantic state of
the building. The sidecars carry non-geometric data (ratings,
quantities, sensor readings, work orders, lease references). The
object store layer gives the whole vault git-grade versioning
semantics. Visualisation derivatives are caches that regenerate at
will from the authoritative source. Any specific BIM viewer or
authoring tool is replaceable. The archive is permanent.

## Five capabilities that follow from the architecture

### 1. Asset-anchored BIM

The digital twin is signed with the land title and travels with the
property deed when ownership changes hands. Multi-tenant SaaS cannot
offer this without breaking the tenancy model — a new owner would
need to be onboarded to the vendor's tenant, the model migrated,
permissions reconstructed, the subscription repriced. A flat-file
twin is owned like the building itself: forever, transferrably,
without vendor permission.

Autodesk's own Tandem licence makes the contrast explicit: a Token
Flex term lapse means "you will need to enter into a new Token Flex
Term… for continued access to Your Project data." The digital twin
rents; it does not sell.

### 2. Offline-capable BIM for field use

Basements, rooftops, remote construction sites, air-gapped defence
facilities, healthcare campuses with strict data-residency rules,
developing-world connectivity — all are workflows where a cloud-
authoritative twin is structurally impossible. ACC, Tandem, iTwin
Experience, and dTwin require live internet access by construction.
The Tauri + Rust shell hosting an offline IFC archive on a laptop
or tablet preserves full BIM functionality without any network
dependency.

### 3. Vendor-obsolescence-survivable BIM

Buildings live 50+ years. Revit's file format lasts roughly three.
The flat-file substrate is readable for decades after any specific
vendor disappears. This matters most for public-sector BIM (UK
Government Level 2, US GSA, DoD, VA), cultural-heritage custodians,
and long-horizon property owners — exactly the buyers most exposed
to vendor-discontinuation risk.

### 4. IoT integration directly into the BIM archive

A flat-file archive with per-element YAML sidecars can ingest sensor
readings via local MQTT broker, written as timestamped JSON records
into the element's sidecar, without the data ever leaving the
owner's premises. This matters economically (no sensor-count-based
token charges), legally (GDPR data residency, HIPAA in healthcare,
export control in defence), and architecturally (the sensor graph is
versioned alongside the model).

### 5. BIM + lease register + financial ledger as one portable archive

For a property owner, the building, the lease, the rent, and the
financing are the same asset. The building is where the lease
applies; the lease is where the rent comes from; the rent services
the loan; the loan justified the building. Multi-tenant cloud cannot
commingle BIM, lease register, and rent roll in a single owner-
controlled archive — commercial confidentiality, data residency,
financial-audit trails, and multi-tenant isolation each prevent it
on its own.

The Foundry workplace family — `app-workplace-memo`,
`app-workplace-presentation`, `app-workplace-proforma`, and now
`app-workplace-bim` — converges these into one portable archive. The
Totebox Archive becomes the first data architecture where a building's
legal, financial, spatial, and operational identity are one artifact
that travels with the asset.

## Government regulatory acceptance is structurally favourable

The format stack — IFC-SPF + IDS 1.0 + BCF 3.0 + COBie — fulfills
mandatory open-standard delivery requirements across US federal (GSA,
USACE, VA, NAVFAC), EU member states (Germany, Italy, Spain, Denmark,
Norway, Netherlands, Poland), the UK BIM Framework, Singapore CORENET
X (mandatory October 2026), Dubai (mandatory since January 2024),
and the broader buildingSMART openBIM movement.

The offline-first, flat-file architecture is the only approach that
natively satisfies ITAR air-gapped requirements for defence projects,
EU Data Act data sovereignty for European projects, HIPAA technical
safeguards for VA healthcare facilities, and GDPR data residency for
EU government clients — without dependency on a cloud vendor's
contractual assurances. The EUPL-1.2 licence is OSI-approved, FAR
12.212-compatible, and EU-procurement-preferred.

## What flat-file BIM does not do well — yet

Honest accounting:

- Real-time multi-user editing is slower than synchronous SaaS for
  charette-style design workshops. Cloud SaaS is genuinely better
  for synchronous design sessions.
- City-scale federation (1M+ buildings) needs a different streaming
  architecture than a single-property archive.
- Generative-AI BIM authoring (Autodesk's Project Bernini class) is
  vendor-closed today. The substrate is AI-ready (the Doorman
  dispatches generative requests through an audit ledger), but
  PointSav does not ship a generative BIM authoring tool at v0.0.1.

These are deliberate trade-offs for the offline-first, vendor-
obsolescence-survivable posture; not oversights to be patched in
the next release.

## Provenance

Authored by Task Claude on cluster `cluster/project-bim`,
2026-04-28, on the basis of Foundry workspace research returned the
same day. Strategic source `~/Foundry/BIM_Buildable Architecture.md`
(96 lines, April 2026) plus three workspace-tier Sonnet research
reports:

- A — Building Design System prior art + token taxonomy
- B — City Code as Composable Geometry leapfrog (Doctrine claim #41 candidate)
- C — US/EU government regulatory acceptance

Refinement target: `vendor/content-wiki-documentation/topic-flat-file-bim-leapfrog.md`
plus a Spanish strategic-adaptation overview at
`topic-flat-file-bim-leapfrog.es.md`.

## Research trail

### Done

- Cross-walked five hyperscaler architectural assumptions to specific
  vendor evidence (Autodesk APS pricing, Tandem licence language,
  Bentley iTwin Snapshot vs production differentiation, Trimble
  Connect cloud-only graph). Workspace sub-agent A.
- Validated open BIM standards stack maturity dates (IFC 4.3 / ISO
  16739-1:2024, IDS 1.0 final June 2024, BCF 3.0, IfcOpenShell 0.8.5
  April 2026). Workspace sub-agent A + strategic source.
- Validated regulatory acceptance posture across US federal + EU
  member states + UK + Singapore + Dubai. Workspace sub-agent C.
- Cross-walked compositional-first code-as-geometry framing against
  20 years of post-design validation prior art. Workspace sub-agent B.

### Suggested

- Verify Tandem licence language quotation against current Autodesk
  contract terms before publication (vendor terms revise quarterly).
- Pull current Forge / APS token-pricing snapshot at publication
  time; pricing has moved twice in 2025-2026.
- Confirm Tauri 2.10 stable release and feature posture at the
  publication date — track post-2.10 minor releases.

### Open questions

- Whether the "EUPL-1.2 + AGPL-3.0 = AGPL-3.0 combined work"
  rule applied to xeokit (the workplace-bim viewer choice per BB.2)
  changes the licensing posture for *this* document. The TOPIC is
  EUPL-1.2 (vendor-public, not workplace-bim itself), so the
  combined-work rule does not bind here. Worth a footnote when
  refining for `app-workplace-bim` consumption.
- Whether the strategic source's "1.5 MB / 49 MB" XKT compression
  number for the Schependomlaan reference model holds for typical
  property-manager-scale models (smaller, fewer elements). Bench
  in v0.0.2 deployment instance.
