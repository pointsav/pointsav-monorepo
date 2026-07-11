---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.2.1"
title: "Site Context Overlays: Decoupling Regulatory and Environmental Assessment from Reusable Building Compositions"
abstract: |
  Architects re-author the same functional space types — a stairwell, a private office, a
  residential unit — on every project, and re-verify code compliance for each instance from
  first principles, because building-code and site-environmental data are conventionally authored
  as prose reports attached to a single project rather than as structured data attached to a
  reusable design. This paper specifies an architecture that separates two concerns conventionally
  merged: a Composition (a reusable functional-space design, jurisdiction-agnostic by
  construction) and a Site Context Overlay (jurisdiction- or location-specific regulatory and
  environmental data, authored once per place and applied to any Composition deployed there).
  Compliance and site-context status become a derived, computed Assessment — never hand-authored,
  always re-derivable — rather than a property baked into either the Composition or the Overlay.
  We specify a closed four-category schema for Site Context Overlays (Regulatory and Entitlement;
  Hazards and Structural Loads; Climate and Energy; Ground and Ecology) covering the layer types
  an architect or engineer needs at a site, and a self-hosted extension model in which
  practitioners maintain their own Composition libraries against a shared base catalogue without a
  centrally hosted accounts system. We ground the spatial-hierarchy terminology in IFC 4.3's
  existing element/space distinction, correcting an earlier internal working assumption that
  conflated the two. The paper closes with falsifiable claims about what this architecture makes
  possible that a project-scoped, prose-based compliance workflow structurally cannot.
target_journal: "Journal of Information Technology in Construction (ITcon)"
target_publisher: "International Council for Research and Innovation in Building and Construction (CIB)"
impact_factor: "3.6"
alternate_venue: "Automation in Construction (Elsevier, IF 12.0)"
authors:
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: jmwoodfine@gmail.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Formal Analysis
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Conceptualization
      - Validation
      - Writing – Review & Editing
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Software
      - Data Curation
      - Writing – Review & Editing
subject_codes:
  - "TH9 Construction management and practice"
  - "T058 Computer-aided design"
  - "QA76.9.I52 Geographic information systems"
keywords:
  - jurisdiction-portable BIM
  - regulatory compliance automation
  - IFC spatial structure
  - composable geometry
  - self-hosted BIM
  - site context data
  - building code as data
bcsc_class: public-disclosure-safe
ai_tool_used: "claude-sonnet-5 (Anthropic)"
corresponding_author: jmwoodfine@gmail.com
word_count_body: 3100
word_count_target: 6000
submission_status: not-submitted
language_pass_date: ""
cites:
  - ifc-4-3
  - ids-1-0
  - bsdd-v1
  - ifc-fragment-spec
  - corenet-x-2021
  - ashrae-90-1
  - bco-guide-to-specification
  - well-v2-daylight-modeling
  - eupl-1-2
forbidden_terms_cleared: false
section_status:
  abstract: complete
  s1_introduction: complete
  s2_literature_review: complete
  s3_methodology: complete
  s4_results: complete
  s5_discussion: complete
  s6_limitations: complete
  s7_conclusion: complete
  formal_hypotheses: complete
  falsification_programme: complete
refs_status:
  count: 9
  quality: adequate-for-v0.1
  blockers:
    - "The self-similar/composable Tile-fraction claim (§4.2) is sourced to an unpublished internal design-response document (primary authorial testimony, not externally citable) — same evidence-class treatment as the companion flat-file-substrate paper; not resolvable by external citation since the source is not public"
    - "No falsification test has yet been executed — programme is specified (§9) but unexecuted"
    - "ORCID IDs required before submission (all three authors) — operator-supplied, not something we can complete ourselves"
scaffolded_from:
  - .agent/briefs/BRIEF-bim-v3-hyperscaler-redesign.md (Round 4, Parts A/A.5/B/C)
  - .agent/drafts-outbound/topic-city-code-as-composable-geometry.draft.md (related prior work, distinct mechanism)
