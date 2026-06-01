---
schema: cluster-manifest-v1
cluster: project-data
opened: 2026-06-01
state: active
slm_endpoint: http://localhost:9080

tetrad:
  vendor:
    repo: pointsav-monorepo
    branch: cluster/project-data
    focus: [app-orchestration-gis, service-extraction, tool-acs-miner]
    status: active
  customer:
    repo: woodfine-fleet-deployment
    focus: [gateway-orchestration-gis-1]
    status: active
  deployment:
    instances: [gateway-orchestration-gis-1]
    status: active
  wiki:
    target: media-knowledge-projects
    planned_topics:
      staged_for_pickup:
        - topic-od-catchment-methodology.md (dispatched 2026-05-16)
        - topic-trade-area-data-sources.md (dispatched 2026-05-16)
        - topic-catchment-ranking-methodology.md (dispatched 2026-05-16)
        - topic-regional-name-resolution.md (dispatched 2026-05-31)
        - topic-colocation-tier-nomenclature.md (dispatched 2026-05-31)
        - topic-gis-as-bim-substrate.md (dispatched 2026-05-31)
        - topic-uk-eu-food-retail-coverage.md (dispatched 2026-05-31)
        - topic-rm-plano-tx.md (dispatched 2026-05-30)
        - topic-rm-mississauga-on.md (dispatched 2026-05-30)
        - topic-rm-krefeld-de.md (dispatched 2026-05-30)
        - topic-top-400-regional-markets-na.md (dispatched 2026-05-30)
        - topic-top-400-regional-markets-eu.md (dispatched 2026-05-30)
    status: leg-active
---

# project-data — Cluster Manifest

## Mission

GIS co-location analysis pipeline and JOURNAL academic papers programme. This cluster owns:

- The retail co-location clustering pipeline (`app-orchestration-gis`)
- JOURNAL papers J1–J6 and their pre-submission blockers
- AEC environmental data layers (Köppen/ASHRAE/seismic/flood)
- Regional Markets editorial production (A-series artifacts)
- O-D mobility catchment analysis

The separate `project-gis` archive at `/srv/foundry/clones/project-gis/` runs the crontab
pipeline builds. This archive (`project-data`) is the JOURNAL programme home and tracks the
editorial artifacts.

## Tetrad

See YAML frontmatter above for the canonical tetrad declaration.

## Notes

- AEC cron jobs run in `project-gis` clone, not here — logs at
  `/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/*.log`
- Monorepo branch: confirm `cd pointsav-monorepo && git branch` at session start
- JOURNAL papers are canonical in `JOURNAL/` at archive root
- Regional Markets topics are staged to `drafts-outbound/` before dispatch to project-editorial
