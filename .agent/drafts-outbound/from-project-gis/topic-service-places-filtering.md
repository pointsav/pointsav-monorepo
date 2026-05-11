---
schema: foundry-v1
state: draft
audience: software-engineers
language_protocol: TECH-TOPIC
---

# Service: service-places (Regional Anchor Filtering)

`service-places` is the service responsible for validating and clustering civic and institutional infrastructure. Its primary role is to filter out "noise" from non-regional facilities (like small clinics or community colleges) to ensure the GIS rankings are institutional-grade.

## Filtering Thresholds

The service applies strict attribute-weight filters to the raw data in `service-fs/service-places/`.

*   **Regional Hospitals:** Must meet a minimum weight (e.g., 50+ staffed beds).
*   **Regional Universities:** Must meet a minimum enrollment threshold (e.g., 1000+ FTE students).
*   **Airports:** Validated as major regional transport hubs.

## Spatial Aggregation
Large institutional campuses (like University or Hospital complexes) often appear in raw data as dozens of separate points. `service-places` uses a 200m spatial buffer to cluster these into a single "Regional Anchor" with a unified center of gravity.

## Data Output
The resulting `cleansed-places.jsonl` provides the "Actionable Tertiaries" that the `app-orchestration-gis` uses to award the final co-location tier rankings.

---
*Back to [Data Lake Overview](topic-service-fs-data-lake.md) | Next: [GIS Orchestration Engine](topic-app-orchestration-gis.md)*
