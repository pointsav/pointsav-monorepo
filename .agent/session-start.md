---
schema: foundry-session-start-v1
archive: project-data
updated: 2026-06-01
---

# Session start — project-data

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** GIS co-location analysis pipeline, JOURNAL academic papers (J1–J6), AEC
  environmental data layers, Regional Markets editorial production. The monorepo sub-clone
  at `./pointsav-monorepo/` carries the `app-orchestration-gis` crate and related pipeline code.
- **Active branch:** `main` (archive-level); monorepo sub-clone on `cluster/project-data`
  (confirm with `cd pointsav-monorepo && git branch`)
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)

## Critical state (as of 2026-06-01)

- **AEC pipeline** — Seismic rebuild crontabbed 2026-06-01T05:00Z; flood build crontabbed
  2026-06-02T05:00Z. Logs in `project-gis` clone: check
  `/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/build-aec-seismic.log`
  and `build-aec-flood.log`. Coverage metrics feed J3 §6 Results.
- **JOURNAL J1** — §7.2 OLS regression blocked on Phase 24B (Kontur H3 population join).
  Permutation test (`sim-tier-permutation.py`) not yet written.
- **JOURNAL J3** — §6 Results blocked on AEC flood + seismic coverage metrics.
- **JOURNAL J4** — §4–§5 language pass at project-editorial (outbox ref: 952b2b09).
- **Stage 6 pending** — 2 unpromoted commits (`59373c45`, `005cc299`) from this session's
  startup cleanup. Command Session needs `bin/promote.sh`.
- **Regional Markets** — A10/A11/A12 on hold pending methodology revision.
  Dispatched A7/A8/A9/A15/A16/A17 are at project-editorial.
- **Briefs** — All contaminated briefs archived 2026-06-01. Main briefs/ is empty of
  active briefs. See `.agent/briefs/README.md`.

## Key directories in this archive

- `JOURNAL/` — canonical JOURNAL draft files (J1–J6)
- `work/` — OLS regression scripts and output (J1 §7.2 prep)
- `pointsav-monorepo/` — monorepo sub-clone; GIS crate lives here
- `app-orchestration-gis/` — GIS pipeline app (also present in monorepo sub-clone's parent at project-gis)
- `.agent/rules/artifact-registry.md` — JOURNAL and A-series artifact tracking

## Note on archive identity

This archive's folder is `project-data`. The NEXT.md and some state files were contaminated
from `project-gis` (a separate archive at `/srv/foundry/clones/project-gis/`). The crontab
AEC pipeline jobs run in `project-gis`. The JOURNAL papers and Regional Markets editorial are
tracked and owned here in `project-data`.

## Known gotchas

- The crontab AEC builds write logs to the `project-gis` clone, not this archive.
- `NEXT.md` may contain items labelled "project-gis" that were contaminated; treat them as
  project-data items or verify against the project-gis NEXT.md.
- Monorepo sub-clone branch: confirm `cd pointsav-monorepo && git status` at session start.
