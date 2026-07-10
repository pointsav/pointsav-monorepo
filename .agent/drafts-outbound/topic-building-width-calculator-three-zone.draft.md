---
schema: foundry-draft-v1
draft_id: topic-building-width-calculator-three-zone
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-projects/building-design/building-width-calculator-three-zone.md
created: 2026-07-10T00:00:00Z
author: task@project-bim
cites: [ashrae-90-1, bco-guide-to-specification, well-v2-daylight-modeling]
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Primary: real web research (WebSearch/WebFetch) into general architectural space-planning
  literature — ASHRAE perimeter-zone HVAC guidance, the British Council for Offices' Guide to
  Specification, LEED v4 and WELL Building Standard v2 daylight-zone provisions — conducted as
  part of the Round 5 background workflow (wf_9c7ae20f-be0, 2026-07-10).
  Secondary: full re-read of the Woodfine internal design-response document
  (CONSTRUCTION_2025_10_31), confirming the three zones are introduced in the source material
  specifically inside its Building Width Calculator walkthrough (L1285-1302), not as an
  independent general statement.
research_inline: false
---

# The Building Width Calculator and the Three-Zone Cross-Section

## Lede

Every Key Plan resolves into three zones — Habitat, Magazine, Corridor. This is presented on this site as Woodfine's own design methodology. It is worth stating precisely what part of that is a well-established general principle of architectural space planning, and what part is Woodfine's own specific contribution — because conflating the two either understates real precedent or overstates a proprietary claim, and this site aims to do neither.

---

## The general principle is real and well-established

Organising a floor plate's depth, working inward from the facade, into a daylight-adjacent zone, a flexible interior zone, and a circulation zone is a long-established principle in building science and commercial space planning — not a Woodfine invention. Three independent sources corroborate this:

- **ASHRAE's energy-modelling guidance** treats a standard perimeter-zone depth as a routine input to HVAC zoning strategy, separating a daylight/perimeter zone from a deeper interior "core" zone for load-calculation purposes [ashrae-90-1].
- **The British Council for Offices' Guide to Specification** — a widely used commercial-office design reference in the United Kingdom — explicitly separates a daylight/ventilation perimeter zone from a core zone sized for building services, with floor-plate-depth guidance built on that same split [bco-guide-to-specification].
- **LEED v4 and the WELL Building Standard v2** both formalise a "daylight zone" as a function of distance from glazing; WELL v2's Daylight Modeling feature specifically targets getting the large majority of occupants within a defined distance of a window [well-v2-daylight-modeling] — the same daylight-depth-from-facade logic Zone 1 encodes here.

## What is Woodfine-specific

No general source reviewed for this TOPIC codifies exactly *three* zones under the names Habitat, Magazine, and Corridor, or ties them one-to-one to a specific building-width-calculation tool. General literature's middle "core" zone is typically defined by building services — elevators, mechanical risers, restrooms — not by storage, which is the specific programmatic choice this site's Zone 2 ("Magazine") makes. That mapping of the middle zone's purpose is Woodfine's own operationalisation of the general perimeter/core/circulation structure, not a citation to external practice, and it is presented as such here rather than as an industry-standard term already in use elsewhere.

Woodfine's internal design-response document supports this framing directly: the three zones are introduced in that source material specifically as the output of analysing the Building Width Calculator — sample Key Plans are tabulated first, and the three zones are derived from that analysis, not stated as an independent general theory. This site's own operational coupling of the three zones to the Calculator reflects that same design process; it is not a looser paraphrase.

## The three zones

**Zone 1 — Habitat.** Held within six metres of the building perimeter so every workstation gets natural light — the daylight-adjacent zone in the general literature above, given a specific depth threshold and a Woodfine name.

**Zone 2 — Magazine.** Storage and flexible depth, sized empirically from real Key Plans rather than by formula — Woodfine's specific programmatic choice for the "core" zone that general literature typically assigns to building services instead.

**Zone 3 — Corridor.** Circulation, sized by its own Key Plan. Zones 1 and 2 mirror each other across Zone 3. The building's width is not assumed; it is computed outward from these three zones.

---

## References

- ASHRAE Standard 90.1 — Energy Standard for Buildings Except Low-Rise Residential Buildings [ashrae-90-1]
- British Council for Offices — Guide to Specification [bco-guide-to-specification]
- WELL Building Standard v2 — Feature: Daylight Modeling [well-v2-daylight-modeling]
- Woodfine internal design-response document, CONSTRUCTION_2025_10_31 (unpublished primary source)
