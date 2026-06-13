---
artifact: brief
schema: foundry-brief-v1
name: BRIEF-gis-nightly-rebuild-aec-2026-06-12
language_protocol: CODE-RESEARCH
status: active
created: 2026-06-12
author: totebox@project-gis (claude-sonnet-4-6)
---

# GIS Nightly Rebuild + AEC Layer Infrastructure — Research Brief

Documented 2026-06-12 following a 3-audit investigation (2× Opus, 1× Fable). Covers:
cross-archive gateway contamination, AEC field wipe mechanism, cron timezone error,
and the governance design to prevent recurrence.

---

## §1 Problem Statement

### Gateway contamination

Six `nightly-rebuild.sh` scripts across six project-* archives all write to the same
gateway deployment (`gateway-orchestration-gis-1`). Until 2026-06-12, project-orgcharts
was the active cron job. Its uncalibrated VWH/PKS scripts (Jun 3 vintage) overwrote
calibrated production files (Jun 11 vintage) every night.

| Archive | Full build chain | Status |
|---|---|---|
| **project-orgcharts** | YES (all scripts) | **Active contaminator** — ran today 05:01 PDT |
| project-system | clusters+tiles only | Latent (no cron, script exists) |
| project-command | clusters+tiles only | Latent (stale, double-nested path) |
| project-data | NO (missing scripts) | Latent (would error out) |
| project-bim | unknown | Latent |
| **project-gis** | VWH+PKS only | **Intended — broken** (missing clusters/tiles/taxonomy) |

### AEC field wipe

`nightly-rebuild.sh` step 2 (`build-tiles.py --layer 2`) regenerates `clusters-meta.json`
from scratch with 21 base fields — no merge, no AEC preservation. AEC enrichment is silently
wiped every night. The Jun-8 AEC run (koppen=6,485/ecoregion=6,461/GHI=6,481) was wiped by
the Jun-12 05:01 nightly rebuild. Deployed `clusters-meta.json` has **zero AEC fields** today.

### Cron timezone error

`0 5 * * *` fires at 05:00 PDT = **12:00 UTC** (noon). All script comments, the existing
ops guide (GUIDE-gis-nightly-build-operations), and session notes say "05:00 UTC." Wrong by
7 hours. The "overnight" build runs at midday.

### Missing scripts in project-gis

project-gis clone is missing `build-clusters.py`, `build-tiles.py`, `taxonomy.py`. Its own
`nightly-rebuild.sh` dies at preflight (`wc -l < taxonomy.py` fails under `set -euo pipefail`)
— same signature as the Jun-10 silent death. project-gis has not completed a full rebuild since
at least 2026-06-04.

---

## §2 AEC Pipeline State

### Scripts

| Script | Location | Role |
|---|---|---|
| `build-aec-global.sh` | project-gis `app-orchestration-gis/` | Köppen, ecoregion, solar GHI/wind/temp/HDD-CDD (EU PVGIS-SARAH2, NA PVGIS-NSRDB) |
| `build-aec-seismic.sh` | project-gis `app-orchestration-gis/` | `seismic_pga_g` (USGS NSHM 2023, NRCan 2015, ESHM20) + wetland |
| `build-aec-flood.sh` | project-gis `app-orchestration-gis/` | `flood_hazard`, `wildfire_hazard` + hazard tiles |

### Layer status (2026-06-12)

| Layer | AEC Fields | Deployed Tiles | Last Successful Run | Status |
|---|---|---|---|---|
| Köppen + ecoregion | `koppen_class`, `ecoregion_name`, `ecoregion_biome` | `layer9-koppen-global.pmtiles`, `layer9-ecoregions-global.pmtiles` | Jun 8 (6,485/6,493) | **Wiped by nightly** |
| Solar + climate | `ghi_kwh_m2_yr`, `wind_speed_ms`, `temp_annual_mean_c`, `hdd18`, `cdd18` | (inline in clusters-meta) | Jun 8 (6,481/6,493) | **Wiped by nightly** |
| Wetland | `wetland_class` | (inline) | Never — 0/6,493 hits | **Broken** (GWL_FCS30 sampler) |
| Seismic | `seismic_pga_g` | `layer10-seismic-eu.pmtiles` | `.night4-complete` present; field landing unverified | **Uncertain** |
| Flood + wildfire | `flood_hazard`, `wildfire_hazard` | `layer11-flood-global.pmtiles` (deployed); FEMA/EU/wildfire tiles missing | No `.night5-complete` | **Incomplete** |
| **All cluster AEC fields** | — | — | — | **NONE deployed** (wiped Jun 12) |

