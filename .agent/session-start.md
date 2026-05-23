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

*2026-05-23 — Phase 19 complete and building tonight. AEC parity research done.*

### What is running tonight (DO NOT STOP)

PID 2507282 — scheduled at 05:00 UTC 2026-05-24:
```
bash nightly-rebuild.sh && bash phase19-rebuild.sh
```
- Pass 1 (nightly-rebuild.sh): cluster rebuild with London fix + geometric T2→T3 split
- Pass 2 (phase19-rebuild.sh): ingest 16 sport chains + final cluster/tile rebuild
- Expected output: T1=1,157 / T2=2,889 / T3=1,656; logs to nightly-rebuild.log + phase19-rebuild.log
- Monitor: `tail -f pointsav-monorepo/app-orchestration-gis/phase19-rebuild.log`

### Commits this session (ready for Stage 6)

| Commit | What |
|---|---|
| a2c974e4 | Phase 19: sport + geometric split + London fix + costco-uk + tonight approval |
| 9886d9fa | AEC nightly build plan (Nights 2–5) |
| e1792934 | AEC parity research (EU flood upgraded, eco-regions, CONABIO blocker) |
| 34a48183 | AEC build plan — parity upgrades incorporated |

### AEC nightly build sequence (Nights 2–5)

Scripts not yet created. Must be written before each night's window:
- **Night 2 (2026-05-25 05:00 UTC):** `build-aec-climate-solar.sh` — ASHRAE+NECB+EU climate zones+solar GHI
- **Night 3 (2026-05-26 05:00 UTC):** `build-aec-koppen-ecozones.sh` — Köppen global + Resolve eco-regions + EU biogeo + PVGIS
- **Night 4 (2026-05-27 05:00 UTC):** `build-aec-seismic.sh` — USGS/NRCan/ESHM20 seismic + GWL_FCS30 wetlands
- **Night 5 (2026-05-28 05:00 UTC):** `build-aec-flood.sh` — FEMA + EU regulatory floods + AQUEDUCT + wildfire
  - ⚠ Disk check required first: need ≥35 GB free on /srv/foundry
  - ⚠ EFFIS wildfire data request needed: https://forest-fire.emergency.copernicus.eu/applications/data-and-services

### After Night 5 (separate session)

BentoBox "Site Conditions" section + MapLibre "Site & Hazard" layer controls.
Full spec in AEC-NIGHTLY-BUILD-PLAN.md §After Night 5.

### Other pending items

- demand_rank pipeline: synthesize-od-study.py needs to run (layers 4/5/6 tiles deployed; scoring not yet populated)
- DE lifestyle chains (XXXLutz/Höffner/Segmüller): Phase 20
- Meijer US / Bodega Aurrera MX: Phase 19 or 20
- Stage 6 promotion: 4 commits above need Command Session promotion
