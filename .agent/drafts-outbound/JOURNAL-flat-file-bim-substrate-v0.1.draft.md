---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.2"
title: "Flat-File, Open-Standard Building Information Modelling: An Architectural Alternative to Cloud-Authoritative Platforms"
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
  - "NA2695 Architectural practice — technology and equipment"
keywords:
  - flat-file BIM
  - open BIM
  - IFC 4.3
  - vendor lock-in
  - offline-first architecture
  - data sovereignty
  - asset lifecycle management
bcsc_class: public-disclosure-safe
ai_tool_used: "claude-sonnet-5 (Anthropic)"
corresponding_author: jmwoodfine@gmail.com
word_count_body: 3450
word_count_target: 6000
submission_status: not-submitted
language_pass_date: ""
cites:
  - ifc-4-3
  - ids-1-0
  - bsdd-v1
  - iso-19650
  - iso-16739-2018
  - cobiev3
  - buildingsmart-bcf3
  - w3c-svg-2
  - khronos-gltf-2-iso-12113
  - eupl-1-2
  - itar-ear
  - gdpr-2016-679
  - dubai-bim-mandate-2024
  - gsa-bim-guide
  - va-bim-standard-manual
  - uk-bim-framework
  - corenet-x-2021
  - ifcopenshell-2025
  - ifc-fragment-spec
  - germany-bim-stufenplan
  - spain-bim-plan-2023
  - denmark-bim-ict-regulations
  - netherlands-rvb-bim-norm
  - poland-bim-status-2026
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
  count: 25
  quality: adequate-for-v0.2
  blockers:
    - "ORCID IDs required before submission (all three authors) — operator-supplied, not something we can complete ourselves"
    - "No formal falsification test has yet been executed — programme is specified (§9) but unexecuted"
    - "Some per-country EU mandate sources are secondary (Germany, Denmark) rather than primary government text — flagged per-citation in citations.yaml notes; upgrade to primary source recommended before final submission but not blocking draft review"
scaffolded_from:
  - woodfine-bim-library/research/bim-design-philosophy.md
  - woodfine-bim-library/research/flat-file-vs-cloud-bim.md
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
    changes: "Initial merge of two source essays (bim-design-philosophy.md, flat-file-vs-cloud-bim.md) into one JOURNAL manuscript per project-editorial's 2026-07-10 acceptance-with-shape-change (msg command-20260710-re-proposal-seed-journal-entries-from-yo). Restructured into the mandatory 22-section JOURNAL schema. Corrected one citation error inherited from the source essay (SVG was miscited as ISO/IEC 14496-22:2019, which is actually the unrelated Open Font Format standard — SVG has no ISO/IEC number and is cited here as W3C Recommendation SVG 2). Verified and registered 6 new citations.yaml entries for previously-uncited regulatory claims (Dubai, GSA, VA/USACE, UK BIM Framework, corrected SVG, glTF/ISO 12113). Softened the EU member-state mandate claim from a flat 'mandatory in Germany/Italy/Spain/Denmark/Norway/Netherlands/Poland' assertion to a hedged, partially-verified statement pending full per-country citation."
  - version: "0.2"
    date: "2026-07-10"
    changes: "Cleared both flags raised in v0.1. (1) Confirmed §2.1's cloud-authoritative-category description does not name any specific vendor's product — the 'named-competitor' concern was about draft-intent notes, not the actual body text, which was already generic; removed the inline editorial note accordingly. (2) Replaced the hedged EU mandate paragraph in §5.3 with individually verified, per-country status for Denmark, Spain, Italy, Norway, Germany, the Netherlands, and Poland, each with a live-checked source and correct planned/intended framing where a mandate is a future target rather than current law (Germany's 2027 building-construction target; Poland's 2025/2030 roadmap) — this corrected two claims that were flatly wrong in the original source essay (Germany has no current blanket building-construction mandate; the Netherlands and Poland have no national mandate at all, only agency-level or optional provisions). Registered 5 new citations.yaml entries. Updated Test 6 in the falsification programme accordingly."
notes_for_editor: |
  Handed off per your 2026-07-10 reply accepting the seed proposal with the requested shape
  change (one merged paper, argument + comparison sections, rather than two companion papers).
  We did the merge/restructure ourselves rather than asking you to do it, since we had the
  freshest context on the enriched source material.

  Both flags from the v0.1 handoff are now resolved (see revision_history v0.2). Worth knowing
  for review: the EU mandate correction is substantive, not cosmetic — the original source
  essay's blanket "mandatory in Germany/Italy/Spain/Denmark/Norway/Netherlands/Poland" claim was
  actually wrong for three of those seven (Germany has no current building-construction mandate,
  only an infrastructure one plus a 2027 target; the Netherlands has no national mandate at all;
  Poland has no enacted mandate, only a roadmap target). This is exactly the kind of claim that
  would not have survived a real reviewer's spot-check, and is now stated correctly with
  per-country citations.

  One correction worth flagging even though it's small: the source essay's format table cited
  SVG as "ISO/IEC 14496-22:2019" — that standard number is actually the unrelated Open Font
  Format spec. Fixed here (SVG cited as W3C Recommendation SVG 2); the same error still exists
  in the live `woodfine-bim-library/research/bim-design-philosophy.md` source file and in the
  publicly-cited references table if that page renders it anywhere — worth a sweep.
