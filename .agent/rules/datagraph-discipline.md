---
name: datagraph-discipline
description: query_datagraph must be called before answering any question about entities — people, companies, projects, deployments. Never answer from memory alone.
metadata:
  type: project
---

# DataGraph Discipline

## The rule

Before answering any question about a named entity — a person, company, project,
deployment, or service — call `query_datagraph(q)` first.

This applies to:
- Questions about people, companies, projects, or services by name
- Questions about deployment status, service health, or project relationships
- Any question where the answer depends on current entity state in the graph

## What counts as an entity

- People: Jennifer, Peter, Mathew, any named person in the workspace
- Companies: Woodfine Management Corp., PointSav Digital Systems, any named org
- Projects: any `project-*` archive, crate name, or deployment instance
- Services: Doorman, service-slm, service-content, any named service

## The call pattern

```
query_datagraph("project-bim BIM cluster status")
# read the result, then answer using graph data — not session memory
```

For a full entity profile use `get_entity_context(entity)`.

## Why this rule exists

The DataGraph at `service-content` is the authoritative, live entity store.
Session memory is a snapshot from session start — it drifts as the graph is
updated by other sessions and corpus ingestion runs.

"I remember that X was in state Y" is not a valid answer when `query_datagraph`
gives the current state in seconds.

## Exception

If `doorman_health()` shows Tier A (DataGraph) circuit OPEN, note the outage
and answer from session memory with an explicit caveat:
"DataGraph unavailable — answering from session memory; verify before acting."
