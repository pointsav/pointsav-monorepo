# Plan: Comprehensive Data Manifest, Legal Compliance, and Service Ingest

## 1. Objective
Establish a transparent, compliant, and optimized infrastructure for demographics, spend, and mobility data. This plan covers the legal disclosures for `gis.woodfinegroup.com`, the creation of new data services, and the spatial optimization of the data ingest.

## 2. Legal Manifest & Compliance Outline
We will maintain a central markdown file (`DATA-MANIFEST.md`) within the project to track sources. This content will be mirrored in a UI modal on the website.

### Data Credits (UI "Button")
*   **WorldPop**: "Source: WorldPop (www.worldpop.org). High-resolution population grids licensed under CC BY 4.0."
*   **OpenStreetMap**: "© OpenStreetMap contributors. Licensed under ODbL."
*   **Overture Maps**: "© Overture Maps Foundation. Licensed under CDLA-Permissive-2.0 / ODbL."
*   **Statistics Canada**: "Adapted from Statistics Canada, Census 2021. This product does not constitute an endorsement by Statistics Canada."
*   **INEGI (Mexico)**: "Source: INEGI. Geostatistical framework and DENUE data provided under the Free Use of Information terms."
*   **Eurostat**: "© European Union, 1995-2026. Eurostat demographics and Household Budget Survey data."
*   **WorldMove**: "WorldMove Synthetic Mobility Dataset (Tsinghua FIB-LAB). CC BY 4.0."

### Disclaimer (UI "Button")
*   **Content**: "The data presented on Woodfine Location Intelligence is provided 'as is' for informational purposes. While we strive for accuracy using high-fidelity open sources (OSM, Overture, WorldPop), Woodfine Group does not guarantee the precision of specific retailer coordinates or synthesized spend metrics. Not for navigation or critical infrastructure planning."
*   **Privacy**: "No individual-level tracking data is used. Mobility metrics are derived from synthetic trajectories (WorldMove) or anonymized POI visitor patterns."

## 3. New Infrastructure (Services)
We will follow the established "Code in Monorepo, Data in Deployment" pattern:

### A. Code Placeholders (Monorepo)
Create the following directories within `pointsav-monorepo/` (tracked in git):
*   `service-census/`: Contains `README.md` defining the demographic schema and ingest logic.
*   `service-spend/`: Contains `README.md` defining the expenditure calculation methodology.
*   `service-mobility/`: Contains `README.md` defining the movement index and accessibility metrics.
*   *Note: These folders will NOT contain raw data.*

### B. Data Archives (Deployment)
The actual data (GeoParquet/JSONL) will be stored on the `cluster-totebox-personnel-1` deployment at the following paths:
*   `/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/`
*   `/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-spend/`
*   `/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-mobility/`
*   *Note: This data is ignored by git and managed as a live deployment artifact.*

## 4. Optimized Ingest Strategy (The "Radius Filter")
To avoid overwhelming the system with global-scale 100m grids, we will only ingest data for areas that fall within the **75km Data Radius** of our 6,815 clusters.

### Implementation:
1.  **Grid Substrate**: Use H3 (Resolution 7) or 1km UTM grid cells.
2.  **AOI (Area of Interest)**: Calculate the union of all 75km buffers around the 6,815 co-location clusters.
3.  **Spatial Join**: During ingest, only write records (Parquet/JSONL) for grid cells that intersect the AOI.
4.  **Benefits**: Reduces disk usage by ~80% while ensuring 100% coverage for the clusters displayed on the map.

## 5. European Data Parity
*   **Demographics**: Eurostat provides 1km population grids (GEOSTAT) which are more precise than WorldPop for the EU. We will prioritize GEOSTAT for Europe.
*   **Spend**: Eurostat Household Budget Survey (HBS) provides NUTS-2/3 expenditure levels. We will use these to populate the 1km grid.
*   **Verification**: Audit a sample cluster in Paris (Carrefour anchor) vs. Edmonton (Walmart anchor) to ensure metric comparability.

## 6. Implementation Steps
1.  **Draft Legal Modal**: Add the 'Data Credits' and 'Disclaimer' buttons to `index.html`.
2.  **Create Manifest**: Author `DATA-MANIFEST.md` in the project root with the credit strings from Section 2.
3.  **Editorial Routing**: Stage `LICENSE-DATA-MANIFEST.draft.md` and `LICENSE-DISCLAIMER.draft.md` in `.agent/drafts-outbound/` for `project-editorial` approval.
4.  **Setup Folders (Monorepo)**: Create `service-census`, `service-spend`, and `service-mobility` in `pointsav-monorepo/` with descriptive `README.md` files.
5.  **Setup Folders (Deployment)**: Create the data archive paths on `cluster-totebox-personnel-1`.
6.  **H3 Mapping**: Generate a list of "Active H3 Cells" covering the 6,815 cluster radii.
7.  **Batch Ingest**: Run the ingest scripts for Census, Spend, and Mobility using the cell filter.
8.  **Rebuild Pipeline**: Update `generate-rankings.py` to pull from these new services.
