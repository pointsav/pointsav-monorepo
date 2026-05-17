---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-business-key-plans-2026-05-17
language_protocol: TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/business-key-plans.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
research_done_count: 6
research_suggested_count: 3
open_questions_count: 2
research_provenance:
  - DISCOVERY Business sketches V3 — `inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Business_Notes.pdf` (4 Sketches: Reception, Small with Data Box B, Medium with Data Box C, Large with Data Box D; plus Building Width Options A/B/C/D and Tenant Washroom Options A-E)
  - V3 Master Summary — `inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Summary_Notes copy.pdf` page 1 (Business row)
  - DTCG token — `woodfine-bim-library/tokens/bim/professional-office-subtypes.dtcg.json` key `bim.professional-office-subtype.business`
research_inline: true
notes_for_editor: |
  Living document — Business is the use type with the most rigorous
  Building Width Calculator interrogation. Four width Options
  (A/B/C/D) explore the trade-off between Habitat and Magazine.
  Capture the MW3 commentary verbatim because it sets the design
  principles that govern every other use type.
---

# Business — Key Plans

Business Office (a.k.a. Professional Office — Business) is one of the
five Professional Office sub-types. Its distinguishing feature is
**Zone 2 Magazine at 7.3 m** — the widest of any sub-type — driven by
open storage walls, server rooms, print areas, and team collaboration
zones characteristic of professional-services practices.

## Entry 2026-05-17 — The three sizes (V3)

| Size | Code | Area (m²) | Area (SF) |
|---|---|---:|---:|
| Small | B-1 | 311 | 3,350 |
| Medium | B-2 | 400 | 4,302 |
| Large | B-3 | 669 | 7,524 |

## Entry 2026-05-17 — Zone vocabulary (V3 baseline)

