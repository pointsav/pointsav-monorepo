---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-co-location-intelligence-overview.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-02T04:00:00Z
authored_by: task-project-gis
authored_with: claude-sonnet-4-6
references:
  - https://gis.woodfinegroup.com
  - ~/Foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/config.py
  - ~/Foundry/deployments/cluster-totebox-personnel-1/data/service-business/
  - ~/Foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/build-clusters.py
notes_for_editor: |
  This is the introductory TOPIC for the Woodfine co-location intelligence system.
  Audience: Woodfine board members, LP directors, 65+ executive readers.
  Tone: Bloomberg article standard — precise, factual, no AI-marketing vocabulary.
  Target length: 600–900 words English.

  When refining:
  - Apply BCSC disclosure posture: anything forward-looking gets "planned/intended/target" language
  - The site count (2,738) and retailer count (31,219+) are current-fact as of 2026-05-02
  - Avoid describing the algorithm as "AI" or "machine learning" — it is a deterministic
    named-anchor combination matrix
  - The Michelin Star analogy in the criteria TOPIC (#2) should not appear here — this is
    the overview; let #2 carry that framing
  - Generate .es.md overview per Doctrine §XII; Spanish overview, not 1:1 translation
  - Suggested section headings: retain as written or improve, do not change concepts
  - Citations to register if not present: Overture Maps Foundation CDLA-2.0 licence,
    OpenStreetMap ODbL licence
---

# Retail Co-location Intelligence — Overview

Retail co-location intelligence is the systematic identification and ranking of
geographic sites where large-format retail categories converge within a defined
catchment radius. The approach answers a specific commercial question: at which
locations does a hypermarket anchor coexist with a warehouse club, a home improvement
superstore, and supporting civic infrastructure — and how does the strength of that
combination vary by market?

Woodfine Management Corp. operates a proprietary co-location intelligence platform
at [gis.woodfinegroup.com](https://gis.woodfinegroup.com), built and maintained by
PointSav Digital Systems. The platform covers eight retail markets across North
America and Europe.

---

## Purpose

Large-format retailers do not locate arbitrarily. Supercenter operators, warehouse
clubs, and home improvement superstores each independently apply capital-intensive
site selection criteria — traffic counts, household income density, road-network
accessibility, and competitive positioning. When two or three such operators converge
on the same node within a given corridor, that convergence signals a validated
commercial location: one where multiple independent parties have independently
committed capital to serve the same trade area.

The co-location intelligence system identifies and ranks those nodes. The output is
a ranked index of sites, expressed as five quality tiers, which can be filtered by
region, country, and secondary radius.

---

## Geographic Coverage

The platform covers 8 retail markets across 13 countries:

| Region | Countries | Anchor operators |
|--------|-----------|-----------------|
| United States | US | Walmart, IKEA |
| Canada | CA | Walmart, IKEA, Real Canadian Superstore |
| Mexico | MX | Walmart, IKEA |
| Spain | ES | IKEA, Carrefour, Alcampo, Leclerc |
| Italy | IT | IKEA, Carrefour, Ipercoop, Iper La Grande, Bennet |
| Greece | GR | IKEA |
| Poland | PL | IKEA, Carrefour, Leclerc, Auchan |
| Nordics | SE · NO · DK · FI · IS | IKEA, Bilka, Prisma, K-Citymarket, Obs Coop |

---

## Dataset

The platform integrates three data sources:

**Service-business locations** (retail operators): sourced from OpenStreetMap via
Overpass API, filtered by brand Wikidata identifier. As of 2 May 2026, the dataset
contains 31,219+ individual retail locations across 60+ chains.

**Service-places locations** (civic infrastructure): hospital and medical centre
records from the Overture Maps Foundation Places dataset (2026-04-15 release,
CDLA Permissive 2.0 licence). As of 2 May 2026: 50,000 healthcare records and
112,791 higher education records.

**Airport data**: 29,020 airport and aviation facility records from Overture Maps
Foundation, retained for future tertiary scoring.

---

## Co-location Sites

The current dataset contains **2,738 ranked co-location sites** across all eight
regions: 2,488 in North America and 250 in Europe. Sites are ranked using the
named-anchor combination matrix described in the accompanying methodology topic.

| Tier | Description | NA count | EU count |
|------|-------------|----------|----------|
| Tier 5 | Full co-location | 102 | 0 |
| Tier 4 | Strong co-location | 259 | 9 |
| Tier 3 | Partial co-location | 1,396 | 175 |
| Tier 2 | Limited co-location | 333 | 23 |
| Tier 1 | Anchor only | 398 | 43 |

The absence of Tier 5 sites in Europe reflects current data coverage: the tertiary
scoring dimension (healthcare and higher education) currently draws on Overture data
that has stronger North American coverage. Expansion of European tertiary data is in
progress.

---

## Platform

The GIS platform renders the ranked site index as an interactive map using MapLibre
GL, PMTiles, and OpenFreeMap basemap tiles. The platform is served from
[gis.woodfinegroup.com](https://gis.woodfinegroup.com) and is accessible to
Woodfine board members, LP directors, and authorised Woodfine Management Corp.
personnel.

Site selection controls include:
- Region filter (North America / Europe; individual country chips)
- Secondary radius selector (1 km · 2 km · 3 km) — affects which secondary operators
  qualify as co-located with a given anchor
- Layer toggles: co-location clusters, catchment radius polygons, full retail
  locations

The platform is updated when new chain data is ingested or when the ranking
algorithm is recalibrated. Version and dataset counts are displayed in the platform
header.

---

*Data sources: OpenStreetMap contributors (ODbL); Overture Maps Foundation
(CDLA Permissive 2.0). Retail location data current as of 2 May 2026.*
