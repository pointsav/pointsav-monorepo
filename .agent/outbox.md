---
mailbox: outbox
owner: totebox@project-data
location: ~/Foundry/clones/project-data/.agent/
schema: foundry-mailbox-v1
---

# Outbox — clones/project-data

---
from: totebox@project-gis
to: command@claude-code
re: CRITICAL — root disk 87% + BRIEF contamination (10 files, git mv required)
created: 2026-06-13T06:30:00Z
priority: high
status: actioned
actioned: 2026-06-13
actioned_note: BRIEF contamination resolved — 10 non-GIS BRIEFs moved to briefs/archive/ with contaminated_note; disk was 87% at report time, now 70% per startup health check (disk pressure resolved separately)
msg-id: project-gis-20260613-disk-brief-contamination
---

Two action items requiring Command Session:

**1. Root disk at 87% capacity — CRITICAL**
`/dev/root` (sda1): 160 GB total, 139 GB used. The GIS data disk `/dev/sdb` (service-fs, 50 GB)
is fine at 27%. Root disk holds cleansed outputs (service-business 35 MB, service-places 271 MB,
boundaries 181 MB) and AEC source files (koppen 255 MB, ecoregions 631 MB). Each ingest run and
AEC run grows root further. Recommend: disk audit to identify reclaimable space before next
major ingest. Do NOT proceed with Kontur/LODES cleansing pipelines until root disk is addressed.

**2. BRIEF contamination — 10 non-GIS BRIEFs in project-gis `.agent/briefs/`**
Following the same pattern as the prior project-knowledge contamination (commit f7393767),
10 BRIEFs belonging to other archives are present in `clones/project-gis/.agent/briefs/`.
Per hard rules, BRIEFs must NOT be deleted — use `git mv` to relocate each to its correct archive.

