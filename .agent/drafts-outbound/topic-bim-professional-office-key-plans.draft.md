---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-professional-office-key-plans-2026-05-17
language_protocol: TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/professional-office-key-plans.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
research_done_count: 4
research_suggested_count: 5
open_questions_count: 3
research_provenance:
  - V3 Master Summary — `inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Summary_Notes copy.pdf` page 1 ("Initial Design" column)
  - Key Plans Notes V2 — `inputs/project-bim/--- March 03, 2025 -- Collaborators #32 --- /AEC_Floor Plates_Key Plans_Notes.pdf` (definition of Professional Office tenant categories)
  - DTCG token — `woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json` key `bim.zone.professional-office` (Z1 6.0 / Z2 3.8 / Z3 2.0 baseline)
  - DTCG token — `woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json` key `bim.key-plan.professional-office.small` (130.06 m² / 1,400 SF)
research_inline: true
notes_for_editor: |
  Living document — Professional Office is the **superordinate
  category**, not a Key Plan itself. It carries an "Initial Design"
  baseline (6.0 / 3.8 / 2.0 m) and a single Small footprint
  (130 m² / 1,400 SF) that serves as the missing-size bridge between
  the five sub-types and Corporate Office 1/8 Floor. Medium and Large
  Professional Office samples are not in any source document.
---

# Professional Office — Key Plans (baseline)

Professional Office is the **superordinate category** for the five
sub-types: Business, Medical, Laboratory, Academic, and Civic. It is
itself a use type with an "Initial Design" baseline that pre-dates
the sub-type specialisation and is preserved as the design lineage.

## Entry 2026-05-17 — The Initial Design baseline

The V3 Master Summary table reserves a column labelled
"Initial Design" that captures the original Professional Office
sample before the five-sub-type taxonomy was articulated:

| Field | Value |
|---|---|
| Small | 130 m² / 1,400 SF |
| Medium | (not sampled) |
| Large | (not sampled) |
| Zone 1 — Habitat | 6.0 m / 19'-8" |
| Zone 2 — Magazine | 3.8 m / 12'-5" |
| Zone 3 — Corridor | 2.0 m / 6'-6" |

