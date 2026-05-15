---
schema: foundry-session-start-v1
archive: project-gis
updated: 2026-05-14
---

# Session start — project-gis

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Customer-facing location intelligence public demo. Owns `service-places` (public-purpose location data) and the GIS tile scoring pipeline. Live at `gis.woodfinegroup.com`. V2 0–1000 scoring live in tiles.
- **Active branch:** `cluster/project-gis`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** `comprehensive-data-and-legal-plan` (see `.agent/plans/`)

## Known gotchas

- This archive is in `state: provisioning` — sub-repos may not be fully cloned. Check `.agent/manifest.md` for current tetrad state.
- GIS Phase C tile rebuild and D1 parent-child model are the next engineering milestones (operator-gated on WireGuard Part A).
- Deep-seal sprint complete as of 2026-05-05; 2 follow-ups pending (boundary download + IPEDS EF URL).
- This archive is primarily a drafts-outbound gateway for wiki TOPIC content — most GIS editorial content routes through `project-editorial`, not committed here.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).

## Last session handoff

*2026-05-05 — Deep-seal sprint complete. V2 scoring live. 2 follow-ups pending (boundary download + IPEDS EF URL). GIS Phase C/D gated on WireGuard Part A.*
