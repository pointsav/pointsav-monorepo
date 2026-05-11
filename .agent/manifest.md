---
schema: foundry-cluster-manifest-v1
cluster_name: project-orgcharts
cluster_branch: cluster/project-orgcharts
created: 2026-04-26
backfilled_triad: 2026-04-26 (per Doctrine v0.0.4)
upgraded_tetrad: 2026-05-01 (per Doctrine v0.0.10 claim #37)
state: active

tetrad:
  vendor:
    - repo: pointsav-design-system
      path: pointsav-design-system/
      upstream: vendor/pointsav-design-system
      focus: components/, tokens/, themes/, templates/ (org-chart components + brand themes)
    - repo: pointsav-media-assets
      path: pointsav-media-assets/
      upstream: vendor/pointsav-media-assets
      focus: PointSav brand marks
    - repo: woodfine-media-assets
      path: woodfine-media-assets/
      upstream: customer/woodfine-media-assets
      focus: Woodfine brand marks
  customer:
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolder: cluster-totebox-corporate/
      tenant: woodfine
      purpose: corporate-document-archive; org-chart authoring runbook + design-system extension procedure
      status: leg-pending — Task to draft GUIDE-orgchart-authoring.md + MANIFEST.md for the catalog folder; Master decision (2026-05-03): Maintain N=3 cluster shape; coordinate via handoff.
  deployment:
    - path: ~/Foundry/deployments/cluster-totebox-corporate-1/
      tenant: woodfine
      shape: corporate-archive
      shared_with: [project-data]   # future Ring 1 services may co-locate here
      runtime_artifacts:
        - inputs/ (Jennifer uploads source files)
        - working/ (Task in-progress drafts)
        - outputs/ (final rendered artifacts: PDF / HTML / SVG)
      status: active — deployment operational since v0.1.5; multiple chart DRAFTs in working/
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-orgcharts/.claude/drafts-outbound/
      gateway: project-language Task
      planned_topics:
        - topic-corporate-chart-design-system.md
        - topic-pre-canon-vs-post-canon-drift.md
      status: leg-pending — skeletons staged 2026-05-01; substance follows after JW7+JW9 REVIEW milestones

clones:
  - repo: pointsav-design-system
    role: primary
    path: pointsav-design-system/
    upstream: vendor/pointsav-design-system
    focus: components/, tokens/, themes/, templates/ (org-chart components + brand themes)
  - repo: pointsav-media-assets
    role: sibling
    path: pointsav-media-assets/
    upstream: vendor/pointsav-media-assets
    focus: PointSav brand marks (signets, wordmarks, favicons)
  - repo: woodfine-media-assets
    role: sibling
    path: woodfine-media-assets/
    upstream: customer/woodfine-media-assets
    focus: Woodfine brand marks (signets, wordmarks, favicons)

deployment_instance: ~/Foundry/deployments/cluster-totebox-corporate-1/
trajectory_capture: enabled

adapter_routing:
  trains:
    - cluster-project-orgcharts  # own cluster adapter (design-system extension during chart authoring)
    - engineering-pointsav       # Vendor engineering corpus (design-system component contributions)
    - tenant-woodfine            # chart authoring is Woodfine-voice content
  consumes:
    - constitutional-doctrine    # always
    - engineering-pointsav       # always — Vendor knowledge
    - cluster-project-orgcharts  # own cluster context (design-system patterns)
    - role-task                  # current role
    - tenant-woodfine            # Woodfine voice for chart artifacts
    # Per Doctrine §IV.b strict tenant isolation: tenant-woodfine adapter
    # trains inside cluster-totebox-corporate-1 (the deployment instance);
    # design-system contributions are Vendor-engineering and ship public
    # (CC BY 4.0 / MIT) per claim #23 Knowledge Commons / Service Commerce.

operator: jennifer-woodfine
---

# Cluster manifest — project-orgcharts

Multi-clone cluster (N=3). Second multi-clone cluster authored
under Doctrine v0.0.2 §IV.c (project-knowledge was the first).
Created 2026-04-26.

## Mission

Author Woodfine corporate org charts (and over time, other
corporate visualizations) using the PointSav design system. Every
UI pattern that emerges during chart authoring — node shapes,
hierarchy connectors, role badges, brand placement — is **backfilled
as a reusable component into `pointsav-design-system`**. The org
charts themselves are corporate artifacts that live in the
deployment instance (`cluster-totebox-corporate-1`); the
extracted components are the open-source design-system contribution
that PointSav publishes.

Operator framing:

> "Build something closer to IBM Carbon or Untitled UI, an
> open-source design system for anyone to use, for our
> Contributors to use for the development of PointSav and for
> Customers and Community Members to use for their own product
> development on top of a Totebox Orchestration, or for Business
> Administration as well. It should be all machine readable."

The org-chart workstream is the first concrete content driving
that growth. Org charts are a high-value, frequently-needed
visualization that exercises a wide range of design-system
primitives (typography, color, layout, hierarchy, brand
identity, print-ready output).

## Operator: Jennifer Woodfine

This cluster is set up for Jennifer Woodfine to operate. Jennifer
is a staging-tier contributor (per workspace `CLAUDE.md` §1) and
operates from her own VM access (provisioned 2026-04-25 per
`infrastructure/operators/jennifer.{pub,setup.md}`).

The Task Claude in this cluster acts as Jennifer's assistant:
- Reads input files Jennifer uploads to
  `~/Foundry/deployments/cluster-totebox-corporate-1/inputs/`
- Authors org-chart drafts in HTML/SVG using design-system
  components
- Backfills new design-system components as patterns emerge
- Coordinates brand-mark usage across both media-assets repos
- Renders final outputs to
  `~/Foundry/deployments/cluster-totebox-corporate-1/outputs/`

## Scope (per sub-clone)

### pointsav-design-system/ — PRIMARY

The PointSav open-source design system. The org-chart workstream
adds new components to this repo as visual patterns emerge:

- `components/` — CSS files (currently controls, egress,
  interactive, layout, ledgers, typography). Likely additions:
  `org-chart-node.css`, `org-chart-tree.css`, `card.css`,
  `badge.css` (role badges), `connector.css` (hierarchy lines).
- `tokens/` — design tokens (color, typography). Org-chart use
  may surface needs for new semantic tokens (e.g.,
  `--token-org-chart-spacing-vertical`).
- `themes/` — currently has `MEMO-Woodfine-Color-Matrix.md`. Add
  per-tenant color themes used by org-chart rendering.
- `templates/` — html + markdown templates. Add
  `org-chart-printable.html` template aligned with
  `template-agnostic-ui.html` precedent.
- `guidelines/` — design memos. Add `MEMO-05-Org-Chart-Patterns.md`
  documenting the conventions extracted from this work.
- `architecture-decisions/` — empty today; first ADR may emerge
  from the org-chart work (e.g., "tree-vs-DAG semantics for
  PointSav org charts").

Repo-level `CLAUDE.md` already configured for staging-tier
(Jennifer/Peter alternation per workspace `CLAUDE.md` §8).

### pointsav-media-assets/ — SIBLING

PointSav brand marks (signets, wordmarks, favicons). Read-only
in most cases — org charts include the PointSav signet only when
the chart represents PointSav-as-Vendor in a relationship
diagram. New PointSav assets land here only if the chart work
surfaces a need (rare).

### woodfine-media-assets/ — SIBLING

Woodfine brand marks (signets, wordmarks, favicons). Org charts
that depict Woodfine corporate structure reference these. New
Woodfine assets land here if e.g. a subsidiary needs its own
signet that doesn't yet exist.

## Deployment instance — cluster-totebox-corporate-1

`~/Foundry/deployments/cluster-totebox-corporate-1/` is the
private corporate document archive for the Woodfine tenant.
Structure:

| Path | Purpose |
|---|---|
| `inputs/` | Jennifer uploads source files (names, titles, hierarchies, photos) |
| `working/` | Task Claude in-progress drafts (HTML mockups, SVG sketches) |
| `outputs/` | Final rendered artifacts (PDF, HTML, SVG) for distribution within Woodfine |
| `.claude/` | Mailbox (rare; most coordination via cluster mailbox) |
| `MANIFEST.md` | Per-instance passport per Doctrine §VII |

This deployment is **private**: gitignored, never pushed, never
ships in the public bundle. Only the design-system components
extracted FROM the chart authoring are public. The charts
themselves stay internal to Woodfine.

## Branch

`cluster/project-orgcharts` in each sub-clone. Created 2026-04-26
from local upstream `main`.

## Remotes

- `origin` — canonical via admin SSH alias (pointsav for
  pointsav-* repos; woodfine for woodfine-media-assets)
- `origin-staging-j` — Jennifer's staging-tier mirror
- `origin-staging-p` — Peter's staging-tier mirror

Push policy: staging-tier only. Per Doctrine §V Action Matrix
and v0.0.10 auto-mode safety brief.

## Trajectory capture

Enabled. L1 capture hook installed in each sub-clone. Every
commit on `cluster/project-orgcharts` writes a JSONL record to
`~/Foundry/data/training-corpus/engineering/project-orgcharts/<sha>.jsonl`.

## Adapter target

`cluster-project-orgcharts` — when L3 (constitutional adapter
training) ships at v0.5.0+, this adapter trains from the
cluster's accumulated commits. The adapter encodes
"design-system extension during chart authoring" as a skill;
composes with per-tenant adapters (PointSav vs Woodfine voice)
at request time per Doctrine claim #22.

## Mailbox

- Inbox: `~/Foundry/clones/project-orgcharts/.claude/inbox.md`
- Outbox: `~/Foundry/clones/project-orgcharts/.claude/outbox.md`
- Trajectory log: `~/Foundry/clones/project-orgcharts/.claude/trajectory-log.md`
  (created on first L2 capture)

## Cross-cluster coordination

- `project-knowledge` — the wiki (when running) may eventually
  serve org-chart documentation as TOPICs. Cross-cluster
  coordination via Master outbox; not synchronous.
- `project-slm` / `project-data` — no direct dependency.

## State as of provisioning (2026-04-26)

| Item | State |
|---|---|
| Cluster directory + manifest | Done — this commit |
| Sub-clones (3) cloned + branched | Done — this commit |
| Remotes configured | Done — this commit |
| Capture hooks installed | Done — this commit |
| Deployment instance directory + MANIFEST | Done — this commit |
| First Task Claude session | Pending — opens via Claude Code in this cluster directory |
| First operator input upload | Pending — Jennifer drops files into deployments/cluster-totebox-corporate-1/inputs/ |

---

*Provisioned 2026-04-26 in workspace v0.1.5 / Doctrine v0.0.2.*
*Tetrad upgrade 2026-05-01 per Doctrine v0.0.10 claim #37.*
oned 2026-04-26 in workspace v0.1.5 / Doctrine v0.0.2.*
*Tetrad upgrade 2026-05-01 per Doctrine v0.0.10 claim #37.*