scaffolded_date: 2026-07-10
writing_pass_date: 2026-07-10
preprint_posted: false
preprint_posted_date: ""
doi: ""
license: ""
cite_as: ""
revision_history:
  - version: "0.1"
    date: "2026-07-10"
    changes: "Initial draft. Realizes the previously-unbuilt Round 4 architecture design (Jurisdiction/Site Context Overlay data model, 4 master categories, architect-signature self-hosted reuse) as a JOURNAL manuscript, per the operator's direction to write this up once the master-category consolidation settled. Corrected an internal terminology error surfaced by this session's Opus consult: an earlier working synthesis described a Key Plan as itself 'a BIM Object' — this draft instead uses the IFC-precise two-ladder model (element/product ladder vs. spatial ladder, joined by containment, unified at the IfcProduct supertype) and flags the correction explicitly in §4.1."
  - version: "0.1.1"
    date: "2026-07-10"
    changes: "Resolved the related-work citation-boundary question raised in v0.1: decided definitively to cite JOURNAL-aec-data-layers-v0.1.draft.md as related work (§3.3) rather than re-derive its per-country layer sourcing independently, since re-deriving would risk the two papers silently drifting on the same factual claims. This is now a stated decision, not an open question for editorial."
  - version: "0.2"
    date: "2026-07-10"
    changes: "Corrected a real error in Table 2 (§4.1) found by a dedicated adversarial Opus review of the corrected hierarchy model: `IfcZone` was listed as a Tile analogue alongside `IfcSpatialZone`, but `IfcZone` is a subtype of `IfcGroup` (joined via `IfcRelAssignsToGroup`), not a placeable/aggregatable `IfcProduct`-descended entity — its presence directly contradicted this section's own claim that every rung is placeable and aggregatable. Removed `IfcZone`; Tile now maps to `IfcSpatialZone` exclusively; Zone (one level down) restated as an `IfcSpace` subdivision rather than reusing `IfcSpatialZone` for two different rungs. Also corrected the convergence sentence: placement/classification converge at `IfcProduct`, but the shared aggregation mechanism (`IfcRelAggregates`) is defined one level higher, on `IfcObjectDefinition` — the prior wording conflated the two."
  - version: "0.2.1"
    date: "2026-07-10"
    changes: "Structural conformance to the wiki render contract (guide-journal §9) plus an authorial voice pass. Structural: moved the abstract into frontmatter (`abstract: |`) and deleted the body `## Abstract` section; deleted the body-level h1 title, author block, and keywords line so the body now opens directly at `## 1. Introduction`; deleted the hand-typed `## References` section entirely (the engine generates references from `cites:` plus citations.yaml). No in-text bracket-ID citation, table row, IFC-entity name, hypothesis, or falsification-test text was changed — the corrected two-ladder terminology model (§4.1, Table 2) is preserved exactly. Voice: sentence-level pass over the prose to remove generic-academic filler (hedge-stacking, empty transitions, over-symmetric constructions), tightening toward the direct, declarative register of the authors' own primary-source design-response document, while preserving every factual claim, every citation, and every planned/delivered hedge distinction. Section-opening sentences rewritten where they labelled a topic rather than advancing the single thesis. No facts, quotes, or citations added or removed."
