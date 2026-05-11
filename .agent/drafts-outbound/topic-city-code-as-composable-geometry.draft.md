---
schema: foundry-draft-v1
draft_id: topic-city-code-as-composable-geometry
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-documentation/topic-city-code-as-composable-geometry.md
created: 2026-05-06T17:30:00Z
author: task@project-bim
cites: [ifc-4-3, ids-1-0, bsdd-v1, ifc-fragment-spec, corenet-x-2021]
doctrine_claims: [41]
research_done_count: 3
research_suggested_count: 1
open_questions_count: 2
research_provenance: |
  Primary: /srv/foundry/.claude/sub-agent-results/B-bim-city-code-as-geometry-2026-04-28.md (375 lines)
  Supplementary: /srv/foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md (414 lines)
  Terminology: /srv/foundry/clones/project-intelligence/service-content/ontology/glossary/glossary_projects.csv
research_inline: false
---

# City Code as Composable Geometry

## Lede

Every building code compliance tool in production follows the same architecture: a completed design model is submitted to a rules engine, which generates a violation report, which a human must review and remediate before resubmission. This post-design validation model has been the industry standard for twenty years. It produces a structural tension between design iteration speed and compliance verification. The more thorough the rules engine, the longer each review cycle. The approach treats compliance as an audit rather than a constraint.

A different architecture is possible. If regulatory requirements are encoded directly into the elements available to a designer — not as rules applied to a finished model but as geometric and numeric constraints embedded in the element specification — then non-compliant configurations cannot be placed. The compliance audit becomes structurally unnecessary because the system never accepts non-compliant input.

This is the City Code as Composable Geometry model, and it is the architectural claim underlying the BIM Token platform.

---

## The Validation-First Paradigm

Post-design validation tools operate on a common pattern:

1. A designer authors a model in an IFC-capable authoring tool (Revit, Archicad, BricsCAD).
2. The model is exported to IFC format and submitted to a validation service.
3. The validation service applies a ruleset — encoded in proprietary rule languages, IDS 1.0 constraint files, or platform-specific scripting — and produces a report listing violations.
4. A human reviews the report, returns to the authoring tool, makes corrections, and resubmits.

This cycle repeats until the model passes. The number of cycles depends on the ruleset complexity and the degree to which the authoring environment enforces constraints during design. Most authoring tools enforce nothing: any element may be placed anywhere until the external validator objects.

The consequences are operational. Design teams budget iteration time for compliance review. On complex projects, regulatory review cycles add weeks to design phases. The validator is a gating function that sits outside the design environment and communicates asynchronously with it.

---

## Prior Art Survey

Sub-agent research identified four categories of prior art.

**IDS 1.0 (Information Delivery Specification).** buildingSMART's IDS standard encodes property constraints in XML. An IDS file declares: "an element of type `IfcWall` in category 'exterior' must have property `ThermalTransmittance` with a value not exceeding 0.18 W/m²K." IDS is a validation language — it defines what a valid model contains, not what elements a designer may place. IDS files are inputs to validators, not constraints on element palettes.

**bSDD (buildingSMART Data Dictionary).** The bSDD is a multilingual dictionary of built-environment concepts with stable URIs and property definitions. It provides semantic identity for element types across jurisdictions and tools. It does not encode regulatory constraints or climate zone performance requirements. A bSDD URI is an identity anchor, not a constraint specification.

**Solibri, Archistar, and equivalent validators.** These platforms operate post-design. Solibri's rules are rich — it can check geometry, topology, property values, and spatial relationships — but it operates as an audit tool on submitted models. Non-compliant models pass through the authoring environment without objection.

**Singapore CORENET X.** The most advanced government BIM submission system in public production. CORENET X accepts IFC models submitted for building permit applications and runs automated code compliance checks across Singapore's regulatory requirements. It is the closest prior art to the PointSav approach. It remains, however, a validator: models are authored freely, submitted to CORENET X, and returned with violation reports. The 2024 CORENET X implementation adds real-time guidance during design in some authoring tool plugins, narrowing but not closing the gap. It is jurisdiction-specific and not available as a neutral platform for other jurisdictions.

**Assessment.** All identified prior art occupies the validation-first quadrant. The compositional-first quadrant — encoding constraints into element specifications before authoring — has no established prior art in public production. The structural gap is real.

---

## The Compositional Mechanism

City Code as Composable Geometry operates through a three-layer technical mechanism.

**Layer 1: Semantic identity via bSDD.** Every BIM Token carries a bSDD concept URI identifying its element type in a jurisdiction-neutral, tool-neutral reference. This URI is the stable identity that allows Regulation and Climate Zone overlays to reference the same element type regardless of IFC version drift.

