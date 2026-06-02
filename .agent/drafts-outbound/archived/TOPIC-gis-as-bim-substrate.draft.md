---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC-*
title: "GIS as BIM Substrate"
slug: topic-gis-as-bim-substrate
status: staged
destination: media-knowledge-documentation
bcsc_class: no-disclosure-implication
paired_with: TOPIC-gis-as-bim-substrate.es.draft.md
research_done_count: 4
research_suggested_count: 2
open_questions_count: 0
research_provenance: bim-product-family.md (.agent/rules/); AEC build pipeline documentation (build-aec-*.sh); clusters-meta.json schema; DOCTRINE.md §IV (BIM domain rationale)
research_inline: true
created: 2026-05-31
---

# GIS as BIM Substrate

Geographic Information System analysis and Building Information Modelling serve
different but complementary purposes in the planning and operation of built
environments. GIS identifies where activity is concentrated and what environmental
conditions shape a location. BIM models how a building at that location is
designed, constructed, and maintained. This article describes how GIS co-location
data is intended to serve as a substrate layer that informs BIM workflows —
supplying site context that the BIM product family is planned to consume.

## Two Distinct Disciplines

GIS and BIM are different in scale, purpose, and data type. GIS operates at the
scale of cities, regions, and continents. It answers questions about the
distribution of activity, demographic patterns, economic flows, and environmental
conditions across geography. The co-location platform is a GIS application: it
analyses the spatial distribution of retail anchors at a continental scale to
identify clusters of co-located activity that indicate market strength.

BIM operates at the scale of a single building or campus. It models the physical
geometry of a structure, its mechanical and electrical systems, its material
specifications, and its maintenance history. BIM is concerned with what is inside
a specific parcel boundary — its geometry, its components, its lifecycle.

These scales rarely overlap in practice. A GIS analysis identifies a site as
commercially significant; BIM models what is built on that site. The substrate
relationship is one-directional: GIS data flows into BIM as context, not the
reverse. A BIM model of a particular building does not change the co-location
data for the cluster in which that building sits.

## The AEC Data Layer Connection

The platform's Architectural, Engineering, and Construction (AEC) data layers
form the bridge between GIS analysis and BIM site context. Each co-location
cluster carries a set of AEC attributes derived from the nightly AEC build
pipeline:

*Climate zone (ASHRAE 169 / NECB / EU).* The applicable heating and cooling
design standard for the cluster's location. This is the single most consequential
environmental parameter for building mechanical system design. An ASHRAE Climate
Zone 3A (Warm-Humid) site requires fundamentally different HVAC specifications
than a Zone 6A (Cold-Humid) site.

*Köppen climate classification.* The long-term climate type at the cluster's
location, drawn from the Beck et al. global 1 km resolution dataset. Köppen
classification informs passive design strategies: orientation, glazing ratios,
natural ventilation potential, and shading requirements.

*Seismic hazard.* Peak ground acceleration values derived from the USGS National
Seismic Hazard Model (North America), Natural Resources Canada seismic maps,
and the European Seismic Hazard Model (ESHM20). Seismic parameters are required
inputs for structural engineering design across all building typologies.

*Flood risk.* Flood hazard categorisation derived from WRI Aqueduct riverine
flood data, FEMA National Flood Hazard Layer, and EU INSPIRE flood zone datasets.
Flood zone classification affects foundation design, ground-floor elevation
requirements, and insurance obligations.

These four AEC layers are computed at each cluster's centroid and attached to the
cluster's metadata record. A planner examining a T1 Regional cluster can read
off the environmental design parameters for that site directly from the cluster
record, without performing a separate spatial query against each environmental
dataset.

## Intended Data Flow

The intended relationship between GIS cluster data and the BIM product family
is as follows. A market analyst uses the co-location platform to identify a T1
Regional cluster in a target geography. The cluster record provides the cluster's
centroid coordinates, its co-location composition (which anchor categories are
present), its tier (T1/T2/T3), and its AEC attributes. This record constitutes
a *site environmental brief* — a structured summary of the market and
environmental conditions at that location.

The BIM product family is planned to consume the site environmental brief as a
starting-point input for site analysis and design. The `service-bim` archive
daemon is intended to read cluster metadata records as one of its input data
sources. The `app-workplace-bim` editor is planned to surface AEC attributes in
a site conditions panel, allowing a designer to see the applicable climate zone
and seismic parameters alongside the BIM model of the building under design.

This integration is planned and intended. The data substrate — cluster metadata
with AEC attributes — is built and live. The BIM product family components that
consume it are in active development.

## The Data Contract

The flat-file archive structure used by the BIM product family defines how
cluster data enters the BIM workflow. Cluster metadata from the co-location
platform is not read directly from GIS tiles. It is read from structured
metadata records stored in the deployment's data directory. This design keeps
GIS and BIM decoupled at the storage layer: changes to the GIS pipeline do not
require changes to the BIM archive format, and vice versa.

The AEC fields in cluster metadata — `ashrae_zone`, `koppen_class`,
`seismic_hazard`, `flood_risk` — are the defined interface between GIS and BIM.
They represent a translation of raw raster data (each AEC layer is a global
raster file) into discrete categorical values that a BIM system can interpret
without needing to perform raster spatial queries.

## Scope Note

This article describes a planned substrate integration. The co-location platform
produces the cluster metadata with AEC attributes; the BIM product family is
intended to consume it. End-to-end integration between a live co-location cluster
record and an active BIM model is not yet implemented. The AEC data layers are
produced and attached to cluster records; the BIM-side consumption of those
fields is in development.

---

*Related:* `bim-product-family.md` — product family map, data contract, and
IFC format decisions for the BIM product family.
