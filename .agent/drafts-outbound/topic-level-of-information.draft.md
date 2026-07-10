---
schema: foundry-draft-v1
draft_id: topic-level-of-information
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-projects/building-design/level-of-information.md
created: 2026-07-10T00:00:00Z
author: task@project-bim
cites: [ifc-4-3]
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Real web research (WebSearch/WebFetch) into the industry-standard Level of Development
  (LOD) 100-500 scale (Autodesk's published LOD documentation) and how it is used to calibrate
  trust in a model element by project phase, conducted as part of the Round 5 background
  workflow (wf_9c7ae20f-be0, 2026-07-10). Corroborated against this site's own live catalog
  data and rendering code (has_zone_data, bill/parts-list linkage) to confirm what maturity
  states this Library already tracks and surfaces.
research_inline: false
---

# Level of Information: How This Library States Catalog Maturity

## Lede

Not every entry in this Library carries the same amount of real, verified data. Rather than hide that, this site states it — a Composition with a fully linked parts list is a different, more mature kind of record than one with only a room programme on file. This TOPIC names the maturity states this Library actually tracks and relates them to the industry's own established Level of Development (LOD) scale, so a visitor can calibrate trust in what they are looking at.

---

## The industry's own scale

Building Information Modelling practice widely uses a Level of Development (LOD) scale, running from LOD 100 (conceptual — a symbol or generic representation) through LOD 200 (approximate geometry, generic placeholder), LOD 300 (accurate geometry and location, suitable for coordination), LOD 350 (LOD 300 plus interfaces to other building systems), LOD 400 (fabrication-ready detail), to LOD 500 (as-built, field-verified). Practitioners use this scale routinely to decide how much to trust a given model element at a given project phase — a schematic-design element at LOD 100 should not be relied on for a fabrication decision, and that is by design, not a defect.

## This Library's own maturity states

This site does not yet publish a formal LOD number per entry, but it already tracks and displays three real, distinct maturity states, and this TOPIC names them against the LOD framework above so the correspondence is explicit rather than left for a visitor to infer:

**Fully specified.** An Object or Composition with real dimensions, manufacturer data, IFC classification, Uniclass code, and — for Compositions — a fully linked parts list, where every furniture item in the room programme resolves to a real catalog Object. This corresponds to LOD 300-350: accurate, coordinated, verifiable geometry and identity.

**Partially linked.** A Composition with a real, specific furniture programme on record, where some but not all of the named items resolve to a real catalog Object — displayed as "Object linking: N of M — pending." This is an honest LOD 200-equivalent state for the unresolved items specifically: the design intent is real and specific, but the catalog does not yet carry a matching product for every named piece. This is not a placeholder awaiting authorship so much as a live record of exactly which real furniture SKUs the catalog is still missing.

**Room programme only.** A Composition with named rooms on record (for example, "2 exam rooms, file room, autoclave") but no furniture-level programme authored yet — displayed as "Room programme only — furniture layout not yet authored." This corresponds to LOD 100: a real, confirmed spatial programme exists, but furniture-level detail does not yet.

**Floor-scale, not zone-modelled.** A Corporate Office leasehold, sized as a defined fraction of a Floor Plate, where the interior furniture layout is deliberately not modelled because the tenant's own architect designs that layout — displayed as "Leasehold sized as a fraction of the Floor Plate — tenant designs interior layout." This is not a lower-maturity state on the same scale as the three above; it is a different *kind* of record entirely — a leasehold-area specification rather than a furniture-level space plan — and the honest-pending framing exists so it is not mistaken for an incomplete version of the other three states.

## Why this matters

A specifier who understands LOD already knows to ask "how far along is this element, and can I rely on it for the decision I'm making right now?" Naming this Library's own maturity states in that same vocabulary — rather than leaving "pending" undifferentiated — lets that same specifier calibrate trust in one glance, the way the industry-standard LOD scale is designed to let them do everywhere else.

---

## References

- IFC 4.3 — Industry Foundation Classes (ISO 16739-1:2024), buildingSMART International [ifc-4-3]
- Autodesk — Levels of Development in BIM (accessed via real web research, 2026-07-10)