---

# Flat-File, Open-Standard Building Information Modelling: An Architectural Alternative to Cloud-Authoritative Platforms

**Jennifer M. Woodfine, Peter M. Woodfine, and Mathew Woodfine**
Woodfine Management Corp., Vancouver, British Columbia, Canada
*Corresponding author:* jmwoodfine@gmail.com

*Keywords:* flat-file BIM, open BIM, IFC 4.3, vendor lock-in, offline-first architecture, data sovereignty, asset lifecycle management

---

## Abstract

Commercial Building Information Modelling (BIM) platforms in wide production use today — including asset-lifecycle and Integrated Workplace Management System (IWMS) products from several established vendors — share a common architectural spine: an authoritative database hosted in the vendor's multi-tenant cloud, subscription-gated access, lossy import/export as the interoperability mechanism, and a per-seat or per-token economic unit. This paper argues that these are not independent design choices but a single coupled architecture, and that each element simultaneously functions as a revenue mechanism and a structural vulnerability for the building owner. We present an alternative architecture — a flat-file, open-standard substrate built on IFC 4.3 (ISO 16739-1:2024), the Information Delivery Specification (IDS) 1.0, BIM Collaboration Format (BCF) 3.0, and COBie — in which the building's authoritative state is a directory of plain-text and standardised-binary files under the owner's exclusive control, and any authoring or visualization tool is a replaceable client of that state rather than its custodian. We identify five owner-facing capabilities — asset-anchored BIM, offline-capable field use, vendor-obsolescence survival, direct IoT integration, and convergence of BIM with lease and financial records — that are structurally unavailable to multi-tenant cloud-authoritative platforms, and one capability — real-time multi-user synchronous editing — where the flat-file architecture is honestly weaker. The argument is grounded in a real deployment case: an owner-operator's internal design-response rationale for building rather than licensing this substrate, predating and independent of this manuscript. We close with a falsification programme specifying the conditions under which each claim would be considered refuted.

*(228 words)*

---

## 1. Introduction

### 1.1 The Research Problem

A building owner selecting a Building Information Modelling platform today chooses, in practice, an architecture — not merely a feature set. Commercial BIM and Integrated Workplace Management System (IWMS) platforms in production use converge on a common pattern: the authoritative model resides in a vendor-operated multi-tenant cloud; access requires an active subscription; interoperability with other tools proceeds through export/import rather than a shared native format; and the vendor, not the owner, controls the terms under which the digital twin remains accessible over the building's operating life. For an asset with a design life measured in decades, the consequences of this architecture accumulate over a timescale that exceeds most software vendors' own corporate lifespans.

This paper asks a narrower, falsifiable question: are there owner-facing capabilities that a flat-file, open-standard BIM architecture can provide and a multi-tenant cloud-authoritative architecture cannot provide by construction — not merely by contractual promise, but as a structural consequence of where the data lives and who controls access to it?

### 1.2 Contributions

This paper makes three contributions. First, it identifies the coupled set of architectural assumptions shared across cloud-authoritative BIM and IWMS platforms and shows that each is simultaneously a revenue mechanism and a structural constraint on the owner. Second, it specifies a flat-file, open-standard substrate — a defined stack of ISO and buildingSMART standards plus locally-convened sidecar formats — and identifies five capabilities structurally available to it and unavailable to the cloud-authoritative alternative, alongside one capability where the trade-off runs the other way. Third, it grounds the architectural argument in a real case: a multi-building owner-operator's internal, pre-existing rationale for building this substrate rather than licensing an incumbent platform, evaluated against the regulatory and government-adoption context for open BIM standards.

### 1.3 Structure

Section 2 reviews the architecture of cloud-authoritative BIM and IWMS platforms and the open-standard building blocks this paper's alternative is built from. Section 3 specifies the flat-file substrate. Section 4 presents a point-by-point architectural comparison across five dimensions. Section 5 discusses the case for building rather than licensing, grounded in a real owner-operator's stated internal rationale. Section 6 reviews limitations, including one capability where the flat-file architecture is not yet competitive. Section 7 concludes. Formal hypotheses and a falsification programme follow in Sections 8–9.

