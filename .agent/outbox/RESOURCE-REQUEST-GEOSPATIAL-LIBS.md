# Resource Request: Geospatial Library Dependencies (GDAL/Rasterio)

**To:** Master
**From:** task@project-gis
**Subject:** Requirement for Geospatial Python Dependencies (GDAL/Rasterio)

Master,

The `project-gis` data enrichment pipeline is currently blocked due to missing geospatial library dependencies. We have successfully provisioned the WorldPop population grids (GeoTIFF format) for North America and Europe, but our ingestion environment lacks the tools to parse these raster datasets.

### Required Dependencies:
*   **Target Libraries**: `python3-rasterio`, `python3-gdal` (or equivalent system-level GDAL binaries).
*   **Justification**:
    *   **Raster Processing**: We are currently ingesting high-resolution population grids (100m/1km) from WorldPop and Eurostat. Without `rasterio` and `gdal`, we cannot extract these gridded demographic values into our target service format (GeoParquet/JSONL).
    *   **Pipeline Integrity**: The `spatial_filter.py` and census ingestion logic relies on standard geospatial operations which are standard for our data enrichment strategy.
    *   **Efficiency**: Using native GDAL/Rasterio is the most performance-optimized way to perform the required spatial joins and gridded summations for our Trade Area analysis.

### Current Status:
*   Raw WorldPop grids are already provisioned and stored at `/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/raw/`.
*   We have verified our spatial filtering logic (O-D Study) on mock data, but need these dependencies to process the actual high-fidelity global inputs.

Please advise on how to proceed with the installation of these geospatial dependencies within our workspace.

task@project-gis
---
EOF
