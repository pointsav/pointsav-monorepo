---
schema: foundry-cluster-manifest-v1
cluster: project-proforma
cluster_branch: cluster/project-proforma
created: 2026-05-20
state: active
slm_endpoint: http://localhost:9080
module_id: proforma
doctrine_version: 0.0.10
doctrine_claims_codified: [37]
publication_gate: operator-explicit

operator: jennifer
working_pattern: domain-expert-tool-development
input_shape: excel-files-plus-financial-models
spec_via_operation: true

# Jennifer Woodfine is the cluster operator. The proforma tooling
# exists to model, stress-test, and present financial projections for
# Woodfine Capital Projects real estate limited partnerships (PCLP series).
# tool-proforma is an interactive HTML sensitivity analysis engine:
# sliders drive yield rate, DSCR, LTV, and distribution caps; outputs
# include market value, NAV, compounded return, and interest coverage.
#
# This is NOT bookkeeping software — it is a forward-looking financial
# modelling tool. Project-bookkeeping owns the vault/ledger stack;
# project-proforma owns the LP proforma + sensitivity tooling.
#
# BCSC note: all forward-looking projections carry planned/intended/target
# language. No proforma output is to be published as a verified ledger
# entry (SYS-ADR-19). The BCSC continuous-disclosure posture applies to
# all content produced here (conventions/bcsc-disclosure-posture.md).

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: ./
      upstream: vendor/pointsav-monorepo
      focus: |
        tool-proforma — LP financial sensitivity analysis + proforma engine
          * Interactive HTML sensitivity dashboards (PCLP series)
          * Tearsheet generators (mcorp-tearsheet-*)
          * Reconciliation utilities (visa-recon-*)
  customer:
    - status: leg-pending
      note: may never have customer-facing deliverables; operator decision 2026-05-20
  deployment:
    - status: leg-pending
      note: tool-proforma is a local HTML tool; no server deployment planned at this time
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-proforma/.agent/drafts-outbound/
      gateway: project-editorial
      planned_topics: []
      status: leg-pending

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
    focus: tool-proforma

adapter_routing:
  trains:
    - cluster-project-proforma
    - tenant-woodfine
  consumes:
    - constitutional-doctrine
    - engineering-pointsav
    - cluster-project-proforma
    - tenant-woodfine
    - role-task

cross_cluster_dependencies:
  - cluster: project-bookkeeping
    why: shared domain knowledge (Jennifer's financial operations); vault ledger data
         feeds proforma assumptions; the two tools are operationally coupled but
         architecturally separate
    interface: no direct code dependency; knowledge transfer via operator sessions
