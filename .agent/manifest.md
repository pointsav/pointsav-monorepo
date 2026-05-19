---
schema: foundry-cluster-manifest-v1
cluster_name: project-knowledge
cluster_branch: cluster/project-knowledge
created: 2026-04-26
backfilled_triad: 2026-04-26 (per Doctrine v0.0.4)
state: active
slm_endpoint: http://localhost:8011
module_id: knowledge

tetrad:                          # upgraded from triad: 2026-04-28 per claim #37 / doctrine v0.0.10
  vendor:
    - repo: content-wiki-documentation
      path: content-wiki-documentation/
      upstream: vendor/content-wiki-documentation
      focus: TOPIC content; ADRs; service summaries; glossary
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: app-mediakit-knowledge/ (the wiki engine crate)
  customer:
    - fleet_deployment_repo: vendor/pointsav-fleet-deployment
      catalog_subfolder: media-knowledge-documentation/
      tenant: pointsav
      purpose: documentation-wiki-deployment-shape; README + MANIFEST + guide-deployment + guide-provision-node ratified 2026-04-26 (v0.1.5); GUIDE-operate-knowledge-wiki bulk draft staged 2026-04-27 in this cluster's drafts-outbound/ for project-language refinement
      status: active
  deployment:
    - path: ~/Foundry/deployments/media-knowledge-documentation-1/
      tenant: pointsav
      shape: wiki-runtime
      runtime_artifacts:
        - /usr/local/bin/app-mediakit-knowledge          # live since 2026-04-27 v0.1.29
        - /etc/systemd/system/local-knowledge.service    # live since 2026-04-26 v0.1.21
        - /var/lib/local-knowledge/state                 # provisioned v0.1.29
        - nginx vhost documentation.pointsav.com         # TLS live since 2026-04-27 v0.1.29 (cert through 2026-07-26)
      status: active                                     # public TLS launch 2026-04-27 16:25Z
  wiki:                          # NEW Tetrad leg per convention §4
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-knowledge/.claude/drafts-outbound/
      gateway: project-language Task (PL.6 in SLM Operationalization Plan §4)
      planned_topics:
        # Already staged as substantive bulk drafts 2026-04-27 (awaiting project-language sweep):
        - topic-app-mediakit-knowledge.md                          # the wiki engine — headline architecture TOPIC
        - topic-documentation-pointsav-com-launch-2026-04-27.md    # current-fact launch milestone
        - topic-substrate-native-compatibility.md                  # Doctrine claim #29 narrative (Action API drop rationale)
        # Skeleton staged 2026-04-28 to demonstrate Tetrad intent:
        - topic-collab-via-passthrough-relay.md                    # Phase 2 Step 7 substrate pattern
        # Substantive bulk drafts staged 2026-04-30 (iteration-2 leapfrog batch):
        - topic-knowledge-wiki-home-page-design.md                 # public-facing home-page narrative (Wikipedia muscle memory + leapfrog)
        - topic-article-shell-leapfrog.md                          # public-facing article-shell leapfrog narrative
        - topic-wiki-provider-landscape.md                         # 25-provider competitive landscape audit (PROSE-TOPIC reference/)
        - guide-keep-the-home-page-the-gold-standard.md            # operational PROSE-GUIDE for keeping home page the gold standard
        # Future planned TOPICs (substance follows in milestone N+1+):
        - topic-source-of-truth-inversion.md                       # git canonical, binary view, CRDT ephemeral
        - topic-wikipedia-leapfrog-design.md                       # the muscle-memory chrome design narrative (existing draft)
      status: active                                     # 3 substantive + 1 skeleton + 4 leapfrog-iteration-2 in flight as of 2026-04-30

design:                          # opt-in per v0.1.57 cluster-design-draft-pipeline; mandatory when triggered (cluster shipped UI)
  rules: [.agent/rules/design-tokens.md]                           # added 2026-05-03 per Design Token Workflow plan
  drafts_via: clones/project-knowledge/.claude/drafts-outbound/
  gateway: project-design Task (DS.* in SLM Operationalization Plan §4)
  planned_drafts:
    # Already staged 2026-04-29 (iteration-1 home-page chrome substrate refinement):
    - component-home-grid                                          # the iteration-1 home page 3×3 category grid
    # Substantive batch staged 2026-04-30 (iteration-2 leapfrog primitives):
    - research-wikipedia-leapfrog-2030                             # DESIGN-RESEARCH — primary research synthesis from 4× Sonnet sub-agents
    - component-citation-authority-ribbon                          # DESIGN-COMPONENT — first-class leapfrog primitive (§6.1 of research)
    - component-research-trail-footer                              # DESIGN-COMPONENT — first-class leapfrog primitive (§6.2; Doctrine claim #39 at article scale)
    - component-freshness-ribbon                                   # DESIGN-COMPONENT — first-class leapfrog primitive (§6.3; per-section dateModified JSON-LD)
    - token-knowledge-wiki-baseline                                # DESIGN-TOKEN-CHANGE — three-tier DTCG bundle additions (PENDING MASTER COSIGN)
  triggers:                                                        # events that warrant staging a DESIGN-* draft
    - new_ui_element_shipped                                       # iteration-1 home-page chrome (component-home-grid precedent)
    - existing_substrate_component_modified                        # font-stack changes, density toggle additions
    - brand_voice_or_accessibility_refinement                      # accessibility-targets changes
    - ai_consumption_hint_introduced                               # JSON-LD schema additions (citation-badges, research-trail, freshness-ribbon)
    - public_launch_component                                      # also stages a topic-component-* PROSE-TOPIC for project-language pickup
  status: active                                                   # 1 staged 2026-04-29 + 5 staged 2026-04-30 in flight

