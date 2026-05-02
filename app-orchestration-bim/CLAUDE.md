# app-orchestration-bim

OrchestrationOS BIM aggregation and delivery hub. Stateless — holds no data of its own. Connects multiple `cluster-totebox-property` instances and provides compute above base-node capacity.

Read `.claude/rules/bim-product-family.md` before touching anything here.

**Licence: Proprietary — paid.** This is the commercial boundary in the BIM product family. Everything below `os-orchestration` is open source. This component is not.

---

## What this crate does

Aggregates BIM data across multiple PropertyArchive instances for three categories of work that a single `cluster-totebox-property` node cannot handle:

| Capability | Why orchestration is needed |
|---|---|
| Federated model viewer | Combining multiple buildings' glTF caches is GPU-heavy |
| Cross-archive spatial queries | "Which HVAC units across all properties need service this month?" |
| Portfolio-level CityJSONSeq | OGC-standard portfolio map, one building per line |
| IFC clash detection | Computationally expensive — above base node spec |
| BIM → lease register joins | Requires `cluster-totebox-property` + `cluster-totebox-corporate` |
| Batch ingestion | Converting 50 RVT files for a new acquisition |
| glTF streaming | Serving large models to multiple clients simultaneously |

---

## Stateless rule

`app-orchestration-bim` holds no persistent data. It reads from `cluster-totebox-property` archives. It serves results to `app-console-bim` and `app-workplace-bim` clients. It writes nothing back to the archive. Write-back always goes through `service-bim`'s ingestion queue with appropriate F12 authorization.

If you find a code path where orchestration writes directly to an archive, it is a bug.

---

## The federated model architecture

For multi-building 3D views, this component assembles a federated scene from multiple `cluster-totebox-property` glTF caches. The rendering engine served to clients is xeokit-sdk (AGPL-3.0) or @thatopen/components (MIT). xeokit is preferred for large federated scenes — its XKT format achieves approximately 25× compression vs raw IFC and handles double-precision coordinates on WebGL.

For portfolio-level spatial queries, CityJSONSeq (OGC Community Standard) is the format — one JSON object per building, one building per line. Directly compatible with standard GIS toolchains.

---

## Relationship to app-orchestration-command

`app-orchestration-bim` runs alongside `app-orchestration-command` (CommandCentre) on the same OrchestrationOS deployment at Woodfine. The operator may have both running simultaneously — CommandCentre for personnel and corporate administration, BIM for property coordination. They share the same OrchestrationOS host but operate independently.

---

## Research context

`RESEARCH.md` in this directory — covers the federated model architecture, xeokit performance characteristics and licence, CityJSONSeq at scale (TU Delft 3DBAG), and the compute boundary between base ToteboxOS and orchestration-tier workloads.
