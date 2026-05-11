---
schema: foundry-v1
state: draft
audience: software-engineers
language_protocol: TECH-TOPIC
---

# Application: app-orchestration-gis (The Spatial Engine)

`app-orchestration-gis` is the stateless analytical engine of the PointSav platform. It performs the linear-geometry calculations and coordinate mapping required to generate the Woodfine co-location rankings and the organic map visualization.

## Linear Geometry Logic

Unlike legacy systems that use rigid distance buffers, `app-orchestration-gis` implements a 100% linear geometric decay model using the Haversine formula.

### The Scoring Algorithm
For every Alpha Anchor in the cleansed data layers, the engine calculates:
*   **Secondary Proximity:** $Score = Max(0, 100 * (3.0 - Distance_{km}) / 3.0)$
*   **Tertiary Proximity:** $Score = Max(0, 100 * (5.0 - Distance_{km}) / 5.0)$

The result is a continuous **Co-location Score** (0 to 400).

## Tile Building & Mapping
The engine is also responsible for generating the organic map assets for [gis.woodfinegroup.com](https://gis.woodfinegroup.com).
*   **Vector Tiles:** Using the Protomaps/PMTiles format for fast, client-side rendering.
*   **Combination Tiers:** Mapping spatial convergence (Alpha + HW + WH + Civic) to the 4-tier visual system on the map.

## Stateless Design
The application resides in the `/gateway-orchestration-gis-1/` deployment. It holds no canonical data; it is a "pure function" that takes cleansed clusters as input and produces ranking bubbles and geo-tiles as output.

---
*Back to [Software Architecture Overview](index.md)*