clones:
  - repo: content-wiki-documentation
    role: primary
    path: content-wiki-documentation/
    upstream: vendor/content-wiki-documentation
  - repo: pointsav-monorepo
    role: sibling
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
    focus: app-mediakit-knowledge/
  - repo: pointsav-fleet-deployment
    role: sibling
    path: pointsav-fleet-deployment/
    upstream: vendor/pointsav-fleet-deployment
    focus: media-knowledge-documentation/

deployment_instance: ~/Foundry/deployments/media-knowledge-documentation-1/
trajectory_capture: enabled

adapter_routing:
  trains:
    - cluster-project-knowledge  # own cluster adapter (documentation/wiki writing skill)
    - engineering-pointsav       # Vendor engineering corpus (wiki engine + TOPIC content patterns)
    # NOTE: per-tenant TOPICs (tenant-pointsav, tenant-woodfine) join trains
    # when Customer wikis spin up under their own deployment instances
  consumes:
    - constitutional-doctrine    # always
    - engineering-pointsav       # always — Vendor knowledge
    - cluster-project-knowledge  # own cluster context (documentation/wiki writing)
    - role-task                  # current role
    # tenant-pointsav consumed when authoring PointSav-voice TOPICs (default
    # for media-knowledge-documentation-1 instance); tenant-woodfine when
    # authoring Customer-voice TOPICs in future Customer deployments

wiki_draft_triggers:                  # added v0.1.31 per cluster-wiki-draft-pipeline.md §1
  # Events that warrant staging a bulk draft to .claude/drafts-outbound/.
  # Discretion is the Task's; project-language can request more via outbox
  # if a milestone passes uncovered.
  - wiki_engine_phase_complete        # any of Phases 1, 1.1, 2 (and per-step), 3, 4-8
  - deployment_goes_public_tls        # documentation.pointsav.com pattern (v0.1.29 precedent)
  - architecture_or_inventions_amend  # ARCHITECTURE.md, UX-DESIGN.md, INVENTIONS.md substantive change
  - operational_gap_surfaced_closed   # e.g., ufw firewall fix v0.1.29 → operational GUIDE
  - novel_pattern_shipped             # e.g., collab passthrough relay; substrate-native API set
  - per_project_readme_stale          # README hasn't matched substantive code shift in N commits
  - bcsc_disclosure_event             # material-change disclosure record per ni-51-102 §4
---

# Cluster manifest — project-knowledge

Multi-clone cluster (N=3). First multi-clone cluster authored under
Doctrine v0.0.2 §IV.c. Created 2026-04-26.

## Mission

Build the PointSav knowledge platform. Three workstreams:

