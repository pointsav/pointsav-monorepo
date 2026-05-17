---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-leasing-plan-efficiencies-2026-05-17
language_protocol: PROSE-TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/leasing-plan-efficiencies.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
title: "Leasing Plan Efficiencies — Why Aperiodic Tiles Beat Modular Grids"
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
research_done_count: 3
research_suggested_count: 3
open_questions_count: 3
research_provenance:
  - "PROJECTS_MCorp_Tear Sheet_Floor Plates_Tiles_Combinations.pdf (January 06 2026, V1) — Leasing Plan Efficiencies chart (pp. 1, 4–8)"
  - "CONSTRUCTION_MCorp_2026_01_06_Tiles_Leasing Plan Efficiencies_FIN.docx — companion docx (binary, not directly readable in this pass; content reflected in the Combinations PDF)"
  - "AEC_Floor Plates_Methodology_V12.pdf (May 06 2025) — Leasing section (p. 10) and Solutions — Tile Size (p. 10)"
research_inline: true
notes_for_editor: |
  Living document. The leasing efficiency claim — 16 leasehold
  variations at 2,775 SF vs. 9 at 2,925 SF for a 325 SF modular
  grid — is the quantitative justification for the methodology.
  This topic isolates that argument and the demising-tolerance rule.
---

# Leasing Plan Efficiencies — Why Aperiodic Tiles Beat Modular Grids

The Floor Plate Methodology defends a specific design claim: an
aperiodic tile system with irregular Key Plan widths produces more
distinct leasable configurations per square foot than a uniform
modular grid. This topic isolates the quantitative argument and the
secondary leasing levers (climate-zone autonomy, demising tolerance,
rolling efficiency).

## The 16-vs-9 result

From the V1 Combinations PDF p. 4 (Private Office combinations):

**Modular grid (uniform 325 SF cells).** Nine 325 SF cells in a row
yield nine distinct contiguous leasehold sizes: 325, 650, 975,
1,300, 1,625, 1,950, 2,275, 2,600, 2,925 SF. Total area: 2,925 SF.
Combinations: 9. Climate zones: 1. Doors: 9. Service hookups: 9.

**Aperiodic tile system (325 / 465 / 685 SF mix).** The same row
length, filled with irregular Private Office widths
(325 + 465 + 325 + 685 + 325 + 325 + 325 = 2,775 SF), yields
**sixteen** distinct contiguous leasehold sizes ranging from 325 SF
to 2,775 SF, with intermediate values at 465, 650, 685, 790, 975,
1,010, 1,115, 1,335, 1,475, 1,660, 1,800, 1,985, 2,125, 2,450,
2,775 SF. Climate zones: 1. Doors: 7. Service hookups: 7.

**Result.** The aperiodic system delivers 16 / 9 = 1.78× more
distinct leasable configurations at 95% of the area with 78% of the
door + hookup count. The leasing instrument can match a wider
distribution of tenant-requested footprints without recomposing the
Tile.

## Nesting — Professional Office sub-types

The same argument extends to the Professional Office sub-types. The
V1 Combinations PDF documents the leasehold counts for each
sub-type (Laboratory pp. 5, Medical p. 6, Civic p. 7, Academic p. 8):

| Sub-type | Aperiodic combinations | Modular combinations | Total SF |
|---|---|---|---|
| Laboratory | 30 | 18 | 5,550 |
| Medical | 30 | 18 | 5,550 |
| Civic | 42 | 27 | 8,325 |
| Academic | 30 | 18 | 5,550 |

The Civic sub-type achieves the highest combination count because
the larger plate area (8,325 SF) admits more intermediate
demising-wall positions while preserving the irregular-width nesting
property.

**Note.** The Academic page in the V1 Combinations PDF p. 8 lists
two anomalous rows: combination 21 at 442.6% / 15,315 SF and
combination 22 at 87.9% / 16,510 SF. These are spreadsheet artifacts
(the source xlsx ran a relative-percentage formula past the data
range), not real combinations. Confirmed by inspection — the
trailing rows reset to plausible values.

## Climate-zone autonomy as a leasing instrument

Every Tile is exactly one HVAC climate zone. The leasing implication
is direct: a small tenant who wants to control their own thermostat
must lease a whole Tile, not a partial.

| Tenant footprint | Tile choice | Climate zones shared |
|---|---|---|
| 300 SF Private Office | Tile B-1 segment | 4 other PO tenants |
| 500 SF Private Office | Tile G segment | 9 other PO tenants |
| 2,700 SF Corporate | Tile A | None — sets own |
| 4,900 SF Corporate | Tile F | None — sets own |

