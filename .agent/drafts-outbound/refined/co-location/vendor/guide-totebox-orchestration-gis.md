---
schema: foundry-doc-v1
title: "GUIDE — Totebox Orchestration for GIS"
slug: guide-totebox-orchestration-gis
category: infrastructure
type: guide
quality: complete
status: active
audience: vendor-internal
bcsc_class: internal
language_protocol: PROSE-GUIDE
last_edited: 2026-05-02
editor: pointsav-engineering
cites:
  - totebox-orchestration-convention
  - pmtiles-spec
---

# GUIDE — Totebox Orchestration for GIS

This guide details the integration and operational lifecycle of the `app-orchestration-gis` application surface with the underlying Totebox Archive data layer. It provides the runbook for maintaining the service-to-data boundary.

## 1. What it does and when it runs

The GIS platform operates on a strict two-deployment topology to ensure data sovereignty and service resilience.

### Topology
- **The Data Layer (`cluster-totebox-personnel-1`):** The Totebox Archive acts as the immutable source of truth. It stores curated location data (retail, places, and parking) in flat-file formats (JSONL/YAML/GeoParquet). The Totebox serves these files but has no rendering or spatial-join capability.
- **The Application Surface (`gateway-orchestration-gis-1`):** The gateway node runs the PointSav GIS Engine. It holds no canonical data. It pulls data from the Totebox, processes the co-location matrix, generates map tiles, and serves the browser interface at `gis.woodfinegroup.com`.

### Execution Cadence
The orchestration process is triggered under three conditions:
- **Initialization:** On node boot or container start.
- **Scheduled Build:** Daily at 01:00 UTC (automated via systemd timer).
- **Manual Trigger:** Initiated by an operator after a major `service-business` dataset update.

## 2. Operation and Configuration

The orchestration workflow is managed via the `build-clusters.py` and `build-radius.py` scripts located in the application root.

### Orchestration Workflow
1.  **Ingestion:** The gateway reads retail and civic infrastructure files via the path defined in the `TOTEBOX_DATA_PATH` environment variable.
2.  **Processing:** `build-clusters.py` executes the co-location algorithm, generating the 12-rank matrix rankings.
3.  **Tile Generation:** `build-radius.py` utilizes Tippecanoe to compile the results into Layer 1 (POI), Layer 2 (Clusters), and Layer 3 (Background) `.pmtiles` archives.
4.  **Delivery:** The PMTiles and the MapLibre `index.html` are served to the client browser via Nginx.

### Environment Variables
| Variable | Purpose |
|----------|---------|
| `TOTEBOX_DATA_PATH` | Path to the mounted Totebox Archive filesystem. |
| `OUTPUT_PATH` | Destination for the generated `.pmtiles` archives. |
| `SECONDARY_RADIUS` | Default proximity radius (1.0, 2.0, or 3.0 km). |

## 3. Failure Recovery

### Connectivity Failure (Totebox Unreachable)
**Symptoms:** `build-clusters.py` exits with code 1; "Error: TOTEBOX_DATA_PATH not found" in logs.
**Recovery:** Verify the Totebox network mount (NFS/SMB). Restart the mount service and re-run the build script.

### Build Failure (Data Corruption)
**Symptoms:** `build-clusters.py` exits with code 2; "JSONL Parse Error" in logs.
**Recovery:** Identify the corrupted file in the Totebox Archive. Revert the file to the previous version in the WORM ledger and trigger a re-build.

### Rendering Failure (Empty Map)
**Symptoms:** PMTiles generated successfully but browser shows no data.
**Recovery:** Check the `index.html` configuration to ensure it points to the correct PMTiles URL. Verify Nginx `application/octet-stream` headers for `.pmtiles` files.

## See Also
*   [[pointsav-gis-engine]]
*   [Totebox Orchestration Convention](totebox-orchestration-convention)

---
## Provenance
- **Draft Source:** `GUIDE-totebox-orchestration-gis.md` (project-gis)
- **Refinement:** 2026-05-02 by project-language Task
- **Verification:** Orchestration steps verified against `gateway-orchestration-gis-1` deployment configuration.
