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
updated: 2026-05-16
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 9 Phase 4 (tier-label rebrand) and Sprint 17 Phase 1/4
  (second rename to ICSC hierarchy). Sprint 9 renamed T0–T3 numeric labels to
  Prime/Strong/Core/Emerging. Sprint 17 renamed again to Regional/District/Local/Fringe
  following G1 operator decision (2026-05-16). Companion to
  DESIGN-RESEARCH-tier-naming-accessibility.draft.md.
research_inline: false
notes_for_editor: |
  Bloomberg-register polish required. This draft reflects the Sprint 17 final names
  (Regional/District/Local/Fringe). The Sprint 9 Prime/Strong/Core/Emerging names are
  documented here as an intermediate state only. Pair with topic-co-location-methodology.md.
---

# Co-Location Tier Nomenclature

The co-location index assigns each cluster to one of four tiers based on the categorical composition of its anchor and secondary stores and on the cluster's position within its national population catchment. The tier labels visible on the map and in the inspector panel — **Regional**, **District**, **Local**, **Fringe** — follow the International Council of Shopping Centres (ICSC) retail property hierarchy, which is used by property developers, planners, and retail analysts across North America and Europe.

## What Each Tier Means

| Tier | Name | Description |
|---|---|---|
| 1 | **Regional** | A nationally significant co-location node. Contains both a hypermarket-format anchor and a warehouse club or lifestyle anchor (IKEA-format), is in the top decile of its country by primary catchment population, and has a regionally classified hospital within the civic ring. The highest tier. |
| 2 | **District** | A sub-regional trade-area node. Contains a hypermarket and a hardware or warehouse anchor, is in the top quartile of its country by primary catchment population, and has hospital access within the civic ring. |
| 3 | **Local** | A hardware or wholesale hub with community-level civic support. Contains at least one hardware or warehouse anchor, is in the top half of its country by primary catchment population, and has any hospital within the civic ring. |
| 4 | **Fringe** | Below threshold on one or more required gates. A commercial cluster with retail co-tenancy but insufficient catchment reach, composition, or civic support to qualify for Local or above. |

## Composition Chips

Each cluster also carries a composition descriptor displayed below the tier badge in the inspector. The descriptor names the anchor classes present, separated by "+": for example, "Hypermarket + Hardware + Warehouse" or "Lifestyle + Hypermarket". The four anchor classes are Hypermarket (general-merchandise stores: Walmart, Target, Mercadona, Tesco, Sainsbury's), Lifestyle (large-format home and furnishings: IKEA), Hardware (home improvement: Home Depot, Lowe's, Leroy Merlin), and Warehouse (membership warehouse clubs: Costco, Sam's Club, Makro).

## Naming History

The tier labels have been renamed twice since the platform launched.

**Sprint 9 (May 2026):** The original numeric labels (T3 Full Complement, T2 Retail Anchor, T0 Commercial Node, etc.) were replaced with plain-English nouns: Prime, Strong (Retail), Strong (Bulk), Strong (Hub), Core (Hyper), Core (Hardware), Core (Wholesale), Emerging. This resolved two reading-load failures: "+" ambiguity in compound descriptors, and the cognitive cost of mapping a tier number to a quality rank.

**Sprint 17 (May 2026):** The Sprint 9 labels were replaced with the ICSC hierarchy: Regional, District, Local, Fringe. The motivation was alignment with an internationally recognised nomenclature that carries meaning independently of platform-specific context. A planner who opens the inspector without reading any documentation knows what "Regional" means; "Prime" required learning.

The Spanish-language equivalents are: Regional, Distrital, Local, Marginal. The four-tier structure is preserved across both languages.

## What Changed and What Did Not

The Sprint 17 rename coincided with the introduction of a pure-predicate tier engine (V3). **Before Sprint 17**, tiers were assigned by a composite score (V2: base score + count bonus + diversity bonus + civic depth + overlap penalty). **From Sprint 17 forward**, tiers are assigned by binary gates: composition + national catchment rank + civic classification + spatial overlap limit. The gates are described in the Catchment Ranking Methodology document.

The tier *names* changed from Sprint 9 labels to ICSC labels. The tier *assignment method* changed from score-threshold to predicate-gate. Both changes shipped together in Sprint 17 (operator decision G1 and G6, 2026-05-16).

## Reading the Inspector Panel

When a user selects a cluster on the map, the inspector panel displays the tier name as a large coloured badge. Below the badge, a muted composition chip names the anchor classes present. A screen-reader user receives the full technical context via the badge's ARIA label: for example, "Regional tier cluster; 7 stores within 3 kilometres."

The badge colour encodes the hierarchy: dark navy for Regional, indigo for District, slate for Local, light grey for Fringe. Tier 4 Fringe uses dark text on a light background; all other tiers use white text.

## See Also

- [Retail Co-location Methodology](topic-co-location-methodology.md)
- [Catchment Ranking Methodology](topic-catchment-ranking-methodology.md)
- [Retail Brand Family Taxonomy](topic-retail-brand-family-taxonomy.md)
