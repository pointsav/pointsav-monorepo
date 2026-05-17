---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-private-office-key-plans-2026-05-17
language_protocol: TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/private-office-key-plans.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
research_done_count: 5
research_suggested_count: 3
open_questions_count: 2
research_provenance:
  - DISCOVERY Private Office sketches — `inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Private Office_Notes copy.pdf` (PO-1/PO-2/PO-3 layouts with mullion lines, corridor reference, Steelcase Planning Ideas reference imagery)
  - Key Plans Samples V2 — `inputs/project-bim/--- March 03, 2025 -- Collaborators #32 --- /AEC_Floor Plates_Key Plans_Samples_V2.pdf` pages 2-4 (PO-1, PO-2, PO-3 dimensioned drawings)
  - Key Plans Notes V2 — `inputs/project-bim/--- March 03, 2025 -- Collaborators #32 --- /AEC_Floor Plates_Key Plans_Notes.pdf` (PO-1 furniture list, Zone derivation rationale)
  - V3 Master Summary — `inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Summary_Notes copy.pdf` page 1 (Private Office totals)
  - DTCG token — `woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json` keys `bim.zone.private-office` and `bim.key-plan.private-office`
research_inline: true
notes_for_editor: |
  Living document — Private Office is the use type most thoroughly
  worked out and the only one where all three sizes (Small/Medium/Large)
  have completed dimensioned drawings. The PO-1 sample is the
  canonical methodology demonstration referenced throughout the
  Key Plans system.
---

# Private Office — Key Plans

Private Office is a separate use type from Professional Office. Each
private office is a single licensable suite that opens directly onto
the shared building corridor — there is no internal bilateral corridor
within the Private Office floor area. The "Tenant" is typically an
individual professional rather than a firm.

## Entry 2026-05-17 — The three sizes

| Size | Code | Area (m²) | Area (SF) | Façade frontage |
|---|---|---:|---:|---|
| Small | PO-1 | 30 | 325 | 13'-5" (~4.09 m) |
| Medium | PO-2 | 43 | 465 | 19'-3" (~5.87 m) |
| Large | PO-3 | 64 | 685 | 28'-6" (~8.69 m) |

Total when stacked side-by-side: **137 m² / 1,475 SF** (one of each
size).

## Entry 2026-05-17 — Zone vocabulary

Private Office uses the same three-zone vocabulary as Professional
Office, but with distinctive numeric values:

