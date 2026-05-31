---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC-*
title: "Regional Name Resolution Architecture"
slug: topic-regional-name-resolution
status: staged
destination: media-knowledge-documentation
bcsc_class: no-disclosure-implication
paired_with: TOPIC-regional-name-resolution.es.draft.md
research_done_count: 4
research_suggested_count: 1
open_questions_count: 0
research_provenance: pipeline source files (config.py, build-clusters.py, ca_places_nominatim.json); TIGER 2023 documentation; GISCO LAU 2021 documentation; OSM Nominatim API documentation
research_inline: true
created: 2026-05-31
---

# Regional Name Resolution Architecture

Co-location clusters are identified by geometry — a set of latitude/longitude
coordinates derived from OpenStreetMap point-of-interest records. Geometry does
not have a name. Giving a cluster a useful, human-recognisable name requires a
separate resolution step that matches the cluster's centroid against authoritative
place-name datasets. This article describes how that resolution works, why it is
necessary, and where it can fail.

## The Problem with Administrative Boundaries

OpenStreetMap and Wikidata organise geography into administrative hierarchies:
country, region, county, municipality. These hierarchies are legally and
politically defined. They do not always correspond to the names that residents,
businesses, and market researchers use to describe a place.

Consider a cluster of retail co-locations in the community of Sherwood Park,
Alberta. Sherwood Park is an unincorporated community within Strathcona County.
Its OSM administrative boundary is the county — *Strathcona County* — not the
community. An algorithm that resolves cluster names solely from administrative
boundaries would label this cluster "Strathcona County," a name that conveys
almost nothing to a researcher studying suburban retail patterns in the Edmonton
metropolitan area. The name "Sherwood Park" is what the community, its retailers,
and its residents use. It is what a Regional Market TOPIC article should be titled.

This disconnect between legal administrative geography and colloquial place names
is not an edge case. It appears wherever unincorporated communities, census
subdivisions, and historical town names persist alongside newer county or borough
structures. The resolution architecture exists to bridge that gap.

## Boundary Datasets

Four datasets supply place-name candidates, each covering a different part of
the geographic scope of the platform.

**TIGER 2023 (United States).** The US Census Bureau's Topologically Integrated
Geographic Encoding and Referencing (TIGER) dataset provides place boundaries for
the United States. The 2023 vintage includes approximately 32,000 named places:
incorporated cities and towns, census-designated places (CDPs), and some
unincorporated communities with recognised names. TIGER places are the primary
resolution source for all US clusters.

**GISCO LAU 2021 (European Union and associated countries).** The European
Commission's Geographic Information Services for the Commission of the EU
(GISCO) publishes Local Administrative Unit (LAU) boundaries derived from
NUTS (Nomenclature of Territorial Units for Statistics). The 2021 vintage covers
approximately 98,600 municipalities across EU member states and neighbouring
countries participating in the Eurostat framework. LAU boundaries are the primary
resolution source for EU clusters in Germany, France, Spain, Italy, Poland, the
Netherlands, Austria, Portugal, Greece, Sweden, Denmark, Finland, and Norway.

**GADM GBR (United Kingdom).** The Global Administrative Areas (GADM) database
provides sub-national boundary data for countries not covered by GISCO. For the
United Kingdom, GADM provides administrative level 3 boundaries (parishes and
wards in England; communities in Wales; civil parishes in Scotland). These
provide finer-grained name candidates than the LAU-equivalent level 2 districts.

**Nominatim overrides (Canada).** Canada presents a particular challenge because
census subdivisions (CSDs) — the standard administrative unit — sometimes cover
large geographic areas that contain multiple distinct communities with different
names. Twelve manual override entries in `ca_places_nominatim.json` provide
canonical place names for cases where the CSD name would be misleading. Sherwood
Park (Strathcona County CSD) is one of these twelve overrides.

## Resolution Logic

For each cluster centroid, the resolution algorithm proceeds as follows:

*Name match.* The algorithm first checks whether the cluster's constituent
retail locations carry a consistent `addr:city` or `addr:suburb` tag in OSM.
If a majority of member records agree on a place name, that name is taken as a
candidate without consulting boundary datasets.

*Boundary containment.* If no OSM tag consensus exists, the centroid is tested
for containment against the applicable boundary dataset. The smallest-area
polygon that contains the centroid is selected. Its name field becomes the
resolution candidate.

*Administrative level fallback.* If no polygon at the preferred administrative
level contains the centroid — which can occur near coast lines, in disputed
areas, or for clusters near the edge of dataset coverage — the algorithm steps
up to the next administrative level and repeats the containment test.

*Override application.* After the initial candidate is identified, the algorithm
checks the candidate name against the override list. For Canada, if the resolved
CSD name matches one of the twelve known problematic names, the override supplies
the correct colloquial name.

## Why Canonical Names Matter

The resolved name is not merely a display label. It is the primary identifier
used in the Regional Markets scoring system. A cluster's resolved name determines
which metro-distance calculation applies to it: the scoring system looks up the
canonical metro reference list using the resolved name to determine whether a
cluster belongs to a metro core, a suburban ring, or a standalone secondary
market. An incorrect resolution — labelling Sherwood Park as Strathcona County,
for instance — would cause the cluster to receive the wrong metro-distance
calculation and potentially be misclassified.

The resolved name also becomes the title of any Regional Market TOPIC article
written for that cluster. Correctness here is a matter of editorial integrity:
an article titled "Strathcona County" about a retail cluster in Sherwood Park
would be factually misleading.

## Known Limitations

The current resolution architecture relies on boundary datasets with fixed
vintages (TIGER 2023, GISCO LAU 2021). Names that have changed since those
vintages — due to incorporation, annexation, or renaming — will not be reflected
until the boundary data is refreshed. Similarly, newly established communities
that postdate the boundary datasets will fall back to administrative-level
resolution, which may produce less specific names.

The twelve Canadian override entries represent the cases identified during
the Phase 14 and Phase 15 build cycles. Other CSD/community name mismatches
may exist in areas not yet covered by the platform.

---

*Data provenance:* TIGER 2023 (US Census Bureau, public domain); GISCO LAU 2021
(Eurostat/EC, CC BY 4.0); GADM GBR (GADM v4.1, non-commercial research licence);
Nominatim overrides (original, project-gis). OSM data CC0.
