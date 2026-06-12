---
schema: foundry-cluster-manifest-v1
cluster: project-bim
cluster_name: BIM Engineering
cluster_branch: cluster/project-bim
created: 2026-05-18
state: active (research phase; four crates scaffold/reserved; IFC pipeline design complete)
slm_endpoint: http://localhost:9080
module_id: bim
doctrine_version: 0.0.14
doctrine_claims_codified: [10, 37]
doctrine_claims_proposed: []

tetrad:
  vendor: pointsav-monorepo (cluster/project-bim) — service-bim, app-console-bim, app-workplace-bim, app-orchestration-bim
  customer: woodfine-fleet-deployment/bim — leg-pending (no guide yet)
  deployment: gateway-bim-1 — leg-pending (no instance provisioned)
  wiki: content-wiki-documentation — BIM material handoff pending (see .agent/rules/handoffs-outbound.md)

datagraph_module_id: bim
cross_cluster_dependencies:
  - project-design (DESIGN-* token and component artifacts for BIM UX)
  - project-editorial (TOPIC-* wiki artifacts from BIM research handoff)

provisioning_notes: |
  Archive cloned from pointsav-monorepo cluster/project-bim branch.
  Working in: ~/Foundry/clones/project-bim/
  Sub-clone (monorepo): ~/Foundry/clones/project-bim/pointsav-monorepo/
  Stage 6 promotion: bin/promote.sh from Command Session.

session_role: totebox
default_starting_dir: ~/Foundry/clones/project-bim/
---

## Cluster mission

Build and maintain the BIM (Building Information Modeling) product family — a four-component
suite for managing construction and facilities data using open buildingSMART standards.

### Product family

| Crate | Layer | Host OS | Role |
|---|---|---|---|
| `service-bim` | Archive daemon | os-totebox | Flat-file BIM archive maintenance; ingestion pipeline; IFC validation |
| `app-console-bim` | Coordination terminal | os-console | Query, coordinate, link work orders; Navisworks muscle memory |
| `app-workplace-bim` | Editor | os-workplace | Author and edit BIM geometry; AutoCAD / Revit muscle memory |
| `app-orchestration-bim` | Aggregation hub | os-orchestration | Multi-archive queries; glTF streaming; proprietary tier |

### Canonical data contract

All components read from and write to `cluster-totebox-property/service-bim/`.
Canonical format: **IFC-SPF (ISO 16739-1:2024)** — plain ASCII, 50-year readable.
`service-bim` is the only process that writes to `canonical/`; all other components
write to `ingestion/queue/` only.
Generated outputs (`cache/model.glb`) are never canonical.

### Key standards

- IFC-SPF: ISO 16739-1:2024 (geometry and semantics — canonical)
- BCF 3.0: buildingSMART issue markup
- IDS 1.0: buildingSMART handover validation (June 2024)
- bSDD: buildingSMART Data Dictionary URIs for element classification
- glTF 2.0 / ISO 12113:2022: 3D viewer cache only (non-canonical)

### ADR hard rules (from .agent/rules/bim-product-family.md)

- SYS-ADR-07: IFC structured data always through IfcOpenShell subprocess — never through AI layer
- SYS-ADR-10 (F12): no file enters `canonical/` without explicit operator commit action
- IfcOpenShell (LGPL-3.0): subprocess invocation only — do not link into any Rust binary

### Current state

All four crates are in research / reserved-folder state per project-registry.md.
IFC pipeline design, data contract layout, muscle-memory targets, and licence split
are documented in `.agent/rules/bim-product-family.md`.
BIM research material handoff to `content-wiki-documentation` is pending
(see `.agent/rules/handoffs-outbound.md`).