1. **TOPIC authorship** in `content-wiki-documentation/` — write
   the wiki content. Every TOPIC committed enters the engineering
   corpus (per `conventions/trajectory-substrate.md`) and shapes
   the `cluster-project-knowledge` adapter (per Doctrine claim
   #21 Role-Conditioned Cluster Adapters).
2. **Wiki engine** in `pointsav-monorepo/app-mediakit-knowledge/`
   — make it buildable, runnable, and theme-aligned with the 95%
   Wikipedia muscle-memory target. The crate already has substantial
   scaffolding (axum server + markdown renderer with wikilinks +
   git-sync editor + search index + four HTML templates).
3. **Deployment** in `pointsav-fleet-deployment/media-knowledge-documentation/`
   — catalog GUIDEs + a runtime instance at
   `~/Foundry/deployments/media-knowledge-documentation-1/`
   serving the wiki at a loopback URL initially, with a
   `documentation.pointsav.com` target for v0.5.0+.

The strategic positioning (Doctrine §XVI, Knowledge Commons): the
wiki becomes a much easier way to access PointSav information than
GitHub. Public-facing entry point. CC BY 4.0 content ships in the
public bundle per Doctrine §VIII at every MINOR doctrine bump.

## Scope (per sub-clone)

### content-wiki-documentation/ — PRIMARY

- TOPIC files (`TOPIC-*.md` and `topic-*.md`): write new ones,
  expand existing ones
- ADRs (`sys-adr-*.yaml`): keep current; add new as architecture
  decisions accrue
- Service summaries (`service-*-NN.yaml`): keep aligned with
  monorepo service state
- Glossary (`glossary-documentation.csv`)
- Per-tenant subfolders (future): `pointsav/`, `woodfine/` for
  per-tenant TOPICs that share a writing protocol but diverge in
  voice. Not present at v0.1.4; introduce when a Woodfine TOPIC
  is needed.
- Repo-level `CLAUDE.md` predates v0.0.2 multi-clone pattern;
  may need amendment by next Root Claude in this repo to reflect
  the new cluster scope. Surface via outbox if material.

### pointsav-monorepo/app-mediakit-knowledge/ — SIBLING (focused)

- Existing crate scaffolding (committed to monorepo `main`):
  - `src/server/` — axum HTTP server
  - `src/renderer/` — markdown + wikilinks + footnotes + TOC
  - `src/editor/` — in-browser editor with git commit
  - `src/search/` — search index
  - `src/sync/git.rs` — git-sync to backing repo
  - `templates/` — article.html, category.html, search.html,
    editor.html
  - `static/` — style.css, wiki.js
  - `tests/fixtures/` — test data
- Task scope: get this crate to a runnable binary state. Do not
  touch other crates in the monorepo this session — only
  `app-mediakit-knowledge/` is in scope.
- Workspace `[members]` may need to add this crate (Layer 1
  audit finding); if blocked, surface via outbox (matches the
  pattern that project-data found with service-fs).

### pointsav-fleet-deployment/media-knowledge-documentation/ — SIBLING (focused)

- Existing catalog GUIDEs:
  - `README.md` (placeholder; references "Sovereign Disclosure
    Standard" which is outdated; update to reflect the
    BCSC-grounded posture per `conventions/bcsc-disclosure-posture.md`)
  - `guide-deployment.md` (catalog runbook; expand with the
    actual deployment procedure once Phase 0 lands)
  - `guide-provision-node.md` (catalog runbook; align with
    `infrastructure/local-slm/` precedent for Linux + systemd
    + binary deployment)
- Add new `MANIFEST.md` for the catalog subfolder per
  Doctrine §VII.
- Task scope: catalog updates + write a starter deployment
  procedure that mirrors `infrastructure/local-slm/` — same
  systemd-unit shape, dedicated `local-knowledge` system user,
  loopback bind by default.

## Branch

`cluster/project-knowledge` in each sub-clone. Created 2026-04-26
from local upstream `main`.

## Remotes (all three sub-clones, same shape)

- `origin` — canonical via admin SSH alias
  (`github.com-pointsav-administrator`)
- `origin-staging-j` — Jennifer's staging-tier mirror
  (`github.com-jwoodfine`)
- `origin-staging-p` — Peter's staging-tier mirror
  (`github.com-pwoodfine`)

Push policy: staging-tier only. Per Doctrine §V Action Matrix
and v0.0.10 auto-mode safety brief.

## Trajectory capture

Enabled. The L1 post-commit hook (`bin/capture-edit.py` v0.1.1)
is installed in each sub-clone's `.git/hooks/post-commit`. Every
commit on `cluster/project-knowledge` writes a corpus record to
`~/Foundry/data/training-corpus/engineering/project-knowledge/<sha>.jsonl`.

## Adapter target

`cluster-project-knowledge` — the planned cluster adapter for
documentation/wiki writing skill (per Doctrine claim #21). When
L3 (constitutional adapter training) ships at v0.5.0+, this
adapter trains from the cluster's accumulated commits + TOPIC
revisions + per-tenant voice signal.

The cluster adapter pairs with per-tenant adapters at request
time (Doctrine claim #22 Adapter Composition Algebra):

```
   base + constitutional + cluster[project-knowledge]
        + tenant[pointsav | woodfine] + role[task]
   = TOPIC-writing personality for this tenant
```

## Mailbox

- Inbox: `~/Foundry/clones/project-knowledge/.claude/inbox.md`
- Outbox: `~/Foundry/clones/project-knowledge/.claude/outbox.md`
- Trajectory log: `~/Foundry/clones/project-knowledge/.claude/trajectory-log.md`
  (created on first L2 capture)

## Cross-cluster coordination

- `project-slm` — service-slm Doorman is the inference path that
  TOPIC-writing Tasks will eventually consume (when service-slm
  matures and the cluster adapter is trained). Coordinate via
  Master outbox; not a synchronous dependency for v0.1.x.
- `project-data` — Ring 1 services (service-fs, service-people,
  service-email, service-input). The wiki may later ingest TOPICs
  via service-input (treating wiki content as another document
  ingest channel) but this is a v0.5.0+ exploration. No
  synchronous dependency now.

## State as of provisioning (2026-04-26)

| Item | State |
|---|---|
| Cluster directory + manifest | Done — this commit |
| Sub-clones (3) cloned + branched | Done — this commit |
| Remotes configured | Done — this commit |
| Capture hooks installed | Done — this commit |
| First Task session | Pending — opens via Claude Code in this directory |
| app-mediakit-knowledge build | First Task priority #1 |
| Wiki running locally | First Task priority #2-3 |
| First TOPICs (or extension of existing) | First Task priority #4 |

---

*Provisioned 2026-04-26 in workspace v0.1.4 / Doctrine v0.0.2.*