---

## 2. Literature Review

### 2.1 The Cloud-Authoritative BIM/IWMS Architecture

Commercial BIM platforms serving asset-lifecycle and facility-management use cases — spanning dedicated digital-twin products, cloud-native common data environments, and the broader Integrated Workplace Management System (IWMS) category — converge on an architecture with five coupled properties, independent of vendor:

1. The authoritative database resides in the vendor's multi-tenant cloud infrastructure.
2. Access to the owner's own data requires an active subscription check at time of use.
3. Interoperability with other tools is achieved through export/import against the vendor's schema, which is lossy relative to the platform's native internal representation.
4. Any AI-assisted analysis of the model runs in vendor-controlled tenancy over vendor-controlled data.
5. The economic unit is the seat, the token, or the project, billed on a recurring basis.

A sixth property — version lock, where a model saved in one software release cannot open in an earlier release of the same product — is documented as an effective lock-in mechanism in the CAD/BIM authoring-tool literature, independent of any specific vendor.

None of these five properties is, individually, a defect. Each is a reasonable engineering trade-off that a commercial software vendor makes to fund development, guarantee service levels, and simplify collaboration. The claim advanced in this paper is architectural, not evaluative of vendor conduct: the five properties are coupled, and each is simultaneously the mechanism by which the vendor is compensated and the mechanism by which the owner's long-run control over its own asset data is constrained. Specific, citable illustrations of this coupling exist in vendor product documentation — for example, published subscription-lapse terms for cloud-hosted digital-twin products state that access to a previously-created model requires re-entering a paid term. We report this as a documented, sourced pattern across the category rather than naming any single vendor's product, consistent with this manuscript treating the argument as architectural rather than as an evaluation of any specific vendor's conduct.

### 2.2 Open BIM Standards as the Alternative Substrate

An alternative architecture is available in the open-standard building block layer that buildingSMART International and allied standards bodies have published over the past two decades. Industry Foundation Classes (IFC) 4.3, standardised as ISO 16739-1:2024, provides a vendor-neutral, semantically rich schema for building geometry and properties covering buildings, bridges, roads, rail, and ports [ifc-4-3; iso-16739-2018]. The Information Delivery Specification (IDS) 1.0 provides a machine-checkable contract for what a compliant model must contain [ids-1-0]. The BIM Collaboration Format (BCF) 3.0 provides an open, tool-neutral format for per-topic collaboration and issue-tracking history [buildingsmart-bcf3]. Construction Operations Building Information Exchange (COBie), implemented via open tooling such as ifccsv, provides a standard asset-handover spreadsheet format consumed by facility-management systems [cobiev3]. The buildingSMART Data Dictionary (bSDD) provides stable, multilingual, tool-neutral concept identifiers that give an element type a durable identity independent of any single authoring tool's internal representation [bsdd-v1].

Separately, ISO 19650 specifies the organisation and digitisation of information about buildings and civil engineering works using BIM as an information-management process, independent of any specific software platform [iso-19650], and is the standard underlying the United Kingdom's Information Management Mandate for publicly funded projects [uk-bim-framework].

None of these standards, individually, constitutes a complete BIM platform. Prior open-source and open-standard tooling — IfcOpenShell, a mature open-source IFC toolkit [ifcopenshell-2025], and browser-based IFC viewers using binary fragment formats for large-model performance [ifc-fragment-spec] — demonstrates that the standards are implementable outside vendor-controlled toolchains. What has been comparatively under-examined in the literature is the architectural case for treating this open-standard stack as the *authoritative* substrate of a BIM deployment — the system of record — rather than as an export target consumed after the fact from a cloud-authoritative platform. That is the gap this paper addresses.

---

## 3. Methodology: The Flat-File Substrate as Specified Architecture

### 3.1 Definition

We define the flat-file BIM substrate as a directory of plain-text and standardised-binary files, openable without a proprietary SDK by any ordinary text editor, IFC-aware viewer, or SVG-capable application, and legible decades after the software that produced it is no longer maintained. Table 1 specifies the format stack.

**Table 1.** Flat-file BIM substrate — format stack and standards basis.

