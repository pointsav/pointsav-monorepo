---
schema: foundry-cluster-manifest-v1
cluster: project-workplace
cluster_name: project-workplace
cluster_branch: cluster/project-workplace
created: 2026-05-27
state: active
slm_endpoint: http://localhost:8011
module_id: workplace

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: >
        os-workplace/ + app-workplace-workbench/ (Wave 1) +
        app-workplace-memo/ (Wave 1) + app-workplace-presentation/ (Wave 1) +
        app-workplace-proforma/ (Wave 2) + app-workplace-pdf/ (Wave 2) +
        app-workplace-gis/ (Wave 2) + app-workplace-bim/ (Wave 3 reserved)
      status: active (foundation commit 2026-05-27; Wave 1 active development)
  customer:
    - status: leg-pending
      note: >
        woodfine-fleet-deployment catalog entries (download links + release notes)
        planned for Wave 2 when first macOS binaries are ready for distribution.
  deployment:
    - status: leg-pending
      note: >
        Desktop apps ship as macOS binaries via project-software binary-targets.yaml.
        No server-side deployment. Wave 1 binaries target x86_64-apple-darwin (Intel Mac).
  wiki:
    - status: leg-pending
      note: >
        BRIEF-workplace-desktop-suite.md staged in this archive.
        Architecture TOPICs and GUIDE-workplace-* to be written when Wave 1 apps
        are functional; route via project-editorial.

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
