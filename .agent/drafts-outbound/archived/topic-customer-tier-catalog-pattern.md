---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-proofreader
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-customer-tier-catalog-pattern.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-04-28T01:30:00Z
authored_by: task-project-proofreader (brief-1-sonnet-subagent)
authored_with: sonnet-4-6
references:
  - ~/Foundry/CLAUDE.md
  - ~/Foundry/DOCTRINE.md
  - ~/Foundry/MEMO-2026-03-30-Development-Overview-V8.md
  - ~/Foundry/customer/woodfine-fleet-deployment/media-proofreader-woodfinegroup/
notes_for_editor: |
  Skeleton stage. Section headings + (draft-pending — substance
  follows in milestone N+1) markers per Tetrad upgrade message.

  This TOPIC generalises the catalog/instance distinction beyond the
  proofreader deployment specifically. The pattern applies to every
  entry in woodfine-fleet-deployment (and, in the intended future,
  to any tenant fleet). The proofreader is used as the worked example
  because it is the cluster's own deployment and the most concrete
  reference available at authoring time — project-language should
  confirm the example remains the most appropriate one when the
  refinement pass runs.

  When refining:
  - Resolve references to Doctrine §VII with the citation ID registered
    against ~/Foundry/citations.yaml; register it if not yet present
  - Apply structural-positioning per CLAUDE.md §6: describe what
    Foundry does and how it composes; do not name external platforms
    by competitive contrast
  - Generate the bilingual .es.md overview per DOCTRINE §XII
  - Apply Bloomberg-article register throughout
  - Pare to approximately 800-1200 words in the English version
  - The tenancy-agnostic / numbered-instance distinction is the
    conceptual core — ensure it reads clearly to a reader who has
    not yet encountered the Foundry deployment model
  - Confirm that the Tier-3 fleet-node convention section accurately
    reflects the Nomenclature Matrix §4 naming rules for fleet-,
    route-, gateway-, cluster-, node-, media-, vault- prefixes at
    the time of the language pass

  Citations to register if not yet present:
  - DOCTRINE.md §VII (deployment lifecycle; the canonical source for
    the catalog/instance distinction)
  - CLAUDE.md §10 (Deployment lifecycle and the catalog/instance
    pattern; operational form of Doctrine §VII)
  - MEMO-2026-03-30-Development-Overview-V8.md §4 (unified deployment
    catalog; confirms what is live)

  Suggested length when substance lands: 800-1200 words English.
---

# The customer-tier catalog pattern

(draft-pending — substance follows in milestone N+1)

The customer tier separates deployment definitions from deployment
instances: the catalog records what a deployment is; numbered instances
record where and how it runs. Substantive coverage forthcoming: why
this separation matters for multi-tenant and multi-instance scenarios;
how the catalog/instance split maps to Git tracking decisions; the
DOCTRINE §VII provenance of the pattern.

## Why catalog and instance are different shapes

(draft-pending — substance follows in milestone N+1)

A catalog entry is tenancy-agnostic: it describes what the deployment
does, what runbooks operate it, and what artefacts it produces — without
encoding any tenant-specific values. Substantive coverage forthcoming:
the contrast between a catalog (tracked in Git, public or semi-public,
reusable) and an instance (local-only, numbered, may contain secrets or
runtime state that must not reach GitHub); why conflating the two is a
recurring operational mistake and how the naming convention prevents it.

## What lives in the catalog (tenancy-agnostic)

(draft-pending — substance follows in milestone N+1)

The catalog subfolder at
`customer/woodfine-fleet-deployment/<deployment-name>/` holds the
static showcase definition for one deployment name. Substantive
coverage forthcoming: the required artefacts (README.md +
README.es.md + MANIFEST.md + guide-*.md runbooks); the rule that
GUIDE files belong inside the owning deployment subfolder, not at
the fleet-deployment root; how a single catalog entry can serve
multiple concurrent instances across tenants.

