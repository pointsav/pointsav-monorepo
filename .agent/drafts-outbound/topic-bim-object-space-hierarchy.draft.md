---
schema: foundry-draft-v1
draft_id: topic-bim-object-space-hierarchy
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-projects/building-design/bim-object-space-hierarchy.md
created: 2026-07-10T00:00:00Z
author: task@project-bim
cites: [ifc-4-3, ids-1-0, bsdd-v1]
research_done_count: 3
research_suggested_count: 0
open_questions_count: 1
research_provenance: |
  Primary: dedicated Opus architectural consult, Round 5 background workflow wf_9c7ae20f-be0
  (2026-07-10), cross-checking the Woodfine BIM vocabulary (Object, Composition, Zone, Key Plan,
  Tile, Floor Plate) against the published IFC 4.3 schema.
  Secondary: full re-read of the internal Woodfine design-response document
  (CONSTRUCTION_2025_10_31, 9,674 lines) confirming the source document never uses "BIM Object"
  terminology for Key Plans/Tiles/Floor Plates, and treats them as a leasing/planning methodology.
  Corroboration: live inspection of app-privategit-bim's own token category metadata
  (woodfine-bim-library/site-content/categories/09-key-plans.md, 18-tile-system.md), which found
  and fixed a matching error (Tile's ifc_anchor field incorrectly set to IfcZone instead of
  IfcSpatialZone) in the same session this TOPIC was written.
research_inline: false
---

# The BIM Object/Space Hierarchy

## Lede

A recurring error in BIM data modelling is treating a room the same way as a piece of furniture — both are "things a building is made of," so it is tempting to model them on one ladder. IFC 4.3 does not do this, and neither should any system built on top of it. This TOPIC states precisely how the Woodfine BIM Library's vocabulary — Object, Composition, Zone, Key Plan, Tile, Floor Plate — maps onto IFC's own element/space distinction, and where the Library's own site copy previously got this wrong.

---

## Two ladders, not one

IFC 4.3 separates every entity in a building model into two families, both descending from a shared supertype (`IfcProduct`, via `IfcObject`/`IfcObjectDefinition`), but never merging into each other:

- **Elements** — physical products. A desk is `IfcFurniture`; a door is `IfcDoor`. These are things you can pick up, replace, or ship.
- **Spatial elements** — volumes of space. `IfcSpace`, `IfcSpatialZone`, `IfcBuildingStorey`, `IfcBuilding`. These are things you occupy, not things you hold.

The Woodfine BIM Library's own vocabulary maps directly onto this split:

| Library term | Kind | IFC 4.3 analogue |
|---|---|---|
| Object | Element | `IfcFurniture` / `IfcElement`, typed via `IfcTypeProduct` |
| Composition | Element assembly | `IfcElementAssembly` |
| Zone | Spatial subdivision | `IfcSpace` subdivision |
| Key Plan | Space | `IfcSpace` |
| Tile | Spatial grouping | `IfcSpatialZone` |
| Floor Plate | Space | `IfcBuildingStorey` |
| Building | Space | `IfcBuilding` |

Object and Composition are on the element ladder. Zone, Key Plan, Tile, and Floor Plate are on the space ladder. They are not the same ladder wearing two names — they are two different ladders that happen to share a design pattern.

---

## Two "part-of" relationships, not one

The reason the two ladders cannot be merged is that they use different relationships to build up their hierarchy:

- **Aggregation** (`IfcRelAggregates`) — true whole/part decomposition. A Tile is aggregated *from* Key Plans; a Floor Plate is aggregated *from* Tiles. Take the parts away and the whole no longer exists. This relationship is defined at the shared `IfcObjectDefinition` level — both the element ladder and the space ladder use it internally, each on its own rungs.
- **Containment** (`IfcRelContainedInSpatialStructure`) — a physical element is located inside a spatial container. A desk is contained in a room. Take the desk out of the room, and the room is still a room — it is simply an empty one.

A Composition (a furnished room type — an Object assembly) is *contained in* a Key Plan. It is not *aggregated into* the Key Plan the way a Key Plan is aggregated into a Tile. This is the exact distinction that a flattened, one-ladder model erases, and erasing it is a mistake an IFC-literate reader will catch immediately — a desk is not "part of" a room in the same sense that a room is "part of" a floor.

---

## What "one data model" actually means

The Woodfine BIM Library states, correctly, that "the catalog and the space-planning system are one data model, not two products." This claim is true — but only at the shared supertype level, not at the Object level. Every rung on both ladders — Object, Composition, Zone, Key Plan, Tile, Floor Plate — is a placeable, classifiable entity descended from the same `IfcProduct`/`IfcObjectDefinition` root. That shared ancestry is the "one data model." It is not a claim that a Key Plan *is* a kind of Object; it is a claim that Objects and Key Plans are *built the same way* underneath, using the same identity, classification, and aggregation machinery, on two separate ladders.

An earlier working draft of this material stated the claim incorrectly, as "a Key Plan is itself a BIM Object — the smallest leasable one." This is a category error: it treats a space (Key Plan) as if it were an element (Object). The corrected statement is in the table above. The error was live on the Library's own "Method" page and, independently, contradicted by the Library's own homepage — both have since been corrected to state the same, accurate answer.

---

## Atomicity is a granularity choice, not an ontological rule

A Key Plan is not truly indivisible: it decomposes into three Zones, and it contains furniture Compositions. It is "atomic" only in a layer-relative sense — the Tile layer above it chooses to stop caring about a Key Plan's internal structure once the Key Plan itself is defined. This is exactly how IFC treats atomicity generally: an `IfcSpace` is the leaf of the *mandatory* spatial hierarchy, yet it can still be subdivided (via `IfcSpatialZone`) and can still contain a fully decomposed element assembly. "Atomic" means "the level of development at which the layer above stops needing to see internal structure" — a decision made per layer, never an intrinsic property of the thing itself.

---

## The one genuine extension beyond IFC

IFC's spatial-structure hierarchy and aggregation machinery do not, on their own, provide a *self-similar, fractional* tiling system — the eighth/quarter/half/three-quarter/full-floor composability that lets a Tile be built at any of several standard fractions of a Floor Plate. This is a real Woodfine extension, layered on top of `IfcSpatialZone`, not a native IFC mechanism. It is stated here explicitly as an extension, not implied to be part of the IFC standard.

---

## Open question

The exact relationship between a Composition (e.g. "Private Office — Small," a furnished room type of roughly 105 m²) and a Key Plan (the smallest leasable unit) has not yet been pinned down at the data-model level: are they the same rung, described with or without a circulation allowance, or does a single Key Plan aggregate one or more Compositions plus circulation space? Both readings are consistent with everything else in this TOPIC — this is a data-model precision question for a future revision, not an error in the model stated above.

---

## References

- IFC 4.3 — Industry Foundation Classes (ISO 16739-1:2024), buildingSMART International [ifc-4-3]
- Information Delivery Specification (IDS) 1.0, buildingSMART International [ids-1-0]
- buildingSMART Data Dictionary (bSDD) [bsdd-v1]
