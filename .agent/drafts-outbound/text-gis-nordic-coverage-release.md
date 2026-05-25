---
schema: foundry-draft-v1
state: draft
language_protocol: PROSE-COMMS
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./comms/
target_filename: text-gis-nordic-coverage-release.md
audience: customer-woodfine
bcsc_class: current-fact
authored: 2026-05-06
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  Cluster counts derived from live clusters.geojson (May 6, 2026 build).
  Obs Bygg / Coop Norge format verified via OSM Overpass data.
  B&Q UK footprint confirmed via 356 OSM-ingested records.
research_inline: false
notes_for_editor: |
  Short release communications text. Bloomberg register. No superlatives.
  Open question: does Woodfine want a client-facing announcement or internal only?
  Translate to Spanish if going to bilingual wiki.
---

# Release Text — Nordic and UK Coverage Expansion

**For internal use / Woodfine client communications**

---

The Woodfine co-location intelligence platform now covers Norway, with 66 scored retail nodes — the first time Norwegian retail corridors have appeared in the dataset. The expansion reflects the completion of location data for Obs Bygg, the DIY hardware format operated by Coop Norge, and its systematic co-occurrence with Obs Coop hypermarket sites across the country.

Sweden and the United Kingdom also saw material coverage improvements. In Sweden, the addition of Bauhaus Sverige as a qualifying alpha secondary unlocked co-location scoring at IKEA nodes in Stockholm, Gothenburg, and six other metropolitan areas. In the United Kingdom, the confirmation of 356 verified B&Q locations resolved a prior data gap, producing 24 scored nodes at the 3 km radius — including sites near London, Birmingham, and Edinburgh.

The Europe dataset now includes 479 scored clusters at the standard 3 km radius, up from 466.

---

**Key figures (as of May 6, 2026):**

- Norway: 66 T2 Hub clusters (previously: none)
- Sweden: T2 clusters active in Stockholm, Gothenburg, Uppsala, Malmö corridors
- United Kingdom: 24 clusters at 3 km; 11 at 1 km
- Europe total: 479 clusters (3 km); 314 clusters (1 km)
- Global total: 4,237 scored clusters across North America and Europe

---

*All figures reflect OSM-sourced location data ingested through the Overpass API. Cluster counts vary by radius selection (1 km / 3 km) and reflect the V1 tier gate.*