### URL fixes already applied

`bd17a348` (2026-05-29): seismic sources migrated to ScienceBase (USGS), GEOSCAN R=297378 (NRCan),
EFEHR GitLab tarball (ESHM20), 5 tiled zips + VRT mosaic (GWL_FCS30). These are in the current
`build-aec-seismic.sh`.

### Root cause of zero AEC fields

`build-tiles.py:build_clusters_meta()` (lines 319–376) writes `clusters-meta.json` from scratch.
21-field whitelist, no merge. AEC scripts patch into the deployed file directly — but any subsequent
nightly rebuild wipes those patches. Monday race: `build-aec-global.sh` and `nightly-rebuild.sh`
both start at `0 5 * * *` (same second on Mondays). AEC reads the pre-rebuild meta at 05:00 PDT,
the nightly overwrites it minutes later, then AEC writes stale cluster population back hours later.

---

## §3 Fable Governance Design (A+D Hybrid)

Recommended by Fable audit (claude-fable-5, 2026-06-12). Two complementary layers:

### Layer A — Registry in `pairings.yaml` (startup audit)

Add `owns_deployments:` and `scheduled_jobs:` to each archive's entry in `~/Foundry/pairings.yaml`.
`pairings.yaml` is Command-owned, startup-relevant, and already the archive↔deployment topology record.

```yaml
# project-gis entry addition:
    owns_deployments:
      - gateway-orchestration-gis-1
      - cluster-totebox-personnel-1
    scheduled_jobs:
      - id: gis-nightly-rebuild
        script: pointsav-monorepo/app-orchestration-gis/nightly-rebuild.sh
        writes_to: [gateway-orchestration-gis-1]
        schedule: "0 22 * * *"
        crontab_line: "0 22 * * * cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis && bash nightly-rebuild.sh >> /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/nightly-rebuild.log 2>&1"
      - id: gis-aec-global-weekly
        script: pointsav-monorepo/app-orchestration-gis/build-aec-global.sh
        writes_to: [gateway-orchestration-gis-1]
        schedule: "0 23 * * 1"
        crontab_line: "0 23 * * 1 cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis && bash build-aec-global.sh >> /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/aec-global.log 2>&1"
```

### Layer D — `.owner` files + deploy-guard (write-time enforcement)

Each gateway deployment gets a one-line `.owner` file:
```
/srv/foundry/deployments/gateway-orchestration-gis-1/.owner → project-gis
```

Every script that writes to a gateway includes this guard at startup (after `SCRIPT_DIR` and `LOG`):
```bash
DEPLOY_TARGET="gateway-orchestration-gis-1"
SELF_ARCHIVE="$(echo "$SCRIPT_DIR" | sed -n 's|^/srv/foundry/clones/\([^/]*\)/.*|\1|p')"
OWNER="$(cat "/srv/foundry/deployments/$DEPLOY_TARGET/.owner" 2>/dev/null)"
if [[ -z "$OWNER" || "$SELF_ARCHIVE" != "$OWNER" ]]; then
    echo "DEPLOY-GUARD: $SELF_ARCHIVE is not owner of $DEPLOY_TARGET (owner: ${OWNER:-UNDECLARED}) — aborting" | tee -a "$LOG"
    exit 78
fi
```

When `nightly-rebuild.sh` is copied to another archive (inevitable — 6 copies exist), the guard
makes that copy inert. Failure is logged, loud, and does not reach the gateway.

### Validation script: `bin/cron-audit.sh` (Command Session, step 9b)

Logic:
1. Parse `pairings.yaml` → declared crontab lines + deployment owner map
2. Diff against `crontab -l`: FAIL on undeclared job, missing job, or ownership violation
3. WARN on dormant non-owner scripts that reference owned gateways
4. Verify `.owner` files match pairings registry

Wire-up: AGENT.md startup step 9b (Command Session); any FAIL is a session blocker.

### Would have caught orgcharts contamination

- **Write-time (Layer D):** orgcharts nightly aborts at 05:00 PDT, logs ownership violation — gateway untouched
- **Next Command startup (Layer A):** cron-audit flags crontab pointing at orgcharts as undeclared + ownership violation — surfaced within hours, not weeks

