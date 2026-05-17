---
schema: foundry-draft-v1
state: draft-pending-editorial-pass
originating_cluster: project-bim
target_repo: vendor/content-wiki-documentation
target_path: topic-property-manager-bim-gap.md
audience: vendor-public
bcsc_class: vendor-public
language_protocol: PROSE-TOPIC
authored: 2026-05-17T00:00:00Z
authored_by: totebox@project-bim
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 2
open_questions_count: 1
research_provenance:
  - workspace-tier sub-agent A — `~/Foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md`
  - workspace-tier sub-agent C — `~/Foundry/.claude/sub-agent-results/C-bim-regulatory-acceptance-2026-04-28.md`
  - strategic source — `~/Foundry/BIM_Buildable Architecture.md`
  - plans/bim-token-strategy.md — market gap analysis, Planon EasyFlow evidence
research_inline: true
references:
  - cites: [iso-19650-1, ifc-43, ni-51-102, osc-sn-51-721]
notes_for_editor: |
  This article makes a market-gap argument grounded in academic
  literature and documented FM platform behaviour. The Planon EasyFlow
  reference is from plans/bim-token-strategy.md; verify the citation
  is current and the product name has not changed before publication.
  The "BIM handover gap" framing is factual and well-documented in
  peer-reviewed FM literature — do not soften.
  BCSC vendor-public: Foundry positioning uses "intended" for forward-
  looking product capabilities.
---

# The property manager BIM gap

Building Information Models are authored by architects and structural
engineers, delivered to contractors for construction, and then handed
to property managers at practical completion. At that handover, the
model's usefulness to the people who operate the building for the
next 30 years depends entirely on whether the property manager has
access to a BIM viewer, knows how to use it, and can afford to
maintain the authoring-tool licence that produced the file.

In practice, most do not.

## What the research documents

The gap between BIM production and BIM consumption in facilities
management is documented in peer-reviewed literature. A consistent
finding across studies published between 2015 and 2024 is that fewer
than 40 percent of facilities managers actively use the BIM models
they receive at handover. The barriers cited most often are:

1. **Software cost.** Full-feature authoring licences (Revit, ArchiCAD)
   are priced for design professionals. A facilities manager who only
   needs to look up a Pset value or file a BCF issue cannot justify
   the cost.

2. **Training requirement.** BIM authoring tools carry a significant
   learning curve. The interface is designed for model creation, not
   model consumption. A facilities manager who needs to locate a door's
   fire rating does not want to learn a coordinate system, a view
   matrix, and a family library to retrieve a single attribute.

3. **File format opacity.** IFC-SPF is plain text, but a 50 MB IFC file
   for a mid-size building is not navigable with a text editor. Without
   a viewer, the file is effectively inaccessible.

The result is what several researchers call the "BIM handover gap": a
detailed digital model of the building exists, was paid for by the
developer, and meets the contractual delivery requirement — and sits
unused on a shared drive.

## How FM platform vendors have addressed it

Facilities management platforms — CAFM and CMMS software categories —
have recognised this gap and begun building BIM viewer integrations.
Planon's EasyFlow integration is a documented example: BIM model data
is imported into the CAFM database at handover, elements are linked to
maintenance schedules and work orders, and FM operators interact with
the data through the CAFM interface rather than a BIM viewer.

This approach solves the viewer-cost and training-cost problems by
translating BIM data into a form the CAFM platform already knows how
to display. The trade-off is fidelity: the CAFM import is lossy. Pset
values that the CAFM schema does not recognise are discarded. The
geometric model is reduced to a floor-plan bitmap or a simple room
list. The bidirectional link between FM work orders and the canonical
IFC model is not maintained.

When the building is renovated — walls moved, systems upgraded — the
CAFM database and the IFC model diverge and must be re-synchronised
manually. In practice, this synchronisation rarely happens. The FM
database reflects the building as it was delivered, not as it stands.

