# DECISIONS.md
# Confirmed Decisions and Open Items
**Version:** 1 · April 19, 2026
**This file must be updated as decisions are made. It is the current state of the project.**

---

## P1 — Blocking Items (Must Resolve Before Trial)

All five must be answered before the GCP node is provisioned.

| # | Item | Status |
|---|---|---|
| 1 | GCP account + billing confirmed | ✅ Confirmed |
| 2 | GPU tier for trial: confirm NVIDIA L4 available in your GCP region. If not, what is available? | **OPEN** |
| 3 | MacPro venv: run `pip list` and share the output | **OPEN** |
| 4 | Laptop-A: run `python3 --version` and share | **OPEN** |
| 5 | SSH key pair (Laptop-A ↔ GCP): does one already exist, or does it need to be generated? | **OPEN** |
| 6 | Top two levels of the 2.5 GB file tree on Laptop-A: share the output of `find . -maxdepth 2` or equivalent | **OPEN** |

---

## P2 — Architecture Decisions (Required Before Coding)

| # | Decision | Status | Choice |
|---|---|---|---|
| 7 | **Database for trial:** LadybugDB (MIT, Kùzu fork, active) vs archived Kùzu v0.11.3 (MIT, frozen)? LadybugDB is recommended — active development, CLA removed. Archived Kùzu is known-stable but no updates. | **OPEN** | Recommend: LadybugDB |
| 8 | **Domain count for Woodfine:** three fixed domains confirmed | ✅ **Corporate · Projects · Documentation — FIXED** |
| 9 | **Archetype standards:** both GICS 6010 (Real Estate sector) AND GLEIF/ISO 20275 (LP legal form) | ✅ **Both — dual-labeled** |
| 10 | **Wiki target audience:** clarified at time of wiki generation, not in architecture | ✅ **Clarify at generation time** |
| 11 | **Verification:** feature that is switched on separately, not part of core Phase 1 | ✅ **Switchable feature** |
| 12 | **Ad exchange target platform:** existing exchange (Google Ad Manager, The Trade Desk, OpenX) or self-hosted? Affects which OpenRTB 2.6 integration to build and consent tooling required. | **OPEN** — Phase 3 decision |
| 13 | **DARP compliance at geometric level:** confirmed as four structural invariants (I1–I4), testable as CI assertion | ✅ **Confirmed — geometric, not policy** |

---

## Confirmed Decisions (Do Not Reopen)

These are the settled architectural decisions from all research and discussion.

### Architecture

| # | Decision | Rationale |
|---|---|---|
| D1 | Six derivative layers (L0–L5) + optional Verification above L5 | Taylor-series attribution model — each layer is a summarization operator applied to the one below |
| D2 | L3 Glossaries and Topics are INDEX NODES — not content | "Topic node is the recipe. Wiki page is the dish." Wiki content is L5, generated FROM Topic nodes. |
| D3 | Themes are analytical outputs showing where data is MOVING OVER TIME | Not static labels. Re-evaluated every weekly batch. Archived Themes never deleted. |
| D4 | Each domain has its OWN Glossary set AND its OWN Topic set | Not a shared global Glossary. Corporate domain has corporate Glossaries/Topics. Projects domain has its own. |
| D5 | Three fixed domains for Woodfine (Corporate/Projects/Documentation) | Navigation convention. Other deployments configure unlimited domains in seeds/domains.csv. |
| D6 | IAB Content Taxonomy v3.0 → wiki PAGES (L5) only | Categorizes what a page is ABOUT. Applied at publication time. |
| D7 | IAB Audience Taxonomy v1.1 → Ad Exchange DATA SEGMENTS only | Categorizes what a data segment REPRESENTS. NOT applied to wiki content. |
| D8 | Ad Exchange is against the TOTAL DATA GRAPH — not wiki content | PropertyArchive IoT, CorporateArchive stakeholder data, PersonnelArchive professional network = the audience. |
| D9 | Verification layer is OPTIONAL, positioned ABOVE L5 outputs | Some operators will never verify. Higher verification = premium pricing. Cost vs. benefit is operator's choice. |
| D10 | No single source of truth — duplicates self-heal over time or remain as duplicates | Intentional design. Nodes never deleted. |
| D11 | Digital micropayments for verifiers | Polygon USDC primary (~$0.002/tx). Lightning Network backup. NOT Stripe/PayPal (fees too high for sub-$2 payments). |
| D12 | service-marketplace = ONE service, TWO modes | Data Marketplace mode + Ad Exchange mode. No separate service needed. |
| D13 | service-state merged into service-content | State tracking is internal to graph management. Reduces service count. |
| D14 | SOC3 and DARP are the ONLY governing regulatory frameworks | BCSC notes removed from all documentation. No BCSC-specific references in architecture docs. |
| D15 | Four generic L5 product categories (generic — not Woodfine-specific) | Content Generation · Wiki · Data Marketplace · Ad Exchange |
| D16 | ARMA GARP (2025 update) added to CoA standards stack | Eight principles inform Records & Information macro-domain |
| D17 | Laptop-A = substitute Totebox Archive for Phase 1 | os-totebox not stable enough for this test. Accepted and stated condition. |
| D18 | DARP compliance is geometric — four invariants (I1–I4) | Testable as CI assertion. Not a policy overlay. |

### Open Source and Licensing

