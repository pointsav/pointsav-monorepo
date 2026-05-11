---
schema: foundry-doc-v1
title: "Location Intelligence Platform"
slug: location-intelligence-platform
category: applications
type: topic
quality: complete
status: active
audience: public
bcsc_class: public-disclosure-safe
language_protocol: PROSE-TOPIC
last_edited: 2026-05-02
editor: pointsav-engineering
paired_with: location-intelligence-platform.es.md
cites:
  - osm-odbl
  - overture-maps-cdla-2-0
  - ni-51-102
  - osc-sn-51-721
---

# Location Intelligence Platform

The PointSav Location Intelligence platform is a sovereign, flat-file GIS application designed for retail cluster analysis and strategic site selection. Built as a high-performance alternative to commercial SaaS products, the platform enables customers to maintain complete control over their geographic datasets, algorithms, and rendering infrastructure.

## Operational Capabilities

The platform transforms raw store locations into actionable commercial nodes by executing the [Retail Co-location Methodology](co-location-methodology). It answers a fundamental commercial question: *which geographic nodes possess the capital-validated density required to support adjacent development?*

### 1. Five-Degree Cluster Identification
The platform computes co-location clusters around Primary Target anchors (e.g., Walmart Supercentres) using a deterministic spatial algorithm. Each cluster is scored based on the convergence of independent, capital-intensive operators (Costco, Home Depot, etc.) and supporting civic infrastructure (hospitals, universities).

### 2. Multi-Layer Interactive Interface
The interactive map at [gis.woodfinegroup.com](https://gis.woodfinegroup.com) utilizes a three-layer architecture:
-   **Layer 1 — Global POIs:** Toggled view of 31,000+ individual retail locations, color-coded by brand family.
-   **Layer 2 — Co-location Clusters:** The primary analytical view, encoding cluster strength through visual saturation and size. 
-   **Layer 3 — Catchment Radii:** Visualized proximity boundaries (default 3.0 km) that define the scope for trade-area analysis and mobility data procurement.

## Sovereign Architecture

The platform adheres to the [PointSav GIS Engine](pointsav-gis-engine) principles of data sovereignty:
-   **Flat-File Operation:** All data persists as versioned JSONL and GeoParquet files within a Totebox Archive, rather than a running database daemon.
-   **Open Standards Rendering:** Utilizes PMTiles and MapLibre GL JS to serve vector maps directly from standard web servers, eliminating proprietary tile-API dependencies.
-   **Reproducible Build:** If a gateway node is destroyed, the application surface can be re-provisioned instantly by pointing a fresh instance at the immutable data layer.

## Data Foundations and Licensing

The platform integrates high-fidelity open data sources to ensure transparency and auditability:
-   **Retail Data:** Sourced from OpenStreetMap contributors and the Overture Maps Foundation.
-   **Civic Infrastructure:** Healthcare and institutional records from the Overture Maps Foundation Places dataset.
-   **Sovereign Basemap:** OpenFreeMap liberty tiles served via the PointSav infrastructure.

*Material assumptions for current platform performance include the continued availability of high-fidelity open geographic datasets. [osm-odbl] [overture-maps-cdla-2-0]*

## Future Roadmap

Planned enhancements to the platform surface include the integration of origin-destination (OD) mobility data for trade-area flow analysis and the expansion of the European institutional dataset. [ni-51-102] [osc-sn-51-721]

---
## Provenance
- **Draft Source:** `PROSE-TOPIC-location-intelligence-platform.draft.md` (project-gis)
- **Refinement:** 2026-05-02 by project-language Task
- **Verification:** Layer architecture and algorithmic composition confirmed against `app-orchestration-gis` deployment.
- **BCSC Posture:** OD data integration and European expansion targets labeled per ni-51-102.
