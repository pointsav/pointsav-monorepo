---
schema: foundry-v1
state: draft
audience: software-engineers
language_protocol: TECH-TOPIC
---

# Service: service-business (Retail Intelligence & Clustering)

`service-business` is the intelligence layer responsible for transforming raw retail data points into "Actionable Clusters." It implements the "SafeGraph-pattern" of parent-child relationships to handle complex physical retail environments.

## The Clustering Logic

Retail data is inherently messy. A single commercial node often contains multiple distinct points (e.g., a Big-Box store, a nested pharmacy, and a gas bar in the parking lot). `service-business` processes these points to ensure the GIS engine only sees a single, unified commercial entity.

### Grid-Based Spatial Indexing
To perform this at scale, the service uses a grid-based spatial index (approx. 1km cells). It iterates through the `service-fs` raw lake and groups entities that share a physical footprint (threshold: < 100m).

### Parent-Child Schema
*   **Parent Node:** The primary commercial driver (usually the Alpha Anchor).
*   **Children (Sub-Entities):** Secondary operators located within the same spatial node.

## Cleansed Data Output
The output is a refined `cleansed-clusters.jsonl` file stored in the `/cluster-totebox-personnel-1/service-business/` directory. This "Actionable Data" is what the downstream `app-orchestration-gis` uses to build the regional index.

---
*Back to [Data Lake Overview](topic-service-fs-data-lake.md) | Next: [Service-Places Filtering](topic-service-places-filtering.md)*