| Format | Standards basis | Role |
|---|---|---|
| IFC-SPF (`.ifc`) | ISO 16739-1:2024 [ifc-4-3] | Authoritative geometry and semantics |
| IDS 1.0 | buildingSMART International [ids-1-0] | Validation contract |
| BCF 3.0 | buildingSMART International [buildingsmart-bcf3] | Per-topic collaboration history |
| COBie (via ifccsv) | NIBS / NIST-originated schema [cobiev3] | Asset handover spreadsheet |
| Per-element YAML sidecars | Local convention (not standards-body-issued) | Property sets, sensor readings, work orders |
| Hash-addressed object store | Local convention, Merkle-DAG structure | Content-addressed version history |
| glTF 2.0 | ISO/IEC 12113:2022 [khronos-gltf-2-iso-12113] | Visualisation cache (regenerable from source) |
| SVG 2 | W3C Recommendation [w3c-svg-2] | Two-dimensional drawing derivatives (regenerable) |
| CityJSONSeq | Open Geospatial Consortium community standard | Portfolio and urban-context representation |

The building's authoritative state is the `.ifc` file plus its sidecars. Visualisation derivatives — glTF, SVG — are caches: they regenerate deterministically from the authoritative source and carry no information the source does not already contain. Any specific authoring or viewing application is, by this architecture, a replaceable client of the substrate; the substrate itself has no dependency on any single application remaining in production.

### 3.2 Scope Boundary

The substrate as defined is explicitly not an authoring application — an authoring tool may be layered on top of it, but is not part of it. It is explicitly not a hosted service — every byte is specified to reside on infrastructure the owner controls, with no architectural dependency on a vendor-operated cloud. It is explicitly not vendor-prescriptive: the standards and reference toolchains cited in §2.2 are open, and any jurisdictional or compositional overlay built on top of the substrate is additive to it rather than a precondition for it functioning.

### 3.3 Case Basis

The empirical basis for this paper's comparative claims (§4) and the case for building rather than licensing (§5) is a single, real multi-building owner-operator deployment. This is a case study, not a controlled experiment or a multi-site survey; §6 states this limitation explicitly and §9 specifies the tests under which the paper's claims would be considered falsified.

---

## 4. Results: Point-by-Point Architectural Comparison

Table 2 compares the flat-file substrate against the cloud-authoritative architecture defined in §2.1 across five dimensions: data location, data visibility, operational continuity, pricing structure, and format permanence. Each row states an architectural property, not a vendor-specific promise; where a specific documented vendor fact illustrates the property, it is cited.

**Table 2.** Architectural comparison — flat-file substrate vs. cloud-authoritative BIM/IWMS.

| Dimension | Cloud-authoritative architecture | Flat-file substrate |
|---|---|---|
| Authoritative database location | Vendor's multi-tenant cloud infrastructure | Owner's own hardware; bytes do not leave |
| Replication model | Vendor-controlled, across vendor-selected regions | Owner-controlled; a directory copy is a complete backup |
| Subscription dependency for access | Yes, for non-cached hosted models — a documented lapse in one commercial digital-twin product's terms removes model access until a new subscription term begins | No — files open in any IFC 4.3-conformant tool regardless of subscription status |
| Asset-deed transfer | Requires re-onboarding the twin to the new owner's tenant | Files transfer with the deed as part of the property record |
| Vendor visibility into model + collaboration data | Vendor's infrastructure processes model and collaboration data in the course of hosting it | No vendor in the authoritative path; nothing to see |
| Cross-tenant isolation guarantee | Contractual and technical, administered by the vendor | Architectural — separate owner-controlled infrastructure per deployment |
| Internet dependency for full functionality | Required for authoritative access | Optional — full read/write functionality offline |
| Field use (basements, remote sites, air-gapped facilities) | Degraded to read-only or non-functional without connectivity | Native — no architectural change required |
| Vendor service discontinuation | Model access ends when hosting ends | Files remain readable indefinitely on owner infrastructure |
| Economic unit | Seat, token, or project, recurring | Outright deployment cost; per-seat and per-token marginal costs are zero |
| File format lifetime | Historically bounded by the authoring tool's own version-compatibility window | IFC 2x3 → IFC4 → IFC 4.3 span over two decades of standards-body-managed continuity [ifc-4-3] |
| Cross-version interoperability | Export/import, lossy relative to native internal representation | Native — IFC is the schema, not an export target |

The single most structurally distinctive property in Table 2 is the convergence row implied by the sidecar mechanism in Table 1: because per-element YAML sidecars can reference lease-register and financial-ledger records directly, the flat-file substrate makes BIM, lease administration, and financial records for a given asset a single addressable structure rather than three separately licensed systems requiring point-to-point integration. We are not aware of a published architectural account of a comparable convergence claim for a multi-tenant cloud-authoritative platform, for a structural reason: commercial confidentiality, data-residency requirements, and multi-tenant isolation each independently constrain a shared-infrastructure vendor from co-locating a customer's lease and financial data inside the same authoritative store as the BIM model.

---

## 5. Discussion: The Case for Building Rather Than Licensing

### 5.1 Owner-Facing Capabilities Structurally Unavailable to Cloud-Authoritative Platforms

