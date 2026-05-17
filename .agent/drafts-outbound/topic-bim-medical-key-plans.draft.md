---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-medical-key-plans-2026-05-17
language_protocol: TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/medical-key-plans.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
research_done_count: 5
research_suggested_count: 4
open_questions_count: 3
research_provenance:
  - DISCOVERY Medical sketches V2 — `inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Medical_Notes copy.pdf` (cappelletti sestito architetti DRAFT 250512, three dental-office sizes M1/M2/M3 with hand-annotated revisions)
  - V3 Master Summary — `inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Summary_Notes copy.pdf` page 1 (Medical row)
  - V2 Summary May 2025 — Medical S/M/L totals reconciled
  - Key Plans Samples V2 — `inputs/project-bim/--- March 03, 2025 -- Collaborators #32 --- /AEC_Floor Plates_Key Plans_Samples_V2.pdf` page 19 (Woodfine - Medical Tile - Dentist by AART architects, March 25 2024 — superseded by cappelletti sestito work because the AART layout was "not people-centered design")
  - DTCG tokens — `woodfine-bim-library/tokens/bim/professional-office-subtypes.dtcg.json` keys `bim.professional-office-subtype.medical`
research_inline: true
notes_for_editor: |
  Living document — Medical is the use type with the most detailed
  DISCOVERY sketches (cappelletti sestito architetti, May 2025).
  The hand-annotated revisions are critical: they capture the
  "people-centred design" critique of the earlier AART layout and
  drive specific layout decisions (move washrooms together, increase
  reception waiting capacity, sterilization room placement).
---

# Medical — Key Plans

Medical Office is one of the five Professional Office sub-types. Its
distinguishing feature is **Zone 1 Habitat at 7.2 m** — the widest of
any sub-type — driven by exam-table depth plus the circulation
required for both patient and clinician access.

## Entry 2026-05-17 — The three sizes (V3)

| Size | Dental chairs | Code | Area (m²) | Area (SF) | Façade frontage |
|---|---:|---|---:|---:|---|
| Small | 2 | M3 | 223 | 2,401.5 | 718" (~18.24 m) |
| Medium | 4 | M1 | 331 | 3,567.7 | 1,066-5/8" (~27.10 m) |
| Large | 6 | M2 | 486 | 5,231.4 | 1,564" (~39.73 m) |

**Note on FFE code numbering.** The Medical FFE codes are numbered by
historical authoring order, not by size: M1 was the first dental-suite
sample drawn, then M2 (the large) and M3 (the small). For external
documents, prefer the **size labels** (Small / Medium / Large) over
the FFE codes; for token references, both are valid.

## Entry 2026-05-17 — Zone vocabulary