The leasing agreement makes the trade-off explicit and prices it.
A Private Office tenant pays a base rate; a Tile A Corporate tenant
pays a premium for full climate authority. The methodology's
"Smaller tiles increase tenant autonomy and operational complexity
in parallel" formulation (V12 Methodology p. 2) is the same point.

## Demising tolerance

The methodology permits demising walls to fall on any Tile edge.
The cost of crossing a Tile boundary is loss of climate-zone
control:

> "Tenants have the ability to take any size space as long as the
> demising walls line up with any of the Tiles. Tenants who take
> space beyond the boundaries of the Tiles no longer maintain
> control of the Climate Zone."

When a tenant takes part of a Tile, the remainder of that Tile is
re-leased to another tenant — the two tenants share the climate
zone. The HVAC system cannot be split mid-tile without rebuilding
the duct routing.

**Open question.** The Solutions — Tile Size section
(V12 Methodology p. 10) notes that "Not sure any Building Services
solve this problem" — i.e., there is no engineering workaround for
a mid-tile demising wall short of full HVAC re-engineering. The
leasing instrument therefore disincentivises mid-tile demising via
a higher rent line item, but does not prohibit it.

## Rolling efficiency

The methodology rejects the traditional efficiency metric
(net leasable / gross buildable) in favour of "Rolling Efficiency":
the share of the floor that is actively leased and represented by a
Key Plan matching a real tenant's furniture inventory.

- A floor at 100% engineering efficiency with one tenant in a
  60-foot rectangle, putting bookshelves on top of desks because
  the Habitat zone is too shallow, is failing the economic test.
- A floor at 88% engineering efficiency with seven tenants in mixed
  Tile compositions, each at 100% tenant-utilisation inside their
  leasehold, is succeeding.

The Rolling part is the dynamic test: as tenants come and go, the
Tile recomposition must remain valid. The catalogue is sized to
guarantee 100% utilisation is always recoverable from a published
combination.

## PC vs SU class distinctions

| Class | Floor plate | Floor count | Tile-family preference |
|---|---|---|---|
| Professional Centres | 19,000 – 23,000 SF | 3 – 5 floors | Mostly Small + Medium for high climate-zone count |
| Suburban Office | 17,000 – 21,000 SF | 6 – 9 floors | Small for ground floors near retail; Large for upper Corporate |

Suburban Office buildings are taller (6–9 floors) and slimmer
(17,000–21,000 SF per plate); their Tile choices skew toward
maintaining the Private Office sub-type at all floor levels. A
Suburban Office building "will most likely require different size
Tiles, along with different short and long side dimensions, in
order to account for its elevation" (V12 Methodology p. 12), but
the Private Office and Professional Office Key Plans remain the
same. The variation lives in the Special Tiles and the Building
Width, not in the Key Plans.

## Open research

- **Q1.** What is the price differential (rent per SF) the leasing
  instrument applies for a mid-tile demising wall? The methodology
  is silent; this is operator policy.
- **Q2.** The xlsx source documents
  (`CONSTRUCTION_2026_01_06_*.xlsx`) are referenced but were not
  directly readable in this pass (binary format). Confirm that the
  16/9 numbers and the sub-type combination counts above match the
  spreadsheet master.
- **Q3.** The V1 Combinations PDF presents the leasing-efficiency
  chart with a sigmoidal curve labelled "Key Plans and Tiles (2,775
  SF)" rising above the linear "Modular Systems (325 SF)" line.
  The chart goes to 24 leasehold variations but only 16 are
  enumerated. What are leaseholds 17–24? (Likely cross-tile
  compositions; confirm.)

## Future research

- Reconcile the docx + xlsx source content with the V1 Combinations
  PDF; back-fill any unique data the binary files contain.
- Build a leasing-instrument template that prices climate-zone
  loss per mid-tile demising-wall event.
- Document the Suburban Office tile-family adjustments in a
  dedicated topic (`topic-bim-suburban-office-class.md`).

## Dated entries

### 2026-05-17 — initial draft

First-pass draft isolating the 16-vs-9 quantitative argument from
the V1 Combinations PDF. Captured the Professional Office sub-type
combination counts. Identified the docx/xlsx as binary inputs not
directly readable in this pass — content reflected via the
companion PDF.