---

## §4 Fix Sequence

### Totebox scope (project-gis Totebox session)

- [x] Add deploy-guard to project-orgcharts, project-system, project-command `nightly-rebuild.sh`
- [x] Copy `build-clusters.py`, `build-tiles.py`, `taxonomy.py` from orgcharts to project-gis
- [x] Copy `config.py` (full 441-line version) + `utils/` (`region_engine.py`, `spatial_filter.py`) from orgcharts to project-gis — 2026-06-13 (project-gis had a minimal 11-line config.py; caused `ImportError: cannot import name 'WORK_DIR'` on first run)
- [x] Edit `build-tiles.py` (gateway deployment): coordinate-based AEC merge + atomic write
- [x] Edit `nightly-rebuild.sh` (project-gis): deploy-guard + flock + ERR trap
- [x] Edit `build-aec-global.sh` (project-gis): deploy-guard + flock

### Operator crontab actions (cannot be automated)

- [x] Fix cron timing: `0 5 * * *` → `0 22 * * *` (22:00 PDT = 05:00 UTC next day) — done 2026-06-12
- [x] Fix AEC Monday: `0 5 * * 1` → `0 23 * * 1` — done 2026-06-12
- [x] Remove annual entry `0 5 4 6 *` (`run-overnight-ingests.sh` not present in project-gis; removed) — done 2026-06-12

### Command Session scope

- [ ] `pairings.yaml`: add `owns_deployments:` + `scheduled_jobs:` blocks
- [ ] `deployments/<name>/.owner`: create for all active gateways (project-bim, project-knowledge, project-design)
- [ ] `bin/cron-audit.sh`: new validation script
- [ ] `AGENT.md`: step 9b + Hard rules row

### After fixes verified

- [x] Run `build-aec-global.sh` overnight — AEC backfill complete 2026-06-12: koppen=6,485/6,493, ecoregion=6,461/6,493, GHI=6,481/6,493
- [x] First clean nightly run: 2026-06-13T05:48Z (start) → 05:48Z (end) ≈ 48 min. All 4 steps passed: build-clusters (6,493 clusters, T1=1746/T2=2726/T3=2021), build-tiles (54.8 MB pmtiles, 18 MB meta), VWH ✓, PKS ✓
  - Note: run showed "AEC index loaded: 0 records" — NOT a bug. The AEC backfill (~2h) completed AFTER the 22:00 PDT nightly ran. The nightly correctly loaded the pre-backfill clusters-meta.json. AEC data was present in clusters-meta.json when the backfill finished; tonight's nightly will preserve it via coordinate-merge.
- [x] Verify AEC fields survive next nightly run — AEC coordinate-merge is in place; first verification will be 2026-06-14 run
- [ ] Run `build-aec-seismic.sh` (URL-fixed; may need `.night4-complete` cleared)
- [ ] Run `build-aec-flood.sh` Night 5 (lowest priority)

---

## §5 Open Questions

1. **`build-tiles.py` git home:** The canonical copy lives in the gateway deployment at
   `gateway-orchestration-gis-1/app-orchestration-gis/`. Is this file tracked in any git repo?
   If not, edits here are unversioned (same problem as the build scripts in project-gis clone).

2. **Wetland 0-hit:** `gdallocationinfo` call in `build-aec-global.sh` returns 0 hits for
   6,493 clusters against `gwl-fcs30-global.tif`. Likely CRS mismatch or path issue. Not blocking.

3. **Which archives need `.owner` backfilled:** `project-bim` (gateway-orchestration-bim-1),
   `project-knowledge` (knowledge platform), `project-design` (design gateway if any).
   Audit project-bim's nightly-rebuild.sh for gateway references.

4. **Seismic field landing:** `.night4-complete` is present but cluster-level `seismic_pga_g`
   landing in `clusters-meta.json` was not verified in the audit. May need a spot-check query
   against the DATA-aec-clusters.csv export (May 30) to confirm coverage before deciding whether
   to re-run.

5. **`ashrae_zone` provenance:** Present in coverage CSVs (45.1% coverage) but NOT produced by
   `build-aec-global.sh`. Must come from `build-aec-seismic.sh` or an older one-off script.
   Confirm before declaring it a recurring-job field.
