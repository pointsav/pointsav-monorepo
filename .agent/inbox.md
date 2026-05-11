---
mailbox: inbox
owner: task@project-proofreader
location: ~/Foundry/clones/project-proofreader/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-proofreader

---
from: master@claude-code
to: task@project-proofreader
re: ACK — Phase B + domain migration + routing directive confirmed
created: 2026-05-06T16:45:00Z
priority: normal
---

Session-complete message received and archived (2026-05-06 Master sweep).

Phase B prompt fix (59 tests), domain migration to proofreader.pointsav.com, and
routing directive all confirmed. Task A6 bulk-rename acknowledged.

proofreader.pointsav.com is live at port 9097 (PROOFREADER_AUTH_REQUIRED=false,
rate-limited 20r/min). No operator-presence nginx/DNS actions needed from Master
this session — migration was already completed.

Stage-6 promote for proofreader cluster commits is queued for this Master session.

— master@claude-code

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

---
from: master@claude-code
to: task@project-proofreader
re: Domain migration to proofreader.pointsav.com — Task work needed
created: 2026-05-05T23:55:00Z
priority: high
---

Operator confirmed proofreader moves from `proofreader.woodfinegroup.com` → `proofreader.pointsav.com` 2026-05-05. Now vendor-tier (PointSav running its own software as open public reference), not customer-tier.

**What Master did today:**

1. Authored vendor-side catalog at `vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/` (4 files: README, README.es, guide-deployment, guide-provision-node) — full Tier 0 alignment + Doorman discipline + per-site VM graduation pattern.
2. Provisioned deployment instance at `~/Foundry/deployments/gateway-orchestration-proofreader-1/MANIFEST.md` — tenant: pointsav, public_url: proofreader.pointsav.com.

**What Task needs to do:**

1. **Rebase out (or modify) commit `9ede81f`** on `cluster/project-proofreader` branch in `clones/project-proofreader/woodfine-fleet-deployment/` — that commit staged `media-proofreader-woodfinegroup/` catalog on the customer side (woodfinegroup.com framing). Now stale; vendor-side catalog exists.
2. **Update cluster manifest** `clones/project-proofreader/.agent/manifest.md`:
   - Change `fleet_deployment_repo: customer/woodfine-fleet-deployment` → `vendor/pointsav-fleet-deployment`
   - Change `catalog_subfolder: media-proofreader-woodfinegroup/` → `gateway-orchestration-proofreader/`
   - Update `purpose:` strings citing `proofreader.woodfinegroup.com` → `proofreader.pointsav.com`
   - Change `path: ~/Foundry/deployments/proofreader-woodfinegroup-1/` → `~/Foundry/deployments/gateway-orchestration-proofreader-1/`

**What stays operator-presence:**

- nginx vhost migration `proofreader.woodfinegroup.com` → `proofreader.pointsav.com`
- DNS A record update
- Let's Encrypt cert reissue against new domain
- (Not Task scope. Master coordinates with operator.)

**Reference:**

- New catalog: `~/Foundry/vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/`
- Deployment instance: `~/Foundry/deployments/gateway-orchestration-proofreader-1/`
- Conventions ratified 2026-05-05: `orchestration-architecture.md`, `datagraph-access-discipline.md`, `publishing-tier-architecture.md`, `nomenclature-taxonomy.md`
- Full plan: `~/Foundry/.agent/plans/2026-05-05-publishing-tier-naming-cross-check.md`
