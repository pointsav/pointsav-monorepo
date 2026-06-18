---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Location Intelligence Co-location Archetypes"
slug: topic-location-intelligence-archetypes
language: en
status: draft
paired_with: TOPIC-location-intelligence-archetypes.es.draft.md
target_repo: content-wiki-projects
target_path: topics/topic-location-intelligence-archetypes.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: "BRIEF-location-intelligence-archetypes-2026-06-01.md (project-system briefs/archive/, author totebox@project-gis); test-cluster-archetypes.py run 2026-06-01; ingest-osm-airports.py run 2026-06-01 (4,024 airports); ingest-osm-railway.py run 2026-06-01 (18,107 intercity stations)"
research_inline: true
created: 2026-06-11
author: totebox@project-gis (claude-sonnet-4-6)
---

# Location Intelligence Co-location Archetypes

The Location Intelligence platform identifies retail and commercial gravity
through three co-location archetypes: Retail Centres (PRO), Urban Fringe
(VWH), and Commuter (PKS). Each archetype describes a distinct clustering
pattern that reflects a different type of commercial activity and a different
relationship to the surrounding urban geography.

The three-letter codes were ratified on 1 June 2026.

## The three archetypes

| Code | Name | Anchor type | Status |
|------|------|-------------|--------|
| **PRO** | Retail Centres | Grocery hypermarket with hardware and at least one of: price club, lifestyle, or electronics | Live — T1/T2/T3 co-location pipeline |
| **VWH** | Urban Fringe | Hardware + trade-supply ecosystem (MRO, tool rental, builders merchant, auto parts) | Live — 6,368 clusters (T1=852 / T2=1,327 / T3=4,189) |
| **PKS** | Commuter | Regional transit anchor (airport, rail, bus) + park-and-ride + car rental/hotel enrichment | Live — 6,953 clusters (T1=691 / T2=2,658 / T3=3,604) |

PRO is the base map product — the foundation of the site-selection dataset.
VWH and PKS are overlay archetypes that identify adjacent market structures
not captured by grocery-anchored clustering.

---

## PRO — Retail Centres

PRO clusters represent grocery-anchored commercial co-locations at three
scales. The pipeline groups anchor-category retail locations that fall within
a defined span distance, then assigns each group to one of three tiers based
on anchor composition.

### Tier definitions

**T1 — Regional:** A cluster containing a grocery hypermarket and a hardware
retailer, plus at least one of a price club, lifestyle retailer, or electronics
retailer. Alternatively: four or more anchor-category retailers in a tight
cluster (span ≤ 1 km), or three or more anchors in any tight cluster.

**T2 — District:** A cluster containing a grocery hypermarket and a hardware
retailer, with a span no greater than 2.5 km.

**T3 — Local:** All remaining anchor pairs that do not qualify for T1 or T2.

### Current dataset (Phase 23 + Change B, 28 May 2026)

| Tier | Clusters | Countries |
|------|----------|-----------|
| T1 | 1,746 | 17 |
| T2 | 2,726 | 17 |
| T3 | 2,021 | 17 |
| **Total** | **6,493** | |

The dataset covers 17 display countries across North America and Europe.
The SPAN_T2_MAX_KM parameter was set to 2.5 km in the Change B rebuild,
tightening the T2 boundary relative to earlier phases.

---

## VWH — Urban Fringe

VWH clusters identify concentrations of hardware and industrial-supply
retailers in the absence of grocery anchors. These sites occupy the urban
fringe — locations between 5 and 80 km from a major metro centre — and
tend to cluster around highway interchanges in areas with adjacent industrial
landuse.

### Definition

A VWH candidate is a location where one or more hardware retailers are
present, no grocery hypermarket is within the cluster span, and the site
sits within metro-distance 5–80 km. The typical built form is a 3–6 story
multi-storey warehouse or light-manufacturing building, distinct from the
one-storey big-box format of the retail park.

VWH locations serve trades contractors, light-manufacturing operators,
and just-in-time logistics tenants — not general retail consumers.

### Co-location signals

**Essential:**

| Signal | Rationale |
|--------|-----------|
| Highway interchange ≤ 2 km | Truck ingress and egress |
| Population ≥ 300,000 within 30-minute drive | Manufacturing and logistics labour |
| Industrial landuse adjacent | Zoning compatibility |

**Significant:**

| Signal | Rationale |
|--------|-----------|
| Air cargo airport ≤ 20 km | Electronics and components, rapid replenishment |
| Freight rail ≤ 2 km | Just-in-time component delivery |
| Transit corridor ≤ 500 m | Workforce access |

**Disqualifying:** Dense residential immediately adjacent; flood plain;
heritage conservation zone; location inside a PRO cluster.

### Production results (11 June 2026)