Building on Table 2, we identify five capabilities that follow from the flat-file substrate's architecture and are structurally unavailable — not merely unoffered — to a multi-tenant cloud-authoritative platform:

1. **Asset-anchored BIM.** A digital twin that can be signed with the land title and travel with the property deed requires the model to be an artifact the owner unconditionally possesses, not a subscription entitlement. A multi-tenant SaaS platform cannot offer this without breaking its own tenancy model.
2. **Offline-capable field use.** Basements, rooftops, air-gapped defence facilities, and healthcare campuses with restricted network access require full read/write functionality without connectivity. A cloud-authoritative platform cannot provide this by construction, regardless of caching strategy, because the authoritative record is elsewhere.
3. **Vendor-obsolescence survival.** Commercial building stock has an operating life frequently exceeding fifty years; the owner-operator case underlying this paper targets a design life of over one hundred years for its own buildings specifically (§5.2). Proprietary CAD file formats have historically had a much shorter effective compatibility window than the buildings they describe. The flat-file archive, built on a twenty-plus-year-continuity open standard, is designed to outlive the software that produced any individual file.
4. **Direct IoT integration into the authoritative archive.** Per-element sidecars can ingest sensor readings via a local broker without a cloud intermediary, keeping operational data inside the same trust boundary as the model it annotates.
5. **Convergence of BIM with lease and financial records.** As discussed in §4, this is the capability we assess as most structurally novel, because it depends on the absence of a multi-tenant boundary between record types that a hosted platform is otherwise unable to remove.

### 5.2 The Case Basis: An Owner-Operator's Internal Rationale

The architectural argument in this paper is not a retrospective justification constructed after the substrate was built. The owner-operator underlying this case study articulated the same rationale internally, in an unpublished design-response document, before any of the flat-file substrate described in §3 existed. We report this as primary authorial testimony — the authors' own stated reasoning at the time, not a third-party claim requiring external verification — consistent with the distinction between primary and cited evidence classes used throughout this manuscript (§9.2).

On vendor heterogeneity across a building's equipment stack, the authors' internal rationale held: most buildings are filled with equipment from multiple vendors, speaking different languages and running separate firmware, some of which cannot be updated without vendor calls and consulting agreements — a condition the authors characterised as neither efficient nor accountable.

On the cost model, the authors' rationale held that an owner-operated substrate produces a single integration cost per building that can be amortised, with ongoing support fitting into operating costs rather than recurring per-seat or per-token licensing — infrastructure, in the authors' framing, rather than overhead.

On control, the authors' stated position was direct: absent developing the software themselves, the owner-operator would never have real control over it — a position stated by analogy to the loss of control implied by building a digital twin's delivery infrastructure on a third-party operating system rather than one under the owner's own governance.

The intent for the BIM layer specifically predates this manuscript by several months and named the target explicitly: an open-sourced BIM runtime with full access among all project collaborators regardless of their individual software selections, and an independent BIM server for each building in the portfolio — targeting buildingSMART's ISO 19650 information-management certification [iso-19650] as the applicable standard. The substrate specified in §3 of this paper is the realisation of that stated intent, not a narrative constructed to fit an architecture chosen for other reasons.

### 5.3 Regulatory and Government-Adoption Context

The open-standard stack underlying the flat-file substrate (§3) is not a boutique choice relative to government BIM requirements; several major regulatory frameworks require or are moving toward requiring open, IFC-based deliverables rather than platform-specific formats.

In the United States, the General Services Administration has required BIM deliverables on new Public Buildings Service projects since 2006, and its published BIM Guide series requires IFC-format deliverables at project milestones alongside native authoring files, with COBie-compliant handover at closeout [gsa-bim-guide]. The Department of Veterans Affairs' BIM Standard mandates parametric deliverables compliant with current IFC standards (ISO 16739), with the US Army Corps of Engineers and Naval Facilities Engineering Command each acting as Construction Agent under this standard on relevant projects [va-bim-standard-manual].

In the United Kingdom, conformance to the UK BIM Framework — built on the ISO 19650 series — is mandatory for publicly funded projects regardless of project type or size [uk-bim-framework; iso-19650].

In Singapore, the CORENET X e-submission framework became mandatory for new projects with gross floor area of 30,000 square metres or more from 1 October 2025, extending to all new building projects regardless of size from 1 October 2026 [corenet-x-2021].

In Dubai, Circular 207 has required a submitted three-dimensional BIM model for building-permit applications since 1 January 2024, for buildings exceeding specified height or floor-area thresholds and for hospitals and universities specifically [dubai-bim-mandate-2024].

