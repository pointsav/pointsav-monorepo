---
schema: foundry-v1
state: draft
audience: software-engineers
language_protocol: TECH-TOPIC
---

# Service: service-fs (The Raw Data Lake)

`service-fs` is the foundational storage unikernel for the PointSav GIS platform. It implements a "Data Lake" philosophy, where raw, unstructured geospatial points are ingested from multiple sources (OSM, Overture, SafeGraph, etc.) and stored in a durable, modular file system.

## Data Ingestion & Storage

The service manages a unified filesystem structure within the `/cluster-totebox-personnel-1/` deployment. 

*   **Root Directory:** `/service-fs/`
*   **Retail Landing:** `/service-fs/service-business/`
*   **Civic Landing:** `/service-fs/service-places/`

### Architectural Role
As the "Stateful" layer of the platform, `service-fs` is responsible for data persistence. It is designed to be independent of the analytical software; even if the GIS orchestration gateways are lost, the core data assets remain safe within this layer.

## Unikernel Implementation
In production, `service-fs` is deployed as a specialized, low-overhead unikernel. It provides a restricted API for the `service-business` and `service-places` intelligence layers to read raw data and write back processed results.

---
*Next: [Service-Business Clustering](topic-service-business-clustering.md)*