**Layer 2: Regulatory constraint via IDS 1.0.** Each registered jurisdictional overlay for a BIM Token includes an IDS 1.0 constraint file. The IDS file encodes numeric and property constraints: maximum U-values, minimum structural ratings, fire resistance class requirements, accessibility clearances. When a BIM Token is placed in a model, its registered IDS constraints are part of its specification — the authoring environment receives these as element requirements, not as post-placement rules.

**Layer 3: Exclusion geometry via IFC fragment.** Where a regulatory requirement has geometric expression — a fire compartment boundary that an element must not cross, a setback from a property line, an accessibility envelope that must remain clear — the jurisdictional overlay includes an IFC fragment: a solid geometry encoded in IFC format that defines the excluded or required space. This fragment is associated with the token and resolves at placement time. It cannot be overridden by numeric constraints.

The composition of these three layers is what makes the geometry "encode" the code. The regulatory constraint is not stored in a separate validation database checked after authoring. It is stored in the token specification and instantiated with the element.

---

## Geometric Exclusion in Detail

The IFC fragment mechanism deserves elaboration because it addresses the class of regulatory requirements that numeric constraints cannot express.

Consider a fire compartment wall in a multi-storey building. The requirement is not simply "this wall must have fire resistance class REI 90." It is also "this wall must form a continuous plane from floor slab to ceiling slab with no penetrations except those covered by appropriately rated closure devices." The second requirement is topological and geometric: the wall must occupy a specific spatial relationship to surrounding elements.

An IDS numeric constraint can express REI 90. It cannot express the topological continuity requirement. An IFC geometric exclusion fragment can: it encodes the spatial volume that the fire compartment boundary must occupy and the adjacent spatial volumes that must be filled by conforming construction. Authoring tools that consume the fragment can display the required geometry as a design guide and flag deviations in real time.

This is qualitatively different from post-design validation. The designer sees the required spatial configuration during authoring, not after submission.

---

## Hyperscaler Structural Constraints

Research identified three structural reasons why hyperscaler cloud platforms cannot replicate the City Code as Composable Geometry model.

**Regulatory data sovereignty.** Jurisdictional regulatory data is public law. Encoding it as a service hosted on a US-headquartered commercial cloud platform creates procurement and sovereignty concerns for non-US jurisdictions under EU data residency requirements, GDPR restrictions, and equivalent national frameworks. A neutral platform that cities and national governments can self-host or have hosted under national cloud frameworks is structurally required for broad adoption.

**Offline-first requirement.** Construction sites frequently operate without reliable network connectivity. ITAR-restricted projects, remote sites, and many public infrastructure projects require the constraint data to be available offline. A cloud-dependent validation service cannot serve these use cases. A BIM Token vault cloned via git and stored locally is available offline unconditionally.

**Commercial platform neutrality.** Cities and national governments issuing regulatory requirements need to publish them to all conformant BIM platforms, not to specific commercial vendors. Publishing code requirements to a neutral, open-format JSON standard (W3C DTCG with BIM extensions) and distributing them via public git repositories is analogous to publishing building codes as PDF — neutral, reproducible, and vendor-independent. A commercial platform that monetises access to regulatory data introduces a commercial dependency into what is legally public information.

---

## Implementation Stages

The City Code as Composable Geometry model is implemented progressively.

**Stage 1 (current, planned for v0.0.3):** BIM Token vault with Specification layer complete. Regulation layer skeleton present with first overlay set: British Columbia residential (RS-1 zoning) — selected because Woodfine Management Corp. operates in BC and is the reference customer. Climate Zone layer populated with BC temperate-coastal zone (ASHRAE 5C equivalent) performance parameters.

**Stage 2 (intended, v0.1.x):** IDS 1.0 constraint file generation. For each registered Regulation overlay, a conformant IDS 1.0 file is generated from the token data and published alongside the DTCG JSON. This enables existing IDS-aware validators to consume PointSav-authored constraint specifications.

**Stage 3 (intended, future):** Authoring tool integration. A plugin or API surface that delivers BIM Token constraints to IFC-capable authoring tools at placement time, not at submission time. The authoring tool receives the element palette constrained to conformant tokens for the project's jurisdiction and climate zone.

---

## Open Questions

1. **IFC fragment authoring toolchain.** The mechanism for producing IFC geometric exclusion fragments from regulatory text has not been formalised. A workflow (regulatory text → IDS numeric + IFC geometric) is needed before Stage 2 can complete.

2. **Jurisdiction overlay publishing governance.** Who may publish a regulatory overlay to a BIM Token vault? A governance model — analogous to buildingSMART certification for IDS files — is not yet specified. This is a v0.1.x design decision.

---

*Draft prepared for project-editorial sweep. Research provenance: sub-agent B (city code as geometry, 2026-04-28); sub-agent A (prior art, 2026-04-28). Doctrine claim: #41 (City Code as Composable Geometry). BCSC posture: all implementation timeline references use planned/intended language.*