## The gap Foundry is intended to fill

The Building Design System's FM-specific interface components —
`GuidSearch` and `AuditLog` — are designed for the facilities manager
who needs read-only access to BIM model data without the cost or
complexity of an authoring-tool licence.

`GuidSearch` is a search interface that takes an IFC GUID — the
alphanumeric identifier stamped on every door, every wall, every
HVAC component in a BIM model — and returns the element's Pset
values, maintenance history, and open BCF issues. A facilities manager
scanning a QR code attached to a piece of equipment, or reading a
GUID from a work-order form, retrieves the building model data for
that specific element without navigating a 3D viewport.

`AuditLog` is a time-ordered log of all changes to the vault: IFC
model updates, BCF topic resolutions, work-order completions, and
sensor-reading anomalies. For a facilities manager whose regulatory
obligation is to demonstrate that a fire door was inspected, tested,
and found compliant, the AuditLog is the audit trail.

Both components are intended to run on the asset-anchored BIM vault —
the same flat-file archive that the design and construction team
used. There is no CAFM import, no schema translation, and no
synchronisation gap. The FM operator reads the canonical model
directly, at full Pset fidelity, through an interface designed for
their workflow rather than for model creation.

## The lease register convergence

The facilities management gap has a financial dimension that is less
documented in the academic literature but is immediate for a property
manager who is also a landlord. Lease register data — tenant names,
lease terms, rent amounts, demising-wall coordinates — lives in a
separate spreadsheet or lease management system, disconnected from
the spatial model of the floor plate.

When a tenant changes use, or a demising wall is moved for a new
tenant, the lease register, the FM work-order system, and the BIM
model are three separate records of the same physical change. Each
must be updated independently; none automatically propagates the
change to the others.

The Woodfine vault's per-element YAML sidecars carry lease references
alongside sensor readings and work-order history. A wall element's
sidecar includes the tenant identifier and lease term for the space it
bounds. When the lease changes, the sidecar is updated in the same
git commit that records the model change. The three records become one.

This convergence is intended to be the most strategically consequential
capability of the platform for a property manager who is also an asset
owner: the building's spatial, operational, and financial identity in
one portable archive that moves with the property deed.

## Provenance

Authored by totebox@project-bim, 2026-05-17. Sources: peer-reviewed FM
literature on BIM handover rates (cited in workspace sub-agent A
research); Planon EasyFlow integration documentation (plans/bim-token-
strategy.md); workspace sub-agent C regulatory acceptance analysis;
strategic source `BIM_Buildable Architecture.md`.

Refinement target: `vendor/content-wiki-documentation/topic-property-manager-bim-gap.md`
plus Spanish adaptation at `topic-property-manager-bim-gap.es.md`.

## Research trail

### Done

- Reviewed FM literature on BIM handover gap rates (sub-40% active
  use); sources included in sub-agent A research report. Sub-agent A.
- Documented Planon EasyFlow BIM import capability and limitations
  from published product documentation. plans/bim-token-strategy.md.
- Validated the lease-register convergence framing against the
  cluster-totebox-property vault layout (vault/elements/ sidecar
  schema includes lease reference fields). manifest.md.
- Confirmed GuidSearch and AuditLog are console-unique components
  (not in the universal 10) per manifest planned_design_drafts. Sub-agent A.

### Suggested

- Pull current Planon EasyFlow product name and feature set at
  publication time — FM platform names and integrations change on
  product roadmap cycles.
- Add 2-3 additional peer-reviewed FM literature citations with
  author/year/journal for the "fewer than 40%" handover-use finding;
  the editorial pass should verify this figure against current sources.

### Open questions

- Whether "lease register in YAML sidecar" satisfies financial-record
  retention requirements under applicable provincial or federal
  regulations for commercial leases (Ontario REBBA, BC Real Estate
  Act). Legal opinion recommended before the claim is made in
  customer-facing materials.
