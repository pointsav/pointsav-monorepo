# Resource Request: Storage Expansion for Project GIS Data Enrichment

**To:** Master
**From:** task@project-gis
**Subject:** Requirement for high-capacity storage volume (500GB+)

Master,

The project-gis cluster is entering the Data Enrichment & Trade Area Ranking phase. Our current workspace capacity is highly constrained (6.5GB free), which is insufficient for the demographic, economic, and mobility datasets required for our 6,815 co-location clusters.

### Resource Requirement:
*   **Requested Capacity**: 100 GB
*   **Target Mount**: /srv/foundry/data-archives/project-gis/
*   **Justification**:
    *   **Data Footprint**: Through the use of our 75km "Radius Filter" and conversion of raw GeoTIFFs to highly compressed GeoParquet, we have optimized our storage footprint. We require a 100 GB dedicated volume to house the filtered demographic (Census), economic (Spend), and mobility datasets.
    *   **Performance**: This volume will serve as the permanent archive for service-fs, ensuring high-performance spatial joins during pipeline rebuilds.
    *   **Efficiency**: We have designed the pipeline to perform "chunked" ingest, meaning we can scale the volume upward in the future if required, but 100 GB represents the current minimum viable capacity for the 6,815 cluster Trade Area analysis.

### Plan Summary:
*   We are moving from a city-centric demographic comparison to a **Cross-Cluster Competitive Ranking** model based on Primary (35km) and Secondary (75km) Trade Areas.
*   We require the new volume to be available as a permanent data archive to maintain our "Code in Monorepo, Data in Deployment" architectural commitment.

Please advise on the provisioning process for this volume mount.

task@project-gis
