---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-retail-brand-family-taxonomy.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Session 8 brand family taxonomy expansion (2026-05-08).
  Food/Furniture/Pharmacy/Department families added to complete the classification.
  Source: CHAIN_FAMILY dict in build-tiles.py; familyToCat mapping in index.html.
  Observation trigger: Save-On-Foods displaying "Hardware" badge — family mapping gap
  discovered; four new families added to cover grocery, home furnishings, pharmacy,
  and traditional department store chains.
research_inline: false
notes_for_editor: |
  Bloomberg-register polish required.
  Note that the Food family is data-layer only — it appears on the map but does not
  affect cluster scoring. This distinction is commercially important; do not elide it.
  "Department" family exists for coverage completeness only (Macy's, London Drugs edge cases).
---

# Retail Brand Family Taxonomy

Every retail location on the co-location map carries a brand family classification — a categorical label that determines how the location is displayed and, for some families, whether it contributes to cluster scoring. The taxonomy was designed around the specific anchor types used in the co-location methodology while remaining extensible to the full range of operators ingested as supporting data.

## The Six Primary Families

### Hypermarket

Large-format general merchandise and grocery retailers operating facilities of 80,000 square feet or more. Includes Walmart Supercenter, Target, Carrefour Hypermarket, IKEA, and equivalents across North American and European markets. Hypermarket stores are the primary anchor type in the co-location methodology — cluster formation requires one Hypermarket anchor. The family is displayed with a navy badge.

### Hardware

Home improvement and building materials retailers. Includes Home Depot, Lowe's, Leroy Merlin, Canadian Tire, Hagebaumarkt, Praktiker, and regional equivalents. Hardware co-presence within 1 km of the primary anchor is the first scoring criterion in the co-location index. Displayed with an orange badge.

### Warehouse

Membership and cash-and-carry retailers operating under a wholesale or club format. Includes Costco, Sam's Club, BJ's Wholesale, Makro, and Metro. Warehouse co-presence within 3 km of the primary anchor is the second scoring criterion. Displayed with a teal badge.

### Food

Conventional and specialty grocery operators. Includes Save-On-Foods, Safeway, Whole Foods, Lidl, Mercadona, Biedronka, and regional grocery chains across all covered markets. Food-family stores are ingested and displayed on the All Locations layer as supporting context — they are visible on the map at Retail Level — but they do not affect cluster scores or tier assignments. This preserves the co-location index as a measure of large-format anchor convergence rather than general retail density. Displayed with a green badge.

### Furniture

Home furnishings and décor specialists operating at scale. Currently populated by Conforama in the Spanish market, with expansion to other European furniture retailers anticipated. Displayed with a purple badge.

### Pharmacy

Health and pharmacy retail with a significant non-pharmaceutical product mix. Currently populated by London Drugs in the Canadian market. Displayed with a violet badge.

## Supporting Families

### Department

Traditional department store operators not classified as large-format hypermarkets. Currently populated by Macy's in the United States. Department stores typically share commercial zones with anchor-category retailers but represent a distinct format that does not map to any of the six co-location methodology tiers. Displayed with a neutral grey badge.

## Medical and Academic

Hospital and university locations, ingested as civic infrastructure, are classified outside the brand family system using a category identifier rather than a family identifier. Medical (hospital) and Academic (university) locations are displayed with distinct badges and contribute to the tertiary scoring tiers in the co-location methodology.

## Classification Scope

All ingested locations carry one of these classifications in the `brand_family` field written during tile construction. Locations without a recognised chain identifier receive no brand family and display without a category badge. Unclassified locations remain visible on the map as grey dots and do not affect scoring.

## See Also

- [Retail Co-location Methodology](topic-co-location-methodology.md)
- [Guide: Adding a New Chain to the GIS Pipeline](guide-gis-adding-a-chain.md)
