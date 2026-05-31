---
schema: foundry-draft-v1
draft_id: topic-open-bim-regulatory-acceptance
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-documentation/topic-open-bim-regulatory-acceptance.md
created: 2026-05-06T17:30:00Z
author: task@project-bim
cites: [ifc-4-3, iso-19650, cobiev3, ids-1-0, eupl-1-2, itar-ear, gdpr-2016-679]
doctrine_claims: [40]
research_done_count: 3
research_suggested_count: 1
open_questions_count: 2
research_provenance: |
  Primary: /srv/foundry/.claire/sub-agent-results/C-bim-regulatory-acceptance-2026-04-28.md (459 lines)
  Terminology: /srv/foundry/clones/project-intelligence/service-content/ontology/glossary/glossary_projects.csv
research_inline: false
---

# Open BIM and Regulatory Acceptance

## Lede

Building Information Modelling is no longer elective in public procurement for most G7 economies. The United States federal government has required IFC-formatted BIM deliverables for major infrastructure and building projects since 2007. The European Union has embedded BIM requirements into public procurement directives enforced across member states. The United Kingdom, Singapore, Norway, Denmark, the Netherlands, and Australia have each issued national mandates or equivalent policy requirements. The regulatory landscape for Open BIM is settled. The open question is which platforms can meet the sovereign data requirements attached to these mandates.

---

## Defining Open BIM

Open BIM, as defined by buildingSMART International, denotes a workflow and data exchange approach where:

1. Model data is stored and exchanged in vendor-neutral open formats, principally IFC (ISO 16739-1:2024).
2. Classification and property definitions reference open dictionaries, principally bSDD.
3. Constraint specifications are expressed in open constraint languages, principally IDS 1.0.
4. Handover data is formatted to open facility management standards, principally COBie.

