---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
audience: general
bcsc_class: no-disclosure-implication
version: "1.0"
date: 2026-05-30
title: "Krefeld — Regional Market"
eu_rank: 5
rm_type: suburban-regional
suburb_of: Düsseldorf
dist_km: 19.4
wikipedia_city: https://en.wikipedia.org/wiki/Krefeld
routes_to: project-editorial
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: Wikipedia API (2026-05-30); score-regional-markets.py corrected output (commit 39aa1b11); clusters-meta.json Phase 23+Change B
research_inline: false
---

# Krefeld — Regional Market

Krefeld ranks 5th among the 400 European Regional Markets identified by the
co-location methodology. The market is suburban-regional in character, situated
19.4 km northwest of Düsseldorf within the Rhine-Ruhr polycentric metropolitan
area. Two Tier 1 co-locations anchor the retail geography, each combining
hypermarket, hardware, and consumer-electronics formats. Civic infrastructure
is substantial: the Helios hospital network operates two campuses in the city
alongside the independent Catholic Krankenhaus Maria-Hilf, and Hochschule
Niederrhein — a University of Applied Sciences — maintains two Krefeld
campuses with approximately 11,000 students enrolled. Krefeld is an
independent city (kreisfreie Stadt) in the state of North Rhine-Westphalia.

## Overview

According to Wikipedia (accessed 2026-05-30), Krefeld had a population of
231,406 as of 31 December 2024. The city is administratively independent
under North Rhine-Westphalia's two-tier structure — a kreisfreie Stadt rather
than a constituent of a surrounding rural district — and sits on the left
bank of the Rhine northwest of Düsseldorf. The borough of Uerdingen, in the
city's east, lies directly on the Rhine. Major motorway connections include
the A57 and A44, which integrate Krefeld into the wider Rhine-Ruhr corridor.

Krefeld's historical economic identity is built on textiles. By 1763 the Von
der Leyen silk merchant families employed roughly half of Krefeld's then
6,082 residents in their workshops, establishing a textile tradition that
gave the city its enduring nickname, the "Velvet and Silk City." Industrial
diversification followed: the Nirosta steelworks — historically associated
with ThyssenKrupp and Outokumpu-owned since 2012 — remains a material
employer, and corporate offices for Hitachi, Canon, Evonik Industries, and
Fressnapf are located in the city. The municipal population peaked at
244,020 in 1990 and has since stabilised near 230,000.

Geographically, Krefeld functions as a component municipality of the
Rhine-Ruhr metropolitan region — a polycentric agglomeration of more than
ten million inhabitants spanning Düsseldorf, Cologne, Essen, Dortmund, and
their neighbours. The market is therefore measured against the centroid of
Düsseldorf at a distance of 19.4 km, classifying it as suburban-regional
rather than free-standing.

## Co-location Profile

Krefeld contains two Tier 1 co-locations. Both combine the three German
big-box retail formats — hypermarket, hardware, and consumer electronics —
producing the highest composition score available in the European dataset.

**Table 1.** Krefeld co-locations.

| Cluster | Tier | Anchor Composition | Representative Members |
|---|---|---|---|
| Krefeld-1 | T1 | Hypermarket + Hardware + Electronics | Kaufland, Hornbach, Bauhaus, MediaMarkt, Helios Klinikum Krefeld, Helios Cäcilien-Hospital Hüls, Hochschule Niederrhein (Krefeld-West), Hochschule Niederrhein (Krefeld-Süd), Krankenhaus Maria-Hilf |
| Krefeld-2 | T1 | Hypermarket + Hardware + Electronics | Globus, Bauhaus, Saturn, Helios Klinikum Krefeld, Hochschule Niederrhein (Krefeld-West), Hochschule Niederrhein (Krefeld-Süd), Krankenhaus Maria-Hilf |

The German Tier 1 anchor profile differs in brand identity from its North
American equivalent but mirrors it functionally. Hornbach and Bauhaus
occupy the hardware role held by Home Depot and Lowe's in the United States;
Kaufland (a Schwarz Group hypermarket, the same parent that operates Lidl)
and Globus (a German family-owned hypermarket chain) replace Walmart and
Target. In consumer electronics, MediaMarkt and Saturn — both operated by
Ceconomy AG under separate banners — substitute for Best Buy. The presence
of MediaMarkt in one cluster and Saturn in the other is characteristic of
the Rhine-Ruhr retail-park pattern: Ceconomy frequently positions its two
brands in the same catchment area but in physically distinct retail parks,
treating them as complementary rather than competing tenancies.

Bauhaus appears in both clusters, reflecting the chain's dual-store
configuration across the city. The Helios Klinikum Krefeld and the two
Hochschule Niederrhein campuses likewise appear in both clusters: they fall
within both retail-park catchment radii under the spatial model.

## Civic Infrastructure

Krefeld's civic anchor profile is dense for a city of its size. The Helios
network — Germany's largest private hospital operator — runs two campuses in
the city. Helios Klinikum Krefeld is the principal tertiary facility and the
city's largest medical employer. Helios Cäcilien-Hospital Hüls operates as a
secondary campus in the northern Hüls borough. Alongside the Helios
facilities, Krankenhaus Maria-Hilf — a Catholic hospital under independent
ownership — serves the western catchment of the city. The three hospitals
collectively give Krefeld redundant secondary and tertiary acute-care
coverage uncommon among European cities in the 200,000–250,000 population
band.

Higher education is anchored by Hochschule Niederrhein, a University of
Applied Sciences with approximately 11,000 students enrolled across its
Krefeld facilities. The institution maintains two physically separated
campuses in the city — Krefeld-West and Krefeld-Süd — and a further campus
in neighbouring Mönchengladbach. The geographic separation of the West and
Süd campuses places one or both within the catchment radius of each Tier 1
retail cluster, which is why both campuses appear as members of both
clusters in the data.

The combined hospital and university footprint produces the 1.5× civic
multiplier applied to Krefeld's composite score.

## AEC Data

**Table 2.** Architecture, Engineering, and Construction climate parameters.

| Parameter | Value |
|---|---|
| Köppen-Geiger Climate | Cfb (Oceanic / Marine West Coast) |
| EU Regulatory Climate Zone | II (Atlantic) |
| WWF Ecoregion | European Atlantic mixed forests |
| WWF Biome | Temperate Broadleaf & Mixed Forests |

Cfb is the dominant climate regime across the Rhine-Ruhr region: mild
summers, cool but rarely severe winters, and rainfall distributed across
the year. For building design, this implies moderate heating demand and
minimal mechanical cooling demand relative to continental European zones
further east. The Atlantic-Oceanic regime contrasts with the Dfb continental
profile that prevails in Bavaria and the eastern German states.

## Composite Score

**Table 3.** Composite score derivation for Krefeld.

| Component | Value | Notes |
|---|---|---|
| Tier score | 8 | (2 × 4) + (0 × 2) + (0 × 1) |
| Civic multiplier | 1.5 | Medical and academic anchors present |
| Confidence factor | 1.0 | High confidence |
| **Composite score** | **12.0** | tier_score × civic_multiplier × confidence_factor |
| Regional Market type | suburban-regional | 19.4 km from Düsseldorf metro centroid |

A composite score of 12.0 is representative of the upper European
suburban-regional tier. The European dataset exhibits lower maximum scores
than the North American dataset because European municipalities are
geographically smaller units, so even economically significant centres tend
to span fewer Tier 1 compositions than their suburban North American
counterparts. The composite score formula does not apply a metro-distance
multiplier; suburban-regional classification is recorded as a categorical
descriptor rather than a scoring penalty.
