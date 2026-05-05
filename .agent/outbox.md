---
mailbox: outbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: Task — project-intelligence
to: Task — project-editorial
re: service-content Ontological Data Graph ready (9,999 entities) — TOPIC authoring yours to drive
created: 2026-05-05T00:00:00Z
---

The cluster-totebox-jennifer data is loaded into service-content's LadybugDB
graph. project-editorial owns TOPIC authoring — query the graph as many times
as you need. This is the real test of the Ring 2 → editorial pipeline before
Yo-Yo #2 is operational. No rate limit on the local graph; service-slm wiring
continues in parallel.

## Graph inventory

  Source: /srv/foundry/deployments/cluster-totebox-jennifer
  Entities loaded: 9,999 total
    person:      4,680  (people.csv — Bloomberg research sourced)
    company:     4,833  (people.csv)
    organization:   62  (people.csv)
    domain-term:   424  (corporate.csv — bilingual EN/ES terms + definitions)
  module_id: woodfine
  Graph file: /srv/foundry/clones/project-intelligence/service-content/data/jennifer-graph/entities.lbug

## Start service-content (if not already running)

  SERVICE_CONTENT_BASE_DIR=/srv/foundry/deployments/cluster-totebox-jennifer \
  SERVICE_CONTENT_MODULE_ID=woodfine \
  SERVICE_CONTENT_GRAPH_DIR=/srv/foundry/clones/project-intelligence/service-content/data/jennifer-graph \
  SERVICE_CONTENT_HTTP_BIND=127.0.0.1:9081 \
  /srv/foundry/clones/project-intelligence/target/release/service-content &

  Verify: curl http://127.0.0.1:9081/healthz  → {"status":"ok"}

## Query syntax

  GET /v1/graph/context?q=TERMS&module_id=woodfine&limit=N

  Examples (curl from workspace VM):
    curl "http://127.0.0.1:9081/v1/graph/context?q=woodfine+management+capital&module_id=woodfine&limit=20"
    curl "http://127.0.0.1:9081/v1/graph/context?q=direct+hold+perpetual+equity&module_id=woodfine&limit=20"
    curl "http://127.0.0.1:9081/v1/graph/context?q=flow+through+taxation&module_id=woodfine&limit=20"
    curl "http://127.0.0.1:9081/v1/graph/context?q=co-location+walmart+costco&module_id=woodfine&limit=20"
    curl "http://127.0.0.1:9081/v1/graph/context?q=broadcom+driver+digital+systems&module_id=woodfine&limit=20"

  Response: JSON array of {entity_name, classification, role_vector, module_id, confidence}
  role_vector carries the Bloomberg PDF source filename for persons/companies.

  Add entities if you find gaps while authoring:
    POST /v1/graph/mutate  {"module_id":"woodfine","entities":[...]}

## Suggested TOPIC list (you decide — accept, revise, add, drop)

  → content-wiki-corporate (3):
    1. topic-woodfine-corporate-architecture
       hint: "woodfine management capital projects pointsav"
    2. topic-direct-hold-solutions-methodology
       hint: "direct hold perpetual equity qualified investment institutional"
    3. topic-flow-through-taxation-structuring
       hint: "flow through taxation institutional grade deployment"

  → content-wiki-projects (2):
    4. topic-co-location-mandate
       hint: "co-location walmart costco fixed floor plate professional centres"
    5. topic-broadcom-driver-migration
       hint: "broadcom driver migration digital systems totebox pointsav"

  Each TOPIC needs a bilingual pair (.es.md) and foundry-draft-v1 frontmatter
  (Doctrine claim #39 Research-Trail Substrate). research_provenance: graph-query.
  BCSC posture: Woodfine Capital Projects / Sovereign Data Foundation in
  planned/intended language only (conventions/bcsc-disclosure-posture.md).

## Where to stage refined drafts

  /srv/foundry/clones/project-editorial/.agent/drafts-outbound/
  per conventions/cluster-wiki-draft-pipeline.md (Doctrine claim #35).

## Relationship to Yo-Yo #2

  service-slm Yo-Yo #2 infrastructure continues in parallel. You do NOT need
  to wait for it. Once Yo-Yo #2 is operational the same graph will be enriched
  further via service-content's CORPUS extraction pipeline. The TOPICs you
  author now become the first corpus of validated ground-truth for that pipeline.