## What lives in the instance (numbered, local-only)

(draft-pending — substance follows in milestone N+1)

An instance lives at `~/Foundry/deployments/<deployment-name>-N/`
and is gitignored at the workspace level. Substantive coverage
forthcoming: the required instance artefacts (MANIFEST.md using the
`templates/deployment-MANIFEST.md.tmpl` shape; README.md +
README.es.md); the fields that distinguish instances of the same
deployment name (tenant, purpose, source_version, state); the
opt-in Git-tracking pattern for long-lived instances whose
configuration evolves.

## The Tier-3 fleet-node convention

(draft-pending — substance follows in milestone N+1)

Deployment names follow the Nomenclature Matrix §4 prefix taxonomy.
Substantive coverage forthcoming: the seven canonical prefixes
(fleet-, route-, gateway-, cluster-, node-, media-, vault-) and the
semantic scope each covers; how the `media-` prefix applies to
customer-facing content-processing deployments such as
`media-proofreader-woodfinegroup`; how the prefix taxonomy makes
instance numbering and tenant attribution readable without consulting
the MANIFEST.

## media-proofreader-woodfinegroup as a worked example

(draft-pending — substance follows in milestone N+1)

`media-proofreader-woodfinegroup` is the project-proofreader cluster's
own deployment. Substantive coverage forthcoming: the catalog entry at
`customer/woodfine-fleet-deployment/media-proofreader-woodfinegroup/`;
the instance at `~/Foundry/deployments/media-proofreader-woodfinegroup-1/`;
how the MANIFEST.md fields differ between catalog and instance for this
specific deployment; which GUIDE files belong to the catalog entry;
why `woodfinegroup` appears in the deployment name rather than just
`woodfine` (tenant disambiguation for future multi-instance expansion).

## How an instance is provisioned from the catalog

(draft-pending — substance follows in milestone N+1)

Provisioning is Task-layer work per the DOCTRINE §V action matrix.
Substantive coverage forthcoming: the sequence a Task Claude follows
to provision a new numbered instance from a catalog entry (read
guide-*.md; create the deployments/ directory; fill MANIFEST.md from
the template; copy or generate per-instance config); the operator
intervention points (credentials, billing, DNS) that are
carve-outs from Task scope; how the instance MANIFEST records
source_version to enable reproducible reprovisioning.

## Decommissioning — graceful tear-down by the owning Task

(draft-pending — substance follows in milestone N+1)

Decommissioning follows a two-party model per DOCTRINE §V: the Task
that owns the instance performs graceful tear-down; Master Claude
audits that the tear-down completed. Substantive coverage forthcoming:
the graceful tear-down sequence (stop services, archive runtime state,
remove deployments/ directory); the audit record Master records in
the workspace CHANGELOG; the distinction between graceful-decommission
(Task-initiated) and emergency-decommission (Master-initiated with
operator authority); what happens to the catalog entry when all
instances of a deployment name are decommissioned (catalog remains;
a future instance can reprovision from it).

## See also

(draft-pending — substance follows in milestone N+1)

- `DOCTRINE.md §VII` (deployment lifecycle; canonical source for
  this pattern)
- `CLAUDE.md §10` (Deployment lifecycle and the catalog/instance
  pattern; operational form)
- `CLAUDE.md §9` (Project lifecycle; the parallel pattern one tier
  up — projects in engineering repos mirror the catalog/instance
  separation)
- `topic-editorial-pipeline-three-stages.md` (the proofreader pipeline
  that the `media-proofreader-woodfinegroup` instance runs; companion
  TOPIC)
- `topic-language-protocol-substrate.md` (the substrate the
  pipeline implements; companion TOPIC in the same Tetrad wiki leg)
- `customer/woodfine-fleet-deployment/media-proofreader-woodfinegroup/`
  (the worked-example catalog entry; vendor-internal)
