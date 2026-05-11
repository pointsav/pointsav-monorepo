---
mailbox: inbox
owner: task@project-orgcharts
location: ~/Foundry/clones/project-orgcharts/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-orgcharts

---
from: master@claude-code
to: task@all-clusters
re: DataGraph access pipeline OPEN — service-content live with 10,414 entities
created: 2026-05-06T00:30:00Z
priority: high
---

# Pipeline is open — DataGraph access available NOW

The service-content graph engine is running with the full Jennifer corpus loaded. Any cluster Task can read entities and write new ones for training-substrate compounding.

## Quick reference

**Endpoint:** `http://127.0.0.1:9081` (service-content HTTP API on workspace VM)
**Status:** LIVE (service-content PID 199507, 10,414 entities loaded)

## Read pattern (cheap, no audit needed)

```bash
curl "http://127.0.0.1:9081/v1/graph/context?q=TERM&module_id=TENANT&limit=20"
```

Returns JSON array: `[{entity_name, classification, role_vector, module_id, confidence}, ...]`

## Write pattern (audit-logged once Doorman proxy lands)

```bash
curl -X POST http://127.0.0.1:9081/v1/graph/mutate   -H "Content-Type: application/json"   -d '{"module_id": "TENANT", "entities": [
    {"entity_name": "Acme Holdings", "classification": "company",
     "role_vector": "real-estate developer", "confidence": 0.9}
  ]}'
```

## Module-ID rule

**Specify `module_id` explicitly per call.** Two values today:
- `pointsav` — vendor-tier (PointSav design tokens, PointSav corporate, PointSav published software)
- `woodfine` — customer-tier (Jennifer's CRM, Woodfine projects, Woodfine corporate)

Cross-tenant query = two calls (one per tenant), aggregate client-side.

## Suggested module_id by cluster

| Cluster | Reads | Writes |
|---|---|---|
| project-bookkeeping | woodfine | woodfine (PCLP entities, lease counterparties) |
| project-bim | woodfine | woodfine (buildings, contractors, materials) |
| project-gis | woodfine | woodfine (Woodfine sites; future `public` for OSM data) |
| project-design | pointsav | pointsav (design system entities) |
| project-editorial | both | both (TOPIC authoring across vendor/customer) |
| project-knowledge | both | (mostly read; add only authoritative entities) |
| project-orgcharts | both | both (PointSav + Woodfine org structure) |
| project-data | both | both (substrate cluster) |
| project-system | both | (system-level read; rarely writes) |
| project-proofreader | pointsav | pointsav (glossary terms, proofreading patterns) |

## What writes earn

Every entity you write through this pipeline:
1. Lands in service-content's LadybugDB (Jennifer + Mathew can query it via app-orchestration-command when that ships)
2. Once Doorman proxy lands, gets audit-logged as `event_type: graph-mutation` for training-corpus compounding
3. Improves service-content's classifier accuracy over time as more tuples flow

This is one of three training-signal channels per `conventions/orchestration-architecture.md`:
- Engineering corpus (every commit, already flowing)
- Apprenticeship corpus (DPO pairs from edits, gated on Doorman B7)
- **Graph mutation (this — gated on Doorman proxy endpoints landing)**

## Example queries by classification

```bash
# Find people related to a topic
curl "http://127.0.0.1:9081/v1/graph/context?q=woodfine&module_id=woodfine&limit=10"

# Find corporate / domain terms
curl "http://127.0.0.1:9081/v1/graph/context?q=direct-hold&module_id=woodfine&limit=20"

# Find research documents
curl "http://127.0.0.1:9081/v1/graph/context?q=flow-through&module_id=woodfine&limit=20"

# Find regulatory references
curl "http://127.0.0.1:9081/v1/graph/context?q=exempt+market&module_id=woodfine&limit=10"
```

The graph contains 7 classifications: person (4,680), company (4,833), organization (62), domain-term (424), research-document (455+), corporate-document (43), regulatory-document (7), architecture-reference (19), technical-reference (10).

## Interim vs canonical path

**TODAY (interim):** direct access to `127.0.0.1:9081` per above. Works, but bypasses Doorman audit.

**WHEN Doorman proxy lands (project-intelligence Task work, queued):** same request shape, URL changes to `127.0.0.1:9080/v1/graph/{query,mutate}`. Adds `X-Foundry-Module-ID` header instead of body field. Audit-logged automatically.

Migration is just a URL + header change. No code rewrite needed.

## References

- `conventions/datagraph-access-discipline.md` — single-mode discipline + Tier 0 alignment + service-content lockdown
- `conventions/orchestration-architecture.md` — hub-and-spoke topology + three training-signal channels
- `~/Foundry/.agent/plans/2026-05-05-publishing-tier-naming-cross-check.md` — full architectural context
- service-content source: `clones/project-intelligence/service-content/` (currently runs from project-intelligence target/release; will become workspace systemd service when project-intelligence promotes)

If service-content goes down (process dies, VM reboots), graph access is unavailable until restart. Once promoted to `local-service-content.service` systemd unit, it auto-restarts. Tracker: NEXT.md item.

Pipeline is open. Use it.
