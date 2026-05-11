---
schema: foundry-cluster-manifest-v1
cluster_name: project-bookkeeping
cluster_branch: cluster/project-bookkeeping
created: 2026-04-27
amended: 2026-04-28 (operator + working_pattern fields added per operator direction; workspace v0.1.51)
state: active
doctrine_version: 0.0.10
doctrine_claims_codified: [36, 37]

operator: jennifer
working_pattern: apprentice-with-domain-expert
input_shape: excel-files-plus-operations
spec_via_operation: true

# Jennifer Woodfine is the cluster operator (not Mathew). She brings
# real bookkeeping work to the cluster: a series of Excel files +
# the procedural knowledge of the operations she does as bookkeeper.
# Task Claude pair-works alongside her as apprentice + structured-
# capture engine: observes operations, asks clarifying questions,
# captures trajectory as JSONL events in apprenticeship corpus,
# gradually builds operational schema. Develop the logic FIRST
# (from real operations); build the automation LATER (from the
# captured spec). The eventual app-workplace-accounting +
# app-console-bookkeeper + service-bookkeeper inherit their
# behavioral spec from Jennifer's actual operations.
#
# This inverts claim #32 Apprenticeship Substrate: in #32 service-
# slm is the apprentice and engineers are seniors; here Task Claude
# is the apprentice and Jennifer is the master craftsperson. The
# captured trajectory eventually trains the software.
#
# The vault layout (/source /ledger /asset) absorbs Excel files as
# source artifacts after ingestion; the operations themselves emit
# trajectory JSONL parallel to the apprenticeship corpus pattern.

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: |
        4 projects in scope —
          * service-bookkeeper (NEW; Ring 2 derived-table cache + EN 16931 parser + Beancount/hledger exporter)
          * service-input (extension only; Ring 1 ZUGFeRD/Factur-X PDF/A-3 + Peppol UBL parser)
          * app-console-bookkeeper (READ surface — browse + audit + CSV export + source-document view)
          * app-workplace-accounting (NEW; PRODUCTIVE surface — trial balance + statements + tax compliance)
        Codifies Doctrine claim #36 (Data Vault Bookkeeping Substrate).
  customer:
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolder: cluster-totebox-corporate/
      tenant: woodfine
      purpose: corporate-document-archive shared with project-orgcharts; bookkeeping operations runbook + vault-export procedure for accountant
      status: leg-pending — Task drafts GUIDE-bookkeeping-operations.md + GUIDE-vault-export.md inside cluster-totebox-corporate/ catalog folder; Master coordinates §11 cross-repo rehoming if fan-out
  deployment:
    - path: ~/Foundry/deployments/cluster-totebox-corporate-3/
      tenant: woodfine
      shape: corporate-bookkeeping-vault
      vault_layout:
        - vault/source/    (immutable PDF + .metadata.json + .peppol.xml — append-only via service-fs)
        - vault/ledger/    (canonical journal entries — append-only via service-fs; YYYY/MM.jsonl)
        - vault/asset/     (chart-of-accounts.yaml + balance/YYYY/MM/<account>.jsonl derived from ledger replay)
      shared_with: [project-data]   # Ring 1 service-fs + service-input services consumed
      runtime_artifacts:
        - inputs/  (operator-staged invoices + receipts before service-input ingestion)
        - working/ (Task in-progress builds)
        - exports/ (Beancount / hledger / CSV / Peppol UBL exports for accountant)
      status: pre-created 2026-04-27 (v0.1.45)
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-bookkeeping/.claude/drafts-outbound/
      gateway: project-language Task (sweep + refine + handoff to content-wiki-documentation Root)
      planned_topics:
        - TOPIC-data-vault-bookkeeping-substrate.md   (vendor-public knowledge of vault-canonical pattern)
        - TOPIC-en-16931-peppol-ubl-ingestion.md      (e-invoicing technical reference)
        - TOPIC-isae-3402-soc-2-vault-attestation.md  (assurance-substrate explanation)
        - TOPIC-trustworthy-system-attestation.md     (TSA quarterly report architecture)
      status: leg-pending — Task drafts in .claude/drafts-outbound/; project-language sweeps + refines

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
    focus: service-bookkeeper + service-input ext + app-console-bookkeeper + app-workplace-accounting
  - repo: woodfine-fleet-deployment
    role: sibling
    path: woodfine-fleet-deployment/
    upstream: customer/woodfine-fleet-deployment
    focus: cluster-totebox-corporate/ catalog GUIDE drafts

deployment_instance: ~/Foundry/deployments/cluster-totebox-corporate-3/
trajectory_capture: enabled (capture-edit hook installed in both sub-clones at provisioning)

adapter_routing:
  trains:
    - cluster-project-bookkeeping  # bookkeeping-substrate authoring skill — vault-as-canonical pattern, EN 16931 parsing, ZUGFeRD/Peppol ingestion, derived-table replay, Beancount/hledger emit
    - tenant-woodfine              # tenant-specific bookkeeping content (chart of accounts, vendor list, account naming)
  consumes:
    - constitutional-doctrine
    - engineering-pointsav
    - cluster-project-bookkeeping
    - tenant-woodfine
    - role-task