Computed building width = 2 × (6.0 + 3.8) + 2.0 = **21.6 m** (70'-10").

## Entry 2026-05-17 — Tenant categories

The Notes V2 document lists eleven overarching tenant categories
that the Key Plans system must accommodate. Five of these are
Professional Office sub-types:

```
Private Office
Professional Office — Business
Professional Office — Medical
Professional Office — Laboratory
Professional Office — Academic
Professional Office — Civic
Corporate Office — 1/8 Floor
Corporate Office — 1/4 Floor
Corporate Office — 1/2 Floor
Corporate Office — 3/4 Floor
Corporate Office — Full Floor
```

A generic "Professional Office" tenant — one that does not fall into
any of the five specialisations — would use the Initial Design
baseline at 130 m² / 1,400 SF. In practice this is rare: most
professional services firms self-identify as Business; most clinical
practices as Medical; most research operations as Laboratory.

## Entry 2026-05-17 — Why Small + Small combinations matter

The Notes V2 includes a critical observation about size gaps:

> *In the case of Professional Offices, combining either Small + Small
> or Small + Medium provides for the missing office size between a
> Professional Office Large and Corporate Office 1/8 Floor.

The combinations table in the V3 Summary captures this:

| Combination | Lab | Academic | Business | Medical | Civic |
|---|---:|---:|---:|---:|---:|
| Small + Small | 390 m² / 4,198 SF | 210 / 2,262 | 622 / 6,700 | 446 / 2,402 | 540 / 5,824 |
| Small + Medium | 511 / 5,500 | 345 / 3,714 | 711 / 7,652 | 554 / 5,969 | 847 / 9,127 |
| Small + Large | 596 / 6,412 | 483 / 5,201 | 980 / 10,874 | 709 / 7,633 | 1,092 / 11,762 |
| Medium + Large | 717 / 7,714 | 618 / 6,653 | 1,069 / 11,826 | 817 / 8,799 | 1,399 / 15,065 |

Combinations are how a Professional Centre fills a floor plate without
leaving gaps. Two Small Lab suites (390 m²) plus one Medium Medical
suite (331 m²) plus one Large Civic suite (822 m²) = 1,543 m² —
which fits inside a typical 19,000–23,000 SF Professional Centre
floor with room for the building core, amenities, and elevator
lobby.

## Entry 2026-05-17 — The Initial Design vs sub-type tension

The Initial Design Z2 = 3.8 m is **narrower than every sub-type
Z2 except Private Office**. The five sub-types push Z2 outward:

| Sub-type | Z2 Magazine | Delta vs Initial Design |
|---|---:|---:|
| Initial Design | 3.8 m | (baseline) |
| Medical | 4.9 m | +1.1 m |
| Laboratory | 4.8 m | +1.0 m |
| Academic | 3.0 m | −0.8 m |
| Business | 7.3 m | +3.5 m |
| Civic | 7.2 m | +3.4 m |

The Initial Design Z3 = 2.0 m is **narrower than every sub-type
Z3 except Private Office and Academic** (which have Z3 = 0 because
suites open to the building corridor). Sub-type Z3 ranges from
2.7 m (Business) to 3.6 m (Civic).

The implication: the Initial Design **cannot satisfy any of the five
specialisations** without dimensional expansion. The 21.6 m baseline
exists as the "Professional Office without specialisation" footprint
— a tenant who needs a 1,400 SF generic office. As soon as the tenant
declares a specialisation, the sub-type's Building Width Calculator
applies.

## Entry 2026-05-17 — The 21 m vs 21.6 m discrepancy

The Floor Plate methodology document (Methodology PDF) presents the
Professional Office total floor-plate width as **21 m** (centerline)
and **21.6 m** when the demising and structural walls are added.
The token arithmetic 2 × (6.0 + 3.8) + 2.0 yields exactly 21.6 m,
suggesting the "21 m" figure is the centerline-to-centerline
distance and 21.6 m includes a **0.6 m demising-structural overhead**.

This 0.6 m overhead should be encoded as a separate DTCG token —
`bim.tolerance.demising-structural` = 0.6 m — to make the arithmetic
self-documenting. (See `plans/tool-buildingwidth-architecture.md`
inconsistency #6.)

## Entry 2026-05-17 — Why no Professional Office Medium / Large samples

No source document carries a Professional Office Medium or Large
sample at the Initial Design baseline. The reason is captured in
the V3 Summary structure: the **sub-type Medium and Large** roles
are filled by the five specialisations' Medium and Large samples.

A "Professional Office Medium" tenant who hasn't declared a
specialisation would, in practice, lease two Smalls (Small + Small =
260 m² / 2,800 SF — see V3 Summary combinations).

## Future research

- The 130.06 m² Professional Office Small key plan area is recorded
  in the DTCG file but no dimensioned drawing accompanies it.
  Locate or commission the sample.
- The "Small + Small = 260.12 m² / 2,800 SF, filling the gap
  between Professional Office Large and Corporate Office 1/8 Floor"
  observation in the DTCG `$description` implies the gap-fill
  function. Confirm with a Tile-level diagram showing how this
  combination tiles.
- Confirm whether the Initial Design footprint is still leasable in
  the current building program or whether all Professional Office
  tenants now self-identify as a sub-type.
- The Corporate Office 1/8 Floor area is not specified in any source
  document examined. Compute or locate it.

## Open questions

1. **Z2 = 3.8 m baseline.** The Initial Design Z2 is narrower than
   four of five sub-types. Is this baseline still used in new buildings
   or is it deprecated?
2. **Demising-structural 0.6 m overhead.** Is this a fixed value
   across all sub-types or does it vary with corridor type / IBC
   occupancy classification?
3. **130.06 m² (4 decimal precision)** — what drove the precision?
   The other key plan areas are integer m² or one-decimal precision.
   Is this a CAD-extracted value that should be rounded to 130 m² for
   display?
