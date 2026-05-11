---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-pointsav-gis-engine.md
audience: public
bcsc_class: public-disclosure-safe
language_protocol: PROSE-TOPIC
notes_for_editor: |
  Route to project-language. This is the technical counterpart to the Woodfine Wiki files.
  Tone should be engineering-focused and authoritative.
---

# PointSav GIS Engine

The PointSav GIS Engine is a high-performance, sovereign Location Intelligence platform. Built in Rust and designed for offline-first, flat-file operation, it represents a departure from traditional, proprietary geographic database systems (such as PostGIS or Esri).

## Architectural Substrate

The engine is engineered to operate within the broader Woodfine Operating System, adhering to the principle of complete data sovereignty. 

### Flat-File Architecture
Traditional GIS systems rely on heavy, running database instances. The PointSav engine utilizes a flat-file substrate, consuming data directly from `JSONL` and `GeoParquet` formats. This architecture ensures that the data layer remains entirely decoupled from the rendering layer, preventing vendor lock-in and eliminating complex database maintenance.

### The Rendering Stack
The platform avoids commercial SaaS dependencies by utilizing open-source rendering components:
*   **PMTiles:** A single-file archive format for tiled data, allowing the map to be served directly from blob storage or standard web servers (Nginx) without a dedicated tile server.
*   **MapLibre GL JS:** An open-source, WebGL-based library for rendering interactive maps in the browser.
*   **Tippecanoe:** Used to compile the flat-file datasets into highly optimized vector tiles for rapid browser delivery.

## Algorithmic Processing

The core of the GIS Engine is the `app-orchestration-gis` service. This component is responsible for executing the [Retail Co-location Methodology] deterministically. It ingests thousands of retail locations, applies the defined spatial radii (1.0km, 3.0km, 5.0km), and generates the 12-rank scoring matrix that powers the visual interface at [gis.woodfinegroup.com](https://gis.woodfinegroup.com).

## See Also
*   [Totebox Orchestration for GIS](guide-totebox-orchestration-gis.md)