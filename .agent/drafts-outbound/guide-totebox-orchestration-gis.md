---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: guide-totebox-orchestration-gis.md
audience: internal-engineering
bcsc_class: internal
language_protocol: GUIDE
notes_for_editor: |
  Route to project-language. Technical guide explaining how the app connects to the Totebox.
---

# Totebox Orchestration for GIS

This guide details the integration of the `app-orchestration-gis` application surface with the underlying Totebox Archive data layer.

## Topology

The Location Intelligence platform operates on a strict two-deployment topology to ensure data sovereignty and service resilience.

### 1. The Data Layer (`cluster-totebox-personnel-1`)
The Totebox Archive acts as the immutable source of truth. It holds all curated location data in flat-file formats (JSONL/YAML):
*   `service-business`: Retail operator locations (Walmart, Costco, etc.).
*   `service-places`: Civic infrastructure sourced from Overture Maps.
*   `service-parking`: Hand-authored geo-fence polygons.

The Totebox has no rendering capability; it simply serves files.

### 2. The Application Surface (`gateway-orchestration-gis-1`)
The gateway runs the PointSav GIS Engine. It holds no canonical data itself. Upon initialization, or during a scheduled build process, the gateway queries the Totebox Archive.

## Orchestration Workflow

1.  **Ingestion:** The gateway reads `service-business` and `service-places` via the configured `TOTEBOX_DATA_PATH`.
2.  **Processing:** The `build-clusters.py` script executes the co-location algorithm, generating the matrix rankings.
3.  **Tile Generation:** `build-radius.py` and Tippecanoe compile the results into Layer 1, Layer 2, and Layer 3 `.pmtiles` archives.
4.  **Delivery:** The resulting PMTiles and the MapLibre `index.html` are served to the client browser at `gis.woodfinegroup.com`.

If the gateway node is destroyed, the application can be re-provisioned instantly and pointed back to the intact Totebox Archive, requiring only a tile rebuild to restore full service.

## See Also
*   [PointSav GIS Engine](topic-pointsav-gis-engine.md)