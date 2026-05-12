---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: text-gis-canada-walmart-hospital-coverage.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TEXT
authored: 2026-05-12
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 13 follow-on (2026-05-12). Two pipeline bugs identified
  and corrected: SKIP_NAME_SUBSTRINGS "supercentre" filter and civic OSM data path
  absence from build_layer1(). Impact measured against live pipeline: walmart-ca
  253 → 453 records, 6,422 → 6,815 clusters, layer1 +60,756 civic records.
research_inline: false
notes_for_editor: |
  Bloomberg-register polish required. Strict word budget — release note format.
  Do not frame as a bug fix in customer-facing copy; frame as a coverage
  expansion. The pipeline correction is an internal fact; the customer-visible
  outcome is broader data and more clusters.
  Companion to text-gis-uk-eu-coverage-release.draft.md.
---

# Woodfine GIS — Canada Coverage and Institutional Data Update

The Woodfine Location Intelligence platform expanded its Canadian retail cluster coverage and added hospital and university data globally in the May 2026 maintenance release. Changes are live at gis.woodfinegroup.com.

## Walmart Canada Cluster Expansion

The platform now maps 453 Walmart Canada locations, an increase from the prior 253. The expanded dataset recovers stores operating under the Walmart Supercentre format — the predominant large-format Walmart configuration in Canada — which were previously absent from the index.

Canada's co-location cluster count rose from 483 to approximately 637, with new clusters appearing in suburbs and secondary markets where Walmart Supercentre locations co-locate with Canadian Tire or other anchor retailers. The Strathcona County retail node east of Edmonton, for example, now carries four distinct co-location clusters, including a new Tier 2 Hub anchored by a Walmart Supercentre and a Canadian Tire located 730 metres away.

Walmart Canada remains classified as an Alpha Anchor — the tier reserved for large-format retailers whose presence materially determines the commercial character of a trade area.

## Hospital and University Data — Global

Hospitals and universities now appear as data points throughout the platform's map at Retail Level zoom. The May 2026 release adds approximately 36,000 hospital locations and 25,000 university locations drawn from the OpenStreetMap civic dataset, extending existing coverage derived from the Overture Maps Foundation.

The additions are global in scope, with meaningful new coverage in France, Mexico, Germany, Italy, Canada, Spain, the United Kingdom, and Poland. In Canada specifically, approximately 4,400 hospital and university records are now present — including regional hospitals, community health centres, and post-secondary campuses.

Hospitals appear in red on the map; universities appear in green. Both are visible at Retail Level zoom alongside the retailer dot layer. They do not contribute to co-location cluster grades, which remain anchored to the large-format retail methodology. Their purpose is to give field operators spatial context about institutional generators — traffic sources that influence the catchment dynamics of nearby retail clusters.