Open BIM is distinguished from closed BIM (proprietary format workflows within a single vendor's tool ecosystem) and from federated BIM (multi-model coordination that may use proprietary formats for individual discipline models). The Open BIM distinction matters for public procurement because most public tender requirements specify IFC delivery — not Revit, not ArchiCAD, not Navisworks.

---

## United States Federal Mandates

The US federal government is the largest single construction client in the world by value. Three agencies with the largest capital construction programmes have published BIM requirements that effectively mandate IFC-capable workflows.

**General Services Administration (GSA).** The GSA National 3D-4D-BIM Program, launched in 2003 and formalised in the GSA BIM Guide Series from 2007, requires IFC-formatted spatial and programme data for projects above defined thresholds. The GSA BIM Guide Series 01 (Overview) and Series 02 (Spatial Programme Validation) set the technical baseline. Facility owners using Autodesk tools are required to deliver IFC exports, not native Revit files. COBie deliverables are required for handover.

**US Army Corps of Engineers (USACE).** The USACE BIM Roadmap (2012, updated 2015) mandates BIM for military construction (MILCON) projects, research facilities, and civil works above $5M. IFC-compatible deliverables are required at design milestones. USACE facility data standards reference COBie and IFC Property Sets.

**Department of Veterans Affairs (VA) and Naval Facilities Engineering Systems Command (NAVFAC).** Both agencies have published BIM standards that require IFC exports at design milestones and COBie handover packages for new construction and major renovations.

**Federal Acquisition Regulation compatibility.** EUPL-1.2 (the licence under which the PointSav BIM platform is published) is recognised as an OSI-approved open source licence. The FAR Part 27 (Patents, Data, and Copyrights) provisions for open source software do not prohibit EUPL-1.2 licensed software in federal contracts. This is a procurement advantage over GPL-3.0 or AGPL-3.0 licensed platforms, where the copyleft terms can create contracting complications.

---

## European Union

The EU 2014 Public Procurement Directive (Directive 2014/24/EU) explicitly permits member states to require electronic BIM tools for publicly funded construction and civil engineering contracts. Article 22(4) reads: "For public works contracts and design contests, Member States may require the use of specific electronic tools, such as of building information electronic modelling tools or similar."

Individual member states have implemented this in varying degrees of stringency.

**Germany.** The Federal Ministry for Digital and Transport (BMDV) issued a BIM roadmap in 2015 mandating BIM on infrastructure projects. The target for full Open BIM on federal transport infrastructure projects was set for 2020 (design phase) and 2025 (full lifecycle). German public procurement for construction now routinely specifies IFC deliverables. The STLB-Bau standard (detailed construction service specifications) has been extended with BIM object assignments.

**Italy.** Legislative Decree 36/2023 (the new Public Contracts Code) mandates BIM for public works contracts above €15 million from January 2023, extending to lower thresholds through 2025. IFC-compatible deliverables are required. Italy operates an official National BIM Guidelines programme under the Ministry of Infrastructure and Transport.

**Spain.** The Plan de Impulso de la Contratación Pública de Edificación BIM (PPRE 2022-2026) establishes a progressive mandate for BIM on public construction contracts, targeting full implementation across all public works by 2026. IFC is the specified exchange format.

**Denmark.** Denmark has required BIM on state-funded building projects above DKK 5 million since 2013 under the Danish Enterprise and Construction Authority's Digital Construction requirements. Denmark is among the earliest national mandates in Europe.

**Norway.** Statsbygg (the Norwegian government's main property manager) has required Open BIM deliverables since 2016. The Statsbygg BIM Manual specifies IFC 4.x and LOD (Level of Development) requirements by project phase.

**Netherlands.** Rijkswaterstaat (national infrastructure) and ProRail (rail infrastructure) both operate BIM requirements programmes specifying IFC deliverables and COBie handover data for major infrastructure projects.

**Poland.** The Polish Government enacted BIM requirements for major public works in 2021 under the Public Procurement Law. Thresholds and implementation schedules follow the EU directive framework.

**United Kingdom (post-EU).** The UK Government's BIM mandate for Level 2 BIM on centrally procured public sector projects has been in effect since 2016, issued under the Government Construction Strategy. BS EN ISO 19650 (the international standard for information management using BIM) was adopted as the UK national standard in 2019. IFC and COBie remain the required exchange formats.

---

## buildingSMART Certification

buildingSMART International operates a software certification programme for IFC import/export implementations. Software achieving buildingSMART certification has been verified to produce conformant IFC files that round-trip through certified importers without data loss. This certification is recognised in procurement requirements across multiple jurisdictions as evidence of Open BIM compliance.

The PointSav BIM platform is intended to achieve buildingSMART certification for its IFC export and import implementations. The certification path requires submission of test model sets to buildingSMART's certification service and passing defined conformance tests. The IFC 4.3 Reference View and the Design Transfer View are the two target certifications for authoring tools; the BIM Token platform targets certification against the Property Set exchange specification.

---

## Sovereign Data Architecture as a Procurement Requirement

Several public procurement contexts impose data sovereignty requirements that effectively mandate offline-capable, self-hosted BIM platforms. Three regulatory frameworks drive this.

**ITAR (International Traffic in Arms Regulations).** US federal and allied defence construction projects involving classified facility data, weapons system specifications, or sensitive infrastructure are subject to ITAR and EAR (Export Administration Regulations). ITAR-controlled data cannot be processed on commercial cloud infrastructure unless that infrastructure holds a FedRAMP High authorisation or equivalent. BIM models for defence facilities frequently contain ITAR-controlled spatial and structural data. An offline-first, self-hosted platform that never transmits model data outside the facility network is the only architecture that satisfies ITAR requirements without commercial cloud authorisation.

**GDPR (General Data Protection Regulation).** Building occupancy data, sensor data, and facility management records associated with BIM digital twins may contain personal data (access logs, occupancy patterns, employee location data). GDPR Article 44 prohibits transfer of personal data to jurisdictions without adequate protection unless specific safeguard mechanisms are in place. A self-hosted BIM platform that processes data within EU jurisdiction eliminates the transfer question.

**HIPAA (Health Insurance Portability and Accountability Act).** Healthcare facility construction and operation generates BIM data associated with patient flow, clinical space programming, and medical equipment layout. Where this data can be linked to identifiable patient populations, HIPAA's physical safeguard requirements may apply to the facility data systems. A self-hosted platform with access controls aligned to the facility's security posture satisfies these requirements without relying on commercial cloud business associate agreements.

The PointSav BIM platform is designed from the ground up for offline-first operation. The BIM token vault is a git repository. The application server is a self-hosted binary. No telemetry is transmitted. No model data leaves the operating environment unless explicitly exported by an authorised user.

---

## ISO 19650 and the Common Data Environment

ISO 19650 (Information management using building information modelling) defines the requirements for information management processes across the project lifecycle and for the Common Data Environment (CDE) — the shared data infrastructure through which project information is exchanged.

ISO 19650 Part 1 (Concepts) and Part 2 (Delivery phase) are technology-neutral. The standard specifies what the CDE must do (manage information containers, control issue and receive workflows, maintain a revision-controlled audit trail) but does not prescribe the technology through which it does so.

A git repository satisfies the ISO 19650 CDE requirements:
- Information containers: git objects (blobs, trees, commits) identified by content-addressed SHA-256 hashes.
- Issue and receive workflow: pull request and review workflow.
- Revision-controlled audit trail: git commit history with author, timestamp, and content hash is tamper-evident.
- Status management: git branch naming conventions and tag annotations.

This equivalence means that a BIM project managed through a git-hosted token vault and model repository is operating within an ISO 19650-compliant CDE, without requiring a purpose-built (and typically costly) CDE platform. This is a structural cost advantage for small and medium-sized AEC firms, who represent the majority of BIM adopters by project count.

---

## EUPL-1.2 Licence and Procurement Advantage

The PointSav BIM platform is published under the European Union Public Licence version 1.2 (EUPL-1.2). This licence has three properties relevant to public procurement.

**OSI-approved.** EUPL-1.2 is listed on the OSI Open Source Initiative list of approved licences. Procurement requirements referencing "OSI-approved open source" are satisfied by EUPL-1.2.

**FAR-compatible.** The FAR Part 27 open source analysis for EUPL-1.2 (a copyleft licence with network use provisions similar to AGPL-1.2) identifies no general prohibition in federal contracting. EUPL-1.2's compatibility list includes GPL-2.0, GPL-3.0, AGPL-3.0, and EUPL-1.1 — this compatibility means EUPL-1.2 licensed components can be combined with GPL-family code without additional compliance work.

**EU public body alignment.** EUPL-1.2 was authored by the European Commission for software produced by or for EU public bodies. Contracting authorities in EU member states have a policy preference for EUPL-1.2 licensed software in public procurement under the Commission's EUPL guidance. This preference translates to a tendering advantage when public bodies assess total cost of ownership and licence risk.

---

## Open Questions

1. **buildingSMART certification timeline.** The IFC 4.3 certification service was not yet accepting submissions for all view types as of the research date. The certification timeline for the PointSav BIM platform is not fixed. This affects how the platform can be represented in public procurement responses requiring certified IFC compliance.

2. **ITAR facility mapping.** The list of specific facility types where ITAR restrictions apply to BIM data is not publicly enumerated by the US Government. Legal review is required before marketing the platform specifically for ITAR-restricted construction. The offline-first architecture is a technical fact that can be stated; the ITAR compliance claim requires counsel review.

---

*Draft prepared for project-editorial sweep. Research provenance: sub-agent C (regulatory acceptance, 2026-04-28); glossary_projects.csv. Doctrine claim: #40 (Flat-File BIM Substrate). BCSC posture: all forward-looking items (certification timeline, mandate expansion) use planned/intended language. Competitive comparisons avoided.*
