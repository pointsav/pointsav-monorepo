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

## 3. Co-Location Tier Methodology

### Anchor Taxonomy (V3, May 2026)

The co-location engine classifies stores into four alpha anchor classes. Stores in these classes can initiate clusters; stores in other categories can join clusters but cannot initiate them.

| Class | Representative chains |
|---|---|
| **ALPHA_HYPERMARKET** | Walmart (US/CA/MX), Target (US), Soriana (MX), Mercadona (ES), Tesco (UK), Sainsbury's (UK), Bilka (DK), K-Citymarket (FI), Prisma (FI), Obs Coop (NO), Hagkaup (IS) |
| **ALPHA_LIFESTYLE** | IKEA (all regions) |
| **ALPHA_HARDWARE** | Home Depot (US/CA/MX), Lowe's (US/CA), Canadian Tire (CA), Leroy Merlin (EU), Brico Dépôt (FR/ES), Bauhaus (EU), Woodies (IE) |
| **ALPHA_WAREHOUSE** | Costco (all regions), Sam's Club (US/MX), BJ's (US), Makro (ES/NL/PL) |

Rewe (DE), Lidl, and Aldi are not ingested by design. These chains operate in neighbourhood grocery formats; their density would produce thousands of clusters below any useful district threshold. Their exclusion is a deliberate semantic decision, not a data gap.

### Tier Assignment (V3, May 2026)

Tiers are assigned by binary predicate gates (pure-geometric engine, `build-geometric-ranking.py`). A cluster is Tier 1 Regional if it passes all five gates: composition (Warehouse+Hypermarket or Lifestyle+Hypermarket), top-10% national primary catchment population, top-20% national secondary catchment population, regional hospital present, and IoU ≤ 0.10 with any stronger co-located cluster. The full predicate specification is in `SCORING-METHODOLOGY.md`.

Prior to Sprint 17 (May 2026), tiers were assigned by a composite score (V2: sum of base score, count bonus, diversity bonus, civic depth, overlap penalty). V2 sub-scores (`score_final`, `rank_v2`, and related fields) have been removed from emitted geometry; the V2 scripts are retained at `legacy/generate-rankings-v2.py`.

### Civic Data

Hospital and university classification is sourced from OpenStreetMap. Hospitals are classified as `regional`, `district`, or `clinic` based on the OSM `healthcare` tag and mapped name patterns. Universities are classified as `regional`, `small`, or `excluded`. Per-tier civic counts (`hc_count_regional`, `hc_count_district`, `he_count_regional`, `he_count_small`) are computed within a 5 km tertiary ring around each cluster anchor.

## 4. Disclaimers & Terms of Use

The Woodfine Location Intelligence platform (`gis.woodfinegroup.com`) provides synthesized location intelligence metrics "as is." 

1.  **Metric Synthesis**: Scores, ranks, and spend estimates are the result of computational modeling using the open-source inputs listed above.
2.  **Accuracy**: While we strive for high fidelity by combining multiple authoritative sources, Woodfine Group does not guarantee the precision of specific retailer coordinates or synthesized economic metrics.
3.  **Privacy**: No individual-level tracking or personally identifiable information (PII) is utilized. All mobility data is synthetic or aggregated.
4.  **Non-Navigation**: This data is not intended for real-time navigation or critical infrastructure planning.
