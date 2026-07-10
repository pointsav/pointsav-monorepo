---
schema: foundry-draft-v1
draft_id: topic-ifc-uniclass-classification-grammar
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-projects/building-design/ifc-uniclass-classification-grammar.md
created: 2026-07-10T00:00:00Z
author: task@project-bim
cites: [ifc-4-3, bsdd-v1, cobiev3]
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Real web research (WebSearch/WebFetch) into how established BIM object libraries and
  platforms introduce their classification vocabulary — Autodesk Revit family/LOD documentation,
  BIMobject, NBS National BIM Library / BIM Object Standard, Graphisoft GDL, Bentley iTwin
  developer portal, bimstore — conducted as part of the Round 5 background workflow
  (wf_9c7ae20f-be0, 2026-07-10). Corroborated against this site's own live catalog data
  (/api/tokens.json) to confirm which conventions this Library already surfaces.
research_inline: false
---

# Classification Grammar: IFC, Uniclass, and Property Sets on This Site

## Lede

Every Object card on this site carries an IFC class badge and a Uniclass code chip. This is not decoration — it is the same vocabulary a specifier in any established BIM object library already expects, and this TOPIC explains what it means and why this site leads with it.

---

## Why classification, not just description

Across the established BIM object-library landscape, a consistent vocabulary recurs regardless of which platform is being used: an entity class (what kind of thing this is, in a tool-neutral sense), a classification code (where this thing sits in an industry-wide taxonomy), and a set of grouped properties (the specific facts that classification implies should be present). A library that surfaces this vocabulary directly — rather than hiding it behind marketing copy — reads immediately as built for specification, not just browsing.

## IFC entity class

Every Object on this site carries an IFC 4.3 entity class — for example, `IfcFurnishingElement` for furniture [ifc-4-3]. This is the vendor-neutral identity that lets a wall, a desk, or a door be recognised the same way regardless of which authoring tool eventually consumes it. It is the same classification concept Autodesk's own family system organises around (a "loadable family" — a component purchased, delivered, and installed, such as furniture or fixtures) and that Graphisoft's GDL library-part format anchors to (a library part carries both the 3D geometry and the 2D plan symbol that represents it). This site's IFC class badge is the same identity concept, stated in the vendor-neutral standard rather than a proprietary one.

## Uniclass 2015 codes

Alongside the IFC class, every Object carries a Uniclass 2015 code — for example, `Pr_40_50_21_59` for an office desk, drawn from the Products (Pr) table. Uniclass is the classification system authored by NBS and aligned to ISO 19650; NBS's own BIM Object Standard states plainly that classification structures the data "irrespective of software platform" — precisely the property this site's own vendor-neutral positioning depends on. Space types carry the parallel Spaces/Locations (SL) code — for example, `SL Private office spaces` on a Composition — which is the same classification grammar applied one level up, to spaces rather than products.

## Property sets

Beyond identity and classification, each Object's specification table states real, sourced facts: manufacturer, model, SKU, dimensions, clearance requirements, weight. This is the same property-set concept IFC formalises as `Pset_*` groupings — a named bundle of properties expected for a given entity class. NBS's own BIM Object Standard requires exactly this kind of structured property data, organised by IFC class and property set, "irrespective of software platform." This site's spec tables are not yet explicitly grouped by property-set name on the page — that refinement is tracked separately — but the underlying data is already structured that way.

## Handover data

Where a Composition or Object's data needs to travel into a facility-management system rather than stay in a design tool, the relevant open format is COBie — Construction Operations Building Information Exchange — a standard asset-handover spreadsheet format implemented via open tooling [cobiev3]. This site's machine-readable data (`/api/tokens.json`, DTCG token files) is structured to be consumable by exactly this kind of downstream handover pipeline, not only by a design-time viewer.

## Identity across tools

The buildingSMART Data Dictionary (bSDD) provides a further layer this site's underlying data model depends on but does not yet surface directly on every page: stable, multilingual, tool-neutral concept identifiers that give an element type a durable identity independent of any single authoring tool's internal representation [bsdd-v1]. Where an IFC class and a Uniclass code answer "what kind of thing is this, and where does it sit in a product taxonomy," a bSDD URI answers "what is this thing's identity, resolvable the same way in any tool that looks it up" — the three layers are complementary, not redundant.

---

## References

- IFC 4.3 — Industry Foundation Classes (ISO 16739-1:2024), buildingSMART International [ifc-4-3]
- buildingSMART Data Dictionary (bSDD) [bsdd-v1]
- COBie v3 — Construction Operations Building Information Exchange [cobiev3]
- NBS — NBS BIM Object Standard (accessed via real web research, 2026-07-10)