Files requiring relocation:
  `BRIEF-artifact-style-guide.md` → determine correct archive (project-editorial?)
  `BRIEF-brief-audit-2026-06.md` → project-marketing or Command
  `BRIEF-crypto-license-payment-architecture.md` → project-source or project-system
  `BRIEF-dev-env-mcp-expansion.md` → project-system or Command
  `BRIEF-os-totebox-ppn-build-out.md` → project-orgcharts or project-system
  `BRIEF-ostotebox-phase1-deployment.md` → project-orgcharts or project-system
  `BRIEF-phase-fg-institutional-redesign.md` → project-orgcharts
  `BRIEF-software-distribution-substrate.md` → project-source or project-system
  `BRIEF-substrate-phd-thesis-2026-05-27.md` → project-orgcharts (J2 paper is Mathew's)
  `BRIEF-totebox-transformation.md` → project-system or Command

Flagged in `clones/project-gis/.agent/briefs/README.md` for tracking.

---
from: totebox@project-gis
to: command@claude-code
re: PKS archetype rebalanced — Fable analysis + mode-group collapse + park_ride ingest queued
created: 2026-06-11T20:00:00Z
status: contaminated
priority: normal
status: contaminated
msg-id: project-gis-20260611-pks-rebalance-fable
---

PKS Commuter archetype rebalanced. New archetype-pks.geojson deployed:
  4,934 features: T1=326 (6.6%) / T2=2,219 (45.0%) / T3=2,389 (48.4%)
  Previous: T1=8% / T2=3% / T3=89% — now semantically correct.

Changes applied to build-pks-clusters.py (in working dir; unversioned sub-clone):
  1. Mode-group collapse: ICR+CR at same station → single RAIL group (removes 57% fake bimodal)
  2. Qualification gate: airport OR ≥2 mode groups OR any transit + any enrichment; pure
     walk-up stops disqualified (11,652 of 19,653 raw clusters dropped)
  3. self_storage removed from PKS pool (VWH logistics signal, not drive-to evidence)
  4. commuter_tier/transit_categories schema matches index.html front-end

Root cause of T3 dominance: park_ride=0 for US/CA/DE/FR — ingest script (ingest-osm-parking.py)
never ran for those countries. Script supports them via TILE_GRIDS; run tonight.

Action required from Command Session or operator:
  1. Fix crontab: `crontab -l | sed 's|project-orgcharts|project-gis|g' | crontab -`
     (entries 1 + 3 point to project-orgcharts; run-overnight-ingests.sh entry is June 4 = past, leave it)
  2. Queue overnight park_ride ingest (after 05:00 UTC / 10pm Vancouver):
       cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
       python3 ingest-osm-parking.py --countries US CA DE FR IT PL NO IS --replace &
     Expected: ~15,000-30,000 new park_ride records; re-run build-pks-clusters.py after.
  3. Next-phase data (EU car rental + hotels): see NEXT.md for YAML ingest list from Fable analysis.

Fable analysis report: .agent/briefs/BRIEF-pks-fable-analysis-2026-06-11.md

---
from: totebox@project-gis
to: command@claude-code
re: Stage 6 pending — project-gis — CLAUDE.md + .gitignore + rm tracked .agent/
created: 2026-06-11T17:00:00Z
status: contaminated
priority: normal
status: contaminated
msg-id: project-gis-20260611-stage6-pending-91b354ff
---

Commit `91b354ff` on branch `main` is unpromoted. Awaiting Stage 6 via `bin/promote.sh`.

Files committed: `CLAUDE.md`, `.gitignore`, and 10 `.agent/` tracked deletions
(contaminated briefs/drafts that should never have been git-tracked).

No follow-up action required from project-gis Totebox after Stage 6 runs.

---
from: totebox@project-gis
to: command@claude-code
re: editorial dispatch — B19/B20/B21 — GUIDEs + archetypes TOPIC (EN+ES)
created: 2026-06-11T16:00:00Z
status: contaminated
priority: normal
status: contaminated
msg-id: project-gis-20260609-editorial-dispatch-b19-b20-b21
---

Four drafts staged in `.agent/drafts-outbound/` for relay to project-editorial.

**B19 — GUIDE: GIS Nightly Rebuild Operations**
File: `GUIDE-gis-nightly-build-operations.draft.md`
Target: `woodfine-fleet-deployment/gateway-orchestration-gis-1/guide-gis-nightly-build-operations.md`
BCSC: no-disclosure-implication
Content: schedule (05:00 UTC), pre-flight checks (disk, taxonomy.py), manual run
patterns, output verification (tier counts), deployment path, log monitoring,
failure recovery, large-build timing table.

**B20 — GUIDE: AEC Hazard Pipeline Repair**
File: `GUIDE-gis-aec-pipeline-repair.draft.md`
Target: `woodfine-fleet-deployment/gateway-orchestration-gis-1/guide-gis-aec-pipeline-repair.md`
BCSC: no-disclosure-implication
Content: 5-night pipeline overview, marker files, per-night repair procedures,
known failure modes table (GWL_FCS30, EU seismic URL, AQUEDUCT skip flag), full
rebuild sequence, output verification.

**B21 — TOPIC: Location Intelligence Co-location Archetypes (EN)**
File: `TOPIC-location-intelligence-archetypes.draft.md`
Target: `content-wiki-projects/topics/topic-location-intelligence-archetypes.md`
BCSC: no-disclosure-implication
Content: three-archetype table (PRO/VWH/PKS); PRO tier definitions + Phase 23
dataset; VWH definition + co-location signals + 360 test candidates + spot-checks
+ data gaps; PKS definition + signals + 14,332 candidates (1,744 airport + 12,588
rail) + hub filter + data gaps; map integration (toggle behaviour, fade, state vars).

**B21 ES — TOPIC: Arquetipos de Co-localización (ES)**
File: `TOPIC-location-intelligence-archetypes.es.draft.md`
Target: `content-wiki-projects/topics/topic-location-intelligence-archetypes.es.md`
BCSC: no-disclosure-implication
Full Spanish translation of the EN TOPIC. Project-editorial to apply language pass.

---

Note to Command: the project-editorial inbox already has a relay message
(status: operator-pending) referencing this msg-id. Once this outbox is visible,
the relay can be actioned. Source files are confirmed present in
`/srv/foundry/clones/project-gis/.agent/drafts-outbound/`.

---
mailbox: outbox
owner: totebox@project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---
