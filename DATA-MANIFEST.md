# Project GIS — Data Manifest & Licensing

This document serves as the master record of all data sources, methodologies, and licensing agreements used within the Woodfine Location Intelligence platform.

## 1. Global Data Layers

### OpenStreetMap (OSM)
*   **Purpose**: Base map data, Point-of-Interest (POI) locations for retail and civic infrastructure.
*   **License**: [Open Database License (ODbL) 1.0](https://opendatacommons.org/licenses/odbl/).
*   **Attribution**: © OpenStreetMap contributors.

### Overture Maps Foundation
*   **Purpose**: Global POI theme (Places), Transportation network, and Building footprints.
*   **License**: [CDLA-Permissive-2.0](https://cdla.dev/permissive-2-0/) (Places) and [ODbL](https://opendatacommons.org/licenses/odbl/) (Transportation/Buildings).
*   **Attribution**: © Overture Maps Foundation.

### WorldPop
*   **Purpose**: High-resolution (100m) gridded population data for global demographic analysis.
*   **License**: [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).
*   **Attribution**: WorldPop (www.worldpop.org).

### WorldMove
*   **Purpose**: Synthetic human mobility trajectories for city-scale flow analysis.
*   **License**: [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).
*   **Attribution**: Yuan et al. (2025). "WorldMove: A Global Open Data for Human Mobility".

## 2. Regional & Administrative Data

### Statistics Canada (StatCan)
*   **Purpose**: Canadian Census 2021 demographics and Household Spending surveys.
*   **License**: [Statistics Canada Open Licence](https://www.statcan.gc.ca/en/reference/licence).
*   **Attribution**: Adapted from Statistics Canada data. This does not constitute an endorsement by Statistics Canada.

### INEGI (Mexico)
*   **Purpose**: Mexican geostatistical framework and DENUE business registry.
*   **License**: [Terms of Free Use of Information (INEGI)](https://www.inegi.org.mx/servicios/api_indicadores.html).
*   **Attribution**: Source: INEGI.

### Eurostat (European Union)
*   **Purpose**: GEOSTAT population grids (1km) and Household Budget Survey (HBS) data for EU member states.
*   **License**: [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://ec.europa.eu/eurostat/about-us/policies/copyright).
*   **Attribution**: © European Union, 1995-2026.

## 3. Disclaimers & Terms of Use

The Woodfine Location Intelligence platform (`gis.woodfinegroup.com`) provides synthesized location intelligence metrics "as is." 

1.  **Metric Synthesis**: Scores, ranks, and spend estimates are the result of computational modeling using the open-source inputs listed above.
2.  **Accuracy**: While we strive for high fidelity by combining multiple authoritative sources, Woodfine Group does not guarantee the precision of specific retailer coordinates or synthesized economic metrics.
3.  **Privacy**: No individual-level tracking or personally identifiable information (PII) is utilized. All mobility data is synthetic or aggregated.
4.  **Non-Navigation**: This data is not intended for real-time navigation or critical infrastructure planning.