notes_for_editor: |
  This is the third bim-surface JOURNAL seed (after desktop-environment and the merged
  flat-file-bim-substrate paper), bringing the surface to 3 of the registry's ~3–5 soft cap —
  within bounds per the 2-3-4 ceiling the operator set for this surface. Not proposing a 4th
  without your sign-off first.

  One item remains genuinely open, not resolvable on our side: **terminology correction
  mid-programme.** An earlier working synthesis this session (now superseded, see
  revision_history) described the Key Plan → Tile → Floor Plate hierarchy using "BIM Object" for
  spatial units. A dedicated architectural consult found this conflates IFC's element/product
  side (`IfcElement`, our "Object") with its spatial side (`IfcSpace`/`IfcSpatialZone`, our "Key
  Plan"/"Tile"/"Floor Plate") — a real BIM-101-level error that would cost credibility with an
  IFC-literate reviewer. This draft uses the corrected model throughout (§4.1). The site's own
  on-page copy and the "Discipline" section still carry the uncorrected framing — that on-site
  content fix is a separate piece of follow-up work, tracked in the archive's BRIEF, not
  something this JOURNAL draft can reach on its own.

  The related-work citation-boundary question from v0.1 is resolved (see revision_history
  v0.1.1) — we're treating aec-data-layers as related work, not re-deriving its citations.
---

## 1. Introduction

### 1.1 The Research Problem

An architect who has designed a compliant emergency stairwell, a functional private office, or a well-configured mop room on one project starts the next project from a blank sheet — even when the two share a jurisdiction, and even when they share the same space type, so the compliance verification is re-run from first principles too. This is not a failure of individual practice; it follows from how building-code and environmental-context information is conventionally packaged: as a project-specific consultant report, not as data attached to a reusable design artifact that can travel with that artifact to the next project.

This paper specifies an architecture in which a reusable space design and the place-specific rules that govern it are two separate, independently authored data structures, joined only at evaluation time. We ask a falsifiable question: can regulatory and environmental site data be authored once per jurisdiction and applied to any number of reusable Compositions deployed there — including Compositions an architect authors independently and reuses across an entire practice — without editing either the Composition or the jurisdiction data when a new pairing occurs?

### 1.2 Terminology

We use **Object** for a single, jurisdiction-agnostic building product (an element in the IFC sense — `IfcFurniture`, `IfcDoor`). We use **Composition** for a reusable functional-space design — a room or assembly type such as "Private Office — Small," a sidewalk segment, or an emergency stairwell — not a whole building. A Composition is jurisdiction-agnostic by construction: the same Composition can be proposed in Vancouver, Rome, or Mexico City, with its compliance status differing by location while its own definition does not change.

### 1.3 Contributions

This paper makes three contributions. First, a data model separating Compositions, Site Context Overlays, and derived Assessments, replacing an ad hoc pattern (documented in §4.3) in which jurisdiction-specific compliance conclusions were stored as unstructured text directly on Composition records with no jurisdiction key. Second, a closed four-category schema organising the site-data layers an architect or engineer needs at a location, distinguishing regulatory rules (pass/fail) from environmental site facts (informational, non-pass/fail). Third, a reuse model in which a practitioner's own accumulated Composition library — potentially hundreds of designs across a career — remains fully browsable and portable across projects and jurisdictions without a centrally hosted accounts system, extending a base catalogue rather than forking it.

---

## 2. Literature Review

### 2.1 Post-Design Validation as the Prevailing Pattern

The dominant architecture for automated code-compliance checking in production BIM workflows today is post-design validation: a designer authors a complete model, exports it, and submits it to a rules engine that returns a violation report for the designer to remediate and resubmit. Singapore's CORENET X e-submission framework is the most advanced government implementation of this pattern in public production, applying automated code-compliance checks — accessibility, fire safety, urban-planning conformance — to submitted IFC models across four sequential approval gateways [corenet-x-2021]. Commercial rules-engine validators occupy the same quadrant: rich in the checks they can perform, but structurally downstream of authoring, meaning a non-compliant configuration is never prevented from being drawn, only flagged afterward.

The Information Delivery Specification (IDS) 1.0 provides the machine-checkable half of a different approach: a validation contract that a model must satisfy, expressed as property and quantity constraints [ids-1-0]. IDS is, however, a specification of what a valid model *contains*; it does not, on its own, specify which of several available design *choices* a designer should draw from, and it says nothing about which requirement set applies at a given place — that remains an external, manually tracked fact.

### 2.2 The Gap: Reusability Across Jurisdiction and Across Project

No part of the reviewed literature addresses reusing the *same* space design across multiple jurisdictions with automatically-varying compliance status, as distinct from validating a single project's model once. The buildingSMART Data Dictionary (bSDD) provides the missing piece this paper's architecture depends on: stable, tool-neutral concept identifiers giving an element or space type a durable identity independent of the specific project or jurisdiction it happens to be evaluated in [bsdd-v1]. bSDD supplies identity; it does not supply the requirement data itself, or a mechanism for attaching that data to a reusable design without editing the design. That mechanism is this paper's contribution.

A companion internal document (`topic-city-code-as-composable-geometry.draft.md`, unpublished, doctrine reference #41) proposes a related but architecturally distinct mechanism at the individual-element level: encoding jurisdiction-specific regulatory constraints directly into a product's specification via IDS numeric constraints plus IFC geometric-exclusion fragments, so that a non-compliant element cannot be placed at all. That mechanism targets element-level physical constraints (a fire-rated wall's required geometry). This paper's Overlay/Assessment mechanism targets a different granularity — Composition-level and site-level regulatory and environmental data — and treats compliance as computed and displayable rather than as a hard placement constraint. The two are compatible, not competing: a future implementation could use element-level geometric exclusion inside a Composition whose overall compliance status is still reported by the Overlay/Assessment mechanism specified here.

---

## 3. Methodology: The Site Context Overlay Data Model

### 3.1 Separating Composition, Overlay, and Assessment

The architecture specifies three record types, deliberately kept independent:

**Composition.** A jurisdiction-agnostic functional-space design, storing jurisdiction-neutral measured metrics — for example, a desk-to-window clearance in metres — rather than a threshold, a pass/fail conclusion, or a named jurisdiction's law. A Composition never stores a verdict.

**Site Context Overlay.** A record scoped to a jurisdiction or a location, holding two kinds of content under a single mechanism: **requirements** (pass/fail regulatory rules, each citing its real source and, where applicable, an IDS 1.0 reference [ids-1-0]) and **site data** (informational, non-pass/fail environmental facts — a climate-zone value, a seismic hazard figure, a flood-zone designation). Both kinds attach to Objects or Compositions by classification (an IFC class, a Uniclass code, a bSDD URI, or a Composition-kind label), never by editing the target record.

**Assessment.** A computed, cacheable result for one (Composition × Overlay) pair — never hand-authored. The same Composition can be simultaneously compliant against one jurisdiction's Overlay and non-compliant against another's; this is not treated as a contradiction, since the Composition's own definition has not changed.

### 3.2 Handling True Geometric Variation

A small minority of regulatory differences are genuine geometry differences — a fire door required to be physically wider in one jurisdiction than another. This architecture does not model such cases as a jurisdiction field mutating the base Object. Instead, the Composition defines a substitution slot, and the Assessment for a given Overlay resolves which concrete Object variant fills that slot, surfacing an explicit "requires substitution" state where no compliant variant is yet registered. The base Object is never mutated by jurisdiction; only the slot resolution varies.

### 3.3 A Closed Four-Category Schema for Site Context Overlays

Site Context Overlays carry a `category` field restricted to exactly four values — a deliberately closed, fixed-per-project schema, distinct from the layers within each category, which remain an open, extensible vocabulary. Table 1 specifies the four categories and the design question each answers.

**Table 1.** Site Context Overlay master categories.

| Category | Design question | Data character | Illustrative layers |
|---|---|---|---|
| Regulatory and Entitlement | What does law require here? | Rules (pass/fail) | Zoning, building-code adoption |
| Hazards and Structural Loads | What extreme event must the structure survive? | Return-period extremes | Flood hazard, seismic peak ground acceleration, wind design speed, wildfire hazard |
| Climate and Energy | What is the long-run environmental average? | Long-run averages | Building-code climate zone, solar irradiance |
| Ground and Ecology | What does the site itself contain? | In-situ measured properties | Soil type, eco-region |

The grouping is by design question and data statistic, not by regulatory status. Most of the underlying layers are code-referenced somewhere, so partitioning on "is this a law" does not cleanly separate them; partitioning on what each layer measures — a rule, a return-period extreme, a long-run average, an in-situ property — does. Cross-category interactions — seismic peak ground acceleration amplified by soil site class; a flood designation that is at once a physical hazard and a trigger for a regulatory freeboard requirement — are resolved at Assessment time, not by copying a layer's data into two categories.

The specific layers, their per-country sourcing, and their licensing status are the subject of a related but separate research programme, cited here as related work rather than re-derived; the finding this paper relies on is narrower — that the layers cluster cleanly into exactly these four design-question categories, independent of which specific national data source ultimately populates each one.

### 3.4 Self-Hosted Extension Model

Ownership follows a self-hosted pattern rather than a centrally hosted accounts system. A practitioner runs their own instance of the underlying open-source platform (EUPL-1.2-licensed [eupl-1-2]), with no requirement to expose that instance to the public internet — a fully private, offline, local-network-only deployment is a first-class supported mode. The practitioner's own Compositions register as extensions layered on top of a shared base catalogue and its Site Context Overlays, importing base-catalogue updates (a corrected specification, a newly registered jurisdiction) while keeping the practitioner's own designs private and under their own governance. The design goal, stated plainly: a practitioner should be able to locate a Composition from a past, unrelated project — a stairwell detail used on a project years earlier, in a different jurisdiction — as quickly as they can browse a small shared base catalogue today, regardless of how many Compositions have accumulated in their own library.

---

## 4. Results

### 4.1 Corrected Spatial-Hierarchy Terminology

An architectural question this programme needed to resolve precisely, because getting it wrong costs credibility with an IFC-literate reader: are the aggregate space types above a Composition — a **Key Plan** (the smallest leasable unit, planned from furniture and circulation), a **Tile** (a self-similar, composable fraction of a floor plate), and a **Floor Plate** — themselves the same kind of thing as an Object?

They are not, and an earlier working assumption in this research programme that called a Key Plan "itself a BIM Object" was a category error, corrected in this draft. IFC 4.3 distinguishes two separate hierarchies that a flattened framing collapses into one: **elements** (physical products — `IfcFurniture`, `IfcDoor`, our "Object") and **spatial elements** (volumes of space — `IfcSpace`, `IfcSpatialZone`, `IfcBuildingStorey`; our Key Plan, Tile, and Floor Plate). IFC also distinguishes two "part-of" relationships that a flattened framing conflates: **aggregation** (`IfcRelAggregates`, true whole/part decomposition — a Tile aggregates Key Plans) and **containment** (`IfcRelContainedInSpatialStructure` — furniture is located in, not aggregated into, a space). A Composition is furniture *contained in* a Key Plan; it is not correct to describe a Key Plan as *composed of* Composition Objects in the aggregation sense.

Table 2 states the corrected model. One additional precision, caught in review and applied here rather than in a later revision: `IfcZone` (a subtype of `IfcGroup`, joined via `IfcRelAssignsToGroup`) is not a placeable, aggregatable entity and does not belong in this table at all — an earlier draft of this table listed it as a Tile analogue alongside `IfcSpatialZone`, which contradicts this section's own claim that every rung is placeable and aggregatable. Only `IfcSpatialZone` (a genuine `IfcSpatialElement`, under `IfcProduct`) is used below, and it is assigned to the Tile rung exclusively — Zone, one level down, is stated as an `IfcSpace` subdivision rather than reusing `IfcSpatialZone` for two different rungs.

**Table 2.** Corrected element/spatial two-ladder model.

| Rung | Ladder | IFC 4.3 analogue |
|---|---|---|
| Object | Element (product) | `IfcFurniture` / `IfcElement`, typed via `IfcTypeProduct` |
| Composition | Element assembly | `IfcElementAssembly` |
| Zone | Spatial subdivision | `IfcSpace` subdivision |
| Key Plan | Space | `IfcSpace` |
| Tile | Spatial grouping | `IfcSpatialZone` |
| Floor Plate | Space | `IfcBuildingStorey` |
| Building | Space | `IfcBuilding` |

Both ladders converge at IFC's common supertype, `IfcProduct` (via `IfcObject`), in the sense of being placeable and classifiable; the aggregation machinery both ladders share (`IfcRelAggregates`) is in fact defined one level higher, on `IfcObjectDefinition`. Every rung on both ladders is placeable, classifiable, and aggregatable, which is the precise sense in which "the catalogue and the space-planning system are one data model" is correct — at the shared-supertype level, not at the Object level. The self-similar, fractional composability of Tiles (an eighth, a quarter, a half, three-quarters, or a full floor plate) is not itself an IFC-native mechanism; it is a genuine extension layered on top of IFC's `IfcSpatialZone`, and is claimed as such rather than presented as an IFC feature.

### 4.2 Consistency With Primary Source Material

An internal, unpublished design-response document predating this manuscript states the relationship among these three levels directly, in one place, as "Floor Plates = Tiles = Key Plans," with an explicit caveat that individual Key Plans are not simply summed to make a Tile, and that for at least two building typologies the Tile and Floor Plate levels collapse into a single level with no intermediate step. The same source states explicitly that Key Plans are composed of "blocks" that also represent climate zones for building-services purposes — i.e., the same spatial partition already serves both a leasing function and an environmental/services function, independent of this paper's Overlay mechanism. We report this as primary authorial testimony (§9.2 of the companion flat-file-substrate paper states the same evidence-class treatment), not as an externally verifiable citation. Separately, and reported honestly as an absence rather than elided: that source document does not use "BIM Object" terminology anywhere, and does not itself present Key Plans, Tiles, or Floor Plates as a software catalogue of discrete objects — the catalogue framing is this platform's own extension of the source material's design methodology, not a claim about what the source document itself asserts.

### 4.3 The Ad Hoc Pattern This Architecture Replaces

Before this specification, jurisdiction-relevant data was stored as an unstructured text field directly on a subset of Composition records, mixing measured geometric facts, a jurisdiction's numeric threshold, and a derived pass/fail conclusion into a single string, with no explicit jurisdiction key distinguishing, for example, a German circulation-width rule from a Canadian one on the same field. This pattern cannot represent the same Composition being compliant in one jurisdiction and non-compliant in another, because it has nowhere to record which jurisdiction a given conclusion applies to. The architecture in §3 replaces it directly: measurements move to Composition-level neutral metrics, thresholds move to Overlay-level requirements with an explicit jurisdiction scope, and conclusions are no longer stored at all — only computed, on demand, as an Assessment.

### 4.4 The Perimeter/Core/Circulation Zoning Principle Is General; the Specific Three-Zone Package Is Not

A related but separable question this research programme investigated: is the three-zone (daylight-adjacent / flexible-interior / circulation) logic underlying a Key Plan a recognised general principle in space planning, independent of any specific tool? The underlying spatial logic — organising floor depth from the facade inward into a daylight/perimeter zone, a core/flexible zone, and a circulation zone — is well established: ASHRAE energy-modeling guidance treats perimeter-zone depth as a standard input to HVAC zoning [ashrae-90-1], the British Council for Offices' Guide to Specification codifies a daylight/ventilation perimeter zone separated from a services core zone with associated floor-plate-depth guidance [bco-guide-to-specification], and the WELL Building Standard v2's daylight-modeling feature formalises a daylight zone as a function of distance from glazing [well-v2-daylight-modeling]. No general source reviewed, however, codifies exactly three zones under the names Habitat, Magazine, and Corridor, or ties them 1:1 to a specific building-width-calculation tool the way this platform's own Key Plan methodology does; that specific three-zone package is this platform's own operationalisation of a well-established general principle, not itself a citation to external practice, and is presented in this paper as such rather than as an industry-standard term already in use elsewhere.

---

## 5. Discussion

### 5.1 Why This Belongs at the Composition Level, Not the Object Level

Keeping Objects strictly jurisdiction-agnostic is a deliberate architectural constraint, not an oversight: a sidewalk's or a desk's geometry does not change between Vancouver and Rome, only its compliance verdict does. Attaching jurisdiction data to Objects would require editing every affected Object every time a new jurisdiction is registered; attaching it to Overlays that apply *to* Objects and Compositions by classification means registering a new jurisdiction is a single new Overlay record, with zero edits to any existing Object or Composition. This is the architectural property that makes the reuse claim in §1.1 possible: the re-authoring problem is solved by construction, not by discipline.

### 5.2 The Thesis This Architecture Serves, Not Competes With

The Object and the Composition — not the Overlay, not the Assessment, not the self-hosted extension model — are the platform's core subject; the architecture specified in this paper exists to make Objects and Compositions portable and reusable across locations and projects, not to introduce a parallel feature of equal standing. A practitioner's growing personal Composition library, portable across every project regardless of jurisdiction, is the value this architecture delivers; the Overlay/Assessment mechanism is what makes that portability compliant rather than merely convenient.

---

## 6. Limitations

The four-category schema (§3.3) is validated by internal design-question consistency, not by an external survey of practitioner classification preferences; whether architects and engineers outside this research programme would independently arrive at the same four categories is untested. The self-hosted extension model (§3.4) is specified but not yet evaluated against a practitioner's real accumulated Composition library at the scale (hundreds of designs) the value proposition in §3.4 describes. The corrected spatial-hierarchy terminology in §4.1, while grounded in IFC 4.3's published schema, has not yet been reviewed by an external IFC-literate practitioner audience — the correction is based on an internal architectural consult, not external validation.

---

## 7. Conclusion

The re-authoring problem this paper opened with — an architect rebuilding the same space type, and re-verifying its compliance from scratch, on every project — is not a discipline problem to be solved by better practice; it is an artifact of packaging code and site data as a project-scoped prose report. Separating the reusable Composition from the jurisdiction-specific Site Context Overlay, and deriving compliance as a computed Assessment rather than authoring it as a verdict, dissolves the problem by construction: a Composition authored once is deployable in any jurisdiction, registering a new jurisdiction adds one Overlay record and edits no existing design, and the same design carries a different, automatically-derived compliance status in each place it is proposed. What a project-scoped, prose-based workflow structurally cannot do — reuse a design and its compliance status across jurisdictions without re-authoring either — is this architecture's ordinary mode of operation. The supporting results hold it together: a closed four-category schema that separates regulatory rules from environmental facts, a spatial-hierarchy terminology corrected against IFC 4.3's own element/space distinction rather than a flattened working assumption, and a self-hosted extension model that keeps a practitioner's accumulated library private and portable while still tracking a shared, updatable base catalogue.

---

## 8. Formal Hypotheses

> **H₁ (Reuse-Without-Edit Hypothesis).** Registering a new jurisdiction's Site Context Overlay requires zero edits to any existing Object or Composition record; only a new Overlay record and, where a true geometric variation exists, a new substitution-slot resolution are added.

> **H₀ (Null).** Registering a new jurisdiction requires editing at least one existing Object or Composition record.

> **H₂ (Terminology-Correctness Hypothesis).** The corrected two-ladder model (§4.1, Table 2) maps every Woodfine-vocabulary term (Object, Composition, Zone, Key Plan, Tile, Floor Plate) to a distinct IFC 4.3 spatial-structure or element concept with no term mapped to more than one IFC concept and no IFC concept required to represent two different Woodfine terms.

---

## 9. Falsification Programme

*Test 1 — Zero-edit jurisdiction onboarding.* A new jurisdiction's Site Context Overlay will be authored for an existing Composition already deployed in at least one other jurisdiction. H₁ is falsified if authoring the new Overlay requires any edit to the Composition record itself (excluding the addition of a new substitution-slot resolution where a genuine geometric variation exists).

*Test 2 — Simultaneous divergent compliance.* A single Composition will be evaluated against two Overlays with materially different requirements. H₁'s underlying claim is falsified if the architecture cannot represent the Composition as compliant against one Overlay and non-compliant against the other without duplicating the Composition record.

*Test 3 — IFC mapping completeness.* Each row of Table 2 will be checked against the published IFC 4.3 schema documentation [ifc-4-3]. H₂ is falsified if any Woodfine-vocabulary term fails to map to a distinct IFC 4.3 concept, or if two Woodfine terms are found to require the same IFC concept.

*Test 4 — Practitioner library scale test.* A practitioner's Composition library of at least 100 entries, spanning multiple jurisdictions, will be evaluated for browse/search/filter performance and result relevance against the same interface used for the shared base catalogue. The self-hosted extension model's value claim (§3.4) is falsified if retrieval quality or speed degrades materially relative to the base-catalogue experience at that scale.

---

## AI Use Disclosure

This paper was developed using Claude Sonnet 5 (Anthropic). The Site Context Overlay data model, the four-category schema, and the self-hosted extension model were designed in an earlier working session and formalised into this manuscript with AI assistance under human editorial direction. The spatial-hierarchy correction in §4.1 was produced by a dedicated architectural consult using a different model instance than the one drafting this manuscript, specifically to obtain an independent check rather than have the drafting model self-review its own prior terminology. All standards citations were checked against the cited source in the course of this drafting pass. The model used is identified per COPE 2024 guidelines.

---

## CRediT Contributor Roles

**Jennifer M. Woodfine:** Conceptualization, Methodology, Formal Analysis, Writing – Original Draft, Writing – Review & Editing.
**Peter M. Woodfine:** Conceptualization, Validation, Writing – Review & Editing.
**Mathew Woodfine:** Software, Data Curation, Writing – Review & Editing.

---

## Conflict of Interest

The authors declare no conflict of interest. The authors are principals of the owner-operator whose deployment and internal design-response document form part of the case basis of this paper (§4.2); this relationship is disclosed rather than treated as a conflict, consistent with this paper's framing as an architectural specification grounded in a real deployment rather than an independent third-party evaluation.

---

## Funding

No external funding received. The authors thank buildingSMART International for the IFC 4.3 spatial-structure and aggregation schema this paper's corrected terminology (§4.1) is grounded in.

---

## Data Availability

The IFC 4.3 schema and IDS 1.0 specification cited in this paper are publicly available from buildingSMART International. The internal design-response document referenced in §4.2 is an unpublished primary source authored by the paper's own authors and is not separately available for third-party verification; the quoted material is reported as primary authorial testimony, not as a third-party-verifiable citation. The related site-data-layer sourcing referenced in §3.3 is documented in a sibling research programme's own paper, cited as related work.