| # | Decision | Rationale |
|---|---|---|
| D19 | 100% open source — everything must be forkable and developable standalone | "We own it." Commercial product must not depend on any non-forkable component. |
| D20 | LadybugDB (MIT, Kùzu fork) as primary graph DB | Active development, CLA removed, monthly releases, NaviX HNSW included. |
| D21 | graspologic-native (Microsoft, MIT, Rust Leiden) — NOT leidenalg (GPL-3) | leidenalg GPL-3 cannot link into a proprietary application. graspologic-native provides identical functionality. |
| D22 | SeaweedFS (Apache 2.0) — NOT MinIO | MinIO Community archived read-only February 13, 2026. AGPL-v3 since 2021. |
| D23 | OLMo 3 / Phi-4 / DeepSeek-R1 as genuinely OSI-approved LLMs | Llama, Gemma, Mistral Large all fail OSI definition. These three are genuinely Apache 2.0 / MIT. |
| D24 | Klaro! (BSD-3) as consent CMP — NOT IAB TCF | IAB TCF v2.3 requires €3,150+/yr fees, external runtime dependency, joint GDPR controllership. |
| D25 | W3C DPV 2.0 + ODRL + ISO 27560 + GPC + GPP as consent stack | $0, self-hostable, no external runtime dependencies. EU upgrade path: add TCF as GPP section. |
| D26 | Raphtory (GPL-3) treated as service boundary — not linked library | Use behind HTTP API or as separate process only. |

### Technical Stack

| # | Decision | Rationale |
|---|---|---|
| D27 | FASTopic (MIT, NeurIPS 2024) as primary topic modeler | Supersedes BERTopic — does not discard multi-domain short text as HDBSCAN outliers. |
| D28 | BERTopic 0.17.4 as backup topic modeler | Serialization: safetensors only, never pickle. |
| D29 | SkyPilot 0.11+ for yo-yo compute management | Spot instance auto-recovery, multi-cloud, declarative YAML. Production references: Shopify. |
| D30 | Pydantic v2 + instructor mandatory on all L5 wiki output | Citation grounding: every claim must link to L0 source. Unsupported claims flagged or dropped. |
| D31 | dbt-core + Dagster for derivative computation pipeline | dbt for SQL-expressible layers, Dagster for Python-heavy layers. |
| D32 | nomic-embed-text-v1.5 (Apache 2.0) as default embedding model | 137M params, 768-dim Matryoshka (down to 64), 8192-token context, fully open weights. |
| D33 | RF2 universal envelope on every node | `(id, effectiveTime, active, moduleId)` + Datomic op-bit + XTDB2 bitemporality + FHIR meta |
| D34 | GS1 GPC Brick 99999999 pattern for Temporary Classification | Formal parking lot for unclassified content with change-request workflow. Never silently dropped. |
| D35 | MeSH-style temporal fragmentation (no retroactive re-tagging) | Retroactive re-tagging costs more than the benefit. Version every layer transition instead. |
| D36 | Consent as a graph primitive using DPV RDF triples | Every query traversing to PersonalDataCategory nodes must pass through valid ConsentRecord edges. |

### L2 Foundation

| # | Decision | Rationale |
|---|---|---|
| D37 | gist (CC-BY, Semantic Arts) as the L2 domain-neutral kernel | ~100–135 classes, 20+ years battle-tested, OWL 2, aligned to BFO and Schema.org |
| D38 | REA (Resource-Event-Agent, ISO/IEC 15944-4) as L2 economic-semantics module | Fills gist's gap in Resource-Event-Agent duality and stockflow primitives. GL accounts become saved queries. |
| D39 | OntoUML as L2 meta-validator | Catches rigidity and identity errors via Kind/Phase/Role/Mixin stereotypes as annotations |
| D40 | openEHR-inspired two-level methodology for L2 | Tiny stable Reference Model + composable Archetype Model compiled to runtime Operational Templates |

---

## Outstanding Research Questions

| # | Question | Status |
|---|---|---|
| R1 | Four novel algorithms to invent: OCTH, REA-Aware Schema Inference, Vocabulary Drift Detector, Catastrophic-Forgetting-Free Ontology Evolution | Not yet started — post-trial work |
| R2 | What Gemma 4 prompt design best discovers derivative layers without seeding? | Requires trial data to calibrate |
| R3 | Glossary export cadence — after each weekly batch or on operator request? | Open |
| R4 | Consent management for Canadian users (PIPEDA) vs EU users — specific differences in implementation | Not yet designed |
| R5 | Digital micropayment mechanism — Stripe Treasury vs Polygon USDC vs Lightning — full compliance implications including MSB registration | Not yet evaluated |
| R6 | DARP compliance chain schema for DCAT catalog data products — exactly which fields constitute the chain? | To be defined before first product ships |
| R7 | Archetype promotion F1 threshold tuning — what are the correct τ_high and τ_low values for enterprise corpora? | Requires real data from trial |

---

## What Changed in Each Version

**V5 → V6:** L3 clarified (index nodes, not content). service-verification added. service-marketplace added. service-state merged into service-content. BCSC replaced with SOC3 and DARP. Specific data product examples removed (now generic). No source of truth principle stated. Digital micropayments added. service-marketplace = one service.

**V6 → V7:** Ad Exchange corrected (against total data graph, not wiki content). IAB taxonomy split confirmed (Content Taxonomy → wiki; Audience Taxonomy → Ad Exchange). Themes clarified (temporal trend analysis, not static labels). Verification layer moved to OPTIONAL above L5. ARMA GARP (2025) added to CoA stack. 100% open source confirmed. LadybugDB confirmed as primary graph DB. graspologic-native confirmed over leidenalg. SeaweedFS confirmed over MinIO. Domains confirmed as three fixed (Woodfine) + unlimited (other deployments). DARP as four geometric invariants confirmed. gist + REA + OntoUML as L2 foundation confirmed. openEHR two-level methodology confirmed. RF2 universal envelope confirmed. Research synthesis integrated.
