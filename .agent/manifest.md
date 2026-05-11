---
schema: foundry-cluster-manifest-v1
cluster_name: project-editorial
cluster_branch: cluster/project-editorial
renamed_from: project-language
renamed: 2026-05-05
created: 2026-04-27
state: active
editorial_gateway_role: true       # this cluster IS service-language per workspace v0.1.31 (Doctrine claim #35); refines bulk drafts from Master/Root/Task drafts-outbound input ports per cluster-wiki-draft-pipeline.md

tetrad:                            # upgraded from `triad:` per Doctrine v0.0.10 / claim #37 (Project Tetrad Discipline, ratified 2026-04-28); supersedes project-triad-discipline.md
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: service-disclosure/ (NEW project — TOPIC/GUIDE/README schemas + CFG validators + genre template registry + frontmatter validators)
    - repo: content-wiki-documentation
      path: content-wiki-documentation/
      upstream: vendor/content-wiki-documentation
      focus: TOPIC content — including the three style-guide TOPICs (topic-style-guide-readme.md, topic-style-guide-topic.md, topic-style-guide-guide.md) plus TOPICs explaining the 4-family taxonomy + service split + customer-hostability
    - repo: pointsav-fleet-deployment
      path: pointsav-fleet-deployment/
      upstream: vendor/pointsav-fleet-deployment
      focus: GUIDE-* runbooks for substrate deployment shapes; receives polish + new GUIDEs as substrate evolves
    - repo: factory-release-engineering
      path: factory-release-engineering/
      upstream: vendor/factory-release-engineering
      focus: read-mode + propose-via-outbox; project-language Task may READ governance content as wiki source and SUGGEST edits via outbox; never commits directly. Master coordinates governance edits via §8 admin-tier procedure.
      access_mode: read-only-write-via-outbox-handoff
  customer:
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolders:
        - cluster-totebox-corporate/
        - fleet-infrastructure-onprem/
        - media-knowledge-corporate/
      tenant: woodfine
      purpose: customer-tier mirror for substrate adoption; receives Woodfine-voice TOPIC + GUIDE drafts at the appropriate catalog subfolders
      status: leg-pending — Task drafts content as needed; Master coordinates §11 cross-repo rehoming for Customer-tier publishing
  deployment:
    - shape: shared-with-project-knowledge
      tenant: pointsav
      purpose: documentation.pointsav.com (project-knowledge cluster's runtime); project-language's substrate enables wiki content rendered by project-knowledge engine
      currently_running:
        - ~/Foundry/clones/project-knowledge/ (the engine renders project-language's content)
      future: per-tenant Customer instances when Customers spin up their own Foundry-pattern wikis
      status: substrate-shaped-cluster — substrate touches every wiki instance; informational/all-instances per Doctrine §IV.d sub-rule
  wiki:                                                # fourth leg per Doctrine v0.0.10 / claim #37 (Project Tetrad Discipline, ratified 2026-04-28)
    - repo: vendor/content-wiki-documentation
      drafts_via: cross-cluster sweep                  # THIS cluster IS the editorial gateway — refines drafts from all three input ports (Master/Root/Task drafts-outbound), not just self-staged
      gateway: project-language Task (this cluster)
      role: refines bulk drafts from Master + Root + Task drafts-outbound input ports per cluster-wiki-draft-pipeline.md (Doctrine claim #35); also self-stages drafts about its own substrate-explainer subjects
      planned_topics:
        - topic-trajectory-substrate                   # Doctrine claim #19; referenced everywhere in apprenticeship prose
        - topic-disclosure-substrate                   # companion to bcsc-disclosure-posture; per-jurisdiction export adapters
        - topic-citation-substrate                     # Doctrine claim #25; citation registry mechanics
        - topic-style-guide-architecture               # remaining 13 style-guide TOPICs covering genre templates in service-disclosure/templates/ (one per template; -architecture, -changelog, -policy, -license-explainer, -memo, -inventory, -email, -chat, -ticket-comment, -meeting-notes, -contract, -cla, -terms)
      completed_topics_this_milestone: 15              # see git log + CHANGELOG: 3 style-guide + 4 substrate-explainer (Phase 4) + 3 substrate-explainer (Part D) + 1 Phase 1B explainer (decode-time-constraints) + 1 meta-recursive (reverse-funnel) + 3 PK refinements (app-mediakit-knowledge, documentation-pointsav-com-launch, substrate-native-compatibility); collision merge for reverse-funnel + GUIDE refinement (PL.6 4th) + profile-readme-jwoodfine pending
      status: active                                   # gateway operational since workspace v0.1.31 ratification; bin/draft-sweep.sh helper landed at v0.1.32

output_surfaces:                                       # editorial-gateway destinations per cluster-wiki-draft-pipeline.md §3
  - surface: vendor-wiki-topics                        # vendor/content-wiki-documentation/topic-*.md (+ .es.md)
    velocity: daily
    bilingual: true
    origin_ports: [task, master]
  - surface: customer-wiki-corporate                   # customer/content-wiki-corporate/topic-*.md (+ .es.md)
    velocity: weekly
    bilingual: true
    origin_ports: [master]
  - surface: customer-wiki-projects                    # customer/content-wiki-projects/topic-*.md (+ .es.md)
    velocity: weekly
    bilingual: true
    origin_ports: [master, task]
  - surface: customer-fleet-guides                     # customer/woodfine-fleet-deployment/<deploy>/GUIDE-*.md
    velocity: per-deployment-stable
    bilingual: false                                   # English-only operational
    origin_ports: [task, master]
  - surface: vendor-fleet-guides                       # vendor/pointsav-fleet-deployment/<deploy>/GUIDE-*.md
    velocity: per-deployment-stable
    bilingual: false
    origin_ports: [task, master]
  - surface: monorepo-project-readmes                  # pointsav-monorepo/<project>/README.md (+ .es.md)
    velocity: per-project-state-change
    bilingual: true
    origin_ports: [task]
  - surface: monorepo-project-claudemd                 # pointsav-monorepo/<project>/CLAUDE.md
    velocity: per-project-Active-or-major-refactor
    bilingual: false                                   # English operational
    origin_ports: [task]
  - surface: engineering-repo-root-readmes             # repo-root README.md (+ .es.md) for each engineering repo
    velocity: quarterly-or-per-MAJOR
    bilingual: true
    origin_ports: [root]
  - surface: workspace-readme                          # ~/Foundry/README.md (+ .es.md)
    velocity: yearly-or-per-doctrine-MAJOR
    bilingual: true
    origin_ports: [master]
  - surface: factory-release-engineering-markdowns     # vendor/factory-release-engineering/*.md (LICENSE-MATRIX, CLAs, policies)
    velocity: per-governance-decision
    bilingual: false                                   # English legal-register
    origin_ports: [master]
  - surface: identity-profile-readmes                  # identity/<id>/.github/profile/README.md × 4 (jwoodfine, pwoodfine, pointsav-administrator org, woodfine-administrator org)
    velocity: quarterly-or-per-public-launch
    bilingual: true
    origin_ports: [master]                             # Master commits + pushes via SSH alias per CLAUDE.md §8 admin-tier
  - surface: workspace-conventions                     # ~/Foundry/conventions/*.md
    velocity: per-doctrine-claim-addition
    bilingual: false                                   # OPTIONAL editorial pass when Master flags
    origin_ports: [master]
    optional: true                                     # Bloomberg-grade by default; pipeline runs only when Master flags

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
    focus: service-disclosure/ (new project)
  - repo: content-wiki-documentation
    role: sibling
    path: content-wiki-documentation/
    upstream: vendor/content-wiki-documentation
    focus: TOPIC source
  - repo: pointsav-fleet-deployment
    role: sibling
    path: pointsav-fleet-deployment/
    upstream: vendor/pointsav-fleet-deployment
    focus: Vendor showcase GUIDE source
  - repo: factory-release-engineering
    role: sibling
    path: factory-release-engineering/
    upstream: vendor/factory-release-engineering
    focus: governance read-mode + propose-via-outbox
    access_mode: read-only-write-via-outbox-handoff
  - repo: woodfine-fleet-deployment
    role: sibling
    path: woodfine-fleet-deployment/
    upstream: customer/woodfine-fleet-deployment
    focus: Customer-tier mirror

trajectory_capture: enabled (L1 capture-edit hook installed in all five sub-clones at provisioning)

adapter_routing:
  trains:
    - cluster-project-language       # own cluster adapter — substrate-curation skill
    - engineering-pointsav           # Vendor engineering corpus
    - tenant-pointsav                # PointSav-voice editorial work
    - tenant-woodfine                # Woodfine-voice editorial work (when touching woodfine-fleet-deployment content)
  consumes:
    - constitutional-doctrine        # always
    - engineering-pointsav           # always — Vendor knowledge
    - cluster-project-language       # own cluster context
    - role-task                      # current role
    - tenant-pointsav | tenant-woodfine  # composed at request time per which tenant the work is for

apprenticeship_task_types:
  - prose-edit                       # editorial polish on PROSE-family artefacts (READMEs, TOPICs, GUIDEs, MEMOs)
  - comms-edit                       # editorial polish on COMMS-family artefacts
  - frontmatter-normalize            # frontmatter validation + fill-in
  - citation-insert                  # adding [citation-id] references
  - register-tighten                 # Bloomberg-grade compression
  - cross-link-verify                # wiki-link + citation-graph integrity
  - schema-validate                  # service-disclosure CFG + frontmatter validator updates
  - template-author                  # genre-template authorship (template-readme, template-topic, etc.)

wiki_draft_triggers:                 # this cluster's own draft-creation triggers per Doctrine claim #35
  - trigger: substrate-explainer-milestone
    description: cluster's substrate work surfaces a public-facing TOPIC candidate (e.g., new convention ratified, new schema shipped, novel pattern emerges)
    examples: [topic-decode-time-constraints (Phase 1B), topic-reverse-funnel-editorial-pattern (this cluster's role), topic-language-protocol-substrate]
    target_repo: content-wiki-documentation
    audience: vendor-public
  - trigger: style-guide-template-coverage
    description: a genre template in service-disclosure/templates/ warrants a public-facing style-guide TOPIC
    examples: [topic-style-guide-readme, topic-style-guide-topic, topic-style-guide-guide]
    target_repo: content-wiki-documentation
    audience: vendor-public
  - trigger: meta-recursive-pilot
    description: substrate components self-stage to demonstrate the pipeline end-to-end (rare; use for canonical worked examples)
    target_repo: content-wiki-documentation
    audience: vendor-public

design_extraction_rules:
  - rule_file: .agent/rules/design-tokens.md
    description: Mandates routing of generic design tokens to pointsav-design-system and branded tokens to respective media asset repositories.
---

# Cluster manifest — project-language

Multi-clone N=5 cluster (sixth multi-clone cluster overall). Five sub-clones
in one cluster directory; one Task session writes to one `.git/index` at a
time per Doctrine §IV.c.

## Mission

The substrate that makes editorial work in Foundry an audited, per-tenant,
forkable practice. Owns:

- The 4-family adapter taxonomy (PROSE / COMMS / LEGAL / TRANSLATE)
- Genre-template prompt scaffolding (template-readme, template-topic,
  template-guide, template-memo, template-architecture, template-inventory,
  template-license-explainer, template-changelog, template-email,
  template-chat, template-ticket-comment, template-meeting-notes,
  template-contract, template-cla, template-policy, template-terms — plus
  TRANSLATE per-language-pair)
- `service-disclosure/` Rust crate (NEW project in pointsav-monorepo) —
  TOPIC/GUIDE/README schemas + CFG validators + frontmatter validators +
  banned-vocabulary CFG + genre-template registry
- Three style-guide TOPICs in content-wiki-documentation
- Apprenticeship corpus directory tree under
  `data/training-corpus/apprenticeship/<task-type>/<tenant>/`
- Per-tenant adapter training pipeline (Pattern A primary; B/C/D evolve)

## Required reading (in order, before Phase 1)

1. **`~/Foundry/conventions/language-protocol-substrate.md`** — the
   convention this cluster implements. Read end-to-end.
2. **`DOCTRINE.md` claims #15, #21, #22, #25, #31, #32, #33** — the
   substrate-composing claim set
3. **`~/Foundry/conventions/apprenticeship-substrate.md`** —
   verdict-signed editorial training mechanism
4. **`~/Foundry/conventions/trajectory-substrate.md`** — corpus typology
5. **`~/Foundry/conventions/adapter-composition.md`** — composition algebra
6. **`~/Foundry/conventions/system-substrate-doctrine.md`** — kernel
   substrate beneath all this
7. **Cluster manifest** at `.claude/manifest.md` — your scope
8. **`~/Foundry/CLAUDE.md`** §6 (Bloomberg-grade language standard + BCSC
   posture) + §11 (action matrix) + §13 (root-files-discipline) + §14
   (TOPIC vs GUIDE)
9. **Project registries** in each sub-clone

## Branch + remotes

`cluster/project-language` in each sub-clone (created 2026-04-27 from local
upstream `main`).

Engineering-tier sub-clones (pointsav-monorepo, content-wiki-documentation,
pointsav-fleet-deployment): `origin` admin alias + `origin-staging-j` +
`origin-staging-p`.

`factory-release-engineering`: `origin` admin alias only — admin-only repo
per CLAUDE.md §2; Task does NOT push here. Read-mode + propose-via-outbox.

`woodfine-fleet-deployment`: customer-tier admin alias `origin` only.

## Trajectory capture

Enabled. L1 capture hook installed in all 5 sub-clones at provisioning.
Every commit on `cluster/project-language` enters
`~/Foundry/data/training-corpus/engineering/project-language/<sha>.jsonl`.

## Cross-cluster coordination

- **`project-proofreader` Task** consumes `service-disclosure/` Rust crate
  via Cargo dependency. When `project-language` ships new schemas /
  templates / CFG, project-proofreader picks them up via version bump. No
  mailbox handoff needed for routine version bumps; mailbox handoff for
  breaking changes (semver MAJOR).
- **`project-knowledge` Task** owns the wiki engine (`app-mediakit-knowledge`)
  that renders content this cluster authors. If project-language needs
  engine support (e.g., `category:` frontmatter parsing for by-category
  panels, `TOPIC-HOME.md` as home), surface to Master via outbox; Master
  relays to project-knowledge Task.
- **`project-slm` Task** owns `service-content` (data substrate) and
  `service-slm` (Doorman). project-language's apprenticeship corpus feeds
  into the per-tenant adapter training pipeline that runs through
  service-slm. Cross-cluster contract is stable; no immediate coordination
  needed.

## Mailbox

- Inbox: `~/Foundry/clones/project-language/.claude/inbox.md`
- Outbox: `~/Foundry/clones/project-language/.claude/outbox.md`
- Trajectory log: `~/Foundry/clones/project-language/.claude/trajectory-log.md`

---

*Provisioned 2026-04-27 in workspace v0.1.22 / Doctrine v0.0.8.*
