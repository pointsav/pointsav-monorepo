---
schema: foundry-cluster-manifest-v1
cluster: project-source
cluster_name: project-source
cluster_branch: cluster/project-source
created: 2026-05-14
state: provisioned
slm_endpoint: http://localhost:8011
module_id: source
---

# project-source — Cluster Manifest

**Mission:** PointSav canonical-tier development archive. Replaces Root sessions
in `vendor/` for canonical PointSav work. All repository work flows through this
archive and promotes to canonical ledger via Stage 6 (`bin/promote.sh`).

## Scope

- `pointsav-monorepo` — canonical PointSav source (os-*, app-*, system-*, service-*, tool-*, vendor-*, moonshot-*)
- `pointsav-design-system` — PointSav design system canonical source

## Sub-clones (provision at Task session start)

```bash
cd ~/Foundry/clones/project-source/
git clone git@github.com-pointsav-administrator:pointsav/pointsav-monorepo.git
git clone git@github.com-pointsav-administrator:pointsav/pointsav-design-system.git
```

## Tetrad

```yaml
tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: ./pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: canonical PointSav codebase — os-*, app-*, system-*, service-*, tool-*
      status: leg-pending — sub-clone not yet provisioned
    - repo: pointsav-design-system
      path: ./pointsav-design-system/
      upstream: vendor/pointsav-design-system
      focus: PointSav design system
      status: leg-pending — sub-clone not yet provisioned
  customer:
    status: not-applicable — canonical-tier work does not have a customer leg
  deployment:
    status: not-applicable — this archive produces source only; deployment is per-crate
  wiki:
    status: leg-pending — architecture TOPICs for canonical codebase
```

## SLM routing

- Endpoint: `http://localhost:8011` (shared Doorman on foundry-workspace VM)
- Module ID: `source`
- See `slm/` directory for routing configuration
