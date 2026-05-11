---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-co-location-tier-nomenclature.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 9 Phase 4 (tier-label rebrand). Companion to
  DESIGN-RESEARCH-tier-naming-accessibility.draft.md (the design-system
  artefact). This is the customer-facing methodology explanation.
research_inline: false
notes_for_editor: |
  Bloomberg-register polish required. Pair this with the existing
  topic-co-location-methodology.md — that document defines tiers; this
  document defines tier *labels*. The two should cross-reference.
---

# Co-Location Tier Nomenclature

The co-location index assigns each cluster to one of four tiers based on the categorical composition of its anchor and secondary stores. The tier labels visible on the map and in the inspector panel — Prime, Strong (Retail), Strong (Bulk), Strong (Hub), Core (Hyper), Core (Hardware), Core (Wholesale), Emerging — are designed to be readable at a glance and accessible to readers who navigate by screen reader or who process compound nouns with difficulty.

## What Each Label Means

| Label | Composition | Meaning |
|---|---|---|
| **Prime** | hypermarket + hardware + warehouse club | All three large-format anchor categories converge within the catchment. Highest cluster grade. |
| **Strong (Retail)** | hypermarket + hardware | Two-format convergence; the dominant pairing in suburban North American power centres. |
| **Strong (Bulk)** | hypermarket + warehouse club | Two-format convergence; bulk-purchase node. Common in periurban markets with car-borne shoppers. |
| **Strong (Hub)** | hardware + warehouse club | Two-format convergence; home-improvement-and-bulk node, no anchor hypermarket. |
| **Core (Hyper)** | hypermarket only | Single anchor; the cluster forms around a hypermarket alone. |
| **Core (Hardware)** | hardware only | Single anchor; the cluster forms around a hardware retailer alone. |
| **Core (Wholesale)** | warehouse club only | Single anchor; the cluster forms around a warehouse club alone. |
| **Emerging** | none of the above | A commercial node with one or more retail tenants but no large-format anchor of any of the three categories. |

The four base words form a monotonic hierarchy: Prime > Strong > Core > Emerging. The parenthetical specialty marker (Retail, Bulk, Hub, Hyper, Hardware, Wholesale) preserves the diagnostic detail of which anchor type or types dominate the cluster, without interrupting the reading flow.

## What Replaced What

The labels were renamed in May 2026 from a numeric-prefix scheme that combined a tier number (T0–T3) with a technical compound descriptor: "T3 Full Complement", "T2 Home + Bulk Hub", "T1 Wholesale Node", and so on. The numeric prefix carried no useful signal once the cluster was on the map; the compound descriptor read ambiguously, with "+" symbols that could be parsed as additive or alternative.

Customer review during the Sprint 9 audit period identified two reading-load failures: the "+" ambiguity, and the difficulty of recalling which numeric tier was best when the descriptor required parsing. Plain English nouns were chosen as a remedy.

## Methodology Is Unchanged

The labels are a presentation change. The underlying co-location methodology — how clusters are detected, how anchors are identified, how scoring is computed, how thresholds are calibrated — is unchanged from earlier publications. A cluster that was a "T3 Full Complement" in April 2026 is a "Prime" cluster in May 2026 with identical geometry, anchor list, score, and rank. The data is the same; the words on the badge are different.

## Reading the Inspector Panel

When a user clicks a cluster on the map, the inspector panel displays the tier label as a coloured badge alongside the cluster's regional name, store count, and rank within country and continent. The badge colour encodes the same hierarchy: deeper saturation for higher tiers. A screen-reader user receives the full technical context via the badge's ARIA label, which restates the original tier number, the Plain-English label, and the geometric site count: "Tier 3 cluster: Prime; 7 stores within 3 kilometres."

## See Also

- [Retail Co-location Methodology](topic-co-location-methodology.md)
- [Retail Brand Family Taxonomy](topic-retail-brand-family-taxonomy.md)
- [North American Tier Index](topic-tier-index-north-america.md)