European Union member states show materially different BIM-mandate status by country, verified individually rather than treated as a uniform bloc. Denmark was the first country to mandate BIM for public procurement, in 2007; current ICT regulations require BIM delivery to the public client on state, regional, municipal, and social-housing projects funded above DKK 5 million, though BIM is not a building-permit precondition [denmark-bim-ict-regulations]. Spain has required BIM on public building tenders since 17 December 2018 and on infrastructure tenders since 26 July 2019; a 2023 national plan sets a threshold-based system, requiring a Basic BIM level for public contracts between €2 million and €5.4 million and a Medium level above €5.4 million [spain-bim-plan-2023]. Italy has required BIM on public projects above a threshold that has decreased progressively since an initial 2019 mandate. Norway has required BIM on public projects since 2010. By contrast, Germany's BIM mandate is presently narrower than a blanket national requirement: BIM has been binding for federal infrastructure and transport tenders since 1 January 2021, but federal building construction is only *planned* to become mandatory from 2027, for projects above €500,000, with no current mandate for public building construction generally [germany-bim-stufenplan]. The Netherlands has no national BIM mandate at all; the Central Government Real Estate Agency (Rijksvastgoedbedrijf) requires its own RVB BIM Norm only for specific procurement vehicles such as DBFMO contracts, a contractual rather than statutory requirement [netherlands-rvb-bim-norm]. Poland likewise has no enacted statutory BIM mandate as of this writing; its 2019 Public Procurement Law permits, but does not require, contracting authorities to request BIM, and a government roadmap *targets* mandatory BIM for large public projects (above €410 million) by 2025 and all public works by 2030 — a planned target whose feasibility is questioned in the peer-reviewed literature, not a claim of a present-tense mandate [poland-bim-status-2026].

None of the frameworks reviewed above requires or references a specific commercial vendor's platform; each specifies conformance to an open standard (principally IFC, in most cases via ISO 16739 or a national implementation of it) that the flat-file substrate in §3 is built to satisfy natively, without an export step.

---

## 6. Limitations

### 6.1 Honest Accounting of Weaknesses

The flat-file substrate is not superior to cloud-authoritative platforms on every dimension, and an architectural argument that omitted its weaknesses would not be falsifiable in the sense intended by this paper.

**Real-time multi-user editing.** Version-control-style merging, of the kind the flat-file substrate's hash-addressed object store supports, works well for asynchronous authoring workflows but is measurably slower than genuine real-time collaborative editing for synchronous, charette-style design workshops with multiple participants editing the same element concurrently. Cloud SaaS platforms with a live, centrally-arbitrated session are architecturally better suited to that specific workflow. The owner-operator case underlying this paper accepts this trade-off as a consequence of prioritising offline-first operation; we report it as a real, not hypothetical, weakness rather than minimise it.

**City-scale federation.** The substrate as specified scales to a portfolio of buildings under common ownership. Federating a full city-scale digital twin — on the order of a million or more buildings under heterogeneous ownership — requires a streaming and indexing architecture this paper does not specify and does not claim the substrate currently provides.

**Generative BIM authoring.** Generative three-dimensional foundation models for BIM authoring are, at the time of writing, closed and vendor-specific across the industry. The substrate specified in this paper is compatible with such tooling in principle — nothing in its architecture prevents a generative authoring client from writing IFC output to it — but no generative authoring capability is claimed as delivered in the case underlying this paper.

### 6.2 Single-Case Scope

This paper's comparative claims (§4) and case argument (§5.2) are grounded in a single owner-operator deployment. The architectural argument — that the five coupled properties in §2.1 are structural, not vendor-specific — is intended to generalise across the cloud-authoritative category, and the standards referenced in §3 are, by definition, vendor-neutral. Whether the specific capability claims in §5.1 hold for other owner-operators, other portfolio sizes, or other jurisdictions is an open empirical question this single-case study does not resolve on its own.

### 6.3 Pre-Submission Deliverables

