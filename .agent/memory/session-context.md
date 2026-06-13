---
schema: foundry-session-context-v1
archive: project-data
---

# Session context — project-data

## Operator preference digest
No preferences recorded yet.

## Cross-archive carry-forward
- JOURNAL-retail-colocation §7.2: pending Phase 24B data before submission to *Economic Geography* (Wiley, IF 7.2).
- Contamination note resolved 2026-06-12: bulk-copied gis files (gis-location-intelligence-archetypes.md, project_ring2_ring3_coupling.md, project_vm_hardening_state.md) were deleted by canonical rebase; .agent/ now gitignored per 2381a169.

## Operator preference digest

- **Scope discipline:** In a project-data Totebox session, only action work owned by this
  archive. Cross-archive blockers (project-infrastructure, project-system, project-console,
  project-gis, etc.) go to outbox — never actioned in-session. Corrected 2026-06-11.

## Session entries

### 2026-06-11 — os-totebox PPN build-out session 1 (totebox@claude-code)

Done: (1) service-people axum HTTP server GET /v1/people + GET /v1/people/{id} on :9091,
reads ledger_personnel.json — committed 997b8d22. (2) service-extraction workspace unification:
removed standalone [workspace], added to root Cargo.toml members — committed 997b8d22.
(3) Cargo.lock duplicate caseless entry removed. (4) J7 HOLD lifted, ~2,600 words written
(Abstract, Intro, Lit Review, Methodology, Hypotheses, Falsification Programme) — committed
8ab01ff2. (5) Outbox → Command: promote project-data (25 commits) + BRIEF redistribution.
(6) Outbox → project-gis: service-people contract ACK for project-console F2 relay.

Pending/carry-forward: Stage 6 promotion (Command); service-people CRUD (deferred);
J7 §4-§8 after first os-totebox deployment; ORCID IDs (operator).

Operator preference surfaced: strict project-data scope discipline (cross-archive → outbox only).
BRIEF-os-totebox-ppn-build-out.md created to track multi-session build-out.

### 2026-06-09 — MCP v0.3.0 readiness update (Command@claude-code)
CLAUDE.md updated with MCP v0.3.0 tools table + artifact-type bright-line rules.
session-context.md stub provisioned (this file).

### 2026-06-03 — Archetype model rework (project-data@claude-code)
Commuter (PKS) redefined as geometric airport-led park-and-ride: regional airports (≤600 km from
metro ref) + outer commuter-rail-belt stations (15–110 km ring). Airport-led expands NA map-cell
coverage 96 → 957. 5,977 features deployed (cache-bust token v=20260603d).
Urban Fringe (VWH) → Retail-density model: qualify_vwh() admits ≥2-category co-locations OR lone
STRONG/BROAD trade stores; composition-score tiering T1/T2/T3. 7,028 features deployed.
New scripts: tools/sim_spread.py, ingest-osm-parking.py, ingest-osm-parcel-depot.py,
run-overnight-ingests.sh (crontab June 4 05:00 UTC).
Code commit: aec2187e (7 source files). Docs commit: same session.
Stale-label fixes applied: session-context.md + MEMORY.md headers corrected from project-orgcharts/
project-infrastructure → project-data.