| Zone | Depth | Note |
|---|---:|---|
| Zone 1 — Habitat | 6.0 m (19'-8") | European Lighting Standard minimum |
| Zone 2 — Magazine | 7.3 m (23'-11") | Widest Magazine across all sub-types |
| Zone 3 — Corridor | 2.7 m (8'-10") | Bilateral corridor |

Computed building width = 2 × (6.0 + 7.3) + 2.7 = **29.3 m** (96'-1").

## Entry 2026-05-17 — Specialisations

The Notes document lists Business specialisations as **professional
services**: Law Firm, Accounting Firm, Engineering Firm. These are
not separate Key Plans — they are variants on the same B-1/B-2/B-3
footprints with adjusted Magazine content (legal files, audit
binders, engineering plotters).

## Entry 2026-05-17 — Building Width Options A/B/C/D

The Business — Small DISCOVERY sketches include a rigorous
exploration of four width options that captures the **bidirectional
adjustment** principle. The summary table (Sketch 1-4):

|  | Option A | Option B (Sketch 2) | Option C | Option D |
|---|---|---|---|---|
| Workstations | 15 | 21 | 15 | 15 |
| Occupancy | 34 | 42 | 28 | 33 |
| Zone 1 Habitat | 5.51 m | 7.39 m | 5.63 m | 5.51 m |
| Zone 2 Magazine | 9.26 m | 6.76 m | 6.63 m | 5.76 m |
| Zone 3 Corridor | 2.75 m | 2.75 m | 2.75 m | 2.75 m |
| **Total width** | **17.52 m** | **16.90 m** | **15.01 m** | **14.02 m** |
| Square footage (m²) | 450.7 | 399.6 | 345.5 | 339.5 |
| Square footage (SF) | 4,850 | 4,300 | 3,718 | 3,654 |

Option A (the widest at 17.52 m) maximises Magazine. Option D (the
narrowest at 14.02 m) compresses both Habitat and Magazine but loses
the "Executive Office" capability.

A second table on a subsequent page explores **mixed-option facades**
(top half one Option, bottom half another) yielding building widths
from 25.29 m to 32.29 m.

## Entry 2026-05-17 — MW3 commentary (design principles)

The MW3 ("Mathew Woodfine, version 3") inline commentary on the
Business sketches sets the design principles that govern every other
use type:

> MW: It is often suggested that 6.0 meters can be the mechanism for
> the provision of Natural Light, which we agree with, but as these
> Sketches hopefully point out, the 6.0 meters must be taken in
> conjunction with the functionality of the Use Cases which are
> determined by their furniture arrangements.

> MW: The final Building Width Calculator and the furniture
> arrangements it represents should take into consideration the
> potential for Tenants who require their own Building Certification
> within their leaseholds as National Tenants looking for Class-A
> Building specifications in the Regional Markets.

> MW: It can be seen here that Zone 1 – Habitat and Zone 2 – Magazine
> are not necessarily fixed after the Building Width Calculator has
> been established. Tenants should find it easy to position enclosed
> offices at the Facade Frontage or push them back into Zone 2 –
> Magazine while maintaining their furniture arrangements and
> circulation.

> MW: Option D highlights the case when we would need to trim the
> Building Width Calculator in order to provide greater building
> aesthetics or as may be required to bring the overall area of the
> Fixed Floor Plates in line (Professional Centres at approximately
> 19,000 SF to 23,000 SF and Suburban Office at approximately
> 17,000 SF to 21,000 SF).

> MW: Any empty Zone 2 — Magazine area that is unallocated will give
> the opportunity for Interior Design to create Informal Areas and
> storage to support the occupancy.

The last commentary captures the **tile-snap principle**: when the
Magazine is partly unallocated, it becomes a flex zone, which the
floor-plate solver should treat as snap-fill territory rather than
forcing a tile family change.

## Entry 2026-05-17 — Tenant Washroom in Zone 2

A key insight from the Business sketches: the Tenant Washroom can be
**contained within Zone 2 — Magazine**. The five washroom options
(A-E) show different configurations of 2-stall + sinks + vestibule,
all fitting within ~7 m × ~3 m. From MW3:

> MW3: The minimum width of Zone 2 — Magazine corresponds to either
> the minimum width of the Staff/Conference Room or the minimum width
> of the Tenant Washrooms when fully accounting for proper furniture
> specifications / dimensions and circulation.

This makes the Tenant Washroom + Staff Room a **co-driver** of
Zone 2 depth. For sub-types where one is larger, that one drives;
for sub-types where Magazine is narrow (e.g., Private Office at 1.4 m),
there is no Tenant Washroom in the suite — tenants use building-core
washrooms.

## Entry 2026-05-17 — Building Width vs Magazine constraint

The MW3 commentary captures a key tension:

> MW3: It may be worth adding a Restroom to the Professional Office —
> Business (Small) in order to take pressure off the Local Codes with
> regard to the capacity of the Building Core Washrooms.

> MW3: The addition of the Washroom is a deviation from the original
> outline that we sent as part of the "Key Plans — Small, Medium, and
> Large" email.

Adding washrooms inside the leasehold relieves the building core but
increases per-suite area. The trade-off is recorded but not yet
resolved — the V3 numbers (311 / 400 / 669 m²) appear to include
in-suite washroom area.

## Entry 2026-05-17 — Sketch Spatial Taxonomy

Sketch 1 ("Spatial Taxonomy — Anatomy — Syntax") shows the full
furniture layout of a Business suite using a grammatical metaphor:

> MW: Here we can see that the Spatial Taxonomy related to the
> "Reception" is similar to that of the full-width laboratory with a
> "Clean Room" or the "Auditorium" for the Professional Office —
> Academic and the "Court Room" for the Professional Office — Civic.

The reception/clean-room/auditorium/courtroom pattern is a recurring
**anchor element** at the façade end. Each sub-type carries one,
and its dimensions drive the Habitat depth.

## Future research

- Resolve the Tenant Washroom inclusion question: is the V3 numbers
  inclusive or exclusive of in-suite washroom area? Affects every
  downstream rent-per-SF calculation.
- The Building Width Calculator Options table (10 mixed-option
  configurations from 25.29 m to 32.29 m) implies a building can
  carry Option A on one half and Option D on the other. Confirm with
  operator whether this is a permitted variation or only an analysis
  exercise.
- Confirm whether Engineering Firm specialisation carries a different
  Magazine requirement (large-format plotter, equipment storage)
  that would push Zone 2 beyond 7.3 m.

## Open questions

1. The V3 Summary table shows Business Z2 = 7.3 m but the Option B
   (Sketch 2) sample uses Z2 = 6.76 m. Is the V3 baseline a Building
   Width Calculator output that supersedes Sketch 2, or are these two
   different snapshots in time?
2. The Sketch 4 (Large) annotation reads "Yes. Each of the Key Plans
   for Small, Medium, and Large for all of the Use Cases we are
   examining are too large — Business, Academic, Laboratory, Medical,
   and Civic." This implies a planned shrink across all sub-types.
   When does that shrink land, and what target sizes?