| Zone | Depth | Note |
|---|---:|---|
| Zone 1 — Habitat | 5.9944 m (19'-8") | Marginally under the 6.0 m European Lighting Standard; V3 table displays 6.0 m / 19'8" |
| Zone 2 — Magazine | 1.3716 m (4'-6") | Shallow storage; V3 table displays 1.4 m / 4'6" |
| Zone 3 — Corridor | 0.0 m | None — each office opens to the shared building corridor |

The 5.9944 m Habitat is **flagged for review** at final Key Plan
sign-off. The 1.3716 m Magazine is intentionally shallow because
Private Offices carry minimal shared storage compared with multi-person
Professional Offices.

## Entry 2026-05-17 — PO-1 (Small) — the canonical sample

The PO-1 sample is the methodology demonstration that anchors the
entire Key Plans system. The Notes V2 document walks through its
derivation step-by-step:

> We start by putting a dot in the middle of a blank page.
> We will then begin laying out the furniture around the dot in the
> middle of the blank page. Then we are going to move the furniture
> around the dot in order to achieve the German Circulation Law,
> along with the European Lighting Standard.

**Furniture (1 desk PO-1):**
- 1 × desk and chair (Steelcase reference SKU)
- 1 × 3-person round table
- 1 × filing cabinet
- 1 × credenza
- 1 × bookshelf
- 1 × coat rack

**Dimensioned drawing (Samples_V2 page 2):**
- Façade frontage: 13'-5"
- Habitat depth: 19'-8" (5.9944 m)
- Magazine depth: 4'-6" (1.3716 m)
- Total office depth (excluding corridor): 24'-2"
- Two mullions span the façade
- Door swings inward from the corridor side

**Occupancy and workstation density:**
- Workstations: 1 / 30 m² (325 SF) per workstation
- Occupancy: 6 / 5 m² (54 SF) per person — this is the assembly
  occupancy load, used for IBC egress calculations, not the actual
  daily occupancy

## Entry 2026-05-17 — PO-2 (Medium)

**Furniture (2 desks PO-2):** PO-1 plus a second desk and chair and
a second credenza. Filing cabinet, bookshelf, coat rack, 3-person
round table remain at quantity 1.

**Dimensioned drawing (Samples_V2 page 3):**
- Façade frontage: 19'-3" (split 10'-0" + 9'-3" between the desk
  workzones)
- Habitat depth: 19'-8"
- Magazine depth: 4'-6"

**Occupancy:**
- Workstations: 2 / 22 m² (233 SF) per workstation
- Occupancy: 6 / 7 m² (77 SF) per person

**The non-modularity principle.** PO-2 is approximately 100 SF
larger than PO-1, not double. The reason is that PO-2 still carries
one 3-person round table, one filing cabinet, one bookshelf, and one
coat rack — the shared furniture does not duplicate. From the Notes:

> Private Office – Small (PO-1) may have to be 300-400 square feet to
> allow for the proper width and Facade Frontage, and Private Office –
> Medium (PO-2) may only need to be ~100 square feet larger to
> accommodate the second person.

This non-modularity is foundational to the entire Key Plans system —
the per-person rentable area is **not** the design driver. Quality of
the space is.

## Entry 2026-05-17 — PO-3 (Large)

**Furniture (3 desks PO-3):** Two filing cabinets, two bookshelves,
two coat racks added relative to PO-2 (per the V1 Professional Office
furniture availability table in Samples_V2 page 9 — "Office – Filing
Cabinet 1/1/2, Bookshelf 1/1/2, Coat Rack 1/1/2").

**Dimensioned drawing (Samples_V2 page 5):**
- Façade frontage: 28'-6" (split 10'-0" + 9'-3" + 9'-3")
- Habitat depth: 19'-8"
- Magazine depth: 4'-6"

**Occupancy:**
- Workstations: 3 / 21 m² (228 SF) per workstation
- Occupancy: 7 / 9 m² (97 SF) per person

PO-3 represents three professionals sharing a suite that retains the
single 3-person round table — typically a partnership or a small
practice with two professionals plus an administrator.

## Entry 2026-05-17 — Furniture availability per tile

The V1 Professional Office furniture availability table
(`Samples_V2.pdf` page 9, dated July 3, 2024) records the distribution
of Private Office sizes across a Private Office Tile:

| Size | Share of a tile |
|---|---:|
| Small (PO-1) | 80% |
| Medium (PO-2) | 10% |
| Large (PO-3) | 10% |

Most Private Office tiles will be predominantly PO-1 cells. Tenants
who want PO-2 or PO-3 space rent it explicitly; the building optimises
for PO-1 throughput.

## Entry 2026-05-17 — Licensing vs leasing

The Notes V2 document is explicit that Private Office tenants are
likely to operate under a **licensing agreement** rather than a
traditional lease:

> The rent for Private Offices (maybe even the Professional Offices)
> is going to be fixed pricing – there are small, medium and large
> offices at three different prices. Tenants of the Private Offices
> will not have their Common Area Costs broken out. We are
> contemplating that they have a "licencing agreement" with the
> building rather than a Lease Agreement.

Building IDs are issued per desk. A PO-1 tenant gets one Building ID;
a PO-2 tenant gets two; a PO-3 tenant gets three (or one main +
two additional).

## Future research

- Confirm with operator: does each PO-1/2/3 carry a unique IFC entity
  identifier or is "Private Office" a single IFC entity with a
  size enumeration? Aligns with the IFC 4.3 + bSDD URI strategy.
- Furniture SKU mapping: the Steelcase Ology (762 mm) desk depth is
  the Zone 1 anchor. Confirm whether Private Office uses the same
  desk or a different Steelcase line.
- Adjust PO-1 façade frontage if a final Key Plan sign-off pushes the
  area into the 300–400 SF range (the Notes acknowledge this
  flexibility).

## Open questions

1. The "300-400 SF" range cited in the Notes suggests PO-1 may be
   adjusted from the sample's 325 SF up to 400 SF on the final Key
   Plan. What is the threshold criterion — Façade Frontage, desk
   depth, or compliance with a specific Local Code?
2. The Habitat depth 5.9944 m is sub-6.0 m and flagged for review.
   What is the resolution path — push to 6.0 m exactly (with attendant
   PO-1 area increase) or accept 5.9944 m with an EN 17037 daylight
   compliance memo?
