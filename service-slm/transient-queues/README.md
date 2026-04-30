# transient-queues

**Runtime data directory — not tracked in Git.**

## Purpose

This directory is the staging area for `cognitive-bridge.sh`. Totebox writes inbound payloads here; the bridge picks them up, routes them through the Doorman, and moves processed results to the appropriate downstream service.

## Lifecycle

```
Totebox inbound
    ↓
transient-queues/ ← TX-*.txt files (YAML frontmatter + content)
    ↓
cognitive-bridge.sh (reads, routes through Doorman, processes response)
    ↓
service-fs/data/ ← persistent state (if needed)
    ↓
service-content/knowledge-graph/ ← structured output (JSON)
```

## File format

Payloads in this directory follow the standard Totebox frontmatter + body pattern:

```
---
asset_id: "TX-<ULID>"
source: "<origin service>"
system_directive: "<instruction>"
status: "PENDING_COGNITIVE_STATE"
---

<text content for cognitive processing>
```

## Gitignore

This directory is **never committed to Git**. The `.gitignore` rule excludes all files except `README.md`:

```
transient-queues/
!transient-queues/README.md
```

Treat this directory as volatile runtime state. Persistent data moves to:
- `service-fs/data/` for long-term storage
- `service-content/knowledge-graph/` for structured extraction output

## References

- `cognitive-bridge.sh` — Doorman router and payload orchestrator
- `service-slm/ARCHITECTURE.md` — Full system design
- `discovery-queue` (monorepo) — Similar pattern for ingest-side runtime data