| Zone | Depth | Note |
|---|---:|---|
| Zone 1 — Habitat | 7.2 m (23'-10") | Widest Z1 across all use types — exam table + patient + clinician circulation |
| Zone 2 — Magazine | 4.9 m (16') | Medical supply storage, sterilisation, equipment staging |
| Zone 3 — Corridor | 2.9 m (9'-5") | Bilateral; ADA/CSA-B651 compliant width for stretcher and wheelchair passage |

Computed building width = 2 × (7.2 + 4.9) + 2.9 = **27.1 m** (88'-11").

## Entry 2026-05-17 — Furniture anchor: the KaVo uniQa dental chair

Every Medical sketch carries the KaVo uniQa dental chair as the
reference SKU:

- Chair envelope: 96-1/2" × 114-1/8" (2,451 mm × 2,898 mm)
- Each exam room is approximately 286-5/8" × 192" (7,280 mm × 4,877 mm)
  = ~35.5 m² per exam room

Twelve exam rooms across the M2 (Large) suite multiplied by 19.5 m² per
chair-bay (the published figure on each sketch) yields ~234 m² of
clinical area, with the balance going to reception (76.2 m² in M2),
doctor's office (14.3 m² × 2), kitchen/files (7.5 m²), staff room,
sterilization, file room, imaging, mechanical and storage.

## Entry 2026-05-17 — M3 (Small) — 2 dental chairs

**Suite components (cappelletti sestito sketch DRAFT 250512):**
- 1 × doctor's office
- 2 × exam room / dental chair
- 1 × administrative office (with reception "window")
- 1 × waiting area (6 patients)
- 1 × storage room
- 1 × lab
- 1 × staff room
- 1 × atrium / restroom adjacent

**Hand-annotated revisions:**
1. Add a second door for emergencies (egress)
2. Add a "bubble" (waiting area capacity) for 4 people, with 2 each
   way of circulation — confirm maximum capacity
3. Line of sight: reception needs to see the door
4. Move waiting seats down out of Zone 1 (so the façade is reserved
   for clinical Habitat)
5. Add a "bubble" to the office table (clearance for chairs)
6. Extend office into Zone 2

**Equipment list (right margin of the sketch):**
- Mechanical room
- Workbench with 2-3 seats and sinks
- Autoclave (sterilization)
- Imaging room
- File room
- Storage room
- Inventory

## Entry 2026-05-17 — M1 (Medium) — 4 dental chairs

The M1 sketch extends M3 by adding two more exam rooms and
reorganising Zone 2 to accommodate the additional clinical throughput.

**Hand-annotated revisions:**
1. Rotate the reception desk
2. Keep the reception "window" (line of sight)
3. Switch the File Room and Staff Room so the File Room is closer
   to the Reception
4. Move waiting seats down and out of Zone 1
5. In the hallway approach to the washroom: add a sink or seat
   before going into the washroom (vestibule pattern)
6. Increase the number of seats for the Medium and Large variants

## Entry 2026-05-17 — M2 (Large) — 6 dental chairs

**Layout:** M1 plus two additional exam rooms, two doctor's offices
(M3 and M1 each had only one), and a full Zone 2 build-out with
mechanical/electrical, workbench (2-3 seats with sink), file room,
imaging + autoclave, staff room, atrium, and corner storage.

**Hand-annotated revisions:**
1. Move washrooms together — pair them as a single block (M+F or
   universal) rather than spreading them across the suite.

## Entry 2026-05-17 — Magazine depth optimisation

A particularly useful note from the cappelletti sestito sketches:

> NTD: The way to reduce the depth of Zone 2 — Magazine would be to
> choose a smaller table, a narrower table, for the Staff Room, but
> still a real table from Steelcase. To change the dimensions, we
> need to use furniture, real furniture, that is easily accessible,
> widely used and affordable.

The Zone 2 depth at 4.9 m is driven by the Staff Room table depth +
circulation. Narrowing the Staff Room table is the lever for narrowing
the building.

A second NTD:

> NTD: If a tenant truly felt they were being forced to take too
> much space, we could use the Key Plans to check and they could shade
> out the area they did not want, and we could discount the rent
> accordingly — the rates applied to the Magazine should not be the
> same as those applied to the Habitat.

This points toward a **differential rent rate** for Habitat vs
Magazine — a future leasing convention worth capturing in a separate
TOPIC.

## Entry 2026-05-17 — Reception line-of-sight requirement

A recurring theme across all three Medical sketches is "Line of sight
— safety — Reception needs to see the door." This is a soft-functional
requirement that translates into the placement of the reception desk
and the orientation of the doctor's-office wall. The Reception
"window" pattern (M3 admin office adjacent to reception with a
glazed line-of-sight) appears to be the canonical solution.

## Entry 2026-05-17 — AART vs cappelletti sestito

An earlier Medical-Tile design by AART architects (March 25, 2024) is
preserved in `AEC_Floor Plates_Key Plans_Samples_V2.pdf` page 19. The
AART work was superseded by the cappelletti sestito sketches for the
following stated reasons (margin annotation on the AART page):

- "Not people-centered design"
- "Modular rather than 'Small, Medium and Large' based on the needs
  of the business and furniture / equipment layouts"
- "Not real furniture / equipment SKUs"
- "Improper circulation for accessibility"
- "No natural light for the persons in the reception"

The AART layout's reception (22.5 m² Small / 45 m² Medium / 76.2 m²
Large) sized linearly with chair count, which violates the Key Plans
non-modularity principle.

## Future research

- Specialisations: the Directory lists Chiropractor, Dentist, and
  General Practitioner. Only Dentist has completed samples. Are
  Chiropractor and GP layout variants of M1/M2/M3, or do they
  warrant separate sketches?
- ADA-specific dimensions: confirm CSA-B651 corridor width compliance
  and the 1,525 mm wheelchair turn radius at all exam-room doors.
- Acoustic and vibration isolation: the dtcg `design_notes` mention
  these requirements but no specific dB or acceleration limits are
  recorded yet.
- KaVo uniQa SKU change: if the dental-chair line is updated, the
  exam room dimensions cascade. Capture the KaVo SKU as a tracked
  dependency.

## Open questions

1. **Chiropractor and General Practitioner sample timing.** When will
   sketches land for the other two Medical specialisations?
2. **Mechanical-room placement.** M2 (Large) is the only sample to
   show mechanical + electrical explicitly in Zone 2. M3 and M1 sketch
   variants imply a building-core service feed. Confirm whether
   each Medical suite carries dedicated mechanical or shares with
   the building core.
3. **Waiting-room capacity formula.** "6 →10 waiting" appears on
   the M3 and M1 sketches, suggesting the waiting-room scales from
   6 (S) to 10 (M) seats. The M2 (Large) sketch shows 76.2 m²
   reception — what is the seat count? Likely 18-24 by linear
   extrapolation, but no explicit annotation.