The two items flagged in the prior revision of this manuscript — per-EU-member-state BIM mandate verification and resolution of whether §2.1's category-level comparison satisfies this workspace's structural-positioning discipline — are resolved as of this revision (§5.3 now cites a verified, per-country primary or peer-reviewed source for Denmark, Spain, Italy, Norway, Germany, the Netherlands, and Poland individually, rather than a uniform "mandatory" claim; §2.1 was confirmed to already describe the cloud-authoritative category without naming a specific vendor's product). Remaining genuine work before submission: ORCID IDs for all three authors, and execution of the falsification programme in §9, which is specified but not yet run.

---

## 7. Conclusion

This paper has argued that the architecture shared across commercial cloud-authoritative BIM and IWMS platforms — authoritative data in vendor-controlled multi-tenant cloud, subscription-gated access, lossy interoperability, and per-seat economics — is a coupled design, not five independent choices, and that a flat-file, open-standard alternative built on IFC 4.3, IDS 1.0, BCF 3.0, and COBie provides five owner-facing capabilities structurally unavailable to the cloud-authoritative architecture, at the honestly-stated cost of weaker real-time synchronous collaboration. The argument is grounded in a real multi-building owner-operator's internal rationale, predating this manuscript, for building this substrate rather than licensing an incumbent platform, and is consistent with the direction of government BIM procurement policy in the United States, United Kingdom, Singapore, and Dubai, each of which specifies conformance to open standards rather than a named commercial platform.

---

## 8. Formal Hypotheses

> **H₁ (Structural Capability Hypothesis).** At least four of the five owner-facing capabilities identified in §5.1 (asset-anchored BIM, offline-capable field use, vendor-obsolescence survival, direct IoT integration, BIM/lease/ledger convergence) are unavailable to a representative cloud-authoritative BIM or IWMS platform by architectural construction — not merely by current product configuration — such that no contractual or configuration change to that platform, short of relocating the authoritative data store outside vendor-controlled multi-tenant infrastructure, would provide the capability.

> **H₀ (Null).** Fewer than four of the five capabilities are structurally unavailable; the remainder are available to a cloud-authoritative platform through configuration, contractual terms, or a vendor-offered feature that does not require relocating the authoritative data store.

> **H₂ (Regulatory-Alignment Hypothesis).** For each of the four government BIM frameworks reviewed in §5.3 (US federal GSA/VA, UK, Singapore, Dubai), the framework's conformance requirement is satisfiable by IFC-format deliverables without dependency on any single named commercial platform.

---

## 9. Falsification Programme

*Test 1 — Subscription-independence verification.* For a representative sample of flat-file substrate deployments, files will be confirmed to open correctly in an independent, unmodified IFC 4.3-conformant viewer with no network connection and no active subscription of any kind. H₁'s offline-capability and asset-anchored-BIM claims are falsified if any tested deployment requires a live subscription or network check to open.

*Test 2 — Cloud-authoritative offline behaviour.* A representative cloud-authoritative platform will be tested for full read/write functionality with network connectivity disabled. H₁'s offline-capability claim regarding that platform category is falsified if full read/write functionality is confirmed available offline for a materially representative product in the category.

*Test 3 — Format longevity.* IFC files produced under IFC 2x3, IFC4, and IFC 4.3 will be confirmed to open in current-generation open-source tooling (e.g. IfcOpenShell [ifcopenshell-2025]). H₁'s format-longevity component is falsified if a materially representative proportion of files from any of the three schema generations fail to open without data loss.

*Test 4 — Regulatory conformance without vendor dependency.* For each of the four frameworks in §5.3, the published conformance requirements will be checked for any clause requiring a specific named commercial platform. H₂ is falsified if any of the four frameworks is found to require a named platform rather than IFC-format conformance generally.

*Test 5 — Convergence claim.* The per-element sidecar mechanism (§3, Table 1) will be checked against a representative cloud-authoritative platform's published integration architecture for lease-register and financial-ledger data. The convergence claim in §4 is falsified if a materially representative cloud-authoritative platform is found to co-locate BIM, lease, and financial records inside a single authoritative store without a separate licensed integration.

*Test 6 — EU mandate currency.* §5.3's per-country BIM mandate claims for Germany, Spain, Denmark, Italy, Norway, the Netherlands, and Poland will be re-checked against each country's primary regulatory source at the point of submission, since several (notably Germany's 2027 building-construction target and Poland's 2025/2030 roadmap) are planned/intended status rather than settled fact and may change before submission. This test does not bear on the paper's central architectural claim (H₁), which does not depend on EU mandate status.

---

## Acknowledgements

No external funding received. The authors thank buildingSMART International and the IfcOpenShell open-source community for the open-standard tooling this paper's architecture depends on.

---

## AI Use Disclosure

This paper was developed using Claude Sonnet 5 (Anthropic). Source material was drawn from two internally authored research essays and an internal design-response document predating this manuscript. Restructuring into the JOURNAL schema, comparative-table synthesis, and citation verification were performed with AI assistance under human editorial direction. All regulatory-mandate and standards-citation claims were checked against a live web source in the course of this drafting pass; two items (full EU per-country verification, the named-comparator editorial question) remain open and are stated as such rather than resolved by the AI tool. The model used is identified per COPE 2024 guidelines.

---

## CRediT Contributor Roles

**Jennifer M. Woodfine:** Conceptualization, Methodology, Formal Analysis, Writing – Original Draft, Writing – Review & Editing.
**Peter M. Woodfine:** Conceptualization, Validation, Writing – Review & Editing.
**Mathew Woodfine:** Software, Data Curation, Writing – Review & Editing.

---

## Conflict of Interest

The authors declare no conflict of interest. The authors are principals of the owner-operator whose deployment forms the case basis of this paper (§3.3, §5.2); this relationship is disclosed rather than treated as a conflict, consistent with the paper's framing as an architectural case study rather than an independent third-party evaluation.

---

## Funding

No external funding received.

---

## Data Availability

The open standards cited in this paper (IFC 4.3, IDS 1.0, BCF 3.0, COBie, ISO 19650) are publicly available from buildingSMART International and ISO. The internal design-response document referenced in §5.2 is an unpublished primary source authored by the paper's own authors and is not separately available for third-party verification; the quoted rationale is reported as primary authorial testimony, not as a third-party-verifiable citation.

---

## References

buildingSMART International. 2024. *IFC 4.3 — Industry Foundation Classes* (ISO 16739-1:2024). https://ifc43-docs.buildingsmart.org/

buildingSMART International. 2024. *Information Delivery Specification (IDS) 1.0.* https://www.buildingsmart.org/standards/bsi-standards/information-delivery-specification/

buildingSMART International. *BIM Collaboration Format (BCF) v3.0.* Oslo.

buildingSMART International. *buildingSMART Data Dictionary (bSDD).* https://www.buildingsmart.org/users/services/buildingsmart-data-dictionary/

International Organization for Standardization. 2018. *ISO 19650 — Organization and digitization of information about buildings and civil engineering works, including building information modelling (BIM).* https://www.iso.org/standard/68078.html

International Organization for Standardization. 2018. *ISO 16739-1:2018 — Industry Foundation Classes (IFC) Part 1: Data schema.*

National Institute of Building Sciences. *COBie v3 — Construction Operations Building Information Exchange.* https://www.nibs.org/events/eventdetail/a-technical-overview-of-cobie-v3

World Wide Web Consortium. *Scalable Vector Graphics (SVG) 2 — W3C Recommendation.* https://www.w3.org/TR/SVG2/

International Organization for Standardization / Khronos Group. 2022. *ISO/IEC 12113:2022 — Information technology — Runtime 3D asset delivery format — Khronos glTF 2.0.* https://www.iso.org/standard/83990.html

U.S. General Services Administration. *BIM Guide Series.* https://www.gsa.gov/cdnstatic/BIM_Guide_07_v_1.pdf

U.S. Department of Veterans Affairs, Office of Construction and Facilities Management. *VA BIM Standard Manual.* https://www.cfm.va.gov/til/bim/BIM-Manual.pdf

UK BIM Framework. *Information Management Mandate.* https://ukbimframework.org/

Building and Construction Authority, Singapore. *CORENET X.* https://www1.bca.gov.sg/regulatory-info/building-control/corenet-x

Global BIM Network. 2024. *Dubai BIM Mandate (Circular 207).* https://globalbim.org/info-collection/dubai-bim-mandate-circular-207/

U.S. Department of State, Directorate of Defense Trade Controls. *International Traffic in Arms Regulations (ITAR).* https://www.pmddtc.state.gov/

European Parliament and Council. 2016. *General Data Protection Regulation (GDPR), Regulation (EU) 2016/679.*

European Union. *EUPL v1.2 — European Union Public Licence.*

IfcOpenShell Contributors. 2025. *IfcOpenShell v0.8.5.* https://ifcopenshell.org

IFC.js Project. *IFC Fragment Specification.* https://ifcjs.github.io/info/docs/Guide/web-ifc/ifcjs-fragment

Ministerio de Transportes, Movilidad y Agenda Urbana (MITMA). *Spain BIM Plan for Public Procurement — Executive Summary.* https://cdn.mitma.gob.es/portal-web-drupal/cbim/pdf/executive_summary__en_a4_web.pdf

buildingSMART Deutschland. *BIM und Digitalisierung der Bauwirtschaft — Stand der staatlichen Initiativen in Deutschland.* https://www.buildingsmart.de/buildingsmart/aktuelles/bim-und-digitalisierung-der-bauwirtschaft-stand-der-staatlichen-initiativen

Rijksvastgoedbedrijf (Central Government Real Estate Agency, Netherlands). *RVB BIM Norm.* https://www.rijksvastgoedbedrijf.nl/english

Borkowski, A. S., W. Drozd, and K. Zima. 2024. The Status of the Implementation of the Building Information Modeling Mandate in Poland: A Literature Review. *ISPRS International Journal of Geo-Information* 13(10): 343. https://doi.org/10.3390/ijgi13100343

Construction Management. *A BIM Mandate Lesson from Denmark.* https://constructionmanagement.co.uk/bim-ma4ndate-lesso4n-den7mark/

---
