---
schema: foundry-draft-v1
draft_id: topic-key-plans-and-tiles
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-projects/building-design/key-plans-and-tiles.md
created: 2026-07-10T00:00:00Z
author: task@project-bim
cites: [ifc-4-3]
research_done_count: 1
research_suggested_count: 0
open_questions_count: 1
research_provenance: |
  Full re-read of the internal Woodfine design-response document
  (CONSTRUCTION_2025_10_31, 9,674 lines), multiple keyword sweeps (key plan, tile, floor
  plate, self-similar, aperiodic, special tile), Round 5 background workflow
  wf_9c7ae20f-be0 (2026-07-10). Line references below are to
  CONSTRUCTION_2025_10_31_Design Slides_Openstudio_Woodfine Response copy 2.md.
research_inline: false
---

# Key Plans and Tiles

## Lede

Woodfine plans a building's space from the furniture out, not the square footage down. This TOPIC states the formal system — Key Plans, Tiles, and how they combine into a Floor Plate — as Woodfine's own design methodology actually defines it, distinguishing what the source material says from what has been layered on top of it elsewhere on this site.

---

## The formal definition

Woodfine's internal design-response document states the system's own definition directly: "Key Plans and Tiles" means a geometric self-similar space planning system based on furniture/equipment arrangements and circulation versus modular area per person progressions. The source document additionally describes the system as "aperiodic" in the same passage — a real quote, reproduced here as a quote, not independently verified as a formal mathematical property of the tiling grammar; readers should not take "aperiodic" as a claim this TOPIC makes on its own authority.

A **Key Plan** is the smallest unit of space worth leasing: a bounded room-scale plan defined by real furniture placement, real circulation, and real daylight — not by an area quota. Key Plans combine into **Tiles** — blocks of Key Plans that serve double duty as the leasing unit and the climate-services zone.

## Tiles are not a simple sum of Key Plans

The source document is explicit that individual Key Plans are not summed together to make a Tile — nor does every Key Plan fit every Tile. At building corners and elevator lobbies, unique "Special Tiles" exist where only certain Key Plans fit. Three basic Tile types are named for leasing purposes — Private Office, Professional Office, and Corporate Office — plus a second category for Amenities (Lobby Atrium, Building Manager, Mail Room, Tenant Lounge, Coffee/Bread, Loading and Recycling).

One documented case makes the Key Plan/Tile relationship concrete: the Tenant Lounge is described in the source material as simultaneously a Key Plan *and* a Tile — the Tenant Lounge Tile is made up of the Tenant Lounge's own washrooms, kitchen, meeting rooms, and lounge furniture. This is the clearest illustration in the source material of a Tile being composed from named sub-elements, and it is consistent with the corrected hierarchy model in the companion TOPIC, "The BIM Object/Space Hierarchy": the Tenant Lounge Key Plan aggregates into its own Tile because, at building-corner or amenity-zone scale, a Tile can be as small as a single Key Plan.

## Fractional sizing

Tiles combine to fill a Floor Plate at fixed fractional sizes: an eighth, a quarter, a half, three-quarters, or a full floor. The source document treats this fractional system as load-bearing for structural planning too — the structural grid is placed around Tiles (and Special Tiles) so that Tiles can always be demised as column-free leaseholds. For at least two building typologies specifically named in the source material — Tech Industrial and Retail Select — the Tile and Floor Plate levels collapse into one: "the Floor Plates are the Tiles," with no intermediate step.

This fractional, self-similar composability is Woodfine's own extension of the underlying spatial-aggregation mechanism — it is not a claim about how any external BIM standard works. See "The BIM Object/Space Hierarchy" for how Tiles relate to IFC 4.3's own spatial-structure concepts [ifc-4-3].

## What the source material does *not* say

Two things worth stating plainly, because they correct framings that have circulated elsewhere on this site: the source document never uses "BIM Object" terminology for Key Plans, Tiles, or Floor Plates — every occurrence of "BIM" in the document refers to Building Information Modelling infrastructure generally, not to this space-planning system specifically. And the document treats Key Plans and Tiles overwhelmingly as a design methodology for generating leasing plans — described in the source material itself as working from "chalk lines," imaginary lines across the Floor Plate showing where a demising wall could go — rather than as a software catalog of discrete, independently-identified objects. Where this site does treat Key Plans and Tiles as catalog-style entities with their own identity and classification, that is this platform's own extension of the source methodology, stated as such.

## Open question

The exact relationship between a Composition (a furnished room type, e.g. "Private Office — Small") and a Key Plan has not been pinned down precisely — whether they are the same rung described with or without a circulation allowance, or whether a Key Plan aggregates one or more Compositions plus circulation space. Both readings are consistent with the source material reviewed for this TOPIC.

---

## References

- IFC 4.3 — Industry Foundation Classes (ISO 16739-1:2024), buildingSMART International [ifc-4-3]
- Woodfine internal design-response document, CONSTRUCTION_2025_10_31 (unpublished primary source; quoted material reported as primary authorial testimony, not externally citable)