The VWH pipeline is production-grade. Hardware stores (10,338 locations, 45 chains)
were profiled as proxy anchors; DBSCAN was run on trade-supply POIs without the hardware
anchor (held-out validation); tier rules use group-collapse logic validated at 73.4%
hardware co-location on T1+T2 clusters (acceptance threshold: 55%).

| Country | Clusters |
|---------|---------|
| United States | 3,167 |
| Germany | 648 |
| United Kingdom | 543 |
| Canada | 506 |
| France | 420 |
| Netherlands | 240 |
| Italy | 226 |
| Poland | 171 |
| **Total (17 countries)** | **6,368** |

Tier distribution: T1 (Full Trade Hub) = 852 (13.4%), T2 (Established) = 1,327 (20.8%),
T3 (Emerging / Thin) = 4,189 (65.8%). T3-heavy distribution is expected: full trade hubs
combining MRO, tool rental, builders merchant, and auto parts are legitimately rare.

A `retail_contamination` flag marks clusters where a grocery hypermarket lies within 1 km
of the centroid (3,048 clusters; 47.9%). These are dual-use commercial parks — valid VWH
co-locations that also include grocery retail.

---

## PKS — Commuter

PKS clusters identify commercial concentrations near regional airports and
intercity train stations that sit in a Commuter belt 15–150 km from a major
metro centre. The defining demand pattern is park-and-fly or park-and-train
travel: residents of a Regional Market drive to a transit node, park, and
travel to the Metro Market.

### Definition

A PKS candidate is a regional transit node — airport or intercity train
station — at metro distance 15–150 km. Nodes within 15 km of the metro
centre are classified as suburban rather than regional; nodes beyond 150 km
are considered standalone markets with a separate metro relationship.

The defining commercial signal at a PKS location is car rental. Auto parts,
fuel stations, quick-service restaurants, and convenience stores are
secondary signals.

### Co-location signals

**Essential:**

| Signal | Rationale |
|--------|-----------|
| Regional transit anchor ≤ 3 km | Airport or intercity station with direct metro service |
| Metro isolation 15–150 km | Defines the regional relationship |
| T1 or T2 cluster ≤ 10 km | Same population generates parking demand |
| Regional population ≥ 150,000 | Minimum demand for multi-storey parking |

**Significant:**

| Signal | Rationale |
|--------|-----------|
| Car rental ≤ 1 km | Arriving travellers require transport |
| Hotel cluster ≤ 500 m | Business travel and multi-day parking |
| Second transit mode ≤ 5 km | Multi-modal integration |

**Disqualifying:** Major hub within 15 km; population under 100,000; no
direct metro service.

### Production results (11 June 2026)

The PKS pipeline is production-grade. Park-and-ride records (23,117 locations) serve
as the primary geographic anchor — actual car→transit transition points distributed
independently of rail network geometry. Transit modes are enrichment signals; car rental
and hotel presence define commercial maturity. Tier rules use mode-group collapse
(intercity_rail + commuter_rail collapse to the RAIL group, preventing artificial
bimodality inflation).

| Tier | Clusters | % | Definition |
|------|---------|---|-----------|
| T1 (Regional Hub) | 691 | 9.9% | Multi-modal + full commercial ecosystem |
| T2 (Transit Interchange) | 2,658 | 38.2% | Transit + at least one commercial signal |
| T3 (Transit Node) | 3,604 | 51.9% | Transit present; commercial opportunity |
| **Total** | **6,953** | | |

Commercial enrichment: car rental chains (hertz-eu, avis-eu, europcar-eu, sixt-eu, and
others) and hotel chains (ibis-eu, premier-inn-gb, holiday-inn-express-us, and others)
are all ingested and active in the production build.

### Major hub filter

Airports with a T1 PRO cluster within 5 km are excluded as likely major commercial hubs.
Major airports generate their own retail gravity and do not exhibit the park-and-transit
pattern. The filter correctly removes LAX, JFK, LHR, and CDG.

### Future enhancements

- Airport passenger volume data (CAPA or IATA) to replace the T1-adjacency hub proxy
  with a direct traffic-based classifier
- Parking operator directory: Q-Park, APCOA, NCP, Indigo/Vinci (EU); SP+ (US)

---

## Map integration

VWH and PKS appear as overlay layers under the **★ Regional Markets** section
in the layer control panel.

**VWH toggle** — displays orange dots at Urban Fringe candidate locations.
When active, cluster bubbles fade to 10 % opacity to reduce visual interference.

**PKS toggle** — displays teal dots at integrated candidates (T1/T2 cluster
within 10 km) and grey dots at standalone candidates. The same 10 % bubble
fade applies.

Both layers persist across view transitions — the fade state is maintained
when switching between the Retail View and the BentoBox market detail panel.

State variables: `vwhActive`, `psActive`. Functions: `toggleVwhLayer(btn)`,
`togglePsLayer(btn)`. The fade behaviour is handled by `applyLiOverlayStyle()`,
which guards against all active overlay states before applying opacity changes.