cross_cluster_dependencies:
  - cluster: project-data
    why: service-fs (WORM ledger) + service-input (parser engine) live in project-data territory
    interface: service-fs HTTP API at FS_ENDPOINT (per local-fs systemd unit on workspace VM since v0.1.23); service-input as MCP server consumed by service-bookkeeper for parser dispatch
    handoff: project-bookkeeping extends service-input with EN 16931 / ZUGFeRD / Peppol UBL parser modules; project-data Task reviews + accepts the extension via cross-cluster handoff
  - cluster: project-language
    why: TOPIC drafts route through editorial gateway per cluster-wiki-draft-pipeline.md
    interface: drop drafts in .claude/drafts-outbound/; project-language sweeps + refines + hands off to content-wiki-documentation Root
  - cluster: project-slm
    why: service-slm Doorman dispatches apprenticeship briefs on bookkeeping work (claim #32 Apprenticeship Substrate); shadow routing on bookkeeping commits exercises apprentice on financial-document parsing
    interface: capture-edit.py post-commit hook auto-dispatches via /v1/shadow when SLM_APPRENTICESHIP_ENABLED=true
---

# Cluster manifest — project-bookkeeping

Multi-clone N=2 cluster authored under Doctrine v0.0.10 §IV.c +
claim #36 (Data Vault Bookkeeping Substrate) + claim #37 (Project
Tetrad Discipline). Two sub-clones in one cluster directory; one
Task session writes to one `.git/index` at a time.

## Scope

A leapfrog-2030 SMB bookkeeping + accounting substrate that:

1. Stores invoices + receipts in the WORM ledger (`service-fs` from
   project-data), keyed by SHA-256, with parsed EN 16931 semantic
   fields beside the source document. **The vault is canonical.**
2. Separates **bookkeeping** (READ surface: browse + audit + CSV
   export + source-document view) from **accounting** (PRODUCTIVE
   surface: trial balance + statements + tax compliance). The
   customer's accountant uses ANY program against the vault export.
3. Ingests EU e-invoicing formats natively (ZUGFeRD/Factur-X PDF/A-3
   + Peppol UBL XML — mandatory in DE/BE/FR by 2027-2028) and US
   PDF (no federal mandate but state-by-state e-invoicing in CA, IL,
   NY by 2027). The plain-text-accounting movement (Beancount,
   hledger, Ledger CLI) has zero Peppol parser as of 2025; this
   cluster closes that gap.
4. Produces a quarterly **Trustworthy System Attestation (TSA)** —
   Master-signed report citing Sigstore Rekor anchoring (Doctrine
   Invention #7) + per-tenant SSH-signed `allowed_signers` chain +
   WORM append-only ledger property. Customer-rooted; no hyperscaler
   SOC 2 report can match it (their attestation covers their
   controls; TSA covers the customer's data).

## Hyperscaler lock-in landscape (what we leapfrog)

QuickBooks (62% US SMB share) + Xero + Sage + NetSuite + FreshBooks
+ Wave generate lock-in structurally:
- proprietary cloud database — no source-document round-trip
- CSV export is point-in-time without source documents
- migration cost = $2K-$20K accountant hours per SMB switch
- per-tenant WORM ledger would 100×+ their storage cost
- per-tenant signing identity requires identity substrate they don't have
- lock-in IS the business model — separating vault from accounting destroys the moat

The "vault-as-canonical, derived-tables-as-cache" pattern is the de
facto practice at FinOps infrastructure (Modern Treasury $400B+
payments, Fragment, TigerBeetle, Formance, Brex, Mercury, Wise via
Column) but no published standard exists for it as of 2026. Foundry
publishes it as the SMB standard.

## Project tetrad satisfaction

This cluster will (per claim #37 Tetrad Discipline) ship four legs
together at every milestone:
1. **Vendor leg** — code in `pointsav-monorepo` (4 projects above)
2. **Customer leg** — `GUIDE-bookkeeping-operations.md` +
   `GUIDE-vault-export.md` in `woodfine-fleet-deployment`
3. **Deployment leg** — running instance at
   `~/Foundry/deployments/cluster-totebox-corporate-3/`
4. **Wiki leg** — TOPIC drafts in `.claude/drafts-outbound/`,
   refined by project-language, published to
   `vendor/content-wiki-documentation`

Master ratification of any milestone work checks all four legs.

## Cross-references

- Doctrine claim #36: `~/Foundry/DOCTRINE.md` §III row 36
- Doctrine claim #37: `~/Foundry/DOCTRINE.md` §III row 37
- Strategic convention: `~/Foundry/conventions/data-vault-bookkeeping-substrate.md`
- Tetrad convention: `~/Foundry/conventions/project-tetrad-discipline.md`
- Triad predecessor: `~/Foundry/conventions/project-triad-discipline.md`
- Sub-agent research: `.claude/sub-agent-results/A4-data-vault-bookkeeping-research-2026-04-27.md`
